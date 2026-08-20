<!-- markdownlint-disable MD033 MD041 -->

<h1 align="center">
  <br>
    <img src="./assets/repository-banner.png" alt="Repository Banner" width="20%">
  <br>
    swelog-cli
</h1>

`swelog-cli` is a Rust CLI for tracking daily accomplishments in Obsidian.

Capture raw notes and optional integration activity throughout the day, summarize
them into daily logs with an LLM, and combine daily logs into a weekly summary.

## Installation

Install the CLI with npm:

```sh
npm install -g swelog-cli
```

Confirm the installation:

```sh
swelog --help
```

## Getting Started

Full documentation lives at **[the swelog docs site](https://google.com)**,
including:

- [Quick Start](https://google.com) — configure your vault and record your first
  summarized day
- [Configuration](https://google.com) — every field in `swelog.json`
- [Summarization](https://google.com) — daily and weekly logs, and the Ollama,
  OpenAI, and OpenRouter providers
- [Integrations](https://google.com) — pull activity from GitHub and Linear
- [Authentication](https://google.com) — how credentials are stored and cleared
- [Command reference](https://google.com) — the full command tree

## Contributing

### Prerequisites

- Rust
- Just
- Obsidian
- Ollama when testing the default summarization provider
- Provider credentials when testing hosted models or integrations

### Development

Run the binary locally:

```sh
just run --help
```

Format the workspace:

```sh
just format
```

Build the workspace:

```sh
just build
```

Run all tests:

```sh
just test
```

Run Clippy:

```sh
just clippy
```

Run the full pull request check:

```sh
just pr
```

Update the version of the CLI when preparing for a release:

```sh
just update-release-version <release-tag>
```

### Documentation Site

The docs site is an Astro + Starlight project in [`docs/`](./docs), with its own
pnpm-based tooling. See [`docs/README.md`](./docs/README.md) for the full set of
commands.

```sh
just docs-install
just docs-dev
just docs-pr
```

### Pull Request Process

Create a branch and open a pull request. Branch naming is flexible, but pull
request titles must use Conventional Commit style, for example:

```text
feat: add weekly log generation
```

Rust changes must pass `just pr`, and docs changes must pass `just docs-pr`.

## License

`swelog-cli` is released under the [MIT License](./LICENSE).
