//! Links the hand-built hello program into a static linux/amd64 ELF and checks the file.
//! Running it is the end-to-end harness's job (`tests/e2e.rs`).
//!
//! `cargo test --test build_hello` leaves the executable at `target/tmp/hello`.

mod programs;

use std::path::PathBuf;

use lingua::build::link::elf;
use programs::{MESSAGE, hello_image};

const BASE: u64 = 0x40_0000;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_GNU_STACK: u32 = 0x6474_e551;

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
