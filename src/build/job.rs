//! One compilation of a package: the modules to check, the profile, and the file the
//! executable is written to.
//!
//! The runner accepts these and executes them. `lingua build` submits one debug job, or one
//! job per `--profile`.

use std::path::{Path, PathBuf};

use super::{Error, check, write_image};
use crate::check::Options;

/// `debug` or `release`. Check does not read this.
///
/// Both profiles emit the same image until a later change makes release differ. The field is
/// on the job so that change has a place to look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// The default job. `lingua build` writes `lingua-out`.
    Debug,
    /// Writes `lingua-out-release`.
    Release,
}

impl Profile {
    /// `debug` or `release`. Anything else is `None`.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "debug" => Some(Self::Debug),
            "release" => Some(Self::Release),
            _ => None,
        }
    }

    /// The file one-shot `lingua build` writes for this profile.
    #[must_use]
    pub const fn output_name(self) -> &'static str {
        match self {
            Self::Debug => "lingua-out",
            Self::Release => "lingua-out-release",
        }
    }
}

/// A compile job. The output path belongs to the job, so two jobs do not share `lingua-out`.
#[derive(Debug, Clone)]
pub struct Job {
    pub paths: Vec<PathBuf>,
    pub options: Options,
    /// `debug` or `release`. The backend emits the same image for both until release differs.
    pub profile: Profile,
    /// Where the executable is written. One-shot `lingua build` uses [`Profile::output_name`].
    pub output: PathBuf,
}

impl Job {
    #[must_use]
    pub fn new(
        paths: Vec<PathBuf>,
        options: Options,
        profile: Profile,
        output: impl AsRef<Path>,
    ) -> Self {
        Self {
            paths,
            options,
            profile,
            output: output.as_ref().to_path_buf(),
        }
    }

    /// Checks `paths`, links a static linux/amd64 executable, and writes it to `output`.
    /// A failed check or link writes nothing. `profile` does not change the image.
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

#[cfg(test)]
mod tests {
    use super::Profile;

    #[test]
    fn parse_and_output_names() {
        assert_eq!(Profile::parse("debug"), Some(Profile::Debug));
        assert_eq!(Profile::parse("release"), Some(Profile::Release));
        assert_eq!(Profile::parse("Debug"), None);
        assert_eq!(Profile::parse("fast"), None);
        assert_eq!(Profile::Debug.output_name(), "lingua-out");
        assert_eq!(Profile::Release.output_name(), "lingua-out-release");
    }
}
