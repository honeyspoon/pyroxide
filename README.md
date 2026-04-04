# embers

> **Warning:** This is AI slop. Built in one session with Claude. Use at your own risk and peril.

Zero-copy FFI bridge between Rust and Mojo — the glowing bridge between oxidation and fire.

```rust
use embers::prelude::*;

mojo_type! {
    pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }
}

unsafe extern "C" {
    fn vec3_length(addr: isize) -> f64;
}

let v = Vec3 { x: 3.0, y: 4.0, z: 0.0 };
let len = unsafe { vec3_length(v.as_mojo().addr()) };
```

## What is this?

Embers lets Rust and Mojo share data across the FFI boundary with zero copies. You define types once in Rust, pass pointers to Mojo, and get results back — no serialization, no allocation, no overhead.

**Crates:**

| Crate | Purpose |
|-------|---------|
| `embers` | Core bridge: types, traits, handles, error handling, strings |
| `max-sys` | Raw bindgen bindings to the Modular MAX C API |

**Key modules in `embers`:**

| Module | What | PyO3 equivalent |
|--------|------|-----------------|
| `bridge` | `IntoMojo`, `FromMojo`, `MojoRef`, `MojoMut` | `IntoPyObject`, `Bound<'py, T>` |
| `trampoline` | `catch_mojo_call`, `MojoResult`, `MojoError` | `PyResult` + trampoline |
| `string` | `MojoStr` (ptr+len for FFI) | `PyString` |
| `types::max` | `DType`, `Tensor<T>`, `TensorDescriptor` | `rust-numpy` |

## Prerequisites

- **Rust** (1.85+, edition 2024)
- **Mojo** — install via [pixi](https://prefix.dev): `pixi global install mojo`

## Quick start

```sh
# Run any example (Mojo is compiled automatically by build.rs)
cargo run -p embers-examples --example 01_hello

# Run all examples
make test
```

## Examples

The examples form a progressive tutorial — each builds on the previous one.

| # | Example | What you'll learn |
|---|---------|-------------------|
| 01 | `hello` | Raw FFI: call a Mojo function from Rust |
| 02 | `structs` | Pass `#[repr(C)]` structs, read and mutate |
| 03 | `tensors` | `TensorDescriptor`, sum, dot product, matmul |
| 04 | `simd` | Mojo's explicit SIMD: 8-wide vectors, ~8x speedup |
| 05 | `dtype_generic` | One Mojo template, specialized for f32/f64/i32 |
| 06 | `comptime` | Compile-time loop unrolling, baked constants |
| 07 | `embeddings` | Download a real HuggingFace model, compute embeddings via Mojo |

Run one:
```sh
cargo run -p embers-examples --example 07_embeddings
```

## How it works

### Rust side

```rust
use embers::prelude::*;

// 1. Define a type — gets #[repr(C)] + zerocopy + IntoMojo
mojo_type! {
    pub struct Particle {
        pub pos: [f64; 3],
        pub vel: [f64; 3],
        pub mass: f64,
    }
}

// 2. Declare the Mojo function
unsafe extern "C" {
    fn compute_energy(addr: isize) -> f64;
}

// 3. Call it — .as_mojo() is zero-cost, .addr() → isize for Mojo
let p = Particle { pos: [0.0; 3], vel: [1.0; 3], mass: 2.0 };
let energy = unsafe { compute_energy(p.as_mojo().addr()) };
```

### Mojo side

```mojo
# Particle layout: [pos(3), vel(3), mass] — 7 x f64
@export
def compute_energy(addr: Int) -> Float64:
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var vx = p[3]
    var vy = p[4]
    var vz = p[5]
    var mass = p[6]
    return 0.5 * mass * (vx*vx + vy*vy + vz*vz)
```

### The bridge

| Rust | Mojo | Cost |
|------|------|------|
| `v.as_mojo().addr()` | receives `Int` | ~1ns (pointer cast) |
| `v.as_mojo_mut().addr()` | writes through pointer | ~1ns |
| `TensorDescriptor` | reads shape/data_ptr | 0 copies |
| `MojoStr::from_str(s)` | reads `(ptr, len)` | 0 copies |
| `catch_mojo_call(\|\| ...)` | catches Rust panics at FFI boundary | ~0ns (no-op on success) |

### Safety

Embers prevents the most common FFI bugs:

- **Dangling pointers**: `MojoRef<'a, T>` ties the pointer lifetime to the Rust borrow
- **Panics across FFI**: `catch_mojo_call` catches panics (unwinding across `extern "C"` is UB)
- **Layout mismatch**: `mojo_type!` enforces `#[repr(C)]` at compile time
- **Ownership confusion**: Rust owns, Mojo borrows — documented and enforced by types

## Workspace layout

```
embers/              Core library (mojo_type!, IntoMojo, MojoRef, MAX types)
max-sys/             Bindgen from real MAX C headers (8 headers, 131 bindings)
examples/
  mojo/              Mojo source files (compiled by build.rs)
  examples/          Rust example binaries (cargo run --example)
scripts/
  fetch-headers.sh   Downloads MAX headers from modular/modular
```

## Feature flags

| Flag | What it enables |
|------|-----------------|
| `max` | DType, TensorShape, TensorDescriptor, Tensor\<T\>, MojoDType trait |

## Status

Embers is early-stage. Some parts are solid, some are sketched out.

| Component | Status |
|-----------|--------|
| `mojo_type!`, `IntoMojo`, `MojoRef`/`MojoMut` | Tested, stable API |
| `TensorDescriptor`, `Tensor<T>` | Tested with real HuggingFace models |
| `catch_mojo_call`, `MojoResult` | Implemented, not yet used in examples |
| `MojoStr` | Implemented, not yet used in examples |
| `MojoOwned<T>` (heap alloc with Mojo lifetime) | Not implemented |
| Proc macros (`#[mojo_fn]`, `#[mojo_module]`) | Not implemented |

## Non-goals

Some things embers will not do:

- **Async bridging.** Mojo and Rust have incompatible async runtimes (Mojo uses its own cooperative scheduler, Rust uses tokio/async-std). Bridging them would require a complex polling adapter with no clear benefit over synchronous FFI calls. If you need async, call the synchronous FFI from a `spawn_blocking` task.

- **GIL or runtime token.** Unlike Python, Mojo has no global interpreter lock. Embers does not inject a runtime context token into every call. Thread safety is your responsibility — don't pass `MojoMut` handles to the same data from multiple threads.

- **Automatic Mojo struct generation.** Embers does not generate `.mojo` files from Rust types. You write both sides by hand. The layout contract is documented, enforced by `#[repr(C)]`, and tested in the examples.

## License

MIT OR Apache-2.0
