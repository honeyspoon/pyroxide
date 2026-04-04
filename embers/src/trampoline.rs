//! FFI trampoline: catch panics at the Rust↔Mojo boundary.
//!
//! A Rust panic unwinding across an `extern "C"` boundary is instant
//! undefined behavior. This module provides [`catch_mojo_call`] which
//! wraps any closure in `catch_unwind`, converting panics into a safe
//! error return.
//!
//! Inspired by PyO3's trampoline layer (`impl_/trampoline.rs`).
//!
//! # Example
//!
//! ```rust,ignore
//! use embers::trampoline::catch_mojo_call;
//!
//! #[unsafe(no_mangle)]
//! extern "C" fn my_function(addr: isize) -> f64 {
//!     catch_mojo_call(|| {
//!         // safe Rust code here — panics won't cross FFI
//!         42.0
//!     })
//! }
//! ```

use std::any::Any;
use std::panic::{self, AssertUnwindSafe};

fn panic_message(payload: &Box<dyn Any + Send>) -> &str {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap_or("unknown panic")
}

/// Wrap a closure so that panics are caught at the FFI boundary.
///
/// If the closure panics, this prints the panic message to stderr
/// and returns the type's default value (0 for numbers, null for pointers).
#[inline]
pub fn catch_mojo_call<T: Default>(f: impl FnOnce() -> T) -> T {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(val) => val,
        Err(payload) => {
            eprintln!(
                "[embers] panic at FFI boundary: {}",
                panic_message(&payload)
            );
            T::default()
        }
    }
}

/// Like [`catch_mojo_call`] but returns a [`MojoResult`] instead of a default.
#[inline]
pub fn catch_mojo_result<T>(f: impl FnOnce() -> T) -> MojoResult<T> {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(val) => MojoResult::ok(val),
        Err(payload) => {
            eprintln!(
                "[embers] panic at FFI boundary: {}",
                panic_message(&payload)
            );
            MojoResult::err(MojoError::Panic)
        }
    }
}

/// Error codes for cross-FFI error reporting.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MojoError {
    Ok = 0,
    Panic = 1,
    InvalidArg = 2,
    ShapeMismatch = 3,
    OutOfMemory = 4,
    Other = 255,
}

/// A `#[repr(C)]` result type that can cross the FFI boundary.
///
/// Mojo reads the `error` field first. If it's `Ok` (0), the `value`
/// field contains the result.
///
/// # Safety
///
/// `T` should be a `#[repr(C)]` / zerocopy-compatible type for Mojo
/// to read the value safely. The struct itself is safe to construct
/// in Rust, but passing it to Mojo requires `T` to have a stable layout.
#[repr(C)]
#[derive(Debug)]
pub struct MojoResult<T> {
    pub error: MojoError,
    pub value: std::mem::MaybeUninit<T>,
}

impl<T> MojoResult<T> {
    pub fn ok(value: T) -> Self {
        Self {
            error: MojoError::Ok,
            value: std::mem::MaybeUninit::new(value),
        }
    }

    pub fn err(code: MojoError) -> Self {
        Self {
            error: code,
            value: std::mem::MaybeUninit::uninit(),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.error == MojoError::Ok
    }

    /// Extract the value.
    ///
    /// # Panics
    ///
    /// Panics if this is an error result. Do not call from `extern "C"`
    /// contexts — use [`catch_mojo_call`] instead.
    pub fn unwrap(self) -> T {
        assert!(self.is_ok(), "called unwrap on MojoResult::Err");
        unsafe { self.value.assume_init() }
    }
}
