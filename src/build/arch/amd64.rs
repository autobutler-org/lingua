//! amd64 encoding table, after Go's `optab`: each row is keyed by opcode and operand classes.

use super::{Encoded, Reloc};
use crate::build::lir::{BinOp, Inst, LirFn, Term, VReg};
use crate::build::prog::{self, Addr, As, CALL, Prog, RET, Reg, TEXT};

pub const AX: Reg = 0;
pub const CX: Reg = 1;
pub const DX: Reg = 2;
pub const BX: Reg = 3;
pub const SP: Reg = 4;
pub const BP: Reg = 5;
pub const SI: Reg = 6;
pub const DI: Reg = 7;
pub const R8: Reg = 8;
pub const R9: Reg = 9;
pub const R10: Reg = 10;
pub const R11: Reg = 11;
pub const R12: Reg = 12;
pub const R13: Reg = 13;
pub const R14: Reg = 14;
pub const R15: Reg = 15;

const REG_NAMES: [&str; 16] = [
    "AX", "CX", "DX", "BX", "SP", "BP", "SI", "DI", "R8", "R9", "R10", "R11", "R12", "R13", "R14",
    "R15",
];

pub const MOVQ: As = As(0x100);
pub const LEAQ: As = As(0x101);
pub const SYSCALL: As = As(0x102);
pub const ADDQ: As = As(0x103);
pub const SUBQ: As = As(0x104);
pub const PUSHQ: As = As(0x105);
pub const POPQ: As = As(0x106);

const OP_NAMES: [&str; 7] = ["MOVQ", "LEAQ", "SYSCALL", "ADDQ", "SUBQ", "PUSHQ", "POPQ"];

const REX_W: u8 = 0x48;

fn op_name(op: As) -> &'static str {
    prog::generic_name(op)
        .or_else(|| {
            OP_NAMES
                .get(usize::from(op.0.wrapping_sub(MOVQ.0)))
                .copied()
        })
        .unwrap_or("?")
}

fn reg_name(reg: Reg) -> &'static str {
    REG_NAMES.get(usize::from(reg)).copied().unwrap_or("?")
}

/// The Go-syntax dump of a `Prog` list with amd64 names.
#[must_use]
pub fn dump(progs: &[Prog]) -> String {
    prog::dump(progs, op_name, reg_name)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    None,
    Reg,
    /// Fits a sign-extended 32-bit immediate.
    Imm32,
    Imm64,
    Mem,
    Sym,
}

fn class(addr: &Addr) -> Class {
    match addr {
        Addr::None => Class::None,
        Addr::Reg(_) => Class::Reg,
        Addr::Imm(value) if i32::try_from(*value).is_ok() => Class::Imm32,
        Addr::Imm(_) => Class::Imm64,
        Addr::Mem { .. } => Class::Mem,
        Addr::Sym { .. } => Class::Sym,
    }
}

// REX.W with R extending ModRM.reg and B extending ModRM.rm (or the opcode's low bits)
fn rex_w(reg: Reg, rm: Reg) -> u8 {
    REX_W | ((reg >> 3) << 2) | (rm >> 3)
}

fn modrm(md: u8, reg: Reg, rm: Reg) -> u8 {
    (md << 6) | ((reg & 7) << 3) | (rm & 7)
}

// ModRM (plus SIB and displacement) for `[base + off]`. No displacement when it is zero and
// the base allows it, else disp8 when it fits, else disp32. SP and R12 need a SIB byte; BP
// and R13 with mod 00 would mean RIP-relative, so they always take a displacement.
fn mem(reg: Reg, base: Reg, off: i32) -> Vec<u8> {
    let disp: Vec<u8> = match i8::try_from(off) {
        Ok(0) if base & 7 != BP => Vec::new(),
        Ok(disp) => disp.to_le_bytes().to_vec(),
        Err(_) => off.to_le_bytes().to_vec(),
    };
    let md = match disp.len() {
        0 => 0,
        1 => 1,
        _ => 2,
    };
    let mut out = vec![modrm(md, reg, base)];
    if base & 7 == SP {
        out.push(0x24); // SIB: no index, base in ModRM.rm
    }
    out.extend(disp);
    out
}

// 50+r / 58+r: REX.B only for R8-R15, no REX.W (64-bit is the default operand size)
fn push_pop(opcode: u8, reg: Reg) -> Vec<u8> {
    let rex = (reg >= R8).then_some(0x41);
    rex.into_iter().chain([opcode | (reg & 7)]).collect()
}

/// Encodes one function: a `TEXT` followed by its body.
///
/// # Errors
/// An internal error naming the `Prog` when the list does not start with `TEXT` or an
/// operand combination has no table row.
pub fn encode(progs: &[Prog]) -> Result<Encoded, String> {
    let [
        Prog {
            as_: TEXT,
            from: Addr::Sym { name, .. },
            ..
        },
        body @ ..,
    ] = progs
    else {
        return Err(format!(
            "internal error: function does not start with TEXT: `{}`",
            dump(&progs[..progs.len().min(1)]).trim()
        ));
    };
    let mut bytes = Vec::new();
    let mut relocs = Vec::new();
    for prog in body {
        let row = (prog.as_, class(&prog.from), prog.reg, class(&prog.to));
        match (row, &prog.from, &prog.to) {
            ((MOVQ, Class::Imm32, None, Class::Reg), Addr::Imm(value), Addr::Reg(to)) => {
                // C7 /0 id: sign-extended imm32
                let value = i32::try_from(*value).map_err(|err| err.to_string())?;
                bytes.extend([rex_w(0, *to), 0xc7, modrm(3, 0, *to)]);
                bytes.extend(value.to_le_bytes());
            }
            ((MOVQ, Class::Imm64, None, Class::Reg), Addr::Imm(value), Addr::Reg(to)) => {
                // B8+r io
                bytes.extend([rex_w(0, *to), 0xb8 | (to & 7)]);
                bytes.extend(value.to_le_bytes());
            }
            ((MOVQ, Class::Reg, None, Class::Reg), Addr::Reg(from), Addr::Reg(to)) => {
                // 89 /r: MOV r/m64, r64
                bytes.extend([rex_w(*from, *to), 0x89, modrm(3, *from, *to)]);
            }
            ((LEAQ, Class::Sym, None, Class::Reg), Addr::Sym { name, off }, Addr::Reg(to)) => {
                // 8D /r with mod 00 rm 101: [RIP + disp32]
                bytes.extend([rex_w(*to, 0), 0x8d, modrm(0, *to, 5)]);
                relocs.push(Reloc::Pcrel32 {
                    at: bytes.len(),
                    sym: name.clone(),
                    off: *off,
                });
                bytes.extend([0; 4]);
            }
            ((MOVQ, Class::Mem, None, Class::Reg), Addr::Mem { base, off }, Addr::Reg(to)) => {
                // 8B /r: MOV r64, r/m64
                bytes.extend([rex_w(*to, *base), 0x8b]);
                bytes.extend(mem(*to, *base, *off));
            }
            ((MOVQ, Class::Reg, None, Class::Mem), Addr::Reg(from), Addr::Mem { base, off }) => {
                // 89 /r: MOV r/m64, r64
                bytes.extend([rex_w(*from, *base), 0x89]);
                bytes.extend(mem(*from, *base, *off));
            }
            ((ADDQ, Class::Reg, None, Class::Reg), Addr::Reg(from), Addr::Reg(to)) => {
                // 01 /r: ADD r/m64, r64
                bytes.extend([rex_w(*from, *to), 0x01, modrm(3, *from, *to)]);
            }
            ((SUBQ, Class::Imm32, None, Class::Reg), Addr::Imm(value), Addr::Reg(to)) => {
                // 83 /5 ib when the immediate fits a sign-extended byte, else 81 /5 id
                let value = i32::try_from(*value).map_err(|err| err.to_string())?;
                if let Ok(byte) = i8::try_from(value) {
                    bytes.extend([rex_w(0, *to), 0x83, modrm(3, 5, *to)]);
                    bytes.extend(byte.to_le_bytes());
                } else {
                    bytes.extend([rex_w(0, *to), 0x81, modrm(3, 5, *to)]);
                    bytes.extend(value.to_le_bytes());
                }
            }
            ((PUSHQ, Class::Reg, None, Class::None), Addr::Reg(from), _) => {
                bytes.extend(push_pop(0x50, *from));
            }
            ((POPQ, Class::None, None, Class::Reg), _, Addr::Reg(to)) => {
                bytes.extend(push_pop(0x58, *to));
            }
            ((CALL, Class::None, None, Class::Sym), _, Addr::Sym { name, off }) => {
                // E8 cd: rel32 from the end of the instruction
                bytes.push(0xe8);
                relocs.push(Reloc::Pcrel32 {
                    at: bytes.len(),
                    sym: name.clone(),
                    off: *off,
                });
                bytes.extend([0; 4]);
            }
            ((RET, Class::None, None, Class::None), _, _) => bytes.push(0xc3),
            ((SYSCALL, Class::None, None, Class::None), _, _) => bytes.extend([0x0f, 0x05]),
            _ => {
                return Err(format!(
                    "internal error: amd64 has no encoding for `{}` in {name}",
                    dump(std::slice::from_ref(prog)).trim()
                ));
            }
        }
    }
    Ok(Encoded {
        name: name.clone(),
        bytes,
        relocs,
    })
}

fn mov(from: Addr, to: Addr) -> Prog {
    Prog::new(MOVQ, from, to)
}

fn sp(off: usize) -> Addr {
    Addr::Mem {
        base: SP,
        off: i32::try_from(off).unwrap_or(i32::MAX),
    }
}

/// Instruction selection, docs/build.md's v1 rule: every vreg lives in a stack slot, and each
/// `Inst` loads its operands into `AX`/`CX`, computes, and stores the result.
///
/// Frame (Lingua ABI): parameter `i` at `16+8i(BP)`, the result slot after the parameters,
/// vreg `k >= params` at `-8(k-params+1)(BP)`, and the outgoing argument area at `0(SP)`.
#[must_use]
pub fn select(function: &LirFn) -> Vec<Prog> {
    let params = function.params;
    let slot = |v: VReg| {
        let off = if v < params {
            16 + 8 * i64::from(v)
        } else {
            -8 * (i64::from(v - params) + 1)
        };
        Addr::Mem {
            base: BP,
            off: i32::try_from(off).unwrap_or(i32::MIN),
        }
    };
    let insts = || function.blocks.iter().flat_map(|block| &block.insts);
    let max_args = insts()
        .filter_map(|inst| match inst {
            Inst::Call(_, _, args) => Some(args.len()),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    let locals = usize::try_from(function.vregs - params).unwrap_or(0);
    let frame = (8 * locals + 8 * (max_args + 1)).next_multiple_of(16);

    let mut out = vec![
        Prog::new(TEXT, Addr::sym(&function.name), Addr::None),
        Prog::new(PUSHQ, Addr::Reg(BP), Addr::None),
        mov(Addr::Reg(SP), Addr::Reg(BP)),
        Prog::new(
            SUBQ,
            Addr::Imm(i64::try_from(frame).unwrap_or(i64::MAX)),
            Addr::Reg(SP),
        ),
    ];
    for block in &function.blocks {
        for inst in &block.insts {
            match inst {
                Inst::Const(v, n) => {
                    out.extend([
                        mov(Addr::Imm(*n), Addr::Reg(AX)),
                        mov(Addr::Reg(AX), slot(*v)),
                    ]);
                }
                Inst::Addr(v, sym) => out.extend([
                    Prog::new(LEAQ, Addr::sym(sym), Addr::Reg(AX)),
                    mov(Addr::Reg(AX), slot(*v)),
                ]),
                Inst::Load(v, a, off) => out.extend([
                    mov(slot(*a), Addr::Reg(AX)),
                    mov(
                        Addr::Mem {
                            base: AX,
                            off: *off,
                        },
                        Addr::Reg(AX),
                    ),
                    mov(Addr::Reg(AX), slot(*v)),
                ]),
                Inst::Bin(BinOp::Add, v, a, b) => out.extend([
                    mov(slot(*a), Addr::Reg(AX)),
                    mov(slot(*b), Addr::Reg(CX)),
                    Prog::new(ADDQ, Addr::Reg(CX), Addr::Reg(AX)),
                    mov(Addr::Reg(AX), slot(*v)),
                ]),
                Inst::Call(result, callee, args) => {
                    for (i, arg) in args.iter().enumerate() {
                        out.extend([
                            mov(slot(*arg), Addr::Reg(AX)),
                            mov(Addr::Reg(AX), sp(8 * i)),
                        ]);
                    }
                    out.push(Prog::new(CALL, Addr::None, Addr::sym(callee)));
                    if let Some(v) = result {
                        out.extend([
                            mov(sp(8 * args.len()), Addr::Reg(AX)),
                            mov(Addr::Reg(AX), slot(*v)),
                        ]);
                    }
                }
            }
        }
        let Term::Ret(value) = block.term;
        if let Some(v) = value {
            let result = Addr::Mem {
                base: BP,
                off: i32::try_from(16 + 8 * params).unwrap_or(i32::MAX),
            };
            out.extend([mov(slot(v), Addr::Reg(AX)), mov(Addr::Reg(AX), result)]);
        }
        out.extend([
            mov(Addr::Reg(BP), Addr::Reg(SP)),
            Prog::new(POPQ, Addr::None, Addr::Reg(BP)),
            Prog::new(RET, Addr::None, Addr::None),
        ]);
    }
    out
}

#[cfg(test)]
#[path = "amd64_tests.rs"]
mod tests;
