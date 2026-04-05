# ADR-006: Panic safety at the FFI boundary

## Status: Accepted

## Context

A Rust panic unwinding across `extern "C"` is undefined behavior. If Rust code called from Mojo panics, the process could corrupt memory or crash in unpredictable ways.

## Options considered

### A. Do nothing — document the risk
Cons: UB waiting to happen. PyO3 proved this must be handled.

### B. `catch_panic_at_ffi` returning `T::default()` (chosen)
```rust
extern "C" fn my_callback() -> f64 {
    catch_panic_at_ffi(|| {
        // panic here is safe — caught and returns 0.0
        42.0
    })
}
```
Pros: Simple, zero-cost on success (catch_unwind is free when no panic). `T::default()` is a reasonable sentinel (0 for numbers, false for bool).
Cons: Silently swallows errors. The default value might be a valid result.

### C. `catch_mojo_result` returning `MojoResult<T>` (tried and removed)
Removed in PR #3. MojoResult is a `#[repr(C)]` tagged union, but nothing on the Mojo side reads it.

### D. abort on panic
Pros: No silent failures.
Cons: Kills the process. Too harsh for a library.

## Decision

Option B for 0.1.0. `catch_panic_at_ffi` is for the reverse-FFI direction (Mojo calling Rust callbacks). It prevents UB. The silent-default tradeoff is acceptable because the alternative (UB) is worse.

Option C deferred — needs Mojo-side convention for error handling.

## Evidence

Example 25 (catch_panic) demonstrates panic recovery: a closure that panics returns `f64::default()` (0.0) instead of unwinding across FFI.
