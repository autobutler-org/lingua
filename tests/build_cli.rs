//! `lingua build`, `lingua tir`, `lingua lir`, and `lingua asm` through the real binary. These run on every
//! host; running the executable is `tests/e2e.rs`'s job.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

const HELLO: &str =
    "fn main() -> Nothing\n  pure\n\n  log(\"hello, world\")\n  return nothing\nend fn main\n";

// A fresh, empty working directory for one test
fn workdir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("build_cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn lingua(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_lingua"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn build_writes_an_executable_lingua_out() {
    let dir = workdir("hello");
    std::fs::write(dir.join("hello.lf"), HELLO).unwrap();
    let output = lingua(&dir, &["build", "hello.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let image = std::fs::read(dir.join("lingua-out")).unwrap();
    assert_eq!(&image[..4], b"\x7fELF");
    assert!(!dir.join("lingua-out.tmp").exists());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(dir.join("lingua-out"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o755);
    }
    if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        assert_eq!(stderr(&output), "");
    } else {
        assert!(
            stderr(&output).starts_with(
                "note: ./lingua-out is a linux/amd64 executable; run it here with `docker run "
            ),
            "{}",
            stderr(&output)
        );
    }
}

#[test]
fn an_unsupported_program_is_a_diagnostic_and_leaves_no_output() {
    let dir = workdir("unsupported");
    std::fs::write(
        dir.join("count.lf"),
        "fn main() -> Nothing\n  pure\n\n  let n: Text = 1\n  return nothing\nend fn main\n",
    )
    .unwrap();
    let output = lingua(&dir, &["build", "count.lf"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        stderr(&output),
        "L020: expected `Text`, found an integer literal\n  --> count.lf:4:17\n   |\n 4 |   let n: Text = 1\n   |                 ^\n"
    );
    assert!(!dir.join("lingua-out").exists());
    assert!(!dir.join("lingua-out.tmp").exists());
}

#[test]
fn no_main_is_l074() {
    let dir = workdir("no_main");
    std::fs::write(dir.join("empty.lf"), "").unwrap();
    let output = lingua(&dir, &["build", "empty.lf"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        stderr(&output),
        "L074: no `main` function to build\n  --> empty.lf:1:1\n   |\n 1 | \n   | ^\n   |\n   = help: add `fn main() -> Nothing`\n"
    );
    assert!(!dir.join("lingua-out").exists());
}

#[test]
fn usage_errors_say_what_to_do() {
    let dir = workdir("usage");
    for (args, message) in [
        (
            &["build", "--dynamic", "hello.lf"][..],
            "error: `--dynamic` is not implemented yet; drop it to build a static linux/amd64 executable\n",
        ),
        (
            &["build", "--target", "linux/arm64", "hello.lf"][..],
            "error: unknown flag `--target` for build\n",
        ),
        (
            &["build", "a.lf", "b.lf"][..],
            "error: path `a.lf` does not exist; check the path and try again\n",
        ),
        (
            &["build", "--refresh", "--offline"][..],
            "error: --refresh and --offline cannot be used together; pick one\n",
        ),
        (
            &["build", "--profile"][..],
            "error: `--profile` needs `debug` or `release`\n",
        ),
        (
            &["build", "--profile", "fast"][..],
            "error: `--profile fast` is not a profile; use `debug` or `release`\n",
        ),
        (
            &["build", "--profile", "debug", "--profile", "debug"][..],
            "error: `--profile debug` was given twice; name each profile once\n",
        ),
        (
            &["check", "--profile", "release"][..],
            "error: unknown flag `--profile` for check\n",
        ),
        (
            &["build", "--serve", "hello.lf"][..],
            "error: `--serve` waits for jobs; run `lingua build PATH` to submit one\n",
        ),
        (
            &["build", "--serve", "--profile", "debug"][..],
            "error: `--serve` does not take `--profile`; the submitted job names its profile\n",
        ),
        (
            &["build", "--serve", "--refresh"][..],
            "error: `--serve` does not take `--refresh` or `--offline`; the submitted job names the cache policy\n",
        ),
        (
            &["build", "--serve", "--stop"][..],
            "error: `--serve` and `--stop` cannot be used together; pick one\n",
        ),
        (
            &["check", "--serve"][..],
            "error: unknown flag `--serve` for check\n",
        ),
    ] {
        let output = lingua(&dir, args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(stderr(&output).starts_with(message), "{}", stderr(&output));
        assert!(stderr(&output).contains("usage: lingua"), "{args:?}");
    }
    assert!(!dir.join("lingua-out").exists());
}

#[test]
fn named_debug_profile_still_writes_lingua_out() {
    let dir = workdir("profile-debug");
    std::fs::write(dir.join("hello.lf"), HELLO).unwrap();
    let output = lingua(&dir, &["build", "--profile", "debug", "hello.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        &std::fs::read(dir.join("lingua-out")).unwrap()[..4],
        b"\x7fELF"
    );
    assert!(!dir.join("lingua-out-release").exists());
}

#[test]
fn release_alone_writes_lingua_out_release() {
    let dir = workdir("profile-release");
    std::fs::write(dir.join("hello.lf"), HELLO).unwrap();
    let output = lingua(&dir, &["build", "--profile", "release", "hello.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let image = std::fs::read(dir.join("lingua-out-release")).unwrap();
    assert_eq!(&image[..4], b"\x7fELF");
    assert!(!dir.join("lingua-out").exists());
    assert!(!dir.join("lingua-out-release.tmp").exists());
    if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        assert_eq!(stderr(&output), "");
    } else {
        let err = stderr(&output);
        assert!(
            err.contains("note: ./lingua-out-release is a linux/amd64"),
            "{err}"
        );
        assert!(
            !err.contains("note: ./lingua-out is a linux/amd64"),
            "{err}"
        );
    }
}

#[test]
fn both_profiles_write_their_own_files() {
    let dir = workdir("profile-both");
    std::fs::write(dir.join("hello.lf"), HELLO).unwrap();
    let output = lingua(
        &dir,
        &[
            "build",
            "--profile",
            "release",
            "--profile",
            "debug",
            "hello.lf",
        ],
    );
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let debug = std::fs::read(dir.join("lingua-out")).unwrap();
    let release = std::fs::read(dir.join("lingua-out-release")).unwrap();
    assert_eq!(&debug[..4], b"\x7fELF");
    assert_eq!(debug, release);
    assert!(!dir.join("lingua-out.tmp").exists());
    assert!(!dir.join("lingua-out-release.tmp").exists());
    if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        assert_eq!(stderr(&output), "");
    } else {
        let err = stderr(&output);
        assert!(err.contains("note: ./lingua-out is a linux/amd64"), "{err}");
        assert!(
            err.contains("note: ./lingua-out-release is a linux/amd64"),
            "{err}"
        );
    }
}

#[test]
fn two_profiles_each_report_a_failed_check() {
    let dir = workdir("profile-failed");
    std::fs::write(
        dir.join("count.lf"),
        "fn main() -> Nothing\n  pure\n\n  let n: Text = 1\n  return nothing\nend fn main\n",
    )
    .unwrap();
    let output = lingua(
        &dir,
        &[
            "build",
            "--profile",
            "debug",
            "--profile",
            "release",
            "count.lf",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    let diagnostic = "L020: expected `Text`, found an integer literal\n  --> count.lf:4:17\n   |\n 4 |   let n: Text = 1\n   |                 ^\n";
    assert_eq!(
        stderr(&output),
        format!("./lingua-out:\n{diagnostic}./lingua-out-release:\n{diagnostic}")
    );
    assert!(!dir.join("lingua-out").exists());
    assert!(!dir.join("lingua-out-release").exists());
}

#[test]
fn stop_without_a_runner_says_how_to_start_one() {
    let dir = workdir("stop-absent");
    let output = lingua(&dir, &["build", "--stop"]);
    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    let err = stderr(&output);
    let root = dir.canonicalize().unwrap();
    assert!(
        err.contains(&format!("no runner is listening in {}", root.display())),
        "{err}"
    );
    assert!(err.contains("lingua build --serve"), "{err}");
}

struct RemoveDir(PathBuf);

impl Drop for RemoveDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct KillChild(Option<Child>);

impl Drop for KillChild {
    fn drop(&mut self) {
        if let Some(child) = self.0.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[test]
fn a_standing_runner_keeps_going_after_a_failed_job() {
    let dir = PathBuf::from(format!("/tmp/lingua-serve-{}-cli", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let _cleanup = RemoveDir(dir.clone());
    std::fs::write(dir.join("hello.lf"), HELLO).unwrap();

    let child = Command::new(env!("CARGO_BIN_EXE_lingua"))
        .args(["build", "--serve"])
        .current_dir(&dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut kill = KillChild(Some(child));
    let child = kill.0.as_mut().unwrap();
    let socket = dir.canonicalize().unwrap().join(".lingua/runner.sock");
    let started = Instant::now();
    while !socket.exists() {
        match child.try_wait() {
            Ok(Some(status)) => panic!("runner exited early: {status}"),
            Ok(None) => {
                assert!(
                    started.elapsed() <= Duration::from_secs(5),
                    "runner did not listen at {}",
                    socket.display()
                );
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(err) => panic!("wait: {err}"),
        }
    }

    let built = lingua(&dir, &["build", "hello.lf"]);
    assert_eq!(built.status.code(), Some(0), "{}", stderr(&built));
    let image = std::fs::read(dir.join("lingua-out")).unwrap();
    assert_eq!(&image[..4], b"\x7fELF");

    std::fs::write(
        dir.join("count.lf"),
        "fn main() -> Nothing\n  pure\n\n  let n: Text = 1\n  return nothing\nend fn main\n",
    )
    .unwrap();
    let failed = lingua(&dir, &["build", "count.lf"]);
    assert_eq!(failed.status.code(), Some(1), "{}", stderr(&failed));
    assert_eq!(
        stderr(&failed),
        "L020: expected `Text`, found an integer literal\n  --> count.lf:4:17\n   |\n 4 |   let n: Text = 1\n   |                 ^\n"
    );
    assert_eq!(std::fs::read(dir.join("lingua-out")).unwrap(), image);

    let again = lingua(&dir, &["build", "hello.lf"]);
    assert_eq!(again.status.code(), Some(0), "{}", stderr(&again));
    assert_eq!(
        &std::fs::read(dir.join("lingua-out")).unwrap()[..4],
        b"\x7fELF"
    );

    let stopped = lingua(&dir, &["build", "--stop"]);
    assert_eq!(stopped.status.code(), Some(0), "{}", stderr(&stopped));
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                assert!(status.success(), "{status}");
                break;
            }
            Ok(None) => {
                assert!(
                    started.elapsed() <= Duration::from_secs(5),
                    "runner did not exit after --stop"
                );
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(err) => panic!("wait: {err}"),
        }
    }
    assert!(!socket.exists());
    kill.0 = None;
}

// The first line where `got` and `want` differ, 1-based, with both versions of it
fn first_difference(got: &str, want: &str) -> String {
    let (got_lines, want_lines): (Vec<&str>, Vec<&str>) =
        (got.lines().collect(), want.lines().collect());
    let line = (0..got_lines.len().max(want_lines.len()))
        .find(|&i| got_lines.get(i) != want_lines.get(i))
        .unwrap_or(got_lines.len().min(want_lines.len()));
    format!(
        "first difference at line {}:\n    expected: {:?}\n    got:      {:?}",
        line + 1,
        want_lines.get(line).unwrap_or(&"<end of file>"),
        got_lines.get(line).unwrap_or(&"<end of file>")
    )
}

// Every e2e case has a `NAME.tir`, `NAME.lir`, and `NAME.amd64.asm` golden beside it. The dumps
// are text, so this runs on every host. `LINGUA_BLESS=1 cargo test --test build_cli` rewrites
// the goldens.
#[test]
fn stage_dumps_match_goldens() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bless = std::env::var_os("LINGUA_BLESS").is_some();
    let dir = root.join("tests/e2e");
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "lf"))
        .collect();
    cases.sort();
    assert!(!cases.is_empty(), "no .lf cases under tests/e2e");
    let failures: Vec<String> = cases
        .iter()
        .flat_map(|source| {
            [("tir", "tir"), ("lir", "lir"), ("asm", "amd64.asm")].map(|dump| (source, dump))
        })
        .filter_map(|(source, (command, extension))| {
            let golden = source.with_extension(extension);
            let name = golden.file_name().unwrap().to_string_lossy().into_owned();
            let output = lingua(root, &[command, source.to_str().unwrap()]);
            if output.status.code() != Some(0) {
                return Some(format!(
                    "{name}: lingua {command} failed:\n{}",
                    stderr(&output)
                ));
            }
            let got = String::from_utf8_lossy(&output.stdout);
            if bless {
                std::fs::write(&golden, got.as_bytes()).unwrap();
                return None;
            }
            match std::fs::read_to_string(&golden) {
                Ok(want) if want == got => None,
                Ok(want) => Some(format!("{name}: {}", first_difference(&got, &want))),
                Err(err) => Some(format!("{name}: cannot read the golden: {err}")),
            }
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} dumps differ from their goldens\n{}\n\nif the change is intended, regenerate \
         them with `LINGUA_BLESS=1 cargo test --test build_cli` and review the diff",
        failures.len(),
        cases.len() * 3,
        failures.join("\n")
    );
}
