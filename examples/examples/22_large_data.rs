// ─────────────────────────────────────────────────────────
// 22: Large data — 1M+ element arrays
// ─────────────────────────────────────────────────────────
//
// Does the FFI hold up with large buffers? Tests dot product,
// scale-add, and reduce-max on million-element arrays.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};
use std::hint::black_box;
use std::time::Instant;

unsafe extern "C" {
    fn dot_f64_large(a: isize, b: isize, n: isize) -> f64;
    fn scale_add_f64(x: isize, y: isize, out: isize, n: isize, alpha: f64, beta: f64);
    fn reduce_max_f64(addr: isize, n: isize) -> f64;
}

fn main() {
    let n = 1_000_000;
    let a: Vec<f64> = (0..n).map(|i| (i as f64) * 0.001).collect();
    let b: Vec<f64> = (0..n).map(|i| ((n - i) as f64) * 0.001).collect();

    // ── Dot product ──
    let mojo_dot = unsafe {
        dot_f64_large(
            MojoSlice::new(&a).as_raw(),
            MojoSlice::new(&b).as_raw(),
            n as isize,
        )
    };
    let rust_dot: f64 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
    assert!(
        (mojo_dot - rust_dot).abs() / rust_dot.abs() < 1e-10,
        "dot mismatch: mojo={mojo_dot} rust={rust_dot}"
    );
    println!("  dot(1M) mojo={mojo_dot:.4} rust={rust_dot:.4} [ok]");

    // ── Scale-add: out = 2.0*a + 0.5*b ──
    let mut out = vec![0.0f64; n];
    unsafe {
        scale_add_f64(
            MojoSlice::new(&a).as_raw(),
            MojoSlice::new(&b).as_raw(),
            MojoSliceMut::new(&mut out).as_raw(),
            n as isize,
            2.0,
            0.5,
        );
    }
    // Check a few elements
    for &i in &[0, n / 2, n - 1] {
        let expected = 2.0 * a[i] + 0.5 * b[i];
        assert!(
            (out[i] - expected).abs() < 1e-10,
            "scale_add[{i}]: {}, expected {}",
            out[i],
            expected
        );
    }
    println!("  scale_add(1M) spot-checked [ok]");

    // ── Reduce max ──
    let max_val = unsafe { reduce_max_f64(MojoSlice::new(&a).as_raw(), n as isize) };
    let rust_max = a.iter().copied().reduce(f64::max).expect("non-empty");
    assert_eq!(max_val, rust_max);
    println!("  reduce_max(1M) = {max_val} [ok]");

    // ── Benchmark ──
    let start = Instant::now();
    for _ in 0..10 {
        black_box(unsafe {
            dot_f64_large(
                MojoSlice::new(&a).as_raw(),
                MojoSlice::new(&b).as_raw(),
                n as isize,
            )
        });
    }
    let elapsed = start.elapsed() / 10;
    println!("  dot(1M) avg: {elapsed:?}");

    println!("all ok");
}
