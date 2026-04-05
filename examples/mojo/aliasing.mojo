# Aliasing tests: same pointer as src and dst.

@export
def scale_inplace(addr: Int, n: Int, factor: Float64):
    """Multiply every element by factor. src == dst."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(n):
        data[i] = data[i] * factor

@export
def add_arrays_aliased(a_addr: Int, b_addr: Int, dst_addr: Int, n: Int):
    """dst[i] = a[i] + b[i]. a, b, dst may alias."""
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_addr)
    var dst = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=dst_addr)
    for i in range(n):
        dst[i] = a[i] + b[i]

@export
def shift_right(addr: Int, n: Int):
    """Shift elements right by 1 (overlapping read/write).
    Tests forward vs backward iteration with aliased pointers."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    # Must iterate backward to avoid overwriting unread data
    var i = n - 1
    while i > 0:
        data[i] = data[i - 1]
        i -= 1
    data[0] = 0.0
