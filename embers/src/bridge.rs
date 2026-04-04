//! Core traits and handle types for the Rust↔Mojo FFI boundary.
//!
//! # Conversion traits
//!
//! | Trait | Direction | PyO3 equivalent |
//! |-------|-----------|-----------------|
//! | [`IntoMojo`] | Rust → Mojo | `IntoPyObject` |
//! | [`FromMojo`] | Mojo → Rust | `FromPyObject` |
//!
//! Both are automatically implemented for any type with the right
//! zerocopy derives (which [`mojo_type!`](crate::mojo_type) adds).
//!
//! # Handle types
//!
//! | Handle | Access | PyO3 equivalent |
//! |--------|--------|-----------------|
//! | [`MojoRef`] | Immutable borrow | `Bound<'py, T>` |
//! | [`MojoMut`] | Mutable borrow | `PyRefMut<T>` |
//! | [`MojoSlice`] | Immutable slice | buffer protocol |
//!
//! # Ownership model
//!
//! **Rust always owns the data.** Mojo borrows it via pointer for the
//! duration of the FFI call. This is the opposite of Python/PyO3 where
//! Python owns objects and Rust borrows via GIL-locked references.
//!
//! Rules:
//! 1. `MojoRef<'a, T>` borrows `&'a T` — Mojo reads through the pointer
//! 2. `MojoMut<'a, T>` borrows `&'a mut T` — Mojo reads and writes
//! 3. Mojo must not store the pointer beyond the call
//! 4. Rust must not move or drop the value while Mojo holds a pointer

use std::marker::PhantomData;
use std::ptr::NonNull;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// A type that can be passed to Mojo by pointer. Zero-copy.
///
/// Automatically implemented for all [`mojo_type!`](crate::mojo_type) structs
/// and all zerocopy-compatible types.
pub trait IntoMojo: IntoBytes + Immutable + KnownLayout {
    /// Get an immutable handle for passing to Mojo.
    fn as_mojo(&self) -> MojoRef<'_, Self>
    where
        Self: Sized,
    {
        MojoRef::new(self)
    }
}

/// A type that can receive data written by Mojo. Zero-copy.
///
/// Automatically implemented for all [`mojo_type!`](crate::mojo_type) structs
/// and all zerocopy-compatible types.
pub trait FromMojo: FromBytes + IntoBytes + Immutable + KnownLayout {
    /// Get a mutable handle for Mojo to write into.
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

/// Immutable, zero-copy reference for passing Rust data to Mojo.
///
/// The lifetime `'a` ties the pointer to the Rust borrow, preventing
/// use-after-free. Internally stores `NonNull<T>` — the `isize`
/// conversion happens only at [`.addr()`](MojoRef::addr).
///
/// # Example
///
/// ```rust,ignore
/// let point = Point::new(1.0, 2.0);
/// let handle = point.as_mojo();    // MojoRef<'_, Point>
/// unsafe { mojo_fn(handle.addr()) };
/// // `handle` cannot outlive `point` — compiler enforces this
/// ```
pub struct MojoRef<'a, T: IntoBytes + Immutable> {
    ptr: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IntoBytes + Immutable> MojoRef<'a, T> {
    #[inline(always)]
    pub fn new(val: &'a T) -> Self {
        Self {
            ptr: NonNull::from(val),
            _marker: PhantomData,
        }
    }

    /// The raw address as `isize` — pass this to Mojo's `Int` parameter.
    #[inline(always)]
    pub fn addr(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    /// The underlying typed pointer.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }
}

// ── MojoMut ──

/// Mutable, zero-copy reference. Mojo can read and write through this pointer.
///
/// Same lifetime guarantees as [`MojoRef`], but allows mutation.
pub struct MojoMut<'a, T: IntoBytes + FromBytes> {
    ptr: NonNull<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: IntoBytes + FromBytes> MojoMut<'a, T> {
    #[inline(always)]
    pub fn new(val: &'a mut T) -> Self {
        Self {
            ptr: NonNull::from(&mut *val),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn addr(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

// ── MojoSlice ──

/// Zero-copy handle to a contiguous slice of values.
///
/// Provides both the data pointer and the length for Mojo to iterate.
pub struct MojoSlice<'a, T: IntoBytes + Immutable> {
    ptr: NonNull<T>,
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T: IntoBytes + Immutable> MojoSlice<'a, T> {
    #[inline(always)]
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            ptr: NonNull::from(slice).cast(),
            len: slice.len(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn addr(&self) -> isize {
        self.ptr.as_ptr() as isize
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub fn size_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
}
