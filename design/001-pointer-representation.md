# ADR-001: Pointer representation across FFI

## Status: Accepted

## Context

Mojo's `@export` functions receive pointers as `Int` (isize). We need to convert Rust references to this representation.

## Options considered

### A. Raw `as isize` casts everywhere
```rust
unsafe { mojo_fn(&val as *const T as isize) }
```
Pros: Zero abstraction, obvious what's happening.
Cons: Verbose, error-prone (can pass any isize), no lifetime safety.

### B. MojoAddr newtype over isize
```rust
unsafe { mojo_fn(val.as_mojo().addr().as_raw()) }
```
Pros: Type safety prevents mixing addresses with lengths.
Cons: Three method calls for one isize. In practice, all addresses are the same "kind" — unlike Vulkan where Buffer ≠ Image. Removed in PR #4.

### C. `.as_raw()` on IntoMojo trait (chosen)
```rust
unsafe { mojo_fn(val.as_raw()) }
```
Pros: One call, zero overhead, returns plain isize that FFI expects.
Cons: No type distinction between address and other isize values.

## Decision

Option C. The `.as_raw()` method on `IntoMojo` converts `&self` to `isize` in one call. `MojoAddr` was tried and removed — it added ceremony without preventing real mistakes since all Mojo addresses are semantically identical.

## Evidence

All 26 examples use `.as_raw()`. Zero instances of address/length mix-up bugs occurred across 26 examples covering every API pattern.
