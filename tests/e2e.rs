//! End-to-end harness: build each program, run the executable, and compare its stdout,
//! stderr, and exit code. Add a row to `CASES` for each new program. Built executables are
//! left in `target/tmp/e2e/` for inspection.
//!
//! The executables are linux/amd64, so this only runs there. `make e2e` runs it in Docker
//! on any other host.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

mod programs;

use std::path::PathBuf;
use std::process::Command;

use lingua::build::link::elf;

struct Case {
    name: &'static str,
    image: fn() -> Vec<u8>,
    stdout: &'static [u8],
    stderr: &'static [u8],
    exit: i32,
}

const CASES: &[Case] = &[Case {
    name: "hello",
    image: programs::hello_image,
    stdout: b"",
    stderr: programs::MESSAGE,
    exit: 0,
}];

fn run(case: &Case, dir: &std::path::Path) -> Result<(), String> {
    let path = dir.join(case.name);
    elf::write_executable(&path, &(case.image)()).map_err(|err| format!("write: {err}"))?;
    let output = Command::new(&path)
        .output()
        .map_err(|err| format!("run {}: {err}", path.display()))?;
    let mut problems = Vec::new();
    for (stream, got, want) in [
        ("stdout", &output.stdout, case.stdout),
        ("stderr", &output.stderr, case.stderr),
    ] {
        if got != want {
            problems.push(format!(
                "{stream}: expected {:?}, got {:?}",
                String::from_utf8_lossy(want),
                String::from_utf8_lossy(got)
            ));
        }
    }
    if output.status.code() != Some(case.exit) {
        problems.push(format!(
            "exit: expected {}, got {:?}",
            case.exit,
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
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("e2e");
    std::fs::create_dir_all(&dir).unwrap();
    let failures: Vec<String> = CASES
        .iter()
        .filter_map(|case| {
            run(case, &dir)
                .err()
                .map(|why| format!("{}:\n  {why}", case.name))
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} e2e cases failed\n{}",
        failures.len(),
        CASES.len(),
        failures.join("\n")
    );
}
