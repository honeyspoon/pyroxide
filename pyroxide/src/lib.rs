//! # pyroxide
//!
//! Zero-copy FFI bridge between Rust and Mojo — the glowing bridge
//! between oxidation and fire.
//!
//! ## Safety model
//!
//! - **No dangling pointers**: [`MojoRef`](bridge::MojoRef) ties the pointer's
//!   validity to the Rust borrow's lifetime.
//! - **No panics across FFI**: [`catch_mojo_call`](trampoline::catch_mojo_call)
//!   catches panics at the boundary, preventing undefined behavior.
//! - **No layout mismatch**: [`mojo_type!`] enforces `#[repr(C)]` and zerocopy
//!   derives at compile time.
//! - **Ownership is explicit**: Rust owns the data, Mojo borrows via pointer.
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
//! let len = unsafe { vec3_length(v.as_raw()) };
//! ```
//!
//! ## Why `mojo_type!` but no `mojo_fn!`?
//!
//! Types need 6 derives that are hard to remember — the macro prevents errors.
//! Functions just need `unsafe extern "C" { ... }` — standard Rust, 1 line,
//! nothing to simplify. See [ADR-014](https://github.com/honeyspoon/pyroxide/blob/main/design/014-macro-asymmetry.md).
//!
//! ## Feature flags
//!
//! - **`max`** — Types matching the Modular MAX ML framework
//!   (`DType`, `TensorShape`, `TensorDescriptor`, `Tensor`, `TensorView`)

pub mod abi;
pub mod bridge;
pub mod string;
pub mod trampoline;
pub mod types;

#[doc(hidden)]
pub use zerocopy;

pub mod prelude {
    pub use crate::abi::OutParam;
    pub use crate::bridge::{FromMojo, IntoMojo, MojoMut, MojoRef, MojoSlice, MojoSliceMut};
    pub use crate::mojo_type;
    pub use crate::string::MojoStr;
    pub use crate::trampoline::catch_mojo_call;
    #[cfg(feature = "max")]
    pub use crate::types::max;
    pub use crate::types::primitives::*;
}

/// Declare a struct that can safely cross the Mojo FFI boundary.
///
/// Adds `#[repr(C)]` and all zerocopy derives automatically.
/// The struct implements [`IntoMojo`](bridge::IntoMojo) and
/// [`FromMojo`](bridge::FromMojo).
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

#[cfg(test)]
mod tests {
    use super::prelude::*;

    mojo_type! {
        pub struct TestVec2 {
            pub x: f64,
            pub y: f64,
        }
    }

    #[test]
    fn mojo_type_is_repr_c() {
        assert_eq!(std::mem::size_of::<TestVec2>(), 16);
    }

    #[test]
    fn mojo_type_has_into_mojo() {
        let v = TestVec2 { x: 1.0, y: 2.0 };
        assert_ne!(v.as_raw(), 0);
    }

    #[test]
    fn mojo_type_has_from_mojo() {
        let mut v = TestVec2 { x: 0.0, y: 0.0 };
        assert_ne!(v.as_raw_mut(), 0);
    }

    #[test]
    fn mojo_type_is_copy_and_eq() {
        let a = TestVec2 { x: 1.0, y: 2.0 };
        let b = a;
        assert_eq!(a, b);
    }
}
