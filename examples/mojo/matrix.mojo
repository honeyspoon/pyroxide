# Matrix operations beyond matmul: transpose, element-wise, trace.
# All matrices are row-major f64 with (rows, cols) passed as scalars.

@export
def transpose_f64(src: Int, dst: Int, rows: Int, cols: Int):
    """Transpose: dst[j][i] = src[i][j]."""
    var s = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=src)
    var d = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=dst)
    for i in range(rows):
        for j in range(cols):
            d[j * rows + i] = s[i * cols + j]

@export
def elementwise_mul_f64(a: Int, b: Int, dst: Int, n: Int):
    """Hadamard product: out[i] = a[i] * b[i]."""
    var ap = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a)
    var bp = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b)
    var dp = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=dst)
    for i in range(n):
        dp[i] = ap[i] * bp[i]

@export
def matrix_trace_f64(addr: Int, n: Int) -> Float64:
    """Trace of an n×n square matrix."""
    var m = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var total: Float64 = 0.0
    for i in range(n):
        total += m[i * n + i]
    return total
