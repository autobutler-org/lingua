//! The playground compiler as a `wasm32-unknown-unknown` dynamic library.
//!
//! The page copies UTF-8 source into [`lingua_alloc`], calls [`lingua_compile`],
//! and reads a little-endian `u32` length plus JSON bytes. [`lingua_free`]
//! releases that pointer. The JSON is [`lingua::playground::compile_json`] for
//! the module `factorial`. The native CLI, the ELF linker, and provider effects
//! are not part of this entry.

use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};

const MODULE: &str = "factorial";

/// Allocates `size` bytes. A zero size returns a dangling non-null pointer.
///
/// # Safety
///
/// Free the pointer once with [`lingua_dealloc`] and the same `size`.
#[must_use]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lingua_alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::without_provenance_mut(1);
    }
    let layout = layout_of(size);
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    ptr
}

/// Releases a pointer from [`lingua_alloc`].
///
/// # Safety
///
/// `ptr` came from [`lingua_alloc`] with this `size`, and this is the one free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lingua_dealloc(ptr: *mut u8, size: usize) {
    if size == 0 || ptr.is_null() {
        return;
    }
    unsafe { dealloc(ptr, layout_of(size)) }
}

/// Compiles the UTF-8 source at `ptr`.
///
/// The returned pointer's first four bytes are the JSON length, little-endian,
/// and the following bytes are the JSON object. Pass the pointer to [`lingua_free`].
///
/// # Safety
///
/// `ptr` is readable for `len` bytes. A `len` of 0 does not read `ptr`.
#[must_use]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lingua_compile(ptr: *const u8, len: usize) -> *mut u8 {
    let bytes = if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    };
    let json = if let Ok(source) = std::str::from_utf8(bytes) {
        lingua::playground::compile_json(MODULE, source)
    } else {
        let message = "source is not UTF-8\n".to_owned();
        lingua::playground::encode(&lingua::playground::Views {
            tokens: message.clone(),
            ast: message.clone(),
            tir: message.clone(),
            lir: message.clone(),
            assembly: message.clone(),
            run: message,
        })
    };
    leak_prefixed(json.as_bytes())
}

/// Releases a pointer from [`lingua_compile`].
///
/// # Safety
///
/// `ptr` came from [`lingua_compile`], and this is the one free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lingua_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let len = u32::from_le_bytes(ptr.cast::<[u8; 4]>().read()) as usize;
        dealloc(ptr, layout_of(4 + len));
    }
}

fn layout_of(size: usize) -> Layout {
    Layout::from_size_align(size, 1).unwrap_or_else(|_| handle_alloc_error(Layout::new::<u8>()))
}

fn leak_prefixed(payload: &[u8]) -> *mut u8 {
    let Ok(len) = u32::try_from(payload.len()) else {
        return leak_prefixed(b"{\"tokens\":\"output too large\\n\"}");
    };
    let total = 4 + len as usize;
    let layout = layout_of(total);
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    unsafe {
        ptr.cast::<[u8; 4]>().write(len.to_le_bytes());
        std::ptr::copy_nonoverlapping(payload.as_ptr(), ptr.add(4), len as usize);
    }
    ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_of(ptr: *mut u8) -> String {
        let len = unsafe { u32::from_le_bytes(ptr.cast::<[u8; 4]>().read()) } as usize;
        let bytes = unsafe { std::slice::from_raw_parts(ptr.add(4), len) };
        std::str::from_utf8(bytes)
            .expect("json is utf-8")
            .to_owned()
    }

    #[test]
    fn factorial_json_round_trips_through_the_c_abi() {
        let source = include_str!("../../tests/e2e/factorial.lf");
        let bytes = source.as_bytes();
        let ptr = unsafe { lingua_alloc(bytes.len()) };
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) };
        let out = unsafe { lingua_compile(ptr, bytes.len()) };
        unsafe { lingua_dealloc(ptr, bytes.len()) };
        let json = text_of(out);
        unsafe { lingua_free(out) };
        assert!(
            json.starts_with("{\"tokens\":\"1:1 Keyword(Fn)\\n"),
            "{json}"
        );
        assert!(json.contains("2432902008176640000\\n"));
        assert!(json.contains("TEXT factorial.factorial(SB)\\n"));
    }

    #[test]
    fn bytes_that_are_not_utf8_are_an_error_on_every_panel() {
        let ptr = unsafe { lingua_alloc(1) };
        unsafe { ptr.write(0xff) };
        let out = unsafe { lingua_compile(ptr, 1) };
        unsafe { lingua_dealloc(ptr, 1) };
        let json = text_of(out);
        unsafe { lingua_free(out) };
        assert!(json.contains("source is not UTF-8"), "{json}");
        assert_eq!(json.matches("source is not UTF-8").count(), 6);
    }
}
