//! The exception and fault report shapes (docs/lingua1.md, Value errors and exceptions).
//! One renderer for every engine: `lingua build` pre-renders each site's report with it, so a
//! native binary prints exactly what the interpreter would.

use crate::ast::Span;

/// The summary line of each fault code the runtime can stop with.
#[must_use]
pub fn fault_summary(code: &str) -> &'static str {
    match code {
        "L040" => "arithmetic overflow",
        "L046" => "split on an empty separator",
        "L047" => "out of memory",
        "L076" => "division by zero",
        "L077" => "stack overflow",
        _ => "internal error",
    }
}

/// `fault CODE: summary` and the span block, the same shape as an exception:
///
/// ```text
/// fault L040: arithmetic overflow
///   --> money.lf:40:12
///    |
/// 40 |   return part * of
///    |          ^^^^^^^^^
/// ```
///
/// With no span (`L047` from the allocator) it is the header line alone.
#[must_use]
pub fn fault(code: &str, path: &str, source: &str, span: Option<Span>) -> String {
    let header = format!("fault {code}: {}", fault_summary(code));
    match span {
        Some(span) => block(&header, path, Some((span, line_of(source, span))), None),
        None => format!("{header}\n"),
    }
}

/// `exception: message`, the position, and the source line with a caret under the span's
/// first line:
///
/// ```text
/// exception: factor must be non-zero
///   --> order.lf:12:3
///    |
/// 12 |   assert factor != 0, "factor must be non-zero"
///    |   ^^^^^^
/// ```
#[must_use]
pub fn exception(message: &str, path: &str, source: &str, span: Span) -> String {
    block(
        &format!("exception: {message}"),
        path,
        Some((span, line_of(source, span))),
        None,
    )
}

/// The source line a span starts on, or `""` when the source has no such line.
#[must_use]
pub fn line_of(source: &str, span: Span) -> &str {
    source
        .lines()
        .nth(span.start.line.wrapping_sub(1))
        .unwrap_or("")
}

/// The span block every report shares: a header, `--> path:line:col`, the source line with
/// carets under the span's first line, and an optional `= help:` line. The gutter is at
/// least two columns wide. Without a span the `-->` line names the path alone.
#[must_use]
pub fn block(header: &str, path: &str, at: Option<(Span, &str)>, help: Option<&str>) -> String {
    let Some((span, line)) = at else {
        let help = help.map_or_else(String::new, |help| format!("   = help: {help}\n"));
        return format!("{header}\n  --> {path}\n{help}");
    };
    let number = span.start.line.to_string();
    let gutter = " ".repeat(number.len().max(2));
    let indent = span.start.column.saturating_sub(1);
    let end = if span.end.line == span.start.line {
        span.end.column.saturating_sub(1)
    } else {
        line.chars().count()
    };
    let carets = "^".repeat(end.saturating_sub(indent).max(1));
    let out = format!(
        "{header}\n  --> {path}:{}:{}\n{gutter} |\n{number:>width$} | {line}\n{gutter} | {}{carets}\n",
        span.start.line,
        span.start.column,
        " ".repeat(indent),
        width = gutter.len(),
    );
    match help {
        Some(help) => format!("{out}{gutter} |\n{gutter} = help: {help}\n"),
        None => out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::Location;

    fn span(line: usize, column: usize, end_line: usize, end_column: usize) -> Span {
        Span {
            start: Location { line, column },
            end: Location {
                line: end_line,
                column: end_column,
            },
        }
    }

    #[test]
    fn exception_matches_the_lingua1_shape() {
        let source = format!(
            "{}  assert factor != 0, \"factor must be non-zero\"\n",
            "\n".repeat(11)
        );
        assert_eq!(
            exception(
                "factor must be non-zero",
                "order.lf",
                &source,
                span(12, 3, 12, 9)
            ),
            "exception: factor must be non-zero
  --> order.lf:12:3
   |
12 |   assert factor != 0, \"factor must be non-zero\"
   |   ^^^^^^
"
        );
    }

    #[test]
    fn a_multi_line_span_carets_to_the_end_of_its_first_line() {
        assert_eq!(
            exception("m", "a.lf", "fail \"m\"\nnext\n", span(1, 1, 2, 3)),
            "exception: m\n  --> a.lf:1:1\n   |\n 1 | fail \"m\"\n   | ^^^^^^^^\n"
        );
    }

    #[test]
    fn fault_prints_the_span_block_like_an_exception() {
        let source = format!("{}  return part * of\n", "\n".repeat(39));
        assert_eq!(
            fault("L040", "money.lf", &source, Some(span(40, 10, 40, 19))),
            "fault L040: arithmetic overflow
  --> money.lf:40:10
   |
40 |   return part * of
   |          ^^^^^^^^^
"
        );
        assert_eq!(
            fault("L076", "a.lf", "1 / 0\n", Some(span(1, 1, 1, 6))),
            "fault L076: division by zero\n  --> a.lf:1:1\n   |\n 1 | 1 / 0\n   | ^^^^^\n"
        );
        assert_eq!(fault("L047", "", "", None), "fault L047: out of memory\n");
    }

    #[test]
    fn a_help_line_follows_a_blank_gutter_line() {
        assert_eq!(
            block(
                "L031: match on Discount is missing a variant",
                "order.lf",
                Some((span(40, 3, 40, 8), "  match discount")),
                Some("legal alternatives: CentsOff"),
            ),
            "L031: match on Discount is missing a variant
  --> order.lf:40:3
   |
40 |   match discount
   |   ^^^^^
   |
   = help: legal alternatives: CentsOff
"
        );
        assert_eq!(
            block(
                "L016: path is outside the source root",
                "x/a.lf",
                None,
                Some("the source root is /r")
            ),
            "L016: path is outside the source root\n  --> x/a.lf\n   = help: the source root is /r\n"
        );
    }
}
