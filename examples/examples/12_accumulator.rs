// ─────────────────────────────────────────────────────────
// 12: Stateful accumulator — repeated as_raw_mut() across calls
// ─────────────────────────────────────────────────────────
//
// Rust owns a stats struct. Mojo updates it across multiple calls
// as data arrives in chunks. Tests whether as_raw_mut() feels natural
// for the repeated-mutation-across-calls pattern.

use pyroxide::prelude::*;

mojo_type! {
    /// Running statistics: count, sum, min, max, sum of squares.
    pub struct Stats {
        pub count: f64,
        pub sum: f64,
        pub min: f64,
        pub max: f64,
        pub sum_sq: f64,
    }
}

unsafe extern "C" {
    fn stats_init(state: isize);
    fn stats_update(state: isize, data: isize, n: isize);
    fn stats_mean(state: isize) -> f64;
    fn stats_variance(state: isize) -> f64;
}

impl Stats {
    fn init(&mut self) {
        unsafe { stats_init(self.as_raw_mut()) };
    }

    fn update(&mut self, data: &[f64]) {
        let s = MojoSlice::new(data);
        unsafe {
            stats_update(self.as_raw_mut(), s.as_raw(), s.len_isize());
        };
    }

    fn mean(&self) -> f64 {
        unsafe { stats_mean(self.as_raw()) }
    }

    fn variance(&self) -> f64 {
        unsafe { stats_variance(self.as_raw()) }
    }
}

fn main() {
    let mut stats = Stats {
        count: 0.0,
        sum: 0.0,
        min: 0.0,
        max: 0.0,
        sum_sq: 0.0,
    };
    stats.init();

    // ── Process data in chunks (simulating streaming) ──
    let chunks: &[&[f64]] = &[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0, 10.0]];

    for (i, chunk) in chunks.iter().enumerate() {
        stats.update(chunk);
        println!(
            "  chunk {i}: n={}, mean={:.2}, min={:.1}, max={:.1}",
            stats.count as usize,
            stats.mean(),
            stats.min,
            stats.max,
        );
    }

    // ── Verify against Rust ground truth ──
    let all_data: Vec<f64> = chunks.iter().flat_map(|c| c.iter().copied()).collect();
    let n = all_data.len() as f64;
    let rust_mean = all_data.iter().sum::<f64>() / n;
    let rust_var = all_data
        .iter()
        .map(|x| (x - rust_mean).powi(2))
        .sum::<f64>()
        / n;
    let rust_min = all_data
        .iter()
        .copied()
        .reduce(f64::min)
        .expect("non-empty");
    let rust_max = all_data
        .iter()
        .copied()
        .reduce(f64::max)
        .expect("non-empty");

    assert_eq!(stats.count, 10.0);
    assert!((stats.mean() - rust_mean).abs() < 1e-10);
    assert!((stats.variance() - rust_var).abs() < 1e-10);
    assert_eq!(stats.min, rust_min);
    assert_eq!(stats.max, rust_max);

    println!(
        "\n  final: n={}, mean={:.2}, var={:.2}, min={:.1}, max={:.1}",
        stats.count as usize,
        stats.mean(),
        stats.variance(),
        stats.min,
        stats.max
    );
    println!(
        "  rust:  n={}, mean={rust_mean:.2}, var={rust_var:.2}, min={rust_min:.1}, max={rust_max:.1}",
        all_data.len()
    );
    println!("  mojo == rust [ok]");

    println!("all ok");
}
