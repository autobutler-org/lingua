//! unlex, the inverse of lex (docs/lingua1.md, Compilation pipeline, Source generation):
//! each token's lexeme at its `line:col`, line breaks from `Newline` tokens. It makes no
//! layout choices, so `lex(unlex(t)) == t` for any token list whose places do not overlap.

use super::token::{StringPart, Token, TokenKind};

/// Text as it is spelled between the quotes of a string literal.
#[must_use]
pub fn escape(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '{' => out.push_str("{{"),
            '}' => out.push_str("}}"),
            _ => out.push(c),
        }
    }
    out
}

/// The source spelling of one token. A hole is spelled with its raw source.
#[must_use]
pub fn lexeme(kind: &TokenKind) -> String {
    let fixed = match kind {
        TokenKind::Eof => "",
        TokenKind::Newline => "\n",
        TokenKind::Wildcard => "_",
        TokenKind::LeftParen => "(",
        TokenKind::RightParen => ")",
        TokenKind::LeftBracket => "[",
        TokenKind::RightBracket => "]",
        TokenKind::Comma => ",",
        TokenKind::Dot => ".",
        TokenKind::Colon => ":",
        TokenKind::Plus => "+",
        TokenKind::Minus => "-",
        TokenKind::Asterisk => "*",
        TokenKind::Slash => "/",
        TokenKind::Percent => "%",
        TokenKind::Equals => "=",
        TokenKind::Arrow => "->",
        TokenKind::PlusEquals => "+=",
        TokenKind::EqualsEquals => "==",
        TokenKind::BangEquals => "!=",
        TokenKind::LessThan => "<",
        TokenKind::GreaterThan => ">",
        TokenKind::LessEquals => "<=",
        TokenKind::GreaterEquals => ">=",
        TokenKind::Keyword(keyword) => keyword.spelling(),
        TokenKind::Identifier(name) => return name.clone(),
        TokenKind::Integer(value) => return value.to_string(),
        TokenKind::String(parts) => {
            let mut out = String::from("\"");
            for part in parts {
                match part {
                    StringPart::Text(text) => out.push_str(&escape(text)),
                    StringPart::Hole { source, .. } => {
                        out.push('{');
                        out.push_str(source);
                        out.push('}');
                    }
                }
            }
            out.push('"');
            return out;
        }
    };
    fixed.to_string()
}

/// Source with each token's lexeme placed at its column, and a line break for each
/// `Newline`. Nothing pads up to a newline, so no line has trailing whitespace.
#[must_use]
pub fn unlex(tokens: &[Token]) -> String {
    let mut out = String::new();
    let mut column = 1;
    for token in tokens.iter().filter(|token| token.kind != TokenKind::Eof) {
        if token.kind == TokenKind::Newline {
            out.push('\n');
            column = 1;
            continue;
        }
        let text = lexeme(&token.kind);
        let pad = token.location.column.saturating_sub(column);
        out.extend(std::iter::repeat_n(' ', pad));
        out.push_str(&text);
        column += pad + text.chars().count();
    }
    out
}
