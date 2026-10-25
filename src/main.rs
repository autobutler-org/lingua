use std::path::PathBuf;
use std::process::ExitCode;

use lingua::ast::dump;
use lingua::build::{self, arch::amd64, lir};
use lingua::check::{self, Diagnostic, Options};
use lingua::lex::{TokenKind, lex_source};
use lingua::parse::parse_source;

const USAGE: &str = "usage: lingua <command> [args]

commands:
  lex <file.lf>       print the tokens in a source file, one per line
  parse <file.lf>     print the untyped AST of a source file as an S-expression
  check [PATH ...]    typecheck modules and their imports; silent when clean
  check --explain L031
                      print the catalog text for a diagnostic code
  build [PATH ...]    check, then build a static linux/amd64 executable at ./lingua-out
  lir PATH            print the LIR that `build` lowers PATH to
  asm PATH            print the amd64 instructions that `build` selects for PATH

PATH is a `.lf` file directly in the source root, or the source root itself (every module
in it); it defaults to `.`. The source root is the nearest directory upward with a
`lingua.toml`, else the directory of the path.

flags for check and build:
  --source-root PATH  use PATH as the source root instead of the upward walk
  --refresh, --offline
                      provider cache policy (no provider runs until the provider cache lands)

Function examples run on the interpreter during check and build, then package examples; a
failing one is a diagnostic. `const`, `comptime`, and comptime bounds fold on the same
interpreter. Generative types, package `comptime` blocks, effectful consts, and macros are
reported as not supported yet.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["-h" | "--help"] => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        ["--version"] => {
            println!("lingua1");
            ExitCode::SUCCESS
        }
        ["lex", path] => lex(path),
        ["lex"] => usage_error("lex needs a file, e.g. `lingua lex examples/lex/money.lf`"),
        ["parse", path] => parse(path),
        ["parse"] => usage_error("parse needs a file, e.g. `lingua parse tests/corpus/order.lf`"),
        ["check", "--explain", code] => match check::explain(code) {
            Some(text) => {
                println!("{code}: {text}");
                ExitCode::SUCCESS
            }
            None => usage_error(&format!(
                "`{code}` is not an allocated diagnostic code; codes look like `L031`"
            )),
        },
        ["check", "--explain"] => {
            usage_error("--explain needs a code, e.g. `lingua check --explain L031`")
        }
        ["check", rest @ ..] => {
            with_flags("check", rest, false, |paths, options| {
                match check::check(&paths, &options) {
                    Ok(_) => ExitCode::SUCCESS,
                    Err(err) => build_error(err.into()),
                }
            })
        }
        ["build", rest @ ..] => with_flags("build", rest, true, |paths, options| {
            build_paths(&paths, &options)
        }),
        ["lir", path] => dump_program(path, |program| {
            let lowered = build::lower(program)?;
            Ok(lir::dump(&lowered.functions, &lowered.sites))
        }),
        ["lir"] => usage_error("lir needs a path, e.g. `lingua lir tests/e2e/hello.lf`"),
        ["asm", path] => dump_program(path, |program| {
            Ok(build::select(program)?
                .iter()
                .map(|progs| amd64::dump(progs))
                .collect::<Vec<_>>()
                .join("\n"))
        }),
        ["asm"] => usage_error("asm needs a path, e.g. `lingua asm tests/e2e/hello.lf`"),
        [] => usage_error("no command given"),
        [command, ..] => usage_error(&format!("unknown command `{command}`")),
    }
}

fn usage_error(message: &str) -> ExitCode {
    eprintln!("error: {message}\n\n{USAGE}");
    ExitCode::from(2)
}

// Parses `--source-root PATH`, `--refresh`, `--offline`, and (build only) `--dynamic`
fn with_flags(
    command: &str,
    args: &[&str],
    build: bool,
    run: impl FnOnce(Vec<PathBuf>, Options) -> ExitCode,
) -> ExitCode {
    let mut options = Options::default();
    let mut paths = Vec::new();
    let (mut refresh, mut offline) = (false, false);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match *arg {
            "--source-root" => match args.next() {
                Some(root) => options.source_root = Some(PathBuf::from(root)),
                None => return usage_error("--source-root needs a directory"),
            },
            "--refresh" => refresh = true,
            "--offline" => offline = true,
            "--dynamic" if build => {
                return usage_error(
                    "`--dynamic` is not implemented yet; drop it to build a static linux/amd64 executable",
                );
            }
            flag if flag.starts_with('-') => {
                return usage_error(&format!("unknown flag `{flag}` for {command}"));
            }
            path => paths.push(PathBuf::from(path)),
        }
    }
    if refresh && offline {
        return usage_error("--refresh and --offline cannot be used together; pick one");
    }
    run(paths, options)
}

fn read(path: &str) -> Option<Vec<u8>> {
    std::fs::read(path)
        .map_err(|err| {
            eprintln!("error: cannot read `{path}`: {err}\ncheck the path and try again");
        })
        .ok()
}

fn report(diagnostics: &[Diagnostic]) -> ExitCode {
    for diagnostic in diagnostics {
        eprint!("{diagnostic}");
    }
    ExitCode::FAILURE
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
        Err(err) => report(&[Diagnostic::from_parse(
            &err.into(),
            path,
            &String::from_utf8_lossy(&bytes),
        )]),
    }
}

fn parse(path: &str) -> ExitCode {
    let Some(bytes) = read(path) else {
        return ExitCode::FAILURE;
    };
    let key = std::path::Path::new(path)
        .file_stem()
        .map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
    let source = String::from_utf8_lossy(&bytes);
    match parse_source(&bytes, &key) {
        Ok(module) => {
            print!("{}", dump(&module));
            ExitCode::SUCCESS
        }
        Err(diagnostics) => report(
            &diagnostics
                .iter()
                .map(|d| Diagnostic::from_parse(d, path, &source))
                .collect::<Vec<_>>(),
        ),
    }
}

const HOST_IS_LINUX_AMD64: bool = cfg!(all(target_os = "linux", target_arch = "x86_64"));

fn build_error(err: build::Error) -> ExitCode {
    match err {
        build::Error::Usage(message) => usage_error(&message),
        build::Error::Diagnostics(diagnostics) => report(&diagnostics),
        build::Error::Internal(message) => {
            eprintln!(
                "error: {message}\nthis is a bug in lingua; please report it with the source that caused it"
            );
            ExitCode::FAILURE
        }
    }
}

fn build_paths(paths: &[PathBuf], options: &Options) -> ExitCode {
    let image = match build::check(paths, options).and_then(|program| build::build_tir(&program)) {
        Ok(image) => image,
        Err(err) => return build_error(err),
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

// Checks `path` as a build would and prints `dump` of the program
fn dump_program(
    path: &str,
    dump: impl FnOnce(&lingua::tir::TirProgram) -> Result<String, build::Error>,
) -> ExitCode {
    match build::check(&[PathBuf::from(path)], &Options::default()).and_then(|p| dump(&p)) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(err) => build_error(err),
    }
}
