.PHONY: help build release clean run test fmt lint

help:
	@echo "Usage: make [target]"
	@echo "Available targets:"
	@echo "  build   - Build the project in debug mode"
	@echo "  release - Build the project in release mode"
	@echo "  clean   - Clean the project"
	@echo "  run     - Run the project"
	@echo "  test    - Run tests"
	@echo "  fmt     - Format the code using rustfmt"
	@echo "  lint    - Lint the code using clippy"

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings