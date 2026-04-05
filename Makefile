.PHONY: build test clean headers

build:
	cargo build --workspace

test:
	@echo "Running all pyroxide examples..."
	@for ex in 01_hello 02_structs 03_tensors 04_simd 05_dtype_generic 06_comptime 07_embeddings 08_abi_edge_cases 09_image_blur 10_tokenizer 11_neural_layer 12_accumulator 13_sorting 14_mandelbrot 15_nested_structs 16_struct_arrays 17_mixed_args 18_padding 19_bytes 20_call_overhead 21_edge_cases 22_large_data 23_chained 24_matrix 25_catch_panic 26_pipeline 27_scalar_types 28_aliasing 29_concurrent 30_null_ptr 31_conditional_outparam; do \
		echo ""; \
		echo "===== $$ex ====="; \
		DYLD_LIBRARY_PATH=examples/target/mojo-libs cargo run -p pyroxide-examples --example $$ex || exit 1; \
	done
	@echo ""
	@echo "ALL EXAMPLES PASSED"

headers:
	./scripts/fetch-headers.sh

clean:
	cargo clean
