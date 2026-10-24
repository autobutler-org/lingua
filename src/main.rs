use std::process::ExitCode;

use lingua::ast::{Module, dump};
use lingua::build::{self, arch::amd64, lir};
use lingua::lex::{TokenKind, lex_source};
use lingua::parse::{Diagnostic, parse_source};

const USAGE: &str = "usage: lingua <command> [args]

commands:
  lex <file.lf>     print the tokens in a source file, one per line
  parse <file.lf>   print the untyped AST of a source file as an S-expression
  check <file.lf>   check a source file's syntax and print `parsed` when it is clean
  build <file.lf>   build a static linux/amd64 executable at ./lingua-out
  lir <file.lf>     print the LIR that `build` lowers a source file to
  asm <file.lf>     print the amd64 instructions that `build` selects for a source file

`build` supports only `fn main() -> Nothing` with `log(\"...\")` statements so far.";

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
        ["parse", path] => with_module(path, |module| {
            print!("{}", dump(module));
            ExitCode::SUCCESS
        }),
        ["parse"] => usage_error("parse needs a file, e.g. `lingua parse tests/corpus/order.lf`"),
        ["check", path] => with_module(path, |_| {
            println!("parsed");
            ExitCode::SUCCESS
        }),
        ["check"] => usage_error("check needs a file, e.g. `lingua check tests/corpus/order.lf`"),
        ["build", args @ ..] => build_command(args),
        ["lir", path] => with_module(path, |module| match build::lower(module) {
            Ok(program) => {
                print!("{}", lir::dump(&program.functions));
                ExitCode::SUCCESS
            }
            Err(diagnostics) => report_all(path, &diagnostics),
        }),
        ["lir"] => usage_error("lir needs a file, e.g. `lingua lir tests/e2e/hello.lf`"),
        ["asm", path] => with_module(path, |module| match build::select(module) {
            Ok(functions) => {
                print!(
                    "{}",
                    functions
                        .iter()
                        .map(|progs| amd64::dump(progs))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                ExitCode::SUCCESS
            }
            Err(diagnostics) => report_all(path, &diagnostics),
        }),
        ["asm"] => usage_error("asm needs a file, e.g. `lingua asm tests/e2e/hello.lf`"),
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

const HOST_IS_LINUX_AMD64: bool = cfg!(all(target_os = "linux", target_arch = "x86_64"));

fn build_command(args: &[&str]) -> ExitCode {
    match args {
        [path] if !path.starts_with('-') => with_module(path, |module| build_module(path, module)),
        [] => usage_error("build needs a file, e.g. `lingua build hello.lf`"),
        _ => match args.iter().find(|arg| arg.starts_with('-')) {
            Some(&"--dynamic") => usage_error(
                "`--dynamic` is not implemented yet; drop it to build a static linux/amd64 executable",
            ),
            Some(flag) => usage_error(&format!("unknown flag `{flag}` for build")),
            None => usage_error("build takes one file so far, e.g. `lingua build hello.lf`"),
        },
    }
}

fn build_module(path: &str, module: &Module) -> ExitCode {
    let image = match build::build(module) {
        Ok(image) => image,
        Err(build::Error::Diagnostics(diagnostics)) => return report_all(path, &diagnostics),
        Err(build::Error::Internal(message)) => {
            eprintln!(
                "error: {message}
this is a bug in lingua; please report it with `{path}`"
            );
            return ExitCode::FAILURE;
        }
    };
    if let Err(err) = build::write_output(std::path::Path::new("."), &image) {
        eprintln!(
            "error: cannot write `./lingua-out`: {err}
check that the current directory is writable"
        );
        return ExitCode::FAILURE;
    }
    if !HOST_IS_LINUX_AMD64 {
        eprintln!(
            "note: ./lingua-out is a linux/amd64 executable; run it here with `docker run --rm --platform linux/amd64 -v \"$PWD\":/w -w /w debian:bookworm-slim ./lingua-out`"
        );
    }
    ExitCode::SUCCESS
}

// Parses `path` and runs `on_success` on a clean module, or prints every diagnostic to stderr
fn with_module(path: &str, on_success: impl FnOnce(&Module) -> ExitCode) -> ExitCode {
    let Some(bytes) = read(path) else {
        return ExitCode::FAILURE;
    };
    let key = std::path::Path::new(path)
        .file_stem()
        .map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
    match parse_source(&bytes, &key) {
        Ok(module) => on_success(&module),
        Err(diagnostics) => report_all(path, &diagnostics),
    }
}

fn report_all(path: &str, diagnostics: &[Diagnostic]) -> ExitCode {
    for diagnostic in diagnostics {
        report(path, diagnostic);
    }
    ExitCode::FAILURE
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
