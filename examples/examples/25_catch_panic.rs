// ─────────────────────────────────────────────────────────
// 25: catch_panic_at_ffi — panic-safe FFI + string output
// ─────────────────────────────────────────────────────────
//
// First example that actually uses catch_panic_at_ffi (it was in the
// prelude but never demonstrated). Also tests Mojo writing string
// data into a Rust buffer (reverse of example 10).

use pyroxide::prelude::*;

unsafe extern "C" {
    fn safe_sqrt(x: f64) -> f64;
    fn write_greeting(name_ptr: isize, name_len: isize, buf_ptr: isize, buf_len: isize) -> isize;
}

fn main() {
    // ── catch_panic_at_ffi: wrap FFI in panic-safe boundary ──
    // This is how Rust callbacks exported to Mojo should be wrapped.
    // Here we use it to demonstrate the pattern, even though we're
    // calling Mojo (not being called by Mojo).

    let result = catch_panic_at_ffi(|| unsafe { safe_sqrt(25.0) });
    assert!((result - 5.0).abs() < 1e-6);
    println!("  catch_panic_at_ffi(sqrt(25)) = {result:.1} [ok]");

    let sentinel = catch_panic_at_ffi(|| unsafe { safe_sqrt(-1.0) });
    assert_eq!(sentinel, -1.0);
    println!("  catch_panic_at_ffi(sqrt(-1)) = {sentinel} (sentinel) [ok]");

    // If the closure panics, catch_panic_at_ffi returns Default::default()
    let panicked = catch_panic_at_ffi(|| -> f64 {
        if true {
            panic!("intentional test panic");
        }
        42.0
    });
    assert_eq!(panicked, 0.0); // f64::default() = 0.0
    println!("  catch_panic_at_ffi(panic) = {panicked} (default) [ok]");

    // ── String output: Mojo writes into Rust buffer ──
    let name = "Rust";
    let s = MojoStr::new(name);
    let mut buf = vec![0u8; 64];

    let bytes_written = unsafe {
        write_greeting(
            s.as_raw(),
            s.len_isize(),
            buf.as_mut_ptr() as isize,
            buf.len() as isize,
        )
    };

    assert!(bytes_written > 0);
    let greeting = std::str::from_utf8(&buf[..bytes_written as usize]).expect("valid UTF-8");
    assert_eq!(greeting, "Hello, Rust!");
    println!("  write_greeting(\"Rust\") = \"{greeting}\" [ok]");

    // Buffer too small → -1 sentinel
    let tiny = unsafe { write_greeting(s.as_raw(), s.len_isize(), buf.as_mut_ptr() as isize, 3) };
    assert_eq!(tiny, -1);
    println!("  write_greeting(buf_too_small) = -1 [ok]");

    println!("all ok");
}
