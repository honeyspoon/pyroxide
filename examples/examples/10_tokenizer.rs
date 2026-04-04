// ─────────────────────────────────────────────────────────
// 10: String processing — tokenizer using MojoStr
// ─────────────────────────────────────────────────────────
//
// First real use of MojoStr. Rust passes text to Mojo, which
// splits on whitespace and writes token offsets to out-buffers.
// Also tests uppercase conversion (Mojo reads + writes bytes).

use pyroxide::bridge::MojoSlice;
use pyroxide::string::MojoStr;

unsafe extern "C" {
    fn count_tokens(str_ptr: isize, str_len: isize) -> isize;
    fn tokenize_whitespace(
        str_ptr: isize,
        str_len: isize,
        out_starts: isize,
        out_lens: isize,
        max_tokens: isize,
    ) -> isize;
    fn to_uppercase(src: isize, dst: isize, len: isize);
}

fn mojo_count_tokens(text: &str) -> usize {
    let s = MojoStr::new(text);
    unsafe { count_tokens(s.as_raw(), s.len_isize()) as usize }
}

fn mojo_tokenize(text: &str) -> Vec<String> {
    let s = MojoStr::new(text);
    let max_tokens = 64;
    let starts = vec![0i64; max_tokens];
    let lens = vec![0i64; max_tokens];

    let n = unsafe {
        tokenize_whitespace(
            s.as_raw(),
            s.len_isize(),
            MojoSlice::new(&starts).as_raw(),
            MojoSlice::new(&lens).as_raw(),
            max_tokens as isize,
        ) as usize
    };

    (0..n)
        .map(|i| {
            let start = starts[i] as usize;
            let len = lens[i] as usize;
            text[start..start + len].to_owned()
        })
        .collect()
}

fn mojo_uppercase(text: &str) -> String {
    let s = MojoStr::new(text);
    let mut dst = vec![0u8; text.len()];
    unsafe {
        to_uppercase(s.as_raw(), dst.as_mut_ptr() as isize, s.len_isize());
    }
    String::from_utf8(dst).expect("uppercase produced invalid UTF-8")
}

fn main() {
    // ── Count tokens ──
    assert_eq!(mojo_count_tokens("hello world"), 2);
    assert_eq!(mojo_count_tokens("  one  two  three  "), 3);
    assert_eq!(mojo_count_tokens(""), 0);
    assert_eq!(mojo_count_tokens("single"), 1);
    assert_eq!(mojo_count_tokens("   "), 0);
    println!("  count_tokens [ok]");

    // ── Tokenize ──
    let tokens = mojo_tokenize("the cat sat on the mat");
    assert_eq!(tokens, ["the", "cat", "sat", "on", "the", "mat"]);
    println!("  tokenize({}) = {tokens:?} [ok]", tokens.len());

    let tokens2 = mojo_tokenize("  leading   and   trailing  ");
    assert_eq!(tokens2, ["leading", "and", "trailing"]);
    println!("  tokenize(whitespace edges) = {tokens2:?} [ok]");

    // ── Uppercase ──
    assert_eq!(mojo_uppercase("hello"), "HELLO");
    assert_eq!(mojo_uppercase("Hello World 123!"), "HELLO WORLD 123!");
    assert_eq!(mojo_uppercase(""), "");
    println!("  to_uppercase [ok]");

    // ── Round-trip: tokenize then verify against Rust ──
    let text = "Mojo computes tokens via FFI";
    let mojo_tokens = mojo_tokenize(text);
    let rust_tokens: Vec<&str> = text.split_whitespace().collect();
    assert_eq!(mojo_tokens.len(), rust_tokens.len());
    for (m, r) in mojo_tokens.iter().zip(&rust_tokens) {
        assert_eq!(m, r);
    }
    println!("  tokenize matches Rust split_whitespace [ok]");

    println!("all ok");
}
