# Multi-step pipeline: same buffer through different operations.
# Also tests enum-like dispatch via integer tag.

@export
def apply_op(addr: Int, n: Int, op: Int) -> Float64:
    """Apply operation to array based on op tag:
       0 = sum, 1 = product, 2 = mean, 3 = min, 4 = max.
       Returns the result. Returns -1e308 for unknown op."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    if op == 0:  # sum
        var total: Float64 = 0.0
        for i in range(n):
            total += data[i]
        return total
    elif op == 1:  # product
        var total: Float64 = 1.0
        for i in range(n):
            total *= data[i]
        return total
    elif op == 2:  # mean
        var total: Float64 = 0.0
        for i in range(n):
            total += data[i]
        return total / Float64(n)
    elif op == 3:  # min
        var m = data[0]
        for i in range(1, n):
            if data[i] < m:
                m = data[i]
        return m
    elif op == 4:  # max
        var m = data[0]
        for i in range(1, n):
            if data[i] > m:
                m = data[i]
        return m
    return -1e308

@export
def transform_inplace(addr: Int, n: Int, op: Int):
    """In-place transform: 0=negate, 1=abs, 2=square, 3=double."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(n):
        if op == 0:
            data[i] = -data[i]
        elif op == 1:
            if data[i] < 0.0:
                data[i] = -data[i]
        elif op == 2:
            data[i] = data[i] * data[i]
        elif op == 3:
            data[i] = data[i] * 2.0
