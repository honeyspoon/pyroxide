# ADR-005: Multiple return values (OutParam)

## Status: Accepted

## Context

Mojo `@export` functions cannot return tuples across the C ABI. The Mojo convention is out-pointers: Mojo writes results into caller-provided addresses.

## Options considered

### A. Manual out-pointers
```rust
let mut q: i64 = 0;
let mut r: i64 = 0;
unsafe { divmod(17, 5, &mut q as *mut i64 as isize, &mut r as *mut i64 as isize) };
```
Pros: No abstraction.
Cons: 4 lines for 2 values. Uses `Default` (wasteful init).

### B. OutParam with `Default` init
```rust
let (q, r): (i64, i64) = OutParam::call2(|qp, rp| unsafe { divmod(17, 5, qp, rp) });
```
Tried first, but wastes a `Default::default()` call that Mojo immediately overwrites.

### C. OutParam with `MaybeUninit` (chosen)
```rust
let (q, r): (i64, i64) = unsafe { OutParam::call2(|qp, rp| divmod(17, 5, qp, rp)) };
```
Pros: Zero-cost (no wasted init). Verified via assembly: compiles to same code as manual.
Cons: Must be `unsafe fn` — caller promises Mojo writes to all pointers.

### D. Tuple trait on `(A, B)`, `(A, B, C)`, etc.
Pros: More Rust-idiomatic.
Cons: Complex implementation for marginal ergonomic gain over `call2`/`call3`.

## Decision

Option C. `OutParam::call1/2/3` as `unsafe fn` with `MaybeUninit`. Soundness is documented: undefined behavior if Mojo doesn't write.

## Evidence

Example 08 (ABI edge cases) uses `OutParam::call2` for divmod.
