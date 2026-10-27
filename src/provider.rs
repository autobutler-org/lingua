//! Type providers (docs/lingua1.md, Type providers, Prelude): the `Type` values a generative
//! body or a `provide` builds, their expansion to real AST, the text of that AST, the
//! `{source_root}/provided/<module>.lf` cache.
//!
//! The checker owns the policy (`--offline`, `--refresh`); this module only builds, prints,
//! reads, and writes.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ast::{Decl, Field, Span, TypeExpr, Variant};
use crate::lex::{Location, TokenKind};

/// A comptime `Type`: one record or one sum. Each field type and payload is type text in the
/// formatter's layout (docs/lingua1.md, Type text); names in it resolve at expansion.
// ponytail: no module `Type`, so `L073` waits for `schema` and `types_from_openapi`.
#[derive(Debug, Clone, PartialEq)]
pub enum Generated {
    /// Each field's name and type text, in order.
    Record(Vec<(String, String)>),
    /// Each variant's name and payload type text, in order.
    Sum(Vec<(String, Option<String>)>),
}

/// What a builder returns: the `Type`, or its `fail` message.
pub type Built = Result<Generated, &'static str>;

/// Whether `text` is one Lingua `ident`: a raw ident that is not a keyword.
#[must_use]
pub fn is_ident(text: &str) -> bool {
    crate::check::is_module_key(text) && crate::lex::Keyword::from_spelling(text).is_none()
}

const NOWHERE: Span = Span {
    start: Location { line: 1, column: 1 },
    end: Location { line: 1, column: 1 },
};

/// `text` in the formatter's layout when it is exactly one `type-expr`.
// ponytail: a newline inside a bound's call is dropped, so a broken call prints on one line;
// the formatter's broken layout is the upgrade once `lingua fmt` exists.
#[must_use]
pub fn type_text(text: &str) -> Option<String> {
    crate::parse::parse_type_text(text, NOWHERE)?;
    let tokens = crate::lex::lex_source(text.as_bytes()).ok()?;
    let lines = char_lines(text);
    let spelling = |t: &crate::lex::Token| {
        spanned(
            &lines,
            Span {
                start: t.location,
                end: t.end,
            },
        )
    };
    let operand = |k: Option<&TokenKind>| {
        matches!(
            k,
            Some(
                TokenKind::Identifier(_)
                    | TokenKind::Integer(_)
                    | TokenKind::String(_)
                    | TokenKind::RightParen
                    | TokenKind::RightBracket
            )
        )
    };
    let mut out = String::new();
    // The previous two tokens, for a unary `-`
    let (mut prev, mut before): (Option<&TokenKind>, Option<&TokenKind>) = (None, None);
    for t in tokens
        .iter()
        .filter(|t| !matches!(t.kind, TokenKind::Newline | TokenKind::Eof))
    {
        let k = &t.kind;
        let tight = match prev {
            None
            | Some(
                TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LessThan
                | TokenKind::Dot,
            ) => true,
            Some(TokenKind::Minus) => !operand(before),
            Some(_) => {
                matches!(
                    k,
                    TokenKind::RightParen
                        | TokenKind::RightBracket
                        | TokenKind::GreaterThan
                        | TokenKind::LessThan
                        | TokenKind::Comma
                        | TokenKind::Colon
                        | TokenKind::Dot
                ) || (*k == TokenKind::LeftParen && operand(prev))
            }
        };
        if !tight {
            out.push(' ');
        }
        out.push_str(&spelling(t));
        (before, prev) = (prev, Some(k));
    }
    Some(out)
}

/// `record_from_columns`: `(name, sql_type)` pairs to one record, or the `fail` message.
///
/// # Errors
/// `"no columns"`, `"bad column name"`, `"unknown sql type"`, or `"duplicate column"`.
pub fn record_from_columns(columns: &[(String, String)]) -> Built {
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
    Ok(Generated::Record(fields))
}

/// `record_from_fields`: `(name, type_text)` pairs to one record, or the `fail` message.
///
/// # Errors
/// `"no fields"`, `"bad field name"`, `"duplicate field"`, or `"bad field type"`.
pub fn record_from_fields(fields: &[(String, String)]) -> Built {
    if fields.is_empty() {
        return Err("no fields");
    }
    let mut out: Vec<(String, String)> = Vec::new();
    for (name, text) in fields {
        if !is_ident(name) {
            return Err("bad field name");
        }
        if out.iter().any(|(n, _)| n == name) {
            return Err("duplicate field");
        }
        out.push((name.clone(), type_text(text).ok_or("bad field type")?));
    }
    Ok(Generated::Record(out))
}

/// `sum_from_variants`: `(name, payload)` pairs to one sum, or the `fail` message. An empty
/// payload is a zero-payload variant.
///
/// # Errors
/// `"no variants"`, `"bad variant name"`, `"duplicate variant"`, or `"bad payload type"`.
pub fn sum_from_variants(variants: &[(String, String)]) -> Built {
    if variants.is_empty() {
        return Err("no variants");
    }
    let mut out: Vec<(String, Option<String>)> = Vec::new();
    for (name, payload) in variants {
        if !is_ident(name) {
            return Err("bad variant name");
        }
        if out.iter().any(|(n, _)| n == name) {
            return Err("duplicate variant");
        }
        let payload = if payload.is_empty() {
            None
        } else {
            Some(type_text(payload).ok_or("bad payload type")?)
        };
        out.push((name.clone(), payload));
    }
    Ok(Generated::Sum(out))
}

/// The ordinary `type` declaration `name` expands to. Every node carries `span`, the
/// generative declaration's name, so a diagnostic about the expansion points there.
#[must_use]
pub fn decl(name: &str, generated: &Generated, span: Span) -> Decl {
    // The text was checked when it was built or read
    let ty = |text: &str| {
        crate::parse::parse_type_text(text, span)
            .unwrap_or_else(|| TypeExpr::Named(span, text.to_owned()))
    };
    let name = name.to_owned();
    match generated {
        Generated::Record(fields) => Decl::Product {
            span,
            name,
            fields: fields
                .iter()
                .map(|(field, text)| Field {
                    span,
                    name: field.clone(),
                    ty: ty(text),
                })
                .collect(),
        },
        Generated::Sum(variants) => Decl::Sum {
            span,
            name,
            variants: variants
                .iter()
                .map(|(variant, payload)| Variant {
                    span,
                    name: variant.clone(),
                    payload: payload.as_deref().map(ty),
                })
                .collect(),
        },
    }
}

/// The source text of an expanded declaration, in the formatter's layout, with a trailing
/// newline.
#[must_use]
pub fn print(name: &str, generated: &Generated) -> String {
    let lines: Vec<String> = match generated {
        Generated::Record(fields) => fields
            .iter()
            .map(|(field, ty)| format!("  {field}: {ty}\n"))
            .collect(),
        Generated::Sum(variants) => variants
            .iter()
            .map(|(variant, payload)| match payload {
                Some(ty) => format!("  {variant}({ty})\n"),
                None => format!("  {variant}\n"),
            })
            .collect(),
    };
    format!("type {name}\n{}end type {name}\n", lines.concat())
}

/// The text of a cache file: declarations in source order, a blank line between.
#[must_use]
pub fn print_all(decls: &[(&str, &Generated)]) -> String {
    decls
        .iter()
        .map(|(name, generated)| print(name, generated))
        .collect::<Vec<_>>()
        .join("\n")
}

/// `{root}/provided/{key}.lf`.
#[must_use]
pub fn cache_path(root: &Path, key: &str) -> PathBuf {
    root.join("provided").join(format!("{key}.lf"))
}

// `text`'s lines as the lexer counts them: `\n`, `\r\n`, or `\r` ends one
fn char_lines(text: &str) -> Vec<Vec<char>> {
    text.replace("\r\n", "\n")
        .split(['\n', '\r'])
        .map(|l| l.chars().collect())
        .collect()
}

// The source text a span covers on its first line
fn spanned(lines: &[Vec<char>], span: Span) -> String {
    let Some(line) = lines.get(span.start.line.wrapping_sub(1)) else {
        return String::new();
    };
    let end = if span.end.line == span.start.line {
        span.end.column.saturating_sub(1)
    } else {
        line.len()
    };
    line.get(span.start.column.saturating_sub(1)..end)
        .map_or_else(String::new, |chars| chars.iter().collect())
}

/// The records and sums in a cache file, by name, rebuilt from their text as the builders
/// would. A missing file, or one that does not parse, is empty.
// ponytail: an entry the user edited into something else (an alias, a function, a field
// type spread over lines) is a miss, not a diagnostic.
#[must_use]
pub fn read_cache(root: &Path, key: &str) -> HashMap<String, Generated> {
    let Ok(bytes) = std::fs::read(cache_path(root, key)) else {
        return HashMap::new();
    };
    let Ok(module) = crate::parse::parse_source(&bytes, key) else {
        return HashMap::new();
    };
    let lines = char_lines(&String::from_utf8_lossy(&bytes));
    let ty = |t: &TypeExpr| spanned(&lines, t.span());
    module
        .decls
        .iter()
        .filter_map(|d| match d {
            Decl::Product { name, fields, .. } => {
                let pairs: Vec<(String, String)> =
                    fields.iter().map(|f| (f.name.clone(), ty(&f.ty))).collect();
                Some((name.clone(), record_from_fields(&pairs).ok()?))
            }
            Decl::Sum { name, variants, .. } => {
                let pairs: Vec<(String, String)> = variants
                    .iter()
                    .map(|v| {
                        (
                            v.name.clone(),
                            v.payload.as_ref().map(ty).unwrap_or_default(),
                        )
                    })
                    .collect();
                Some((name.clone(), sum_from_variants(&pairs).ok()?))
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

#[cfg(test)]
mod tests {
    use super::*;

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
            print("OrderRow", &record),
            "type OrderRow\n  id: Int\n  name: Text\n  active: Bool\nend type OrderRow\n"
        );
    }

    #[test]
    fn record_from_columns_normalizes_and_rejects() {
        let types = |t: &str| match record_from_columns(&cols(&[("x", t)])) {
            Ok(Generated::Record(fields)) => Ok(fields[0].1.clone()),
            Ok(Generated::Sum(_)) => Err("a sum"),
            Err(e) => Err(e),
        };
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

    fn texts(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        cols(pairs)
    }

    #[test]
    fn the_promo_and_offer_fixtures_expand_exactly() {
        let promo = sum_from_variants(&texts(&[
            ("NoPromo", ""),
            ("PercentOff", "Int in 1 through 100"),
            ("AmountOff", "Cents"),
        ]))
        .unwrap();
        let offer = record_from_fields(&texts(&[
            ("id", "Int"),
            ("price", "Cents"),
            ("quantity", "Int in 1 through 100"),
            ("promo", "Promo"),
        ]))
        .unwrap();
        assert_eq!(
            print_all(&[("Promo", &promo), ("Offer", &offer)]),
            "type Promo\n  NoPromo\n  PercentOff(Int in 1 through 100)\n  AmountOff(Cents)\nend type Promo\n\ntype Offer\n  id: Int\n  price: Cents\n  quantity: Int in 1 through 100\n  promo: Promo\nend type Offer\n"
        );
        let Decl::Sum { variants, .. } = decl("Promo", &promo, NOWHERE) else {
            panic!("expected a sum")
        };
        assert!(matches!(
            &variants[1].payload,
            Some(TypeExpr::Range(span, ..)) if *span == NOWHERE
        ));
    }

    #[test]
    fn type_text_prints_in_the_formatter_layout() {
        for (text, printed) in [
            ("Int  in 1   through 100", "Int in 1 through 100"),
            ("Result< Cents ,ParseError >", "Result<Cents, ParseError>"),
            ("List<List<Int>>", "List<List<Int>>"),
            ("Int at least - 1", "Int at least -1"),
            (
                "Int in 0 - 1 through max( a:1 ,b:2 )*2",
                "Int in 0 - 1 through max(a: 1, b: 2) * 2",
            ),
            (
                "Int at least max(\r  a: 1,\r\n  b: 2,\n)",
                "Int at least max(a: 1, b: 2,)",
            ),
        ] {
            let record = record_from_fields(&texts(&[("x", text)])).unwrap();
            assert_eq!(
                record,
                Generated::Record(texts(&[("x", printed)])),
                "{text}"
            );
        }
    }

    #[test]
    fn record_from_fields_rejects() {
        let fields = |pairs: &[(&str, &str)]| record_from_fields(&texts(pairs));
        assert_eq!(fields(&[]), Err("no fields"));
        for bad in ["end", "1x", "_", "a-b", ""] {
            assert_eq!(fields(&[(bad, "Int")]), Err("bad field name"), "{bad}");
        }
        assert_eq!(
            fields(&[("a", "Int"), ("a", "Text")]),
            Err("duplicate field")
        );
        for bad in [
            "",
            "Int Int",
            "1",
            "Int in 1",
            "List<Int",
            "Int\nText",
            "\"Int\"",
            "Int,",
        ] {
            assert_eq!(fields(&[("a", bad)]), Err("bad field type"), "{bad:?}");
        }
    }

    #[test]
    fn sum_from_variants_rejects() {
        let variants = |pairs: &[(&str, &str)]| sum_from_variants(&texts(pairs));
        assert_eq!(variants(&[]), Err("no variants"));
        for bad in ["Ok", "end", "1x", "_", "A-B", ""] {
            assert_eq!(variants(&[(bad, "")]), Err("bad variant name"), "{bad}");
        }
        assert_eq!(
            variants(&[("A", ""), ("A", "Int")]),
            Err("duplicate variant")
        );
        for bad in ["Int Int", "1", "in", "List<Int"] {
            assert_eq!(variants(&[("A", bad)]), Err("bad payload type"), "{bad:?}");
        }
        assert_eq!(
            variants(&[("A", ""), ("B", "List<Int>")]),
            Ok(Generated::Sum(vec![
                ("A".into(), None),
                ("B".into(), Some("List<Int>".into()))
            ]))
        );
    }

    #[test]
    fn the_cache_round_trips_and_is_not_rewritten_when_unchanged() {
        let root = std::env::temp_dir().join(format!("lingua-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let a = Generated::Record(texts(&[("x", "Int")]));
        let b = Generated::Record(texts(&[("t", "Text"), ("ok", "Bool")]));
        let promo = Generated::Sum(vec![
            ("NoPromo".into(), None),
            ("PercentOff".into(), Some("Int in 1 through 100".into())),
            ("Again".into(), Some("List<Promo>".into())),
        ]);
        let offer = Generated::Record(texts(&[
            ("price", "Cents"),
            ("bound", "Int at least -1"),
            ("r", "Result<Cents, Text>"),
        ]));
        let text = print_all(&[("A", &a), ("B", &b), ("Promo", &promo), ("Offer", &offer)]);
        assert!(
            text.starts_with(
                "type A\n  x: Int\nend type A\n\ntype B\n  t: Text\n  ok: Bool\nend type B\n\ntype Promo\n"
            ),
            "{text}"
        );
        write_cache(&root, "m", &text).unwrap();
        let read = read_cache(&root, "m");
        assert_eq!(
            ["A", "B", "Promo", "Offer"].map(|n| read.get(n)),
            [Some(&a), Some(&b), Some(&promo), Some(&offer)]
        );
        assert!(read_cache(&root, "other").is_empty());
    }
}
