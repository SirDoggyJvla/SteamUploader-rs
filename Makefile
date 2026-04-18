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
	cp mod-manifest.example.json mod-manifest.json
	./SteamUploader upload
	rm mod-manifest.json

	cp mod-manifest.example.toml mod-manifest.toml
	./SteamUploader upload
	rm mod-manifest.toml

	cp mod-manifest.example.yaml mod-manifest.yaml
	./SteamUploader upload
	rm mod-manifest.yaml

	rm libsteam_api.so
	rm SteamUploader

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings