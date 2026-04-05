// ─────────────────────────────────────────────────────────
// 28: Pointer aliasing — same buffer as src and dst
// ─────────────────────────────────────────────────────────
//
// Real code often passes the same buffer as input and output
// (in-place normalization, scale). Does Mojo's UnsafePointer
// handle aliased reads/writes correctly?

use pyroxide::bridge::MojoSliceMut;

unsafe extern "C" {
    fn scale_inplace(addr: isize, n: isize, factor: f64);
    fn add_arrays_aliased(a: isize, b: isize, dst: isize, n: isize);
    fn shift_right(addr: isize, n: isize);
}

fn main() {
    // ── In-place scale (trivial aliasing: one pointer, read+write) ──
    let mut data = vec![1.0, 2.0, 3.0, 4.0];
    unsafe { scale_inplace(MojoSliceMut::new(&mut data).as_raw(), 4, 2.5) };
    assert_eq!(data, [2.5, 5.0, 7.5, 10.0]);
    println!("  scale_inplace(×2.5) [ok]");

    // ── dst aliases src: a + a → a (double in place) ──
    let mut x = vec![1.0, 2.0, 3.0];
    let ptr = MojoSliceMut::new(&mut x).as_raw();
    unsafe { add_arrays_aliased(ptr, ptr, ptr, 3) };
    assert_eq!(x, [2.0, 4.0, 6.0]);
    println!("  add(a, a, a) = 2a [ok]");

    // ── Overlapping shift right ──
    let mut arr = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    unsafe { shift_right(MojoSliceMut::new(&mut arr).as_raw(), 5) };
    assert_eq!(arr, [0.0, 10.0, 20.0, 30.0, 40.0]);
    println!("  shift_right [ok]");

    // Shift again
    unsafe { shift_right(MojoSliceMut::new(&mut arr).as_raw(), 5) };
    assert_eq!(arr, [0.0, 0.0, 10.0, 20.0, 30.0]);
    println!("  shift_right×2 [ok]");

    println!("all ok");
}
