# Architecture Decision Records

Each ADR documents a design choice, what alternatives were considered, and why we chose what we chose.

| ADR | Decision |
|-----|----------|
| [001](001-pointer-representation.md) | `.as_raw()` returns plain `isize` — MojoAddr newtype tried and removed |
| [002](002-struct-layout.md) | `mojo_type!` macro for `#[repr(C)]` + zerocopy, explicit padding required |
| [003](003-slice-handles.md) | `MojoSlice` + `MojoSliceMut` — mirrors `&[T]` / `&mut [T]` |
| [004](004-string-passing.md) | `MojoStr` — dedicated type for `(ptr, len)` text passing |
| [005](005-multiple-returns.md) | `OutParam::call*` — unsafe fn with MaybeUninit for out-pointers |
| [006](006-panic-safety.md) | `catch_mojo_call` — catch_unwind at FFI boundary |
| [007](007-error-handling.md) | Error sentinels (NaN, -1) — no framework-level error type |
| [008](008-tensor-types.md) | `TensorDescriptor` — 152-byte repr(C) struct matching MAX layout |
| [009](009-conversion-traits.md) | `IntoMojo`/`FromMojo` — blanket impls on zerocopy types |
| [010](010-no-safe-wrappers.md) | Users write `unsafe extern "C"` — no generated wrappers for 0.1.0 |
| [011](011-removed-abstractions.md) | Record of tried-and-removed: MojoAddr, MojoArg, MojoResult, mojo_import! |
| [012](012-build-system.md) | `build.rs` compiles .mojo files automatically via cargo |
| [013](013-opinionless.md) | Core principle: map conventions 1:1, don't invent abstractions |
