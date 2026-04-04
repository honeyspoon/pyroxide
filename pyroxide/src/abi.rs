//! Mojo `@export` ABI documentation and type mapping.
//!
//! This module documents the empirically verified ABI between Mojo's
//! `@export` functions and Rust's `unsafe extern "C"` declarations.
//!
//! # Type mapping
//!
//! | Mojo type | Rust type | Notes |
//! |-----------|-----------|-------|
//! | `Int` | `isize` | Pointer-width signed (NOT C `int`) |
//! | `Int8` | `i8` | |
//! | `Int16` | `i16` | |
//! | `Int32` | `i32` | |
//! | `Int64` | `i64` | |
//! | `UInt8` | `u8` | |
//! | `UInt16` | `u16` | |
//! | `UInt32` | `u32` | |
//! | `UInt64` | `u64` | |
//! | `Float32` | `f32` | |
//! | `Float64` | `f64` | NaN, ±Inf, -0.0 all preserved |
//! | `Bool` | `bool` | 1 byte, true/false |
//! | `UnsafePointer[T]` | `*mut T` | Same as `isize` in practice |
//! | void (no return) | `()` | |
//!
//! # Types that CANNOT cross the boundary
//!
//! | Mojo type | Why | Workaround |
//! |-----------|-----|------------|
//! | `String` | Mojo-internal heap layout | Pass `(ptr: Int, len: Int)` |
//! | `List[T]` | Mojo-internal heap type | Pass `(ptr: Int, len: Int)` |
//! | `Tuple` | No stable C ABI | Use out-pointers |
//! | `SIMD[_, N]` | Register type, not C-ABI | Use inside function only |
//! | `Float16` | Hardware `_Float16`, no stable Rust mapping | Avoid in signatures |
//! | Mojo structs | No `#[repr(C)]` guarantee | Pass pointer as `Int` |
//!
//! # The `raises` trap
//!
//! Mojo `@export` functions **can** be declared with `raises`, and they
//! compile fine. However, if an error actually escapes the function,
//! **the process segfaults** (exit code 139). There is no way for Rust
//! to catch this.
//!
//! Always catch errors inside the `@export` function:
//!
//! ```mojo
//! @export
//! def safe_sqrt(x: Float64) -> Float64:
//!     try:
//!         return _sqrt_impl(x)  # this can raise
//!     except:
//!         return -1.0           # return sentinel on error
//! ```
//!
//! # Multiple return values
//!
//! Mojo can't return tuples across FFI. Use out-pointers instead:
//!
//! ```mojo
//! @export
//! def divmod(a: Int, b: Int, quot_ptr: Int, rem_ptr: Int):
//!     var q = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=quot_ptr)
//!     var r = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=rem_ptr)
//!     q[0] = Int64(a // b)
//!     r[0] = Int64(a % b)
//! ```
//!
//! On the Rust side, use [`OutParam`]:
//!
//! ```rust,ignore
//! unsafe extern "C" { fn divmod(a: isize, b: isize, quot: isize, rem: isize); }
//!
//! let (q, r) = OutParam::call2(|q, r| unsafe { divmod(17, 5, q, r) });
//! assert_eq!((q, r), (3i64, 2i64));
//! ```

/// Zero-cost helper for Mojo functions that write results via out-pointers.
///
/// Uses `MaybeUninit` internally — no wasted default initialization.
/// The closure and pointer casts are all `#[inline]`, so this compiles
/// to the same code as manual pointer juggling.
///
/// ```rust,ignore
/// // Before: 4 lines of boilerplate
/// let mut q: i64 = 0;
/// let mut r: i64 = 0;
/// unsafe { divmod(17, 5, &mut q as *mut i64 as isize, &mut r as *mut i64 as isize) };
///
/// // After: 1 line, zero overhead
/// let (q, r): (i64, i64) = OutParam::call2(|q, r| unsafe { divmod(17, 5, q, r) });
/// ```
pub struct OutParam;

impl OutParam {
    /// Call a Mojo function that writes one result via out-pointer.
    #[inline]
    pub fn call1<T>(f: impl FnOnce(isize)) -> T {
        let mut val = std::mem::MaybeUninit::<T>::uninit();
        f(val.as_mut_ptr() as isize);
        // SAFETY: Mojo wrote to the pointer in f()
        unsafe { val.assume_init() }
    }

    /// Call a Mojo function that writes two results via out-pointers.
    #[inline]
    pub fn call2<A, B>(f: impl FnOnce(isize, isize)) -> (A, B) {
        let mut a = std::mem::MaybeUninit::<A>::uninit();
        let mut b = std::mem::MaybeUninit::<B>::uninit();
        f(a.as_mut_ptr() as isize, b.as_mut_ptr() as isize);
        // SAFETY: Mojo wrote to both pointers in f()
        unsafe { (a.assume_init(), b.assume_init()) }
    }

    /// Call a Mojo function that writes three results via out-pointers.
    #[inline]
    pub fn call3<A, B, C>(f: impl FnOnce(isize, isize, isize)) -> (A, B, C) {
        let mut a = std::mem::MaybeUninit::<A>::uninit();
        let mut b = std::mem::MaybeUninit::<B>::uninit();
        let mut c = std::mem::MaybeUninit::<C>::uninit();
        f(
            a.as_mut_ptr() as isize,
            b.as_mut_ptr() as isize,
            c.as_mut_ptr() as isize,
        );
        // SAFETY: Mojo wrote to all three pointers in f()
        unsafe { (a.assume_init(), b.assume_init(), c.assume_init()) }
    }
}

/// Trait for types that can be passed as scalar arguments to Mojo.
///
/// This maps Rust types to their Mojo ABI equivalent. All implementations
/// convert to `isize` (Mojo's `Int`) for uniform FFI calling.
///
/// Implemented for: all integer types, `f32`, `f64`, `bool`,
/// `*const T`, `*mut T`, and pyroxide handle types.
pub trait MojoArg {
    /// Convert to the ABI representation for passing to Mojo.
    fn to_mojo_arg(&self) -> isize;
}

// Pointer-width integers pass through
impl MojoArg for isize {
    fn to_mojo_arg(&self) -> isize {
        *self
    }
}

// Other integers widen to isize
impl MojoArg for i32 {
    fn to_mojo_arg(&self) -> isize {
        *self as isize
    }
}
impl MojoArg for i64 {
    fn to_mojo_arg(&self) -> isize {
        *self as isize
    }
}
impl MojoArg for usize {
    fn to_mojo_arg(&self) -> isize {
        *self as isize
    }
}

// Bool
impl MojoArg for bool {
    fn to_mojo_arg(&self) -> isize {
        isize::from(*self)
    }
}

// Raw pointers
impl<T> MojoArg for *const T {
    fn to_mojo_arg(&self) -> isize {
        *self as isize
    }
}
impl<T> MojoArg for *mut T {
    fn to_mojo_arg(&self) -> isize {
        *self as isize
    }
}
