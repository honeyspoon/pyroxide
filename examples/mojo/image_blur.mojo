# Image blur: 3x3 box blur on an RGB f32 image.
# Image layout: flat [height * width * 3] row-major, channels last.
# Rust passes (src_ptr, dst_ptr, width, height).

@export
def box_blur_rgb(src_addr: Int, dst_addr: Int, width: Int, height: Int):
    var src = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=src_addr)
    var dst = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=dst_addr)

    for y in range(height):
        for x in range(width):
            for c in range(3):
                var sum: Float32 = 0.0
                var count: Float32 = 0.0
                for dy in range(-1, 2):
                    for dx in range(-1, 2):
                        var ny = y + dy
                        var nx = x + dx
                        if 0 <= ny < height and 0 <= nx < width:
                            sum += src[(ny * width + nx) * 3 + c]
                            count += 1.0
                dst[(y * width + x) * 3 + c] = sum / count

@export
def brightness_f32(img_addr: Int, n_pixels: Int) -> Float32:
    """Average brightness across all channels."""
    var img = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=img_addr)
    var total: Float32 = 0.0
    var count = n_pixels * 3
    for i in range(count):
        total += img[i]
    return total / Float32(count)
