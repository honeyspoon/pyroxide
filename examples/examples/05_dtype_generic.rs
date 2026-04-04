// ─────────────────────────────────────────────────────────
// 05: DType-generic programming — one template, many types
// ─────────────────────────────────────────────────────────
//
// Mojo can parameterize functions on DType — write one algorithm
// that works for f32, f64, i32, etc. The compiler monomorphizes
// each variant at compile time. Zero runtime dispatch.
//
// In mojo/dtype_generic.mojo:
//
//     def _sum_impl[dtype: DType](addr: Int, n: Int) -> Scalar[dtype]:
//         ...
//
//     @export
//     def sum_f32(addr: Int, n: Int) -> Float32:
//         return _sum_impl[DType.float32](addr, n)
//
// One implementation, three exports. Each is fully specialized.

unsafe extern "C" {
    fn sum_f32(addr: isize, n: isize) -> f32;
    fn sum_f64(addr: isize, n: isize) -> f64;
    fn sum_i32(addr: isize, n: isize) -> i32;
}

fn main() {
    let f32s = [1.0f32, 2.0, 3.0];
    let f64s = [1.0f64, 2.0, 3.0];
    let i32s = [10i32, 20, 30];

    let sf32 = unsafe { sum_f32(f32s.as_ptr() as isize, 3) };
    let sf64 = unsafe { sum_f64(f64s.as_ptr() as isize, 3) };
    let si32 = unsafe { sum_i32(i32s.as_ptr() as isize, 3) };

    assert_eq!(sf32, 6.0);
    assert_eq!(sf64, 6.0);
    assert_eq!(si32, 60);

    println!("  sum_f32([1,2,3]) = {sf32} [ok]");
    println!("  sum_f64([1,2,3]) = {sf64} [ok]");
    println!("  sum_i32([10,20,30]) = {si32} [ok]");
    println!("all ok");
}
