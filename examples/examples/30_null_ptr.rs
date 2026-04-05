// ─────────────────────────────────────────────────────────
// 30: Null pointer behavior across FFI
// ─────────────────────────────────────────────────────────
//
// addr=0 is a valid isize. What happens when Mojo receives it?
// Answer: segfault if dereferenced, fine if guarded by n==0.
//
// This documents the safe pattern: always check n before
// dereferencing the pointer.

unsafe extern "C" {
    fn safe_sum(addr: isize, n: isize) -> f64;
    fn safe_write(addr: isize, n: isize, val: f64) -> bool;
}

fn main() {
    // ── n=0 with null pointer: safe (Mojo guards on n) ──
    let result = unsafe { safe_sum(0, 0) };
    assert_eq!(result, 0.0);
    println!("  safe_sum(null, 0) = 0.0 [ok]");

    let wrote = unsafe { safe_write(0, 0, 42.0) };
    assert!(!wrote);
    println!("  safe_write(null, 0) = false [ok]");

    // ── Normal case still works ──
    let data = [1.0, 2.0, 3.0];
    let sum = unsafe { safe_sum(data.as_ptr() as isize, 3) };
    assert_eq!(sum, 6.0);
    println!("  safe_sum([1,2,3]) = 6.0 [ok]");

    let mut buf = [0.0f64; 3];
    let wrote2 = unsafe { safe_write(buf.as_mut_ptr() as isize, 3, 7.0) };
    assert!(wrote2);
    assert_eq!(buf, [7.0, 7.0, 7.0]);
    println!("  safe_write(buf, 3, 7.0) = true, buf=[7,7,7] [ok]");

    println!("all ok");
}
