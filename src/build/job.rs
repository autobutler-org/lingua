//! One compilation of a package: the modules to check, and the file the executable is written to.
//!
//! The runner accepts these and executes them. `lingua build` submits one.

use std::path::{Path, PathBuf};

use super::{Error, check, write_image};
use crate::check::Options;

/// A compile job. The output path belongs to the job, so two jobs do not share `lingua-out`.
#[derive(Debug, Clone)]
pub struct Job {
    pub paths: Vec<PathBuf>,
    pub options: Options,
    /// Where the executable is written. One-shot `lingua build` uses `lingua-out`.
    pub output: PathBuf,
}

impl Job {
    #[must_use]
    pub fn new(paths: Vec<PathBuf>, options: Options, output: impl AsRef<Path>) -> Self {
        Self {
            paths,
            options,
            output: output.as_ref().to_path_buf(),
        }
    }

    /// Checks `paths`, links a static linux/amd64 executable, and writes it to `output`.
    /// A failed check or link writes nothing.
    #[must_use]
    pub fn run(&self) -> Outcome {
        let image = match check(&self.paths, &self.options)
            .and_then(|program| super::build_tir(&program))
        {
            Ok(image) => image,
            Err(err) => return Outcome::Failed(err),
        };
        match write_image(&self.output, &image) {
            Ok(()) => Outcome::Wrote(self.output.clone()),
            Err(err) => Outcome::Unwritten {
                path: self.output.clone(),
                error: err.to_string(),
            },
        }
    }
}

/// What [`Job::run`] produced.
#[derive(Debug, PartialEq)]
pub enum Outcome {
    /// The executable is at this path.
    Wrote(PathBuf),
    /// Check or link failed. The output path was not written.
    Failed(Error),
    /// The image was built and the write failed. No partial file was left at `path`.
    Unwritten { path: PathBuf, error: String },
}
