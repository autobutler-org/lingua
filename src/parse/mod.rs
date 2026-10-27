//! Recursive-descent parser for the lingua1 grammar: one `parse_*` per production,
//! and a `*_first` check per production that picks which alternative to try.

use crate::ast::{
    Arg, Arm, ArmBody, ArmKind, AssignOp, BinOp, Bind, Decl, Effect, Example, Expect, Expr, Field,
    FnBody, Import, Module, Param, Pattern, QuoteBody, ResourceEffect, ResourceOp, Span, Stmt,
    TextPart, TypeExpr, Variant,
};
use crate::lex::{
    Keyword, LexError, Lexer, Location, StringPart, Token, TokenKind, lex_source, lexeme,
};

mod unparse;

pub use unparse::{unparse, unparse_type};

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
    pub location: Location,
    pub help: Option<String>,
}

// Same form as `LexError`: the caller knows the path
impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n  --> {}:{}",
            self.code, self.message, self.location.line, self.location.column
        )?;
        if let Some(help) = &self.help {
            write!(f, "\n   = help: {help}")?;
        }
        Ok(())
    }
}

impl From<LexError> for Diagnostic {
    fn from(err: LexError) -> Self {
        Diagnostic {
            code: err.code,
            message: err.message,
            location: err.location,
            help: None,
        }
    }
}

/// Lexes and parses one module. `key` is the module key, the file stem.
///
/// # Errors
///
/// The lex error, or every syntax diagnostic (`L001`, `L002`, `L003`, ...) found
/// while recovering to the next `end fn` / `end type` / `end resource`.
pub fn parse_source(bytes: &[u8], key: &str) -> Result<Module, Vec<Diagnostic>> {
    parse_tokens(lex_source(bytes).map_err(|err| vec![err.into()])?, key)
}

/// Parses a token list that ends with `Eof`, from lex or from `unparse`.
///
/// # Errors
///
/// Every syntax diagnostic, as `parse_source`.
pub fn parse_tokens(tokens: Vec<Token>, key: &str) -> Result<Module, Vec<Diagnostic>> {
    let mut parser = Parser::new(tokens, false);
    let module = parser.parse_module(key);
    if parser.diagnostics.is_empty() {
        Ok(module)
    } else {
        Err(parser.diagnostics)
    }
}

/// Lexes and parses `text` as exactly one `type-expr` (docs/lingua1.md, Type text), with
/// every node at `span`; `None` when it is anything else.
#[must_use]
pub fn parse_type_text(text: &str, span: Span) -> Option<TypeExpr> {
    let tokens: Vec<Token> = lex_source(text.as_bytes())
        .ok()?
        .into_iter()
        .map(|t| Token {
            location: span.start,
            end: span.end,
            ..t
        })
        .collect();
    let mut parser = Parser::new(tokens, false);
    let ty = parser.parse_type_expr().ok()?;
    (parser.at(&TokenKind::Eof) && parser.diagnostics.is_empty()).then_some(ty)
}

type Parsed<T> = Result<T, Diagnostic>;

const MAX_NESTING: usize = 64;

// What a closer must spell: bare `end`, or `end <kind> <name>`
enum Closer<'a> {
    Bare(&'static str),
    Named(Keyword, &'a str),
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
    // Top is whether newlines are skipped: true inside `()` and `[]`, false inside `quote ( )` and `< >`
    skip_newlines: Vec<bool>,
    depth: usize,
    in_hole: bool,
    previous_end: Location,
    diagnostics: Vec<Diagnostic>,
}

fn syntax_error(message: String, location: Location) -> Diagnostic {
    Diagnostic {
        code: "L001",
        message,
        location,
        help: None,
    }
}

fn describe(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Eof => "end of file".to_string(),
        TokenKind::Newline => "newline".to_string(),
        TokenKind::String(_) => "a string".to_string(),
        _ => format!("`{}`", lexeme(kind)),
    }
}

fn is_keyword(kind: &TokenKind, keyword: Keyword) -> bool {
    *kind == TokenKind::Keyword(keyword)
}

// First sets

fn effect_first(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Keyword(Keyword::Pure | Keyword::Reads | Keyword::Writes)
    )
}

fn cmp_first(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::String(_)
            | TokenKind::Integer(_)
            | TokenKind::Identifier(_)
            | TokenKind::LeftBracket
            | TokenKind::LeftParen
            | TokenKind::Minus
            | TokenKind::Keyword(
                Keyword::True
                    | Keyword::False
                    | Keyword::Nothing
                    | Keyword::Ok
                    | Keyword::Err
                    | Keyword::Clock
                    | Keyword::Fs
                    | Keyword::Catalog
                    | Keyword::Quote
                    | Keyword::Unquote
                    | Keyword::IntIn
                    | Keyword::IntAtLeast
                    | Keyword::Comptime
            )
    )
}

fn expr_first(kind: &TokenKind) -> bool {
    is_keyword(kind, Keyword::Match) || cmp_first(kind)
}

// Statement keywords; `quote` tries a statement first when it sees one of these
fn stmt_keyword_first(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Keyword(
            Keyword::Let
                | Keyword::Var
                | Keyword::Const
                | Keyword::Set
                | Keyword::Return
                | Keyword::Assert
                | Keyword::Fail
                | Keyword::Provide
                | Keyword::If
                | Keyword::For
                | Keyword::While
                | Keyword::Match
        )
    )
}

fn stmt_first(kind: &TokenKind) -> bool {
    stmt_keyword_first(kind) || expr_first(kind)
}

fn pattern_first(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Wildcard
            | TokenKind::Integer(_)
            | TokenKind::String(_)
            | TokenKind::Identifier(_)
            | TokenKind::LeftBracket
            | TokenKind::Minus
            | TokenKind::Keyword(Keyword::Ok | Keyword::Err)
    )
}

fn cmp_op(kind: &TokenKind) -> Option<BinOp> {
    Some(match kind {
        TokenKind::EqualsEquals => BinOp::Eq,
        TokenKind::BangEquals => BinOp::Ne,
        TokenKind::LessThan => BinOp::Lt,
        TokenKind::GreaterThan => BinOp::Gt,
        TokenKind::LessEquals => BinOp::Le,
        TokenKind::GreaterEquals => BinOp::Ge,
        _ => return None,
    })
}

fn closer_kind(kind: &TokenKind) -> Option<Keyword> {
    match kind {
        TokenKind::Keyword(keyword @ (Keyword::Fn | Keyword::Type | Keyword::Resource)) => {
            Some(*keyword)
        }
        _ => None,
    }
}

impl Parser {
    fn new(tokens: Vec<Token>, in_hole: bool) -> Self {
        let previous_end = tokens[0].location;
        Parser {
            tokens,
            position: 0,
            skip_newlines: Vec::new(),
            depth: 0,
            in_hole,
            previous_end,
            diagnostics: Vec::new(),
        }
    }

    // Token plumbing

    // Index of the `offset`-th significant token; newlines are skipped inside `()` and `[]`
    fn index_of(&self, offset: usize) -> usize {
        let skip = self.skip_newlines.last() == Some(&true);
        let mut index = self.position;
        let mut remaining = offset;
        loop {
            let kind = &self.tokens[index].kind;
            if skip && *kind == TokenKind::Newline {
                index += 1;
            } else if remaining == 0 || *kind == TokenKind::Eof {
                return index;
            } else {
                remaining -= 1;
                index += 1;
            }
        }
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens[self.index_of(0)].kind
    }

    fn peek_at(&self, offset: usize) -> &TokenKind {
        &self.tokens[self.index_of(offset)].kind
    }

    fn location(&self) -> Location {
        self.tokens[self.index_of(0)].location
    }

    fn at(&self, kind: &TokenKind) -> bool {
        self.peek() == kind
    }

    fn at_keyword(&self, keyword: Keyword) -> bool {
        is_keyword(self.peek(), keyword)
    }

    fn advance(&mut self) -> TokenKind {
        let index = self.index_of(0);
        let token = &self.tokens[index];
        if token.kind != TokenKind::Eof {
            self.position = index + 1;
        }
        if token.kind != TokenKind::Newline {
            self.previous_end = token.end;
        }
        token.kind.clone()
    }

    fn span_from(&self, start: Location) -> Span {
        Span {
            start,
            end: self.previous_end,
        }
    }

    fn error_here(&self, expected: &str) -> Diagnostic {
        syntax_error(
            format!("expected {expected}, found {}", describe(self.peek())),
            self.location(),
        )
    }

    fn expect(&mut self, kind: &TokenKind) -> Parsed<Location> {
        if self.at(kind) {
            let location = self.location();
            self.advance();
            Ok(location)
        } else {
            Err(self.error_here(&describe(kind)))
        }
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Parsed<Location> {
        self.expect(&TokenKind::Keyword(keyword))
    }

    fn expect_ident(&mut self, what: &str) -> Parsed<String> {
        if let TokenKind::Identifier(name) = self.peek() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(self.error_here(what))
        }
    }

    fn expect_ident_named(&mut self, name: &str) -> Parsed<()> {
        if *self.peek() == TokenKind::Identifier(name.to_string()) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_here(&format!("`{name}`")))
        }
    }

    // End of file also ends a line, so a missing final newline is not an error
    fn expect_newline(&mut self) -> Parsed<()> {
        match self.peek() {
            TokenKind::Newline => {
                self.advance();
                Ok(())
            }
            TokenKind::Eof => Ok(()),
            _ => Err(self.error_here("newline")),
        }
    }

    fn skip_blank_lines(&mut self) {
        while self.at(&TokenKind::Newline) {
            self.advance();
        }
    }

    // Opens `(`, `[`, or `<` and returns the raw index after it, for `close`
    fn open(&mut self, kind: &TokenKind, skip_newlines: bool) -> Parsed<usize> {
        let location = self.expect(kind)?;
        self.nest(location)?;
        self.skip_newlines.push(skip_newlines);
        Ok(self.position)
    }

    // Returns whether a newline appeared between the delimiters
    fn close(&mut self, kind: &TokenKind, opened_at: usize) -> Parsed<bool> {
        self.expect(kind)?;
        self.skip_newlines.pop();
        self.depth -= 1;
        Ok(self.tokens[opened_at..self.position]
            .iter()
            .any(|token| token.kind == TokenKind::Newline))
    }

    fn nest(&mut self, location: Location) -> Parsed<()> {
        self.depth += 1;
        if self.depth > MAX_NESTING {
            return Err(Diagnostic {
                code: "L047",
                message: format!("nesting is deeper than {MAX_NESTING}"),
                location,
                help: None,
            });
        }
        Ok(())
    }

    // `item { "," item } [ "," ]` between delimiters, possibly empty
    fn parse_delimited<T>(
        &mut self,
        open: &TokenKind,
        close: &TokenKind,
        skip_newlines: bool,
        mut item: impl FnMut(&mut Self) -> Parsed<T>,
    ) -> Parsed<(Vec<T>, bool)> {
        let opened_at = self.open(open, skip_newlines)?;
        let mut items = Vec::new();
        while !self.at(close) {
            items.push(item(self)?);
            if !self.at(&TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        let broken = self.close(close, opened_at)?;
        Ok((items, broken))
    }

    // A string that must not have holes: labels, messages, import paths
    fn parse_literal_string(&mut self, code: &'static str, what: &str) -> Parsed<String> {
        let location = self.location();
        let TokenKind::String(parts) = self.peek().clone() else {
            return Err(self.error_here(what));
        };
        self.advance();
        let mut text = String::new();
        for part in parts {
            match part {
                StringPart::Text(literal) => text.push_str(&literal),
                StringPart::Hole { .. } => {
                    return Err(Diagnostic {
                        code,
                        message: format!("{what} must be a string literal without holes"),
                        location,
                        help: None,
                    });
                }
            }
        }
        Ok(text)
    }

    // Recovery after a syntax error: skip past the next `end fn` / `end type` / `end resource` line
    fn recover(&mut self) {
        self.skip_newlines.clear();
        self.depth = 0;
        loop {
            match self.peek() {
                TokenKind::Eof => return,
                TokenKind::Keyword(Keyword::End) if closer_kind(self.peek_at(1)).is_some() => {
                    while !matches!(self.peek(), TokenKind::Newline | TokenKind::Eof) {
                        self.advance();
                    }
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    // closer = "end" [ ( "fn" | "type" | "resource" ) ident ]
    // A wrong name (L002) or kind (L003) on a named closer is reported and parsing goes on
    fn parse_closer(&mut self, closer: &Closer) -> Parsed<()> {
        let end_location = self.location();
        if !self.at_keyword(Keyword::End) {
            return Err(self.error_here("`end`"));
        }
        match *closer {
            Closer::Bare(construct) => {
                if let Some(kind) = closer_kind(self.peek_at(1)) {
                    return Err(Diagnostic {
                        code: "L003",
                        message: format!("`end {}` cannot close {construct}", kind.spelling()),
                        location: end_location,
                        help: Some("legal alternatives: end".to_string()),
                    });
                }
                self.advance();
            }
            Closer::Named(expected_kind, expected_name) => {
                self.advance();
                let opener = format!("{} {expected_name}", expected_kind.spelling());
                let kind_location = self.location();
                let Some(kind) = closer_kind(self.peek()) else {
                    self.diagnostics.push(Diagnostic {
                        code: "L003",
                        message: format!("bare `end` cannot close `{opener}`"),
                        location: end_location,
                        help: Some(format!("legal alternatives: {opener}")),
                    });
                    return Ok(());
                };
                self.advance();
                let name_location = self.location();
                let name = self.expect_ident("the closer name")?;
                if kind != expected_kind {
                    self.diagnostics.push(Diagnostic {
                        code: "L003",
                        message: "closer kind does not match opener".to_string(),
                        location: kind_location,
                        help: Some(format!("legal alternatives: {opener}")),
                    });
                } else if name != expected_name {
                    self.diagnostics.push(Diagnostic {
                        code: "L002",
                        message: "closer name does not match opener".to_string(),
                        location: name_location,
                        help: Some(format!("legal alternatives: {expected_name}")),
                    });
                }
            }
        }
        Ok(())
    }

    // Module

    // module = { newline | import-decl | type-decl | fn-decl | resource-decl
    //          | const-decl | package-example | comptime-block | provide-stmt }
    fn parse_module(&mut self, key: &str) -> Module {
        let start = self.location();
        let mut imports = Vec::new();
        let mut decls = Vec::new();
        loop {
            let parsed = match self.peek() {
                TokenKind::Eof => break,
                TokenKind::Newline => {
                    self.advance();
                    continue;
                }
                TokenKind::Keyword(Keyword::Import) => self.parse_import().map(|i| imports.push(i)),
                _ => self.parse_decl().map(|d| decls.push(d)),
            };
            if let Err(diagnostic) = parsed {
                self.diagnostics.push(diagnostic);
                self.recover();
            }
        }
        Module {
            span: self.span_from(start),
            key: key.to_string(),
            imports,
            decls,
        }
    }

    // import-decl = "import" ident { "," ident } "from" string newline
    fn parse_import(&mut self) -> Parsed<Import> {
        let start = self.expect_keyword(Keyword::Import)?;
        let mut names = vec![self.expect_ident("an imported name")?];
        while self.at(&TokenKind::Comma) {
            self.advance();
            names.push(self.expect_ident("an imported name")?);
        }
        self.expect_keyword(Keyword::From)?;
        let path = self.parse_literal_string("L001", "an import path")?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Import { span, names, path })
    }

    fn parse_decl(&mut self) -> Parsed<Decl> {
        match self.peek() {
            TokenKind::Keyword(Keyword::Type) => self.parse_type_decl(),
            TokenKind::Keyword(Keyword::Fn) => self.parse_fn_decl(),
            TokenKind::Keyword(Keyword::Resource) => self.parse_resource_decl(),
            TokenKind::Keyword(Keyword::Const) => self.parse_const_decl(),
            TokenKind::Keyword(Keyword::Example) => {
                let start = self.location();
                let example = self.parse_example(true)?;
                Ok(Decl::PackageExample {
                    span: self.span_from(start),
                    example,
                })
            }
            TokenKind::Keyword(Keyword::Comptime) => self.parse_comptime_block(),
            TokenKind::Keyword(Keyword::Provide) => {
                let (span, name, init) = self.parse_provide()?;
                Ok(Decl::Provide { span, name, init })
            }
            _ => Err(self.error_here(
                "a declaration (import, type, fn, resource, const, example, comptime)",
            )),
        }
    }

    // type-decl = alias-decl | generative-decl | product-or-sum
    fn parse_type_decl(&mut self) -> Parsed<Decl> {
        let start = self.expect_keyword(Keyword::Type)?;
        let name = self.expect_ident("a type name")?;
        if self.at(&TokenKind::Equals) {
            self.advance();
            let ty = self.parse_type_expr()?;
            let span = self.span_from(start);
            self.expect_newline()?;
            return Ok(Decl::Alias { span, name, ty });
        }
        self.expect_newline()?;
        let closer = Closer::Named(Keyword::Type, &name);
        // generative-decl: chosen when the first body line is an effect
        if effect_first(self.peek()) {
            let effect = self.parse_effect()?;
            if self.at_keyword(Keyword::Comptime) && *self.peek_at(1) == TokenKind::Newline {
                self.advance();
                self.advance();
            }
            let body = self.parse_stmt_seq(false)?;
            self.parse_closer(&Closer::Bare("a generative type body"))?;
            self.expect_newline()?;
            self.parse_closer(&closer)?;
            let span = self.span_from(start);
            self.expect_newline()?;
            return Ok(Decl::Generative {
                span,
                name,
                effect,
                body,
            });
        }
        // product-body when the first line parses as a field, else sum-body
        let decl = if self.field_first() {
            let mut fields = Vec::new();
            while self.field_first() {
                fields.push(self.parse_field()?);
            }
            self.parse_closer(&closer)?;
            Decl::Product {
                span: self.span_from(start),
                name,
                fields,
            }
        } else {
            let mut variants = vec![self.parse_variant()?];
            while matches!(self.peek(), TokenKind::Identifier(_)) {
                variants.push(self.parse_variant()?);
            }
            self.parse_closer(&closer)?;
            Decl::Sum {
                span: self.span_from(start),
                name,
                variants,
            }
        };
        self.expect_newline()?;
        Ok(decl)
    }

    fn field_first(&self) -> bool {
        matches!(self.peek(), TokenKind::Identifier(_)) && *self.peek_at(1) == TokenKind::Colon
    }

    // field = ident ":" type-expr newline
    fn parse_field(&mut self) -> Parsed<Field> {
        let start = self.location();
        let name = self.expect_ident("a field name")?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type_expr()?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Field { span, name, ty })
    }

    // variant = ident [ "(" type-expr ")" ] newline
    fn parse_variant(&mut self) -> Parsed<Variant> {
        let start = self.location();
        let name = self.expect_ident("a field or variant")?;
        let payload = if self.at(&TokenKind::LeftParen) {
            let opened_at = self.open(&TokenKind::LeftParen, true)?;
            let ty = self.parse_type_expr()?;
            self.close(&TokenKind::RightParen, opened_at)?;
            Some(ty)
        } else {
            None
        };
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Variant {
            span,
            name,
            payload,
        })
    }

    // resource-decl = "resource" ident newline resource-fn "end" "resource" ident newline
    // resource-fn = "fn" ident params "->" type-expr newline ( "reads" | "writes" ) newline
    fn parse_resource_decl(&mut self) -> Parsed<Decl> {
        let start = self.expect_keyword(Keyword::Resource)?;
        let name = self.expect_ident("a resource name")?;
        self.expect_newline()?;
        let op_start = self.expect_keyword(Keyword::Fn)?;
        let op_name = self.expect_ident("an operation name")?;
        let params = self.parse_params()?;
        self.expect(&TokenKind::Arrow)?;
        let ret = self.parse_type_expr()?;
        self.expect_newline()?;
        let effect = match self.peek() {
            TokenKind::Keyword(Keyword::Reads) => ResourceEffect::Read,
            TokenKind::Keyword(Keyword::Writes) => ResourceEffect::Write,
            _ => return Err(self.error_here("`reads` or `writes`")),
        };
        self.advance();
        let op = ResourceOp {
            span: self.span_from(op_start),
            name: op_name,
            params,
            ret,
            effect,
        };
        self.expect_newline()?;
        self.parse_closer(&Closer::Named(Keyword::Resource, &name))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Decl::Resource { span, name, op })
    }

    // fn-decl = "fn" ident params "->" type-expr newline effect { newline }
    //           fn-examples { newline } fn-tail "end" "fn" ident newline
    // fn-examples may be empty only when the function is `main`
    fn parse_fn_decl(&mut self) -> Parsed<Decl> {
        let start = self.expect_keyword(Keyword::Fn)?;
        let name = self.expect_ident("a function name")?;
        let params = self.parse_params()?;
        self.expect(&TokenKind::Arrow)?;
        let ret = self.parse_type_expr()?;
        self.expect_newline()?;
        let effect = self.parse_effect()?;
        self.skip_blank_lines();
        if name != "main" && !self.at_keyword(Keyword::Example) {
            return Err(syntax_error(
                format!("function `{name}` needs at least one example before its body"),
                self.location(),
            ));
        }
        let mut examples = Vec::new();
        while self.at_keyword(Keyword::Example) {
            examples.push(self.parse_example(false)?);
            self.skip_blank_lines();
        }
        // fn-tail: `primitive` is an ident, taken only when a newline follows it
        let body = if *self.peek() == TokenKind::Identifier("primitive".to_string())
            && *self.peek_at(1) == TokenKind::Newline
        {
            self.advance();
            self.advance();
            FnBody::Primitive
        } else {
            FnBody::Statements(self.parse_stmt_seq(false)?)
        };
        self.parse_closer(&Closer::Named(Keyword::Fn, &name))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Decl::Function {
            span,
            name,
            params,
            ret,
            effect,
            examples,
            body,
        })
    }

    // "(" [ param { "," param } [ "," ] ] ")", param = ident ":" type-expr
    fn parse_params(&mut self) -> Parsed<Vec<Param>> {
        let (params, _) = self.parse_delimited(
            &TokenKind::LeftParen,
            &TokenKind::RightParen,
            true,
            |parser| {
                let start = parser.location();
                let name = parser.expect_ident("a parameter name")?;
                parser.expect(&TokenKind::Colon)?;
                let ty = parser.parse_type_expr()?;
                Ok(Param {
                    span: parser.span_from(start),
                    name,
                    ty,
                })
            },
        )?;
        Ok(params)
    }

    // effect = ( "pure" | "reads" ( "clock" | "net" | "fs" | "catalog" | ident )
    //          | "writes" ident ) newline
    fn parse_effect(&mut self) -> Parsed<Effect> {
        let effect = match self.peek() {
            TokenKind::Keyword(Keyword::Pure) => {
                self.advance();
                Effect::Pure
            }
            TokenKind::Keyword(Keyword::Reads) => {
                self.advance();
                let effect = match self.peek() {
                    TokenKind::Keyword(Keyword::Clock) => Effect::ReadsClock,
                    TokenKind::Keyword(Keyword::Net) => Effect::ReadsNet,
                    TokenKind::Keyword(Keyword::Fs) => Effect::ReadsFs,
                    TokenKind::Keyword(Keyword::Catalog) => Effect::ReadsCatalog,
                    TokenKind::Identifier(resource) => Effect::Reads(resource.clone()),
                    _ => {
                        return Err(self.error_here(
                            "`clock`, `net`, `fs`, `catalog`, or a resource name after `reads`",
                        ));
                    }
                };
                self.advance();
                effect
            }
            TokenKind::Keyword(Keyword::Writes) => {
                self.advance();
                Effect::Writes(self.expect_ident("a resource name after `writes`")?)
            }
            _ => return Err(self.error_here("an effect (`pure`, `reads`, or `writes`)")),
        };
        self.expect_newline()?;
        Ok(effect)
    }

    // const-decl = "const" ident [ ":" type-expr ] newline effect "=" expr newline
    //            | "const" ident [ ":" type-expr ] "=" expr newline
    fn parse_const_decl(&mut self) -> Parsed<Decl> {
        let start = self.expect_keyword(Keyword::Const)?;
        let name = self.expect_ident("a const name")?;
        let ty = self.parse_type_annotation()?;
        let effect = if self.at(&TokenKind::Newline) {
            self.advance();
            self.parse_effect()?
        } else {
            Effect::Pure
        };
        self.expect(&TokenKind::Equals)?;
        let init = self.parse_expr()?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Decl::Const {
            span,
            name,
            ty,
            effect,
            init,
        })
    }

    // comptime-block = "comptime" newline [ effect ] stmt-seq "end" newline
    fn parse_comptime_block(&mut self) -> Parsed<Decl> {
        let start = self.expect_keyword(Keyword::Comptime)?;
        self.expect_newline()?;
        let effect = if effect_first(self.peek()) {
            self.parse_effect()?
        } else {
            Effect::Pure
        };
        let body = self.parse_stmt_seq(false)?;
        self.parse_closer(&Closer::Bare("a comptime block"))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Decl::ComptimeBlock { span, effect, body })
    }

    // package-example = "example" string newline stmt-seq example-result "end" newline
    // example-block   = "example" string newline { example-bind } example-result "end" newline
    fn parse_example(&mut self, package: bool) -> Parsed<Example> {
        let start = self.expect_keyword(Keyword::Example)?;
        let label = self.parse_literal_string("L068", "an example label")?;
        self.expect_newline()?;
        let mut bindings = Vec::new();
        let body = if package {
            self.parse_stmt_seq(false)?
        } else {
            while self.bind_first() {
                bindings.push(self.parse_example_bind()?);
            }
            Vec::new()
        };
        // example-result = "->" "exception" string newline | "->" expr newline
        self.expect(&TokenKind::Arrow)?;
        let expect = if self.at_keyword(Keyword::Exception) {
            self.advance();
            Expect::Exception {
                message: self.parse_literal_string("L068", "an exception message")?,
            }
        } else {
            Expect::Value(self.parse_expr()?)
        };
        self.expect_newline()?;
        self.parse_closer(&Closer::Bare("an example"))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Example {
            span,
            label,
            bindings,
            body,
            expect,
        })
    }

    // A `reads clock` example binds `clock`, a keyword, so it is accepted beside ident
    fn bind_first(&self) -> bool {
        matches!(
            self.peek(),
            TokenKind::Identifier(_) | TokenKind::Keyword(Keyword::Clock)
        ) && *self.peek_at(1) == TokenKind::Colon
    }

    // example-bind = ( ident | "clock" ) ":" { newline } expr newline
    fn parse_example_bind(&mut self) -> Parsed<Bind> {
        let start = self.location();
        let name = if self.at_keyword(Keyword::Clock) {
            self.advance();
            Keyword::Clock.spelling().to_string()
        } else {
            self.expect_ident("a binding name")?
        };
        self.expect(&TokenKind::Colon)?;
        self.skip_blank_lines();
        let value = self.parse_expr()?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Bind { span, name, value })
    }

    // Types

    // type-expr = type-atom [ "at" "least" bound-expr | "in" bound-expr "through" bound-expr ]
    fn parse_type_expr(&mut self) -> Parsed<TypeExpr> {
        let start = self.location();
        let name = self.expect_ident("a type name")?;
        // type-atom = ident [ "<" type-expr { "," type-expr } [ "," ] ">" ]; a newline here is L001
        let atom = if self.at(&TokenKind::LessThan) {
            let (args, _) = self.parse_delimited(
                &TokenKind::LessThan,
                &TokenKind::GreaterThan,
                false,
                Self::parse_type_expr,
            )?;
            TypeExpr::Apply(self.span_from(start), name, args)
        } else {
            TypeExpr::Named(self.span_from(start), name)
        };
        match self.peek() {
            TokenKind::Keyword(Keyword::At) => {
                self.advance();
                self.expect_keyword(Keyword::Least)?;
                let low = self.parse_sum_expr()?;
                Ok(TypeExpr::AtLeast(
                    self.span_from(start),
                    Box::new(atom),
                    low,
                ))
            }
            TokenKind::Keyword(Keyword::In) => {
                self.advance();
                let low = self.parse_sum_expr()?;
                self.expect_keyword(Keyword::Through)?;
                let high = self.parse_sum_expr()?;
                Ok(TypeExpr::Range(
                    self.span_from(start),
                    Box::new(atom),
                    low,
                    high,
                ))
            }
            _ => Ok(atom),
        }
    }

    fn parse_type_annotation(&mut self) -> Parsed<Option<TypeExpr>> {
        if self.at(&TokenKind::Colon) {
            self.advance();
            Ok(Some(self.parse_type_expr()?))
        } else {
            Ok(None)
        }
    }

    // Statements

    // stmt-seq = { newline | stmt }; inside a match arm block, a successful arm-start ends it
    fn parse_stmt_seq(&mut self, in_arm: bool) -> Parsed<Vec<Stmt>> {
        let mut stmts = Vec::new();
        loop {
            if self.at(&TokenKind::Newline) {
                self.advance();
            } else if (in_arm && self.at_arm_start()) || !stmt_first(self.peek()) {
                return Ok(stmts);
            } else {
                stmts.push(self.parse_stmt()?);
            }
        }
    }

    fn parse_stmt(&mut self) -> Parsed<Stmt> {
        let start = self.location();
        let stmt = match self.peek() {
            TokenKind::Keyword(keyword @ (Keyword::Let | Keyword::Var | Keyword::Const)) => {
                let keyword = *keyword;
                self.advance();
                let name = self.expect_ident("a binding name")?;
                let ty = self.parse_type_annotation()?;
                self.expect(&TokenKind::Equals)?;
                let init = self.parse_expr()?;
                let span = self.span_from(start);
                match keyword {
                    Keyword::Let => Stmt::Let {
                        span,
                        name,
                        ty,
                        init,
                    },
                    Keyword::Var => Stmt::Var {
                        span,
                        name,
                        ty,
                        init,
                    },
                    _ => Stmt::Const {
                        span,
                        name,
                        ty,
                        init,
                    },
                }
            }
            TokenKind::Keyword(Keyword::Set) => {
                self.advance();
                let name = self.expect_ident("a variable name")?;
                let op = match self.peek() {
                    TokenKind::Equals => AssignOp::Assign,
                    TokenKind::PlusEquals => AssignOp::AddAssign,
                    _ => return Err(self.error_here("`=` or `+=`")),
                };
                self.advance();
                let value = self.parse_expr()?;
                Stmt::Set {
                    span: self.span_from(start),
                    name,
                    op,
                    value,
                }
            }
            TokenKind::Keyword(Keyword::Return) => {
                self.advance();
                let value = self.parse_expr()?;
                Stmt::Return {
                    span: self.span_from(start),
                    value,
                }
            }
            TokenKind::Keyword(Keyword::Assert) => {
                self.advance();
                let cond = self.parse_expr()?;
                self.expect(&TokenKind::Comma)?;
                let message = self.parse_expr()?;
                Stmt::Assert {
                    span: self.span_from(start),
                    cond,
                    message,
                }
            }
            TokenKind::Keyword(Keyword::Fail) => {
                self.advance();
                let message = self.parse_expr()?;
                Stmt::Fail {
                    span: self.span_from(start),
                    message,
                }
            }
            TokenKind::Keyword(Keyword::Provide) => {
                let (span, name, init) = self.parse_provide()?;
                return Ok(Stmt::Provide { span, name, init });
            }
            TokenKind::Keyword(Keyword::If) => return self.parse_if(),
            TokenKind::Keyword(Keyword::For) => return self.parse_for(),
            TokenKind::Keyword(Keyword::While) => return self.parse_while(),
            _ => {
                let expr = self.parse_expr()?;
                Stmt::ExprStmt {
                    span: self.span_from(start),
                    expr,
                }
            }
        };
        self.expect_newline()?;
        Ok(stmt)
    }

    // provide-stmt = "provide" ident "=" expr newline
    fn parse_provide(&mut self) -> Parsed<(Span, String, Expr)> {
        let start = self.expect_keyword(Keyword::Provide)?;
        let name = self.expect_ident("a provided type name")?;
        self.expect(&TokenKind::Equals)?;
        let init = self.parse_expr()?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok((span, name, init))
    }

    // if-stmt = "if" expr newline stmt-seq [ "else" newline stmt-seq ] "end" newline
    fn parse_if(&mut self) -> Parsed<Stmt> {
        let start = self.expect_keyword(Keyword::If)?;
        let cond = self.parse_expr()?;
        self.expect_newline()?;
        let then = self.parse_stmt_seq(false)?;
        let otherwise = if self.at_keyword(Keyword::Else) {
            self.advance();
            self.expect_newline()?;
            Some(self.parse_stmt_seq(false)?)
        } else {
            None
        };
        self.parse_closer(&Closer::Bare("an `if`"))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Stmt::If {
            span,
            cond,
            then,
            otherwise,
        })
    }

    // for-stmt = "for" ident "in" expr [ "until" expr ] newline stmt-seq "end" newline
    fn parse_for(&mut self) -> Parsed<Stmt> {
        let start = self.expect_keyword(Keyword::For)?;
        let name = self.expect_ident("a loop variable")?;
        self.expect_keyword(Keyword::In)?;
        let iter = self.parse_expr()?;
        let high = if self.at_keyword(Keyword::Until) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect_newline()?;
        let body = self.parse_stmt_seq(false)?;
        self.parse_closer(&Closer::Bare("a `for`"))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(match high {
            Some(high) => Stmt::ForRange {
                span,
                name,
                low: iter,
                high,
                body,
            },
            None => Stmt::ForList {
                span,
                name,
                iter,
                body,
            },
        })
    }

    // while-stmt = "while" ident "is" pattern newline stmt-seq "end" newline
    fn parse_while(&mut self) -> Parsed<Stmt> {
        let start = self.expect_keyword(Keyword::While)?;
        let name = self.expect_ident("a variable name")?;
        self.expect_keyword(Keyword::Is)?;
        let pattern = self.parse_pattern()?;
        self.expect_newline()?;
        let body = self.parse_stmt_seq(false)?;
        self.parse_closer(&Closer::Bare("a `while`"))?;
        let span = self.span_from(start);
        self.expect_newline()?;
        Ok(Stmt::WhileIs {
            span,
            name,
            pattern,
            body,
        })
    }

    // Match

    // Predicate: arm-start = pattern "->", tried with backtracking
    fn at_arm_start(&mut self) -> bool {
        if !pattern_first(self.peek()) {
            return false;
        }
        let saved = (
            self.position,
            self.skip_newlines.len(),
            self.depth,
            self.previous_end,
        );
        let found = self.parse_pattern().is_ok() && self.at(&TokenKind::Arrow);
        self.position = saved.0;
        self.skip_newlines.truncate(saved.1);
        self.depth = saved.2;
        self.previous_end = saved.3;
        found
    }

    // match-expr = "match" expr newline arm { arm } "end"
    fn parse_match(&mut self) -> Parsed<Expr> {
        let start = self.location();
        if self.in_hole {
            return Err(syntax_error(
                "`match` is not allowed in an interpolation hole".to_string(),
                start,
            ));
        }
        self.expect_keyword(Keyword::Match)?;
        self.nest(start)?;
        let scrutinee = self.parse_expr()?;
        self.expect_newline()?;
        let mut arms = vec![self.parse_arm()?];
        while !self.at_keyword(Keyword::End) {
            arms.push(self.parse_arm()?);
        }
        self.parse_closer(&Closer::Bare("a `match`"))?;
        self.depth -= 1;
        let expr_arms = arms
            .iter()
            .filter(|arm| matches!(arm.body, ArmBody::ExprArm(_)))
            .count();
        if expr_arms != 0 && expr_arms != arms.len() {
            self.diagnostics.push(syntax_error(
                "match mixes expression arms and statement arms".to_string(),
                start,
            ));
        }
        let kind = if expr_arms == arms.len() {
            ArmKind::ExprArms
        } else {
            ArmKind::StmtArms
        };
        Ok(Expr::Match(
            self.span_from(start),
            Box::new(scrutinee),
            arms,
            kind,
        ))
    }

    // arm = pattern "->" ( "return" expr newline | newline stmt-seq | expr newline )
    fn parse_arm(&mut self) -> Parsed<Arm> {
        let start = self.location();
        let pattern = self.parse_pattern()?;
        self.expect(&TokenKind::Arrow)?;
        let body = match self.peek() {
            TokenKind::Keyword(Keyword::Return) => ArmBody::StmtArm(vec![self.parse_stmt()?]),
            TokenKind::Newline => {
                self.advance();
                ArmBody::StmtArm(self.parse_stmt_seq(true)?)
            }
            _ => {
                let expr = self.parse_expr()?;
                self.expect_newline()?;
                ArmBody::ExprArm(expr)
            }
        };
        Ok(Arm {
            span: self.span_from(start),
            pattern,
            body,
        })
    }

    // pattern = wildcard | "Ok" "(" pattern ")" | "Err" "(" pattern ")" | int-literal
    //         | string | "[" [ pattern { "," pattern } [ "," ] ] "]" | ident [ "(" pattern ")" ]
    fn parse_pattern(&mut self) -> Parsed<Pattern> {
        let start = self.location();
        match self.peek().clone() {
            TokenKind::Wildcard => {
                self.advance();
                Ok(Pattern::Wild(self.span_from(start)))
            }
            TokenKind::Keyword(keyword @ (Keyword::Ok | Keyword::Err)) => {
                self.advance();
                let payload = Box::new(self.parse_pattern_payload()?);
                let span = self.span_from(start);
                Ok(if keyword == Keyword::Ok {
                    Pattern::OkPat(span, payload)
                } else {
                    Pattern::ErrPat(span, payload)
                })
            }
            TokenKind::Minus => {
                self.advance();
                let TokenKind::Integer(value) = *self.peek() else {
                    return Err(self.error_here("an integer after `-`"));
                };
                self.advance();
                Ok(Pattern::LitInt(self.span_from(start), -value))
            }
            TokenKind::Integer(value) => {
                self.advance();
                Ok(Pattern::LitInt(self.span_from(start), value))
            }
            TokenKind::String(_) => {
                let text = self.parse_literal_string("L001", "a string pattern")?;
                Ok(Pattern::LitText(self.span_from(start), text))
            }
            TokenKind::LeftBracket => {
                let (items, _) = self.parse_delimited(
                    &TokenKind::LeftBracket,
                    &TokenKind::RightBracket,
                    true,
                    Self::parse_pattern,
                )?;
                Ok(Pattern::ListPat(self.span_from(start), items))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                if self.at(&TokenKind::LeftParen) {
                    let payload = Box::new(self.parse_pattern_payload()?);
                    Ok(Pattern::VariantPat(
                        self.span_from(start),
                        name,
                        Some(payload),
                    ))
                } else {
                    Ok(Pattern::Bind(self.span_from(start), name))
                }
            }
            _ => Err(self.error_here("a pattern")),
        }
    }

    fn parse_pattern_payload(&mut self) -> Parsed<Pattern> {
        let opened_at = self.open(&TokenKind::LeftParen, true)?;
        let pattern = self.parse_pattern()?;
        self.close(&TokenKind::RightParen, opened_at)?;
        Ok(pattern)
    }

    // Expressions

    // expr = match-expr | or-expr
    fn parse_expr(&mut self) -> Parsed<Expr> {
        if self.at_keyword(Keyword::Match) {
            self.parse_match()
        } else {
            self.parse_or_expr()
        }
    }

    // or-expr = cmp-expr [ "or" "return" [ cmp-expr ] | "or" "fail" expr ]
    fn parse_or_expr(&mut self) -> Parsed<Expr> {
        let start = self.location();
        let value = self.parse_cmp_expr()?;
        if !self.at_keyword(Keyword::Or) {
            return Ok(value);
        }
        if self.in_hole {
            return Err(syntax_error(
                "`or` is not allowed in an interpolation hole".to_string(),
                self.location(),
            ));
        }
        self.advance();
        match self.peek() {
            TokenKind::Keyword(Keyword::Return) => {
                self.advance();
                let mapping = if cmp_first(self.peek()) {
                    Some(Box::new(self.parse_cmp_expr()?))
                } else {
                    None
                };
                Ok(Expr::OrReturn(
                    self.span_from(start),
                    Box::new(value),
                    mapping,
                ))
            }
            TokenKind::Keyword(Keyword::Fail) => {
                self.advance();
                let message = self.parse_expr()?;
                Ok(Expr::OrFail(
                    self.span_from(start),
                    Box::new(value),
                    Box::new(message),
                ))
            }
            _ => Err(self.error_here("`return` or `fail` after `or`")),
        }
    }

    // cmp-expr = sum-expr [ cmp-op sum-expr ]; comparisons do not chain
    fn parse_cmp_expr(&mut self) -> Parsed<Expr> {
        let start = self.location();
        let left = self.parse_sum_expr()?;
        let Some(op) = cmp_op(self.peek()) else {
            return Ok(left);
        };
        self.advance();
        let right = self.parse_sum_expr()?;
        if cmp_op(self.peek()).is_some() {
            return Err(syntax_error(
                "comparison operators do not chain; `a == b == c` is not an expression".to_string(),
                self.location(),
            ));
        }
        Ok(Expr::Binary(
            self.span_from(start),
            op,
            Box::new(left),
            Box::new(right),
        ))
    }

    // sum-expr = mul-expr { ( "+" | "-" ) mul-expr }
    fn parse_sum_expr(&mut self) -> Parsed<Expr> {
        let start = self.location();
        let mut left = self.parse_mul_expr()?;
        loop {
            let op = match self.peek() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.parse_mul_expr()?;
            left = Expr::Binary(self.span_from(start), op, Box::new(left), Box::new(right));
        }
    }

    // mul-expr = unary { ( "*" | "/" | "%" ) unary }
    fn parse_mul_expr(&mut self) -> Parsed<Expr> {
        let start = self.location();
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                TokenKind::Asterisk => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Rem,
                _ => return Ok(left),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary(self.span_from(start), op, Box::new(left), Box::new(right));
        }
    }

    // unary = [ "-" | "comptime" ] postfix; the prefixes do not stack
    fn parse_unary(&mut self) -> Parsed<Expr> {
        let start = self.location();
        match self.peek() {
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expr::UnaryNeg(self.span_from(start), Box::new(operand)))
            }
            TokenKind::Keyword(Keyword::Comptime) => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expr::Comptime(self.span_from(start), Box::new(operand)))
            }
            _ => self.parse_postfix(),
        }
    }

    // postfix = primary { call-suffix | index-suffix | field-suffix }
    fn parse_postfix(&mut self) -> Parsed<Expr> {
        let start = self.location();
        let mut expr = self.parse_primary()?;
        loop {
            expr = match self.peek() {
                TokenKind::LeftParen => {
                    let (args, broken) = self.parse_delimited(
                        &TokenKind::LeftParen,
                        &TokenKind::RightParen,
                        true,
                        Self::parse_arg,
                    )?;
                    Expr::Call(self.span_from(start), Box::new(expr), args, broken)
                }
                TokenKind::LeftBracket => {
                    let opened_at = self.open(&TokenKind::LeftBracket, true)?;
                    let index = self.parse_expr()?;
                    self.close(&TokenKind::RightBracket, opened_at)?;
                    Expr::Index(self.span_from(start), Box::new(expr), Box::new(index))
                }
                TokenKind::Dot => {
                    self.advance();
                    let field = self.expect_ident("a field name")?;
                    Expr::Field(self.span_from(start), Box::new(expr), field)
                }
                _ => return Ok(expr),
            };
        }
    }

    // arg = [ ident ":" ] expr
    fn parse_arg(&mut self) -> Parsed<Arg> {
        let start = self.location();
        let name = self.parse_arg_name(None)?;
        let value = self.parse_expr()?;
        Ok(Arg {
            span: self.span_from(start),
            name,
            value,
        })
    }

    // `ident ":"` before an argument; `only` restricts it to one spelling (`path:`, `table:`)
    fn parse_arg_name(&mut self, only: Option<&str>) -> Parsed<Option<String>> {
        let TokenKind::Identifier(name) = self.peek() else {
            return Ok(None);
        };
        if *self.peek_at(1) != TokenKind::Colon {
            return Ok(None);
        }
        if let Some(only) = only.filter(|only| name != only) {
            return Err(self.error_here(&format!("`{only}:` or an expression")));
        }
        let name = name.clone();
        self.advance();
        self.advance();
        Ok(Some(name))
    }

    // "(" [ name ":" ] expr ")"; `named` is whether that name was written, then `broken`
    fn parse_parenthesized(&mut self, name: Option<&str>) -> Parsed<(Expr, bool, bool)> {
        let opened_at = self.open(&TokenKind::LeftParen, true)?;
        let named = if name.is_some() {
            self.parse_arg_name(name)?.is_some()
        } else {
            false
        };
        let expr = self.parse_expr()?;
        let broken = self.close(&TokenKind::RightParen, opened_at)?;
        Ok((expr, named, broken))
    }

    // keyword "." member, as in `clock.now`, `fs.read`, `catalog.columns`
    fn parse_member(&mut self, member: &str) -> Parsed<()> {
        self.advance();
        self.expect(&TokenKind::Dot)?;
        self.expect_ident_named(member)
    }

    #[allow(clippy::too_many_lines)]
    fn parse_primary(&mut self) -> Parsed<Expr> {
        let start = self.location();
        let expr = match self.peek().clone() {
            TokenKind::String(parts) => {
                self.advance();
                let parts = parts
                    .into_iter()
                    .map(|part| match part {
                        StringPart::Text(literal) => Ok(TextPart::Literal(literal)),
                        StringPart::Hole { source, location } => {
                            Ok(TextPart::Hole(parse_hole(&source, location)?))
                        }
                    })
                    .collect::<Parsed<_>>()?;
                Expr::Text(self.span_from(start), parts)
            }
            TokenKind::LeftBracket => {
                let (items, broken) = self.parse_delimited(
                    &TokenKind::LeftBracket,
                    &TokenKind::RightBracket,
                    true,
                    Self::parse_expr,
                )?;
                Expr::ListLit(self.span_from(start), items, broken)
            }
            // special-call = "int_in" "(" expr "," bound-expr "through" bound-expr ")"
            //              | "int_at_least" "(" expr "," bound-expr ")"
            TokenKind::Keyword(keyword @ (Keyword::IntIn | Keyword::IntAtLeast)) => {
                self.advance();
                let opened_at = self.open(&TokenKind::LeftParen, true)?;
                let value = Box::new(self.parse_expr()?);
                self.expect(&TokenKind::Comma)?;
                let low = Box::new(self.parse_sum_expr()?);
                let high = if keyword == Keyword::IntIn {
                    self.expect_keyword(Keyword::Through)?;
                    Some(Box::new(self.parse_sum_expr()?))
                } else {
                    None
                };
                self.close(&TokenKind::RightParen, opened_at)?;
                let span = self.span_from(start);
                match high {
                    Some(high) => Expr::IntIn(span, value, low, high),
                    None => Expr::IntAtLeast(span, value, low),
                }
            }
            TokenKind::Keyword(keyword @ (Keyword::True | Keyword::False)) => {
                self.advance();
                Expr::BoolLit(self.span_from(start), keyword == Keyword::True)
            }
            TokenKind::Keyword(Keyword::Nothing) => {
                self.advance();
                Expr::NothingLit(self.span_from(start))
            }
            TokenKind::Integer(value) => {
                self.advance();
                Expr::IntLit(self.span_from(start), value)
            }
            // `Ok(e)` and `Err(e)` are calls whose callee is the name
            TokenKind::Keyword(keyword @ (Keyword::Ok | Keyword::Err)) => {
                self.advance();
                let callee = Expr::Name(self.span_from(start), keyword.spelling().to_string());
                let arg_start = self.location();
                let (value, _, broken) = self.parse_parenthesized(None)?;
                let arg = Arg {
                    span: self.span_from(arg_start),
                    name: None,
                    value,
                };
                Expr::Call(self.span_from(start), Box::new(callee), vec![arg], broken)
            }
            // clock-now = "clock" "." "now" "(" ")"
            TokenKind::Keyword(Keyword::Clock) => {
                self.parse_member("now")?;
                let opened_at = self.open(&TokenKind::LeftParen, true)?;
                self.close(&TokenKind::RightParen, opened_at)?;
                Expr::ClockNow(self.span_from(start))
            }
            // fs-read = "fs" "." "read" "(" [ "path" ":" ] expr ")"
            TokenKind::Keyword(Keyword::Fs) => {
                self.parse_member("read")?;
                let (path, named, broken) = self.parse_parenthesized(Some("path"))?;
                Expr::FsRead(self.span_from(start), Box::new(path), named, broken)
            }
            // catalog-columns = "catalog" "." "columns" "(" [ "table" ":" ] expr ")"
            TokenKind::Keyword(Keyword::Catalog) => {
                self.parse_member("columns")?;
                let (table, named, broken) = self.parse_parenthesized(Some("table"))?;
                Expr::CatalogColumns(self.span_from(start), Box::new(table), named, broken)
            }
            // quote-expr = "quote" "(" { newline } ( stmt | expr ) { newline } ")"
            TokenKind::Keyword(Keyword::Quote) => {
                self.advance();
                let opened_at = self.open(&TokenKind::LeftParen, false)?;
                self.skip_blank_lines();
                let body = if stmt_keyword_first(self.peek()) {
                    QuoteBody::QuotedStmt(Box::new(self.parse_stmt()?))
                } else {
                    QuoteBody::QuotedExpr(Box::new(self.parse_expr()?))
                };
                self.skip_blank_lines();
                self.close(&TokenKind::RightParen, opened_at)?;
                Expr::Quote(self.span_from(start), body)
            }
            // unquote-expr = "unquote" "(" expr ")"
            TokenKind::Keyword(Keyword::Unquote) => {
                self.advance();
                let (value, _, _) = self.parse_parenthesized(None)?;
                Expr::Unquote(self.span_from(start), Box::new(value))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                Expr::Name(self.span_from(start), name)
            }
            // In a hole, `expr` rejects `match` and `or`, so this re-enters interp-expr
            TokenKind::LeftParen => self.parse_parenthesized(None)?.0,
            _ => return Err(self.error_here("an expression")),
        };
        Ok(expr)
    }
}

// interp-expr = cmp-expr, lexed from the hole's raw source at its place in the file
fn parse_hole(source: &str, location: Location) -> Parsed<Expr> {
    let mut lexer = Lexer::starting_at(source, location);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        let is_eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    let mut parser = Parser::new(tokens, true);
    let expr = parser.parse_expr()?;
    if !parser.at(&TokenKind::Eof) {
        return Err(parser.error_here("the end of the interpolation hole"));
    }
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn const_init(source: &str) -> Expr {
        let module = parse_source(source.as_bytes(), "test").unwrap();
        match module.decls.into_iter().next() {
            Some(Decl::Const { init, .. }) => init,
            other => panic!("expected a const, got {other:?}"),
        }
    }

    fn span(start: (usize, usize), end: (usize, usize)) -> Span {
        Span {
            start: Location {
                line: start.0,
                column: start.1,
            },
            end: Location {
                line: end.0,
                column: end.1,
            },
        }
    }

    fn codes(source: &str) -> Vec<&'static str> {
        parse_source(source.as_bytes(), "test")
            .unwrap_err()
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect()
    }

    #[test]
    fn spans_cover_the_node_and_stop_before_the_newline() {
        let init = const_init("const x = 1 + 22\n");
        assert_eq!(init.span(), span((1, 11), (1, 17)));
        let module = parse_source(b"const x = 1 + 22\n", "test").unwrap();
        assert_eq!(module.span, span((1, 1), (1, 17)));
    }

    #[test]
    fn hole_expressions_keep_their_file_position() {
        let Expr::Text(_, parts) = const_init("const s = \"a\\n{b.c}\"") else {
            panic!("expected text");
        };
        let TextPart::Hole(hole) = &parts[1] else {
            panic!("expected a hole");
        };
        assert_eq!(hole.span(), span((1, 16), (1, 19)));
    }

    #[test]
    fn newlines_inside_brackets_mark_the_list_broken() {
        let Expr::ListLit(_, items, broken) = const_init("const xs = [\n  1,\n  2,\n]\n") else {
            panic!("expected a list");
        };
        assert_eq!(items.len(), 2);
        assert!(broken);
    }

    #[test]
    fn div_and_rem_bind_like_mul_and_associate_left() {
        let Expr::Binary(_, BinOp::Add, _, right) = const_init("const x = a + b / c * d % e")
        else {
            panic!("expected an add");
        };
        let Expr::Binary(_, BinOp::Rem, left, _) = *right else {
            panic!("expected a rem");
        };
        let Expr::Binary(_, BinOp::Mul, left, _) = *left else {
            panic!("expected a mul");
        };
        assert!(matches!(*left, Expr::Binary(_, BinOp::Div, _, _)));
    }

    #[test]
    fn nesting_deeper_than_64_is_l047() {
        let deep = format!("const x = {}1{}", "(".repeat(65), ")".repeat(65));
        assert_eq!(codes(&deep), vec!["L047"]);
        let ok = format!("const x = {}1{}", "(".repeat(64), ")".repeat(64));
        assert!(parse_source(ok.as_bytes(), "test").is_ok());
    }

    #[test]
    fn recovery_resumes_after_the_next_named_closer() {
        let source = "fn a() -> Int\n  pure\n  let\nend fn a\nconst b = )\n";
        assert_eq!(codes(source), vec!["L001", "L001"]);
    }
}
