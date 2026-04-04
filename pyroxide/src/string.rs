//! FFI-safe string type for passing text between Rust and Mojo.
//!
//! Mojo's `String` is not `#[repr(C)]`, so we use [`MojoStr`] — a
//! `(ptr, len)` pair. Rust owns the string data; Mojo borrows it
//! for the duration of the FFI call.

use std::marker::PhantomData;

/// A borrowed UTF-8 string slice for FFI. Layout: `(ptr, len)`.
///
/// Mojo receives the two fields and reads `ptr[0..len]`.
/// The data is valid only for the duration of the FFI call.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MojoStr<'a> {
    ptr: *const u8,
    len: usize,
    _marker: PhantomData<&'a str>,
}

impl<'a> MojoStr<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            ptr: s.as_ptr(),
            len: s.len(),
            _marker: PhantomData,
        }
    }

    pub fn ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Reconstruct a `&str` from a Mojo-returned `(ptr, len)`.
    ///
    /// # Safety
    ///
    /// The pointer must point to valid UTF-8 data of at least `len` bytes.
    pub unsafe fn as_str(&self) -> &'a str {
        // SAFETY: caller guarantees ptr is valid UTF-8 for len bytes (see doc above)
        unsafe {
            let bytes = std::slice::from_raw_parts(self.ptr, self.len);
            std::str::from_utf8_unchecked(bytes)
        }
    }
}

impl<'a> From<&'a str> for MojoStr<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}
