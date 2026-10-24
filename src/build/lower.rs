//! Lowering TIR to LIR (docs/build.md, Lowering TIR to LIR): one `LirFn` per function in the
//! runtime residue, plus `rt.entry`, the interned Text blocks, variant singletons, and fault
//! and exception sites with their reports rendered by `crate::report`.

use std::collections::HashMap;

use super::lir::{
    BinOp, BlockId, ChkOp, CmpOp, FnBuilder, LirFn, LirProgram, Report, ReportKind, RoData, Site,
    SiteId, VReg, singleton_block, text_block,
};
use crate::ast::Span;
use crate::report;
use crate::tir::{
    self, Symbol, SymbolId, SymbolKind, TirArm, TirArmBody, TirBody, TirExpr, TirFn, TirKind,
    TirModule, TirPat, TirProgram, TirStmt, Type, TypeKind, prelude,
};

type Lowered<T> = Result<T, String>;

fn internal(what: &str) -> String {
    format!("internal error: {what}")
}

/// The symbol name of a user function: `<module>.<name>`.
#[must_use]
pub fn function_symbol(symbol: &Symbol) -> String {
    format!("{}.{}", symbol.module, symbol.name)
}

// Per-program state
struct Lowerer<'a> {
    tir: &'a TirProgram,
    strings: Vec<String>,
    singletons: Vec<RoData>,
    sites: Vec<Site>,
}

impl<'a> Lowerer<'a> {
    fn symbol(&self, id: SymbolId) -> Lowered<&'a Symbol> {
        self.tir
            .symbols
            .get(id as usize)
            .ok_or_else(|| internal(&format!("unknown symbol {id}")))
    }

    fn site(&mut self, module: &TirModule, span: Option<Span>, reports: Vec<Report>) -> SiteId {
        let at = span.map_or_else(String::new, |span| {
            format!("{}:{}:{}", module.path, span.start.line, span.start.column)
        });
        self.sites.push(Site { at, reports });
        SiteId::try_from(self.sites.len() - 1).unwrap_or(SiteId::MAX)
    }

    fn fault_site(&mut self, module: &TirModule, span: Span, codes: &[&'static str]) -> SiteId {
        let reports = codes
            .iter()
            .map(|code| Report {
                kind: ReportKind::Fault(code),
                text: report::fault(code, &module.path, &module.source, Some(span)),
            })
            .collect();
        self.site(module, Some(span), reports)
    }

    fn intern(&mut self, text: &str) -> String {
        let index = self
            .strings
            .iter()
            .position(|s| s == text)
            .unwrap_or_else(|| {
                self.strings.push(text.to_owned());
                self.strings.len() - 1
            });
        format!("str.{index}")
    }

    // The singleton of a zero-payload variant. The runtime defines the prelude's own.
    fn singleton(&mut self, variant: SymbolId) -> Lowered<String> {
        let symbol = self.symbol(variant)?;
        let SymbolKind::Variant { parent, tag, .. } = symbol.kind else {
            return Err(internal(&format!("`{}` is not a variant", symbol.name)));
        };
        let name = format!(
            "var.{}.{}.{}",
            symbol.module,
            self.symbol(parent)?.name,
            symbol.name
        );
        if symbol.module != "prelude" && !self.singletons.iter().any(|block| block.name == name) {
            self.singletons.push(singleton_block(&name, tag));
        }
        Ok(name)
    }

    fn tag(&self, variant: SymbolId) -> Lowered<i64> {
        match self.symbol(variant)?.kind {
            SymbolKind::Variant { tag, .. } => Ok(i64::from(tag)),
            _ => Err(internal(&format!("symbol {variant} is not a variant"))),
        }
    }

    fn base(&self, ty: &'a Type) -> &'a Type {
        match ty {
            Type::Nominal(id) => match self.tir.symbols.get(*id as usize).map(|s| &s.kind) {
                Some(SymbolKind::Type(TypeKind::Newtype { base, .. })) => self.base(base),
                _ => ty,
            },
            _ => ty,
        }
    }

    fn is_text(&self, ty: &'a Type) -> bool {
        *self.base(ty) == Type::Text
    }
}

// Per-function state
struct FnLowerer<'a, 'p> {
    program: &'p mut Lowerer<'a>,
    module: &'a TirModule,
    b: FnBuilder,
    locals: HashMap<SymbolId, VReg>,
    /// The function being lowered, for self tail calls.
    id: SymbolId,
}

fn bytes_offset(i: usize) -> Lowered<i32> {
    i32::try_from(8 * i).map_err(|_| internal("block offset out of range"))
}

impl<'a> FnLowerer<'a, '_> {
    fn local(&self, id: SymbolId) -> Lowered<VReg> {
        self.locals
            .get(&id)
            .copied()
            .ok_or_else(|| internal(&format!("unbound local {id}")))
    }

    fn bind(&mut self, id: SymbolId, value: VReg) {
        let v = self.b.fresh();
        self.b.mov(v, value);
        self.locals.insert(id, v);
    }

    fn alloc_words(&mut self, words: usize) -> Lowered<VReg> {
        let size = self.b.konst(i64::from(bytes_offset(words)?));
        Ok(self.b.call("rt.alloc", &[size]))
    }

    // A block of `{len, elem...}` or `{field...}` from already-lowered words
    fn block_of(&mut self, header: Option<usize>, words: &[VReg]) -> Lowered<VReg> {
        let skip = usize::from(header.is_some());
        let p = self.alloc_words(skip + words.len())?;
        if let Some(len) = header {
            let n = self.b.konst(i64::try_from(len).unwrap_or(i64::MAX));
            self.b.store(p, 0, n);
        }
        for (i, word) in words.iter().enumerate() {
            self.b.store(p, bytes_offset(skip + i)?, *word);
        }
        Ok(p)
    }

    fn exprs(&mut self, exprs: &'a [TirExpr]) -> Lowered<Vec<VReg>> {
        exprs.iter().map(|e| self.expr(e)).collect()
    }

    fn chk(&mut self, op: ChkOp, span: Span, a: &'a TirExpr, b: &'a TirExpr) -> Lowered<VReg> {
        let (a, b) = (self.expr(a)?, self.expr(b)?);
        let codes: &[&'static str] = if matches!(op, ChkOp::Div | ChkOp::Rem) {
            &["L076", "L040"]
        } else {
            &["L040"]
        };
        let site = self.program.fault_site(self.module, span, codes);
        Ok(self.b.chk(op, a, b, site))
    }

    fn call(&mut self, expr: &'a TirExpr, callee: SymbolId, args: &'a [TirExpr]) -> Lowered<VReg> {
        let values = self.exprs(args)?;
        let routine = match callee {
            prelude::TRIM => "rt.trim",
            prelude::LOWER => "rt.lower",
            prelude::JOIN => "rt.join",
            prelude::SPLIT => {
                let site = self.program.fault_site(self.module, expr.span, &["L046"]);
                let report = self.b.addr(&super::lir::site_symbol(site, 0));
                return Ok(self.b.call("rt.split", &[values[0], values[1], report]));
            }
            prelude::LENGTH => return Ok(self.b.load(values[0], 0)),
            prelude::MAX => {
                let (a, b) = (values[0], values[1]);
                let result = self.b.fresh();
                let ge = self.b.cmp(CmpOp::Ge, a, b);
                self.b
                    .if_else(ge, |f| f.mov(result, a), |f| f.mov(result, b));
                return Ok(result);
            }
            _ => {
                let symbol = self.program.symbol(callee)?;
                if symbol.kind != SymbolKind::Function {
                    return Err(internal(&format!("call of non-function `{}`", symbol.name)));
                }
                return Ok(self.b.call(&function_symbol(symbol), &values));
            }
        };
        Ok(self.b.call(routine, &values))
    }

    fn cmp(&mut self, op: tir::CmpOp, a: &'a TirExpr, b: &'a TirExpr) -> Lowered<VReg> {
        let text = self.program.is_text(&a.ty);
        let (a, b) = (self.expr(a)?, self.expr(b)?);
        let op = match op {
            tir::CmpOp::Eq => CmpOp::Eq,
            tir::CmpOp::Ne => CmpOp::Ne,
            tir::CmpOp::Lt => CmpOp::Lt,
            tir::CmpOp::Le => CmpOp::Le,
            tir::CmpOp::Gt => CmpOp::Gt,
            tir::CmpOp::Ge => CmpOp::Ge,
        };
        if text {
            let equal = self.b.call("rt.text_eq", &[a, b]);
            Ok(self.b.cmp_imm(op, equal, 1))
        } else {
            Ok(self.b.cmp(op, a, b))
        }
    }

    fn expr(&mut self, expr: &'a TirExpr) -> Lowered<VReg> {
        Ok(match &expr.kind {
            TirKind::Int(n) => self.b.konst(*n),
            TirKind::Bool(value) => self.b.konst(i64::from(*value)),
            TirKind::Nothing => self.b.konst(0),
            TirKind::Text(text) => {
                let sym = self.program.intern(text);
                self.b.addr(&sym)
            }
            TirKind::Local(id) => self.local(*id)?,
            TirKind::List(elems) => {
                let values = self.exprs(elems)?;
                self.block_of(Some(values.len()), &values)?
            }
            TirKind::Record(_, fields) => {
                let values = self.exprs(fields)?;
                self.block_of(None, &values)?
            }
            TirKind::Variant(variant, None) => {
                let sym = self.program.singleton(*variant)?;
                self.b.addr(&sym)
            }
            TirKind::Variant(variant, Some(payload)) => {
                let tag = self.program.tag(*variant)?;
                let payload = self.expr(payload)?;
                let tag = self.b.konst(tag);
                self.block_of(None, &[tag, payload])?
            }
            TirKind::Call(callee, args) => self.call(expr, *callee, args)?,
            TirKind::Field(record, index) => {
                let record = self.expr(record)?;
                self.b.load(record, bytes_offset(*index as usize)?)
            }
            TirKind::Index {
                list,
                index,
                checked,
            } => {
                let (list, index) = (self.expr(list)?, self.expr(index)?);
                if *checked {
                    self.b.call("rt.list_index", &[list, index])
                } else {
                    let three = self.b.konst(3);
                    let off = self.b.bin(BinOp::Shl, index, three);
                    let at = self.b.bin(BinOp::Add, list, off);
                    self.b.load(at, 8)
                }
            }
            TirKind::Neg(a) => {
                let a = self.expr(a)?;
                let site = self.program.fault_site(self.module, expr.span, &["L040"]);
                self.b.neg(a, site)
            }
            TirKind::Add(a, b) => self.chk(ChkOp::Add, expr.span, a, b)?,
            TirKind::Sub(a, b) => self.chk(ChkOp::Sub, expr.span, a, b)?,
            TirKind::Mul(a, b) => self.chk(ChkOp::Mul, expr.span, a, b)?,
            TirKind::Div(a, b) => self.chk(ChkOp::Div, expr.span, a, b)?,
            TirKind::Rem(a, b) => self.chk(ChkOp::Rem, expr.span, a, b)?,
            TirKind::Cmp(op, a, b) => self.cmp(*op, a, b)?,
            TirKind::Match(scrutinee, arms) => {
                let result = self.b.fresh();
                self.lower_match(expr.span, scrutinee, arms, Some(result))?;
                result
            }
            TirKind::Concat(parts) => {
                let mut texts = Vec::new();
                for part in parts {
                    let value = self.expr(part)?;
                    texts.push(if self.program.is_text(&part.ty) {
                        value
                    } else {
                        self.b.call("rt.int_to_text", &[value])
                    });
                }
                let list = self.block_of(Some(texts.len()), &texts)?;
                self.b.call("rt.text_concat", &[list])
            }
            TirKind::IntIn(source, lo, hi) => self.int_in(source, *lo, *hi)?,
            TirKind::IntAtLeast(source, lo) => self.int_in(source, *lo, i64::MAX)?,
            TirKind::ClockNow => self.b.call("rt.clock_now", &[]),
            TirKind::Log(text) => {
                let text = self.expr(text)?;
                self.b.call("rt.log", &[text])
            }
            TirKind::Let(binds, body) => {
                self.binds(binds)?;
                self.expr(body)?
            }
            TirKind::FsRead(_) | TirKind::CatalogColumns(_) | TirKind::ResourceCall(..) => {
                return Err(internal(
                    "a comptime-only or resource operation reached the runtime residue",
                ));
            }
        })
    }

    fn binds(&mut self, binds: &'a [(SymbolId, TirExpr)]) -> Lowered<()> {
        for (id, e) in binds {
            let value = self.expr(e)?;
            self.bind(*id, value);
        }
        Ok(())
    }

    // `return e`. A returned call of this function itself reuses the frame: the arguments are
    // evaluated in order into temporaries, moved into the parameters, and the body restarts
    // at the entry block, after the prologue's stack check. A `Let` is the checker's
    // source-order binding of the arguments.
    // ponytail: self tail calls only. A general tail call would have to write the callee's
    // arguments into this frame's incoming area, which the caller sized for this function's
    // parameters; reserving that area for the largest tail callee is the upgrade.
    fn ret(&mut self, e: &'a TirExpr) -> Lowered<()> {
        match &e.kind {
            TirKind::Let(binds, body) => {
                self.binds(binds)?;
                self.ret(body)
            }
            TirKind::Call(callee, args) if *callee == self.id => {
                let temps: Vec<VReg> = self
                    .exprs(args)?
                    .into_iter()
                    .map(|value| {
                        let temp = self.b.fresh();
                        self.b.mov(temp, value);
                        temp
                    })
                    .collect();
                for (param, temp) in (0..).zip(temps) {
                    self.b.mov(param, temp);
                }
                self.b.jump(0);
                Ok(())
            }
            _ => {
                let v = self.expr(e)?;
                self.b.ret(Some(v));
                Ok(())
            }
        }
    }

    fn int_in(&mut self, source: &'a TirExpr, lo: i64, hi: i64) -> Lowered<VReg> {
        let routine = if self.program.is_text(&source.ty) {
            "rt.int_in_text"
        } else {
            "rt.int_in_int"
        };
        let source = self.expr(source)?;
        let (lo, hi) = (self.b.konst(lo), self.b.konst(hi));
        Ok(self.b.call(routine, &[source, lo, hi]))
    }

    // Tests `value` against `pat`, binding as it goes; jumps to `fail` on a mismatch and
    // leaves the current block at the success path.
    fn pattern(&mut self, value: VReg, pat: &'a TirPat, fail: BlockId) -> Lowered<()> {
        let check = |f: &mut Self, cond: VReg| {
            let ok = f.b.block();
            f.b.branch(cond, ok, fail);
            f.b.switch(ok);
        };
        match pat {
            TirPat::Wild => {}
            TirPat::Bind(id) => self.bind(*id, value),
            TirPat::Variant(variant, inner) => {
                let tag = self.program.tag(*variant)?;
                let actual = self.b.load(value, 0);
                let cond = self.b.cmp_imm(CmpOp::Eq, actual, tag);
                check(self, cond);
                if let Some(inner) = inner {
                    let payload = self.b.load(value, 8);
                    self.pattern(payload, inner, fail)?;
                }
            }
            TirPat::LitInt(n) => {
                let cond = self.b.cmp_imm(CmpOp::Eq, value, *n);
                check(self, cond);
            }
            TirPat::LitText(text) => {
                let sym = self.program.intern(text);
                let literal = self.b.addr(&sym);
                let cond = self.b.call("rt.text_eq", &[value, literal]);
                check(self, cond);
            }
            TirPat::List(elems) => {
                let len = self.b.load(value, 0);
                let cond = self
                    .b
                    .cmp_imm(CmpOp::Eq, len, i64::try_from(elems.len()).unwrap_or(-1));
                check(self, cond);
                for (i, elem) in elems.iter().enumerate() {
                    let item = self.b.load(value, bytes_offset(1 + i)?);
                    self.pattern(item, elem, fail)?;
                }
            }
        }
        Ok(())
    }

    // Arms in order; an expression arm writes `result` and joins. Falling off the last arm is
    // unreachable in a checked program and raises an internal site.
    fn lower_match(
        &mut self,
        span: Span,
        scrutinee: &'a TirExpr,
        arms: &'a [TirArm],
        result: Option<VReg>,
    ) -> Lowered<()> {
        let value = self.expr(scrutinee)?;
        let join = self.b.block();
        for arm in arms {
            let next = self.b.block();
            self.pattern(value, &arm.pat, next)?;
            match &arm.body {
                TirArmBody::Expr(e) => {
                    let v = self.expr(e)?;
                    if let Some(result) = result {
                        self.b.mov(result, v);
                    }
                }
                TirArmBody::Stmts(stmts) => self.stmts(stmts)?,
            }
            self.b.jump(join);
            self.b.switch(next);
        }
        let text = format!(
            "internal error: no match arm matched at {}:{}:{}\n",
            self.module.path, span.start.line, span.start.column
        );
        let site = self.program.site(
            self.module,
            Some(span),
            vec![Report {
                kind: ReportKind::Internal,
                text,
            }],
        );
        self.b.raise(site);
        self.b.switch(join);
        Ok(())
    }

    fn exception(&mut self, message: &str, span: Span) {
        let text = report::exception(message, &self.module.path, &self.module.source, span);
        let site = self.program.site(
            self.module,
            Some(span),
            vec![Report {
                kind: ReportKind::Exception,
                text,
            }],
        );
        self.b.raise(site);
    }

    // `i` counts from `start` (moved in) while `i < end`; `body` sees `i` each iteration
    fn counted(
        &mut self,
        start: VReg,
        end: VReg,
        body: impl FnOnce(&mut Self, VReg) -> Lowered<()>,
    ) -> Lowered<()> {
        let i = self.b.fresh();
        self.b.mov(i, start);
        let (head, body_block, exit) = (self.b.block(), self.b.block(), self.b.block());
        self.b.jump(head);
        self.b.switch(head);
        let cond = self.b.cmp(CmpOp::Lt, i, end);
        self.b.branch(cond, body_block, exit);
        self.b.switch(body_block);
        body(self, i)?;
        if self.b.is_open() {
            // i < end <= INT_MAX, so i + 1 cannot overflow
            self.b.add_imm(i, 1);
            self.b.jump(head);
        }
        self.b.switch(exit);
        Ok(())
    }

    fn stmt(&mut self, stmt: &'a TirStmt) -> Lowered<()> {
        match stmt {
            TirStmt::Let(id, e) => {
                let v = self.expr(e)?;
                self.bind(*id, v);
            }
            TirStmt::Set(id, e) => {
                let v = self.expr(e)?;
                let var = self.local(*id)?;
                self.b.mov(var, v);
            }
            TirStmt::Return(e) => self.ret(e)?,
            TirStmt::Assert {
                cond,
                message,
                span,
            } => {
                let cond = self.expr(cond)?;
                let (ok, fail) = (self.b.block(), self.b.block());
                self.b.branch(cond, ok, fail);
                self.b.switch(fail);
                self.exception(message, *span);
                self.b.switch(ok);
            }
            TirStmt::Fail { message, span } => self.exception(message, *span),
            TirStmt::If(cond, then, otherwise) => {
                let cond = self.expr(cond)?;
                let (then_block, else_block, join) =
                    (self.b.block(), self.b.block(), self.b.block());
                self.b.branch(cond, then_block, else_block);
                for (block, body) in [(then_block, then), (else_block, otherwise)] {
                    self.b.switch(block);
                    self.stmts(body)?;
                    self.b.jump(join);
                }
                self.b.switch(join);
            }
            TirStmt::ForList(id, list, body) => {
                // A copy: the list may be a `var` the body sets
                let list = self.expr(list)?;
                let list = {
                    let copy = self.b.fresh();
                    self.b.mov(copy, list);
                    copy
                };
                let len = self.b.load(list, 0);
                let zero = self.b.konst(0);
                self.counted(zero, len, |f, i| {
                    let three = f.b.konst(3);
                    let off = f.b.bin(BinOp::Shl, i, three);
                    let at = f.b.bin(BinOp::Add, list, off);
                    let item = f.b.load(at, 8);
                    f.locals.insert(*id, item);
                    f.stmts(body)
                })?;
            }
            TirStmt::ForRange(id, low, high, body) => {
                // Both bounds are evaluated once, on entry: copy `high` out of any `var`
                let (low, high) = (self.expr(low)?, self.expr(high)?);
                let end = self.b.fresh();
                self.b.mov(end, high);
                self.counted(low, end, |f, i| {
                    f.bind(*id, i);
                    f.stmts(body)
                })?;
            }
            TirStmt::WhileIs {
                scrutinee,
                variant,
                payload,
                body,
            } => {
                let tag = self.program.tag(*variant)?;
                let var = self.local(*scrutinee)?;
                let (head, body_block, exit) = (self.b.block(), self.b.block(), self.b.block());
                self.b.jump(head);
                self.b.switch(head);
                let actual = self.b.load(var, 0);
                let cond = self.b.cmp_imm(CmpOp::Eq, actual, tag);
                self.b.branch(cond, body_block, exit);
                self.b.switch(body_block);
                if let Some(id) = payload {
                    let value = self.b.load(var, 8);
                    self.bind(*id, value);
                }
                self.stmts(body)?;
                self.b.jump(head);
                self.b.switch(exit);
            }
            TirStmt::Match(scrutinee, arms) => {
                self.lower_match(scrutinee.span, scrutinee, arms, None)?;
            }
            TirStmt::Expr(e) => {
                self.expr(e)?;
            }
        }
        Ok(())
    }

    // Statements after a diverging one are unreachable (L018) and are not lowered
    fn stmts(&mut self, stmts: &'a [TirStmt]) -> Lowered<()> {
        for stmt in stmts {
            if !self.b.is_open() {
                break;
            }
            self.stmt(stmt)?;
        }
        Ok(())
    }
}

fn lower_fn<'a>(
    program: &mut Lowerer<'a>,
    module: &'a TirModule,
    function: &'a TirFn,
) -> Lowered<LirFn> {
    let symbol = program.symbol(function.id)?;
    let name = function_symbol(symbol);
    let stack = program.fault_site(module, function.span, &["L077"]);
    let params = u32::try_from(function.params.len()).map_err(|_| internal("too many params"))?;
    let mut lowerer = FnLowerer {
        program,
        module,
        b: FnBuilder::new(&name, params, Some(stack)),
        locals: function.params.iter().copied().zip(0..).collect(),
        id: function.id,
    };
    match &function.body {
        TirBody::Primitive if name == "money.percent" && params == 3 => {
            let result = lowerer.b.call("rt.percent", &[0, 1, 2]);
            lowerer.b.ret(Some(result));
        }
        TirBody::Primitive => {
            return Err(internal(&format!("`{name}` is not an allowed primitive")));
        }
        TirBody::Stmts(stmts) => {
            lowerer.stmts(stmts)?;
            // Every path returns (L012); this only closes a `Nothing` body left open
            if lowerer.b.is_open() {
                let nothing = lowerer.b.konst(0);
                lowerer.b.ret(Some(nothing));
            }
        }
    }
    Ok(lowerer.b.finish())
}

// rt.entry: `Process` if `main` takes it, `main`, then exit 0
fn entry(main: &str, takes_process: bool) -> LirFn {
    let mut b = FnBuilder::new("rt.entry", 0, None);
    let args = if takes_process {
        vec![b.call("rt.process", &[])]
    } else {
        Vec::new()
    };
    b.call_void(main, &args);
    let zero = b.konst(0);
    b.call_void("rt.sys_exit", &[zero]);
    b.ret(None);
    b.finish()
}

/// Lowers the runtime residue of `program`. Text literals are interned once per program.
///
/// # Errors
/// An internal error for TIR that cannot reach the backend: an unknown symbol, a resource
/// or comptime-only node, or a primitive other than `money.percent`.
pub fn lower(program: &TirProgram) -> Lowered<LirProgram> {
    let mut lowerer = Lowerer {
        tir: program,
        strings: Vec::new(),
        singletons: Vec::new(),
        sites: Vec::new(),
    };
    let mut functions = Vec::new();
    for module in &program.modules {
        for function in &module.functions {
            functions.push(lower_fn(&mut lowerer, module, function)?);
        }
    }
    let main = program
        .modules
        .iter()
        .flat_map(|module| &module.functions)
        .find(|function| function.id == program.main)
        .ok_or_else(|| internal("`main` is not in the link set"))?;
    let entry = entry(
        &function_symbol(lowerer.symbol(program.main)?),
        main.params.len() == 1,
    );
    let mut rodata: Vec<RoData> = lowerer
        .strings
        .iter()
        .enumerate()
        .map(|(index, text)| text_block(&format!("str.{index}"), text))
        .collect();
    rodata.append(&mut lowerer.singletons);
    for (id, site) in lowerer.sites.iter().enumerate() {
        for (index, report) in site.reports.iter().enumerate() {
            let id = SiteId::try_from(id).unwrap_or(SiteId::MAX);
            rodata.push(text_block(
                &super::lir::site_symbol(id, index),
                &report.text,
            ));
        }
    }
    Ok(LirProgram {
        functions,
        entry,
        rodata,
        sites: lowerer.sites,
    })
}

#[cfg(test)]
#[path = "lower_tests.rs"]
mod tests;
