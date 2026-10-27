//! Lingua source for statements, expressions, and the declarations that hold them: what
//! `lingua expand` prints for a spliced body, and how a comptime `Expr` renders and compares.
//! Parentheses come from precedence, since the AST keeps none. Two-space indent, one
//! statement per line, closers from the opener, no comments and no `broken` layout.
// ponytail: a printer for spliced code only; lingua1.md's unparse (the formatter, #53)
// should replace it once it lands.

use std::fmt::Write;

use super::{
    Arg, ArmBody, AssignOp, BinOp, Decl, Effect, Example, Expect, Expr, FnBody, Pattern, QuoteBody,
    Span, Stmt, TextPart, TypeExpr,
};

/// The splice at a call site, if one replaced it.
pub type Spliced<'a> = &'a dyn Fn(Span) -> Option<&'a QuoteBody>;

fn none(_: Span) -> Option<&'static QuoteBody> {
    None
}

/// A quoted tree as the source inside `quote( )`, without the hygiene marks cleaned.
#[must_use]
pub fn quote_body(body: &QuoteBody) -> String {
    Printer { spliced: &none }.quote(body, 0)
}

/// `decl` as source, each call site `spliced` names replaced by its splice; `None` for a
/// declaration kind that holds no body.
#[must_use]
pub fn decl(decl: &Decl, spliced: Spliced<'_>) -> Option<String> {
    let p = Printer { spliced };
    match decl {
        Decl::Function {
            name,
            params,
            ret,
            effect,
            examples,
            body,
            ..
        } => {
            let params: Vec<String> = params
                .iter()
                .map(|q| format!("{}: {}", q.name, p.ty(&q.ty)))
                .collect();
            let mut out = format!("fn {name}({}) -> {}\n", params.join(", "), p.ty(ret));
            let _ = writeln!(out, "  {}", effect_text(effect));
            for ex in examples {
                out += &p.example(ex, 1);
            }
            out.push('\n');
            match body {
                FnBody::Primitive => out += "  primitive\n",
                FnBody::Statements(stmts) => out += &p.stmts(stmts, 1),
            }
            Some(out + &format!("end fn {name}\n"))
        }
        Decl::PackageExample { example, .. } => Some(p.example(example, 0)),
        Decl::Const { name, ty, init, .. } => {
            let ty = ty.as_ref().map(|t| format!(": {}", p.ty(t)));
            Some(format!(
                "const {name}{} = {}\n",
                ty.unwrap_or_default(),
                p.expr(init, 0)
            ))
        }
        _ => None,
    }
}

fn effect_text(effect: &Effect) -> String {
    match effect {
        Effect::Pure => "pure".into(),
        Effect::ReadsClock => "reads clock".into(),
        Effect::ReadsNet => "reads net".into(),
        Effect::ReadsFs => "reads fs".into(),
        Effect::ReadsCatalog => "reads catalog".into(),
        Effect::Reads(r) => format!("reads {r}"),
        Effect::Writes(r) => format!("writes {r}"),
    }
}

// A string literal with `text` as its content
fn string(text: &str) -> String {
    let mut out = String::from('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '{' => out.push_str("{{"),
            '}' => out.push_str("}}"),
            c => out.push(c),
        }
    }
    out + "\""
}

fn pattern(p: &Pattern) -> String {
    match p {
        Pattern::Wild(_) => "_".into(),
        Pattern::LitInt(_, n) => n.to_string(),
        Pattern::LitText(_, t) => string(t),
        Pattern::Bind(_, n) | Pattern::VariantPat(_, n, None) => n.clone(),
        Pattern::OkPat(_, inner) => format!("Ok({})", pattern(inner)),
        Pattern::ErrPat(_, inner) => format!("Err({})", pattern(inner)),
        Pattern::VariantPat(_, n, Some(inner)) => format!("{n}({})", pattern(inner)),
        Pattern::ListPat(_, items) => format!(
            "[{}]",
            items.iter().map(pattern).collect::<Vec<_>>().join(", ")
        ),
    }
}

fn pad(indent: usize) -> String {
    "  ".repeat(indent)
}

// Lowest first (docs/lingua1.md, Operator precedence and associativity)
fn precedence(e: &Expr) -> u8 {
    match e {
        Expr::Match(..) => 0,
        Expr::OrReturn(..) | Expr::OrFail(..) => 1,
        Expr::Binary(_, op, ..) => match op {
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => 2,
            BinOp::Add | BinOp::Sub => 3,
            BinOp::Mul | BinOp::Div | BinOp::Rem => 4,
        },
        Expr::UnaryNeg(..) | Expr::Comptime(..) => 5,
        _ => 6,
    }
}

fn op_text(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Rem => "%",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Lt => "<",
        BinOp::Gt => ">",
        BinOp::Le => "<=",
        BinOp::Ge => ">=",
    }
}

struct Printer<'a> {
    spliced: Spliced<'a>,
}

impl Printer<'_> {
    fn example(&self, ex: &Example, indent: usize) -> String {
        let (at, inner) = (pad(indent), pad(indent + 1));
        let mut out = format!("{at}example {}\n", string(&ex.label));
        for b in &ex.bindings {
            let _ = writeln!(
                out,
                "{inner}{}: {}",
                b.name,
                self.expr(&b.value, indent + 1)
            );
        }
        out += &self.stmts(&ex.body, indent + 1);
        let _ = match &ex.expect {
            Expect::Value(e) => writeln!(out, "{inner}-> {}", self.expr(e, indent + 1)),
            Expect::Exception { message } => {
                writeln!(out, "{inner}-> exception {}", string(message))
            }
        };
        out + &format!("{at}end\n")
    }

    fn stmts(&self, stmts: &[Stmt], indent: usize) -> String {
        stmts.iter().map(|s| self.stmt(s, indent)).collect()
    }

    fn block(&self, head: &str, body: &[Stmt], indent: usize) -> String {
        format!(
            "{head}\n{}{}end\n",
            self.stmts(body, indent + 1),
            pad(indent)
        )
    }

    #[allow(clippy::too_many_lines)]
    fn stmt(&self, s: &Stmt, indent: usize) -> String {
        let at = pad(indent);
        let e = |e: &Expr| self.expr(e, indent);
        let typed = |ty: &Option<TypeExpr>| {
            ty.as_ref()
                .map(|t| format!(": {}", self.ty(t)))
                .unwrap_or_default()
        };
        match s {
            Stmt::Let { name, ty, init, .. } => {
                format!("{at}let {name}{} = {}\n", typed(ty), e(init))
            }
            Stmt::Var { name, ty, init, .. } => {
                format!("{at}var {name}{} = {}\n", typed(ty), e(init))
            }
            Stmt::Const { name, ty, init, .. } => {
                format!("{at}const {name}{} = {}\n", typed(ty), e(init))
            }
            Stmt::Set {
                name, op, value, ..
            } => {
                let op = if *op == AssignOp::Assign { "=" } else { "+=" };
                format!("{at}set {name} {op} {}\n", e(value))
            }
            Stmt::Return { value, .. } => format!("{at}return {}\n", e(value)),
            Stmt::Assert { cond, message, .. } => {
                format!("{at}assert {}, {}\n", e(cond), e(message))
            }
            Stmt::Fail { message, .. } => format!("{at}fail {}\n", e(message)),
            Stmt::If {
                cond,
                then,
                otherwise,
                ..
            } => {
                let mut out = format!("{at}if {}\n{}", e(cond), self.stmts(then, indent + 1));
                if let Some(o) = otherwise {
                    let _ = write!(out, "{at}else\n{}", self.stmts(o, indent + 1));
                }
                out + &format!("{at}end\n")
            }
            Stmt::ForList {
                name, iter, body, ..
            } => self.block(&format!("{at}for {name} in {}", e(iter)), body, indent),
            Stmt::ForRange {
                name,
                low,
                high,
                body,
                ..
            } => self.block(
                &format!("{at}for {name} in {} until {}", e(low), e(high)),
                body,
                indent,
            ),
            Stmt::WhileIs {
                name,
                pattern: p,
                body,
                ..
            } => self.block(&format!("{at}while {name} is {}", pattern(p)), body, indent),
            Stmt::Provide { name, init, .. } => format!("{at}provide {name} = {}\n", e(init)),
            Stmt::ExprStmt { expr, .. } => match (self.spliced)(expr.span()) {
                Some(QuoteBody::QuotedStmt(s)) => Printer { spliced: &none }.stmt(s, indent),
                _ => format!("{at}{}\n", e(expr)),
            },
        }
    }

    fn expr(&self, e: &Expr, indent: usize) -> String {
        self.operand(e, 0, indent)
    }

    // `e` at precedence `min` or tighter, else in parentheses; a spliced call site prints
    // its splice
    fn operand(&self, e: &Expr, min: u8, indent: usize) -> String {
        if let Some(QuoteBody::QuotedExpr(spliced)) = (self.spliced)(e.span()) {
            return Printer { spliced: &none }.operand(spliced, min, indent);
        }
        let text = self.raw(e, indent);
        if precedence(e) < min {
            format!("({text})")
        } else {
            text
        }
    }

    #[allow(clippy::too_many_lines)]
    fn raw(&self, e: &Expr, indent: usize) -> String {
        let args = |args: &[Arg]| {
            args.iter()
                .map(|a| match &a.name {
                    Some(n) => format!("{n}: {}", self.expr(&a.value, indent)),
                    None => self.expr(&a.value, indent),
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        match e {
            Expr::IntLit(_, n) => n.to_string(),
            Expr::Text(_, parts) => {
                let mut out = String::from('"');
                for part in parts {
                    match part {
                        TextPart::Literal(s) => {
                            let quoted = string(s);
                            out += &quoted[1..quoted.len() - 1];
                        }
                        TextPart::Hole(h) => {
                            let _ = write!(out, "{{{}}}", self.operand(h, 2, indent));
                        }
                    }
                }
                out + "\""
            }
            Expr::BoolLit(_, b) => b.to_string(),
            Expr::NothingLit(_) => "nothing".into(),
            Expr::Name(_, n) => n.clone(),
            Expr::ListLit(_, items, _) => format!(
                "[{}]",
                items
                    .iter()
                    .map(|i| self.expr(i, indent))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Call(_, callee, a, _) => {
                format!("{}({})", self.operand(callee, 6, indent), args(a))
            }
            Expr::Field(_, base, field) => format!("{}.{field}", self.operand(base, 6, indent)),
            Expr::Index(_, list, index) => format!(
                "{}[{}]",
                self.operand(list, 6, indent),
                self.expr(index, indent)
            ),
            Expr::UnaryNeg(_, inner) => format!("-{}", self.operand(inner, 6, indent)),
            Expr::Comptime(_, inner) => format!("comptime {}", self.operand(inner, 6, indent)),
            Expr::Binary(_, op, a, b) => {
                let p = precedence(e);
                // Comparisons do not chain; the others are left-associative
                let left = if p == 2 { p + 1 } else { p };
                format!(
                    "{} {} {}",
                    self.operand(a, left, indent),
                    op_text(*op),
                    self.operand(b, p + 1, indent)
                )
            }
            Expr::OrReturn(_, inner, mapping) => {
                let mapping = mapping
                    .as_ref()
                    .map(|m| format!(" {}", self.operand(m, 2, indent)));
                format!(
                    "{} or return{}",
                    self.operand(inner, 2, indent),
                    mapping.unwrap_or_default()
                )
            }
            Expr::OrFail(_, inner, message) => format!(
                "{} or fail {}",
                self.operand(inner, 2, indent),
                self.expr(message, indent)
            ),
            Expr::Match(_, scrutinee, arms, _) => {
                let mut out = format!("match {}\n", self.expr(scrutinee, indent));
                for arm in arms {
                    let head = format!("{}{} ->", pad(indent + 1), pattern(&arm.pattern));
                    let _ = match &arm.body {
                        ArmBody::ExprArm(e) => writeln!(out, "{head} {}", self.expr(e, indent + 1)),
                        ArmBody::StmtArm(stmts) => {
                            write!(out, "{head}\n{}", self.stmts(stmts, indent + 2))
                        }
                    };
                }
                out + &format!("{}end", pad(indent))
            }
            Expr::IntIn(_, source, lo, hi) => format!(
                "int_in({}, {} through {})",
                self.expr(source, indent),
                self.operand(lo, 3, indent),
                self.operand(hi, 3, indent)
            ),
            Expr::IntAtLeast(_, source, lo) => format!(
                "int_at_least({}, {})",
                self.expr(source, indent),
                self.operand(lo, 3, indent)
            ),
            Expr::Quote(_, body) => format!("quote({})", self.quote(body, indent)),
            Expr::Unquote(_, inner) => format!("unquote({})", self.expr(inner, indent)),
            Expr::ClockNow(_) => "clock.now()".into(),
            Expr::FsRead(_, path) => format!("fs.read({})", self.expr(path, indent)),
            Expr::CatalogColumns(_, table) => {
                format!("catalog.columns({})", self.expr(table, indent))
            }
        }
    }

    // A quoted statement keeps its own line, which `quote( )` requires
    fn quote(&self, body: &QuoteBody, indent: usize) -> String {
        match body {
            QuoteBody::QuotedExpr(e) => self.expr(e, indent),
            QuoteBody::QuotedStmt(s) => format!("\n{}{}", self.stmt(s, indent + 1), pad(indent)),
        }
    }

    fn ty(&self, t: &TypeExpr) -> String {
        match t {
            TypeExpr::Named(_, n) => n.clone(),
            TypeExpr::Apply(_, n, args) => format!(
                "{n}<{}>",
                args.iter()
                    .map(|a| self.ty(a))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TypeExpr::AtLeast(_, base, lo) => {
                format!("{} at least {}", self.ty(base), self.operand(lo, 3, 0))
            }
            TypeExpr::Range(_, base, lo, hi) => format!(
                "{} in {} through {}",
                self.ty(base),
                self.operand(lo, 3, 0),
                self.operand(hi, 3, 0)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_source;

    // Each declaration prints back to the source it parsed from
    #[test]
    fn prints_what_it_parsed() {
        let source = "fn f(n: Int, xs: List<Int>) -> Int in 0 through 9\n  pure\n  example \"e\"\n    n: 1\n    xs: [1, 2]\n    -> 1\n  end\n\n  let a: Int = n - 1 - (2 - n) * -n\n  var b = \"x{n + 1}{{y}}\\n\"\n  set b += 1\n  if a == (b < 2)\n    assert true, \"ok\"\n  else\n    fail \"no\"\n  end\n  for i in 0 until n\n    log(text: \"{i}\")\n  end\n  for x in xs\n    const c = comptime f(1)\n  end\n  while s is More(c)\n    set s = Done\n  end\n  let r = int_in(\"5\", 1 through 3 + 1) or return Err(Bad)\n  let q = xs[0] or fail \"empty\"\n  match r\n    Ok([a, _]) ->\n      return a\n    Err(-1) ->\n      return 0\n  end\n  let m = match q\n    \"a\" -> 1\n    _ -> 2\n  end\n  let e = quote(\n    assert unquote(v) != 0, \"nonzero\"\n  )\n  return quote(1)\nend fn f\n\nexample \"zero is fatal\"\n  comptime assert_nonzero(0)\n  -> exception \"nonzero\"\nend\n\nconst k: Int at least 1 = clock.now()\n";
        let module = parse_source(source.as_bytes(), "m").unwrap();
        let printed: String = module
            .decls
            .iter()
            .map(|d| decl(d, &none).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(printed, source);
    }
}
