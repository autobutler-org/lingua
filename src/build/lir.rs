//! LIR, the target-independent lowered IR (docs/build.md, LIR): types, lowering from TIR,
//! and the `lingua lir` dump. Only the instructions the current subset needs exist so far.

use std::fmt::Write as _;

use super::prog::SymName;
use crate::tir::{TirExpr, TirFn, TirKind, TirStmt};

/// A virtual register; `v0 .. v(params-1)` are the parameters.
pub type VReg = u32;

/// A read-only data symbol.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoData {
    pub name: SymName,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct LirProgram {
    pub functions: Vec<LirFn>,
    pub rodata: Vec<RoData>,
}

// ponytail: no `stack_site` yet, so no prologue bound check and no L077; add it with fault
// sites, when a native stack overflow must report instead of segfaulting.
#[derive(Debug, PartialEq)]
pub struct LirFn {
    pub name: SymName,
    pub params: u32,
    /// Total, parameters included.
    pub vregs: u32,
    /// `blocks[0]` is the entry.
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub insts: Vec<Inst>,
    pub term: Term,
}

#[derive(Debug, PartialEq)]
pub enum Inst {
    Const(VReg, i64),
    /// The address of a function or rodata symbol.
    Addr(VReg, SymName),
    /// 8-byte load from `a + off`.
    Load(VReg, VReg, i32),
    /// Unchecked; runtime only.
    Bin(BinOp, VReg, VReg, VReg),
    Call(Option<VReg>, SymName, Vec<VReg>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Ret(Option<VReg>),
}

/// A Text block: `+0 len: i64`, `+8 bytes`, zero-padded to a multiple of 8.
#[must_use]
pub fn text_block(name: &str, text: &str) -> RoData {
    let mut bytes = (text.len() as u64).to_le_bytes().to_vec();
    bytes.extend(text.as_bytes());
    bytes.resize(bytes.len().next_multiple_of(8), 0);
    RoData {
        name: name.to_owned(),
        bytes,
    }
}

// Per-program state: interned Text blocks, `str.<n>` in first-use order
#[derive(Default)]
struct Lowerer {
    strings: Vec<String>,
}

// Per-function state
struct FnLowerer<'a> {
    program: &'a mut Lowerer,
    vregs: VReg,
    insts: Vec<Inst>,
}

impl FnLowerer<'_> {
    fn fresh(&mut self) -> VReg {
        self.vregs += 1;
        self.vregs - 1
    }

    fn intern(&mut self, text: &str) -> SymName {
        let strings = &mut self.program.strings;
        let index = strings.iter().position(|s| s == text).unwrap_or_else(|| {
            strings.push(text.to_owned());
            strings.len() - 1
        });
        format!("str.{index}")
    }

    fn expr(&mut self, expr: &TirExpr) -> VReg {
        let inst = match &expr.kind {
            TirKind::Nothing => Inst::Const(self.fresh(), 0),
            TirKind::Text(text) => {
                let sym = self.intern(text);
                Inst::Addr(self.fresh(), sym)
            }
            TirKind::Log(arg) => {
                let arg = self.expr(arg);
                Inst::Call(Some(self.fresh()), "rt.log".to_owned(), vec![arg])
            }
        };
        self.insts.push(inst);
        self.vregs - 1
    }
}

fn lower_fn(program: &mut Lowerer, function: &TirFn) -> LirFn {
    let mut lowerer = FnLowerer {
        program,
        vregs: 0,
        insts: Vec::new(),
    };
    let mut term = None;
    for stmt in &function.body {
        match stmt {
            TirStmt::Expr(expr) => {
                lowerer.expr(expr);
            }
            TirStmt::Return(expr) => {
                term = Some(Term::Ret(Some(lowerer.expr(expr))));
                break;
            }
        }
    }
    LirFn {
        name: function.name.clone(),
        params: 0,
        vregs: lowerer.vregs,
        blocks: vec![Block {
            insts: lowerer.insts,
            // The checker guarantees a `return` on every path (L012)
            term: term.unwrap_or(Term::Ret(None)),
        }],
    }
}

/// Lowers the runtime residue. Text literals are interned once per program.
#[must_use]
pub fn lower(functions: &[TirFn]) -> LirProgram {
    let mut program = Lowerer::default();
    let functions = functions
        .iter()
        .map(|function| lower_fn(&mut program, function))
        .collect();
    let rodata = program
        .strings
        .iter()
        .enumerate()
        .map(|(index, text)| text_block(&format!("str.{index}"), text))
        .collect();
    LirProgram { functions, rodata }
}

fn args(vregs: &[VReg]) -> String {
    vregs
        .iter()
        .map(|v| format!("v{v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The `lingua lir` dump: one function per paragraph.
#[must_use]
pub fn dump(functions: &[LirFn]) -> String {
    let mut out = String::new();
    for (index, function) in functions.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        let _ = writeln!(
            out,
            "fn {} params={} vregs={}",
            function.name, function.params, function.vregs
        );
        for (id, block) in function.blocks.iter().enumerate() {
            let _ = writeln!(out, "b{id}:");
            for inst in &block.insts {
                let _ = match inst {
                    Inst::Const(v, n) => writeln!(out, "  v{v} = const {n}"),
                    Inst::Addr(v, sym) => writeln!(out, "  v{v} = addr {sym}"),
                    Inst::Load(v, a, off) => writeln!(out, "  v{v} = load v{a}, {off}"),
                    Inst::Bin(BinOp::Add, v, a, b) => writeln!(out, "  v{v} = add v{a}, v{b}"),
                    Inst::Call(Some(v), f, a) => writeln!(out, "  v{v} = call {f}({})", args(a)),
                    Inst::Call(None, f, a) => writeln!(out, "  call {f}({})", args(a)),
                };
            }
            let _ = match block.term {
                Term::Ret(Some(v)) => writeln!(out, "  ret v{v}"),
                Term::Ret(None) => writeln!(out, "  ret"),
            };
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::lex::Location;

    const AT: Span = Span {
        start: Location { line: 1, column: 1 },
        end: Location { line: 1, column: 1 },
    };

    fn expr(kind: TirKind) -> TirExpr {
        TirExpr { span: AT, kind }
    }

    fn log(text: &str) -> TirStmt {
        TirStmt::Expr(expr(TirKind::Log(Box::new(expr(TirKind::Text(
            text.to_owned(),
        ))))))
    }

    #[test]
    fn log_lowers_to_a_call_of_rt_log_with_an_interned_text_block() {
        let main = TirFn {
            name: "hello.main".to_owned(),
            body: vec![
                log("hi"),
                log("bye"),
                log("hi"),
                TirStmt::Return(expr(TirKind::Nothing)),
            ],
        };
        let program = lower(&[main]);
        assert_eq!(
            dump(&program.functions),
            "fn hello.main params=0 vregs=7
b0:
  v0 = addr str.0
  v1 = call rt.log(v0)
  v2 = addr str.1
  v3 = call rt.log(v2)
  v4 = addr str.0
  v5 = call rt.log(v4)
  v6 = const 0
  ret v6
"
        );
        assert_eq!(
            program.rodata,
            [text_block("str.0", "hi"), text_block("str.1", "bye")]
        );
    }

    #[test]
    fn a_text_block_is_length_then_bytes_padded_to_eight() {
        assert_eq!(
            text_block("s", "hi").bytes,
            [2, 0, 0, 0, 0, 0, 0, 0, b'h', b'i', 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(text_block("s", "").bytes, [0; 8]);
        assert_eq!(text_block("s", "12345678").bytes.len(), 16);
    }

    #[test]
    fn dump_separates_functions_and_prints_every_form() {
        let function = |name: &str, insts, term| LirFn {
            name: name.to_owned(),
            params: 1,
            vregs: 4,
            blocks: vec![Block { insts, term }],
        };
        let functions = [
            function(
                "rt.f",
                vec![
                    Inst::Const(1, -8),
                    Inst::Load(2, 0, 8),
                    Inst::Bin(BinOp::Add, 3, 1, 2),
                    Inst::Call(None, "rt.g".to_owned(), vec![0, 3]),
                ],
                Term::Ret(Some(3)),
            ),
            function("rt.h", vec![], Term::Ret(None)),
        ];
        assert_eq!(
            dump(&functions),
            "fn rt.f params=1 vregs=4
b0:
  v1 = const -8
  v2 = load v0, 8
  v3 = add v1, v2
  call rt.g(v0, v3)
  ret v3

fn rt.h params=1 vregs=4
b0:
  ret
"
        );
    }
}
