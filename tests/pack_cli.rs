//! `lingua pack` through the real binary (docs/lingua1.md, Context packer): a header with the
//! imported names' declarations and signatures, then the file's bytes unchanged. Every test
//! copies its fixtures into a fresh source root.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SOURCE_LINE: &str = "# --- source (unchanged) ---\n";

fn read(dir: &str, name: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(dir).join(name)).unwrap()
}

// A fresh source root holding `files` as (name, text)
fn workdir(name: &str, files: &[(&str, String)]) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("pack_cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (file, text) in files {
        std::fs::write(dir.join(file), text).unwrap();
    }
    std::fs::canonicalize(dir).unwrap()
}

fn corpus(name: &str) -> (&str, String) {
    (name, read("tests/corpus", name))
}

fn lingua(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_lingua"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

// The `order.lf` example under "### Context packer" in docs/lingua1.md, up to its `...`
fn spec_example() -> String {
    let doc = read("docs", "lingua1.md");
    let section = &doc[doc.find("### Context packer").unwrap()..];
    let start = section.find("```text\n").unwrap() + "```text\n".len();
    let block = &section[start..];
    let block = &block[..block.find("```").unwrap()];
    block.strip_suffix("...\n").unwrap().to_owned()
}

// The output's header, through the source marker
fn header(output: &str) -> String {
    let end = output.find(SOURCE_LINE).unwrap() + SOURCE_LINE.len();
    output[..end].to_owned()
}

#[test]
fn pack_order_matches_the_spec_and_leaves_the_file_alone() {
    let dir = workdir("order", &[corpus("money.lf"), corpus("order.lf")]);
    let before = std::fs::read(dir.join("order.lf")).unwrap();
    let output = lingua(&dir, &["pack", "order.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(stderr(&output), "");
    let out = stdout(&output);
    let header = header(&out);
    // Byte for byte: the spec's header and the first source line it shows
    assert!(
        format!("{header}{}", read("tests/corpus", "order.lf")).starts_with(&spec_example()),
        "{header}"
    );
    // The tail is the file's bytes, unchanged, and the file is untouched
    assert_eq!(&out.as_bytes()[out.len() - before.len()..], &before[..]);
    assert_eq!(
        out.len(),
        out.find(SOURCE_LINE).unwrap() + SOURCE_LINE.len() + before.len()
    );
    assert_eq!(std::fs::read(dir.join("order.lf")).unwrap(), before);
    assert!(!dir.join("provided").exists());
}

#[test]
fn a_broken_body_still_packs() {
    let broken = read("tests/corpus", "order.lf").replace(
        "return line.price * line.quantity",
        "return line.price * \"oops\"",
    );
    let dir = workdir(
        "broken",
        &[corpus("money.lf"), ("order.lf", broken.clone())],
    );
    let output = lingua(&dir, &["pack", "order.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let out = stdout(&output);
    assert_eq!(format!("{}{broken}", header(&out)), {
        let spec = spec_example();
        let header = &spec[..spec.find(SOURCE_LINE).unwrap() + SOURCE_LINE.len()];
        format!("{header}{broken}")
    });
    // `check` does see the broken body
    assert_eq!(lingua(&dir, &["check", "order.lf"]).status.code(), Some(1));
}

#[test]
fn an_unresolvable_import_is_in_the_header_and_exits_1() {
    let source =
        "import Cents, nope from \"money\"\nimport x from \"shop/money\"\n\nconst c: Cents = 1\n";
    let dir = workdir(
        "unresolved",
        &[corpus("money.lf"), ("use.lf", source.to_owned())],
    );
    let output = lingua(&dir, &["pack", "use.lf"]);
    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    let out = stdout(&output);
    let head = header(&out);
    assert!(
        head.contains("# L010: money.lf does not export `nope`\n"),
        "{head}"
    );
    assert!(head.contains("# L016: "), "{head}");
    assert!(
        head.contains(
            "# import Cents from \"money\"\n# defined in money.lf\ntype Cents = Int at least 0\n"
        ),
        "{head}"
    );
    assert!(out.ends_with(source), "{out}");
    assert_eq!(std::fs::read_to_string(dir.join("use.lf")).unwrap(), source);
}

#[test]
fn functions_consts_and_constructors_print_their_interface() {
    let source = "import page_cap, next_pow2, Batch, Go from \"money\"\n\nconst x = page_cap\n";
    let dir = workdir(
        "interface",
        &[corpus("money.lf"), ("use.lf", source.to_owned())],
    );
    let output = lingua(&dir, &["pack", "use.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let expected = format!(
        "# lingua pack use.lf
# module use
# language: lingua1

# import Go from \"money\"
# defined in money.lf
type PowStep
  Go(Int)
  Stop(Int)
end type PowStep

# import next_pow2 from \"money\"
# defined in money.lf
# next_pow2(n: Int at least 1) -> Int at least 1
#   pure

# import page_cap from \"money\"
# defined in money.lf
const page_cap: Int at least 1 = 128

# import Batch from \"money\"
# defined in money.lf
type Batch = Int in 1 through comptime next_pow2(100)

{SOURCE_LINE}{source}"
    );
    let out = stdout(&output);
    assert_eq!(
        format!("{}{}", header(&out), &out[out.len() - source.len()..]),
        expected
    );
}

#[test]
fn imported_generative_types_print_expanded_and_share_the_cache_policy() {
    let source = "import Offer, PercentOff from \"promo\"\n\nconst p = PercentOff(5)\n";
    let files = [
        ("money.lf", read("tests/providers", "money.lf")),
        ("promo.lf", read("tests/providers", "promo.lf")),
        ("use.lf", source.to_owned()),
    ];
    let dir = workdir("generative", &files);
    let expanded = read("tests/providers", "promo.expanded");
    // Each declaration, a blank line after `Promo` and none after `Offer`
    let (promo, offer) = expanded.split_at(expanded.find("type Offer").unwrap());
    let expected = format!(
        "# lingua pack use.lf
# module use
# language: lingua1

# import PercentOff from \"promo\"
# defined in promo.lf
# expanded
{promo}# import Offer from \"promo\"
# defined in promo.lf
# expanded
{offer}
{SOURCE_LINE}{source}"
    );

    // `--offline` with no cache entry: the L071s are in the header, and no cache is written
    let output = lingua(&dir, &["pack", "--offline", "use.lf"]);
    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    assert!(stdout(&output).contains("# L071: "), "{}", stdout(&output));
    assert!(stdout(&output).ends_with(source));
    assert!(!dir.join("provided").exists());

    // The default runs the effect and writes the cache, as check does
    let output = lingua(&dir, &["pack", "use.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let out = stdout(&output);
    assert_eq!(
        format!("{}{}", header(&out), &out[out.len() - source.len()..]),
        expected
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("provided/promo.lf")).unwrap(),
        expanded
    );

    // Offline now reuses the cache; refresh reruns the body
    for flag in ["--offline", "--refresh"] {
        let output = lingua(&dir, &["pack", flag, "use.lf"]);
        assert_eq!(output.status.code(), Some(0), "{flag}: {}", stderr(&output));
        assert_eq!(stdout(&output), out, "{flag}");
    }
}

#[test]
fn pack_usage_is_self_guiding() {
    let dir = workdir(
        "usage",
        &[
            ("a.lf", "const one = 1\n".to_owned()),
            ("b.lf", "const two = 2\n".to_owned()),
        ],
    );
    let output = lingua(&dir, &["pack", "-h"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(stdout(&output).contains("pack FILE"), "{}", stdout(&output));
    for (args, message) in [
        (
            &["pack"][..],
            "error: pack needs one `.lf` file, e.g. `lingua pack order.lf`\n",
        ),
        (
            &["pack", "."],
            "error: pack takes one `.lf` file, not a directory; pass a file such as `lingua pack a.lf`\n",
        ),
        (
            &["pack", "a.lf", "b.lf"],
            "error: pack takes one file; run `lingua pack` once per file\n",
        ),
        (
            &["pack", "missing.lf"],
            "error: path `missing.lf` does not exist; check the path and try again\n",
        ),
        (
            &["pack", "--refresh", "--offline", "a.lf"],
            "error: --refresh and --offline cannot be used together; pick one\n",
        ),
        (
            &["pack", "--dynamic", "a.lf"],
            "error: unknown flag `--dynamic` for pack\n",
        ),
    ] {
        let output = lingua(&dir, args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(
            stderr(&output).starts_with(message),
            "{args:?}: {}",
            stderr(&output)
        );
        assert_eq!(stdout(&output), "", "{args:?}");
    }
    // No imports: the preamble, then the source
    let output = lingua(&dir, &["pack", "a.lf"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!(
            "# lingua pack a.lf\n# module a\n# language: lingua1\n\n{SOURCE_LINE}const one = 1\n"
        )
    );
}
