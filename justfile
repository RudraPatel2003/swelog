set positional-arguments

run *args:
    cargo run -p cli -- "$@"

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

check-release-version *args:
    cargo run -p xtask -- check-release-version "$@"

npm-pack:
    npm pack ./npm

npm-pack-dry-run:
    npm pack ./npm --dry-run

npm-publish:
    npm publish ./npm --access public

npm-release-check: pr npm-pack-dry-run
