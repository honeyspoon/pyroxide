// ─────────────────────────────────────────────────────────
// 09: Image processing — box blur on an RGB pixel buffer
// ─────────────────────────────────────────────────────────
//
// Pass a flat f32 pixel buffer (height × width × 3) to Mojo for
// a 3×3 box blur. Tests large mutable buffers through the FFI.

use pyroxide::bridge::{MojoSlice, MojoSliceMut};

unsafe extern "C" {
    fn box_blur_rgb(src: isize, dst: isize, width: isize, height: isize);
    fn brightness_f32(img: isize, n_pixels: isize) -> f32;
}

fn main() {
    let width: usize = 8;
    let height: usize = 6;
    let n_pixels = width * height;

    // Create a test image: checkerboard (high-frequency, blur will visibly change it)
    let src: Vec<f32> = (0..n_pixels)
        .flat_map(|i| {
            let x = i % width;
            let y = i / width;
            let v = if (x + y).is_multiple_of(2) {
                1.0f32
            } else {
                0.0
            };
            [v, v, v]
        })
        .collect();
    let mut dst = vec![0.0f32; n_pixels * 3];

    // Blur via Mojo — src is read-only (MojoSlice), dst is written (MojoSliceMut)
    let src_s = MojoSlice::new(&src);
    let dst_s = MojoSliceMut::new(&mut dst);
    unsafe {
        box_blur_rgb(
            src_s.as_raw(),
            dst_s.as_raw(),
            width as isize,
            height as isize,
        );
    }

    // Interior pixels should be averaged (not equal to source)
    let mid = (3 * width + 4) * 3;
    assert!(
        (dst[mid] - src[mid]).abs() > 0.001,
        "blur should change interior pixels"
    );
    println!("  box_blur changed interior pixels [ok]");

    // Edge pixels should still have valid values
    assert!(dst[0] >= 0.0 && dst[0] <= 1.0);
    assert!(*dst.last().expect("non-empty") >= 0.0 && *dst.last().expect("non-empty") <= 1.0);
    println!("  edge pixels in valid range [ok]");

    // Brightness should be roughly preserved by box blur
    let src_bright = unsafe { brightness_f32(src_s.as_raw(), n_pixels as isize) };
    let dst_bright = unsafe { brightness_f32(MojoSlice::new(&dst).as_raw(), n_pixels as isize) };
    assert!(
        (src_bright - dst_bright).abs() < 0.05,
        "brightness should be preserved: src={src_bright:.4}, dst={dst_bright:.4}"
    );
    println!("  brightness preserved: {src_bright:.4} → {dst_bright:.4} [ok]");

    // Blur should reduce variance (smoothing)
    let variance = |data: &[f32]| -> f32 {
        let mean = data.iter().sum::<f32>() / data.len() as f32;
        data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / data.len() as f32
    };
    let src_var = variance(&src);
    let dst_var = variance(&dst);
    assert!(dst_var < src_var, "blur should reduce variance");
    println!("  variance reduced: {src_var:.4} → {dst_var:.4} [ok]");

    println!("all ok");
}
