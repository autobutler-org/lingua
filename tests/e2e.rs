//! End-to-end harness: build each `tests/e2e/*.lf` with the real `lingua build` command, run
//! the executable, and compare its stdout, stderr, and exit code.
//!
//! To add a case, add `tests/e2e/NAME.lf` and its expected output beside it:
//! `NAME.stderr` and `NAME.stdout` (each empty when the file is missing) and `NAME.exit`
//! (the exit code, 0 when missing). Each case builds in `target/tmp/e2e/NAME/`, where its
//! `lingua-out` is left for inspection.
//!
//! The executables are linux/amd64, so this only runs there. `make e2e` runs it in Docker
//! on any other host.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::path::{Path, PathBuf};
use std::process::Command;

fn cases() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/e2e");
    let mut found: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "lf"))
        .collect();
    found.sort();
    found
}

fn expected(source: &Path, extension: &str) -> Option<Vec<u8>> {
    std::fs::read(source.with_extension(extension)).ok()
}

fn run(source: &Path, dir: &Path) -> Result<(), String> {
    // Fresh each run, so a stale lingua-out cannot stand in for one this build failed to write
    let _ = std::fs::remove_dir_all(dir).or_else(|_| std::fs::remove_file(dir));
    std::fs::create_dir_all(dir).map_err(|err| format!("mkdir {}: {err}", dir.display()))?;
    let build = Command::new(env!("CARGO_BIN_EXE_lingua"))
        .arg("build")
        .arg(source)
        .current_dir(dir)
        .output()
        .map_err(|err| format!("run lingua build: {err}"))?;
    if !build.status.success() {
        return Err(format!(
            "lingua build failed ({}):\n{}",
            build.status,
            String::from_utf8_lossy(&build.stderr)
        ));
    }
    let output = Command::new(dir.join("lingua-out"))
        .current_dir(dir)
        .output()
        .map_err(|err| format!("run lingua-out: {err}"))?;
    let mut problems = Vec::new();
    for (stream, got) in [("stdout", &output.stdout), ("stderr", &output.stderr)] {
        let want = expected(source, stream).unwrap_or_default();
        if *got != want {
            problems.push(format!(
                "{stream}: expected {:?}, got {:?}",
                String::from_utf8_lossy(&want),
                String::from_utf8_lossy(got)
            ));
        }
    }
    let exit = expected(source, "exit").map_or(0, |bytes| {
        String::from_utf8_lossy(&bytes)
            .trim()
            .parse()
            .expect("an .exit file holds one integer")
    });
    if output.status.code() != Some(exit) {
        problems.push(format!(
            "exit: expected {exit}, got {:?}",
            output.status.code()
        ));
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n  "))
    }
}

#[test]
fn e2e() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("e2e");
    let cases = cases();
    assert!(!cases.is_empty(), "no .lf files under tests/e2e");
    let failures: Vec<String> = cases
        .iter()
        .filter_map(|source| {
            let name = source.file_stem().unwrap().to_string_lossy();
            run(source, &root.join(name.as_ref()))
                .err()
                .map(|why| format!("{name}:\n  {why}"))
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} e2e cases failed\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
