GODOT        ?= godot
CARGO        ?= cargo
RUST_DIR     := rust
GODOT_DIR    := godot
GODOT_VERSION := 4.6.2.stable.official

.PHONY: help build build-release build-wasm build-wasm-release build-ios build-ios-release check test run run-editor watch clean export-ios export-ios-release ios-open ios-clean serve-wasm

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

# ---------------------------------------------------------------------------
# Rust
# ---------------------------------------------------------------------------

build: ## Compile the Rust extension (debug)
	$(CARGO) build --manifest-path $(RUST_DIR)/Cargo.toml

build-release: ## Compile the Rust extension (release)
	$(CARGO) build --release --manifest-path $(RUST_DIR)/Cargo.toml

build-wasm: ## Compile for Wasm (debug, no-threads)
	cd $(RUST_DIR)/game && \
	cargo +nightly build \
		--features nothreads \
		-Zbuild-std \
		--target wasm32-unknown-emscripten \
		--manifest-path Cargo.toml

build-wasm-release: ## Compile for Wasm (release, no-threads)
	cd $(RUST_DIR)/game && \
	cargo +nightly build \
		--features nothreads \
		-Zbuild-std \
		--target wasm32-unknown-emscripten \
		--release \
		--manifest-path Cargo.toml

EMSDK_ENV    := ../emsdk/emsdk_env.sh
WEB_EXPORT_DIR := godot/export/web
WEB_SERVE_PORT := 8060

serve-wasm: ## Build wasm (release) + export web + open in browser + serve on :8060
	. $(EMSDK_ENV) && cd $(RUST_DIR)/game && \
		cargo +nightly build \
			--features nothreads \
			-Zbuild-std \
			--target wasm32-unknown-emscripten \
			--release \
			--manifest-path Cargo.toml
	mkdir -p $(WEB_EXPORT_DIR)
	$(GODOT) --headless --path $(GODOT_DIR) \
		--export-release "Web" "../$(WEB_EXPORT_DIR)/godot-gdext-demo.html"
	@echo "Serving at http://localhost:$(WEB_SERVE_PORT) — Ctrl-C to stop"
	@open "http://localhost:$(WEB_SERVE_PORT)/godot-gdext-demo.html"
	python3 -m http.server $(WEB_SERVE_PORT) --directory $(WEB_EXPORT_DIR)

build-ios: ## Compile for iOS device — aarch64-apple-ios (debug)
	$(CARGO) build \
		--manifest-path $(RUST_DIR)/Cargo.toml \
		--target aarch64-apple-ios

build-ios-release: ## Compile for iOS device — aarch64-apple-ios (release)
	$(CARGO) build \
		--manifest-path $(RUST_DIR)/Cargo.toml \
		--target aarch64-apple-ios \
		--release


check: ## cargo check + clippy (no full compile)
	$(CARGO) check --manifest-path $(RUST_DIR)/Cargo.toml
	$(CARGO) clippy --manifest-path $(RUST_DIR)/Cargo.toml -- -D warnings

test: ## Run pure-Rust unit tests (no Godot required)
	$(CARGO) test --manifest-path $(RUST_DIR)/Cargo.toml

# ---------------------------------------------------------------------------
# Godot
# ---------------------------------------------------------------------------

run: build ## Build then launch the game scene headlessly (quick smoke test)
	$(GODOT) --headless --path $(GODOT_DIR) res://scenes/game.tscn --quit-after 2

run-editor: build ## Build then open the Godot editor
	$(GODOT) --editor --path $(GODOT_DIR) &

import: ## Warm-up import (needed in CI before any headless run)
	$(GODOT) --headless --path $(GODOT_DIR) --import --quit || true

# ---------------------------------------------------------------------------
# Dev convenience
# ---------------------------------------------------------------------------

watch: ## Auto-rebuild Rust on save (requires cargo-watch: cargo install cargo-watch)
	$(CARGO) watch --manifest-path $(RUST_DIR)/Cargo.toml \
		--clear \
		-x check \
		-x "clippy -- -D warnings" \
		-x build

# ---------------------------------------------------------------------------
# iOS — "Designed for iPad" on macOS and on-device
# ---------------------------------------------------------------------------
# Godot 4.6's template has no arm64 iOS Simulator slice, so simulator is not viable.
# Instead:
#   make export-ios  →  builds Rust (arm64 device) + exports the Xcode project
#   make ios-open    →  opens the project in Xcode
#
# In Xcode, choose one of two destinations:
#   "My Mac (Designed for iPad)"  — runs today, free Apple ID is enough
#   Your physical iOS device      — requires a paid Apple Developer account ($99/yr)

IOS_EXPORT_DIR := godot/export/ios
IOS_APP_NAME   := godot-gdext-demo
IOS_XCODE_PROJ := $(IOS_EXPORT_DIR)/$(IOS_APP_NAME).xcodeproj

export-ios: build-ios ## Build arm64 extension (debug) + export Godot → Xcode project
	rm -rf "$(IOS_XCODE_PROJ)"
	mkdir -p $(IOS_EXPORT_DIR)
	$(GODOT) --headless --path $(GODOT_DIR) \
		--export-debug "iOS" "../$(IOS_XCODE_PROJ)"
	@echo "Xcode project ready: $(IOS_XCODE_PROJ)"
	@echo "Run 'make ios-open' then pick 'My Mac (Designed for iPad)' or your device."

export-ios-release: build-ios-release ## Build arm64 extension (release) + export Godot → Xcode project
	rm -rf "$(IOS_XCODE_PROJ)"
	mkdir -p $(IOS_EXPORT_DIR)
	$(GODOT) --headless --path $(GODOT_DIR) \
		--export-release "iOS" "../$(IOS_XCODE_PROJ)"
	@echo "Xcode project ready: $(IOS_XCODE_PROJ)"
	@echo "Run 'make ios-open' then pick 'My Mac (Designed for iPad)' or your device."

ios-open: ## Open the exported Xcode project
	@test -d "$(IOS_XCODE_PROJ)" || \
		{ echo "Run 'make export-ios' first"; exit 1; }
	open "$(IOS_XCODE_PROJ)"

ios-clean: ## Remove the exported Xcode project
	rm -rf $(IOS_EXPORT_DIR)

# ---------------------------------------------------------------------------
# Housekeeping
# ---------------------------------------------------------------------------

clean: ## Remove Rust build artifacts
	$(CARGO) clean --manifest-path $(RUST_DIR)/Cargo.toml
