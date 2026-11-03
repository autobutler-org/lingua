//! The binder and typechecker (docs/lingua1.md, Module and file model, Name resolution, Types,
//! Effects): a package of parsed modules in, the typed IR in `crate::tir` out.
//!
//! Comptime roots (`const`, `comptime`, bounds) fold on `crate::eval` as the checker reaches
//! them; each use of a folded value is literal TIR. Function examples, then package examples,
//! run on the same interpreter when their module and its imports checked clean.
//!
//! Generative types and package `comptime` blocks expand first in each module
//! (`generative.rs`), under the `provided/` cache policy, to ordinary product declarations.
//!
//! Out of scope until their PRs, and reported per declaration as
//! `error: ... not supported yet` rather than guessed at: `http_get`, `catalog.columns`,
//! `fs.read`, `schema`, `types_from_openapi`, and `quote` / `unquote`. Such a diagnostic does
//! not stop the module's examples, only those of functions whose check had a diagnostic or met
//! the unsupported declaration, and of examples that call them.

mod body;
mod catalog;
mod comptime;
mod decl;
mod generative;
mod package;
mod pattern;
mod types;

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::path::PathBuf;

use crate::ast::Span;
use crate::report;
use crate::tir::{SymbolId, SymbolKind, TirModule, TirProgram, Type};

pub use catalog::explain;
pub use package::{Cache, Options, is_module_key, resolve};

/// The code for a construct that is legal Lingua but that this checker cannot handle yet.
/// Not an `L` code.
pub const NOT_YET: &str = "error";

/// One issue: code, summary, where, and an optional `help:` line.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub summary: String,
    /// The path relative to the source root (`order.lf`), or the path as given.
    pub path: String,
    pub span: Option<Span>,
    /// The source line the span starts on.
    pub line: String,
    pub help: Option<String>,
    /// A runtime fault (`L040`, `L042`, `L046`, `L076`): rendered as `fault CODE: summary`.
    pub fault: bool,
}

impl Diagnostic {
    #[must_use]
    pub fn new(code: &'static str, summary: impl Into<String>, path: &str) -> Self {
        Diagnostic {
            code,
            summary: summary.into(),
            path: path.to_owned(),
            span: None,
            line: String::new(),
            help: None,
            fault: false,
        }
    }

    /// The fault report for `code` at `span`, the block `crate::report::fault` renders.
    #[must_use]
    pub fn fault(code: &'static str, path: &str, source: &str, span: Span) -> Self {
        Diagnostic {
            fault: true,
            ..Diagnostic::new(code, report::fault_summary(code), path).at(source, span)
        }
    }

    /// Points the diagnostic at `span` in `source`.
    #[must_use]
    pub fn at(self, source: &str, span: Span) -> Self {
        Diagnostic {
            span: Some(span),
            line: report::line_of(source, span).to_owned(),
            ..self
        }
    }

    #[must_use]
    pub fn help(self, help: impl Into<String>) -> Self {
        Diagnostic {
            help: Some(help.into()),
            ..self
        }
    }

    /// A parser or lexer diagnostic, which has a position but no extent.
    #[must_use]
    pub fn from_parse(d: &crate::parse::Diagnostic, path: &str, source: &str) -> Self {
        let span = Span {
            start: d.location,
            end: d.location,
        };
        Diagnostic {
            code: d.code,
            summary: d.message.clone(),
            path: path.to_owned(),
            span: Some(span),
            line: report::line_of(source, span).to_owned(),
            help: d.help.clone(),
            fault: false,
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let at = self.span.map(|span| (span, self.line.as_str()));
        let fault = if self.fault { "fault " } else { "" };
        f.write_str(&report::block(
            &format!("{fault}{}: {}", self.code, self.summary),
            &self.path,
            at,
            self.help.as_deref(),
        ))
    }
}

/// Why a check produced nothing to build.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Exit 2: not a Lingua diagnostic.
    Usage(String),
    /// Exit 1, sorted by path, position, and code, and capped at 100 per file.
    Diagnostics(Vec<Diagnostic>),
}

/// A checked package: every module in `paths` and its import closure.
#[derive(Debug)]
pub struct Checked {
    pub symbols: Vec<crate::tir::Symbol>,
    /// Imports before importers.
    pub modules: Vec<TirModule>,
    /// The keys of the modules named on the command line, closure excluded.
    pub requested: Vec<String>,
    /// Resource operation calls in each module's residue, for `L072`.
    resource_calls: Vec<(String, Diagnostic)>,
    /// Functions whose examples do not run (see the module docs).
    skip: HashSet<SymbolId>,
    /// Each module's generated declarations as source text, imports first.
    pub expanded: Vec<(String, String)>,
}

/// `lingua check`: loads `paths` (default `.`) and their import closure and typechecks them.
///
/// # Errors
/// A usage error, or every diagnostic.
pub fn check(paths: &[PathBuf], options: &Options) -> Result<Checked, Error> {
    let loaded = package::load(paths, options)?;
    let mut diagnostics = loaded.diagnostics.clone();
    // Comptime roots and examples may recurse deeper than the main thread's stack allows
    let (checked, more) = crate::eval::on_big_stack(|| {
        let (checked, mut more) = decl::check_package(&loaded);
        let runnable = runnable(&loaded, &more);
        more.extend(crate::eval::run_examples(
            &checked.symbols,
            &checked.modules,
            &checked.skip,
            &|module| runnable.contains(&module.key),
        ));
        (checked, more)
    });
    diagnostics.extend(more);
    if diagnostics.is_empty() {
        Ok(checked)
    } else {
        Err(Error::Diagnostics(finish(diagnostics)))
    }
}

/// `lingua expand`: the same fixed point and cache policy as [`check`], without the examples.
/// The expanded declarations of the requested modules as source text, a blank line between,
/// and every diagnostic, sorted. The text holds what expanded even when there are
/// diagnostics.
///
/// # Errors
/// A usage error.
pub fn expand(paths: &[PathBuf], options: &Options) -> Result<(String, Vec<Diagnostic>), Error> {
    let loaded = package::load(paths, options)?;
    let mut diagnostics = loaded.diagnostics.clone();
    let (checked, more) = crate::eval::on_big_stack(|| decl::check_package(&loaded));
    diagnostics.extend(more);
    let text = checked
        .expanded
        .iter()
        .filter(|(key, _)| checked.requested.contains(key))
        .map(|(_, text)| text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    Ok((text, finish(diagnostics)))
}

// The modules whose examples run: those with no diagnostic but a "not yet" one, whose imports
// all run too, since a function with a diagnostic may be missing from the residue. Sources
// come imports first, so one pass sees every import before its importer. `diagnostics` must
// include the loader's.
fn runnable(loaded: &package::Loaded, diagnostics: &[Diagnostic]) -> HashSet<String> {
    let mut out = HashSet::new();
    for source in &loaded.sources {
        let Some(module) = &source.module else {
            continue;
        };
        if diagnostics
            .iter()
            .chain(&loaded.diagnostics)
            .all(|d| d.path != source.path || d.code == NOT_YET)
            && module.imports.iter().all(|i| out.contains(&i.path))
        {
            out.insert(source.key.clone());
        }
    }
    out
}

/// `lingua build`: checks `paths`, then applies the link-set rules: exactly one `main`
/// (`L074`) with the entry signature (`L075`) and effect (`L025`), and no resource operation
/// in the residue (`L072`). The link set is every module checked.
///
/// # Errors
/// As [`check`], plus the link-set diagnostics.
pub fn link(paths: &[PathBuf], options: &Options) -> Result<TirProgram, Error> {
    let checked = check(paths, options)?;
    let mut diagnostics: Vec<Diagnostic> = checked
        .resource_calls
        .iter()
        .map(|(_, d)| d.clone())
        .collect();
    let mains: Vec<(usize, usize)> = checked
        .modules
        .iter()
        .enumerate()
        .flat_map(|(m, module)| {
            module
                .functions
                .iter()
                .enumerate()
                .filter(|(_, f)| checked.symbols[f.id as usize].name == "main")
                .map(move |(i, _)| (m, i))
        })
        .collect();
    for &(m, i) in &mains {
        diagnostics.extend(decl::entry_rules(&checked, m, i));
    }
    for &(m, i) in mains.iter().skip(1) {
        let module = &checked.modules[m];
        diagnostics.push(
            Diagnostic::new("L074", "more than one `main` function", &module.path)
                .at(&module.source, module.functions[i].span)
                .help("keep one `main` and rename or delete the others"),
        );
    }
    if mains.is_empty() {
        let first = checked
            .requested
            .first()
            .and_then(|key| checked.modules.iter().find(|m| &m.key == key))
            .or(checked.modules.last());
        let start = crate::lex::Location { line: 1, column: 1 };
        let d = match first {
            Some(m) => Diagnostic::new("L074", "no `main` function to build", &m.path)
                .at(&m.source, Span { start, end: start }),
            None => Diagnostic::new("L074", "no `main` function to build: no modules", "."),
        };
        diagnostics.push(d.help("add `fn main() -> Nothing`"));
    }
    if !diagnostics.is_empty() {
        return Err(Error::Diagnostics(finish(diagnostics)));
    }
    let (m, i) = mains[0];
    let main = checked.modules[m].functions[i].id;
    Ok(TirProgram {
        symbols: checked.symbols,
        modules: checked.modules,
        main,
    })
}

// Sorts by file, position, and code, and caps each file at 100 (the 100th is `L047`)
fn finish(mut diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    diagnostics.sort_by(|a, b| {
        let key = |d: &Diagnostic| {
            (
                d.path.clone(),
                d.span.map_or((0, 0), |s| (s.start.line, s.start.column)),
                d.code,
            )
        };
        key(a).cmp(&key(b))
    });
    diagnostics.dedup();
    let mut out: Vec<Diagnostic> = Vec::new();
    let mut count = 0;
    for d in diagnostics {
        if out.last().is_some_and(|last| last.path != d.path) {
            count = 0;
        }
        count += 1;
        match count {
            ..100 => out.push(d),
            100 => out.push(Diagnostic::new(
                "L047",
                "diagnostic limit: 100 per file; fix these and check again",
                &d.path,
            )),
            _ => {}
        }
    }
    out
}

/// Whether `ty` is a nominal type whose symbol is a user or prelude sum.
fn is_sum(symbols: &[crate::tir::Symbol], ty: &Type) -> bool {
    matches!(ty, Type::Nominal(id) if matches!(symbols[*id as usize].kind, SymbolKind::Type(crate::tir::TypeKind::Sum)))
}
