# ADR-010: Users write unsafe extern "C" — no generated safe wrappers

## Status: Accepted for 0.1.0

## Context

Every example requires the user to write:
```rust
unsafe extern "C" {
    fn point_distance(a: isize, b: isize) -> f64;
}
let dist = unsafe { point_distance(a.as_raw(), b.as_raw()) };
```

Libraries like `nix` and `ash` wrap this in safe functions. Should we?

## Options considered

### A. Users write unsafe extern "C" (chosen for 0.1.0)
Pros: Explicit, no magic, user controls the exact declaration. Works for any Mojo function without pyroxide needing to know about it.
Cons: Every call site is `unsafe`. Verbose.

### B. `mojo_fn!` declarative macro
```rust
mojo_fn! {
    fn point_distance(a: &Point, b: &Point) -> f64;
}
// Generates safe wrapper
```
Tried early on (as `mojo_import!`) and removed for two reasons:
1. **Epistemic** — we didn't know the API shape (mixed args, &mut, scalars vs pointers)
2. **Technical** — the macro would hide `unsafe`, which is dishonest about FFI
See also ADR-014 which elaborates on reason #2.

### C. Proc macro `#[mojo_fn]`
```rust
#[mojo_fn]
fn point_distance(a: &Point, b: &Point) -> f64;
```
Pros: Best ergonomics.
Cons: Requires a proc-macro crate. Complex to implement correctly for all argument patterns (&T, &mut T, scalars, mixed).

### D. Runtime function loading (like libloading)
```rust
let lib = MojoLibrary::load("./libmodel.dylib")?;
let f = lib.get::<fn(&Point, &Point) -> f64>("point_distance")?;
```
Pros: Dynamic loading, no compile-time linking.
Cons: Runtime overhead, no compile-time type checking.

## Decision

Option A for 0.1.0. The 26 examples prove the pattern is workable. The `unsafe` is honest — FFI IS unsafe. Wrapping it in a "safe" function just moves the unsafety to a macro expansion the user can't see.

Options B/C deferred to 0.2.0 when the API has stabilized enough to commit to a generation strategy.

## Evidence

26 examples successfully use the explicit pattern. Example 12 (accumulator) shows the cleanest version: a Rust impl block wrapping Mojo calls as methods on a struct — the user-side ergonomic solution that doesn't require framework support.
