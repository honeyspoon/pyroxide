# ADR-002: Struct layout across FFI (mojo_type! macro)

## Status: Accepted

## Context

Rust structs passed to Mojo must have a stable, C-compatible memory layout. Mojo reads fields at byte offsets via `UnsafePointer`.

## Options considered

### A. Manual `#[repr(C)]` + manual zerocopy derives
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, IntoBytes, FromBytes, Immutable, KnownLayout)]
pub struct Point { pub x: f64, pub y: f64 }
```
Pros: Explicit, no macro magic.
Cons: 6 derives to remember. Easy to forget one. Requires `zerocopy` as a direct dependency.

### B. `mojo_type!{}` macro (chosen)
```rust
mojo_type! {
    pub struct Point { pub x: f64, pub y: f64 }
}
```
Pros: One line, all derives automatic, `IntoMojo`/`FromMojo` for free.
Cons: Macro hides what's happening. Can't use for structs with padding (zerocopy rejects implicit padding — see example 18).

### C. Proc macro `#[mojo_struct]`
Pros: Better IDE support, can generate Mojo-side struct definition.
Cons: Requires a proc-macro crate, heavier dependency. Deferred to 0.2.0.

## Decision

Option B for 0.1.0. The macro is a thin wrapper — users can always write Option A if they need custom behavior.

## Tradeoff: padding

zerocopy's `IntoBytes` rejects structs with implicit padding. A struct like `{ flag: u8, value: f64 }` won't compile. Users must add explicit `_pad: [u8; 7]` fields. This is intentional — it forces the user to be aware of the exact byte layout Mojo will see.

## Evidence

Example 18 (padding) demonstrates the explicit padding requirement.
Example 15 (nested structs) shows that nesting mojo_type structs works correctly.
