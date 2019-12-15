build: RUSTFLAGS="-C link-arg=-fuse-ld=lld"
build:
	@cargo build

test:
	@cargo test

test-single: RUST_TEST_THREADS=1
test-single: test

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-features

dev: RUN_ENV=development
dev: RUSTFLAGS="-C link-arg=-fuse-ld=lld"
dev:
	@cargo run

.PHONY: build test docs style-check lint
