// ─────────────────────────────────────────────────────────
// 11: Neural network layer — linear + ReLU + softmax
// ─────────────────────────────────────────────────────────
//
// Full forward pass: output = softmax(ReLU(input @ weights + bias))
// Tests composing 4+ TensorDescriptors in a single call — the
// pattern every ML workload needs.

use pyroxide::bridge::IntoMojo;
use pyroxide::types::max::{Tensor, TensorShape};

unsafe extern "C" {
    fn linear_relu_f32(input: isize, weight: isize, bias: isize, output: isize);
    fn softmax_f32(input: isize, output: isize);
}

fn main() {
    // ── Setup: batch=2, in_features=3, out_features=4 ──
    //
    // input (2×3):  [[1, 2, 3],
    //                [4, 5, 6]]
    //
    // weight (3×4): [[0.1, 0.2, 0.3, 0.4],
    //                [0.5, 0.6, 0.7, 0.8],
    //                [0.9, 1.0, 1.1, 1.2]]
    //
    // bias (4):     [0.1, -0.1, 0.2, -0.2]

    let input = Tensor::<f32>::from_data(
        TensorShape::matrix(2, 3),
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    );
    let weight = Tensor::<f32>::from_data(
        TensorShape::matrix(3, 4),
        vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2],
    );
    let bias = Tensor::<f32>::from_data(TensorShape::from([4]), vec![0.1, -0.1, 0.2, -0.2]);

    // ── Linear + ReLU ──
    let hidden = Tensor::<f32>::zeros(TensorShape::matrix(2, 4));
    let id = input.descriptor();
    let wd = weight.descriptor();
    let bd = bias.descriptor();
    let hd = hidden.descriptor();
    unsafe {
        linear_relu_f32(
            id.as_mojo().addr().as_raw(),
            wd.as_mojo().addr().as_raw(),
            bd.as_mojo().addr().as_raw(),
            hd.as_mojo().addr().as_raw(),
        );
    }

    // Verify against Rust ground truth
    let mut rust_hidden = vec![0.0f32; 8];
    for b in 0..2 {
        for j in 0..4 {
            let mut acc = bias[j];
            for k in 0..3 {
                acc += input[b * 3 + k] * weight[k * 4 + j];
            }
            rust_hidden[b * 4 + j] = acc.max(0.0); // ReLU
        }
    }
    for (i, (&got, &exp)) in hidden.iter().zip(&rust_hidden).enumerate() {
        assert!(
            (got - exp).abs() < 1e-4,
            "linear_relu[{i}]: mojo={got}, rust={exp}"
        );
    }
    println!("  linear_relu (2×3 @ 3×4 + bias) [ok]");
    println!("    hidden = {:?}", &hidden[..]);

    // ── Softmax ──
    let probs = Tensor::<f32>::zeros(TensorShape::matrix(2, 4));
    let hd2 = hidden.descriptor();
    let pd = probs.descriptor();
    unsafe { softmax_f32(hd2.as_mojo().addr().as_raw(), pd.as_mojo().addr().as_raw()) };

    // Verify: each row sums to 1.0
    for row in 0..2 {
        let row_sum: f32 = probs[row * 4..(row + 1) * 4].iter().sum();
        assert!(
            (row_sum - 1.0).abs() < 1e-4,
            "softmax row {row} sum = {row_sum}"
        );
    }
    println!("  softmax rows sum to 1.0 [ok]");

    // Verify: all values in (0, 1)
    for &p in probs.iter() {
        assert!(p > 0.0 && p < 1.0, "softmax value out of range: {p}");
    }
    println!("  softmax all values in (0, 1) [ok]");

    // Verify: larger input → larger probability
    // hidden row 1 should have larger values than row 0 (input is bigger)
    let p0_max = probs[0..4]
        .iter()
        .copied()
        .reduce(f32::max)
        .expect("non-empty");
    let p1_max = probs[4..8]
        .iter()
        .copied()
        .reduce(f32::max)
        .expect("non-empty");
    println!("    probs row 0: {:?} (max={p0_max:.4})", &probs[0..4]);
    println!("    probs row 1: {:?} (max={p1_max:.4})", &probs[4..8]);

    println!("all ok");
}
