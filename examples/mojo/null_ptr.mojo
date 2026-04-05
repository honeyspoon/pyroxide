# Null pointer behavior: what happens when addr=0?

@export
def safe_sum(addr: Int, n: Int) -> Float64:
    """Sum with null guard. Returns 0.0 if n==0 (avoids dereferencing addr)."""
    if n == 0:
        return 0.0
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var total: Float64 = 0.0
    for i in range(n):
        total += data[i]
    return total

@export
def safe_write(addr: Int, n: Int, val: Float64) -> Bool:
    """Write val to all elements. Returns false if n==0."""
    if n == 0:
        return False
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(n):
        data[i] = val
    return True
