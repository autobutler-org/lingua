//! `lingua check --explain CODE`: the catalog paragraph for every allocated code
//! (docs/lingua1.md, Diagnostics > Catalog).

const CATALOG: &[(&str, &str)] = &[
    (
        "L001",
        "Syntax error, including a missing function slot, mixed match-arm classes, or a closer that ended a different construct than the parser was in",
    ),
    ("L002", "Closer name does not match the opener"),
    (
        "L003",
        "Closer kind does not match the opener (`end type` closing a `fn`)",
    ),
    (
        "L004",
        "Bad token, bad escape, lone `}`, newline in a string, non-UTF-8, or a BOM",
    ),
    (
        "L005",
        "Integer token outside `0 through INT_MAX`, or a digit string longer than 20 characters (leading zeros count)",
    ),
    (
        "L010",
        "Import name is not exported; help lists the module's exports",
    ),
    (
        "L011",
        "Unknown name, field, or operation; help lists what is visible",
    ),
    (
        "L012",
        "A path falls off a function, a generative body, or a statement-arm without returning, or `return` appears in a package example or a package `comptime` block",
    ),
    ("L013", "Duplicate binding, or a binding that shadows"),
    ("L014", "`set` on a `let`, parameter, or pattern binding"),
    ("L015", "Import cycle"),
    (
        "L016",
        "Module not found, bad module key, self-import, or a `PATH` outside the source root",
    ),
    (
        "L017",
        "Duplicate declaration, duplicate import, or clash with the prelude",
    ),
    ("L018", "Unreachable statement"),
    ("L020", "Type mismatch that is not specifically narrowing"),
    (
        "L021",
        "Positional argument for a parameter whose type is not unique",
    ),
    ("L022", "No `Text` conversion for an interpolation hole"),
    ("L023", "Implicit narrowing or nominal introduction"),
    (
        "L024",
        "Literal or static bound violation, including `L > U`",
    ),
    ("L025", "Effect violation"),
    (
        "L026",
        "`or return` with different error types and no mapping expression",
    ),
    ("L027", "`if` condition is not `Bool`"),
    (
        "L028",
        "Wrong arity, unknown parameter name, or named argument on a variant payload",
    ),
    ("L029", "Expression statement whose type is not `Nothing`"),
    (
        "L030",
        "`while` path returns to the condition without `set`",
    ),
    (
        "L031",
        "Non-exhaustive match; help lists missing variants, or says add `_` for an open scrutinee",
    ),
    ("L032", "`_` on a closed scrutinee; help lists variants"),
    ("L033", "Unreachable pattern"),
    (
        "L034",
        "Operator does not apply, or comparison across nominal types",
    ),
    ("L035", "`+=` on a non-integer `var`"),
    (
        "L040",
        "Arithmetic overflow in an example, a comptime root, or the native binary. Fault header, not an exception. Still a sorted diagnostic and it counts toward the cap of 100",
    ),
    ("L041", "Example script exhausted"),
    (
        "L042",
        "Fuel exceeded (100_000) in an example, const, provider, or macro. Fault header, not an exception. Interpreter only. Still sorted and counted toward the cap. The native binary has no fuel cap",
    ),
    (
        "L043",
        "Example value mismatch, including `-> Err(...)` versus a different value",
    ),
    ("L044", "Example missing a parameter or resource binding"),
    (
        "L045",
        "Example binds a name that is not a parameter or the effect resource",
    ),
    ("L046", "`split` on an empty separator"),
    (
        "L047",
        "Implementation limit (file size, depth, heap, list length, diagnostic cap)",
    ),
    (
        "L049",
        "`primitive` is not the allowlisted `money.percent` signature",
    ),
    (
        "L050",
        "`lingua.toml` `version` is not `lingua1`, or the file contains a key other than `version`, `[db] url`, and `[net] allow_non_public`",
    ),
    ("L055", "Package-example `->` expression is not `Bool`"),
    ("L056", "Package-example `Bool` evaluated to `false`"),
    (
        "L057",
        "Example expected a value and the root raised an exception",
    ),
    (
        "L058",
        "Exception aborted a const, provider, macro, or other comptime root. The check fails",
    ),
    (
        "L059",
        "Example expected an exception but the message differed, a value was returned, or a package example fell off its statements without raising",
    ),
    (
        "L060",
        "Runtime value used where a comptime value is required",
    ),
    ("L061", "Comptime dependency cycle"),
    (
        "L062",
        "`let` or `var` in a comptime context. The fix is `const`. Does not apply inside an ordinary function that a comptime root called",
    ),
    ("L063", "`Expr` or `Type` used as a runtime type"),
    ("L064", "Splice is not a node the grammar can parse"),
    (
        "L065",
        "Splice kind mismatch: statement splice in expression position, or the reverse",
    ),
    ("L066", "`unquote` outside `quote`"),
    (
        "L067",
        "`provide` parsed outside a package-level `comptime` block. The statement is in the grammar; this code rejects the placement",
    ),
    (
        "L068",
        "A string that must be a literal without holes is not: `assert` / `fail` / `or fail` message, an example label, or `-> exception`",
    ),
    (
        "L069",
        "Ambiguous package-level name; help lists the defining files",
    ),
    (
        "L071",
        "`--offline` and one generated declaration has no entry in `provided/<module>.lf`. One diagnostic per missing name",
    ),
    (
        "L072",
        "A declared provider effect cannot be performed by the host at all, or `lingua build` sees a runtime resource operation with no backend (`open_orders`). A missing `[db] url` is `Err(NotConfigured)`. A timeout is `Err(NetUnavailable(\"timeout\"))` or `Err(DbUnavailable(\"timeout\"))`. Neither is this code",
    ),
    (
        "L073",
        "`types_from_openapi` returned a module `Type` where one record or sum is required",
    ),
    (
        "L074",
        "`lingua build` found no file-level `main`, or more than one, in the modules it is linking. No binary is written. A `main` with the wrong signature still counts toward this code",
    ),
    (
        "L075",
        "A file-level `main` has a parameter list other than empty or one `Process`, or its return type is not `Nothing`. No binary is written. The effect rule stays `L025`. Zero or duplicate `main` stays `L074`",
    ),
    (
        "L076",
        "Division by zero: the divisor of `/` or `%` is `0`. Compile-time if the divisor is the literal `0`, otherwise a fault in an example, a comptime root, or the native binary. Fault header, not an exception. Still sorted and counted toward the cap",
    ),
];

/// The catalog text for `code`, or `None` when it is not allocated.
#[must_use]
pub fn explain(code: &str) -> Option<&'static str> {
    CATALOG
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, text)| *text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_codes_explain_and_reserved_ones_do_not() {
        assert!(
            super::explain("L031")
                .unwrap()
                .starts_with("Non-exhaustive match")
        );
        assert_eq!(super::explain("L019"), None);
        assert_eq!(super::explain("L999"), None);
    }
}
