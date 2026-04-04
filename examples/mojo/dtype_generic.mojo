def _sum_impl[dtype: DType](addr: Int, n: Int) -> Scalar[dtype]:
    var data = UnsafePointer[Scalar[dtype], MutExternalOrigin](unsafe_from_address=addr)
    var total = Scalar[dtype](0)
    for i in range(n):
        total += data[i]
    return total

@export
def sum_f32(addr: Int, n: Int) -> Float32:
    return _sum_impl[DType.float32](addr, n)

@export
def sum_f64(addr: Int, n: Int) -> Float64:
    return _sum_impl[DType.float64](addr, n)

@export
def sum_i32(addr: Int, n: Int) -> Int32:
    return _sum_impl[DType.int32](addr, n)
