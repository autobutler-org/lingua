//! Lexes every `examples/lex/**/*.lf` and compares against its golden file.
//! A `.tokens` golden holds the token lines; a `.err` golden holds the diagnostic.
//! Run with `BLESS=1` to rewrite the goldens from the current output.

use std::path::{Path, PathBuf};

use lingua::lex::{TokenKind, lex_source};

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
    match lex_source(&std::fs::read(path).unwrap()) {
        Ok(tokens) => {
            let mut lines = String::new();
            for token in tokens.iter().filter(|token| token.kind != TokenKind::Eof) {
                lines.push_str(&token.to_string());
                lines.push('\n');
            }
            ("tokens", lines)
        }
        Err(err) => ("err", format!("{err}\n")),
    }
}

#[test]
fn lex_examples_match_goldens() {
    let bless = std::env::var("BLESS").is_ok_and(|value| value == "1");
    let examples = sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/lex"));
    assert!(
        !examples.is_empty(),
        "no .lf files found under examples/lex"
    );

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
                "{}: missing golden (lexing produced .{extension})",
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
