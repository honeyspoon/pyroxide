# Arrays of structs: operate on Point[] and Particle[] arrays.
# Point layout: [x: f64, y: f64] = 16 bytes each
# Particle layout: [x: f64, y: f64, vx: f64, vy: f64, mass: f64] = 40 bytes each

@export
def closest_pair_distance(points_addr: Int, n: Int) -> Float64:
    """Find the minimum distance between any two points (brute force)."""
    var pts = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=points_addr)
    var min_dist: Float64 = 1e308
    for i in range(n):
        for j in range(i + 1, n):
            var dx = pts[j * 2] - pts[i * 2]
            var dy = pts[j * 2 + 1] - pts[i * 2 + 1]
            var d = (dx * dx + dy * dy) ** 0.5
            if d < min_dist:
                min_dist = d
    return min_dist

@export
def step_particles(addr: Int, n: Int, dt: Float64):
    """Euler integration step: position += velocity * dt. In-place."""
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    # Each particle: [x, y, vx, vy, mass] = 5 fields
    for i in range(n):
        var base = i * 5
        p[base + 0] += p[base + 2] * dt  # x += vx * dt
        p[base + 1] += p[base + 3] * dt  # y += vy * dt

@export
def total_kinetic_energy(addr: Int, n: Int) -> Float64:
    """Sum of 0.5 * mass * |v|^2 for all particles."""
    var p = UnsafePointer[Float64, MutExternalOrigin](unsafe_from_address=addr)
    var total: Float64 = 0.0
    for i in range(n):
        var base = i * 5
        var vx = p[base + 2]
        var vy = p[base + 3]
        var mass = p[base + 4]
        total += 0.5 * mass * (vx * vx + vy * vy)
    return total
