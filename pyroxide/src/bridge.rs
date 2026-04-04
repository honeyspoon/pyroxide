//! Core traits and handle types for the Rust↔Mojo FFI boundary.
//!
//! # Ownership model
//!
//! **Rust always owns the data.** Mojo borrows it via pointer for the
//! duration of the FFI call.
//!
//! 1. [`MojoRef`] borrows `&T` — Mojo reads through the pointer
//! 2. [`MojoMut`] borrows `&mut T` — Mojo reads and writes
//! 3. Mojo must not store the pointer beyond the call
//! 4. Rust must not move or drop the value while Mojo holds a pointer
//!
//! # Thread safety
//!
//! All handle types are `!Send` and `!Sync`. They represent borrowed
//! pointers valid only for one FFI call on the calling thread.

use std::marker::PhantomData;
use std::ptr::NonNull;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

// ── Conversion traits ──

/// A type that can be passed to Mojo by pointer. Zero-copy.
///
/// Automatically implemented for all [`mojo_type!`](crate::mojo_type) structs.
/// Call `.as_raw()` to get the `isize` address Mojo expects.
pub trait IntoMojo: IntoBytes + Immutable + KnownLayout {
    /// Get the address as `isize` — pass this to Mojo's `Int` parameter.
    fn as_raw(&self) -> isize
    where
        Self: Sized,
    {
        std::ptr::from_ref(self) as isize
    }

    /// Get a lifetime-bound immutable handle.
    fn as_mojo(&self) -> MojoRef<'_, Self>
    where
        Self: Sized,
    {
        MojoRef::new(self)
    }
}

/// A type that can receive data written by Mojo. Zero-copy.
///
/// Automatically implemented for all [`mojo_type!`](crate::mojo_type) structs.
pub trait FromMojo: FromBytes + IntoBytes + Immutable + KnownLayout {
    /// Get the mutable address as `isize`.
    fn as_raw_mut(&mut self) -> isize
    where
        Self: Sized,
    {
        std::ptr::from_mut(self) as isize
    }

    /// Get a lifetime-bound mutable handle.
    fn as_mojo_mut(&mut self) -> MojoMut<'_, Self>
    where
        Self: Sized,
    {
        MojoMut::new(self)
    }

    /// Reinterpret a byte buffer as this type. Zero-copy.
    fn from_mojo_bytes(bytes: &[u8]) -> Option<&Self>
    where
        Self: Sized,
    {
        Self::ref_from_bytes(bytes).ok()
    }
}

impl<T: IntoBytes + Immutable + KnownLayout> IntoMojo for T {}
impl<T: FromBytes + IntoBytes + Immutable + KnownLayout> FromMojo for T {}

// ── MojoRef ──

/// Immutable, lifetime-bound pointer to Rust data for Mojo.
///
/// `MojoRef` is `!Send` and `!Sync`.
pub struct MojoRef<'a, T: IntoBytes + Immutable> {
    ptr: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IntoBytes + Immutable> MojoRef<'a, T> {
    #[inline]
    pub fn new(val: &'a T) -> Self {
        Self {
            ptr: NonNull::from(val),
            _marker: PhantomData,
        }
    }

    /// Address as `isize` for Mojo's `Int` parameter.
    #[inline]
    pub fn as_raw(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }
}

// ── MojoMut ──

/// Mutable, lifetime-bound pointer. Mojo can read and write.
///
/// `MojoMut` is `!Send` and `!Sync`.
pub struct MojoMut<'a, T: IntoBytes + FromBytes> {
    ptr: NonNull<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: IntoBytes + FromBytes> MojoMut<'a, T> {
    #[inline]
    pub fn new(val: &'a mut T) -> Self {
        Self {
            ptr: NonNull::from(&mut *val),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

// ── MojoSlice ──

/// Immutable slice handle: `(ptr, len)` pair for Mojo.
///
/// `MojoSlice` is `!Send` and `!Sync`.
pub struct MojoSlice<'a, T: IntoBytes + Immutable> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T: IntoBytes + Immutable> MojoSlice<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            ptr: NonNull::from(slice).cast(),
            len: slice.len(),
            _marker: PhantomData,
        }
    }

    /// Address of first element as `isize`.
    #[inline]
    pub fn as_raw(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Length as `isize` for Mojo's `Int` parameter.
    #[inline]
    pub fn len_isize(&self) -> isize {
        self.len as isize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn size_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
}

// ── MojoSliceMut ──

/// Mutable slice handle: `(ptr, len)` pair for Mojo to write into.
///
/// `MojoSliceMut` is `!Send` and `!Sync`.
pub struct MojoSliceMut<'a, T: IntoBytes + FromBytes> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<&'a mut [T]>,
}

impl<'a, T: IntoBytes + FromBytes> MojoSliceMut<'a, T> {
    #[inline]
    pub fn new(slice: &'a mut [T]) -> Self {
        Self {
            ptr: NonNull::from(&mut *slice).cast(),
            len: slice.len(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Length as `isize` for Mojo's `Int` parameter.
    #[inline]
    pub fn len_isize(&self) -> isize {
        self.len as isize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn size_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        zerocopy::IntoBytes,
        zerocopy::FromBytes,
        zerocopy::Immutable,
        zerocopy::KnownLayout,
    )]
    struct TestPoint {
        x: f64,
        y: f64,
    }

    #[test]
    fn as_raw_returns_valid_address() {
        let p = TestPoint { x: 1.0, y: 2.0 };
        let addr = p.as_raw();
        assert_ne!(addr, 0);
        // Round-trip: reconstruct reference from address
        let ptr = addr as *const TestPoint;
        let recovered = unsafe { &*ptr };
        assert_eq!(recovered.x, 1.0);
        assert_eq!(recovered.y, 2.0);
    }

    #[test]
    fn as_raw_mut_allows_write() {
        let mut val = 42.0f64;
        let addr = val.as_raw_mut();
        unsafe { *(addr as *mut f64) = 99.0 };
        assert_eq!(val, 99.0);
    }

    #[test]
    fn mojo_ref_preserves_address() {
        let p = TestPoint { x: 3.0, y: 4.0 };
        let r = p.as_mojo();
        assert_eq!(r.as_raw(), &p as *const TestPoint as isize);
        assert_eq!(r.as_ptr(), &p as *const TestPoint);
    }

    #[test]
    fn mojo_mut_preserves_address() {
        let mut p = TestPoint { x: 1.0, y: 2.0 };
        let addr = &mut p as *mut TestPoint as isize;
        let m = p.as_mojo_mut();
        assert_eq!(m.as_raw(), addr);
    }

    #[test]
    fn mojo_slice_ptr_and_len() {
        let data = [1.0f64, 2.0, 3.0];
        let s = MojoSlice::new(&data);
        assert_eq!(s.len(), 3);
        assert!(!s.is_empty());
        assert_eq!(s.size_bytes(), 24); // 3 * 8
        assert_eq!(s.as_raw(), data.as_ptr() as isize);
    }

    #[test]
    fn mojo_slice_empty() {
        let empty: &[f64] = &[];
        let s = MojoSlice::new(empty);
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
        assert_eq!(s.size_bytes(), 0);
    }

    #[test]
    fn mojo_slice_mut_ptr_and_len() {
        let mut data = [1.0f64, 2.0, 3.0];
        let s = MojoSliceMut::new(&mut data);
        assert_eq!(s.len(), 3);
        assert_eq!(s.size_bytes(), 24);
    }

    #[test]
    fn from_mojo_bytes_roundtrip() {
        let p = TestPoint { x: 1.5, y: 2.5 };
        let bytes = zerocopy::IntoBytes::as_bytes(&p);
        let recovered = TestPoint::from_mojo_bytes(bytes).expect("roundtrip failed");
        assert_eq!(*recovered, p);
    }

    #[test]
    fn from_mojo_bytes_wrong_size() {
        let bytes = [0u8; 3]; // too small for TestPoint (16 bytes)
        assert!(TestPoint::from_mojo_bytes(&bytes).is_none());
    }

    #[test]
    fn primitives_implement_into_mojo() {
        // Verify blanket impl works for primitive types
        let v: f64 = 3.14;
        let addr = v.as_raw();
        assert_ne!(addr, 0);

        let i: i32 = 42;
        let addr2 = i.as_raw();
        assert_ne!(addr2, 0);
    }
}
