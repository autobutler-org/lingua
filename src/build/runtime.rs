//! The runtime, written as LIR so every target compiles it through the same selection
//! (docs/build.md, Runtime). Only what the current subset calls exists so far.

use super::lir::{BinOp, Block, Inst, LirFn, RoData, Term, text_block};

/// The runtime's functions, in layout order.
#[must_use]
pub fn functions() -> Vec<LirFn> {
    vec![log()]
}

/// The runtime's read-only data.
#[must_use]
pub fn rodata() -> Vec<RoData> {
    vec![text_block("rt.newline", "\n")]
}

// rt.log(text) -> 0: the text's bytes, then "\n", to fd 2.
// ponytail: one `rt.sys_write` per part, so a partial write or EINTR drops bytes; route through
// `rt.write_all` (loop until written, retry EINTR) once LIR has branches.
fn log() -> LirFn {
    let write = |fd, ptr, len| Inst::Call(None, "rt.sys_write".to_owned(), vec![fd, ptr, len]);
    LirFn {
        name: "rt.log".to_owned(),
        params: 1,
        vregs: 9,
        blocks: vec![Block {
            insts: vec![
                Inst::Const(1, 2),
                Inst::Const(2, 8),
                Inst::Bin(BinOp::Add, 3, 0, 2),
                Inst::Load(4, 0, 0),
                write(1, 3, 4),
                Inst::Addr(5, "rt.newline".to_owned()),
                Inst::Bin(BinOp::Add, 6, 5, 2),
                Inst::Const(7, 1),
                write(1, 6, 7),
                Inst::Const(8, 0),
            ],
            term: Term::Ret(Some(8)),
        }],
    }
}
