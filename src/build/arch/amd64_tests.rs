//! Encoder vectors: one row per table entry, `Prog` text and the expected bytes in hex.
//! The bytes were produced with GNU `as` (binutils, `--64`) and checked in.

use super::*;
use crate::build::prog::{Addr, Prog, TEXT};

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

#[test]
fn table_rows_match_reference_bytes() {
    let rows = [
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
        (
            Prog::new(SYSCALL, Addr::None, Addr::None),
            "SYSCALL",
            "0f 05",
        ),
    ];
    for (prog, text, bytes) in rows {
        assert_eq!(dump(std::slice::from_ref(&prog)), format!("  {text}\n"));
        assert_eq!(hex(&encode_one(&prog).bytes), bytes, "{text}");
    }
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
