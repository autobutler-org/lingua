//! Match patterns and exhaustiveness (docs/lingua1.md, Types > Match): closed scrutinees
//! (sums, `Result`, `BoundsError`) need every variant, checked through nested closed sums;
//! open scrutinees need `_`. `L031`, `L032`, `L033`.

use super::body::FnCx;
use super::decl::{Binding, Checker, alternatives};
use super::types;
use crate::ast::{Pattern, Span};
use crate::tir::{Range, SymbolId, SymbolKind, TirPat, Type, prelude};

pub(super) fn span_of(p: &Pattern) -> Span {
    match p {
        Pattern::Wild(s)
        | Pattern::LitInt(s, _)
        | Pattern::LitText(s, _)
        | Pattern::Bind(s, _)
        | Pattern::OkPat(s, _)
        | Pattern::ErrPat(s, _)
        | Pattern::VariantPat(s, ..)
        | Pattern::ListPat(s, _) => *s,
    }
}

// Whether every value `q` matches is matched by `p`
#[allow(clippy::many_single_char_names)]
fn subsumes(p: &TirPat, q: &TirPat) -> bool {
    match (p, q) {
        (TirPat::Wild | TirPat::Bind(_), _) => true,
        (TirPat::Variant(a, pa), TirPat::Variant(b, qb)) if a == b => match (pa, qb) {
            (None, _) => true,
            (Some(x), Some(y)) => subsumes(x, y),
            (Some(_), None) => false,
        },
        (TirPat::LitInt(a), TirPat::LitInt(b)) => a == b,
        (TirPat::LitText(a), TirPat::LitText(b)) => a == b,
        (TirPat::List(a), TirPat::List(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| subsumes(x, y))
        }
        _ => false,
    }
}

impl Checker<'_> {
    /// The variants of a closed scrutinee type, with their payload types.
    pub(super) fn sum_variants(&self, ty: &Type) -> Option<Vec<(SymbolId, Option<Type>)>> {
        match ty {
            Type::Result(ok, err) => Some(vec![
                (prelude::OK, Some(ok.as_ref().clone())),
                (prelude::ERR, Some(err.as_ref().clone())),
            ]),
            Type::Nominal(id) => self.variants.get(id).map(|vs| {
                vs.iter()
                    .map(|v| match &self.symbols[*v as usize].kind {
                        SymbolKind::Variant { payload, .. } => (*v, payload.clone()),
                        _ => (*v, None),
                    })
                    .collect()
            }),
            _ => None,
        }
    }

    fn name_of(&self, id: SymbolId) -> String {
        self.symbols[id as usize].name.clone()
    }

    fn variant_pattern(
        &mut self,
        cx: &mut FnCx,
        span: Span,
        name: &str,
        payload: Option<&Pattern>,
        ty: &Type,
    ) -> Option<TirPat> {
        let Some(variants) = self.sum_variants(ty) else {
            let shown = self.show(ty);
            self.report(
                cx.m,
                "L020",
                span,
                format!("a variant pattern cannot match `{shown}`"),
            );
            return None;
        };
        let Some((id, payload_ty)) = variants
            .iter()
            .find(|(v, _)| self.symbols[*v as usize].name == name)
            .cloned()
        else {
            let names: Vec<String> = variants.iter().map(|(v, _)| self.name_of(*v)).collect();
            let summary = format!("`{name}` is not a variant of `{}`", self.show(ty));
            self.report_help(cx.m, "L020", span, summary, alternatives(&names));
            return None;
        };
        match (payload_ty, payload) {
            (None, None) => Some(TirPat::Variant(id, None)),
            (None, Some(_)) => {
                self.report(
                    cx.m,
                    "L020",
                    span,
                    format!("`{name}` has no payload; write it bare"),
                );
                None
            }
            (Some(_), None) => {
                self.report(
                    cx.m,
                    "L028",
                    span,
                    format!("`{name}` has a payload; write `{name}(...)` or `{name}(_)`"),
                );
                None
            }
            (Some(t), Some(p)) => {
                let inner = self.pattern(cx, p, &t)?;
                Some(TirPat::Variant(id, Some(Box::new(inner))))
            }
        }
    }

    /// Checks `pat` against the scrutinee type `ty`, binding names in the current scope.
    pub(super) fn pattern(&mut self, cx: &mut FnCx, pat: &Pattern, ty: &Type) -> Option<TirPat> {
        let m = cx.m;
        match pat {
            Pattern::Wild(_) => Some(TirPat::Wild),
            Pattern::Bind(span, name) => {
                if let Some(Binding::Variant(_)) = self.lookup(m, name) {
                    return self.variant_pattern(cx, *span, name, None, ty);
                }
                let id = self.bind_local(cx, name, *span, ty.clone(), false);
                Some(TirPat::Bind(id))
            }
            Pattern::LitInt(span, v) => match types::range(&self.symbols, ty) {
                Some(r) if r.lo <= *v && *v <= r.hi => Some(TirPat::LitInt(*v)),
                Some(Range { lo, hi }) => {
                    let summary =
                        format!("`{v}` is outside `{}` ({lo} through {hi})", self.show(ty));
                    self.report(m, "L024", *span, summary);
                    None
                }
                None => {
                    let shown = self.show(ty);
                    self.report(
                        m,
                        "L020",
                        *span,
                        format!("an integer pattern cannot match `{shown}`"),
                    );
                    None
                }
            },
            Pattern::LitText(span, text) => {
                if types::is_textual(&self.symbols, ty) {
                    Some(TirPat::LitText(text.clone()))
                } else {
                    let shown = self.show(ty);
                    self.report(
                        m,
                        "L020",
                        *span,
                        format!("a text pattern cannot match `{shown}`"),
                    );
                    None
                }
            }
            Pattern::OkPat(span, inner) => self.variant_pattern(cx, *span, "Ok", Some(inner), ty),
            Pattern::ErrPat(span, inner) => self.variant_pattern(cx, *span, "Err", Some(inner), ty),
            Pattern::VariantPat(span, name, payload) => {
                self.variant_pattern(cx, *span, name, payload.as_deref(), ty)
            }
            Pattern::ListPat(span, items) => {
                let Type::List(elem) = ty else {
                    let shown = self.show(ty);
                    self.report(
                        m,
                        "L020",
                        *span,
                        format!("a list pattern cannot match `{shown}`"),
                    );
                    return None;
                };
                let items: Vec<Option<TirPat>> =
                    items.iter().map(|p| self.pattern(cx, p, elem)).collect();
                Some(TirPat::List(items.into_iter().collect::<Option<_>>()?))
            }
        }
    }

    // What `pats` leave uncovered of `ty`: variant names (nested as `Err(OutOfRange)`), or
    // `_` for an open type without a wildcard
    fn missing(&self, pats: &[&TirPat], ty: &Type) -> Vec<String> {
        if pats
            .iter()
            .any(|p| matches!(p, TirPat::Wild | TirPat::Bind(_)))
        {
            return Vec::new();
        }
        let Some(variants) = self.sum_variants(ty) else {
            return vec!["_".to_owned()];
        };
        let mut out = Vec::new();
        for (v, payload) in variants {
            let name = self.name_of(v);
            let subs: Vec<Option<&TirPat>> = pats
                .iter()
                .filter_map(|p| match p {
                    TirPat::Variant(id, inner) if *id == v => Some(inner.as_deref()),
                    _ => None,
                })
                .collect();
            if subs.is_empty() {
                out.push(name);
                continue;
            }
            if let Some(t) = payload {
                let inner: Vec<&TirPat> = subs.into_iter().flatten().collect();
                out.extend(
                    self.missing(&inner, &t)
                        .into_iter()
                        .map(|r| format!("{name}({r})")),
                );
            }
        }
        out
    }

    /// `L032` for `_` on a closed scrutinee, `L031` for what the arms miss, and `L033` for
    /// an arm an earlier arm already covers.
    pub(super) fn exhaustive(
        &mut self,
        m: usize,
        keyword: Span,
        ty: &Type,
        arms: &[(Span, TirPat)],
    ) {
        let product = matches!(ty, Type::Nominal(id) if matches!(self.symbols[*id as usize].kind, SymbolKind::Type(crate::tir::TypeKind::Product)));
        if *ty == Type::Bool || product {
            let shown = self.show(ty);
            self.report(
                m,
                "L020",
                keyword,
                format!("cannot `match` on `{shown}`; use `if` or its fields"),
            );
            return;
        }
        for (i, (span, p)) in arms.iter().enumerate() {
            if arms[..i].iter().any(|(_, q)| subsumes(q, p)) {
                self.report(
                    m,
                    "L033",
                    *span,
                    "this arm cannot match: an earlier arm covers it".into(),
                );
            }
        }
        let shown = self.show(ty);
        if let Some(variants) = self.sum_variants(ty) {
            let names: Vec<String> = variants.iter().map(|(v, _)| self.name_of(*v)).collect();
            if let Some((span, _)) = arms
                .iter()
                .find(|(_, p)| matches!(p, TirPat::Wild | TirPat::Bind(_)))
            {
                let summary = format!("`_` on the closed type `{shown}`; name every variant");
                self.report_help(m, "L032", *span, summary, alternatives(&names));
                return;
            }
        }
        let pats: Vec<&TirPat> = arms.iter().map(|(_, p)| p).collect();
        let missing = self.missing(&pats, ty);
        if missing.is_empty() {
            return;
        }
        if self.sum_variants(ty).is_some() {
            let summary = format!("match on {shown} is missing a variant");
            self.report_help(m, "L031", keyword, summary, alternatives(&missing));
        } else {
            let help = if matches!(ty, Type::List(_)) {
                "add a wildcard arm; list patterns are not exhaustive"
            } else {
                "add a wildcard arm `_`"
            };
            let summary = format!("match on {shown} is not exhaustive");
            self.report_help(m, "L031", keyword, summary, help.into());
        }
    }
}
