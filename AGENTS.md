# Repository Guidelines

## Project Structure

This is a Rust workspace for `swelog`, a CLI for tracking daily accomplishments in
Obsidian. Workspace crates live under `crates/*`.

## Commands

- `just format` formats all Rust code with nightly rustfmt.
- `just build` builds the workspace.
- `just test` runs all workspace tests with all features.
- `just clippy` runs clippy with warnings denied.
- `just pr` runs format checks, clippy, and tests.

## Coding Style

- Use Rust 2024 conventions and the repository `rustfmt.toml`.
- Prefer explicit, typed errors using `thiserror` and user-facing diagnostics with `miette`.
- Prefer explicit variable names such as `directory` over abbreviations like `dir`.
- Prefer white space between lines of code unless they are related to each other.
- Prefer extracting logic into files so that core logic remains readable.
- When importing items from other modules in this repository, fully qualify the import path and call the item unqualified. For example, write `use crate::utils::read_npm_package_json;` then call `read_npm_package_json()`, rather than `use crate::utils;` then `utils::read_npm_package_json()`.
- When creating new features, if the content is self-contained, place it in a sub-module of the crate. This keeps files small and focused.

## CLI

- The CLI should read arguments and call the corresponding `run` function. If the `run` function has no flags, avoid an unused self warning by using `let _ = self;`.

## Testing

- Place crate-local unit tests in a crate test module such as `crates/config/src/tests.rs`.
- Place crate integration tests in the `tests/` folder of the crate, at the same level as the `src/` folder.
- Name tests by behavior, for example `write_config_fails_when_file_exists_without_force`.

## Crates

- `crates/cli` contains the `swelog` binary, command parsing, and top-level CLI flow.
- `crates/config` contains configuration logic
- `crates/dates` contains the shared `MM-DD-YYYY` date format, parsing, and formatting

## Making Changes

- Before finishing a task, run `just format` and then `just pr`.
- Use Conventional Commit style for commit messages and PR titles, such as `feat: add init command`.
