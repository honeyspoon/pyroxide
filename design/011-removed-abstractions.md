# ADR-011: Removed abstractions (things we tried and cut)

## Status: Record

## Abstractions that were removed

### MojoAddr (PR #4)
Newtype over isize. Added `.addr().as_raw()` two-call chain to every call site. Removed because all Mojo addresses are semantically identical — there are no different "kinds" of address to confuse. The Vulkan analogy (vk::Buffer ≠ vk::Image) doesn't apply.

### MojoArg trait (PR #3)
Trait mapping Rust types to isize for FFI. Zero callers in the entire codebase. Was aspirational for a `mojo_fn!` macro that didn't exist. Dead code.

### MojoResult / MojoError / catch_mojo_result (PR #3)
`#[repr(C)]` tagged union for cross-FFI error reporting. Nothing returned it. Mojo's error system can't be marshaled into C structs. The error sentinel convention (ADR-007) is simpler and works.

### mojo_import! macro (early, pre-PR)
Generated safe wrappers from function declarations. Removed because we didn't know the API shape yet. The explicit `unsafe extern "C"` pattern proved sufficient. Deferred to 0.2.0.

### Parallelize example (early)
`parallelize` in Mojo segfaults when called from a shared library loaded via FFI — the Mojo thread pool isn't initialized. Removed the example. Documented the limitation.

### MojoRef / MojoMut (PR #24)
Lifetime-bound pointer handles. Removed because `&T` + `IntoMojo::as_raw()` provides the same invariant — the borrow checker already tracks the reference lifetime. See ADR-015.

### OutParam::call1/2/3 (PR #24)
Closure-based out-parameter helper with numbered variants. Replaced by `OutSlot<T>` which follows the stdlib `MaybeUninit` pattern and composes to any count. See ADR-005.

## Principle

Pyroxide maps Mojo conventions to Rust conventions 1:1. It does not invent abstractions that exist in neither language. Each removal was motivated by finding zero callers or finding that the abstraction added ceremony without preventing real mistakes.
