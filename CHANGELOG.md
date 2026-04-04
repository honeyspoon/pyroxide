# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

## [0.1.0] - 2026-04-03

### Added

- `pyroxide` crate: `mojo_type!` macro, `IntoMojo`/`FromMojo` traits, `MojoRef`/`MojoMut`/`MojoSlice` handles
- `pyroxide::trampoline`: `catch_mojo_call` for panic-safe FFI, `MojoResult<T>` error type
- `pyroxide::string`: `MojoStr` for FFI-safe string passing
- `pyroxide::types::max`: `DType`, `TensorShape`, `TensorDescriptor`, `Tensor<T>`, `MojoDType` trait
- `pyroxide::types::primitives`: `Point`, `Vec4`, `Mat4` with operator overloads
- `max-sys` crate: bindgen from Modular MAX C headers (types, tensor, context, model, device, weights, common, symbol_export)
- 7 progressive examples (01_hello through 07_embeddings)
- `build.rs` auto-compiles `.mojo` files to shared libraries
- `scripts/fetch-headers.sh` downloads MAX headers from GitHub
