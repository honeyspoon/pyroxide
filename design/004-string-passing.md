# ADR-004: String passing (MojoStr)

## Status: Accepted

## Context

Mojo's `String` is a heap-allocated internal type that cannot cross the C ABI. Text must be passed as `(ptr: Int, len: Int)`.

## Options considered

### A. Just use `MojoSlice<u8>`
```rust
let bytes = text.as_bytes();
let s = MojoSlice::new(bytes);
unsafe { mojo_fn(s.as_raw(), s.len() as isize) }
```
Pros: No new type.
Cons: Loses the semantic distinction between bytes and UTF-8 text. No `len_isize()` convenience.

### B. Dedicated `MojoStr` (chosen)
```rust
let s = MojoStr::new(text);
unsafe { mojo_fn(s.as_raw(), s.len_isize()) }
```
Pros: Clear intent (this is text, not arbitrary bytes). Has `as_str()` for reconstructing `&str` from Mojo-returned pointers. `len_isize()` avoids `as isize` casts.
Cons: Structurally identical to `MojoSlice<u8>`. One more type to learn.

### C. `MojoStr` as a newtype over `MojoSlice<u8>`
Pros: Code reuse via delegation.
Cons: `MojoSlice<u8>` requires `IntoBytes + Immutable` which `u8` satisfies, but the delegation adds indirection without benefit.

## Decision

Option B. `MojoStr` is a thin separate type because:
1. It provides `unsafe fn as_str()` which `MojoSlice<u8>` can't
2. It documents intent: this is UTF-8 text
3. It has `len_isize()` for the common Mojo calling pattern

## Evidence

Example 10 (tokenizer) and example 25 (catch_panic / string output) exercise MojoStr in both directions.
