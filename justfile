# Lists all available commands
_default:
    @just --list

# Runs performance benchmarks
@benches:
    cargo +nightly bench --verbose

# Create an optimized 'release' build
@build:
    cargo build --release --verbose

# Simple sanity check to ensure the project compiles
@check:
    cargo check --locked --workspace

# Update project dependencies then check for unused and outdated dependencies
@check-deps:
    cargo update
    command -v cargo-outdated >/dev/null || (echo "cargo-outdated not installed" && exit 1)
    cargo outdated
    command -v cargo-udeps >/dev/null || (echo "cargo-udeps not installed" && exit 1)
    cargo udeps

# Check all possible combinations of feature flags with cargo-hack.
@check-features:
    command -v cargo-hack >/dev/null || (echo "cargo-hack not installed" && exit 1)
    cargo hack --feature-powerset --exclude-no-default-features clippy --locked -- -D warnings

# Runs all checks sequentially
@check-all:
    @just check
    @just check-deps
    @just check-features

# Run linter with explicit `nightly` flag
@lint:
    cargo +nightly clippy --workspace

# Create an HTML chart showing compilation timings
@timings:
    cargo clean
    cargo build -Z timings

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo fmt --all -- --check
    cargo test --locked
    cargo clippy -- --D warnings
    cargo doc --no-deps --document-private-items --all-features --workspace --verbose

# Runs all tests with minimal console output
@test:
    cargo test --workspace -- --quiet

# Runs all tests sequentially with console output
# This command can be useful for diagnosing problems
@test-verbose:
    cargo test -- --test-threads=1 --nocapture

# Runs all tests in release mode
@test-release:
    cargo test --workspace --release --verbose

# Build the crate documentation, failing on any errors
@docs:
    cargo doc --no-deps --document-private-items --all-features --workspace --verbose

# Show the versions of required build tools
@versions:
    echo "[rustc]:\n$(rustc --version)\n"
    echo "[cargo]:\n$(cargo --version)\n"
    echo "[rustup]:\n$(rustup --version)\n"
