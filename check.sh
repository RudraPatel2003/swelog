#!/bin/bash

if ! cargo +nightly fmt --all -- --check; then
    echo "Error: cargo fmt failed"
    exit 1
fi

if ! cargo clippy --workspace --all-targets --all-features -- -D warnings; then
    echo "Error: cargo clippy failed"
    exit 1
fi

if ! cargo test --workspace --all-features; then
    echo "Error: cargo test failed"
    exit 1
fi
