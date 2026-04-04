// ─────────────────────────────────────────────────────────
// 13: In-place sorting — Mojo mutates a Rust-owned array
// ─────────────────────────────────────────────────────────
//
// Rust allocates data, Mojo sorts it in-place. Tests the
// mutable-buffer handoff and verifies against Rust's sort.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn sort_f64(addr: isize, n: isize);
    fn is_sorted_f64(addr: isize, n: isize) -> bool;
    fn reverse_f64(addr: isize, n: isize);
}

fn main() {
    // ── Sort ──
    let mut data = vec![5.0, 3.0, 8.0, 1.0, 9.0, 2.0, 7.0, 4.0, 6.0, 0.0];
    let n = data.len() as isize;
    unsafe { sort_f64(MojoSliceMut::new(&mut data).as_raw(), n) };
    assert_eq!(data, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    println!("  sort_f64 = {data:?} [ok]");

    // ── is_sorted ──
    assert!(unsafe { is_sorted_f64(MojoSlice::new(&data).as_raw(), n) });
    println!("  is_sorted(sorted) = true [ok]");

    let unsorted = [3.0, 1.0, 2.0];
    assert!(!unsafe { is_sorted_f64(MojoSlice::new(&unsorted).as_raw(), 3) });
    println!("  is_sorted(unsorted) = false [ok]");

    // ── Reverse ──
    unsafe { reverse_f64(MojoSliceMut::new(&mut data).as_raw(), n) };
    assert_eq!(data, [9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
    println!("  reverse = {data:?} [ok]");

    // ── Sort + verify against Rust ──
    let mut mojo_data = vec![42.0, -1.0, 3.14, 0.0, 99.0, -50.0, 2.718];
    let mut rust_data = mojo_data.clone();
    let mn = mojo_data.len() as isize;
    unsafe { sort_f64(MojoSliceMut::new(&mut mojo_data).as_raw(), mn) };
    rust_data.sort_by(|a, b| a.partial_cmp(b).expect("no NaN"));
    assert_eq!(mojo_data, rust_data);
    println!("  mojo sort == rust sort [ok]");

    println!("all ok");
}
