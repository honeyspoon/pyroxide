// ─────────────────────────────────────────────────────────
// 06: Compile-time metaprogramming with `comptime`
// ─────────────────────────────────────────────────────────
//
// Mojo's `comptime` keyword resolves values at compile time.
// Combined with `comptime for`, this lets you unroll loops
// entirely — the binary contains no loop, just straight-line
// instructions. Rust has const generics, but Mojo's comptime
// can run arbitrary code including loops and conditionals.
//
// We also test compile-time constants (golden ratio, pi) that
// are baked into the binary with zero runtime cost.

unsafe extern "C" {
    fn dot_unrolled_4(a: isize, b: isize) -> f64;
    fn get_golden_ratio() -> f64;
    fn fibonacci_ratio(n: isize) -> f64;
}

fn main() {
    // Compile-time unrolled dot product: no loop in the binary
    let a = [1.0f64, 2.0, 3.0, 4.0];
    let b = [5.0f64, 6.0, 7.0, 8.0];
    let dot = unsafe { dot_unrolled_4(a.as_ptr() as isize, b.as_ptr() as isize) };
    assert_eq!(dot, 70.0);
    println!("  dot_unrolled_4 = {dot} [ok]");

    // Compile-time constant: golden ratio baked into the binary
    let phi = unsafe { get_golden_ratio() };
    assert!((phi - 1.618033988749895).abs() < 1e-12);
    println!("  golden_ratio = {phi} [ok]");

    // Runtime Fibonacci converges to the comptime golden ratio
    let fib20 = unsafe { fibonacci_ratio(20) };
    assert!((fib20 - phi).abs() < 1e-6);
    println!("  fib_ratio(20) = {fib20} ≈ phi [ok]");

    println!("all ok");
}
