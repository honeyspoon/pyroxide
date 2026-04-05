// ─────────────────────────────────────────────────────────
// 08: ABI edge cases — Bool, out-pointers, boundary values
// ─────────────────────────────────────────────────────────
//
// This example tests every edge case of the Mojo @export ABI
// that we've empirically verified. It serves as both a test
// and documentation of what works (and what doesn't).
//
// Tested ABI features:
//   - Bool: true/false round-trip
//   - Int: MIN/MAX boundary values
//   - Float64: NaN, ±Inf, -0.0 preservation
//   - Out-pointers: multiple return values via OutSlot
//   - In-place mutation: swap
//   - Error handling: raises caught inside @export
//
// Known ABI limitations (not tested, would crash):
//   - Float16: no stable Rust mapping
//   - raises escaping @export: segfaults (exit 139)
//   - Returning Tuple/String/List: not C-ABI compatible

use pyroxide::bridge::FromMojo;
use pyroxide::bridge::OutSlot;

unsafe extern "C" {
    fn negate(b: bool) -> bool;
    fn is_positive(x: f64) -> bool;
    fn echo_int(x: isize) -> isize;
    fn echo_f64(x: f64) -> f64;
    fn max_int() -> isize;
    fn min_int() -> isize;
    fn divmod_out(a: isize, b: isize, quot: isize, rem: isize);
    fn swap_f64(a: isize, b: isize);
    fn safe_div(a: f64, b: f64) -> f64;
}

fn main() {
    // ── Bool ABI ──
    assert!(!unsafe { negate(true) });
    assert!(unsafe { negate(false) });
    assert!(unsafe { is_positive(1.0) });
    assert!(!unsafe { is_positive(-1.0) });
    assert!(!unsafe { is_positive(0.0) });
    println!("  Bool: ok");

    // ── Int boundary values ──
    assert_eq!(unsafe { echo_int(0) }, 0);
    assert_eq!(unsafe { echo_int(-1) }, -1);
    assert_eq!(unsafe { echo_int(isize::MAX) }, isize::MAX);
    assert_eq!(unsafe { echo_int(isize::MIN) }, isize::MIN);
    assert_eq!(unsafe { max_int() }, isize::MAX);
    assert_eq!(unsafe { min_int() }, isize::MIN);
    println!("  Int (0, -1, MAX, MIN): ok");

    // ── Float64 special values ──
    assert_eq!(unsafe { echo_f64(0.0) }, 0.0);
    assert_eq!(unsafe { echo_f64(-0.0) }.to_bits(), (-0.0f64).to_bits());
    assert!(unsafe { echo_f64(f64::NAN) }.is_nan());
    assert_eq!(unsafe { echo_f64(f64::INFINITY) }, f64::INFINITY);
    assert_eq!(unsafe { echo_f64(f64::NEG_INFINITY) }, f64::NEG_INFINITY);
    assert_eq!(unsafe { echo_f64(f64::MIN) }, f64::MIN);
    assert_eq!(unsafe { echo_f64(f64::MAX) }, f64::MAX);
    assert_eq!(unsafe { echo_f64(f64::MIN_POSITIVE) }, f64::MIN_POSITIVE);
    println!("  Float64 (0, -0, NaN, ±Inf, MIN, MAX): ok");

    // ── Out-pointers via OutSlot ──
    let mut q_slot = OutSlot::<i64>::uninit();
    let mut r_slot = OutSlot::<i64>::uninit();
    unsafe { divmod_out(17, 5, q_slot.as_raw(), r_slot.as_raw()) };
    let (q, r) = unsafe { (q_slot.assume_init(), r_slot.assume_init()) };
    assert_eq!((q, r), (3, 2));
    println!("  OutSlot divmod(17, 5) = ({q}, {r}): ok");

    let mut q2_slot = OutSlot::<i64>::uninit();
    let mut r2_slot = OutSlot::<i64>::uninit();
    unsafe { divmod_out(-7, 3, q2_slot.as_raw(), r2_slot.as_raw()) };
    let (q2, r2) = unsafe { (q2_slot.assume_init(), r2_slot.assume_init()) };
    assert_eq!(q2, -3); // Mojo // is floor division
    println!("  OutSlot divmod(-7, 3) = ({q2}, {r2}): ok");

    // ── In-place swap via as_raw_mut ──
    let mut a = 1.0f64;
    let mut b = 2.0f64;
    unsafe {
        swap_f64(a.as_raw_mut(), b.as_raw_mut());
    };
    assert_eq!((a, b), (2.0, 1.0));
    println!("  swap(1.0, 2.0) → ({a}, {b}): ok");

    // ── Error handling (raises caught inside @export) ──
    assert_eq!(unsafe { safe_div(10.0, 2.0) }, 5.0);
    let err_result = unsafe { safe_div(10.0, 0.0) };
    assert!(err_result < -1e300); // sentinel value
    println!("  safe_div(10, 0) = {err_result:.2e} (sentinel): ok");

    println!("all ok");
}
