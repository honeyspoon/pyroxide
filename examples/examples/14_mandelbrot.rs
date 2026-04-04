// ─────────────────────────────────────────────────────────
// 14: Mandelbrot set — Mojo computes, Rust owns the buffer
// ─────────────────────────────────────────────────────────
//
// Rust allocates an i32 grid, Mojo fills it with escape iterations.
// Demonstrates: large output buffer, multiple scalar params, compute-heavy
// workload where Mojo does the real work and Rust orchestrates.

use pyroxide::bridge::MojoSliceMut;

unsafe extern "C" {
    fn mandelbrot(
        out: isize,
        width: isize,
        height: isize,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        max_iter: isize,
    );
}

fn main() {
    let width = 80;
    let height = 40;
    let max_iter: usize = 100;
    let mut grid = vec![0i32; width * height];

    let mut s = MojoSliceMut::new(&mut grid);
    unsafe {
        mandelbrot(
            s.as_raw(),
            width as isize,
            height as isize,
            -2.0,
            1.0,
            -1.0,
            1.0,
            max_iter as isize,
        );
    }

    // Verify basic properties
    let total: usize = grid.iter().map(|&x| x as usize).sum();
    assert!(total > 0, "grid should have some non-zero iterations");
    println!("  total iterations: {total}");

    // Center of Mandelbrot set (0, 0) should be max_iter (inside the set)
    let cx = width / 3; // roughly x=0 at this mapping
    let cy = height / 2;
    let center_val = grid[cy * width + cx];
    assert_eq!(
        center_val, max_iter as i32,
        "center should be inside the set"
    );
    println!("  center ({cx}, {cy}) = {center_val} (max_iter) [ok]");

    // Corners should escape quickly (outside the set)
    let corner = grid[0]; // top-left: (-2, -1)
    assert!(corner < max_iter as i32, "corner should escape");
    println!("  corner (0,0) = {corner} (escaped) [ok]");

    // Print a tiny ASCII visualization
    println!();
    let chars = " .:-=+*#%@";
    for row in 0..height {
        let mut line = String::with_capacity(width);
        for col in 0..width {
            let val = grid[row * width + col];
            let idx = ((val as usize) * (chars.len() - 1)) / max_iter;
            line.push(chars.as_bytes()[idx.min(chars.len() - 1)] as char);
        }
        println!("  {line}");
    }
    println!();

    println!("all ok");
}
