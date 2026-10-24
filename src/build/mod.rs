//! The native backend (docs/build.md). The driver below runs the stages in order:
//! subset check to TIR, lowering to LIR, the runtime, amd64 selection, encoding, and the ELF
//! linker. The only target so far is static linux/amd64.

pub mod arch;
pub mod link;
pub mod lir;
pub mod prog;
pub mod runtime;
pub mod subset;
pub mod target {
    pub mod linux_amd64;
}

use std::path::{Path, PathBuf};

use crate::ast::Module;
use crate::parse::Diagnostic;
use arch::amd64;
use link::elf;
use lir::LirProgram;
use target::linux_amd64;

/// Why a build wrote nothing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The source is outside what `lingua build` accepts.
    Diagnostics(Vec<Diagnostic>),
    /// A bug in the backend.
    Internal(String),
}

/// Checks `module` and lowers its user functions to LIR. This is what `lingua lir` prints.
///
/// # Errors
/// The diagnostics from `subset::check`.
pub fn lower(module: &Module) -> Result<LirProgram, Vec<Diagnostic>> {
    Ok(lir::lower(&[subset::check(module)?]))
}

/// Selects amd64 `Prog`s for each user function. This is what `lingua asm` prints.
///
/// # Errors
/// The diagnostics from `subset::check`.
pub fn select(module: &Module) -> Result<Vec<Vec<prog::Prog>>, Vec<Diagnostic>> {
    Ok(lower(module)?.functions.iter().map(amd64::select).collect())
}

/// Builds `module` into a static linux/amd64 executable image.
///
/// # Errors
/// `Error::Diagnostics` when the source is outside the supported subset, or
/// `Error::Internal` when encoding or linking fails.
pub fn build(module: &Module) -> Result<Vec<u8>, Error> {
    let program = lower(module).map_err(Error::Diagnostics)?;
    let main = &program.functions[0].name;
    // Layout order: rt0, user functions, runtime, shims
    let functions: Vec<Vec<prog::Prog>> = std::iter::once(linux_amd64::rt0(main))
        .chain(
            program
                .functions
                .iter()
                .chain(&runtime::functions())
                .map(amd64::select),
        )
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
        .collect();
    elf::link(&encoded, &rodata).map_err(Error::Internal)
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
