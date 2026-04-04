# Minimal functions for measuring raw FFI call overhead.

@export
def noop():
    """Do nothing. Measures pure call overhead."""
    pass

@export
def identity_i64(x: Int) -> Int:
    """Return the argument unchanged."""
    return x

@export
def add_one(x: Int) -> Int:
    return x + 1

@export
def is_even(x: Int) -> Bool:
    return x % 2 == 0

@export
def ptr_stable_check(addr: Int, expected: Float64) -> Bool:
    """Read a f64 from the pointer and check it matches expected."""
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    return p[0] == expected
