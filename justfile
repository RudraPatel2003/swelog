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

update-release-version *args:
    cargo run -p xtask -- update-release-version "$@"

docs-install:
    cd docs && pnpm install

docs-dev:
    cd docs && pnpm dev

docs-build:
    cd docs && pnpm build

docs-preview:
    cd docs && pnpm preview

docs-lint:
    cd docs && pnpm lint

docs-lint-fix:
    cd docs && pnpm lint:fix

docs-format:
    cd docs && pnpm format

docs-check-format:
    cd docs && pnpm format:check

docs-knip:
    cd docs && pnpm knip

docs-pr: docs-lint docs-check-format docs-build

npm-pack-dry-run:
    npm pack ./npm --dry-run

npm-publish:
    npm publish ./npm --access public

npm-release-check: pr npm-pack-dry-run