# Large data: operations on 1M+ element arrays.

@export
def dot_f64_large(a_addr: Int, b_addr: Int, n: Int) -> Float64:
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_addr)
    var total: Float64 = 0.0
    for i in range(n):
        total += a[i] * b[i]
    return total

@export
def scale_add_f64(x_addr: Int, y_addr: Int, out_addr: Int, n: Int, alpha: Float64, beta: Float64):
    """out = alpha * x + beta * y (BLAS-style axpby)."""
    var x = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=x_addr)
    var y = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=y_addr)
    var out = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=out_addr)
    for i in range(n):
        out[i] = alpha * x[i] + beta * y[i]

@export
def reduce_max_f64(addr: Int, n: Int) -> Float64:
    if n == 0:
        return -1e308
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var m = data[0]
    for i in range(1, n):
        if data[i] > m:
            m = data[i]
    return m
