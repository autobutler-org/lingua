//! A test-only LIR evaluator (docs/build.md, Testing): runs runtime routines and lowered
//! programs on any host, with the system-call shims modeled in Rust.
// ponytail: test-only LIR evaluator; drop it if end-to-end tests come to cover every routine
// on every CI target.

use std::collections::HashMap;

use super::lir::{BinOp, ChkOp, CmpOp, Inst, LirFn, LirProgram, RoData, Term, site_symbol};
use super::runtime;

const NULL_PAGE: usize = 0x1000;

/// Why evaluation stopped short of a return.
#[derive(Debug, PartialEq)]
pub enum Stop {
    Exit(i64),
}

// How a function body ended: a return, or a tail call of `callee` with these arguments
enum Exit<'a> {
    Ret(u64),
    Tail(&'a str, Vec<u64>),
}

pub struct Machine<'a> {
    functions: HashMap<&'a str, &'a LirFn>,
    symbols: HashMap<String, u64>,
    mem: Vec<u8>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// Each `sys_write` writes at most this many bytes, and every other call fails `EINTR`.
    pub short_writes: Option<usize>,
    writes: usize,
    pub mmap_fails: bool,
    /// Calls deeper than this fail the callee's stack check, if it has one.
    pub max_depth: usize,
    depth: usize,
}

impl<'a> Machine<'a> {
    /// A machine holding `functions` (user and runtime) and `rodata`, plus the runtime's
    /// rodata, `rt.os`/`rt.arch`, and `.bss`.
    #[must_use]
    pub fn new(functions: &'a [LirFn], rodata: &[RoData]) -> Machine<'a> {
        let mut machine = Machine {
            functions: functions.iter().map(|f| (f.name.as_str(), f)).collect(),
            symbols: HashMap::new(),
            mem: vec![0; NULL_PAGE],
            stdout: Vec::new(),
            stderr: Vec::new(),
            short_writes: None,
            writes: 0,
            mmap_fails: false,
            max_depth: 1000,
            depth: 0,
        };
        let target = [
            super::lir::text_block("rt.os", "linux"),
            super::lir::text_block("rt.arch", "x86_64"),
        ];
        for block in rodata.iter().chain(&runtime::rodata()).chain(&target) {
            let at = machine.place(&block.bytes);
            machine.symbols.insert(block.name.clone(), at);
        }
        for name in runtime::bss() {
            let at = machine.place(&[0; 8]);
            machine.symbols.insert(name, at);
        }
        machine
    }

    fn place(&mut self, bytes: &[u8]) -> u64 {
        let at = self.mem.len().next_multiple_of(8);
        self.mem.resize(at, 0);
        self.mem.extend(bytes);
        at as u64
    }

    /// Lays out `argv` the way the kernel does (`argc`, pointers, NUL, then the strings)
    /// and points `rt.argp` at it.
    pub fn set_argv(&mut self, argv: &[&[u8]]) {
        let strings: Vec<u64> = argv
            .iter()
            .map(|arg| self.place(&[*arg, &[0]].concat()))
            .collect();
        let mut words = vec![argv.len() as u64];
        words.extend(strings);
        words.push(0);
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        let at = self.place(&bytes);
        let slot = self.symbols["rt.argp"];
        self.store(slot, at);
    }

    fn index(&self, addr: u64, len: usize) -> usize {
        let at = usize::try_from(addr).unwrap();
        assert!(
            at >= NULL_PAGE && at + len <= self.mem.len(),
            "bad access at {addr:#x}"
        );
        at
    }

    #[must_use]
    pub fn load(&self, addr: u64) -> u64 {
        let at = self.index(addr, 8);
        u64::from_le_bytes(self.mem[at..at + 8].try_into().unwrap())
    }

    pub fn store(&mut self, addr: u64, value: u64) {
        let at = self.index(addr, 8);
        self.mem[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }

    /// A fresh Text block holding `text`.
    pub fn text(&mut self, text: &[u8]) -> u64 {
        let mut bytes = (text.len() as u64).to_le_bytes().to_vec();
        bytes.extend(text);
        self.place(&bytes)
    }

    /// The bytes of the Text block at `addr`.
    #[must_use]
    pub fn read_text(&self, addr: u64) -> Vec<u8> {
        let len = usize::try_from(self.load(addr)).unwrap();
        let at = self.index(addr + 8, len);
        self.mem[at..at + len].to_vec()
    }

    /// A fresh list of `words`.
    pub fn list(&mut self, words: &[u64]) -> u64 {
        let bytes: Vec<u8> = std::iter::once(words.len() as u64)
            .chain(words.iter().copied())
            .flat_map(u64::to_le_bytes)
            .collect();
        self.place(&bytes)
    }

    /// The words of the list at `addr`.
    #[must_use]
    pub fn read_list(&self, addr: u64) -> Vec<u64> {
        (0..self.load(addr))
            .map(|i| self.load(addr + 8 + 8 * i))
            .collect()
    }

    #[must_use]
    pub fn symbol(&self, name: &str) -> u64 {
        self.symbols[name]
    }

    fn raise(&mut self, symbol: &str) -> Stop {
        let report = self.read_text(self.symbols[symbol]);
        self.stderr.extend(report);
        Stop::Exit(1)
    }

    fn shim(&mut self, name: &str, args: &[u64]) -> Result<u64, Stop> {
        Ok(match name {
            "rt.sys_write" => {
                self.writes += 1;
                if self.short_writes.is_some() && self.writes.is_multiple_of(2) {
                    return Ok((-4i64).cast_unsigned());
                }
                let len = usize::try_from(args[2]).unwrap();
                let len = self.short_writes.map_or(len, |most| len.min(most));
                let at = self.index(args[1], len);
                let bytes = self.mem[at..at + len].to_vec();
                match args[0] {
                    1 => self.stdout.extend(bytes),
                    _ => self.stderr.extend(bytes),
                }
                len as u64
            }
            "rt.sys_exit" => return Err(Stop::Exit(args[0].cast_signed())),
            "rt.sys_mmap" if self.mmap_fails => 0,
            "rt.sys_mmap" => {
                let at = self.mem.len().next_multiple_of(0x1000);
                self.mem.resize(at + usize::try_from(args[0]).unwrap(), 0);
                at as u64
            }
            "rt.sys_clock" => {
                self.store(args[0], 1_700_000_000);
                self.store(args[0] + 8, 5);
                0
            }
            "rt.sys_getpid" => 42,
            "rt.sys_gettid" => 43,
            "rt.sys_getaffinity" => {
                self.store(args[0], 0b1011_0000_0001);
                8
            }
            _ => panic!("call of undefined `{name}`"),
        })
    }

    /// Calls `name` with `args` under the Lingua ABI.
    ///
    /// # Errors
    /// `Stop::Exit` when the program exits, faults, or raises.
    pub fn call(&mut self, name: &str, args: &[u64]) -> Result<u64, Stop> {
        // A tail call replaces the frame: the callee runs at the same depth, after its check
        let (mut name, mut args) = (name, args.to_vec());
        loop {
            let Some(&function) = self.functions.get(name) else {
                return self.shim(name, &args);
            };
            assert_eq!(args.len(), function.params as usize, "arity of {name}");
            if let (Some(site), true) = (function.stack_site, self.depth >= self.max_depth) {
                return Err(self.raise(&site_symbol(site, 0)));
            }
            self.depth += 1;
            let result = self.run(function, &args);
            self.depth -= 1;
            match result? {
                Exit::Ret(value) => return Ok(value),
                Exit::Tail(callee, callee_args) => (name, args) = (callee, callee_args),
            }
        }
    }

    #[allow(clippy::too_many_lines, clippy::many_single_char_names)]
    fn run(&mut self, function: &'a LirFn, args: &[u64]) -> Result<Exit<'a>, Stop> {
        let mut v = vec![0u64; function.vregs as usize];
        v[..args.len()].copy_from_slice(args);
        let mut block = &function.blocks[0];
        loop {
            for inst in &block.insts {
                match inst {
                    Inst::Const(d, n) => v[*d as usize] = n.cast_unsigned(),
                    Inst::Addr(d, sym) => {
                        v[*d as usize] = *self
                            .symbols
                            .get(sym)
                            .unwrap_or_else(|| panic!("undefined symbol `{sym}`"));
                    }
                    Inst::Move(d, a) => v[*d as usize] = v[*a as usize],
                    Inst::Load(d, a, off) => {
                        v[*d as usize] =
                            self.load(v[*a as usize].wrapping_add_signed((*off).into()));
                    }
                    Inst::Store(a, off, b) => {
                        let at = v[*a as usize].wrapping_add_signed((*off).into());
                        self.store(at, v[*b as usize]);
                    }
                    Inst::Load8(d, a, off) => {
                        let at = self.index(v[*a as usize].wrapping_add_signed((*off).into()), 1);
                        v[*d as usize] = u64::from(self.mem[at]);
                    }
                    Inst::Store8(a, off, b) => {
                        let at = self.index(v[*a as usize].wrapping_add_signed((*off).into()), 1);
                        self.mem[at] = v[*b as usize].to_le_bytes()[0];
                    }
                    Inst::Bin(op, d, a, b) => {
                        let (a, b) = (v[*a as usize], v[*b as usize]);
                        v[*d as usize] = match op {
                            BinOp::Add => a.wrapping_add(b),
                            BinOp::Sub => a.wrapping_sub(b),
                            BinOp::Mul => a.wrapping_mul(b),
                            BinOp::And => a & b,
                            BinOp::Or => a | b,
                            BinOp::Shl => a << (b & 63),
                            BinOp::Shr => a >> (b & 63),
                            BinOp::UDiv => a / b,
                            BinOp::URem => a % b,
                        };
                    }
                    Inst::Chk(op, d, a, b, site) => {
                        let (a, b) = (v[*a as usize].cast_signed(), v[*b as usize].cast_signed());
                        let result = match op {
                            ChkOp::Add => a.checked_add(b).ok_or(0),
                            ChkOp::Sub => a.checked_sub(b).ok_or(0),
                            ChkOp::Mul => a.checked_mul(b).ok_or(0),
                            ChkOp::Div | ChkOp::Rem if b == 0 => Err(0),
                            ChkOp::Div => a.checked_div(b).ok_or(1),
                            ChkOp::Rem => a.checked_rem(b).ok_or(1),
                        };
                        match result {
                            Ok(n) => v[*d as usize] = n.cast_unsigned(),
                            Err(index) => return Err(self.raise(&site_symbol(*site, index))),
                        }
                    }
                    Inst::Neg(d, a, site) => match v[*a as usize].cast_signed().checked_neg() {
                        Some(n) => v[*d as usize] = n.cast_unsigned(),
                        None => return Err(self.raise(&site_symbol(*site, 0))),
                    },
                    Inst::Cmp(op, d, a, b) => {
                        let (ua, ub) = (v[*a as usize], v[*b as usize]);
                        let (a, b) = (ua.cast_signed(), ub.cast_signed());
                        v[*d as usize] = u64::from(match op {
                            CmpOp::Eq => a == b,
                            CmpOp::Ne => a != b,
                            CmpOp::Lt => a < b,
                            CmpOp::Le => a <= b,
                            CmpOp::Gt => a > b,
                            CmpOp::Ge => a >= b,
                            CmpOp::ULt => ua < ub,
                        });
                    }
                    Inst::Call(d, f, a) => {
                        let values: Vec<u64> = a.iter().map(|r| v[*r as usize]).collect();
                        let result = self.call(f, &values)?;
                        if let Some(d) = d {
                            v[*d as usize] = result;
                        }
                    }
                }
            }
            block = match block.term {
                Term::Jump(to) => &function.blocks[to as usize],
                Term::Branch(c, then, otherwise) => {
                    &function.blocks[if v[c as usize] == 0 { otherwise } else { then } as usize]
                }
                Term::Ret(value) => return Ok(Exit::Ret(value.map_or(0, |r| v[r as usize]))),
                Term::Raise(site) => return Err(self.raise(&site_symbol(site, 0))),
                Term::TailCall(ref callee) => {
                    let params = v[..function.params as usize].to_vec();
                    return Ok(Exit::Tail(callee, params));
                }
            };
        }
    }
}

/// Runs a lowered program from `rt.entry` with `argv`: the exit code (0 when `main`
/// returns), stderr, and the machine.
#[must_use]
pub fn run_program(program: &LirProgram, argv: &[&[u8]]) -> (i64, String) {
    let owned: Vec<LirFn> = program
        .functions
        .iter()
        .chain([&program.entry])
        .cloned()
        .chain(runtime::functions())
        .collect();
    let mut machine = Machine::new(&owned, &program.rodata);
    machine.set_argv(argv);
    let Err(Stop::Exit(code)) = machine.call("rt.entry", &[]) else {
        panic!("rt.entry returned instead of exiting");
    };
    (code, String::from_utf8_lossy(&machine.stderr).into_owned())
}
