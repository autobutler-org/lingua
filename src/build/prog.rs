//! `Prog`, the generic instruction form after Go's `obj.Prog`, and its dump.
//!
//! Selection emits a `Vec<Prog>` per function and encoding is the only consumer. The
//! dump syntax is Go's: sources first, destination last, `(SB)` for symbols.

/// A register, numbered by the architecture's table.
pub type Reg = u8;

/// The name of a function or read-only data symbol.
pub type SymName = String;

/// An opcode: the generic range below, or a `u16` range owned by one architecture's table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct As(pub u16);

/// Opens a function; `from` is the function's `Sym`.
pub const TEXT: As = As(0);
pub const LABEL: As = As(1);
pub const JMP: As = As(2);
pub const CALL: As = As(3);
pub const RET: As = As(4);

/// The name of a generic opcode, or `None` for an architecture's own.
#[must_use]
pub fn generic_name(op: As) -> Option<&'static str> {
    ["TEXT", "LABEL", "JMP", "CALL", "RET"]
        .get(usize::from(op.0))
        .copied()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Addr {
    None,
    Reg(Reg),
    Imm(i64),
    /// PC-relative reference to `name + off`.
    Sym {
        name: SymName,
        off: i32,
    },
}

impl Addr {
    #[must_use]
    pub fn sym(name: &str) -> Addr {
        Addr::Sym {
            name: name.to_owned(),
            off: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prog {
    pub as_: As,
    pub from: Addr,
    /// Second source, for three-operand forms.
    pub reg: Option<Reg>,
    pub to: Addr,
}

impl Prog {
    /// A `Prog` with no second source.
    #[must_use]
    pub fn new(as_: As, from: Addr, to: Addr) -> Prog {
        Prog {
            as_,
            from,
            reg: None,
            to,
        }
    }

    /// One dump line with no trailing newline. `op_name` and `reg_name` come from the arch table.
    #[must_use]
    pub fn dump(
        &self,
        op_name: fn(As) -> &'static str,
        reg_name: fn(Reg) -> &'static str,
    ) -> String {
        let addr = |addr: &Addr| match addr {
            Addr::None => None,
            Addr::Reg(reg) => Some(reg_name(*reg).to_owned()),
            Addr::Imm(value) => Some(format!("${value}")),
            Addr::Sym { name, off: 0 } => Some(format!("{name}(SB)")),
            Addr::Sym { name, off } => Some(format!("{name}{off:+}(SB)")),
        };
        let operands: Vec<String> = [
            addr(&self.from),
            self.reg.map(|reg| reg_name(reg).to_owned()),
            addr(&self.to),
        ]
        .into_iter()
        .flatten()
        .collect();
        let op = op_name(self.as_);
        match (self.as_, operands.is_empty()) {
            (TEXT | LABEL, _) => format!("{op} {}", operands.join(", ")),
            (_, true) => format!("  {op}"),
            (_, false) => format!("  {op:<5} {}", operands.join(", ")),
        }
    }
}

/// The dump of a `Prog` list, one line per `Prog`.
#[must_use]
pub fn dump(
    progs: &[Prog],
    op_name: fn(As) -> &'static str,
    reg_name: fn(Reg) -> &'static str,
) -> String {
    progs
        .iter()
        .map(|prog| prog.dump(op_name, reg_name) + "\n")
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOVQ: As = As(0x100);

    fn op_name(op: As) -> &'static str {
        generic_name(op).unwrap_or("MOVQ")
    }

    fn reg_name(reg: Reg) -> &'static str {
        ["AX", "CX"][usize::from(reg)]
    }

    #[test]
    fn dump_puts_sources_first_and_symbols_in_sb() {
        let progs = [
            Prog::new(TEXT, Addr::sym("f"), Addr::None),
            Prog::new(MOVQ, Addr::Imm(-2), Addr::Reg(0)),
            Prog {
                as_: MOVQ,
                from: Addr::Reg(1),
                reg: Some(0),
                to: Addr::Reg(1),
            },
            Prog::new(
                MOVQ,
                Addr::Sym {
                    name: "msg".to_owned(),
                    off: 8,
                },
                Addr::Reg(1),
            ),
            Prog::new(
                MOVQ,
                Addr::Sym {
                    name: "msg".to_owned(),
                    off: -8,
                },
                Addr::Reg(1),
            ),
            Prog::new(RET, Addr::None, Addr::None),
        ];
        assert_eq!(
            dump(&progs, op_name, reg_name),
            "TEXT f(SB)\n  MOVQ  $-2, AX\n  MOVQ  CX, AX, CX\n  MOVQ  msg+8(SB), CX\n  MOVQ  msg-8(SB), CX\n  RET\n"
        );
    }
}
