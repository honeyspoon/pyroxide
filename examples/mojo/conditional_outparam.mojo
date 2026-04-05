# Functions that only sometimes write to out-params.
# Tests the OutParam soundness boundary.

@export
def find_first_above(addr: Int, n: Int, threshold: Float64, result_addr: Int) -> Bool:
    """Find first element > threshold. Writes index to result_addr.
    Returns true if found, false if not.
    IMPORTANT: always writes to result_addr (sentinel -1 if not found)."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var result = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=result_addr)
    for i in range(n):
        if data[i] > threshold:
            result[0] = Int64(i)
            return True
    result[0] = -1  # always write sentinel
    return False

@export
def try_divide(a: Float64, b: Float64, result_addr: Int) -> Bool:
    """Divide a/b. Writes result to result_addr.
    Returns false and writes 0.0 if b==0."""
    var result = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=result_addr)
    if b == 0.0:
        result[0] = 0.0  # always write something
        return False
    result[0] = a / b
    return True
