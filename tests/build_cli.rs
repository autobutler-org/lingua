//! `lingua build`, `lingua lir`, and `lingua asm` through the real binary. These run on every
//! host; running the executable is `tests/e2e.rs`'s job.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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
    ] {
        let output = lingua(&dir, args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(stderr(&output).starts_with(message), "{}", stderr(&output));
        assert!(stderr(&output).contains("usage: lingua"), "{args:?}");
    }
    assert!(!dir.join("lingua-out").exists());
}

#[test]
fn lir_and_asm_dumps_match_goldens() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (command, golden) in [("lir", "hello.lir"), ("asm", "hello.amd64.asm")] {
        let output = lingua(root, &[command, "tests/e2e/hello.lf"]);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        let expected = std::fs::read_to_string(root.join("tests/e2e").join(golden)).unwrap();
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            expected,
            "{golden}"
        );
    }
}
