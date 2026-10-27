//! Quoted trees (docs/lingua1.md, Macros): a mutable walk over statements and expressions,
//! and the spellings the checker gives names inside a quote template so a splice keeps the
//! macro's meaning in the caller's body.
//!
//! A template name is a plain `String`, so the hygiene marks are characters no source
//! identifier can hold:
//! - `name\u{1}M` is the file-level `name` of module `M` (a free name, resolved at the macro).
//! - `name\u{2}` is a binder the quote introduces; each splice makes it `name\u{2}N`, fresh.
//! - `\u{3}I` is the `I`th `unquote` hole of a template.

use super::{Arg, Arm, ArmBody, Expr, Pattern, QuoteBody, Span, Stmt, TextPart, TypeExpr};

/// Marks a free name qualified with its module index.
pub const QUALIFIED: char = '\u{1}';
/// Marks a binder the quote introduces.
pub const BINDER: char = '\u{2}';
/// Starts an `unquote` hole's name.
pub const HOLE: char = '\u{3}';

/// What a walk does at each node. Every hook defaults to doing nothing.
pub trait VisitMut {
    /// Before an expression's children; `true` skips them.
    fn expr(&mut self, _e: &mut Expr) -> bool {
        false
    }
    /// Before a statement's children; `true` skips them.
    fn stmt(&mut self, _s: &mut Stmt) -> bool {
        false
    }
    fn span(&mut self, _span: &mut Span) {}
    /// A use of a name: an expression name, a `set` target, a variant pattern, a named type.
    fn name(&mut self, _name: &mut String, _span: Span) {}
    /// A name a statement or a pattern binds. A bare pattern ident may still be a variant.
    fn binder(&mut self, _name: &mut String, _span: Span, _pattern: bool) {}
}

pub fn body_mut(v: &mut impl VisitMut, body: &mut QuoteBody) {
    match body {
        QuoteBody::QuotedStmt(s) => stmt_mut(v, s),
        QuoteBody::QuotedExpr(e) => expr_mut(v, e),
    }
}

fn stmts_mut(v: &mut impl VisitMut, stmts: &mut [Stmt]) {
    for s in stmts {
        stmt_mut(v, s);
    }
}

#[allow(clippy::too_many_lines)]
pub fn stmt_mut(v: &mut impl VisitMut, s: &mut Stmt) {
    if v.stmt(s) {
        return;
    }
    match s {
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
        }
        | Stmt::Const {
            span,
            name,
            ty,
            init,
        } => {
            if let Some(ty) = ty {
                type_mut(v, ty);
            }
            expr_mut(v, init);
            v.binder(name, *span, false);
            v.span(span);
        }
        Stmt::Set {
            span, name, value, ..
        } => {
            v.name(name, *span);
            expr_mut(v, value);
            v.span(span);
        }
        Stmt::Return { span, value } => {
            expr_mut(v, value);
            v.span(span);
        }
        Stmt::Assert {
            span,
            cond,
            message,
        } => {
            expr_mut(v, cond);
            expr_mut(v, message);
            v.span(span);
        }
        Stmt::Fail { span, message } => {
            expr_mut(v, message);
            v.span(span);
        }
        Stmt::If {
            span,
            cond,
            then,
            otherwise,
        } => {
            expr_mut(v, cond);
            stmts_mut(v, then);
            if let Some(o) = otherwise {
                stmts_mut(v, o);
            }
            v.span(span);
        }
        Stmt::ForList {
            span,
            name,
            iter,
            body,
        } => {
            expr_mut(v, iter);
            v.binder(name, *span, false);
            stmts_mut(v, body);
            v.span(span);
        }
        Stmt::ForRange {
            span,
            name,
            low,
            high,
            body,
        } => {
            expr_mut(v, low);
            expr_mut(v, high);
            v.binder(name, *span, false);
            stmts_mut(v, body);
            v.span(span);
        }
        Stmt::WhileIs {
            span,
            name,
            pattern,
            body,
        } => {
            v.name(name, *span);
            pattern_mut(v, pattern);
            stmts_mut(v, body);
            v.span(span);
        }
        Stmt::Provide { span, init, .. } => {
            expr_mut(v, init);
            v.span(span);
        }
        Stmt::ExprStmt { span, expr } => {
            expr_mut(v, expr);
            v.span(span);
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn expr_mut(v: &mut impl VisitMut, e: &mut Expr) {
    if v.expr(e) {
        return;
    }
    match e {
        Expr::IntLit(span, _)
        | Expr::BoolLit(span, _)
        | Expr::NothingLit(span)
        | Expr::ClockNow(span) => v.span(span),
        Expr::Text(span, parts) => {
            for part in parts {
                if let TextPart::Hole(h) = part {
                    expr_mut(v, h);
                }
            }
            v.span(span);
        }
        Expr::Name(span, name) => {
            v.name(name, *span);
            v.span(span);
        }
        Expr::ListLit(span, items, _) => {
            for item in items {
                expr_mut(v, item);
            }
            v.span(span);
        }
        Expr::Call(span, callee, args, _) => {
            expr_mut(v, callee);
            for Arg { span, value, .. } in args {
                expr_mut(v, value);
                v.span(span);
            }
            v.span(span);
        }
        Expr::Field(span, base, _)
        | Expr::UnaryNeg(span, base)
        | Expr::Comptime(span, base)
        | Expr::Unquote(span, base)
        | Expr::FsRead(span, base)
        | Expr::CatalogColumns(span, base) => {
            expr_mut(v, base);
            v.span(span);
        }
        Expr::Index(span, a, b) | Expr::Binary(span, _, a, b) | Expr::OrFail(span, a, b) => {
            expr_mut(v, a);
            expr_mut(v, b);
            v.span(span);
        }
        Expr::OrReturn(span, a, mapping) => {
            expr_mut(v, a);
            if let Some(m) = mapping {
                expr_mut(v, m);
            }
            v.span(span);
        }
        Expr::Match(span, scrutinee, arms, _) => {
            expr_mut(v, scrutinee);
            for Arm {
                span,
                pattern,
                body,
            } in arms
            {
                pattern_mut(v, pattern);
                match body {
                    ArmBody::ExprArm(e) => expr_mut(v, e),
                    ArmBody::StmtArm(stmts) => stmts_mut(v, stmts),
                }
                v.span(span);
            }
            v.span(span);
        }
        Expr::IntIn(span, a, lo, hi) => {
            expr_mut(v, a);
            expr_mut(v, lo);
            expr_mut(v, hi);
            v.span(span);
        }
        Expr::IntAtLeast(span, a, lo) => {
            expr_mut(v, a);
            expr_mut(v, lo);
            v.span(span);
        }
        Expr::Quote(span, body) => {
            body_mut(v, body);
            v.span(span);
        }
    }
}

fn pattern_mut(v: &mut impl VisitMut, p: &mut Pattern) {
    match p {
        Pattern::Wild(span) | Pattern::LitInt(span, _) | Pattern::LitText(span, _) => v.span(span),
        Pattern::Bind(span, name) => {
            v.binder(name, *span, true);
            v.span(span);
        }
        Pattern::OkPat(span, inner) | Pattern::ErrPat(span, inner) => {
            pattern_mut(v, inner);
            v.span(span);
        }
        Pattern::VariantPat(span, name, payload) => {
            v.name(name, *span);
            if let Some(p) = payload {
                pattern_mut(v, p);
            }
            v.span(span);
        }
        Pattern::ListPat(span, items) => {
            for item in items {
                pattern_mut(v, item);
            }
            v.span(span);
        }
    }
}

fn type_mut(v: &mut impl VisitMut, t: &mut TypeExpr) {
    match t {
        TypeExpr::Named(span, name) => {
            v.name(name, *span);
            v.span(span);
        }
        TypeExpr::Apply(span, name, args) => {
            v.name(name, *span);
            for a in args {
                type_mut(v, a);
            }
            v.span(span);
        }
        TypeExpr::AtLeast(span, base, lo) => {
            type_mut(v, base);
            expr_mut(v, lo);
            v.span(span);
        }
        TypeExpr::Range(span, base, lo, hi) => {
            type_mut(v, base);
            expr_mut(v, lo);
            expr_mut(v, hi);
            v.span(span);
        }
    }
}

/// The index of the `unquote` hole `e`, if it is one.
#[must_use]
pub fn hole(e: &Expr) -> Option<usize> {
    match e {
        Expr::Unquote(_, inner) => match inner.as_ref() {
            Expr::Name(_, n) => n.strip_prefix(HOLE)?.parse().ok(),
            _ => None,
        },
        _ => None,
    }
}

/// Why [`fill`] failed: a statement went into an expression slot (`L064`).
#[derive(Debug)]
pub struct NotANode;

/// Fills each `unquote` hole of `template` with `values[i]`. A hole that is a whole
/// expression statement takes a statement or an expression; any other hole takes an
/// expression.
///
/// # Errors
/// A statement for a hole in expression position.
pub fn fill(template: &mut QuoteBody, values: &[&QuoteBody]) -> Result<(), NotANode> {
    struct Fill<'v> {
        values: &'v [&'v QuoteBody],
        bad: bool,
    }
    impl VisitMut for Fill<'_> {
        fn stmt(&mut self, s: &mut Stmt) -> bool {
            if let Stmt::ExprStmt { expr, .. } = s
                && let Some(QuoteBody::QuotedStmt(value)) =
                    hole(expr).and_then(|i| self.values.get(i))
            {
                *s = value.as_ref().clone();
                return true;
            }
            false
        }
        fn expr(&mut self, e: &mut Expr) -> bool {
            match hole(e).and_then(|i| self.values.get(i)) {
                Some(QuoteBody::QuotedExpr(value)) => {
                    *e = value.as_ref().clone();
                    true
                }
                Some(QuoteBody::QuotedStmt(_)) => {
                    self.bad = true;
                    true
                }
                // Another quote's holes are its own
                None => matches!(e, Expr::Quote(..)),
            }
        }
    }
    let mut v = Fill { values, bad: false };
    if let QuoteBody::QuotedExpr(e) = template
        && let Some(QuoteBody::QuotedStmt(s)) = hole(e).and_then(|i| values.get(i))
    {
        // A whole-quote hole takes the value as it is
        *template = QuoteBody::QuotedStmt(s.clone());
        return Ok(());
    }
    body_mut(&mut v, template);
    if v.bad { Err(NotANode) } else { Ok(()) }
}

/// A splice of `body` at `site`: its binders made fresh with `n`, and every span the
/// template left empty (line 0) set to `site`.
pub fn freshen(body: &mut QuoteBody, n: usize, site: Span) {
    struct Fresh {
        n: usize,
        site: Span,
    }
    impl Fresh {
        fn rename(&self, name: &mut String) {
            if name.ends_with(BINDER) {
                name.push_str(&self.n.to_string());
            }
        }
    }
    impl VisitMut for Fresh {
        fn span(&mut self, span: &mut Span) {
            if span.start.line == 0 {
                *span = self.site;
            }
        }
        fn name(&mut self, name: &mut String, _: Span) {
            self.rename(name);
        }
        fn binder(&mut self, name: &mut String, _: Span, _: bool) {
            self.rename(name);
        }
    }
    body_mut(&mut Fresh { n, site }, body);
}

/// `text` with the hygiene marks spelled as source: a qualified name loses its module and
/// a fresh binder `t\u{2}7` reads `t_7`.
// ponytail: so printed source re-parses, but a macro-module name the caller does not import
// does not resolve there, and `t_7` could meet a user's `t_7`; print a qualified spelling if
// expand output needs to check as well as parse.
#[must_use]
pub fn clean(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            QUALIFIED => while chars.next_if(char::is_ascii_digit).is_some() {},
            BINDER => out.push('_'),
            c => out.push(c),
        }
    }
    out
}
