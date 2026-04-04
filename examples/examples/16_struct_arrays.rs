// ─────────────────────────────────────────────────────────
// 16: Arrays of structs — Point[] and Particle[] slices
// ─────────────────────────────────────────────────────────
//
// Mojo iterates over arrays of structs by stride. Tests that
// MojoSlice/MojoSliceMut work with struct element types.

use pyroxide::prelude::*;

mojo_type! {
    pub struct Particle2D {
        pub x: f64,
        pub y: f64,
        pub vx: f64,
        pub vy: f64,
        pub mass: f64,
    }
}

unsafe extern "C" {
    fn closest_pair_distance(points: isize, n: isize) -> f64;
    fn step_particles(particles: isize, n: isize, dt: f64);
    fn total_kinetic_energy(particles: isize, n: isize) -> f64;
}

fn main() {
    // ── Closest pair in a Point array ──
    let points = [
        Point::new(0.0, 0.0),
        Point::new(3.0, 4.0),
        Point::new(1.0, 0.0), // closest to (0,0): distance = 1.0
    ];
    let s = MojoSlice::new(&points);
    let dist = unsafe { closest_pair_distance(s.as_raw(), s.len() as isize) };
    assert!((dist - 1.0).abs() < 1e-6);
    println!("  closest_pair_distance = {dist:.6} [ok]");

    // ── Particle simulation step ──
    let mut particles = vec![
        Particle2D {
            x: 0.0,
            y: 0.0,
            vx: 1.0,
            vy: 2.0,
            mass: 1.0,
        },
        Particle2D {
            x: 10.0,
            y: 10.0,
            vx: -1.0,
            vy: 0.0,
            mass: 2.0,
        },
    ];
    let dt = 0.5;

    // Save initial positions
    let x0 = particles[0].x;
    let y0 = particles[0].y;

    let mut ms = MojoSliceMut::new(&mut particles);
    unsafe { step_particles(ms.as_raw(), ms.len() as isize, dt) };

    // Verify: position += velocity * dt
    assert!((particles[0].x - (x0 + 1.0 * dt)).abs() < 1e-10);
    assert!((particles[0].y - (y0 + 2.0 * dt)).abs() < 1e-10);
    assert!((particles[1].x - (10.0 - 1.0 * dt)).abs() < 1e-10);
    println!("  step_particles (dt={dt}): positions updated [ok]");

    // ── Kinetic energy ──
    let ke = unsafe {
        total_kinetic_energy(
            MojoSlice::new(&particles).as_raw(),
            particles.len() as isize,
        )
    };
    // KE = 0.5 * 1.0 * (1^2 + 2^2) + 0.5 * 2.0 * (1^2 + 0^2) = 2.5 + 1.0 = 3.5
    assert!((ke - 3.5).abs() < 1e-6);
    println!("  total_kinetic_energy = {ke:.6} [ok]");

    // Verify against Rust computation
    let rust_ke: f64 = particles
        .iter()
        .map(|p| 0.5 * p.mass * (p.vx * p.vx + p.vy * p.vy))
        .sum();
    assert!((ke - rust_ke).abs() < 1e-10);
    println!("  mojo ke == rust ke [ok]");

    println!("all ok");
}
