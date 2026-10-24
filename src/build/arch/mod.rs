//! Per-architecture encoding tables and their shared output.

pub mod amd64;

use super::prog::SymName;

/// A field in a function's bytes that the linker patches once every symbol has an address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reloc {
    /// amd64: the 32-bit displacement at byte `at`, measured from the end of that field
    /// (which is the end of the instruction), to `sym + off`.
    Pcrel32 { at: usize, sym: SymName, off: i32 },
}

/// One function after encoding: its bytes and the relocations into them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Encoded {
    pub name: SymName,
    pub bytes: Vec<u8>,
    pub relocs: Vec<Reloc>,
}
