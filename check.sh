#!/bin/bash

RED='\033[0;31m'
RESET='\033[0m'

if ! cargo +nightly fmt --all -- --check; then
    echo -e "${RED}Error: cargo fmt failed${RESET}"
    exit 1
fi

if ! cargo clippy --workspace --all-targets --all-features -- -D warnings; then
    echo -e "${RED}Error: cargo clippy failed${RESET}"
    exit 1
fi

if ! cargo test --workspace --all-features; then
    echo -e "${RED}Error: cargo test failed${RESET}"
    exit 1
fi
