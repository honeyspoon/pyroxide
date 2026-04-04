// ─────────────────────────────────────────────────────────
// 01: Hello Mojo
// ─────────────────────────────────────────────────────────
//
// The absolute simplest Rust → Mojo call. One function, two
// integers, one result. This is where every journey starts.
//
// On the Mojo side (mojo/hello.mojo):
//
//     @export
//     def add(a: Int, b: Int) -> Int:
//         return a + b
//
// `@export` makes the function callable via the C ABI.
// Mojo's `Int` is pointer-width, same as Rust's `isize`.
//
// On the Rust side, we declare it as `unsafe extern "C"`.
// That's the entire bridge — no frameworks, no macros.

unsafe extern "C" {
    fn add(a: isize, b: isize) -> isize;
}

fn main() {
    let tests: &[(isize, isize, isize)] = &[
        (40, 2, 42),
        (-1, 1, 0),
        (1_000_000, 999_999, 1_999_999),
    ];

    for &(a, b, expected) in tests {
        let got = unsafe { add(a, b) };
        assert_eq!(got, expected, "add({a}, {b})");
        println!("  add({a}, {b}) = {got} [ok]");
    }
    println!("all ok");
}
