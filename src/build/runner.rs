//! Accepts compile jobs and executes them.
//!
//! Jobs that share a source root run one at a time, because they share that root's `provided/`
//! cache. Jobs of different roots run together, up to the worker count. Results come back in
//! submission order.

use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::thread;

use super::job::{Job, Outcome};
use crate::check;

/// Holds compile jobs and runs them.
#[derive(Debug)]
pub struct Runner {
    jobs: Vec<Job>,
    workers: usize,
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

impl Runner {
    /// A runner with one worker per available CPU. One submitted job still runs alone.
    #[must_use]
    pub fn new() -> Self {
        let workers = thread::available_parallelism().map_or(1, std::num::NonZero::get);
        Self {
            jobs: Vec::new(),
            workers,
        }
    }

    /// `workers` is how many jobs may run at once. Zero means one.
    #[must_use]
    pub fn with_workers(workers: usize) -> Self {
        Self {
            jobs: Vec::new(),
            workers: workers.max(1),
        }
    }

    pub fn submit(&mut self, job: Job) {
        self.jobs.push(job);
    }

    /// Runs every submitted job and returns one outcome per job, in submission order.
    /// The runner is empty afterwards.
    #[must_use]
    pub fn run(&mut self) -> Vec<Outcome> {
        let jobs = std::mem::take(&mut self.jobs);
        if jobs.len() <= 1 || self.workers == 1 {
            return jobs.iter().map(Job::run).collect();
        }
        let roots: Vec<Option<PathBuf>> = jobs.iter().map(root_of).collect();
        schedule(self.workers, &roots, |index| jobs[index].run())
    }
}

/// The source root `check` would use, or `None` when the paths do not resolve.
/// `None` does not share a root with any other job.
fn root_of(job: &Job) -> Option<PathBuf> {
    check::resolve(&job.paths, job.options.source_root.as_deref())
        .ok()
        .map(|(root, _, _)| root)
}

struct Queue {
    pending: VecDeque<usize>,
    busy: HashSet<PathBuf>,
    running: usize,
}

fn schedule<T: Send>(
    workers: usize,
    roots: &[Option<PathBuf>],
    run_one: impl Fn(usize) -> T + Sync,
) -> Vec<T> {
    let count = roots.len();
    let workers = workers.min(count).max(1);
    let queue = Mutex::new(Queue {
        pending: (0..count).collect(),
        busy: HashSet::new(),
        running: 0,
    });
    let ready = Condvar::new();
    let slots: Vec<Mutex<Option<T>>> = (0..count).map(|_| Mutex::new(None)).collect();

    thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| worker(&queue, &ready, roots, &slots, &run_one));
        }
    });

    slots
        .into_iter()
        .enumerate()
        .map(|(index, slot)| {
            lock(&slot).take().unwrap_or_else(|| {
                panic!("job {index} produced no outcome; this is a bug in the runner")
            })
        })
        .collect()
}

fn worker<T, F>(
    queue: &Mutex<Queue>,
    ready: &Condvar,
    roots: &[Option<PathBuf>],
    slots: &[Mutex<Option<T>>],
    run_one: &F,
) where
    T: Send,
    F: Fn(usize) -> T + Sync,
{
    loop {
        let Some(index) = next_index(queue, ready, roots) else {
            ready.notify_all();
            break;
        };
        // `_release` frees the source root on drop, including when the job panics.
        {
            let _release = Release {
                queue,
                ready,
                roots,
                index,
            };
            let value = run_one(index);
            *lock(&slots[index]) = Some(value);
        }
    }
}

struct Release<'a> {
    queue: &'a Mutex<Queue>,
    ready: &'a Condvar,
    roots: &'a [Option<PathBuf>],
    index: usize,
}

impl Drop for Release<'_> {
    fn drop(&mut self) {
        finish(self.queue, self.roots, self.index);
        self.ready.notify_all();
    }
}

fn next_index(queue: &Mutex<Queue>, ready: &Condvar, roots: &[Option<PathBuf>]) -> Option<usize> {
    let mut guard = lock(queue);
    loop {
        if let Some(index) = take_ready(&mut guard, roots) {
            return Some(index);
        }
        if guard.pending.is_empty() && guard.running == 0 {
            return None;
        }
        guard = ready.wait(guard).unwrap_or_else(PoisonError::into_inner);
    }
}

fn take_ready(queue: &mut Queue, roots: &[Option<PathBuf>]) -> Option<usize> {
    let pos = queue
        .pending
        .iter()
        .position(|&index| match &roots[index] {
            Some(root) => !queue.busy.contains(root),
            None => true,
        })?;
    let index = queue.pending.remove(pos)?;
    if let Some(root) = &roots[index] {
        queue.busy.insert(root.clone());
    }
    queue.running += 1;
    Some(index)
}

fn finish(queue: &Mutex<Queue>, roots: &[Option<PathBuf>], index: usize) {
    let mut guard = lock(queue);
    if let Some(root) = &roots[index] {
        guard.busy.remove(root);
    }
    guard.running -= 1;
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    use super::schedule;

    #[test]
    fn different_roots_run_together() {
        let roots = [Some(PathBuf::from("/a")), Some(PathBuf::from("/b"))];
        let started = AtomicUsize::new(0);
        schedule(2, &roots, |_| {
            started.fetch_add(1, Ordering::SeqCst);
            let begin = Instant::now();
            while started.load(Ordering::SeqCst) < 2 {
                assert!(
                    begin.elapsed() < Duration::from_secs(2),
                    "the two jobs never overlapped"
                );
                std::thread::yield_now();
            }
        });
    }

    #[test]
    fn unknown_roots_run_together() {
        let roots = [None, None];
        let started = AtomicUsize::new(0);
        schedule(2, &roots, |_| {
            started.fetch_add(1, Ordering::SeqCst);
            let begin = Instant::now();
            while started.load(Ordering::SeqCst) < 2 {
                assert!(
                    begin.elapsed() < Duration::from_secs(2),
                    "jobs with no source root never overlapped"
                );
                std::thread::yield_now();
            }
        });
    }

    #[test]
    fn one_root_runs_one_at_a_time() {
        let roots = [Some(PathBuf::from("/a")), Some(PathBuf::from("/a"))];
        let inflight = AtomicUsize::new(0);
        schedule(2, &roots, |_| {
            let now = inflight.fetch_add(1, Ordering::SeqCst);
            assert_eq!(now, 0, "two jobs of one source root overlapped");
            std::thread::sleep(Duration::from_millis(40));
            inflight.fetch_sub(1, Ordering::SeqCst);
        });
    }

    #[test]
    fn results_follow_submission_order() {
        let roots = [Some(PathBuf::from("/slow")), Some(PathBuf::from("/fast"))];
        let got = schedule(2, &roots, |index| {
            if index == 0 {
                std::thread::sleep(Duration::from_millis(40));
            }
            index
        });
        assert_eq!(got, vec![0, 1]);
    }
}
