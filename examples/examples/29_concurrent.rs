// ─────────────────────────────────────────────────────────
// 29: Concurrent Mojo calls from multiple Rust threads
// ─────────────────────────────────────────────────────────
//
// The README says "thread safety is your responsibility."
// This tests whether pure Mojo functions (no shared state)
// can be called from multiple threads simultaneously.
//
// The parallelize segfault (ADR-011) proved Mojo has runtime state.
// Do sequential @export functions share that state?

use pyroxide::bridge::MojoSlice;

unsafe extern "C" {
    fn pure_dot(a: isize, b: isize, n: isize) -> f64;
    fn pure_sum_sq(addr: isize, n: isize) -> f64;
}

fn main() {
    let n = 1000;

    // ── Single-threaded baseline ──
    let a: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let b: Vec<f64> = (0..n).map(|i| (n - i) as f64).collect();
    let expected_dot = unsafe {
        pure_dot(
            MojoSlice::new(&a).as_raw(),
            MojoSlice::new(&b).as_raw(),
            n as isize,
        )
    };
    let expected_sq = unsafe { pure_sum_sq(MojoSlice::new(&a).as_raw(), n as isize) };
    println!("  baseline: dot={expected_dot:.0}, sum_sq={expected_sq:.0}");

    // ── Concurrent: 8 threads, each with own data ──
    let results: Vec<f64> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..8)
            .map(|t| {
                s.spawn(move || {
                    let a: Vec<f64> = (0..n).map(|i| (i + t * n) as f64).collect();
                    let b: Vec<f64> = (0..n).map(|i| (n - i) as f64).collect();
                    let sa = MojoSlice::new(&a);
                    let sb = MojoSlice::new(&b);
                    unsafe { pure_dot(sa.as_raw(), sb.as_raw(), n as isize) }
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("thread panicked"))
            .collect()
    });

    // All threads should complete without crash
    assert_eq!(results.len(), 8);
    for (i, &r) in results.iter().enumerate() {
        assert!(r.is_finite(), "thread {i} returned non-finite: {r}");
    }
    println!("  8 threads, independent data: all finite [ok]");

    // ── Concurrent: all threads read same data ──
    let shared_a: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let shared_b: Vec<f64> = (0..n).map(|i| (n - i) as f64).collect();

    let results2: Vec<f64> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let a_ref = &shared_a;
                let b_ref = &shared_b;
                s.spawn(move || {
                    let sa = MojoSlice::new(a_ref);
                    let sb = MojoSlice::new(b_ref);
                    unsafe { pure_dot(sa.as_raw(), sb.as_raw(), n as isize) }
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("thread panicked"))
            .collect()
    });

    // All should return same value as baseline
    for (i, &r) in results2.iter().enumerate() {
        assert!(
            (r - expected_dot).abs() < 1e-6,
            "thread {i}: {r} != {expected_dot}"
        );
    }
    println!("  8 threads, shared data, all match baseline [ok]");

    println!("all ok");
}
