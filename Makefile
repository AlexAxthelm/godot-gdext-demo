GODOT        ?= godot
CARGO        ?= cargo
RUST_DIR     := rust
GODOT_DIR    := godot
GODOT_VERSION := 4.6.2.stable.official

.PHONY: help build build-release check test run run-editor watch clean

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
# Housekeeping
# ---------------------------------------------------------------------------

clean: ## Remove Rust build artifacts
	$(CARGO) clean --manifest-path $(RUST_DIR)/Cargo.toml
