# Nested structs: a Line is two Points, a Triangle is three Points.
# Layout (all f64):
#   Point:    [x, y]         = 2 x f64 = 16 bytes
#   Line:     [p1.x, p1.y, p2.x, p2.y] = 4 x f64 = 32 bytes
#   Triangle: [a.x, a.y, b.x, b.y, c.x, c.y] = 6 x f64 = 48 bytes

@export
def line_length(addr: Int) -> Float64:
    """Length of a line segment (two Points)."""
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var dx = p[2] - p[0]
    var dy = p[3] - p[1]
    return (dx * dx + dy * dy) ** 0.5

@export
def triangle_area(addr: Int) -> Float64:
    """Area of a triangle (three Points) using the shoelace formula."""
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var ax = p[0]
    var ay = p[1]
    var bx = p[2]
    var by = p[3]
    var cx = p[4]
    var cy = p[5]
    var area = (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)) / 2.0
    if area < 0.0:
        area = -area
    return area

@export
def centroid(tri_addr: Int, out_addr: Int):
    """Centroid of a triangle → writes a Point to out_addr."""
    var t = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=tri_addr)
    var out = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=out_addr)
    out[0] = (t[0] + t[2] + t[4]) / 3.0
    out[1] = (t[1] + t[3] + t[5]) / 3.0
