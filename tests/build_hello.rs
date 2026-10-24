//! Hand-builds the `Prog` list that `fn main() -> Nothing pure; log("hello, world")` will
//! select to, links it into a static linux/amd64 ELF, and checks the file.
//!
//! `cargo test --test build_hello` leaves the executable at `target/tmp/hello`.

use std::path::PathBuf;

use lingua::build::arch::amd64::{self, AX, DI, DX, LEAQ, MOVQ, SI, SYSCALL};
use lingua::build::link::elf::{self, RoData};
use lingua::build::prog::{Addr, Prog, TEXT};

const MESSAGE: &[u8] = b"hello, world\n";
const BASE: u64 = 0x40_0000;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_GNU_STACK: u32 = 0x6474_e551;

// rt0 with `log` inlined: write(2, msg, len), then exit_group(0)
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

fn hello_image() -> Vec<u8> {
    let rt0 = amd64::encode(&hello_progs()).unwrap();
    let rodata = RoData {
        name: "hello".to_owned(),
        bytes: MESSAGE.to_vec(),
    };
    elf::link(&[rt0], &[rodata]).unwrap()
}

fn write_hello(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    elf::write_executable(&path, &hello_image()).unwrap();
    path
}

fn u16_at(image: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(image[at..at + 2].try_into().unwrap())
}

fn u32_at(image: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(image[at..at + 4].try_into().unwrap())
}

fn u64_at(image: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(image[at..at + 8].try_into().unwrap())
}

struct Phdr {
    kind: u32,
    flags: u32,
    offset: u64,
    vaddr: u64,
    filesz: u64,
    memsz: u64,
}

fn phdrs(image: &[u8]) -> Vec<Phdr> {
    let phoff = usize::try_from(u64_at(image, 0x20)).unwrap();
    assert_eq!(u16_at(image, 0x36), 56, "e_phentsize");
    (0..usize::from(u16_at(image, 0x38)))
        .map(|i| {
            let at = phoff + 56 * i;
            Phdr {
                kind: u32_at(image, at),
                flags: u32_at(image, at + 4),
                offset: u64_at(image, at + 8),
                vaddr: u64_at(image, at + 16),
                filesz: u64_at(image, at + 32),
                memsz: u64_at(image, at + 40),
            }
        })
        .collect()
}

#[test]
fn hello_is_a_static_amd64_exec() {
    let path = write_hello("hello");
    let image = std::fs::read(&path).unwrap();

    assert_eq!(&image[..4], b"\x7fELF");
    assert_eq!(image[4], 2, "ELFCLASS64");
    assert_eq!(image[5], 1, "ELFDATA2LSB");
    assert_eq!(u16_at(&image, 0x10), 2, "ET_EXEC");
    assert_eq!(u16_at(&image, 0x12), 62, "EM_X86_64");

    let phdrs = phdrs(&image);
    let kinds: Vec<u32> = phdrs.iter().map(|phdr| phdr.kind).collect();
    assert_eq!(kinds, [PT_LOAD, PT_LOAD, PT_GNU_STACK]);
    assert!(!kinds.contains(&PT_INTERP) && !kinds.contains(&PT_DYNAMIC));

    let (text, rodata, stack) = (&phdrs[0], &phdrs[1], &phdrs[2]);
    assert_eq!(
        (text.flags, text.offset, text.vaddr),
        (5, 0, BASE),
        "text R+X at the base"
    );
    assert_eq!(rodata.flags, 4, "rodata R");
    assert_eq!(stack.flags, 6, "stack R+W, not executable");
    for load in [text, rodata] {
        assert_eq!(load.vaddr % 0x1000, load.offset % 0x1000);
        assert_eq!(load.filesz, load.memsz);
    }
    assert!(text.offset + text.filesz <= rodata.offset);

    let entry = u64_at(&image, 0x18);
    assert!((text.vaddr..text.vaddr + text.filesz).contains(&entry));
    assert_eq!(entry % 16, 0, "functions are 16-byte aligned");

    // The LEAQ after `MOVQ $2, DI` (7 bytes) must point at the message in rodata
    let to_file =
        |vaddr: u64, phdr: &Phdr| usize::try_from(vaddr - phdr.vaddr + phdr.offset).unwrap();
    let lea = to_file(entry + 7, text);
    assert_eq!(image[lea..lea + 3], [0x48, 0x8d, 0x35]);
    let disp = i32::from_le_bytes(image[lea + 3..lea + 7].try_into().unwrap());
    let target = (entry + 14).checked_add_signed(i64::from(disp)).unwrap();
    assert!((rodata.vaddr..rodata.vaddr + rodata.filesz).contains(&target));
    let message = to_file(target, rodata);
    assert_eq!(&image[message..message + MESSAGE.len()], MESSAGE);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o755);
    }
}

#[test]
fn linking_is_reproducible() {
    assert_eq!(hello_image(), hello_image());
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn hello_runs() {
    let output = std::process::Command::new(write_hello("hello-run"))
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&output.stderr), "hello, world\n");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert_eq!(output.status.code(), Some(0));
}
