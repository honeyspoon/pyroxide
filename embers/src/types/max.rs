//! Types matching the Modular MAX ML framework (C API + Mojo kernel level).
//!
//! DType values match `max/include/max/c/types.h` (bit-flag encoded).
//! Tensor layout matches `ManagedTensorSlice` from MAX kernels.
//!
//! These are pure data structures — no FFI calls. Your application
//! uses `mojo_import!` or `unsafe extern "C"` for Mojo functions.

use crate::mojo_type;
use std::fmt;
use std::ops::{Deref, DerefMut};

// ── DType ──

/// MAX dtype, encoded as bit flags per `max/include/max/c/types.h`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, zerocopy::IntoBytes, zerocopy::Immutable)]
pub enum DType {
    Unknown = 0,
    Bool = 1,
    Float4E2M1FN = 64,
    Float8E8M0FNU = 73,
    Float8E3M4 = 74,
    Float8E4M3FN = 75,
    Float8E4M3FNUZ = 76,
    Float8E5M2 = 77,
    Float8E5M2FNUZ = 78,
    Float16 = 79,
    BFloat16 = 80,
    Float32 = 81,
    Float64 = 82,
    UInt8 = 134,
    UInt16 = 136,
    UInt32 = 138,
    UInt64 = 140,
    Int8 = 135,
    Int16 = 137,
    Int32 = 139,
    Int64 = 141,
}

impl DType {
    pub const fn byte_width(self) -> usize {
        match self {
            Self::Bool | Self::Int8 | Self::UInt8 => 1,
            Self::Float8E8M0FNU | Self::Float8E3M4 | Self::Float8E4M3FN
            | Self::Float8E4M3FNUZ | Self::Float8E5M2 | Self::Float8E5M2FNUZ => 1,
            Self::Int16 | Self::UInt16 | Self::Float16 | Self::BFloat16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 => 4,
            Self::Int64 | Self::UInt64 | Self::Float64 => 8,
            Self::Unknown | Self::Float4E2M1FN => 1,
        }
    }

    pub const fn is_float(self) -> bool { (self as u8) & 0x40 != 0 }
    pub const fn is_integer(self) -> bool { (self as u8) & 0x80 != 0 }
    pub const fn is_signed(self) -> bool { self.is_integer() && (self as u8) & 0x01 != 0 }
}

impl fmt::Display for DType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

// ── DeviceType ──

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, zerocopy::IntoBytes, zerocopy::Immutable)]
pub enum DeviceType {
    Host = 0,
    Accelerator = 1,
}

// ── TensorShape ──

mojo_type! {
    /// Tensor shape — up to 8 dimensions.
    pub struct TensorShape {
        pub rank: i64,
        pub dims: [i64; 8],
    }
}

impl TensorShape {
    pub fn new(dims: &[i64]) -> Self {
        assert!(dims.len() <= 8, "TensorShape supports up to 8 dimensions");
        let mut shape = Self { rank: dims.len() as i64, dims: [0; 8] };
        shape.dims[..dims.len()].copy_from_slice(dims);
        shape
    }

    pub fn scalar() -> Self { Self::new(&[]) }
    pub fn vector(n: i64) -> Self { Self::new(&[n]) }
    pub fn matrix(rows: i64, cols: i64) -> Self { Self::new(&[rows, cols]) }

    pub fn ndim(&self) -> usize {
        self.rank as usize
    }

    pub fn numel(&self) -> usize {
        self.as_slice().iter().product::<i64>() as usize
    }

    /// Excludes trailing zero-padded entries beyond `rank`.
    pub fn as_slice(&self) -> &[i64] {
        &self.dims[..self.rank as usize]
    }
}

impl fmt::Display for TensorShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, &d) in self.as_slice().iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{d}")?;
        }
        write!(f, ")")
    }
}

impl<const N: usize> From<[i64; N]> for TensorShape {
    fn from(dims: [i64; N]) -> Self {
        Self::new(&dims)
    }
}

// ── TensorDescriptor ──

mojo_type! {
    /// Tensor descriptor for FFI. 152 bytes.
    ///
    /// Offsets: dtype(0) rank(8) dims(16) strides(80) data_ptr(144)
    pub struct TensorDescriptor {
        pub dtype: u8,
        _pad: [u8; 7],
        pub rank: i64,
        pub dims: [i64; 8],
        pub strides: [i64; 8],
        pub data_ptr: i64,
    }
}

impl TensorDescriptor {
    pub fn contiguous(dtype: DType, shape: &TensorShape, data_ptr: *const u8) -> Self {
        let mut strides = [0i64; 8];
        if shape.rank > 0 {
            strides[shape.rank as usize - 1] = 1;
            for i in (0..shape.rank as usize - 1).rev() {
                strides[i] = strides[i + 1] * shape.dims[i + 1];
            }
        }
        Self {
            dtype: dtype as u8, _pad: [0; 7],
            rank: shape.rank, dims: shape.dims, strides,
            data_ptr: data_ptr as i64,
        }
    }
}

// ── Tensor<T> ──

/// Owned, typed tensor. Dereferences to `[T]` for idiomatic slice access.
pub struct Tensor<T: Copy + zerocopy::IntoBytes + zerocopy::Immutable> {
    data: Vec<T>,
    shape: TensorShape,
}

impl<T: Copy + Default + zerocopy::IntoBytes + zerocopy::Immutable + MojoDType> Tensor<T> {
    pub fn zeros(shape: TensorShape) -> Self {
        Self { data: vec![T::default(); shape.numel()], shape }
    }

    pub fn from_data(shape: TensorShape, data: Vec<T>) -> Self {
        let expected = shape.numel();
        assert_eq!(expected, data.len(), "shape has {expected} elements but got {}", data.len());
        Self { data, shape }
    }

    pub fn from_slice(shape: TensorShape, data: &[T]) -> Self {
        Self::from_data(shape, data.to_vec())
    }

    pub fn descriptor(&self) -> TensorDescriptor {
        TensorDescriptor::contiguous(T::DTYPE, &self.shape, self.data.as_ptr() as *const u8)
    }

    pub fn shape(&self) -> &TensorShape { &self.shape }
    pub fn dtype(&self) -> DType { T::DTYPE }
    pub fn numel(&self) -> usize { self.data.len() }
}

impl<T: Copy + zerocopy::IntoBytes + zerocopy::Immutable> Deref for Tensor<T> {
    type Target = [T];
    fn deref(&self) -> &[T] { &self.data }
}

impl<T: Copy + zerocopy::IntoBytes + zerocopy::Immutable> DerefMut for Tensor<T> {
    fn deref_mut(&mut self) -> &mut [T] { &mut self.data }
}

impl<T: Copy + fmt::Debug + zerocopy::IntoBytes + zerocopy::Immutable> fmt::Debug for Tensor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tensor")
            .field("dtype", &std::any::type_name::<T>())
            .field("shape", &format_args!("{}", self.shape))
            .field("data", &&self.data[..self.data.len().min(8)])
            .finish()
    }
}

impl<T: Copy + fmt::Display + zerocopy::IntoBytes + zerocopy::Immutable + MojoDType> fmt::Display for Tensor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor<{}>{} [", T::DTYPE, self.shape)?;
        let show = self.data.len().min(6);
        for (i, v) in self.data[..show].iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{v}")?;
        }
        if self.data.len() > show {
            write!(f, ", ... ({} more)", self.data.len() - show)?;
        }
        write!(f, "]")
    }
}

// ── MojoDType trait ──

/// Maps Rust primitives to MAX DType values.
pub trait MojoDType: Copy + Default + zerocopy::IntoBytes + zerocopy::Immutable {
    const DTYPE: DType;
}

macro_rules! impl_mojo_dtype {
    ($($rust_ty:ty => $variant:ident),* $(,)?) => {
        $(impl MojoDType for $rust_ty { const DTYPE: DType = DType::$variant; })*
    };
}

impl_mojo_dtype! {
    f32 => Float32, f64 => Float64,
    i8  => Int8,    i16 => Int16,   i32 => Int32,  i64 => Int64,
    u8  => UInt8,   u16 => UInt16,  u32 => UInt32, u64 => UInt64,
}
