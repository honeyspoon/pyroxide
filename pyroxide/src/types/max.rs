//! Types matching the Modular MAX ML framework (C API + Mojo kernel level).
//!
//! `DType` values match `max/include/max/c/types.h` (bit-flag encoded).
//! Tensor layout matches `ManagedTensorSlice` from MAX kernels.
//!
//! **Requires 64-bit.** `TensorDescriptor::data_ptr` stores pointers as `i64`.

#[cfg(not(target_pointer_width = "64"))]
compile_error!("pyroxide MAX types require a 64-bit target (data_ptr is i64)");

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
            Self::Int16 | Self::UInt16 | Self::Float16 | Self::BFloat16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 => 4,
            Self::Int64 | Self::UInt64 | Self::Float64 => 8,
            Self::Unknown
            | Self::Bool
            | Self::Float4E2M1FN
            | Self::Float8E8M0FNU
            | Self::Float8E3M4
            | Self::Float8E4M3FN
            | Self::Float8E4M3FNUZ
            | Self::Float8E5M2
            | Self::Float8E5M2FNUZ
            | Self::Int8
            | Self::UInt8 => 1,
        }
    }

    pub const fn is_float(self) -> bool {
        (self as u8) & 0x40 != 0
    }
    pub const fn is_integer(self) -> bool {
        (self as u8) & 0x80 != 0
    }
    pub const fn is_signed(self) -> bool {
        self.is_integer() && (self as u8) & 0x01 != 0
    }
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
    /// Create a shape from dimension sizes.
    ///
    /// # Panics
    ///
    /// Panics if `dims.len() > 8`. The 8-dimension cap is a pyroxide
    /// design choice for fixed-size `#[repr(C)]` layout — Mojo itself
    /// has no such restriction.
    pub fn new(dims: &[i64]) -> Self {
        assert!(dims.len() <= 8, "TensorShape supports up to 8 dimensions");
        let mut shape = Self {
            rank: dims.len() as i64,
            dims: [0; 8],
        };
        shape.dims[..dims.len()].copy_from_slice(dims);
        shape
    }

    pub fn scalar() -> Self {
        Self::new(&[])
    }
    pub fn vector(n: i64) -> Self {
        Self::new(&[n])
    }
    pub fn matrix(rows: i64, cols: i64) -> Self {
        Self::new(&[rows, cols])
    }

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
            if i > 0 {
                write!(f, ", ")?;
            }
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
    /// Tensor descriptor for FFI. 152 bytes, `#[repr(C)]`.
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
    /// Create a descriptor for a contiguous (row-major) tensor.
    ///
    /// Strides are computed for contiguous layout. Non-contiguous tensors
    /// (e.g., transposed views) are not yet supported — the Mojo-side
    /// examples assume contiguous data and ignore stride fields.
    pub fn contiguous(dtype: DType, shape: &TensorShape, data_ptr: *const u8) -> Self {
        let mut strides = [0i64; 8];
        if shape.rank > 0 {
            strides[shape.rank as usize - 1] = 1;
            for i in (0..shape.rank as usize - 1).rev() {
                strides[i] = strides[i + 1] * shape.dims[i + 1];
            }
        }
        Self {
            dtype: dtype as u8,
            _pad: [0; 7],
            rank: shape.rank,
            dims: shape.dims,
            strides,
            data_ptr: data_ptr as i64,
        }
    }
}

fn make_descriptor<T: MojoDType + zerocopy::IntoBytes + zerocopy::Immutable>(
    shape: &TensorShape,
    data: &[T],
) -> TensorDescriptor {
    TensorDescriptor::contiguous(T::DTYPE, shape, data.as_ptr().cast::<u8>())
}

// ── DescriptorGuard ──

/// A `TensorDescriptor` tied to the lifetime of its source data.
///
/// Follows the `MutexGuard` pattern: holds the borrow, derefs to
/// `TensorDescriptor`. The compiler ensures the tensor outlives
/// the descriptor — no dangling `data_ptr`.
///
/// ```rust,ignore
/// let desc = tensor.descriptor(); // DescriptorGuard<'_>
/// unsafe { mojo_fn(desc.as_raw()) };
/// // desc borrows tensor — can't drop tensor while desc is alive
/// ```
pub struct DescriptorGuard<'a> {
    desc: TensorDescriptor,
    _marker: std::marker::PhantomData<&'a ()>,
}

// DescriptorGuard derefs to TensorDescriptor, which implements
// IntoMojo::as_raw(). So desc.as_raw() works via auto-deref.

impl std::ops::Deref for DescriptorGuard<'_> {
    type Target = TensorDescriptor;
    fn deref(&self) -> &TensorDescriptor {
        &self.desc
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
        Self {
            data: vec![T::default(); shape.numel()],
            shape,
        }
    }

    pub fn from_data(shape: TensorShape, data: Vec<T>) -> Self {
        let expected = shape.numel();
        assert_eq!(
            expected,
            data.len(),
            "shape has {expected} elements but got {}",
            data.len()
        );
        Self { data, shape }
    }

    pub fn from_slice(shape: TensorShape, data: &[T]) -> Self {
        Self::from_data(shape, data.to_vec())
    }

    /// Get a lifetime-bound descriptor for FFI.
    ///
    /// The descriptor borrows `self` — the compiler prevents dangling `data_ptr`.
    pub fn descriptor(&self) -> DescriptorGuard<'_> {
        DescriptorGuard {
            desc: make_descriptor(&self.shape, &self.data),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }
    #[allow(
        clippy::unused_self,
        reason = "consistent accessor API alongside shape()/numel()"
    )]
    pub fn dtype(&self) -> DType {
        T::DTYPE
    }
    pub fn numel(&self) -> usize {
        self.data.len()
    }
}

impl<T: Copy + zerocopy::IntoBytes + zerocopy::Immutable> Deref for Tensor<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.data
    }
}

impl<T: Copy + zerocopy::IntoBytes + zerocopy::Immutable> DerefMut for Tensor<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
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

impl<T: Copy + fmt::Display + zerocopy::IntoBytes + zerocopy::Immutable + MojoDType> fmt::Display
    for Tensor<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor<{}>{} [", T::DTYPE, self.shape)?;
        let show = self.data.len().min(6);
        for (i, v) in self.data[..show].iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{v}")?;
        }
        if self.data.len() > show {
            write!(f, ", ... ({} more)", self.data.len() - show)?;
        }
        write!(f, "]")
    }
}

// ── MojoDType trait ──

/// Maps Rust primitives to MAX `DType` values.
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

// ── TensorView ──

/// Borrowed tensor view — zero-copy over external data (e.g., safetensors mmap).
///
/// Unlike [`Tensor<T>`] which owns a `Vec<T>`, `TensorView` borrows a `&[T]`.
/// Use this when you have data from an external source and want to pass it
/// to Mojo without copying.
///
/// ```rust,ignore
/// let weights: &[f32] = /* from safetensors */;
/// let view = TensorView::new(TensorShape::matrix(vocab, hidden), weights);
/// let desc = view.descriptor();
/// unsafe { mojo_fn(desc.as_raw()) };
/// ```
pub struct TensorView<'a, T: Copy + zerocopy::IntoBytes + zerocopy::Immutable + MojoDType> {
    data: &'a [T],
    shape: TensorShape,
}

impl<'a, T: Copy + zerocopy::IntoBytes + zerocopy::Immutable + MojoDType> TensorView<'a, T> {
    pub fn new(shape: TensorShape, data: &'a [T]) -> Self {
        assert_eq!(
            shape.numel(),
            data.len(),
            "shape has {} elements but slice has {}",
            shape.numel(),
            data.len()
        );
        Self { data, shape }
    }

    pub fn descriptor(&self) -> DescriptorGuard<'_> {
        DescriptorGuard {
            desc: make_descriptor(&self.shape, self.data),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    #[allow(
        clippy::unused_self,
        reason = "consistent accessor API alongside shape()"
    )]
    pub fn dtype(&self) -> DType {
        T::DTYPE
    }

    pub fn data(&self) -> &'a [T] {
        self.data
    }

    pub fn numel(&self) -> usize {
        self.data.len()
    }
}

impl<T: Copy + fmt::Debug + zerocopy::IntoBytes + zerocopy::Immutable + MojoDType> fmt::Debug
    for TensorView<'_, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TensorView")
            .field("dtype", &std::any::type_name::<T>())
            .field("shape", &format_args!("{}", self.shape))
            .field("data", &&self.data[..self.data.len().min(8)])
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensor_shape_basics() {
        let s = TensorShape::vector(5);
        assert_eq!(s.ndim(), 1);
        assert_eq!(s.numel(), 5);
        assert_eq!(s.as_slice(), &[5]);
        assert_eq!(s.to_string(), "(5)");
    }

    #[test]
    fn tensor_shape_matrix() {
        let s = TensorShape::matrix(3, 4);
        assert_eq!(s.ndim(), 2);
        assert_eq!(s.numel(), 12);
        assert_eq!(s.to_string(), "(3, 4)");
    }

    #[test]
    fn tensor_shape_from_array() {
        let s = TensorShape::from([2, 3, 4]);
        assert_eq!(s.ndim(), 3);
        assert_eq!(s.numel(), 24);
    }

    #[test]
    fn tensor_zeros() {
        let t = Tensor::<f64>::zeros(TensorShape::vector(10));
        assert_eq!(t.len(), 10);
        assert!(t.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn tensor_from_data() {
        let t = Tensor::<f32>::from_data(TensorShape::matrix(2, 3), vec![1.0; 6]);
        assert_eq!(t.numel(), 6);
        assert_eq!(t.shape().ndim(), 2);
    }

    #[test]
    fn tensor_descriptor_layout() {
        let t = Tensor::<f64>::from_data(TensorShape::vector(5), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let desc = t.descriptor();
        assert_eq!(desc.dtype, DType::Float64 as u8);
        assert_eq!(desc.rank, 1);
        assert_eq!(desc.dims[0], 5);
        assert_ne!(desc.data_ptr, 0);
    }

    #[test]
    fn tensor_view_zero_copy() {
        let data = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        let view = TensorView::new(TensorShape::matrix(2, 3), &data);
        assert_eq!(view.numel(), 6);
        assert_eq!(view.dtype(), DType::Float32);

        // Descriptor should point to the original data, not a copy
        let desc = view.descriptor();
        assert_eq!(desc.data_ptr, data.as_ptr() as i64);
    }

    #[test]
    fn dtype_byte_width() {
        assert_eq!(DType::Float32.byte_width(), 4);
        assert_eq!(DType::Float64.byte_width(), 8);
        assert_eq!(DType::Int8.byte_width(), 1);
        assert_eq!(DType::Int64.byte_width(), 8);
        assert_eq!(DType::Bool.byte_width(), 1);
    }

    #[test]
    fn dtype_classification() {
        assert!(DType::Float32.is_float());
        assert!(!DType::Float32.is_integer());
        assert!(DType::Int32.is_integer());
        assert!(DType::Int32.is_signed());
        assert!(DType::UInt32.is_integer());
        assert!(!DType::UInt32.is_signed());
    }
}
