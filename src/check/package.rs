//! The source root, `lingua.toml`, and module loading (docs/lingua1.md, Module and file
//! model): which files are modules, their import graph (`L015`, `L016`), and `L050`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{Diagnostic, Error};
use crate::ast::{Module, Span};
use crate::lex::Location;
use crate::parse::parse_source;

/// Flags shared by `check`, `expand`, `pack`, and `build`.
#[derive(Debug, Default, Clone)]
pub struct Options {
    /// `--source-root`: overrides the upward `lingua.toml` walk.
    pub source_root: Option<PathBuf>,
    pub cache: Cache,
}

/// The provider cache policy (docs/lingua1.md, Compilation pipeline, Provider cache flags).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Cache {
    /// Reuse a declaration already in `provided/<module>.lf`; otherwise run the effect.
    #[default]
    Default,
    /// `--refresh`: run every effect again and replace the cache.
    Refresh,
    /// `--offline`: run no provider effect; a missing declaration is `L071`.
    Offline,
}

/// One module file.
#[derive(Debug)]
pub struct Source {
    pub key: String,
    /// Relative to the source root (`money.lf`).
    pub path: String,
    pub text: String,
    /// `None` when the file did not lex or parse; its diagnostics are already reported.
    pub module: Option<Module>,
}

/// Every module in the request and its import closure, imports before importers.
#[derive(Debug)]
pub struct Loaded {
    /// The source root, canonical.
    pub root: PathBuf,
    pub cache: Cache,
    pub sources: Vec<Source>,
    pub requested: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Loaded {
    pub fn index(&self, key: &str) -> Option<usize> {
        self.sources.iter().position(|s| s.key == key)
    }
}

fn whole_line(line: usize, text: &str) -> Span {
    Span {
        start: Location { line, column: 1 },
        end: Location {
            line,
            column: text.chars().count() + 1,
        },
    }
}

/// Whether `key` is one `ident` spelling: a letter, or `_` and at least one more character,
/// then letters, digits, and `_`.
#[must_use]
pub fn is_module_key(key: &str) -> bool {
    let mut chars = key.chars();
    let first_ok = match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => true,
        Some('_') => key.len() > 1,
        _ => false,
    };
    first_ok && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

// The directory a path's source root walk starts from
fn start_dir(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.to_path_buf()
    } else {
        path.parent()
            .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    }
}

fn root_for(path: &Path) -> PathBuf {
    let start = start_dir(path);
    start
        .ancestors()
        .find(|dir| dir.join("lingua.toml").is_file())
        .map_or(start.clone(), Path::to_path_buf)
}

fn canonical(path: &Path) -> Result<PathBuf, Error> {
    std::fs::canonicalize(path).map_err(|_| {
        Error::Usage(format!(
            "path `{}` does not exist; check the path and try again",
            path.display()
        ))
    })
}

/// Checks `lingua.toml`: `version = "lingua1"`, `[db] url`, and `[net] allow_non_public`.
/// Anything else is `L050`.
pub fn check_toml(text: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut section = String::new();
    let mut seen: Vec<String> = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        let bad = |summary: String| {
            Diagnostic::new("L050", summary, "lingua.toml")
                .at(text, whole_line(i + 1, raw))
                .help("legal alternatives: version, [db] url, [net] allow_non_public")
        };
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            name.trim().clone_into(&mut section);
            if section != "db" && section != "net" {
                diagnostics.push(bad(format!("`[{section}]` is not a lingua.toml section")));
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            diagnostics.push(bad("expected `key = value`".to_owned()));
            continue;
        };
        let (key, value) = (key.trim(), value.trim());
        let value = match value.find(" #") {
            Some(at) if !value.starts_with('"') || value[..at].ends_with('"') => value[..at].trim(),
            _ => value,
        };
        let string = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .filter(|v| !v.contains('"'));
        let full = if section.is_empty() {
            key.to_owned()
        } else {
            format!("{section}.{key}")
        };
        if seen.contains(&full) {
            diagnostics.push(bad(format!("`{full}` is set twice")));
            continue;
        }
        seen.push(full.clone());
        match full.as_str() {
            "version" => match string {
                Some("lingua1") => {}
                Some(other) => {
                    diagnostics.push(bad(format!("version `{other}` is not `lingua1`")));
                }
                None => diagnostics.push(bad("`version` must be the string \"lingua1\"".into())),
            },
            "db.url" if string.is_none() => {
                diagnostics.push(bad("`[db] url` must be a string".into()));
            }
            "net.allow_non_public" if value != "true" && value != "false" => {
                diagnostics.push(bad("`[net] allow_non_public` must be true or false".into()));
            }
            "db.url" | "net.allow_non_public" => {}
            _ => diagnostics.push(bad(format!("`{full}` is not a lingua.toml key"))),
        }
    }
    diagnostics
}

// The `*.lf` files directly in `root`, in lexicographic order
fn root_files(root: &Path) -> Vec<String> {
    let mut keys: Vec<String> = std::fs::read_dir(root)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "lf"))
                .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .collect()
        })
        .unwrap_or_default();
    keys.sort();
    keys
}

struct Loader {
    root: PathBuf,
    sources: Vec<Source>,
    index: HashMap<String, usize>,
    // 0 unvisited, 1 on the stack, 2 done
    state: HashMap<String, u8>,
    stack: Vec<String>,
    order: Vec<String>,
    diagnostics: Vec<Diagnostic>,
}

impl Loader {
    fn read(&mut self, key: &str) -> usize {
        if let Some(&i) = self.index.get(key) {
            return i;
        }
        let path = format!("{key}.lf");
        let bytes = std::fs::read(self.root.join(&path)).unwrap_or_default();
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let module = match parse_source(&bytes, key) {
            Ok(module) => Some(module),
            Err(errors) => {
                self.diagnostics.extend(
                    errors
                        .iter()
                        .map(|d| Diagnostic::from_parse(d, &path, &text)),
                );
                None
            }
        };
        self.sources.push(Source {
            key: key.to_owned(),
            path,
            text,
            module,
        });
        self.index.insert(key.to_owned(), self.sources.len() - 1);
        self.sources.len() - 1
    }

    // Depth-first over imports: L016 for a bad or missing module, L015 for a cycle
    fn visit(&mut self, key: &str) {
        if let Some(1 | 2) = self.state.get(key) {
            return;
        }
        self.state.insert(key.to_owned(), 1);
        self.stack.push(key.to_owned());
        let i = self.read(key);
        let imports: Vec<(String, Span)> = self.sources[i]
            .module
            .as_ref()
            .map(|m| {
                m.imports
                    .iter()
                    .map(|im| (im.path.clone(), im.span))
                    .collect()
            })
            .unwrap_or_default();
        for (target, span) in imports {
            let (path, text) = (self.sources[i].path.clone(), self.sources[i].text.clone());
            let help = format!("the source root is {}", self.root.display());
            let problem = if !is_module_key(&target) {
                Some(format!(
                    "`\"{target}\"` is not a module key; import a file directly in the source root by its stem"
                ))
            } else if target == key {
                Some(format!("module `{key}` imports itself"))
            } else if !self.root.join(format!("{target}.lf")).is_file() {
                Some(format!(
                    "module `{target}` not found: no `{target}.lf` in the source root"
                ))
            } else {
                None
            };
            if let Some(summary) = problem {
                self.diagnostics.push(
                    Diagnostic::new("L016", summary, &path)
                        .at(&text, span)
                        .help(help),
                );
                continue;
            }
            if self.state.get(&target) == Some(&1) {
                let from = self.stack.iter().position(|k| *k == target).unwrap_or(0);
                let mut cycle: Vec<&str> = self.stack[from..].iter().map(String::as_str).collect();
                cycle.push(&target);
                self.diagnostics.push(
                    Diagnostic::new(
                        "L015",
                        format!("import cycle: {}", cycle.join(" -> ")),
                        &path,
                    )
                    .at(&text, span),
                );
                continue;
            }
            self.visit(&target);
        }
        self.stack.pop();
        self.state.insert(key.to_owned(), 2);
        self.order.push(key.to_owned());
    }
}

/// Resolves the source root and loads every requested module and its import closure.
///
/// # Errors
/// A usage error: a path that does not exist, or paths that imply different roots.
pub fn load(paths: &[PathBuf], options: &Options) -> Result<Loaded, Error> {
    let paths: Vec<PathBuf> = if paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        paths.to_vec()
    };
    let canon: Vec<PathBuf> = paths
        .iter()
        .map(|p| canonical(p))
        .collect::<Result<_, _>>()?;
    let root = if let Some(root) = &options.source_root {
        canonical(root)?
    } else {
        let roots: Vec<PathBuf> = canon.iter().map(|p| root_for(p)).collect();
        if let Some(other) = roots.iter().find(|r| **r != roots[0]) {
            return Err(Error::Usage(format!(
                "the paths imply different source roots, {} and {}; pass --source-root to choose one",
                roots[0].display(),
                other.display()
            )));
        }
        roots[0].clone()
    };
    let mut diagnostics = Vec::new();
    if let Ok(text) = std::fs::read_to_string(root.join("lingua.toml")) {
        diagnostics.extend(check_toml(&text));
    }
    let mut requested: Vec<String> = Vec::new();
    for (given, path) in paths.iter().zip(&canon) {
        let keys = if *path == root {
            root_files(&root)
        } else if path.is_file()
            && path.extension().is_some_and(|e| e == "lf")
            && path.parent() == Some(root.as_path())
        {
            path.file_stem()
                .map(|s| vec![s.to_string_lossy().into_owned()])
                .unwrap_or_default()
        } else {
            diagnostics.push(
                Diagnostic::new(
                    "L016",
                    "path is outside the source root",
                    &given.display().to_string(),
                )
                .help(format!("the source root is {}", root.display())),
            );
            continue;
        };
        for key in keys {
            if !requested.contains(&key) {
                requested.push(key);
            }
        }
    }
    let mut loader = Loader {
        root: root.clone(),
        sources: Vec::new(),
        index: HashMap::new(),
        state: HashMap::new(),
        stack: Vec::new(),
        order: Vec::new(),
        diagnostics,
    };
    for key in &requested {
        loader.visit(key);
    }
    let Loader {
        mut sources,
        order,
        diagnostics,
        ..
    } = loader;
    // Imports before importers: the depth-first postorder
    sources.sort_by_key(|s| order.iter().position(|k| *k == s.key));
    Ok(Loaded {
        root,
        cache: options.cache,
        sources,
        requested,
        diagnostics,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_keys_are_one_ident_spelling() {
        for good in ["money", "_money", "__", "a1_b"] {
            assert!(is_module_key(good), "{good}");
        }
        for bad in ["_", "", "1a", "a/b", "a.b", "a-b"] {
            assert!(!is_module_key(bad), "{bad}");
        }
    }

    #[test]
    fn toml_accepts_the_legal_keys() {
        let ok = "# c\nversion = \"lingua1\"\n\n[db]\nurl = \"postgres://x\"\n\n[net]\nallow_non_public = false\n";
        assert_eq!(check_toml(ok), []);
    }

    #[test]
    fn toml_rejects_other_versions_and_keys_with_l050() {
        let codes = |t: &str| {
            check_toml(t)
                .iter()
                .map(|d| (d.code, d.span.unwrap().start.line))
                .collect::<Vec<_>>()
        };
        assert_eq!(codes("version = \"lingua2\"\n"), [("L050", 1)]);
        assert_eq!(
            codes("name = \"x\"\n[db]\nuser = \"u\"\n"),
            [("L050", 1), ("L050", 3)]
        );
        assert_eq!(codes("[tools]\n"), [("L050", 1)]);
        assert_eq!(codes("[net]\nallow_non_public = 1\n"), [("L050", 2)]);
    }
}
