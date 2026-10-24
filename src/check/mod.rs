//! The binder and typechecker (docs/lingua1.md, Module and file model, Name resolution, Types,
//! Effects): a package of parsed modules in, the typed IR in `crate::tir` out.
//!
//! Out of scope until the interpreter exists, and reported as `error: ... not supported yet`
//! rather than guessed at: comptime evaluation (`const`, `comptime`, a bound that is not
//! integer literal arithmetic), generative types and providers, `quote` / `unquote`, and
//! package examples. Function examples are typechecked against the signature; they do not run.

mod body;
mod catalog;
mod decl;
mod package;
mod pattern;
mod types;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

use crate::ast::Span;
use crate::report;
use crate::tir::{SymbolKind, TirModule, TirProgram, Type};

pub use catalog::explain;
pub use package::Options;

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
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let at = self.span.map(|span| (span, self.line.as_str()));
        f.write_str(&report::block(
            &format!("{}: {}", self.code, self.summary),
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
}

/// `lingua check`: loads `paths` (default `.`) and their import closure and typechecks them.
///
/// # Errors
/// A usage error, or every diagnostic.
pub fn check(paths: &[PathBuf], options: &Options) -> Result<Checked, Error> {
    let loaded = package::load(paths, options)?;
    let mut diagnostics = loaded.diagnostics.clone();
    let (checked, more) = decl::check_package(&loaded);
    diagnostics.extend(more);
    if diagnostics.is_empty() {
        Ok(checked)
    } else {
        Err(Error::Diagnostics(finish(diagnostics)))
    }
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
