//! Integer ranges, elimination (`S <: T`), and how types print in diagnostics
//! (docs/lingua1.md, Types > Ranges, Elimination and introduction, Arithmetic).

use crate::tir::{Range, Symbol, SymbolKind, Type, TypeKind};

pub const FULL: Range = Range {
    lo: i64::MIN,
    hi: i64::MAX,
};

/// The anonymous type of a range: `Int` for the full range, else a bound.
pub fn of_range(r: Range) -> Type {
    if r == FULL {
        Type::Int
    } else {
        Type::Bound { lo: r.lo, hi: r.hi }
    }
}

pub fn contains(outer: Range, inner: Range) -> bool {
    outer.lo <= inner.lo && inner.hi <= outer.hi
}

/// The newtype behind a nominal, if `ty` is one.
fn newtype<'a>(symbols: &'a [Symbol], ty: &Type) -> Option<(&'a Type, Option<Range>)> {
    match ty {
        Type::Nominal(id) => match &symbols[*id as usize].kind {
            SymbolKind::Type(TypeKind::Newtype { base, range }) => Some((base, *range)),
            _ => None,
        },
        _ => None,
    }
}

/// The static range of an integer type (`Int`, a bound, or an integer nominal).
pub fn range(symbols: &[Symbol], ty: &Type) -> Option<Range> {
    match ty {
        Type::Int => Some(FULL),
        Type::Bound { lo, hi } => Some(Range { lo: *lo, hi: *hi }),
        _ => match newtype(symbols, ty) {
            Some((Type::Int, range)) => Some(range.unwrap_or(FULL)),
            _ => None,
        },
    }
}

pub fn is_int(symbols: &[Symbol], ty: &Type) -> bool {
    range(symbols, ty).is_some()
}

pub fn is_nominal_int(symbols: &[Symbol], ty: &Type) -> bool {
    matches!(newtype(symbols, ty), Some((Type::Int, _)))
}

pub fn is_nominal_text(symbols: &[Symbol], ty: &Type) -> bool {
    matches!(newtype(symbols, ty), Some((Type::Text, _)))
}

/// `Text` or a nominal text type.
pub fn is_textual(symbols: &[Symbol], ty: &Type) -> bool {
    *ty == Type::Text || is_nominal_text(symbols, ty)
}

/// `S <: T`: identity, an integer type into `Int` or a wider anonymous bound, and nominal
/// text into `Text`. Never into a nominal.
pub fn subtype(symbols: &[Symbol], s: &Type, t: &Type) -> bool {
    if s == t {
        return true;
    }
    match t {
        Type::Int => is_int(symbols, s),
        Type::Bound { lo, hi } => {
            range(symbols, s).is_some_and(|r| contains(Range { lo: *lo, hi: *hi }, r))
        }
        Type::Text => is_nominal_text(symbols, s),
        _ => false,
    }
}

fn bound_text(lo: i64, hi: i64) -> String {
    if hi == i64::MAX {
        format!("Int at least {lo}")
    } else {
        format!("Int in {lo} through {hi}")
    }
}

/// The type as source spells it: `Cents`, `List<Line>`, `Int in 1 through 100`.
pub fn show(symbols: &[Symbol], ty: &Type) -> String {
    match ty {
        Type::Int => "Int".into(),
        Type::Text => "Text".into(),
        Type::Bool => "Bool".into(),
        Type::Nothing => "Nothing".into(),
        Type::Nominal(id) => symbols[*id as usize].name.clone(),
        Type::List(t) => format!("List<{}>", show(symbols, t)),
        Type::Result(t, e) => format!("Result<{}, {}>", show(symbols, t), show(symbols, e)),
        Type::Bound { lo, hi } => bound_text(*lo, *hi),
        Type::ExprAst => "Expr".into(),
        Type::TypeAst => "Type".into(),
    }
}

/// Folds integer arithmetic for a comptime bound; `None` on overflow or a zero divisor.
pub fn fold(op: crate::ast::BinOp, a: i64, b: i64) -> Option<i64> {
    use crate::ast::BinOp;
    match op {
        BinOp::Add => a.checked_add(b),
        BinOp::Sub => a.checked_sub(b),
        BinOp::Mul => a.checked_mul(b),
        BinOp::Div => a.checked_div(b),
        BinOp::Rem => a.checked_rem(b),
        _ => None,
    }
}

/// The static result of `+ - * / %` from the operand ranges.
pub fn arith(op: crate::ast::BinOp, a: Range, b: Range) -> Type {
    use crate::ast::BinOp;
    let natural = |r: Range| r.lo >= 0;
    match op {
        BinOp::Add | BinOp::Mul if natural(a) && natural(b) => of_range(Range {
            lo: 0,
            hi: i64::MAX,
        }),
        BinOp::Div if b.lo == b.hi && b.lo >= 1 => of_range(Range {
            lo: a.lo / b.lo,
            hi: a.hi / b.lo,
        }),
        BinOp::Div if natural(a) && b.lo >= 1 => of_range(Range { lo: 0, hi: a.hi }),
        BinOp::Rem if b.lo >= 1 => {
            let top = b.hi - 1;
            of_range(Range {
                lo: if natural(a) { 0 } else { -top },
                hi: top,
            })
        }
        _ => Type::Int,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinOp;

    fn r(lo: i64, hi: i64) -> Range {
        Range { lo, hi }
    }

    #[test]
    fn arithmetic_bounds_follow_the_table() {
        let cents = r(0, i64::MAX);
        assert_eq!(
            arith(BinOp::Add, cents, r(1, i64::MAX)),
            Type::Bound {
                lo: 0,
                hi: i64::MAX
            }
        );
        assert_eq!(arith(BinOp::Sub, cents, cents), Type::Int);
        assert_eq!(arith(BinOp::Add, FULL, cents), Type::Int);
        assert_eq!(
            arith(BinOp::Rem, cents, r(100, 100)),
            Type::Bound { lo: 0, hi: 99 }
        );
        assert_eq!(
            arith(BinOp::Div, cents, r(100, 100)),
            Type::Bound {
                lo: 0,
                hi: 92_233_720_368_547_758
            }
        );
        assert_eq!(
            arith(BinOp::Rem, FULL, r(2, 2)),
            Type::Bound { lo: -1, hi: 1 }
        );
        assert_eq!(
            arith(BinOp::Div, cents, r(1, 9)),
            Type::Bound {
                lo: 0,
                hi: i64::MAX
            }
        );
        assert_eq!(arith(BinOp::Div, cents, FULL), Type::Int);
        assert_eq!(
            arith(BinOp::Div, r(0, 100), r(2, 2)),
            Type::Bound { lo: 0, hi: 50 }
        );
    }

    #[test]
    fn elimination_widens_but_never_narrows() {
        let s = crate::tir::prelude::symbols();
        let b = |lo, hi| Type::Bound { lo, hi };
        assert!(subtype(&s, &b(0, 10), &Type::Int));
        assert!(subtype(&s, &b(0, 10), &b(0, 100)));
        assert!(!subtype(&s, &b(0, 100), &b(0, 10)));
        assert!(!subtype(&s, &Type::Int, &b(0, 10)));
        assert!(!subtype(&s, &Type::Text, &Type::Int));
    }
}
