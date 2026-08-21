run *args:
    cargo run -p cli -- "$@"

format:
    cargo fmt --all

check-format:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# cargo nextest does not run doctests, so they need their own cargo test invocation.
test:
    cargo nextest run --workspace --all-features
    cargo test --workspace --all-features --doc

build:
    cargo build --workspace

build-release target:
    cargo build --package cli --bin swelog --locked --release --target "{{ target }}"

# Compiles every dependency without running anything, so CI can populate the shared Rust cache.
build-dependencies:
    cargo clippy --workspace --all-targets --all-features
    cargo nextest run --workspace --all-features --no-run

pr: check-format clippy test
