# Changelog

Auto-generated from merged pull requests. Do not list specific types
in version entries — they change. See the current API in the README.

## [Unreleased]

Breaking changes since 0.1.1 (will be 0.2.0):
- Removed `MojoRef`, `MojoMut`, `OutParam`, `catch_mojo_call`, `MojoAddr`, `MojoArg`, `MojoResult`
- `descriptor()` returns `DescriptorGuard<'_>` (lifetime-bound, Deref to TensorDescriptor)
- `OutParam` replaced by `OutSlot<T>` (composable MaybeUninit pattern)
- `catch_mojo_call` renamed to `catch_panic_at_ffi`
- Added `TensorView<'a, T>` for zero-copy borrowed tensors
- Added `MojoSlice::len_isize()` / `MojoSliceMut::len_isize()`
- 64-bit compile gate on MAX types
- `!Send`/`!Sync` enforced on slice handles
- 31 examples, 30 unit tests, 14 ADRs

## [0.1.1] - 2026-04-04

Published to crates.io with docs.rs metadata.

## [0.1.0] - 2026-04-03

Initial publish to crates.io.
