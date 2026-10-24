use std::process::ExitCode;

use lingua::lex::{TokenKind, lex_source};

const USAGE: &str = "usage: lingua <command> [args]

commands:
  lex <file.lf>   print the tokens in a source file, one per line";

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
        [] => usage_error("no command given"),
        [command, ..] => usage_error(&format!("unknown command `{command}`")),
    }
}

fn usage_error(message: &str) -> ExitCode {
    eprintln!("error: {message}\n\n{USAGE}");
    ExitCode::from(2)
}

fn lex(path: &str) -> ExitCode {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("error: cannot read `{path}`: {err}\ncheck the path and try again");
            return ExitCode::FAILURE;
        }
    };
    match lex_source(&bytes) {
        Ok(tokens) => {
            for token in tokens.iter().filter(|token| token.kind != TokenKind::Eof) {
                println!("{token}");
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!(
                "{}: {}\n  --> {path}:{}:{}",
                err.code, err.message, err.location.line, err.location.column
            );
            ExitCode::FAILURE
        }
    }
}
