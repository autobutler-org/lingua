use std::process::ExitCode;

use lingua::ast::{Module, dump};
use lingua::lex::{TokenKind, lex_source};
use lingua::parse::{Diagnostic, parse_source};

const USAGE: &str = "usage: lingua <command> [args]

commands:
  lex <file.lf>     print the tokens in a source file, one per line
  parse <file.lf>   print the untyped AST of a source file as an S-expression
  check <file.lf>   check a source file's syntax and print `parsed` when it is clean";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["-h" | "--help"] => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        ["lex", path] => lex(path),
        ["lex"] => usage_error("lex needs a file, e.g. `lingua lex examples/lex/money.lf`"),
        ["parse", path] => with_module(path, |module| print!("{}", dump(module))),
        ["parse"] => usage_error("parse needs a file, e.g. `lingua parse tests/corpus/order.lf`"),
        ["check", path] => with_module(path, |_| println!("parsed")),
        ["check"] => usage_error("check needs a file, e.g. `lingua check tests/corpus/order.lf`"),
        [] => usage_error("no command given"),
        [command, ..] => usage_error(&format!("unknown command `{command}`")),
    }
}

fn usage_error(message: &str) -> ExitCode {
    eprintln!("error: {message}\n\n{USAGE}");
    ExitCode::from(2)
}

fn read(path: &str) -> Option<Vec<u8>> {
    std::fs::read(path)
        .map_err(|err| {
            eprintln!("error: cannot read `{path}`: {err}\ncheck the path and try again");
        })
        .ok()
}

fn lex(path: &str) -> ExitCode {
    let Some(bytes) = read(path) else {
        return ExitCode::FAILURE;
    };
    match lex_source(&bytes) {
        Ok(tokens) => {
            for token in tokens.iter().filter(|token| token.kind != TokenKind::Eof) {
                println!("{token}");
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            report(path, &err.into());
            ExitCode::FAILURE
        }
    }
}

// Parses `path`, runs `on_success` on a clean module, or prints every diagnostic to stderr
fn with_module(path: &str, on_success: impl FnOnce(&Module)) -> ExitCode {
    let Some(bytes) = read(path) else {
        return ExitCode::FAILURE;
    };
    let key = std::path::Path::new(path)
        .file_stem()
        .map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
    match parse_source(&bytes, &key) {
        Ok(module) => {
            on_success(&module);
            ExitCode::SUCCESS
        }
        Err(diagnostics) => {
            for diagnostic in &diagnostics {
                report(path, diagnostic);
            }
            ExitCode::FAILURE
        }
    }
}

fn report(path: &str, diagnostic: &Diagnostic) {
    eprintln!(
        "{}: {}\n  --> {path}:{}:{}",
        diagnostic.code, diagnostic.message, diagnostic.location.line, diagnostic.location.column
    );
    if let Some(help) = &diagnostic.help {
        eprintln!("   = help: {help}");
    }
}
