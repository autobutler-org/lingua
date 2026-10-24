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
                "fault L040: arithmetic overflow\n  --> order.lf:21:10\n   |\n21 | \n   |          ^\n"
            )
            .bytes
        )
    );
}

#[test]
fn a_returned_self_call_jumps_to_the_entry_and_any_other_call_stays_a_call() {
    let mut p = Program::default();
    p.module("m", "");
    let swap = p.declare("swap", Type::Int);
    let (a, b) = (p.param("a", Type::Int), p.param("b", Type::Int));
    p.define(
        swap,
        1,
        vec![a, b],
        TirBody::Stmts(vec![ret(call(
            swap,
            vec![local(b, Type::Int), local(a, Type::Int)],
            Type::Int,
        ))]),
    );
    let count = p.declare("count", Type::Int);
    let n = p.param("n", Type::Int);
    p.define(
        count,
        5,
        vec![n],
        TirBody::Stmts(vec![ret(add(
            int(1),
            call(count, vec![local(n, Type::Int)], Type::Int),
        ))]),
    );
    let lowered = lower(&p.finish(count)).unwrap();
    assert_eq!(
        dump(&lowered.functions, &lowered.sites),
        "fn m.swap params=2 vregs=4 stack=site.0
b0:
  v2 = move v1
  v3 = move v0
  v0 = move v2
  v1 = move v3
  jump b0

fn m.count params=1 vregs=4 stack=site.1
b0:
  v1 = const 1
  v2 = call m.count(v0)
  v3 = add.chk v1, v2 @site.2
  ret v3

site.0 fault L077 m.lf:1:1
site.1 fault L077 m.lf:5:1
site.2 fault L040 m.lf:1:1
"
    );
}

#[test]
fn a_returned_call_of_another_function_tail_calls_and_its_group_shares_one_width() {
    let mut p = Program::default();
    p.module("m", "");
    let (wide, narrow) = (p.declare("wide", Type::Int), p.declare("narrow", Type::Int));
    let n = p.param("n", Type::Int);
    p.define(
        narrow,
        1,
        vec![n],
        TirBody::Stmts(vec![ret(call(
            wide,
            vec![local(n, Type::Int), local(n, Type::Int)],
            Type::Int,
        ))]),
    );
    let (first, second) = (p.param("a", Type::Int), p.param("b", Type::Int));
    p.define(
        wide,
        5,
        vec![first, second],
        TirBody::Stmts(vec![ret(call(
            narrow,
            vec![local(second, Type::Int)],
            Type::Int,
        ))]),
    );
    let outer = p.declare("outer", Type::Int);
    let value = p.param("x", Type::Int);
    p.define(
        outer,
        9,
        vec![value],
        TirBody::Stmts(vec![ret(add(
            int(1),
            call(narrow, vec![local(value, Type::Int)], Type::Int),
        ))]),
    );
    let lowered = lower(&p.finish(outer)).unwrap();
    assert_eq!(
        dump(&lowered.functions, &lowered.sites),
        "fn m.narrow params=2 vregs=4 stack=site.0
b0:
  v2 = move v0
  v3 = move v0
  v0 = move v2
  v1 = move v3
  tailcall m.wide

fn m.wide params=2 vregs=3 stack=site.1
b0:
  v2 = move v1
  v0 = move v2
  tailcall m.narrow

fn m.outer params=1 vregs=5 stack=site.2
b0:
  v1 = const 1
  v2 = const 0
  v3 = call m.narrow(v0, v2)
  v4 = add.chk v1, v3 @site.3
  ret v4

site.0 fault L077 m.lf:1:1
site.1 fault L077 m.lf:5:1
site.2 fault L077 m.lf:9:1
site.3 fault L040 m.lf:1:1
"
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
