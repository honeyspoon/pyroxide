# Neural network layer: output = ReLU(input @ weights + bias)
# All tensors passed via TensorDescriptor.
#
# Descriptor layout: dtype(0) rank(8) dims(16) strides(80) data_ptr(144)

def _desc_data_f32(desc_addr: Int) -> UnsafePointer[Float32, MutExternalOrigin]:
    var p = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=desc_addr)
    return UnsafePointer[Float32, MutExternalOrigin](unsafe_from_address=Int(p[18]))

def _desc_dims(desc_addr: Int) -> Tuple[Int, Int]:
    var p = UnsafePointer[Int64, MutExternalOrigin](unsafe_from_address=desc_addr)
    return (Int(p[2]), Int(p[3]))

@export
def linear_relu_f32(
    input_desc: Int,   # (batch, in_features)
    weight_desc: Int,  # (in_features, out_features)
    bias_desc: Int,    # (out_features,)
    output_desc: Int,  # (batch, out_features)
):
    """Fused linear + ReLU: output = max(0, input @ weight + bias)."""
    var inp = _desc_data_f32(input_desc)
    var wt = _desc_data_f32(weight_desc)
    var bias = _desc_data_f32(bias_desc)
    var out = _desc_data_f32(output_desc)

    var dims = _desc_dims(input_desc)
    var batch = dims[0]
    var in_f = dims[1]
    var wdims = _desc_dims(weight_desc)
    var out_f = wdims[1]

    for b in range(batch):
        for j in range(out_f):
            var acc: Float32 = bias[j]
            for k in range(in_f):
                acc += inp[b * in_f + k] * wt[k * out_f + j]
            # ReLU
            if acc < 0.0:
                acc = 0.0
            out[b * out_f + j] = acc

@export
def softmax_f32(input_desc: Int, output_desc: Int):
    """Row-wise softmax on a 2D tensor."""
    var inp = _desc_data_f32(input_desc)
    var out = _desc_data_f32(output_desc)
    var dims = _desc_dims(input_desc)
    var rows = dims[0]
    var cols = dims[1]

    for r in range(rows):
        var offset = r * cols
        # Find max for numerical stability
        var max_val = inp[offset]
        for c in range(1, cols):
            if inp[offset + c] > max_val:
                max_val = inp[offset + c]
        # Exp and sum
        var sum: Float32 = 0.0
        for c in range(cols):
            var e = (inp[offset + c] - max_val)
            # Approximate exp: just use the built-in
            out[offset + c] = 2.718281828 ** e
            sum += out[offset + c]
        # Normalize
        for c in range(cols):
            out[offset + c] = out[offset + c] / sum
