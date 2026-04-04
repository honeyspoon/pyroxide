# Simple whitespace tokenizer using MojoStr (ptr + len).
# Rust passes a string, Mojo writes token offsets to an out-buffer.
#
# Token format: each token is (start_offset: Int64, length: Int64) pair.

@export
def count_tokens(str_ptr: Int, str_len: Int) -> Int:
    """Count whitespace-separated tokens in a string."""
    var s = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=str_ptr)
    var in_word = False
    var count: Int = 0
    for i in range(str_len):
        var is_space = s[i] == 32 or s[i] == 10 or s[i] == 13 or s[i] == 9
        if not is_space and not in_word:
            count += 1
        in_word = not is_space
    return count

@export
def tokenize_whitespace(
    str_ptr: Int, str_len: Int,
    out_starts: Int, out_lens: Int,
    max_tokens: Int,
) -> Int:
    """Split string on whitespace, write (start, len) pairs to out buffers.
    Returns actual number of tokens written."""
    var s = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=str_ptr)
    var starts = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=out_starts)
    var lens = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=out_lens)

    var token_idx: Int = 0
    var word_start: Int = -1

    for i in range(str_len):
        var is_space = s[i] == 32 or s[i] == 10 or s[i] == 13 or s[i] == 9
        if not is_space and word_start == -1:
            word_start = i
        elif is_space and word_start != -1:
            if token_idx < max_tokens:
                starts[token_idx] = Int64(word_start)
                lens[token_idx] = Int64(i - word_start)
                token_idx += 1
            word_start = -1

    # Last token if string doesn't end with space
    if word_start != -1 and token_idx < max_tokens:
        starts[token_idx] = Int64(word_start)
        lens[token_idx] = Int64(str_len - word_start)
        token_idx += 1

    return token_idx

@export
def to_uppercase(src_ptr: Int, dst_ptr: Int, len: Int):
    """ASCII uppercase: copy src to dst, converting a-z to A-Z."""
    var src = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=src_ptr)
    var dst = UnsafePointer[UInt8, MutExternalOrigin](unsafe_from_address=dst_ptr)
    for i in range(len):
        var c = src[i]
        if c >= 97 and c <= 122:  # a-z
            dst[i] = c - 32
        else:
            dst[i] = c
