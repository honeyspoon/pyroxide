# ADR-009: Conversion traits (IntoMojo / FromMojo)

## Status: Accepted

## Context

Types that cross the FFI boundary need `.as_raw()` for immutable access and `.as_raw_mut()` for mutable access. These could be free functions, methods on concrete types, or trait methods.

## Options considered

### A. Free functions
```rust
fn as_raw<T: IntoBytes>(val: &T) -> isize { ... }
```
Pros: No traits.
Cons: Can't use method syntax (`val.as_raw()`). Less discoverable.

### B. Blanket trait impls (chosen)
```rust
pub trait IntoMojo: IntoBytes + Immutable + KnownLayout {
    fn as_raw(&self) -> isize { ... }
}
impl<T: IntoBytes + Immutable + KnownLayout> IntoMojo for T {}
```
Pros: Any zerocopy-compatible type gets `.as_raw()` for free. Method syntax. Users can write `T: IntoMojo` bounds.
Cons: Trait with only defaulted methods — some argue this should be a free function.

*Note: an earlier version also had `as_mojo() -> MojoRef<'_, Self>`, but `MojoRef` was removed in the stdlib-idioms refactor — the borrow checker already tracks `&T` lifetimes directly.*

### C. Methods only on mojo_type! structs (via proc macro)
Pros: Only explicitly marked types get the methods.
Cons: Requires proc macro. Primitive types (f64, i32) wouldn't get `.as_raw()`.

## Decision

Option B. The blanket impl means any `#[repr(C)]` zerocopy type — including primitives, arrays, and user structs — gets `.as_raw()` automatically. This is the right abstraction level for "anything with a stable byte layout can be passed to Mojo."

## Resolved: unsealed by design

The traits are intentionally unsealed. Any external type satisfying the zerocopy bounds gets `.as_raw()` automatically. Sealing would gate-keep which types can cross FFI — the zerocopy bounds are the real safety gate, not trait sealing.
