//! Panic-safe trampoline for the FFI boundary.
//!
//! [`catch_mojo_call`] catches **Rust panics** inside a closure, preventing
//! undefined behavior from stack unwinding across `extern "C"`.
//!
//! # What it catches
//!
//! - Rust panics (`panic!`, `unwrap()` failure, assertion failures)
//!
//! # What it does NOT catch
//!
//! - Mojo errors (those segfault if uncaught — see `abi` module docs)
//! - Hardware exceptions (segfaults, SIGBUS, etc.)
//! - Any error originating in Mojo code
//!
//! # When to use
//!
//! Use this when Rust code is called back from Mojo (e.g., function pointers),
//! or when wrapping any closure that might panic before an FFI call:
//!
//! ```rust,ignore
//! // Callback that Mojo will invoke:
//! #[unsafe(no_mangle)]
//! extern "C" fn my_callback() -> f64 {
//!     catch_mojo_call(|| {
//!         // If this panics, returns 0.0 instead of unwinding across FFI
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

/// Catch Rust panics at the FFI boundary.
///
/// Wraps a closure in [`std::panic::catch_unwind`]. On panic, prints
/// the message to stderr and returns `T::default()`.
///
/// **This does NOT catch Mojo errors or segfaults** — only Rust panics.
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
        assert_eq!(result, 0.0);
    }

    #[test]
    fn panic_returns_false_for_bool() {
        let result: bool = catch_mojo_call(|| {
            panic!("test");
        });
        assert!(!result);
    }
}
