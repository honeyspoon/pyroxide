// ─────────────────────────────────────────────────────────
// 23: Chained calls + error sentinels
// ─────────────────────────────────────────────────────────
//
// Output of one Mojo call feeds into the next. Also tests
// NaN and -1 as error sentinels — the Mojo convention for
// signaling errors without raises.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn normalize_f64(addr: isize, n: isize) -> f64;
    fn argmax_f64(addr: isize, n: isize) -> isize;
    fn histogram_u8(data: isize, n: isize, bins: isize);
}

fn main() {
    // ── Chain: normalize → argmax ──
    let mut data = vec![10.0, 50.0, 30.0, 90.0, 20.0];
    let original_max = unsafe { normalize_f64(MojoSliceMut::new(&mut data).as_raw(), 5) };
    assert_eq!(original_max, 90.0);

    // After normalize: values should be in [0,1]
    assert!(data.iter().all(|&x| (0.0..=1.0).contains(&x)));
    assert_eq!(data[3], 1.0); // 90 was max → 1.0
    assert_eq!(data[0], 0.0); // 10 was min → 0.0
    println!("  normalize → [0,1] range [ok]");

    // Argmax on normalized data
    let idx = unsafe { argmax_f64(MojoSlice::new(&data).as_raw(), 5) };
    assert_eq!(idx, 3); // element 3 was the max
    println!("  argmax(normalized) = {idx} [ok]");

    // ── Error sentinel: NaN for degenerate input ──
    let mut all_same = vec![5.0, 5.0, 5.0];
    let result = unsafe { normalize_f64(MojoSliceMut::new(&mut all_same).as_raw(), 3) };
    assert!(result.is_nan(), "all-same should return NaN");
    println!("  normalize(all-same) = NaN [ok]");

    // Empty returns NaN too
    let result2 = unsafe { normalize_f64(std::ptr::null_mut::<f64>() as isize, 0) };
    assert!(result2.is_nan());
    println!("  normalize(empty) = NaN [ok]");

    // ── Error sentinel: -1 for empty argmax ──
    let idx2 = unsafe { argmax_f64(std::ptr::null::<f64>() as isize, 0) };
    assert_eq!(idx2, -1);
    println!("  argmax(empty) = -1 [ok]");

    // ── Histogram ──
    let bytes: Vec<u8> = vec![0, 1, 1, 2, 2, 2, 255, 255];
    let mut bins = vec![0i64; 256];
    unsafe {
        histogram_u8(
            MojoSlice::new(&bytes).as_raw(),
            bytes.len() as isize,
            MojoSliceMut::new(&mut bins).as_raw(),
        );
    }
    assert_eq!(bins[0], 1);
    assert_eq!(bins[1], 2);
    assert_eq!(bins[2], 3);
    assert_eq!(bins[255], 2);
    assert_eq!(bins[128], 0);
    let total: i64 = bins.iter().sum();
    assert_eq!(total, bytes.len() as i64);
    println!(
        "  histogram: [0]={}, [1]={}, [2]={}, [255]={}, total={total} [ok]",
        bins[0], bins[1], bins[2], bins[255]
    );

    println!("all ok");
}
