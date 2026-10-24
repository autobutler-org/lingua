//! Lowering: dumps of small functions, and every shared case run through the LIR evaluator.

use super::*;
use crate::build::lir::dump;
use crate::build::lir_eval::run_program;
use crate::tir::{TirKind, Type};

#[path = "../../tests/support/mod.rs"]
mod support;

use support::tir::*;

#[test]
fn line_subtotal_lowers_as_in_the_worked_example() {
    let mut p = Program::default();
    p.module("order", "");
    let line = p.param("line", Type::Nominal(0));
    let f = p.declare("line_subtotal", Type::Int);
    let field_of = |i| field(local(line, Type::Nominal(0)), i, Type::Int);
    p.define(
        f,
        14,
        vec![line],
        TirBody::Stmts(vec![ret(at(mul(field_of(1), field_of(2)), 21, 10, 1))]),
    );
    let lowered = lower(&p.finish(f)).unwrap();
    assert_eq!(
        dump(&lowered.functions, &lowered.sites),
        "fn order.line_subtotal params=1 vregs=4 stack=site.0
b0:
  v1 = load v0, 8
  v2 = load v0, 16
  v3 = mul.chk v1, v2 @site.1
  ret v3

site.0 fault L077 order.lf:14:1
site.1 fault L040 order.lf:21:10
"
    );
    let report = |name: &str| {
        lowered
            .rodata
            .iter()
            .find(|block| block.name == name)
            .map(|block| block.bytes.clone())
    };
    assert_eq!(
        report("site.1"),
        Some(
            text_block(
                "site.1",
                "fault L040: arithmetic overflow\n  --> order.lf:21:10\n"
            )
            .bytes
        )
    );
}

#[test]
fn entry_builds_process_only_when_main_takes_it() {
    let lowered = lower(&main_program("", vec![])).unwrap();
    assert_eq!(
        dump(&[lowered.entry], &[]),
        "fn rt.entry params=0 vregs=1\nb0:\n  call m.main()\n  v0 = const 0\n  call rt.sys_exit(v0)\n  ret\n"
    );
}

#[test]
fn every_case_runs_in_the_lir_evaluator() {
    let mut failures = Vec::new();
    for case in support::cases::all() {
        let lowered = lower(&case.program).unwrap();
        let argv: Vec<&[u8]> = std::iter::once(&b"./lingua-out"[..])
            .chain(case.args.iter().map(Vec::as_slice))
            .collect();
        let (exit, stderr) = run_program(&lowered, &argv);
        if (exit, stderr.as_str()) != (case.exit, case.stderr.as_str()) {
            failures.push(format!(
                "{}: expected exit {} and {:?}\n  got exit {exit} and {stderr:?}",
                case.name, case.exit, case.stderr
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn comptime_and_resource_nodes_are_internal_errors() {
    let read = expr(Type::Nothing, TirKind::FsRead(Box::new(text("x"))));
    let err = lower(&main_program("", vec![TirStmt::Expr(read)])).unwrap_err();
    assert!(err.starts_with("internal error: "), "{err}");
}

#[test]
fn only_money_percent_may_be_primitive() {
    let mut p = Program::default();
    p.module("m", "");
    let main = p.declare("main", Type::Nothing);
    p.define(main, 1, vec![], TirBody::Primitive);
    assert_eq!(
        lower(&p.finish(main)).unwrap_err(),
        "internal error: `m.main` is not an allowed primitive"
    );
}
