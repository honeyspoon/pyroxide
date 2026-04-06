# ADR-017: DescriptorGuard (MutexGuard pattern for tensor descriptors)

## Status: Accepted

## Context

`Tensor::descriptor()` returns a `TensorDescriptor` containing a `data_ptr` that points into the tensor's `Vec<T>`. If the tensor is dropped, the descriptor's pointer dangles — UB.

## Options considered

### A. `BorrowedDescriptor` with manual forwarding
The first attempt. Had `.as_raw()` and `.inner()` methods. Users had to call `.inner().dtype` to access fields. Verbose.

### B. Closure-based API: `tensor.with_descriptor(|desc| { ... })`
Python-style `with` pattern. Un-Rustic — Rust uses guards, not closures, for scoped access.

### C. `DescriptorGuard<'_>` with `Deref<Target = TensorDescriptor>` (chosen)
Follows the `MutexGuard` pattern: holds the borrow, derefs to the inner type. `desc.dtype` works via auto-deref. `desc.as_raw()` works via `IntoMojo` on `TensorDescriptor` through Deref.

## Decision

Option C. `DescriptorGuard<'_>` owns the descriptor value while borrowing the lifetime from the tensor. The compiler prevents:
```rust
let desc = { let t = Tensor::zeros(...); t.descriptor() };
// error: t does not live long enough
```

## Future: ArcTensor

For model weights loaded once and used forever, fighting a borrow lifetime is painful. The answer is `ArcTensor<T>` wrapping `Arc<[T]>` — the data lives as long as any clone exists. Deferred to 0.2.0 (see ADR-019).
