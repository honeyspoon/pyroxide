//! # pyroxide
//!
//! Zero-copy FFI bridge between Rust and Mojo — the glowing bridge
//! between oxidation and fire.
//!
//! ## Architecture
//!
//! Pyroxide follows the same layered design as [PyO3](https://pyo3.rs),
//! adapted for Mojo's value-oriented memory model:
//!
//! | Layer | `PyO3` equivalent | Pyroxide |
//! |-------|-----------------|--------|
//! | Type declaration | `#[pyclass]` | [`mojo_type!`] |
//! | Rust → Mojo | `IntoPyObject` | [`IntoMojo`](bridge::IntoMojo) |
//! | Mojo → Rust | `FromPyObject` | [`FromMojo`](bridge::FromMojo) |
//! | Pointer handles | `Bound<'py, T>` | [`MojoRef`](bridge::MojoRef) / [`MojoMut`](bridge::MojoMut) |
//! | Error handling | `PyResult` + trampoline | [`MojoResult`](trampoline::MojoResult) + [`catch_mojo_call`](trampoline::catch_mojo_call) |
//! | String passing | `PyString` | [`MojoStr`](string::MojoStr) |
//!
//! ## Safety model
//!
//! Pyroxide wraps the raw `unsafe` FFI boundary in typed, lifetime-bound
//! handles. The safety guarantees:
//!
//! - **No dangling pointers**: [`MojoRef`](bridge::MojoRef) ties the pointer's
//!   validity to the Rust borrow's lifetime.
//! - **No panics across FFI**: [`catch_mojo_call`](trampoline::catch_mojo_call)
//!   catches panics at the boundary, preventing undefined behavior.
//! - **No layout mismatch**: [`mojo_type!`] enforces `#[repr(C)]` and zerocopy
//!   derives at compile time.
//! - **Ownership is explicit**: Rust owns the data, Mojo borrows via pointer.
//!   Mojo must not store pointers beyond the call duration.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use pyroxide::prelude::*;
//!
//! mojo_type! {
//!     pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }
//! }
//!
//! unsafe extern "C" {
//!     fn vec3_length(addr: isize) -> f64;
//! }
//!
//! let v = Vec3 { x: 3.0, y: 4.0, z: 0.0 };
//! let len = unsafe { vec3_length(v.as_mojo().addr().as_raw()) };
//! ```
//!
//! ## Feature flags
//!
//! - **`max`** — Types matching the Modular MAX ML framework
//!   (`DType`, `TensorShape`, `TensorDescriptor`, Tensor)

pub mod abi;
pub mod bridge;
pub mod string;
pub mod trampoline;
pub mod types;

#[doc(hidden)]
pub use zerocopy;

pub mod prelude {
    pub use crate::abi::OutParam;
    pub use crate::bridge::{FromMojo, IntoMojo, MojoAddr, MojoMut, MojoRef, MojoSlice};
    pub use crate::mojo_type;
    pub use crate::string::MojoStr;
    pub use crate::trampoline::{MojoError, MojoResult, catch_mojo_call};
    #[cfg(feature = "max")]
    pub use crate::types::max;
    pub use crate::types::primitives::*;
}

/// Declare a struct that can safely cross the Mojo FFI boundary.
///
/// Adds `#[repr(C)]` and all zerocopy derives automatically.
/// The struct implements [`IntoMojo`](bridge::IntoMojo) and
/// [`FromMojo`](bridge::FromMojo).
///
/// # Example
///
/// ```rust,ignore
/// use pyroxide::prelude::*;
///
/// mojo_type! {
///     /// A particle in 2D space.
///     pub struct Particle {
///         pub x: f64,
///         pub y: f64,
///         pub mass: f64,
///     }
/// }
///
/// // The struct now has:
/// //   .as_mojo()     → MojoRef (immutable pointer handle)
/// //   .as_mojo_mut() → MojoMut (mutable pointer handle)
/// //   #[repr(C)]     → stable memory layout
/// //   IntoBytes       → can be viewed as &[u8]
/// //   FromBytes       → can be reinterpreted from &[u8]
/// ```
#[macro_export]
macro_rules! mojo_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $ty:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[repr(C)]
        #[derive(
            Debug, Clone, Copy, PartialEq,
            $crate::zerocopy::IntoBytes,
            $crate::zerocopy::FromBytes,
            $crate::zerocopy::Immutable,
            $crate::zerocopy::KnownLayout,
        )]
        $vis struct $name {
            $($field_vis $field : $ty),*
        }
    };
}
