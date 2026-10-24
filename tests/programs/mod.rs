//! Hand-built programs shared by the build and end-to-end tests. Each one is the `Prog` list
//! that a Lingua program will select to once `lingua build` lowers source.

use lingua::build::arch::amd64::{self, AX, DI, DX, LEAQ, MOVQ, SI, SYSCALL};
use lingua::build::link::elf::{self, RoData};
use lingua::build::prog::{Addr, Prog, TEXT};

pub const MESSAGE: &[u8] = b"hello, world\n";

// `fn main() -> Nothing pure; log("hello, world")` with `log` inlined into rt0:
// write(2, msg, len), then exit_group(0)
fn hello_progs() -> Vec<Prog> {
    let len = i64::try_from(MESSAGE.len()).unwrap();
    vec![
        Prog::new(TEXT, Addr::sym("_rt0_linux_amd64"), Addr::None),
        Prog::new(MOVQ, Addr::Imm(2), Addr::Reg(DI)),
        Prog::new(LEAQ, Addr::sym("hello"), Addr::Reg(SI)),
        Prog::new(MOVQ, Addr::Imm(len), Addr::Reg(DX)),
        Prog::new(MOVQ, Addr::Imm(1), Addr::Reg(AX)),
        Prog::new(SYSCALL, Addr::None, Addr::None),
        Prog::new(MOVQ, Addr::Imm(0), Addr::Reg(DI)),
        Prog::new(MOVQ, Addr::Imm(231), Addr::Reg(AX)),
        Prog::new(SYSCALL, Addr::None, Addr::None),
    ]
}

pub fn hello_image() -> Vec<u8> {
    let rt0 = amd64::encode(&hello_progs()).unwrap();
    let rodata = RoData {
        name: "hello".to_owned(),
        bytes: MESSAGE.to_vec(),
    };
    elf::link(&[rt0], &[rodata]).unwrap()
}
