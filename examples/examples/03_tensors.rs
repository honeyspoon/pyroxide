// ─────────────────────────────────────────────────────────
// 03: Multi-dimensional data with TensorDescriptor
// ─────────────────────────────────────────────────────────
//
// Tensors need metadata: shape, strides, data pointer, dtype.
// Pyroxide provides `TensorDescriptor` — a 152-byte repr(C)
// struct that Mojo reads to find and iterate the data.
//
// The workflow:
//   1. Create a Tensor<f64> in Rust (owns the data as Vec<T>)
//   2. Call .descriptor() → TensorDescriptor (captures the pointer)
//   3. Pass the descriptor's address to Mojo
//   4. Mojo reads shape/strides/data_ptr from the descriptor
//
// The data never copies — Mojo reads directly from Rust's heap.

use pyroxide::bridge::IntoMojo;
use pyroxide::types::max::{Tensor, TensorShape};

unsafe extern "C" {
    fn tensor_sum_f64(desc: isize) -> f64;
    fn tensor_dot_f64(a: isize, b: isize) -> f64;
    fn tensor_matmul_f32(a: isize, b: isize, out: isize);
}

fn main() {
    // ── Sum: Mojo reads 5 elements via TensorDescriptor ──
    let t = Tensor::<f64>::from_data(TensorShape::vector(5), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let sum = unsafe { tensor_sum_f64(t.descriptor().as_mojo().addr()) };
    assert_eq!(sum, 15.0);
    println!("  sum([1..5]) = {sum} [ok]");

    // ── Dot product: two descriptors, one result ──
    let a = Tensor::<f64>::from_slice([3].into(), &[1.0, 2.0, 3.0]);
    let b = Tensor::<f64>::from_slice([3].into(), &[4.0, 5.0, 6.0]);
    let dot = unsafe {
        tensor_dot_f64(
            a.descriptor().as_mojo().addr(),
            b.descriptor().as_mojo().addr(),
        )
    };
    assert_eq!(dot, 32.0);
    println!("  dot = {dot} [ok]");

    // ── Matrix multiply: Mojo reads shapes from descriptors ──
    //
    //   A(2×3) @ B(3×2) = C(2×2)
    //   [[1,2,3],[4,5,6]] @ [[7,8],[9,10],[11,12]] = [[58,64],[139,154]]
    let ma = Tensor::<f32>::from_data(
        TensorShape::matrix(2, 3),
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    );
    let mb = Tensor::<f32>::from_data(
        TensorShape::matrix(3, 2),
        vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
    );
    let mc = Tensor::<f32>::zeros(TensorShape::matrix(2, 2));
    unsafe {
        tensor_matmul_f32(
            ma.descriptor().as_mojo().addr(),
            mb.descriptor().as_mojo().addr(),
            mc.descriptor().as_mojo().addr(),
        );
    }
    assert_eq!(mc.as_ref() as &[f32], &[58.0, 64.0, 139.0, 154.0]);
    println!("  matmul = {:?} [ok]", mc.as_ref() as &[f32]);

    println!("all ok");
}
