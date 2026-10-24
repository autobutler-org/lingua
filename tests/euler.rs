//! Parses every `.lf` under `euler/` and asserts it has no diagnostics.
//! When the checker lands, the examples in these files will run as well.

use std::path::{Path, PathBuf};

use lingua::parse::parse_source;

fn sources(dir: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "lf"))
        .collect();
    found.sort();
    found
}

#[test]
fn euler_solutions_parse() {
    let files = sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("euler"));
    assert!(!files.is_empty(), "no .lf files found under euler/");

    let failures: Vec<String> = files
        .iter()
        .filter_map(|path| {
            let key = path.file_stem().unwrap().to_str().unwrap();
            parse_source(&std::fs::read(path).unwrap(), key)
                .err()
                .map(|diagnostics| {
                    let lines: Vec<String> = diagnostics.iter().map(ToString::to_string).collect();
                    format!("{}:\n{}", path.display(), lines.join("\n"))
                })
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
