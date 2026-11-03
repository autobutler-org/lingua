//! The `lingua tir` dump: one S-expression for a checked link set. Spans are omitted.
//!
//! A node is `(head child ...)`. Strings, including symbol names, are quoted. A bare atom is a
//! head, a tag, `true` / `false`, an integer, `nil`, `int-min`, `int-max`, or `:`. Every
//! expression ends with `:` and its type. A name that more than one symbol uses is `name#id`.

use std::collections::HashSet;

use super::{
    CmpOp, ExampleKind, Symbol, SymbolId, SymbolKind, TirArm, TirArmBody, TirBody, TirExample,
    TirExpect, TirExpr, TirFn, TirKind, TirModule, TirPat, TirProgram, TirStmt, Type, TypeKind,
};
use crate::ast::Effect;
use crate::provider::Generated;

const WIDTH: usize = 80;

enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}

/// Prints `program` as one S-expression, with a trailing newline.
#[must_use]
pub fn dump(program: &TirProgram) -> String {
    let names = Names::new(&program.symbols);
    let mut modules = Vec::new();
    for module in &program.modules {
        modules.push(names.module(module));
    }
    let sexpr = node(
        "program",
        [node("main", [string(&names.label(program.main))])]
            .into_iter()
            .chain(modules),
    );
    let mut out = String::new();
    sexpr.pretty(0, &mut out);
    out.push('\n');
    out
}

struct Names<'a> {
    symbols: &'a [Symbol],
    duplicated: HashSet<String>,
}

impl Names<'_> {
    fn new(symbols: &[Symbol]) -> Names<'_> {
        let mut seen = HashSet::new();
        let mut duplicated = HashSet::new();
        for symbol in symbols {
            if !seen.insert(symbol.name.clone()) {
                duplicated.insert(symbol.name.clone());
            }
        }
        Names {
            symbols,
            duplicated,
        }
    }

    fn label(&self, id: SymbolId) -> String {
        let Some(symbol) = self.symbols.get(id as usize) else {
            return format!("#{id}");
        };
        if self.duplicated.contains(&symbol.name) {
            format!("{}#{id}", symbol.name)
        } else {
            symbol.name.clone()
        }
    }

    fn module(&self, module: &TirModule) -> Sexpr {
        let mut children = vec![string(&module.key), string(&module.path)];
        let types = self.declared(&module.key);
        if !types.is_empty() {
            children.push(node("types", types));
        }
        if !module.examples.is_empty() {
            children.push(node(
                "examples",
                module.examples.iter().map(|example| self.example(example)),
            ));
        }
        children.push(node(
            "functions",
            module
                .functions
                .iter()
                .map(|function| self.function(function)),
        ));
        node("module", children)
    }

    fn declared(&self, key: &str) -> Vec<Sexpr> {
        self.symbols
            .iter()
            .enumerate()
            .filter(|(_, symbol)| symbol.module == key)
            .filter_map(|(index, symbol)| {
                let id = SymbolId::try_from(index).ok()?;
                self.declaration(id, symbol)
            })
            .collect()
    }

    fn declaration(&self, id: SymbolId, symbol: &Symbol) -> Option<Sexpr> {
        let name = string(&self.label(id));
        match &symbol.kind {
            SymbolKind::Type(kind) => Some(node("type", [name, self.type_kind(kind)])),
            SymbolKind::Variant {
                parent,
                tag,
                payload,
            } => Some(node(
                "variant",
                [
                    name,
                    string(&self.label(*parent)),
                    atom(&tag.to_string()),
                    payload
                        .as_ref()
                        .map_or_else(|| atom("nil"), |ty| self.ty(ty)),
                ],
            )),
            SymbolKind::Const => Some(node("const", [name, self.ty(&symbol.ty)])),
            SymbolKind::Resource => Some(node("resource", [name])),
            SymbolKind::ResourceOp { resource } => Some(node(
                "resource-op",
                [
                    name,
                    string(&self.label(*resource)),
                    symbol.effect.as_ref().map_or_else(|| atom("nil"), effect),
                ],
            )),
            SymbolKind::Field { .. }
            | SymbolKind::Function
            | SymbolKind::Param
            | SymbolKind::Local
            | SymbolKind::PreludeFn
            | SymbolKind::PreludeType => None,
        }
    }

    fn function(&self, function: &TirFn) -> Sexpr {
        node(
            "fn",
            [
                string(&self.label(function.id)),
                node(
                    "params",
                    function.params.iter().map(|id| {
                        let ty = self
                            .symbols
                            .get(*id as usize)
                            .map_or_else(|| atom("nil"), |symbol| self.ty(&symbol.ty));
                        node("param", [string(&self.label(*id)), ty])
                    }),
                ),
                self.ty(&function.ret),
                effect(&function.effect),
                node(
                    "examples",
                    function
                        .examples
                        .iter()
                        .map(|example| self.example(example)),
                ),
                self.body(&function.body),
            ],
        )
    }

    fn example(&self, example: &TirExample) -> Sexpr {
        let mut children = vec![
            string(&example.label),
            atom(match example.kind {
                ExampleKind::Function => "function",
                ExampleKind::Package => "package",
            }),
            node(
                "inputs",
                example
                    .inputs
                    .iter()
                    .map(|(id, value)| node("input", [string(&self.label(*id)), self.expr(value)])),
            ),
        ];
        if let Some(script) = &example.script {
            children.push(node("script", [self.expr(script)]));
        }
        if !example.body.is_empty() {
            children.push(self.statements(&example.body));
        }
        children.push(match &example.expect {
            TirExpect::Value(value) => node("value", [self.expr(value)]),
            TirExpect::Exception { message } => node("exception", [string(message)]),
        });
        node("example", children)
    }

    fn body(&self, body: &TirBody) -> Sexpr {
        match body {
            TirBody::Primitive => atom("primitive"),
            TirBody::Stmts(statements) => self.statements(statements),
        }
    }

    fn statements(&self, statements: &[TirStmt]) -> Sexpr {
        node("statements", statements.iter().map(|stmt| self.stmt(stmt)))
    }

    fn stmt(&self, stmt: &TirStmt) -> Sexpr {
        match stmt {
            TirStmt::Let(id, value) => node("let", [string(&self.label(*id)), self.expr(value)]),
            TirStmt::Set(id, value) => node("set", [string(&self.label(*id)), self.expr(value)]),
            TirStmt::Return(value) => node("return", [self.expr(value)]),
            TirStmt::Assert {
                cond,
                message,
                span: _,
            } => node("assert", [self.expr(cond), string(message)]),
            TirStmt::Fail { message, span: _ } => node("fail", [string(message)]),
            TirStmt::If(cond, then, else_) => node(
                "if",
                [
                    self.expr(cond),
                    node("then", then.iter().map(|stmt| self.stmt(stmt))),
                    node("else", else_.iter().map(|stmt| self.stmt(stmt))),
                ],
            ),
            TirStmt::ForList(id, list, body) => node(
                "for-list",
                [
                    string(&self.label(*id)),
                    self.expr(list),
                    self.statements(body),
                ],
            ),
            TirStmt::ForRange(id, low, high, body) => node(
                "for-range",
                [
                    string(&self.label(*id)),
                    self.expr(low),
                    self.expr(high),
                    self.statements(body),
                ],
            ),
            TirStmt::WhileIs {
                scrutinee,
                variant,
                payload,
                body,
            } => node(
                "while-is",
                [
                    string(&self.label(*scrutinee)),
                    string(&self.label(*variant)),
                    payload.map_or_else(|| atom("nil"), |id| string(&self.label(id))),
                    self.statements(body),
                ],
            ),
            TirStmt::Match(scrutinee, arms) => {
                node("match", [self.expr(scrutinee), self.arms(arms)])
            }
            TirStmt::Expr(value) => node("expr", [self.expr(value)]),
        }
    }

    fn arms(&self, arms: &[TirArm]) -> Sexpr {
        node(
            "arms",
            arms.iter()
                .map(|arm| node("arm", [self.pat(&arm.pat), self.arm_body(&arm.body)])),
        )
    }

    fn arm_body(&self, body: &TirArmBody) -> Sexpr {
        match body {
            TirArmBody::Expr(value) => node("expr", [self.expr(value)]),
            TirArmBody::Stmts(statements) => self.statements(statements),
        }
    }

    fn pat(&self, pat: &TirPat) -> Sexpr {
        match pat {
            TirPat::Wild => atom("wild"),
            TirPat::Bind(id) => node("bind", [string(&self.label(*id))]),
            TirPat::Variant(id, inner) => {
                let mut children = vec![string(&self.label(*id))];
                if let Some(inner) = inner {
                    children.push(self.pat(inner));
                }
                node("variant", children)
            }
            TirPat::LitInt(value) => node("lit-int", [integer(*value)]),
            TirPat::LitText(value) => node("lit-text", [string(value)]),
            TirPat::List(items) => node("list", items.iter().map(|item| self.pat(item))),
        }
    }

    fn expr(&self, expr: &TirExpr) -> Sexpr {
        let (head, children) = self.kind(expr);
        node(
            head,
            children.into_iter().chain([atom(":"), self.ty(&expr.ty)]),
        )
    }

    fn kind(&self, expr: &TirExpr) -> (&'static str, Vec<Sexpr>) {
        match &expr.kind {
            TirKind::Int(value) => ("int", vec![integer(*value)]),
            TirKind::Text(value) => ("text", vec![string(value)]),
            TirKind::Bool(value) => ("bool", vec![boolean(*value)]),
            TirKind::Nothing => ("nothing", vec![]),
            TirKind::Local(id) => ("local", vec![string(&self.label(*id))]),
            TirKind::List(items) => ("list", items.iter().map(|item| self.expr(item)).collect()),
            TirKind::Record(id, fields) => ("record", self.named(*id, fields)),
            TirKind::Variant(id, payload) => {
                let mut children = vec![string(&self.label(*id))];
                if let Some(payload) = payload {
                    children.push(self.expr(payload));
                }
                ("variant", children)
            }
            TirKind::Call(id, args) => ("call", self.named(*id, args)),
            TirKind::Field(target, index) => (
                "field",
                vec![self.expr(target), self.field_name(target, *index)],
            ),
            TirKind::Index {
                list,
                index,
                checked,
            } => (
                "index",
                vec![self.expr(list), self.expr(index), boolean(*checked)],
            ),
            TirKind::Neg(value) => ("neg", vec![self.expr(value)]),
            TirKind::Add(lhs, rhs) => ("add", vec![self.expr(lhs), self.expr(rhs)]),
            TirKind::Sub(lhs, rhs) => ("sub", vec![self.expr(lhs), self.expr(rhs)]),
            TirKind::Mul(lhs, rhs) => ("mul", vec![self.expr(lhs), self.expr(rhs)]),
            TirKind::Div(lhs, rhs) => ("div", vec![self.expr(lhs), self.expr(rhs)]),
            TirKind::Rem(lhs, rhs) => ("rem", vec![self.expr(lhs), self.expr(rhs)]),
            TirKind::Cmp(op, lhs, rhs) => {
                ("cmp", vec![atom(cmp(*op)), self.expr(lhs), self.expr(rhs)])
            }
            TirKind::Match(scrutinee, arms) => {
                ("match", vec![self.expr(scrutinee), self.arms(arms)])
            }
            TirKind::Concat(parts) => {
                ("concat", parts.iter().map(|part| self.expr(part)).collect())
            }
            TirKind::IntIn(value, lo, hi) => {
                ("int-in", vec![self.expr(value), integer(*lo), integer(*hi)])
            }
            TirKind::IntAtLeast(value, lo) => {
                ("int-at-least", vec![self.expr(value), integer(*lo)])
            }
            TirKind::ClockNow => ("clock-now", vec![]),
            TirKind::FsRead(path) => ("fs-read", vec![self.expr(path)]),
            TirKind::CatalogColumns(table) => ("catalog-columns", vec![self.expr(table)]),
            TirKind::TypeValue(generated) => ("type-value", vec![generated_sexpr(generated)]),
            TirKind::Quote(template, holes) => (
                "quote",
                std::iter::once(string(&crate::ast::print::quote_body(template)))
                    .chain(holes.iter().map(|hole| self.expr(hole)))
                    .collect(),
            ),
            TirKind::ResourceCall(op, args) => ("resource-call", self.named(*op, args)),
            TirKind::Log(message) => ("log", vec![self.expr(message)]),
            TirKind::Let(bindings, body) => (
                "let",
                vec![
                    node(
                        "bindings",
                        bindings.iter().map(|(id, value)| {
                            node("binding", [string(&self.label(*id)), self.expr(value)])
                        }),
                    ),
                    self.expr(body),
                ],
            ),
        }
    }

    fn named(&self, id: SymbolId, args: &[TirExpr]) -> Vec<Sexpr> {
        std::iter::once(string(&self.label(id)))
            .chain(args.iter().map(|arg| self.expr(arg)))
            .collect()
    }

    fn field_name(&self, target: &TirExpr, index: u32) -> Sexpr {
        let Type::Nominal(parent) = target.ty else {
            return atom(&index.to_string());
        };
        self.symbols
            .iter()
            .enumerate()
            .find_map(|(id, symbol)| match symbol.kind {
                SymbolKind::Field {
                    parent: field_parent,
                    index: field_index,
                } if field_parent == parent && field_index == index => SymbolId::try_from(id)
                    .ok()
                    .map(|id| string(&self.label(id))),
                _ => None,
            })
            .unwrap_or_else(|| atom(&index.to_string()))
    }

    fn ty(&self, ty: &Type) -> Sexpr {
        match ty {
            Type::Int => atom("int"),
            Type::Text => atom("text"),
            Type::Bool => atom("bool"),
            Type::Nothing => atom("nothing"),
            Type::Nominal(id) => node("nominal", [string(&self.label(*id))]),
            Type::List(inner) => node("list", [self.ty(inner)]),
            Type::Result(ok, err) => node("result", [self.ty(ok), self.ty(err)]),
            Type::Bound { lo, hi } => node("bound", [integer(*lo), integer(*hi)]),
            Type::ExprAst => atom("expr-ast"),
            Type::TypeAst => atom("type-ast"),
        }
    }

    fn type_kind(&self, kind: &TypeKind) -> Sexpr {
        match kind {
            TypeKind::Product => atom("product"),
            TypeKind::Sum => atom("sum"),
            TypeKind::Newtype { base, range } => {
                let mut children = vec![self.ty(base)];
                if let Some(range) = range {
                    children.push(node("range", [integer(range.lo), integer(range.hi)]));
                }
                node("newtype", children)
            }
        }
    }
}

fn generated_sexpr(generated: &Generated) -> Sexpr {
    match generated {
        Generated::Record(fields) => node(
            "record",
            fields
                .iter()
                .map(|(name, ty)| node("field", [string(name), string(ty)])),
        ),
        Generated::Sum(variants) => node(
            "sum",
            variants.iter().map(|(name, payload)| {
                node(
                    "variant",
                    [
                        string(name),
                        payload.as_deref().map_or_else(|| atom("nil"), string),
                    ],
                )
            }),
        ),
    }
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

    // One line when it fits; otherwise the head and its leading atoms, then a child per line.
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

fn integer(value: i64) -> Sexpr {
    if value == i64::MAX {
        atom("int-max")
    } else if value == i64::MIN {
        atom("int-min")
    } else {
        atom(&value.to_string())
    }
}

fn node(head: &str, children: impl IntoIterator<Item = Sexpr>) -> Sexpr {
    Sexpr::List(std::iter::once(atom(head)).chain(children).collect())
}

fn effect(effect: &Effect) -> Sexpr {
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

fn cmp(op: CmpOp) -> &'static str {
    match op {
        CmpOp::Eq => "eq",
        CmpOp::Ne => "ne",
        CmpOp::Lt => "lt",
        CmpOp::Le => "le",
        CmpOp::Gt => "gt",
        CmpOp::Ge => "ge",
    }
}
