//! Encoder vectors: one row per table entry, `Prog` text and the expected bytes in hex.
//! The bytes were produced with GNU `as` (binutils, `--64`) and checked in.

use super::*;
use crate::build::prog::{Addr, CALL, Prog, RET, TEXT};

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn encode_one(prog: &Prog) -> Encoded {
    encode(&[Prog::new(TEXT, Addr::sym("f"), Addr::None), prog.clone()]).unwrap()
}

fn check_rows(rows: &[(Prog, &str, &str)]) {
    for (prog, text, bytes) in rows {
        assert_eq!(dump(std::slice::from_ref(prog)), format!("  {text}\n"));
        assert_eq!(hex(&encode_one(prog).bytes), *bytes, "{text}");
    }
}

#[test]
fn register_and_immediate_rows_match_reference_bytes() {
    check_rows(&[
        (
            Prog::new(MOVQ, Addr::Imm(2), Addr::Reg(DI)),
            "MOVQ  $2, DI",
            "48 c7 c7 02 00 00 00",
        ),
        (
            Prog::new(MOVQ, Addr::Imm(231), Addr::Reg(AX)),
            "MOVQ  $231, AX",
            "48 c7 c0 e7 00 00 00",
        ),
        (
            Prog::new(MOVQ, Addr::Imm(-1), Addr::Reg(R10)),
            "MOVQ  $-1, R10",
            "49 c7 c2 ff ff ff ff",
        ),
        (
            Prog::new(MOVQ, Addr::Imm(0x1_2345_6789), Addr::Reg(AX)),
            "MOVQ  $4886718345, AX",
            "48 b8 89 67 45 23 01 00 00 00",
        ),
        (
            Prog::new(MOVQ, Addr::Imm(i64::MIN), Addr::Reg(R9)),
            "MOVQ  $-9223372036854775808, R9",
            "49 b9 00 00 00 00 00 00 00 80",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(SP), Addr::Reg(BP)),
            "MOVQ  SP, BP",
            "48 89 e5",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(R11), Addr::Reg(AX)),
            "MOVQ  R11, AX",
            "4c 89 d8",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(AX), Addr::Reg(R15)),
            "MOVQ  AX, R15",
            "49 89 c7",
        ),
        (
            Prog::new(LEAQ, Addr::sym("msg"), Addr::Reg(SI)),
            "LEAQ  msg(SB), SI",
            "48 8d 35 00 00 00 00",
        ),
        (
            Prog::new(LEAQ, Addr::sym("msg"), Addr::Reg(R11)),
            "LEAQ  msg(SB), R11",
            "4c 8d 1d 00 00 00 00",
        ),
    ]);
}

#[test]
fn memory_rows_match_reference_bytes() {
    check_rows(&[
        (
            Prog::new(MOVQ, Addr::Mem { base: BP, off: 16 }, Addr::Reg(AX)),
            "MOVQ  16(BP), AX",
            "48 8b 45 10",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: BP, off: -8 }, Addr::Reg(AX)),
            "MOVQ  -8(BP), AX",
            "48 8b 45 f8",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: BP, off: -16 }, Addr::Reg(CX)),
            "MOVQ  -16(BP), CX",
            "48 8b 4d f0",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: AX, off: 8 }, Addr::Reg(AX)),
            "MOVQ  8(AX), AX",
            "48 8b 40 08",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: AX, off: 0 }, Addr::Reg(AX)),
            "MOVQ  0(AX), AX",
            "48 8b 00",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: SP, off: 8 }, Addr::Reg(DI)),
            "MOVQ  8(SP), DI",
            "48 8b 7c 24 08",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: BP, off: 0 }, Addr::Reg(AX)),
            "MOVQ  0(BP), AX",
            "48 8b 45 00",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: R13, off: 0 }, Addr::Reg(AX)),
            "MOVQ  0(R13), AX",
            "49 8b 45 00",
        ),
        (
            Prog::new(MOVQ, Addr::Mem { base: R12, off: 24 }, Addr::Reg(DX)),
            "MOVQ  24(R12), DX",
            "49 8b 54 24 18",
        ),
        (
            Prog::new(
                MOVQ,
                Addr::Mem {
                    base: BP,
                    off: -1024,
                },
                Addr::Reg(AX),
            ),
            "MOVQ  -1024(BP), AX",
            "48 8b 85 00 fc ff ff",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(AX), Addr::Mem { base: BP, off: -8 }),
            "MOVQ  AX, -8(BP)",
            "48 89 45 f8",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(AX), Addr::Mem { base: SP, off: 0 }),
            "MOVQ  AX, 0(SP)",
            "48 89 04 24",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(AX), Addr::Mem { base: SP, off: 32 }),
            "MOVQ  AX, 32(SP)",
            "48 89 44 24 20",
        ),
        (
            Prog::new(MOVQ, Addr::Reg(R10), Addr::Mem { base: SP, off: 8 }),
            "MOVQ  R10, 8(SP)",
            "4c 89 54 24 08",
        ),
    ]);
}

#[test]
fn arithmetic_stack_and_call_rows_match_reference_bytes() {
    check_rows(&[
        (
            Prog::new(ADDQ, Addr::Reg(CX), Addr::Reg(AX)),
            "ADDQ  CX, AX",
            "48 01 c8",
        ),
        (
            Prog::new(ADDQ, Addr::Reg(R11), Addr::Reg(R8)),
            "ADDQ  R11, R8",
            "4d 01 d8",
        ),
        (
            Prog::new(SUBQ, Addr::Imm(32), Addr::Reg(SP)),
            "SUBQ  $32, SP",
            "48 83 ec 20",
        ),
        (
            Prog::new(SUBQ, Addr::Imm(4096), Addr::Reg(SP)),
            "SUBQ  $4096, SP",
            "48 81 ec 00 10 00 00",
        ),
        (
            Prog::new(PUSHQ, Addr::Reg(BP), Addr::None),
            "PUSHQ BP",
            "55",
        ),
        (
            Prog::new(PUSHQ, Addr::Reg(R12), Addr::None),
            "PUSHQ R12",
            "41 54",
        ),
        (Prog::new(POPQ, Addr::None, Addr::Reg(BP)), "POPQ  BP", "5d"),
        (
            Prog::new(POPQ, Addr::None, Addr::Reg(R15)),
            "POPQ  R15",
            "41 5f",
        ),
        (
            Prog::new(CALL, Addr::None, Addr::sym("f")),
            "CALL  f(SB)",
            "e8 00 00 00 00",
        ),
        (Prog::new(RET, Addr::None, Addr::None), "RET", "c3"),
        (
            Prog::new(SYSCALL, Addr::None, Addr::None),
            "SYSCALL",
            "0f 05",
        ),
    ]);
}

#[test]
fn leaq_of_a_symbol_records_a_pcrel32_at_its_displacement() {
    let encoded = encode(&[
        Prog::new(TEXT, Addr::sym("_rt0_linux_amd64"), Addr::None),
        Prog::new(MOVQ, Addr::Imm(2), Addr::Reg(DI)),
        Prog::new(
            LEAQ,
            Addr::Sym {
                name: "msg".to_owned(),
                off: 8,
            },
            Addr::Reg(SI),
        ),
    ])
    .unwrap();
    assert_eq!(encoded.name, "_rt0_linux_amd64");
    assert_eq!(
        encoded.relocs,
        [Reloc::Pcrel32 {
            at: 10,
            sym: "msg".to_owned(),
            off: 8
        }]
    );
}

#[test]
fn call_records_a_pcrel32_after_its_opcode() {
    let encoded = encode(&[
        Prog::new(TEXT, Addr::sym("f"), Addr::None),
        Prog::new(CALL, Addr::None, Addr::sym("rt.log")),
    ])
    .unwrap();
    assert_eq!(
        encoded.relocs,
        [Reloc::Pcrel32 {
            at: 1,
            sym: "rt.log".to_owned(),
            off: 0
        }]
    );
}

#[test]
fn an_operand_combination_with_no_row_names_the_prog() {
    let err = encode(&[
        Prog::new(TEXT, Addr::sym("f"), Addr::None),
        Prog::new(LEAQ, Addr::Imm(1), Addr::Reg(AX)),
    ])
    .unwrap_err();
    assert_eq!(
        err,
        "internal error: amd64 has no encoding for `LEAQ  $1, AX` in f"
    );
}

#[test]
fn a_function_must_open_with_text() {
    let err = encode(&[Prog::new(SYSCALL, Addr::None, Addr::None)]).unwrap_err();
    assert_eq!(
        err,
        "internal error: function does not start with TEXT: `SYSCALL`"
    );
}

#[test]
fn selection_spills_every_vreg_and_follows_the_lingua_abi() {
    use crate::build::lir::{BinOp, Block, Inst, LirFn, Term};
    let function = LirFn {
        name: "f".to_owned(),
        params: 1,
        vregs: 5,
        blocks: vec![Block {
            insts: vec![
                Inst::Const(1, 2),
                Inst::Addr(2, "str.0".to_owned()),
                Inst::Load(3, 0, 8),
                Inst::Bin(BinOp::Add, 4, 1, 3),
                Inst::Call(Some(4), "rt.g".to_owned(), vec![1, 4]),
                Inst::Call(None, "rt.h".to_owned(), vec![]),
            ],
            term: Term::Ret(Some(4)),
        }],
    };
    let progs = select(&function);
    // Frame: 4 local slots + 2 args + 1 result = 56, rounded up to 64.
    // v0 is the parameter at 16(BP); this function's result slot is 24(BP).
    assert_eq!(
        dump(&progs),
        "TEXT f(SB)
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $64, SP
  MOVQ  $2, AX
  MOVQ  AX, -8(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -16(BP)
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -24(BP)
  MOVQ  -8(BP), AX
  MOVQ  -24(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -32(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.g(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -32(BP)
  CALL  rt.h(SB)
  MOVQ  -32(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
"
    );
    assert!(
        encode(&progs).is_ok(),
        "every selected Prog has a table row"
    );
}

#[test]
fn a_frame_without_calls_still_reserves_a_result_slot_and_stays_16_byte_aligned() {
    use crate::build::lir::{Block, LirFn, Term};
    let function = LirFn {
        name: "f".to_owned(),
        params: 0,
        vregs: 0,
        blocks: vec![Block {
            insts: vec![],
            term: Term::Ret(None),
        }],
    };
    assert_eq!(
        dump(&select(&function)),
        "TEXT f(SB)\n  PUSHQ BP\n  MOVQ  SP, BP\n  SUBQ  $16, SP\n  MOVQ  BP, SP\n  POPQ  BP\n  RET\n"
    );
}
