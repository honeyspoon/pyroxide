# Functions for testing catch_mojo_call and string output.

@export
def safe_sqrt(x: Float64) -> Float64:
    """Square root. Returns -1.0 for negative input (sentinel)."""
    if x < 0.0:
        return -1.0
    return x ** 0.5

@export
def write_greeting(name_ptr: Int, name_len: Int, buf_ptr: Int, buf_len: Int) -> Int:
    """Write 'Hello, <name>!' into buf. Returns bytes written, or -1 if too small."""
    var name = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=name_ptr)
    var buf = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=buf_ptr)

    var needed = 7 + name_len + 1
    if needed > buf_len:
        return -1

    # "Hello, " as raw bytes
    buf[0] = 72   # H
    buf[1] = 101  # e
    buf[2] = 108  # l
    buf[3] = 108  # l
    buf[4] = 111  # o
    buf[5] = 44   # ,
    buf[6] = 32   # space
    # Copy name
    for i in range(name_len):
        buf[7 + i] = name[i]
    # "!"
    buf[7 + name_len] = 33
    return 7 + name_len + 1
