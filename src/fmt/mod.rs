//! `lingua fmt` (docs/lingua1.md, Formatter): unparse then unlex, with the input's comments
//! put back. It stops after parse: no typecheck, no examples, no provider expansion.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::process::ExitCode;

use crate::check::{self, Diagnostic};
use crate::lex::{Comment, Location, TokenKind, lex_with_comments, unlex};
use crate::parse::{self, parse_tokens, unparse};

const USAGE: &str = "usage: lingua fmt [--check] [--source-root PATH] [PATH ...]";

/// The source in canonical layout, with its comments.
///
/// # Errors
///
/// The lex or parse diagnostics; a file that does not parse is not formatted.
pub fn format_source(bytes: &[u8]) -> Result<String, Vec<parse::Diagnostic>> {
    let (tokens, comments) = lex_with_comments(bytes).map_err(|err| vec![err.into()])?;
    let input: Vec<(Location, TokenKind)> = tokens
        .iter()
        .filter(|t| t.kind != TokenKind::Newline && t.kind != TokenKind::Eof)
        .map(|t| (t.location, t.kind.clone()))
        .collect();
    let module = parse_tokens(tokens, "fmt")?;
    let output = unparse(&module);
    let text = unlex(&output);
    if comments.is_empty() {
        return Ok(text);
    }
    let output: Vec<(usize, &TokenKind)> = output
        .iter()
        .filter(|t| anchor(&t.kind))
        .map(|t| (t.location.line, &t.kind))
        .collect();
    Ok(put_back(&text, &input, &output, &comments))
}

// Tokens a comment can be placed by: unparse may drop or add parentheses and commas
fn anchor(kind: &TokenKind) -> bool {
    !matches!(
        kind,
        TokenKind::Newline
            | TokenKind::Eof
            | TokenKind::Comma
            | TokenKind::LeftParen
            | TokenKind::RightParen
    )
}

// A string's holes may be respaced, so any string matches any string
fn same(a: &TokenKind, b: &TokenKind) -> bool {
    matches!((a, b), (TokenKind::String(_), TokenKind::String(_))) || a == b
}

// How far ahead an output token looks for its input token, past ones unparse dropped (`pure`
// on a const)
const LOOKAHEAD: usize = 8;

struct Placed<'a> {
    text: &'a str,
    // A blank input line before it, or between it and its token; kept only at file level
    blank_before: bool,
    blank_after: bool,
}

/// Puts each comment back by the token it sat beside in the input. A full-line comment goes on
/// its own line before the output line of the next token, at that line's indent; a trailing
/// comment goes at the end of the output line of the token before it.
// ponytail: tokens are matched greedily within LOOKAHEAD, so a comment near a declaration
// unparse moves (an import after a declaration) lands by the nearest token that matched.
fn put_back(
    text: &str,
    input: &[(Location, TokenKind)],
    output: &[(usize, &TokenKind)],
    comments: &[Comment],
) -> String {
    let anchors: Vec<(Location, &TokenKind)> = input
        .iter()
        .filter(|(_, kind)| anchor(kind))
        .map(|(location, kind)| (*location, kind))
        .collect();
    // The output line of each anchor token, when unparse kept it
    let mut placed: Vec<Option<usize>> = vec![None; anchors.len()];
    let mut next = 0;
    for (line, kind) in output {
        let end = (next + LOOKAHEAD).min(anchors.len());
        if let Some(k) = (next..end).find(|&k| same(anchors[k].1, kind)) {
            placed[k] = Some(*line);
            next = k + 1;
        }
    }
    let content: HashSet<usize> = input
        .iter()
        .map(|(location, _)| location.line)
        .chain(comments.iter().map(|(location, _)| location.line))
        .collect();
    let first = content.iter().min().copied().unwrap_or(1);
    let blank = |line: usize| line > first && !content.contains(&line);

    let mut before: BTreeMap<usize, Vec<Placed>> = BTreeMap::new();
    let mut trailing: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    let mut tail: Vec<Placed> = Vec::new();
    for (location, comment) in comments {
        let at = (location.line, location.column);
        let index = input.partition_point(|(l, _)| (l.line, l.column) < at);
        let split = anchors.partition_point(|(l, _)| (l.line, l.column) < at);
        let is_trailing = index > 0 && input[index - 1].0.line == location.line;
        if is_trailing && let Some(line) = placed[..split].iter().rev().find_map(|p| *p) {
            trailing.entry(line).or_default().push(comment);
            continue;
        }
        let entry = Placed {
            text: comment,
            blank_before: blank(location.line - 1),
            blank_after: blank(location.line + 1),
        };
        match placed[split..].iter().find_map(|p| *p) {
            Some(line) => before.entry(line).or_default().push(entry),
            None => tail.push(entry),
        }
    }

    let mut out = String::new();
    let push_run = |out: &mut String, run: &[Placed], indent: &str| {
        let top = indent.is_empty();
        for comment in run {
            if top && comment.blank_before && !out.is_empty() && !out.ends_with("\n\n") {
                out.push('\n');
            }
            out.push_str(indent);
            out.push_str(comment.text);
            out.push('\n');
        }
    };
    for (number, line) in text.lines().enumerate() {
        if let Some(run) = before.get(&(number + 1)) {
            let indent = &line[..line.len() - line.trim_start().len()];
            push_run(&mut out, run, indent);
            if indent.is_empty() && run.last().is_some_and(|c| c.blank_after) {
                out.push('\n');
            }
        }
        out.push_str(line);
        for comment in trailing.get(&(number + 1)).into_iter().flatten() {
            out.push(' ');
            out.push_str(comment);
        }
        out.push('\n');
    }
    push_run(&mut out, &tail, "");
    out
}

/// `lingua fmt [--check] [--source-root PATH] [PATH ...]`: rewrites each module file in
/// canonical layout, or with `--check` lists the files that would change and writes nothing.
/// Exit 0 when done or clean, 1 for a diagnostic or a file `--check` would change, 2 for a
/// usage error.
#[must_use]
pub fn command(args: &[&str]) -> ExitCode {
    let mut check = false;
    let mut source_root = None;
    let mut paths = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match *arg {
            "--check" => check = true,
            "--source-root" => match args.next() {
                Some(root) => source_root = Some(PathBuf::from(root)),
                None => return usage("--source-root needs a directory"),
            },
            flag if flag.starts_with('-') => {
                return usage(&format!("unknown flag `{flag}` for fmt"));
            }
            path => paths.push(PathBuf::from(path)),
        }
    }
    let (_, files, mut diagnostics) = match check::resolve(&paths, source_root.as_deref()) {
        Ok(resolved) => resolved,
        Err(check::Error::Usage(message)) => return usage(&message),
        Err(check::Error::Diagnostics(diagnostics)) => return report(&diagnostics),
    };
    let mut changed = Vec::new();
    for (path, _) in &files {
        let shown = path.display().to_string();
        let source = match std::fs::read(path) {
            Ok(source) => source,
            Err(err) => {
                eprintln!("error: cannot read `{shown}`: {err}\ncheck the path and try again");
                return ExitCode::FAILURE;
            }
        };
        let formatted = match format_source(&source) {
            Ok(formatted) => formatted,
            Err(errors) => {
                let text = String::from_utf8_lossy(&source);
                diagnostics.extend(
                    errors
                        .iter()
                        .map(|d| Diagnostic::from_parse(d, &shown, &text)),
                );
                continue;
            }
        };
        if formatted.as_bytes() == source {
            continue;
        }
        if check {
            println!("{shown}");
        } else if let Err(err) = std::fs::write(path, &formatted) {
            eprintln!("error: cannot write `{shown}`: {err}\ncheck that the file is writable");
            return ExitCode::FAILURE;
        }
        changed.push(shown);
    }
    if check && !changed.is_empty() {
        eprintln!(
            "{} file(s) would be reformatted; run `lingua fmt` with the same paths to fix them",
            changed.len()
        );
    }
    if !diagnostics.is_empty() {
        return report(&diagnostics);
    }
    if check && !changed.is_empty() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn usage(message: &str) -> ExitCode {
    eprintln!("error: {message}\n{USAGE}\nrun `lingua --help` for every command");
    ExitCode::from(2)
}

fn report(diagnostics: &[Diagnostic]) -> ExitCode {
    for diagnostic in diagnostics {
        eprint!("{diagnostic}");
    }
    ExitCode::FAILURE
}
