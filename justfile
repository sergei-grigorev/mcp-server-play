# List available commands
default:
    @just --list

# Clean build artifacts and dependencies
clean:
    cargo clean
    rm -rf target/

# Format code using rustfmt
format:
    cargo fmt --all
    cargo fix --allow-dirty --allow-staged

# Build release artifact
build:
    cargo build --release

# Run checks (clippy, fmt)
check:
    cargo fmt -- --check
    cargo clippy -- -D warnings
    cargo check --all-targets

# Run tests
test:
    cargo test --all-features

# Run the server in development mode
serve:
    cargo run
