//! The example runner through `check`, and interpreter parity with the native binary.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::check::{Diagnostic, Error, Options, check, link};

static NEXT: AtomicUsize = AtomicUsize::new(0);

// A fresh source root holding `files`
fn package(files: &[(&str, &str)]) -> PathBuf {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("lingua-eval-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    dir
}

fn diagnostics_of(files: &[(&str, &str)]) -> Vec<Diagnostic> {
    match check(&[package(files)], &Options::default()) {
        Ok(_) => Vec::new(),
        Err(Error::Diagnostics(d)) => d,
        Err(Error::Usage(u)) => panic!("usage error: {u}"),
    }
}

fn diagnostics(source: &str) -> Vec<Diagnostic> {
    diagnostics_of(&[("m.lf", source)])
}

fn codes(source: &str) -> Vec<&'static str> {
    diagnostics(source).iter().map(|d| d.code).collect()
}

fn summaries(source: &str) -> Vec<String> {
    diagnostics(source)
        .iter()
        .map(|d| format!("{}: {}", d.code, d.summary))
        .collect()
}

const CUPS: &str = "fn cups(n: Int) -> Int
  pure
  example \"two cups\"
    n: 2
    -> 1000
  end

  return n * 250
end fn cups
";

#[test]
fn a_value_mismatch_is_l043_at_the_arrow() {
    let d = diagnostics(CUPS);
    assert_eq!(d.len(), 1);
    assert_eq!(
        d[0].to_string(),
        "L043: example \"two cups\" expected 1000 got 500
  --> m.lf:5:5
   |
 5 |     -> 1000
   |     ^^^^^^^
"
    );
    assert_eq!(codes(&CUPS.replace("-> 1000", "-> 500")), [""; 0]);
}

#[test]
fn every_example_runs_after_an_earlier_one_fails() {
    let source = CUPS.replace(
        "  end\n",
        "  end\n  example \"three cups\"\n    n: 3\n    -> 1\n  end\n  example \"one cup\"\n    n: 1\n    -> 250\n  end\n",
    );
    assert_eq!(
        summaries(&source),
        [
            "L043: example \"two cups\" expected 1000 got 500",
            "L043: example \"three cups\" expected 1 got 750",
        ]
    );
}

const SHAPES: &str = "type Shape
  Dot
  Box(Int)
end type Shape

type Pair
  name: Text
  shapes: List<Shape>
end type Pair

fn make(n: Int) -> Result<Pair, Shape>
  pure
  example \"renders\"
    n: 1
    -> EXPECTED
  end

  return Ok(Pair(\"a\\\"{{\", [Dot, Box(n)]))
end fn make
";

#[test]
fn structural_equality_walks_records_lists_and_variants() {
    let with = |expected: &str| SHAPES.replace("EXPECTED", expected);
    assert_eq!(
        codes(&with("Ok(Pair(\"a\\\"{{\", [Dot, Box(1)]))")),
        [""; 0]
    );
    assert_eq!(
        summaries(&with("Ok(Pair(\"b\", [Dot]))")),
        [
            "L043: example \"renders\" expected Ok(Pair(\"b\", [Dot])) got Ok(Pair(\"a\\\"{{\", [Dot, Box(1)]))"
        ]
    );
    assert_eq!(
        summaries(&with("Err(Box(1))")),
        ["L043: example \"renders\" expected Err(Box(1)) got Ok(Pair(\"a\\\"{{\", [Dot, Box(1)]))"]
    );
}

#[test]
fn exception_arrows_match_the_message_exactly() {
    let source = "fn half(n: Int) -> Int
  pure
  example \"even\"
    n: 4
    -> 2
  end
  example \"odd is a bug\"
    n: 3
    -> exception \"n must be even\"
  end
  example \"wrong message\"
    n: 3
    -> exception \"nope\"
  end
  example \"no exception\"
    n: 4
    -> exception \"n must be even\"
  end
  example \"unexpected\"
    n: 5
    -> 2
  end

  assert n % 2 == 0, \"n must be even\"
  return n / 2
end fn half
";
    assert_eq!(
        summaries(source),
        [
            "L059: example \"wrong message\" expected exception \"nope\" got exception \"n must be even\"",
            "L059: example \"no exception\" expected exception \"n must be even\" got 2",
            "L057: example \"unexpected\" expected a value but raised exception \"n must be even\" at m.lf:24:3",
        ]
    );
}

#[test]
fn fail_and_or_fail_raise_exceptions() {
    let source = "fn pick(n: Int) -> Int
  pure
  example \"fails\"
    n: 0
    -> exception \"zero\"
  end
  example \"or fails\"
    n: 500
    -> exception \"not a percent\"
  end
  example \"passes\"
    n: 7
    -> 7
  end

  if n == 0
    fail \"zero\"
  end
  let p = int_in(n, 1 through 100) or fail \"not a percent\"
  return p
end fn pick
";
    assert_eq!(codes(source), [""; 0]);
}

// `src` with one function `f(n: Int) -> Int` returning `body`, and an example binding `n`
fn faulting(input: &str, body: &str) -> String {
    format!(
        "fn f(n: Int) -> Int\n  pure\n  example \"faults\"\n    n: {input}\n    -> 0\n  end\n\n{body}\nend fn f\n"
    )
}

#[test]
fn faults_render_through_the_shared_report() {
    let cases = [
        ("9223372036854775807", "  return n + 1", "L040", (8, 10, 15)),
        ("0", "  return 10 / n", "L076", (8, 10, 16)),
        (
            "-1",
            "  return (0 - 9223372036854775807 - 1) % n",
            "L040",
            (8, 10, 43),
        ),
        (
            "0",
            "  let sep: Text = \"\"\n  return length(split(text: \"a\", separator: sep)) + n",
            "L046",
            (9, 17, 49),
        ),
    ];
    for (input, body, code, (line, start, end)) in cases {
        let source = faulting(input, body);
        let d = diagnostics(&source);
        assert_eq!(d.len(), 1, "{d:?}");
        let span = crate::ast::Span {
            start: crate::lex::Location {
                line,
                column: start,
            },
            end: crate::lex::Location { line, column: end },
        };
        assert_eq!(
            d[0].to_string(),
            crate::report::fault(code, "m.lf", &source, Some(span)),
            "{source}"
        );
    }
}

#[test]
fn a_loop_that_never_ends_runs_out_of_fuel() {
    let source = "type Step
  More(Int)
  Done
end type Step

fn spin(n: Int) -> Int
  pure
  example \"never ends\"
    n: 1
    -> 1
  end

  var next: Step = More(n)
  while next is More(k)
    set next = More(k)
  end
  return n
end fn spin
";
    let d = diagnostics(source);
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].code, "L042");
    assert!(
        d[0].to_string()
            .starts_with("fault L042: fuel exceeded (100_000 steps)\n  --> m.lf:"),
        "{}",
        d[0]
    );
}

#[test]
fn the_heap_is_capped_per_example() {
    let source = "fn grow(n: Int) -> Int
  pure
  example \"small\"
    n: 3
    -> 8
  end
  example \"doubles past the heap\"
    n: 40
    -> 0
  end

  var xs: List<Int> = [1]
  for i in 0 until n
    set xs = join(left: xs, right: xs)
  end
  return length(xs)
end fn grow
";
    assert_eq!(
        summaries(source),
        ["L047: example \"doubles past the heap\" exceeded the 64 MiB heap limit"]
    );
}

#[test]
fn clock_scripts_pop_in_order_and_run_out_with_l041() {
    let source = "fn stamped(customer: Text) -> Int
  reads clock
  example \"fixed now\"
    clock: [1700000000, 5]
    customer: \"c1\"
    -> 1700000004
  end
  example \"empty script\"
    clock: [1]
    customer: \"c1\"
    -> 0
  end

  let first = clock.now()
  return first + clock.now() - 1
end fn stamped
";
    let d = diagnostics(source);
    assert_eq!(
        d.iter()
            .map(|d| format!("{}: {}", d.code, d.summary))
            .collect::<Vec<_>>(),
        ["L041: example \"empty script\" ran out of script for `clock`"]
    );
    assert_eq!(d[0].span.unwrap().start.line, 15);
}

#[test]
fn resource_scripts_wrap_ok_and_keep_err() {
    let source = "type DbError
  Unavailable
end type DbError

resource db
  fn fetch(key: Text) -> Result<Int, DbError>
    reads
end resource db

fn total(key: Text) -> Result<Int, DbError>
  reads db
  example \"two rows\"
    db: [2, 3]
    key: \"k\"
    -> Ok(5)
  end
  example \"down\"
    db: [2, Err(Unavailable)]
    key: \"k\"
    -> Err(Unavailable)
  end
  example \"short\"
    db: [2]
    key: \"k\"
    -> Ok(2)
  end

  let a = db.fetch(key) or return
  let b = db.fetch(key) or return
  return Ok(a + b)
end fn total
";
    assert_eq!(
        summaries(source),
        ["L041: example \"short\" ran out of script for `db`"]
    );
}

#[test]
fn a_module_with_errors_skips_its_examples() {
    let bad = format!(
        "{CUPS}\nfn bad() -> Int\n  pure\n  example \"x\"\n    -> 1\n  end\n\n  return \"x\"\nend fn bad\n"
    );
    assert_eq!(codes(&bad), ["L020"]);
    let codes_of = |files: &[(&str, &str)]| -> Vec<&'static str> {
        diagnostics_of(files).iter().map(|d| d.code).collect()
    };
    // Other modules still run theirs
    assert_eq!(
        codes_of(&[("a.lf", &bad), ("b.lf", CUPS)]),
        ["L020", "L043"]
    );
    // ...unless they import a module with errors
    let importer = format!("import cups from \"a\"\n\n{CUPS}").replace("fn cups", "fn mugs");
    let importer = importer.replace("end fn cups", "end fn mugs");
    assert_eq!(codes_of(&[("a.lf", &bad), ("b.lf", &importer)]), ["L020"]);
}

#[test]
fn examples_call_across_modules_and_log_is_discarded() {
    let money = "type Cents = Int at least 0\n\nfn double(c: Cents) -> Cents\n  pure\n  example \"two\"\n    c: 1\n    -> 2\n  end\n\n  log(\"doubling {c}\")\n  return c + c\nend fn double\n";
    let order = "import Cents, double from \"money\"\n\nfn quad(c: Cents) -> Cents\n  pure\n  example \"four\"\n    c: 2\n    -> 8\n  end\n\n  return double(double(c))\nend fn quad\n";
    assert_eq!(
        diagnostics_of(&[("money.lf", money), ("order.lf", order)]),
        [] as [Diagnostic; 0]
    );
}

#[test]
fn percent_is_the_allowlisted_primitive() {
    let corpus = |name: &str| {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/corpus")
                .join(name),
        )
        .unwrap()
    };
    let money = corpus("money.lf");
    assert_eq!(
        diagnostics_of(&[("money.lf", &money)]),
        [] as [Diagnostic; 0]
    );
    let wrong = money.replace("-> 120", "-> 121");
    assert_eq!(
        diagnostics_of(&[("money.lf", &wrong)])
            .iter()
            .map(|d| d.code)
            .collect::<Vec<_>>(),
        ["L043"]
    );
}

// Runs each e2e fixture's `main` on the interpreter with enough fuel to finish, and compares
// the logs and the report with what the native binary is expected to print (docs/build.md,
// Testing, Parity).
#[test]
fn the_interpreter_matches_the_e2e_fixtures() {
    // Native-only behavior: `L077`, and tail calls 100000000 deep
    const NATIVE_ONLY: [&str; 3] = ["deep_recursion", "tail_recursion", "mutual_recursion"];
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/e2e");
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "lf"))
        .collect();
    cases.sort();
    let mut checked = 0;
    let mut problems = Vec::new();
    for case in &cases {
        let name = case.file_stem().unwrap().to_str().unwrap();
        if NATIVE_ONLY.contains(&name) {
            continue;
        }
        let program = match link(std::slice::from_ref(case), &Options::default()) {
            Ok(program) => program,
            Err(err) => {
                problems.push(format!("{name}: does not check: {err:?}"));
                continue;
            }
        };
        let Some(stderr) = super::run_main(&program, 50_000_000) else {
            continue; // `main` takes a `Process`
        };
        let want = std::fs::read_to_string(case.with_extension("stderr")).unwrap_or_default();
        if stderr != want {
            problems.push(format!("{name}:\nwant {want:?}\ngot  {stderr:?}"));
        }
        checked += 1;
    }
    assert!(checked > 40, "only {checked} fixtures ran");
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn unbounded_recursion_stops_at_the_depth_limit() {
    let source = "fn down(n: Int) -> Int
  pure
  example \"forever\"
    n: 1
    -> 0
  end

  return down(n) + 1
end fn down
";
    assert_eq!(
        summaries(source),
        ["L047: example \"forever\" exceeded the call depth limit of 10000"]
    );
}

const DOUBLE: &str = "fn double(n: Int) -> Int
  pure
  example \"two\"
    n: 2
    -> 4
  end

  return n + n
end fn double

";

#[test]
fn folded_consts_reach_function_examples() {
    let source = format!(
        "{DOUBLE}const four = double(2)

fn plus_four(n: Int) -> Int
  pure
  example \"one\"
    n: 1
    -> 5
  end

  const eight = double(four)
  return n + eight - four + comptime double(0)
end fn plus_four
"
    );
    assert_eq!(codes(&source), [""; 0]);
    assert_eq!(
        summaries(&source.replace("-> 5", "-> 6")),
        ["L043: example \"one\" expected 6 got 5"]
    );
}

#[test]
fn package_examples_run_after_function_examples() {
    let passing = format!(
        "{DOUBLE}example \"doubles\"
  const two = 2
  -> double(two) == 4
end

example \"fails\"
  fail \"boom\"
  -> exception \"boom\"
end
"
    );
    assert_eq!(codes(&passing), [""; 0]);
    let summaries_of = |example: &str| summaries(&format!("{DOUBLE}{example}"));
    assert_eq!(
        summaries_of("example \"false\"\n  -> double(2) == 5\nend\n"),
        ["L056: package example \"false\" evaluated to false: double(2) == 5"]
    );
    assert_eq!(
        summaries_of("example \"raises\"\n  fail \"boom\"\n  -> true\nend\n"),
        ["L057: example \"raises\" expected a value but raised exception \"boom\" at m.lf:12:3"]
    );
    assert_eq!(
        summaries_of("example \"quiet\"\n  -> exception \"boom\"\nend\n"),
        [
            "L059: package example \"quiet\" expected exception \"boom\" but finished without raising"
        ]
    );
    assert_eq!(
        summaries_of("example \"other\"\n  fail \"bang\"\n  -> exception \"boom\"\nend\n"),
        ["L059: example \"other\" expected exception \"boom\" got exception \"bang\""]
    );
}

#[test]
fn comptime_roots_fault_and_raise() {
    let spin = "type Step
  More(Int)
  Done
end type Step

fn spin(n: Int) -> Int
  pure
  example \"stops\"
    n: 0
    -> 0
  end

  var next: Step = More(n)
  if n == 0
    set next = Done
  end
  while next is More(k)
    set next = More(k)
  end
  return n
end fn spin

";
    assert_eq!(codes(&format!("{spin}const s = spin(1)\n")), ["L042"]);
    assert_eq!(
        codes(&format!("{spin}type T = Int in 1 through spin(1)\n")),
        ["L042"]
    );
    let raising = "fn picky(n: Int) -> Int
  pure
  example \"fine\"
    n: 1
    -> 1
  end

  assert n == 1, \"not one\"
  return n
end fn picky

const p = picky(2)
";
    assert_eq!(
        summaries(raising),
        ["L058: exception \"not one\" aborted the comptime evaluation of `p` at m.lf:8:3"]
    );
}
