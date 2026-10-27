//! Function bodies and examples: bidirectional `check` / `synth` into TIR (docs/lingua1.md,
//! Types, Effects, Value errors and exceptions). Patterns and exhaustiveness are
//! `pattern.rs`.

use super::decl::{Binding, Checker, Sig, alternatives, find_word, word_at};
use super::types::{self, contains};
use crate::ast::{
    Arg, Arm, ArmBody, ArmKind, AssignOp, BinOp, Decl, Effect, Example, Expect, Expr, FnBody,
    Pattern, Span, Stmt, TextPart,
};
use crate::lex::Location;
use crate::tir::{
    CmpOp, ExampleKind, Range, SymbolId, SymbolKind, TirArm, TirArmBody, TirBody, TirExample,
    TirExpect, TirExpr, TirFn, TirKind, TirPat, TirStmt, Type, TypeKind, prelude,
};

/// The enclosing function, and the local scopes from outermost to innermost.
pub(super) struct FnCx {
    pub m: usize,
    pub effect: Effect,
    pub ret: Type,
    pub scopes: Vec<Vec<(String, SymbolId)>>,
    /// Inside a function example's inputs or expected value.
    pub example: bool,
    /// A comptime context: a `const` initializer, a bound, a `comptime` operand, or a package
    /// example. A runtime local here is `L060`, and `let` or `var` is `L062`.
    pub comptime: bool,
    /// The body of a function that returns `Expr` or `Type`, whose parameters are the
    /// call's comptime values. Not an inner `const` there, which folds at check time.
    pub macro_body: bool,
}

impl FnCx {
    /// A comptime root at file level in module `m`.
    pub(super) fn root(m: usize, effect: Effect) -> Self {
        FnCx {
            m,
            effect,
            ret: Type::Nothing,
            scopes: vec![Vec::new()],
            example: false,
            comptime: true,
            macro_body: false,
        }
    }

    /// A comptime context inside this one, seeing the same locals.
    pub(super) fn comptime(&self) -> Self {
        FnCx {
            m: self.m,
            effect: self.effect.clone(),
            ret: self.ret.clone(),
            scopes: self.scopes.clone(),
            example: self.example,
            comptime: true,
            macro_body: false,
        }
    }
}

pub(super) fn tx(ty: Type, span: Span, kind: TirKind) -> TirExpr {
    TirExpr { ty, span, kind }
}

#[allow(clippy::unnecessary_box_returns)]
fn bx(e: TirExpr) -> Box<TirExpr> {
    Box::new(e)
}

fn stmt_span(stmt: &Stmt) -> Span {
    match stmt {
        Stmt::Let { span, .. }
        | Stmt::Var { span, .. }
        | Stmt::Const { span, .. }
        | Stmt::Set { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::Assert { span, .. }
        | Stmt::Fail { span, .. }
        | Stmt::If { span, .. }
        | Stmt::ForList { span, .. }
        | Stmt::ForRange { span, .. }
        | Stmt::WhileIs { span, .. }
        | Stmt::Provide { span, .. }
        | Stmt::ExprStmt { span, .. } => *span,
    }
}

/// The value of an integer literal or unary `-` of one.
pub(super) fn literal_int(e: &Expr) -> Option<i64> {
    match e {
        Expr::IntLit(_, v) => Some(*v),
        Expr::UnaryNeg(_, inner) => match inner.as_ref() {
            Expr::IntLit(_, v) => Some(-*v),
            _ => None,
        },
        _ => None,
    }
}

// Flexible literals wait in arm joins and adopt across `==`
fn flexible(e: &Expr) -> bool {
    literal_int(e).is_some() || (matches!(e, Expr::Text(..)) && hole_free(e).is_some())
}

fn hole_free(e: &Expr) -> Option<String> {
    match e {
        Expr::Text(_, parts) => parts
            .iter()
            .map(|p| match p {
                TextPart::Literal(s) => Some(s.as_str()),
                TextPart::Hole(_) => None,
            })
            .collect(),
        _ => None,
    }
}

fn effect_text(effect: &Effect) -> String {
    match effect {
        Effect::Pure => "pure".into(),
        Effect::ReadsClock => "reads clock".into(),
        Effect::ReadsNet => "reads net".into(),
        Effect::ReadsFs => "reads fs".into(),
        Effect::ReadsCatalog => "reads catalog".into(),
        Effect::Reads(r) => format!("reads {r}"),
        Effect::Writes(r) => format!("writes {r}"),
    }
}

// The effect lattice: pure < reads R < writes R; pure < reads clock. Resources are per module.
fn may_call(from: &Effect, from_module: usize, to: &Effect, to_module: usize) -> bool {
    match (to, from) {
        (Effect::Pure, _) | (Effect::ReadsClock, Effect::ReadsClock) => true,
        (Effect::Reads(r), Effect::Reads(c) | Effect::Writes(c))
        | (Effect::Writes(r), Effect::Writes(c)) => r == c && from_module == to_module,
        _ => false,
    }
}

// Whether a path through `stmts` can reach the end without `set var`; `None` if every path
// leaves the function (docs/lingua1.md, `while` definite assignment)
fn reaches_end(stmts: &[Stmt], var: &str, assigned: bool) -> Option<bool> {
    let mut assigned = assigned;
    for stmt in stmts {
        match stmt {
            Stmt::Set { name, .. } if name == var => assigned = true,
            Stmt::Return { .. } | Stmt::Fail { .. } => return None,
            Stmt::ExprStmt {
                expr: Expr::Match(_, _, arms, ArmKind::StmtArms),
                ..
            } => {
                let mut out: Option<bool> = None;
                for arm in arms {
                    if let ArmBody::StmtArm(body) = &arm.body
                        && let Some(a) = reaches_end(body, var, assigned)
                    {
                        out = Some(out.unwrap_or(true) && a);
                    }
                }
                {
                    let a = out?;
                    assigned = a;
                }
            }
            Stmt::If {
                then, otherwise, ..
            } => {
                let t = reaches_end(then, var, assigned);
                let e = reaches_end(otherwise.as_deref().unwrap_or(&[]), var, assigned);
                match (t, e) {
                    (None, None) => return None,
                    (Some(a), None) | (None, Some(a)) => assigned = a,
                    (Some(a), Some(b)) => assigned = a && b,
                }
            }
            _ => {}
        }
    }
    Some(assigned)
}

/// Arguments in parameter order, by the positional-call rule.
pub(super) struct Matched<'e> {
    pub args: Vec<Option<&'e Expr>>,
    /// The parameter index of each argument, in source order.
    order: Vec<usize>,
}

impl Checker<'_> {
    fn sym_ty(&self, id: SymbolId) -> Type {
        self.symbols[id as usize].ty.clone()
    }

    fn int_range(&self, ty: &Type) -> Option<Range> {
        types::range(&self.symbols, ty)
    }

    // The range an operand contributes: a literal's singleton, else its type's
    fn expr_range(&self, e: &TirExpr) -> Option<Range> {
        match e.kind {
            TirKind::Int(v) => Some(Range { lo: v, hi: v }),
            _ => self.int_range(&e.ty),
        }
    }

    fn local(cx: &FnCx, name: &str) -> Option<SymbolId> {
        cx.scopes
            .iter()
            .rev()
            .flat_map(|s| s.iter().rev())
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
    }

    pub(super) fn resolve_name(&self, cx: &FnCx, name: &str) -> Option<Binding> {
        Self::local(cx, name)
            .map(Binding::Local)
            .or_else(|| self.lookup(cx.m, name))
    }

    fn visible_in(&self, cx: &FnCx) -> Vec<String> {
        let mut names: Vec<String> = cx
            .scopes
            .iter()
            .rev()
            .flat_map(|s| s.iter().rev().map(|(n, _)| n.clone()))
            .collect();
        names.extend(self.visible(cx.m));
        names
    }

    pub(super) fn unknown(&mut self, cx: &FnCx, span: Span, name: &str) {
        let help = alternatives(&self.visible_in(cx));
        self.report_help(cx.m, "L011", span, format!("unknown name `{name}`"), help);
    }

    /// Binds a local, `L013` if the name is already visible anywhere.
    pub(super) fn bind_local(
        &mut self,
        cx: &mut FnCx,
        name: &str,
        span: Span,
        ty: Type,
        mutable: bool,
    ) -> SymbolId {
        if self.resolve_name(cx, name).is_some() {
            self.report(
                cx.m,
                "L013",
                span,
                format!("`{name}` is already bound; lingua1 has no shadowing"),
            );
        }
        let id = self.push_symbol(name, SymbolKind::Local, ty, cx.m, span);
        self.symbols[id as usize].mutable = mutable;
        if let Some(scope) = cx.scopes.last_mut() {
            scope.push((name.to_owned(), id));
        }
        id
    }

    fn fresh(&mut self, cx: &FnCx, name: &str, ty: Type, span: Span) -> SymbolId {
        self.push_symbol(name, SymbolKind::Local, ty, cx.m, span)
    }

    fn mismatch(&mut self, cx: &FnCx, span: Span, want: &Type, got: &Type) {
        let summary = format!("expected `{}`, found `{}`", self.show(want), self.show(got));
        if types::is_nominal_text(&self.symbols, want) && types::is_textual(&self.symbols, got) {
            let help = format!("wrap it: `{}(...)`", self.show(want));
            self.report_help(cx.m, "L020", span, summary, help);
        } else {
            self.report(cx.m, "L020", span, summary);
        }
    }

    // ---- functions and examples

    pub(super) fn function(&mut self, m: usize, decl: &Decl) -> Option<TirFn> {
        let Decl::Function {
            span,
            name,
            ret,
            effect,
            examples,
            body,
            ..
        } = decl
        else {
            return None;
        };
        let Some(Binding::Function(id)) = self.mods[m].names.get(name).copied() else {
            return None;
        };
        let sig = self.sigs.get(&id)?.clone();
        let header = Span {
            start: span.start,
            end: ret.span().end,
        };
        let tir_examples: Vec<TirExample> = examples
            .iter()
            .filter_map(|ex| self.example(m, &sig, ex))
            .collect();
        // A function that returns `Expr` or `Type` has a comptime body
        let comptime_only = matches!(sig.ret, Type::ExprAst | Type::TypeAst);
        let mut cx = FnCx {
            m,
            effect: effect.clone(),
            ret: sig.ret.clone(),
            scopes: vec![
                sig.params
                    .iter()
                    .map(|(n, _)| n.clone())
                    .zip(sig.ids.iter().copied())
                    .collect(),
            ],
            example: false,
            comptime: comptime_only,
            macro_body: comptime_only,
        };
        let body = match body {
            FnBody::Primitive => TirBody::Primitive,
            FnBody::Statements(stmts) => {
                let (out, diverges) = self.block(&mut cx, stmts);
                if !diverges {
                    self.report(
                        m,
                        "L012",
                        header,
                        format!("`{name}` can reach `end fn` without returning"),
                    );
                }
                TirBody::Stmts(out)
            }
        };
        if self.poisoned.contains(&id) {
            return None;
        }
        Some(TirFn {
            id,
            span: header,
            effect: effect.clone(),
            params: sig.ids.clone(),
            ret: sig.ret,
            body,
            examples: tir_examples,
        })
    }

    // A function example: each parameter bound once by name, the effect's script, and the
    // expected value, all checked against the signature. `crate::eval` runs it.
    fn example(&mut self, m: usize, sig: &Sig, ex: &Example) -> Option<TirExample> {
        let mut cx = FnCx {
            m,
            effect: sig.effect.clone(),
            ret: sig.ret.clone(),
            scopes: vec![Vec::new()],
            example: true,
            comptime: false,
            macro_body: false,
        };
        let resource = match &sig.effect {
            Effect::Reads(r) | Effect::Writes(r) => Some(r.clone()),
            Effect::ReadsClock => Some("clock".to_owned()),
            _ => None,
        };
        let mut inputs = Vec::new();
        let mut script = None;
        let mut seen: Vec<&str> = Vec::new();
        let mut ok = true;
        for bind in &ex.bindings {
            let span = word_at(bind.span, bind.name.chars().count());
            if seen.contains(&bind.name.as_str()) {
                self.report(
                    m,
                    "L017",
                    span,
                    format!("`{}` is bound twice in this example", bind.name),
                );
                continue;
            }
            seen.push(&bind.name);
            if let Some(i) = sig.params.iter().position(|(n, _)| *n == bind.name) {
                match self.check(&mut cx, &bind.value, &sig.params[i].1) {
                    Some(e) => inputs.push((sig.ids[i], e)),
                    None => ok = false,
                }
            } else if resource.as_deref() == Some(bind.name.as_str()) {
                script = self.script(&mut cx, sig, &bind.value);
                ok &= script.is_some();
            } else {
                let mut names: Vec<String> = sig.params.iter().map(|(n, _)| n.clone()).collect();
                names.extend(resource.clone());
                let summary = format!(
                    "`{}` is not a parameter or the effect's resource",
                    bind.name
                );
                self.report_help(m, "L045", span, summary, alternatives(&names));
            }
        }
        let label_span = word_at(ex.span, "example".len());
        for (name, _) in &sig.params {
            if !seen.contains(&name.as_str()) {
                let summary = format!("example \"{}\" does not bind `{name}`", ex.label);
                self.report(m, "L044", label_span, summary);
                ok = false;
            }
        }
        if let Some(r) = &resource
            && !seen.contains(&r.as_str())
        {
            let summary = format!(
                "example \"{}\" does not bind the script for `{r}`",
                ex.label
            );
            self.report(m, "L044", label_span, summary);
            ok = false;
        }
        let expect = match &ex.expect {
            Expect::Value(e) => TirExpect::Value(self.check(&mut cx, e, &sig.ret)?),
            Expect::Exception { message } => TirExpect::Exception {
                message: message.clone(),
            },
        };
        ok.then(|| TirExample {
            label: ex.label.clone(),
            span: ex.span,
            kind: ExampleKind::Function,
            inputs,
            script,
            body: Vec::new(),
            expect,
        })
    }

    // The script is `List<Return>`; an element that checks as `S` of `Result<S, E>` is
    // wrapped in `Ok`
    fn script(&mut self, cx: &mut FnCx, sig: &Sig, value: &Expr) -> Option<TirExpr> {
        let ret = match &sig.effect {
            Effect::ReadsClock => Type::Int,
            _ => self
                .resource_ops
                .values()
                .find(|op| {
                    self.sigs.get(op).is_some_and(|s| {
                        s.module == cx.m && matches!((&s.effect, &sig.effect), (Effect::Reads(a) | Effect::Writes(a), Effect::Reads(b) | Effect::Writes(b)) if a == b)
                    })
                })
                .map(|op| self.sigs[op].ret.clone())?,
        };
        let list = Type::List(Box::new(ret.clone()));
        let Expr::ListLit(span, items, _) = value else {
            return self.check(cx, value, &list);
        };
        let mut out = Vec::new();
        for item in items {
            if let Type::Result(ok, _) = &ret {
                let mark = self.diags.len();
                if let Some(e) = self.check(cx, item, ok) {
                    let span = e.span;
                    out.push(tx(
                        ret.clone(),
                        span,
                        TirKind::Variant(prelude::OK, Some(bx(e))),
                    ));
                    continue;
                }
                self.diags.truncate(mark);
            }
            out.push(self.check(cx, item, &ret)?);
        }
        Some(tx(list, *span, TirKind::List(out)))
    }

    // ---- statements

    pub(super) fn block(&mut self, cx: &mut FnCx, stmts: &[Stmt]) -> (Vec<TirStmt>, bool) {
        cx.scopes.push(Vec::new());
        let mut out = Vec::new();
        let mut dead = false;
        let mut reported = false;
        for stmt in stmts {
            if dead && !reported {
                self.report(
                    cx.m,
                    "L018",
                    stmt_span(stmt),
                    "unreachable statement".into(),
                );
                reported = true;
            }
            let (lowered, diverges) = self.stmt(cx, stmt);
            out.extend(lowered);
            dead |= diverges;
        }
        cx.scopes.pop();
        (out, dead)
    }

    fn message(&mut self, cx: &FnCx, e: &Expr, what: &str) -> Option<String> {
        let text = hole_free(e);
        if text.is_none() {
            self.report(
                cx.m,
                "L068",
                e.span(),
                format!("{what} must be a string literal without holes"),
            );
        }
        text
    }

    fn bool_cond(
        &mut self,
        cx: &mut FnCx,
        e: &Expr,
        code: &'static str,
        what: &str,
    ) -> Option<TirExpr> {
        let c = self.synth(cx, e)?;
        if c.ty == Type::Bool {
            Some(c)
        } else {
            let shown = self.show(&c.ty);
            self.report(
                cx.m,
                code,
                e.span(),
                format!("{what} needs a `Bool`, not `{shown}`; lingua1 has no truthiness"),
            );
            None
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn stmt(&mut self, cx: &mut FnCx, stmt: &Stmt) -> (Option<TirStmt>, bool) {
        let m = cx.m;
        match stmt {
            Stmt::Let {
                span,
                name,
                ty,
                init,
            }
            | Stmt::Var {
                span,
                name,
                ty,
                init,
            } => {
                let mutable = matches!(stmt, Stmt::Var { .. });
                if cx.comptime {
                    let keyword = if mutable { "var" } else { "let" };
                    self.report_help(
                        m,
                        "L062",
                        word_at(*span, keyword.len()),
                        format!("`{keyword}` in a comptime context"),
                        "bind it with `const`".into(),
                    );
                }
                let value = match ty {
                    Some(ty) => self
                        .resolve_type(m, ty)
                        .filter(|t| cx.comptime || self.runtime_type(m, t, ty.span()))
                        .and_then(|t| self.check(cx, init, &t).map(|e| (e, t))),
                    None => self.synth(cx, init).map(|e| {
                        let t = e.ty.clone();
                        (e, t)
                    }),
                };
                let keyword = if mutable { "var" } else { "let" };
                let at = Span {
                    start: Location {
                        line: span.start.line,
                        column: span.start.column + keyword.len(),
                    },
                    end: span.end,
                };
                let name_span = find_word(self.source(m), at, name);
                let ty = value.as_ref().map_or(Type::Nothing, |(_, t)| t.clone());
                let id = self.bind_local(cx, name, name_span, ty, mutable);
                if cx.comptime {
                    // `L062` above already explains every use
                    self.poisoned.insert(id);
                    return (None, false);
                }
                if let Some((e, _)) = value {
                    (Some(TirStmt::Let(id, e)), false)
                } else {
                    self.poisoned.insert(id);
                    (None, false)
                }
            }
            Stmt::Const {
                span,
                name,
                ty,
                init,
            } => {
                self.inner_const(cx, *span, name, ty.as_ref(), init);
                (None, false)
            }
            Stmt::Set {
                span,
                name,
                op,
                value,
            } => {
                let name_span = find_word(self.source(m), *span, name);
                let id = match self.resolve_name(cx, name) {
                    Some(Binding::Local(id)) if self.symbols[id as usize].mutable => id,
                    Some(Binding::Poison) => return (None, false),
                    Some(_) => {
                        self.report(
                            m,
                            "L014",
                            name_span,
                            format!("`set` needs a `var`; `{name}` is not one"),
                        );
                        return (None, false);
                    }
                    None => {
                        self.unknown(cx, name_span, name);
                        return (None, false);
                    }
                };
                if self.is_poisoned(id) {
                    let _ = self.synth(cx, value);
                    return (None, false);
                }
                let ty = self.sym_ty(id);
                let e = match op {
                    AssignOp::Assign => self.check(cx, value, &ty),
                    AssignOp::AddAssign => self.add_assign(cx, *span, id, &ty, value),
                };
                (e.map(|e| TirStmt::Set(id, e)), false)
            }
            Stmt::Return { span, .. }
                if cx.comptime && !matches!(cx.ret, Type::TypeAst | Type::ExprAst) =>
            {
                let summary = "`return` in a package example or a package `comptime` block: there is no function to return from";
                self.report(m, "L012", word_at(*span, "return".len()), summary.into());
                (None, true)
            }
            Stmt::Return { value, .. } => {
                let ret = cx.ret.clone();
                (self.check(cx, value, &ret).map(TirStmt::Return), true)
            }
            Stmt::Assert {
                span,
                cond,
                message,
            } => {
                let c = self.bool_cond(cx, cond, "L020", "`assert`");
                let msg = self.message(cx, message, "an `assert` message");
                let out = c.zip(msg).map(|(cond, message)| TirStmt::Assert {
                    cond,
                    message,
                    span: word_at(*span, "assert".len()),
                });
                (out, false)
            }
            Stmt::Fail { span, message } => {
                let msg = self.message(cx, message, "a `fail` message");
                let out = msg.map(|message| TirStmt::Fail {
                    message,
                    span: word_at(*span, "fail".len()),
                });
                (out, true)
            }
            Stmt::If {
                cond,
                then,
                otherwise,
                ..
            } => {
                let c = self.bool_cond(cx, cond, "L027", "`if`");
                let (then, a) = self.block(cx, then);
                let (otherwise, b) = match otherwise {
                    Some(stmts) => self.block(cx, stmts),
                    None => (Vec::new(), false),
                };
                (c.map(|c| TirStmt::If(c, then, otherwise)), a && b)
            }
            Stmt::ForList {
                span,
                name,
                iter,
                body,
            } => {
                let list = self.synth(cx, iter);
                let elem = match list.as_ref().map(|l| l.ty.clone()) {
                    Some(Type::List(t)) => Some(*t),
                    Some(other) => {
                        let shown = self.show(&other);
                        self.report(
                            m,
                            "L020",
                            iter.span(),
                            format!("`for ... in` needs a `List`, not `{shown}`"),
                        );
                        None
                    }
                    None => None,
                };
                cx.scopes.push(Vec::new());
                let name_span = find_word(self.source(m), *span, name);
                let id = self.bind_local(
                    cx,
                    name,
                    name_span,
                    elem.clone().unwrap_or(Type::Nothing),
                    false,
                );
                if elem.is_none() {
                    self.poisoned.insert(id);
                }
                let (body, _) = self.block(cx, body);
                cx.scopes.pop();
                (
                    list.filter(|_| elem.is_some())
                        .map(|l| TirStmt::ForList(id, l, body)),
                    false,
                )
            }
            Stmt::ForRange {
                span,
                name,
                low,
                high,
                body,
            } => {
                let lo = self.synth(cx, low);
                let hi = self.synth(cx, high);
                let mut ok = lo.is_some() && hi.is_some();
                for e in [&lo, &hi].into_iter().flatten() {
                    if !types::is_int(&self.symbols, &e.ty) {
                        let shown = self.show(&e.ty);
                        self.report(
                            m,
                            "L020",
                            e.span,
                            format!("a `for` range needs integer bounds, not `{shown}`"),
                        );
                        ok = false;
                    }
                }
                let ty = match (literal_int(low), literal_int(high)) {
                    (Some(l), Some(u)) if l < u => types::of_range(Range { lo: l, hi: u - 1 }),
                    (Some(l), _) if l >= 0 => types::of_range(Range {
                        lo: l,
                        hi: i64::MAX,
                    }),
                    _ => Type::Int,
                };
                cx.scopes.push(Vec::new());
                let name_span = find_word(self.source(m), *span, name);
                let id = self.bind_local(cx, name, name_span, ty, false);
                let (body, _) = self.block(cx, body);
                cx.scopes.pop();
                let out = lo
                    .zip(hi)
                    .filter(|_| ok)
                    .map(|(lo, hi)| TirStmt::ForRange(id, lo, hi, body));
                (out, false)
            }
            Stmt::WhileIs {
                span,
                name,
                pattern,
                body,
            } => (self.while_is(cx, *span, name, pattern, body), false),
            Stmt::Provide { span, .. } => {
                self.report(
                    m,
                    "L067",
                    *span,
                    "`provide` is legal only in a package `comptime` block".into(),
                );
                (None, false)
            }
            Stmt::ExprStmt { span, expr } => {
                if let Some((id, call)) = self.macro_call(cx, expr) {
                    return self
                        .splice_stmt(cx, expr, id, call)
                        .unwrap_or((None, false));
                }
                if let Expr::Match(mspan, scrutinee, arms, ArmKind::StmtArms) = expr {
                    return self.stmt_match(cx, *mspan, scrutinee, arms);
                }
                let Some(e) = self.synth(cx, expr) else {
                    return (None, false);
                };
                if e.ty != Type::Nothing {
                    let shown = self.show(&e.ty);
                    self.report_help(
                        m,
                        "L029",
                        *span,
                        format!("an expression statement must have type `Nothing`, not `{shown}`"),
                        "bind the value with `let`, or remove the statement".into(),
                    );
                    return (None, false);
                }
                (Some(TirStmt::Expr(e)), false)
            }
        }
    }

    // `set x += e` checks `x + e` against `x`'s type
    fn add_assign(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        id: SymbolId,
        ty: &Type,
        value: &Expr,
    ) -> Option<TirExpr> {
        if !types::is_int(&self.symbols, ty) {
            let shown = self.show(ty);
            self.report(
                cx.m,
                "L035",
                span,
                format!("`+=` needs an integer `var`, not `{shown}`"),
            );
            return None;
        }
        let right = self.synth(cx, value)?;
        if !types::is_int(&self.symbols, &right.ty) {
            let shown = self.show(&right.ty);
            self.report(
                cx.m,
                "L034",
                value.span(),
                format!("`+` needs integer operands, not `{shown}`"),
            );
            return None;
        }
        let left = tx(ty.clone(), span, TirKind::Local(id));
        let result = types::arith(
            BinOp::Add,
            self.expr_range(&left)?,
            self.expr_range(&right)?,
        );
        let sum = tx(result, value.span(), TirKind::Add(bx(left), bx(right)));
        self.coerce(cx, sum, ty, value.span(), Some(false))
    }

    #[allow(clippy::too_many_lines)]
    fn while_is(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        name: &str,
        pattern: &Pattern,
        body: &[Stmt],
    ) -> Option<TirStmt> {
        let m = cx.m;
        let name_span = find_word(
            self.source(m),
            Span {
                start: Location {
                    line: span.start.line,
                    column: span.start.column + 5,
                },
                end: span.end,
            },
            name,
        );
        let id = match self.resolve_name(cx, name) {
            Some(Binding::Local(id)) if self.symbols[id as usize].mutable => Some(id),
            Some(Binding::Poison) => None,
            Some(_) => {
                self.report(
                    m,
                    "L014",
                    name_span,
                    format!("`while {name} is ...` needs a `var`; `{name}` is not one"),
                );
                None
            }
            None => {
                self.unknown(cx, name_span, name);
                None
            }
        }
        .filter(|id| !self.is_poisoned(*id));
        let ty = id.map(|id| self.sym_ty(id));
        let variants = ty.as_ref().and_then(|t| self.sum_variants(t));
        if let (Some(t), None) = (&ty, &variants) {
            let shown = self.show(t);
            self.report(
                m,
                "L020",
                name_span,
                format!("`while ... is` needs a closed sum, not `{shown}`"),
            );
        }
        let (vname, vspan, inner) = match pattern {
            Pattern::Bind(s, n) => (n.clone(), *s, None),
            Pattern::VariantPat(s, n, p) => (n.clone(), *s, p.as_deref()),
            Pattern::OkPat(s, p) => ("Ok".to_owned(), *s, Some(p.as_ref())),
            Pattern::ErrPat(s, p) => ("Err".to_owned(), *s, Some(p.as_ref())),
            other => {
                self.report(
                    m,
                    "L020",
                    super::pattern::span_of(other),
                    "`while ... is` needs a variant pattern".into(),
                );
                (String::new(), span, None)
            }
        };
        let found = variants.as_ref().and_then(|vs| {
            vs.iter()
                .find(|(v, _)| self.symbols[*v as usize].name == vname)
                .cloned()
        });
        if variants.is_some() && found.is_none() && !vname.is_empty() {
            let shown = ty.as_ref().map(|t| self.show(t)).unwrap_or_default();
            let names: Vec<String> = variants
                .iter()
                .flatten()
                .map(|(v, _)| self.symbols[*v as usize].name.clone())
                .collect();
            self.report_help(
                m,
                "L020",
                vspan,
                format!("`{vname}` is not a variant of `{shown}`"),
                alternatives(&names),
            );
        }
        cx.scopes.push(Vec::new());
        let mut payload = None;
        let mut ok = found.is_some();
        if let Some((_, payload_ty)) = &found {
            match (payload_ty, inner) {
                (None, None) | (Some(_), Some(Pattern::Wild(_))) => {}
                (None, Some(_)) => {
                    self.report(
                        m,
                        "L020",
                        vspan,
                        format!("`{vname}` has no payload; write it bare"),
                    );
                    ok = false;
                }
                (Some(_), None) => {
                    self.report(
                        m,
                        "L028",
                        vspan,
                        format!("`{vname}` has a payload; write `{vname}(name)` or `{vname}(_)`"),
                    );
                    ok = false;
                }
                (Some(t), Some(Pattern::Bind(s, n))) => {
                    payload = Some(self.bind_local(cx, n, *s, t.clone(), false));
                }
                (Some(_), Some(other)) => {
                    self.not_yet(
                        m,
                        super::pattern::span_of(other),
                        "a nested pattern in `while ... is`",
                    );
                    ok = false;
                }
            }
        }
        let (body_tir, _) = self.block(cx, body);
        cx.scopes.pop();
        if reaches_end(body, name, false) == Some(false) {
            self.report(
                m,
                "L030",
                word_at(span, "while".len()),
                format!(
                    "a path through this `while` returns to the condition without `set {name}`"
                ),
            );
        }
        let (id, (variant, _)) = (id?, found?);
        ok.then_some(TirStmt::WhileIs {
            scrutinee: id,
            variant,
            payload,
            body: body_tir,
        })
    }

    // ---- matches

    fn stmt_match(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        scrutinee: &Expr,
        arms: &[Arm],
    ) -> (Option<TirStmt>, bool) {
        let s = self.synth(cx, scrutinee);
        let mut all_diverge = true;
        let mut out = Vec::new();
        let mut pats = Vec::new();
        let mut ok = s.is_some();
        for arm in arms {
            cx.scopes.push(Vec::new());
            let pat = s
                .as_ref()
                .and_then(|s| self.pattern(cx, &arm.pattern, &s.ty.clone()));
            let ArmBody::StmtArm(stmts) = &arm.body else {
                cx.scopes.pop();
                continue;
            };
            let (body, diverges) = self.block(cx, stmts);
            cx.scopes.pop();
            all_diverge &= diverges;
            if !diverges {
                self.report(
                    cx.m,
                    "L012",
                    super::pattern::span_of(&arm.pattern),
                    "this arm can fall off the `match` without returning".into(),
                );
            }
            match pat {
                Some(pat) => {
                    pats.push((super::pattern::span_of(&arm.pattern), pat.clone()));
                    out.push(TirArm {
                        span: arm.span,
                        pat,
                        body: TirArmBody::Stmts(body),
                    });
                }
                None => ok = false,
            }
        }
        let Some(s) = s else {
            return (None, all_diverge);
        };
        if ok {
            self.exhaustive(cx.m, word_at(span, "match".len()), &s.ty, &pats);
        }
        (ok.then_some(TirStmt::Match(s, out)), all_diverge)
    }

    fn match_expr(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        scrutinee: &Expr,
        arms: &[Arm],
        expected: Option<&Type>,
    ) -> Option<TirExpr> {
        let s = self.synth(cx, scrutinee);
        let mut pats = Vec::new();
        let mut bodies: Vec<(Option<TirPat>, Span, Result<TirExpr, &Expr>)> = Vec::new();
        let mut ok = s.is_some();
        for arm in arms {
            cx.scopes.push(Vec::new());
            let pat = s
                .as_ref()
                .and_then(|s| self.pattern(cx, &arm.pattern, &s.ty.clone()));
            if let Some(p) = &pat {
                pats.push((super::pattern::span_of(&arm.pattern), p.clone()));
            } else {
                ok = false;
            }
            let ArmBody::ExprArm(e) = &arm.body else {
                cx.scopes.pop();
                continue;
            };
            let body = match expected {
                Some(t) => self.check(cx, e, t).ok_or(e),
                None if flexible(e) => Err(e),
                None => self.synth(cx, e).ok_or(e),
            };
            cx.scopes.pop();
            bodies.push((pat, arm.span, body));
        }
        // Arm join: non-literal arms agree on T; waiting literals then check against T
        let joined = if let Some(t) = expected {
            Some(t.clone())
        } else {
            let mut t: Option<Type> = None;
            for (_, _, body) in &bodies {
                if let Ok(e) = body {
                    match &t {
                        None => t = Some(e.ty.clone()),
                        Some(prev) if *prev != e.ty => {
                            let summary = format!(
                                "match arms have different types: `{}` and `{}`",
                                self.show(prev),
                                self.show(&e.ty)
                            );
                            self.report(cx.m, "L020", e.span, summary);
                            ok = false;
                        }
                        Some(_) => {}
                    }
                }
            }
            let waiting: Vec<&Expr> = bodies
                .iter()
                .filter_map(|(_, _, b)| b.as_ref().err().copied())
                .filter(|e| flexible(e))
                .collect();
            match t {
                Some(t) => Some(t),
                None if waiting.iter().all(|e| literal_int(e).is_some()) => Some(Type::Int),
                None if waiting.iter().all(|e| matches!(e, Expr::Text(..))) => Some(Type::Text),
                None => {
                    self.report(
                        cx.m,
                        "L020",
                        span,
                        "match arms mix integer and text literals".into(),
                    );
                    None
                }
            }
        };
        let t = joined?;
        let mut arms_out = Vec::new();
        for (pat, arm_span, body) in bodies {
            let e = match body {
                Ok(e) => Some(e),
                Err(e) if expected.is_none() && flexible(e) => self.check(cx, e, &t),
                Err(_) => None,
            };
            match (pat, e) {
                (Some(pat), Some(e)) => arms_out.push(TirArm {
                    span: arm_span,
                    pat,
                    body: TirArmBody::Expr(e),
                }),
                _ => ok = false,
            }
        }
        let s = s?;
        if ok {
            self.exhaustive(cx.m, word_at(span, "match".len()), &s.ty, &pats);
        }
        ok.then(|| tx(t, span, TirKind::Match(bx(s), arms_out)))
    }

    // ---- expressions

    /// `check(e, T)`.
    #[allow(clippy::too_many_lines)]
    pub(super) fn check(&mut self, cx: &mut FnCx, e: &Expr, t: &Type) -> Option<TirExpr> {
        if let Some((id, call)) = self.macro_call(cx, e) {
            return self.splice_expr(cx, e, id, call, Some(t));
        }
        if let Some(v) = literal_int(e) {
            return self.literal(cx, e.span(), v, t);
        }
        match e {
            Expr::Text(span, parts) => {
                if types::is_textual(&self.symbols, t) {
                    self.text(cx, *span, parts, t.clone())
                } else {
                    let shown = self.show(t);
                    self.report(
                        cx.m,
                        "L020",
                        *span,
                        format!("expected `{shown}`, found a string literal"),
                    );
                    None
                }
            }
            Expr::ListLit(span, items, _) => match t {
                Type::List(elem) => {
                    let items: Option<Vec<TirExpr>> = items
                        .iter()
                        .map(|i| self.check(cx, i, elem))
                        .collect::<Vec<_>>()
                        .into_iter()
                        .collect();
                    Some(tx(t.clone(), *span, TirKind::List(items?)))
                }
                _ if items.is_empty() => {
                    let shown = self.show(t);
                    self.report(
                        cx.m,
                        "L020",
                        *span,
                        format!("expected `{shown}`, found `[]`"),
                    );
                    None
                }
                _ => {
                    let s = self.synth(cx, e)?;
                    self.coerce(cx, s, t, e.span(), None)
                }
            },
            Expr::Call(span, callee, args, _) if matches!(callee.as_ref(), Expr::Name(_, n) if n == "Ok" || n == "Err") =>
            {
                let Expr::Name(_, n) = callee.as_ref() else {
                    return None;
                };
                let Type::Result(okt, errt) = t else {
                    let shown = self.show(t);
                    self.report(
                        cx.m,
                        "L020",
                        *span,
                        format!("expected `{shown}`, found `{n}(...)`"),
                    );
                    return None;
                };
                let (variant, payload) = if n == "Ok" {
                    (prelude::OK, okt)
                } else {
                    (prelude::ERR, errt)
                };
                let arg = self.payload_arg(cx, *span, n, args)?;
                let arg = self.check(cx, arg, payload)?;
                Some(tx(
                    t.clone(),
                    *span,
                    TirKind::Variant(variant, Some(bx(arg))),
                ))
            }
            Expr::Match(span, scrutinee, arms, ArmKind::ExprArms) => {
                self.match_expr(cx, *span, scrutinee, arms, Some(t))
            }
            Expr::Comptime(_, inner) if cx.comptime || cx.example => self.check(cx, inner, t),
            Expr::Comptime(_, inner) => {
                let root = self.check(&mut cx.comptime(), inner, t)?;
                self.fold(cx.m, &root, "a `comptime` expression")
            }
            _ => {
                let s = self.synth(cx, e)?;
                let adoption = match e {
                    Expr::Binary(_, BinOp::Add | BinOp::Mul | BinOp::Div | BinOp::Rem, ..) => {
                        Some(false)
                    }
                    Expr::Call(_, callee, ..) if matches!(callee.as_ref(), Expr::Name(_, n) if n == "max" && self.lookup(cx.m, "max") == Some(Binding::PreludeFn(prelude::MAX))) => {
                        Some(false)
                    }
                    Expr::IntIn(..) | Expr::IntAtLeast(..) => Some(true),
                    Expr::OrReturn(_, inner, _)
                        if matches!(inner.as_ref(), Expr::IntIn(..) | Expr::IntAtLeast(..)) =>
                    {
                        Some(true)
                    }
                    _ => None,
                };
                self.coerce(cx, s, t, e.span(), adoption)
            }
        }
    }

    /// A synthesized `s` against expected `t`: identity, elimination, then adoption for the
    /// forms that allow it (`Some(equal_ranges)`), else `L023` or `L020`.
    fn coerce(
        &mut self,
        cx: &FnCx,
        mut s: TirExpr,
        t: &Type,
        span: Span,
        adoption: Option<bool>,
    ) -> Option<TirExpr> {
        if types::subtype(&self.symbols, &s.ty, t) {
            return Some(s);
        }
        let (rs, rt) = (self.int_range(&s.ty), self.int_range(t));
        if let (Some(equal), Some(rs), Some(rt)) = (adoption, rs, rt)
            && (if equal { rs == rt } else { contains(rt, rs) })
        {
            s.ty = t.clone();
            return Some(s);
        }
        if rs.is_some() && rt.is_some() && *t != Type::Int {
            let summary = format!(
                "implicit narrowing from `{}` to `{}`",
                self.show(&s.ty),
                self.show(t)
            );
            self.report_help(cx.m, "L023", span, summary, "legal forms: a literal, unary `-` of a literal, `int_at_least` / `int_in`, or `max` / `+` / `*` / `/` / `%` whose static range already fits".into());
            return None;
        }
        self.mismatch(cx, span, t, &s.ty);
        None
    }

    fn literal(&mut self, cx: &FnCx, span: Span, v: i64, t: &Type) -> Option<TirExpr> {
        match self.int_range(t) {
            Some(r) if contains(r, Range { lo: v, hi: v }) => {
                Some(tx(t.clone(), span, TirKind::Int(v)))
            }
            Some(r) => {
                let summary = format!(
                    "`{v}` is outside `{}` ({} through {})",
                    self.show(t),
                    r.lo,
                    r.hi
                );
                self.report(cx.m, "L024", span, summary);
                None
            }
            None => {
                let shown = self.show(t);
                self.report(
                    cx.m,
                    "L020",
                    span,
                    format!("expected `{shown}`, found an integer literal"),
                );
                None
            }
        }
    }

    fn text(&mut self, cx: &mut FnCx, span: Span, parts: &[TextPart], ty: Type) -> Option<TirExpr> {
        if parts.iter().all(|p| matches!(p, TextPart::Literal(_))) {
            let text: String = parts
                .iter()
                .map(|p| match p {
                    TextPart::Literal(s) => s.as_str(),
                    TextPart::Hole(_) => "",
                })
                .collect();
            return Some(tx(ty, span, TirKind::Text(text)));
        }
        let mut out = Vec::new();
        let mut ok = true;
        for part in parts {
            match part {
                TextPart::Literal(s) if s.is_empty() => {}
                TextPart::Literal(s) => out.push(tx(Type::Text, span, TirKind::Text(s.clone()))),
                TextPart::Hole(e) => match self.synth(cx, e) {
                    Some(h)
                        if types::is_int(&self.symbols, &h.ty)
                            || types::is_textual(&self.symbols, &h.ty) =>
                    {
                        out.push(h);
                    }
                    Some(h) => {
                        let shown = self.show(&h.ty);
                        self.report(
                            cx.m,
                            "L022",
                            e.span(),
                            format!("`{shown}` has no `Text` conversion for an interpolation hole"),
                        );
                        ok = false;
                    }
                    None => ok = false,
                },
            }
        }
        ok.then(|| tx(ty, span, TirKind::Concat(out)))
    }

    /// `synth(e)`.
    #[allow(clippy::too_many_lines)]
    pub(super) fn synth(&mut self, cx: &mut FnCx, e: &Expr) -> Option<TirExpr> {
        let m = cx.m;
        if let Some((id, call)) = self.macro_call(cx, e) {
            return self.splice_expr(cx, e, id, call, None);
        }
        if let Some(v) = literal_int(e) {
            return Some(tx(Type::Int, e.span(), TirKind::Int(v)));
        }
        match e {
            Expr::IntLit(..) => None,
            Expr::UnaryNeg(span, inner) => {
                let a = self.synth(cx, inner)?;
                if !types::is_int(&self.symbols, &a.ty) {
                    let shown = self.show(&a.ty);
                    self.report(
                        m,
                        "L034",
                        *span,
                        format!("unary `-` needs an integer, not `{shown}`"),
                    );
                    return None;
                }
                Some(tx(Type::Int, *span, TirKind::Neg(bx(a))))
            }
            Expr::Text(span, parts) => self.text(cx, *span, parts, Type::Text),
            Expr::BoolLit(span, b) => Some(tx(Type::Bool, *span, TirKind::Bool(*b))),
            Expr::NothingLit(span) => Some(tx(Type::Nothing, *span, TirKind::Nothing)),
            Expr::Name(span, name) => self.name(cx, *span, name),
            Expr::ListLit(span, items, _) => {
                let Some(first) = items.iter().find(|i| !flexible(i)).or(items.first()) else {
                    self.report(m, "L020", *span, "`[]` needs an expected type; annotate the binding, e.g. `let xs: List<Int> = []`".into());
                    return None;
                };
                let elem = self.synth(cx, first)?.ty;
                let list = Type::List(Box::new(elem));
                self.check(cx, e, &list)
            }
            Expr::Call(span, callee, args, _) => self.call(cx, *span, callee, args),
            Expr::Field(span, base, field) => self.field(cx, *span, base, field),
            Expr::Index(span, list, index) => self.index(cx, *span, list, index),
            Expr::Binary(span, op, a, b) => self.binary(cx, *span, *op, a, b),
            Expr::Comptime(_, inner) if cx.comptime || cx.example => self.synth(cx, inner),
            Expr::Comptime(_, inner) => {
                let root = self.synth(&mut cx.comptime(), inner)?;
                self.fold(m, &root, "a `comptime` expression")
            }
            Expr::Quote(span, body) => self.quote(cx, *span, body),
            Expr::Unquote(span, _) => {
                let summary = "`unquote` outside `quote`".to_owned();
                let help = "use the `Expr` directly, or write `quote(... unquote(e) ...)`";
                self.report_help(m, "L066", *span, summary, help.into());
                None
            }
            Expr::OrReturn(span, inner, mapping) => {
                self.or_return(cx, *span, inner, mapping.as_deref())
            }
            Expr::OrFail(span, inner, message) => self.or_fail(cx, *span, inner, message),
            Expr::Match(span, scrutinee, arms, kind) => {
                if *kind == ArmKind::StmtArms {
                    self.report(
                        m,
                        "L020",
                        *span,
                        "a statement `match` has no value; use expression arms".into(),
                    );
                    return None;
                }
                self.match_expr(cx, *span, scrutinee, arms, None)
            }
            Expr::IntIn(span, source, lo, hi) => self.int_in(cx, *span, source, lo, Some(hi)),
            Expr::IntAtLeast(span, source, lo) => self.int_in(cx, *span, source, lo, None),
            Expr::ClockNow(span) => {
                if cx.example || cx.effect != Effect::ReadsClock {
                    self.report(m, "L025", *span, "`clock.now()` needs `reads clock`".into());
                    return None;
                }
                if cx.comptime {
                    let summary = "`clock.now()` is a runtime value; a comptime context cannot read the clock";
                    self.report(m, "L060", *span, summary.into());
                    return None;
                }
                Some(tx(Type::Int, *span, TirKind::ClockNow))
            }
            Expr::FsRead(span, _) if cx.comptime && cx.effect == Effect::ReadsFs => {
                self.not_yet(m, *span, "`fs.read` (it lands with the fs library)");
                None
            }
            Expr::CatalogColumns(span, _) if cx.comptime && cx.effect == Effect::ReadsCatalog => {
                self.not_yet(
                    m,
                    *span,
                    "`catalog.columns` (lingua has no database client yet)",
                );
                None
            }
            Expr::FsRead(span, _) | Expr::CatalogColumns(span, _) => {
                self.report(m, "L025", *span, "`fs.read` and `catalog.columns` need `reads fs` or `reads catalog`, which only a const, a generative type, or a package `comptime` block can declare".into());
                None
            }
        }
    }

    // Resolves the type `id` names, or that variant `id` belongs to, before a use reads its
    // fields or payloads
    fn ensure_parent(&mut self, id: SymbolId) -> bool {
        match self.symbols[id as usize].kind {
            SymbolKind::Variant { parent, .. } => self.ensure(parent),
            _ => self.ensure(id),
        }
    }

    // A use of a folded const: its value at `span`, or quietly nothing when it did not fold.
    // A `Type` is `L063` outside a comptime context.
    fn const_use(&mut self, cx: &FnCx, id: SymbolId, span: Span) -> Option<TirExpr> {
        let Some(value) = self.consts.get(&id) else {
            self.hit_poison.set(true);
            return None;
        };
        if matches!(value.ty, Type::TypeAst | Type::ExprAst) && !cx.comptime {
            let name = self.symbols[id as usize].name.clone();
            let shown = self.show(&value.ty);
            let summary = format!("`{name}` is a `{shown}`, which only exists at compile time");
            self.report(cx.m, "L063", span, summary);
            return None;
        }
        Some(TirExpr {
            span,
            ..value.clone()
        })
    }

    #[allow(clippy::too_many_lines)]
    fn name(&mut self, cx: &mut FnCx, span: Span, name: &str) -> Option<TirExpr> {
        let m = cx.m;
        match self.resolve_name(cx, name) {
            Some(Binding::Local(id)) => {
                if self.is_poisoned(id) {
                    return None;
                }
                if self.consts.contains_key(&id) {
                    return self.const_use(cx, id, span);
                }
                let param = self.symbols[id as usize].kind == SymbolKind::Param;
                if cx.comptime && !(param && cx.macro_body) {
                    self.report_help(
                        m,
                        "L060",
                        span,
                        format!(
                            "`{name}` is a runtime value; a comptime context needs a comptime value"
                        ),
                        "bind it with `const`, or move the computation out of the comptime context"
                            .into(),
                    );
                    return None;
                }
                Some(tx(self.sym_ty(id), span, TirKind::Local(id)))
            }
            Some(Binding::Const(id)) => {
                if !self.ensure(id) {
                    return None;
                }
                self.const_use(cx, id, span)
            }
            Some(Binding::Variant(id)) => {
                if self.is_poisoned(id) || !self.ensure_parent(id) {
                    return None;
                }
                let SymbolKind::Variant {
                    parent, payload, ..
                } = &self.symbols[id as usize].kind
                else {
                    return None;
                };
                if payload.is_some() {
                    self.report(
                        m,
                        "L028",
                        span,
                        format!("`{name}` takes one payload: write `{name}(...)`"),
                    );
                    return None;
                }
                Some(tx(Type::Nominal(*parent), span, TirKind::Variant(id, None)))
            }
            Some(Binding::Function(_) | Binding::PreludeFn(_) | Binding::Log) => {
                self.report(
                    m,
                    "L020",
                    span,
                    format!("`{name}` is a function, not a value; call it"),
                );
                None
            }
            Some(Binding::Type(_) | Binding::Builtin(_)) => {
                self.report(m, "L020", span, format!("`{name}` is a type, not a value"));
                None
            }
            Some(Binding::Resource(_)) => {
                self.report(
                    m,
                    "L020",
                    span,
                    format!("`{name}` is a resource, not a value; call its operation"),
                );
                None
            }
            Some(Binding::Comptime) => {
                self.not_yet(m, span, &format!("the comptime prelude name `{name}`"));
                None
            }
            Some(Binding::Poison) => None,
            None if name == "Ok" || name == "Err" => {
                self.report(
                    m,
                    "L028",
                    span,
                    format!("`{name}` takes one payload: write `{name}(...)`"),
                );
                None
            }
            None => {
                self.unknown(cx, span, name);
                None
            }
        }
    }

    // The one positional argument of a variant payload or a text wrap
    fn payload_arg<'e>(
        &mut self,
        cx: &FnCx,
        span: Span,
        name: &str,
        args: &'e [Arg],
    ) -> Option<&'e Expr> {
        match args {
            [
                Arg {
                    name: None, value, ..
                },
            ] => Some(value),
            [
                Arg {
                    name: Some(_),
                    span: s,
                    ..
                },
            ] => {
                self.report(
                    cx.m,
                    "L028",
                    *s,
                    format!("`{name}(...)` takes its payload positionally, without a name"),
                );
                None
            }
            _ => {
                self.report(
                    cx.m,
                    "L028",
                    span,
                    format!(
                        "`{name}(...)` takes exactly one argument, got {}",
                        args.len()
                    ),
                );
                None
            }
        }
    }

    // Arguments to parameters: names by name, positionals in order, positionals only for a
    // parameter whose type is unique (L021), every parameter once (L028)
    pub(super) fn match_args<'e>(
        &mut self,
        cx: &FnCx,
        span: Span,
        what: &str,
        params: &[(String, Type)],
        args: &'e [Arg],
    ) -> Option<Matched<'e>> {
        let names: Vec<String> = params.iter().map(|(n, _)| n.clone()).collect();
        let mut out: Vec<Option<&Expr>> = vec![None; params.len()];
        let mut order = Vec::with_capacity(args.len());
        let mut ok = true;
        let mut position = 0;
        for arg in args {
            let i = if let Some(n) = &arg.name {
                if let Some(i) = names.iter().position(|p| p == n) {
                    i
                } else {
                    self.report_help(
                        cx.m,
                        "L028",
                        arg.span,
                        format!("`{what}` has no parameter `{n}`"),
                        alternatives(&names),
                    );
                    ok = false;
                    continue;
                }
            } else {
                let i = position;
                position += 1;
                if i >= params.len() {
                    self.report_help(
                        cx.m,
                        "L028",
                        arg.span,
                        format!(
                            "`{what}` takes {} argument{}, got {}",
                            params.len(),
                            if params.len() == 1 { "" } else { "s" },
                            args.len()
                        ),
                        alternatives(&names),
                    );
                    ok = false;
                    continue;
                }
                let ty = &params[i].1;
                if params.iter().filter(|(_, t)| t == ty).count() > 1 && ok {
                    let summary = format!(
                        "positional argument for `{}`, whose type `{}` is not unique in `{what}`; name it",
                        params[i].0,
                        self.show(ty)
                    );
                    self.report_help(cx.m, "L021", arg.span, summary, alternatives(&names));
                    ok = false;
                }
                i
            };
            if out[i].is_some() {
                self.report(
                    cx.m,
                    "L028",
                    arg.span,
                    format!("`{}` is given twice", params[i].0),
                );
                ok = false;
            }
            out[i] = Some(&arg.value);
            order.push(i);
        }
        for (i, slot) in out.iter().enumerate() {
            if slot.is_none() && ok {
                self.report_help(
                    cx.m,
                    "L028",
                    span,
                    format!("`{what}` is missing argument `{}`", params[i].0),
                    alternatives(&names),
                );
                ok = false;
            }
        }
        ok.then_some(Matched { args: out, order })
    }

    fn checked_args(
        &mut self,
        cx: &mut FnCx,
        matched: &Matched,
        params: &[(String, Type)],
    ) -> Option<Vec<TirExpr>> {
        let args: Vec<Option<TirExpr>> = matched
            .args
            .iter()
            .zip(params)
            .map(|(a, (_, t))| a.and_then(|a| self.check(cx, a, t)))
            .collect();
        args.into_iter().collect()
    }

    // `make` builds the call from `args`, which are in parameter order. Arguments are evaluated
    // in source order (lingua1.md), so if that would move one with effects past another, the
    // non-trivial ones are bound to temporaries in source order first.
    fn in_source_order(
        &mut self,
        cx: &FnCx,
        order: &[usize],
        args: Vec<TirExpr>,
        make: impl FnOnce(Vec<TirExpr>) -> TirExpr,
    ) -> TirExpr {
        let trivial = |e: &TirExpr| {
            matches!(
                e.kind,
                TirKind::Int(_)
                    | TirKind::Text(_)
                    | TirKind::Bool(_)
                    | TirKind::Nothing
                    | TirKind::Local(_)
            )
        };
        let moved: Vec<usize> = order
            .iter()
            .copied()
            .filter(|&i| !trivial(&args[i]))
            .collect();
        if moved.is_sorted() {
            return make(args);
        }
        let mut args = args;
        let mut binds = Vec::with_capacity(moved.len());
        for i in moved {
            let (ty, span) = (args[i].ty.clone(), args[i].span);
            let id = self.fresh(cx, "arg", ty.clone(), span);
            binds.push((
                id,
                std::mem::replace(&mut args[i], tx(ty, span, TirKind::Local(id))),
            ));
        }
        let call = make(args);
        tx(call.ty.clone(), call.span, TirKind::Let(binds, bx(call)))
    }

    pub(super) fn effect_check(
        &mut self,
        cx: &FnCx,
        span: Span,
        what: &str,
        effect: &Effect,
        module: usize,
    ) -> bool {
        if cx.example {
            self.report(
                cx.m,
                "L025",
                span,
                format!(
                    "an example may call only prelude functions and constructors, not `{what}`"
                ),
            );
            return false;
        }
        if may_call(&cx.effect, cx.m, effect, module) {
            if cx.comptime && *effect != Effect::Pure {
                self.report(
                    cx.m,
                    "L060",
                    span,
                    format!(
                        "`{what}` is `{}`, which needs a runtime resource; a comptime context can call only `pure` functions",
                        effect_text(effect)
                    ),
                );
                return false;
            }
            return true;
        }
        let summary = format!(
            "a `{}` function cannot call `{what}`, which is `{}`",
            effect_text(&cx.effect),
            effect_text(effect)
        );
        self.report(cx.m, "L025", span, summary);
        false
    }

    #[allow(clippy::too_many_lines)]
    fn call(&mut self, cx: &mut FnCx, span: Span, callee: &Expr, args: &[Arg]) -> Option<TirExpr> {
        let m = cx.m;
        if let Expr::Field(_, base, op) = callee
            && let Expr::Name(rspan, r) = base.as_ref()
            && let Some(Binding::Resource(rid)) = self.resolve_name(cx, r)
        {
            return self.resource_call(cx, span, *rspan, rid, r, op, args);
        }
        let Expr::Name(nspan, name) = callee else {
            self.report(
                m,
                "L020",
                callee.span(),
                "only a named function, constructor, or resource operation can be called".into(),
            );
            return None;
        };
        let binding = self.resolve_name(cx, name);
        match binding {
            None if name == "Ok" || name == "Err" => {
                self.report(m, "L020", span, format!("`{name}(...)` needs an expected `Result` type; annotate the binding or return it"));
                None
            }
            None => {
                self.unknown(cx, *nspan, name);
                None
            }
            Some(Binding::Poison) => None,
            Some(Binding::Log) => {
                let params = [("message".to_owned(), Type::Text)];
                let matched = self.match_args(cx, span, "log", &params, args)?;
                let mut args = self.checked_args(cx, &matched, &params)?;
                Some(tx(Type::Nothing, span, TirKind::Log(bx(args.remove(0)))))
            }
            Some(Binding::PreludeFn(id)) => self.prelude_call(cx, span, id, name, args),
            Some(Binding::Function(id)) => {
                if !self.ensure(id) {
                    return None;
                }
                let sig = self.sigs.get(&id)?.clone();
                if matches!(sig.ret, Type::ExprAst | Type::TypeAst) && !cx.comptime && !cx.example {
                    let shown = self.show(&sig.ret);
                    let summary =
                        format!("`{name}` returns `{shown}`, which only exists at compile time");
                    let help = "call it in a comptime context: a `const`, a generative type, or a package example";
                    self.report_help(m, "L063", span, summary, help.into());
                    return None;
                }
                let matched = self.match_args(cx, span, name, &sig.params, args);
                let effect_ok = self.effect_check(cx, *nspan, name, &sig.effect, sig.module);
                let matched = matched?;
                let args = self.checked_args(cx, &matched, &sig.params)?;
                (effect_ok && !self.is_poisoned(id)).then(|| {
                    self.in_source_order(cx, &matched.order, args, |args| {
                        tx(sig.ret, span, TirKind::Call(id, args))
                    })
                })
            }
            Some(Binding::Type(id) | Binding::Variant(id)) if !self.ensure_parent(id) => None,
            Some(Binding::Type(id)) => match self.symbols[id as usize].kind.clone() {
                SymbolKind::Type(TypeKind::Product) => {
                    let fields = self.fields.get(&id)?.clone();
                    let matched = self.match_args(cx, span, name, &fields, args)?;
                    let args = self.checked_args(cx, &matched, &fields)?;
                    (!self.is_poisoned(id)).then(|| {
                        self.in_source_order(cx, &matched.order, args, |args| {
                            tx(Type::Nominal(id), span, TirKind::Record(id, args))
                        })
                    })
                }
                SymbolKind::Type(TypeKind::Newtype {
                    base: Type::Text, ..
                }) => {
                    let arg = self.payload_arg(cx, span, name, args)?;
                    let mut e = self.check(cx, arg, &Type::Text)?;
                    e.ty = Type::Nominal(id);
                    e.span = span;
                    Some(e)
                }
                SymbolKind::Type(TypeKind::Newtype { .. }) => {
                    let help = "use `int_in(...)` or `int_at_least(...)`, or a literal where the type is expected".to_owned();
                    self.report_help(
                        m,
                        "L023",
                        span,
                        format!("there is no `{name}(...)` conversion for an integer type"),
                        help,
                    );
                    None
                }
                _ => {
                    self.report(
                        m,
                        "L020",
                        *nspan,
                        format!("`{name}` is a sum; construct one of its variants"),
                    );
                    None
                }
            },
            Some(Binding::Variant(id)) => {
                let SymbolKind::Variant {
                    parent, payload, ..
                } = self.symbols[id as usize].kind.clone()
                else {
                    return None;
                };
                let Some(payload) = payload else {
                    if self.is_poisoned(id) {
                        return None;
                    }
                    self.report(
                        m,
                        "L028",
                        span,
                        format!("`{name}` takes no payload; write it bare"),
                    );
                    return None;
                };
                let arg = self.payload_arg(cx, span, name, args)?;
                let arg = self.check(cx, arg, &payload)?;
                Some(tx(
                    Type::Nominal(parent),
                    span,
                    TirKind::Variant(id, Some(bx(arg))),
                ))
            }
            Some(Binding::Comptime)
                if name == "http_get" && cx.comptime && cx.effect == Effect::ReadsNet =>
            {
                self.not_yet(m, *nspan, "`http_get` (lingua has no HTTPS client yet)");
                None
            }
            Some(Binding::Comptime) if name == "http_get" => {
                self.report(m, "L025", *nspan, "`http_get` needs `reads net`, which only a const or a generative type can declare".into());
                None
            }
            Some(Binding::Comptime) => {
                self.not_yet(
                    m,
                    *nspan,
                    &format!("the comptime prelude function `{name}`"),
                );
                None
            }
            Some(
                Binding::Local(_) | Binding::Builtin(_) | Binding::Resource(_) | Binding::Const(_),
            ) => {
                self.report(m, "L020", *nspan, format!("`{name}` is not a function"));
                None
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resource_call(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        rspan: Span,
        rid: SymbolId,
        r: &str,
        op: &str,
        args: &[Arg],
    ) -> Option<TirExpr> {
        let op_id = *self.resource_ops.get(&rid)?;
        let op_name = self.symbols[op_id as usize].name.clone();
        if op_name != op {
            let summary = format!("resource `{r}` has no operation `{op}`");
            self.report_help(cx.m, "L011", span, summary, alternatives(&[op_name]));
            return None;
        }
        let sig = self.sigs.get(&op_id)?.clone();
        let what = format!("{r}.{op}");
        let matched = self.match_args(cx, span, &what, &sig.params, args);
        let effect_ok = self.effect_check(cx, rspan, &what, &sig.effect, sig.module);
        let args = self.checked_args(cx, &matched?, &sig.params)?;
        if !effect_ok || self.is_poisoned(op_id) {
            return None;
        }
        let d = self.diag(
            cx.m,
            "L072",
            span,
            format!(
                "`{what}` has no runtime backend; a built binary cannot call a resource operation"
            ),
        );
        self.resource_calls.push((self.key(cx.m).to_owned(), d));
        Some(tx(sig.ret, span, TirKind::ResourceCall(op_id, args)))
    }

    #[allow(clippy::too_many_lines)]
    fn prelude_call(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        id: SymbolId,
        name: &str,
        args: &[Arg],
    ) -> Option<TirExpr> {
        let m = cx.m;
        let text = || ("text".to_owned(), Type::Text);
        let any_list = Type::List(Box::new(Type::Nothing));
        match id {
            prelude::TRIM | prelude::LOWER => {
                let params = [text()];
                let matched = self.match_args(cx, span, name, &params, args)?;
                let args = self.checked_args(cx, &matched, &params)?;
                Some(tx(Type::Text, span, TirKind::Call(id, args)))
            }
            prelude::SPLIT => {
                let params = [text(), ("separator".to_owned(), Type::Text)];
                let matched = self.match_args(cx, span, name, &params, args)?;
                if let Some(sep) = matched.args[1]
                    && hole_free(sep).is_some_and(|s| s.is_empty())
                {
                    self.report(
                        m,
                        "L046",
                        sep.span(),
                        "`split` on an empty separator".into(),
                    );
                    return None;
                }
                let args = self.checked_args(cx, &matched, &params)?;
                let ty = Type::List(Box::new(Type::Text));
                Some(self.in_source_order(cx, &matched.order, args, |args| {
                    tx(ty, span, TirKind::Call(id, args))
                }))
            }
            prelude::LENGTH => {
                let params = [("list".to_owned(), any_list)];
                let matched = self.match_args(cx, span, name, &params, args)?;
                let list = self.synth(cx, matched.args[0]?)?;
                if !matches!(list.ty, Type::List(_)) {
                    let shown = self.show(&list.ty);
                    self.report(
                        m,
                        "L020",
                        list.span,
                        format!("`length` needs a `List`, not `{shown}`"),
                    );
                    return None;
                }
                Some(tx(
                    Type::Bound {
                        lo: 0,
                        hi: i64::MAX,
                    },
                    span,
                    TirKind::Call(id, vec![list]),
                ))
            }
            prelude::MAX => {
                let params = [("a".to_owned(), Type::Int), ("b".to_owned(), Type::Int)];
                let matched = self.match_args(cx, span, name, &params, args)?;
                let args = self.checked_args(cx, &matched, &params)?;
                let (a, b) = (self.expr_range(&args[0])?, self.expr_range(&args[1])?);
                let r = Range {
                    lo: a.lo.max(b.lo),
                    hi: a.hi.max(b.hi),
                };
                Some(self.in_source_order(cx, &matched.order, args, |args| {
                    tx(types::of_range(r), span, TirKind::Call(id, args))
                }))
            }
            prelude::JOIN => {
                let params = [
                    ("left".to_owned(), any_list.clone()),
                    ("right".to_owned(), any_list),
                ];
                let matched = self.match_args(cx, span, name, &params, args)?;
                let (left, right) = (matched.args[0]?, matched.args[1]?);
                let empty = |e: &Expr| matches!(e, Expr::ListLit(_, items, _) if items.is_empty());
                let (first, second, swapped) = match (empty(left), empty(right)) {
                    (true, true) => {
                        self.report(
                            m,
                            "L020",
                            span,
                            "`join` of two `[]` has no element type; annotate one side".into(),
                        );
                        return None;
                    }
                    (true, false) => (right, left, true),
                    _ => (left, right, false),
                };
                let a = self.synth(cx, first)?;
                if !matches!(a.ty, Type::List(_)) {
                    let shown = self.show(&a.ty);
                    self.report(m, "L020", a.span, format!("`join` needs lists, not `{shown}`; it is list concatenation, not text join"));
                    return None;
                }
                let ty = a.ty.clone();
                let b = self.check(cx, second, &ty)?;
                let args = if swapped { vec![b, a] } else { vec![a, b] };
                Some(self.in_source_order(cx, &matched.order, args, |args| {
                    tx(ty, span, TirKind::Call(id, args))
                }))
            }
            prelude::RECORD_FROM_COLUMNS
            | prelude::RECORD_FROM_FIELDS
            | prelude::SUM_FROM_VARIANTS => {
                let (param, item) = match id {
                    prelude::RECORD_FROM_COLUMNS => ("columns", prelude::COLUMN),
                    prelude::RECORD_FROM_FIELDS => ("fields", prelude::RECORD_FIELD),
                    _ => ("variants", prelude::SUM_VARIANT),
                };
                let params = [(param.to_owned(), Type::List(Box::new(Type::Nominal(item))))];
                let matched = self.match_args(cx, span, name, &params, args)?;
                let args = self.checked_args(cx, &matched, &params)?;
                if !cx.comptime && !cx.example {
                    let summary =
                        format!("`{name}` returns a `Type`, which only exists at compile time");
                    self.report_help(
                        m,
                        "L063",
                        span,
                        summary,
                        "call it in a generative type's body or a `provide`".into(),
                    );
                    return None;
                }
                Some(tx(Type::TypeAst, span, TirKind::Call(id, args)))
            }
            _ => None,
        }
    }

    fn field(&mut self, cx: &mut FnCx, span: Span, base: &Expr, field: &str) -> Option<TirExpr> {
        let b = self.synth(cx, base)?;
        let Type::Nominal(id) = b.ty else {
            let shown = self.show(&b.ty);
            self.report(
                cx.m,
                "L020",
                span,
                format!("field access needs a product, not `{shown}`"),
            );
            return None;
        };
        let Some(fields) = self.fields.get(&id).cloned() else {
            let shown = self.show(&b.ty);
            self.report(
                cx.m,
                "L020",
                span,
                format!("field access needs a product, not `{shown}`"),
            );
            return None;
        };
        let Some(index) = fields.iter().position(|(n, _)| n == field) else {
            let names: Vec<String> = fields.iter().map(|(n, _)| n.clone()).collect();
            let summary = format!("`{}` has no field `{field}`", self.show(&b.ty));
            self.report_help(cx.m, "L011", span, summary, alternatives(&names));
            return None;
        };
        let ty = fields[index].1.clone();
        Some(tx(
            ty,
            span,
            TirKind::Field(bx(b), u32::try_from(index).unwrap_or(u32::MAX)),
        ))
    }

    fn index(&mut self, cx: &mut FnCx, span: Span, list: &Expr, index: &Expr) -> Option<TirExpr> {
        let l = self.synth(cx, list);
        let i = self.synth(cx, index);
        let (l, i) = (l?, i?);
        let Type::List(elem) = l.ty.clone() else {
            let shown = self.show(&l.ty);
            self.report(
                cx.m,
                "L020",
                list.span(),
                format!("indexing needs a `List`, not `{shown}`"),
            );
            return None;
        };
        let Some(r) = self.expr_range(&i) else {
            let shown = self.show(&i.ty);
            self.report(
                cx.m,
                "L020",
                index.span(),
                format!("an index must be an integer, not `{shown}`"),
            );
            return None;
        };
        if let Expr::ListLit(_, items, _) = list {
            let top = i64::try_from(items.len()).unwrap_or(i64::MAX) - 1;
            if contains(Range { lo: 0, hi: top }, r) {
                return Some(tx(
                    *elem,
                    span,
                    TirKind::Index {
                        list: bx(l),
                        index: bx(i),
                        checked: false,
                    },
                ));
            }
            if r.hi < 0 || r.lo > top {
                self.report(
                    cx.m,
                    "L024",
                    index.span(),
                    format!("index is outside the list literal's 0 through {top}"),
                );
                return None;
            }
        }
        let ty = Type::Result(elem, Box::new(Type::Nominal(prelude::BOUNDS_ERROR)));
        Some(tx(
            ty,
            span,
            TirKind::Index {
                list: bx(l),
                index: bx(i),
                checked: true,
            },
        ))
    }

    // Synthesizes both sides of a comparison. A flexible literal adopts a nominal other side;
    // against `Int` or an anonymous bound it stays `Int`, since those compare after elimination.
    fn operands(&mut self, cx: &mut FnCx, a: &Expr, b: &Expr) -> Option<(TirExpr, TirExpr)> {
        let nominal = |_: &Self, t: &Type| matches!(t, Type::Nominal(_));
        match (flexible(a), flexible(b)) {
            (true, false) => {
                let y = self.synth(cx, b)?;
                let x = if nominal(self, &y.ty) {
                    self.check(cx, a, &y.ty)?
                } else {
                    self.synth(cx, a)?
                };
                Some((x, y))
            }
            (false, true) => {
                let x = self.synth(cx, a)?;
                let y = if nominal(self, &x.ty) {
                    self.check(cx, b, &x.ty)?
                } else {
                    self.synth(cx, b)?
                };
                Some((x, y))
            }
            _ => {
                let x = self.synth(cx, a);
                let y = self.synth(cx, b);
                x.zip(y)
            }
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn binary(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        op: BinOp,
        a: &Expr,
        b: &Expr,
    ) -> Option<TirExpr> {
        let m = cx.m;
        let sym = match op {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
        };
        if let BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem = op {
            let x = self.synth(cx, a);
            let y = self.synth(cx, b);
            let (x, y) = (x?, y?);
            let (Some(rx), Some(ry)) = (self.expr_range(&x), self.expr_range(&y)) else {
                let summary = format!(
                    "`{sym}` needs integer operands, not `{}` and `{}`",
                    self.show(&x.ty),
                    self.show(&y.ty)
                );
                self.report(m, "L034", span, summary);
                return None;
            };
            if matches!(op, BinOp::Div | BinOp::Rem) && literal_int(b) == Some(0) {
                self.report(m, "L076", b.span(), "division by zero".into());
                return None;
            }
            let ty = if op == BinOp::Sub {
                Type::Int
            } else {
                types::arith(op, rx, ry)
            };
            let kind = match op {
                BinOp::Add => TirKind::Add(bx(x), bx(y)),
                BinOp::Sub => TirKind::Sub(bx(x), bx(y)),
                BinOp::Mul => TirKind::Mul(bx(x), bx(y)),
                BinOp::Div => TirKind::Div(bx(x), bx(y)),
                _ => TirKind::Rem(bx(x), bx(y)),
            };
            return Some(tx(ty, span, kind));
        }
        let (x, y) = self.operands(cx, a, b)?;
        let s = &self.symbols;
        let ints = types::is_int(s, &x.ty) && types::is_int(s, &y.ty);
        let nominal = types::is_nominal_int(s, &x.ty) || types::is_nominal_int(s, &y.ty);
        let legal = match op {
            BinOp::Eq | BinOp::Ne => {
                (x.ty == y.ty
                    && (x.ty == Type::Bool
                        || types::is_int(s, &x.ty)
                        || types::is_textual(s, &x.ty)))
                    || (ints && !nominal)
            }
            _ => ints && (!nominal || x.ty == y.ty),
        };
        if !legal {
            let summary = format!(
                "`{sym}` does not apply to `{}` and `{}`",
                self.show(&x.ty),
                self.show(&y.ty)
            );
            self.report(m, "L034", span, summary);
            return None;
        }
        let cmp = match op {
            BinOp::Eq => CmpOp::Eq,
            BinOp::Ne => CmpOp::Ne,
            BinOp::Lt => CmpOp::Lt,
            BinOp::Gt => CmpOp::Gt,
            BinOp::Le => CmpOp::Le,
            _ => CmpOp::Ge,
        };
        Some(tx(Type::Bool, span, TirKind::Cmp(cmp, bx(x), bx(y))))
    }

    fn result_of(
        &mut self,
        cx: &mut FnCx,
        inner: &Expr,
        what: &str,
    ) -> Option<(TirExpr, Type, Type)> {
        let e = self.synth(cx, inner)?;
        match e.ty.clone() {
            Type::Result(s, err) => Some((e, *s, *err)),
            other => {
                let shown = self.show(&other);
                self.report(
                    cx.m,
                    "L020",
                    inner.span(),
                    format!("`{what}` needs a `Result`, not `{shown}`"),
                );
                None
            }
        }
    }

    fn ok_arm(&mut self, cx: &FnCx, span: Span, s: &Type) -> TirArm {
        let v = self.fresh(cx, "ok", s.clone(), span);
        TirArm {
            span,
            pat: TirPat::Variant(prelude::OK, Some(Box::new(TirPat::Bind(v)))),
            body: TirArmBody::Expr(tx(s.clone(), span, TirKind::Local(v))),
        }
    }

    fn or_return(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        inner: &Expr,
        mapping: Option<&Expr>,
    ) -> Option<TirExpr> {
        let (e, s, err) = self.result_of(cx, inner, "or return")?;
        let ret = cx.ret.clone();
        let err_arm = match mapping {
            None => {
                let fn_err = match &ret {
                    Type::Result(_, fe) => Some(fe.as_ref().clone()),
                    _ => None,
                };
                if fn_err.as_ref() != Some(&err) {
                    let theirs = fn_err.map_or_else(
                        || format!("the function returns `{}`", self.show(&ret)),
                        |fe| format!("`{}` in the function's return type", self.show(&fe)),
                    );
                    let summary = format!(
                        "`or return` across different error types: `{}` here, {theirs}",
                        self.show(&err)
                    );
                    self.report_help(
                        cx.m,
                        "L026",
                        span,
                        summary,
                        "map the error: `or return Err(...)`".into(),
                    );
                    return None;
                }
                let id = self.fresh(cx, "err", err.clone(), span);
                let value = tx(
                    ret.clone(),
                    span,
                    TirKind::Variant(prelude::ERR, Some(bx(tx(err, span, TirKind::Local(id))))),
                );
                TirArm {
                    span,
                    pat: TirPat::Variant(prelude::ERR, Some(Box::new(TirPat::Bind(id)))),
                    body: TirArmBody::Stmts(vec![TirStmt::Return(value)]),
                }
            }
            Some(mapped) => {
                let value = self.check(cx, mapped, &ret)?;
                TirArm {
                    span,
                    pat: TirPat::Variant(prelude::ERR, Some(Box::new(TirPat::Wild))),
                    body: TirArmBody::Stmts(vec![TirStmt::Return(value)]),
                }
            }
        };
        let ok = self.ok_arm(cx, span, &s);
        Some(tx(s, span, TirKind::Match(bx(e), vec![ok, err_arm])))
    }

    fn or_fail(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        inner: &Expr,
        message: &Expr,
    ) -> Option<TirExpr> {
        let result = self.result_of(cx, inner, "or fail");
        let message_text = self.message(cx, message, "an `or fail` message");
        let ((e, s, _), message_text) = (result?, message_text?);
        // The report points at the `or fail` keywords
        let source = self.source(cx.m);
        let after = Span {
            start: inner.span().end,
            end: span.end,
        };
        let or = find_word(source, after, "or");
        let fail = find_word(
            source,
            Span {
                start: or.end,
                end: span.end,
            },
            "fail",
        );
        let keywords = Span {
            start: or.start,
            end: fail.end,
        };
        let ok = self.ok_arm(cx, span, &s);
        let err_arm = TirArm {
            span,
            pat: TirPat::Variant(prelude::ERR, Some(Box::new(TirPat::Wild))),
            body: TirArmBody::Stmts(vec![TirStmt::Fail {
                message: message_text,
                span: keywords,
            }]),
        };
        Some(tx(s, span, TirKind::Match(bx(e), vec![ok, err_arm])))
    }

    fn int_in(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        source: &Expr,
        lo: &Expr,
        hi: Option<&Expr>,
    ) -> Option<TirExpr> {
        let src = self.synth(cx, source);
        let lo_v = self.fold_bound(cx.m, lo, &cx.scopes);
        let hi_v = match hi {
            Some(hi) => self.fold_bound(cx.m, hi, &cx.scopes),
            None => Some(i64::MAX),
        };
        let (src, lo_v, hi_v) = (src?, lo_v?, hi_v?);
        if lo_v > hi_v {
            self.report(
                cx.m,
                "L024",
                span,
                format!("empty range: {lo_v} is greater than {hi_v}"),
            );
            return None;
        }
        let s = &self.symbols;
        if !types::is_textual(s, &src.ty) && !types::is_int(s, &src.ty) {
            let shown = self.show(&src.ty);
            self.report(
                cx.m,
                "L020",
                source.span(),
                format!("the source must be `Text` or an integer, not `{shown}`"),
            );
            return None;
        }
        if let Some(v) = literal_int(source)
            && !(lo_v..=hi_v).contains(&v)
        {
            self.report(
                cx.m,
                "L024",
                source.span(),
                format!("`{v}` is already outside {lo_v} through {hi_v}"),
            );
            return None;
        }
        let ok = types::of_range(Range { lo: lo_v, hi: hi_v });
        let ty = Type::Result(Box::new(ok), Box::new(Type::Nominal(prelude::BOUNDS_ERROR)));
        let kind = match hi {
            Some(_) => TirKind::IntIn(bx(src), lo_v, hi_v),
            None => TirKind::IntAtLeast(bx(src), lo_v),
        };
        Some(tx(ty, span, kind))
    }
}
