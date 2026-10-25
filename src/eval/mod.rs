//! The TIR interpreter and the function-example runner (docs/lingua1.md, Examples, Typed IR).
//! `crate::check` runs every function example on it after typechecking; a failing example is
//! a diagnostic. The runner trusts the types on the TIR and checks overflow, fuel, the heap,
//! and scripts dynamically.
//!
//! Faults render through `crate::report`, the renderer the native binary's pre-rendered
//! reports come from, so the two engines print the same block (docs/build.md, Parity).
//!
//! Values differ from lingua1.md's listing in one way: nominal integers and text are plain
//! `Int` and `Text`. Both sides of every comparison have the same static type, so the
//! nominal id never decides an equality the checker has not already decided.

#[cfg(test)]
mod tests;

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::ast::Span;
use crate::check::Diagnostic;
use crate::tir::{
    CmpOp, ExampleKind, Symbol, SymbolId, SymbolKind, TirArm, TirArmBody, TirBody, TirExample,
    TirExpect, TirExpr, TirFn, TirKind, TirModule, TirPat, TirStmt, prelude,
};

/// Steps per example: one per expression evaluated, statement entered, or loop back-edge.
const FUEL: u64 = 100_000;
/// Bytes per example, counted as the native heap would allocate them (docs/build.md, Value
/// representation) and never freed, like that bump heap.
// ponytail: cumulative allocation, not live bytes, so a loop that rebuilds a big list can hit
// `L047` while holding little; count live bytes (drop of `Rc`s) if a real example trips it.
const HEAP: usize = 64 << 20;
/// Nested user calls per example, reported as `L047`. Fuel alone would allow about 33 000.
// ponytail: a fixed cap instead of a stack probe; raise it with `STACK` if examples recurse
// deeper.
const DEPTH: usize = 10_000;
/// The runner's thread stack, sized so `DEPTH` nested calls fit.
const STACK: usize = 256 << 20;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Int(i64),
    Text(Rc<str>),
    Bool(bool),
    Nothing,
    List(Rc<[Value]>),
    Record(SymbolId, Rc<[Value]>),
    Variant(SymbolId, Option<Rc<Value>>),
}

/// Where a node is: a module index and a span in its source.
#[derive(Debug, Clone, Copy)]
struct At {
    module: usize,
    span: Span,
}

/// Why evaluation left the normal path. `Return` unwinds to the enclosing call.
#[derive(Debug)]
enum Stop {
    Return(Value),
    Exception { message: String, at: At },
    Fault { code: &'static str, at: At },
    Script { resource: String, at: At },
    Heap(At),
    Depth(At),
    Internal(String),
}

type Eval<T> = Result<T, Stop>;
type Env = HashMap<SymbolId, Value>;

const NOWHERE: Span = Span {
    start: crate::lex::Location { line: 0, column: 0 },
    end: crate::lex::Location { line: 0, column: 0 },
};

struct Machine<'a> {
    symbols: &'a [Symbol],
    modules: &'a [TirModule],
    functions: HashMap<SymbolId, (usize, &'a TirFn)>,
    fuel: u64,
    heap: usize,
    depth: usize,
    script: VecDeque<Value>,
    logs: Vec<String>,
    /// The module whose code is running.
    module: usize,
    /// The last expression entered, which a statement or back-edge out of fuel points at.
    last: Span,
}

fn internal(what: &str) -> Stop {
    Stop::Internal(what.to_owned())
}

impl<'a> Machine<'a> {
    fn new(symbols: &'a [Symbol], modules: &'a [TirModule]) -> Self {
        let functions = modules
            .iter()
            .enumerate()
            .flat_map(|(m, module)| module.functions.iter().map(move |f| (f.id, (m, f))))
            .collect();
        Machine {
            symbols,
            modules,
            functions,
            fuel: FUEL,
            heap: 0,
            depth: 0,
            script: VecDeque::new(),
            logs: Vec::new(),
            module: 0,
            last: NOWHERE,
        }
    }

    // A fresh root in module `m`
    fn reset(&mut self, m: usize, fuel: u64) {
        self.fuel = fuel;
        self.heap = 0;
        self.depth = 0;
        self.script.clear();
        self.logs.clear();
        self.module = m;
    }

    fn at(&self, span: Span) -> At {
        At {
            module: self.module,
            span,
        }
    }

    fn tick(&mut self) -> Eval<()> {
        if self.fuel == 0 {
            return Err(Stop::Fault {
                code: "L042",
                at: self.at(self.last),
            });
        }
        self.fuel -= 1;
        Ok(())
    }

    fn alloc(&mut self, span: Span, bytes: usize) -> Eval<()> {
        self.heap = self.heap.saturating_add(bytes);
        if self.heap > HEAP {
            return Err(Stop::Heap(self.at(span)));
        }
        Ok(())
    }

    fn text(&mut self, span: Span, text: String) -> Eval<Value> {
        self.alloc(span, 8 + text.len().next_multiple_of(8))?;
        Ok(Value::Text(text.into()))
    }

    fn list(&mut self, span: Span, items: Vec<Value>) -> Eval<Value> {
        self.alloc(span, 8 + 8 * items.len())?;
        Ok(Value::List(items.into()))
    }

    fn variant(&mut self, span: Span, id: SymbolId, payload: Option<Value>) -> Eval<Value> {
        if payload.is_some() {
            self.alloc(span, 16)?;
        }
        Ok(Value::Variant(id, payload.map(Rc::new)))
    }

    fn fault(&self, code: &'static str, span: Span) -> Stop {
        Stop::Fault {
            code,
            at: self.at(span),
        }
    }

    fn int(&mut self, e: &TirExpr, env: &mut Env) -> Eval<i64> {
        match self.eval(e, env)? {
            Value::Int(n) => Ok(n),
            v => Err(internal(&format!("expected an integer, found {v:?}"))),
        }
    }

    fn texts(&mut self, e: &TirExpr, env: &mut Env) -> Eval<Rc<str>> {
        match self.eval(e, env)? {
            Value::Text(t) => Ok(t),
            v => Err(internal(&format!("expected text, found {v:?}"))),
        }
    }

    fn items(&mut self, e: &TirExpr, env: &mut Env) -> Eval<Rc<[Value]>> {
        match self.eval(e, env)? {
            Value::List(items) => Ok(items),
            v => Err(internal(&format!("expected a list, found {v:?}"))),
        }
    }

    fn arith(
        &mut self,
        e: &TirExpr,
        env: &mut Env,
        a: &TirExpr,
        b: &TirExpr,
        op: fn(i64, i64) -> Option<i64>,
        divides: bool,
    ) -> Eval<Value> {
        let (a, b) = (self.int(a, env)?, self.int(b, env)?);
        if divides && b == 0 {
            return Err(self.fault("L076", e.span));
        }
        op(a, b)
            .map(Value::Int)
            .ok_or_else(|| self.fault("L040", e.span))
    }

    #[allow(clippy::too_many_lines)]
    fn eval(&mut self, e: &TirExpr, env: &mut Env) -> Eval<Value> {
        self.last = e.span;
        self.tick()?;
        match &e.kind {
            TirKind::Int(n) => Ok(Value::Int(*n)),
            TirKind::Text(t) => Ok(Value::Text(t.as_str().into())),
            TirKind::Bool(b) => Ok(Value::Bool(*b)),
            TirKind::Nothing => Ok(Value::Nothing),
            TirKind::Local(id) => env
                .get(id)
                .cloned()
                .ok_or_else(|| internal(&format!("unbound local {id}"))),
            TirKind::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.eval(item, env))
                    .collect::<Eval<Vec<_>>>()?;
                self.list(e.span, items)
            }
            TirKind::Record(id, fields) => {
                let fields = fields
                    .iter()
                    .map(|f| self.eval(f, env))
                    .collect::<Eval<Vec<_>>>()?;
                self.alloc(e.span, 8 * fields.len())?;
                Ok(Value::Record(*id, fields.into()))
            }
            TirKind::Variant(id, payload) => {
                let payload = payload.as_ref().map(|p| self.eval(p, env)).transpose()?;
                self.variant(e.span, *id, payload)
            }
            TirKind::Call(id, args) => {
                let args = args
                    .iter()
                    .map(|a| self.eval(a, env))
                    .collect::<Eval<Vec<_>>>()?;
                self.call(e.span, *id, args)
            }
            TirKind::Field(record, index) => match self.eval(record, env)? {
                Value::Record(_, fields) => fields
                    .get(*index as usize)
                    .cloned()
                    .ok_or_else(|| internal("field index out of range")),
                v => Err(internal(&format!("field of a non-record {v:?}"))),
            },
            TirKind::Index {
                list,
                index,
                checked,
            } => {
                let items = self.items(list, env)?;
                let i = self.int(index, env)?;
                let item = usize::try_from(i).ok().and_then(|i| items.get(i)).cloned();
                match (item, checked) {
                    (Some(item), false) => Ok(item),
                    (None, false) => Err(internal("a static index was out of range")),
                    (Some(item), true) => self.variant(e.span, prelude::OK, Some(item)),
                    (None, true) => {
                        let error = Value::Variant(prelude::OUT_OF_RANGE, None);
                        self.variant(e.span, prelude::ERR, Some(error))
                    }
                }
            }
            TirKind::Neg(a) => {
                let a = self.int(a, env)?;
                a.checked_neg()
                    .map(Value::Int)
                    .ok_or_else(|| self.fault("L040", e.span))
            }
            TirKind::Add(a, b) => self.arith(e, env, a, b, i64::checked_add, false),
            TirKind::Sub(a, b) => self.arith(e, env, a, b, i64::checked_sub, false),
            TirKind::Mul(a, b) => self.arith(e, env, a, b, i64::checked_mul, false),
            TirKind::Div(a, b) => self.arith(e, env, a, b, i64::checked_div, true),
            TirKind::Rem(a, b) => self.arith(e, env, a, b, i64::checked_rem, true),
            TirKind::Cmp(op, a, b) => {
                let (a, b) = (self.eval(a, env)?, self.eval(b, env)?);
                let ordered = |f: fn(&i64, &i64) -> bool| match (&a, &b) {
                    (Value::Int(a), Value::Int(b)) => Ok(f(a, b)),
                    _ => Err(internal("an ordering of non-integers")),
                };
                Ok(Value::Bool(match op {
                    CmpOp::Eq => a == b,
                    CmpOp::Ne => a != b,
                    CmpOp::Lt => ordered(i64::lt)?,
                    CmpOp::Le => ordered(i64::le)?,
                    CmpOp::Gt => ordered(i64::gt)?,
                    CmpOp::Ge => ordered(i64::ge)?,
                }))
            }
            TirKind::Match(scrutinee, arms) => self.matching(scrutinee, arms, env),
            TirKind::Concat(parts) => {
                let mut out = String::new();
                for part in parts {
                    match self.eval(part, env)? {
                        Value::Text(t) => out.push_str(&t),
                        Value::Int(n) => out.push_str(&n.to_string()),
                        v => return Err(internal(&format!("a text hole holds {v:?}"))),
                    }
                }
                self.text(e.span, out)
            }
            TirKind::IntIn(source, lo, hi) => self.int_in(e.span, source, *lo, *hi, env),
            TirKind::IntAtLeast(source, lo) => self.int_in(e.span, source, *lo, i64::MAX, env),
            TirKind::ClockNow => self.pop(e.span, "clock"),
            TirKind::ResourceCall(op, args) => {
                for a in args {
                    self.eval(a, env)?;
                }
                let resource = match self.symbols.get(*op as usize).map(|s| &s.kind) {
                    Some(SymbolKind::ResourceOp { resource }) => *resource,
                    _ => return Err(internal("a resource call of a non-operation")),
                };
                let name = self.symbols[resource as usize].name.clone();
                self.pop(e.span, &name)
            }
            TirKind::Log(message) => {
                let message = self.texts(message, env)?;
                self.logs.push(message.to_string());
                Ok(Value::Nothing)
            }
            TirKind::Let(binds, body) => {
                for (id, value) in binds {
                    let value = self.eval(value, env)?;
                    env.insert(*id, value);
                }
                self.eval(body, env)
            }
            TirKind::FsRead(_) | TirKind::CatalogColumns(_) => {
                Err(internal("a comptime-only operation reached an example"))
            }
        }
    }

    // The next script element, or `L041`
    fn pop(&mut self, span: Span, resource: &str) -> Eval<Value> {
        self.script.pop_front().ok_or_else(|| Stop::Script {
            resource: resource.to_owned(),
            at: self.at(span),
        })
    }

    fn int_in(
        &mut self,
        span: Span,
        source: &TirExpr,
        lo: i64,
        hi: i64,
        env: &mut Env,
    ) -> Eval<Value> {
        let parsed = match self.eval(source, env)? {
            Value::Int(n) => Some(i128::from(n)),
            Value::Text(t) => parse_int(&t),
            v => return Err(internal(&format!("int_in of {v:?}"))),
        };
        let value = parsed.map(|n| i64::try_from(n).ok().filter(|n| (lo..=hi).contains(n)));
        match value {
            Some(Some(n)) => self.variant(span, prelude::OK, Some(Value::Int(n))),
            Some(None) => {
                let error = Value::Variant(prelude::OUT_OF_RANGE, None);
                self.variant(span, prelude::ERR, Some(error))
            }
            None => {
                let error = Value::Variant(prelude::NOT_AN_INTEGER, None);
                self.variant(span, prelude::ERR, Some(error))
            }
        }
    }

    fn call(&mut self, span: Span, id: SymbolId, args: Vec<Value>) -> Eval<Value> {
        let text = |v: &Value| match v {
            Value::Text(t) => Ok(t.clone()),
            v => Err(internal(&format!("expected text, found {v:?}"))),
        };
        let int = |v: &Value| match v {
            Value::Int(n) => Ok(*n),
            v => Err(internal(&format!("expected an integer, found {v:?}"))),
        };
        let list = |v: &Value| match v {
            Value::List(items) => Ok(items.clone()),
            v => Err(internal(&format!("expected a list, found {v:?}"))),
        };
        let arg = |i: usize| {
            args.get(i)
                .ok_or_else(|| internal("a prelude call is missing an argument"))
        };
        match id {
            prelude::TRIM => {
                let t = text(arg(0)?)?;
                self.text(span, t.trim_matches([' ', '\t', '\n', '\r']).to_owned())
            }
            prelude::LOWER => {
                let t = text(arg(0)?)?;
                self.text(span, t.to_ascii_lowercase())
            }
            prelude::SPLIT => {
                let (t, separator) = (text(arg(0)?)?, text(arg(1)?)?);
                if separator.is_empty() {
                    return Err(self.fault("L046", span));
                }
                let parts = t
                    .split(&*separator)
                    .map(|part| self.text(span, part.to_owned()))
                    .collect::<Eval<Vec<_>>>()?;
                self.list(span, parts)
            }
            prelude::JOIN => {
                let (left, right) = (list(arg(0)?)?, list(arg(1)?)?);
                self.list(span, left.iter().chain(right.iter()).cloned().collect())
            }
            prelude::MAX => Ok(Value::Int(int(arg(0)?)?.max(int(arg(1)?)?))),
            prelude::LENGTH => Ok(Value::Int(
                i64::try_from(list(arg(0)?)?.len()).unwrap_or(i64::MAX),
            )),
            _ => self.call_fn(span, id, args),
        }
    }

    fn call_fn(&mut self, span: Span, id: SymbolId, args: Vec<Value>) -> Eval<Value> {
        let &(m, f) = self
            .functions
            .get(&id)
            .ok_or_else(|| internal(&format!("call of unknown function {id}")))?;
        if self.depth >= DEPTH {
            return Err(Stop::Depth(self.at(span)));
        }
        let TirBody::Stmts(body) = &f.body else {
            return self.primitive(f, &args);
        };
        let mut env: Env = f.params.iter().copied().zip(args).collect();
        let caller = self.module;
        self.module = m;
        self.depth += 1;
        let result = self.block(body, &mut env);
        self.depth -= 1;
        self.module = caller;
        match result {
            Err(Stop::Return(v)) => Ok(v),
            Ok(()) => Err(internal("a function fell off its end")),
            Err(stop) => Err(stop),
        }
    }

    // `money.percent`, the one allowlisted primitive (lingua1.md, Examples): an exact
    // product, floored, plus one when rounding `half_up` leaves a remainder of 50 or more
    fn primitive(&self, f: &TirFn, args: &[Value]) -> Eval<Value> {
        let symbol = &self.symbols[f.id as usize];
        let (Some(Value::Int(part)), Some(Value::Int(of)), Some(Value::Variant(round, _))) =
            (args.first(), args.get(1), args.get(2))
        else {
            return Err(internal("`percent` called with the wrong arguments"));
        };
        if (symbol.module.as_str(), symbol.name.as_str()) != ("money", "percent") {
            return Err(internal("a primitive other than `money.percent`"));
        }
        let product = i128::from(*part) * i128::from(*of);
        let (q, r) = (product.div_euclid(100), product.rem_euclid(100));
        let half_up = self.symbols[*round as usize].name == "half_up";
        let result = if half_up && r >= 50 { q + 1 } else { q };
        i64::try_from(result)
            .map(Value::Int)
            .map_err(|_| internal("`percent` out of range"))
    }

    fn block(&mut self, stmts: &[TirStmt], env: &mut Env) -> Eval<()> {
        for stmt in stmts {
            self.tick()?;
            self.stmt(stmt, env)?;
        }
        Ok(())
    }

    fn stmt(&mut self, stmt: &TirStmt, env: &mut Env) -> Eval<()> {
        match stmt {
            TirStmt::Let(id, e) | TirStmt::Set(id, e) => {
                let v = self.eval(e, env)?;
                env.insert(*id, v);
            }
            TirStmt::Return(e) => return Err(Stop::Return(self.eval(e, env)?)),
            TirStmt::Assert {
                cond,
                message,
                span,
            } => {
                if self.eval(cond, env)? != Value::Bool(true) {
                    return Err(Stop::Exception {
                        message: message.clone(),
                        at: self.at(*span),
                    });
                }
            }
            TirStmt::Fail { message, span } => {
                return Err(Stop::Exception {
                    message: message.clone(),
                    at: self.at(*span),
                });
            }
            TirStmt::If(cond, then, otherwise) => {
                let branch = if self.eval(cond, env)? == Value::Bool(true) {
                    then
                } else {
                    otherwise
                };
                self.block(branch, env)?;
            }
            TirStmt::ForList(id, list, body) => {
                for item in self.items(list, env)?.iter() {
                    env.insert(*id, item.clone());
                    self.block(body, env)?;
                    self.tick()?;
                }
            }
            TirStmt::ForRange(id, low, high, body) => {
                let (low, high) = (self.int(low, env)?, self.int(high, env)?);
                for i in low..high {
                    env.insert(*id, Value::Int(i));
                    self.block(body, env)?;
                    self.tick()?;
                }
            }
            TirStmt::WhileIs {
                scrutinee,
                variant,
                payload,
                body,
            } => loop {
                match env.get(scrutinee) {
                    Some(Value::Variant(v, p)) if v == variant => {
                        if let (Some(id), Some(p)) = (payload, p) {
                            let p = Value::clone(p);
                            env.insert(*id, p);
                        }
                    }
                    Some(_) => break,
                    None => return Err(internal("unbound `while` scrutinee")),
                }
                self.block(body, env)?;
                self.tick()?;
            },
            TirStmt::Match(scrutinee, arms) => {
                self.matching(scrutinee, arms, env)?;
            }
            TirStmt::Expr(e) => {
                self.eval(e, env)?;
            }
        }
        Ok(())
    }

    // The first arm whose pattern matches; a statement arm's value is `nothing`
    fn matching(&mut self, scrutinee: &TirExpr, arms: &[TirArm], env: &mut Env) -> Eval<Value> {
        let value = self.eval(scrutinee, env)?;
        for arm in arms {
            if bind(&arm.pat, &value, env) {
                return match &arm.body {
                    TirArmBody::Expr(e) => self.eval(e, env),
                    TirArmBody::Stmts(stmts) => {
                        self.block(stmts, env)?;
                        Ok(Value::Nothing)
                    }
                };
            }
        }
        Err(internal("no match arm matched"))
    }

    fn render(&self, v: &Value) -> String {
        let name = |id: SymbolId| self.symbols[id as usize].name.as_str();
        let all = |items: &[Value]| {
            items
                .iter()
                .map(|v| self.render(v))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match v {
            Value::Int(n) => n.to_string(),
            Value::Text(t) => quote(t),
            Value::Bool(b) => b.to_string(),
            Value::Nothing => "nothing".to_owned(),
            Value::List(items) => format!("[{}]", all(items)),
            Value::Record(id, fields) => format!("{}({})", name(*id), all(fields)),
            Value::Variant(id, None) => name(*id).to_owned(),
            Value::Variant(id, Some(p)) => format!("{}({})", name(*id), self.render(p)),
        }
    }
}

// Tests `value` against `pat`, binding as it goes
fn bind(pat: &TirPat, value: &Value, env: &mut Env) -> bool {
    match (pat, value) {
        (TirPat::Wild, _) => true,
        (TirPat::Bind(id), v) => {
            env.insert(*id, v.clone());
            true
        }
        (TirPat::Variant(id, inner), Value::Variant(v, payload)) if id == v => {
            match (inner, payload) {
                (None, _) => true,
                (Some(inner), Some(payload)) => bind(inner, payload, env),
                (Some(_), None) => false,
            }
        }
        (TirPat::LitInt(n), Value::Int(v)) => n == v,
        (TirPat::LitText(t), Value::Text(v)) => t.as_str() == &**v,
        (TirPat::List(pats), Value::List(items)) => {
            pats.len() == items.len() && pats.iter().zip(items.iter()).all(|(p, v)| bind(p, v, env))
        }
        _ => false,
    }
}

// lingua1.md `int_in` on text: an optional `-` and at least one digit, nothing else. The
// value saturates past `Int` so it reads as out of range.
fn parse_int(text: &str) -> Option<i128> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let cap = i128::from(i64::MAX) + 2;
    let magnitude = digits
        .bytes()
        .fold(0_i128, |n, b| (n * 10 + i128::from(b - b'0')).min(cap));
    Some(if negative { -magnitude } else { magnitude })
}

// A text value as a Lingua literal
fn quote(text: &str) -> String {
    let mut out = String::from('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '{' => out.push_str("{{"),
            '}' => out.push_str("}}"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// The example's `->` line, from the arrow to the end of the expected value
fn arrow_span(source: &str, ex: &TirExample) -> Span {
    let lines: Vec<&str> = source.lines().collect();
    let arrow = (ex.span.start.line..=ex.span.end.line)
        .rev()
        .find_map(|line| {
            let text = lines.get(line.checked_sub(1)?)?;
            let indent = text.chars().take_while(|c| c.is_whitespace()).count();
            text.trim_start()
                .starts_with("->")
                .then(|| (line, indent, text.trim_end().chars().count()))
        });
    let Some((line, indent, width)) = arrow else {
        return ex.span;
    };
    let end = match &ex.expect {
        TirExpect::Value(e) if e.span.end.line == line => e.span.end.column,
        _ => width + 1,
    };
    let at = |column| crate::lex::Location { line, column };
    Span {
        start: at(indent + 1),
        end: at(end),
    }
}

impl Machine<'_> {
    fn diagnostic(&self, code: &'static str, at: At, summary: String) -> Diagnostic {
        let module = &self.modules[at.module];
        Diagnostic::new(code, summary, &module.path).at(&module.source, at.span)
    }

    // Runs one function example in module `m`; `None` when it passes
    fn example(&mut self, m: usize, f: &TirFn, ex: &TirExample) -> Option<Diagnostic> {
        self.reset(m, FUEL);
        let mut env = Env::new();
        let result = self.run(f, ex, &mut env);
        let label = &ex.label;
        let arrow = At {
            module: m,
            span: arrow_span(&self.modules[m].source, ex),
        };
        let summary = match (result, &ex.expect) {
            (Ok(got), TirExpect::Value(e)) => {
                self.module = m;
                match self.eval(e, &mut env) {
                    Ok(want) if want == got => return None,
                    Ok(want) => (
                        "L043",
                        format!(
                            "example \"{label}\" expected {} got {}",
                            self.render(&want),
                            self.render(&got)
                        ),
                    ),
                    Err(stop) => return Some(self.stopped(label, arrow, stop)),
                }
            }
            (Ok(got), TirExpect::Exception { message }) => (
                "L059",
                format!(
                    "example \"{label}\" expected exception \"{message}\" got {}",
                    self.render(&got)
                ),
            ),
            (Err(Stop::Exception { message: got, .. }), TirExpect::Exception { message })
                if got == *message =>
            {
                return None;
            }
            (Err(Stop::Exception { message: got, .. }), TirExpect::Exception { message }) => (
                "L059",
                format!(
                    "example \"{label}\" expected exception \"{message}\" got exception \"{got}\""
                ),
            ),
            (Err(Stop::Exception { message, at }), TirExpect::Value(_)) => {
                let module = &self.modules[at.module];
                let summary = format!(
                    "example \"{label}\" expected a value but raised exception \"{message}\" at {}:{}:{}",
                    module.path, at.span.start.line, at.span.start.column
                );
                ("L057", summary)
            }
            (Err(stop), _) => return Some(self.stopped(label, arrow, stop)),
        };
        Some(self.diagnostic(summary.0, arrow, summary.1))
    }

    // The script, the inputs in source order, then the call
    fn run(&mut self, f: &TirFn, ex: &TirExample, env: &mut Env) -> Eval<Value> {
        if let Some(script) = &ex.script {
            let items = self.items(script, env)?;
            self.script = items.iter().cloned().collect();
        }
        let mut inputs = Env::new();
        for (param, e) in &ex.inputs {
            let value = self.eval(e, env)?;
            inputs.insert(*param, value);
        }
        let args = f
            .params
            .iter()
            .map(|p| inputs.remove(p))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| internal("an example input is missing"))?;
        self.call_fn(f.span, f.id, args)
    }

    // A fault, an exhausted script, a limit, or a bug, as the example's diagnostic
    fn stopped(&self, label: &str, arrow: At, stop: Stop) -> Diagnostic {
        match stop {
            Stop::Fault { code, at } => {
                let module = &self.modules[at.module];
                Diagnostic::fault(code, &module.path, &module.source, at.span)
            }
            Stop::Script { resource, at } => self.diagnostic(
                "L041",
                at,
                format!("example \"{label}\" ran out of script for `{resource}`"),
            ),
            Stop::Heap(at) => self.diagnostic(
                "L047",
                at,
                format!("example \"{label}\" exceeded the 64 MiB heap limit"),
            ),
            Stop::Depth(at) => self.diagnostic(
                "L047",
                at,
                format!("example \"{label}\" exceeded the call depth limit of {DEPTH}"),
            ),
            Stop::Exception { .. } | Stop::Return(_) => self.diagnostic(
                crate::check::NOT_YET,
                arrow,
                format!("internal error: example \"{label}\" stopped unexpectedly; this is a bug in lingua"),
            ),
            Stop::Internal(what) => self.diagnostic(
                crate::check::NOT_YET,
                arrow,
                format!("internal error in example \"{label}\": {what}; this is a bug in lingua"),
            ),
        }
    }
}

/// Runs every function example in the modules `runnable` accepts, in link order and then
/// source order, and returns a diagnostic for each one that fails.
pub(crate) fn run_examples(
    symbols: &[Symbol],
    modules: &[TirModule],
    runnable: &(dyn Fn(&TirModule) -> bool + Sync),
) -> Vec<Diagnostic> {
    let run = || {
        let mut machine = Machine::new(symbols, modules);
        let mut out = Vec::new();
        for (m, module) in modules.iter().enumerate().filter(|(_, m)| runnable(m)) {
            for f in &module.functions {
                for ex in &f.examples {
                    if ex.kind == ExampleKind::Function {
                        out.extend(machine.example(m, f, ex));
                    }
                }
            }
        }
        out
    };
    // A deep recursion needs more stack than the main thread has
    std::thread::scope(|scope| {
        match std::thread::Builder::new()
            .stack_size(STACK)
            .spawn_scoped(scope, run)
        {
            Ok(handle) => handle
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic)),
            Err(_) => run(),
        }
    })
}

/// Runs `main` with no arguments on the interpreter with `fuel` steps, and returns what the
/// native binary would write to stderr; `None` when `main` takes a `Process`.
#[cfg(test)]
pub(crate) fn run_main(program: &crate::tir::TirProgram, fuel: u64) -> Option<String> {
    let run = || {
        let mut machine = Machine::new(&program.symbols, &program.modules);
        let &(m, f) = machine.functions.get(&program.main)?;
        if !f.params.is_empty() {
            return None;
        }
        machine.reset(m, fuel);
        let result = machine.call_fn(f.span, f.id, Vec::new());
        let module = |at: At| &program.modules[at.module];
        let tail = match result {
            Ok(_) => String::new(),
            Err(Stop::Exception { message, at }) => {
                crate::report::exception(&message, &module(at).path, &module(at).source, at.span)
            }
            Err(Stop::Fault { code, at }) => {
                crate::report::fault(code, &module(at).path, &module(at).source, Some(at.span))
            }
            Err(stop) => format!("{stop:?}\n"),
        };
        let logs: String = machine.logs.iter().map(|l| l.clone() + "\n").collect();
        Some(logs + &tail)
    };
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(STACK)
            .spawn_scoped(scope, run)
            .unwrap()
            .join()
            .unwrap()
    })
}
