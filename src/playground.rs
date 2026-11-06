//! One source buffer in, and the text the playground shows: tokens, AST, TIR, LIR, amd64
//! assembly, and the interpreter's stdout for a parameterless `main`.
//!
//! The buffer is one module. Imports are not loaded, and provider effects are not run
//! (`--offline`). The ELF linker and the native process entry are not used.

use crate::ast::dump as dump_ast;
use crate::build::{self, arch::amd64, lir};
use crate::check::{self, is_module_key};
use crate::lex::{TokenKind, lex_source};
use crate::parse::parse_source;
use crate::tir::dump as dump_tir;

/// The interpreter budget for `main`. The end-to-end runner uses the same number of steps.
const RUN_FUEL: u64 = 50_000_000;
const HEX: &[u8; 16] = b"0123456789abcdef";

/// The six panels. A stage that did not run holds the diagnostic text instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Views {
    pub tokens: String,
    pub ast: String,
    pub tir: String,
    pub lir: String,
    pub assembly: String,
    pub run: String,
}

/// Compiles `source` as module `module` (`factorial` for the playground sample).
#[must_use]
pub fn compile(module: &str, source: &str) -> Views {
    if !is_module_key(module) {
        return fill(format!("`{module}` is not a module key\n"));
    }
    let path = format!("{module}.lf");
    let tokens = match lex_source(source.as_bytes()) {
        Ok(tokens) => {
            let mut out = String::new();
            for token in tokens.iter().filter(|token| token.kind != TokenKind::Eof) {
                out.push_str(&token.to_string());
                out.push('\n');
            }
            out
        }
        Err(err) => {
            return fill(check::Diagnostic::from_parse(&err.into(), &path, source).to_string());
        }
    };
    let ast = match parse_source(source.as_bytes(), module) {
        Ok(module) => dump_ast(&module),
        Err(errors) => {
            let text = errors
                .iter()
                .map(|d| check::Diagnostic::from_parse(d, &path, source).to_string())
                .collect();
            return Views {
                tokens,
                ..fill(text)
            };
        }
    };
    let program = match check::link_source(module, source) {
        Ok(program) => program,
        Err(err) => {
            return Views {
                tokens,
                ast,
                ..fill(check_text(err))
            };
        }
    };
    let tir = dump_tir(&program);
    let lir_text = match build::lower(&program) {
        Ok(lowered) => lir::dump(&lowered.functions, &lowered.sites),
        Err(err) => build_text(err),
    };
    let assembly = match build::select(&program) {
        Ok(functions) => functions
            .iter()
            .map(|progs| amd64::dump(progs))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(err) => build_text(err),
    };
    let run = match crate::eval::run_main(&program, RUN_FUEL) {
        Some(text) => text,
        None => {
            "fn main takes parameters; the playground runs only `fn main()` with none\n".to_owned()
        }
    };
    Views {
        tokens,
        ast,
        tir,
        lir: lir_text,
        assembly,
        run,
    }
}

/// [`compile`] as one JSON object. Strings are escaped. The keys are `tokens`, `ast`,
/// `tir`, `lir`, `assembly`, and `run`.
#[must_use]
pub fn compile_json(module: &str, source: &str) -> String {
    encode(&compile(module, source))
}

/// `views` as the JSON object [`compile_json`] returns.
#[must_use]
pub fn encode(views: &Views) -> String {
    let mut out = String::from("{");
    for (index, (key, value)) in [
        ("tokens", views.tokens.as_str()),
        ("ast", views.ast.as_str()),
        ("tir", views.tir.as_str()),
        ("lir", views.lir.as_str()),
        ("assembly", views.assembly.as_str()),
        ("run", views.run.as_str()),
    ]
    .into_iter()
    .enumerate()
    {
        if index > 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(key);
        out.push_str("\":");
        push_json_string(&mut out, value);
    }
    out.push('}');
    out
}

fn fill(text: String) -> Views {
    Views {
        tokens: text.clone(),
        ast: text.clone(),
        tir: text.clone(),
        lir: text.clone(),
        assembly: text.clone(),
        run: text,
    }
}

fn check_text(err: check::Error) -> String {
    match err {
        check::Error::Usage(message) => format!("error: {message}\n"),
        check::Error::Diagnostics(diagnostics) => {
            diagnostics.iter().map(ToString::to_string).collect()
        }
    }
}

fn build_text(err: build::Error) -> String {
    match err {
        build::Error::Usage(message) => format!("error: {message}\n"),
        build::Error::Diagnostics(diagnostics) => {
            diagnostics.iter().map(ToString::to_string).collect()
        }
        build::Error::Internal(message) => format!(
            "error: {message}\nthis is a bug in lingua; please report it with the source that caused it\n"
        ),
        build::Error::Rendered(text) => text,
    }
}

fn push_json_string(out: &mut String, text: &str) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if (ch as u32) < 0x20 => {
                let code = ch as u32;
                out.push_str("\\u00");
                out.push(char::from(HEX[((code >> 4) & 0xf) as usize]));
                out.push(char::from(HEX[(code & 0xf) as usize]));
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    const FACTORIAL_RUN: &str = "\
0! = 1
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120
20! = 2432902008176640000
";

    #[test]
    fn factorial_matches_the_filesystem_pipeline_and_the_sample_run() {
        let source = include_str!("../tests/e2e/factorial.lf");
        let views = compile("factorial", source);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/e2e/factorial.lf");
        let program = build::check(&[path], &check::Options::default()).expect("factorial checks");
        assert_eq!(views.tir, dump_tir(&program));
        let lowered = build::lower(&program).expect("factorial lowers");
        assert_eq!(views.lir, lir::dump(&lowered.functions, &lowered.sites));
        let assembly = build::select(&program)
            .expect("factorial selects")
            .iter()
            .map(|progs| amd64::dump(progs))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(views.assembly, assembly);
        assert_eq!(views.run, FACTORIAL_RUN);
        assert!(views.tokens.starts_with("1:1 Keyword(Fn)\n"));
        assert!(views.ast.starts_with("(module \"factorial\"\n"));
        assert!(views.tir.contains("factorial.lf"));
        let json = compile_json("factorial", source);
        assert!(json.starts_with("{\"tokens\":\"1:1 Keyword(Fn)\\n"));
        assert!(json.contains("\"run\":\"0! = 1\\n"));
        assert!(json.ends_with("2432902008176640000\\n\"}"));
    }

    #[test]
    fn a_lex_error_fills_every_panel() {
        let views = compile("factorial", "!");
        assert!(views.tokens.contains("L004"), "{}", views.tokens);
        assert_eq!(views.tokens, views.ast);
        assert_eq!(views.tokens, views.run);
    }

    #[test]
    fn a_parse_error_keeps_tokens_and_reports_on_the_later_panels() {
        let views = compile("factorial", "fn\n");
        assert!(views.tokens.starts_with("1:1 Keyword(Fn)\n"));
        assert!(views.ast.contains("L001") || views.ast.contains("--> factorial.lf"));
        assert_eq!(views.ast, views.tir);
        assert_eq!(views.ast, views.run);
    }

    #[test]
    fn an_import_is_not_loaded_from_disk() {
        let source = "\
import percent from \"money\"

fn main() -> Nothing
  pure

  return nothing
end fn main
";
        let views = compile("factorial", source);
        assert!(views.tokens.contains("Keyword(Import)"));
        assert!(views.ast.contains("percent"));
        assert!(views.tir.contains("L016"));
        assert!(views.tir.contains("compiles this file alone"));
        assert!(!PathBuf::from("/lingua-playground").exists());
    }

    #[test]
    fn a_provider_effect_is_not_run() {
        let source = "\
const t: Text
  reads fs
  = fs.read(\"secret.txt\") or fail \"missing\"

fn main() -> Nothing
  pure

  return nothing
end fn main
";
        let views = compile("factorial", source);
        assert!(
            views.tir.contains("not supported yet") || views.tir.contains("L071"),
            "{}",
            views.tir
        );
        assert!(!views.run.contains("secret.txt contents"));
        assert!(!PathBuf::from("/lingua-playground").exists());
    }

    #[test]
    fn an_offline_generative_type_is_l071_and_writes_no_cache() {
        let source = "\
type Row
  pure
  comptime
    return record_from_columns([Column(name: \"id\", sql_type: \"integer\"), Column(name: \"name\", sql_type: \"text\"), Column(name: \"active\", sql_type: \"boolean\")])
  end
end type Row
";
        let views = compile("factorial", source);
        assert!(views.tir.contains("L071"), "{}", views.tir);
        assert!(!PathBuf::from("/lingua-playground/provided/factorial.lf").exists());
    }

    #[test]
    fn a_bad_module_key_is_rejected_before_compile() {
        let views = compile(
            "not a key",
            "fn main() -> Nothing\n  pure\n  return nothing\nend fn main\n",
        );
        assert!(views.tokens.contains("not a module key"));
        assert_eq!(views.tokens, views.assembly);
    }
}
