//! The `.ast` stage dump: one S-expression per module, lossless except for spans.
//!
//! Each node is `(head child ...)` with children in Untyped AST field order. Every
//! `String` in the AST is a quoted string, so a bare atom is always a head, a tag
//! (`pure`, `add`, `expr-arms`, ...), `true` / `false`, an integer, or `nil` for an
//! absent option. A list field is `(label item ...)`; a node whose only payload is a
//! list puts the items right after its head.

use super::{
    Arg, Arm, ArmBody, ArmKind, AssignOp, BinOp, Decl, Effect, Example, Expect, Expr, FnBody,
    Module, Param, Pattern, QuoteBody, ResourceEffect, Stmt, TextPart, TypeExpr,
};

const WIDTH: usize = 80;

enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}

#[must_use]
pub fn dump(module: &Module) -> String {
    let mut out = String::new();
    module_sexpr(module).pretty(0, &mut out);
    out.push('\n');
    out
}

impl Sexpr {
    fn flat(&self) -> String {
        match self {
            Sexpr::Atom(atom) => atom.clone(),
            Sexpr::List(items) => {
                let inner: Vec<String> = items.iter().map(Sexpr::flat).collect();
                format!("({})", inner.join(" "))
            }
        }
    }

    // One line when it fits; otherwise the head and its leading atoms, then a child per line
    fn pretty(&self, indent: usize, out: &mut String) {
        let flat = self.flat();
        let Sexpr::List(items) = self else {
            out.push_str(&flat);
            return;
        };
        if indent + flat.len() <= WIDTH {
            out.push_str(&flat);
            return;
        }
        let leading = items
            .iter()
            .take_while(|item| matches!(item, Sexpr::Atom(_)))
            .count();
        let head: Vec<String> = items[..leading].iter().map(Sexpr::flat).collect();
        out.push('(');
        out.push_str(&head.join(" "));
        for item in &items[leading..] {
            out.push('\n');
            out.push_str(&" ".repeat(indent + 2));
            item.pretty(indent + 2, out);
        }
        out.push(')');
    }
}

fn atom(text: &str) -> Sexpr {
    Sexpr::Atom(text.to_string())
}

fn string(text: &str) -> Sexpr {
    let mut quoted = String::from('"');
    for c in text.chars() {
        match c {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\t' => quoted.push_str("\\t"),
            _ => quoted.push(c),
        }
    }
    quoted.push('"');
    Sexpr::Atom(quoted)
}

fn boolean(value: bool) -> Sexpr {
    atom(if value { "true" } else { "false" })
}

fn node(head: &str, children: impl IntoIterator<Item = Sexpr>) -> Sexpr {
    Sexpr::List(std::iter::once(atom(head)).chain(children).collect())
}

fn labeled<T>(label: &str, items: &[T], each: fn(&T) -> Sexpr) -> Sexpr {
    node(label, items.iter().map(each))
}

fn optional<T>(value: Option<&T>, each: fn(&T) -> Sexpr) -> Sexpr {
    value.map_or_else(|| atom("nil"), each)
}

fn module_sexpr(module: &Module) -> Sexpr {
    node(
        "module",
        [
            string(&module.key),
            labeled("imports", &module.imports, |import| {
                node(
                    "import",
                    [
                        labeled("names", &import.names, |name| string(name)),
                        string(&import.path),
                    ],
                )
            }),
            labeled("decls", &module.decls, decl_sexpr),
        ],
    )
}

fn param_sexpr(param: &Param) -> Sexpr {
    node("param", [string(&param.name), type_sexpr(&param.ty)])
}

fn decl_sexpr(decl: &Decl) -> Sexpr {
    match decl {
        Decl::Alias { name, ty, .. } => node("alias", [string(name), type_sexpr(ty)]),
        Decl::Product { name, fields, .. } => node(
            "product",
            [
                string(name),
                labeled("fields", fields, |field| {
                    node("field", [string(&field.name), type_sexpr(&field.ty)])
                }),
            ],
        ),
        Decl::Sum { name, variants, .. } => node(
            "sum",
            [
                string(name),
                labeled("variants", variants, |variant| {
                    node(
                        "variant",
                        [
                            string(&variant.name),
                            optional(variant.payload.as_ref(), type_sexpr),
                        ],
                    )
                }),
            ],
        ),
        Decl::Generative {
            name, effect, body, ..
        } => node(
            "generative",
            [
                string(name),
                effect_sexpr(effect),
                labeled("body", body, stmt_sexpr),
            ],
        ),
        Decl::Resource { name, op, .. } => node(
            "resource",
            [
                string(name),
                node(
                    "resource-op",
                    [
                        string(&op.name),
                        labeled("params", &op.params, param_sexpr),
                        type_sexpr(&op.ret),
                        atom(match op.effect {
                            ResourceEffect::Read => "read",
                            ResourceEffect::Write => "write",
                        }),
                    ],
                ),
            ],
        ),
        Decl::Function {
            name,
            params,
            ret,
            effect,
            examples,
            body,
            ..
        } => node(
            "fn",
            [
                string(name),
                labeled("params", params, param_sexpr),
                type_sexpr(ret),
                effect_sexpr(effect),
                labeled("examples", examples, example_sexpr),
                match body {
                    FnBody::Primitive => atom("primitive"),
                    FnBody::Statements(stmts) => node("statements", stmts.iter().map(stmt_sexpr)),
                },
            ],
        ),
        Decl::Const {
            name,
            ty,
            effect,
            init,
            ..
        } => node(
            "const",
            [
                string(name),
                optional(ty.as_ref(), type_sexpr),
                effect_sexpr(effect),
                expr_sexpr(init),
            ],
        ),
        Decl::PackageExample { example, .. } => node("package-example", [example_sexpr(example)]),
        Decl::ComptimeBlock { effect, body, .. } => node(
            "comptime-block",
            [effect_sexpr(effect), labeled("body", body, stmt_sexpr)],
        ),
        Decl::Provide { name, init, .. } => node("provide", [string(name), expr_sexpr(init)]),
    }
}

fn effect_sexpr(effect: &Effect) -> Sexpr {
    match effect {
        Effect::Pure => atom("pure"),
        Effect::ReadsClock => atom("reads-clock"),
        Effect::ReadsNet => atom("reads-net"),
        Effect::ReadsFs => atom("reads-fs"),
        Effect::ReadsCatalog => atom("reads-catalog"),
        Effect::Reads(resource) => node("reads", [string(resource)]),
        Effect::Writes(resource) => node("writes", [string(resource)]),
    }
}

fn example_sexpr(example: &Example) -> Sexpr {
    node(
        "example",
        [
            string(&example.label),
            labeled("bindings", &example.bindings, |bind| {
                node("binding", [string(&bind.name), expr_sexpr(&bind.value)])
            }),
            labeled("body", &example.body, stmt_sexpr),
            match &example.expect {
                Expect::Value(value) => node("value", [expr_sexpr(value)]),
                Expect::Exception { message } => node("exception", [string(message)]),
            },
        ],
    )
}

fn type_sexpr(ty: &TypeExpr) -> Sexpr {
    match ty {
        TypeExpr::Named(_, name) => node("named", [string(name)]),
        TypeExpr::Apply(_, name, args) => {
            node("apply", [string(name), labeled("args", args, type_sexpr)])
        }
        TypeExpr::AtLeast(_, base, low) => node("at-least", [type_sexpr(base), expr_sexpr(low)]),
        TypeExpr::Range(_, base, low, high) => node(
            "range",
            [type_sexpr(base), expr_sexpr(low), expr_sexpr(high)],
        ),
    }
}

fn binding_sexpr(head: &str, name: &str, ty: Option<&TypeExpr>, init: &Expr) -> Sexpr {
    node(
        head,
        [string(name), optional(ty, type_sexpr), expr_sexpr(init)],
    )
}

fn stmt_sexpr(stmt: &Stmt) -> Sexpr {
    match stmt {
        Stmt::Let { name, ty, init, .. } => binding_sexpr("let", name, ty.as_ref(), init),
        Stmt::Var { name, ty, init, .. } => binding_sexpr("var", name, ty.as_ref(), init),
        Stmt::Const { name, ty, init, .. } => binding_sexpr("const", name, ty.as_ref(), init),
        Stmt::Set {
            name, op, value, ..
        } => node(
            "set",
            [
                string(name),
                atom(match op {
                    AssignOp::Assign => "assign",
                    AssignOp::AddAssign => "add-assign",
                }),
                expr_sexpr(value),
            ],
        ),
        Stmt::Return { value, .. } => node("return", [expr_sexpr(value)]),
        Stmt::Assert { cond, message, .. } => {
            node("assert", [expr_sexpr(cond), expr_sexpr(message)])
        }
        Stmt::Fail { message, .. } => node("fail", [expr_sexpr(message)]),
        Stmt::If {
            cond,
            then,
            otherwise,
            ..
        } => node(
            "if",
            [
                expr_sexpr(cond),
                labeled("then", then, stmt_sexpr),
                optional(otherwise.as_ref(), |stmts| {
                    labeled("else", stmts, stmt_sexpr)
                }),
            ],
        ),
        Stmt::ForList {
            name, iter, body, ..
        } => node(
            "for-list",
            [
                string(name),
                expr_sexpr(iter),
                labeled("body", body, stmt_sexpr),
            ],
        ),
        Stmt::ForRange {
            name,
            low,
            high,
            body,
            ..
        } => node(
            "for-range",
            [
                string(name),
                expr_sexpr(low),
                expr_sexpr(high),
                labeled("body", body, stmt_sexpr),
            ],
        ),
        Stmt::WhileIs {
            name,
            pattern,
            body,
            ..
        } => node(
            "while-is",
            [
                string(name),
                pattern_sexpr(pattern),
                labeled("body", body, stmt_sexpr),
            ],
        ),
        Stmt::Provide { name, init, .. } => node("provide", [string(name), expr_sexpr(init)]),
        Stmt::ExprStmt { expr, .. } => node("expr-stmt", [expr_sexpr(expr)]),
    }
}

fn expr_sexpr(expr: &Expr) -> Sexpr {
    match expr {
        Expr::IntLit(_, value) => node("int", [atom(&value.to_string())]),
        Expr::Text(_, parts) => node(
            "text",
            parts.iter().map(|part| match part {
                TextPart::Literal(literal) => node("literal", [string(literal)]),
                TextPart::Hole(hole) => node("hole", [expr_sexpr(hole)]),
            }),
        ),
        Expr::BoolLit(_, value) => node("bool", [boolean(*value)]),
        Expr::NothingLit(_) => node("nothing", []),
        Expr::Name(_, name) => node("name", [string(name)]),
        Expr::ListLit(_, items, broken) => node(
            "list",
            [labeled("items", items, expr_sexpr), boolean(*broken)],
        ),
        Expr::Call(_, callee, args, broken) => node(
            "call",
            [
                expr_sexpr(callee),
                labeled("args", args, arg_sexpr),
                boolean(*broken),
            ],
        ),
        Expr::Field(_, base, field) => node("field", [expr_sexpr(base), string(field)]),
        Expr::Index(_, base, index) => node("index", [expr_sexpr(base), expr_sexpr(index)]),
        Expr::UnaryNeg(_, operand) => node("neg", [expr_sexpr(operand)]),
        Expr::Binary(_, op, left, right) => node(
            "binary",
            [binop_sexpr(*op), expr_sexpr(left), expr_sexpr(right)],
        ),
        Expr::Comptime(_, operand) => node("comptime", [expr_sexpr(operand)]),
        Expr::OrReturn(_, value, mapping) => node(
            "or-return",
            [expr_sexpr(value), optional(mapping.as_deref(), expr_sexpr)],
        ),
        Expr::OrFail(_, value, message) => {
            node("or-fail", [expr_sexpr(value), expr_sexpr(message)])
        }
        Expr::Match(_, scrutinee, arms, kind) => node(
            "match",
            [
                expr_sexpr(scrutinee),
                labeled("arms", arms, arm_sexpr),
                atom(match kind {
                    ArmKind::ExprArms => "expr-arms",
                    ArmKind::StmtArms => "stmt-arms",
                }),
            ],
        ),
        Expr::IntIn(_, value, low, high) => node(
            "int-in",
            [expr_sexpr(value), expr_sexpr(low), expr_sexpr(high)],
        ),
        Expr::IntAtLeast(_, value, low) => {
            node("int-at-least", [expr_sexpr(value), expr_sexpr(low)])
        }
        Expr::Quote(_, body) => node(
            "quote",
            [match body {
                QuoteBody::QuotedStmt(stmt) => node("quoted-stmt", [stmt_sexpr(stmt)]),
                QuoteBody::QuotedExpr(expr) => node("quoted-expr", [expr_sexpr(expr)]),
            }],
        ),
        Expr::Unquote(_, value) => node("unquote", [expr_sexpr(value)]),
        Expr::ClockNow(_) => node("clock-now", []),
        Expr::FsRead(_, path, named, broken) => node(
            "fs-read",
            [expr_sexpr(path), boolean(*named), boolean(*broken)],
        ),
        Expr::CatalogColumns(_, table, named, broken) => node(
            "catalog-columns",
            [expr_sexpr(table), boolean(*named), boolean(*broken)],
        ),
    }
}

fn binop_sexpr(op: BinOp) -> Sexpr {
    atom(match op {
        BinOp::Add => "add",
        BinOp::Sub => "sub",
        BinOp::Mul => "mul",
        BinOp::Div => "div",
        BinOp::Rem => "rem",
        BinOp::Eq => "eq",
        BinOp::Ne => "ne",
        BinOp::Lt => "lt",
        BinOp::Gt => "gt",
        BinOp::Le => "le",
        BinOp::Ge => "ge",
    })
}

fn arg_sexpr(arg: &Arg) -> Sexpr {
    node(
        "arg",
        [
            optional(arg.name.as_ref(), |name| string(name)),
            expr_sexpr(&arg.value),
        ],
    )
}

fn arm_sexpr(arm: &Arm) -> Sexpr {
    node(
        "arm",
        [
            pattern_sexpr(&arm.pattern),
            match &arm.body {
                ArmBody::ExprArm(expr) => node("expr-arm", [expr_sexpr(expr)]),
                ArmBody::StmtArm(stmts) => node("stmt-arm", stmts.iter().map(stmt_sexpr)),
            },
        ],
    )
}

fn pattern_sexpr(pattern: &Pattern) -> Sexpr {
    match pattern {
        Pattern::Wild(_) => node("wild", []),
        Pattern::LitInt(_, value) => node("lit-int", [atom(&value.to_string())]),
        Pattern::LitText(_, text) => node("lit-text", [string(text)]),
        Pattern::Bind(_, name) => node("bind", [string(name)]),
        Pattern::OkPat(_, inner) => node("ok-pat", [pattern_sexpr(inner)]),
        Pattern::ErrPat(_, inner) => node("err-pat", [pattern_sexpr(inner)]),
        Pattern::VariantPat(_, name, payload) => node(
            "variant-pat",
            [string(name), optional(payload.as_deref(), pattern_sexpr)],
        ),
        Pattern::ListPat(_, items) => node("list-pat", items.iter().map(pattern_sexpr)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breaks_long_lists_after_leading_atoms() {
        let long = node(
            "fn",
            [
                string("name"),
                node("a", [atom(&"x".repeat(40))]),
                node("b", [atom(&"y".repeat(40))]),
            ],
        );
        let mut out = String::new();
        long.pretty(0, &mut out);
        assert_eq!(
            out,
            format!(
                "(fn \"name\"\n  (a {})\n  (b {}))",
                "x".repeat(40),
                "y".repeat(40)
            )
        );
    }

    #[test]
    fn quotes_and_escapes_strings() {
        assert_eq!(string("a\"b\\c\nd").flat(), r#""a\"b\\c\nd""#);
    }
}
