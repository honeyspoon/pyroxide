# ADR-012: Build system — cargo build.rs compiles Mojo

## Status: Accepted

## Context

Mojo files must be compiled to shared libraries (`.dylib`/`.so`) before Rust can link against them.

## Options considered

### A. Makefiles per example
Tried first (PRs 0-early). Each example had its own Makefile. Verbose, un-Rustic.

### B. Single `build.rs` that compiles all .mojo files (chosen)
```rust
// examples/build.rs
for entry in read_dir("mojo/") {
    mojo build --emit shared-lib -o lib{stem}.dylib {path}
    println!("cargo:rustc-link-lib=dylib={stem}");
}
```
Pros: `cargo run --example foo` just works. No manual steps. Recompiles on source change (`cargo:rerun-if-changed`).
Cons: Requires Mojo installed. CI can't run examples (only library checks).

### C. Pre-compiled .dylib files checked into git
Pros: No Mojo dependency for consumers.
Cons: Platform-specific binaries in git. Stale.

### D. Feature flag to skip Mojo compilation
Pros: CI-friendly.
Cons: Adds complexity.

## Decision

Option B. The `build.rs` finds `mojo` via: `MOJO_PATH` env → `~/.pixi/bin/mojo` → `~/.modular/bin/mojo` → PATH. CI runs library checks only (no Mojo needed). Examples require local Mojo installation.

## Evidence

All 26 examples compile and run via `cargo run -p pyroxide-examples --example X` with no manual steps beyond having Mojo installed.
