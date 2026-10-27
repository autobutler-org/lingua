//! Declarations: the prelude, each module's file-level names and imports (`L010`, `L017`),
//! type expressions, signatures, file-level consts, and the `primitive` allowlist (`L049`).
//! Bodies are `body.rs`; folding comptime roots is `comptime.rs`.
//!
//! Declarations resolve on demand (docs/lingua1.md, Compilation pipeline, Work items): a use
//! of a type, function, or const declared later in the module resolves it first, and a
//! comptime root that calls a function checks that body first. Each item in progress is on
//! `Checker.stack`; meeting one again is `L061`.

use std::cell::Cell;
use std::collections::{HashMap, HashSet};

use super::body::FnCx;
use super::package::Loaded;
use super::types::{self, FULL};
use super::{Checked, Diagnostic, NOT_YET};
use crate::ast::{Decl, Effect, Expr, FnBody, Span, TypeExpr};
use crate::lex::Location;
use crate::tir::{
    Range, Symbol, SymbolId, SymbolKind, TirExpr, TirFn, TirKind, TirModule, Type, TypeKind,
    prelude,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Builtin {
    Int,
    Text,
    Bool,
    Nothing,
    List,
    Result,
}

/// What a name means at a use site.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Binding {
    Builtin(Builtin),
    Type(SymbolId),
    Variant(SymbolId),
    Function(SymbolId),
    PreludeFn(SymbolId),
    Log,
    Resource(SymbolId),
    /// A file-level `const`.
    Const(SymbolId),
    Local(SymbolId),
    /// A prelude name that only comptime code can use.
    Comptime,
    /// Its declaration already has a diagnostic; uses report nothing more.
    Poison,
}

/// A function or resource operation signature.
#[derive(Debug, Clone)]
pub(super) struct Sig {
    pub params: Vec<(String, Type)>,
    pub ids: Vec<SymbolId>,
    pub ret: Type,
    pub effect: Effect,
    pub module: usize,
}

#[derive(Debug, Default)]
pub(super) struct ModuleCx {
    pub names: HashMap<String, Binding>,
    /// Visible file-level names in the order they were bound, for `L011` help.
    pub order: Vec<String>,
    pub exports: Vec<(String, Binding)>,
    pub declared: bool,
}

pub(super) struct Checker<'a> {
    pub loaded: &'a Loaded,
    pub symbols: Vec<Symbol>,
    pub diags: Vec<Diagnostic>,
    pub mods: Vec<ModuleCx>,
    pub sigs: HashMap<SymbolId, Sig>,
    pub fields: HashMap<SymbolId, Vec<(String, Type)>>,
    pub variants: HashMap<SymbolId, Vec<SymbolId>>,
    pub resource_ops: HashMap<SymbolId, SymbolId>,
    pub poisoned: HashSet<SymbolId>,
    pub resource_calls: Vec<(String, Diagnostic)>,
    pub prelude: Vec<(&'static str, Binding)>,
    /// Where each type, function, const, and resource is declared: module and decl index.
    pub decl_of: HashMap<SymbolId, (usize, usize)>,
    pub decl_ids: HashMap<(usize, usize), SymbolId>,
    pub resolved: HashSet<SymbolId>,
    /// The items being resolved, checked, or evaluated, outermost first.
    pub stack: Vec<Item>,
    /// Checked function bodies and the module of each.
    pub bodies: HashMap<SymbolId, (usize, TirFn)>,
    pub checked: HashSet<SymbolId>,
    /// Functions whose check had a diagnostic or met a poisoned name: their examples do not
    /// run, and comptime code cannot call them.
    pub tainted: HashSet<SymbolId>,
    /// Folded consts, file-level and inner, as literal TIR.
    pub consts: HashMap<SymbolId, crate::tir::TirExpr>,
    /// Set when a use meets a poisoned name, which reports nothing more.
    pub hit_poison: Cell<bool>,
    /// Each module's `provided/` cache, read on first use.
    pub caches: HashMap<usize, HashMap<String, crate::provider::Generated>>,
    /// Expansions: module, the generated name's span, the name, and what it expanded to.
    pub generated: Vec<(usize, Span, String, crate::provider::Generated)>,
    /// Modules where a provider failed: their cache is left as it was.
    pub provider_failed: HashSet<usize>,
    /// Package `comptime` blocks already run: module and the block's start.
    pub blocks_done: HashSet<(usize, usize, usize)>,
}

/// A work item on the comptime stack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Item {
    Decl(SymbolId),
    Body(SymbolId),
}

const COMPTIME_PRELUDE: [&str; 23] = [
    "Expr",
    "Type",
    "http_get",
    "NetError",
    "NetUnavailable",
    "BadStatus",
    "FsError",
    "NotFound",
    "EscapesRoot",
    "BadEncoding",
    "types_from_openapi",
    "schema",
    "Column",
    "SumVariant",
    "RecordField",
    "record_from_columns",
    "record_from_fields",
    "sum_from_variants",
    "DbMetaError",
    "NotConfigured",
    "DbUnavailable",
    "quote",
    "unquote",
];

// The prelude table's names, in table order
fn prelude_names() -> Vec<(&'static str, Binding)> {
    let mut names = vec![
        ("Int", Binding::Builtin(Builtin::Int)),
        ("Text", Binding::Builtin(Builtin::Text)),
        ("Bool", Binding::Builtin(Builtin::Bool)),
        ("Nothing", Binding::Builtin(Builtin::Nothing)),
        ("List", Binding::Builtin(Builtin::List)),
        ("Result", Binding::Builtin(Builtin::Result)),
        ("BoundsError", Binding::Type(prelude::BOUNDS_ERROR)),
        ("OutOfRange", Binding::Variant(prelude::OUT_OF_RANGE)),
        ("NotAnInteger", Binding::Variant(prelude::NOT_AN_INTEGER)),
        ("trim", Binding::PreludeFn(prelude::TRIM)),
        ("lower", Binding::PreludeFn(prelude::LOWER)),
        ("split", Binding::PreludeFn(prelude::SPLIT)),
        ("join", Binding::PreludeFn(prelude::JOIN)),
        ("max", Binding::PreludeFn(prelude::MAX)),
        ("length", Binding::PreludeFn(prelude::LENGTH)),
        ("log", Binding::Log),
        ("Process", Binding::Type(prelude::PROCESS)),
        ("Thread", Binding::Type(prelude::THREAD)),
    ];
    names.extend(
        COMPTIME_PRELUDE
            .iter()
            .filter(|n| !matches!(**n, "quote" | "unquote"))
            .map(|n| {
                let binding = match *n {
                    "Column" => Binding::Type(prelude::COLUMN),
                    "SumVariant" => Binding::Type(prelude::SUM_VARIANT),
                    "RecordField" => Binding::Type(prelude::RECORD_FIELD),
                    "record_from_columns" => Binding::PreludeFn(prelude::RECORD_FROM_COLUMNS),
                    "record_from_fields" => Binding::PreludeFn(prelude::RECORD_FROM_FIELDS),
                    "sum_from_variants" => Binding::PreludeFn(prelude::SUM_FROM_VARIANTS),
                    _ => Binding::Comptime,
                };
                (*n, binding)
            }),
    );
    names
}

/// The span of the first whole-word `word` inside `within`, or `within` itself.
pub(super) fn find_word(source: &str, within: Span, word: &str) -> Span {
    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    for (i, line) in source
        .lines()
        .enumerate()
        .skip(within.start.line.saturating_sub(1))
        .take(within.end.line.saturating_sub(within.start.line) + 1)
    {
        let line_no = i + 1;
        let chars: Vec<char> = line.chars().collect();
        let from = if line_no == within.start.line {
            within.start.column.saturating_sub(1)
        } else {
            0
        };
        let word: Vec<char> = word.chars().collect();
        for at in from..chars.len() {
            if chars[at..].starts_with(&word)
                && (at == 0 || !is_ident(chars[at - 1]))
                && chars.get(at + word.len()).is_none_or(|c| !is_ident(*c))
            {
                return Span {
                    start: Location {
                        line: line_no,
                        column: at + 1,
                    },
                    end: Location {
                        line: line_no,
                        column: at + 1 + word.len(),
                    },
                };
            }
        }
    }
    within
}

/// The first `len` columns of `span`: a keyword at the start of a statement.
pub(super) fn word_at(span: Span, len: usize) -> Span {
    Span {
        start: span.start,
        end: Location {
            line: span.start.line,
            column: span.start.column + len,
        },
    }
}

/// A list of names for a `help:` line, capped at 64.
pub(super) fn alternatives<S: AsRef<str>>(names: &[S]) -> String {
    let shown: Vec<&str> = names.iter().take(64).map(AsRef::as_ref).collect();
    let mut out = format!("legal alternatives: {}", shown.join(", "));
    if names.len() > 64 {
        out = format!("{out}, and {} more", names.len() - 64);
    }
    out
}

impl<'a> Checker<'a> {
    pub fn new(loaded: &'a Loaded) -> Self {
        let symbols = prelude::symbols();
        let mut fields = HashMap::new();
        let field_list = |parent: SymbolId| {
            symbols
                .iter()
                .filter(|s| matches!(s.kind, SymbolKind::Field { parent: p, .. } if p == parent))
                .map(|s| (s.name.clone(), s.ty.clone()))
                .collect::<Vec<_>>()
        };
        fields.insert(prelude::PROCESS, field_list(prelude::PROCESS));
        fields.insert(prelude::THREAD, field_list(prelude::THREAD));
        for product in [prelude::COLUMN, prelude::SUM_VARIANT, prelude::RECORD_FIELD] {
            fields.insert(product, field_list(product));
        }
        let mut variants = HashMap::new();
        variants.insert(
            prelude::BOUNDS_ERROR,
            vec![prelude::OUT_OF_RANGE, prelude::NOT_AN_INTEGER],
        );
        Checker {
            loaded,
            symbols,
            diags: Vec::new(),
            mods: loaded.sources.iter().map(|_| ModuleCx::default()).collect(),
            sigs: HashMap::new(),
            fields,
            variants,
            resource_ops: HashMap::new(),
            poisoned: HashSet::new(),
            resource_calls: Vec::new(),
            prelude: prelude_names(),
            decl_of: HashMap::new(),
            decl_ids: HashMap::new(),
            resolved: HashSet::new(),
            stack: Vec::new(),
            bodies: HashMap::new(),
            checked: HashSet::new(),
            tainted: HashSet::new(),
            consts: HashMap::new(),
            hit_poison: Cell::new(false),
            caches: HashMap::new(),
            generated: Vec::new(),
            provider_failed: HashSet::new(),
            blocks_done: HashSet::new(),
        }
    }

    pub fn source(&self, m: usize) -> &'a str {
        &self.loaded.sources[m].text
    }

    pub fn path(&self, m: usize) -> &'a str {
        &self.loaded.sources[m].path
    }

    pub fn key(&self, m: usize) -> &'a str {
        &self.loaded.sources[m].key
    }

    pub fn diag(&self, m: usize, code: &'static str, span: Span, summary: String) -> Diagnostic {
        Diagnostic::new(code, summary, self.path(m)).at(self.source(m), span)
    }

    pub fn report(&mut self, m: usize, code: &'static str, span: Span, summary: String) {
        let d = self.diag(m, code, span, summary);
        self.diags.push(d);
    }

    pub fn report_help(
        &mut self,
        m: usize,
        code: &'static str,
        span: Span,
        summary: String,
        help: String,
    ) {
        let d = self.diag(m, code, span, summary).help(help);
        self.diags.push(d);
    }

    /// `error: WHAT is not supported yet`, and why when the parenthetical gives a reason.
    pub fn not_yet(&mut self, m: usize, span: Span, what: &str) {
        let summary = match what.strip_suffix(')').and_then(|w| w.split_once(" (")) {
            Some((what, why)) => format!("{what} is not supported yet: {why}"),
            None => format!("{what} is not supported yet"),
        };
        self.report(m, NOT_YET, span, summary);
    }

    pub fn show(&self, ty: &Type) -> String {
        types::show(&self.symbols, ty)
    }

    pub fn push_symbol(
        &mut self,
        name: &str,
        kind: SymbolKind,
        ty: Type,
        m: usize,
        span: Span,
    ) -> SymbolId {
        self.symbols.push(Symbol {
            name: name.to_owned(),
            kind,
            ty,
            effect: None,
            mutable: false,
            span,
            module: self.key(m).to_owned(),
        });
        SymbolId::try_from(self.symbols.len() - 1).unwrap_or(SymbolId::MAX)
    }

    pub fn prelude_binding(&self, name: &str) -> Option<Binding> {
        self.prelude
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, b)| *b)
    }

    /// A file-level name: the module's own declarations and imports, then the prelude.
    pub fn lookup(&self, m: usize, name: &str) -> Option<Binding> {
        let found = self.mods[m]
            .names
            .get(name)
            .copied()
            .or_else(|| self.prelude_binding(name));
        if found == Some(Binding::Poison) {
            self.hit_poison.set(true);
        }
        found
    }

    /// Whether `id` is poisoned; a use of it reports nothing more.
    pub fn is_poisoned(&self, id: SymbolId) -> bool {
        let poisoned = self.poisoned.contains(&id);
        if poisoned {
            self.hit_poison.set(true);
        }
        poisoned
    }

    /// Every file-level name visible in `m`, for `L011` help.
    pub fn visible(&self, m: usize) -> Vec<String> {
        self.mods[m]
            .order
            .iter()
            .cloned()
            .chain(self.prelude.iter().map(|(n, _)| (*n).to_owned()))
            .collect()
    }

    // L017 when a file-level name repeats a declaration, an import, or a prelude name
    pub(super) fn declare(&mut self, m: usize, name: &str, binding: Binding, span: Span) {
        let span = find_word(self.source(m), span, name);
        if self.prelude_binding(name).is_some() {
            self.report(
                m,
                "L017",
                span,
                format!("`{name}` is a prelude name and cannot be declared"),
            );
        } else if self.mods[m].names.contains_key(name) {
            self.report(
                m,
                "L017",
                span,
                format!("`{name}` is already declared or imported in this module"),
            );
        } else {
            self.mods[m].names.insert(name.to_owned(), binding);
            self.mods[m].order.push(name.to_owned());
        }
    }

    pub(super) fn export(&mut self, m: usize, name: &str, binding: Binding) {
        self.mods[m].exports.push((name.to_owned(), binding));
    }

    // An imported name; a sum also brings its constructors. The same binding twice through
    // a type and its constructor is one binding.
    fn import_name(&mut self, m: usize, name: &str, binding: Binding, span: Span, explicit: bool) {
        match self.mods[m].names.get(name) {
            Some(existing) if *existing == binding && !explicit => {}
            Some(_) => {
                self.report(
                    m,
                    "L017",
                    span,
                    format!("`{name}` is already imported or declared in this module"),
                );
            }
            None => {
                self.mods[m].names.insert(name.to_owned(), binding);
                self.mods[m].order.push(name.to_owned());
            }
        }
    }

    fn imports(&mut self, m: usize) {
        let Some(module) = &self.loaded.sources[m].module else {
            return;
        };
        let mut explicit: HashSet<String> = HashSet::new();
        for import in &module.imports {
            let target = self.loaded.index(&import.path).filter(|&t| {
                t != m && self.loaded.sources[t].module.is_some() && self.mods[t].declared
            });
            for name in &import.names {
                let span = find_word(self.source(m), import.span, name);
                let Some(t) = target else {
                    // L016 or L015 is already reported for this import
                    self.mods[m]
                        .names
                        .entry(name.clone())
                        .or_insert(Binding::Poison);
                    continue;
                };
                let found = self.mods[t]
                    .exports
                    .iter()
                    .find(|(n, _)| n == name)
                    .map(|(_, b)| *b);
                let Some(binding) = found else {
                    let exports: Vec<String> = self.mods[t]
                        .exports
                        .iter()
                        .map(|(n, _)| n.clone())
                        .collect();
                    let summary = format!("{} does not export `{name}`", self.path(t));
                    self.report_help(m, "L010", span, summary, alternatives(&exports));
                    self.mods[m]
                        .names
                        .entry(name.clone())
                        .or_insert(Binding::Poison);
                    continue;
                };
                let again = !explicit.insert(name.clone());
                if again {
                    self.report(m, "L017", span, format!("`{name}` is imported twice"));
                    continue;
                }
                self.import_name(m, name, binding, span, true);
                if let Binding::Type(id) = binding {
                    for variant in self.variants.get(&id).cloned().unwrap_or_default() {
                        let vname = self.symbols[variant as usize].name.clone();
                        if explicit.contains(&vname) {
                            continue;
                        }
                        self.import_name(m, &vname, Binding::Variant(variant), span, false);
                    }
                }
            }
        }
    }

    /// Folds a comptime bound to an integer. Literal arithmetic folds here; anything else is
    /// typechecked as a comptime root over `scopes` (a runtime local there is `L060`) and
    /// evaluated with its own fuel.
    pub fn fold_bound(
        &mut self,
        m: usize,
        e: &Expr,
        scopes: &[Vec<(String, SymbolId)>],
    ) -> Option<i64> {
        match e {
            Expr::IntLit(_, v) => Some(*v),
            Expr::UnaryNeg(span, inner) => {
                let v = self.fold_bound(m, inner, scopes)?;
                v.checked_neg().or_else(|| {
                    self.report(m, "L040", *span, "arithmetic overflow in a bound".into());
                    None
                })
            }
            Expr::Binary(span, op, a, b) => {
                let (a, b) = (self.fold_bound(m, a, scopes), self.fold_bound(m, b, scopes));
                let (a, b) = (a?, b?);
                if matches!(op, crate::ast::BinOp::Div | crate::ast::BinOp::Rem) && b == 0 {
                    self.report(m, "L076", *span, "division by zero in a bound".into());
                    return None;
                }
                if matches!(
                    op,
                    crate::ast::BinOp::Add
                        | crate::ast::BinOp::Sub
                        | crate::ast::BinOp::Mul
                        | crate::ast::BinOp::Div
                        | crate::ast::BinOp::Rem
                ) {
                    types::fold(*op, a, b).or_else(|| {
                        self.report(m, "L040", *span, "arithmetic overflow in a bound".into());
                        None
                    })
                } else {
                    self.report(
                        m,
                        "L020",
                        *span,
                        "a bound must be an integer expression".into(),
                    );
                    None
                }
            }
            _ => {
                let mut cx = FnCx::root(m, Effect::Pure);
                cx.scopes = scopes.to_vec();
                let root = self.synth(&mut cx, e)?;
                if !types::is_int(&self.symbols, &root.ty) {
                    let shown = self.show(&root.ty);
                    self.report(
                        m,
                        "L020",
                        e.span(),
                        format!("a bound must be an integer expression, not `{shown}`"),
                    );
                    return None;
                }
                match self.fold(m, &root, "a bound")?.kind {
                    TirKind::Int(v) => Some(v),
                    _ => None,
                }
            }
        }
    }

    fn bound_base(&mut self, m: usize, base: &TypeExpr) -> Option<()> {
        match self.resolve_type(m, base)? {
            Type::Int => Some(()),
            other => {
                let shown = self.show(&other);
                self.report(
                    m,
                    "L020",
                    base.span(),
                    format!("a bound's base must be `Int`, not `{shown}`"),
                );
                None
            }
        }
    }

    /// The type a type expression names.
    pub fn resolve_type(&mut self, m: usize, ty: &TypeExpr) -> Option<Type> {
        match ty {
            TypeExpr::Named(span, name) => match self.lookup(m, name) {
                Some(Binding::Builtin(b)) => match b {
                    Builtin::Int => Some(Type::Int),
                    Builtin::Text => Some(Type::Text),
                    Builtin::Bool => Some(Type::Bool),
                    Builtin::Nothing => Some(Type::Nothing),
                    Builtin::List | Builtin::Result => {
                        let n = if b == Builtin::List {
                            "one type argument"
                        } else {
                            "two type arguments"
                        };
                        self.report(m, "L028", *span, format!("`{name}` takes {n}"));
                        None
                    }
                },
                Some(Binding::Type(id)) => self.ensure(id).then_some(Type::Nominal(id)),
                Some(Binding::Comptime) => {
                    self.not_yet(m, *span, &format!("the comptime type `{name}`"));
                    None
                }
                Some(Binding::Poison) => None,
                Some(_) => {
                    self.report(m, "L020", *span, format!("`{name}` is not a type"));
                    None
                }
                None => {
                    let help = alternatives(&self.visible_types(m));
                    self.report_help(m, "L011", *span, format!("unknown type `{name}`"), help);
                    None
                }
            },
            TypeExpr::Apply(span, name, args) => {
                let want = match self.lookup(m, name) {
                    Some(Binding::Builtin(Builtin::List)) => 1,
                    Some(Binding::Builtin(Builtin::Result)) => 2,
                    _ => {
                        self.report(
                            m,
                            "L020",
                            *span,
                            "only `List` and `Result` take type arguments".into(),
                        );
                        return None;
                    }
                };
                if args.len() != want {
                    self.report(
                        m,
                        "L028",
                        *span,
                        format!(
                            "`{name}` takes {want} type argument{}",
                            if want == 1 { "" } else { "s" }
                        ),
                    );
                    return None;
                }
                let args: Vec<Option<Type>> =
                    args.iter().map(|a| self.resolve_type(m, a)).collect();
                let mut args = args.into_iter().collect::<Option<Vec<Type>>>()?.into_iter();
                let first = Box::new(args.next()?);
                Some(match args.next() {
                    Some(err) => Type::Result(first, Box::new(err)),
                    None => Type::List(first),
                })
            }
            TypeExpr::AtLeast(_, base, lo) => {
                let base_ok = self.bound_base(m, base);
                let lo = self.fold_bound(m, lo, &[])?;
                base_ok?;
                Some(types::of_range(Range { lo, hi: i64::MAX }))
            }
            TypeExpr::Range(span, base, lo, hi) => {
                let base_ok = self.bound_base(m, base);
                let lo = self.fold_bound(m, lo, &[]);
                let hi = self.fold_bound(m, hi, &[]);
                let (lo, hi) = (lo?, hi?);
                base_ok?;
                if lo > hi {
                    self.report(
                        m,
                        "L024",
                        *span,
                        format!("empty range: {lo} is greater than {hi}"),
                    );
                    return None;
                }
                Some(types::of_range(Range { lo, hi }))
            }
        }
    }

    fn visible_types(&self, m: usize) -> Vec<String> {
        self.visible(m)
            .into_iter()
            .filter(|n| {
                matches!(
                    self.lookup(m, n),
                    Some(Binding::Type(_) | Binding::Builtin(_))
                )
            })
            .collect()
    }

    // First pass: allocate a symbol for every file-level name
    #[allow(clippy::too_many_lines)]
    fn collect(&mut self, m: usize) {
        let Some(module) = &self.loaded.sources[m].module else {
            return;
        };
        for (i, decl) in module.decls.iter().enumerate() {
            let first = self.symbols.len();
            match decl {
                Decl::Alias { span, name, .. } | Decl::Product { span, name, .. } => {
                    let kind = if matches!(decl, Decl::Product { .. }) {
                        TypeKind::Product
                    } else {
                        TypeKind::Newtype {
                            base: Type::Int,
                            range: None,
                        }
                    };
                    let id =
                        self.push_symbol(name, SymbolKind::Type(kind), Type::Nothing, m, *span);
                    self.symbols[id as usize].ty = Type::Nominal(id);
                    self.declare(m, name, Binding::Type(id), *span);
                    self.export(m, name, Binding::Type(id));
                }
                Decl::Sum {
                    span,
                    name,
                    variants,
                } => {
                    let id = self.push_symbol(
                        name,
                        SymbolKind::Type(TypeKind::Sum),
                        Type::Nothing,
                        m,
                        *span,
                    );
                    self.symbols[id as usize].ty = Type::Nominal(id);
                    self.declare(m, name, Binding::Type(id), *span);
                    self.export(m, name, Binding::Type(id));
                    let mut ids = Vec::new();
                    for (tag, v) in (0..).zip(variants) {
                        let kind = SymbolKind::Variant {
                            parent: id,
                            tag,
                            payload: None,
                        };
                        let vid = self.push_symbol(&v.name, kind, Type::Nominal(id), m, v.span);
                        self.declare(m, &v.name, Binding::Variant(vid), v.span);
                        self.export(m, &v.name, Binding::Variant(vid));
                        ids.push(vid);
                    }
                    self.variants.insert(id, ids);
                }
                Decl::Function {
                    span, name, effect, ..
                } => {
                    let id = self.push_symbol(name, SymbolKind::Function, Type::Nothing, m, *span);
                    self.symbols[id as usize].effect = Some(effect.clone());
                    self.declare(m, name, Binding::Function(id), *span);
                    self.export(m, name, Binding::Function(id));
                }
                Decl::Resource { span, name, op } => {
                    let id = self.push_symbol(name, SymbolKind::Resource, Type::Nothing, m, *span);
                    let op_id = self.push_symbol(
                        &op.name,
                        SymbolKind::ResourceOp { resource: id },
                        Type::Nothing,
                        m,
                        op.span,
                    );
                    self.resource_ops.insert(id, op_id);
                    self.declare(m, name, Binding::Resource(id), *span);
                }
                Decl::Generative { span, name, .. } => {
                    self.generated_name(m, name, *span);
                }
                Decl::Const { span, name, .. } => {
                    let id = self.push_symbol(name, SymbolKind::Const, Type::Nothing, m, *span);
                    self.declare(m, name, Binding::Const(id), *span);
                    self.export(m, name, Binding::Const(id));
                }
                // Checked after the bodies, in `check_package`
                Decl::PackageExample { .. } => {}
                Decl::ComptimeBlock { body, .. } => {
                    for stmt in body {
                        if let crate::ast::Stmt::Provide { span, name, .. } = stmt {
                            let id = self.generated_name(m, name, *span);
                            self.decl_of.insert(id, (m, i));
                            self.decl_ids.entry((m, i)).or_insert(id);
                        }
                    }
                }
                Decl::Provide { span, .. } => {
                    self.report(
                        m,
                        "L067",
                        *span,
                        "`provide` is legal only in a package `comptime` block".into(),
                    );
                }
            }
            // The declaration's own symbol is the first it pushed
            if matches!(
                decl,
                Decl::Alias { .. }
                    | Decl::Product { .. }
                    | Decl::Sum { .. }
                    | Decl::Function { .. }
                    | Decl::Const { .. }
                    | Decl::Resource { .. }
                    | Decl::Generative { .. }
            ) && let Ok(id) = SymbolId::try_from(first)
            {
                self.decl_of.insert(id, (m, i));
                self.decl_ids.insert((m, i), id);
            }
        }
    }

    /// Resolves the declaration of `id` now if it has not been; `false` when it is part of a
    /// comptime cycle (`L061`). A product or sum met again is a recursive type, not a cycle.
    pub fn ensure(&mut self, id: SymbolId) -> bool {
        if self.resolved.contains(&id) {
            return true;
        }
        let Some(&(m, i)) = self.decl_of.get(&id) else {
            return true;
        };
        if self.stack.contains(&Item::Decl(id)) {
            if matches!(
                self.symbols[id as usize].kind,
                SymbolKind::Type(TypeKind::Product | TypeKind::Sum)
            ) {
                return true;
            }
            self.cycle(Item::Decl(id));
            return false;
        }
        let Some(module) = &self.loaded.sources[m].module else {
            return true;
        };
        self.stack.push(Item::Decl(id));
        self.resolve_decl(m, &module.decls[i]);
        self.stack.pop();
        self.resolved.insert(id);
        true
    }

    /// `L061` for `again`, which is already on the stack: the stack from it to the top, then
    /// `again`, at the declaration that repeats.
    pub fn cycle(&mut self, again: Item) {
        let id = |item: &Item| match *item {
            Item::Decl(id) | Item::Body(id) => id,
        };
        let from = self.stack.iter().position(|i| *i == again).unwrap_or(0);
        let names: Vec<String> = self.stack[from..]
            .iter()
            .chain([&again])
            .map(|item| format!("`{}`", self.symbols[id(item) as usize].name))
            .collect();
        let symbol = &self.symbols[id(&again) as usize];
        let (name, span) = (symbol.name.clone(), symbol.span);
        let m = self.decl_of.get(&id(&again)).map_or(0, |&(m, _)| m);
        let span = find_word(self.source(m), span, &name);
        let summary = format!("comptime dependency cycle: {}", names.join(" -> "));
        self.report(m, "L061", span, summary);
    }

    // Second pass: resolve types, signatures, and consts, in source order unless a use
    // resolved one sooner
    fn resolve(&mut self, m: usize) {
        let Some(module) = &self.loaded.sources[m].module else {
            return;
        };
        for (i, decl) in module.decls.iter().enumerate() {
            match self.decl_ids.get(&(m, i)) {
                Some(&id) => {
                    self.ensure(id);
                }
                None => self.resolve_decl(m, decl),
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_decl(&mut self, m: usize, decl: &Decl) {
        match decl {
            Decl::Alias { name, ty, .. } => self.alias(m, name, ty),
            Decl::Product { name, fields, .. } => {
                let Some(Binding::Type(id)) = self.mods[m].names.get(name).copied() else {
                    return;
                };
                let mut list: Vec<(String, Type)> = Vec::new();
                for f in fields {
                    let ty = self.resolve_type(m, &f.ty);
                    if list.iter().any(|(n, _)| *n == f.name) {
                        let span = find_word(self.source(m), f.span, &f.name);
                        self.report(
                            m,
                            "L017",
                            span,
                            format!("field `{}` is declared twice", f.name),
                        );
                        continue;
                    }
                    match ty {
                        Some(ty) => list.push((f.name.clone(), ty)),
                        None => {
                            self.poisoned.insert(id);
                        }
                    }
                }
                self.fields.insert(id, list);
            }
            Decl::Sum { name, variants, .. } => {
                let Some(Binding::Type(id)) = self.mods[m].names.get(name).copied() else {
                    return;
                };
                let ids = self.variants.get(&id).cloned().unwrap_or_default();
                for (v, vid) in variants.iter().zip(ids) {
                    let Some(payload_expr) = &v.payload else {
                        continue;
                    };
                    let payload = self.resolve_type(m, payload_expr);
                    if payload.is_none() {
                        self.poisoned.insert(vid);
                    }
                    if let SymbolKind::Variant { payload: p, .. } =
                        &mut self.symbols[vid as usize].kind
                    {
                        *p = payload;
                    }
                }
            }
            Decl::Function {
                span,
                name,
                params,
                ret,
                effect,
                body,
                ..
            } => {
                let Some(Binding::Function(id)) = self.mods[m].names.get(name).copied() else {
                    return;
                };
                self.check_effect(m, *span, effect);
                let mut sig = Sig {
                    params: Vec::new(),
                    ids: Vec::new(),
                    ret: Type::Nothing,
                    effect: effect.clone(),
                    module: m,
                };
                let mut ok = true;
                for p in params {
                    let ty = self.resolve_type(m, &p.ty);
                    let pspan = find_word(self.source(m), p.span, &p.name);
                    if sig.params.iter().any(|(n, _)| *n == p.name) {
                        self.report(
                            m,
                            "L013",
                            pspan,
                            format!("parameter `{}` is declared twice", p.name),
                        );
                    } else if self.lookup(m, &p.name).is_some() {
                        self.report(
                            m,
                            "L013",
                            pspan,
                            format!("parameter `{}` shadows a visible name", p.name),
                        );
                    }
                    let ty = ty.unwrap_or_else(|| {
                        ok = false;
                        Type::Nothing
                    });
                    let pid = self.push_symbol(&p.name, SymbolKind::Param, ty.clone(), m, p.span);
                    if !ok {
                        self.poisoned.insert(pid);
                    }
                    sig.params.push((p.name.clone(), ty));
                    sig.ids.push(pid);
                }
                match self.resolve_type(m, ret) {
                    Some(ty) => sig.ret = ty,
                    None => ok = false,
                }
                if !ok {
                    self.poisoned.insert(id);
                }
                self.symbols[id as usize].ty = sig.ret.clone();
                if matches!(body, FnBody::Primitive) && ok && !self.is_percent(m, name, &sig) {
                    self.report(
                        m,
                        "L049",
                        *span,
                        format!(
                            "`primitive` is allowed only on `money.percent`, not `{}.{name}`",
                            self.key(m)
                        ),
                    );
                }
                self.sigs.insert(id, sig);
            }
            Decl::Resource { name, op, .. } => {
                let Some(Binding::Resource(id)) = self.mods[m].names.get(name).copied() else {
                    return;
                };
                let op_id = self.resource_ops[&id];
                let mut sig = Sig {
                    params: Vec::new(),
                    ids: Vec::new(),
                    ret: Type::Nothing,
                    effect: match op.effect {
                        crate::ast::ResourceEffect::Read => Effect::Reads(name.clone()),
                        crate::ast::ResourceEffect::Write => Effect::Writes(name.clone()),
                    },
                    module: m,
                };
                let mut ok = true;
                for p in &op.params {
                    match self.resolve_type(m, &p.ty) {
                        Some(ty) => sig.params.push((p.name.clone(), ty)),
                        None => ok = false,
                    }
                }
                match self.resolve_type(m, &op.ret) {
                    Some(ty) => sig.ret = ty,
                    None => ok = false,
                }
                if !ok {
                    self.poisoned.insert(op_id);
                }
                self.symbols[op_id as usize].ty = sig.ret.clone();
                self.symbols[op_id as usize].effect = Some(sig.effect.clone());
                self.sigs.insert(op_id, sig);
            }
            Decl::Const {
                span,
                name,
                ty,
                effect,
                init,
            } => {
                if let Some(Binding::Const(id)) = self.mods[m].names.get(name).copied() {
                    self.file_const(m, id, *span, ty.as_ref(), effect, init);
                }
            }
            Decl::Generative {
                span,
                name,
                effect,
                body,
            } => self.generative(m, *span, name, effect, body),
            Decl::ComptimeBlock { span, effect, body } => {
                self.comptime_block(m, *span, effect, body);
            }
            _ => {}
        }
    }

    // A file-level const: pure, or a provider effect (`--offline` runs none); its value is
    // folded now
    fn file_const(
        &mut self,
        m: usize,
        id: SymbolId,
        span: Span,
        ty: Option<&TypeExpr>,
        effect: &Effect,
        init: &Expr,
    ) {
        match effect {
            Effect::Pure => {}
            Effect::ReadsNet | Effect::ReadsFs | Effect::ReadsCatalog => {
                if self.loaded.cache != super::Cache::Offline {
                    return self.fold_file_const(m, id, ty, effect, init);
                }
                // ponytail: lingua1.md caches only generated declarations, so an effectful
                // const has nothing to reuse offline; it is `L071` rather than a silent read.
                let name = self.symbols[id as usize].name.clone();
                let span = find_word(self.source(m), span, &name);
                self.report_help(
                    m,
                    "L071",
                    span,
                    format!("`--offline` and the const `{name}` needs its provider effect"),
                    "run without `--offline` to run its effect".into(),
                );
                self.poisoned.insert(id);
                return;
            }
            _ => {
                self.report(
                    m,
                    "L025",
                    span,
                    "a `const` may be `pure`, `reads net`, `reads fs`, or `reads catalog`".into(),
                );
                self.poisoned.insert(id);
                return;
            }
        }
        self.fold_file_const(m, id, ty, effect, init);
    }

    fn fold_file_const(
        &mut self,
        m: usize,
        id: SymbolId,
        ty: Option<&TypeExpr>,
        effect: &Effect,
        init: &Expr,
    ) {
        let name = self.symbols[id as usize].name.clone();
        let mut cx = FnCx::root(m, effect.clone());
        let declared = ty.map(|t| self.resolve_type(m, t));
        let tir = match declared {
            Some(None) => None,
            Some(Some(t)) => self
                .check(&mut cx, init, &t)
                .map(|e| TirExpr { ty: t, ..e }),
            None => self.synth(&mut cx, init),
        };
        let value = tir.and_then(|e| {
            self.symbols[id as usize].ty = e.ty.clone();
            self.fold(m, &e, &format!("`{name}`"))
        });
        match value {
            Some(v) => {
                self.consts.insert(id, v);
            }
            None => {
                self.poisoned.insert(id);
            }
        }
    }

    fn alias(&mut self, m: usize, name: &str, ty: &TypeExpr) {
        let Some(Binding::Type(id)) = self.mods[m].names.get(name).copied() else {
            return;
        };
        let kind = match self.resolve_type(m, ty) {
            Some(Type::Text) => TypeKind::Newtype {
                base: Type::Text,
                range: None,
            },
            Some(ty @ (Type::Int | Type::Bound { .. })) => TypeKind::Newtype {
                base: Type::Int,
                range: types::range(&self.symbols, &ty).filter(|r| *r != FULL),
            },
            Some(other) => {
                let shown = self.show(&other);
                self.report(
                    m,
                    "L020",
                    ty.span(),
                    format!(
                        "a newtype's base must be `Int`, an integer bound, or `Text`, not `{shown}`"
                    ),
                );
                self.poisoned.insert(id);
                return;
            }
            None => {
                self.poisoned.insert(id);
                return;
            }
        };
        self.symbols[id as usize].kind = SymbolKind::Type(kind);
    }

    // reads net / fs / catalog are not function effects; reads/writes R names a resource here
    fn check_effect(&mut self, m: usize, span: Span, effect: &Effect) {
        match effect {
            Effect::Pure | Effect::ReadsClock => {}
            Effect::ReadsNet | Effect::ReadsFs | Effect::ReadsCatalog => {
                self.report(m, "L025", span, "`reads net`, `reads fs`, and `reads catalog` are legal only on a const or a generative type, not on a function".into());
            }
            Effect::Reads(r) | Effect::Writes(r) => {
                if !matches!(self.mods[m].names.get(r), Some(Binding::Resource(_))) {
                    let resources: Vec<String> = self.mods[m]
                        .order
                        .iter()
                        .filter(|n| {
                            matches!(self.mods[m].names.get(*n), Some(Binding::Resource(_)))
                        })
                        .cloned()
                        .collect();
                    let span = find_word(self.source(m), span, r);
                    self.report_help(
                        m,
                        "L011",
                        span,
                        format!("no resource `{r}` is declared in this module"),
                        alternatives(&resources),
                    );
                }
            }
        }
    }

    // percent(part: Int in 1 through 100, of: Cents, round: Rounding) -> Cents, pure, in money
    fn is_percent(&self, m: usize, name: &str, sig: &Sig) -> bool {
        let named = |ty: &Type, want: &str| matches!(ty, Type::Nominal(id) if self.symbols[*id as usize].name == want);
        let cents = |ty: &Type| {
            named(ty, "Cents")
                && types::is_nominal_int(&self.symbols, ty)
                && types::range(&self.symbols, ty)
                    == Some(Range {
                        lo: 0,
                        hi: i64::MAX,
                    })
        };
        let param_names: Vec<&str> = sig.params.iter().map(|(n, _)| n.as_str()).collect();
        self.key(m) == "money"
            && name == "percent"
            && sig.effect == Effect::Pure
            && param_names == ["part", "of", "round"]
            && sig.params[0].1 == Type::Bound { lo: 1, hi: 100 }
            && cents(&sig.params[1].1)
            && named(&sig.params[2].1, "Rounding")
            && super::is_sum(&self.symbols, &sig.params[2].1)
            && cents(&sig.ret)
    }
}

/// Checks every loaded module: declarations in import order, then bodies and package
/// examples. A comptime root reached along the way resolves and checks what it needs first.
pub(super) fn check_package(loaded: &Loaded) -> (Checked, Vec<Diagnostic>) {
    let mut c = Checker::new(loaded);
    for m in 0..loaded.sources.len() {
        c.imports(m);
        c.collect(m);
        c.expand(m);
        c.resolve(m);
        c.mods[m].declared = true;
    }
    let mut modules = Vec::new();
    for (m, source) in loaded.sources.iter().enumerate() {
        let Some(module) = &source.module else {
            continue;
        };
        let mut ids = Vec::new();
        let mut examples = Vec::new();
        for (i, decl) in module.decls.iter().enumerate() {
            match decl {
                Decl::Function { .. } => {
                    if let Some(&id) = c.decl_ids.get(&(m, i)) {
                        c.body(id);
                        ids.push(id);
                    }
                }
                Decl::PackageExample { example, .. } => {
                    examples.extend(c.package_example(m, example));
                }
                _ => {}
            }
        }
        let functions = ids
            .iter()
            .filter_map(|id| c.bodies.remove(id).map(|(_, f)| f))
            .collect();
        modules.push(TirModule {
            key: source.key.clone(),
            path: source.path.clone(),
            source: source.text.clone(),
            functions,
            examples,
        });
    }
    let expanded = c.write_caches();
    let Checker {
        symbols,
        diags,
        resource_calls,
        tainted,
        consts,
        generated,
        ..
    } = c;
    (
        Checked {
            symbols,
            modules,
            requested: loaded.requested.clone(),
            resource_calls,
            skip: tainted,
            expanded,
            consts,
            generated,
        },
        diags,
    )
}

/// `L075` and the entry effect rule for the `main` at `checked.modules[m].functions[i]`.
pub(super) fn entry_rules(checked: &Checked, m: usize, i: usize) -> Vec<Diagnostic> {
    let module = &checked.modules[m];
    let main = &module.functions[i];
    let at = |code, span, summary: &str| {
        Diagnostic::new(code, summary, &module.path)
            .at(&module.source, span)
            .help("use `fn main() -> Nothing` or `fn main(process: Process) -> Nothing`")
    };
    let mut out = Vec::new();
    let process = Type::Nominal(prelude::PROCESS);
    let params: Vec<&Symbol> = main
        .params
        .iter()
        .map(|p| &checked.symbols[*p as usize])
        .collect();
    match params.as_slice() {
        [] => {}
        [p] if p.ty == process => {}
        [p, ..] => out.push(at(
            "L075",
            p.span,
            "`main` must take no parameters or one `Process`",
        )),
    }
    if main.ret != Type::Nothing {
        out.push(at("L075", main.span, "`main` must return `Nothing`"));
    }
    if !matches!(main.effect, Effect::Pure | Effect::ReadsClock) {
        out.push(
            Diagnostic::new(
                "L025",
                "`main` must be `pure` or `reads clock`",
                &module.path,
            )
            .at(&module.source, main.span),
        );
    }
    out
}
