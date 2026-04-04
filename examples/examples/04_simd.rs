// ─────────────────────────────────────────────────────────
// 04: Mojo's explicit SIMD — hardware vectors, no guesswork
// ─────────────────────────────────────────────────────────
//
// This is Mojo's killer feature. Where Rust (and C/C++) rely on
// the compiler to *maybe* auto-vectorize your loops, Mojo lets
// you write `SIMD[DType.float32, 8]` directly — an 8-wide vector
// that maps to a real hardware SIMD register.
//
// The Mojo side (mojo/simd.mojo) exports two versions of dot product:
//   - dot_scalar: one element at a time (for i in range(n))
//   - dot_simd:   8 elements at a time using SIMD load/multiply/accumulate
//
// We verify they produce the same results, then benchmark the speedup.

use std::hint::black_box;
use std::time::Instant;

unsafe extern "C" {
    fn dot_scalar(a: isize, b: isize, n: isize) -> f32;
    fn dot_simd(a: isize, b: isize, n: isize) -> f32;
}

fn main() {
    // ── Correctness: small vectors, exact match ──
    let a: Vec<f32> = (1..=10).map(|i| i as f32).collect();
    let b: Vec<f32> = (1..=10).rev().map(|i| i as f32).collect();
    let scalar = unsafe { dot_scalar(a.as_ptr() as isize, b.as_ptr() as isize, 10) };
    let simd = unsafe { dot_simd(a.as_ptr() as isize, b.as_ptr() as isize, 10) };
    assert_eq!(scalar, 220.0);
    assert_eq!(simd, 220.0);
    println!("  dot(small): scalar={scalar}, simd={simd} [ok]");

    // ── Correctness: non-aligned size (tests the scalar tail loop) ──
    let n = 1003; // not divisible by 8
    let a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.01).collect();
    let b: Vec<f32> = (0..n).map(|i| ((n - i) as f32) * 0.01).collect();
    let scalar = unsafe { dot_scalar(a.as_ptr() as isize, b.as_ptr() as isize, n as isize) };
    let simd = unsafe { dot_simd(a.as_ptr() as isize, b.as_ptr() as isize, n as isize) };
    assert!((scalar - simd).abs() / scalar.abs() < 1e-4);
    println!("  dot(n={n}): scalar={scalar:.2}, simd={simd:.2} [ok]");

    // ── Benchmark: scalar vs SIMD ──
    let n = 100_000;
    let a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.001).collect();
    let b: Vec<f32> = (0..n).map(|i| ((n - i) as f32) * 0.001).collect();

    let bench = |f: &dyn Fn() -> f32| -> u64 {
        for _ in 0..100 { black_box(f()); }
        let start = Instant::now();
        for _ in 0..1000 { black_box(f()); }
        start.elapsed().as_nanos() as u64 / 1000
    };

    let ns_scalar = bench(&|| unsafe { dot_scalar(a.as_ptr() as isize, b.as_ptr() as isize, n as isize) });
    let ns_simd = bench(&|| unsafe { dot_simd(a.as_ptr() as isize, b.as_ptr() as isize, n as isize) });
    println!("\n  dot(n={n}):");
    println!("    scalar: {ns_scalar}ns");
    println!("    simd:   {ns_simd}ns");
    println!("    speedup: {:.1}x", ns_scalar as f64 / ns_simd.max(1) as f64);

    println!("all ok");
}
