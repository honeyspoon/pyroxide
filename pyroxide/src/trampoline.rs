//! Panic-safe trampoline for the Rust↔Mojo FFI boundary.
//!
//! A Rust panic unwinding across `extern "C"` is undefined behavior.
//! [`catch_mojo_call`] wraps a closure in `catch_unwind` to prevent this.
//!
//! # Example
//!
//! ```rust,ignore
//! use pyroxide::trampoline::catch_mojo_call;
//!
//! #[unsafe(no_mangle)]
//! extern "C" fn my_callback() -> f64 {
//!     catch_mojo_call(|| {
//!         // safe Rust code — panics won't cross FFI
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
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("unknown panic")
}

/// Wrap a closure so panics are caught at the FFI boundary.
///
/// On panic, prints the message to stderr and returns `T::default()`
/// (0 for numbers, false for bool, null for pointers).
#[inline]
pub fn catch_mojo_call<T: Default>(f: impl FnOnce() -> T) -> T {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(val) => val,
        Err(payload) => {
            eprintln!(
                "[pyroxide] panic at FFI boundary: {}",
                panic_message(&payload)
            );
            T::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_returns_value() {
        let result = catch_mojo_call(|| 42i32);
        assert_eq!(result, 42);
    }

    #[test]
    fn panic_returns_default() {
        let result: f64 = catch_mojo_call(|| {
            panic!("test panic");
        });
        assert_eq!(result, 0.0); // f64::default()
    }

    #[test]
    fn panic_returns_false_for_bool() {
        let result: bool = catch_mojo_call(|| {
            panic!("test");
        });
        assert!(!result);
    }
}
