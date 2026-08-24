docs-install *args:
    cd docs && pnpm install "$@"

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

docs-pr: docs-lint docs-check-format docs-build docs-knip
