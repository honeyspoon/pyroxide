# Pure functions for concurrent calling from Rust threads.
# No shared state — tests whether Mojo FFI is reentrant.

@export
def pure_dot(a_addr: Int, b_addr: Int, n: Int) -> Float64:
    """Pure function: reads two arrays, returns dot product."""
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_addr)
    var total: Float64 = 0.0
    for i in range(n):
        total += a[i] * b[i]
    return total

@export
def pure_sum_sq(addr: Int, n: Int) -> Float64:
    """Pure function: sum of squares."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var total: Float64 = 0.0
    for i in range(n):
        total += data[i] * data[i]
    return total
