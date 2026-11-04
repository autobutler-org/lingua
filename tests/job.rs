//! A compile job writes the executable it was given, and a failed job writes nothing.

use std::path::PathBuf;

use lingua::build::{Error, Job, Outcome};
use lingua::check::Options;

const HELLO: &str =
    "fn main() -> Nothing\n  pure\n\n  log(\"hello, world\")\n  return nothing\nend fn main\n";

fn workdir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("job")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn hello(dir: &std::path::Path) -> PathBuf {
    let source = dir.join("hello.lf");
    std::fs::write(&source, HELLO).unwrap();
    source
}

#[test]
fn a_job_writes_its_own_output() {
    let dir = workdir("own-output");
    let source = hello(&dir);
    let output = dir.join("bin").join("hello");
    let outcome = Job::new(vec![source], Options::default(), &output).run();
    assert_eq!(outcome, Outcome::Wrote(output.clone()));
    let image = std::fs::read(&output).unwrap();
    assert_eq!(&image[..4], b"\x7fELF");
    assert!(!dir.join("bin").join("hello.tmp").exists());
    assert!(!dir.join("lingua-out").exists());
}

#[test]
fn a_failed_job_writes_nothing() {
    let dir = workdir("failed");
    let source = dir.join("count.lf");
    std::fs::write(
        &source,
        "fn main() -> Nothing\n  pure\n\n  let n: Text = 1\n  return nothing\nend fn main\n",
    )
    .unwrap();
    let output = dir.join("count-out");
    let outcome = Job::new(vec![source], Options::default(), &output).run();
    assert!(
        matches!(outcome, Outcome::Failed(Error::Diagnostics(ref diagnostics)) if diagnostics.iter().any(|d| d.code == "L020"))
    );
    assert!(!output.exists());
    assert!(!PathBuf::from(format!("{}.tmp", output.display())).exists());
}

#[test]
fn two_jobs_keep_their_own_artifacts() {
    let dir = workdir("two");
    let source = hello(&dir);
    let first = dir.join("first");
    let second = dir.join("second");
    let one = Job::new(vec![source.clone()], Options::default(), &first).run();
    let two = Job::new(vec![source], Options::default(), &second).run();
    assert_eq!(one, Outcome::Wrote(first.clone()));
    assert_eq!(two, Outcome::Wrote(second.clone()));
    assert_eq!(
        std::fs::read(&first).unwrap(),
        std::fs::read(&second).unwrap()
    );
}
