//! amd64 instruction selection and encoding table, after Go's `optab`: each row is keyed by
//! opcode and operand classes. `CMPQ a, b` sets the flags from `a - b`, as in Go.

use super::{Encoded, Reloc};
use crate::build::lir::{BinOp, ChkOp, CmpOp, Inst, LirFn, SiteId, Term, VReg, site_symbol};
use crate::build::prog::{self, Addr, As, CALL, JMP, LABEL, LabelId, Prog, RET, Reg, TEXT};

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

macro_rules! opcodes {
    ($($name:ident),* $(,)?) => {
        opcodes!(@each 0x100u16; $($name,)*);
        const OP_NAMES: &[&str] = &[$(stringify!($name)),*];
    };
    (@each $n:expr; $name:ident, $($rest:ident,)*) => {
        pub const $name: As = As($n);
        opcodes!(@each $n + 1; $($rest,)*);
    };
    (@each $n:expr;) => {};
}

opcodes!(
    MOVQ, LEAQ, SYSCALL, ADDQ, SUBQ, PUSHQ, POPQ, IMULQ, ANDQ, ORQ, SHLQ, SHRQ, NEGQ, CQO, IDIVQ,
    DIVQ, CMPQ, TESTQ, SETEQ, SETNE, SETLT, SETLE, SETGT, SETGE, SETCS, MOVBQZX, MOVB, JEQ, JNE,
    JOS, JCS,
);

const REX_W: u8 = 0x48;
const INT_MIN: i64 = i64::MIN;

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
    Label,
}

fn class(addr: &Addr) -> Class {
    match addr {
        Addr::None => Class::None,
        Addr::Reg(_) => Class::Reg,
        Addr::Imm(value) if i32::try_from(*value).is_ok() => Class::Imm32,
        Addr::Imm(_) => Class::Imm64,
        Addr::Mem { .. } => Class::Mem,
        Addr::Sym { .. } => Class::Sym,
        Addr::Label(_) => Class::Label,
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

// A byte register in ModRM.reg needs a REX prefix for SPL..DIL (4-7) and R8B..R15B, and so
// does a memory base of R8-R15
fn rex_byte(reg: Reg, base: Reg) -> Option<u8> {
    (reg >= SP || base >= R8).then_some(0x40 | ((reg >> 3) << 2) | (base >> 3))
}

// `/digit` group-1 ALU op with an immediate: 83 /digit ib, or 81 /digit id
fn alu_imm(bytes: &mut Vec<u8>, digit: u8, value: i32, to: Reg) {
    if let Ok(byte) = i8::try_from(value) {
        bytes.extend([rex_w(0, to), 0x83, modrm(3, digit, to)]);
        bytes.extend(byte.to_le_bytes());
    } else {
        bytes.extend([rex_w(0, to), 0x81, modrm(3, digit, to)]);
        bytes.extend(value.to_le_bytes());
    }
}

fn pcrel(bytes: &mut Vec<u8>, relocs: &mut Vec<Reloc>, name: &str, off: i32) {
    relocs.push(Reloc::Pcrel32 {
        at: bytes.len(),
        sym: name.to_owned(),
        off,
    });
    bytes.extend([0; 4]);
}

// The condition code of each Jcc and SETcc, the low nibble of 0F 8x / 0F 9x
fn condition(op: As) -> Option<u8> {
    Some(match op {
        JOS => 0x0,
        JCS | SETCS => 0x2,
        JEQ | SETEQ => 0x4,
        JNE | SETNE => 0x5,
        SETLT => 0xc,
        SETGE => 0xd,
        SETLE => 0xe,
        SETGT => 0xf,
        _ => return None,
    })
}

/// Encodes one function: a `TEXT` followed by its body. Branches to labels always use the
/// 32-bit displacement form and are patched once every label's offset is known.
///
/// # Errors
/// An internal error naming the `Prog` when the list does not start with `TEXT`, an operand
/// combination has no table row, or a label is undefined.
#[allow(clippy::too_many_lines)]
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
    let mut labels: Vec<(LabelId, usize)> = Vec::new();
    let mut fixups: Vec<(usize, LabelId)> = Vec::new();
    for prog in body {
        let row = (prog.as_, class(&prog.from), prog.reg, class(&prog.to));
        match (row, &prog.from, &prog.to) {
            ((LABEL, Class::Label, None, Class::None), Addr::Label(label), _) => {
                labels.push((*label, bytes.len()));
            }
            ((JMP, Class::None, None, Class::Label), _, Addr::Label(label)) => {
                // E9 cd
                bytes.push(0xe9);
                fixups.push((bytes.len(), *label));
                bytes.extend([0; 4]);
            }
            ((op, Class::None, None, Class::Label), _, Addr::Label(label))
                if condition(op).is_some() && op.0 >= JEQ.0 =>
            {
                // 0F 80+cc cd
                bytes.extend([0x0f, 0x80 | condition(op).unwrap_or(0)]);
                fixups.push((bytes.len(), *label));
                bytes.extend([0; 4]);
            }
            ((JMP, Class::None, None, Class::Sym), _, Addr::Sym { name, off }) => {
                bytes.push(0xe9);
                pcrel(&mut bytes, &mut relocs, name, *off);
            }
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
                pcrel(&mut bytes, &mut relocs, name, *off);
            }
            ((LEAQ, Class::Mem, None, Class::Reg), Addr::Mem { base, off }, Addr::Reg(to)) => {
                // 8D /r
                bytes.extend([rex_w(*to, *base), 0x8d]);
                bytes.extend(mem(*to, *base, *off));
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
            ((MOVBQZX, Class::Mem, None, Class::Reg), Addr::Mem { base, off }, Addr::Reg(to)) => {
                // REX.W 0F B6 /r: MOVZX r64, r/m8
                bytes.extend([rex_w(*to, *base), 0x0f, 0xb6]);
                bytes.extend(mem(*to, *base, *off));
            }
            ((MOVBQZX, Class::Reg, None, Class::Reg), Addr::Reg(from), Addr::Reg(to)) => {
                bytes.extend([rex_w(*to, *from), 0x0f, 0xb6, modrm(3, *to, *from)]);
            }
            ((MOVB, Class::Reg, None, Class::Mem), Addr::Reg(from), Addr::Mem { base, off }) => {
                // 88 /r: MOV r/m8, r8
                bytes.extend(rex_byte(*from, *base));
                bytes.push(0x88);
                bytes.extend(mem(*from, *base, *off));
            }
            ((op, Class::Reg, None, Class::Reg), Addr::Reg(from), Addr::Reg(to))
                if matches!(op, ADDQ | SUBQ | ANDQ | ORQ | TESTQ) =>
            {
                // 01 / 29 / 21 / 09 / 85 /r: OP r/m64, r64
                let opcode = match op {
                    ADDQ => 0x01,
                    SUBQ => 0x29,
                    ANDQ => 0x21,
                    ORQ => 0x09,
                    _ => 0x85,
                };
                bytes.extend([rex_w(*from, *to), opcode, modrm(3, *from, *to)]);
            }
            ((CMPQ, Class::Reg, None, Class::Reg), Addr::Reg(a), Addr::Reg(b)) => {
                // 39 /r: CMP r/m64 (a), r64 (b), flags from a - b
                bytes.extend([rex_w(*b, *a), 0x39, modrm(3, *b, *a)]);
            }
            ((CMPQ, Class::Reg, None, Class::Imm32), Addr::Reg(a), Addr::Imm(value)) => {
                let value = i32::try_from(*value).map_err(|err| err.to_string())?;
                alu_imm(&mut bytes, 7, value, *a);
            }
            ((CMPQ, Class::Reg, None, Class::Sym), Addr::Reg(a), Addr::Sym { name, off }) => {
                // 3B /r: CMP r64 (a), [RIP + disp32]
                bytes.extend([rex_w(*a, 0), 0x3b, modrm(0, *a, 5)]);
                pcrel(&mut bytes, &mut relocs, name, *off);
            }
            ((op, Class::Imm32, None, Class::Reg), Addr::Imm(value), Addr::Reg(to))
                if matches!(op, ADDQ | SUBQ | ANDQ) =>
            {
                let value = i32::try_from(*value).map_err(|err| err.to_string())?;
                let digit = match op {
                    ADDQ => 0,
                    SUBQ => 5,
                    _ => 4,
                };
                alu_imm(&mut bytes, digit, value, *to);
            }
            ((IMULQ, Class::Reg, None, Class::Reg), Addr::Reg(from), Addr::Reg(to)) => {
                // 0F AF /r: IMUL r64 (to), r/m64 (from)
                bytes.extend([rex_w(*to, *from), 0x0f, 0xaf, modrm(3, *to, *from)]);
            }
            ((SHLQ | SHRQ, Class::Reg, None, Class::Reg), Addr::Reg(CX), Addr::Reg(to)) => {
                // D3 /4 or /5: shift r/m64 by CL
                let digit = if prog.as_ == SHLQ { 4 } else { 5 };
                bytes.extend([rex_w(0, *to), 0xd3, modrm(3, digit, *to)]);
            }
            ((NEGQ, Class::None, None, Class::Reg), _, Addr::Reg(to)) => {
                // F7 /3
                bytes.extend([rex_w(0, *to), 0xf7, modrm(3, 3, *to)]);
            }
            ((IDIVQ | DIVQ, Class::Reg, None, Class::None), Addr::Reg(from), _) => {
                // F7 /7 or /6: DX:AX by r/m64
                let digit = if prog.as_ == IDIVQ { 7 } else { 6 };
                bytes.extend([rex_w(0, *from), 0xf7, modrm(3, digit, *from)]);
            }
            ((CQO, Class::None, None, Class::None), _, _) => bytes.extend([REX_W, 0x99]),
            ((op, Class::None, None, Class::Reg), _, Addr::Reg(to))
                if condition(op).is_some() && op.0 < JEQ.0 =>
            {
                // 0F 90+cc /0: SETcc r/m8; SPL..DIL and R8B.. need a REX prefix
                bytes.extend((*to >= SP).then_some(0x40 | (to >> 3)));
                bytes.extend([0x0f, 0x90 | condition(op).unwrap_or(0), modrm(3, 0, *to)]);
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
                pcrel(&mut bytes, &mut relocs, name, *off);
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
    for (at, label) in fixups {
        let target = labels
            .iter()
            .find(|(id, _)| *id == label)
            .map(|(_, offset)| *offset)
            .ok_or_else(|| format!("internal error: undefined label L{label} in {name}"))?;
        let disp = i64::try_from(target).unwrap_or(0) - i64::try_from(at + 4).unwrap_or(0);
        let disp =
            i32::try_from(disp).map_err(|_| format!("internal error: {name} is too large"))?;
        bytes[at..at + 4].copy_from_slice(&disp.to_le_bytes());
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

fn reg(r: Reg) -> Addr {
    Addr::Reg(r)
}

fn mem_at(base: Reg, off: i64) -> Addr {
    Addr::Mem {
        base,
        off: i32::try_from(off).unwrap_or(i32::MIN),
    }
}

// The offset of word `i`
fn word(i: usize) -> i64 {
    i64::try_from(8 * i).unwrap_or(i64::MAX)
}

fn op(as_: As, from: Addr, to: Addr) -> Prog {
    Prog::new(as_, from, to)
}

fn jump(as_: As, label: LabelId) -> Prog {
    Prog::new(as_, Addr::None, Addr::Label(label))
}

// Selection state for one function
struct Selector {
    params: u32,
    out: Vec<Prog>,
    next_label: LabelId,
    /// Out-of-line stubs: label, then the report symbol to raise.
    stubs: Vec<(LabelId, String)>,
}

impl Selector {
    fn slot(&self, v: VReg) -> Addr {
        if v < self.params {
            mem_at(BP, 16 + 8 * i64::from(v))
        } else {
            mem_at(BP, -8 * (i64::from(v - self.params) + 1))
        }
    }

    fn load(&mut self, v: VReg, r: Reg) {
        let slot = self.slot(v);
        self.out.push(mov(slot, reg(r)));
    }

    fn store(&mut self, r: Reg, v: VReg) {
        let slot = self.slot(v);
        self.out.push(mov(reg(r), slot));
    }

    fn label(&mut self) -> LabelId {
        self.next_label += 1;
        self.next_label - 1
    }

    // A label whose stub raises `symbol`
    fn stub(&mut self, symbol: String) -> LabelId {
        let label = self.label();
        self.stubs.push((label, symbol));
        label
    }

    fn raise(&mut self, symbol: &str) {
        self.out.extend([
            op(LEAQ, Addr::sym(symbol), reg(AX)),
            op(JMP, Addr::None, Addr::sym("rt.fault")),
        ]);
    }

    fn checked(&mut self, chk: ChkOp, v: VReg, a: VReg, b: VReg, site: SiteId) {
        self.load(a, AX);
        self.load(b, CX);
        match chk {
            ChkOp::Add | ChkOp::Sub | ChkOp::Mul => {
                let as_ = match chk {
                    ChkOp::Add => ADDQ,
                    ChkOp::Sub => SUBQ,
                    _ => IMULQ,
                };
                let overflow = self.stub(site_symbol(site, 0));
                self.out
                    .extend([op(as_, reg(CX), reg(AX)), jump(JOS, overflow)]);
                self.store(AX, v);
            }
            ChkOp::Div | ChkOp::Rem => {
                let zero = self.stub(site_symbol(site, 0));
                let overflow = self.stub(site_symbol(site, 1));
                let ok = self.label();
                self.out.extend([
                    op(TESTQ, reg(CX), reg(CX)),
                    jump(JEQ, zero),
                    op(CMPQ, reg(CX), Addr::Imm(-1)),
                    jump(JNE, ok),
                    mov(Addr::Imm(INT_MIN), reg(DX)),
                    op(CMPQ, reg(AX), reg(DX)),
                    jump(JEQ, overflow),
                    op(LABEL, Addr::Label(ok), Addr::None),
                    op(CQO, Addr::None, Addr::None),
                    op(IDIVQ, reg(CX), Addr::None),
                ]);
                self.store(if chk == ChkOp::Div { AX } else { DX }, v);
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn inst(&mut self, inst: &Inst) {
        match inst {
            Inst::Const(v, n) => {
                self.out.push(mov(Addr::Imm(*n), reg(AX)));
                self.store(AX, *v);
            }
            Inst::Addr(v, sym) => {
                self.out.push(op(LEAQ, Addr::sym(sym), reg(AX)));
                self.store(AX, *v);
            }
            Inst::Move(v, a) => {
                self.load(*a, AX);
                self.store(AX, *v);
            }
            Inst::Load(v, a, off) => {
                self.load(*a, AX);
                self.out.push(mov(mem_at(AX, (*off).into()), reg(AX)));
                self.store(AX, *v);
            }
            Inst::Load8(v, a, off) => {
                self.load(*a, AX);
                self.out
                    .push(op(MOVBQZX, mem_at(AX, (*off).into()), reg(AX)));
                self.store(AX, *v);
            }
            Inst::Store(a, off, b) | Inst::Store8(a, off, b) => {
                self.load(*a, AX);
                self.load(*b, CX);
                let as_ = if matches!(inst, Inst::Store(..)) {
                    MOVQ
                } else {
                    MOVB
                };
                self.out.push(op(as_, reg(CX), mem_at(AX, (*off).into())));
            }
            Inst::Bin(bin, v, a, b) => {
                self.load(*a, AX);
                self.load(*b, CX);
                let result = match bin {
                    BinOp::UDiv | BinOp::URem => {
                        self.out
                            .extend([mov(Addr::Imm(0), reg(DX)), op(DIVQ, reg(CX), Addr::None)]);
                        if *bin == BinOp::UDiv { AX } else { DX }
                    }
                    _ => {
                        let as_ = match bin {
                            BinOp::Add => ADDQ,
                            BinOp::Sub => SUBQ,
                            BinOp::Mul => IMULQ,
                            BinOp::And => ANDQ,
                            BinOp::Or => ORQ,
                            BinOp::Shl => SHLQ,
                            _ => SHRQ,
                        };
                        self.out.push(op(as_, reg(CX), reg(AX)));
                        AX
                    }
                };
                self.store(result, *v);
            }
            Inst::Chk(chk, v, a, b, site) => self.checked(*chk, *v, *a, *b, *site),
            Inst::Neg(v, a, site) => {
                self.load(*a, AX);
                let overflow = self.stub(site_symbol(*site, 0));
                self.out
                    .extend([op(NEGQ, Addr::None, reg(AX)), jump(JOS, overflow)]);
                self.store(AX, *v);
            }
            Inst::Cmp(cmp, v, a, b) => {
                self.load(*a, AX);
                self.load(*b, CX);
                let set = match cmp {
                    CmpOp::Eq => SETEQ,
                    CmpOp::Ne => SETNE,
                    CmpOp::Lt => SETLT,
                    CmpOp::Le => SETLE,
                    CmpOp::Gt => SETGT,
                    CmpOp::Ge => SETGE,
                    CmpOp::ULt => SETCS,
                };
                self.out.extend([
                    op(CMPQ, reg(AX), reg(CX)),
                    op(set, Addr::None, reg(AX)),
                    op(MOVBQZX, reg(AX), reg(AX)),
                ]);
                self.store(AX, *v);
            }
            Inst::Call(result, callee, args) => {
                for (i, arg) in args.iter().enumerate() {
                    self.load(*arg, AX);
                    self.out.push(mov(reg(AX), mem_at(SP, word(i))));
                }
                self.out.push(op(CALL, Addr::None, Addr::sym(callee)));
                if let Some(v) = result {
                    self.out.push(mov(mem_at(SP, word(args.len())), reg(AX)));
                    self.store(AX, *v);
                }
            }
        }
    }

    fn term(&mut self, term: &Term) {
        match term {
            Term::Jump(to) => self.out.push(jump(JMP, *to)),
            Term::Branch(cond, then, otherwise) => {
                self.load(*cond, AX);
                self.out.extend([
                    op(TESTQ, reg(AX), reg(AX)),
                    jump(JNE, *then),
                    jump(JMP, *otherwise),
                ]);
            }
            Term::Ret(value) => {
                if let Some(v) = value {
                    self.load(*v, AX);
                    let result = mem_at(BP, 16 + 8 * i64::from(self.params));
                    self.out.push(mov(reg(AX), result));
                }
                self.out.extend([
                    mov(reg(BP), reg(SP)),
                    op(POPQ, Addr::None, reg(BP)),
                    op(RET, Addr::None, Addr::None),
                ]);
            }
            Term::Raise(site) => self.raise(&site_symbol(*site, 0)),
            Term::TailCall(callee) => self.out.extend([
                mov(reg(BP), reg(SP)),
                op(POPQ, Addr::None, reg(BP)),
                op(JMP, Addr::None, Addr::sym(callee)),
            ]),
        }
    }
}

/// Instruction selection, docs/build.md's v1 rule: every vreg lives in a stack slot, and each
/// `Inst` loads its operands into `AX`/`CX`, computes, and stores the result.
///
/// Frame (Lingua ABI): parameter `i` at `16+8i(BP)`, the result slot after the parameters,
/// vreg `k >= params` at `-8(k-params+1)(BP)`, and the outgoing argument area at `0(SP)`.
/// Block `i` is label `Li`; `L0` is only emitted when a self tail call jumps to it. A tail
/// call of another function restores the caller's `BP` and `SP` and jumps to its entry. A function
/// with a stack site first checks that its frame fits above `rt.stack_limit`; checks and the
/// stack check branch to out-of-line stubs.
#[must_use]
pub fn select(function: &LirFn) -> Vec<Prog> {
    let params = function.params;
    let max_args = function
        .blocks
        .iter()
        .flat_map(|block| &block.insts)
        .filter_map(|inst| match inst {
            Inst::Call(_, _, args) => Some(args.len()),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    let locals = usize::try_from(function.vregs - params).unwrap_or(0);
    let frame =
        i64::try_from((8 * locals + 8 * (max_args + 1)).next_multiple_of(16)).unwrap_or(i64::MAX);
    let mut s = Selector {
        params,
        out: vec![Prog::new(TEXT, Addr::sym(&function.name), Addr::None)],
        next_label: LabelId::try_from(function.blocks.len()).unwrap_or(LabelId::MAX),
        stubs: Vec::new(),
    };
    if let Some(site) = function.stack_site {
        let fault = s.stub(site_symbol(site, 0));
        s.out.extend([
            op(LEAQ, mem_at(SP, -(frame + 16)), reg(R11)),
            op(CMPQ, reg(R11), Addr::sym("rt.stack_limit")),
            jump(JCS, fault),
        ]);
    }
    s.out.extend([
        op(PUSHQ, reg(BP), Addr::None),
        mov(reg(SP), reg(BP)),
        op(SUBQ, Addr::Imm(frame), reg(SP)),
    ]);
    // A self tail call jumps back to the entry block, after the prologue
    let entry_label = function.blocks.iter().any(|block| match &block.term {
        Term::Jump(to) => *to == 0,
        Term::Branch(_, then, otherwise) => *then == 0 || *otherwise == 0,
        Term::Ret(_) | Term::Raise(_) | Term::TailCall(_) => false,
    });
    for (id, block) in function.blocks.iter().enumerate() {
        if id > 0 || entry_label {
            let label = LabelId::try_from(id).unwrap_or(LabelId::MAX);
            s.out.push(op(LABEL, Addr::Label(label), Addr::None));
        }
        for inst in &block.insts {
            s.inst(inst);
        }
        s.term(&block.term);
    }
    for (label, symbol) in std::mem::take(&mut s.stubs) {
        s.out.push(op(LABEL, Addr::Label(label), Addr::None));
        s.raise(&symbol);
    }
    s.out
}

#[cfg(test)]
#[path = "amd64_tests.rs"]
mod tests;
