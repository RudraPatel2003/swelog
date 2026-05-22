run *args:
    cargo run -p cli -- {{args}}

format:
    cargo +nightly fmt --all

check-format:
    cargo +nightly fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-features

build:
    cargo build --workspace

pr: check-format clippy test
