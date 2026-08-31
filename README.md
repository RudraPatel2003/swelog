<!-- markdownlint-disable MD033 MD041 -->

<h1 align="center">
  <br>
    <img src="./assets/repository-banner.png" alt="Repository Banner" width="20%">
  <br>
    swelog-cli
</h1>

`swelog-cli` is a Rust CLI for tracking daily accomplishments in Obsidian.

Write notes as you work, then file the day into a dated daily log with one
command. Optional integrations pull in real activity, and an
optional LLM pass can summarize the notes for you.

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

Full documentation lives at
**[the swelog docs site](https://swelog.rudrapatel.net/)**, including:

- [Quick Start](https://swelog.rudrapatel.net/getting-started/quick-start/) —
  configure your vault and file your first day
- [Configuration](https://swelog.rudrapatel.net/getting-started/configuration/)
  — every field in `swelog.json`
- [Summarization](https://swelog.rudrapatel.net/summarization/) — the optional
  LLM pass, weekly logs, and the Ollama, OpenAI, OpenRouter, and Anthropic
  providers
- [Integrations](https://swelog.rudrapatel.net/integrations/) — pull activity
  from GitHub, Linear, and Google Calendar
- [Authentication](https://swelog.rudrapatel.net/reference/authentication/) —
  how credentials are stored and cleared
- [Command reference](https://swelog.rudrapatel.net/reference/commands/) — the
  full command tree
- [Privacy Policy](https://swelog.rudrapatel.net/legal/privacy/) and
  [Terms of Service](https://swelog.rudrapatel.net/legal/terms/) — what stays on
  your machine and what does not

## Contributing

### Prerequisites

- Rust
- Just (`cargo install just`)
- Cargo Nextest (`cargo install cargo-nextest`)
- Obsidian
- Ollama when testing the optional summarization feature
- Provider credentials when testing hosted models or integrations
- A Google Cloud OAuth client when testing the Google Calendar integration

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

### Google Calendar OAuth Client

`swelog fetch google-calendar` authorizes against a Google OAuth client that is compiled into the binary.

The release binary gets it from the `SWELOG_GOOGLE_CLIENT_ID` and `SWELOG_GOOGLE_CLIENT_SECRET` repository secrets. Both are required to build with `--release`, so `just build-release` fails when either is missing or empty. Debug builds do not need them, and report the missing client at runtime instead.

To test the integration locally, create your own client:

1. In the [Google Cloud Console](https://console.cloud.google.com/), create a
   project and enable the **Google Calendar API**.
2. On the **OAuth consent screen**, add the
   `https://www.googleapis.com/auth/calendar.events.readonly` scope, and add
   your own Google account as a test user.
3. Under **Credentials**, create an **OAuth client ID** of type **Desktop app**.
   Google treats the secret it issues as non-confidential, which is why it can
   ship in the binary.

Then export both variables in the shell you build from:

```sh
export SWELOG_GOOGLE_CLIENT_ID="<client-id>.apps.googleusercontent.com"
export SWELOG_GOOGLE_CLIENT_SECRET="<client-secret>"

just run fetch google-calendar
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
