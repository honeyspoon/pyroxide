# Embedding operations for the HuggingFace inference pipeline.
# Rust loads the model, Mojo computes on it.

def _unpack(desc_addr: Int) -> Tuple[Int, Int]:
    var p = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=desc_addr)
    var rank = Int(p[1])
    var numel: Int = 1
    for i in range(rank):
        numel *= Int(p[2 + i])
    return (numel, Int(p[18]))

@export
def embedding_lookup_f32(weight_desc: Int, ids_desc: Int, out_desc: Int):
    var w_p = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=weight_desc)
    var hidden = Int(w_p[3])
    var w_data = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=Int(w_p[18]))
    var id_meta = _unpack(ids_desc)
    var seq_len = id_meta[0]
    var ids = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=id_meta[1])
    var out_meta = _unpack(out_desc)
    var out = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=out_meta[1])
    for s in range(seq_len):
        var token_id = Int(ids[s])
        var src = token_id * hidden
        var dst = s * hidden
        for h in range(hidden):
            out[dst + h] = w_data[src + h]

@export
def mean_pool_f32(input_desc: Int, out_desc: Int):
    var p = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=input_desc)
    var seq_len = Int(p[2])
    var hidden = Int(p[3])
    var inp = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=Int(p[18]))
    var out_meta = _unpack(out_desc)
    var out = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=out_meta[1])
    for h in range(hidden):
        var total: Float32 = 0.0
        for s in range(seq_len):
            total += inp[s * hidden + h]
        out[h] = total / Float32(seq_len)

@export
def cosine_similarity_f32(a_desc: Int, b_desc: Int) -> Float32:
    var a_meta = _unpack(a_desc)
    var n = a_meta[0]
    var a = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=a_meta[1])
    var b_meta = _unpack(b_desc)
    var b = UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=b_meta[1])
    var dot: Float32 = 0.0
    var na: Float32 = 0.0
    var nb: Float32 = 0.0
    for i in range(n):
        dot += a[i] * b[i]
        na += a[i] * a[i]
        nb += b[i] * b[i]
    var denom = (na ** 0.5) * (nb ** 0.5)
    if denom == 0.0:
        return Float32(0.0)
    return dot / denom
