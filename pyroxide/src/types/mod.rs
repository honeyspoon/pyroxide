//! Types for the Mojo FFI bridge.
//!
//! - `primitives`: Math/physics types with Mojo-computed methods
//! - `max` (feature = "max"): MAX ML framework types (Tensor, `DType`, etc.)

pub mod primitives;

#[cfg(feature = "max")]
pub mod max;

pub use primitives::*;
