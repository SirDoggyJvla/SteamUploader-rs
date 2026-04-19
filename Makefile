.ONESHELL:
.PHONY: help build release clean run test fmt lint

SHELL := /bin/bash

# detect OS and set Steam library paths
UNAME_S := $(shell uname -s)
# check for Windows (native Windows_NT, or MINGW/MSYS environments)
ifeq ($(OS),Windows_NT)
    IS_WINDOWS := 1
else ifeq ($(findstring MINGW,$(UNAME_S)),MINGW)
    IS_WINDOWS := 1
else ifeq ($(findstring MSYS,$(UNAME_S)),MSYS)
    IS_WINDOWS := 1
else
    IS_WINDOWS := 0
endif

ifeq ($(IS_WINDOWS),1)
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
	@echo "  tests    - Run tests"
	@echo "  test_upload - Test the upload functionality"
	@echo "  test_manifests - Test manifest parsing with different formats (json, toml, yaml, yml)"

release: tests
	@echo "Releasing version $(VERSION)"

	git tag -d $(VERSION)
	git push --delete origin $(VERSION)
	gh release delete $(VERSION) --yes

	git tag $(VERSION)
	git push origin $(VERSION)

tests: test_manifests test_upload

test_upload:
	cargo build

	cp $(APP) test/example_mod
	cp $(STEAM_LIBS) test/example_mod
	cd test/example_mod

	cp ../example_manifests/mod-manifest.json mod-manifest.json
	./$(APP_RELATIVE) upload
	rm mod-manifest.json

	rm $(STEAM_LIBS_RELATIVE)
	rm $(APP_RELATIVE)

test_manifests:
	cargo build

	cp $(APP) test/example_mod
	cp $(STEAM_LIBS) test/example_mod
	cd test/example_mod

	cp ../example_manifests/mod-manifest.json mod-manifest.json
	./$(APP_RELATIVE) upload --dry-run
	rm mod-manifest.json

	cp ../example_manifests/mod-manifest.toml mod-manifest.toml
	./$(APP_RELATIVE) upload --dry-run
	rm mod-manifest.toml

	cp ../example_manifests/mod-manifest.yaml mod-manifest.yaml
	./$(APP_RELATIVE) upload --dry-run
	rm mod-manifest.yaml

	cp ../example_manifests/mod-manifest.yaml mod-manifest.yml
	./$(APP_RELATIVE) upload --dry-run
	rm mod-manifest.yml

	rm $(STEAM_LIBS_RELATIVE)
	rm $(APP_RELATIVE)
