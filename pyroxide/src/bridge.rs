//! Core traits and handle types for the Rust↔Mojo FFI boundary.
//!
//! # Ownership model
//!
//! **Rust always owns the data.** Mojo borrows it via pointer for the
//! duration of the FFI call.
//!
//! # Thread safety
//!
//! `MojoSlice` and `MojoSliceMut` are `!Send` and `!Sync` — they
//! represent borrowed pointers valid only on the calling thread.

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

// ── MojoSlice ──

/// Immutable slice handle: `(ptr, len)` pair for Mojo.
///
/// `MojoSlice` is `!Send` and `!Sync`.
pub struct MojoSlice<'a, T: IntoBytes + Immutable> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<&'a [T]>,
    _not_send: PhantomData<*const ()>,
}

impl<'a, T: IntoBytes + Immutable> MojoSlice<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            ptr: NonNull::from(slice).cast(),
            len: slice.len(),
            _marker: PhantomData,
            _not_send: PhantomData,
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
    _not_send: PhantomData<*const ()>,
}

impl<'a, T: IntoBytes + FromBytes> MojoSliceMut<'a, T> {
    #[inline]
    pub fn new(slice: &'a mut [T]) -> Self {
        Self {
            ptr: NonNull::from(&mut *slice).cast(),
            len: slice.len(),
            _marker: PhantomData,
            _not_send: PhantomData,
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

// ── OutSlot ──

/// A single out-parameter slot for Mojo to write into.
///
/// Follows the stdlib `MaybeUninit` pattern — composable, no numbered variants.
///
/// ```rust,ignore
/// let mut q = OutSlot::<i64>::uninit();
/// let mut r = OutSlot::<i64>::uninit();
/// unsafe { divmod(17, 5, q.as_raw(), r.as_raw()) };
/// let (q, r) = unsafe { (q.assume_init(), r.assume_init()) };
/// ```
pub struct OutSlot<T> {
    inner: std::mem::MaybeUninit<T>,
}

impl<T> OutSlot<T> {
    /// Create an uninitialized slot (mirrors `MaybeUninit::uninit()`).
    #[inline]
    pub fn uninit() -> Self {
        Self {
            inner: std::mem::MaybeUninit::uninit(),
        }
    }

    /// Address as `isize` for Mojo's `Int` parameter.
    #[inline]
    pub fn as_raw(&mut self) -> isize {
        self.inner.as_mut_ptr() as isize
    }

    /// Extract the value after Mojo has written to it.
    ///
    /// # Safety
    ///
    /// Mojo must have written a valid `T` to this slot's address.
    #[inline]
    pub unsafe fn assume_init(self) -> T {
        // SAFETY: caller guarantees Mojo wrote to the pointer
        unsafe { self.inner.assume_init() }
    }
}

impl<T> Default for OutSlot<T> {
    fn default() -> Self {
        Self::uninit()
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
    fn mojo_slice_ptr_and_len() {
        let data = [1.0f64, 2.0, 3.0];
        let s = MojoSlice::new(&data);
        assert_eq!(s.len(), 3);
        assert!(!s.is_empty());
        assert_eq!(s.size_bytes(), 24);
        assert_eq!(s.as_raw(), data.as_ptr() as isize);
        assert_eq!(s.len_isize(), 3);
    }

    #[test]
    fn mojo_slice_empty() {
        let empty: &[f64] = &[];
        let s = MojoSlice::new(empty);
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
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
        let bytes = [0u8; 3];
        assert!(TestPoint::from_mojo_bytes(&bytes).is_none());
    }

    #[test]
    fn primitives_implement_into_mojo() {
        let v: f64 = 3.14;
        assert_ne!(v.as_raw(), 0);
        let i: i32 = 42;
        assert_ne!(i.as_raw(), 0);
    }

    #[test]
    fn out_slot_basic() {
        let mut slot = OutSlot::<i64>::uninit();
        let addr = slot.as_raw();
        // Simulate Mojo writing to the slot
        unsafe { *(addr as *mut i64) = 42 };
        let val = unsafe { slot.assume_init() };
        assert_eq!(val, 42);
    }
}
