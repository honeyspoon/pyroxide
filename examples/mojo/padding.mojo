# Struct padding: test that Rust and Mojo agree on field offsets
# when fields have different sizes (alignment padding matters).
#
# MixedStruct layout (Rust #[repr(C)]):
#   offset 0:  flag (u8)
#   offset 1:  _pad1 (7 bytes)
#   offset 8:  value (f64)
#   offset 16: count (i32)
#   offset 20: _pad2 (4 bytes)
#   total: 24 bytes

@export
def read_mixed(addr: Int) -> Float64:
    """Read the f64 'value' field from a padded struct."""
    var p = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=addr)
    # flag is at offset 0
    var flag = p[0]
    # value is at offset 8 (after 7 bytes padding)
    var value_ptr = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr + 8)
    # count is at offset 16
    var count_ptr = UnsafePointer[Int32, MutExternalOrigin](unsafe_from_address=addr + 16)

    if flag == 0:
        return 0.0
    return value_ptr[0] * Float64(Int(count_ptr[0]))

@export
def write_mixed(addr: Int, flag: UInt8, value: Float64, count: Int32):
    """Write fields into a padded struct at correct offsets."""
    var p = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=addr)
    p[0] = flag
    var value_ptr = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr + 8)
    value_ptr[0] = value
    var count_ptr = UnsafePointer[Int32, MutExternalOrigin](unsafe_from_address=addr + 16)
    count_ptr[0] = count
