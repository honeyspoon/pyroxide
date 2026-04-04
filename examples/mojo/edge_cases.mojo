# Edge cases: zero-length, empty slices, special float values in structs.

@export
def sum_or_zero(addr: Int, n: Int) -> Float64:
    """Sum n elements. If n==0, returns 0.0 (tests zero-length slice)."""
    if n == 0:
        return 0.0
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var total: Float64 = 0.0
    for i in range(n):
        total += data[i]
    return total

@export
def count_nans(addr: Int, n: Int) -> Int:
    """Count NaN values in a f64 array."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var count: Int = 0
    for i in range(n):
        # NaN != NaN
        if data[i] != data[i]:
            count += 1
    return count

@export
def roundtrip_struct(src: Int, dst: Int, n_bytes: Int):
    """Copy n_bytes from src to dst (tests raw byte-level struct copy)."""
    var s = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=src)
    var d = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=dst)
    for i in range(n_bytes):
        d[i] = s[i]
