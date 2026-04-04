# Point: [x: f64, y: f64], Color: [r: f64, g: f64, b: f64]

@export
def point_distance(a_addr: Int, b_addr: Int) -> Float64:
    var a = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=a_addr)
    var b = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=b_addr)
    var dx = a[0] - b[0]
    var dy = a[1] - b[1]
    return (dx * dx + dy * dy) ** 0.5

@export
def color_luminance(addr: Int) -> Float64:
    var c = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]

@export
def color_invert(addr: Int):
    var c = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    c[0] = 1.0 - c[0]
    c[1] = 1.0 - c[1]
    c[2] = 1.0 - c[2]
