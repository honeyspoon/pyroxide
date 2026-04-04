# pyroxide

[![CI](https://github.com/honeyspoon/pyroxide/actions/workflows/ci.yml/badge.svg)](https://github.com/honeyspoon/pyroxide/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

> **Warning:** This is AI slop. Built in one session with Claude. Use at your own risk and peril.

Zero-copy FFI bridge between Rust and [Mojo](https://docs.modular.com/mojo/) — the glowing bridge between oxidation and fire.

```rust
use pyroxide::prelude::*;

mojo_type! {
    pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }
}

unsafe extern "C" {
    fn vec3_length(addr: isize) -> f64;
}

let v = Vec3 { x: 3.0, y: 4.0, z: 0.0 };
let len = unsafe { vec3_length(v.as_mojo().addr().as_raw()) };
```

## Overview

Pyroxide lets Rust and Mojo share data with zero copies. Define types once in Rust, pass pointers to Mojo, get results back — no serialization, no allocation.

| Crate | Purpose |
|-------|---------|
| [`pyroxide`](pyroxide/) | Core bridge: types, traits, handles, error handling |
| [`max-sys`](max-sys/) | Raw bindgen bindings to the [Modular MAX](https://docs.modular.com/max/) C API |

> **Why no `mojo-sys`?** Mojo has no C SDK. You call Mojo via `@export` → shared library → `extern "C"`. `max-sys` is for the MAX inference engine, which does have a C API.

| Module | What |
|--------|------|
| `bridge` | `IntoMojo`, `FromMojo`, `MojoRef`, `MojoMut`, `MojoSlice`, `MojoSliceMut`, `MojoAddr` |
| `abi` | ABI type mapping docs, `OutParam`, `MojoArg` |
| `trampoline` | `catch_mojo_call`, `MojoResult`, `MojoError` |
| `string` | `MojoStr` (ptr+len for FFI) |
| `types::max` | `DType`, `Tensor<T>`, `TensorDescriptor`, `TensorShape` |

## Prerequisites

- [Rust](https://rustup.rs) 1.85+ (edition 2024)
- [Mojo](https://docs.modular.com/mojo/manual/get-started) via pixi: `pixi global install mojo`

## Quick start

```sh
git clone https://github.com/honeyspoon/pyroxide
cd pyroxide
cargo run -p pyroxide-examples --example 01_hello     # simplest possible call
cargo run -p pyroxide-examples --example 07_embeddings # real HuggingFace model
make test                                             # run all 7 examples
```

Mojo files are compiled automatically by `build.rs` — no manual steps.

## Examples

Progressive tutorial — each builds on the previous.

| # | Example | What you learn |
|---|---------|----------------|
| 01 | [`hello`](examples/examples/01_hello.rs) | Raw FFI: call one Mojo function |
| 02 | [`structs`](examples/examples/02_structs.rs) | Pass `#[repr(C)]` structs, read and mutate |
| 03 | [`tensors`](examples/examples/03_tensors.rs) | `TensorDescriptor`, sum, dot, matmul |
| 04 | [`simd`](examples/examples/04_simd.rs) | Mojo's explicit SIMD: ~8x speedup |
| 05 | [`dtype_generic`](examples/examples/05_dtype_generic.rs) | One Mojo template → f32/f64/i32 |
| 06 | [`comptime`](examples/examples/06_comptime.rs) | Compile-time unrolling, baked constants |
| 07 | [`embeddings`](examples/examples/07_embeddings.rs) | HuggingFace model → Mojo inference → similarity matrix |
| 08 | [`abi_edge_cases`](examples/examples/08_abi_edge_cases.rs) | Bool, Int boundaries, Float64 specials, `OutParam` |
| 09 | [`image_blur`](examples/examples/09_image_blur.rs) | Large mutable buffer, `MojoSliceMut` |
| 10 | [`tokenizer`](examples/examples/10_tokenizer.rs) | `MojoStr` string passing, variable-length output |
| 11 | [`neural_layer`](examples/examples/11_neural_layer.rs) | Linear + ReLU + softmax, 4 `TensorDescriptor`s |
| 12 | [`accumulator`](examples/examples/12_accumulator.rs) | Stateful struct, repeated `MojoMut` across calls |

## How it works

### Rust side

```rust
use pyroxide::prelude::*;

mojo_type! {
    pub struct Particle {
        pub pos: [f64; 3],
        pub vel: [f64; 3],
        pub mass: f64,
    }
}

unsafe extern "C" {
    fn compute_energy(addr: isize) -> f64;
}

let p = Particle { pos: [0.0; 3], vel: [1.0; 3], mass: 2.0 };
let energy = unsafe { compute_energy(p.as_mojo().addr().as_raw()) };
```

### Mojo side

```mojo
@export
def compute_energy(addr: Int) -> Float64:
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var vx = p[3]
    var vy = p[4]
    var vz = p[5]
    var mass = p[6]
    return 0.5 * mass * (vx*vx + vy*vy + vz*vz)
```

### Cost

| Operation | Overhead |
|-----------|----------|
| `v.as_mojo().addr().as_raw()` | ~1ns (pointer cast) |
| `v.as_mojo_mut().addr().as_raw()` | ~1ns |
| `MojoSlice::new(&data).addr()` | 0 copies (read-only slice) |
| `MojoSliceMut::new(&mut data).addr()` | 0 copies (mutable slice) |
| `MojoStr::new(s).addr()` | 0 copies (string → ptr+len) |
| `TensorDescriptor` | 0 copies |
| `OutParam::call2(\|\| ...)` | 0ns overhead (MaybeUninit) |
| `catch_mojo_call(\|\| ...)` | 0ns on success |

### Safety

- **Dangling pointers**: `MojoRef<'a, T>` ties pointer lifetime to the Rust borrow
- **Panics across FFI**: `catch_mojo_call` catches panics (unwinding across `extern "C"` is UB)
- **Layout mismatch**: `mojo_type!` enforces `#[repr(C)]` at compile time
- **Ownership**: Rust owns, Mojo borrows — documented and enforced by types

## Feature flags

| Flag | Enables |
|------|---------|
| `max` | `DType`, `TensorShape`, `TensorDescriptor`, `Tensor<T>`, `MojoDType` |

## Status

Early stage. API will change.

| Component | Status |
|-----------|--------|
| `mojo_type!`, `IntoMojo`, `MojoRef`/`MojoMut` | Tested across 12 examples |
| `MojoSlice` / `MojoSliceMut` | Tested (image blur, SIMD, dtype) |
| `MojoAddr` (typed address newtype) | Tested, zero-cost verified via asm |
| `MojoStr` | Tested (tokenizer, uppercase) |
| `OutParam` | Tested (divmod, ABI edge cases) |
| `TensorDescriptor`, `Tensor<T>` | Tested with HuggingFace + neural layer |
| `catch_mojo_call`, `MojoResult` | Implemented, trampoline ready |
| `abi` module (type mapping docs) | Empirically verified against Mojo 0.26 |
| Proc macros (`#[mojo_fn]`) | Not implemented |

## Non-goals

- **Async bridging.** Incompatible runtimes. Use `spawn_blocking` for async Rust.
- **GIL / runtime token.** Mojo has no GIL. Thread safety is your responsibility.
- **Mojo codegen.** No `.mojo` generation from Rust types. Write both sides by hand.

## Project layout

```
pyroxide/           Core library
max-sys/          Bindgen from MAX C headers (8 headers, 131 bindings)
examples/
  mojo/           Mojo source files (auto-compiled by build.rs)
  examples/       Rust example binaries
scripts/
  fetch-headers.sh
```

## License

[MIT](LICENSE)
