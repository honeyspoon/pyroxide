@export
def dot_unrolled_4(a_addr: Int, b_addr: Int) -> Float64:
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_addr)
    var total: Float64 = 0.0
    comptime
    for i in range(4):
        total += a[i] * b[i]
    return total

comptime GOLDEN_RATIO: Float64 = 1.6180339887498949

@export
def get_golden_ratio() -> Float64:
    return GOLDEN_RATIO

@export
def fibonacci_ratio(n: Int) -> Float64:
    if n <= 0:
        return Float64(0.0)
    var a: Float64 = 1.0
    var b: Float64 = 1.0
    for _ in range(n):
        var tmp = b
        b = a + b
        a = tmp
    return b / a
