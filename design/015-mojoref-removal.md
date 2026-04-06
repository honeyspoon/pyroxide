# ADR-015: MojoRef/MojoMut removal

## Status: Accepted

## Context

`MojoRef<'a, T>` and `MojoMut<'a, T>` were wrapper types that held a `NonNull<T>` plus `PhantomData<&'a T>` to tie the pointer's lifetime to its source. They were removed in the stdlib-idioms refactor.

## Why they were unnecessary

The invariant they enforced — that the pointer doesn't outlive its source — is already enforced by the `&'a T` / `&'a mut T` passed into `IntoMojo::as_raw()` / `FromMojo::as_raw_mut()`. The borrow checker tracks the lifetime of the underlying reference, not the wrapper.

The `!Send + !Sync` via `PhantomData<*const ()>` was fixing a problem that only existed because the wrapper introduced a named type that could be moved across threads. With `val.as_raw()` returning a bare `isize`, there's no type to accidentally send — the `unsafe` at the call site IS the safety boundary.

## Comparison with PyO3

PyO3's `Bound<'py, T>` is necessary because Python objects need a GIL token (`Python<'py>`) to be valid. The `'py` lifetime ties the object to a specific GIL acquisition. Mojo has no equivalent runtime token — a plain Rust borrow is sufficient.

## Decision

Remove `MojoRef` and `MojoMut`. `IntoMojo::as_raw()` on `&T` and `FromMojo::as_raw_mut()` on `&mut T` provide the same functionality with zero wrapper types.

## Evidence

All 31 examples work with `.as_raw()` directly. No example ever called `.as_mojo()` — they all used `.as_raw()`.
