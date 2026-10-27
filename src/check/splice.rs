//! Macros (docs/lingua1.md, Macros, Compilation pipeline): a `quote` template checked in the
//! macro's scope, and a macro call spliced into the caller's body before the caller's check
//! goes on.
//!
//! Hygiene works on the template's names (see `crate::ast::quote`). When the macro is
//! checked, each free name in a quote resolves on the macro's scope stack and is qualified
//! with the macro's module, and each binder the quote introduces is marked. Each splice then
//! makes the marked binders fresh, so they neither clash with nor capture a caller local.
//! The caller's AST enters only through `unquote` and keeps its own names and spans; every
//! node from the template gets the call site's span.

use std::collections::HashSet;

use super::body::{FnCx, tx};
use super::decl::{Binding, Checker, Item};
use crate::ast::quote::{self, BINDER, HOLE, QUALIFIED, VisitMut};
use crate::ast::{Expr, QuoteBody, Span};
use crate::eval::{Root, Value};
use crate::lex::Location;
use crate::tir::{SymbolId, TirExpr, TirKind, TirStmt, Type};

/// A template span, which a splice replaces with the call site.
const NOWHERE: Span = Span {
    start: Location { line: 0, column: 0 },
    end: Location { line: 0, column: 0 },
};

// The binders a template introduces: every `let`, `var`, `const`, and `for` name, and every
// bare pattern ident that is not a variant
// ponytail: a binder's spelling marks every use of it in the quote, not only those in its
// scope, so a use outside the scope is `L011` at the splice rather than at the macro; track
// scopes if a macro needs a binder and a free name with one spelling.
struct Binders<'c, 'a> {
    checker: &'c Checker<'a>,
    m: usize,
    names: HashSet<String>,
}

impl VisitMut for Binders<'_, '_> {
    fn expr(&mut self, e: &mut Expr) -> bool {
        matches!(e, Expr::Unquote(..) | Expr::Quote(..))
    }
    fn binder(&mut self, name: &mut String, _: Span, pattern: bool) {
        if !(pattern && matches!(self.checker.lookup(self.m, name), Some(Binding::Variant(_)))) {
            self.names.insert(name.clone());
        }
    }
}

// Marks the binders, resolves and qualifies the free names, numbers the `unquote` holes, and
// empties the spans
struct Prepare<'c, 'a> {
    checker: &'c mut Checker<'a>,
    cx: &'c FnCx,
    binders: HashSet<String>,
    holes: Vec<Expr>,
    ok: bool,
}

impl VisitMut for Prepare<'_, '_> {
    fn expr(&mut self, e: &mut Expr) -> bool {
        match e {
            Expr::Unquote(span, inner) => {
                let hole = Expr::Name(*span, format!("{HOLE}{}", self.holes.len()));
                self.holes.push(std::mem::replace(inner.as_mut(), hole));
                true
            }
            // ponytail: a quote inside a quote is kept as written, its holes and names
            // unresolved; resolve it as its own template if a macro needs to build a macro.
            Expr::Quote(..) => true,
            _ => false,
        }
    }

    fn span(&mut self, span: &mut Span) {
        *span = NOWHERE;
    }

    fn binder(&mut self, name: &mut String, span: Span, _: bool) {
        if self.binders.contains(name.as_str()) {
            name.push(BINDER);
        } else {
            self.name(name, span);
        }
    }

    fn name(&mut self, name: &mut String, span: Span) {
        if self.binders.contains(name.as_str()) {
            name.push(BINDER);
            return;
        }
        if name == "Ok" || name == "Err" {
            return;
        }
        let m = self.cx.m;
        match self.checker.resolve_name(self.cx, name) {
            Some(Binding::Local(_)) => {
                // ponytail: lingua1.md puts the macro's locals on the quote's scope stack but
                // does not say how a value becomes a tree; lift it (an `Int` to a literal) if
                // a macro needs one. `unquote` covers an `Expr`.
                let what = format!(
                    "a quoted name for the macro's local `{name}` (write `unquote({name})` for an `Expr`)"
                );
                self.checker.not_yet(m, span, &what);
                self.ok = false;
            }
            Some(Binding::Poison) => self.ok = false,
            // A prelude name means the same in every module, and a caller cannot shadow one
            Some(_) if !self.checker.mods[m].names.contains_key(name.as_str()) => {}
            Some(_) => {
                name.push(QUALIFIED);
                name.push_str(&m.to_string());
            }
            None => {
                self.checker.unknown(self.cx, span, name);
                self.ok = false;
            }
        }
    }
}

impl Checker<'_> {
    /// `quote`, legal in a comptime context or an example: the template with each free name
    /// resolved now, on `cx`'s scope stack (`L011` if it is unknown), and the `unquote`
    /// operands checked as `Expr`s in `cx`.
    pub(super) fn quote(&mut self, cx: &mut FnCx, span: Span, body: &QuoteBody) -> Option<TirExpr> {
        if !cx.comptime && !cx.example {
            let summary = "`quote` builds an `Expr`, which only exists at compile time".into();
            let help = "quote in a function that returns `Expr`, or in an example".into();
            self.report_help(cx.m, "L063", span, summary, help);
            return None;
        }
        let mut template = body.clone();
        let mut binders = Binders {
            checker: self,
            m: cx.m,
            names: HashSet::new(),
        };
        quote::body_mut(&mut binders, &mut template);
        let binders = binders.names;
        let mut prepare = Prepare {
            checker: self,
            cx,
            binders,
            holes: Vec::new(),
            ok: true,
        };
        quote::body_mut(&mut prepare, &mut template);
        let (holes, ok) = (prepare.holes, prepare.ok);
        let holes: Vec<Option<TirExpr>> = holes
            .iter()
            .map(|h| self.check(cx, h, &Type::ExprAst))
            .collect();
        let holes = holes.into_iter().collect::<Option<Vec<_>>>()?;
        ok.then(|| {
            tx(
                Type::ExprAst,
                span,
                TirKind::Quote(Box::new(template), holes),
            )
        })
    }

    /// The macro that `e` calls, `comptime f(...)` or `f(...)` with `f` returning `Expr`, and
    /// the call. `None` in an example or in a body that returns `Expr` or `Type`, where such
    /// a call is an ordinary call that builds a tree.
    pub(super) fn macro_call<'e>(
        &mut self,
        cx: &FnCx,
        e: &'e Expr,
    ) -> Option<(SymbolId, &'e Expr)> {
        if cx.example || matches!(cx.ret, Type::ExprAst | Type::TypeAst) {
            return None;
        }
        let call = match e {
            Expr::Comptime(_, inner) => inner.as_ref(),
            _ => e,
        };
        let Expr::Call(_, callee, ..) = call else {
            return None;
        };
        let Expr::Name(_, name) = callee.as_ref() else {
            return None;
        };
        let Some(Binding::Function(id)) = self.resolve_name(cx, name) else {
            return None;
        };
        if !self.ensure(id) {
            return None;
        }
        (self.sigs.get(&id)?.ret == Type::ExprAst).then_some((id, call))
    }

    /// The macro call `e` in statement position: its splice must be a statement (`L065`).
    pub(super) fn splice_stmt(
        &mut self,
        cx: &mut FnCx,
        e: &Expr,
        id: SymbolId,
        call: &Expr,
    ) -> Option<(Option<TirStmt>, bool)> {
        let site = e.span();
        self.splice(cx, e, id, call, |c, cx, body| match body {
            QuoteBody::QuotedStmt(s) => {
                let (out, diverges) = c.stmt(cx, &s);
                // An exception report points at the whole call site
                let out = out.map(|stmt| match stmt {
                    TirStmt::Assert { cond, message, .. } => TirStmt::Assert {
                        cond,
                        message,
                        span: site,
                    },
                    TirStmt::Fail { message, .. } => TirStmt::Fail {
                        message,
                        span: site,
                    },
                    other => other,
                });
                Some((out, diverges))
            }
            QuoteBody::QuotedExpr(_) => {
                c.kind_mismatch(cx.m, site, id, "an expression", "a statement");
                None
            }
        })
    }

    /// The macro call `e` in expression position, checked against `want` if given: its
    /// splice must be an expression (`L065`).
    pub(super) fn splice_expr(
        &mut self,
        cx: &mut FnCx,
        e: &Expr,
        id: SymbolId,
        call: &Expr,
        want: Option<&Type>,
    ) -> Option<TirExpr> {
        let site = e.span();
        self.splice(cx, e, id, call, |c, cx, body| match body {
            QuoteBody::QuotedExpr(x) => match want {
                Some(t) => c.check(cx, &x, t),
                None => c.synth(cx, &x),
            },
            QuoteBody::QuotedStmt(_) => {
                c.kind_mismatch(cx.m, site, id, "a statement", "an expression");
                None
            }
        })
    }

    fn kind_mismatch(&mut self, m: usize, site: Span, id: SymbolId, got: &str, want: &str) {
        let name = &self.symbols[id as usize].name;
        let summary = format!("`{name}` splices {got} where {want} goes");
        self.report(m, "L065", site, summary);
    }

    // Runs the macro `id` on the call `call` at `e` and hands `then` the tree, its binders
    // fresh and its spans the call site. Each `Expr` argument is the caller's AST; any other
    // folds in a comptime context. Without `comptime` in a runtime body it is `L063`.
    fn splice<T>(
        &mut self,
        cx: &mut FnCx,
        e: &Expr,
        id: SymbolId,
        call: &Expr,
        then: impl FnOnce(&mut Self, &mut FnCx, QuoteBody) -> Option<T>,
    ) -> Option<T> {
        let (m, site) = (cx.m, e.span());
        let Expr::Call(span, callee, args, _) = call else {
            return None;
        };
        let name = self.symbols[id as usize].name.clone();
        if !matches!(e, Expr::Comptime(..)) && !cx.comptime {
            let summary = format!("`{name}` returns `Expr`, which only exists at compile time");
            let help = format!("splice it with `comptime {name}(...)`");
            self.report_help(m, "L063", site, summary, help);
            return None;
        }
        let sig = self.sigs.get(&id)?.clone();
        let matched = self.match_args(cx, *span, &name, &sig.params, args)?;
        let mut root = cx.comptime();
        if !self.effect_check(&root, callee.span(), &name, &sig.effect, sig.module) {
            return None;
        }
        let args: Vec<Option<TirExpr>> = matched
            .args
            .iter()
            .zip(&sig.params)
            .map(|(a, (_, t))| {
                let a = (*a)?;
                if *t == Type::ExprAst {
                    let ast = QuoteBody::QuotedExpr(Box::new(a.clone()));
                    Some(tx(
                        t.clone(),
                        a.span(),
                        TirKind::Quote(Box::new(ast), Vec::new()),
                    ))
                } else {
                    self.check(&mut root, a, t)
                }
            })
            .collect();
        let args = args.into_iter().collect::<Option<Vec<_>>>()?;
        if self.is_poisoned(id) {
            return None;
        }
        if self.stack.contains(&Item::Splice(id)) {
            self.cycle(Item::Splice(id));
            return None;
        }
        self.stack.push(Item::Splice(id));
        let run = tx(Type::ExprAst, *span, TirKind::Call(id, args));
        let out = match self.eval_root(m, Root::Expr(&run), &format!("the macro `{name}`")) {
            Some(Value::Expr(ast)) => {
                let mut body = ast.0.as_ref().clone();
                quote::freshen(&mut body, self.spliced.len(), site);
                self.spliced.push((m, site, body.clone()));
                then(self, cx, body)
            }
            _ => None,
        };
        self.stack.pop();
        out
    }

    /// Each module's declarations that hold a splice, printed with the splices in place, for
    /// `lingua expand`.
    // ponytail: a splice inside spliced code has the outer call site as its span, so it prints
    // as its unspliced call; key splices by more than the span if nested macros need the view.
    pub(super) fn spliced_text(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for (m, source) in self.loaded.sources.iter().enumerate() {
            let Some(module) = &source.module else {
                continue;
            };
            let hit = std::cell::Cell::new(false);
            let lookup = |span: Span| {
                let found = self
                    .spliced
                    .iter()
                    .find(|(sm, site, _)| *sm == m && *site == span)
                    .map(|(.., body)| body);
                hit.set(hit.get() || found.is_some());
                found
            };
            let decls: Vec<String> = module
                .decls
                .iter()
                .filter_map(|d| {
                    hit.set(false);
                    let text = crate::ast::print::decl(d, &lookup)?;
                    hit.get().then(|| quote::clean(&text))
                })
                .collect();
            if !decls.is_empty() {
                out.push((source.key.clone(), decls.join("\n")));
            }
        }
        out
    }
}
