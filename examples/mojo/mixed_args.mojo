# Mixed argument types: scalars, pointers, bools in one call.
# Tests the full range of the ABI in a single function.

@export
def weighted_blend(
    src_a: Int,    # f64 array pointer
    src_b: Int,    # f64 array pointer
    dst: Int,      # f64 array pointer (output)
    n: Int,        # length
    weight: Float64, # blend factor 0.0-1.0
    clamp: Bool,   # whether to clamp output to [0,1]
) -> Float64:
    """Blend two arrays: dst = a*weight + b*(1-weight). Returns max output value."""
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=src_a)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=src_b)
    var out = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=dst)
    var max_val: Float64 = -1e308

    for i in range(n):
        var v = a[i] * weight + b[i] * (1.0 - weight)
        if clamp:
            if v < 0.0:
                v = 0.0
            if v > 1.0:
                v = 1.0
        out[i] = v
        if v > max_val:
            max_val = v

    return max_val

@export
def find_threshold(
    data: Int,     # f64 array
    n: Int,        # length
    threshold: Float64,
    above: Bool,   # true = find first above, false = find first below
) -> Int:
    """Find index of first element above/below threshold. Returns -1 if not found."""
    var arr = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=data)
    for i in range(n):
        if above and arr[i] > threshold:
            return i
        if not above and arr[i] < threshold:
            return i
    return -1
