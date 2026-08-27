# swelog-cli

`swelog-cli` is a Rust CLI for tracking daily accomplishments in Obsidian.

Write notes as you work, then file the day into a dated daily log with one
command. No API key required. Optional integrations pull in real activity, and an
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
  LLM pass, weekly logs, and the Ollama, OpenAI, and OpenRouter providers
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

Source, issues, and contributing guidelines live on
[GitHub](https://github.com/RudraPatel2003/swelog-cli).

## License

`swelog-cli` is released under the MIT License.
