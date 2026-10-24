//! Small TIR programs and what each prints, shared by the LIR evaluator tests
//! (`src/build/lower_tests.rs`) and the native end-to-end tests (`tests/build_tir.rs`).
#![allow(dead_code)]

use super::tir::*;
use lingua::tir::{CmpOp, TirBody, TirExpr, TirPat, TirProgram, TirStmt, Type, prelude};

pub struct Case {
    pub name: &'static str,
    pub program: TirProgram,
    /// argv after the program name.
    pub args: Vec<Vec<u8>>,
    pub stderr: String,
    pub exit: i64,
}

fn case(name: &'static str, program: TirProgram, stderr: &str, exit: i64) -> Case {
    Case {
        name,
        program,
        args: Vec::new(),
        stderr: stderr.to_owned(),
        exit,
    }
}

fn if_(cond: TirExpr, then: Vec<TirStmt>, otherwise: Vec<TirStmt>) -> TirStmt {
    TirStmt::If(cond, then, otherwise)
}

// A fault on an empty source line: the span block with `len` carets
fn fault(code: &str, summary: &str, line: usize, column: usize, len: usize) -> String {
    format!(
        "fault {code}: {summary}\n  --> m.lf:{line}:{column}\n   |\n{line:>2} | \n   | {}{}\n",
        " ".repeat(column - 1),
        "^".repeat(len)
    )
}

// main whose body faults at line 4, column 10, through `e`
fn faulting(name: &'static str, e: TirExpr, code: &str, summary: &str) -> Case {
    let program = main_program("", vec![log_int(at(e, 4, 10, 1)), log(text("unreachable"))]);
    case(name, program, &fault(code, summary, 4, 10, 1), 1)
}

fn arithmetic() -> Case {
    let lines = [
        add(int(2), int(3)),
        sub(int(2), int(5)),
        mul(int(-4), int(6)),
        div(int(7), int(-2)),
        rem(int(-7), int(2)),
        div(int(i64::MIN), int(2)),
        rem(int(i64::MIN), int(10)),
        neg(int(i64::MAX)),
        add(int(i64::MAX - 1), int(1)),
        mul(int(3_037_000_499), int(3_037_000_499)),
    ];
    let program = main_program("", lines.into_iter().map(log_int).collect());
    case(
        "arithmetic",
        program,
        "5\n-3\n-24\n-3\n-1\n-4611686018427387904\n-8\n-9223372036854775807\n9223372036854775807\n9223372030926249001\n",
        0,
    )
}

fn comparisons() -> Case {
    let check =
        |op, a, b, label: &str| if_(cmp(op, int(a), int(b)), vec![log(text(label))], vec![]);
    let program = main_program(
        "",
        vec![
            check(CmpOp::Lt, 1, 2, "lt"),
            check(CmpOp::Lt, 2, 1, "not lt"),
            check(CmpOp::Le, 2, 2, "le"),
            check(CmpOp::Gt, -1, i64::MIN, "gt"),
            check(CmpOp::Ge, 0, 1, "not ge"),
            check(CmpOp::Eq, 5, 5, "eq"),
            check(CmpOp::Ne, 5, 5, "not ne"),
            if_(
                cmp(CmpOp::Eq, boolean(true), boolean(false)),
                vec![log(text("bad"))],
                vec![log(text("else"))],
            ),
        ],
    );
    case("comparisons", program, "lt\nle\ngt\neq\nelse\n", 0)
}

const ASSERT_SOURCE: &str = "fn main() -> Nothing
  pure

  assert 1 == 1, \"fine\"
  assert 1 == 2, \"one is not two\"
  return nothing
end fn main
";

fn assert_fails() -> Case {
    let assert = |a, b, message: &str, line| TirStmt::Assert {
        cond: cmp(CmpOp::Eq, int(a), int(b)),
        message: message.to_owned(),
        span: span(line, 3, 6),
    };
    let program = main_program(
        ASSERT_SOURCE,
        vec![
            assert(1, 1, "fine", 4),
            log(text("passed")),
            assert(1, 2, "one is not two", 5),
            log(text("unreachable")),
        ],
    );
    case(
        "assert_fails",
        program,
        "passed
exception: one is not two
  --> m.lf:5:3
   |
 5 |   assert 1 == 2, \"one is not two\"
   |   ^^^^^^
",
        1,
    )
}

fn fail_raises() -> Case {
    let source = "fn main() -> Nothing\n  pure\n\n  fail \"schema unavailable\"\nend fn main\n";
    let program = main_program(
        source,
        vec![TirStmt::Fail {
            message: "schema unavailable".to_owned(),
            span: span(4, 3, 4),
        }],
    );
    case(
        "fail_raises",
        program,
        "exception: schema unavailable\n  --> m.lf:4:3\n   |\n 4 |   fail \"schema unavailable\"\n   |   ^^^^\n",
        1,
    )
}

fn or_fail_raises() -> Case {
    let source = "x\nx\nx\n  let n = int_at_least(\"-3\", 0) or fail \"negative\"\n";
    let mut p = Program::default();
    p.module("m", source);
    let main = p.declare("main", Type::Nothing);
    let n = p.local("n", Type::Int);
    let ok = p.local("ok", Type::Int);
    let parsed = |s: &str| {
        expr(
            bounds_result(Type::Int),
            lingua::tir::TirKind::IntAtLeast(Box::new(text(s)), 0),
        )
    };
    let good = or_fail(&mut p, parsed("7"), Type::Int, "negative", span(4, 33, 7));
    let bad = or_fail(&mut p, parsed("-3"), Type::Int, "negative", span(4, 33, 7));
    p.define(
        main,
        1,
        vec![],
        stmts(vec![
            let_(ok, good),
            log_int(local(ok, Type::Int)),
            let_(n, bad),
            log_int(local(n, Type::Int)),
        ]),
    );
    case(
        "or_fail_raises",
        p.finish(main),
        "7\nexception: negative\n  --> m.lf:4:33\n   |\n 4 |   let n = int_at_least(\"-3\", 0) or fail \"negative\"\n   |                                 ^^^^^^^\n",
        1,
    )
}

// type Discount: NoDiscount, Percent(Int), CentsOff(Int), matched in `off`
fn sum_match() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let variants = p.sum(
        "Discount",
        &[
            ("NoDiscount", None),
            ("Percent", Some(Type::Int)),
            ("CentsOff", Some(Type::Int)),
        ],
    );
    let (none, percent, cents) = (variants[0], variants[1], variants[2]);
    let discount = Type::Nominal(none - 1);
    let off = p.declare("off", Type::Int);
    let (d, total) = (p.param("d", discount.clone()), p.param("total", Type::Int));
    let (pct, c) = (p.local("p", Type::Int), p.local("c", Type::Int));
    let body = match_(
        local(d, discount.clone()),
        vec![
            arm(pvariant(none, None), int(0)),
            arm(
                pvariant(percent, Some(TirPat::Bind(pct))),
                div(
                    mul(local(total, Type::Int), local(pct, Type::Int)),
                    int(100),
                ),
            ),
            arm(pvariant(cents, Some(TirPat::Bind(c))), local(c, Type::Int)),
        ],
        Type::Int,
    );
    p.define(off, 1, vec![d, total], TirBody::Stmts(vec![ret(body)]));
    let main = p.declare("main", Type::Nothing);
    let apply = |v: TirExpr| log_int(call(off, vec![v, int(1000)], Type::Int));
    p.define(
        main,
        2,
        vec![],
        stmts(vec![
            apply(variant(none, None, discount.clone())),
            apply(variant(percent, Some(int(10)), discount.clone())),
            apply(variant(cents, Some(int(250)), discount)),
        ]),
    );
    case("sum_match", p.finish(main), "0\n100\n250\n", 0)
}

// parse(t) = Ok(int_in(t, 1 through 100) or return * 2), matched on every outcome
fn result_or_return() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let result = bounds_result(Type::Int);
    let bounds = Type::Nominal(prelude::BOUNDS_ERROR);
    let parse = p.declare("parse", result.clone());
    let t = p.param("t", Type::Text);
    let n = p.local("n", Type::Int);
    let checked = expr(
        result.clone(),
        lingua::tir::TirKind::IntIn(Box::new(local(t, Type::Text)), 1, 100),
    );
    let unwrapped = or_return(&mut p, checked, Type::Int, bounds.clone(), result.clone());
    p.define(
        parse,
        1,
        vec![t],
        TirBody::Stmts(vec![
            let_(n, unwrapped),
            ret(variant(
                prelude::OK,
                Some(mul(local(n, Type::Int), int(2))),
                result.clone(),
            )),
        ]),
    );
    let main = p.declare("main", Type::Nothing);
    let mut body = Vec::new();
    for input in ["10", "0", "x", "100"] {
        let v = p.local("v", Type::Int);
        body.push(TirStmt::Match(
            call(parse, vec![text(input)], result.clone()),
            vec![
                stmt_arm(
                    pvariant(prelude::OK, Some(TirPat::Bind(v))),
                    vec![log_int(local(v, Type::Int))],
                ),
                stmt_arm(
                    pvariant(prelude::ERR, Some(pvariant(prelude::OUT_OF_RANGE, None))),
                    vec![log(text("out of range"))],
                ),
                stmt_arm(
                    pvariant(prelude::ERR, Some(pvariant(prelude::NOT_AN_INTEGER, None))),
                    vec![log(text("not an integer"))],
                ),
            ],
        ));
    }
    p.define(main, 2, vec![], stmts(body));
    case(
        "result_or_return",
        p.finish(main),
        "20\nout of range\nnot an integer\n200\n",
        0,
    )
}

// money.lf's next_pow2, with `while state is Go(p)`
fn next_pow2(p: &mut Program) -> lingua::tir::SymbolId {
    let variants = p.sum(
        "PowStep",
        &[("Go", Some(Type::Int)), ("Stop", Some(Type::Int))],
    );
    let (go, stop) = (variants[0], variants[1]);
    let pow_step = Type::Nominal(go - 1);
    let f = p.declare("next_pow2", Type::Int);
    let n = p.param("n", Type::Int);
    let state = p.local("state", pow_step.clone());
    let (pv, q) = (p.local("p", Type::Int), p.local("p", Type::Int));
    let int_local = |id| local(id, Type::Int);
    let body = vec![
        let_(state, variant(go, Some(int(1)), pow_step.clone())),
        TirStmt::WhileIs {
            scrutinee: state,
            variant: go,
            payload: Some(pv),
            body: vec![if_(
                cmp(CmpOp::Ge, int_local(pv), int_local(n)),
                vec![set(
                    state,
                    variant(stop, Some(int_local(pv)), pow_step.clone()),
                )],
                vec![set(
                    state,
                    variant(
                        go,
                        Some(add(int_local(pv), int_local(pv))),
                        pow_step.clone(),
                    ),
                )],
            )],
        },
        ret(match_(
            local(state, pow_step),
            vec![
                arm(
                    pvariant(stop, Some(TirPat::Bind(q))),
                    call(prelude::MAX, vec![int(1), int_local(q)], Type::Int),
                ),
                arm(pvariant(go, Some(TirPat::Wild)), int(1)),
            ],
            Type::Int,
        )),
    ];
    p.define(f, 1, vec![n], TirBody::Stmts(body));
    f
}

fn loops() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let pow2 = next_pow2(&mut p);
    let main = p.declare("main", Type::Nothing);
    let (sum, x, i, j) = (
        p.local("sum", Type::Int),
        p.local("x", Type::Int),
        p.local("i", Type::Int),
        p.local("j", Type::Int),
    );
    let (high, step) = (p.local("high", Type::Int), p.local("step", Type::Int));
    let ints = |ns: &[i64]| list(Type::Int, ns.iter().map(|&n| int(n)).collect());
    let body = vec![
        let_(sum, int(0)),
        TirStmt::ForList(
            x,
            ints(&[1, 2, 3, 4]),
            vec![set(sum, add(local(sum, Type::Int), local(x, Type::Int)))],
        ),
        log_int(local(sum, Type::Int)),
        TirStmt::ForList(x, ints(&[]), vec![log(text("never"))]),
        TirStmt::ForRange(i, int(2), int(5), vec![log_int(local(i, Type::Int))]),
        TirStmt::ForRange(j, int(5), int(2), vec![log(text("never"))]),
        log_int(call(pow2, vec![int(1)], Type::Int)),
        log_int(call(pow2, vec![int(3)], Type::Int)),
        log_int(call(pow2, vec![int(100)], Type::Int)),
        // Bounds are fixed on entry: setting the `var` `high` does not extend the loop
        let_(high, int(2)),
        TirStmt::ForRange(
            step,
            int(0),
            local(high, Type::Int),
            vec![set(high, int(10)), log_int(local(step, Type::Int))],
        ),
    ];
    p.define(main, 2, vec![], stmts(body));
    case("loops", p.finish(main), "10\n2\n3\n4\n1\n4\n128\n0\n1\n", 0)
}

fn lists() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let main = p.declare("main", Type::Nothing);
    let list_ty = Type::List(Box::new(Type::Int));
    let (xs, ys) = (
        p.local("xs", list_ty.clone()),
        p.local("ys", list_ty.clone()),
    );
    let xs_e = || local(xs, Type::List(Box::new(Type::Int)));
    let mut body = vec![
        let_(xs, list(Type::Int, vec![int(10), int(20), int(30)])),
        log_int(index(xs_e(), int(1), false, Type::Int)),
        log_int(call(prelude::LENGTH, vec![xs_e()], Type::Int)),
    ];
    for i in [2, 3, -1] {
        let v = p.local("v", Type::Int);
        body.push(TirStmt::Match(
            index(xs_e(), int(i), true, bounds_result(Type::Int)),
            vec![
                stmt_arm(
                    pvariant(prelude::OK, Some(TirPat::Bind(v))),
                    vec![log_int(local(v, Type::Int))],
                ),
                stmt_arm(
                    pvariant(prelude::ERR, Some(TirPat::Wild)),
                    vec![log(text("out of range"))],
                ),
            ],
        ));
    }
    body.extend([
        let_(
            ys,
            call(
                prelude::JOIN,
                vec![xs_e(), list(Type::Int, vec![int(40)])],
                list_ty.clone(),
            ),
        ),
        log_int(call(
            prelude::LENGTH,
            vec![local(ys, list_ty.clone())],
            Type::Int,
        )),
        log_int(index(local(ys, list_ty), int(3), false, Type::Int)),
    ]);
    p.define(main, 1, vec![], stmts(body));
    case(
        "lists",
        p.finish(main),
        "20\n3\n30\nout of range\nout of range\n4\n40\n",
        0,
    )
}

fn text_holes() -> Case {
    let program = main_program(
        "",
        vec![log(concat(vec![
            text("n="),
            int(-5),
            text(" min="),
            int(i64::MIN),
            text(" max="),
            int(i64::MAX),
            text(" zero="),
            int(0),
            text(" t="),
            text("ok"),
        ]))],
    );
    case(
        "text_holes",
        program,
        "n=-5 min=-9223372036854775808 max=9223372036854775807 zero=0 t=ok\n",
        0,
    )
}

fn text_ops() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let customer = p.newtype("CustomerId", Type::Text);
    let main = p.declare("main", Type::Nothing);
    let part = p.local("part", Type::Text);
    let n = p.local("n", Type::Text);
    let texts = Type::List(Box::new(Type::Text));
    let split = |t: &str, s: &str| {
        call(
            prelude::SPLIT,
            vec![text(t), text(s)],
            Type::List(Box::new(Type::Text)),
        )
    };
    let trim = |e| call(prelude::TRIM, vec![e], Type::Text);
    let lower = |e| call(prelude::LOWER, vec![e], Type::Text);
    let classify = |input: &str| {
        TirStmt::Match(
            split(input, ":"),
            vec![
                stmt_arm(
                    TirPat::List(vec![TirPat::LitText("percent".to_owned()), TirPat::Bind(n)]),
                    vec![log(concat(vec![text("percent "), local(n, Type::Text)]))],
                ),
                stmt_arm(
                    TirPat::List(vec![TirPat::LitText("none".to_owned())]),
                    vec![log(text("none"))],
                ),
                stmt_arm(TirPat::Wild, vec![log(text("other"))]),
            ],
        )
    };
    let id = |s: &str| {
        expr(
            Type::Nominal(customer),
            lingua::tir::TirKind::Text(s.to_owned()),
        )
    };
    let body = vec![
        log(concat(vec![
            text("["),
            trim(text(" \t hi there \r\n")),
            text("]"),
        ])),
        log(lower(text("HeLLo, ÀB"))),
        TirStmt::ForList(
            part,
            split("a::b", ":"),
            vec![log(concat(vec![
                text("["),
                local(part, Type::Text),
                text("]"),
            ]))],
        ),
        log_int(call(prelude::LENGTH, vec![split("", ":")], Type::Int)),
        let_(p.local("unused", texts.clone()), split("x", "x")),
        if_(
            cmp(CmpOp::Eq, trim(text(" x ")), text("x")),
            vec![log(text("eq"))],
            vec![log(text("not eq"))],
        ),
        if_(
            cmp(CmpOp::Ne, text("a"), text("ab")),
            vec![log(text("ne"))],
            vec![],
        ),
        if_(
            cmp(CmpOp::Eq, id("c1"), id("c1")),
            vec![log(text("id eq"))],
            vec![],
        ),
        TirStmt::Match(
            lower(text("NONE")),
            vec![
                stmt_arm(TirPat::LitText("none".to_owned()), vec![log(text("lit"))]),
                stmt_arm(TirPat::Wild, vec![log(text("wild"))]),
            ],
        ),
        classify("percent:10"),
        classify("none"),
        classify("none:"),
        classify("percent:10:extra"),
        log_int(call(
            prelude::MAX,
            vec![int(0), sub(int(200), int(500))],
            Type::Int,
        )),
        log_int(call(prelude::MAX, vec![int(7), int(3)], Type::Int)),
    ];
    p.define(main, 1, vec![], stmts(body));
    case(
        "text_ops",
        p.finish(main),
        "[hi there]\nhello, Àb\n[a]\n[]\n[b]\n1\neq\nne\nid eq\nlit\npercent 10\nnone\nother\nother\n0\n7\n",
        0,
    )
}

fn int_in() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let main = p.declare("main", Type::Nothing);
    let mut body = Vec::new();
    let sources: [(TirExpr, Option<i64>); 6] = [
        (text("42"), Some(100)),
        (int(150), Some(100)),
        (int(1), Some(100)),
        (text("-1"), None),
        (text("9223372036854775807"), None),
        (text("4 2"), None),
    ];
    for (source, hi) in sources {
        let v = p.local("v", Type::Int);
        let kind = match hi {
            Some(hi) => lingua::tir::TirKind::IntIn(Box::new(source), 1, hi),
            None => lingua::tir::TirKind::IntAtLeast(Box::new(source), 0),
        };
        body.push(TirStmt::Match(
            expr(bounds_result(Type::Int), kind),
            vec![
                stmt_arm(
                    pvariant(prelude::OK, Some(TirPat::Bind(v))),
                    vec![log_int(local(v, Type::Int))],
                ),
                stmt_arm(
                    pvariant(prelude::ERR, Some(pvariant(prelude::OUT_OF_RANGE, None))),
                    vec![log(text("OutOfRange"))],
                ),
                stmt_arm(
                    pvariant(prelude::ERR, Some(pvariant(prelude::NOT_AN_INTEGER, None))),
                    vec![log(text("NotAnInteger"))],
                ),
            ],
        ));
    }
    p.define(main, 1, vec![], stmts(body));
    case(
        "int_in",
        p.finish(main),
        "42\nOutOfRange\n1\nOutOfRange\n9223372036854775807\nNotAnInteger\n",
        0,
    )
}

// money.percent (primitive) called from order.discount, called from m.main
fn modules_and_percent() -> Case {
    let mut p = Program::default();
    p.module("money", "");
    let rounding = p.sum("Rounding", &[("half_up", None)]);
    let half_up = rounding[0];
    let rounding_ty = Type::Nominal(half_up - 1);
    let percent = p.declare("percent", Type::Int);
    let params = vec![
        p.param("part", Type::Int),
        p.param("of", Type::Int),
        p.param("round", rounding_ty.clone()),
    ];
    p.define(percent, 7, params, TirBody::Primitive);
    p.module("order", "");
    let discount = p.declare("discount", Type::Int);
    let (part, of) = (p.param("part", Type::Int), p.param("of", Type::Int));
    let half = variant(half_up, None, rounding_ty);
    p.define(
        discount,
        3,
        vec![part, of],
        TirBody::Stmts(vec![ret(call(
            percent,
            vec![local(part, Type::Int), local(of, Type::Int), half],
            Type::Int,
        ))]),
    );
    p.module("m", "");
    let main = p.declare("main", Type::Nothing);
    let cases = [
        (10, 1200),
        (1, 150),
        (100, i64::MAX),
        (50, i64::MAX),
        (1, i64::MAX),
        (99, 1),
    ];
    let body = cases
        .iter()
        .map(|&(part, of)| log_int(call(discount, vec![int(part), int(of)], Type::Int)))
        .collect();
    p.define(main, 1, vec![], stmts(body));
    let expected: String = cases
        .iter()
        .map(|&(part, of)| ((i128::from(part) * i128::from(of) + 50) / 100).to_string() + "\n")
        .collect();
    case("modules_and_percent", p.finish(main), &expected, 0)
}

fn process() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let main = p.declare("main", Type::Nothing);
    let process_ty = Type::Nominal(prelude::PROCESS);
    let process = p.param("process", process_ty.clone());
    let arg = p.local("arg", Type::Text);
    let texts = Type::List(Box::new(Type::Text));
    let f = |i: u32, ty: Type| field(local(process, process_ty.clone()), i, ty);
    let thread = || f(5, Type::Nominal(prelude::THREAD));
    let at_least_one =
        |e, label: &str| if_(cmp(CmpOp::Ge, e, int(1)), vec![log(text(label))], vec![]);
    let body = vec![
        log(concat(vec![
            call(prelude::LENGTH, vec![f(1, texts.clone())], Type::Int),
            text(" args"),
        ])),
        TirStmt::ForList(
            arg,
            f(1, texts),
            vec![log(concat(vec![
                text("["),
                local(arg, Type::Text),
                text("]"),
            ]))],
        ),
        log(f(2, Type::Text)),
        log(f(3, Type::Text)),
        at_least_one(f(4, Type::Int), "pid ok"),
        at_least_one(field(thread(), 0, Type::Int), "tid ok"),
        log(field(thread(), 1, Type::Text)),
        at_least_one(f(6, Type::Int), "parallelism ok"),
    ];
    p.define(main, 1, vec![process], stmts(body));
    Case {
        name: "process",
        program: p.finish(main),
        args: vec![b"one".to_vec(), b"two words".to_vec(), b"t\xffo".to_vec()],
        stderr: "3 args\n[one]\n[two words]\n[t\u{fffd}o]\nlinux\nx86_64\npid ok\ntid ok\nmain\nparallelism ok\n".to_owned(),
        exit: 0,
    }
}

fn deep_recursion() -> Case {
    let mut p = Program::default();
    p.module("m", "");
    let down = p.declare("down", Type::Int);
    let n = p.param("n", Type::Int);
    p.define(
        down,
        3,
        vec![n],
        TirBody::Stmts(vec![ret(call(
            down,
            vec![add(local(n, Type::Int), int(1))],
            Type::Int,
        ))]),
    );
    let main = p.declare("main", Type::Nothing);
    p.define(
        main,
        8,
        vec![],
        stmts(vec![log_int(call(down, vec![int(0)], Type::Int))]),
    );
    case(
        "deep_recursion",
        p.finish(main),
        &fault("L077", "stack overflow", 3, 1, 2),
        1,
    )
}

fn clock() -> Case {
    let now = expr(Type::Int, lingua::tir::TirKind::ClockNow);
    let program = main_program(
        "",
        vec![if_(
            cmp(CmpOp::Gt, now, int(1_600_000_000)),
            vec![log(text("clock ok"))],
            vec![log(text("clock wrong"))],
        )],
    );
    case("clock", program, "clock ok\n", 0)
}

fn split_empty_separator() -> Case {
    let call = at(
        call(
            prelude::SPLIT,
            vec![text("abc"), text("")],
            Type::List(Box::new(Type::Text)),
        ),
        4,
        10,
        5,
    );
    let program = main_program(
        "",
        vec![log_int(call_length(call)), log(text("unreachable"))],
    );
    case(
        "split_empty_separator",
        program,
        &fault("L046", "split on an empty separator", 4, 10, 5),
        1,
    )
}

fn call_length(e: TirExpr) -> TirExpr {
    call(prelude::LENGTH, vec![e], Type::Int)
}

fn logs() -> Case {
    let program = main_program("", vec![log(text("")), log(text("héllo, 世界"))]);
    case("logs", program, "\nhéllo, 世界\n", 0)
}

/// Every case, in a fixed order.
#[must_use]
pub fn all() -> Vec<Case> {
    let overflow = "arithmetic overflow";
    let zero = "division by zero";
    vec![
        arithmetic(),
        comparisons(),
        faulting("add_overflow", add(int(i64::MAX), int(1)), "L040", overflow),
        faulting("sub_overflow", sub(int(i64::MIN), int(1)), "L040", overflow),
        faulting("mul_overflow", mul(int(i64::MAX), int(2)), "L040", overflow),
        faulting("neg_overflow", neg(int(i64::MIN)), "L040", overflow),
        faulting("div_zero", div(int(1), int(0)), "L076", zero),
        faulting("rem_zero", rem(int(1), int(0)), "L076", zero),
        faulting(
            "div_overflow",
            div(int(i64::MIN), int(-1)),
            "L040",
            overflow,
        ),
        faulting(
            "rem_overflow",
            rem(int(i64::MIN), int(-1)),
            "L040",
            overflow,
        ),
        assert_fails(),
        fail_raises(),
        or_fail_raises(),
        sum_match(),
        result_or_return(),
        loops(),
        lists(),
        text_holes(),
        text_ops(),
        int_in(),
        modules_and_percent(),
        process(),
        deep_recursion(),
        clock(),
        split_empty_separator(),
        logs(),
    ]
}
