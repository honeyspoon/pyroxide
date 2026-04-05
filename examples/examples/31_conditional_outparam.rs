// ─────────────────────────────────────────────────────────
// 31: Conditional out-parameters — the OutParam soundness test
// ─────────────────────────────────────────────────────────
//
// OutParam::call* uses MaybeUninit and is UB if Mojo doesn't write.
// The safe pattern: Mojo ALWAYS writes to the out-param (sentinel
// if not found). This example tests both found and not-found paths.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn find_first_above(data: isize, n: isize, threshold: f64, result: isize) -> bool;
    fn try_divide(a: f64, b: f64, result: isize) -> bool;
}

fn main() {
    // ── find_first_above: found ──
    let data = [0.1, 0.5, 0.9, 0.3];
    let mut idx: i64 = -99;
    let found = unsafe {
        find_first_above(
            MojoSlice::new(&data).as_raw(),
            4,
            0.8,
            MojoSliceMut::new(&mut std::slice::from_mut(&mut idx)).as_raw(),
        )
    };
    assert!(found);
    assert_eq!(idx, 2); // data[2] = 0.9 > 0.8
    println!("  find_first_above(>0.8) = index {idx} [ok]");

    // ── find_first_above: not found → sentinel -1 ──
    let mut idx2: i64 = -99;
    let found2 = unsafe {
        find_first_above(
            MojoSlice::new(&data).as_raw(),
            4,
            2.0,
            MojoSliceMut::new(&mut std::slice::from_mut(&mut idx2)).as_raw(),
        )
    };
    assert!(!found2);
    assert_eq!(idx2, -1); // sentinel written even on failure
    println!("  find_first_above(>2.0) = not found, sentinel={idx2} [ok]");

    // ── try_divide: success ──
    let mut result: f64 = -99.0;
    let ok = unsafe {
        try_divide(
            10.0,
            3.0,
            MojoSliceMut::new(&mut std::slice::from_mut(&mut result)).as_raw(),
        )
    };
    assert!(ok);
    assert!((result - 10.0 / 3.0).abs() < 1e-10);
    println!("  try_divide(10, 3) = {result:.6} [ok]");

    // ── try_divide: division by zero → writes 0.0 sentinel ──
    let mut result2: f64 = -99.0;
    let ok2 = unsafe {
        try_divide(
            10.0,
            0.0,
            MojoSliceMut::new(&mut std::slice::from_mut(&mut result2)).as_raw(),
        )
    };
    assert!(!ok2);
    assert_eq!(result2, 0.0); // sentinel written
    println!("  try_divide(10, 0) = false, sentinel={result2} [ok]");

    println!("all ok");
}
