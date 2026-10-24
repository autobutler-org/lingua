//! Typed IR (docs/lingua1.md, Data Model > Typed IR), cut down to the forms `lingua build`
//! accepts so far. The typechecker will produce TIR; until it exists, `build::subset` does.
//! Grow these toward the full shapes rather than adding parallel ones.

use crate::ast::Span;

/// A function in the runtime residue. `name` is its symbol, `<module>.<name>`.
#[derive(Debug, PartialEq)]
pub struct TirFn {
    pub name: String,
    pub body: Vec<TirStmt>,
}

#[derive(Debug, PartialEq)]
pub enum TirStmt {
    Return(TirExpr),
    /// An expression of type `Nothing`.
    Expr(TirExpr),
}

// ponytail: no `ty` field yet; every expression's type is implied by its kind. Add it with the
// typechecker, when lowering first needs to tell types apart (text equality, nominals).
#[derive(Debug, PartialEq)]
pub struct TirExpr {
    pub span: Span,
    pub kind: TirKind,
}

#[derive(Debug, PartialEq)]
pub enum TirKind {
    Text(String),
    Nothing,
    Log(Box<TirExpr>),
}
