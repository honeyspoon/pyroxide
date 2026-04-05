// ─────────────────────────────────────────────────────────
// 27: Every scalar type round-tripped through FFI
// ─────────────────────────────────────────────────────────
//
// The ABI table claims i8–i64, u8–u64, f32, f64, bool all work.
// This example verifies each one including MIN/MAX boundaries.

unsafe extern "C" {
    fn echo_i8(x: i8) -> i8;
    fn echo_i16(x: i16) -> i16;
    fn echo_i32(x: i32) -> i32;
    fn echo_i64(x: i64) -> i64;
    fn echo_u8(x: u8) -> u8;
    fn echo_u16(x: u16) -> u16;
    fn echo_u32(x: u32) -> u32;
    fn echo_u64(x: u64) -> u64;
    fn echo_f32(x: f32) -> f32;
    fn echo_f64(x: f64) -> f64;
    fn echo_bool(x: bool) -> bool;
    fn add_i8(a: i8, b: i8) -> i8;
    fn add_u16(a: u16, b: u16) -> u16;
}

fn main() {
    // Integer round-trips with boundary values
    assert_eq!(unsafe { echo_i8(0) }, 0);
    assert_eq!(unsafe { echo_i8(i8::MAX) }, i8::MAX);
    assert_eq!(unsafe { echo_i8(i8::MIN) }, i8::MIN);
    println!("  i8: 0, MAX={}, MIN={} [ok]", i8::MAX, i8::MIN);

    assert_eq!(unsafe { echo_i16(i16::MAX) }, i16::MAX);
    assert_eq!(unsafe { echo_i16(i16::MIN) }, i16::MIN);
    println!("  i16: MAX, MIN [ok]");

    assert_eq!(unsafe { echo_i32(i32::MAX) }, i32::MAX);
    assert_eq!(unsafe { echo_i32(i32::MIN) }, i32::MIN);
    println!("  i32: MAX, MIN [ok]");

    assert_eq!(unsafe { echo_i64(i64::MAX) }, i64::MAX);
    assert_eq!(unsafe { echo_i64(i64::MIN) }, i64::MIN);
    println!("  i64: MAX, MIN [ok]");

    assert_eq!(unsafe { echo_u8(0) }, 0);
    assert_eq!(unsafe { echo_u8(u8::MAX) }, u8::MAX);
    println!("  u8: 0, MAX={} [ok]", u8::MAX);

    assert_eq!(unsafe { echo_u16(u16::MAX) }, u16::MAX);
    println!("  u16: MAX [ok]");

    assert_eq!(unsafe { echo_u32(u32::MAX) }, u32::MAX);
    println!("  u32: MAX [ok]");

    assert_eq!(unsafe { echo_u64(u64::MAX) }, u64::MAX);
    println!("  u64: MAX [ok]");

    // Float round-trips
    assert_eq!(unsafe { echo_f32(3.14) }, 3.14);
    assert_eq!(unsafe { echo_f32(f32::MAX) }, f32::MAX);
    assert!(unsafe { echo_f32(f32::NAN) }.is_nan());
    println!("  f32: 3.14, MAX, NaN [ok]");

    assert_eq!(unsafe { echo_f64(2.718) }, 2.718);
    println!("  f64: 2.718 [ok]");

    // Bool
    assert!(unsafe { echo_bool(true) });
    assert!(!unsafe { echo_bool(false) });
    println!("  bool: true, false [ok]");

    // Arithmetic on narrow types
    assert_eq!(unsafe { add_i8(100, 20) }, 120);
    assert_eq!(unsafe { add_u16(60000, 5000) }, 65000);
    println!("  add_i8(100,20)=120, add_u16(60000,5000)=65000 [ok]");

    println!("all ok");
}
