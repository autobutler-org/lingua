//! Untyped AST from the parser. Names are strings; sugar (`or return`, `+=`, examples) is kept.

mod dump;

pub use dump::dump;

use crate::lex::Location;

// The file is implied by the module being parsed; diagnostics add the path
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub span: Span,
    pub key: String,
    pub imports: Vec<Import>,
    pub decls: Vec<Decl>,
}

#[derive(Debug, PartialEq)]
pub struct Import {
    pub span: Span,
    pub names: Vec<String>,
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Alias {
        span: Span,
        name: String,
        ty: TypeExpr,
    },
    Product {
        span: Span,
        name: String,
        fields: Vec<Field>,
    },
    Sum {
        span: Span,
        name: String,
        variants: Vec<Variant>,
    },
    Generative {
        span: Span,
        name: String,
        effect: Effect,
        body: Vec<Stmt>,
    },
    Resource {
        span: Span,
        name: String,
        op: ResourceOp,
    },
    Function {
        span: Span,
        name: String,
        params: Vec<Param>,
        ret: TypeExpr,
        effect: Effect,
        examples: Vec<Example>,
        body: FnBody,
    },
    Const {
        span: Span,
        name: String,
        ty: Option<TypeExpr>,
        effect: Effect,
        init: Expr,
    },
    PackageExample {
        span: Span,
        example: Example,
    },
    ComptimeBlock {
        span: Span,
        effect: Effect,
        body: Vec<Stmt>,
    },
    // Top-level only; always L067 later
    Provide {
        span: Span,
        name: String,
        init: Expr,
    },
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub span: Span,
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub span: Span,
    pub name: String,
    pub payload: Option<TypeExpr>,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub span: Span,
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, PartialEq)]
pub struct ResourceOp {
    pub span: Span,
    pub name: String,
    pub params: Vec<Param>,
    pub ret: TypeExpr,
    pub effect: ResourceEffect,
}

#[derive(Debug, PartialEq)]
pub enum ResourceEffect {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub enum FnBody {
    Primitive,
    Statements(Vec<Stmt>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    Pure,
    ReadsClock,
    ReadsNet,
    ReadsFs,
    ReadsCatalog,
    Reads(String),
    Writes(String),
}

#[derive(Debug, PartialEq)]
pub enum Expect {
    Value(Expr),
    Exception { message: String },
}

// A package example's statements live in `body`; a function example's are always empty
#[derive(Debug, PartialEq)]
pub struct Example {
    pub span: Span,
    pub label: String,
    pub bindings: Vec<Bind>,
    pub body: Vec<Stmt>,
    pub expect: Expect,
}

#[derive(Debug, PartialEq)]
pub struct Bind {
    pub span: Span,
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, PartialEq)]
pub enum TypeExpr {
    Named(Span, String),
    Apply(Span, String, Vec<TypeExpr>),
    AtLeast(Span, Box<TypeExpr>, Expr),
    Range(Span, Box<TypeExpr>, Expr, Expr),
}

impl TypeExpr {
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            TypeExpr::Named(span, _)
            | TypeExpr::Apply(span, ..)
            | TypeExpr::AtLeast(span, ..)
            | TypeExpr::Range(span, ..) => *span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AssignOp {
    Assign,
    AddAssign,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let {
        span: Span,
        name: String,
        ty: Option<TypeExpr>,
        init: Expr,
    },
    Var {
        span: Span,
        name: String,
        ty: Option<TypeExpr>,
        init: Expr,
    },
    Const {
        span: Span,
        name: String,
        ty: Option<TypeExpr>,
        init: Expr,
    },
    Set {
        span: Span,
        name: String,
        op: AssignOp,
        value: Expr,
    },
    Return {
        span: Span,
        value: Expr,
    },
    Assert {
        span: Span,
        cond: Expr,
        message: Expr,
    },
    Fail {
        span: Span,
        message: Expr,
    },
    If {
        span: Span,
        cond: Expr,
        then: Vec<Stmt>,
        otherwise: Option<Vec<Stmt>>,
    },
    ForList {
        span: Span,
        name: String,
        iter: Expr,
        body: Vec<Stmt>,
    },
    ForRange {
        span: Span,
        name: String,
        low: Expr,
        high: Expr,
        body: Vec<Stmt>,
    },
    WhileIs {
        span: Span,
        name: String,
        pattern: Pattern,
        body: Vec<Stmt>,
    },
    // Legal only as a direct child of a package `ComptimeBlock`; L067 later otherwise
    Provide {
        span: Span,
        name: String,
        init: Expr,
    },
    ExprStmt {
        span: Span,
        expr: Expr,
    },
}

// `broken` records a newline inside the delimiters, for the formatter
#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLit(Span, i64),
    Text(Span, Vec<TextPart>),
    BoolLit(Span, bool),
    NothingLit(Span),
    Name(Span, String),
    ListLit(Span, Vec<Expr>, bool),
    Call(Span, Box<Expr>, Vec<Arg>, bool),
    Field(Span, Box<Expr>, String),
    Index(Span, Box<Expr>, Box<Expr>),
    UnaryNeg(Span, Box<Expr>),
    Binary(Span, BinOp, Box<Expr>, Box<Expr>),
    Comptime(Span, Box<Expr>),
    OrReturn(Span, Box<Expr>, Option<Box<Expr>>),
    OrFail(Span, Box<Expr>, Box<Expr>),
    Match(Span, Box<Expr>, Vec<Arm>, ArmKind),
    IntIn(Span, Box<Expr>, Box<Expr>, Box<Expr>),
    IntAtLeast(Span, Box<Expr>, Box<Expr>),
    Quote(Span, QuoteBody),
    Unquote(Span, Box<Expr>),
    ClockNow(Span),
    FsRead(Span, Box<Expr>, bool, bool),         // named, broken
    CatalogColumns(Span, Box<Expr>, bool, bool), // named, broken
}

impl Expr {
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Expr::IntLit(span, ..)
            | Expr::Text(span, ..)
            | Expr::BoolLit(span, ..)
            | Expr::NothingLit(span)
            | Expr::Name(span, ..)
            | Expr::ListLit(span, ..)
            | Expr::Call(span, ..)
            | Expr::Field(span, ..)
            | Expr::Index(span, ..)
            | Expr::UnaryNeg(span, ..)
            | Expr::Binary(span, ..)
            | Expr::Comptime(span, ..)
            | Expr::OrReturn(span, ..)
            | Expr::OrFail(span, ..)
            | Expr::Match(span, ..)
            | Expr::IntIn(span, ..)
            | Expr::IntAtLeast(span, ..)
            | Expr::Quote(span, ..)
            | Expr::Unquote(span, ..)
            | Expr::ClockNow(span)
            | Expr::FsRead(span, ..)
            | Expr::CatalogColumns(span, ..) => *span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum QuoteBody {
    QuotedStmt(Box<Stmt>),
    QuotedExpr(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum TextPart {
    Literal(String),
    Hole(Expr),
}

#[derive(Debug, PartialEq)]
pub struct Arg {
    pub span: Span,
    pub name: Option<String>,
    pub value: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Arm {
    pub span: Span,
    pub pattern: Pattern,
    pub body: ArmBody,
}

#[derive(Debug, PartialEq)]
pub enum ArmKind {
    ExprArms,
    StmtArms,
}

#[derive(Debug, PartialEq)]
pub enum ArmBody {
    ExprArm(Expr),
    StmtArm(Vec<Stmt>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Wild(Span),
    LitInt(Span, i64),
    LitText(Span, String),
    Bind(Span, String),
    OkPat(Span, Box<Pattern>),
    ErrPat(Span, Box<Pattern>),
    VariantPat(Span, String, Option<Box<Pattern>>),
    ListPat(Span, Vec<Pattern>),
}
