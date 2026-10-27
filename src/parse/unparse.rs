//! unparse, the inverse of parse (docs/lingua1.md, Compilation pipeline, Source
//! generation): the untyped AST to a token list in the Formatter layout. It picks every line
//! break and indent and gives each token its `line:col`; unlex then only places them.
//! `parse(unparse(a)) == a`, ignoring spans.

use crate::ast::{
    Arg, Arm, ArmBody, AssignOp, BinOp, Decl, Effect, Example, Expect, Expr, FnBody, Module,
    Pattern, QuoteBody, ResourceEffect, Stmt, TextPart, TypeExpr,
};
use crate::lex::{Keyword, Location, StringPart, Token, TokenKind, escape, lexeme, unlex};

/// The module's tokens in the Formatter layout, ending with `Eof`.
#[must_use]
pub fn unparse(module: &Module) -> Vec<Token> {
    let mut printer = Printer::new();
    printer.module(module);
    printer.finish()
}

/// One `type-expr` on one line, ending with `Eof`.
#[must_use]
pub fn unparse_type(ty: &TypeExpr) -> Vec<Token> {
    let mut printer = Printer::new();
    printer.type_expr(ty);
    printer.finish()
}

// Binding strength, loosest first: the `min` an operand position accepts without parentheses
const MATCH: u8 = 0;
const OR: u8 = 1;
const CMP: u8 = 2;
const SUM: u8 = 3;
const MUL: u8 = 4;
const UNARY: u8 = 5;
const POSTFIX: u8 = 6;
const PRIMARY: u8 = 7;

fn strength(expr: &Expr) -> u8 {
    match expr {
        Expr::Match(..) => MATCH,
        Expr::OrReturn(..) | Expr::OrFail(..) => OR,
        Expr::Binary(_, op, ..) => match op {
            BinOp::Add | BinOp::Sub => SUM,
            BinOp::Mul | BinOp::Div | BinOp::Rem => MUL,
            _ => CMP,
        },
        Expr::UnaryNeg(..) | Expr::Comptime(..) => UNARY,
        Expr::Call(..) | Expr::Field(..) | Expr::Index(..) => POSTFIX,
        _ => PRIMARY,
    }
}

// A name is an ident unless it spells a keyword: `clock` in an example binding, `Ok` as a callee
fn name_kind(name: &str) -> TokenKind {
    Keyword::from_spelling(name).map_or_else(
        || TokenKind::Identifier(name.to_string()),
        TokenKind::Keyword,
    )
}

fn hugs(arg: &Arg, callee: &Expr) -> bool {
    matches!(
        arg.value,
        Expr::ListLit(_, _, true) | Expr::Call(_, _, _, true)
    ) && arg.span.start.line == callee.span().end.line
}

fn kw(keyword: Keyword) -> TokenKind {
    TokenKind::Keyword(keyword)
}

// Last column a packed literal's trailing comma may occupy. The printer's `column` counts
// Unicode scalars, the same way the lexer does.
const COLUMNS: usize = 80;

// Width of a literal's lexeme, or none when `expr` is not one. A literal is an int, unary
// `-` of an int, `true` / `false` / `nothing`, or a string with no holes.
fn literal_width(expr: &Expr) -> Option<usize> {
    let kind = match expr {
        Expr::IntLit(_, value) => TokenKind::Integer(*value),
        Expr::UnaryNeg(_, inner) => {
            let Expr::IntLit(_, value) = inner.as_ref() else {
                return None;
            };
            return Some(1 + lexeme(&TokenKind::Integer(*value)).chars().count());
        }
        Expr::BoolLit(_, value) => kw(if *value {
            Keyword::True
        } else {
            Keyword::False
        }),
        Expr::NothingLit(_) => kw(Keyword::Nothing),
        Expr::Text(_, parts) => {
            let mut spelled = Vec::with_capacity(parts.len());
            for part in parts {
                let TextPart::Literal(text) = part else {
                    return None;
                };
                spelled.push(StringPart::Text(text.clone()));
            }
            TokenKind::String(spelled)
        }
        _ => return None,
    };
    Some(lexeme(&kind).chars().count())
}

struct Printer {
    tokens: Vec<Token>,
    line: usize,
    column: usize,
    // Open constructs; a line starts at two spaces each
    indent: usize,
    // Whether the next token on this line has a space before it
    space: bool,
}

impl Printer {
    fn new() -> Self {
        Printer {
            tokens: Vec::new(),
            line: 1,
            column: 1,
            indent: 0,
            space: false,
        }
    }

    fn finish(mut self) -> Vec<Token> {
        let here = self.here();
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            location: here,
            end: here,
        });
        self.tokens
    }

    fn here(&self) -> Location {
        Location {
            line: self.line,
            column: self.column,
        }
    }

    // Where the next token starts
    fn next_column(&self) -> usize {
        if self.column == 1 {
            1 + 2 * self.indent
        } else {
            self.column + usize::from(self.space)
        }
    }

    // A token glued to the one before it, unless `sp` asked for a space
    fn t(&mut self, kind: TokenKind) {
        self.column = self.next_column();
        let location = self.here();
        self.column += lexeme(&kind).chars().count();
        self.tokens.push(Token {
            kind,
            location,
            end: self.here(),
        });
        self.space = false;
    }

    fn sp(&mut self) -> &mut Self {
        self.space = true;
        self
    }

    // A token with a space before it
    fn w(&mut self, kind: TokenKind) {
        self.sp().t(kind);
    }

    fn nl(&mut self) {
        let location = self.here();
        self.tokens.push(Token {
            kind: TokenKind::Newline,
            location,
            end: location,
        });
        self.line += 1;
        self.column = 1;
        self.space = false;
    }

    fn name(&mut self, name: &str) {
        self.w(name_kind(name));
    }

    fn string(&mut self, text: &str) {
        let parts = if text.is_empty() {
            Vec::new()
        } else {
            vec![StringPart::Text(text.to_string())]
        };
        self.t(TokenKind::String(parts));
    }

    // A text literal; each hole is its expression printed on one line, placed where it lands
    fn text(&mut self, parts: &[TextPart]) {
        let mut column = self.next_column() + 1;
        let parts = parts
            .iter()
            .map(|part| match part {
                TextPart::Literal(literal) => {
                    column += escape(literal).chars().count();
                    StringPart::Text(literal.clone())
                }
                TextPart::Hole(expr) => {
                    let mut hole = Printer::new();
                    hole.expr(expr, CMP);
                    let source = unlex(&hole.tokens);
                    let location = Location {
                        line: self.line,
                        column: column + 1,
                    };
                    column += source.chars().count() + 2;
                    StringPart::Hole { source, location }
                }
            })
            .collect();
        self.t(TokenKind::String(parts));
    }

    // `end`, then `fn` / `type` / `resource` and the name at file level
    fn closer(&mut self, kind: Option<(Keyword, &str)>) {
        self.w(kw(Keyword::End));
        if let Some((keyword, name)) = kind {
            self.w(kw(keyword));
            self.name(name);
        }
        self.nl();
    }

    // Module

    fn module(&mut self, module: &Module) {
        for import in &module.imports {
            self.w(kw(Keyword::Import));
            for (i, name) in import.names.iter().enumerate() {
                if i > 0 {
                    self.t(TokenKind::Comma);
                }
                self.name(name);
            }
            self.w(kw(Keyword::From));
            self.sp().string(&import.path);
            self.nl();
        }
        for (i, decl) in module.decls.iter().enumerate() {
            if i > 0 || !module.imports.is_empty() {
                self.nl();
            }
            self.decl(decl);
        }
    }

    fn block(&mut self, stmts: &[Stmt]) {
        self.indent += 1;
        for stmt in stmts {
            self.stmt(stmt);
        }
        self.indent -= 1;
    }

    #[allow(clippy::too_many_lines)]
    fn decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Alias { name, ty, .. } => {
                self.w(kw(Keyword::Type));
                self.name(name);
                self.w(TokenKind::Equals);
                self.sp().type_expr(ty);
                self.nl();
            }
            Decl::Product { name, fields, .. } => {
                self.w(kw(Keyword::Type));
                self.name(name);
                self.nl();
                self.indent += 1;
                for field in fields {
                    self.name(&field.name);
                    self.t(TokenKind::Colon);
                    self.sp().type_expr(&field.ty);
                    self.nl();
                }
                self.indent -= 1;
                self.closer(Some((Keyword::Type, name)));
            }
            Decl::Sum { name, variants, .. } => {
                self.w(kw(Keyword::Type));
                self.name(name);
                self.nl();
                self.indent += 1;
                for variant in variants {
                    self.name(&variant.name);
                    if let Some(payload) = &variant.payload {
                        self.t(TokenKind::LeftParen);
                        self.type_expr(payload);
                        self.t(TokenKind::RightParen);
                    }
                    self.nl();
                }
                self.indent -= 1;
                self.closer(Some((Keyword::Type, name)));
            }
            // The optional `comptime` line is always printed, so the body sits one level in
            Decl::Generative {
                name, effect, body, ..
            } => {
                self.w(kw(Keyword::Type));
                self.name(name);
                self.nl();
                self.indent += 1;
                self.effect(effect);
                self.w(kw(Keyword::Comptime));
                self.nl();
                self.block(body);
                self.closer(None);
                self.indent -= 1;
                self.closer(Some((Keyword::Type, name)));
            }
            Decl::Resource { name, op, .. } => {
                self.w(kw(Keyword::Resource));
                self.name(name);
                self.nl();
                self.indent += 1;
                self.w(kw(Keyword::Fn));
                self.name(&op.name);
                self.params(&op.params);
                self.w(TokenKind::Arrow);
                self.sp().type_expr(&op.ret);
                self.nl();
                self.indent += 1;
                self.w(kw(match op.effect {
                    ResourceEffect::Read => Keyword::Reads,
                    ResourceEffect::Write => Keyword::Writes,
                }));
                self.nl();
                self.indent -= 2;
                self.closer(Some((Keyword::Resource, name)));
            }
            Decl::Function {
                name,
                params,
                ret,
                effect,
                examples,
                body,
                ..
            } => {
                self.w(kw(Keyword::Fn));
                self.name(name);
                self.params(params);
                self.w(TokenKind::Arrow);
                self.sp().type_expr(ret);
                self.nl();
                self.indent += 1;
                self.effect(effect);
                for example in examples {
                    self.example(example);
                }
                if let FnBody::Primitive = body {
                    self.nl();
                    self.name("primitive");
                    self.nl();
                }
                self.indent -= 1;
                if let FnBody::Statements(stmts) = body {
                    if !stmts.is_empty() {
                        self.nl();
                    }
                    self.block(stmts);
                }
                self.closer(Some((Keyword::Fn, name)));
            }
            // The pure form keeps `=` on the header line
            Decl::Const {
                name,
                ty,
                effect,
                init,
                ..
            } => {
                self.w(kw(Keyword::Const));
                self.name(name);
                self.annotation(ty.as_ref());
                if *effect != Effect::Pure {
                    self.nl();
                    self.indent += 1;
                    self.effect(effect);
                }
                self.w(TokenKind::Equals);
                self.sp().expr(init, MATCH);
                self.nl();
                if *effect != Effect::Pure {
                    self.indent -= 1;
                }
            }
            Decl::PackageExample { example, .. } => self.example(example),
            Decl::ComptimeBlock { effect, body, .. } => {
                self.w(kw(Keyword::Comptime));
                self.nl();
                self.indent += 1;
                self.effect(effect);
                self.indent -= 1;
                self.block(body);
                self.closer(None);
            }
            Decl::Provide { name, init, .. } => self.provide(name, init),
        }
    }

    fn params(&mut self, params: &[crate::ast::Param]) {
        self.t(TokenKind::LeftParen);
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                self.t(TokenKind::Comma);
                self.space = true;
            }
            self.t(name_kind(&param.name));
            self.t(TokenKind::Colon);
            self.sp().type_expr(&param.ty);
        }
        self.t(TokenKind::RightParen);
    }

    fn effect(&mut self, effect: &Effect) {
        let reads = |p: &mut Self, what: TokenKind| {
            p.w(kw(Keyword::Reads));
            p.w(what);
        };
        match effect {
            Effect::Pure => self.w(kw(Keyword::Pure)),
            Effect::ReadsClock => reads(self, kw(Keyword::Clock)),
            Effect::ReadsNet => reads(self, kw(Keyword::Net)),
            Effect::ReadsFs => reads(self, kw(Keyword::Fs)),
            Effect::ReadsCatalog => reads(self, kw(Keyword::Catalog)),
            Effect::Reads(name) => reads(self, name_kind(name)),
            Effect::Writes(name) => {
                self.w(kw(Keyword::Writes));
                self.name(name);
            }
        }
        self.nl();
    }

    // A function example binds parameters; a package example runs statements
    fn example(&mut self, example: &Example) {
        self.w(kw(Keyword::Example));
        self.sp().string(&example.label);
        self.nl();
        self.indent += 1;
        for bind in &example.bindings {
            self.name(&bind.name);
            self.t(TokenKind::Colon);
            // The value may start on the line after `:`, as the author wrote it
            if bind.value.span().start.line > bind.span.start.line {
                self.nl();
                self.indent += 1;
                self.expr(&bind.value, MATCH);
                self.indent -= 1;
            } else {
                self.sp().expr(&bind.value, MATCH);
            }
            self.nl();
        }
        self.indent -= 1;
        self.block(&example.body);
        self.indent += 1;
        self.w(TokenKind::Arrow);
        match &example.expect {
            Expect::Value(value) => self.sp().expr(value, MATCH),
            Expect::Exception { message } => {
                self.w(kw(Keyword::Exception));
                self.sp().string(message);
            }
        }
        self.nl();
        self.indent -= 1;
        self.closer(None);
    }

    // Types

    fn type_expr(&mut self, ty: &TypeExpr) {
        match ty {
            TypeExpr::Named(_, name) => self.t(name_kind(name)),
            TypeExpr::Apply(_, name, args) => {
                self.t(name_kind(name));
                self.t(TokenKind::LessThan);
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.t(TokenKind::Comma);
                        self.space = true;
                    }
                    self.type_expr(arg);
                }
                self.t(TokenKind::GreaterThan);
            }
            TypeExpr::AtLeast(_, atom, low) => {
                self.type_expr(atom);
                self.w(kw(Keyword::At));
                self.w(kw(Keyword::Least));
                self.sp().expr(low, SUM);
            }
            TypeExpr::Range(_, atom, low, high) => {
                self.type_expr(atom);
                self.w(kw(Keyword::In));
                self.sp().expr(low, SUM);
                self.w(kw(Keyword::Through));
                self.sp().expr(high, SUM);
            }
        }
    }

    fn annotation(&mut self, ty: Option<&TypeExpr>) {
        if let Some(ty) = ty {
            self.t(TokenKind::Colon);
            self.sp().type_expr(ty);
        }
    }

    // Statements

    fn provide(&mut self, name: &str, init: &Expr) {
        self.w(kw(Keyword::Provide));
        self.name(name);
        self.w(TokenKind::Equals);
        self.sp().expr(init, MATCH);
        self.nl();
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, init, .. } => self.binding(Keyword::Let, name, ty.as_ref(), init),
            Stmt::Var { name, ty, init, .. } => self.binding(Keyword::Var, name, ty.as_ref(), init),
            Stmt::Const { name, ty, init, .. } => {
                self.binding(Keyword::Const, name, ty.as_ref(), init);
            }
            Stmt::Set {
                name, op, value, ..
            } => {
                self.w(kw(Keyword::Set));
                self.name(name);
                self.w(match op {
                    AssignOp::Assign => TokenKind::Equals,
                    AssignOp::AddAssign => TokenKind::PlusEquals,
                });
                self.sp().expr(value, MATCH);
                self.nl();
            }
            Stmt::Return { value, .. } => {
                self.w(kw(Keyword::Return));
                self.sp().expr(value, MATCH);
                self.nl();
            }
            Stmt::Assert { cond, message, .. } => {
                self.w(kw(Keyword::Assert));
                self.sp().expr(cond, MATCH);
                self.t(TokenKind::Comma);
                self.sp().expr(message, MATCH);
                self.nl();
            }
            Stmt::Fail { message, .. } => {
                self.w(kw(Keyword::Fail));
                self.sp().expr(message, MATCH);
                self.nl();
            }
            Stmt::If {
                cond,
                then,
                otherwise,
                ..
            } => {
                self.w(kw(Keyword::If));
                self.sp().expr(cond, MATCH);
                self.nl();
                self.block(then);
                if let Some(otherwise) = otherwise {
                    self.w(kw(Keyword::Else));
                    self.nl();
                    self.block(otherwise);
                }
                self.closer(None);
            }
            Stmt::ForList {
                name, iter, body, ..
            } => {
                self.w(kw(Keyword::For));
                self.name(name);
                self.w(kw(Keyword::In));
                self.sp().expr(iter, MATCH);
                self.nl();
                self.block(body);
                self.closer(None);
            }
            Stmt::ForRange {
                name,
                low,
                high,
                body,
                ..
            } => {
                self.w(kw(Keyword::For));
                self.name(name);
                self.w(kw(Keyword::In));
                self.sp().expr(low, MATCH);
                self.w(kw(Keyword::Until));
                self.sp().expr(high, MATCH);
                self.nl();
                self.block(body);
                self.closer(None);
            }
            Stmt::WhileIs {
                name,
                pattern,
                body,
                ..
            } => {
                self.w(kw(Keyword::While));
                self.name(name);
                self.w(kw(Keyword::Is));
                self.sp().pattern(pattern);
                self.nl();
                self.block(body);
                self.closer(None);
            }
            Stmt::Provide { name, init, .. } => self.provide(name, init),
            Stmt::ExprStmt { expr, .. } => {
                self.sp().expr(expr, MATCH);
                self.nl();
            }
        }
    }

    fn binding(&mut self, keyword: Keyword, name: &str, ty: Option<&TypeExpr>, init: &Expr) {
        self.w(kw(keyword));
        self.name(name);
        self.annotation(ty);
        self.w(TokenKind::Equals);
        self.sp().expr(init, MATCH);
        self.nl();
    }

    // Match

    fn arm(&mut self, arm: &Arm) {
        self.pattern(&arm.pattern);
        self.w(TokenKind::Arrow);
        match &arm.body {
            ArmBody::ExprArm(expr) => {
                self.sp().expr(expr, MATCH);
                self.nl();
            }
            // `pat -> return e` stays on one line when the author wrote it there
            ArmBody::StmtArm(stmts) => match stmts.as_slice() {
                [stmt @ Stmt::Return { span, .. }] if span.start.line == arm.span.start.line => {
                    self.stmt(stmt);
                }
                _ => {
                    self.nl();
                    self.block(stmts);
                }
            },
        }
    }

    fn pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Wild(_) => self.t(TokenKind::Wildcard),
            Pattern::LitInt(_, value) => {
                if *value < 0 {
                    self.t(TokenKind::Minus);
                }
                self.t(TokenKind::Integer(value.unsigned_abs().cast_signed()));
            }
            Pattern::LitText(_, text) => {
                self.string(text);
            }
            Pattern::Bind(_, name) | Pattern::VariantPat(_, name, None) => {
                self.t(name_kind(name));
            }
            Pattern::OkPat(_, inner) => self.pattern_call(&kw(Keyword::Ok), inner),
            Pattern::ErrPat(_, inner) => self.pattern_call(&kw(Keyword::Err), inner),
            Pattern::VariantPat(_, name, Some(inner)) => {
                self.pattern_call(&name_kind(name), inner);
            }
            Pattern::ListPat(_, items) => {
                self.t(TokenKind::LeftBracket);
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.t(TokenKind::Comma);
                        self.space = true;
                    }
                    self.pattern(item);
                }
                self.t(TokenKind::RightBracket);
            }
        }
    }

    fn pattern_call(&mut self, head: &TokenKind, inner: &Pattern) {
        self.t(head.clone());
        self.t(TokenKind::LeftParen);
        self.pattern(inner);
        self.t(TokenKind::RightParen);
    }

    // Expressions

    // `expr` where an operand needs at least `min` binding strength, parenthesized otherwise
    #[allow(clippy::too_many_lines)]
    fn expr(&mut self, expr: &Expr, min: u8) {
        if strength(expr) < min {
            self.t(TokenKind::LeftParen);
            self.expr(expr, MATCH);
            self.t(TokenKind::RightParen);
            return;
        }
        match expr {
            Expr::IntLit(_, value) => self.t(TokenKind::Integer(*value)),
            Expr::Text(_, parts) => self.text(parts),
            Expr::BoolLit(_, value) => {
                self.t(kw(if *value {
                    Keyword::True
                } else {
                    Keyword::False
                }));
            }
            Expr::NothingLit(_) => self.t(kw(Keyword::Nothing)),
            Expr::Name(_, name) => self.t(name_kind(name)),
            Expr::ListLit(_, items, broken) => {
                let packed = *broken && self.pack_literals(items);
                if !packed {
                    self.delimited(
                        (TokenKind::LeftBracket, TokenKind::RightBracket),
                        items,
                        *broken,
                        true,
                        |p, item| p.expr(item, MATCH),
                    );
                }
            }
            // A sole broken list or call argument the author started on the `(` line hugs the
            // parentheses, as in `sum_from_variants([` ... `])`
            Expr::Call(_, callee, args, true) if matches!(args.as_slice(), [arg] if hugs(arg, callee)) =>
            {
                self.expr(callee, POSTFIX);
                self.t(TokenKind::LeftParen);
                self.arg(&args[0]);
                self.t(TokenKind::RightParen);
            }
            Expr::Call(_, callee, args, broken) => {
                self.expr(callee, POSTFIX);
                // `Ok(e)` and `Err(e)` are one parenthesized expression: no trailing comma
                let comma =
                    !matches!(&**callee, Expr::Name(_, name) if name == "Ok" || name == "Err");
                self.delimited(
                    (TokenKind::LeftParen, TokenKind::RightParen),
                    args,
                    *broken,
                    comma,
                    Self::arg,
                );
            }
            Expr::Field(_, base, field) => {
                self.expr(base, POSTFIX);
                self.t(TokenKind::Dot);
                self.t(name_kind(field));
            }
            Expr::Index(_, base, index) => {
                self.expr(base, POSTFIX);
                self.t(TokenKind::LeftBracket);
                self.expr(index, MATCH);
                self.t(TokenKind::RightBracket);
            }
            Expr::UnaryNeg(_, operand) => {
                self.t(TokenKind::Minus);
                self.expr(operand, POSTFIX);
            }
            Expr::Comptime(_, operand) => {
                self.t(kw(Keyword::Comptime));
                self.sp().expr(operand, POSTFIX);
            }
            Expr::Binary(_, op, left, right) => {
                let (lhs, rhs, token) = match op {
                    BinOp::Add => (SUM, MUL, TokenKind::Plus),
                    BinOp::Sub => (SUM, MUL, TokenKind::Minus),
                    BinOp::Mul => (MUL, UNARY, TokenKind::Asterisk),
                    BinOp::Div => (MUL, UNARY, TokenKind::Slash),
                    BinOp::Rem => (MUL, UNARY, TokenKind::Percent),
                    BinOp::Eq => (SUM, SUM, TokenKind::EqualsEquals),
                    BinOp::Ne => (SUM, SUM, TokenKind::BangEquals),
                    BinOp::Lt => (SUM, SUM, TokenKind::LessThan),
                    BinOp::Gt => (SUM, SUM, TokenKind::GreaterThan),
                    BinOp::Le => (SUM, SUM, TokenKind::LessEquals),
                    BinOp::Ge => (SUM, SUM, TokenKind::GreaterEquals),
                };
                self.expr(left, lhs);
                self.w(token);
                self.sp().expr(right, rhs);
            }
            Expr::OrReturn(_, value, mapping) => {
                self.expr(value, CMP);
                self.w(kw(Keyword::Or));
                self.w(kw(Keyword::Return));
                if let Some(mapping) = mapping {
                    self.sp().expr(mapping, CMP);
                }
            }
            Expr::OrFail(_, value, message) => {
                self.expr(value, CMP);
                self.w(kw(Keyword::Or));
                self.w(kw(Keyword::Fail));
                self.sp().expr(message, MATCH);
            }
            Expr::Match(_, scrutinee, arms, _) => {
                self.t(kw(Keyword::Match));
                self.sp().expr(scrutinee, MATCH);
                self.nl();
                self.indent += 1;
                for arm in arms {
                    self.arm(arm);
                }
                self.indent -= 1;
                self.t(kw(Keyword::End));
            }
            Expr::IntIn(_, value, low, high) => {
                self.t(kw(Keyword::IntIn));
                self.t(TokenKind::LeftParen);
                self.expr(value, MATCH);
                self.t(TokenKind::Comma);
                self.sp().expr(low, SUM);
                self.w(kw(Keyword::Through));
                self.sp().expr(high, SUM);
                self.t(TokenKind::RightParen);
            }
            Expr::IntAtLeast(_, value, low) => {
                self.t(kw(Keyword::IntAtLeast));
                self.t(TokenKind::LeftParen);
                self.expr(value, MATCH);
                self.t(TokenKind::Comma);
                self.sp().expr(low, SUM);
                self.t(TokenKind::RightParen);
            }
            // A quoted statement keeps its own line, one level in, and `)` on the next
            Expr::Quote(_, body) => {
                self.t(kw(Keyword::Quote));
                self.t(TokenKind::LeftParen);
                match body {
                    QuoteBody::QuotedExpr(expr) => self.expr(expr, MATCH),
                    QuoteBody::QuotedStmt(stmt) => {
                        self.nl();
                        self.block(std::slice::from_ref(stmt));
                    }
                }
                self.t(TokenKind::RightParen);
            }
            Expr::Unquote(_, value) => {
                self.member_call(&[kw(Keyword::Unquote)], None, value, false);
            }
            Expr::ClockNow(_) => {
                self.t(kw(Keyword::Clock));
                self.t(TokenKind::Dot);
                self.t(TokenKind::Identifier("now".to_string()));
                self.t(TokenKind::LeftParen);
                self.t(TokenKind::RightParen);
            }
            Expr::FsRead(_, path, named, broken) => self.member_call(
                &[
                    kw(Keyword::Fs),
                    TokenKind::Dot,
                    TokenKind::Identifier("read".to_string()),
                ],
                named.then_some("path"),
                path,
                *broken,
            ),
            Expr::CatalogColumns(_, table, named, broken) => self.member_call(
                &[
                    kw(Keyword::Catalog),
                    TokenKind::Dot,
                    TokenKind::Identifier("columns".to_string()),
                ],
                named.then_some("table"),
                table,
                *broken,
            ),
        }
    }

    // `head([name: ]value)`, one parenthesized expression. No trailing comma: the grammar is
    // `"(" [ name ":" ] expr ")"`. A broken call puts the closer on its own line.
    fn member_call(&mut self, head: &[TokenKind], name: Option<&str>, value: &Expr, broken: bool) {
        for token in head {
            self.t(token.clone());
        }
        self.delimited(
            (TokenKind::LeftParen, TokenKind::RightParen),
            &[()],
            broken,
            false,
            |p, ()| {
                if let Some(name) = name {
                    p.t(name_kind(name));
                    p.t(TokenKind::Colon);
                    p.space = true;
                }
                p.expr(value, MATCH);
            },
        );
    }

    // A broken list of literals: several per line, trailing comma included, so that the comma's
    // last column is <= 80. Returns false when any element is not a literal.
    fn pack_literals(&mut self, items: &[Expr]) -> bool {
        let Some(widths) = items.iter().map(literal_width).collect::<Option<Vec<_>>>() else {
            return false;
        };
        self.t(TokenKind::LeftBracket);
        self.nl();
        self.indent += 1;
        let mut at_line_start = true;
        for (item, width) in items.iter().zip(widths) {
            if !at_line_start {
                // `column` is the next free column. A space, the lexeme, then the comma.
                let start = self.column + 1;
                if start + width > COLUMNS {
                    self.nl();
                    at_line_start = true;
                }
            }
            if !at_line_start {
                self.space = true;
            }
            self.expr(item, MATCH);
            self.t(TokenKind::Comma);
            at_line_start = false;
        }
        if !items.is_empty() {
            self.nl();
        }
        self.indent -= 1;
        self.t(TokenKind::RightBracket);
        true
    }

    fn arg(&mut self, arg: &Arg) {
        if let Some(name) = &arg.name {
            self.t(name_kind(name));
            self.t(TokenKind::Colon);
            self.space = true;
        }
        self.expr(&arg.value, MATCH);
    }

    // One line, or when the parser saw a newline inside: one item per line, a trailing
    // comma when the grammar takes one, and the closer back at the opening line's indent
    fn delimited<T>(
        &mut self,
        (open, close): (TokenKind, TokenKind),
        items: &[T],
        broken: bool,
        trailing_comma: bool,
        mut item: impl FnMut(&mut Self, &T),
    ) {
        self.t(open);
        if broken {
            self.nl();
            self.indent += 1;
            for each in items {
                item(self, each);
                if trailing_comma {
                    self.t(TokenKind::Comma);
                }
                self.nl();
            }
            self.indent -= 1;
        } else {
            for (i, each) in items.iter().enumerate() {
                if i > 0 {
                    self.t(TokenKind::Comma);
                    self.space = true;
                }
                item(self, each);
            }
        }
        self.t(close);
    }
}
