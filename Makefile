.PHONY: build test clean headers

build:
	cargo build --workspace

test:
	@echo "Running all embers examples..."
	@for ex in 01_hello 02_structs 03_tensors 04_simd 05_dtype_generic 06_comptime 07_embeddings; do \
		echo ""; \
		echo "===== $$ex ====="; \
		DYLD_LIBRARY_PATH=examples/target/mojo-libs cargo run -p embers-examples --example $$ex || exit 1; \
	done
	@echo ""
	@echo "ALL EXAMPLES PASSED"

headers:
	./scripts/fetch-headers.sh

clean:
	cargo clean
