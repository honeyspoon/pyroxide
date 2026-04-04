# Chained operations: output of one call feeds into the next.
# Also tests error sentinels (NaN, -1).

@export
def normalize_f64(addr: Int, n: Int) -> Float64:
    """Normalize array to [0,1] range in-place. Returns the original max.
    Returns NaN if n==0 or all values are equal (sentinel for error)."""
    if n == 0:
        return Float64(0.0) / Float64(0.0)  # NaN
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var lo = data[0]
    var hi = data[0]
    for i in range(1, n):
        if data[i] < lo:
            lo = data[i]
        if data[i] > hi:
            hi = data[i]
    var range_val = hi - lo
    if range_val == 0.0:
        return Float64(0.0) / Float64(0.0)  # NaN
    for i in range(n):
        data[i] = (data[i] - lo) / range_val
    return hi

@export
def argmax_f64(addr: Int, n: Int) -> Int:
    """Index of maximum element. Returns -1 if n==0."""
    if n == 0:
        return -1
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var best: Int = 0
    for i in range(1, n):
        if data[i] > data[best]:
            best = i
    return best

@export
def histogram_u8(data_addr: Int, n: Int, bins_addr: Int):
    """Count occurrences of each byte value (256 bins)."""
    var data = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=data_addr)
    var bins = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=bins_addr)
    # Zero bins
    for i in range(256):
        bins[i] = 0
    for i in range(n):
        var idx = Int(data[i])
        bins[idx] = bins[idx] + 1
