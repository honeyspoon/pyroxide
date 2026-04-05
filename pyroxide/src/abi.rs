//! Mojo `@export` ABI documentation.
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
//!
//! # Multiple return values
//!
//! Use [`OutSlot`](crate::bridge::OutSlot) for out-parameters:
//!
//! ```rust,ignore
//! let mut q = OutSlot::<i64>::new();
//! let mut r = OutSlot::<i64>::new();
//! unsafe { divmod(17, 5, q.as_raw(), r.as_raw()) };
//! let (q, r) = unsafe { (q.assume_init(), r.assume_init()) };
//! ```
