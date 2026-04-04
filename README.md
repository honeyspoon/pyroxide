# pyroxide

[![crates.io](https://img.shields.io/crates/v/pyroxide.svg)](https://crates.io/crates/pyroxide)
[![docs.rs](https://docs.rs/pyroxide/badge.svg)](https://docs.rs/pyroxide)
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
let len = unsafe { vec3_length(v.as_raw()) };
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
| `bridge` | `IntoMojo`, `FromMojo`, `MojoRef`, `MojoMut`, `MojoSlice`, `MojoSliceMut` |
| `abi` | ABI type mapping docs, `OutParam` |
| `trampoline` | `catch_mojo_call` (panic-safe FFI) |
| `string` | `MojoStr` (ptr+len for FFI) |
| `types::max` | `DType`, `Tensor<T>`, `TensorView<T>`, `TensorDescriptor`, `TensorShape` |

## Prerequisites

- [Rust](https://rustup.rs) 1.85+ (edition 2024)
- [Mojo](https://docs.modular.com/mojo/manual/get-started) via pixi: `pixi global install mojo`

## Quick start

Add to your `Cargo.toml`:
```toml
[dependencies]
pyroxide = "0.1"
```

Or with MAX tensor types:
```toml
[dependencies]
pyroxide = { version = "0.1", features = ["max"] }
```

Run the examples:
```sh
git clone https://github.com/honeyspoon/pyroxide
cd pyroxide
cargo run -p pyroxide-examples --example 01_hello     # simplest possible call
cargo run -p pyroxide-examples --example 07_embeddings # real HuggingFace model
make test                                             # run all 26 examples
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
| 13 | [`sorting`](examples/examples/13_sorting.rs) | In-place sort + reverse, verify against Rust |
| 14 | [`mandelbrot`](examples/examples/14_mandelbrot.rs) | Compute-heavy grid, ASCII visualization |
| 15 | [`nested_structs`](examples/examples/15_nested_structs.rs) | Line (2 Points), Triangle (3 Points), centroid |
| 16 | [`struct_arrays`](examples/examples/16_struct_arrays.rs) | `&[Point]`, particle simulation, kinetic energy |
| 17 | [`mixed_args`](examples/examples/17_mixed_args.rs) | 6 args: pointers + scalars + bools in one call |
| 18 | [`padding`](examples/examples/18_padding.rs) | `u8 + f64 + i32` struct with alignment padding |
| 19 | [`bytes`](examples/examples/19_bytes.rs) | `u8`/`i64` arrays, XOR, prefix sum |
| 20 | [`call_overhead`](examples/examples/20_call_overhead.rs) | Noop/identity benchmark, pointer stability |
| 21 | [`edge_cases`](examples/examples/21_edge_cases.rs) | Zero-length, NaN in structs, byte roundtrip |
| 22 | [`large_data`](examples/examples/22_large_data.rs) | 1M-element dot, scale-add, reduce-max |
| 23 | [`chained`](examples/examples/23_chained.rs) | Normalize→argmax pipeline, NaN/-1 sentinels, histogram |
| 24 | [`matrix`](examples/examples/24_matrix.rs) | Transpose, Hadamard product, trace |
| 25 | [`catch_panic`](examples/examples/25_catch_panic.rs) | `catch_mojo_call`, string output, panic recovery |
| 26 | [`pipeline`](examples/examples/26_pipeline.rs) | Enum-like dispatch, multi-step transforms |

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
let energy = unsafe { compute_energy(p.as_raw()) };
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
| `v.as_raw()` | ~1ns (pointer cast) |
| `v.as_raw_mut()` | ~1ns |
| `MojoSlice::new(&data).as_raw()` | 0 copies (read-only slice) |
| `MojoSliceMut::new(&mut data).as_raw()` | 0 copies (mutable slice) |
| `MojoStr::new(s).as_raw()` | 0 copies (string → ptr+len) |
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
| `max` | `DType`, `TensorShape`, `TensorDescriptor`, `Tensor<T>`, `TensorView<T>`, `MojoDType` |

## Status

Early stage. API will change.

| Component | Status |
|-----------|--------|
| `mojo_type!`, `IntoMojo`/`FromMojo` | 31 unit tests + 26 examples |
| `MojoSlice` / `MojoSliceMut` | Tested across 15+ examples |
| `MojoStr` | Tested (tokenizer, string output) |
| `OutParam` | Tested (divmod, ABI edge cases) |
| `catch_mojo_call` | Tested (panic recovery example) |
| `Tensor<T>` / `TensorView<T>` | Tested with HuggingFace + neural layer |
| `abi` module | Empirically verified against Mojo 0.26 |
| Proc macros (`#[mojo_fn]`) | Not implemented — deferred to 0.2.0 |

## Non-goals

- **Async bridging.** Incompatible runtimes. Use `spawn_blocking` for async Rust.
- **GIL / runtime token.** Mojo has no GIL. Thread safety is your responsibility.
- **Mojo codegen.** No `.mojo` generation from Rust types. Write both sides by hand.

## Project layout

```
pyroxide/           Core library (31 unit tests)
max-sys/            Bindgen from MAX C headers (8 headers, 131 bindings)
examples/
  mojo/             Mojo source files (auto-compiled by build.rs)
  examples/         26 Rust example binaries
design/             13 Architecture Decision Records
scripts/
  fetch-headers.sh  Download MAX C headers from GitHub
  pre-commit        Git pre-commit hook
  check-docs.sh     Verify docs stay in sync
```

## License

[MIT](LICENSE)
