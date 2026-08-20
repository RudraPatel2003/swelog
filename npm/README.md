# swelog-cli

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

Full documentation lives at
**[the swelog docs site](https://swelog-cli.vercel.app/)**, including:

- [Quick Start](https://swelog-cli.vercel.app/getting-started/quick-start/) —
  configure your vault and record your first summarized day
- [Configuration](https://swelog-cli.vercel.app/getting-started/configuration/)
  — every field in `swelog.json`
- [Summarization](https://swelog-cli.vercel.app/summarization/) — daily and
  weekly logs, and the Ollama, OpenAI, and OpenRouter providers
- [Integrations](https://swelog-cli.vercel.app/integrations/) — pull activity
  from GitHub and Linear
- [Authentication](https://swelog-cli.vercel.app/reference/authentication/) —
  how credentials are stored and cleared
- [Command reference](https://swelog-cli.vercel.app/reference/commands/) — the
  full command tree

## Contributing

Source, issues, and contributing guidelines live on
[GitHub](https://github.com/RudraPatel2003/swelog-cli).

## License

`swelog-cli` is released under the MIT License.
