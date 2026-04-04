# Sort an array in-place. Tests large mutable buffer + Mojo algorithms.

@export
def sort_f64(addr: Int, n: Int):
    """In-place insertion sort on Float64 array."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(1, n):
        var key = data[i]
        var j = i - 1
        while j >= 0 and data[j] > key:
            data[j + 1] = data[j]
            j -= 1
        data[j + 1] = key

@export
def is_sorted_f64(addr: Int, n: Int) -> Bool:
    """Check if array is sorted ascending."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    for i in range(1, n):
        if data[i] < data[i - 1]:
            return False
    return True

@export
def reverse_f64(addr: Int, n: Int):
    """Reverse array in-place."""
    var data = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var lo: Int = 0
    var hi = n - 1
    while lo < hi:
        var tmp = data[lo]
        data[lo] = data[hi]
        data[hi] = tmp
        lo += 1
        hi -= 1
