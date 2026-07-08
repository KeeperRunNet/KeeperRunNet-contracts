# KeeperRunNet Contracts — Makefile
# Run `make help` to see all available commands.

WASM_TARGET  := wasm32-unknown-unknown
RELEASE_DIR  := target/$(WASM_TARGET)/release
WASM_FILE    := $(RELEASE_DIR)/keeperrunnet_registry.wasm
OPT_WASM     := $(RELEASE_DIR)/keeperrunnet_registry.optimized.wasm
NETWORK      ?= testnet

.PHONY: help setup build optimize test fmt lint audit deploy clean

# ---------------------------------------------------------------------------
# Help
# ---------------------------------------------------------------------------
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

# ---------------------------------------------------------------------------
# Setup
# ---------------------------------------------------------------------------
setup: ## Install the Wasm compilation target and required tools
	rustup target add $(WASM_TARGET)
	cargo install soroban-cli cargo-audit

# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------
build: ## Compile the contract to a Wasm binary (release profile)
	cargo build --target $(WASM_TARGET) --release

# ---------------------------------------------------------------------------
# Optimize
# ---------------------------------------------------------------------------
optimize: build ## Run Soroban optimizer on the Wasm binary to minimize size
	soroban contract optimize --wasm $(WASM_FILE)
	@echo "Optimized binary: $(OPT_WASM)"

# ---------------------------------------------------------------------------
# Test
# ---------------------------------------------------------------------------
test: ## Run the full test suite against the local Soroban environment
	cargo test

test-verbose: ## Run tests with output logging enabled
	cargo test -- --nocapture

# ---------------------------------------------------------------------------
# Format & Lint
# ---------------------------------------------------------------------------
fmt: ## Format all Rust source files with rustfmt
	cargo fmt

lint: ## Run Clippy and fail on any warnings
	cargo clippy -- -D warnings

# ---------------------------------------------------------------------------
# Security audit
# ---------------------------------------------------------------------------
audit: ## Audit all Rust dependencies for known vulnerabilities
	cargo audit

# ---------------------------------------------------------------------------
# Deploy
# ---------------------------------------------------------------------------
deploy: optimize ## Deploy the optimized Wasm to the configured network
	@echo "Deploying to $(NETWORK)..."
	soroban contract deploy \
		--wasm $(OPT_WASM) \
		--source $(STELLAR_SECRET_KEY) \
		--network $(NETWORK)

# ---------------------------------------------------------------------------
# Clean
# ---------------------------------------------------------------------------
clean: ## Remove build artifacts
	cargo clean
	@echo "Build artifacts removed."
