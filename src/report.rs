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

/// `fault CODE: summary` and the span's position:
///
/// ```text
/// fault L040: arithmetic overflow
///   --> money.lf:40:12
/// ```
#[must_use]
pub fn fault(code: &str, path: &str, span: Option<Span>) -> String {
    let mut out = format!("fault {code}: {}\n", fault_summary(code));
    if let Some(span) = span {
        out.push_str(&location(path, span));
    }
    out
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
    let line = source
        .lines()
        .nth(span.start.line.wrapping_sub(1))
        .unwrap_or("");
    let number = span.start.line.to_string();
    let gutter = " ".repeat(number.len());
    let indent = span.start.column.saturating_sub(1);
    let end = if span.end.line == span.start.line {
        span.end.column.saturating_sub(1)
    } else {
        line.chars().count()
    };
    let carets = "^".repeat(end.saturating_sub(indent).max(1));
    format!(
        "exception: {message}\n{}{gutter} |\n{number} | {line}\n{gutter} | {}{carets}\n",
        location(path, span),
        " ".repeat(indent)
    )
}

fn location(path: &str, span: Span) -> String {
    format!("  --> {path}:{}:{}\n", span.start.line, span.start.column)
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
            "exception: m\n  --> a.lf:1:1\n  |\n1 | fail \"m\"\n  | ^^^^^^^^\n"
        );
    }

    #[test]
    fn fault_matches_the_lingua1_shape() {
        assert_eq!(
            fault("L040", "money.lf", Some(span(40, 12, 40, 13))),
            "fault L040: arithmetic overflow\n  --> money.lf:40:12\n"
        );
        assert_eq!(
            fault("L076", "a.lf", Some(span(1, 1, 1, 2))),
            "fault L076: division by zero\n  --> a.lf:1:1\n"
        );
        assert_eq!(fault("L047", "", None), "fault L047: out of memory\n");
    }
}
