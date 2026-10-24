//! ELF64 writer for a static linux/amd64 `ET_EXEC`: no `PT_INTERP`, nothing to relocate at load.
//!
//! File layout: ELF header, program headers, code (the text segment, R+X, mapped from file
//! offset 0 at the base), rodata on the next page (R), then `.shstrtab` and the section
//! headers, which are not loaded and exist for `objdump` and `readelf`.

use std::collections::HashMap;

use crate::build::arch::{Encoded, Reloc};
use crate::build::lir::RoData;

const BASE: u64 = 0x40_0000;
const PAGE: u64 = 0x1000;
const EHDR_SIZE: u64 = 64;
const PHDR_SIZE: u64 = 56;
const SHDR_SIZE: u64 = 64;
const PHNUM: u64 = 3;
const FUNCTION_ALIGN: u64 = 16;
const RODATA_ALIGN: u64 = 8;
const INT3: u8 = 0xcc;

const ET_EXEC: u16 = 2;
const EM_X86_64: u16 = 62;
const PT_LOAD: u32 = 1;
const PT_GNU_STACK: u32 = 0x6474_e551;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;
const SHT_PROGBITS: u32 = 1;
const SHT_STRTAB: u32 = 3;
const SHF_ALLOC: u64 = 2;
const SHF_EXECINSTR: u64 = 4;
const SHSTRTAB: &[u8] = b"\0.text\0.rodata\0.shstrtab\0";

fn align(value: u64, to: u64) -> u64 {
    value.next_multiple_of(to)
}

fn len(bytes: &[u8]) -> u64 {
    bytes.len() as u64
}

fn pad(image: &mut Vec<u8>, to: u64, fill: u8) {
    image.resize(usize::try_from(to).unwrap_or(usize::MAX), fill);
}

fn define<'a>(
    symbols: &mut HashMap<&'a str, u64>,
    name: &'a str,
    offset: u64,
) -> Result<(), String> {
    match symbols.insert(name, offset) {
        None => Ok(()),
        Some(_) => Err(format!("internal error: symbol `{name}` is defined twice")),
    }
}

/// File offsets of each part; a loaded part's address is `BASE` plus its offset.
struct Layout {
    entry: u64,
    text_start: u64,
    text_end: u64,
    rodata_start: u64,
    rodata_end: u64,
    shoff: u64,
}

impl Layout {
    // ELF header, then the program headers: text, rodata, stack
    fn headers(&self) -> Vec<u8> {
        let mut out = b"\x7fELF\x02\x01\x01".to_vec(); // ELFCLASS64, ELFDATA2LSB, EV_CURRENT
        out.resize(16, 0);
        out.extend(ET_EXEC.to_le_bytes());
        out.extend(EM_X86_64.to_le_bytes());
        out.extend(1u32.to_le_bytes()); // e_version
        for word in [BASE + self.entry, EHDR_SIZE, self.shoff] {
            out.extend(word.to_le_bytes()); // e_entry, e_phoff, e_shoff
        }
        out.extend(0u32.to_le_bytes()); // e_flags
        // e_ehsize, e_phentsize, e_phnum, e_shentsize, e_shnum, e_shstrndx
        for half in [EHDR_SIZE, PHDR_SIZE, PHNUM, SHDR_SIZE, 4, 3] {
            out.extend(u16::try_from(half).unwrap_or(u16::MAX).to_le_bytes());
        }
        let rodata_size = self.rodata_end - self.rodata_start;
        let segments = [
            (PT_LOAD, PF_R | PF_X, 0, BASE, self.text_end, PAGE),
            (
                PT_LOAD,
                PF_R,
                self.rodata_start,
                BASE + self.rodata_start,
                rodata_size,
                PAGE,
            ),
            (PT_GNU_STACK, PF_R | PF_W, 0, 0, 0, 16),
        ];
        for (kind, flags, offset, addr, size, align) in segments {
            out.extend(kind.to_le_bytes());
            out.extend(flags.to_le_bytes());
            for word in [offset, addr, addr, size, size, align] {
                out.extend(word.to_le_bytes()); // p_paddr = p_vaddr, p_memsz = p_filesz
            }
        }
        out
    }

    // null, .text, .rodata, .shstrtab
    fn section_headers(&self) -> Vec<u8> {
        let text = (self.text_start, self.text_end - self.text_start);
        let rodata = (self.rodata_start, self.rodata_end - self.rodata_start);
        let sections = [
            (0, 0, 0, (0, 0), 0),
            (
                1,
                SHT_PROGBITS,
                SHF_ALLOC | SHF_EXECINSTR,
                text,
                FUNCTION_ALIGN,
            ),
            (7, SHT_PROGBITS, SHF_ALLOC, rodata, RODATA_ALIGN),
            (15, SHT_STRTAB, 0, (self.rodata_end, len(SHSTRTAB)), 1),
        ];
        let mut out = Vec::new();
        for (name, kind, flags, (offset, size), align) in sections {
            let addr = if flags & SHF_ALLOC == 0 {
                0
            } else {
                BASE + offset
            };
            out.extend(u32::to_le_bytes(name));
            out.extend(u32::to_le_bytes(kind));
            for word in [flags, addr, offset, size, 0, align, 0] {
                // sh_link and sh_info share the zero word; sh_entsize is the last
                out.extend(word.to_le_bytes());
            }
        }
        out
    }
}

// Patches each relocation in a copy of `function`'s bytes placed at file offset `at`
fn relocate(function: &Encoded, at: u64, symbols: &HashMap<&str, u64>) -> Result<Vec<u8>, String> {
    let mut bytes = function.bytes.clone();
    for Reloc::Pcrel32 {
        at: field,
        sym,
        off,
    } in &function.relocs
    {
        let target = symbols.get(sym.as_str()).ok_or_else(|| {
            format!(
                "internal error: `{}` refers to undefined symbol `{sym}`",
                function.name
            )
        })?;
        let end = at + len(&bytes[..*field]) + 4;
        let disp = i128::from(*target) + i128::from(*off) - i128::from(end);
        let disp = i32::try_from(disp).map_err(|_| {
            format!(
                "internal error: `{}` is out of 32-bit range of `{sym}`",
                function.name
            )
        })?;
        bytes[*field..*field + 4].copy_from_slice(&disp.to_le_bytes());
    }
    Ok(bytes)
}

/// Lays out `functions` (the first is rt0, the entry) and `rodata`, resolves every
/// relocation, and returns the file image.
///
/// # Errors
/// An internal error for no functions, an undefined or duplicate symbol, or a displacement
/// that does not fit its field.
pub fn link(functions: &[Encoded], rodata: &[RoData]) -> Result<Vec<u8>, String> {
    let mut symbols: HashMap<&str, u64> = HashMap::new();
    let text_start = align(EHDR_SIZE + PHNUM * PHDR_SIZE, FUNCTION_ALIGN);
    let mut function_offsets = Vec::new();
    let mut at = text_start;
    for function in functions {
        at = align(at, FUNCTION_ALIGN);
        define(&mut symbols, &function.name, at)?;
        function_offsets.push(at);
        at += len(&function.bytes);
    }
    let text_end = at;
    let rodata_start = align(text_end, PAGE);
    let mut rodata_offsets = Vec::new();
    at = rodata_start;
    for block in rodata {
        at = align(at, RODATA_ALIGN);
        define(&mut symbols, &block.name, at)?;
        rodata_offsets.push(at);
        at += len(&block.bytes);
    }
    let layout = Layout {
        entry: *function_offsets
            .first()
            .ok_or("internal error: nothing to link; rt0 must come first")?,
        text_start,
        text_end,
        rodata_start,
        rodata_end: at,
        shoff: align(at + len(SHSTRTAB), 8),
    };

    let mut image = layout.headers();
    for (function, &offset) in functions.iter().zip(&function_offsets) {
        pad(
            &mut image,
            offset,
            if offset == text_start { 0 } else { INT3 },
        );
        image.extend(relocate(function, offset, &symbols)?);
    }
    for (block, &offset) in rodata.iter().zip(&rodata_offsets) {
        pad(&mut image, offset, 0);
        image.extend(&block.bytes);
    }
    pad(&mut image, layout.rodata_end, 0);
    image.extend(SHSTRTAB);
    pad(&mut image, layout.shoff, 0);
    image.extend(layout.section_headers());
    Ok(image)
}

/// Writes `image` to `path` with mode `0755`.
///
/// # Errors
/// The I/O error from writing the file or setting its mode.
pub fn write_executable(path: &std::path::Path, image: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, image)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn function(name: &str, sym: &str) -> Encoded {
        Encoded {
            name: name.to_owned(),
            bytes: vec![0x48, 0x8d, 0x35, 0, 0, 0, 0],
            relocs: vec![Reloc::Pcrel32 {
                at: 3,
                sym: sym.to_owned(),
                off: 0,
            }],
        }
    }

    #[test]
    fn an_undefined_symbol_is_an_internal_error() {
        assert_eq!(
            link(&[function("rt0", "missing")], &[]),
            Err("internal error: `rt0` refers to undefined symbol `missing`".to_owned())
        );
    }

    #[test]
    fn a_duplicate_symbol_is_an_internal_error() {
        let block = RoData {
            name: "rt0".to_owned(),
            bytes: vec![1],
        };
        assert_eq!(
            link(&[function("rt0", "rt0")], &[block]),
            Err("internal error: symbol `rt0` is defined twice".to_owned())
        );
    }

    #[test]
    fn a_pcrel32_is_measured_from_the_end_of_its_field() {
        let image = link(&[function("rt0", "rt0")], &[]).unwrap();
        // rt0 refers to its own start: -(3 + 4)
        let at = usize::try_from(align(EHDR_SIZE + PHNUM * PHDR_SIZE, FUNCTION_ALIGN)).unwrap() + 3;
        assert_eq!(image[at..at + 4], (-7i32).to_le_bytes());
    }
}
