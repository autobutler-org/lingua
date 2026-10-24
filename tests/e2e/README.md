# End-to-end cases

`tests/e2e.rs` builds every `tests/e2e/*.lf` with the real `lingua build`, runs `./lingua-out` with no arguments, and compares the result against files beside the case:

| File | Holds | When missing |
|---|---|---|
| `NAME.stderr` | exact stderr bytes (`log` output, then any exception or fault report) | empty |
| `NAME.stdout` | exact stdout bytes. lingua1 never writes stdout, so no case has one | empty |
| `NAME.exit` | the exit code | `0` |

Each case builds in a fresh `target/tmp/e2e/NAME/`. The binaries are linux/amd64. On any other host, run `make e2e`, which uses Docker.

Every case goes through the real checker and the native backend. The expected outputs were worked out by hand from `docs/lingua1.md` and `docs/build.md`.

## Single-file cases

`NAME.lf` is one module with a file-level `main`. It is built as `lingua build tests/e2e/NAME.lf`, so the source root is `tests/e2e/` and the link set is that file alone. No single-file case imports anything.

## Multi-module cases: `NAME.lf/` directories

A helper module can't sit next to the cases. The harness treats every `*.lf` in this directory as a case, and a helper has no `main` (`L074`). A helper with its own `main` doesn't work either, because importing it puts two `main`s in the link set (also `L074`). A `lib/` subdirectory is ruled out too, because an import can't reach outside the source root (`L016`).

So a multi-module case is a **directory** named `NAME.lf/`:

```text
tests/e2e/order_total.lf/     <- the case; the harness passes this path to `lingua build`
  lingua.toml                 <- version = "lingua1"; makes this directory the source root
  main.lf                     <- the only `main` in the package
  money.lf, order.lf          <- helper modules, imported by key ("money", "order")
tests/e2e/order_total.stderr  <- expected output, beside the directory as usual
```

The harness's `.lf` extension filter also matches directories. `lingua build DIR` links every module directly in the source root, and `DIR` has to be that root. The `lingua.toml` inside makes it the root even if a `lingua.toml` is later added further up the tree. The package has exactly one `main`. Expected files use the directory's stem (`order_total.stderr`).

Caveat: anything that lists `tests/e2e/*.lf` and expects files, such as `lingua check tests/e2e` or a glob in another test, will also see these directories. If that becomes a problem, change the harness to skip helpers instead (for example, skip `*.lf` files that have no `fn main`).

## Report shapes these cases assume

The exception and fault blocks follow `lingua1.md` § Value errors and exceptions, using these rules:

- The path is the module's file name relative to the source root (`overflow_add.lf`), not the absolute path the harness passes on the command line.
- The gutter is two columns wide, as in the spec's ` 1 |` and `12 |` examples. Every faulting line is below 100.
- The carets cover: the `assert` or `fail` keyword (as in the spec example); the `or fail` keywords; the whole binary expression for `L040` and `L076` (`top + one`), or the unary expression (`-bottom`); the whole `split(...)` call for `L046`; and the function header line, `fn` through the return type, for `L077`.
- Faults print the full span block (header, `-->`, source line, carets), the same as exceptions.

## Cases

| Case | Covers |
|---|---|
| `hello` | minimal `main`, one `log` |
| `logs` | empty line, escapes, non-ASCII text |
| `int_arith` | `+ - *`, precedence, parentheses, left associativity, unary minus |
| `int_div_rem` | `/` truncates toward zero, `%` takes the dividend's sign, the identity, one `* / %` level |
| `int_holes` | text holes with zero, negative, `INT_MAX`, `INT_MIN`, literals, `{{ }}` |
| `overflow_add` | `L040` on `+`, output before the fault |
| `overflow_sub` | `L040` on `-` |
| `overflow_mul` | `L040` on `*` (`-2^63` fits, `2^63` faults) |
| `overflow_neg` | `L040` on unary minus of `INT_MIN` |
| `div_zero` | `L076` on `/` |
| `rem_zero` | `L076` on `%` |
| `div_int_min` | `INT_MIN / -1` is `L040` |
| `rem_int_min` | `INT_MIN % -1` is `L040` |
| `factorial_overflow` | `L040` inside a recursive function (21!) |
| `deep_recursion` | `L077` stack overflow at the function header |
| `split_empty_separator` | runtime `L046` from `split` with an empty separator held in a variable |
| `assert_fail` | passing `assert`, then a failing one: exception block, exit 1 |
| `assert_in_helper` | exception raised in a helper (whose example expects it), reported at the helper's line |
| `fail_stmt` | `fail` inside an `if` |
| `or_fail` | `or fail` on `Ok` continues, on `Err` aborts |
| `or_fail_ok` | `or fail` on `int_at_least` and a checked index that both succeed |
| `bool_compare` | `< > <= >= == !=` on `Int`, `Text`, and `Bool`; a `Bool`-returning helper |
| `if_else_nested` | nested `if`/`else`, `if` without `else`, all return paths |
| `while_countdown` | `while x is Variant(p)` with `set` on every path; a loop that runs zero times |
| `while_collatz` | `while` with `+=`, `%`, `/`, and nested `if` (27 takes 111 steps) |
| `while_zero_payload` | `while x is Warmup` (zero payload), then a payload loop |
| `for_list` | `for` over list variables, a literal list, and an empty list; `+=` |
| `for_range` | `until` ranges: literal, variable, empty, reversed, negative high, bounds fixed on entry |
| `fizzbuzz` | FizzBuzz 1 to 15 |
| `primes` | primes below 50, with nested loops and an early `return` from a `for` |
| `factorial` | recursion, 0! through 5! and 20! |
| `fibonacci` | doubly recursive fib, iterative fib(90) close to `INT_MAX` |
| `nominal_text` | `type CustomerId = Text`, literal adoption, `CustomerId(...)` wrap, `==` against a literal, elimination to `Text` |
| `nominal_int` | `Cents` and `Quantity` bounded nominals, `*` adoption, `+=` adoption, `Cents - Cents` is `Int`, named same-type args |
| `records` | products, named and reordered constructor args, nested field access, positional construction with distinct field types |
| `sum_no_payload` | zero-payload sum, exhaustive expression match |
| `sum_payload` | payload variants (`Int` and product), statement match where every arm returns |
| `arm_join` | match-arm join: a flexible `0` adopts `Cents`; all-literal arms synthesize `Int` and `Text` |
| `match_open` | `Int` and `Text` scrutinees with literal patterns (including `-1`) and `_` |
| `list_patterns` | `[]`, `[x]`, `["cmd", x]`, `[a, b]`, `_`; matching on `split` results |
| `result_match` | user error sum, `Ok`/`Err` match, payload error |
| `or_return` | bare `or return` across calls; `log` in a pure function shows it short-circuits |
| `or_return_mapped` | `or return Err(...)` mapping `BoundsError`; `or return nothing` exits `main` cleanly |
| `int_in_text` | `int_in` on text: range ends, leading zeros, `-0`, empty, spaces, `+`, junk, beyond `Int` |
| `int_at_least` | `int_at_least` on `Int` values including `INT_MIN`/`INT_MAX`, adoption into `Cents` under bare `or return` |
| `bounded_types` | elimination to wider bounds, `/` and `%` bound synthesis, a loop index as a static list-literal index |
| `max_bound` | `max` restoring `Cents` (either argument order), named args in any order, extremes |
| `list_basics` | literals, a trailing comma, a multi-line list, `length`, `join` (with `[]` on either side), `List<List<Int>>` |
| `list_index` | static literal index, checked index returning `Ok`/`Err` (negative, past the end, `INT_MAX`, empty list) |
| `text_trim_lower` | `trim` removes only space/tab/LF/CR (not U+00A0); `lower` is ASCII-only |
| `split_cases` | every `split` example from the prelude table, plus leading and trailing separators and reordered named args |
| `split_multichar` | multi-scalar separators, no match, non-ASCII separator |
| `text_equality` | `==`/`!=` on text, case sensitivity, no Unicode normalization |
| `process_fields` | `main(process: Process)`: `os`, `arch`, no `args`, `program` base name, `thread.name`, `pid`, `thread.id`, `parallelism` |
| `main_param_name` | the `Process` parameter can have any name |
| `layout` | comments, blank lines, multi-line calls with trailing commas, an example on `main`, `//` inside a string |
| `order_total` | **package**: corpus `money.lf` (the `percent` primitive) and `order.lf`, imported by `main.lf`; order totals, `percent` rounding, `parse_discount` |
| `geometry` | **package**: three modules, transitive imports, importing a sum also binds its constructors |
| `config` | **package**: imported nominal types and error sum, bare `or return` across modules, `Host(...)` wrap, `int_in` adopting `Port` |
