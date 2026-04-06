# ADR-019: ArcTensor deferred to 0.2.0

## Status: Deferred

## Context

`TensorView<'a, T>` borrows `&'a [T]`. For model weights loaded once and used for the entire model lifetime, fighting a borrow lifetime is painful. The user's code must thread `'a` through every function that touches the weights.

## Proposed solution

`ArcTensor<T>` wrapping `Arc<[T]>` + `TensorShape`. The data lives as long as any clone exists. Can return `TensorDescriptor` by value safely because `Arc` guarantees the backing data won't be freed.

```rust
let weights = ArcTensor::<f32>::from_data(shape, data);
let clone = weights.clone(); // Arc clone, not data copy
let desc = clone.descriptor(); // safe — Arc keeps data alive
```

## Why deferred

1. No real usage yet to validate the design
2. Unsoundness question: can a caller drop all Arc clones and still hold a descriptor? With `DescriptorGuard<'_>` this is prevented, but ArcTensor's descriptor would need its own lifetime story
3. Cost: one atomic operation per clone/drop — negligible for weights, but adds complexity

## Criteria for 0.2.0

Add when: a real user loads safetensors weights and needs to share them across multiple inference calls without lifetime threading.
