# ADR-005: Multiple return values (OutSlot)

## Status: Accepted (revised)

## Context

Mojo `@export` functions cannot return tuples across the C ABI. The Mojo convention is out-pointers.

## Options considered

### A. Manual out-pointers
```rust
let mut q: i64 = 0;
let mut r: i64 = 0;
unsafe { divmod(17, 5, &mut q as *mut i64 as isize, &mut r as *mut i64 as isize) };
```
Cons: Verbose. Wastes `Default::default()` that Mojo overwrites.

### B. Closure-based `OutParam::call1/2/3` (tried, removed)
Numbered variants — un-Rust, doesn't compose past 3.

### C. `OutSlot<T>` following `MaybeUninit` pattern (chosen)
```rust
let mut q = OutSlot::<i64>::uninit();
let mut r = OutSlot::<i64>::uninit();
unsafe { divmod(17, 5, q.as_raw(), r.as_raw()) };
let (q, r) = unsafe { (q.assume_init(), r.assume_init()) };
```
Composable (create any number), follows stdlib naming (`uninit()` mirrors `MaybeUninit::uninit()`).

## Decision

Option C. `OutSlot<T>` with `uninit()`, `as_raw()`, `assume_init()`.

## Evidence

Example 08 (divmod), example 31 (conditional out-param with sentinel).
