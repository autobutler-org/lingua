//! Encoder vectors: one row per table entry, `Prog` text and the expected bytes in hex.
//! The bytes were produced with GNU `as` (binutils 2.40, `--64`) and checked in; label
//! branches against `{disp32}` forms, since the table always uses rel32. `as` picks the
//! shorter `48 3d id` for `CMPQ AX, $imm32`; the table's `81 /7 id` is equivalent, so the
//! vector uses `CX`.

use super::*;
use crate::build::prog::{Addr, CALL, JMP, LABEL, Prog, RET, TEXT};

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
#[allow(clippy::too_many_lines)]
fn backend_rows_match_reference_bytes() {
    let m = |base, off| Addr::Mem { base, off };
    let none = Addr::None;
    check_rows(&[
        (
            Prog::new(LEAQ, m(SP, -48), Addr::Reg(R11)),
            "LEAQ  -48(SP), R11",
            "4c 8d 5c 24 d0",
        ),
        (
            Prog::new(LEAQ, m(AX, 8), Addr::Reg(CX)),
            "LEAQ  8(AX), CX",
            "48 8d 48 08",
        ),
        (
            Prog::new(CMPQ, Addr::Reg(R11), Addr::sym("rt.stack_limit")),
            "CMPQ  R11, rt.stack_limit(SB)",
            "4c 3b 1d 00 00 00 00",
        ),
        (
            Prog::new(CMPQ, Addr::Reg(AX), Addr::Reg(CX)),
            "CMPQ  AX, CX",
            "48 39 c8",
        ),
        (
            Prog::new(CMPQ, Addr::Reg(R9), Addr::Reg(DX)),
            "CMPQ  R9, DX",
            "49 39 d1",
        ),
        (
            Prog::new(CMPQ, Addr::Reg(CX), Addr::Imm(-1)),
            "CMPQ  CX, $-1",
            "48 83 f9 ff",
        ),
        (
            Prog::new(CMPQ, Addr::Reg(CX), Addr::Imm(100_000)),
            "CMPQ  CX, $100000",
            "48 81 f9 a0 86 01 00",
        ),
        (
            Prog::new(TESTQ, Addr::Reg(CX), Addr::Reg(CX)),
            "TESTQ CX, CX",
            "48 85 c9",
        ),
        (
            Prog::new(SUBQ, Addr::Reg(CX), Addr::Reg(AX)),
            "SUBQ  CX, AX",
            "48 29 c8",
        ),
        (
            Prog::new(ANDQ, Addr::Reg(CX), Addr::Reg(AX)),
            "ANDQ  CX, AX",
            "48 21 c8",
        ),
        (
            Prog::new(ORQ, Addr::Reg(CX), Addr::Reg(AX)),
            "ORQ   CX, AX",
            "48 09 c8",
        ),
        (
            Prog::new(ADDQ, Addr::Imm(1), Addr::Reg(AX)),
            "ADDQ  $1, AX",
            "48 83 c0 01",
        ),
        (
            Prog::new(ANDQ, Addr::Imm(-16), Addr::Reg(SP)),
            "ANDQ  $-16, SP",
            "48 83 e4 f0",
        ),
        (
            Prog::new(IMULQ, Addr::Reg(CX), Addr::Reg(AX)),
            "IMULQ CX, AX",
            "48 0f af c1",
        ),
        (
            Prog::new(IMULQ, Addr::Reg(R11), Addr::Reg(R8)),
            "IMULQ R11, R8",
            "4d 0f af c3",
        ),
        (
            Prog::new(SHLQ, Addr::Reg(CX), Addr::Reg(AX)),
            "SHLQ  CX, AX",
            "48 d3 e0",
        ),
        (
            Prog::new(SHRQ, Addr::Reg(CX), Addr::Reg(AX)),
            "SHRQ  CX, AX",
            "48 d3 e8",
        ),
        (
            Prog::new(NEGQ, none.clone(), Addr::Reg(AX)),
            "NEGQ  AX",
            "48 f7 d8",
        ),
        (Prog::new(CQO, none.clone(), none.clone()), "CQO", "48 99"),
        (
            Prog::new(IDIVQ, Addr::Reg(CX), none.clone()),
            "IDIVQ CX",
            "48 f7 f9",
        ),
        (
            Prog::new(DIVQ, Addr::Reg(CX), none.clone()),
            "DIVQ  CX",
            "48 f7 f1",
        ),
        (
            Prog::new(SETEQ, none.clone(), Addr::Reg(AX)),
            "SETEQ AX",
            "0f 94 c0",
        ),
        (
            Prog::new(SETNE, none.clone(), Addr::Reg(AX)),
            "SETNE AX",
            "0f 95 c0",
        ),
        (
            Prog::new(SETLT, none.clone(), Addr::Reg(AX)),
            "SETLT AX",
            "0f 9c c0",
        ),
        (
            Prog::new(SETLE, none.clone(), Addr::Reg(AX)),
            "SETLE AX",
            "0f 9e c0",
        ),
        (
            Prog::new(SETGT, none.clone(), Addr::Reg(AX)),
            "SETGT AX",
            "0f 9f c0",
        ),
        (
            Prog::new(SETGE, none.clone(), Addr::Reg(AX)),
            "SETGE AX",
            "0f 9d c0",
        ),
        (
            Prog::new(SETCS, none.clone(), Addr::Reg(AX)),
            "SETCS AX",
            "0f 92 c0",
        ),
        (
            Prog::new(SETEQ, none.clone(), Addr::Reg(SI)),
            "SETEQ SI",
            "40 0f 94 c6",
        ),
        (
            Prog::new(SETEQ, none.clone(), Addr::Reg(R8)),
            "SETEQ R8",
            "41 0f 94 c0",
        ),
        (
            Prog::new(MOVBQZX, Addr::Reg(AX), Addr::Reg(AX)),
            "MOVBQZX AX, AX",
            "48 0f b6 c0",
        ),
        (
            Prog::new(MOVBQZX, m(AX, 8), Addr::Reg(AX)),
            "MOVBQZX 8(AX), AX",
            "48 0f b6 40 08",
        ),
        (
            Prog::new(MOVBQZX, m(R9, 0), Addr::Reg(R10)),
            "MOVBQZX 0(R9), R10",
            "4d 0f b6 11",
        ),
        (
            Prog::new(MOVB, Addr::Reg(CX), m(AX, 8)),
            "MOVB  CX, 8(AX)",
            "88 48 08",
        ),
        (
            Prog::new(MOVB, Addr::Reg(SI), m(AX, 0)),
            "MOVB  SI, 0(AX)",
            "40 88 30",
        ),
        (
            Prog::new(MOVB, Addr::Reg(CX), m(R8, 0)),
            "MOVB  CX, 0(R8)",
            "41 88 08",
        ),
        (
            Prog::new(MOVQ, Addr::Imm(0), Addr::Reg(DX)),
            "MOVQ  $0, DX",
            "48 c7 c2 00 00 00 00",
        ),
        (
            Prog::new(JMP, none.clone(), Addr::sym("rt.fault")),
            "JMP   rt.fault(SB)",
            "e9 00 00 00 00",
        ),
    ]);
}

#[test]
fn label_branches_are_rel32_from_the_end_of_the_instruction() {
    let encoded = encode(&[
        Prog::new(TEXT, Addr::sym("f"), Addr::None),
        Prog::new(LABEL, Addr::Label(0), Addr::None),
        Prog::new(JNE, Addr::None, Addr::Label(1)),
        Prog::new(JEQ, Addr::None, Addr::Label(1)),
        Prog::new(JOS, Addr::None, Addr::Label(1)),
        Prog::new(JCS, Addr::None, Addr::Label(1)),
        Prog::new(JMP, Addr::None, Addr::Label(0)),
        Prog::new(LABEL, Addr::Label(1), Addr::None),
        Prog::new(RET, Addr::None, Addr::None),
    ])
    .unwrap();
    assert_eq!(
        hex(&encoded.bytes),
        "0f 85 17 00 00 00 0f 84 11 00 00 00 0f 80 0b 00 00 00 0f 82 05 00 00 00 e9 e3 ff ff ff c3"
    );
    assert_eq!(encoded.relocs, []);
}

#[test]
fn an_undefined_label_is_an_internal_error() {
    let err = encode(&[
        Prog::new(TEXT, Addr::sym("f"), Addr::None),
        Prog::new(JMP, Addr::None, Addr::Label(7)),
    ])
    .unwrap_err();
    assert_eq!(err, "internal error: undefined label L7 in f");
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
        stack_site: None,
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
fn line_subtotal_selects_as_in_the_worked_example() {
    use crate::build::lir::{Block, ChkOp, Inst, LirFn, Term};
    let function = LirFn {
        name: "order.line_subtotal".to_owned(),
        params: 1,
        vregs: 4,
        blocks: vec![Block {
            insts: vec![
                Inst::Load(1, 0, 8),
                Inst::Load(2, 0, 16),
                Inst::Chk(ChkOp::Mul, 3, 1, 2, 1),
            ],
            term: Term::Ret(Some(3)),
        }],
        stack_site: Some(0),
    };
    let progs = select(&function);
    assert_eq!(
        dump(&progs),
        "TEXT order.line_subtotal(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  -16(BP), CX
  IMULQ CX, AX
  JOS   L2
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
"
    );
    assert!(encode(&progs).is_ok());
}

#[test]
fn every_lir_form_selects_to_encodable_progs() {
    use crate::build::lir::{BinOp, Block, ChkOp, CmpOp, Inst, LirFn, Term};
    let mut insts = vec![Inst::Const(1, i64::MIN), Inst::Move(2, 1)];
    insts.extend(
        [
            BinOp::Add,
            BinOp::Sub,
            BinOp::Mul,
            BinOp::And,
            BinOp::Or,
            BinOp::Shl,
            BinOp::Shr,
            BinOp::UDiv,
            BinOp::URem,
        ]
        .map(|op| Inst::Bin(op, 3, 1, 2)),
    );
    insts.extend(
        [ChkOp::Add, ChkOp::Sub, ChkOp::Mul, ChkOp::Div, ChkOp::Rem]
            .map(|op| Inst::Chk(op, 3, 1, 2, 0)),
    );
    insts.extend(
        [
            CmpOp::Eq,
            CmpOp::Ne,
            CmpOp::Lt,
            CmpOp::Le,
            CmpOp::Gt,
            CmpOp::Ge,
            CmpOp::ULt,
        ]
        .map(|op| Inst::Cmp(op, 3, 1, 2)),
    );
    insts.extend([
        Inst::Neg(3, 1, 0),
        Inst::Load8(3, 0, 9),
        Inst::Store(0, 8, 3),
        Inst::Store8(0, 9, 3),
    ]);
    let function = LirFn {
        name: "f".to_owned(),
        params: 1,
        vregs: 4,
        blocks: vec![
            Block {
                insts,
                term: Term::Branch(3, 1, 2),
            },
            Block {
                insts: vec![],
                term: Term::Jump(2),
            },
            Block {
                insts: vec![],
                term: Term::Raise(0),
            },
        ],
        stack_site: Some(1),
    };
    let progs = select(&function);
    let text = dump(&progs);
    for expected in [
        "  TESTQ CX, CX\n  JEQ   L7\n  CMPQ  CX, $-1\n  JNE   L9\n  MOVQ  $-9223372036854775808, DX\n  CMPQ  AX, DX\n  JEQ   L8\nLABEL L9\n  CQO\n  IDIVQ CX\n  MOVQ  AX, -24(BP)\n",
        "  MOVQ  $0, DX\n  DIVQ  CX\n  MOVQ  DX, -24(BP)\n",
        "  CMPQ  AX, CX\n  SETCS AX\n  MOVBQZX AX, AX\n",
        "  TESTQ AX, AX\n  JNE   L1\n  JMP   L2\nLABEL L1\n  JMP   L2\nLABEL L2\n  LEAQ  site.0(SB), AX\n  JMP   rt.fault(SB)\n",
        "  MOVQ  -24(BP), CX\n  MOVB  CX, 9(AX)\n",
    ] {
        assert!(text.contains(expected), "{expected}\nnot in\n{text}");
    }
    assert!(encode(&progs).is_ok(), "{:?}", encode(&progs));
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
        stack_site: None,
    };
    assert_eq!(
        dump(&select(&function)),
        "TEXT f(SB)\n  PUSHQ BP\n  MOVQ  SP, BP\n  SUBQ  $16, SP\n  MOVQ  BP, SP\n  POPQ  BP\n  RET\n"
    );
}
