# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Branch protection on `main` (CI checks required)
- Pre-commit hooks (`scripts/install-hooks.sh`)
- `cargo-semver-checks` in CI for API compatibility
- `cargo-machete` for unused dependency detection
- 4 new examples: image blur, tokenizer, neural layer, accumulator

### Changed
- `MojoRef::addr()` / `MojoMut::addr()` now return `MojoAddr` (typed newtype) instead of raw `isize`
- `OutParam` uses `MaybeUninit` (zero-cost, no wasted default init)

## [0.1.0] - 2026-04-03

### Added
- `pyroxide` crate: `mojo_type!` macro, `IntoMojo`/`FromMojo` traits, `MojoRef`/`MojoMut`/`MojoSlice` handles
- `pyroxide::trampoline`: `catch_mojo_call` for panic-safe FFI, `MojoResult<T>` error type
- `pyroxide::string`: `MojoStr` for FFI-safe string passing
- `pyroxide::abi`: ABI documentation, `OutParam` helper, `MojoArg` trait, `MojoAddr` newtype
- `pyroxide::types::max`: `DType`, `TensorShape`, `TensorDescriptor`, `Tensor<T>`, `MojoDType` trait
- `pyroxide::types::primitives`: `Point`, `Vec4`, `Mat4` with operator overloads
- `max-sys` crate: bindgen from Modular MAX C headers (8 headers, 131 bindings)
- 12 progressive examples (01_hello through 12_accumulator)
- `build.rs` auto-compiles `.mojo` files to shared libraries
- `scripts/fetch-headers.sh` downloads MAX headers from GitHub
- GitHub Actions CI (check, clippy, fmt, semver)
- Strict clippy config: pedantic + nursery + cargo groups, `unwrap_used = deny`
