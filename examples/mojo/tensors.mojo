# TensorDescriptor layout: dtype(0) rank(8) dims(16) strides(80) data_ptr(144)

def _unpack(desc_addr: Int) -> Tuple[Int, Int]:
    var p = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=desc_addr)
    var rank = Int(p[1])
    var numel: Int = 1
    for i in range(rank):
        numel *= Int(p[2 + i])
    return (numel, Int(p[18]))

@export
def tensor_sum_f64(desc_addr: Int) -> Float64:
    var meta = _unpack(desc_addr)
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=meta[1])
    var total: Float64 = 0.0
    for i in range(meta[0]):
        total += data[i]
    return total

@export
def tensor_dot_f64(a_addr: Int, b_addr: Int) -> Float64:
    var a_meta = _unpack(a_addr)
    var b_meta = _unpack(b_addr)
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_meta[1])
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_meta[1])
    var total: Float64 = 0.0
    for i in range(a_meta[0]):
        total += a[i] * b[i]
    return total

@export
def tensor_matmul_f32(a_addr: Int, b_addr: Int, out_addr: Int):
    var ap = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=a_addr)
    var bp = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=b_addr)
    var op = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=out_addr)
    var m = Int(ap[2])
    var k = Int(ap[3])
    var n = Int(bp[3])
    var a = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=Int(ap[18]))
    var b = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=Int(bp[18]))
    var out = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=Int(op[18]))
    for i in range(m):
        for j in range(n):
            var acc: Float32 = 0.0
            for p in range(k):
                acc += a[i * k + p] * b[p * n + j]
            out[i * n + j] = acc
