@export
def dot_scalar(a_addr: Int, b_addr: Int, n: Int) -> Float32:
    var a = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=b_addr)
    var total: Float32 = 0.0
    for i in range(n):
        total += a[i] * b[i]
    return total

@export
def dot_simd(a_addr: Int, b_addr: Int, n: Int) -> Float32:
    var a = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=b_addr)
    comptime width: Int = 8
    var accum = SIMD[DType.float32, width]()
    var i: Int = 0
    while i + width <= n:
        accum += a.load[width=width](i) * b.load[width=width](i)
        i += width
    var total = accum.reduce_add()
    while i < n:
        total += a[i] * b[i]
        i += 1
    return total
