# ADR-007: Error handling across FFI

## Status: Accepted (convention, not enforcement)

## Context

Mojo's `raises` keyword allows functions to throw errors, but uncaught errors in `@export` functions segfault (empirically verified). There is no C ABI mechanism to propagate Mojo errors to Rust.

## Options considered

### A. MojoResult<T> — repr(C) tagged union
Tried and removed. Requires both sides to agree on the error format, but Mojo's error system can't be marshaled into a C struct. Nothing returned it.

### B. Error sentinels (chosen convention)
```mojo
@export
def safe_div(a: Float64, b: Float64) -> Float64:
    try:
        return _div_impl(a, b)
    except:
        return -1.0  # sentinel
```
Rust side:
```rust
let result = unsafe { safe_div(10.0, 0.0) };
if result == -1.0 { /* handle error */ }
```
Pros: No framework needed. Matches C convention. Works with any return type.
Cons: Sentinel values can collide with valid results. No structured error info.

### C. Out-parameter error code
```mojo
@export
def compute(addr: Int, err_addr: Int) -> Float64:
    ...
```
Pros: Rich error info (code + message).
Cons: Every function needs an extra parameter. Mojo-side boilerplate.

### D. Thread-local error slot (like errno)
Pros: No extra parameter.
Cons: Not thread-safe without TLS. Complex implementation.

## Decision

Option B as documented convention. Pyroxide does not enforce an error handling strategy — it documents the patterns:
- NaN for float errors (example 23)
- -1 for index/count errors (example 23)
- Sentinel values for domain errors (example 25)
- `try/except` inside `@export` to prevent segfaults

Pyroxide's role: document the ABI's `raises` trap and provide `catch_mojo_call` for the Rust→Mojo→Rust callback direction.

## Evidence

Example 08: safe_div returns sentinel on div-by-zero.
Example 23: normalize returns NaN for degenerate input, argmax returns -1 for empty.
Example 25: write_greeting returns -1 for buffer-too-small.
