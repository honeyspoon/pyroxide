# ADR-018: Thread safety empirics — what we tested

## Status: Record

## Context

The README says "thread safety is your responsibility." Example 29 tested what that actually means.

## Findings (Mojo 0.26.x on macOS ARM64)

### Pure @export functions: SAFE for concurrent calls
8 Rust threads calling `pure_dot()` and `pure_sum_sq()` simultaneously — no crash, correct results, both with independent and shared input data.

### parallelize: SEGFAULTS from shared library
Mojo's `parallelize` segfaults when called from a shared library loaded via FFI (ADR-011). The Mojo thread pool isn't initialized in the dylib context.

### Unknown: mutable shared state across threads
Not tested. If two threads call a Mojo function that writes to the same address, the result is a data race — same as in C. Pyroxide does not prevent this.

## Decision

Document the findings. Pure read-only Mojo functions are safe to call concurrently. Mutation and parallelism require user-side synchronization.

## Evidence

Example 29 — 8 threads, 1000-element vectors, shared and independent data, all correct.
