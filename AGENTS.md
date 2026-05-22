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

## Testing

- Place crate-local unit tests in a crate test module such as `crates/config/src/tests.rs`.
- Place crate integration tests in the `tests/` folder of the crate, at the same level as the `src/` folder.
- Name tests by behavior, for example `write_config_fails_when_file_exists_without_force`.

## Crates

- `crates/cli` contains the `swelog` binary, command parsing, and top-level CLI flow.
- `crates/config` contains configuration logic

## Making Changes

- Before finishing a task, run `just format` and then `just pr`.
- Use Conventional Commit style for commit messages and PR titles, such as `feat: add init command`.
