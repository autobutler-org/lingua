//! amd64 encoding table, after Go's `optab`: each row is keyed by opcode and operand classes.

use super::{Encoded, Reloc};
use crate::build::prog::{self, Addr, As, Prog, Reg, TEXT};

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

const OP_NAMES: [&str; 3] = ["MOVQ", "LEAQ", "SYSCALL"];

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
    Sym,
}

fn class(addr: &Addr) -> Class {
    match addr {
        Addr::None => Class::None,
        Addr::Reg(_) => Class::Reg,
        Addr::Imm(value) if i32::try_from(*value).is_ok() => Class::Imm32,
        Addr::Imm(_) => Class::Imm64,
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

#[cfg(test)]
#[path = "amd64_tests.rs"]
mod tests;
