// ─────────────────────────────────────────────────────────
// 05: DType-generic programming — one template, many types
// ─────────────────────────────────────────────────────────
//
// Mojo parameterizes functions on DType — one algorithm, specialized
// for f32, f64, i32 at compile time. Zero runtime dispatch.

use pyroxide::bridge::MojoSlice;

unsafe extern "C" {
    fn sum_f32(addr: isize, n: isize) -> f32;
    fn sum_f64(addr: isize, n: isize) -> f64;
    fn sum_i32(addr: isize, n: isize) -> i32;
}

fn main() {
    let f32s = [1.0f32, 2.0, 3.0];
    let f64s = [1.0f64, 2.0, 3.0];
    let i32s = [10i32, 20, 30];

    let sf = MojoSlice::new(&f32s);
    let sd = MojoSlice::new(&f64s);
    let si = MojoSlice::new(&i32s);

    let sf32 = unsafe { sum_f32(sf.addr().as_raw(), sf.len() as isize) };
    let sf64 = unsafe { sum_f64(sd.addr().as_raw(), sd.len() as isize) };
    let si32 = unsafe { sum_i32(si.addr().as_raw(), si.len() as isize) };

    assert_eq!(sf32, 6.0);
    assert_eq!(sf64, 6.0);
    assert_eq!(si32, 60);

    println!("  sum_f32([1,2,3]) = {sf32} [ok]");
    println!("  sum_f64([1,2,3]) = {sf64} [ok]");
    println!("  sum_i32([10,20,30]) = {si32} [ok]");
    println!("all ok");
}
