# ADR-016: Thread safety for handle types

## Status: Accepted

## Context

`MojoSlice<'_, f64>` would be accidentally `Send` without explicit opt-out, because `f64: Send` and `NonNull<f64>: Send`. Slice handles represent borrowed pointers valid only for one FFI call on the calling thread — they must be `!Send + !Sync`.

## Options considered

### A. `impl !Send for MojoSlice` (negative impl)
Requires nightly Rust. Not acceptable for a stable crate.

### B. `PhantomData<*const ()>` (chosen)
`*const ()` is `!Send + !Sync`, and `PhantomData` inherits that. Adding it to the struct makes the whole type `!Send + !Sync` regardless of `T`'s bounds.

### C. Do nothing — document the risk
Users could accidentally share handles across threads with `Send` types. UB waiting to happen.

## Decision

Option B. `PhantomData<*const ()>` on `MojoSlice` and `MojoSliceMut`. Zero runtime cost, unconditional `!Send + !Sync`.

Note: `MojoRef`/`MojoMut` also had this, but were removed (ADR-015). The problem only remains on slice handles because they're the only handle types left.

## Evidence

Example 29 (concurrent) proves pure Mojo functions ARE thread-safe — but the slice handles themselves should not cross threads because they represent a borrow valid only for the call duration.
