// ─────────────────────────────────────────────────────────
// 17: Mixed argument types — scalars, pointers, bools
// ─────────────────────────────────────────────────────────
//
// One Mojo function taking 6 arguments of different types.
// Tests the full ABI surface in a single call.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn weighted_blend(
        src_a: isize,
        src_b: isize,
        dst: isize,
        n: isize,
        weight: f64,
        clamp: bool,
    ) -> f64;

    fn find_threshold(data: isize, n: isize, threshold: f64, above: bool) -> isize;
}

fn main() {
    // ── Weighted blend ──
    let a = [1.0, 0.5, 0.0, 0.8];
    let b = [0.0, 0.5, 1.0, 0.2];
    let mut dst = [0.0f64; 4];

    // 50/50 blend, no clamp
    let max_val = unsafe {
        weighted_blend(
            MojoSlice::new(&a).as_raw(),
            MojoSlice::new(&b).as_raw(),
            MojoSliceMut::new(&mut dst).as_raw(),
            4,
            0.5,
            false,
        )
    };
    // 50/50 blend: each element should be average
    assert!((dst[0] - 0.5).abs() < 1e-10);
    assert!((dst[1] - 0.5).abs() < 1e-10);
    assert!((dst[2] - 0.5).abs() < 1e-10);
    assert!((dst[3] - 0.5).abs() < 1e-10);
    println!("  blend(50/50) = {dst:?} max={max_val} [ok]");

    // Full weight on A, with clamp
    let a2 = [1.5, -0.5, 0.7];
    let b2 = [0.0, 0.0, 0.0];
    let mut dst2 = [0.0f64; 3];
    let max2 = unsafe {
        weighted_blend(
            MojoSlice::new(&a2).as_raw(),
            MojoSlice::new(&b2).as_raw(),
            MojoSliceMut::new(&mut dst2).as_raw(),
            3,
            1.0,
            true, // clamp to [0,1]
        )
    };
    assert_eq!(dst2[0], 1.0); // 1.5 clamped to 1.0
    assert_eq!(dst2[1], 0.0); // -0.5 clamped to 0.0
    assert!((dst2[2] - 0.7).abs() < 1e-10);
    assert!((max2 - 1.0).abs() < 1e-10);
    println!("  blend(100/0, clamp) = {dst2:?} [ok]");

    // ── Find threshold ──
    let data = [0.1, 0.5, 0.9, 0.3, 0.7];
    let s = MojoSlice::new(&data);

    // First element above 0.8
    let idx = unsafe { find_threshold(s.as_raw(), s.len_isize(), 0.8, true) };
    assert_eq!(idx, 2); // data[2] = 0.9 > 0.8
    println!("  find_threshold(>0.8) = {idx} [ok]");

    // First element below 0.2
    let idx2 = unsafe { find_threshold(s.as_raw(), s.len_isize(), 0.2, false) };
    assert_eq!(idx2, 0); // data[0] = 0.1 < 0.2
    println!("  find_threshold(<0.2) = {idx2} [ok]");

    // Not found → -1
    let idx3 = unsafe { find_threshold(s.as_raw(), s.len_isize(), 2.0, true) };
    assert_eq!(idx3, -1);
    println!("  find_threshold(>2.0) = {idx3} (not found) [ok]");

    println!("all ok");
}
