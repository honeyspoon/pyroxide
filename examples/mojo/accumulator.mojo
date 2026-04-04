# Stateful accumulator: Rust owns the state struct, Mojo updates it.
#
# Stats layout (repr(C), all f64):
#   offset 0:  count
#   offset 8:  sum
#   offset 16: min
#   offset 24: max
#   offset 32: sum_sq (for variance)

@export
def stats_init(state_addr: Int):
    """Initialize stats to neutral values."""
    var s = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=state_addr)
    s[0] = 0.0       # count
    s[1] = 0.0       # sum
    s[2] = 1e308      # min (start high)
    s[3] = -1e308     # max (start low)
    s[4] = 0.0       # sum_sq

@export
def stats_update(state_addr: Int, data_addr: Int, n: Int):
    """Process a chunk of data, updating running statistics."""
    var s = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=state_addr)
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=data_addr)

    for i in range(n):
        var v = data[i]
        s[0] += 1.0        # count
        s[1] += v           # sum
        if v < s[2]:
            s[2] = v        # min
        if v > s[3]:
            s[3] = v        # max
        s[4] += v * v       # sum_sq

@export
def stats_mean(state_addr: Int) -> Float64:
    var s = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=state_addr)
    if s[0] == 0.0:
        return 0.0
    return s[1] / s[0]

@export
def stats_variance(state_addr: Int) -> Float64:
    """Population variance: E[X²] - E[X]²."""
    var s = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=state_addr)
    if s[0] == 0.0:
        return 0.0
    var mean = s[1] / s[0]
    return s[4] / s[0] - mean * mean
