# ADR-003: Slice passing (MojoSlice / MojoSliceMut)

## Status: Accepted

## Context

Mojo receives arrays as `(ptr: Int, len: Int)` — two separate parameters. Rust's `&[T]` is a fat pointer containing the same data.

## Options considered

### A. Raw pointer + length, manually
```rust
unsafe { mojo_fn(data.as_ptr() as isize, data.len() as isize) }
```
Pros: Zero abstraction.
Cons: Two casts per call, easy to swap ptr/len, no lifetime tracking.

### B. Single `MojoSlice<T>` for both read and write
```rust
let s = MojoSlice::new(&mut data);
```
Cons: Conflates `&[T]` and `&mut [T]`. Can't enforce exclusive mutable access.

### C. `MojoSlice` + `MojoSliceMut` (chosen)
```rust
let s = MojoSlice::new(&data);      // immutable
let m = MojoSliceMut::new(&mut data); // mutable
```
Pros: Mirrors `&[T]` / `&mut [T]`. PhantomData enforces borrow rules.
Cons: Two types instead of one.

### D. Generic `MojoSlice<'a, T, Mut>` with mutability parameter
Cons: Requires sealed traits, marker types, complex bounds. No real benefit over two concrete types.

## Decision

Option C. The split mirrors Rust's own `&[T]` / `&mut [T]` — not our invention, just a mapping.

## Evidence

Example 09 (image blur) discovered the need for MojoSliceMut — MojoSlice couldn't handle writable output buffers. Example 13 (sorting) uses MojoSliceMut for in-place sort. Example 17 (mixed args) uses both in one call.
