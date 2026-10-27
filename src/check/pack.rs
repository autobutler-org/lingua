//! `lingua pack` (docs/lingua1.md, Context packer): a comment-shaped header with what each
//! imported name means, then the file's original bytes. It runs the fixed point, so imported
//! generative types are expanded under the same cache policy as `check`, but it does not run
//! examples and does not need the file itself to typecheck.

use std::path::{Path, PathBuf};

use super::body::effect_text;
use super::{Checked, Error, Options, decl, finish, package};
use crate::ast::{Decl, Module, Span, Stmt, TypeExpr};
use crate::provider::{self, Generated};
use crate::tir::{SymbolKind, TirExpr, TirKind};

/// `lingua pack FILE`: the header and the file's bytes, and whether the header is free of
/// diagnostics (an import that does not resolve).
///
/// # Errors
/// A usage error: no file or several, a directory, or a path that does not exist.
pub fn pack(paths: &[PathBuf], options: &Options) -> Result<(Vec<u8>, bool), Error> {
    let file = match paths {
        [file] => file,
        [] => {
            return Err(Error::Usage(
                "pack needs one `.lf` file, e.g. `lingua pack order.lf`".into(),
            ));
        }
        _ => {
            return Err(Error::Usage(
                "pack takes one file; run `lingua pack` once per file".into(),
            ));
        }
    };
    if file.is_dir() {
        return Err(Error::Usage(
            "pack takes one `.lf` file, not a directory; pass a file such as `lingua pack a.lf`"
                .into(),
        ));
    }
    let loaded = package::load(std::slice::from_ref(file), options)?;
    let bytes = std::fs::read(file).map_err(|err| {
        Error::Usage(format!(
            "cannot read `{}`: {err}; check the path and try again",
            file.display()
        ))
    })?;
    let (checked, more) = crate::eval::on_big_stack(|| decl::check_package(&loaded));

    let name = |p: &Path| p.file_name().map(|s| s.to_string_lossy().into_owned());
    let key = file
        .file_stem()
        .map_or_else(String::new, |s| s.to_string_lossy().into_owned());
    let mut out = format!(
        "# lingua pack {}\n# module {key}\n# language: lingua1\n\n",
        name(file).unwrap_or_default(),
    );

    let source = loaded
        .requested
        .first()
        .and_then(|k| loaded.index(k))
        .map(|i| &loaded.sources[i]);
    let own = source.map_or_else(|| file.display().to_string(), |s| s.path.clone());
    let imports = source
        .and_then(|s| s.module.as_ref())
        .map_or(&[][..], |m| &m.imports[..]);
    let targets: Vec<usize> = imports
        .iter()
        .filter_map(|i| loaded.index(&i.path))
        .collect();
    let is_target = |path: &str| targets.iter().any(|&t| loaded.sources[t].path == path);

    // What stops an import resolving: the loader's report on this file (`L016`, `L015`, a
    // parse error) or on a module it imports, an unknown export (`L010`), and an imported
    // module's `--offline` miss (`L071`)
    let diagnostics = finish(
        loaded
            .diagnostics
            .iter()
            .filter(|d| d.path == own || is_target(&d.path))
            .chain(more.iter().filter(|d| {
                (d.code == "L010" && d.path == own) || (d.code == "L071" && is_target(&d.path))
            }))
            .cloned()
            .collect(),
    );
    for d in &diagnostics {
        for line in d.to_string().lines() {
            out.extend(["# ", line, "\n"]);
        }
        out.push('\n');
    }

    for import in imports {
        let Some(t) = loaded.index(&import.path) else {
            continue;
        };
        let Some(module) = &loaded.sources[t].module else {
            continue;
        };
        let mut entries: Vec<((usize, usize), String)> = import
            .names
            .iter()
            .filter_map(|n| {
                let (at, text) = entry(&checked, &loaded.sources[t].text, t, module, n)?;
                let path = &loaded.sources[t].path;
                let head = format!(
                    "# import {n} from \"{}\"\n# defined in {path}\n",
                    import.path
                );
                Some(((at.start.line, at.start.column), format!("{head}{text}\n")))
            })
            .collect();
        // Definition order in the defining module
        entries.sort_by_key(|(at, _)| *at);
        for (_, text) in entries {
            out.push_str(&text);
        }
    }
    out.push_str("# --- source (unchanged) ---\n");
    Ok(([out.into_bytes(), bytes].concat(), diagnostics.is_empty()))
}

// The interface text of `name` from module `t`, and where it is defined
fn entry(
    checked: &Checked,
    source: &str,
    t: usize,
    module: &Module,
    name: &str,
) -> Option<(Span, String)> {
    let ty = |ty: &TypeExpr| {
        let text = spanned(source, ty.span());
        provider::type_text(&text).unwrap_or(text)
    };
    // A generated type that is `name` or has the constructor `name`
    let expanded = |n: &str, span: Span| {
        let (.., generated) = checked
            .generated
            .iter()
            .find(|(m, _, g, _)| *m == t && g == n)?;
        let defines = n == name
            || matches!(generated, Generated::Sum(vs) if vs.iter().any(|(v, _)| v == name));
        defines.then(|| {
            (
                span,
                format!("# expanded\n{}", provider::print(n, generated)),
            )
        })
    };
    module.decls.iter().find_map(|decl| match decl {
        Decl::Function {
            span,
            name: n,
            params,
            ret,
            effect,
            ..
        } if n == name => {
            let params: Vec<String> = params
                .iter()
                .map(|p| format!("{}: {}", p.name, ty(&p.ty)))
                .collect();
            let sig = format!(
                "# {n}({}) -> {}\n#   {}\n",
                params.join(", "),
                ty(ret),
                effect_text(effect)
            );
            Some((*span, sig))
        }
        Decl::Alias {
            span,
            name: n,
            ty: aliased,
        } if n == name => Some((*span, format!("type {n} = {}\n", ty(aliased)))),
        Decl::Product {
            span,
            name: n,
            fields,
        } if n == name => {
            let fields = fields.iter().map(|f| (f.name.clone(), ty(&f.ty))).collect();
            Some((*span, provider::print(n, &Generated::Record(fields))))
        }
        Decl::Sum {
            span,
            name: n,
            variants,
        } if n == name || variants.iter().any(|v| v.name == name) => {
            let variants = variants
                .iter()
                .map(|v| (v.name.clone(), v.payload.as_ref().map(ty)))
                .collect();
            Some((*span, provider::print(n, &Generated::Sum(variants))))
        }
        Decl::Generative { span, name: n, .. } => expanded(n, *span),
        Decl::ComptimeBlock { body, .. } => body.iter().find_map(|stmt| match stmt {
            Stmt::Provide { span, name: n, .. } => expanded(n, *span),
            _ => None,
        }),
        Decl::Const {
            span,
            name: n,
            ty: declared,
            effect,
            ..
        } if n == name => {
            let declared = declared
                .as_ref()
                .map_or_else(String::new, |t| format!(": {}", ty(t)));
            let effect = match effect {
                crate::ast::Effect::Pure => " ".to_owned(),
                e => format!("\n  {}\n  ", effect_text(e)),
            };
            // A const that did not fold prints without a value rather than an invented one
            let value = checked
                .symbols
                .iter()
                .position(|s| s.name == *n && s.module == module.key && s.kind == SymbolKind::Const)
                .and_then(|id| checked.consts.get(&u32::try_from(id).ok()?))
                .and_then(|e| literal(checked, e));
            let text = match value {
                Some(v) => format!("const {n}{declared}{effect}= {v}\n"),
                None => format!("const {n}{declared}\n"),
            };
            Some((*span, text))
        }
        _ => None,
    })
}

// A folded value as a Lingua literal; record fields are named
fn literal(checked: &Checked, e: &TirExpr) -> Option<String> {
    let symbol = |id: u32| &checked.symbols[id as usize];
    let all = |items: &[TirExpr]| -> Option<Vec<String>> {
        items.iter().map(|i| literal(checked, i)).collect()
    };
    Some(match &e.kind {
        TirKind::Int(n) => n.to_string(),
        TirKind::Text(t) => crate::eval::quote(t),
        TirKind::Bool(b) => b.to_string(),
        TirKind::Nothing => "nothing".to_owned(),
        TirKind::List(items) => format!("[{}]", all(items)?.join(", ")),
        TirKind::Record(id, fields) => {
            let names = checked.symbols.iter().filter_map(|s| match s.kind {
                SymbolKind::Field { parent, index } if parent == *id => Some((index, &s.name)),
                _ => None,
            });
            let mut names: Vec<(u32, &String)> = names.collect();
            names.sort();
            let fields: Vec<String> = names
                .iter()
                .zip(all(fields)?)
                .map(|((_, n), v)| format!("{n}: {v}"))
                .collect();
            format!("{}({})", symbol(*id).name, fields.join(", "))
        }
        TirKind::Variant(id, None) => symbol(*id).name.clone(),
        TirKind::Variant(id, Some(p)) => format!("{}({})", symbol(*id).name, literal(checked, p)?),
        _ => return None,
    })
}

// The source text `span` covers, across lines
fn spanned(source: &str, span: Span) -> String {
    let (first, last) = (span.start.line, span.end.line);
    source
        .lines()
        .enumerate()
        .skip(first.saturating_sub(1))
        .take((last + 1).saturating_sub(first))
        .map(|(i, line)| {
            let chars: Vec<char> = line.chars().collect();
            let from = if i + 1 == first {
                span.start.column.saturating_sub(1)
            } else {
                0
            };
            let to = if i + 1 == last {
                span.end.column.saturating_sub(1)
            } else {
                chars.len()
            };
            chars[from.min(chars.len())..to.clamp(from.min(chars.len()), chars.len())]
                .iter()
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
