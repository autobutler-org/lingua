//! One accepting and one rejecting program per diagnostic code the checker emits.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{Diagnostic, Error, Options, check, link};

static NEXT: AtomicUsize = AtomicUsize::new(0);

// A fresh source root holding `files`
fn package(files: &[(&str, &str)]) -> PathBuf {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("lingua-check-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    dir
}

fn diagnostics(result: Result<(), Error>) -> Vec<Diagnostic> {
    match result {
        Ok(()) => Vec::new(),
        Err(Error::Diagnostics(d)) => d,
        Err(Error::Usage(u)) => panic!("usage error: {u}"),
    }
}

// The codes from checking every module in a package
fn codes_of(files: &[(&str, &str)]) -> Vec<&'static str> {
    let root = package(files);
    let d = diagnostics(check(&[root], &Options::default()).map(|_| ()));
    d.iter().map(|d| d.code).collect()
}

fn codes(source: &str) -> Vec<&'static str> {
    codes_of(&[("m.lf", source)])
}

fn build_codes(files: &[(&str, &str)]) -> Vec<&'static str> {
    let root = package(files);
    diagnostics(link(&[root], &Options::default()).map(|_| ()))
        .iter()
        .map(|d| d.code)
        .collect()
}

// A `main` whose body is `body` (indented by the caller), then `return nothing`
fn main(body: &str) -> String {
    format!("fn main() -> Nothing\n  pure\n\n{body}\n  return nothing\nend fn main\n")
}

#[track_caller]
fn accepts(source: &str) {
    let root = package(&[("m.lf", source)]);
    if let Err(Error::Diagnostics(d)) = check(&[root], &Options::default()) {
        let text: String = d.iter().map(ToString::to_string).collect();
        panic!("expected a clean check of\n{source}\ngot\n{text}");
    }
}

#[track_caller]
fn rejects(source: &str, code: &str) {
    assert_eq!(codes(source), [code], "{source}");
}

const SUM: &str = "type Light\n  Red\n  Green(Int)\nend type Light\n\n";
const CENTS: &str = "type Cents = Int at least 0\n\n";

fn with(prefix: &str, body: &str) -> String {
    format!("{prefix}{}", main(body))
}

#[test]
fn the_normative_corpus_checks_clean() {
    let corpus = |name| {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/corpus")
                .join(name),
        )
        .unwrap()
    };
    let money = corpus("money.lf");
    let order = corpus("order.lf");
    let fulfillment = corpus("fulfillment.lf");
    assert_eq!(
        codes_of(&[
            ("money.lf", &money),
            ("order.lf", &order),
            ("fulfillment.lf", &fulfillment)
        ]),
        Vec::<&str>::new()
    );
}

#[test]
fn l010_unknown_export_lists_the_exports() {
    let money = "type Cents = Int at least 0\n\ntype Rounding\n  half_up\nend type Rounding\n";
    assert_eq!(
        codes_of(&[
            ("money.lf", money),
            (
                "m.lf",
                &format!(
                    "import Cents, half_up from \"money\"\n\n{}",
                    main("  let c: Cents = 1")
                )
            )
        ]),
        Vec::<&str>::new()
    );
    let root = package(&[
        ("money.lf", money),
        ("m.lf", "import percet from \"money\"\n"),
    ]);
    let Err(Error::Diagnostics(d)) = check(&[root], &Options::default()) else {
        panic!()
    };
    assert_eq!(d.len(), 1);
    assert_eq!(
        d[0].to_string(),
        "L010: money.lf does not export `percet`\n  --> m.lf:1:8\n   |\n 1 | import percet from \"money\"\n   |        ^^^^^^\n   |\n   = help: legal alternatives: Cents, Rounding, half_up\n"
    );
}

#[test]
fn l010_help_lists_the_end_state_money_exports() {
    let money = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/money.lf"),
    )
    .unwrap();
    let root = package(&[
        ("money.lf", &money),
        ("order.lf", "import percet from \"money\"\n"),
    ]);
    let Err(Error::Diagnostics(d)) = check(&[root], &Options::default()) else {
        panic!()
    };
    assert_eq!(
        d[0].help.as_deref(),
        Some(
            "legal alternatives: Cents, Rounding, half_up, percent, PowStep, Go, Stop, next_pow2, page_cap, Batch"
        )
    );
}

#[test]
fn l011_unknown_names_fields_and_types() {
    accepts(&main("  let x = 1\n  log(\"{x}\")"));
    rejects(&main("  log(\"{y}\")"), "L011");
    rejects(&main("  let p: Process = nope"), "L011");
    rejects(&main("  let x: Nope = 1"), "L011");
    rejects(
        &format!(
            "type P\n  a: Int\nend type P\n\n{}",
            main("  let p = P(1)\n  log(\"{p.b}\")")
        ),
        "L011",
    );
}

#[test]
fn l012_falling_off_a_function_or_a_statement_arm() {
    accepts(
        "fn f(n: Int) -> Int\n  pure\n  example \"e\"\n    n: 1\n    -> 1\n  end\n\n  return n\nend fn f\n",
    );
    rejects(
        "fn f(n: Int) -> Int\n  pure\n  example \"e\"\n    n: 1\n    -> 1\n  end\n\n  log(\"x\")\nend fn f\n",
        "L012",
    );
    rejects(
        &with(
            SUM,
            "  let l: Light = Red\n  match l\n    Red -> return nothing\n    Green(_) ->\n      log(\"g\")\n  end",
        ),
        "L012",
    );
}

#[test]
fn l013_no_shadowing() {
    accepts(&main("  let a = 1\n  let b = 2\n  log(\"{a}{b}\")"));
    rejects(&main("  let a = 1\n  let a = 2"), "L013");
    rejects(&main("  let trim = 1"), "L013");
    rejects(
        &with(
            SUM,
            "  let l: Light = Red\n  let n = 1\n  let x = match l\n    Red -> 0\n    Green(n) -> n\n  end",
        ),
        "L013",
    );
}

#[test]
fn l014_set_needs_a_var() {
    accepts(&main("  var a = 1\n  set a = 2"));
    rejects(&main("  let a = 1\n  set a = 2"), "L014");
}

#[test]
fn l015_import_cycles() {
    let a = "import b from \"b\"\n\nfn a() -> Int\n  pure\n  example \"e\"\n    -> 1\n  end\n\n  return 1\nend fn a\n";
    let b = "import a from \"a\"\n\nfn b() -> Int\n  pure\n  example \"e\"\n    -> 1\n  end\n\n  return 1\nend fn b\n";
    let b_ok = "fn b() -> Int\n  pure\n  example \"e\"\n    -> 1\n  end\n\n  return 1\nend fn b\n";
    assert_eq!(codes_of(&[("a.lf", a), ("b.lf", b_ok)]), Vec::<&str>::new());
    assert_eq!(codes_of(&[("a.lf", a), ("b.lf", b)]), ["L015"]);
}

#[test]
fn l016_bad_missing_and_self_imports_and_paths_outside_the_root() {
    rejects("import x from \"nowhere\"\n", "L016");
    rejects("import x from \"a/b\"\n", "L016");
    rejects("import x from \"_\"\n", "L016");
    rejects("import x from \"m\"\n", "L016");
    let root = package(&[("lingua.toml", "version = \"lingua1\"\n")]);
    std::fs::create_dir_all(root.join("sub")).unwrap();
    std::fs::write(root.join("sub/x.lf"), "").unwrap();
    let d = diagnostics(check(&[root.join("sub/x.lf")], &Options::default()).map(|_| ()));
    assert_eq!(d.iter().map(|d| d.code).collect::<Vec<_>>(), ["L016"]);
    assert!(
        d[0].help
            .as_ref()
            .unwrap()
            .starts_with("the source root is ")
    );
}

#[test]
fn l017_duplicates_and_prelude_clashes() {
    accepts(&with(CENTS, "  let c: Cents = 1"));
    rejects(&format!("{CENTS}{CENTS}"), "L017");
    rejects("type Result = Int\n", "L017");
    rejects("type A\n  x: Int\n  x: Int\nend type A\n", "L017");
}

#[test]
fn l018_unreachable_statements() {
    accepts(&main("  log(\"a\")"));
    rejects(
        "fn main() -> Nothing\n  pure\n\n  return nothing\n  log(\"a\")\nend fn main\n",
        "L018",
    );
}

#[test]
fn l020_type_mismatches() {
    accepts(&main("  let t: Text = \"x\""));
    rejects(&main("  let t: Text = 1"), "L020");
    rejects(&main("  let r = Ok(1)"), "L020");
    rejects(&main("  let xs = []"), "L020");
    rejects(
        &main("  let b = true\n  let x = match b\n    _ -> 1\n  end"),
        "L020",
    );
}

#[test]
fn l021_positional_arguments_need_unique_types() {
    accepts(&main("  let m = max(a: 1, b: 2)\n  log(\"{m}\")"));
    rejects(&main("  let m = max(1, 2)"), "L021");
}

#[test]
fn l022_holes_need_a_text_conversion() {
    accepts(&main("  let n = 1\n  log(\"{n}\")"));
    rejects(&main("  let b = true\n  log(\"{b}\")"), "L022");
}

#[test]
fn l023_implicit_narrowing() {
    accepts(&with(CENTS, "  let a: Cents = 1\n  let b: Cents = a + a"));
    rejects(&with(CENTS, "  let n: Int = 1\n  let c: Cents = n"), "L023");
    rejects(&with(CENTS, "  let c = Cents(1)"), "L023");
    rejects(
        &with(
            CENTS,
            "  let n = int_at_least(\"1\", 0) or return nothing\n  let c: Cents = n",
        ),
        "L023",
    );
}

#[test]
fn l024_literals_and_static_bounds() {
    accepts(&with(CENTS, "  let c: Cents = 0\n  let x = [1, 2][1]"));
    rejects(&with(CENTS, "  let c: Cents = -1"), "L024");
    rejects("type Bad = Int in 5 through 1\n", "L024");
    rejects(&main("  let x = [1, 2][2]"), "L024");
}

#[test]
fn l025_effects() {
    let reads = "fn now() -> Int\n  reads clock\n  example \"e\"\n    clock: [1]\n    -> 1\n  end\n\n  return clock.now()\nend fn now\n\n";
    accepts(&format!(
        "{reads}fn main() -> Nothing\n  reads clock\n\n  log(\"{{now()}}\")\n  return nothing\nend fn main\n"
    ));
    rejects(&format!("{reads}{}", main("  log(\"{now()}\")")), "L025");
    rejects(&main("  let t = clock.now()"), "L025");
    rejects(
        "fn f() -> Int\n  reads net\n  example \"e\"\n    -> 1\n  end\n\n  return 1\nend fn f\n",
        "L025",
    );
}

#[test]
fn l026_or_return_needs_the_same_error_type() {
    let e = "type E\n  Bad\nend type E\n\n";
    let f = |body: &str| {
        format!(
            "{e}fn f(raw: Text) -> Result<Int, E>\n  pure\n  example \"e\"\n    raw: \"1\"\n    -> Ok(1)\n  end\n\n{body}\n  return Ok(n)\nend fn f\n"
        )
    };
    accepts(&f("  let n = int_in(raw, 1 through 9) or return Err(Bad)"));
    rejects(&f("  let n = int_in(raw, 1 through 9) or return"), "L026");
}

#[test]
fn l027_if_needs_bool() {
    accepts(&main("  if 1 == 1\n    log(\"x\")\n  end"));
    rejects(&main("  if 1\n    log(\"x\")\n  end"), "L027");
}

#[test]
fn l028_arity_and_parameter_names() {
    accepts(&with(SUM, "  let l = Green(1)"));
    rejects(&with(SUM, "  let l = Red(1)"), "L028");
    rejects(&with(SUM, "  let l = Green"), "L028");
    rejects(&main("  let t = trim(text: \"a\", extra: \"b\")"), "L028");
    rejects(&main("  let t = trim()"), "L028");
}

#[test]
fn l029_expression_statements_are_nothing() {
    accepts(&main("  log(\"x\")"));
    rejects(&main("  trim(\"x\")"), "L029");
}

#[test]
fn l030_while_paths_must_set_the_variable() {
    let s = "type S\n  Go(Int)\n  Stop\nend type S\n\n";
    accepts(&with(
        s,
        "  var s: S = Go(1)\n  while s is Go(n)\n    if n > 3\n      set s = Stop\n    else\n      set s = Go(n + 1)\n    end\n  end",
    ));
    rejects(
        &with(
            s,
            "  var s: S = Go(1)\n  while s is Go(n)\n    if n > 3\n      set s = Stop\n    end\n  end",
        ),
        "L030",
    );
}

#[test]
fn l031_l032_l033_closed_and_open_matches() {
    let d = "type D\n  A\n  B(BoundsError)\nend type D\n\n";
    accepts(&with(
        d,
        "  let d: D = A\n  let x = match d\n    A -> 1\n    B(OutOfRange) -> 2\n    B(NotAnInteger) -> 3\n  end",
    ));
    rejects(
        &with(
            d,
            "  let d: D = A\n  let x = match d\n    A -> 1\n    B(OutOfRange) -> 2\n  end",
        ),
        "L031",
    );
    rejects(
        &with(
            d,
            "  let d: D = A\n  let x = match d\n    A -> 1\n    _ -> 2\n  end",
        ),
        "L032",
    );
    rejects(
        &with(
            d,
            "  let d: D = A\n  let x = match d\n    A -> 1\n    B(_) -> 2\n    B(OutOfRange) -> 3\n  end",
        ),
        "L033",
    );
    rejects(
        &main("  let n = 1\n  let x = match n\n    1 -> 1\n  end"),
        "L031",
    );
    accepts(&main(
        "  let n = 1\n  let x = match n\n    1 -> 1\n    _ -> 2\n  end",
    ));
}

#[test]
fn l031_help_names_nested_missing_variants() {
    let root = package(&[(
        "m.lf",
        &main(
            "  let r = int_in(\"1\", 1 through 2)\n  let x = match r\n    Ok(_) -> 1\n    Err(OutOfRange) -> 2\n  end",
        ),
    )]);
    let d = diagnostics(check(&[root], &Options::default()).map(|_| ()));
    assert_eq!(
        d[0].help.as_deref(),
        Some("legal alternatives: Err(NotAnInteger)")
    );
}

#[test]
fn l034_operators_and_cross_nominal_comparison() {
    accepts(&with(
        CENTS,
        "  let a: Cents = 1\n  if a == 1\n    log(\"one\")\n  end",
    ));
    rejects(
        &with(
            CENTS,
            "  let a: Cents = 1\n  let n: Int = 1\n  if a == n\n    log(\"x\")\n  end",
        ),
        "L034",
    );
    rejects(&main("  let t = \"a\" + \"b\""), "L034");
}

#[test]
fn l035_add_assign_needs_an_integer() {
    accepts(&main("  var n = 1\n  set n += 2"));
    rejects(&main("  var t = \"a\"\n  set t += 1"), "L035");
}

#[test]
fn l044_l045_example_bindings() {
    let f = |bind: &str| {
        format!(
            "fn f(n: Int) -> Int\n  pure\n  example \"e\"\n{bind}    -> 1\n  end\n\n  return n\nend fn f\n"
        )
    };
    accepts(&f("    n: 1\n"));
    rejects(&f(""), "L044");
    rejects(&f("    n: 1\n    m: 2\n"), "L045");
}

#[test]
fn l046_split_on_a_literal_empty_separator() {
    accepts(&main("  let p = split(text: \"a\", separator: \":\")"));
    rejects(
        &main("  let p = split(text: \"a\", separator: \"\")"),
        "L046",
    );
}

#[test]
fn l049_primitive_is_only_money_percent() {
    let money = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/money.lf"),
    )
    .unwrap();
    assert_eq!(codes_of(&[("money.lf", &money)]), Vec::<&str>::new());
    assert_eq!(codes_of(&[("cash.lf", &money)]), ["L049"]);
}

#[test]
fn l050_toml_keys() {
    assert_eq!(
        codes_of(&[("lingua.toml", "version = \"lingua1\"\n")]),
        Vec::<&str>::new()
    );
    assert_eq!(
        codes_of(&[("lingua.toml", "version = \"lingua2\"\n")]),
        ["L050"]
    );
}

#[test]
fn l060_runtime_bound() {
    accepts(&main("  let r = int_at_least(\"1\", 0)"));
    rejects(
        &main("  let n = 1\n  let r = int_at_least(\"1\", n)"),
        "L060",
    );
}

#[test]
fn l067_provide_outside_a_comptime_block() {
    rejects("provide X = 1\n", "L067");
}

#[test]
fn l068_messages_are_hole_free_literals() {
    accepts(&main("  assert 1 == 1, \"fine\""));
    rejects(&main("  let n = 1\n  assert n == 1, \"n is {n}\""), "L068");
}

#[test]
fn l072_l074_l075_build_rules() {
    let fulfillment = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/fulfillment.lf"),
    )
    .unwrap();
    assert_eq!(
        build_codes(&[("m.lf", &main("  log(\"x\")"))]),
        Vec::<&str>::new()
    );
    assert_eq!(
        build_codes(&[("m.lf", &main("  log(\"x\")")), ("f.lf", &fulfillment)]),
        ["L072"]
    );
    assert_eq!(
        build_codes(&[("a.lf", &main("")), ("b.lf", &main(""))]),
        ["L074"]
    );
    assert_eq!(build_codes(&[("a.lf", "type A = Int\n")]), ["L074"]);
    assert_eq!(
        build_codes(&[(
            "m.lf",
            "fn main(n: Int) -> Nothing\n  pure\n\n  return nothing\nend fn main\n"
        )]),
        ["L075"]
    );
}

#[test]
fn l076_literal_zero_divisor() {
    accepts(&main("  let n = 1\n  let q = n / 2"));
    rejects(&main("  let n = 1\n  let q = n / 0"), "L076");
}

// A pure function the comptime tests fold: `double(n)` is `n + n`
const DOUBLE: &str = "fn double(n: Int) -> Int\n  pure\n  example \"two\"\n    n: 2\n    -> 4\n  end\n\n  return n + n\nend fn double\n\n";

#[test]
fn consts_and_comptime_fold() {
    // File-level, imported, inner, a `comptime` call, and a bound, each folding a user call
    accepts(&format!(
        "{DOUBLE}const four = double(2)\n\n{}",
        main("  let x: Int = four + 1")
    ));
    accepts(&format!(
        "{DOUBLE}{}",
        main("  const six = double(3)\n  let x = six + comptime double(4)")
    ));
    accepts(&format!(
        "{DOUBLE}type Small = Int in 1 through comptime double(4)\n\n{}",
        main("  let s: Small = 8")
    ));
    rejects(
        &format!(
            "{DOUBLE}type Small = Int in 1 through double(4)\n\n{}",
            main("  let s: Small = 9")
        ),
        "L024",
    );
    // A const declared after its use, and a bound that calls a function declared after it
    accepts(&format!(
        "{}\nconst late: Int = 3\n",
        main("  let x = late")
    ));
    accepts(&format!(
        "type Small = Int in 1 through double(4)\n\n{DOUBLE}"
    ));
    // A body checked for a bound uses a sum declared after both
    accepts(
        "type Small = Int in 1 through pick()\n\nfn pick() -> Int\n  pure\n  example \"e\"\n    -> 4\n  end\n\n  return match Go(4)\n    Go(n) -> n\n    Stop -> 1\n  end\nend fn pick\n\ntype Step\n  Go(Int)\n  Stop\nend type Step\n",
    );
    assert_eq!(
        codes_of(&[
            ("a.lf", &format!("{DOUBLE}const four: Int = double(2)\n")),
            (
                "b.lf",
                &format!("import four from \"a\"\n\n{}", main("  let x = four"))
            )
        ]),
        Vec::<&str>::new()
    );
    // The declared type checks the initializer
    rejects("const t: Text = 1\n", "L020");
}

#[test]
fn l025_const_effects() {
    rejects("const x\n  reads clock\n  = 1\n", "L025");
    rejects("const x\n  writes db\n  = 1\n", "L025");
    // A pure const cannot call a `reads clock` function
    rejects(
        "fn now() -> Int\n  reads clock\n  example \"e\"\n    clock: [1]\n    -> 1\n  end\n\n  return clock.now()\nend fn now\n\nconst t = now()\n",
        "L025",
    );
    // `http_get` is a per-declaration "not yet": lingua has no HTTPS client
    rejects(
        "const x: Text\n  reads net\n  = http_get(\"https://example.com\") or fail \"down\"\n",
        super::NOT_YET,
    );
}

#[test]
fn l060_runtime_values_in_comptime_contexts() {
    rejects(
        &format!("{DOUBLE}{}", main("  let n = 1\n  const m = double(n)")),
        "L060",
    );
    rejects(
        &format!(
            "{DOUBLE}{}",
            main("  let n = 1\n  let m = comptime double(n)")
        ),
        "L060",
    );
    rejects(
        "fn now() -> Int\n  reads clock\n  example \"e\"\n    clock: [1]\n    -> 1\n  end\n\n  const t = clock.now()\n  return t\nend fn now\n",
        "L060",
    );
}

#[test]
fn l061_comptime_cycles() {
    rejects("const a: Int = b\nconst b: Int = a\n", "L061");
    rejects("const a: Int = a + 1\n", "L061");
    rejects(
        "const a: Int = f()\n\nfn f() -> Int\n  pure\n  example \"e\"\n    -> 1\n  end\n\n  return a\nend fn f\n",
        "L061",
    );
    let root = package(&[("m.lf", "const a: Int = b\nconst b: Int = a\n")]);
    let Err(Error::Diagnostics(d)) = check(&[root], &Options::default()) else {
        panic!()
    };
    assert_eq!(d[0].summary, "comptime dependency cycle: `a` -> `b` -> `a`");
}

#[test]
fn l062_let_and_var_in_a_comptime_context() {
    rejects("example \"e\"\n  let x = 1\n  -> x == 1\nend\n", "L062");
    rejects("example \"e\"\n  var x = 1\n  -> x == 1\nend\n", "L062");
    accepts("example \"e\"\n  const x = 1\n  -> x == 1\nend\n");
    // Not inside an ordinary function a comptime root calls
    accepts(&format!(
        "{DOUBLE}const six = twice(3)\n\nfn twice(n: Int) -> Int\n  pure\n  example \"e\"\n    n: 1\n    -> 2\n  end\n\n  let m = double(n)\n  return m\nend fn twice\n"
    ));
}

#[test]
fn package_example_rules() {
    rejects("example \"e\"\n  -> 1\nend\n", "L055");
    rejects("example \"e\"\n  return 1\n  -> true\nend\n", "L012");
    // May call pure functions, not `reads` ones
    accepts(&format!(
        "{DOUBLE}example \"e\"\n  -> double(2) == 4\nend\n"
    ));
    rejects(
        "fn now() -> Int\n  reads clock\n  example \"e\"\n    clock: [1]\n    -> 1\n  end\n\n  return clock.now()\nend fn now\n\nexample \"e\"\n  -> now() == 1\nend\n",
        "L025",
    );
}

#[test]
fn comptime_constructs_still_not_supported_are_per_declaration() {
    let generative = "type Customers\n  reads net\n  comptime\n    const body = http_get(\"https://x\") or fail \"down\"\n    return record_from_columns([])\n  end\nend type Customers\n\n";
    // The rest of the module still checks and runs its examples
    let wrong =
        "fn one() -> Int\n  pure\n  example \"e\"\n    -> 2\n  end\n\n  return 1\nend fn one\n";
    assert_eq!(
        codes(&format!("{generative}{wrong}")),
        [super::NOT_YET, "L043"]
    );
    // ...but not the examples of a function that leans on an unsupported declaration, or of
    // one that calls such a function
    let url = "const url: Text\n  reads net\n  = http_get(\"https://x\") or fail \"down\"\n\n";
    let leans = "fn two() -> Int\n  pure\n  example \"e\"\n    -> 3\n  end\n\n  log(url)\n  return 2\nend fn two\n\nfn three() -> Int\n  pure\n  example \"e\"\n    -> 0\n  end\n\n  return two() + 1\nend fn three\n\n";
    assert_eq!(
        codes(&format!("{url}{leans}{wrong}")),
        [super::NOT_YET, "L043"]
    );
}

#[test]
fn bound_synthesis_adopts_where_the_spec_says() {
    // max restores Cents; `/` and `%` bounds; a loop index as a static index
    accepts(&with(
        CENTS,
        "  let n: Int = -5\n  let c: Cents = max(a: 0, b: n)\n  let d: Int in 0 through 9 = c % 10\n  for i in 0 until 2\n    let x = [\"a\", \"b\"][i]\n  end",
    ));
    rejects(
        &with(CENTS, "  let n: Int = -5\n  let c: Cents = n - 1"),
        "L023",
    );
}

#[test]
fn out_of_order_arguments_are_bound_in_source_order() {
    use crate::tir::{TirBody, TirExpr, TirKind, TirStmt};
    let noisy = "fn noisy(n: Int) -> Int\n  pure\n  example \"e\"\n    n: 1\n    -> 1\n  end\n\n  log(\"{n}\")\n  return n\nend fn noisy\n\n";
    let diff = "fn diff(a: Int, b: Int) -> Int\n  pure\n  example \"e\"\n    a: 3\n    b: 1\n    -> 2\n  end\n\n  return a - b\nend fn diff\n\n";
    // The initializer of `let d = CALL` in `main`
    let initializer = |call: &str| -> TirExpr {
        let body = format!("  let d = {call}\n  log(\"{{d}}\")");
        let source = format!("{noisy}{diff}{}", main(&body));
        let mut program = link(&[package(&[("m.lf", &source)])], &Options::default()).unwrap();
        let main_id = program.main;
        let module = program.modules.pop().unwrap();
        let f = module
            .functions
            .into_iter()
            .find(|f| f.id == main_id)
            .unwrap();
        let TirBody::Stmts(mut stmts) = f.body else {
            panic!("main has statements")
        };
        let TirStmt::Let(_, e) = stmts.remove(0) else {
            panic!("main starts with the let")
        };
        e
    };
    // `n` in `noisy(n)`
    let noisy_arg = |e: &TirExpr| match &e.kind {
        TirKind::Call(_, args) => match args[..] {
            [
                TirExpr {
                    kind: TirKind::Int(n),
                    ..
                },
            ] => n,
            _ => panic!("noisy takes one literal"),
        },
        _ => panic!("expected a call of noisy"),
    };
    let local = |e: &TirExpr| match e.kind {
        TirKind::Local(id) => id,
        _ => panic!("expected a temporary"),
    };
    // In parameter order, or out of order but moving only a literal: a plain call
    for call in ["diff(a: noisy(3), b: noisy(1))", "diff(b: 1, a: noisy(3))"] {
        assert!(
            matches!(initializer(call).kind, TirKind::Call(..)),
            "{call}"
        );
    }
    // Out of order: `b`'s argument is bound first, and the call takes the temporaries as `a, b`
    let TirKind::Let(binds, call) = initializer("diff(b: noisy(1), a: noisy(3))").kind else {
        panic!("expected the arguments bound first")
    };
    assert_eq!(
        binds.iter().map(|(_, e)| noisy_arg(e)).collect::<Vec<_>>(),
        [1, 3]
    );
    let TirKind::Call(_, args) = &call.kind else {
        panic!("expected the call last")
    };
    assert_eq!(
        args.iter().map(local).collect::<Vec<_>>(),
        [binds[1].0, binds[0].0]
    );
}

// ---- generative types, package `comptime` blocks, and the provider cache

const COLUMNS: &str = "[Column(name: \"id\", sql_type: \"integer\"), Column(name: \"name\", sql_type: \"text\"), Column(name: \"active\", sql_type: \"boolean\")]";

// A generative `type Row` with effect `effect` and body `body` (indented by the caller)
fn generative(effect: &str, body: &str) -> String {
    format!("type Row\n  {effect}\n  comptime\n{body}\n  end\nend type Row\n\n")
}

fn cache_of(root: &std::path::Path, key: &str) -> Option<String> {
    std::fs::read_to_string(root.join("provided").join(format!("{key}.lf"))).ok()
}

fn check_with(root: &std::path::Path, cache: super::Cache) -> Vec<&'static str> {
    let options = Options {
        cache,
        ..Options::default()
    };
    diagnostics(check(&[root.to_path_buf()], &options).map(|_| ()))
        .iter()
        .map(|d| d.code)
        .collect()
}

#[test]
fn a_generative_type_is_an_ordinary_product_to_the_rest_of_the_module() {
    let row = generative(
        "pure",
        &format!("    return record_from_columns({COLUMNS})"),
    );
    let uses = "fn label(r: Row) -> Text\n  pure\n  example \"e\"\n    r: Row(id: 1, name: \"a\", active: true)\n    -> \"a\"\n  end\n\n  return r.name\nend fn label\n";
    let root = package(&[("m.lf", &format!("{row}{uses}"))]);
    assert_eq!(check_with(&root, super::Cache::Default), Vec::<&str>::new());
    assert_eq!(
        cache_of(&root, "m").as_deref(),
        Some("type Row\n  id: Int\n  name: Text\n  active: Bool\nend type Row\n")
    );
    // The `comptime` line is optional, and the body may call ordinary functions and consts
    accepts(&format!(
        "{DOUBLE}const k = double(2)\n\ntype P\n  pure\n  return record_from_columns([Column(name: \"n{{k}}\", sql_type: \"int4\")])\nend\nend type P\n"
    ));
    // A field that does not exist is the usual diagnostic at the use
    rejects(
        &format!("{row}const r = Row(id: 1, nom: \"a\", active: true)\n"),
        "L028",
    );
}

#[test]
fn record_from_columns_failures_abort_the_check_with_l058() {
    for (columns, message) in [
        ("[]", "no columns"),
        (
            "[Column(name: \"x\", sql_type: \"json\")]",
            "unknown sql type",
        ),
        (
            "[Column(name: \"end\", sql_type: \"text\")]",
            "bad column name",
        ),
        (
            "[Column(name: \"x\", sql_type: \"text\"), Column(name: \"x\", sql_type: \"text\")]",
            "duplicate column",
        ),
    ] {
        let source = generative(
            "pure",
            &format!("    return record_from_columns({columns})"),
        );
        let root = package(&[("m.lf", &source)]);
        let Err(Error::Diagnostics(d)) = check(std::slice::from_ref(&root), &Options::default())
        else {
            panic!("{columns} checked clean")
        };
        assert_eq!(
            d.iter().map(|d| d.code).collect::<Vec<_>>(),
            ["L058"],
            "{columns}"
        );
        assert!(
            d[0].summary.contains(&format!("\"{message}\"")),
            "{}",
            d[0].summary
        );
        assert_eq!(cache_of(&root, "m"), None);
    }
    rejects(&generative("pure", "    fail \"no schema\""), "L058");
}

#[test]
fn generative_body_rules() {
    // Falling off the body, or returning something other than a `Type`
    rejects(&generative("pure", "    const x = 1"), "L012");
    rejects(&generative("pure", "    return 1"), "L020");
    // `let` and `var` are `L062`; `provide` is `L067` here
    rejects(
        &generative(
            "pure",
            &format!("    let c = {COLUMNS}\n    return record_from_columns(c)"),
        ),
        "L062",
    );
    rejects(
        &generative(
            "pure",
            &format!(
                "    provide X = record_from_columns({COLUMNS})\n    return record_from_columns({COLUMNS})"
            ),
        ),
        "L067",
    );
    // The effect clause: no function effect, no user resource
    for effect in ["reads clock", "reads db", "writes db"] {
        rejects(
            &generative(
                effect,
                &format!("    return record_from_columns({COLUMNS})"),
            ),
            "L025",
        );
    }
    // `Type` is comptime only: a runtime call is `L063`, and so is a runtime use of a const
    // holding one, which a comptime context may use
    rejects(
        &main(&format!("  let t = record_from_columns({COLUMNS})")),
        "L063",
    );
    let t = format!("const t = record_from_columns({COLUMNS})\n\n");
    rejects(&with(&t, "  let u = t"), "L063");
    accepts(&format!(
        "{t}{}",
        generative("pure", "    const u = t\n    return u")
    ));
}

#[test]
fn package_comptime_blocks_provide_types() {
    let block = format!(
        "comptime\n  pure\n  provide A = record_from_columns({COLUMNS})\n  provide B = record_from_columns([Column(name: \"b\", sql_type: \"bool\")])\nend\n\n"
    );
    let uses = "const b = B(b: true)\n";
    let root = package(&[("m.lf", &format!("{block}{uses}"))]);
    assert_eq!(check_with(&root, super::Cache::Default), Vec::<&str>::new());
    assert_eq!(
        cache_of(&root, "m").as_deref(),
        Some(
            "type A\n  id: Int\n  name: Text\n  active: Bool\nend type A\n\ntype B\n  b: Bool\nend type B\n"
        )
    );
    // The effect is optional; `return` is `L012`; a `provide` nested in a statement is `L067`
    accepts(&format!(
        "comptime\n  provide A = record_from_columns({COLUMNS})\nend\n"
    ));
    rejects(
        &format!("comptime\n  return record_from_columns({COLUMNS})\nend\n"),
        "L012",
    );
    rejects(
        &format!(
            "comptime\n  if true\n    provide A = record_from_columns({COLUMNS})\n  end\nend\n"
        ),
        "L067",
    );
    rejects("comptime\n  reads clock\n  log(\"x\")\nend\n", "L025");
    rejects("comptime\n  provide A = 1\nend\n", "L020");
    rejects("comptime\n  assert 1 == 2, \"broken\"\nend\n", "L058");
    // `provide` anywhere else is `L067`
    rejects(
        &main(&format!("  provide A = record_from_columns({COLUMNS})")),
        "L067",
    );
}

#[test]
fn reads_fs_is_legal_and_fs_read_is_not_yet() {
    // `reads fs` passes the effect rules; nothing performs it yet
    let row = generative(
        "reads fs",
        "    const text = fs.read(\"cols.txt\") or fail \"no columns file\"\n    return record_from_columns([Column(name: trim(text), sql_type: \"text\")])",
    );
    rejects(&row, super::NOT_YET);
    rejects(
        "const t: Text\n  reads fs\n  = fs.read(\"x\") or fail \"missing\"\n",
        super::NOT_YET,
    );
    // `fs.read` needs `reads fs`: not in a pure const, not in a function
    rejects("const t = fs.read(\"x\")\n", "L025");
    rejects(
        "fn f() -> Int\n  reads fs\n  example \"e\"\n    -> 1\n  end\n\n  return 1\nend fn f\n",
        "L025",
    );
}

#[test]
fn the_cache_policy() {
    let row = generative(
        "pure",
        &format!("    return record_from_columns({COLUMNS})"),
    );
    let expanded = "type Row\n  id: Int\n  name: Text\n  active: Bool\nend type Row\n";
    let edited = "type Row\n  other: Text\nend type Row\n";
    let root = package(&[("m.lf", &row)]);
    // Offline with no cache: one `L071` per missing declaration, even a pure one, and
    // nothing written
    assert_eq!(check_with(&root, super::Cache::Offline), ["L071"]);
    assert_eq!(cache_of(&root, "m"), None);
    // The default writes it; later checks reuse it without running the body
    assert_eq!(check_with(&root, super::Cache::Default), Vec::<&str>::new());
    assert_eq!(cache_of(&root, "m").as_deref(), Some(expanded));
    std::fs::write(root.join("provided/m.lf"), edited).unwrap();
    for policy in [super::Cache::Default, super::Cache::Offline] {
        assert_eq!(check_with(&root, policy), Vec::<&str>::new());
        assert_eq!(cache_of(&root, "m").as_deref(), Some(edited));
    }
    // ...and what it reuses is what the module sees
    let uses = format!("{row}const r = Row(other: \"x\")\n");
    let reuse = package(&[("m.lf", &uses)]);
    std::fs::create_dir_all(reuse.join("provided")).unwrap();
    std::fs::write(reuse.join("provided/m.lf"), edited).unwrap();
    assert_eq!(
        check_with(&reuse, super::Cache::Default),
        Vec::<&str>::new()
    );
    // Refresh reruns it and replaces the entry
    assert_eq!(check_with(&root, super::Cache::Refresh), Vec::<&str>::new());
    assert_eq!(cache_of(&root, "m").as_deref(), Some(expanded));
    // A failed provider leaves the cache file alone
    let failing = generative("pure", "    return record_from_columns([])");
    std::fs::write(root.join("m.lf"), failing).unwrap();
    assert_eq!(check_with(&root, super::Cache::Refresh), ["L058"]);
    assert_eq!(cache_of(&root, "m").as_deref(), Some(expanded));
}
