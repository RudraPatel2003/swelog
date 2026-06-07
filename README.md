<!-- markdownlint-disable MD033-->

# swelog-cli

<h1 align="center">
  <br>
    <img src="./assets/repository-banner.png" alt="Repository Banner" width="20%">
  <br>
    swelog-cli
</h1>

## Description

`swelog-cli` is a Rust CLI for tracking daily accomplishments in Obsidian.

The tool creates a small set of Markdown files in your Obsidian vault. Collect raw work notes throughout the day, and use LLMs to summarize the notes into a daily work log.

Then, aggregate the daily logs into a weekly log for more long-term tracking.

## Installation

Install with npm:

```sh
npm install -g swelog-cli
```

## Usage

### Getting Started

- Run `swelog init` to create a default config file

- Update the config file with your Obsidian vault path and model provider

- Run `swelog setup` to create the default swelog files in your Obsidian vault

- Update the context file with information about your role, team, and current priorities

### Daily Workflow

- Add notes throughout the day with `swelog log "Meeting with manager"` or manually in Obsidian

- Run `swelog fetch github` to fetch your GitHub PR activity and add it to your work file

- Run `swelog summarize` or `swelog summarize day` to generate a daily log

- Run `swelog summarize week` to generate a weekly log

### Other

- Run `swelog reset` to reset your work file to its default content

## Configuration

`swelog init` writes a config file to your OS config directory at
`swelog/swelog.json` (for example `~/.config/swelog/swelog.json` on Linux). The
default config looks like this:

```json
{
  "obsidianVaultPath": "",
  "swelogFolderName": "swelog",
  "workFileName": "WORK.md",
  "contextFileName": "CONTEXT.md",
  "dailyLogFolderName": "Daily",
  "weeklyLogFolderName": "Weekly",
  "llm": "ollama",
  "ollamaModel": "llama3.2",
  "openAiModel": "gpt-5.4-mini"
}
```

Every field is described below. All of the swelog files and folders live inside
`<obsidianVaultPath>/<swelogFolderName>`.

| Field | Description |
| --- | --- |
| `obsidianVaultPath` | Absolute path to your Obsidian vault. Empty by default; you must set this before running other commands, otherwise swelog errors out. |
| `swelogFolderName` | Name of the folder created inside your vault that holds all swelog files. |
| `workFileName` | Name of the file where you collect raw daily work notes. |
| `contextFileName` | Name of the file holding context about your role, team, and priorities, which is fed to the LLM during summarization. |
| `dailyLogFolderName` | Name of the folder (inside the swelog folder) where generated daily logs are stored. |
| `weeklyLogFolderName` | Name of the folder (inside the swelog folder) where generated weekly logs are stored. |
| `llm` | LLM provider used for summarization. One of `ollama` or `openAi`. |
| `ollamaModel` | Model name used when `llm` is `ollama`. |
| `openAiModel` | Model name used when `llm` is `openAi`. |

### AI Summarization

Configure your desired LLM provider and model in your config file.

To use Ollama, ensure it is up and running at `localhost:11434` before running `swelog summarize`.

To use OpenAI, provide your OpenAI API key through the environment before running
`swelog summarize`:

```sh
export OPENAI_API_KEY="your_api_key_here"
```

Do not store your OpenAI API key in the swelog config file.

### GitHub Integration

To fetch your GitHub PR activity and add it to your work file, you need to provide your GitHub token.

```sh
export GITHUB_TOKEN="your_github_token_here"
```

Do not store your GitHub token in the swelog config file.

## Contributing

### Prerequisites

Please have the following installed on your machine:

- Rust
- Just
- Ollama, when using the default Ollama provider
- Obsidian

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

### Pull Request Process

Branch protections are enabled on this repository.

To contribute, please create a new branch and make a pull request.

The rules for branch names are lax, just be sure to include your name.

An example branch name for a change that adds weekly log generation would be:

```text
rudra-weekly-log-generation
```

Your pull request title must follow the conventional commits specification. An
example of a valid pull request title is:

```text
feat: Add weekly log generation
```

All PRs must pass `just pr`, which checks formatting, linting, and tests.
