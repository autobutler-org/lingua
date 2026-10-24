//! Parses every `.lf` under `tests/corpus` and `examples/parse` and compares against its golden file.
//! An `.ast` golden holds the S-expression dump; an `.err` golden holds the diagnostics.
//! Run with `BLESS=1` to rewrite the goldens from the current output.

use std::path::{Path, PathBuf};

use lingua::ast::dump;
use lingua::parse::parse_source;

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

fn render(path: &Path) -> (&'static str, String) {
    let key = path.file_stem().unwrap().to_str().unwrap();
    match parse_source(&std::fs::read(path).unwrap(), key) {
        Ok(module) => ("ast", dump(&module)),
        Err(diagnostics) => {
            let mut lines = String::new();
            for diagnostic in &diagnostics {
                lines.push_str(&diagnostic.to_string());
                lines.push('\n');
            }
            ("err", lines)
        }
    }
}

fn check_goldens(dir: &str) {
    let bless = std::env::var("BLESS").is_ok_and(|value| value == "1");
    let examples = sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join(dir));
    assert!(!examples.is_empty(), "no .lf files found under {dir}");

    let mut failures = Vec::new();
    for source in &examples {
        let (extension, actual) = render(source);
        let golden = source.with_extension(extension);
        if bless {
            std::fs::write(&golden, &actual).unwrap();
            continue;
        }
        match std::fs::read_to_string(&golden) {
            Ok(expected) if expected == actual => {}
            Ok(expected) => failures.push(format!(
                "{}: output differs\n--- expected\n{expected}--- actual\n{actual}",
                golden.display()
            )),
            Err(_) => failures.push(format!(
                "{}: missing golden (parsing produced .{extension})",
                golden.display()
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "{}\nrerun with BLESS=1 to update the goldens",
        failures.join("\n")
    );
}

#[test]
fn corpus_parses_and_matches_goldens() {
    for source in sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus")) {
        let (extension, output) = render(&source);
        assert_eq!(
            extension,
            "ast",
            "{} must parse:\n{output}",
            source.display()
        );
    }
    check_goldens("tests/corpus");
}

#[test]
fn parse_examples_match_goldens() {
    check_goldens("examples/parse");
}
