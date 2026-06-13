# Leptos + Jama Explorer - just commands

# One-time setup
install-tools:
    cargo install trunk
    rustup target add wasm32-unknown-unknown

# Development
serve:
    trunk serve --port 3000 --open

# Production build
build:
    trunk build --release

# Check without building WASM
check:
    cargo check --target wasm32-unknown-unknown

# Format code
fmt:
    cargo fmt

# Clean build artifacts
clean:
    trunk clean
    cargo clean

# Fresh start (clean + reinstall tools + serve)
fresh:
    just clean
    just install-tools
    just serve

# Show versions
info:
    @echo "Rust: $(rustc --version)"
    @echo "Cargo: $(cargo --version)"
    @echo "Trunk: $(trunk --version 2>/dev/null || echo 'not installed')"
