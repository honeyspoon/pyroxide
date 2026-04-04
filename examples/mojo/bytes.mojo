# Raw byte operations: u8 arrays, i64 arrays, memset, memcmp.
# Tests non-float data types across FFI.

@export
def byte_sum(addr: Int, n: Int) -> Int:
    """Sum of u8 array as Int."""
    var data = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=addr)
    var total: Int = 0
    for i in range(n):
        total += Int(data[i])
    return total

@export
def memset_byte(addr: Int, n: Int, val: UInt8):
    """Fill n bytes with val."""
    var data = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(n):
        data[i] = val

@export
def byte_xor(a_addr: Int, b_addr: Int, out_addr: Int, n: Int):
    """XOR two byte arrays into output."""
    var a = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=b_addr)
    var out = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=out_addr)
    for i in range(n):
        out[i] = a[i] ^ b[i]

@export
def prefix_sum_i64(addr: Int, n: Int):
    """In-place prefix sum on i64 array."""
    var data = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(1, n):
        data[i] = data[i] + data[i - 1]
