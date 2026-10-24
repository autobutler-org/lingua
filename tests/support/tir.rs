//! Helpers that build TIR programs by hand, for the backend's tests until the typechecker
//! produces TIR from source. Shared by `src/build/lower_tests.rs` and `tests/build_tir.rs`.
#![allow(dead_code)]

use lingua::ast::{Effect, Span};
use lingua::lex::Location;
use lingua::tir::{
    CmpOp, Symbol, SymbolId, SymbolKind, TirArm, TirArmBody, TirBody, TirExpr, TirFn, TirKind,
    TirModule, TirPat, TirProgram, TirStmt, Type, TypeKind, prelude,
};

#[must_use]
pub fn span(line: usize, column: usize, len: usize) -> Span {
    Span {
        start: Location { line, column },
        end: Location {
            line,
            column: column + len,
        },
    }
}

const AT: Span = Span {
    start: Location { line: 1, column: 1 },
    end: Location { line: 1, column: 2 },
};

/// A program under construction: the symbol table and each module's functions.
pub struct Program {
    pub symbols: Vec<Symbol>,
    modules: Vec<TirModule>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            symbols: prelude::symbols(),
            modules: Vec::new(),
        }
    }
}

impl Program {
    /// Adds module `key` (file `<key>.lf`) with `source`, for exception source lines.
    pub fn module(&mut self, key: &str, source: &str) {
        self.modules.push(TirModule {
            key: key.to_owned(),
            path: format!("{key}.lf"),
            source: source.to_owned(),
            functions: Vec::new(),
        });
    }

    fn current(&self) -> String {
        self.modules.last().expect("add a module first").key.clone()
    }

    pub fn symbol(&mut self, name: &str, kind: SymbolKind, ty: Type) -> SymbolId {
        self.symbols.push(Symbol {
            name: name.to_owned(),
            kind,
            ty,
            effect: None,
            mutable: false,
            span: AT,
            module: self.current(),
        });
        SymbolId::try_from(self.symbols.len() - 1).unwrap()
    }

    pub fn local(&mut self, name: &str, ty: Type) -> SymbolId {
        self.symbol(name, SymbolKind::Local, ty)
    }

    pub fn param(&mut self, name: &str, ty: Type) -> SymbolId {
        self.symbol(name, SymbolKind::Param, ty)
    }

    /// Declares a function in the current module, so calls can name it before its body.
    pub fn declare(&mut self, name: &str, ret: Type) -> SymbolId {
        self.symbol(name, SymbolKind::Function, ret)
    }

    /// Defines a declared function in the current module; `line` is its header line.
    pub fn define(&mut self, id: SymbolId, line: usize, params: Vec<SymbolId>, body: TirBody) {
        let ret = self.symbols[id as usize].ty.clone();
        let function = TirFn {
            id,
            span: span(line, 1, 2),
            effect: Effect::Pure,
            params,
            ret,
            body,
            examples: Vec::new(),
        };
        self.modules.last_mut().unwrap().functions.push(function);
    }

    /// `type name` with `variants` (name, payload), tags in order.
    pub fn sum(&mut self, name: &str, variants: &[(&str, Option<Type>)]) -> Vec<SymbolId> {
        let parent = self.symbol(name, SymbolKind::Type(TypeKind::Sum), Type::Nothing);
        self.symbols[parent as usize].ty = Type::Nominal(parent);
        variants
            .iter()
            .zip(0..)
            .map(|((variant, payload), tag)| {
                self.symbol(
                    variant,
                    SymbolKind::Variant {
                        parent,
                        tag,
                        payload: payload.clone(),
                    },
                    Type::Nominal(parent),
                )
            })
            .collect()
    }

    /// `type name = base`.
    pub fn newtype(&mut self, name: &str, base: Type) -> SymbolId {
        self.symbol(
            name,
            SymbolKind::Type(TypeKind::Newtype { base, range: None }),
            Type::Nothing,
        )
    }

    #[must_use]
    pub fn finish(self, main: SymbolId) -> TirProgram {
        TirProgram {
            symbols: self.symbols,
            modules: self.modules,
            main,
        }
    }
}

/// `fn main() -> Nothing` with `body` then `return nothing`, alone in module `m`.
#[must_use]
pub fn main_program(source: &str, body: Vec<TirStmt>) -> TirProgram {
    let mut p = Program::default();
    p.module("m", source);
    let main = p.declare("main", Type::Nothing);
    p.define(main, 1, vec![], stmts(body));
    p.finish(main)
}

#[must_use]
pub fn stmts(mut body: Vec<TirStmt>) -> TirBody {
    body.push(TirStmt::Return(nothing()));
    TirBody::Stmts(body)
}

pub fn expr(ty: Type, kind: TirKind) -> TirExpr {
    TirExpr { ty, span: AT, kind }
}

/// Moves an expression to a span.
#[must_use]
pub fn at(mut e: TirExpr, line: usize, column: usize, len: usize) -> TirExpr {
    e.span = span(line, column, len);
    e
}

pub fn int(n: i64) -> TirExpr {
    expr(Type::Int, TirKind::Int(n))
}

pub fn boolean(b: bool) -> TirExpr {
    expr(Type::Bool, TirKind::Bool(b))
}

pub fn text(s: &str) -> TirExpr {
    expr(Type::Text, TirKind::Text(s.to_owned()))
}

pub fn nothing() -> TirExpr {
    expr(Type::Nothing, TirKind::Nothing)
}

pub fn local(id: SymbolId, ty: Type) -> TirExpr {
    expr(ty, TirKind::Local(id))
}

pub fn add(a: TirExpr, b: TirExpr) -> TirExpr {
    expr(Type::Int, TirKind::Add(Box::new(a), Box::new(b)))
}

pub fn sub(a: TirExpr, b: TirExpr) -> TirExpr {
    expr(Type::Int, TirKind::Sub(Box::new(a), Box::new(b)))
}

pub fn mul(a: TirExpr, b: TirExpr) -> TirExpr {
    expr(Type::Int, TirKind::Mul(Box::new(a), Box::new(b)))
}

pub fn div(a: TirExpr, b: TirExpr) -> TirExpr {
    expr(Type::Int, TirKind::Div(Box::new(a), Box::new(b)))
}

pub fn rem(a: TirExpr, b: TirExpr) -> TirExpr {
    expr(Type::Int, TirKind::Rem(Box::new(a), Box::new(b)))
}

pub fn neg(a: TirExpr) -> TirExpr {
    expr(Type::Int, TirKind::Neg(Box::new(a)))
}

pub fn cmp(op: CmpOp, a: TirExpr, b: TirExpr) -> TirExpr {
    expr(Type::Bool, TirKind::Cmp(op, Box::new(a), Box::new(b)))
}

pub fn call(f: SymbolId, args: Vec<TirExpr>, ty: Type) -> TirExpr {
    expr(ty, TirKind::Call(f, args))
}

pub fn list(elem: Type, elems: Vec<TirExpr>) -> TirExpr {
    expr(Type::List(Box::new(elem)), TirKind::List(elems))
}

pub fn field(e: TirExpr, index: u32, ty: Type) -> TirExpr {
    expr(ty, TirKind::Field(Box::new(e), index))
}

pub fn variant(v: SymbolId, payload: Option<TirExpr>, ty: Type) -> TirExpr {
    expr(ty, TirKind::Variant(v, payload.map(Box::new)))
}

pub fn result_ty(ok: Type, err: Type) -> Type {
    Type::Result(Box::new(ok), Box::new(err))
}

pub fn bounds_result(ok: Type) -> Type {
    result_ty(ok, Type::Nominal(prelude::BOUNDS_ERROR))
}

pub fn index(list: TirExpr, i: TirExpr, checked: bool, ty: Type) -> TirExpr {
    expr(
        ty,
        TirKind::Index {
            list: Box::new(list),
            index: Box::new(i),
            checked,
        },
    )
}

pub fn concat(parts: Vec<TirExpr>) -> TirExpr {
    expr(Type::Text, TirKind::Concat(parts))
}

pub fn log(e: TirExpr) -> TirStmt {
    TirStmt::Expr(expr(Type::Nothing, TirKind::Log(Box::new(e))))
}

/// `log("{e}")` for an integer `e`.
pub fn log_int(e: TirExpr) -> TirStmt {
    log(concat(vec![e]))
}

pub fn let_(x: SymbolId, e: TirExpr) -> TirStmt {
    TirStmt::Let(x, e)
}

pub fn set(x: SymbolId, e: TirExpr) -> TirStmt {
    TirStmt::Set(x, e)
}

pub fn ret(e: TirExpr) -> TirStmt {
    TirStmt::Return(e)
}

pub fn arm(pat: TirPat, e: TirExpr) -> TirArm {
    TirArm {
        span: AT,
        pat,
        body: TirArmBody::Expr(e),
    }
}

pub fn stmt_arm(pat: TirPat, body: Vec<TirStmt>) -> TirArm {
    TirArm {
        span: AT,
        pat,
        body: TirArmBody::Stmts(body),
    }
}

pub fn pvariant(v: SymbolId, inner: Option<TirPat>) -> TirPat {
    TirPat::Variant(v, inner.map(Box::new))
}

pub fn match_(scrutinee: TirExpr, arms: Vec<TirArm>, ty: Type) -> TirExpr {
    expr(ty, TirKind::Match(Box::new(scrutinee), arms))
}

/// `e or return`: `Ok(v) -> v`, `Err(err) -> return Err(err)`.
pub fn or_return(p: &mut Program, e: TirExpr, ok: Type, err: Type, ret: Type) -> TirExpr {
    let (v, e_sym) = (p.local("v", ok.clone()), p.local("err", err.clone()));
    let back = variant(prelude::ERR, Some(local(e_sym, err)), ret);
    match_(
        e,
        vec![
            arm(
                pvariant(prelude::OK, Some(TirPat::Bind(v))),
                local(v, ok.clone()),
            ),
            stmt_arm(
                pvariant(prelude::ERR, Some(TirPat::Bind(e_sym))),
                vec![ret_stmt(back)],
            ),
        ],
        ok,
    )
}

fn ret_stmt(e: TirExpr) -> TirStmt {
    TirStmt::Return(e)
}

/// `e or fail "message"` with the `fail` at `span`.
pub fn or_fail(p: &mut Program, e: TirExpr, ok: Type, message: &str, span: Span) -> TirExpr {
    let v = p.local("v", ok.clone());
    match_(
        e,
        vec![
            arm(
                pvariant(prelude::OK, Some(TirPat::Bind(v))),
                local(v, ok.clone()),
            ),
            stmt_arm(
                pvariant(prelude::ERR, Some(TirPat::Wild)),
                vec![TirStmt::Fail {
                    message: message.to_owned(),
                    span,
                }],
            ),
        ],
        ok,
    )
}
