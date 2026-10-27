//! Comptime roots (docs/lingua1.md, Compile-time execution, Examples): folding a `const`
//! initializer, a bound, or a `comptime` expression on `crate::eval`, the bodies those roots
//! call, and package examples. Splicing a macro call is `splice.rs`.

use super::body::FnCx;
use super::decl::{Checker, Item, find_word};
use crate::ast::{Effect, Example, Expect, Expr, Span, TypeExpr};
use crate::eval::{self, File, Root, Unfolded, Value};
use crate::lex::Location;
use crate::tir::{
    ExampleKind, SymbolId, SymbolKind, TirExample, TirExpect, TirExpr, TirKind, Type, prelude,
};

impl Checker<'_> {
    /// Checks the body of function `id` once, and records it as tainted when its check had a
    /// diagnostic or met a poisoned name. Meeting a body that is still being checked is `L061`.
    pub(super) fn body(&mut self, id: SymbolId) {
        if self.checked.contains(&id) {
            return;
        }
        if self.stack.contains(&Item::Body(id)) {
            self.cycle(Item::Body(id));
            return;
        }
        let Some(&(m, i)) = self.decl_of.get(&id) else {
            return;
        };
        let Some(module) = &self.loaded.sources[m].module else {
            return;
        };
        self.ensure(id);
        self.stack.push(Item::Body(id));
        let (mark, poison) = (self.diags.len(), self.hit_poison.replace(false));
        let f = self.function(m, &module.decls[i]);
        let tainted = self.diags.len() > mark || self.hit_poison.replace(poison);
        self.stack.pop();
        self.checked.insert(id);
        if tainted || f.is_none() {
            self.tainted.insert(id);
        }
        if let Some(f) = f {
            self.bodies.insert(id, (m, f));
        }
    }

    /// Evaluates the comptime root `root` in module `m` and returns its value as literal TIR
    /// of `root`'s type. `what` names the root for `L058`. `None` after a diagnostic, or
    /// quietly when it calls a function whose check already failed.
    pub(super) fn fold(&mut self, m: usize, root: &TirExpr, what: &str) -> Option<TirExpr> {
        let v = self.eval_root(m, Root::Expr(root), what)?;
        Some(self.reify(&v, &root.ty, root.span))
    }

    /// Evaluates the comptime root `root` in module `m` to a value; see [`Self::fold`].
    pub(super) fn eval_root(&mut self, m: usize, root: Root<'_>, what: &str) -> Option<Value> {
        // ponytail: a call of an unchecked function stops the run, checks that body, and
        // folds again from the start, so a root re-runs once per function it reaches; walk the
        // root's calls up front if a root that reaches many functions gets slow.
        loop {
            let folded = {
                let files = self
                    .loaded
                    .sources
                    .iter()
                    .map(|s| File {
                        path: &s.path,
                        source: &s.text,
                    })
                    .collect();
                let functions = self
                    .bodies
                    .iter()
                    .filter(|(id, _)| !self.tainted.contains(id))
                    .map(|(id, (m, f))| (*id, (*m, f)))
                    .collect();
                eval::fold(&self.symbols, files, functions, m, root, what)
            };
            match folded {
                Ok(e) => return Some(e),
                Err(Unfolded::Missing(id)) => {
                    self.body(id);
                    if !self.bodies.contains_key(&id) || self.tainted.contains(&id) {
                        self.hit_poison.set(true);
                        return None;
                    }
                }
                Err(Unfolded::Failed(d)) => {
                    self.diags.push(*d);
                    return None;
                }
            }
        }
    }

    // A value as literal TIR of type `ty`
    fn reify(&self, v: &Value, ty: &Type, span: Span) -> TirExpr {
        let kind = match v {
            Value::Int(n) => TirKind::Int(*n),
            Value::Type(generated) => TirKind::TypeValue(generated.as_ref().clone()),
            Value::Expr(ast) => TirKind::Quote(Box::new(ast.0.as_ref().clone()), Vec::new()),
            Value::Text(t) => TirKind::Text(t.to_string()),
            Value::Bool(b) => TirKind::Bool(*b),
            Value::Nothing => TirKind::Nothing,
            Value::List(items) => {
                let elem = match ty {
                    Type::List(elem) => elem.as_ref(),
                    _ => &Type::Nothing,
                };
                TirKind::List(items.iter().map(|v| self.reify(v, elem, span)).collect())
            }
            Value::Record(id, fields) => {
                let types = self.fields.get(id).map_or(&[][..], Vec::as_slice);
                let fields = fields
                    .iter()
                    .zip(types)
                    .map(|(v, (_, t))| self.reify(v, t, span))
                    .collect();
                TirKind::Record(*id, fields)
            }
            Value::Variant(id, payload) => {
                let payload_ty = match (ty, *id) {
                    (Type::Result(ok, _), prelude::OK) => Some(ok.as_ref().clone()),
                    (Type::Result(_, err), prelude::ERR) => Some(err.as_ref().clone()),
                    _ => match &self.symbols[*id as usize].kind {
                        SymbolKind::Variant { payload, .. } => payload.clone(),
                        _ => None,
                    },
                };
                let payload = payload.as_ref().map(|p| {
                    Box::new(self.reify(p, &payload_ty.clone().unwrap_or(Type::Nothing), span))
                });
                TirKind::Variant(*id, payload)
            }
        };
        TirExpr {
            ty: ty.clone(),
            span,
            kind,
        }
    }

    /// An inner `const`: its initializer is a comptime context over the enclosing locals,
    /// under the enclosing effect; the local it binds stands for the folded value.
    pub(super) fn inner_const(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        name: &str,
        ty: Option<&TypeExpr>,
        init: &Expr,
    ) {
        let m = cx.m;
        let declared = ty.map(|t| self.resolve_type(m, t));
        let mut root = cx.comptime();
        let tir = match declared {
            Some(None) => None,
            Some(Some(t)) => self
                .check(&mut root, init, &t)
                .map(|e| TirExpr { ty: t, ..e }),
            None => self.synth(&mut root, init),
        };
        let value = tir.and_then(|e| self.fold(m, &e, &format!("`{name}`")));
        let name_span = find_word(self.source(m), word_after(span, "const".len()), name);
        let ty = value.as_ref().map_or(Type::Nothing, |v| v.ty.clone());
        let id = self.bind_local(cx, name, name_span, ty, false);
        match value {
            Some(v) => {
                self.consts.insert(id, v);
            }
            None => {
                self.poisoned.insert(id);
            }
        }
    }

    /// A package example: its statements and arrow in a comptime context, `-> expr` a `Bool`
    /// (`L055`). `None` when its check had a diagnostic or met a poisoned name.
    // ponytail: names resolve in the example's module (its declarations, imports, and the
    // prelude), not across every module in the package as lingua1.md allows; widen it with
    // `L069` for ambiguous names when a package example needs an export it does not import.
    pub(super) fn package_example(&mut self, m: usize, ex: &Example) -> Option<TirExample> {
        let mut cx = FnCx::root(m, Effect::Pure);
        let (mark, poison) = (self.diags.len(), self.hit_poison.replace(false));
        let body: Vec<_> = ex
            .body
            .iter()
            .filter_map(|stmt| self.stmt(&mut cx, stmt).0)
            .collect();
        let expect = match &ex.expect {
            Expect::Value(e) => self.synth(&mut cx, e).and_then(|e| {
                if e.ty == Type::Bool {
                    Some(TirExpect::Value(e))
                } else {
                    let shown = self.show(&e.ty);
                    let summary =
                        format!("a package example's `->` must be a `Bool`, not `{shown}`");
                    self.report(m, "L055", e.span, summary);
                    None
                }
            }),
            Expect::Exception { message } => Some(TirExpect::Exception {
                message: message.clone(),
            }),
        };
        let clean = self.diags.len() == mark && !self.hit_poison.replace(poison);
        expect.filter(|_| clean).map(|expect| TirExample {
            label: ex.label.clone(),
            span: ex.span,
            kind: ExampleKind::Package,
            inputs: Vec::new(),
            script: None,
            body,
            expect,
        })
    }
}

// `span` without its first `len` columns: what follows a keyword
fn word_after(span: Span, len: usize) -> Span {
    Span {
        start: Location {
            line: span.start.line,
            column: span.start.column + len,
        },
        end: span.end,
    }
}
