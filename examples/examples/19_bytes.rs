// ─────────────────────────────────────────────────────────
// 19: Raw bytes and i64 arrays — non-float data types
// ─────────────────────────────────────────────────────────
//
// Most examples use f32/f64. This tests u8 (bytes) and i64
// (indices/tokens) — common in serialization and ML pipelines.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn byte_sum(addr: isize, n: isize) -> isize;
    fn memset_byte(addr: isize, n: isize, val: u8);
    fn byte_xor(a: isize, b: isize, out: isize, n: isize);
    fn prefix_sum_i64(addr: isize, n: isize);
}

fn main() {
    // ── u8 sum ──
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let sum = unsafe { byte_sum(MojoSlice::new(&data).as_raw(), data.len() as isize) };
    assert_eq!(sum, 15);
    println!("  byte_sum([1,2,3,4,5]) = {sum} [ok]");

    // ── memset ──
    let mut buf = vec![0u8; 10];
    unsafe { memset_byte(MojoSliceMut::new(&mut buf).as_raw(), 10, 0xAB) };
    assert!(buf.iter().all(|&b| b == 0xAB));
    println!("  memset(10, 0xAB) [ok]");

    // ── XOR ──
    let a = vec![0xFF_u8, 0x00, 0xAA, 0x55];
    let b = vec![0x0F_u8, 0xF0, 0x55, 0xAA];
    let mut out = vec![0u8; 4];
    unsafe {
        byte_xor(
            MojoSlice::new(&a).as_raw(),
            MojoSlice::new(&b).as_raw(),
            MojoSliceMut::new(&mut out).as_raw(),
            4,
        );
    }
    assert_eq!(out, [0xF0, 0xF0, 0xFF, 0xFF]);
    println!("  byte_xor = {out:02X?} [ok]");

    // Verify against Rust
    let rust_xor: Vec<u8> = a.iter().zip(&b).map(|(x, y)| x ^ y).collect();
    assert_eq!(out, rust_xor);
    println!("  xor matches Rust [ok]");

    // ── i64 prefix sum ──
    let mut arr: Vec<i64> = vec![1, 2, 3, 4, 5];
    unsafe { prefix_sum_i64(MojoSliceMut::new(&mut arr).as_raw(), arr.len() as isize) };
    assert_eq!(arr, [1, 3, 6, 10, 15]);
    println!("  prefix_sum_i64 = {arr:?} [ok]");

    println!("all ok");
}
