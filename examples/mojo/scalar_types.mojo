# Round-trip every scalar type through FFI.

@export
def echo_i8(x: Int8) -> Int8:
    return x

@export
def echo_i16(x: Int16) -> Int16:
    return x

@export
def echo_i32(x: Int32) -> Int32:
    return x

@export
def echo_i64(x: Int64) -> Int64:
    return x

@export
def echo_u8(x: UInt8) -> UInt8:
    return x

@export
def echo_u16(x: UInt16) -> UInt16:
    return x

@export
def echo_u32(x: UInt32) -> UInt32:
    return x

@export
def echo_u64(x: UInt64) -> UInt64:
    return x

@export
def echo_f32(x: Float32) -> Float32:
    return x

@export
def echo_f64(x: Float64) -> Float64:
    return x

@export
def echo_bool(x: Bool) -> Bool:
    return x

@export
def add_i8(a: Int8, b: Int8) -> Int8:
    return a + b

@export
def add_u16(a: UInt16, b: UInt16) -> UInt16:
    return a + b
