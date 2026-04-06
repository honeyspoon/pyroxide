# Architecture Decision Records

Each ADR documents a design choice, what alternatives were considered, and why we chose what we chose.

| ADR | Decision |
|-----|----------|
| [001](001-pointer-representation.md) | `.as_raw()` returns plain `isize` |
| [002](002-struct-layout.md) | `mojo_type!` macro for `#[repr(C)]` + zerocopy |
| [003](003-slice-handles.md) | `MojoSlice` + `MojoSliceMut` mirrors `&[T]` / `&mut [T]` |
| [004](004-string-passing.md) | `MojoStr` for `(ptr, len)` text passing |
| [005](005-multiple-returns.md) | `OutSlot<T>` — composable `MaybeUninit` pattern |
| [006](006-panic-safety.md) | `catch_panic_at_ffi` catches Rust panics only |
| [007](007-error-handling.md) | Error sentinels (NaN, -1), no framework error type |
| [008](008-tensor-types.md) | `TensorDescriptor` + `DescriptorGuard` |
| [009](009-conversion-traits.md) | `IntoMojo`/`FromMojo` blanket impls, unsealed |
| [010](010-no-safe-wrappers.md) | Users write `unsafe extern "C"` |
| [011](011-removed-abstractions.md) | Record: MojoAddr, MojoArg, MojoResult, MojoRef, MojoMut, OutParam |
| [012](012-build-system.md) | `build.rs` compiles .mojo files via cargo |
| [013](013-opinionless.md) | Map conventions 1:1, don't invent abstractions |
| [014](014-macro-asymmetry.md) | `mojo_type!` exists, `mojo_fn!` doesn't |
| [015](015-mojoref-removal.md) | `MojoRef`/`MojoMut` removed — `&T` + `as_raw()` is sufficient |
| [016](016-handle-thread-safety.md) | `PhantomData<*const ()>` for `!Send`/`!Sync` on slices |
| [017](017-descriptor-guard.md) | `DescriptorGuard` with `Deref` — `MutexGuard` pattern |
| [018](018-thread-safety-empirics.md) | Pure `@export` is thread-safe, `parallelize` segfaults |
| [019](019-arc-tensor-deferred.md) | `ArcTensor` deferred to 0.2.0 |
