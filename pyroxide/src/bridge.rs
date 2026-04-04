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

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn size_bytes(&self) -> usize {
        self.len * std::mem::size_of::<T>()
    }
}
