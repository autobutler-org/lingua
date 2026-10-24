# Lingua Type System

| | |
|---|---|
| **Document** | Overview of how `lingua1` typechecks |
| **Date** | 2026-10-24 |
| **Status** | Informative. [lingua1.md](lingua1.md) is normative; where this page and it disagree, lingua1.md wins |
| **Audience** | Engineers reading or changing `src/check`, and anyone asking "what kind of type system is this?" |

Lingua's checker is **bidirectional** over **declared, monomorphic, nominal, algebraic** types, with **interval subtyping on integers**, **literal adoption**, and a **declared effect lattice**. It is not Hindley–Milner and not structural. Every rule exists so that a mismatch is reported at the line where it happens, against a type someone wrote down.

## What it is not

**Not Hindley–Milner.** HM inference is a non-goal of lingua1. There are no type variables to solve and no unification engine. Every function signature is written in full: parameter types, return type, and effect. The checker never infers a signature, because the signature is the specification a model generates against, and inference would let a wrong body change it.

**Not structural.** Two types are the same only when they are the same declaration. Matching shapes do not make types compatible.

## Bidirectional checking

Each expression is checked in one of two modes:

| Mode | Judgment | Where the type comes from |
|---|---|---|
| Synthesize | `Γ ⊢ e ⇒ T` | The expression itself: a local's declared type, a call's return type, a field's type, a literal's singleton range |
| Check | `Γ ⊢ e ⇐ T` | The context: a parameter type, a declared return type, a `let x: T` annotation, an element type |

Some forms only work in check mode:

- `[]` has no type of its own. It checks against an expected `List<T>`.
- A string literal becomes a nominal text type only when that type is expected.
- An integer literal must fall inside the expected range (`L024`).

This is why Lingua needs few annotations without any inference: the expected type usually comes from a signature that is already written. The full judgment list is [Subtype judgment summary](lingua1.md#subtype-judgment-summary).

## Algebraic and monomorphic

The data types are algebraic:

- **Products**: `type Line ... end type Line`, a record of named fields.
- **Sums**: `type Discount ... end type Discount`, variants with at most one payload each. A product is the payload when a variant needs several values.
- **Prelude constructors**: `List<T>` and `Result<T, E>`, plus the closed sum `BoundsError`.

`match` takes them apart. A match on a closed sum (a sum declared in the program, `Result`, or `BoundsError`) must cover every variant, and nested payload patterns may split one variant across arms; exhaustiveness is checked through the nested closed sums. `_` on a closed type is `L032`. Open scrutinees (`Text`, `Int`, integer bounds, lists) need `_`.

User code is **monomorphic**. `List` and `Result` are the only generic type constructors, and users cannot declare `type Pair<T>`. The prelude functions `join` and `length` are polymorphic: a call reads `T` from its arguments, the same way any built-in reads its types from its arguments, with no general solver.

## Nominal

Type identity is the declaration:

- `type CustomerId = Text` and `type Cursor = Text` do not mix, though both are text.
- Two records with identical fields are different types.
- The same name declared in two modules is two types (`Order` in `order.lf` and in `fulfillment.lf`).

A swapped pair of same-representation arguments is therefore a type error (`L020`). Where two parameters do share a type, the positional-call rule requires named arguments (`L021`).

## Three refinements on direct matching

### Literal adoption

A literal takes on the expected nominal type: `"c1"` checks as `CustomerId` when one is expected. A `Text` variable does not; only literals adopt. An integer literal adopts a bounded type only when it is inside the range.

### Interval subtyping on integers

This is the only subtyping in the language, in the style of Ada's bounded integers.

```text
Int in 1 through 100  <:  Int at least 0  <:  Int
```

- **Elimination**, toward a wider range or `Int`, is implicit.
- **Introduction**, toward a narrower range, is never implicit. It goes through `int_in` or `int_at_least`, which return `Result<_, BoundsError>`.
- **Arithmetic synthesizes ranges.** `+` and `*` of non-negative operands give `Int at least 0`. `-` always gives `Int`. `/` and `%` by a positive divisor can keep a bound. `max(a: 0, b: e)` restores a lower bound of `0`, which is how `order_total` returns `Cents` without a runtime check.
- **Overflow** of the 64-bit implementation integer is a fault (`L040`), not wrap and not `Result`.

This is interval arithmetic, not refinement types: a type carries a lower and an upper bound and nothing else. See [Ranges](lingua1.md#ranges), [Elimination and introduction](lingua1.md#elimination-and-introduction), and [Arithmetic](lingua1.md#arithmetic).

### A declared effect lattice

Each function declares exactly one effect, and calls may only go down the lattice. There is no effect inference.

```text
pure  ->  reads R  ->  writes R
pure  ->  reads clock
```

`reads R` and `reads clock` are incomparable. `log` is the one ambient exemption. Crossing the lattice is `L025`. See [Effects](lingua1.md#effects).

## Where it lives

`src/check/` implements these rules and produces the typed IR (`src/tir.rs`) that `lingua build` lowers. Each implemented diagnostic code has an accepting and a rejecting test in `src/check/tests.rs`, and `tests/e2e/` runs whole programs through the checker and the native backend.
