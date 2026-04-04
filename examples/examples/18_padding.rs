// ─────────────────────────────────────────────────────────
// 18: Struct padding — mixed-type fields with alignment gaps
// ─────────────────────────────────────────────────────────
//
// #[repr(C)] inserts padding between fields of different sizes.
// Mojo must read at the correct byte offsets, not just field indices.
// This is the test that catches alignment bugs.

use pyroxide::prelude::*;

mojo_type! {
    /// u8 + explicit padding + f64 + i32 + explicit padding = 24 bytes.
    /// Zerocopy requires ALL bytes to be accounted for — no implicit padding.
    /// This is a feature: it forces you to be explicit about the layout.
    pub struct MixedStruct {
        pub flag: u8,
        pub _pad1: [u8; 7],   // explicit padding to align value
        pub value: f64,
        pub count: i32,
        pub _pad2: [u8; 4],   // explicit padding to align struct
    }
}

unsafe extern "C" {
    fn read_mixed(addr: isize) -> f64;
    fn write_mixed(addr: isize, flag: u8, value: f64, count: i32);
}

fn main() {
    // Verify layout
    assert_eq!(
        std::mem::size_of::<MixedStruct>(),
        24,
        "sizeof should be 24 (with padding)"
    );
    println!("  sizeof(MixedStruct) = 24 [ok]");

    // Read: Mojo reads fields at correct padded offsets
    let s = MixedStruct {
        flag: 1,
        _pad1: [0; 7],
        value: 3.14,
        count: 10,
        _pad2: [0; 4],
    };
    let result = unsafe { read_mixed(s.as_raw()) };
    assert!(
        (result - 31.4).abs() < 1e-6,
        "3.14 * 10 = 31.4, got {result}"
    );
    println!("  read_mixed(flag=1, value=3.14, count=10) = {result:.1} [ok]");

    // Read with flag=0 should return 0
    let s2 = MixedStruct {
        flag: 0,
        _pad1: [0; 7],
        value: 99.0,
        count: 99,
        _pad2: [0; 4],
    };
    let result2 = unsafe { read_mixed(s2.as_raw()) };
    assert_eq!(result2, 0.0);
    println!("  read_mixed(flag=0) = 0.0 [ok]");

    // Write: Mojo writes fields at correct padded offsets
    let mut out = MixedStruct {
        flag: 0,
        _pad1: [0; 7],
        value: 0.0,
        count: 0,
        _pad2: [0; 4],
    };
    unsafe { write_mixed(out.as_raw_mut(), 42, 2.718, 7) };
    assert_eq!(out.flag, 42);
    assert!((out.value - 2.718).abs() < 1e-10);
    assert_eq!(out.count, 7);
    println!(
        "  write_mixed: flag={}, value={:.3}, count={} [ok]",
        out.flag, out.value, out.count
    );

    println!("all ok");
}
