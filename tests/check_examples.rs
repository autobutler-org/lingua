//! Function examples through the real binary: a failing example is a diagnostic and exit 1
//! from both `lingua check` and `lingua build`. Goldens live in `tests/check_examples/`.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/check_examples")
}

// A fresh, empty working directory for one test
fn workdir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("check_examples")
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

#[test]
fn a_failing_example_fails_check_and_build_with_the_l043_golden() {
    let golden = std::fs::read_to_string(fixtures().join("two_cups.stderr")).unwrap();
    for command in ["check", "build"] {
        let dir = workdir(command);
        std::fs::copy(fixtures().join("two_cups.lf"), dir.join("two_cups.lf")).unwrap();
        let output = lingua(&dir, &[command, "two_cups.lf"]);
        assert_eq!(output.status.code(), Some(1), "{command}");
        assert_eq!(String::from_utf8_lossy(&output.stderr), golden, "{command}");
        assert!(!dir.join("lingua-out").exists());
    }
}

#[test]
fn passing_examples_check_silently() {
    let dir = workdir("passing");
    let source = std::fs::read_to_string(fixtures().join("two_cups.lf")).unwrap();
    std::fs::write(dir.join("cups.lf"), source.replace("-> 1000", "-> 500")).unwrap();
    let output = lingua(&dir, &["check", "cups.lf"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn help_no_longer_says_examples_do_not_run() {
    let output = lingua(&fixtures(), &["--help"]);
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(!help.contains("Examples do not run yet"), "{help}");
    assert!(help.contains("Function examples run"), "{help}");
}
