# Leptos + Trunk + Microsoft SAML Hello World
# Run with: just <recipe>

set shell := ["bash", "-c"]
set dotenv-load := false

default:
    @just --list

# Install required tools (run once)
install-tools:
    cargo install trunk leptosfmt
    rustup target add wasm32-unknown-unknown
    @echo "✅ Tools installed. You may also want: cargo install cargo-leptos (for SSR later)"

# Development server with hot reload
serve:
    trunk serve --port 3000 --open

# Build for production (outputs to dist/)
build:
    trunk build --release

# Check / lint
check:
    cargo check --target wasm32-unknown-unknown

# Format code
fmt:
    leptosfmt .
    cargo fmt

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist/

# Update dependencies
update:
    cargo update

# Full clean + install + serve (fresh start)
fresh: clean install-tools serve

# Show current config (useful for debugging)
info:
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Trunk version: $(trunk --version 2>/dev/null || echo 'not installed')"
    @echo "Target: wasm32-unknown-unknown"