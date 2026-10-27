//! The source-generation round trips and `lingua fmt` (docs/lingua1.md, Formatter and
//! Compilation pipeline, Source generation): `lex(unlex(t)) == t` on every lex fixture,
//! `parse(unparse(a)) == a` ignoring spans on every parse fixture, and `fmt` is idempotent
//! and a no-op on the corpus, the e2e programs, and the Euler solutions.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use lingua::ast::dump;
use lingua::fmt::format_source;
use lingua::lex::{Token, TokenKind, lex_source, unlex};
use lingua::parse::{parse_source, parse_tokens, unparse};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

// Every `.lf` under `dir`, recursively
fn sources(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            found.extend(sources(&path));
        } else if path.extension().is_some_and(|ext| ext == "lf") {
            found.push(path);
        }
    }
    found.sort();
    found
}

// The `.tokens` golden form, except a newline's column: it is where the line's text ended,
// and unlex drops comments and trailing whitespace
fn token_lines(tokens: &[Token]) -> Vec<String> {
    tokens
        .iter()
        .filter(|token| token.kind != TokenKind::Eof)
        .map(|token| match token.kind {
            TokenKind::Newline => format!("{}:_ Newline", token.location.line),
            _ => token.to_string(),
        })
        .collect()
}

#[test]
fn unlex_round_trips_every_lex_fixture() {
    let mut checked = 0;
    for path in sources(&root().join("examples/lex")) {
        let Ok(tokens) = lex_source(&std::fs::read(&path).unwrap()) else {
            continue;
        };
        let source = unlex(&tokens);
        let again = lex_source(source.as_bytes())
            .unwrap_or_else(|err| panic!("{}: unlex output does not lex: {err}", path.display()));
        assert_eq!(
            token_lines(&again),
            token_lines(&tokens),
            "{}:\n{source}",
            path.display()
        );
        assert!(
            source.lines().all(|line| line == line.trim_end()),
            "{}: trailing whitespace",
            path.display()
        );
        checked += 1;
    }
    assert!(checked > 0);
}

#[test]
fn unlex_reescapes_strings_and_holes() {
    let tokens = lex_source(r#"x = "a\"\\\n\t{{}}{f("{y}")}é" "#.as_bytes()).unwrap();
    let source = unlex(&tokens);
    assert_eq!(source, "x = \"a\\\"\\\\\\n\\t{{}}{f(\"{y}\")}é\"");
    assert_eq!(
        token_lines(&lex_source(source.as_bytes()).unwrap()),
        token_lines(&tokens)
    );
}

fn parse_fixtures() -> Vec<PathBuf> {
    let mut found = sources(&root().join("examples/parse"));
    found.extend(sources(&root().join("tests/corpus")));
    found
}

#[test]
fn unparse_round_trips_every_parse_fixture() {
    let mut checked = 0;
    for path in parse_fixtures() {
        let key = path.file_stem().unwrap().to_str().unwrap();
        let Ok(module) = parse_source(&std::fs::read(&path).unwrap(), key) else {
            continue;
        };
        let tokens = unparse(&module);
        let reparsed = parse_tokens(unparse(&module), key).unwrap_or_else(|diagnostics| {
            panic!(
                "{}: unparse output does not parse: {diagnostics:?}\n{}",
                path.display(),
                unlex(&tokens)
            )
        });
        assert_eq!(dump(&reparsed), dump(&module), "{}", path.display());
        // The composition: parse(lex(unlex(unparse(a)))) == a
        let source = unlex(&tokens);
        let relexed = parse_source(source.as_bytes(), key).unwrap();
        assert_eq!(
            dump(&relexed),
            dump(&module),
            "{}:\n{source}",
            path.display()
        );
        // unparse gives each token the place unlex puts it
        assert_eq!(
            token_lines(&lex_source(source.as_bytes()).unwrap()),
            token_lines(&tokens),
            "{}",
            path.display()
        );
        checked += 1;
    }
    assert!(checked > 0);
}

fn format(path: &Path) -> String {
    format_source(&std::fs::read(path).unwrap())
        .unwrap_or_else(|diagnostics| panic!("{}: {diagnostics:?}", path.display()))
}

// e2e cases whose source `fmt` rewrites on purpose: two blank lines before a body and one
// inside it, a one-line list with a trailing comma, and parentheses that spell out the
// `(a / b) * b + a % b` identity, which the AST does not keep. Their `.lir` sites name columns.
const NOT_CANONICAL: [&str; 3] = [
    "tests/e2e/layout.lf",
    "tests/e2e/list_basics.lf",
    "tests/e2e/int_div_rem.lf",
];

fn canonical_files() -> Vec<PathBuf> {
    let mut found = sources(&root().join("tests/corpus"));
    found.extend(
        sources(&root().join("tests/e2e"))
            .into_iter()
            .filter(|path| !NOT_CANONICAL.iter().any(|skip| path.ends_with(skip))),
    );
    found.extend(sources(&root().join("euler")));
    found
}

#[test]
fn fmt_is_a_no_op_on_the_corpus_e2e_and_euler() {
    let mut changed = Vec::new();
    for path in canonical_files() {
        let source = std::fs::read_to_string(&path).unwrap();
        let formatted = format(&path);
        if formatted != source {
            changed.push(format!(
                "{}:\n--- source\n{source}--- formatted\n{formatted}",
                path.display()
            ));
        }
    }
    assert!(changed.is_empty(), "{}", changed.join("\n"));
}

#[test]
fn fmt_is_idempotent() {
    let mut paths = canonical_files();
    paths.extend(NOT_CANONICAL.iter().map(|path| root().join(path)));
    paths.extend(
        parse_fixtures()
            .into_iter()
            .filter(|path| parse_source(&std::fs::read(path).unwrap(), "x").is_ok()),
    );
    for path in paths {
        let once = format(&path);
        let twice = format_source(once.as_bytes())
            .unwrap_or_else(|d| panic!("{}: {d:?}\n{once}", path.display()));
        assert_eq!(twice, once, "{}", path.display());
    }
}

#[test]
fn fmt_puts_layout_in_canonical_form_and_keeps_comments() {
    let source = std::fs::read_to_string(root().join("tests/e2e/layout.lf")).unwrap();
    let expected = "\
// A comment before the first declaration
fn add(left: Int, right: Int) -> Int
  pure
  example \"small\"
    left: 2
    right: 3
    -> 5
  end

  // comments and blank lines inside a body are layout
  return left + right // trailing comment
end fn add

fn main() -> Nothing
  pure
  example \"main may have an example\"
    -> nothing
  end

  let total = add(
    left: 40,
    right: 2,
  )
  log(\"total {total}\")
  log(
    \"broken call\",
  )
  log(\"a // b is not a comment inside a string\")
  return nothing
end fn main
";
    assert_eq!(format_source(source.as_bytes()).unwrap(), expected);
}

#[test]
fn fmt_normalizes_spacing_breaks_and_comments() {
    let source = "\
// header

// about Pair
type Pair
\tleft :Int   // the left one
  right: List< Int,>
end type Pair
fn f(a:Int,b :Int,) -> Int
    pure


    example \"x\"
        a: 1
        b:
          [1,
           2]
        -> 3
    end
    example \"y\"
      a: 1
      b: 2
      -> exception \"no\"
    end
    let x = ( a+b )*2 // why
    if x>1
        // inside if
        return f(a : 1,b: 2)
    else
          return -(x)
    end
    match x
      1 ->
        return 1
      _ -> return 0
    end
end fn f
const k = -(1 + 2)
// at the end
";
    let expected = "\
// header

// about Pair
type Pair
  left: Int // the left one
  right: List<Int>
end type Pair

fn f(a: Int, b: Int) -> Int
  pure
  example \"x\"
    a: 1
    b:
      [
        1, 2,
      ]
    -> 3
  end
  example \"y\"
    a: 1
    b: 2
    -> exception \"no\"
  end

  let x = (a + b) * 2 // why
  if x > 1
    // inside if
    return f(a: 1, b: 2)
  else
    return -x
  end
  match x
    1 ->
      return 1
    _ -> return 0
  end
end fn f

const k = -(1 + 2)
// at the end
";
    let formatted = format_source(source.as_bytes()).unwrap();
    assert_eq!(formatted, expected);
    assert_eq!(format_source(formatted.as_bytes()).unwrap(), formatted);
}

fn assert_canonical(source: &str) {
    let once = format_source(source.as_bytes()).unwrap_or_else(|d| panic!("{d:?}\n{source}"));
    assert_eq!(once, source);
    assert_eq!(format_source(once.as_bytes()).unwrap(), once);
    let module = parse_source(source.as_bytes(), "t").unwrap();
    let again = parse_tokens(unparse(&module), "t").unwrap();
    assert_eq!(dump(&again), dump(&module));
}

#[test]
fn fmt_preserves_fs_read_and_catalog_columns_argument_names() {
    let named = "\
const bytes = fs.read(path: \"openapi.json\")

const columns = catalog.columns(table: \"orders\")
";
    assert_canonical(named);
    let dumped = dump(&parse_source(named.as_bytes(), "t").unwrap());
    assert!(
        dumped.contains("(fs-read (text (literal \"openapi.json\")) true false)"),
        "{dumped}"
    );
    assert!(
        dumped.contains("(catalog-columns (text (literal \"orders\")) true false)"),
        "{dumped}"
    );

    let positional = "\
const bytes = fs.read(\"openapi.json\")

const columns = catalog.columns(\"orders\")
";
    assert_canonical(positional);
    let dumped = dump(&parse_source(positional.as_bytes(), "t").unwrap());
    assert!(
        dumped.contains("(fs-read (text (literal \"openapi.json\")) false false)"),
        "{dumped}"
    );
    assert!(
        dumped.contains("(catalog-columns (text (literal \"orders\")) false false)"),
        "{dumped}"
    );

    let broken = "\
const bytes = fs.read(
  path: \"openapi.json\"
)

const columns = catalog.columns(
  table: \"orders\"
)
";
    assert_canonical(broken);
    let dumped = dump(&parse_source(broken.as_bytes(), "t").unwrap());
    assert!(
        dumped.contains("(fs-read (text (literal \"openapi.json\")) true true)"),
        "{dumped}"
    );
    assert!(
        dumped.contains("(catalog-columns (text (literal \"orders\")) true true)"),
        "{dumped}"
    );
}

fn packed_line(items: &[&str]) -> String {
    let mut line = String::from("  ");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            line.push(' ');
        }
        line.push_str(item);
        line.push(',');
    }
    line
}

#[test]
fn fmt_packs_a_broken_list_of_literals_and_leaves_other_lists_alone() {
    let small = format!("const xs = [\n{}\n]\n", packed_line(&["1", "1", "1"]));
    assert!(small.lines().nth(1).unwrap().chars().count() <= 80);
    assert_canonical(&small);

    // 26 single-digit ints fill a line to column 79; the 27th starts the next line
    let first = packed_line(&["1"; 26]);
    let rest = packed_line(&["1"; 4]);
    assert_eq!(first.chars().count(), 79);
    assert_canonical(&format!("const xs = [\n{first}\n{rest}\n]\n"));

    assert_canonical(
        "\
const xs = [
  f(1),
  2,
]
",
    );
    assert_canonical(
        "\
const xs = [
  [1],
  [2],
]
",
    );

    let long = format!(
        "const xs = [{}]\n",
        std::iter::repeat_n("1", 40).collect::<Vec<_>>().join(", ")
    );
    assert!(long.lines().next().unwrap().chars().count() > 80);
    assert_eq!(long.matches('\n').count(), 1);
    assert_canonical(&long);

    assert_canonical(
        "\
const xs = [
  true, false, nothing, \"hi\", -3,
]
",
    );

    // `"é"` is 3 scalars and 4 bytes; scalar columns fit 15 on the line, bytes would not
    let marks = packed_line(&["\"é\""; 15]);
    let tail = packed_line(&["\"é\""]);
    assert!(marks.chars().count() <= 80);
    assert_canonical(&format!("const xs = [\n{marks}\n{tail}\n]\n"));
}

#[test]
fn fmt_rejects_a_file_that_does_not_parse() {
    let diagnostics = format_source(b"type T\n  A\nend type U\n").unwrap_err();
    assert_eq!(diagnostics[0].code, "L002");
}

// The CLI

fn workdir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("fmt_cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("lingua.toml"), "version = \"lingua1\"\n").unwrap();
    dir
}

fn lingua(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_lingua"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

const MESSY: &str = "const  x =1+2\n";
const CLEAN: &str = "const x = 1 + 2\n";

#[test]
fn fmt_check_lists_files_that_would_change_and_writes_nothing() {
    let dir = workdir("check");
    std::fs::write(dir.join("messy.lf"), MESSY).unwrap();
    std::fs::write(dir.join("clean.lf"), CLEAN).unwrap();
    let output = lingua(&dir, &["fmt", "--check"]);
    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "messy.lf\n");
    assert!(
        text(&output.stderr).contains("run `lingua fmt`"),
        "{}",
        text(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("messy.lf")).unwrap(),
        MESSY
    );

    let output = lingua(&dir, &["fmt", "--check", "clean.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "");
}

#[test]
fn fmt_rewrites_files_in_place() {
    let dir = workdir("rewrite");
    std::fs::write(dir.join("messy.lf"), MESSY).unwrap();
    let output = lingua(&dir, &["fmt", "messy.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(
        std::fs::read_to_string(dir.join("messy.lf")).unwrap(),
        CLEAN
    );
    let output = lingua(&dir, &["fmt", "--check", "."]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
}

#[test]
fn fmt_reports_a_file_that_does_not_parse_and_leaves_it() {
    let dir = workdir("broken");
    let broken = "const x = \n";
    std::fs::write(dir.join("broken.lf"), broken).unwrap();
    let output = lingua(&dir, &["fmt", "broken.lf"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        text(&output.stderr).contains("L001"),
        "{}",
        text(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("broken.lf")).unwrap(),
        broken
    );
}

#[test]
fn fmt_usage_errors_exit_2() {
    let dir = workdir("usage");
    let output = lingua(&dir, &["fmt", "--bogus"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(text(&output.stderr).contains("unknown flag `--bogus`"));
    let output = lingua(&dir, &["fmt", "missing.lf"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(text(&output.stderr).contains("does not exist"));
    let output = lingua(&dir, &["fmt", "-h"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(text(&output.stdout).contains("fmt [--check]"));
}
