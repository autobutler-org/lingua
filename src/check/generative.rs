//! Generative types and package `comptime` blocks (docs/lingua1.md, Type providers): each
//! generated name is a type symbol from the first pass; expanding it runs its body on the
//! interpreter, or reuses `provided/<module>.lf` under the cache policy, and the result is an
//! ordinary product or sum declaration the rest of the checker resolves like a written one.

use super::Cache;
use super::body::FnCx;
use super::decl::{Binding, Checker, find_word, word_at};
use crate::ast::{Decl, Effect, Span, Stmt};
use crate::eval::{Root, Value};
use crate::provider::{self, Generated};
use crate::tir::{SymbolKind, TirStmt, Type, TypeKind};

// One step of a package `comptime` block
enum Step {
    Run(Vec<TirStmt>),
    Provide(crate::tir::TirExpr),
}

impl Checker<'_> {
    /// Declares and exports the generated type `name`, still unexpanded.
    pub(super) fn generated_name(&mut self, m: usize, name: &str, span: Span) -> u32 {
        let kind = SymbolKind::Type(TypeKind::Newtype {
            base: Type::Nothing,
            range: None,
        });
        let id = self.push_symbol(name, kind, Type::Nothing, m, span);
        self.symbols[id as usize].ty = Type::Nominal(id);
        self.declare(m, name, Binding::Type(id), span);
        self.export(m, name, Binding::Type(id));
        id
    }

    /// Expands module `m`'s generative types and package `comptime` blocks, in source
    /// order, before anything else in it resolves, so every generated name is a product or a
    /// sum, with its constructors declared and exported, by the time a body, another
    /// declaration, or an importing module uses it.
    pub(super) fn expand(&mut self, m: usize) {
        let Some(module) = &self.loaded.sources[m].module else {
            return;
        };
        for (i, decl) in module.decls.iter().enumerate() {
            if !matches!(decl, Decl::Generative { .. } | Decl::ComptimeBlock { .. }) {
                continue;
            }
            match self.decl_ids.get(&(m, i)) {
                Some(&id) => {
                    self.ensure(id);
                }
                None => self.comptime_block_decl(m, decl),
            }
        }
    }

    fn comptime_block_decl(&mut self, m: usize, decl: &Decl) {
        if let Decl::ComptimeBlock { span, effect, body } = decl {
            self.comptime_block(m, *span, effect, body);
        }
    }

    // `L025` unless `effect` is legal on a generative type, a package `comptime` block, or a
    // file-level const
    fn provider_effect(&mut self, m: usize, span: Span, effect: &Effect) -> bool {
        if matches!(
            effect,
            Effect::Pure | Effect::ReadsNet | Effect::ReadsFs | Effect::ReadsCatalog
        ) {
            return true;
        }
        let summary = "a generative type or a package `comptime` block may be `pure`, `reads net`, `reads fs`, or `reads catalog`";
        self.report(m, "L025", span, summary.into());
        false
    }

    // The cached expansion of `name` in module `m`, unless the policy is `--refresh`
    fn cached(&mut self, m: usize, name: &str) -> Option<Generated> {
        if self.loaded.cache == Cache::Refresh {
            return None;
        }
        let key = self.key(m);
        let root = &self.loaded.root;
        self.caches
            .entry(m)
            .or_insert_with(|| provider::read_cache(root, key))
            .get(name)
            .cloned()
    }

    // `L071` for `name`, which `--offline` cannot generate
    fn offline(&mut self, m: usize, name: &str, span: Span) {
        let path = format!("provided/{}.lf", self.key(m));
        self.report_help(
            m,
            "L071",
            span,
            format!("`--offline` and `{name}` has no entry in {path}"),
            "run without `--offline` once to expand it and write the cache".into(),
        );
    }

    /// A generative `type Name`: its body, a comptime program under its own effect, must
    /// return a `Type` on every path.
    pub(super) fn generative(
        &mut self,
        m: usize,
        span: Span,
        name: &str,
        effect: &Effect,
        body: &[Stmt],
    ) {
        let Some(Binding::Type(id)) = self.mods[m].names.get(name).copied() else {
            return;
        };
        let name_span = find_word(self.source(m), span, name);
        let generated = if let Some(r) = self.cached(m, name) {
            Some(r)
        } else if !self.provider_effect(m, span, effect) {
            None
        } else if self.loaded.cache == Cache::Offline {
            self.offline(m, name, name_span);
            None
        } else {
            self.generative_body(m, name, name_span, effect, body)
        };
        self.adopt(m, id, name, name_span, generated);
    }

    fn generative_body(
        &mut self,
        m: usize,
        name: &str,
        name_span: Span,
        effect: &Effect,
        body: &[Stmt],
    ) -> Option<Generated> {
        let mut cx = FnCx {
            ret: Type::TypeAst,
            ..FnCx::root(m, effect.clone())
        };
        let (mark, poison) = (self.diags.len(), self.hit_poison.replace(false));
        let (stmts, returns) = self.block(&mut cx, body);
        if !returns {
            let summary =
                format!("`type {name}` can reach the end of its body without returning a `Type`");
            self.report(m, "L012", name_span, summary);
        }
        let clean = self.diags.len() == mark && !self.hit_poison.replace(poison);
        if !clean {
            return None;
        }
        match self.eval_root(m, Root::Body(&stmts, name_span), &format!("`type {name}`"))? {
            Value::Type(generated) => Some(generated.as_ref().clone()),
            _ => None,
        }
    }

    // The expansion of `name` becomes a product or sum declaration that resolves like a
    // written one, in module `m`, with every diagnostic at the generated name; without one,
    // the name is poisoned and its uses report nothing more.
    fn adopt(&mut self, m: usize, id: u32, name: &str, span: Span, generated: Option<Generated>) {
        let Some(generated) = generated else {
            self.provider_failed.insert(m);
            self.poisoned.insert(id);
            self.mods[m].names.insert(name.to_owned(), Binding::Poison);
            for export in &mut self.mods[m].exports {
                if export.0 == name {
                    export.1 = Binding::Poison;
                }
            }
            return;
        };
        let decl = provider::decl(name, &generated, span);
        let kind = match &generated {
            Generated::Record(_) => TypeKind::Product,
            Generated::Sum(variants) => {
                // A written sum's constructors, as `collect` declares them
                let ids = (0..)
                    .zip(variants)
                    .map(|(tag, (vname, _))| {
                        let kind = SymbolKind::Variant {
                            parent: id,
                            tag,
                            payload: None,
                        };
                        let vid =
                            self.push_symbol(vname.as_str(), kind, Type::Nominal(id), m, span);
                        self.declare(m, vname.as_str(), Binding::Variant(vid), span);
                        self.export(m, vname.as_str(), Binding::Variant(vid));
                        vid
                    })
                    .collect();
                self.variants.insert(id, ids);
                TypeKind::Sum
            }
        };
        self.symbols[id as usize].kind = SymbolKind::Type(kind);
        self.resolve_decl(m, &decl);
        self.generated.push((m, span, name.to_owned(), generated));
    }

    /// A package `comptime` block: its statements run in order under its effect, and each
    /// `provide` names one generated type. It runs once, on the first use of any name it
    /// provides, and not at all when the cache has every one.
    // ponytail: the block is all or nothing against the cache: one missing name reruns the
    // whole block; a block that provides nothing has no cache entry and reruns every check.
    pub(super) fn comptime_block(&mut self, m: usize, span: Span, effect: &Effect, body: &[Stmt]) {
        if !self
            .blocks_done
            .insert((m, span.start.line, span.start.column))
        {
            return;
        }
        let provides: Vec<(String, Span, u32)> = body
            .iter()
            .filter_map(|stmt| match stmt {
                Stmt::Provide { span, name, .. } => {
                    let name_span = find_word(self.source(m), *span, name);
                    match self.mods[m].names.get(name) {
                        Some(Binding::Type(id)) => Some((name.clone(), name_span, *id)),
                        _ => None,
                    }
                }
                _ => None,
            })
            .collect();
        let cached: Vec<Option<Generated>> = provides
            .iter()
            .map(|(name, ..)| self.cached(m, name))
            .collect();
        let all_cached = !provides.is_empty() && cached.iter().all(Option::is_some);
        let expansions: Vec<Option<Generated>> = if all_cached {
            cached
        } else if !self.provider_effect(m, span, effect) {
            vec![None; provides.len()]
        } else if self.loaded.cache == Cache::Offline {
            for ((name, name_span, _), hit) in provides.iter().zip(&cached) {
                if hit.is_none() {
                    self.offline(m, name, *name_span);
                }
            }
            cached
        } else {
            self.run_block(m, span, effect, body, provides.len())
        };
        for ((name, name_span, id), generated) in provides.into_iter().zip(expansions) {
            self.adopt(m, id, &name, name_span, generated);
        }
    }

    // Checks the block's statements, then runs them in order; the expansion of each `provide`
    // in source order, `None` for all of them after a diagnostic
    fn run_block(
        &mut self,
        m: usize,
        span: Span,
        effect: &Effect,
        body: &[Stmt],
        n: usize,
    ) -> Vec<Option<Generated>> {
        let mut cx = FnCx::root(m, effect.clone());
        let (mark, poison) = (self.diags.len(), self.hit_poison.replace(false));
        // Statements between `provide`s run as one body; nothing flows between steps but
        // consts, which are already folded
        let mut steps = vec![Step::Run(Vec::new())];
        for stmt in body {
            if let Stmt::Provide { init, .. } = stmt {
                if let Some(e) = self.check(&mut cx, init, &Type::TypeAst) {
                    steps.extend([Step::Provide(e), Step::Run(Vec::new())]);
                }
            } else {
                let (out, _) = self.stmt(&mut cx, stmt);
                if let Some(Step::Run(last)) = steps.last_mut() {
                    last.extend(out);
                }
            }
        }
        let clean = self.diags.len() == mark && !self.hit_poison.replace(poison);
        if !clean {
            return vec![None; n];
        }
        let what = "the package `comptime` block";
        let mut out = Vec::new();
        for step in &steps {
            match step {
                Step::Run(stmts) if stmts.is_empty() => {}
                Step::Run(stmts) => {
                    let at = word_at(span, "comptime".len());
                    if self.eval_root(m, Root::Body(stmts, at), what).is_none() {
                        return vec![None; n];
                    }
                }
                Step::Provide(e) => match self.eval_root(m, Root::Expr(e), what) {
                    Some(Value::Type(generated)) => out.push(Some(generated.as_ref().clone())),
                    _ => return vec![None; n],
                },
            }
        }
        out
    }

    /// Each module's expanded declarations as source text, in source order; and the cache
    /// file of each, written unless `--offline` or one of its providers failed.
    pub(super) fn write_caches(&mut self) -> Vec<(String, String)> {
        let mut generated = self.generated.clone();
        generated.sort_by_key(|(m, span, ..)| (*m, span.start.line, span.start.column));
        let mut out = Vec::new();
        for m in 0..self.loaded.sources.len() {
            let decls: Vec<(&str, &Generated)> = generated
                .iter()
                .filter(|(gm, ..)| *gm == m)
                .map(|(.., name, g)| (name.as_str(), g))
                .collect();
            if decls.is_empty() {
                continue;
            }
            let text = provider::print_all(&decls);
            if self.loaded.cache != Cache::Offline && !self.provider_failed.contains(&m) {
                let key = self.key(m);
                if let Err(err) = provider::write_cache(&self.loaded.root, key, &text) {
                    let summary = format!(
                        "cannot write provided/{key}.lf: {err}; check that the source root is writable"
                    );
                    let d = super::Diagnostic::new(super::NOT_YET, summary, self.path(m));
                    self.diags.push(d);
                }
            }
            out.push((self.key(m).to_owned(), text));
        }
        out
    }
}
