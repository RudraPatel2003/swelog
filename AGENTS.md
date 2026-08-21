# Repository Guidelines

## Project Structure

This is a Rust workspace for `swelog`, a CLI for tracking daily accomplishments in
Obsidian. Workspace crates live under `crates/*`.

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

## Coding Style

- Use Rust 2024 conventions and the repository `rustfmt.toml`.
- Typed errors with `thiserror`, user-facing diagnostics with `miette`.
- Spell names out: `directory`, not `dir`.
- Put a blank line between unrelated lines of code.
- Keep files small and focused: give a self-contained feature its own sub-module, and extract logic so the core flow stays readable.
- A crate's `lib.rs` holds only module declarations, in one alphabetical block. Callers write `use updates::check::print_update_notice;`.
- Import the full path and call unqualified: `use crate::utils::read_npm_package_json;` then `read_npm_package_json()`.
- Never `pub use` to shorten an import path. The path should name the module that defines the item.
- No naked `bool` parameters, because `write_default_config(&path, &config, true)` does not say what `true` means. Use an enum such as `Overwrite::Yes`, and model mutually exclusive flags as one enum, the way `DateSelection` covers `--date` and `--yesterday`. clap fields stay `bool`; convert once in a constructor such as `Overwrite::from_force_flag`.
- Keep comments out of function bodies; when a block needs one, extract a named function. Comment only what the code cannot say: how an external system behaves, why a workaround exists, a constraint with no local evidence.
- `///` doc comments are for the caller. Keep them short: what the item does and anything surprising, not how it works.

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
- `crates/updates` contains the daily check for a newer published version of swelog

## Documentation

- `docs/` contains the Astro + Starlight documentation site. Content lives in
  `docs/src/content/docs/`.
- Documentation pages use the `.mdx` extension, never `.md`. New pages must be
  created as `.mdx`, and any `.md` page that is touched should be renamed to
  `.mdx`.
- User-facing documentation belongs in `docs/`, not in `README.md`. The README
  covers installation, contributing, and links to the docs site.
- `README.md` and `npm/README.md` are kept in sync by hand. Update both.
- Run `just docs-pr` before finishing a task that touches `docs/`.

## Making Changes

- Before finishing a task, run `just format` and then `just pr` and then `just docs-pr`.
- Use Conventional Commit style for commit messages and PR titles, such as `feat: add init command`.
