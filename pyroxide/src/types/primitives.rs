//! Common math/geometry types, ready for Mojo FFI.

use crate::mojo_type;
use std::fmt;
use std::ops::{Add, Mul, Sub};

mojo_type! {
    /// A 2D point. 16 bytes.
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }
}

mojo_type! {
    /// A 3D vector with scalar weight. 32 bytes.
    pub struct Vec4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
}

mojo_type! {
    /// A 4x4 matrix, column-major. 128 bytes.
    pub struct Mat4 {
        pub cols: [f64; 16],
    }
}

// ── Point ──

impl Point {
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (*self - *other).length()
    }

    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<[f64; 2]> for Point {
    fn from([x, y]: [f64; 2]) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

// ── Vec4 ──

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn xyz(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x.mul_add(
            other.x,
            self.y
                .mul_add(other.y, self.z.mul_add(other.z, self.w * other.w)),
        )
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl From<(f64, f64, f64, f64)> for Vec4 {
    fn from((x, y, z, w): (f64, f64, f64, f64)) -> Self {
        Self { x, y, z, w }
    }
}

impl From<[f64; 4]> for Vec4 {
    fn from([x, y, z, w]: [f64; 4]) -> Self {
        Self { x, y, z, w }
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }
}

// ── Mat4 ──

impl Mat4 {
    pub fn identity() -> Self {
        let mut cols = [0.0; 16];
        for i in 0..4 {
            cols[i * 4 + i] = 1.0;
        }
        Self { cols }
    }

    pub fn trace(&self) -> f64 {
        self.cols[0] + self.cols[5] + self.cols[10] + self.cols[15]
    }

    /// Access element at (row, col) in column-major layout.
    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.cols[col * 4 + row]
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..4 {
            write!(f, "[")?;
            for col in 0..4 {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.3}", self.at(row, col))?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}
