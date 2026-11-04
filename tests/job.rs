//! A compile job writes the executable it was given, and a failed job writes nothing.

use std::path::PathBuf;

use lingua::build::{Error, Job, Outcome, Runner};
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

#[test]
fn the_runner_keeps_submission_order_when_the_second_job_finishes_first() {
    let good_dir = workdir("order-good");
    let bad_dir = workdir("order-bad");
    let good = hello(&good_dir);
    let bad = bad_dir.join("count.lf");
    std::fs::write(
        &bad,
        "fn main() -> Nothing\n  pure\n\n  let n: Text = 1\n  return nothing\nend fn main\n",
    )
    .unwrap();
    let good_out = good_dir.join("out");
    let bad_out = bad_dir.join("out");
    let mut runner = Runner::with_workers(2);
    runner.submit(Job::new(vec![good], Options::default(), &good_out));
    runner.submit(Job::new(vec![bad], Options::default(), &bad_out));
    let outcomes = runner.run();
    assert_eq!(outcomes.len(), 2);
    assert_eq!(outcomes[0], Outcome::Wrote(good_out));
    assert!(matches!(
        outcomes[1],
        Outcome::Failed(Error::Diagnostics(_))
    ));
    assert!(!bad_out.exists());
}

#[test]
fn two_jobs_of_one_package_both_write() {
    let dir = workdir("same-root");
    let source = hello(&dir);
    let first = dir.join("first");
    let second = dir.join("second");
    let mut runner = Runner::with_workers(2);
    runner.submit(Job::new(vec![source.clone()], Options::default(), &first));
    runner.submit(Job::new(vec![source], Options::default(), &second));
    let outcomes = runner.run();
    assert_eq!(
        outcomes,
        vec![
            Outcome::Wrote(first.clone()),
            Outcome::Wrote(second.clone())
        ]
    );
    assert_eq!(
        std::fs::read(first).unwrap(),
        std::fs::read(second).unwrap()
    );
}
