// ─────────────────────────────────────────────────────────
// 02: Passing structs across the FFI boundary
// ─────────────────────────────────────────────────────────
//
// Mojo functions receive raw memory addresses. Rust guarantees
// the layout with `#[repr(C)]` via the `mojo_type!{}` macro.
//
// The pattern:
//   1. Define a struct with `mojo_type!{}`
//   2. Call `.as_raw()` to get a pointer as `isize`
//   3. Pass it to Mojo — Mojo reads fields at known byte offsets
//
// For mutation, use `.as_raw_mut()` instead. Mojo writes
// directly into Rust's memory — zero copies either way.

use pyroxide::prelude::*;

// Define a custom type. `mojo_type!` adds #[repr(C)] and all
// zerocopy derives, so the struct automatically implements
// IntoMojo (→ .as_mojo()) and FromMojo (→ .as_mojo_mut()).
mojo_type! {
    pub struct Color {
        pub r: f64,
        pub g: f64,
        pub b: f64,
    }
}

// Declare Mojo functions. Each takes an `isize` — the raw address
// of the struct in Rust's memory. Mojo accesses fields by offset:
//   Point:  x=offset(0), y=offset(8)
//   Color:  r=offset(0), g=offset(8), b=offset(16)
unsafe extern "C" {
    fn point_distance(a: isize, b: isize) -> f64;
    fn color_luminance(c: isize) -> f64;
    fn color_invert(c: isize); // mutates in-place
}

fn main() {
    // Read: pass two Points, get distance back
    let a = Point::new(3.0, 4.0);
    let b = Point::ORIGIN;
    let dist = unsafe { point_distance(a.as_raw(), b.as_raw()) };
    assert!((dist - a.distance(&b)).abs() < 1e-6);
    println!("  point_distance = {dist:.6} [ok]");

    // Read: pass a Color, get luminance back
    let sky = Color {
        r: 0.4,
        g: 0.7,
        b: 1.0,
    };
    let lum = unsafe { color_luminance(sky.as_raw()) };
    let expected = 0.0722f64.mul_add(1.0, 0.2126f64.mul_add(0.4, 0.7152 * 0.7));
    assert!((lum - expected).abs() < 1e-6);
    println!("  color_luminance = {lum:.4} [ok]");

    // Mutate: Mojo inverts the color channels in-place
    let mut c = Color {
        r: 0.2,
        g: 0.8,
        b: 0.5,
    };
    unsafe { color_invert(c.as_raw_mut()) };
    assert!((c.r - 0.8).abs() < 1e-10);
    assert!((c.g - 0.2).abs() < 1e-10);
    assert!((c.b - 0.5).abs() < 1e-10);
    println!("  color_invert = ({:.1}, {:.1}, {:.1}) [ok]", c.r, c.g, c.b);

    println!("all ok");
}
