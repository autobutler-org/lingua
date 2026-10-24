//! The backend end to end from hand-built TIR (`tests/support/cases.rs`): every case builds
//! into a static linux/amd64 executable on every host, and on linux/amd64 each one runs and
//! is compared on stdout, stderr, and exit code. The same cases run through the LIR evaluator
//! in `src/build/lower_tests.rs`.

mod support;

use lingua::build;

#[test]
fn every_case_builds() {
    for case in support::cases::all() {
        let image =
            build::build_tir(&case.program).unwrap_or_else(|err| panic!("{}: {err:?}", case.name));
        assert_eq!(&image[..4], b"\x7fELF", "{}", case.name);
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn every_case_runs_natively() {
    use std::os::unix::ffi::OsStrExt;
    use std::process::Command;

    let root = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("build_tir");
    std::fs::create_dir_all(&root).unwrap();
    let mut failures = Vec::new();
    for case in support::cases::all() {
        let image = build::build_tir(&case.program).unwrap();
        let dir = root.join(case.name);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let exe = build::write_output(&dir, &image).unwrap();
        let output = Command::new(&exe)
            .args(case.args.iter().map(|arg| std::ffi::OsStr::from_bytes(arg)))
            .output()
            .unwrap();
        let got = (
            output.status.code().map(i64::from),
            String::from_utf8_lossy(&output.stderr).into_owned(),
            output.stdout.is_empty(),
        );
        if got != (Some(case.exit), case.stderr.clone(), true) {
            failures.push(format!(
                "{}: expected exit {} and {:?}\n  got {got:?}",
                case.name, case.exit, case.stderr
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
