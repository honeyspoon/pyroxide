// ─────────────────────────────────────────────────────────
// 31: Conditional out-parameters — the OutSlot soundness test
// ─────────────────────────────────────────────────────────
//
// OutSlot uses MaybeUninit — UB if Mojo doesn't write.
// Safe pattern: Mojo ALWAYS writes (sentinel if not found).

use pyroxide::bridge::{MojoSlice, OutSlot};

unsafe extern "C" {
    fn find_first_above(data: isize, n: isize, threshold: f64, result: isize) -> bool;
    fn try_divide(a: f64, b: f64, result: isize) -> bool;
}

fn main() {
    // ── find_first_above: found ──
    let data = [0.1, 0.5, 0.9, 0.3];
    let mut idx = OutSlot::<i64>::uninit();
    let found = unsafe { find_first_above(MojoSlice::new(&data).as_raw(), 4, 0.8, idx.as_raw()) };
    let idx = unsafe { idx.assume_init() };
    assert!(found);
    assert_eq!(idx, 2);
    println!("  find(>0.8) = index {idx} [ok]");

    // ── find_first_above: not found → sentinel -1 ──
    let mut idx2 = OutSlot::<i64>::uninit();
    let found2 = unsafe { find_first_above(MojoSlice::new(&data).as_raw(), 4, 2.0, idx2.as_raw()) };
    let idx2 = unsafe { idx2.assume_init() };
    assert!(!found2);
    assert_eq!(idx2, -1); // sentinel always written
    println!("  find(>2.0) = not found, sentinel={idx2} [ok]");

    // ── try_divide: success ──
    let mut result = OutSlot::<f64>::uninit();
    let ok = unsafe { try_divide(10.0, 3.0, result.as_raw()) };
    let result = unsafe { result.assume_init() };
    assert!(ok);
    assert!((result - 10.0 / 3.0).abs() < 1e-10);
    println!("  try_divide(10, 3) = {result:.6} [ok]");

    // ── try_divide: div by zero → sentinel 0.0 ──
    let mut result2 = OutSlot::<f64>::uninit();
    let ok2 = unsafe { try_divide(10.0, 0.0, result2.as_raw()) };
    let result2 = unsafe { result2.assume_init() };
    assert!(!ok2);
    assert_eq!(result2, 0.0);
    println!("  try_divide(10, 0) = false, sentinel={result2} [ok]");

    println!("all ok");
}
