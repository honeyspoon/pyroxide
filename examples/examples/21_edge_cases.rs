// ─────────────────────────────────────────────────────────
// 21: Edge cases — zero-length, NaN in structs, byte roundtrip
// ─────────────────────────────────────────────────────────
//
// Tests the boundaries of the FFI: what happens with empty data,
// special float values inside structs, and byte-level struct copies.

use pyroxide::prelude::*;

mojo_type! {
    /// Struct with special float values to test round-trip.
    pub struct FloatEdges {
        pub normal: f64,
        pub nan: f64,
        pub inf: f64,
        pub neg_inf: f64,
        pub neg_zero: f64,
    }
}

unsafe extern "C" {
    fn sum_or_zero(addr: isize, n: isize) -> f64;
    fn count_nans(addr: isize, n: isize) -> isize;
    fn roundtrip_struct(src: isize, dst: isize, n_bytes: isize);
}

fn main() {
    // ── Zero-length slice ──
    let empty: &[f64] = &[];
    let result = unsafe { sum_or_zero(empty.as_ptr() as isize, 0) };
    assert_eq!(result, 0.0);
    println!("  sum_or_zero(empty) = 0.0 [ok]");

    // Normal case still works
    let data = [1.0, 2.0, 3.0];
    let result2 = unsafe { sum_or_zero(MojoSlice::new(&data).as_raw(), 3) };
    assert_eq!(result2, 6.0);
    println!("  sum_or_zero([1,2,3]) = 6.0 [ok]");

    // ── NaN counting ──
    let nans = [1.0, f64::NAN, 3.0, f64::NAN, f64::NAN, 6.0];
    let count = unsafe { count_nans(MojoSlice::new(&nans).as_raw(), nans.len() as isize) };
    assert_eq!(count, 3);
    println!("  count_nans = {count} [ok]");

    // Zero NaNs
    let no_nans = [1.0, 2.0, 3.0];
    let count2 = unsafe { count_nans(MojoSlice::new(&no_nans).as_raw(), 3) };
    assert_eq!(count2, 0);
    println!("  count_nans(no NaNs) = 0 [ok]");

    // ── Struct byte roundtrip with special values ──
    let src = FloatEdges {
        normal: 42.0,
        nan: f64::NAN,
        inf: f64::INFINITY,
        neg_inf: f64::NEG_INFINITY,
        neg_zero: -0.0,
    };
    let mut dst = FloatEdges {
        normal: 0.0,
        nan: 0.0,
        inf: 0.0,
        neg_inf: 0.0,
        neg_zero: 0.0,
    };

    let size = std::mem::size_of::<FloatEdges>() as isize;
    unsafe { roundtrip_struct(src.as_raw(), dst.as_raw_mut(), size) };

    assert_eq!(dst.normal, 42.0);
    assert!(dst.nan.is_nan());
    assert_eq!(dst.inf, f64::INFINITY);
    assert_eq!(dst.neg_inf, f64::NEG_INFINITY);
    assert_eq!(dst.neg_zero.to_bits(), (-0.0f64).to_bits());
    println!("  struct roundtrip (normal, NaN, Inf, -Inf, -0.0) [ok]");

    // Verify byte-for-byte equality (except NaN which has multiple representations)
    let src_bytes = zerocopy::IntoBytes::as_bytes(&src);
    let dst_bytes = zerocopy::IntoBytes::as_bytes(&dst);
    assert_eq!(src_bytes, dst_bytes, "byte-level mismatch");
    println!("  byte-level equality [ok]");

    println!("all ok");
}
