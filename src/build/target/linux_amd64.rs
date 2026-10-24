//! linux/amd64 process entry, the fault stub, and raw system-call shims, written as `Prog`
//! lists (docs/build.md, Per-target layer).

use crate::build::arch::amd64::{
    ADDQ, ANDQ, AX, CMPQ, CX, DI, DX, JCS, JEQ, LEAQ, MOVQ, R8, R9, R10, SI, SP, SUBQ, SYSCALL,
    TESTQ,
};
use crate::build::lir::{RoData, text_block};
use crate::build::prog::{Addr, CALL, JMP, LABEL, Prog, RET, TEXT};
use crate::build::runtime::OOM;

const SYS_WRITE: i64 = 1;
const SYS_MMAP: i64 = 9;
const SYS_GETPID: i64 = 39;
const SYS_GETTID: i64 = 186;
const SYS_SCHED_GETAFFINITY: i64 = 204;
const SYS_CLOCK_GETTIME: i64 = 228;
const SYS_EXIT_GROUP: i64 = 231;

const PROT_READ_WRITE: i64 = 3;
const MAP_PRIVATE_ANONYMOUS: i64 = 0x22;
const CLOCK_REALTIME: i64 = 0;
const AFFINITY_BYTES: i64 = 128;

/// The Lingua stack rt0 maps, and the reserve below `rt.stack_limit` for nosplit routines.
pub const STACK_SIZE: i64 = 64 << 20;
pub const STACK_RESERVE: i64 = 64 << 10;

fn sp(off: i32) -> Addr {
    Addr::Mem { base: SP, off }
}

fn reg(r: u8) -> Addr {
    Addr::Reg(r)
}

fn mov(from: Addr, to: Addr) -> Prog {
    Prog::new(MOVQ, from, to)
}

fn op(as_: crate::build::prog::As, from: Addr, to: Addr) -> Prog {
    Prog::new(as_, from, to)
}

/// `Process.os` and `Process.arch`.
#[must_use]
pub fn rodata() -> Vec<RoData> {
    vec![
        text_block("rt.os", "linux"),
        text_block("rt.arch", "x86_64"),
    ]
}

/// Process entry: save the initial `SP` (at `argc`) in `rt.argp`, map the 64 MiB Lingua
/// stack, set `rt.stack_limit` to its base plus the reserve, switch to its top, and call
/// `rt.entry`, which runs `main` and exits.
///
/// The kernel enters with `SP` 16-byte aligned at `argc`; the stack top is page-aligned, and
/// each call reserves 16 bytes, so `SP` stays aligned at every call.
#[must_use]
pub fn rt0() -> Vec<Prog> {
    vec![
        Prog::new(TEXT, Addr::sym("_rt0_linux_amd64"), Addr::None),
        mov(reg(SP), reg(AX)),
        op(LEAQ, Addr::sym("rt.argp"), reg(CX)),
        mov(reg(AX), Addr::Mem { base: CX, off: 0 }),
        op(SUBQ, Addr::Imm(16), reg(SP)),
        mov(Addr::Imm(STACK_SIZE), reg(AX)),
        mov(reg(AX), sp(0)),
        op(CALL, Addr::None, Addr::sym("rt.sys_mmap")),
        mov(sp(8), reg(AX)),
        op(TESTQ, reg(AX), reg(AX)),
        Prog::new(JEQ, Addr::None, Addr::Label(0)),
        mov(Addr::Imm(STACK_RESERVE), reg(CX)),
        op(ADDQ, reg(AX), reg(CX)),
        op(LEAQ, Addr::sym("rt.stack_limit"), reg(DX)),
        mov(reg(CX), Addr::Mem { base: DX, off: 0 }),
        mov(Addr::Imm(STACK_SIZE), reg(CX)),
        op(ADDQ, reg(CX), reg(AX)),
        mov(reg(AX), reg(SP)),
        op(SUBQ, Addr::Imm(16), reg(SP)),
        op(CALL, Addr::None, Addr::sym("rt.entry")),
        // rt.entry exits; this is never reached
        mov(Addr::Imm(0), reg(DI)),
        mov(Addr::Imm(SYS_EXIT_GROUP), reg(AX)),
        op(SYSCALL, Addr::None, Addr::None),
        Prog::new(LABEL, Addr::Label(0), Addr::None),
        op(LEAQ, Addr::sym(OOM), reg(AX)),
        op(JMP, Addr::None, Addr::sym("rt.fault")),
    ]
}

/// `rt.fault`, entered by `JMP` with a report's address in `AX` from any frame (including a
/// prologue whose stack check failed). It aligns `SP` and calls `rt.die(report)`, which
/// writes the report and exits 1; it runs inside the stack reserve.
#[must_use]
pub fn fault() -> Vec<Prog> {
    vec![
        Prog::new(TEXT, Addr::sym("rt.fault"), Addr::None),
        op(ANDQ, Addr::Imm(-16), reg(SP)),
        op(SUBQ, Addr::Imm(16), reg(SP)),
        mov(reg(AX), sp(0)),
        op(CALL, Addr::None, Addr::sym("rt.die")),
    ]
}

// A shim: Lingua arguments at 8+8i(SP) (no frame) into the syscall registers, `syscall`, and
// the result (if any) into the slot after the arguments. `syscall` clobbers CX and R11, which
// are scratch.
fn shim(name: &str, number: i64, args: &[Addr], result: Option<i32>) -> Vec<Prog> {
    let registers = [DI, SI, DX, R10, R8, R9];
    let mut out = vec![Prog::new(TEXT, Addr::sym(name), Addr::None)];
    out.extend(
        args.iter()
            .zip(registers)
            .map(|(arg, r)| mov(arg.clone(), reg(r))),
    );
    out.extend([
        mov(Addr::Imm(number), reg(AX)),
        op(SYSCALL, Addr::None, Addr::None),
    ]);
    if let Some(slot) = result {
        out.push(mov(reg(AX), sp(slot)));
    }
    out.push(op(RET, Addr::None, Addr::None));
    out
}

/// The raw shims, one function each.
#[must_use]
pub fn shims() -> Vec<Vec<Prog>> {
    let imm = Addr::Imm;
    // rt.sys_mmap(size) -> ptr, or 0 for -4095..-1
    let mut mmap = shim(
        "rt.sys_mmap",
        SYS_MMAP,
        &[
            imm(0),
            sp(8),
            imm(PROT_READ_WRITE),
            imm(MAP_PRIVATE_ANONYMOUS),
            imm(-1),
            imm(0),
        ],
        None,
    );
    mmap.pop();
    mmap.extend([
        mov(imm(-4096), reg(CX)),
        op(CMPQ, reg(CX), reg(AX)),
        Prog::new(JCS, Addr::None, Addr::Label(0)),
        mov(reg(AX), sp(16)),
        op(RET, Addr::None, Addr::None),
        Prog::new(LABEL, Addr::Label(0), Addr::None),
        mov(imm(0), reg(AX)),
        mov(reg(AX), sp(16)),
        op(RET, Addr::None, Addr::None),
    ]);
    vec![
        shim(
            "rt.sys_write",
            SYS_WRITE,
            &[sp(8), sp(16), sp(24)],
            Some(32),
        ),
        shim("rt.sys_exit", SYS_EXIT_GROUP, &[sp(8)], None),
        mmap,
        shim(
            "rt.sys_clock",
            SYS_CLOCK_GETTIME,
            &[imm(CLOCK_REALTIME), sp(8)],
            None,
        ),
        shim("rt.sys_getpid", SYS_GETPID, &[], Some(8)),
        shim("rt.sys_gettid", SYS_GETTID, &[], Some(8)),
        shim(
            "rt.sys_getaffinity",
            SYS_SCHED_GETAFFINITY,
            &[imm(0), imm(AFFINITY_BYTES), sp(8)],
            Some(16),
        ),
    ]
}
