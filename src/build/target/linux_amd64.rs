//! linux/amd64 process entry and raw system-call shims, written as `Prog` lists
//! (docs/build.md, Per-target layer).

use crate::build::arch::amd64::{AX, DI, DX, MOVQ, SI, SP, SUBQ, SYSCALL};
use crate::build::prog::{Addr, CALL, Prog, RET, TEXT};

const SYS_WRITE: i64 = 1;
const SYS_EXIT_GROUP: i64 = 231;

fn sp(off: i32) -> Addr {
    Addr::Mem { base: SP, off }
}

/// Process entry: call `main` with the Lingua ABI, then `exit_group(0)`.
///
/// The kernel enters with `SP` 16-byte aligned at `argc`. Reserving 16 bytes keeps it aligned
/// and gives `main` its result slot without overwriting `argc`.
// ponytail: runs on the OS main-thread stack with no bound check; map the 64 MiB stack and set
// `rt.stack_limit` with the L077 prologue check.
#[must_use]
pub fn rt0(main: &str) -> Vec<Prog> {
    vec![
        Prog::new(TEXT, Addr::sym("_rt0_linux_amd64"), Addr::None),
        Prog::new(SUBQ, Addr::Imm(16), Addr::Reg(SP)),
        Prog::new(CALL, Addr::None, Addr::sym(main)),
        Prog::new(MOVQ, Addr::Imm(0), Addr::Reg(DI)),
        Prog::new(MOVQ, Addr::Imm(SYS_EXIT_GROUP), Addr::Reg(AX)),
        Prog::new(SYSCALL, Addr::None, Addr::None),
    ]
}

/// The raw shims, one function each. A shim has no frame: on entry the return address is at
/// `0(SP)`, argument `i` at `8+8i(SP)`, and the result slot follows the arguments.
#[must_use]
pub fn shims() -> Vec<Vec<Prog>> {
    // rt.sys_write(fd, ptr, len) -> n; `syscall` clobbers CX and R11, which are scratch
    vec![vec![
        Prog::new(TEXT, Addr::sym("rt.sys_write"), Addr::None),
        Prog::new(MOVQ, sp(8), Addr::Reg(DI)),
        Prog::new(MOVQ, sp(16), Addr::Reg(SI)),
        Prog::new(MOVQ, sp(24), Addr::Reg(DX)),
        Prog::new(MOVQ, Addr::Imm(SYS_WRITE), Addr::Reg(AX)),
        Prog::new(SYSCALL, Addr::None, Addr::None),
        Prog::new(MOVQ, Addr::Reg(AX), sp(32)),
        Prog::new(RET, Addr::None, Addr::None),
    ]]
}
