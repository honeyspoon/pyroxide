// ─────────────────────────────────────────────────────────
// 04: Mojo's explicit SIMD — hardware vectors, no guesswork
// ─────────────────────────────────────────────────────────
//
// Mojo lets you write SIMD[DType.float32, 8] — an 8-wide vector
// mapped to hardware registers. No auto-vectorization guesswork.
//
// We compare scalar vs SIMD dot product and benchmark the speedup.

use pyroxide::bridge::MojoSlice;
use std::hint::black_box;
use std::time::Instant;

unsafe extern "C" {
    fn dot_scalar(a: isize, b: isize, n: isize) -> f32;
    fn dot_simd(a: isize, b: isize, n: isize) -> f32;
}

/// Call a Mojo dot function using typed `MojoSlice` handles.
fn mojo_dot(f: unsafe extern "C" fn(isize, isize, isize) -> f32, a: &[f32], b: &[f32]) -> f32 {
    let sa = MojoSlice::new(a);
    let sb = MojoSlice::new(b);
    unsafe { f(sa.as_raw(), sb.as_raw(), sa.len() as isize) }
}

fn main() {
    // ── Correctness: small vectors ──
    let a: Vec<f32> = (1..=10).map(|i| i as f32).collect();
    let b: Vec<f32> = (1..=10).rev().map(|i| i as f32).collect();
    assert_eq!(mojo_dot(dot_scalar, &a, &b), 220.0);
    assert_eq!(mojo_dot(dot_simd, &a, &b), 220.0);
    println!("  dot(small): scalar=220, simd=220 [ok]");

    // ── Correctness: non-aligned size (tests the scalar tail loop) ──
    let n = 1003;
    let a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.01).collect();
    let b: Vec<f32> = (0..n).map(|i| ((n - i) as f32) * 0.01).collect();
    let scalar = mojo_dot(dot_scalar, &a, &b);
    let simd = mojo_dot(dot_simd, &a, &b);
    assert!((scalar - simd).abs() / scalar.abs() < 1e-4);
    println!("  dot(n={n}): scalar={scalar:.2}, simd={simd:.2} [ok]");

    // ── Benchmark ──
    let n = 100_000;
    let a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.001).collect();
    let b: Vec<f32> = (0..n).map(|i| ((n - i) as f32) * 0.001).collect();

    let bench = |f: unsafe extern "C" fn(isize, isize, isize) -> f32| -> u64 {
        for _ in 0..100 {
            black_box(mojo_dot(f, &a, &b));
        }
        let start = Instant::now();
        for _ in 0..1000 {
            black_box(mojo_dot(f, &a, &b));
        }
        start.elapsed().as_nanos() as u64 / 1000
    };

    let ns_scalar = bench(dot_scalar);
    let ns_simd = bench(dot_simd);
    println!("\n  dot(n={n}):");
    println!("    scalar: {ns_scalar}ns");
    println!("    simd:   {ns_simd}ns");
    println!(
        "    speedup: {:.1}x",
        ns_scalar as f64 / ns_simd.max(1) as f64
    );

    println!("all ok");
}
