.ONESHELL:
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

test_upload: build
	set -e

	cp target/debug/SteamUploader test/example_mod
	cp steamworks-rs/steamworks-sys/lib/steam/redistributable_bin/linux64/libsteam_api.so test/example_mod

	cd test/example_mod
	cp manifest.example.json manifest.json
	./SteamUploader upload
	rm manifest.json

	cp manifest.example.toml manifest.toml
	./SteamUploader upload
	rm manifest.toml

	cp manifest.example.yaml manifest.yaml
	./SteamUploader upload
	rm manifest.yaml

	rm libsteam_api.so
	rm SteamUploader

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings