//! The runtime, written as LIR so every target compiles it through the same selection
//! (docs/build.md, Runtime). Every routine is nosplit (no stack check): none recurses, and
//! together they fit the 64 KiB stack reserve.
//!
//! The per-target shims they call: `rt.sys_write(fd, ptr, len) -> n`, `rt.sys_exit(code)`,
//! `rt.sys_mmap(size) -> ptr or 0`, `rt.sys_clock(buf)`, `rt.sys_getpid() -> n`,
//! `rt.sys_gettid() -> n`, and `rt.sys_getaffinity(buf) -> bytes or negative`. Targets also
//! define the Text blocks `rt.os` and `rt.arch`.

use super::lir::{BinOp, CmpOp, FnBuilder, LirFn, RoData, VReg, singleton_block, text_block};
use super::prog::SymName;
use crate::report;

/// The `fault L047: out of memory` report, which has no span.
pub const OOM: &str = "site.oom";
pub const OUT_OF_RANGE: &str = "var.prelude.BoundsError.OutOfRange";
pub const NOT_AN_INTEGER: &str = "var.prelude.BoundsError.NotAnInteger";

/// Writable words in `.bss`: the heap's bump pointer and end, the stack bound rt0 sets, and
/// the process's initial stack pointer (`argc`, then `argv`).
pub const BSS: [&str; 4] = ["rt.heap_ptr", "rt.heap_end", "rt.stack_limit", "rt.argp"];

const CHUNK: i64 = 4 << 20;
const CHUNK_ALIGN: i64 = 64 << 10;
const EINTR: i64 = -4;
const STDERR: i64 = 2;

/// The runtime's functions, in layout order.
#[must_use]
pub fn functions() -> Vec<LirFn> {
    vec![
        alloc(),
        die(),
        write_all(),
        log(),
        clock_now(),
        bytes_eq(),
        text_eq(),
        copy(),
        text_new(),
        substr(),
        text_concat(),
        int_to_text(),
        trim(),
        lower(),
        split(),
        join(),
        result("rt.ok", 0),
        result("rt.err", 1),
        list_index(),
        int_in_int(),
        int_in_text(),
        percent(),
        cstr_to_text(),
        ncpu(),
        process(),
    ]
}

/// The runtime's read-only data.
#[must_use]
pub fn rodata() -> Vec<RoData> {
    vec![
        text_block("rt.newline", "\n"),
        text_block("rt.main_name", "main"),
        text_block(OOM, &report::fault("L047", "", "", None)),
        singleton_block(OUT_OF_RANGE, 0),
        singleton_block(NOT_AN_INTEGER, 1),
    ]
}

/// The `.bss` symbols, 8 bytes each.
#[must_use]
pub fn bss() -> Vec<SymName> {
    BSS.iter().map(|&name| name.to_owned()).collect()
}

fn add(b: &mut FnBuilder, x: VReg, y: VReg) -> VReg {
    b.bin(BinOp::Add, x, y)
}

fn sub(b: &mut FnBuilder, x: VReg, y: VReg) -> VReg {
    b.bin(BinOp::Sub, x, y)
}

fn add_n(b: &mut FnBuilder, x: VReg, n: i64) -> VReg {
    let k = b.konst(n);
    add(b, x, k)
}

// A mutable copy of `init`
fn var(b: &mut FnBuilder, init: VReg) -> VReg {
    let v = b.fresh();
    b.mov(v, init);
    v
}

fn set_const(b: &mut FnBuilder, dst: VReg, n: i64) {
    let k = b.konst(n);
    b.mov(dst, k);
}

// `&text.bytes[i]`
fn byte_at(b: &mut FnBuilder, text: VReg, i: VReg) -> VReg {
    let base = add(b, text, i);
    b.load8(base, 8)
}

// `&list.elems[i]`
fn elem_addr(b: &mut FnBuilder, list: VReg, i: VReg) -> VReg {
    let three = b.konst(3);
    let off = b.bin(BinOp::Shl, i, three);
    let at = add(b, list, off);
    add_n(b, at, 8)
}

// for i in 0 until n
fn for_each(b: &mut FnBuilder, n: VReg, body: impl FnOnce(&mut FnBuilder, VReg)) {
    let zero = b.konst(0);
    let i = var(b, zero);
    b.while_(
        |b| b.cmp(CmpOp::Lt, i, n),
        |b| {
            body(b, i);
            b.add_imm(i, 1);
        },
    );
}

// rt.alloc(bytes) -> ptr: bump allocation over mmap'd chunks, zeroed, never freed
fn alloc() -> LirFn {
    let mut b = FnBuilder::new("rt.alloc", 1, None);
    let seven = b.konst(7);
    let mask = b.konst(-8);
    let rounded = add(&mut b, 0, seven);
    let n = b.bin(BinOp::And, rounded, mask);
    let ptr_sym = b.addr("rt.heap_ptr");
    let end_sym = b.addr("rt.heap_end");
    let loaded = b.load(ptr_sym, 0);
    let p = var(&mut b, loaded);
    let end = b.load(end_sym, 0);
    let next = add(&mut b, p, n);
    let full = b.cmp(CmpOp::ULt, end, next);
    b.if_(full, |b| {
        let round = b.konst(CHUNK_ALIGN - 1);
        let align = b.konst(-CHUNK_ALIGN);
        let up = add(b, n, round);
        let wanted = b.bin(BinOp::And, up, align);
        let size = b.konst(CHUNK);
        let small = b.cmp(CmpOp::ULt, wanted, size);
        b.if_(small, |b| b.mov(wanted, size));
        let base = b.call("rt.sys_mmap", &[wanted]);
        let failed = b.cmp_imm(CmpOp::Eq, base, 0);
        b.if_(failed, |b| {
            let report = b.addr(OOM);
            b.call_void("rt.die", &[report]);
        });
        let chunk_end = add(b, base, wanted);
        b.store(end_sym, 0, chunk_end);
        b.mov(p, base);
    });
    let bumped = add(&mut b, p, n);
    b.store(ptr_sym, 0, bumped);
    b.ret(Some(p));
    b.finish()
}

// rt.die(report): write the pre-rendered report to stderr and exit 1
fn die() -> LirFn {
    let mut b = FnBuilder::new("rt.die", 1, None);
    let fd = b.konst(STDERR);
    let len = b.load(0, 0);
    let bytes = add_n(&mut b, 0, 8);
    b.call_void("rt.write_all", &[fd, bytes, len]);
    let one = b.konst(1);
    b.call_void("rt.sys_exit", &[one]);
    b.ret(None);
    b.finish()
}

// rt.write_all(fd, ptr, len): write until done, retrying EINTR; give up on another error
fn write_all() -> LirFn {
    let mut b = FnBuilder::new("rt.write_all", 3, None);
    let p = var(&mut b, 1);
    let left = var(&mut b, 2);
    b.while_(
        |b| {
            let zero = b.konst(0);
            b.cmp(CmpOp::Lt, zero, left)
        },
        |b| {
            let n = b.call("rt.sys_write", &[0, p, left]);
            let zero = b.konst(0);
            let wrote = b.cmp(CmpOp::Lt, zero, n);
            b.if_else(
                wrote,
                |b| {
                    b.bin_into(p, BinOp::Add, p, n);
                    b.bin_into(left, BinOp::Sub, left, n);
                },
                |b| {
                    let other = b.cmp_imm(CmpOp::Ne, n, EINTR);
                    b.if_(other, |b| set_const(b, left, 0));
                },
            );
        },
    );
    let zero = b.konst(0);
    b.ret(Some(zero));
    b.finish()
}

// rt.log(text) -> 0: the text, then "\n", to stderr
fn log() -> LirFn {
    let mut b = FnBuilder::new("rt.log", 1, None);
    let fd = b.konst(STDERR);
    let len = b.load(0, 0);
    let bytes = add_n(&mut b, 0, 8);
    b.call_void("rt.write_all", &[fd, bytes, len]);
    let newline = b.addr("rt.newline");
    let newline = add_n(&mut b, newline, 8);
    let one = b.konst(1);
    b.call_void("rt.write_all", &[fd, newline, one]);
    let zero = b.konst(0);
    b.ret(Some(zero));
    b.finish()
}

// rt.clock_now() -> seconds: clock_gettime(CLOCK_REALTIME).tv_sec
fn clock_now() -> LirFn {
    let mut b = FnBuilder::new("rt.clock_now", 0, None);
    let size = b.konst(16);
    let buf = b.call("rt.alloc", &[size]);
    b.call_void("rt.sys_clock", &[buf]);
    let seconds = b.load(buf, 0);
    b.ret(Some(seconds));
    b.finish()
}

// rt.bytes_eq(p, q, n) -> 0/1
fn bytes_eq() -> LirFn {
    let mut b = FnBuilder::new("rt.bytes_eq", 3, None);
    for_each(&mut b, 2, |b, i| {
        let x = add(b, 0, i);
        let y = add(b, 1, i);
        let x = b.load8(x, 0);
        let y = b.load8(y, 0);
        let differ = b.cmp(CmpOp::Ne, x, y);
        b.if_(differ, |b| {
            let zero = b.konst(0);
            b.ret(Some(zero));
        });
    });
    let one = b.konst(1);
    b.ret(Some(one));
    b.finish()
}

// rt.text_eq(a, b) -> 0/1: equal lengths and equal bytes
fn text_eq() -> LirFn {
    let mut b = FnBuilder::new("rt.text_eq", 2, None);
    let la = b.load(0, 0);
    let lb = b.load(1, 0);
    let differ = b.cmp(CmpOp::Ne, la, lb);
    b.if_(differ, |b| {
        let zero = b.konst(0);
        b.ret(Some(zero));
    });
    let pa = add_n(&mut b, 0, 8);
    let pb = add_n(&mut b, 1, 8);
    let equal = b.call("rt.bytes_eq", &[pa, pb, la]);
    b.ret(Some(equal));
    b.finish()
}

// rt.copy(dst, src, n) -> 0: n bytes
fn copy() -> LirFn {
    let mut b = FnBuilder::new("rt.copy", 3, None);
    for_each(&mut b, 2, |b, i| {
        let from = add(b, 1, i);
        let byte = b.load8(from, 0);
        let to = add(b, 0, i);
        b.store8(to, 0, byte);
    });
    let zero = b.konst(0);
    b.ret(Some(zero));
    b.finish()
}

// rt.text_new(len) -> text with `len` set and zeroed bytes
fn text_new() -> LirFn {
    let mut b = FnBuilder::new("rt.text_new", 1, None);
    let size = add_n(&mut b, 0, 8);
    let text = b.call("rt.alloc", &[size]);
    b.store(text, 0, 0);
    b.ret(Some(text));
    b.finish()
}

// rt.substr(text, start, n) -> text
fn substr() -> LirFn {
    let mut b = FnBuilder::new("rt.substr", 3, None);
    let text = b.call("rt.text_new", &[2]);
    let to = add_n(&mut b, text, 8);
    let from = add(&mut b, 0, 1);
    let from = add_n(&mut b, from, 8);
    b.call_void("rt.copy", &[to, from, 2]);
    b.ret(Some(text));
    b.finish()
}

// rt.text_concat(list) -> text: one allocation of the summed length
fn text_concat() -> LirFn {
    let mut b = FnBuilder::new("rt.text_concat", 1, None);
    let n = b.load(0, 0);
    let zero = b.konst(0);
    let total = var(&mut b, zero);
    for_each(&mut b, n, |b, i| {
        let at = elem_addr(b, 0, i);
        let text = b.load(at, 0);
        let len = b.load(text, 0);
        b.bin_into(total, BinOp::Add, total, len);
    });
    let out = b.call("rt.text_new", &[total]);
    let start = add_n(&mut b, out, 8);
    let cursor = var(&mut b, start);
    for_each(&mut b, n, |b, i| {
        let at = elem_addr(b, 0, i);
        let text = b.load(at, 0);
        let len = b.load(text, 0);
        let bytes = add_n(b, text, 8);
        b.call_void("rt.copy", &[cursor, bytes, len]);
        b.bin_into(cursor, BinOp::Add, cursor, len);
    });
    b.ret(Some(out));
    b.finish()
}

// rt.int_to_text(n) -> text: decimal, `-` only if negative. The magnitude is unsigned, so
// `0 - INT_MIN` (2^63) is exact.
fn int_to_text() -> LirFn {
    let mut b = FnBuilder::new("rt.int_to_text", 1, None);
    let zero = b.konst(0);
    let ten = b.konst(10);
    let negative = b.cmp(CmpOp::Lt, 0, zero);
    let magnitude = var(&mut b, 0);
    b.if_(negative, |b| b.bin_into(magnitude, BinOp::Sub, zero, 0));
    let one = b.konst(1);
    let digits = var(&mut b, one);
    let rest = var(&mut b, magnitude);
    b.while_(
        |b| {
            let small = b.cmp(CmpOp::ULt, rest, ten);
            b.cmp_imm(CmpOp::Eq, small, 0)
        },
        |b| {
            b.bin_into(rest, BinOp::UDiv, rest, ten);
            b.add_imm(digits, 1);
        },
    );
    let len = add(&mut b, digits, negative);
    let text = b.call("rt.text_new", &[len]);
    b.if_(negative, |b| {
        let minus = b.konst(i64::from(b'-'));
        b.store8(text, 8, minus);
    });
    // the last digit's address, walking left
    let end = add(&mut b, text, len);
    let at = var(&mut b, end);
    b.while_(
        |b| b.cmp(CmpOp::Lt, zero, digits),
        |b| {
            let digit = b.bin(BinOp::URem, magnitude, ten);
            let ascii = add_n(b, digit, i64::from(b'0'));
            b.store8(at, 7, ascii);
            b.bin_into(magnitude, BinOp::UDiv, magnitude, ten);
            b.add_imm(at, -1);
            b.add_imm(digits, -1);
        },
    );
    b.ret(Some(text));
    b.finish()
}

// 1 when `byte` is U+0020, U+0009, U+000A, or U+000D
fn is_space(b: &mut FnBuilder, byte: VReg) -> VReg {
    let checks: Vec<VReg> = b" \t\n\r"
        .iter()
        .copied()
        .map(|c| b.cmp_imm(CmpOp::Eq, byte, i64::from(c)))
        .collect();
    checks[1..]
        .iter()
        .fold(checks[0], |acc, &c| b.bin(BinOp::Or, acc, c))
}

// rt.trim(text) -> text
fn trim() -> LirFn {
    let mut b = FnBuilder::new("rt.trim", 1, None);
    let len = b.load(0, 0);
    let zero = b.konst(0);
    let start = var(&mut b, zero);
    b.while_(
        |b| {
            let go = b.cmp(CmpOp::Lt, start, len);
            b.if_(go, |b| {
                let byte = byte_at(b, 0, start);
                let space = is_space(b, byte);
                b.mov(go, space);
            });
            go
        },
        |b| b.add_imm(start, 1),
    );
    let end = var(&mut b, len);
    b.while_(
        |b| {
            let go = b.cmp(CmpOp::Lt, start, end);
            b.if_(go, |b| {
                let last = add_n(b, end, -1);
                let byte = byte_at(b, 0, last);
                let space = is_space(b, byte);
                b.mov(go, space);
            });
            go
        },
        |b| b.add_imm(end, -1),
    );
    let n = sub(&mut b, end, start);
    let out = b.call("rt.substr", &[0, start, n]);
    b.ret(Some(out));
    b.finish()
}

// rt.lower(text) -> text: bytes A-Z to a-z
fn lower() -> LirFn {
    let mut b = FnBuilder::new("rt.lower", 1, None);
    let len = b.load(0, 0);
    let out = b.call("rt.text_new", &[len]);
    for_each(&mut b, len, |b, i| {
        let byte = byte_at(b, 0, i);
        let c = var(b, byte);
        let a = b.konst(i64::from(b'A'));
        let offset = sub(b, c, a);
        let upper = b.cmp_imm(CmpOp::ULt, offset, 26);
        b.if_(upper, |b| b.add_imm(c, 32));
        let at = add(b, out, i);
        b.store8(at, 8, c);
    });
    b.ret(Some(out));
    b.finish()
}

// The separator matches `text` at byte `i`
fn sep_at(b: &mut FnBuilder, text: VReg, sep: VReg, sep_len: VReg, i: VReg) -> VReg {
    let at = add(b, text, i);
    let at = add_n(b, at, 8);
    let bytes = add_n(b, sep, 8);
    b.call("rt.bytes_eq", &[at, bytes, sep_len])
}

// Scans `text` for non-overlapping separators left to right, calling `found(i)` at each
fn each_sep(
    b: &mut FnBuilder,
    text_len: VReg,
    sep_len: VReg,
    mut found: impl FnMut(&mut FnBuilder, VReg),
) {
    let zero = b.konst(0);
    let i = var(b, zero);
    b.while_(
        |b| {
            let end = add(b, i, sep_len);
            b.cmp(CmpOp::Le, end, text_len)
        },
        |b| {
            let hit = sep_at(b, 0, 1, sep_len, i);
            b.if_else(
                hit,
                |b| {
                    found(b, i);
                    b.bin_into(i, BinOp::Add, i, sep_len);
                },
                |b| b.add_imm(i, 1),
            );
        },
    );
}

// rt.split(text, sep, report) -> List<Text>; an empty separator dies with `report` (L046)
fn split() -> LirFn {
    let mut b = FnBuilder::new("rt.split", 3, None);
    let sep_len = b.load(1, 0);
    let empty = b.cmp_imm(CmpOp::Eq, sep_len, 0);
    b.if_(empty, |b| b.call_void("rt.die", &[2]));
    let text_len = b.load(0, 0);
    let one = b.konst(1);
    let count = var(&mut b, one);
    each_sep(&mut b, text_len, sep_len, |b, _| b.add_imm(count, 1));
    let three = b.konst(3);
    let words = b.bin(BinOp::Shl, count, three);
    let size = add_n(&mut b, words, 8);
    let list = b.call("rt.alloc", &[size]);
    b.store(list, 0, count);
    let zero = b.konst(0);
    let k = var(&mut b, zero);
    let start = var(&mut b, zero);
    let piece = |b: &mut FnBuilder, end: VReg| {
        let n = sub(b, end, start);
        let text = b.call("rt.substr", &[0, start, n]);
        let at = elem_addr(b, list, k);
        b.store(at, 0, text);
        b.add_imm(k, 1);
    };
    each_sep(&mut b, text_len, sep_len, |b, i| {
        piece(b, i);
        let next = add(b, i, sep_len);
        b.mov(start, next);
    });
    piece(&mut b, text_len);
    b.ret(Some(list));
    b.finish()
}

// rt.join(left, right) -> list: both element runs copied
fn join() -> LirFn {
    let mut b = FnBuilder::new("rt.join", 2, None);
    let n = b.load(0, 0);
    let m = b.load(1, 0);
    let total = add(&mut b, n, m);
    let three = b.konst(3);
    let left_bytes = b.bin(BinOp::Shl, n, three);
    let right_bytes = b.bin(BinOp::Shl, m, three);
    let words = b.bin(BinOp::Shl, total, three);
    let size = add_n(&mut b, words, 8);
    let list = b.call("rt.alloc", &[size]);
    b.store(list, 0, total);
    let to = add_n(&mut b, list, 8);
    let from = add_n(&mut b, 0, 8);
    b.call_void("rt.copy", &[to, from, left_bytes]);
    let to = add(&mut b, to, left_bytes);
    let from = add_n(&mut b, 1, 8);
    b.call_void("rt.copy", &[to, from, right_bytes]);
    b.ret(Some(list));
    b.finish()
}

// rt.ok(v) / rt.err(v) -> Result
fn result(name: &str, tag: i64) -> LirFn {
    let mut b = FnBuilder::new(name, 1, None);
    let size = b.konst(16);
    let block = b.call("rt.alloc", &[size]);
    let tag = b.konst(tag);
    b.store(block, 0, tag);
    b.store(block, 8, 0);
    b.ret(Some(block));
    b.finish()
}

fn ret_err(b: &mut FnBuilder, variant: &str) {
    let payload = b.addr(variant);
    let err = b.call("rt.err", &[payload]);
    b.ret(Some(err));
}

// rt.list_index(list, i) -> Ok(elem) or Err(OutOfRange)
fn list_index() -> LirFn {
    let mut b = FnBuilder::new("rt.list_index", 2, None);
    let len = b.load(0, 0);
    let inside = b.cmp(CmpOp::ULt, 1, len);
    b.if_(inside, |b| {
        let at = elem_addr(b, 0, 1);
        let elem = b.load(at, 0);
        let ok = b.call("rt.ok", &[elem]);
        b.ret(Some(ok));
    });
    ret_err(&mut b, OUT_OF_RANGE);
    b.finish()
}

// rt.int_in_int(n, lo, hi) -> Ok(n) or Err(OutOfRange)
fn int_in_int() -> LirFn {
    let mut b = FnBuilder::new("rt.int_in_int", 3, None);
    let above = b.cmp(CmpOp::Le, 1, 0);
    let below = b.cmp(CmpOp::Le, 0, 2);
    let inside = b.bin(BinOp::And, above, below);
    b.if_(inside, |b| {
        let ok = b.call("rt.ok", &[0]);
        b.ret(Some(ok));
    });
    ret_err(&mut b, OUT_OF_RANGE);
    b.finish()
}

// rt.int_in_text(text, lo, hi) -> Result: an optional `-` and one or more digits, else
// Err(NotAnInteger); a value outside `Int` or `lo..=hi` is Err(OutOfRange). The magnitude is
// accumulated unsigned against the limit for the sign, so INT_MIN parses.
fn int_in_text() -> LirFn {
    let mut b = FnBuilder::new("rt.int_in_text", 3, None);
    let len = b.load(0, 0);
    let zero = b.konst(0);
    let i = var(&mut b, zero);
    let negative = var(&mut b, zero);
    let nonempty = b.cmp(CmpOp::Lt, zero, len);
    b.if_(nonempty, |b| {
        let first = b.load8(0, 8);
        let minus = b.cmp_imm(CmpOp::Eq, first, i64::from(b'-'));
        b.if_(minus, |b| {
            set_const(b, negative, 1);
            set_const(b, i, 1);
        });
    });
    let no_digits = b.cmp(CmpOp::Eq, i, len);
    b.if_(no_digits, |b| ret_err(b, NOT_AN_INTEGER));
    let int_max = b.konst(i64::MAX);
    let limit = var(&mut b, int_max);
    b.if_(negative, |b| set_const(b, limit, i64::MIN));
    let magnitude = var(&mut b, zero);
    let over = var(&mut b, zero);
    let ten = b.konst(10);
    b.while_(
        |b| b.cmp(CmpOp::Lt, i, len),
        |b| {
            let byte = byte_at(b, 0, i);
            let ascii_zero = b.konst(i64::from(b'0'));
            let digit = sub(b, byte, ascii_zero);
            let is_digit = b.cmp(CmpOp::ULt, digit, ten);
            b.if_else(
                is_digit,
                |b| {
                    let room = sub(b, limit, digit);
                    let room = b.bin(BinOp::UDiv, room, ten);
                    let too_big = b.cmp(CmpOp::ULt, room, magnitude);
                    b.if_else(
                        too_big,
                        |b| set_const(b, over, 1),
                        |b| {
                            b.bin_into(magnitude, BinOp::Mul, magnitude, ten);
                            b.bin_into(magnitude, BinOp::Add, magnitude, digit);
                        },
                    );
                },
                |b| ret_err(b, NOT_AN_INTEGER),
            );
            b.add_imm(i, 1);
        },
    );
    b.if_(over, |b| ret_err(b, OUT_OF_RANGE));
    b.if_(negative, |b| {
        b.bin_into(magnitude, BinOp::Sub, zero, magnitude);
    });
    let result = b.call("rt.int_in_int", &[magnitude, 1, 2]);
    b.ret(Some(result));
    b.finish()
}

// rt.percent(part, of, round) -> Cents: round-half-up `part * of / 100` without a wide
// intermediate. With `of = 100a + b`: q = part*a + (part*b)/100, r = (part*b) % 100, and
// q + 1 when r >= 50. `round` is ignored: `Rounding` has the one variant `half_up`.
fn percent() -> LirFn {
    let mut b = FnBuilder::new("rt.percent", 3, None);
    let hundred = b.konst(100);
    let a = b.bin(BinOp::UDiv, 1, hundred);
    let rem = b.bin(BinOp::URem, 1, hundred);
    let whole = b.bin(BinOp::Mul, 0, a);
    let partial = b.bin(BinOp::Mul, 0, rem);
    let carry = b.bin(BinOp::UDiv, partial, hundred);
    let q = add(&mut b, whole, carry);
    let r = b.bin(BinOp::URem, partial, hundred);
    let up = b.cmp_imm(CmpOp::Ge, r, 50);
    b.if_(up, |b| b.add_imm(q, 1));
    b.ret(Some(q));
    b.finish()
}

// Sets `dst` to `then_value` when `cond`, else leaves it
fn set_if(b: &mut FnBuilder, cond: VReg, dst: VReg, then_value: i64) {
    b.if_(cond, |b| set_const(b, dst, then_value));
}

// For a non-ASCII lead byte: the sequence width (0 when the byte cannot start one) and the
// allowed range of the second byte (Unicode Table 3-7)
fn utf8_lead(b: &mut FnBuilder, lead: VReg) -> (VReg, VReg, VReg) {
    let zero = b.konst(0);
    let width = var(b, zero);
    let lo = b.konst(0x80);
    let lo = var(b, lo);
    let hi = b.konst(0xbf);
    let hi = var(b, hi);
    for (op, byte, dst, value) in [
        (CmpOp::Ge, 0xc2, width, 2),
        (CmpOp::Ge, 0xe0, width, 3),
        (CmpOp::Eq, 0xe0, lo, 0xa0),
        (CmpOp::Eq, 0xed, hi, 0x9f),
        (CmpOp::Ge, 0xf0, width, 4),
        (CmpOp::Eq, 0xf0, lo, 0x90),
        (CmpOp::Eq, 0xf4, hi, 0x8f),
        (CmpOp::Ge, 0xf5, width, 0),
    ] {
        let c = b.cmp_imm(op, lead, byte);
        set_if(b, c, dst, value);
    }
    (width, lo, hi)
}

// rt.cstr_to_text(ptr) -> text: a NUL-terminated C string copied as UTF-8, each invalid
// sequence (maximal subpart) replaced by U+FFFD, the rule of Rust's `from_utf8_lossy`.
fn cstr_to_text() -> LirFn {
    let mut b = FnBuilder::new("rt.cstr_to_text", 1, None);
    let zero = b.konst(0);
    let len = var(&mut b, zero);
    b.while_(
        |b| {
            let at = add(b, 0, len);
            let byte = b.load8(at, 0);
            b.cmp_imm(CmpOp::Ne, byte, 0)
        },
        |b| b.add_imm(len, 1),
    );
    // every invalid byte can become three
    let three = b.konst(3);
    let room = b.bin(BinOp::Mul, len, three);
    let text = b.call("rt.text_new", &[room]);
    let out = var(&mut b, zero);
    let i = var(&mut b, zero);
    let emit = |b: &mut FnBuilder, byte: VReg| {
        let at = add(b, text, out);
        b.store8(at, 8, byte);
        b.add_imm(out, 1);
    };
    let replace = |b: &mut FnBuilder, consumed: VReg| {
        for byte in [0xef, 0xbf, 0xbd] {
            let k = b.konst(byte);
            emit(b, k);
        }
        b.bin_into(i, BinOp::Add, i, consumed);
    };
    b.while_(
        |b| b.cmp(CmpOp::Lt, i, len),
        |b| {
            let at = add(b, 0, i);
            let lead = b.load8(at, 0);
            let ascii = b.cmp_imm(CmpOp::Lt, lead, 0x80);
            b.if_else(
                ascii,
                |b| {
                    emit(b, lead);
                    b.add_imm(i, 1);
                },
                |b| {
                    let (width, lo, hi) = utf8_lead(b, lead);
                    let one = b.konst(1);
                    let invalid_lead = b.cmp_imm(CmpOp::Eq, width, 0);
                    b.if_else(
                        invalid_lead,
                        |b| replace(b, one),
                        |b| {
                            let second = b.load8(at, 1);
                            let low = b.cmp(CmpOp::Lt, second, lo);
                            let high = b.cmp(CmpOp::Lt, hi, second);
                            let bad = b.bin(BinOp::Or, low, high);
                            b.if_else(
                                bad,
                                |b| replace(b, one),
                                |b| {
                                    let two = b.konst(2);
                                    let k = var(b, two);
                                    let ok = var(b, one);
                                    b.while_(
                                        |b| {
                                            let go = b.cmp(CmpOp::Lt, k, width);
                                            b.bin(BinOp::And, go, ok)
                                        },
                                        |b| {
                                            let next = add(b, at, k);
                                            let byte = b.load8(next, 0);
                                            let low = b.konst(0x80);
                                            let offset = sub(b, byte, low);
                                            let cont = b.cmp_imm(CmpOp::ULt, offset, 0x40);
                                            b.if_else(
                                                cont,
                                                |b| b.add_imm(k, 1),
                                                |b| set_const(b, ok, 0),
                                            );
                                        },
                                    );
                                    b.if_else(
                                        ok,
                                        |b| {
                                            let to = add(b, text, out);
                                            let to = add_n(b, to, 8);
                                            b.call_void("rt.copy", &[to, at, width]);
                                            b.bin_into(out, BinOp::Add, out, width);
                                            b.bin_into(i, BinOp::Add, i, width);
                                        },
                                        |b| replace(b, k),
                                    );
                                },
                            );
                        },
                    );
                },
            );
        },
    );
    b.store(text, 0, out);
    b.ret(Some(text));
    b.finish()
}

// rt.ncpu() -> n: CPUs in the affinity mask, at least 1
fn ncpu() -> LirFn {
    let mut b = FnBuilder::new("rt.ncpu", 0, None);
    let size = b.konst(128);
    let mask = b.call("rt.alloc", &[size]);
    let bytes = b.call("rt.sys_getaffinity", &[mask]);
    let zero = b.konst(0);
    let count = var(&mut b, zero);
    let got = b.cmp(CmpOp::Lt, zero, bytes);
    b.if_(got, |b| {
        for_each(b, bytes, |b, i| {
            let at = add(b, mask, i);
            let byte = b.load8(at, 0);
            let bits = var(b, byte);
            let one = b.konst(1);
            b.while_(
                |b| b.cmp(CmpOp::Ne, bits, zero),
                |b| {
                    let low = b.bin(BinOp::And, bits, one);
                    b.bin_into(count, BinOp::Add, count, low);
                    b.bin_into(bits, BinOp::Shr, bits, one);
                },
            );
        });
    });
    let none = b.cmp(CmpOp::Eq, count, zero);
    set_if(&mut b, none, count, 1);
    b.ret(Some(count));
    b.finish()
}

// rt.process() -> Process from the initial stack (`argc`, `argv...`) that rt0 saved
fn process() -> LirFn {
    let mut b = FnBuilder::new("rt.process", 0, None);
    let slot = b.addr("rt.argp");
    let initial_sp = b.load(slot, 0);
    let argc = b.load(initial_sp, 0);
    let zero = b.konst(0);
    let program = b.call("rt.text_new", &[zero]);
    let nargs = var(&mut b, zero);
    let has_program = b.cmp(CmpOp::Lt, zero, argc);
    b.if_(has_program, |b| {
        let argv0 = b.load(initial_sp, 8);
        let text = b.call("rt.cstr_to_text", &[argv0]);
        b.mov(program, text);
        let rest = add_n(b, argc, -1);
        b.mov(nargs, rest);
    });
    let three = b.konst(3);
    let words = b.bin(BinOp::Shl, nargs, three);
    let size = add_n(&mut b, words, 8);
    let list = b.call("rt.alloc", &[size]);
    b.store(list, 0, nargs);
    // argv[i + 1] is at initial_sp + 16 + 8i, the offset of list element i plus 8
    let argv1 = add_n(&mut b, initial_sp, 8);
    for_each(&mut b, nargs, |b, i| {
        let from = elem_addr(b, argv1, i);
        let arg = b.load(from, 0);
        let text = b.call("rt.cstr_to_text", &[arg]);
        let to = elem_addr(b, list, i);
        b.store(to, 0, text);
    });
    let os = b.addr("rt.os");
    let arch = b.addr("rt.arch");
    let pid = b.call("rt.sys_getpid", &[]);
    let tid = b.call("rt.sys_gettid", &[]);
    let name = b.addr("rt.main_name");
    let size = b.konst(16);
    let thread = b.call("rt.alloc", &[size]);
    b.store(thread, 0, tid);
    b.store(thread, 8, name);
    let parallelism = b.call("rt.ncpu", &[]);
    let size = b.konst(56);
    let process = b.call("rt.alloc", &[size]);
    for (i, field) in [program, list, os, arch, pid, thread, parallelism]
        .into_iter()
        .enumerate()
    {
        b.store(process, i32::try_from(8 * i).unwrap_or(0), field);
    }
    b.ret(Some(process));
    b.finish()
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
