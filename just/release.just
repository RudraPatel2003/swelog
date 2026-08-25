list-release-version:
    cargo run -p xtask -- list-version

check-release-version *args:
    cargo run -p xtask -- check-release-version "$@"

update-release-version *args:
    cargo run -p xtask -- update-release-version "$@"

npm-pack-dry-run:
    npm pack ./npm --dry-run

npm-publish:
    npm publish ./npm --access public

npm-release-check: pr npm-pack-dry-run
