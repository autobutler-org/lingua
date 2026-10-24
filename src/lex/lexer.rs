use super::location::Location;
use super::token::Keyword;
use super::token::StringPart;
use super::token::Token;
use super::token::TokenKind;

#[derive(Debug, PartialEq)]
pub struct LexError {
    pub code: &'static str,
    pub message: String,
    pub location: Location,
}

// Diagnostic form from the spec, minus the path, which the caller knows
impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n  --> {}:{}",
            self.code, self.message, self.location.line, self.location.column
        )
    }
}

const MAX_SOURCE_BYTES: usize = 1024 * 1024;

/// Lexes raw file bytes into tokens, ending with `Eof`.
///
/// # Errors
///
/// `L047` for a source over 1 MiB, `L004` for invalid UTF-8, or the first token error.
pub fn lex_source(bytes: &[u8]) -> Result<Vec<Token>, LexError> {
    let start = Location { line: 1, column: 1 };
    if bytes.len() > MAX_SOURCE_BYTES {
        return Err(LexError {
            code: "L047",
            message: "source file is larger than 1 MiB".to_string(),
            location: start,
        });
    }
    let contents = std::str::from_utf8(bytes).map_err(|err| LexError {
        code: "L004",
        message: format!("invalid UTF-8 at byte {}", err.valid_up_to()),
        location: start,
    })?;
    let mut lexer = Lexer::new(contents);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        let is_eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if is_eof {
            return Ok(tokens);
        }
    }
}

pub struct Lexer {
    contents: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    #[must_use]
    pub fn new(contents: &str) -> Self {
        Lexer {
            contents: contents.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Input is a `&str`, so it is already valid UTF-8; `lex_source` handles raw bytes.
    ///
    /// # Errors
    ///
    /// The `L004` or `L005` diagnostic for the first bad token.
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        if self.position == 0 && self.peek(0) == Some('\u{FEFF}') {
            return Err(self.error("L004", "byte order mark".to_string()));
        }
        self.skip_blanks();
        let location = self.location();
        let Some(current_char) = self.peek(0) else {
            return Ok(Token {
                kind: TokenKind::Eof,
                location,
            });
        };

        let kind = match current_char {
            '\n' | '\r' => {
                self.advance();
                if current_char == '\r' && self.peek(0) == Some('\n') {
                    self.advance();
                }
                self.line += 1;
                self.column = 1;
                TokenKind::Newline
            }
            '"' => TokenKind::String(self.scan_string()?),
            c if c.is_ascii_alphabetic() || c == '_' => self.scan_word(),
            c if c.is_ascii_digit() => self.scan_integer()?,
            _ => self.scan_punctuation(current_char)?,
        };
        Ok(Token { kind, location })
    }

    #[must_use]
    pub fn location(&self) -> Location {
        Location {
            line: self.line,
            column: self.column,
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.contents.get(self.position + offset).copied()
    }

    // Callers handle newlines themselves; everything else stays on the current line
    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
    }

    fn error(&self, code: &'static str, message: String) -> LexError {
        LexError {
            code,
            message,
            location: self.location(),
        }
    }

    fn skip_blanks(&mut self) {
        while let Some(current_char) = self.peek(0) {
            match current_char {
                ' ' | '\t' => self.advance(),
                // A comment stops before the newline so the newline is still a token
                '/' if self.peek(1) == Some('/') => {
                    while self.peek(0).is_some_and(|c| c != '\n' && c != '\r') {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn scan_word(&mut self) -> TokenKind {
        let mut token_buffer = String::new();
        while let Some(current_char) = self.peek(0) {
            if !(current_char.is_ascii_alphanumeric() || current_char == '_') {
                break;
            }
            token_buffer.push(current_char);
            self.advance();
        }
        if token_buffer == "_" {
            return TokenKind::Wildcard;
        }
        match Keyword::from_spelling(&token_buffer) {
            Some(keyword) => TokenKind::Keyword(keyword),
            None => TokenKind::Identifier(token_buffer),
        }
    }

    fn scan_integer(&mut self) -> Result<TokenKind, LexError> {
        let start = self.location();
        let mut token_buffer = String::new();
        while let Some(current_char) = self.peek(0).filter(char::is_ascii_digit) {
            token_buffer.push(current_char);
            self.advance();
        }
        // Over 20 digits is rejected before parsing; parse also rejects anything above INT_MAX
        let message = if token_buffer.len() > 20 {
            format!("integer {token_buffer} is longer than 20 characters")
        } else if let Ok(value) = token_buffer.parse::<i64>() {
            return Ok(TokenKind::Integer(value));
        } else {
            format!("integer {token_buffer} is outside 0 through INT_MAX")
        };
        Err(LexError {
            code: "L005",
            message,
            location: start,
        })
    }

    fn scan_punctuation(&mut self, current_char: char) -> Result<TokenKind, LexError> {
        let next_char = self.peek(1);
        // Longest match, except '>' never joins into '>>'
        let (kind, length) = match (current_char, next_char) {
            ('-', Some('>')) => (TokenKind::Arrow, 2),
            ('+', Some('=')) => (TokenKind::PlusEquals, 2),
            ('=', Some('=')) => (TokenKind::EqualsEquals, 2),
            ('!', Some('=')) => (TokenKind::BangEquals, 2),
            ('<', Some('=')) => (TokenKind::LessEquals, 2),
            ('>', Some('=')) => (TokenKind::GreaterEquals, 2),
            ('(', _) => (TokenKind::LeftParen, 1),
            (')', _) => (TokenKind::RightParen, 1),
            ('[', _) => (TokenKind::LeftBracket, 1),
            (']', _) => (TokenKind::RightBracket, 1),
            (',', _) => (TokenKind::Comma, 1),
            ('.', _) => (TokenKind::Dot, 1),
            (':', _) => (TokenKind::Colon, 1),
            ('+', _) => (TokenKind::Plus, 1),
            ('-', _) => (TokenKind::Minus, 1),
            ('*', _) => (TokenKind::Asterisk, 1),
            ('=', _) => (TokenKind::Equals, 1),
            ('<', _) => (TokenKind::LessThan, 1),
            ('>', _) => (TokenKind::GreaterThan, 1),
            _ => {
                return Err(self.error("L004", format!("unexpected character '{current_char}'")));
            }
        };
        for _ in 0..length {
            self.advance();
        }
        Ok(kind)
    }

    fn scan_string(&mut self) -> Result<Vec<StringPart>, LexError> {
        let start = self.location();
        let unterminated = || LexError {
            code: "L004",
            message: "unterminated string".to_string(),
            location: start,
        };
        self.advance();
        let mut parts = Vec::new();
        let mut token_buffer = String::new();
        loop {
            let Some(current_char) = self.peek(0) else {
                return Err(unterminated());
            };
            match (current_char, self.peek(1)) {
                ('"', _) => {
                    self.advance();
                    break;
                }
                ('\n' | '\r', _) => {
                    return Err(self.error("L004", "newline in string".to_string()));
                }
                ('\\', escaped) => {
                    let unescaped = match escaped {
                        Some('"') => '"',
                        Some('\\') => '\\',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        _ => return Err(self.error("L004", "bad escape".to_string())),
                    };
                    token_buffer.push(unescaped);
                    self.advance();
                    self.advance();
                }
                ('{', Some('{')) | ('}', Some('}')) => {
                    token_buffer.push(current_char);
                    self.advance();
                    self.advance();
                }
                ('}', _) => return Err(self.error("L004", "lone '}' in string".to_string())),
                ('{', _) => {
                    if !token_buffer.is_empty() {
                        parts.push(StringPart::Text(std::mem::take(&mut token_buffer)));
                    }
                    self.advance();
                    let hole_start = self.position;
                    loop {
                        match self.peek(0) {
                            None => return Err(unterminated()),
                            Some('\n' | '\r') => {
                                return Err(self.error("L004", "newline in string".to_string()));
                            }
                            Some('"') => {
                                self.scan_string()?;
                            }
                            Some('}') => break,
                            Some(_) => self.advance(),
                        }
                    }
                    let hole = self.contents[hole_start..self.position].iter().collect();
                    parts.push(StringPart::Hole(hole));
                    self.advance();
                }
                _ => {
                    token_buffer.push(current_char);
                    self.advance();
                }
            }
        }
        if !token_buffer.is_empty() {
            parts.push(StringPart::Text(token_buffer));
        }
        Ok(parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_all(contents: &str) -> Result<Vec<Token>, LexError> {
        let mut lexer = Lexer::new(contents);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token()?;
            if token.kind == TokenKind::Eof {
                return Ok(tokens);
            }
            tokens.push(token);
        }
    }

    fn kinds(contents: &str) -> Vec<TokenKind> {
        lex_all(contents)
            .unwrap()
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    fn error(contents: &str) -> (&'static str, usize, usize) {
        let err = lex_all(contents).unwrap_err();
        (err.code, err.location.line, err.location.column)
    }

    fn identifier(name: &str) -> TokenKind {
        TokenKind::Identifier(name.to_string())
    }

    fn text(value: &str) -> StringPart {
        StringPart::Text(value.to_string())
    }

    fn hole(value: &str) -> StringPart {
        StringPart::Hole(value.to_string())
    }

    #[test]
    fn lexes_punctuation() {
        assert_eq!(
            kinds("_ ( ) [ ] , . : + - * = -> += == != < > <= >="),
            vec![
                TokenKind::Wildcard,
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::LeftBracket,
                TokenKind::RightBracket,
                TokenKind::Comma,
                TokenKind::Dot,
                TokenKind::Colon,
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Asterisk,
                TokenKind::Equals,
                TokenKind::Arrow,
                TokenKind::PlusEquals,
                TokenKind::EqualsEquals,
                TokenKind::BangEquals,
                TokenKind::LessThan,
                TokenKind::GreaterThan,
                TokenKind::LessEquals,
                TokenKind::GreaterEquals,
            ]
        );
    }

    #[test]
    fn never_joins_greater_thans() {
        assert_eq!(
            kinds("List<List<Int>>="),
            vec![
                identifier("List"),
                TokenKind::LessThan,
                identifier("List"),
                TokenKind::LessThan,
                identifier("Int"),
                TokenKind::GreaterThan,
                TokenKind::GreaterEquals,
            ]
        );
    }

    #[test]
    fn lexes_keywords() {
        assert_eq!(
            kinds("fn end Ok Err int_in int_at_least primitive log"),
            vec![
                TokenKind::Keyword(Keyword::Fn),
                TokenKind::Keyword(Keyword::End),
                TokenKind::Keyword(Keyword::Ok),
                TokenKind::Keyword(Keyword::Err),
                TokenKind::Keyword(Keyword::IntIn),
                TokenKind::Keyword(Keyword::IntAtLeast),
                identifier("primitive"),
                identifier("log"),
            ]
        );
    }

    #[test]
    fn lexes_identifiers_and_wildcard() {
        assert_eq!(
            kinds("_ _foo __ a_1 Go(_)"),
            vec![
                TokenKind::Wildcard,
                identifier("_foo"),
                identifier("__"),
                identifier("a_1"),
                identifier("Go"),
                TokenKind::LeftParen,
                TokenKind::Wildcard,
                TokenKind::RightParen,
            ]
        );
    }

    #[test]
    fn rejects_non_ascii_identifier() {
        assert_eq!(error("héllo"), ("L004", 1, 2));
    }

    #[test]
    fn rejects_unknown_characters() {
        assert_eq!(error("a ; b"), ("L004", 1, 3));
        assert_eq!(error("!"), ("L004", 1, 1));
        assert_eq!(error("/"), ("L004", 1, 1));
    }

    #[test]
    fn lexes_integers() {
        assert_eq!(
            kinds("0 -5 9223372036854775807 007"),
            vec![
                TokenKind::Integer(0),
                TokenKind::Minus,
                TokenKind::Integer(5),
                TokenKind::Integer(i64::MAX),
                TokenKind::Integer(7),
            ]
        );
    }

    #[test]
    fn rejects_out_of_range_integers() {
        assert_eq!(error("x 9223372036854775808"), ("L005", 1, 3));
        assert_eq!(error(&"0".repeat(21)), ("L005", 1, 1));
        assert_eq!(error("99999999999999999999999999999"), ("L005", 1, 1));
    }

    #[test]
    fn lexes_newlines() {
        let tokens = lex_all("a\nb\r\nc\rd").unwrap();
        let summary: Vec<_> = tokens
            .iter()
            .map(|token| (&token.kind, token.location.line, token.location.column))
            .collect();
        assert_eq!(
            summary,
            vec![
                (&identifier("a"), 1, 1),
                (&TokenKind::Newline, 1, 2),
                (&identifier("b"), 2, 1),
                (&TokenKind::Newline, 2, 2),
                (&identifier("c"), 3, 1),
                (&TokenKind::Newline, 3, 2),
                (&identifier("d"), 4, 1),
            ]
        );
    }

    #[test]
    fn comments_keep_the_newline() {
        assert_eq!(
            kinds("a // hi there\n\tb // trailing"),
            vec![identifier("a"), TokenKind::Newline, identifier("b")]
        );
    }

    #[test]
    fn records_token_start_locations() {
        let tokens = lex_all("fn foo\n  -> \"x\"").unwrap();
        let locations: Vec<_> = tokens
            .iter()
            .map(|token| (token.location.line, token.location.column))
            .collect();
        assert_eq!(locations, vec![(1, 1), (1, 4), (1, 7), (2, 3), (2, 6)]);
    }

    #[test]
    fn eof_repeats() {
        let mut lexer = Lexer::new("a");
        lexer.next_token().unwrap();
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn lexes_strings_with_escapes() {
        assert_eq!(
            kinds(r#""" "a\"\\\n\t{{}}é""#),
            vec![
                TokenKind::String(vec![]),
                TokenKind::String(vec![text("a\"\\\n\t{}é")]),
            ]
        );
    }

    #[test]
    fn lexes_interpolation_holes() {
        assert_eq!(
            kinds(r#""loaded {n} orders" "{a.b}""#),
            vec![
                TokenKind::String(vec![text("loaded "), hole("n"), text(" orders")]),
                TokenKind::String(vec![hole("a.b")]),
            ]
        );
    }

    #[test]
    fn hole_may_contain_nested_strings() {
        assert_eq!(
            kinds(r#""x{f("}}\"", "{y}")}z""#),
            vec![TokenKind::String(vec![
                text("x"),
                hole(r#"f("}}\"", "{y}")"#),
                text("z"),
            ])]
        );
    }

    #[test]
    fn rejects_bad_strings() {
        assert_eq!(error(r#""a\q""#), ("L004", 1, 3));
        assert_eq!(error(r#""a}b""#), ("L004", 1, 3));
        assert_eq!(error("\"a\nb\""), ("L004", 1, 3));
        assert_eq!(error("\"{a\n}\""), ("L004", 1, 4));
        assert_eq!(error("\"abc"), ("L004", 1, 1));
        assert_eq!(error("\"{abc"), ("L004", 1, 1));
    }

    #[test]
    fn rejects_byte_order_mark() {
        assert_eq!(error("\u{FEFF}fn"), ("L004", 1, 1));
    }

    #[test]
    fn displays_error_in_diagnostic_form() {
        let err = lex_all("a\n  ;").unwrap_err();
        assert_eq!(err.to_string(), "L004: unexpected character ';'\n  --> 2:3");
    }

    #[test]
    fn lex_source_ends_with_eof() {
        let tokens = lex_source(b"a").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }

    #[test]
    fn lex_source_rejects_invalid_utf8() {
        assert_eq!(lex_source(b"ab\xff").unwrap_err().code, "L004");
    }

    #[test]
    fn lex_source_rejects_files_over_one_mebibyte() {
        let source = vec![b' '; 1024 * 1024 + 1];
        assert_eq!(lex_source(&source).unwrap_err().code, "L047");
        assert!(lex_source(&source[1..]).is_ok());
    }
}
