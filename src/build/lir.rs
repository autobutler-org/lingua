//! LIR, the target-independent lowered IR (docs/build.md, LIR): types, a builder shared by
//! TIR lowering (`build::lower`) and the runtime (`build::runtime`), and the `lingua lir` dump.

use std::fmt::Write as _;

use super::prog::SymName;

/// A virtual register; `v0 .. v(params-1)` are the parameters.
pub type VReg = u32;
/// An index into `LirFn.blocks`.
pub type BlockId = u32;
/// An index into `LirProgram.sites`.
pub type SiteId = u32;

/// A read-only data symbol.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoData {
    pub name: SymName,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct LirProgram {
    /// The user functions, in module order and then source order.
    pub functions: Vec<LirFn>,
    /// `rt.entry`: builds `Process` if `main` takes one, calls `main`, and exits 0.
    pub entry: LirFn,
    pub rodata: Vec<RoData>,
    pub sites: Vec<Site>,
}

/// A place where the binary can stop: a checked op, an `assert` or `fail`, a stack check, a
/// runtime `split`. Each report is pre-rendered; its rodata block is [`site_symbol`].
#[derive(Debug, PartialEq)]
pub struct Site {
    /// `path:line:column`, for the dump.
    pub at: String,
    /// One, or two for `Div`/`Rem` (`L076` then `L040`).
    pub reports: Vec<Report>,
}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub kind: ReportKind,
    /// Exactly what is written to stderr, trailing newline included.
    pub text: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReportKind {
    Exception,
    Fault(&'static str),
    /// Unreachable in a checked program, such as falling off the last match arm.
    Internal,
}

/// The rodata symbol of report `index` of `site`: `site.N`, then `site.N.1`.
#[must_use]
pub fn site_symbol(site: SiteId, index: usize) -> SymName {
    if index == 0 {
        format!("site.{site}")
    } else {
        format!("site.{site}.{index}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LirFn {
    pub name: SymName,
    pub params: u32,
    /// Total, parameters included.
    pub vregs: u32,
    /// `blocks[0]` is the entry.
    pub blocks: Vec<Block>,
    /// The `L077` site of the prologue's stack check; `None` for nosplit runtime functions.
    pub stack_site: Option<SiteId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub insts: Vec<Inst>,
    pub term: Term,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inst {
    Const(VReg, i64),
    /// The address of a function, rodata, or bss symbol.
    Addr(VReg, SymName),
    Move(VReg, VReg),
    /// 8-byte load from `a + off`.
    Load(VReg, VReg, i32),
    /// 8-byte store of `b` to `a + off`.
    Store(VReg, i32, VReg),
    /// Zero-extended byte load; runtime only.
    Load8(VReg, VReg, i32),
    /// Low-byte store; runtime only.
    Store8(VReg, i32, VReg),
    /// Unchecked; runtime only.
    Bin(BinOp, VReg, VReg, VReg),
    /// Checked Lingua arithmetic.
    Chk(ChkOp, VReg, VReg, VReg, SiteId),
    /// Checked unary minus.
    Neg(VReg, VReg, SiteId),
    /// `v = 1` or `0`.
    Cmp(CmpOp, VReg, VReg, VReg),
    Call(Option<VReg>, SymName, Vec<VReg>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Shl,
    Shr,
    UDiv,
    URem,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChkOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

/// Signed, except `ULt`.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    ULt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Jump(BlockId),
    /// To `then` when `a != 0`.
    Branch(VReg, BlockId, BlockId),
    Ret(Option<VReg>),
    /// Never returns.
    Raise(SiteId),
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

/// A zero-payload variant's singleton, `{tag, 0}`.
#[must_use]
pub fn singleton_block(name: &str, tag: u32) -> RoData {
    let mut bytes = u64::from(tag).to_le_bytes().to_vec();
    bytes.extend([0; 8]);
    RoData {
        name: name.to_owned(),
        bytes,
    }
}

/// Builds one `LirFn` block by block. Instructions go to the current block; once it has a
/// terminator, [`FnBuilder::is_open`] is false until another block is selected.
pub struct FnBuilder {
    name: SymName,
    params: u32,
    vregs: u32,
    blocks: Vec<(Vec<Inst>, Option<Term>)>,
    current: BlockId,
    stack_site: Option<SiteId>,
}

impl FnBuilder {
    #[must_use]
    pub fn new(name: &str, params: u32, stack_site: Option<SiteId>) -> FnBuilder {
        FnBuilder {
            name: name.to_owned(),
            params,
            vregs: params,
            blocks: vec![(Vec::new(), None)],
            current: 0,
            stack_site,
        }
    }

    pub fn fresh(&mut self) -> VReg {
        self.vregs += 1;
        self.vregs - 1
    }

    fn push(&mut self, inst: Inst) {
        self.blocks[self.current as usize].0.push(inst);
    }

    fn def(&mut self, inst: impl FnOnce(VReg) -> Inst) -> VReg {
        let v = self.fresh();
        self.push(inst(v));
        v
    }

    pub fn konst(&mut self, n: i64) -> VReg {
        self.def(|v| Inst::Const(v, n))
    }

    pub fn addr(&mut self, sym: &str) -> VReg {
        self.def(|v| Inst::Addr(v, sym.to_owned()))
    }

    pub fn mov(&mut self, dst: VReg, src: VReg) {
        self.push(Inst::Move(dst, src));
    }

    pub fn load(&mut self, a: VReg, off: i32) -> VReg {
        self.def(|v| Inst::Load(v, a, off))
    }

    pub fn load8(&mut self, a: VReg, off: i32) -> VReg {
        self.def(|v| Inst::Load8(v, a, off))
    }

    pub fn store(&mut self, a: VReg, off: i32, b: VReg) {
        self.push(Inst::Store(a, off, b));
    }

    pub fn store8(&mut self, a: VReg, off: i32, b: VReg) {
        self.push(Inst::Store8(a, off, b));
    }

    pub fn bin(&mut self, op: BinOp, a: VReg, b: VReg) -> VReg {
        self.def(|v| Inst::Bin(op, v, a, b))
    }

    /// `dst = a op b`, for a variable updated in place.
    pub fn bin_into(&mut self, dst: VReg, op: BinOp, a: VReg, b: VReg) {
        self.push(Inst::Bin(op, dst, a, b));
    }

    /// `dst = dst + n`.
    pub fn add_imm(&mut self, dst: VReg, n: i64) {
        let k = self.konst(n);
        self.bin_into(dst, BinOp::Add, dst, k);
    }

    pub fn chk(&mut self, op: ChkOp, a: VReg, b: VReg, site: SiteId) -> VReg {
        self.def(|v| Inst::Chk(op, v, a, b, site))
    }

    pub fn neg(&mut self, a: VReg, site: SiteId) -> VReg {
        self.def(|v| Inst::Neg(v, a, site))
    }

    pub fn cmp(&mut self, op: CmpOp, a: VReg, b: VReg) -> VReg {
        self.def(|v| Inst::Cmp(op, v, a, b))
    }

    /// `a op n` against a constant.
    pub fn cmp_imm(&mut self, op: CmpOp, a: VReg, n: i64) -> VReg {
        let k = self.konst(n);
        self.cmp(op, a, k)
    }

    pub fn call(&mut self, f: &str, args: &[VReg]) -> VReg {
        self.def(|v| Inst::Call(Some(v), f.to_owned(), args.to_vec()))
    }

    pub fn call_void(&mut self, f: &str, args: &[VReg]) {
        self.push(Inst::Call(None, f.to_owned(), args.to_vec()));
    }

    /// A new, empty block; the current block does not change.
    pub fn block(&mut self) -> BlockId {
        self.blocks.push((Vec::new(), None));
        BlockId::try_from(self.blocks.len() - 1).unwrap_or(BlockId::MAX)
    }

    pub fn switch(&mut self, block: BlockId) {
        self.current = block;
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.blocks[self.current as usize].1.is_none()
    }

    fn terminate(&mut self, term: Term) {
        let slot = &mut self.blocks[self.current as usize].1;
        if slot.is_none() {
            *slot = Some(term);
        }
    }

    pub fn jump(&mut self, to: BlockId) {
        self.terminate(Term::Jump(to));
    }

    pub fn branch(&mut self, cond: VReg, then: BlockId, otherwise: BlockId) {
        self.terminate(Term::Branch(cond, then, otherwise));
    }

    pub fn ret(&mut self, value: Option<VReg>) {
        self.terminate(Term::Ret(value));
    }

    pub fn raise(&mut self, site: SiteId) {
        self.terminate(Term::Raise(site));
    }

    /// Runs `then` when `cond != 0`, then continues after it.
    pub fn if_(&mut self, cond: VReg, then: impl FnOnce(&mut Self)) {
        self.if_else(cond, then, |_| {});
    }

    pub fn if_else(
        &mut self,
        cond: VReg,
        then: impl FnOnce(&mut Self),
        otherwise: impl FnOnce(&mut Self),
    ) {
        let (then_block, else_block, join) = (self.block(), self.block(), self.block());
        self.branch(cond, then_block, else_block);
        self.switch(then_block);
        then(self);
        self.jump(join);
        self.switch(else_block);
        otherwise(self);
        self.jump(join);
        self.switch(join);
    }

    /// `while cond { body }`; `cond` is re-evaluated at the loop head.
    pub fn while_(&mut self, cond: impl FnOnce(&mut Self) -> VReg, body: impl FnOnce(&mut Self)) {
        let (head, body_block, exit) = (self.block(), self.block(), self.block());
        self.jump(head);
        self.switch(head);
        let c = cond(self);
        self.branch(c, body_block, exit);
        self.switch(body_block);
        body(self);
        self.jump(head);
        self.switch(exit);
    }

    /// The function; a block left open (dead code after a diverging statement) returns.
    #[must_use]
    pub fn finish(self) -> LirFn {
        LirFn {
            name: self.name,
            params: self.params,
            vregs: self.vregs,
            blocks: self
                .blocks
                .into_iter()
                .map(|(insts, term)| Block {
                    insts,
                    term: term.unwrap_or(Term::Ret(None)),
                })
                .collect(),
            stack_site: self.stack_site,
        }
    }
}

fn args(vregs: &[VReg]) -> String {
    vregs
        .iter()
        .map(|v| format!("v{v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn bin_name(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "add",
        BinOp::Sub => "sub",
        BinOp::Mul => "mul",
        BinOp::And => "and",
        BinOp::Or => "or",
        BinOp::Shl => "shl",
        BinOp::Shr => "shr",
        BinOp::UDiv => "udiv",
        BinOp::URem => "urem",
    }
}

fn chk_name(op: ChkOp) -> &'static str {
    match op {
        ChkOp::Add => "add",
        ChkOp::Sub => "sub",
        ChkOp::Mul => "mul",
        ChkOp::Div => "div",
        ChkOp::Rem => "rem",
    }
}

fn cmp_name(op: CmpOp) -> &'static str {
    match op {
        CmpOp::Eq => "eq",
        CmpOp::Ne => "ne",
        CmpOp::Lt => "lt",
        CmpOp::Le => "le",
        CmpOp::Gt => "gt",
        CmpOp::Ge => "ge",
        CmpOp::ULt => "ult",
    }
}

fn dump_fn(out: &mut String, function: &LirFn) {
    let _ = write!(
        out,
        "fn {} params={} vregs={}",
        function.name, function.params, function.vregs
    );
    if let Some(site) = function.stack_site {
        let _ = write!(out, " stack=site.{site}");
    }
    out.push('\n');
    for (id, block) in function.blocks.iter().enumerate() {
        let _ = writeln!(out, "b{id}:");
        for inst in &block.insts {
            let _ = match inst {
                Inst::Const(v, n) => writeln!(out, "  v{v} = const {n}"),
                Inst::Addr(v, sym) => writeln!(out, "  v{v} = addr {sym}"),
                Inst::Move(v, a) => writeln!(out, "  v{v} = move v{a}"),
                Inst::Load(v, a, off) => writeln!(out, "  v{v} = load v{a}, {off}"),
                Inst::Load8(v, a, off) => writeln!(out, "  v{v} = load8 v{a}, {off}"),
                Inst::Store(a, off, b) => writeln!(out, "  store v{a}, {off}, v{b}"),
                Inst::Store8(a, off, b) => writeln!(out, "  store8 v{a}, {off}, v{b}"),
                Inst::Bin(op, v, a, b) => writeln!(out, "  v{v} = {} v{a}, v{b}", bin_name(*op)),
                Inst::Chk(op, v, a, b, site) => {
                    writeln!(
                        out,
                        "  v{v} = {}.chk v{a}, v{b} @site.{site}",
                        chk_name(*op)
                    )
                }
                Inst::Neg(v, a, site) => writeln!(out, "  v{v} = neg.chk v{a} @site.{site}"),
                Inst::Cmp(op, v, a, b) => writeln!(out, "  v{v} = {} v{a}, v{b}", cmp_name(*op)),
                Inst::Call(Some(v), f, a) => writeln!(out, "  v{v} = call {f}({})", args(a)),
                Inst::Call(None, f, a) => writeln!(out, "  call {f}({})", args(a)),
            };
        }
        let _ = match block.term {
            Term::Jump(to) => writeln!(out, "  jump b{to}"),
            Term::Branch(c, then, otherwise) => {
                writeln!(out, "  branch v{c}, b{then}, b{otherwise}")
            }
            Term::Ret(Some(v)) => writeln!(out, "  ret v{v}"),
            Term::Ret(None) => writeln!(out, "  ret"),
            Term::Raise(site) => writeln!(out, "  raise site.{site}"),
        };
    }
}

/// The `lingua lir` dump: one function per paragraph, then one line per site.
#[must_use]
pub fn dump(functions: &[LirFn], sites: &[Site]) -> String {
    let mut out = String::new();
    for (index, function) in functions.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        dump_fn(&mut out, function);
    }
    if !sites.is_empty() {
        out.push('\n');
    }
    for (id, site) in sites.iter().enumerate() {
        let kinds: Vec<String> = site
            .reports
            .iter()
            .map(|report| match report.kind {
                ReportKind::Exception => "exception".to_owned(),
                ReportKind::Fault(code) => format!("fault {code}"),
                ReportKind::Internal => "internal".to_owned(),
            })
            .collect();
        let _ = writeln!(out, "site.{id} {} {}", kinds.join(" "), site.at);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_text_block_is_length_then_bytes_padded_to_eight() {
        assert_eq!(
            text_block("s", "hi").bytes,
            [2, 0, 0, 0, 0, 0, 0, 0, b'h', b'i', 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(text_block("s", "").bytes, [0; 8]);
        assert_eq!(text_block("s", "12345678").bytes.len(), 16);
        assert_eq!(
            singleton_block("v", 1).bytes,
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn the_builder_numbers_vregs_after_the_params_and_closes_blocks() {
        let mut b = FnBuilder::new("rt.f", 1, None);
        let one = b.konst(1);
        let c = b.cmp(CmpOp::Lt, 0, one);
        b.if_(c, |b| b.ret(Some(one)));
        b.ret(Some(0));
        assert!(!b.is_open());
        b.ret(None); // ignored: the block already has a terminator
        let function = b.finish();
        assert_eq!(function.vregs, 3);
        assert_eq!(
            dump(&[function], &[]),
            "fn rt.f params=1 vregs=3
b0:
  v1 = const 1
  v2 = lt v0, v1
  branch v2, b1, b2
b1:
  ret v1
b2:
  jump b3
b3:
  ret v0
"
        );
    }

    #[test]
    fn dump_prints_every_form_and_the_sites() {
        let function = LirFn {
            name: "m.f".to_owned(),
            params: 2,
            vregs: 9,
            blocks: vec![
                Block {
                    insts: vec![
                        Inst::Const(2, -8),
                        Inst::Addr(3, "str.0".to_owned()),
                        Inst::Move(4, 3),
                        Inst::Load(5, 0, 8),
                        Inst::Store(0, 16, 5),
                        Inst::Load8(5, 0, 8),
                        Inst::Store8(0, 9, 5),
                        Inst::Bin(BinOp::Shl, 6, 1, 2),
                        Inst::Chk(ChkOp::Div, 7, 0, 1, 1),
                        Inst::Neg(7, 7, 0),
                        Inst::Cmp(CmpOp::ULt, 8, 0, 1),
                        Inst::Call(None, "rt.g".to_owned(), vec![0, 3]),
                        Inst::Call(Some(8), "rt.h".to_owned(), vec![]),
                    ],
                    term: Term::Branch(8, 1, 2),
                },
                Block {
                    insts: vec![],
                    term: Term::Jump(2),
                },
                Block {
                    insts: vec![],
                    term: Term::Raise(2),
                },
            ],
            stack_site: Some(0),
        };
        let report = |kind| Report {
            kind,
            text: String::new(),
        };
        let sites = [
            Site {
                at: "m.lf:1:1".to_owned(),
                reports: vec![report(ReportKind::Fault("L077"))],
            },
            Site {
                at: "m.lf:3:10".to_owned(),
                reports: vec![
                    report(ReportKind::Fault("L076")),
                    report(ReportKind::Fault("L040")),
                ],
            },
            Site {
                at: "m.lf:4:3".to_owned(),
                reports: vec![report(ReportKind::Exception)],
            },
        ];
        assert_eq!(
            dump(&[function], &sites),
            "fn m.f params=2 vregs=9 stack=site.0
b0:
  v2 = const -8
  v3 = addr str.0
  v4 = move v3
  v5 = load v0, 8
  store v0, 16, v5
  v5 = load8 v0, 8
  store8 v0, 9, v5
  v6 = shl v1, v2
  v7 = div.chk v0, v1 @site.1
  v7 = neg.chk v7 @site.0
  v8 = ult v0, v1
  call rt.g(v0, v3)
  v8 = call rt.h()
  branch v8, b1, b2
b1:
  jump b2
b2:
  raise site.2

site.0 fault L077 m.lf:1:1
site.1 fault L076 fault L040 m.lf:3:10
site.2 exception m.lf:4:3
"
        );
        assert_eq!(site_symbol(1, 0), "site.1");
        assert_eq!(site_symbol(1, 1), "site.1.1");
    }
}
