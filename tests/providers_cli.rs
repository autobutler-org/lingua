//! Generative types, package `comptime` blocks, and the `provided/` cache through the real
//! binary: `lingua expand`, and the `--offline` / `--refresh` policy `check` shares with it.
//! Fixtures and the expected expanded text live in `tests/providers/`; every test copies them
//! into a fresh source root, so no cache file is left in the repository.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/providers")
            .join(name),
    )
    .unwrap()
}

// A fresh source root holding the named fixtures
fn workdir(name: &str, fixtures: &[&str]) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("providers_cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("lingua.toml"), "version = \"lingua1\"\n").unwrap();
    for f in fixtures {
        std::fs::write(dir.join(f), fixture(f)).unwrap();
    }
    dir
}

fn lingua(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_lingua"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn cache(dir: &Path, key: &str) -> Option<String> {
    std::fs::read_to_string(dir.join("provided").join(format!("{key}.lf"))).ok()
}

#[test]
fn expand_prints_a_generated_record_and_writes_the_cache() {
    let dir = workdir("orders", &["orders.lf"]);
    let output = lingua(&dir, &["expand", "orders.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), fixture("orders.expanded"));
    assert_eq!(stderr(&output), "");
    assert_eq!(
        cache(&dir, "orders").as_deref(),
        Some(&*fixture("orders.expanded"))
    );
}

#[test]
fn provide_writes_each_name_in_source_order() {
    let dir = workdir("shapes", &["shapes.lf"]);
    let output = lingua(&dir, &["expand", "shapes.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), fixture("shapes.expanded"));
    assert_eq!(
        cache(&dir, "shapes").as_deref(),
        Some(&*fixture("shapes.expanded"))
    );
}

#[test]
fn expand_prints_the_promo_and_offer_fixtures_and_their_cache() {
    let dir = workdir("promo", &["money.lf", "promo.lf"]);
    let output = lingua(&dir, &["expand", "promo.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), fixture("promo.expanded"));
    assert_eq!(
        cache(&dir, "promo").as_deref(),
        Some(&*fixture("promo.expanded"))
    );
    // The cache re-parses to the same declarations: offline, an importer sees the sum's
    // constructors and the record's fields
    std::fs::write(
        dir.join("use.lf"),
        "import Offer, Promo from \"promo\"\n\nconst o = Offer(id: 1, price: 250, quantity: 3, promo: PercentOff(10))\n",
    )
    .unwrap();
    let output = lingua(&dir, &["check", "--offline"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let output = lingua(&dir, &["expand", "--offline", "promo.lf"]);
    assert_eq!(stdout(&output), fixture("promo.expanded"));
}

// What a cache hit stands for: the body's result, here a different record than the body
// builds, so a hit is visible
const EDITED: &str = "type OrderRow\n  id: Int\n  note: Text\nend type OrderRow\n";

#[test]
fn check_writes_the_cache_and_reuses_it_without_running_the_body() {
    let dir = workdir("reuse", &["orders.lf"]);
    let output = lingua(&dir, &["check", "orders.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        cache(&dir, "orders").as_deref(),
        Some(&*fixture("orders.expanded"))
    );
    std::fs::write(dir.join("provided/orders.lf"), EDITED).unwrap();
    for args in [
        &["expand", "orders.lf"][..],
        &["expand", "--offline", "orders.lf"],
    ] {
        let output = lingua(&dir, args);
        assert_eq!(
            output.status.code(),
            Some(0),
            "{args:?}: {}",
            stderr(&output)
        );
        assert_eq!(stdout(&output), EDITED, "{args:?}");
    }
    assert_eq!(cache(&dir, "orders").as_deref(), Some(EDITED));
}

#[test]
fn offline_without_a_cache_entry_is_l071_and_writes_nothing() {
    let dir = workdir("offline", &["orders.lf"]);
    let output = lingua(&dir, &["check", "--offline", "orders.lf"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        stderr(&output),
        "L071: `--offline` and `OrderRow` has no entry in provided/orders.lf\n  --> orders.lf:1:6\n   |\n 1 | type OrderRow\n   |      ^^^^^^^^\n   |\n   = help: run without `--offline` once to expand it and write the cache\n"
    );
    assert_eq!(cache(&dir, "orders"), None);
}

#[test]
fn refresh_reruns_the_body_and_replaces_the_cache() {
    let dir = workdir("refresh", &["orders.lf"]);
    std::fs::create_dir_all(dir.join("provided")).unwrap();
    std::fs::write(dir.join("provided/orders.lf"), EDITED).unwrap();
    let output = lingua(&dir, &["expand", "--refresh"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), fixture("orders.expanded"));
    assert_eq!(
        cache(&dir, "orders").as_deref(),
        Some(&*fixture("orders.expanded"))
    );
}

#[test]
fn a_generated_type_is_an_ordinary_type_to_importers() {
    let dir = workdir("importer", &["shapes.lf"]);
    std::fs::write(
        dir.join("use.lf"),
        "import Point from \"shapes\"\n\nfn sum(p: Point) -> Int\n  pure\n  example \"e\"\n    p: Point(x: 1, y: 2)\n    -> 3\n  end\n\n  return p.x + p.y\nend fn sum\n",
    )
    .unwrap();
    let output = lingua(&dir, &["check", "use.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    // A wrong field is the usual diagnostic, at the use
    std::fs::write(
        dir.join("bad.lf"),
        "import Point from \"shapes\"\n\nconst p: Point = Point(x: 1, z: 2)\n",
    )
    .unwrap();
    let output = lingua(&dir, &["check", "bad.lf"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).starts_with("L028"), "{}", stderr(&output));
}

#[test]
fn expand_usage_is_self_guiding() {
    let dir = workdir("usage", &[]);
    let output = lingua(&dir, &["expand", "-h"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(
        stdout(&output).contains("expand [PATH ...]"),
        "{}",
        stdout(&output)
    );
    for (args, message) in [
        (
            &["expand", "--refresh", "--offline"][..],
            "error: --refresh and --offline cannot be used together; pick one\n",
        ),
        (
            &["expand", "--dynamic"][..],
            "error: unknown flag `--dynamic` for expand\n",
        ),
    ] {
        let output = lingua(&dir, args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(stderr(&output).starts_with(message), "{}", stderr(&output));
    }
    // A package with no generative type expands to nothing
    std::fs::write(dir.join("plain.lf"), "const one = 1\n").unwrap();
    let output = lingua(&dir, &["expand"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stdout(&output), "");
}

// `lingua expand` also prints each declaration that holds a macro splice, with the splice in
// place, as source that parses again; a splice writes no cache
#[test]
fn expand_prints_spliced_declarations_as_source() {
    for (from, name) in [("tests/corpus", "assert"), ("tests/e2e", "splice")] {
        let from = Path::new(env!("CARGO_MANIFEST_DIR")).join(from);
        let dir = workdir(name, &[]);
        let source = format!("{name}.lf");
        std::fs::copy(from.join(&source), dir.join(&source)).unwrap();
        let output = lingua(&dir, &["expand", &source]);
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        let golden = std::fs::read_to_string(from.join(format!("{name}.expanded"))).unwrap();
        assert_eq!(stdout(&output), golden);
        assert!(
            lingua::parse::parse_source(golden.as_bytes(), name).is_ok(),
            "{golden}"
        );
        assert!(cache(&dir, name).is_none());
    }
}
