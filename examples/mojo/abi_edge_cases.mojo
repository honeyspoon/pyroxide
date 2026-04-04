# ABI edge case tests — Bool, out-pointers, boundary values.

@export
def negate(b: Bool) -> Bool:
    return not b

@export
def is_positive(x: Float64) -> Bool:
    return x > 0.0

@export
def echo_int(x: Int) -> Int:
    return x

@export
def echo_f64(x: Float64) -> Float64:
    return x

@export
def max_int() -> Int:
    return Int.MAX

@export
def min_int() -> Int:
    return Int.MIN

@export
def divmod_out(a: Int, b: Int, quot_ptr: Int, rem_ptr: Int):
    """Multiple return values via out-pointers."""
    var q = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=quot_ptr)
    var r = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=rem_ptr)
    q[0] = Int64(a // b)
    r[0] = Int64(a % b)

@export
def swap_f64(a_ptr: Int, b_ptr: Int):
    """In-place swap of two f64 values."""
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_ptr)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_ptr)
    var tmp = a[0]
    a[0] = b[0]
    b[0] = tmp

@export
def safe_div(a: Float64, b: Float64) -> Float64:
    """Division with error handling — never raises across FFI."""
    try:
        return _div_impl(a, b)
    except:
        return Float64.MAX_FINITE * -1.0  # sentinel for error

def _div_impl(a: Float64, b: Float64) raises -> Float64:
    if b == 0.0:
        raise "division by zero"
    return a / b
