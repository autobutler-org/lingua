//! The native backend (docs/build.md). The driver below runs the stages in order: TIR from
//! the checker, lowering to LIR, the runtime, amd64
//! selection, encoding, and the ELF linker. The only target so far is static linux/amd64.

pub mod arch;
pub mod link;
pub mod lir;
#[cfg(test)]
mod lir_eval;
pub mod lower;
pub mod prog;
pub mod runtime;
pub mod target {
    pub mod linux_amd64;
}

use std::path::{Path, PathBuf};

use crate::check::{self, Diagnostic};
use crate::tir::TirProgram;
use arch::amd64;
use link::elf;
use lir::LirProgram;
use target::linux_amd64;

/// Why a build wrote nothing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Exit 2: a usage error, not a Lingua diagnostic.
    Usage(String),
    /// The source does not check, or breaks a link-set rule (`L072`, `L074`, `L075`).
    Diagnostics(Vec<Diagnostic>),
    /// A bug in the backend.
    Internal(String),
}

impl From<check::Error> for Error {
    fn from(err: check::Error) -> Self {
        match err {
            check::Error::Usage(message) => Error::Usage(message),
            check::Error::Diagnostics(diagnostics) => Error::Diagnostics(diagnostics),
        }
    }
}

/// Checks `paths` and their import closure into the link set's TIR.
///
/// # Errors
/// The check's usage error or diagnostics, or the link-set diagnostics.
pub fn check(paths: &[PathBuf], options: &check::Options) -> Result<TirProgram, Error> {
    Ok(check::link(paths, options)?)
}

/// Lowers a checked program to LIR. This is what `lingua lir` prints.
///
/// # Errors
/// An internal lowering error.
pub fn lower(program: &TirProgram) -> Result<LirProgram, Error> {
    lower::lower(program).map_err(Error::Internal)
}

/// Selects amd64 `Prog`s for each user function. This is what `lingua asm` prints.
///
/// # Errors
/// As [`lower`].
pub fn select(program: &TirProgram) -> Result<Vec<Vec<prog::Prog>>, Error> {
    Ok(lower(program)?
        .functions
        .iter()
        .map(amd64::select)
        .collect())
}

/// Builds a checked TIR program into a static linux/amd64 executable image. This is the
/// typechecker's entry into the backend.
///
/// # Errors
/// `Error::Internal` when lowering, encoding, or linking fails.
pub fn build_tir(program: &TirProgram) -> Result<Vec<u8>, Error> {
    let program = lower::lower(program).map_err(Error::Internal)?;
    // Layout order: rt0, user functions, runtime, the fault stub, shims
    let functions: Vec<Vec<prog::Prog>> = std::iter::once(linux_amd64::rt0())
        .chain(
            program
                .functions
                .iter()
                .chain([&program.entry])
                .chain(&runtime::functions())
                .map(amd64::select),
        )
        .chain([linux_amd64::fault()])
        .chain(linux_amd64::shims())
        .collect();
    let encoded = functions
        .iter()
        .map(|progs| amd64::encode(progs))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::Internal)?;
    let rodata: Vec<_> = program
        .rodata
        .into_iter()
        .chain(runtime::rodata())
        .chain(linux_amd64::rodata())
        .collect();
    elf::link(&encoded, &rodata, &runtime::bss()).map_err(Error::Internal)
}

/// Writes `image` to `dir/lingua-out.tmp`, makes it executable, and renames it to
/// `dir/lingua-out`, so a failed write leaves no partial binary. Returns the output path.
///
/// # Errors
/// The I/O error from writing, setting the mode, or renaming.
pub fn write_output(dir: &Path, image: &[u8]) -> std::io::Result<PathBuf> {
    let tmp = dir.join("lingua-out.tmp");
    let out = dir.join("lingua-out");
    elf::write_executable(&tmp, image)
        .and_then(|()| std::fs::rename(&tmp, &out))
        .inspect_err(|_| {
            let _ = std::fs::remove_file(&tmp);
        })?;
    Ok(out)
}
