build:
	@cargo build

test:
	@RUST_TEST_TASKS=1 cargo test

docs: build
	@cargo doc --no-deps

update-rust:
	@curl -s https://static.rust-lang.org/rustup.sh | sudo sh

.PHONY: build test docs
