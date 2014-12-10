build:
	@cargo build

test:
	@RUST_TEST_TASKS=1 cargo test

docs:
	@cargo doc --no-deps --open

update-rust:
	@curl -s https://static.rust-lang.org/rustup.sh | sudo sh

upload-docs: docs
	@./upload-docs.sh

.PHONY: build test docs
