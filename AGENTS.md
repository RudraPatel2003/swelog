# Repository Guidelines

## Project Structure

This is a Rust workspace for `swelog`, a CLI for tracking daily accomplishments in
Obsidian.

Workspace crates live under `crates/*`.

`docs/` is the one non-crate source tree: an Astro + Starlight documentation site
with its own pnpm-based tooling. It is invisible to Cargo and is deployed
separately to Vercel.

## Commands

- `just format` formats all Rust code with the pinned nightly rustfmt.
- `just build` builds the workspace.
- `just test` runs all workspace tests with all features.
- `just clippy` runs clippy with warnings denied.
- `just pr` runs format checks, clippy, and tests.
- `just docs-dev` starts the docs site dev server.
- `just docs-lint-fix` fixes lint and formatting issues in the docs site.
- `just docs-pr` runs linting, format checks, the build, and knip for the docs site.

## Making Changes

- Before finishing a task, read the [CODE_STYLE_GUIDE.md](CODE_STYLE_GUIDE.md) and then run `just format` and then `just pr` and then `just docs-pr`.
