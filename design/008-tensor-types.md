# ADR-008: Tensor types (TensorDescriptor, Tensor<T>)

## Status: Accepted

## Context

ML workloads pass multi-dimensional arrays with metadata (shape, strides, dtype). MAX's kernel-level tensor is `(data_ptr, shape[rank], strides[rank])`.

## Options considered

### A. No tensor types — just use MojoSlice + separate shape params
```rust
unsafe { mojo_fn(data.as_raw(), data.len() as isize, rows as isize, cols as isize) }
```
Pros: No new types.
Cons: Shape/stride info scattered across many parameters. Mojo side must parse individual args.

### B. TensorDescriptor as a repr(C) struct (chosen)
```rust
let t = Tensor::<f64>::from_data(TensorShape::matrix(2, 3), data);
let desc = t.descriptor();
unsafe { mojo_fn(desc.as_raw()) }
```
Mojo reads shape/strides/data_ptr from the descriptor at known offsets.
Pros: One pointer passes all metadata. Matches MAX's internal layout.
Cons: Descriptor contains a raw data_ptr as i64 — lifetimes don't track it.

### C. Pass shape separately, data as MojoSlice
Pros: Simpler struct.
Cons: Two parameters instead of one. Doesn't match MAX convention.

## Decision

Option B. `TensorDescriptor` is a 152-byte `#[repr(C)]` struct that passes all tensor metadata in one pointer. `Tensor<T>` owns the data and creates descriptors on demand.

## Tradeoffs

- `descriptor()` returns `DescriptorGuard<'_>` (see ADR-017) — the compiler prevents dangling `data_ptr` by tying the descriptor's lifetime to the tensor.
- `Tensor<T>` implements `Deref<[T]>` for slice access — convenient but exposes methods like `sort()` that don't respect tensor shape.
- Strides are computed for contiguous layout but no Mojo example reads them. Non-contiguous tensors are not yet supported.

## Evidence

Examples 03, 07, 11 use TensorDescriptor for sum, dot, matmul, embedding lookup, and neural layer forward pass.
