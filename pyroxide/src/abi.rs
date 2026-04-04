//! Mojo `@export` ABI documentation and helpers.
//!
//! # Type mapping
//!
//! | Mojo type | Rust type | Notes |
//! |-----------|-----------|-------|
//! | `Int` | `isize` | Pointer-width signed (NOT C `int`) |
//! | `Int8`..`Int64` | `i8`..`i64` | |
//! | `UInt8`..`UInt64` | `u8`..`u64` | |
//! | `Float32` / `Float64` | `f32` / `f64` | NaN, ±Inf, -0.0 preserved |
//! | `Bool` | `bool` | |
//! | void | `()` | |
//!
//! Types that **cannot** cross: `String`, `List`, `Tuple`, `SIMD`, `Float16`, structs.
//!
//! # The `raises` trap
//!
//! `@export` with `raises` compiles, but uncaught errors **segfault**.

/// Zero-cost helper for out-pointer return values from Mojo.
///
/// # Safety
///
/// The Mojo function **must** write a valid value to every out-pointer
/// before returning. If it doesn't, the result is undefined behavior.
pub struct OutParam;

impl OutParam {
    /// # Safety
    ///
    /// Closure must write a valid `T` to the provided address.
    #[inline]
    pub unsafe fn call1<T>(f: impl FnOnce(isize)) -> T {
        let mut val = std::mem::MaybeUninit::<T>::uninit();
        f(val.as_mut_ptr() as isize);
        // SAFETY: contract requires Mojo wrote to the pointer
        unsafe { val.assume_init() }
    }

    /// # Safety
    ///
    /// Closure must write valid values to both addresses.
    #[inline]
    pub unsafe fn call2<A, B>(f: impl FnOnce(isize, isize)) -> (A, B) {
        let mut a = std::mem::MaybeUninit::<A>::uninit();
        let mut b = std::mem::MaybeUninit::<B>::uninit();
        f(a.as_mut_ptr() as isize, b.as_mut_ptr() as isize);
        // SAFETY: contract requires Mojo wrote to both pointers
        unsafe { (a.assume_init(), b.assume_init()) }
    }

    /// # Safety
    ///
    /// Closure must write valid values to all three addresses.
    #[inline]
    pub unsafe fn call3<A, B, C>(f: impl FnOnce(isize, isize, isize)) -> (A, B, C) {
        let mut a = std::mem::MaybeUninit::<A>::uninit();
        let mut b = std::mem::MaybeUninit::<B>::uninit();
        let mut c = std::mem::MaybeUninit::<C>::uninit();
        f(
            a.as_mut_ptr() as isize,
            b.as_mut_ptr() as isize,
            c.as_mut_ptr() as isize,
        );
        // SAFETY: contract requires Mojo wrote to all three pointers
        unsafe { (a.assume_init(), b.assume_init(), c.assume_init()) }
    }
}
