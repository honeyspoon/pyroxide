// ─────────────────────────────────────────────────────────
// 15: Nested structs — Line (2 Points), Triangle (3 Points)
// ─────────────────────────────────────────────────────────
//
// Tests that #[repr(C)] nested structs have the expected flat layout.
// Mojo accesses fields by offset — the nesting is invisible at the ABI.

use pyroxide::prelude::*;

mojo_type! {
    pub struct Line {
        pub p1: Point,
        pub p2: Point,
    }
}

mojo_type! {
    pub struct Triangle {
        pub a: Point,
        pub b: Point,
        pub c: Point,
    }
}

unsafe extern "C" {
    fn line_length(addr: isize) -> f64;
    fn triangle_area(addr: isize) -> f64;
    fn centroid(tri: isize, out: isize);
}

fn main() {
    // Line: (0,0) → (3,4), length = 5
    let line = Line {
        p1: Point::new(0.0, 0.0),
        p2: Point::new(3.0, 4.0),
    };
    let len = unsafe { line_length(line.as_raw()) };
    assert!((len - 5.0).abs() < 1e-6);
    println!("  line_length((0,0)→(3,4)) = {len:.6} [ok]");

    // Triangle: (0,0), (4,0), (0,3), area = 6
    let tri = Triangle {
        a: Point::new(0.0, 0.0),
        b: Point::new(4.0, 0.0),
        c: Point::new(0.0, 3.0),
    };
    let area = unsafe { triangle_area(tri.as_raw()) };
    assert!((area - 6.0).abs() < 1e-6);
    println!("  triangle_area = {area:.6} [ok]");

    // Centroid: (0+4+0)/3, (0+0+3)/3 = (1.333, 1.0)
    let mut center = Point::new(0.0, 0.0);
    unsafe { centroid(tri.as_raw(), center.as_raw_mut()) };
    assert!((center.x - 4.0 / 3.0).abs() < 1e-6);
    assert!((center.y - 1.0).abs() < 1e-6);
    println!("  centroid = ({:.4}, {:.4}) [ok]", center.x, center.y);

    // Verify layout: Line should be exactly 4 × f64 = 32 bytes
    assert_eq!(std::mem::size_of::<Line>(), 32);
    assert_eq!(std::mem::size_of::<Triangle>(), 48);
    println!("  sizeof(Line) = 32, sizeof(Triangle) = 48 [ok]");

    println!("all ok");
}
