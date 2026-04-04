// ─────────────────────────────────────────────────────────
// 26: Multi-step pipeline — one buffer, many operations
// ─────────────────────────────────────────────────────────
//
// Same data flows through multiple Mojo calls. Also tests
// enum-like dispatch: an integer tag selects the operation.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn apply_op(addr: isize, n: isize, op: isize) -> f64;
    fn transform_inplace(addr: isize, n: isize, op: isize);
}

// Operation tags — mirrors the Mojo side
const OP_SUM: isize = 0;
const OP_PRODUCT: isize = 1;
const OP_MEAN: isize = 2;
const OP_MIN: isize = 3;
const OP_MAX: isize = 4;

const TRANSFORM_NEGATE: isize = 0;
const TRANSFORM_ABS: isize = 1;
const TRANSFORM_SQUARE: isize = 2;
const TRANSFORM_DOUBLE: isize = 3;

fn main() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let s = MojoSlice::new(&data);
    let n = data.len() as isize;

    // ── Dispatch by tag ──
    let sum = unsafe { apply_op(s.as_raw(), n, OP_SUM) };
    assert_eq!(sum, 15.0);
    println!("  apply_op(SUM) = {sum} [ok]");

    let product = unsafe { apply_op(s.as_raw(), n, OP_PRODUCT) };
    assert_eq!(product, 120.0);
    println!("  apply_op(PRODUCT) = {product} [ok]");

    let mean = unsafe { apply_op(s.as_raw(), n, OP_MEAN) };
    assert_eq!(mean, 3.0);
    println!("  apply_op(MEAN) = {mean} [ok]");

    let min = unsafe { apply_op(s.as_raw(), n, OP_MIN) };
    assert_eq!(min, 1.0);
    println!("  apply_op(MIN) = {min} [ok]");

    let max = unsafe { apply_op(s.as_raw(), n, OP_MAX) };
    assert_eq!(max, 5.0);
    println!("  apply_op(MAX) = {max} [ok]");

    // Unknown op → sentinel
    let unknown = unsafe { apply_op(s.as_raw(), n, 99) };
    assert!(unknown < -1e300);
    println!("  apply_op(unknown) = sentinel [ok]");

    // ── Pipeline: negate → abs → double ──
    let mut pipeline = vec![-3.0, 1.0, -4.0, 1.0, 5.0];
    let pn = pipeline.len() as isize;

    // Step 1: negate → [3, -1, 4, -1, -5]
    unsafe {
        transform_inplace(
            MojoSliceMut::new(&mut pipeline).as_raw(),
            pn,
            TRANSFORM_NEGATE,
        )
    };
    assert_eq!(pipeline, [3.0, -1.0, 4.0, -1.0, -5.0]);
    println!("  negate → {pipeline:?} [ok]");

    // Step 2: abs → [3, 1, 4, 1, 5]
    unsafe { transform_inplace(MojoSliceMut::new(&mut pipeline).as_raw(), pn, TRANSFORM_ABS) };
    assert_eq!(pipeline, [3.0, 1.0, 4.0, 1.0, 5.0]);
    println!("  abs → {pipeline:?} [ok]");

    // Step 3: square → [9, 1, 16, 1, 25]
    unsafe {
        transform_inplace(
            MojoSliceMut::new(&mut pipeline).as_raw(),
            pn,
            TRANSFORM_SQUARE,
        )
    };
    assert_eq!(pipeline, [9.0, 1.0, 16.0, 1.0, 25.0]);
    println!("  square → {pipeline:?} [ok]");

    // Step 4: sum the squared values
    let sum_sq = unsafe { apply_op(MojoSlice::new(&pipeline).as_raw(), pn, OP_SUM) };
    assert_eq!(sum_sq, 52.0);
    println!("  sum(squared) = {sum_sq} [ok]");

    println!("all ok");
}
