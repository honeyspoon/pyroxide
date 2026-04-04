// ─────────────────────────────────────────────────────────
// 07: Real embedding inference from HuggingFace
// ─────────────────────────────────────────────────────────
//
// This is the capstone example. Nothing synthetic — we download
// a real model from HuggingFace, load its weights in Rust, and
// compute embeddings through Mojo.
//
// Pipeline:
//   Rust: download model → load safetensors → create TensorDescriptors
//   Mojo: embedding lookup → mean pooling → cosine similarity
//   Rust: collect results → print similarity matrix
//
// The model is sentence-transformers/all-MiniLM-L6-v2:
//   - 30,522 token vocabulary
//   - 384-dimensional embeddings
//   - ~90 MB safetensors file (cached after first download)
//
// We use a byte-level tokenizer (not a real WordPiece tokenizer),
// so the embeddings won't be semantically perfect — but the compute
// pipeline is identical to production: same weights, same ops, same
// data flow through Mojo.

use pyroxide::bridge::IntoMojo;
use pyroxide::types::max::{DType, Tensor, TensorDescriptor, TensorShape};

use hf_hub::api::sync::Api;
use safetensors::SafeTensors;

// ── Mojo functions (mojo/embeddings.mojo) ──

unsafe extern "C" {
    fn embedding_lookup_f32(weight_desc: isize, ids_desc: isize, out_desc: isize);
    fn mean_pool_f32(input_desc: isize, out_desc: isize);
    fn cosine_similarity_f32(a_desc: isize, b_desc: isize) -> f32;
}

// ── Helpers ──

fn desc(dtype: DType, shape: &TensorShape, ptr: *const u8) -> TensorDescriptor {
    TensorDescriptor::contiguous(dtype, shape, ptr)
}

fn embed(emb_desc: &TensorDescriptor, hidden_dim: usize, ids: &[i64]) -> Tensor<f32> {
    let seq_len = ids.len();
    let ids_desc = desc(
        DType::Int64,
        &TensorShape::vector(seq_len as i64),
        ids.as_ptr() as *const u8,
    );
    let embeddings = Tensor::<f32>::zeros(TensorShape::matrix(seq_len as i64, hidden_dim as i64));
    let emb_out_desc = embeddings.descriptor();
    unsafe {
        embedding_lookup_f32(
            emb_desc.as_mojo().addr(),
            ids_desc.as_mojo().addr(),
            emb_out_desc.as_mojo().addr(),
        )
    };

    let pooled = Tensor::<f32>::zeros(TensorShape::vector(hidden_dim as i64));
    let pool_in_desc = embeddings.descriptor();
    let pool_out_desc = pooled.descriptor();
    unsafe {
        mean_pool_f32(
            pool_in_desc.as_mojo().addr(),
            pool_out_desc.as_mojo().addr(),
        )
    };
    pooled
}

fn tokenize(text: &str, vocab_size: usize) -> Vec<i64> {
    text.bytes()
        .map(|b| (b as i64) % (vocab_size as i64))
        .collect()
}

fn cosine(a: &Tensor<f32>, b: &Tensor<f32>) -> f32 {
    unsafe {
        cosine_similarity_f32(
            a.descriptor().as_mojo().addr(),
            b.descriptor().as_mojo().addr(),
        )
    }
}

// ── Main ──

fn main() {
    // Step 1: Download from HuggingFace
    println!("Downloading sentence-transformers/all-MiniLM-L6-v2...");
    let api = Api::new().expect("HuggingFace API");
    let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string());
    let path = repo.get("model.safetensors").expect("download failed");
    println!("  cached: {}", path.display());

    // Step 2: Load the embedding weight matrix
    let file_data = std::fs::read(&path).expect("read failed");
    let tensors = SafeTensors::deserialize(&file_data).expect("parse failed");
    let weight = tensors
        .tensor("embeddings.word_embeddings.weight")
        .expect("no embedding");
    let [vocab_size, hidden_dim] = weight.shape() else {
        panic!("expected 2D")
    };
    let (vocab_size, hidden_dim) = (*vocab_size, *hidden_dim);
    println!("  embedding matrix: {vocab_size} tokens x {hidden_dim} dims");

    let weight_f32: &[f32] = unsafe {
        std::slice::from_raw_parts(
            weight.data().as_ptr() as *const f32,
            weight.data().len() / 4,
        )
    };
    let emb_desc = desc(
        DType::Float32,
        &TensorShape::matrix(vocab_size as i64, hidden_dim as i64),
        weight_f32.as_ptr() as *const u8,
    );

    // Step 3: Verify embedding lookup matches direct memory access
    let test_ids: Vec<i64> = vec![0, 1, 2];
    let test_emb = Tensor::<f32>::zeros(TensorShape::matrix(3, hidden_dim as i64));
    let test_ids_desc = desc(
        DType::Int64,
        &TensorShape::vector(3),
        test_ids.as_ptr() as *const u8,
    );
    unsafe {
        embedding_lookup_f32(
            emb_desc.as_mojo().addr(),
            test_ids_desc.as_mojo().addr(),
            test_emb.descriptor().as_mojo().addr(),
        )
    };

    for t in 0..3 {
        let mojo_row = &test_emb[t * hidden_dim..(t + 1) * hidden_dim];
        let truth_row = &weight_f32[t * hidden_dim..(t + 1) * hidden_dim];
        assert_eq!(mojo_row, truth_row, "lookup mismatch at token {t}");
    }
    println!("  embedding_lookup verified [ok]");

    // Step 4: Verify mean pooling against Rust ground truth
    let pooled = Tensor::<f32>::zeros(TensorShape::vector(hidden_dim as i64));
    unsafe {
        mean_pool_f32(
            test_emb.descriptor().as_mojo().addr(),
            pooled.descriptor().as_mojo().addr(),
        )
    };
    let mut rust_pooled = vec![0.0f32; hidden_dim];
    for h in 0..hidden_dim {
        for s in 0..3 {
            rust_pooled[h] += test_emb[s * hidden_dim + h];
        }
        rust_pooled[h] /= 3.0;
    }
    for (i, (&got, &exp)) in pooled.iter().zip(&rust_pooled).enumerate() {
        assert!(
            (got - exp).abs() < 1e-5,
            "mean_pool dim {i}: {got} != {exp}"
        );
    }
    println!("  mean_pool verified [ok]");

    // Step 5: Self-similarity = 1.0
    let self_sim = cosine(&pooled, &pooled);
    assert!((self_sim - 1.0).abs() < 1e-4);
    println!("  cosine(self) = {self_sim:.4} [ok]");

    // Step 6: Embed real sentences → similarity matrix
    let sentences = [
        "The cat sat on the mat",
        "A kitten rested on the rug",
        "Quantum computing uses qubits",
        "Machine learning models are trained on data",
        "The dog slept on the couch",
    ];

    println!(
        "\n  Embedding {} sentences through Mojo...",
        sentences.len()
    );
    let embeddings: Vec<Tensor<f32>> = sentences
        .iter()
        .map(|s| embed(&emb_desc, hidden_dim, &tokenize(s, vocab_size)))
        .collect();

    println!("\n  Cosine similarity matrix:");
    print!("  {:>6}", "");
    for (i, _) in sentences.iter().enumerate() {
        print!("  [{i}]  ");
    }
    println!();
    for (i, si) in sentences.iter().enumerate() {
        print!("  [{i}] ");
        for j in 0..sentences.len() {
            print!(" {:.3} ", cosine(&embeddings[i], &embeddings[j]));
        }
        println!("  \"{si}\"");
    }

    // Verify diagonal = 1.0 and symmetry
    for (i, emb) in embeddings.iter().enumerate() {
        assert!(
            (cosine(emb, emb) - 1.0).abs() < 1e-3,
            "diagonal [{i}] != 1.0"
        );
    }
    for i in 0..sentences.len() {
        for j in (i + 1)..sentences.len() {
            let ij = cosine(&embeddings[i], &embeddings[j]);
            let ji = cosine(&embeddings[j], &embeddings[i]);
            assert!((ij - ji).abs() < 1e-6, "asymmetric: [{i},{j}]");
        }
    }
    println!("\n  diagonal = 1.0 [ok]");
    println!("  symmetry [ok]");
    println!("all ok");
}
