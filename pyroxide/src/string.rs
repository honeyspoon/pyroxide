//! FFI-safe string type for passing text between Rust and Mojo.
//!
//! Mojo's `String` is not `#[repr(C)]`, so we pass `(ptr, len)` as
//! two separate `Int` parameters. [`MojoStr`] wraps a `&str` and
//! provides `.as_raw()` for the pointer and `.len_isize()` for the length.

use std::marker::PhantomData;

/// A borrowed UTF-8 string for FFI. Pass `.as_raw()` and `.len_isize()`
/// as two separate `Int` parameters to Mojo.
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

    /// Pointer address as `isize` for Mojo's `Int` parameter.
    #[inline]
    pub fn as_raw(&self) -> isize {
        self.ptr as isize
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Length as `isize` for Mojo's `Int` parameter.
    #[inline]
    pub fn len_isize(&self) -> isize {
        self.len as isize
    }

    /// Reconstruct a `&str` from Mojo-returned `(ptr, len)`.
    ///
    /// # Safety
    ///
    /// The pointer must point to valid UTF-8 data of at least `len` bytes.
    pub unsafe fn as_str(&self) -> &'a str {
        // SAFETY: caller guarantees ptr is valid UTF-8 for len bytes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_str() {
        let s = MojoStr::new("hello");
        assert_eq!(s.len(), 5);
        assert!(!s.is_empty());
        assert_eq!(s.len_isize(), 5);
        assert_ne!(s.as_raw(), 0);
    }

    #[test]
    fn empty_str() {
        let s = MojoStr::new("");
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }

    #[test]
    fn from_trait() {
        let s: MojoStr<'_> = "world".into();
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn as_raw_points_to_data() {
        let text = "test";
        let s = MojoStr::new(text);
        assert_eq!(s.as_raw(), text.as_ptr() as isize);
    }

    #[test]
    fn roundtrip_as_str() {
        let text = "hello world";
        let s = MojoStr::new(text);
        let recovered = unsafe { s.as_str() };
        assert_eq!(recovered, text);
    }
}
