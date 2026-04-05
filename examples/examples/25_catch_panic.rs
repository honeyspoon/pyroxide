// ─────────────────────────────────────────────────────────
// 25: catch_panic_at_ffi — prevent Rust panics from crossing FFI
// ─────────────────────────────────────────────────────────
//
// catch_panic_at_ffi catches RUST PANICS only. It does NOT catch:
//   - Mojo errors (those segfault if uncaught)
//   - Hardware exceptions (segfaults, SIGBUS)
//
// Use case: wrap Rust code that Mojo calls back into, so a panic
// in the callback returns a default value instead of UB.
//
// This example also tests Mojo writing string data into a Rust buffer.

use pyroxide::prelude::*;

unsafe extern "C" {
    fn safe_sqrt(x: f64) -> f64;
    fn write_greeting(name_ptr: isize, name_len: isize, buf_ptr: isize, buf_len: isize) -> isize;
}

// This simulates what a Rust callback exported to Mojo would look like:
#[allow(dead_code, reason = "demonstration of the callback pattern")]
extern "C" fn example_callback(x: f64) -> f64 {
    // If the Rust code inside panics, this prevents UB
    catch_panic_at_ffi(|| {
        assert!(x >= 0.0, "negative input");
        x.sqrt()
    })
}

fn main() {
    // ── Demonstrate panic recovery ──
    // The real use case is callbacks, but we can test the mechanism directly.
    let panicked: f64 = catch_panic_at_ffi(|| {
        panic!("intentional test panic");
    });
    assert_eq!(panicked, 0.0); // f64::default()
    println!("  panic caught, returned default (0.0) [ok]");

    let ok = catch_panic_at_ffi(|| 42.0f64);
    assert_eq!(ok, 42.0);
    println!("  no panic, returned value (42.0) [ok]");

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
