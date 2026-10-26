//! Type providers (docs/lingua1.md, Type providers, Prelude): the `Type` values a generative
//! body or a `provide` builds, their expansion to real AST, the text of that AST, the
//! `{source_root}/provided/<module>.lf` cache, and `fs.read`.
//!
//! The checker owns the policy (`--offline`, `--refresh`); this module only builds, prints,
//! reads, and writes.

use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

use crate::ast::{Decl, Field, Span, TypeExpr};

/// A comptime `Type`: one record, its fields in order, each with the name of its type.
// ponytail: records only. The prelude has no pure way to build a sum (`schema` and
// `types_from_openapi` are deferred), so a module `Type` and `L073` wait for them too.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub fields: Vec<(String, String)>,
}

/// Whether `text` is one Lingua `ident`: a raw ident that is not a keyword.
#[must_use]
pub fn is_ident(text: &str) -> bool {
    crate::check::is_module_key(text) && crate::lex::Keyword::from_spelling(text).is_none()
}

/// `record_from_columns`: `(name, sql_type)` pairs to one record, or the `fail` message.
///
/// # Errors
/// `"no columns"`, `"bad column name"`, `"unknown sql type"`, or `"duplicate column"`.
pub fn record_from_columns(columns: &[(String, String)]) -> Result<Record, &'static str> {
    if columns.is_empty() {
        return Err("no columns");
    }
    let mut fields: Vec<(String, String)> = Vec::new();
    for (name, sql_type) in columns {
        if !is_ident(name) {
            return Err("bad column name");
        }
        let ty = match sql_type.as_str() {
            "smallint" | "integer" | "bigint" | "int2" | "int4" | "int8" => "Int",
            "boolean" | "bool" => "Bool",
            "text" | "varchar" | "character varying" | "bpchar" => "Text",
            _ => return Err("unknown sql type"),
        };
        if fields.iter().any(|(n, _)| n == name) {
            return Err("duplicate column");
        }
        fields.push((name.clone(), ty.to_owned()));
    }
    Ok(Record { fields })
}

/// The ordinary `type` declaration `name` expands to. Every node carries `span`, the
/// generative declaration's name, so a diagnostic about the expansion points there.
#[must_use]
pub fn decl(name: &str, record: &Record, span: Span) -> Decl {
    Decl::Product {
        span,
        name: name.to_owned(),
        fields: record
            .fields
            .iter()
            .map(|(field, ty)| Field {
                span,
                name: field.clone(),
                ty: TypeExpr::Named(span, ty.clone()),
            })
            .collect(),
    }
}

/// The source text of an expanded declaration, in the formatter's layout, with a trailing
/// newline. Only what an expansion builds: a product whose field types are names.
#[must_use]
pub fn print(decl: &Decl) -> String {
    let Decl::Product { name, fields, .. } = decl else {
        return String::new();
    };
    let fields: String = fields
        .iter()
        .map(|f| match &f.ty {
            TypeExpr::Named(_, ty) => format!("  {}: {ty}\n", f.name),
            _ => String::new(),
        })
        .collect();
    format!("type {name}\n{fields}end type {name}\n")
}

/// The text of a cache file: declarations in source order, a blank line between.
#[must_use]
pub fn print_all(decls: &[&Decl]) -> String {
    decls
        .iter()
        .map(|d| print(d))
        .collect::<Vec<_>>()
        .join("\n")
}

/// `{root}/provided/{key}.lf`.
#[must_use]
pub fn cache_path(root: &Path, key: &str) -> PathBuf {
    root.join("provided").join(format!("{key}.lf"))
}

/// The records in a cache file, by name. A missing file, or one that does not parse, is
/// empty; a declaration that is not a record of named field types is absent.
// ponytail: an entry the user edited into something else is a miss, not a diagnostic.
#[must_use]
pub fn read_cache(root: &Path, key: &str) -> HashMap<String, Record> {
    let Ok(bytes) = std::fs::read(cache_path(root, key)) else {
        return HashMap::new();
    };
    let Ok(module) = crate::parse::parse_source(&bytes, key) else {
        return HashMap::new();
    };
    module
        .decls
        .iter()
        .filter_map(|d| match d {
            Decl::Product { name, fields, .. } => {
                let fields = fields
                    .iter()
                    .map(|f| match &f.ty {
                        TypeExpr::Named(_, ty) => Some((f.name.clone(), ty.clone())),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                Some((name.clone(), Record { fields }))
            }
            _ => None,
        })
        .collect()
}

/// Writes `text` to the cache file of `key` when it differs from what is there.
///
/// # Errors
/// The I/O error.
pub fn write_cache(root: &Path, key: &str, text: &str) -> std::io::Result<()> {
    let path = cache_path(root, key);
    if std::fs::read_to_string(&path).is_ok_and(|old| old == text) {
        return Ok(());
    }
    std::fs::create_dir_all(root.join("provided"))?;
    std::fs::write(path, text)
}

/// Why `fs.read` has no text.
#[derive(Debug, PartialEq)]
pub enum FsFail {
    NotFound,
    EscapesRoot,
    BadEncoding,
    /// Over 1 MiB: `L047`, not a `Result`.
    TooBig,
}

const FS_LIMIT: u64 = 1 << 20;

/// `fs.read(path)`: `path` resolved against `root` must stay under it after `.` and `..`,
/// and after following symlinks.
///
/// # Errors
/// See [`FsFail`].
pub fn fs_read(root: &Path, path: &str) -> Result<String, FsFail> {
    let mut depth = 0usize;
    for part in Path::new(path).components() {
        match part {
            Component::Normal(_) => depth += 1,
            Component::CurDir => {}
            Component::ParentDir => depth = depth.checked_sub(1).ok_or(FsFail::EscapesRoot)?,
            Component::RootDir | Component::Prefix(_) => return Err(FsFail::EscapesRoot),
        }
    }
    let full = root.join(path);
    let Ok(real) = std::fs::canonicalize(&full) else {
        return Err(FsFail::NotFound);
    };
    let real_root = std::fs::canonicalize(root).map_err(|_| FsFail::NotFound)?;
    if !real.starts_with(&real_root) {
        return Err(FsFail::EscapesRoot);
    }
    let meta = std::fs::metadata(&real).map_err(|_| FsFail::NotFound)?;
    if !meta.is_file() {
        return Err(FsFail::NotFound);
    }
    if meta.len() > FS_LIMIT {
        return Err(FsFail::TooBig);
    }
    let bytes = std::fs::read(&real).map_err(|_| FsFail::NotFound)?;
    String::from_utf8(bytes).map_err(|_| FsFail::BadEncoding)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOWHERE: Span = Span {
        start: crate::lex::Location { line: 1, column: 1 },
        end: crate::lex::Location { line: 1, column: 1 },
    };

    fn cols(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(a, b)| ((*a).to_owned(), (*b).to_owned()))
            .collect()
    }

    #[test]
    fn the_orders_column_fixture_expands_exactly() {
        let record = record_from_columns(&cols(&[
            ("id", "integer"),
            ("name", "text"),
            ("active", "boolean"),
        ]))
        .unwrap();
        assert_eq!(
            print(&decl("OrderRow", &record, NOWHERE)),
            "type OrderRow\n  id: Int\n  name: Text\n  active: Bool\nend type OrderRow\n"
        );
    }

    #[test]
    fn record_from_columns_normalizes_and_rejects() {
        let types =
            |t: &str| record_from_columns(&cols(&[("x", t)])).map(|r| r.fields[0].1.clone());
        for (sql, ty) in [
            ("smallint", "Int"),
            ("int8", "Int"),
            ("bool", "Bool"),
            ("character varying", "Text"),
            ("bpchar", "Text"),
        ] {
            assert_eq!(types(sql).as_deref(), Ok(ty), "{sql}");
        }
        assert_eq!(types("int"), Err("unknown sql type"));
        assert_eq!(record_from_columns(&[]), Err("no columns"));
        for bad in ["end", "1x", "_", "a-b", ""] {
            assert_eq!(
                record_from_columns(&cols(&[(bad, "text")])),
                Err("bad column name"),
                "{bad}"
            );
        }
        assert_eq!(
            record_from_columns(&cols(&[("a", "text"), ("a", "text")])),
            Err("duplicate column")
        );
    }

    #[test]
    fn fs_read_stays_under_the_root() {
        let root = std::env::temp_dir().join(format!("lingua-fs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("sub/a.txt"), "hi").unwrap();
        std::fs::write(root.join("bad.txt"), [0xff, 0xfe]).unwrap();
        assert_eq!(fs_read(&root, "sub/a.txt").as_deref(), Ok("hi"));
        assert_eq!(fs_read(&root, "./sub/../sub/a.txt").as_deref(), Ok("hi"));
        assert_eq!(fs_read(&root, "nope.txt"), Err(FsFail::NotFound));
        assert_eq!(fs_read(&root, "sub"), Err(FsFail::NotFound));
        assert_eq!(fs_read(&root, "../x"), Err(FsFail::EscapesRoot));
        assert_eq!(fs_read(&root, "sub/../../x"), Err(FsFail::EscapesRoot));
        assert_eq!(fs_read(&root, "/etc/hosts"), Err(FsFail::EscapesRoot));
        assert_eq!(fs_read(&root, "bad.txt"), Err(FsFail::BadEncoding));
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/etc", root.join("out")).unwrap();
            assert_eq!(fs_read(&root, "out/hosts"), Err(FsFail::EscapesRoot));
        }
        std::fs::write(root.join("big.txt"), vec![b'a'; (1 << 20) + 1]).unwrap();
        assert_eq!(fs_read(&root, "big.txt"), Err(FsFail::TooBig));
    }

    #[test]
    fn the_cache_round_trips_and_is_not_rewritten_when_unchanged() {
        let root = std::env::temp_dir().join(format!("lingua-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let a = Record {
            fields: vec![("x".into(), "Int".into())],
        };
        let b = Record {
            fields: vec![("t".into(), "Text".into()), ("ok".into(), "Bool".into())],
        };
        let text = print_all(&[&decl("A", &a, NOWHERE), &decl("B", &b, NOWHERE)]);
        assert_eq!(
            text,
            "type A\n  x: Int\nend type A\n\ntype B\n  t: Text\n  ok: Bool\nend type B\n"
        );
        write_cache(&root, "m", &text).unwrap();
        let read = read_cache(&root, "m");
        assert_eq!((read.get("A"), read.get("B")), (Some(&a), Some(&b)));
        assert!(read_cache(&root, "other").is_empty());
    }
}
