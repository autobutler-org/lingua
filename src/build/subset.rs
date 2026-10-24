//! The subset `lingua build` accepts so far, checked straight from the untyped AST into TIR.
//! The typechecker replaces this module; lowering from TIR stays.
//!
//! Accepted: one file, no imports, exactly one `fn main() -> Nothing` that is `pure`, whose
//! body is `log("literal")` statements ending in `return nothing`. Examples are ignored, since
//! they do not run natively. Anything else is a diagnostic.

use crate::ast::{Arg, Decl, Effect, Expr, FnBody, Module, Span, Stmt, TextPart, TypeExpr};
use crate::parse::Diagnostic;
use crate::tir::{TirExpr, TirFn, TirKind, TirStmt};

/// The code for a construct that is legal Lingua but not built yet. Not an `L` code.
const NOT_YET: &str = "error";

fn diagnostic(code: &'static str, span: Span, message: &str, help: Option<&str>) -> Diagnostic {
    Diagnostic {
        code,
        message: message.to_owned(),
        location: span.start,
        help: help.map(str::to_owned),
    }
}

fn not_yet(span: Span, message: &str) -> Diagnostic {
    diagnostic(NOT_YET, span, message, None)
}

fn decl_span(decl: &Decl) -> Span {
    match decl {
        Decl::Alias { span, .. }
        | Decl::Product { span, .. }
        | Decl::Sum { span, .. }
        | Decl::Generative { span, .. }
        | Decl::Resource { span, .. }
        | Decl::Function { span, .. }
        | Decl::Const { span, .. }
        | Decl::PackageExample { span, .. }
        | Decl::ComptimeBlock { span, .. }
        | Decl::Provide { span, .. } => *span,
    }
}

fn type_span(ty: &TypeExpr) -> Span {
    match ty {
        TypeExpr::Named(span, _)
        | TypeExpr::Apply(span, ..)
        | TypeExpr::AtLeast(span, ..)
        | TypeExpr::Range(span, ..) => *span,
    }
}

fn stmt_span(stmt: &Stmt) -> Span {
    match stmt {
        Stmt::Let { span, .. }
        | Stmt::Var { span, .. }
        | Stmt::Const { span, .. }
        | Stmt::Set { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::Assert { span, .. }
        | Stmt::Fail { span, .. }
        | Stmt::If { span, .. }
        | Stmt::ForList { span, .. }
        | Stmt::ForRange { span, .. }
        | Stmt::WhileIs { span, .. }
        | Stmt::Provide { span, .. }
        | Stmt::ExprStmt { span, .. } => *span,
    }
}

const STATEMENTS: &str =
    "lingua build supports only `log(\"...\")` and `return nothing` statements in main so far";

// `log("literal")`, or the reason it is outside the subset
fn log_call(span: Span, expr: &Expr) -> Result<TirStmt, Diagnostic> {
    let Expr::Call(_, callee, args, _) = expr else {
        return Err(not_yet(span, STATEMENTS));
    };
    let (
        Expr::Name(_, name),
        [
            Arg {
                name: None, value, ..
            },
        ],
    ) = (callee.as_ref(), args.as_slice())
    else {
        return Err(not_yet(span, STATEMENTS));
    };
    let Expr::Text(text_span, parts) = value else {
        return Err(not_yet(span, STATEMENTS));
    };
    if name != "log" {
        return Err(not_yet(span, STATEMENTS));
    }
    let text = parts
        .iter()
        .map(|part| match part {
            TextPart::Literal(literal) => Ok(literal.as_str()),
            TextPart::Hole(_) => Err(not_yet(
                *text_span,
                "lingua build does not support text holes yet; log a plain string literal",
            )),
        })
        .collect::<Result<String, _>>()?;
    let arg = TirExpr {
        span: *text_span,
        kind: TirKind::Text(text),
    };
    Ok(TirStmt::Expr(TirExpr {
        span: expr.span(),
        kind: TirKind::Log(Box::new(arg)),
    }))
}

// The body of `main`, which must end in `return nothing` (L012) with nothing after it (L018)
fn body(span: Span, stmts: &[Stmt], diagnostics: &mut Vec<Diagnostic>) -> Vec<TirStmt> {
    let mut out = Vec::new();
    let mut returned = false;
    for stmt in stmts {
        if returned {
            diagnostics.push(diagnostic(
                "L018",
                stmt_span(stmt),
                "unreachable statement after `return`",
                Some("delete it, or move it before `return nothing`"),
            ));
            break;
        }
        let lowered = match stmt {
            Stmt::Return {
                value: Expr::NothingLit(span),
                ..
            } => {
                returned = true;
                Ok(TirStmt::Return(TirExpr {
                    span: *span,
                    kind: TirKind::Nothing,
                }))
            }
            Stmt::ExprStmt { span, expr } => log_call(*span, expr),
            other => Err(not_yet(stmt_span(other), STATEMENTS)),
        };
        match lowered {
            Ok(stmt) => out.push(stmt),
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }
    if !returned {
        diagnostics.push(diagnostic(
            "L012",
            span,
            "`main` must end with `return nothing`",
            None,
        ));
    }
    out
}

fn main_fn(module: &Module, decl: &Decl, diagnostics: &mut Vec<Diagnostic>) -> Option<TirFn> {
    let Decl::Function {
        span,
        params,
        ret,
        effect,
        body: fn_body,
        ..
    } = decl
    else {
        return None;
    };
    match params.as_slice() {
        [] => {}
        [param] if matches!(&param.ty, TypeExpr::Named(_, ty) if ty == "Process") => {
            diagnostics.push(not_yet(
                param.span,
                "lingua build does not support `main(process: Process)` yet; use `fn main() -> Nothing`",
            ));
        }
        [param, ..] => diagnostics.push(diagnostic(
            "L075",
            param.span,
            "`main` must take no parameters or one `Process`",
            Some("use `fn main() -> Nothing` or `fn main(process: Process) -> Nothing`"),
        )),
    }
    if !matches!(ret, TypeExpr::Named(_, ty) if ty == "Nothing") {
        diagnostics.push(diagnostic(
            "L075",
            type_span(ret),
            "`main` must return `Nothing`",
            None,
        ));
    }
    match effect {
        Effect::Pure => {}
        Effect::ReadsClock => diagnostics.push(not_yet(
            *span,
            "lingua build supports only a `pure` main so far",
        )),
        _ => diagnostics.push(diagnostic(
            "L025",
            *span,
            "`main` must be `pure` or `reads clock`",
            None,
        )),
    }
    let FnBody::Statements(stmts) = fn_body else {
        diagnostics.push(not_yet(*span, STATEMENTS));
        return None;
    };
    Some(TirFn {
        name: format!("{}.main", module.key),
        body: body(*span, stmts, diagnostics),
    })
}

/// Checks that `module` is in the subset and returns its `main` as TIR.
///
/// # Errors
/// Every diagnostic found, sorted by position: `L074` for zero or several mains, `L075` for
/// a wrong signature, `L025` for a wrong effect, `L012`/`L018` for return paths, and
/// `error: lingua build ...` for legal Lingua that is not built yet.
pub fn check(module: &Module) -> Result<TirFn, Vec<Diagnostic>> {
    let mut diagnostics: Vec<Diagnostic> = module
        .imports
        .iter()
        .map(|import| {
            not_yet(
                import.span,
                "lingua build supports only one file with no imports so far",
            )
        })
        .collect();
    let mut mains = Vec::new();
    for decl in &module.decls {
        match decl {
            Decl::Function { name, .. } if name == "main" => {
                if !mains.is_empty() {
                    diagnostics.push(diagnostic(
                        "L074",
                        decl_span(decl),
                        "more than one `main` function",
                        Some("keep one `main` and rename or delete the others"),
                    ));
                }
                mains.push(main_fn(module, decl, &mut diagnostics));
            }
            _ => diagnostics.push(not_yet(
                decl_span(decl),
                "lingua build supports only a single `fn main` so far",
            )),
        }
    }
    if mains.is_empty() {
        diagnostics.push(diagnostic(
            "L074",
            module.span,
            "no `main` function to build",
            Some("add `fn main() -> Nothing`"),
        ));
    }
    if let (true, Some(main)) = (diagnostics.is_empty(), mains.into_iter().next().flatten()) {
        Ok(main)
    } else {
        diagnostics.sort_by_key(|d| (d.location.line, d.location.column, d.code));
        Err(diagnostics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_source;

    fn run(source: &str) -> Result<TirFn, Vec<String>> {
        let module = parse_source(source.as_bytes(), "hello").expect("test source parses");
        check(&module).map_err(|diagnostics| {
            diagnostics
                .iter()
                .map(|d| {
                    format!(
                        "{} {}:{} {}",
                        d.code, d.location.line, d.location.column, d.message
                    )
                })
                .collect()
        })
    }

    fn texts(main: &TirFn) -> Vec<Option<&str>> {
        main.body
            .iter()
            .map(|stmt| match stmt {
                TirStmt::Expr(TirExpr {
                    kind: TirKind::Log(arg),
                    ..
                }) => match &arg.kind {
                    TirKind::Text(text) => Some(text.as_str()),
                    _ => panic!("log of a non-text"),
                },
                TirStmt::Return(TirExpr {
                    kind: TirKind::Nothing,
                    ..
                }) => None,
                other => panic!("unexpected {other:?}"),
            })
            .collect()
    }

    #[test]
    fn accepts_logs_then_return_nothing() {
        let main = run("fn main() -> Nothing\n  pure\n\n  log(\"hello, world\")\n  log(\"a\\tb\")\n  return nothing\nend fn main\n").unwrap();
        assert_eq!(main.name, "hello.main");
        assert_eq!(texts(&main), [Some("hello, world"), Some("a\tb"), None]);
    }

    #[test]
    fn ignores_examples_on_main() {
        let main = run("fn main() -> Nothing\n  pure\n  example \"runs\"\n    -> nothing\n  end\n\n  log(\"ok\")\n  return nothing\nend fn main\n").unwrap();
        assert_eq!(texts(&main), [Some("ok"), None]);
    }

    #[test]
    fn rejects_other_statements_with_their_span() {
        let errors =
            run("fn main() -> Nothing\n  pure\n\n  let x = 1\n  return nothing\nend fn main\n")
                .unwrap_err();
        assert_eq!(errors, [format!("error 4:3 {STATEMENTS}")]);
    }

    #[test]
    fn rejects_calls_other_than_log() {
        let errors = run("fn main() -> Nothing\n  pure\n\n  print(\"x\")\n  log(1)\n  return nothing\nend fn main\n").unwrap_err();
        assert_eq!(
            errors,
            [
                format!("error 4:3 {STATEMENTS}"),
                format!("error 5:3 {STATEMENTS}")
            ]
        );
    }

    #[test]
    fn rejects_text_holes() {
        let errors = run(
            "fn main() -> Nothing\n  pure\n\n  log(\"n={n}\")\n  return nothing\nend fn main\n",
        )
        .unwrap_err();
        assert_eq!(
            errors,
            ["error 4:7 lingua build does not support text holes yet; log a plain string literal"]
        );
    }

    #[test]
    fn rejects_imports_and_other_declarations() {
        let errors = run("import Cents from \"money\"\n\ntype Id = Int\n\nfn main() -> Nothing\n  pure\n\n  return nothing\nend fn main\n").unwrap_err();
        assert_eq!(
            errors,
            [
                "error 1:1 lingua build supports only one file with no imports so far",
                "error 3:1 lingua build supports only a single `fn main` so far",
            ]
        );
    }

    #[test]
    fn zero_mains_is_l074() {
        let errors = run("type Id = Int\n").unwrap_err();
        assert_eq!(
            errors,
            [
                "L074 1:1 no `main` function to build",
                "error 1:1 lingua build supports only a single `fn main` so far",
            ]
        );
    }

    #[test]
    fn a_second_main_is_l074() {
        let main = "fn main() -> Nothing\n  pure\n\n  return nothing\nend fn main\n";
        let errors = run(&format!("{main}\n{main}")).unwrap_err();
        assert_eq!(errors, ["L074 7:1 more than one `main` function"]);
    }

    #[test]
    fn a_wrong_signature_is_l075() {
        let errors =
            run("fn main(n: Int) -> Int\n  pure\n\n  return nothing\nend fn main\n").unwrap_err();
        assert_eq!(
            errors,
            [
                "L075 1:9 `main` must take no parameters or one `Process`",
                "L075 1:20 `main` must return `Nothing`",
            ]
        );
    }

    #[test]
    fn main_with_process_and_reads_clock_are_not_built_yet() {
        let errors = run("fn main(process: Process) -> Nothing\n  reads clock\n\n  return nothing\nend fn main\n").unwrap_err();
        assert_eq!(
            errors,
            [
                "error 1:1 lingua build supports only a `pure` main so far",
                "error 1:9 lingua build does not support `main(process: Process)` yet; use `fn main() -> Nothing`",
            ]
        );
    }

    #[test]
    fn return_paths_are_l012_and_l018() {
        let errors =
            run("fn main() -> Nothing\n  pure\n\n  log(\"a\")\nend fn main\n").unwrap_err();
        assert_eq!(errors, ["L012 1:1 `main` must end with `return nothing`"]);
        let errors =
            run("fn main() -> Nothing\n  pure\n\n  return nothing\n  log(\"a\")\nend fn main\n")
                .unwrap_err();
        assert_eq!(errors, ["L018 5:3 unreachable statement after `return`"]);
    }
}
