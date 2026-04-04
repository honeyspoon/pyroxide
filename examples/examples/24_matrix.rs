// ─────────────────────────────────────────────────────────
// 24: Matrix operations — transpose, hadamard, trace
// ─────────────────────────────────────────────────────────
//
// Flat row-major matrices with (rows, cols) as separate params.
// Tests the "many scalar args" pattern and data layout conventions.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn transpose_f64(src: isize, dst: isize, rows: isize, cols: isize);
    fn elementwise_mul_f64(a: isize, b: isize, out: isize, n: isize);
    fn matrix_trace_f64(addr: isize, n: isize) -> f64;
}

fn main() {
    // ── Transpose: 2×3 → 3×2 ──
    // [[1,2,3],[4,5,6]] → [[1,4],[2,5],[3,6]]
    let src = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut dst = [0.0f64; 6];
    unsafe {
        transpose_f64(
            MojoSlice::new(&src).as_raw(),
            MojoSliceMut::new(&mut dst).as_raw(),
            2,
            3,
        );
    }
    assert_eq!(dst, [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    println!("  transpose(2×3) = {dst:?} [ok]");

    // Transpose is its own inverse: transpose(transpose(A)) = A
    let mut roundtrip = [0.0f64; 6];
    unsafe {
        transpose_f64(
            MojoSlice::new(&dst).as_raw(),
            MojoSliceMut::new(&mut roundtrip).as_raw(),
            3,
            2,
        );
    }
    assert_eq!(roundtrip, src);
    println!("  transpose(transpose(A)) = A [ok]");

    // ── Hadamard (element-wise multiply) ──
    let a = [1.0, 2.0, 3.0, 4.0];
    let b = [5.0, 6.0, 7.0, 8.0];
    let mut out = [0.0f64; 4];
    unsafe {
        elementwise_mul_f64(
            MojoSlice::new(&a).as_raw(),
            MojoSlice::new(&b).as_raw(),
            MojoSliceMut::new(&mut out).as_raw(),
            4,
        );
    }
    assert_eq!(out, [5.0, 12.0, 21.0, 32.0]);
    println!("  hadamard = {out:?} [ok]");

    // ── Trace of 3×3 identity ──
    let identity = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    let tr = unsafe { matrix_trace_f64(MojoSlice::new(&identity).as_raw(), 3) };
    assert_eq!(tr, 3.0);
    println!("  trace(I₃) = {tr} [ok]");

    // Trace of arbitrary 2×2
    let m = [3.0, 1.0, 4.0, 1.5];
    let tr2 = unsafe { matrix_trace_f64(MojoSlice::new(&m).as_raw(), 2) };
    assert_eq!(tr2, 4.5); // 3.0 + 1.5
    println!("  trace([[3,1],[4,1.5]]) = {tr2} [ok]");

    println!("all ok");
}
