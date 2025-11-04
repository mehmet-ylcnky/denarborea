.PHONY: fmt clippy test check build clean install

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-features -- -D warnings

# Run tests
test:
	cargo test --all-features

# Run all checks (fmt, clippy, test)
check: fmt-check clippy test

# Build release
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Install locally
install:
	cargo install --path .

# Development workflow
dev: fmt clippy test

# CI workflow
ci: fmt-check clippy test
