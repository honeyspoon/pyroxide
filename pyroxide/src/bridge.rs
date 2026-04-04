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
//! pointers valid only for one FFI call on the calling thread. Do not
//! attempt to share them across threads.

use std::marker::PhantomData;
use std::ptr::NonNull;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

// ── MojoAddr ──

/// A typed wrapper around a raw memory address for Mojo FFI.
///
/// Prevents accidentally passing a length, index, or other `isize`
/// where an address was expected. Zero-cost: `#[repr(transparent)]`
/// over `isize`.
///
/// Analogous to `ash::vk::Buffer` (Vulkan handle newtype) or
/// `windows::Win32::Foundation::HANDLE`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MojoAddr(isize);

impl MojoAddr {
    /// Create from a raw `isize` address.
    ///
    /// # Safety
    ///
    /// The address must point to a valid, live object of the expected
    /// type for the duration of the FFI call.
    #[inline]
    pub const unsafe fn from_raw(addr: isize) -> Self {
        Self(addr)
    }

    /// The raw `isize` value to pass to Mojo's `Int` parameter.
    #[inline]
    pub const fn as_raw(self) -> isize {
        self.0
    }

    /// Create from any raw pointer.
    #[inline]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self(ptr as isize)
    }

    /// Create from any mutable raw pointer.
    #[inline]
    pub fn from_mut_ptr<T>(ptr: *mut T) -> Self {
        Self(ptr as isize)
    }
}

impl From<MojoAddr> for isize {
    #[inline]
    fn from(addr: MojoAddr) -> Self {
        addr.0
    }
}

// ── Conversion traits ──

/// A type that can be passed to Mojo by pointer. Zero-copy.
///
/// Automatically implemented for all [`mojo_type!`](crate::mojo_type) structs.
pub trait IntoMojo: IntoBytes + Immutable + KnownLayout {
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
    fn as_mojo_mut(&mut self) -> MojoMut<'_, Self>
    where
        Self: Sized,
    {
        MojoMut::new(self)
    }

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
/// use-after-free. Returns [`MojoAddr`] from `.addr()` — a typed
/// wrapper that prevents mixing up addresses with other `isize` values.
///
/// `MojoRef` is `!Send` and `!Sync` — it must not leave the calling thread.
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

    /// The address to pass to Mojo's `Int` parameter.
    #[inline]
    pub fn addr(&self) -> MojoAddr {
        MojoAddr(self.ptr.as_ptr() as isize)
    }

    /// The underlying typed pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }
}

// ── MojoMut ──

/// Mutable, zero-copy reference. Mojo can read and write through this pointer.
///
/// Same lifetime guarantees as [`MojoRef`], but allows mutation.
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

    /// The address to pass to Mojo's `Int` parameter.
    #[inline]
    pub fn addr(&self) -> MojoAddr {
        MojoAddr(self.ptr.as_ptr() as isize)
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

// ── MojoSlice ──

/// Zero-copy handle to a contiguous slice of values.
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

    /// The address of the first element.
    #[inline]
    pub fn addr(&self) -> MojoAddr {
        MojoAddr(self.ptr.as_ptr() as isize)
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

/// Mutable zero-copy handle to a contiguous slice.
///
/// Use this when Mojo needs to write into a Rust-owned buffer.
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

    /// The address of the first element.
    #[inline]
    pub fn addr(&self) -> MojoAddr {
        MojoAddr(self.ptr.as_ptr() as isize)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
