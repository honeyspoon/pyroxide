# ADR-013: Opinionless design — map conventions, don't invent them

## Status: Accepted (core principle)

## Context

Pyroxide sits between two languages with their own conventions. It could impose its own idioms (like PyO3 does with `PyResult`, `Bound<'py, T>`, `#[pyclass]`) or stay thin and just map one convention to the other.

## Decision

Pyroxide maps conventions 1:1. Every type in pyroxide corresponds to a real convention in either Mojo or Rust:

| Pyroxide | Mojo convention | Rust convention |
|----------|----------------|-----------------|
| `mojo_type!` | Struct layout at byte offsets | `#[repr(C)]` |
| `.as_raw()` | `Int` (pointer as integer) | `*const T as isize` |
| `MojoSlice` | `(ptr: Int, len: Int)` pair | `&[T]` (fat pointer) |
| `MojoStr` | `(ptr: Int, len: Int)` for text | `&str` |
| `OutSlot` | Out-pointer return convention | `MaybeUninit` |
| `catch_panic_at_ffi` | N/A (Mojo doesn't have this) | `catch_unwind` |
| `TensorDescriptor` | MAX's tensor metadata layout | `#[repr(C)]` struct |

Things pyroxide does NOT do:
- Generate Mojo code from Rust types
- Invent error handling beyond sentinel conventions
- Create a GIL/runtime token (Mojo has none)
- Provide async bridging (incompatible runtimes)
- Wrap `unsafe` in "safe" abstractions that hide the FFI nature

## Why

Mojo is a new language. Its conventions are still evolving. If pyroxide invents its own abstractions, they'll be wrong when Mojo changes. By mapping 1:1, pyroxide stays correct as long as the C ABI stays stable — which it will, because it's the C ABI.

## Evidence

ADR-011 documents every abstraction we tried and removed. Each was removed because it didn't map a real convention. The 26 examples prove the thin mapping is sufficient for real workloads including HuggingFace model inference.
