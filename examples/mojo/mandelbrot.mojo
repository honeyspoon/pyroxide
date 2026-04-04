# Mandelbrot set: compute escape iterations for a grid of points.
# Rust allocates the output buffer, Mojo fills it.

@export
def mandelbrot(
    out_addr: Int,
    width: Int, height: Int,
    x_min: Float64, x_max: Float64,
    y_min: Float64, y_max: Float64,
    max_iter: Int,
):
    """Compute Mandelbrot escape iterations for a width×height grid."""
    var out = UnsafePointer[Int32, MutExternalOrigin](unsafe_from_address=out_addr)

    var dx = (x_max - x_min) / Float64(width)
    var dy = (y_max - y_min) / Float64(height)

    for row in range(height):
        var cy = y_min + Float64(row) * dy
        for col in range(width):
            var cx = x_min + Float64(col) * dx
            var zx: Float64 = 0.0
            var zy: Float64 = 0.0
            var iter: Int = 0
            while zx * zx + zy * zy <= 4.0 and iter < max_iter:
                var tmp = zx * zx - zy * zy + cx
                zy = 2.0 * zx * zy + cy
                zx = tmp
                iter += 1
            out[row * width + col] = Int32(iter)
