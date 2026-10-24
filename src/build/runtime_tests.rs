//! Each `rt.*` routine run through the LIR evaluator.

use super::*;
use crate::build::lir_eval::{Machine, Stop};

fn with_machine<T>(test: impl FnOnce(&mut Machine) -> T) -> T {
    let functions = functions();
    let mut machine = Machine::new(&functions, &[]);
    test(&mut machine)
}

fn call(m: &mut Machine, name: &str, args: &[u64]) -> u64 {
    m.call(name, args).unwrap()
}

// Ok(v) as Ok(v), Err(variant singleton) as Err(name)
fn result(m: &Machine, value: u64) -> Result<u64, &'static str> {
    let payload = m.load(value + 8);
    match m.load(value) {
        0 => Ok(payload),
        1 if payload == m.symbol(OUT_OF_RANGE) => Err("OutOfRange"),
        1 if payload == m.symbol(NOT_AN_INTEGER) => Err("NotAnInteger"),
        other => panic!("not a Result tag: {other}"),
    }
}

#[test]
fn alloc_rounds_to_eight_and_grows_by_chunks() {
    with_machine(|m| {
        let a = call(m, "rt.alloc", &[1]);
        let b = call(m, "rt.alloc", &[8]);
        assert_eq!(a % 8, 0);
        assert_eq!(b - a, 8);
        // larger than a chunk: a new mapping rounded to 64 KiB
        let big = call(m, "rt.alloc", &[(5 << 20) + 1]);
        assert_eq!(
            m.load(m.symbol("rt.heap_end")) - big,
            (5 << 20) + (64 << 10)
        );
        let c = call(m, "rt.alloc", &[16]);
        assert_eq!(c, big + (5 << 20) + 8);
    });
}

#[test]
fn alloc_failure_is_l047_out_of_memory() {
    with_machine(|m| {
        m.mmap_fails = true;
        assert_eq!(m.call("rt.alloc", &[8]), Err(Stop::Exit(1)));
        assert_eq!(m.stderr, b"fault L047: out of memory\n");
    });
}

#[test]
fn log_writes_all_bytes_through_short_writes_and_eintr() {
    with_machine(|m| {
        m.short_writes = Some(3);
        let text = m.text("hello, world".as_bytes());
        assert_eq!(call(m, "rt.log", &[text]), 0);
        assert_eq!(m.stderr, b"hello, world\n");
    });
}

#[test]
fn clock_now_is_the_seconds_field() {
    with_machine(|m| assert_eq!(call(m, "rt.clock_now", &[]), 1_700_000_000));
}

#[test]
fn text_eq_compares_length_then_bytes() {
    with_machine(|m| {
        for (a, b, equal) in [
            ("", "", 1),
            ("abc", "abc", 1),
            ("abc", "abd", 0),
            ("abc", "ab", 0),
            ("héllo", "héllo", 1),
        ] {
            let (x, y) = (m.text(a.as_bytes()), m.text(b.as_bytes()));
            assert_eq!(call(m, "rt.text_eq", &[x, y]), equal, "{a:?} {b:?}");
        }
    });
}

#[test]
fn text_concat_joins_parts() {
    with_machine(|m| {
        let parts: Vec<u64> = ["a", "", "bc", "日本"]
            .iter()
            .map(|s| m.text(s.as_bytes()))
            .collect();
        let list = m.list(&parts);
        let out = call(m, "rt.text_concat", &[list]);
        assert_eq!(m.read_text(out), "abc日本".as_bytes());
        let empty = m.list(&[]);
        let out = call(m, "rt.text_concat", &[empty]);
        assert_eq!(m.read_text(out), b"");
    });
}

#[test]
fn int_to_text_is_decimal_with_a_minus_only_when_negative() {
    with_machine(|m| {
        for n in [
            0,
            7,
            -7,
            10,
            99,
            -100,
            1_234_567_890,
            i64::MAX,
            i64::MIN,
            i64::MIN + 1,
        ] {
            let out = call(m, "rt.int_to_text", &[n.cast_unsigned()]);
            assert_eq!(m.read_text(out), n.to_string().as_bytes(), "{n}");
        }
    });
}

#[test]
fn trim_strips_only_the_four_spaces() {
    with_machine(|m| {
        for (input, want) in [
            ("  a b \t\n\r", "a b"),
            ("", ""),
            (" \t ", ""),
            ("x", "x"),
            ("\u{a0}x\u{a0}", "\u{a0}x\u{a0}"),
            ("\x0bx", "\x0bx"),
        ] {
            let text = m.text(input.as_bytes());
            let out = call(m, "rt.trim", &[text]);
            assert_eq!(m.read_text(out), want.as_bytes(), "{input:?}");
        }
    });
}

#[test]
fn lower_maps_only_ascii_capitals() {
    with_machine(|m| {
        let text = m.text("AZaz@[ ÀÉ Hello".as_bytes());
        let out = call(m, "rt.lower", &[text]);
        assert_eq!(m.read_text(out), "azaz@[ ÀÉ hello".as_bytes());
    });
}

#[test]
fn split_matches_the_prelude_table() {
    with_machine(|m| {
        for (text, sep, want) in [
            ("percent:10", ":", &["percent", "10"][..]),
            ("bogof", ":", &["bogof"]),
            ("a::b", ":", &["a", "", "b"]),
            (":a:", ":", &["", "a", ""]),
            ("", ":", &[""]),
            ("a--b---c", "--", &["a", "b", "-c"]),
            ("ab", "abc", &["ab"]),
        ] {
            let (t, s) = (m.text(text.as_bytes()), m.text(sep.as_bytes()));
            let report = m.text(b"unused");
            let out = call(m, "rt.split", &[t, s, report]);
            let pieces: Vec<Vec<u8>> = m
                .read_list(out)
                .into_iter()
                .map(|p| m.read_text(p))
                .collect();
            let want: Vec<Vec<u8>> = want.iter().map(|s| s.as_bytes().to_vec()).collect();
            assert_eq!(pieces, want, "{text:?} {sep:?}");
        }
    });
}

#[test]
fn split_on_an_empty_separator_dies_with_the_given_report() {
    with_machine(|m| {
        let (t, s) = (m.text(b"abc"), m.text(b""));
        let report = m.text(b"fault L046: split on an empty separator\n");
        assert_eq!(m.call("rt.split", &[t, s, report]), Err(Stop::Exit(1)));
        assert_eq!(m.stderr, b"fault L046: split on an empty separator\n");
    });
}

#[test]
fn join_copies_both_runs() {
    with_machine(|m| {
        let (a, b, e) = (m.list(&[1, 2]), m.list(&[3]), m.list(&[]));
        let out = call(m, "rt.join", &[a, b]);
        assert_eq!(m.read_list(out), [1, 2, 3]);
        let out = call(m, "rt.join", &[e, e]);
        assert_eq!(m.read_list(out), [0u64; 0]);
    });
}

#[test]
fn list_index_is_ok_or_out_of_range() {
    with_machine(|m| {
        let list = m.list(&[10, 20]);
        for (i, want) in [
            (0, Ok(10)),
            (1, Ok(20)),
            (2, Err("OutOfRange")),
            (-1, Err("OutOfRange")),
        ] {
            let r = call(m, "rt.list_index", &[list, i64::cast_unsigned(i)]);
            assert_eq!(result(m, r), want, "{i}");
        }
    });
}

#[test]
fn int_in_matches_the_lingua1_table() {
    with_machine(|m| {
        let (lo, hi) = (1i64.cast_unsigned(), 100i64.cast_unsigned());
        for (input, want) in [
            ("10", Ok(10)),
            ("100", Ok(100)),
            ("0100", Ok(100)),
            ("0", Err("OutOfRange")),
            ("101", Err("OutOfRange")),
            ("-5", Err("OutOfRange")),
            ("", Err("NotAnInteger")),
            ("-", Err("NotAnInteger")),
            (" 5", Err("NotAnInteger")),
            ("5 ", Err("NotAnInteger")),
            ("+5", Err("NotAnInteger")),
            ("1x", Err("NotAnInteger")),
            ("99999999999999999999", Err("OutOfRange")),
            ("99999999999999999999x", Err("NotAnInteger")),
        ] {
            let text = m.text(input.as_bytes());
            let r = call(m, "rt.int_in_text", &[text, lo, hi]);
            assert_eq!(result(m, r), want.map(i64::cast_unsigned), "{input:?}");
        }
        let (min, max) = (i64::MIN.cast_unsigned(), i64::MAX.cast_unsigned());
        for (input, want) in [
            ("9223372036854775807", Ok(i64::MAX)),
            ("-9223372036854775808", Ok(i64::MIN)),
            ("9223372036854775808", Err("OutOfRange")),
            ("-9223372036854775809", Err("OutOfRange")),
            ("-0", Ok(0)),
        ] {
            let text = m.text(input.as_bytes());
            let r = call(m, "rt.int_in_text", &[text, min, max]);
            assert_eq!(result(m, r), want.map(i64::cast_unsigned), "{input:?}");
        }
        for (n, want) in [
            (0, Ok(0)),
            (-1, Err("OutOfRange")),
            (i64::MAX, Ok(i64::MAX)),
        ] {
            let r = call(m, "rt.int_in_int", &[n.cast_unsigned(), 0, max]);
            assert_eq!(result(m, r), want.map(i64::cast_unsigned), "{n}");
        }
    });
}

fn percent_reference(part: i64, of: i64) -> i64 {
    let product = i128::from(part) * i128::from(of);
    i64::try_from((product + 50) / 100).unwrap()
}

#[test]
fn percent_matches_a_wide_reference() {
    with_machine(|m| {
        let check = |m: &mut Machine, part: i64, of: i64| {
            let got = call(
                m,
                "rt.percent",
                &[part.cast_unsigned(), of.cast_unsigned(), 0],
            );
            assert_eq!(
                got.cast_signed(),
                percent_reference(part, of),
                "{part} {of}"
            );
        };
        assert_eq!(call(m, "rt.percent", &[10, 1200, 0]), 120);
        assert_eq!(call(m, "rt.percent", &[1, 150, 0]), 2);
        for part in 1..=100 {
            for of in [0, 1, 49, 50, 99, 100, 12345, i64::MAX - 1, i64::MAX] {
                check(m, part, of);
            }
        }
        // a deterministic sample (xorshift)
        let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
        for _ in 0..2000 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let part = i64::try_from(x % 100).unwrap() + 1;
            let of = (x >> 1).cast_signed();
            check(m, part, of);
        }
    });
}

#[test]
fn cstr_to_text_replaces_invalid_utf8_like_from_utf8_lossy() {
    let cases: [&[u8]; 12] = [
        b"plain",
        "héllo 世界 🎉".as_bytes(),
        b"\xff",
        b"a\x80b",
        b"\xc0\xaf",
        b"\xe0\x80\xaf",
        b"\xed\xa0\x80",
        b"\xf0\x9f\x8e",
        b"\xf4\x90\x80\x80",
        b"\xe2\x82",
        b"\xf5\x80",
        b"\xc3",
    ];
    with_machine(|m| {
        for case in cases {
            let at = m.text(&[case, &[0]].concat()) + 8;
            let out = call(m, "rt.cstr_to_text", &[at]);
            assert_eq!(
                m.read_text(out),
                String::from_utf8_lossy(case).as_bytes(),
                "{case:?}"
            );
        }
    });
}

#[test]
fn process_fills_every_field_from_argv() {
    with_machine(|m| {
        m.set_argv(&[b"./lingua-out", b"one", b"t\xffo"]);
        let p = call(m, "rt.process", &[]);
        let field = |i: u64| m.load(p + 8 * i);
        assert_eq!(m.read_text(field(0)), b"./lingua-out");
        let args: Vec<Vec<u8>> = m
            .read_list(field(1))
            .into_iter()
            .map(|a| m.read_text(a))
            .collect();
        assert_eq!(args, [b"one".to_vec(), "t\u{fffd}o".as_bytes().to_vec()]);
        assert_eq!(m.read_text(field(2)), b"linux");
        assert_eq!(m.read_text(field(3)), b"x86_64");
        assert_eq!(field(4), 42);
        assert_eq!(m.load(field(5)), 43);
        assert_eq!(m.read_text(m.load(field(5) + 8)), b"main");
        assert_eq!(field(6), 4, "popcount of the affinity mask");
    });
}

#[test]
fn process_with_no_argv_has_an_empty_program_and_no_args() {
    with_machine(|m| {
        m.set_argv(&[]);
        let p = call(m, "rt.process", &[]);
        assert_eq!(m.read_text(m.load(p)), b"");
        assert_eq!(m.read_list(m.load(p + 8)), [0u64; 0]);
    });
}

#[test]
fn every_runtime_function_is_nosplit() {
    assert!(functions().iter().all(|f| f.stack_site.is_none()));
}
