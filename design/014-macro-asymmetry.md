# ADR-014: Why mojo_type! exists but mojo_fn! doesn't

## Status: Accepted

## Context

pyroxide has `mojo_type!` for declaring FFI-safe structs but no corresponding `mojo_fn!` for declaring FFI functions. This asymmetry was questioned.

## Options considered

### A. Remove mojo_type!, go fully explicit on both sides

Types:
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, IntoBytes, FromBytes, Immutable, KnownLayout)]
pub struct Point { pub x: f64, pub y: f64 }
```

Functions:
```rust
unsafe extern "C" { fn point_distance(a: isize, b: isize) -> f64; }
```

Pros: Symmetric — both sides explicit.
Cons: 6 derives to remember per struct. Easy to forget one and get a confusing error. The derive list IS the type's FFI contract — hiding it in a macro is documenting intent.

### B. Add mojo_fn! to match mojo_type!

```rust
mojo_fn! { fn point_distance(a: &Point, b: &Point) -> f64; }
```

Tried as `mojo_import!` and removed (ADR-010). The function side has challenges the type side doesn't:
- Mixed argument types (scalars pass directly, structs need `.as_raw()`, slices need `.as_raw()` + length)
- `&mut T` arguments need different handling than `&T`
- Return types vary (scalar, void, sentinel)
- The macro would hide `unsafe` — dishonest about FFI being unsafe

### C. Keep mojo_type!, no mojo_fn! (chosen)

The asymmetry is justified because the two sides solve different problems:

**Types**: The problem is 6 mandatory derives that are hard to remember and easy to get wrong. `mojo_type!` solves this. The macro IS the type's documentation — when you read `mojo_type!` you instantly know "this crosses FFI."

**Functions**: The problem is just `unsafe extern "C" { ... }`. This is 1 line, uses standard Rust syntax, and every Rust developer recognizes it. There's nothing to simplify. A macro would hide the `unsafe`, which is dishonest.

## Decision

Option C. The asymmetry reflects a real difference in complexity:

| Side | Without macro | With macro | Macro value |
|------|--------------|------------|-------------|
| Types | 8 lines of derives | 1 line | High — prevents errors |
| Functions | 1 line `unsafe extern "C"` | 1 line macro | Zero — same length, hides unsafe |

A macro should exist only when it prevents errors or reduces real complexity. `mojo_type!` does both. `mojo_fn!` would do neither.
