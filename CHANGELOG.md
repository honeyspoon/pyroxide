# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Fixed
- Enforce `!Send`/`!Sync` on all handle types (#18)
- `catch_mojo_call` docs clarified: catches Rust panics only, not Mojo errors (#18)
- 64-bit compile gate on MAX tensor types (#18, #19)
- Semver: restored `MojoStr::ptr()` as deprecated, `#[repr(C)]` kept (#17)
- Stale references to removed types in README (#12)

### Added
- `TensorView<'a, T>` for zero-copy borrowed tensor data (#10)
- `MojoSliceMut` for mutable slice handles (#2)
- `MojoSlice::len_isize()` / `MojoSliceMut::len_isize()` (#14)
- `OutParam::call1/2/3` marked `unsafe fn` with safety docs (#3)
- `catch_mojo_call` — panic-safe FFI trampoline (#8, #10)
- 31 unit tests (no Mojo required) (#10)
- 26 examples covering every API pattern (#1-#8)
- 14 Architecture Decision Records (#9, #16)
- CI: clippy, fmt, test, machete, semver, docs freshness (#1, #2)
- Pre-commit hooks (#1)
- `scripts/check-docs.sh` for README/Cargo/Makefile sync (#2)

### Removed
- `MojoAddr` newtype — replaced by `.as_raw()` returning `isize` (#4)
- `MojoArg` trait — zero callers (#3)
- `MojoResult` / `MojoError` / `catch_mojo_result` — deferred to 0.2.0 (#3)

## [0.1.1] - 2026-04-04

Published to crates.io with docs.rs metadata.

- `[package.metadata.docs.rs] all-features = true`
- `max-sys` dependency versioned for crates.io

## [0.1.0] - 2026-04-03

Initial release.

- `mojo_type!` macro for `#[repr(C)]` + zerocopy derives
- `IntoMojo` / `FromMojo` traits with `.as_raw()` / `.as_raw_mut()`
- `MojoRef` / `MojoMut` — lifetime-bound pointer handles
- `MojoSlice` / `MojoSliceMut` — zero-copy slice handles
- `MojoStr` — FFI-safe string type
- `OutParam` — out-pointer helper with `MaybeUninit`
- `catch_mojo_call` — panic-safe FFI trampoline
- MAX types: `DType`, `TensorShape`, `TensorDescriptor`, `Tensor<T>`
- `max-sys` crate: bindgen from 8 MAX C headers
- `build.rs` auto-compiles `.mojo` files
