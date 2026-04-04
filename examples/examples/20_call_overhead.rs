// ─────────────────────────────────────────────────────────
// 20: Call overhead — measure raw FFI cost
// ─────────────────────────────────────────────────────────
//
// How much does crossing the Rust→Mojo boundary cost?
// Measure noop, identity, add_one, and pointer stability.

use pyroxide::bridge::IntoMojo;
use std::hint::black_box;
use std::time::Instant;

unsafe extern "C" {
    fn noop();
    fn identity_i64(x: isize) -> isize;
    fn add_one(x: isize) -> isize;
    fn is_even(x: isize) -> bool;
    fn ptr_stable_check(addr: isize, expected: f64) -> bool;
}

fn bench_ns(name: &str, f: &dyn Fn()) -> u64 {
    for _ in 0..1000 {
        f();
    }
    let n = 100_000u64;
    let start = Instant::now();
    for _ in 0..n {
        black_box(f());
    }
    let ns = start.elapsed().as_nanos() as u64 / n;
    println!("    {name}: {ns}ns");
    ns
}

fn main() {
    // ── Correctness ──
    unsafe { noop() };
    println!("  noop [ok]");

    assert_eq!(unsafe { identity_i64(42) }, 42);
    assert_eq!(unsafe { identity_i64(-1) }, -1);
    assert_eq!(unsafe { identity_i64(isize::MAX) }, isize::MAX);
    println!("  identity_i64 [ok]");

    assert_eq!(unsafe { add_one(0) }, 1);
    assert_eq!(unsafe { add_one(-1) }, 0);
    println!("  add_one [ok]");

    assert!(unsafe { is_even(4) });
    assert!(!unsafe { is_even(3) });
    println!("  is_even [ok]");

    // ── Pointer stability across calls ──
    let val = 3.14f64;
    for _ in 0..100 {
        assert!(unsafe { ptr_stable_check(val.as_raw(), 3.14) });
    }
    println!("  pointer stable across 100 calls [ok]");

    // ── Overhead benchmark ──
    println!("\n  call overhead:");
    bench_ns("noop()", &|| unsafe { noop() });
    bench_ns("identity(42)", &|| unsafe {
        black_box(identity_i64(black_box(42)));
    });
    bench_ns("add_one(42)", &|| unsafe {
        black_box(add_one(black_box(42)));
    });
    bench_ns("is_even(42)", &|| unsafe {
        black_box(is_even(black_box(42)));
    });

    // Compare to a Rust no-op function call
    #[inline(never)]
    fn rust_noop() {}
    bench_ns("rust noop()", &rust_noop);

    println!("all ok");
}
