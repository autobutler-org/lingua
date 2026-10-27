//! Typed IR (docs/lingua1.md, Data Model > Symbols and resolved types, Typed IR): the
//! contract between the typechecker, which produces it, and the interpreter and
//! `lingua build`, which consume it. `crate::check` produces it.
//!
//! Additions to lingua1.md's listing, each needed to lower or report:
//! - `TirKind::Concat` for text with holes (lingua1.md lowers holes but names no node).
//! - `TirKind::Let`, which binds temporaries so arguments written out of parameter order are
//!   still evaluated in source order (lingua1.md, Operator precedence and associativity).
//! - Spans on `Assert` and `Fail`, for the exception report, and on `TirFn`, for `L077`.
//! - `Variant.tag`, the variant's declaration index (docs/build.md, Value representation).
//! - `TirProgram` and `TirModule`, which carry each module's printed path and source text,
//!   so fault and exception reports can be rendered at build time.
//! - `TirModule.examples` and `TirExample.body`, for package examples.
//!
//! A folded `const` or `comptime` expression is not a node: the checker replaces each use with
//! the value as literal TIR (`Int`, `Text`, `List`, `Record`, `Variant`), so the interpreter and
//! lowering see an ordinary literal.
//!
//! `Quote` and `Unquote` are one node, `TirKind::Quote(template, holes)`: the template is the
//! quoted AST with each `unquote` replaced by a numbered hole, and `holes` is the TIR of each
//! `unquote` operand in hole order (lingua1.md's `Unquote(TirExpr)`). A comptime `Expr`
//! value as literal TIR is a `Quote` with no holes. Both are comptime IR only and never reach
//! the runtime residue. `Type::ExprAst` and `Type::TypeAst` are the types of the comptime
//! values.
//!
//! Conventions the backend relies on:
//! - Call arguments, record fields, and prelude arguments are in declaration order; names are
//!   resolved away. `split(text: t, separator: s)` is `Call(prelude::SPLIT, [t, s])`.
//!   When reordering would move an argument that is not a literal or local past another such
//!   argument, the checker wraps the call in a `Let` that evaluates those arguments in source
//!   order: `split(separator: f(), text: g())` is
//!   `Let([(s, f()), (t, g())], Call(prelude::SPLIT, [Local(t), Local(s)]))`.
//! - Prelude symbols are the fixed ids in [`prelude`]; a symbol table starts with
//!   [`prelude::symbols`] and appends user symbols after it.
//! - `log(e)` is `TirKind::Log`, not a `Call` of a prelude symbol.

use crate::ast::{Effect, Span};

/// A dense index into `TirProgram.symbols`.
pub type SymbolId = u32;

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    /// Filled by the typechecker. Functions have their return type here.
    pub ty: Type,
    /// Functions and resource ops.
    pub effect: Option<Effect>,
    /// `var` only.
    pub mutable: bool,
    pub span: Span,
    /// The module key (`"order"`), or `"prelude"`.
    pub module: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Type(TypeKind),
    /// `tag` is the variant's index in its sum's declaration order (`Ok` 0, `Err` 1).
    Variant {
        parent: SymbolId,
        tag: u32,
        payload: Option<Type>,
    },
    Field {
        parent: SymbolId,
        index: u32,
    },
    Function,
    Const,
    Resource,
    ResourceOp {
        resource: SymbolId,
    },
    Param,
    Local,
    PreludeFn,
    PreludeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Product,
    Sum,
    Newtype { base: Type, range: Option<Range> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Text,
    Bool,
    Nothing,
    Nominal(SymbolId),
    List(Box<Type>),
    Result(Box<Type>, Box<Type>),
    /// Anonymous integer bound; `hi` may be `i64::MAX`.
    Bound {
        lo: i64,
        hi: i64,
    },
    /// Comptime only; runtime use is `L063`.
    ExprAst,
    TypeAst,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub lo: i64,
    pub hi: i64,
}

/// The link set: every module's functions, sharing one symbol table.
#[derive(Debug, PartialEq)]
pub struct TirProgram {
    pub symbols: Vec<Symbol>,
    /// In link order: the import closure first, the entry's module last.
    pub modules: Vec<TirModule>,
    /// The entry `main`.
    pub main: SymbolId,
}

#[derive(Debug, PartialEq)]
pub struct TirModule {
    pub key: String,
    /// The path as the checker prints it in diagnostics (`order.lf`).
    pub path: String,
    /// The source text, for the source line in exception reports.
    pub source: String,
    /// The runtime residue, in source order.
    pub functions: Vec<TirFn>,
    /// Package examples, in source order.
    pub examples: Vec<TirExample>,
}

#[derive(Debug, PartialEq)]
pub struct TirFn {
    pub id: SymbolId,
    /// The function header, which `L077` points at.
    pub span: Span,
    pub effect: Effect,
    pub params: Vec<SymbolId>,
    pub ret: Type,
    pub body: TirBody,
    pub examples: Vec<TirExample>,
}

#[derive(Debug, PartialEq)]
pub enum TirBody {
    Primitive,
    Stmts(Vec<TirStmt>),
}

#[derive(Debug, PartialEq)]
pub struct TirExample {
    pub label: String,
    pub span: Span,
    pub kind: ExampleKind,
    pub inputs: Vec<(SymbolId, TirExpr)>,
    /// The resource or clock list.
    pub script: Option<TirExpr>,
    /// A package example's statements; empty for a function example.
    pub body: Vec<TirStmt>,
    pub expect: TirExpect,
}

#[derive(Debug, PartialEq)]
pub enum ExampleKind {
    Function,
    Package,
}

#[derive(Debug, PartialEq)]
pub enum TirExpect {
    Value(TirExpr),
    Exception { message: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TirStmt {
    /// `let` and `var`; `Symbol.mutable` tells them apart.
    Let(SymbolId, TirExpr),
    /// `+=` is already `Set(x, Add(Local(x), e))`.
    Set(SymbolId, TirExpr),
    Return(TirExpr),
    /// The message is a hole-free literal. `span` is what the exception report points at.
    Assert {
        cond: TirExpr,
        message: String,
        span: Span,
    },
    Fail {
        message: String,
        span: Span,
    },
    If(TirExpr, Vec<TirStmt>, Vec<TirStmt>),
    ForList(SymbolId, TirExpr, Vec<TirStmt>),
    /// `for i in low until high`: `low` and `high` are evaluated once.
    ForRange(SymbolId, TirExpr, TirExpr, Vec<TirStmt>),
    WhileIs {
        scrutinee: SymbolId,
        variant: SymbolId,
        payload: Option<SymbolId>,
        body: Vec<TirStmt>,
    },
    /// Statement arms; every arm's body is `Stmts`.
    Match(TirExpr, Vec<TirArm>),
    /// Type `Nothing`.
    Expr(TirExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TirExpr {
    pub ty: Type,
    pub span: Span,
    pub kind: TirKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TirKind {
    Int(i64),
    Text(String),
    Bool(bool),
    Nothing,
    Local(SymbolId),
    List(Vec<TirExpr>),
    /// A product, fields in declaration order.
    Record(SymbolId, Vec<TirExpr>),
    Variant(SymbolId, Option<Box<TirExpr>>),
    /// A user function or a prelude function (`trim`, `lower`, `split`, `join`, `length`,
    /// `max`), arguments in parameter order.
    Call(SymbolId, Vec<TirExpr>),
    /// Field index in declaration order.
    Field(Box<TirExpr>, u32),
    /// `checked` is false only for the static in-bounds case; then `ty` is the element type,
    /// otherwise `Result<T, BoundsError>`.
    Index {
        list: Box<TirExpr>,
        index: Box<TirExpr>,
        checked: bool,
    },
    Neg(Box<TirExpr>),
    Add(Box<TirExpr>, Box<TirExpr>),
    Sub(Box<TirExpr>, Box<TirExpr>),
    Mul(Box<TirExpr>, Box<TirExpr>),
    Div(Box<TirExpr>, Box<TirExpr>),
    Rem(Box<TirExpr>, Box<TirExpr>),
    Cmp(CmpOp, Box<TirExpr>, Box<TirExpr>),
    /// Expression arms (an arm may still be `Stmts` that diverge, as in `or return`).
    Match(Box<TirExpr>, Vec<TirArm>),
    /// Text with holes, in order. Each part is `Text`, nominal text, or an integer type;
    /// integer parts render as decimal.
    Concat(Vec<TirExpr>),
    IntIn(Box<TirExpr>, i64, i64),
    IntAtLeast(Box<TirExpr>, i64),
    ClockNow,
    /// Comptime only; never in the runtime residue.
    FsRead(Box<TirExpr>),
    /// Comptime only; never in the runtime residue.
    CatalogColumns(Box<TirExpr>),
    /// A folded `Type` (a generated record or sum). Comptime only; never in the runtime
    /// residue.
    TypeValue(crate::provider::Generated),
    /// `quote` with its `unquote` operands, in hole order (see the module docs). Comptime
    /// only; never in the runtime residue.
    Quote(Box<crate::ast::QuoteBody>, Vec<TirExpr>),
    /// Rejected by `L072` before lowering.
    ResourceCall(SymbolId, Vec<TirExpr>),
    Log(Box<TirExpr>),
    /// Evaluates each binding in order, binding its local, then the body; the body's value.
    /// Only the checker's argument temporaries use it.
    Let(Vec<(SymbolId, TirExpr)>, Box<TirExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TirArm {
    pub span: Span,
    pub pat: TirPat,
    pub body: TirArmBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TirArmBody {
    Expr(TirExpr),
    Stmts(Vec<TirStmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TirPat {
    Wild,
    Bind(SymbolId),
    /// `Ok` and `Err` are the prelude variants [`prelude::OK`] and [`prelude::ERR`].
    Variant(SymbolId, Option<Box<TirPat>>),
    LitInt(i64),
    LitText(String),
    List(Vec<TirPat>),
}

/// The prelude's runtime symbols at fixed ids, so every producer and consumer of TIR agrees.
pub mod prelude {
    use super::{Symbol, SymbolId, SymbolKind, Type, TypeKind};
    use crate::ast::Span;
    use crate::lex::Location;

    pub const RESULT: SymbolId = 0;
    pub const OK: SymbolId = 1;
    pub const ERR: SymbolId = 2;
    pub const BOUNDS_ERROR: SymbolId = 3;
    pub const OUT_OF_RANGE: SymbolId = 4;
    pub const NOT_AN_INTEGER: SymbolId = 5;
    pub const PROCESS: SymbolId = 6;
    pub const THREAD: SymbolId = 7;
    pub const TRIM: SymbolId = 8;
    pub const LOWER: SymbolId = 9;
    pub const SPLIT: SymbolId = 10;
    pub const JOIN: SymbolId = 11;
    pub const MAX: SymbolId = 12;
    pub const LENGTH: SymbolId = 13;
    /// `Process`'s fields follow at `PROCESS_FIELD + i`, then `Thread`'s `id` and `name`.
    pub const PROCESS_FIELD: SymbolId = 14;
    pub const THREAD_FIELD: SymbolId = 21;
    pub const COLUMN: SymbolId = 23;
    /// `Column`'s `name` and `sql_type`.
    pub const COLUMN_FIELD: SymbolId = 24;
    /// Comptime only: its result is a `Type`.
    pub const RECORD_FROM_COLUMNS: SymbolId = 26;
    pub const SUM_VARIANT: SymbolId = 27;
    /// `SumVariant`'s `name` and `payload`.
    pub const SUM_VARIANT_FIELD: SymbolId = 28;
    pub const RECORD_FIELD: SymbolId = 30;
    /// `RecordField`'s `name` and `type_text`.
    pub const RECORD_FIELD_FIELD: SymbolId = 31;
    /// Comptime only: its result is a `Type`.
    pub const RECORD_FROM_FIELDS: SymbolId = 33;
    /// Comptime only: its result is a `Type`.
    pub const SUM_FROM_VARIANTS: SymbolId = 34;
    /// The first id after the prelude.
    pub const END: SymbolId = 35;

    /// `Process` fields in prelude order.
    pub const PROCESS_FIELDS: [&str; 7] = [
        "program",
        "args",
        "os",
        "arch",
        "pid",
        "thread",
        "parallelism",
    ];

    const NOWHERE: Span = Span {
        start: Location { line: 0, column: 0 },
        end: Location { line: 0, column: 0 },
    };

    fn symbol(name: &str, kind: SymbolKind, ty: Type) -> Symbol {
        Symbol {
            name: name.to_owned(),
            kind,
            ty,
            effect: None,
            mutable: false,
            span: NOWHERE,
            module: "prelude".to_owned(),
        }
    }

    fn variant(name: &str, parent: SymbolId, tag: u32, payload: Option<Type>) -> Symbol {
        symbol(
            name,
            SymbolKind::Variant {
                parent,
                tag,
                payload,
            },
            Type::Nominal(parent),
        )
    }

    /// The prelude symbols, indexed by the constants above. `Result`'s payload types are
    /// per use, so `Ok` and `Err` record none here.
    #[must_use]
    pub fn symbols() -> Vec<Symbol> {
        let function = |name| symbol(name, SymbolKind::PreludeFn, Type::Nothing);
        let at_least = |lo| Type::Bound { lo, hi: i64::MAX };
        let field = |name, parent, index, ty| symbol(name, SymbolKind::Field { parent, index }, ty);
        let process_types = [
            Type::Text,
            Type::List(Box::new(Type::Text)),
            Type::Text,
            Type::Text,
            at_least(1),
            Type::Nominal(THREAD),
            at_least(1),
        ];
        let fields = PROCESS_FIELDS
            .into_iter()
            .zip(process_types)
            .zip(0..)
            .map(|((name, ty), index)| field(name, PROCESS, index, ty))
            .chain([
                field("id", THREAD, 0, Type::Int),
                field("name", THREAD, 1, Type::Text),
            ]);
        let mut symbols = vec![
            symbol("Result", SymbolKind::PreludeType, Type::Nothing),
            variant("Ok", RESULT, 0, None),
            variant("Err", RESULT, 1, None),
            symbol(
                "BoundsError",
                SymbolKind::Type(TypeKind::Sum),
                Type::Nominal(BOUNDS_ERROR),
            ),
            variant("OutOfRange", BOUNDS_ERROR, 0, None),
            variant("NotAnInteger", BOUNDS_ERROR, 1, None),
            symbol(
                "Process",
                SymbolKind::Type(TypeKind::Product),
                Type::Nominal(PROCESS),
            ),
            symbol(
                "Thread",
                SymbolKind::Type(TypeKind::Product),
                Type::Nominal(THREAD),
            ),
            function("trim"),
            function("lower"),
            function("split"),
            function("join"),
            function("max"),
            Symbol {
                ty: at_least(0),
                ..function("length")
            },
        ];
        symbols.extend(fields);
        symbols.extend([
            symbol(
                "Column",
                SymbolKind::Type(TypeKind::Product),
                Type::Nominal(COLUMN),
            ),
            field("name", COLUMN, 0, Type::Text),
            field("sql_type", COLUMN, 1, Type::Text),
            Symbol {
                ty: Type::TypeAst,
                ..function("record_from_columns")
            },
            symbol(
                "SumVariant",
                SymbolKind::Type(TypeKind::Product),
                Type::Nominal(SUM_VARIANT),
            ),
            field("name", SUM_VARIANT, 0, Type::Text),
            field("payload", SUM_VARIANT, 1, Type::Text),
            symbol(
                "RecordField",
                SymbolKind::Type(TypeKind::Product),
                Type::Nominal(RECORD_FIELD),
            ),
            field("name", RECORD_FIELD, 0, Type::Text),
            field("type_text", RECORD_FIELD, 1, Type::Text),
            Symbol {
                ty: Type::TypeAst,
                ..function("record_from_fields")
            },
            Symbol {
                ty: Type::TypeAst,
                ..function("sum_from_variants")
            },
        ]);
        symbols
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn the_constants_index_the_symbols() {
            let symbols = symbols();
            let name = |id: SymbolId| symbols[id as usize].name.as_str();
            assert_eq!(symbols.len(), END as usize);
            assert_eq!(
                [OK, ERR, OUT_OF_RANGE, NOT_AN_INTEGER, SPLIT, LENGTH].map(name),
                ["Ok", "Err", "OutOfRange", "NotAnInteger", "split", "length"]
            );
            assert_eq!(name(PROCESS_FIELD + 6), "parallelism");
            assert_eq!([THREAD_FIELD, THREAD_FIELD + 1].map(name), ["id", "name"]);
            assert_eq!(
                [COLUMN, COLUMN_FIELD + 1, RECORD_FROM_COLUMNS].map(name),
                ["Column", "sql_type", "record_from_columns"]
            );
            assert_eq!(
                [
                    SUM_VARIANT,
                    SUM_VARIANT_FIELD + 1,
                    RECORD_FIELD,
                    RECORD_FIELD_FIELD + 1,
                    RECORD_FROM_FIELDS,
                    SUM_FROM_VARIANTS
                ]
                .map(name),
                [
                    "SumVariant",
                    "payload",
                    "RecordField",
                    "type_text",
                    "record_from_fields",
                    "sum_from_variants"
                ]
            );
        }
    }
}
