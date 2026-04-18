.ONESHELL:
.PHONY: help build release clean run test fmt lint

SHELL := /bin/bash

# detect OS and set Steam library paths
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Windows_NT)
    STEAM_LIBS := steamworks-rs/steamworks-sys/lib/steam/redistributable_bin/win64/steam_api64.dll steamworks-rs/steamworks-sys/lib/steam/redistributable_bin/win64/steam_api64.lib
	STEAM_LIBS_RELATIVE := steam_api64.dll steam_api64.lib

	APP := target/debug/SteamUploader.exe
	APP_RELATIVE := SteamUploader.exe
else
    STEAM_LIBS := steamworks-rs/steamworks-sys/lib/steam/redistributable_bin/linux64/libsteam_api.so
	STEAM_LIBS_RELATIVE := libsteam_api.so

	APP := target/debug/SteamUploader
	APP_RELATIVE := SteamUploader
endif

help:
	@echo "Usage: make [target]"
	@echo "Available targets:"
	@echo "  build   - Build the project in debug mode"
	@echo "  release - Build the project in release mode"
	@echo "  clean   - Clean the project"
	@echo "  run     - Run the project"
	@echo "  test    - Run tests"

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

run:
	cargo run

test_upload: build
# 	set -e

	cp $(APP) test/example_mod
	cp $(STEAM_LIBS) test/example_mod
	cd test/example_mod

	cp mod-manifest.example.json mod-manifest.json
	./$(APP_RELATIVE) upload
	rm mod-manifest.json

	rm $(STEAM_LIBS_RELATIVE)
	rm $(APP_RELATIVE)

test_manifests: build
# 	set -e

	cp $(APP) test/example_mod
	cp $(STEAM_LIBS) test/example_mod
	cd test/example_mod
	
	cp mod-manifest.example.json mod-manifest.json
	./$(APP_RELATIVE) upload
	rm mod-manifest.json

	cp mod-manifest.example.toml mod-manifest.toml
	./$(APP_RELATIVE) upload
	rm mod-manifest.toml

	cp mod-manifest.example.yaml mod-manifest.yaml
	./$(APP_RELATIVE) upload
	rm mod-manifest.yaml

	rm $(STEAM_LIBS_RELATIVE)
	rm $(APP_RELATIVE)

tests: test_upload
