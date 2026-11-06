use std::path::{Path, PathBuf};
use std::process::ExitCode;

use lingua::ast::dump;
use lingua::build::{self, arch::amd64, lir};
use lingua::check::{self, Cache, Diagnostic, Options};
use lingua::lex::{TokenKind, lex_source};
use lingua::parse::parse_source;
use lingua::tir::dump as dump_tir;

const USAGE: &str = "usage: lingua <command> [args]

commands:
  lex <file.lf>       print the tokens in a source file, one per line
  parse <file.lf>     print the untyped AST of a source file as an S-expression
  check [PATH ...]    typecheck modules and their imports; silent when clean
  check --explain L031
                      print the catalog text for a diagnostic code
  fmt [--check] [--source-root PATH] [PATH ...]
                      rewrite modules in canonical layout; --check lists files that would change
  expand [PATH ...]   print the types that generative types and `provide` expand to, and
                      each declaration that holds a macro splice, spliced
  pack FILE           print FILE for a prompt, its imports' declarations and signatures first
  build [PATH ...]    check, then build a static linux/amd64 executable at ./lingua-out
  tir PATH            print the typed IR of PATH as an S-expression
  lir PATH            print the LIR that `build` lowers PATH to
  asm PATH            print the amd64 instructions that `build` selects for PATH

PATH is a `.lf` file directly in the source root, or the source root itself (every module
in it); it defaults to `.`. The source root is the nearest directory upward with a
`lingua.toml`, else the directory of the path.

flags for check, expand, pack, and build:
  --source-root PATH  use PATH as the source root instead of the upward walk
  --refresh           run every provider effect again and replace the provider cache
  --offline           run no provider effect; a type missing from the cache is L071

flags for build:
  --profile debug|release
                      debug writes ./lingua-out; release writes ./lingua-out-release.
                      Name each once. Omit the flag for one debug job at ./lingua-out.
  --serve             listen in this source root until `lingua build --stop`
  --stop              stop the runner listening in this source root

A build in a source root where `--serve` is listening submits its jobs there. With no
runner, it builds in this process. A failed job does not stop the runner.

Function examples run on the interpreter during check and build, then package examples; a
failing one is a diagnostic. `const`, `comptime`, and comptime bounds fold on the same
interpreter. A generative type (`type Name` whose body starts with an effect) and each
`provide` in a package `comptime` block expand to an ordinary `type` declaration, cached in
`provided/<module>.lf` under the source root and reused on later runs. `record_from_columns`
builds a record type from a `List<Column>`; `record_from_fields` and `sum_from_variants` build
a record or a sum whose field and payload types are type text, such as \"Int in 1 through 100\".
A macro is a function that returns `Expr`, built with `quote` and `unquote`; `comptime f(...)`
splices its tree into the caller before the caller is checked. `fs.read`, `http_get`,
`catalog.columns`, `schema`, and `types_from_openapi` are reported as not supported yet.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["-h" | "--help"] | [_, "-h" | "--help"] => {
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
            with_flags(
                "check",
                rest,
                false,
                |paths, options, _profiles| match check::check(&paths, &options) {
                    Ok(_) => ExitCode::SUCCESS,
                    Err(err) => build_error(err.into()),
                },
            )
        }
        ["fmt", rest @ ..] => lingua::fmt::command(rest),
        ["expand", rest @ ..] => {
            with_flags(
                "expand",
                rest,
                false,
                |paths, options, _profiles| match check::expand(&paths, &options) {
                    Ok((text, diagnostics)) => {
                        print!("{text}");
                        if diagnostics.is_empty() {
                            ExitCode::SUCCESS
                        } else {
                            report(&diagnostics)
                        }
                    }
                    Err(err) => build_error(err.into()),
                },
            )
        }
        ["pack", rest @ ..] => {
            with_flags(
                "pack",
                rest,
                false,
                |paths, options, _profiles| match check::pack(&paths, &options) {
                    Ok((out, clean)) => {
                        match std::io::Write::write_all(&mut std::io::stdout(), &out) {
                            Ok(()) if clean => ExitCode::SUCCESS,
                            _ => ExitCode::FAILURE,
                        }
                    }
                    Err(err) => build_error(err.into()),
                },
            )
        }
        ["build", rest @ ..] => with_flags("build", rest, true, |paths, options, profiles| {
            build_paths(&paths, &options, &profiles)
        }),
        ["tir", path] => dump_program(path, |program| Ok(dump_tir(program))),
        ["tir"] => usage_error("tir needs a path, e.g. `lingua tir tests/e2e/hello.lf`"),
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
// and `--profile`.
fn with_flags(
    command: &str,
    args: &[&str],
    build: bool,
    run: impl FnOnce(Vec<PathBuf>, Options, Vec<build::Profile>) -> ExitCode,
) -> ExitCode {
    let mut options = Options::default();
    let mut paths = Vec::new();
    let mut profiles = Vec::new();
    let (mut refresh, mut offline) = (false, false);
    let (mut serve, mut stop) = (false, false);
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
            "--profile" if build => match args.next() {
                Some(name) => match build::Profile::parse(name) {
                    Some(profile) if profiles.contains(&profile) => {
                        return usage_error(&format!(
                            "`--profile {name}` was given twice; name each profile once"
                        ));
                    }
                    Some(profile) => profiles.push(profile),
                    None => {
                        return usage_error(&format!(
                            "`--profile {name}` is not a profile; use `debug` or `release`"
                        ));
                    }
                },
                None => return usage_error("`--profile` needs `debug` or `release`"),
            },
            "--serve" if build => serve = true,
            "--stop" if build => stop = true,
            flag if flag.starts_with('-') => {
                return usage_error(&format!("unknown flag `{flag}` for {command}"));
            }
            path => paths.push(PathBuf::from(path)),
        }
    }
    options.cache = match (refresh, offline) {
        (true, true) => {
            return usage_error("--refresh and --offline cannot be used together; pick one");
        }
        (true, false) => Cache::Refresh,
        (false, true) => Cache::Offline,
        (false, false) => Cache::Default,
    };
    if serve && stop {
        return usage_error("`--serve` and `--stop` cannot be used together; pick one");
    }
    if serve {
        return serve_command(&paths, &options, &profiles);
    }
    if stop {
        return stop_command(&paths, &options, &profiles);
    }
    run(paths, options, profiles)
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
        build::Error::Rendered(text) => {
            eprint!("{text}");
            ExitCode::FAILURE
        }
    }
}

fn shown_output(path: &Path) -> String {
    let text = path.display().to_string();
    if path.is_relative() && !text.starts_with('.') {
        format!("./{text}")
    } else {
        text
    }
}

fn runner_flags(
    which: &str,
    paths: &[PathBuf],
    options: &Options,
    profiles: &[build::Profile],
) -> Result<(), ExitCode> {
    if !paths.is_empty() {
        let message = if which == "--serve" {
            "`--serve` waits for jobs; run `lingua build PATH` to submit one"
        } else {
            "`--stop` does not take a path"
        };
        return Err(usage_error(message));
    }
    if !profiles.is_empty() {
        return Err(usage_error(&format!(
            "`{which}` does not take `--profile`; the submitted job names its profile"
        )));
    }
    if options.cache != Cache::Default {
        return Err(usage_error(&format!(
            "`{which}` does not take `--refresh` or `--offline`; the submitted job names the cache policy"
        )));
    }
    Ok(())
}

fn source_root(options: &Options) -> Result<PathBuf, ExitCode> {
    match check::resolve(&[], options.source_root.as_deref()) {
        Ok((root, _, _)) => Ok(root),
        Err(err) => Err(build_error(err.into())),
    }
}

fn serve_exit(err: build::serve::ServeError) -> ExitCode {
    match err {
        build::serve::ServeError::Usage(message) => usage_error(&message),
        build::serve::ServeError::Broken(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn serve_command(paths: &[PathBuf], options: &Options, profiles: &[build::Profile]) -> ExitCode {
    if let Err(code) = runner_flags("--serve", paths, options, profiles) {
        return code;
    }
    let root = match source_root(options) {
        Ok(root) => root,
        Err(code) => return code,
    };
    let listener = match build::serve::bind(&root) {
        Ok(listener) => listener,
        Err(err) => return serve_exit(err),
    };
    println!(
        "listening for jobs at {}\n`lingua build` in this source root submits here. Stop with `lingua build --stop`.",
        listener.path().display()
    );
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut runner = build::Runner::new();
    match listener.run(&mut runner) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => serve_exit(err),
    }
}

fn stop_command(paths: &[PathBuf], options: &Options, profiles: &[build::Profile]) -> ExitCode {
    if let Err(code) = runner_flags("--stop", paths, options, profiles) {
        return code;
    }
    let root = match source_root(options) {
        Ok(root) => root,
        Err(code) => return code,
    };
    match build::serve::stop(&root) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => usage_error(&format!(
            "no runner is listening in {}; start one with `lingua build --serve`",
            root.display()
        )),
        Err(err) => serve_exit(err),
    }
}

fn take_standing(
    jobs: &[build::Job],
) -> Result<Option<Vec<build::Outcome>>, build::serve::ServeError> {
    let job = &jobs[0];
    let Ok((root, _, _)) = check::resolve(&job.paths, job.options.source_root.as_deref()) else {
        return Ok(None);
    };
    build::serve::submit(&root, jobs)
}

fn build_paths(paths: &[PathBuf], options: &Options, profiles: &[build::Profile]) -> ExitCode {
    let profiles = if profiles.is_empty() {
        vec![build::Profile::Debug]
    } else {
        profiles.to_vec()
    };
    let jobs: Vec<build::Job> = profiles
        .iter()
        .map(|profile| {
            build::Job::new(
                paths.to_vec(),
                options.clone(),
                *profile,
                profile.output_name(),
            )
        })
        .collect();
    let outputs: Vec<PathBuf> = jobs.iter().map(|job| job.output.clone()).collect();
    match take_standing(&jobs) {
        Ok(Some(outcomes)) => report_outcomes(&outputs, outcomes),
        Ok(None) => {
            let mut runner = build::Runner::new();
            for job in jobs {
                runner.submit(job);
            }
            report_outcomes(&outputs, runner.run())
        }
        Err(err) => serve_exit(err),
    }
}

fn report_outcomes(outputs: &[PathBuf], outcomes: Vec<build::Outcome>) -> ExitCode {
    if outcomes.is_empty() {
        eprintln!(
            "error: lingua build submitted no job\nthis is a bug in lingua; please report it"
        );
        return ExitCode::FAILURE;
    }
    if outputs.len() != outcomes.len() {
        eprintln!(
            "error: lingua build lost a job outcome\nthis is a bug in lingua; please report it"
        );
        return ExitCode::FAILURE;
    }
    let many = outcomes.len() > 1;
    // 0 success, 1 a job failed, 2 usage. Usage outranks a diagnostic.
    let mut worst = 0u8;
    let mut reported_usage = false;
    for (output, outcome) in outputs.iter().zip(outcomes) {
        match outcome {
            build::Outcome::Wrote(path) => {
                if !HOST_IS_LINUX_AMD64 {
                    let shown = shown_output(&path);
                    eprintln!(
                        "note: {shown} is a linux/amd64 executable; run it here with `docker run --rm --platform linux/amd64 -v \"$PWD\":/w -w /w debian:bookworm-slim {shown}`"
                    );
                }
            }
            build::Outcome::Failed(build::Error::Usage(message)) => {
                if !reported_usage {
                    let _ = usage_error(&message);
                    reported_usage = true;
                }
                worst = 2;
            }
            build::Outcome::Failed(err) => {
                if many {
                    eprintln!("{}:", shown_output(output));
                }
                let _ = build_error(err);
                worst = worst.max(1);
            }
            build::Outcome::Unwritten { path, error } => {
                let shown = shown_output(&path);
                eprintln!(
                    "error: cannot write `{shown}`: {error}\ncheck that the current directory is writable"
                );
                worst = worst.max(1);
            }
        }
    }
    match worst {
        0 => ExitCode::SUCCESS,
        2 => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    }
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
