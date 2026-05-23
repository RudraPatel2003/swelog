<!-- markdownlint-disable MD033-->

# swelog

<h1 align="center">
  <br>
    <img src="./assets/repository-banner.png" alt="Repository Banner" width="70%">
  <br>
    swelog
</h1>

## Description

`swelog` is a Rust CLI for tracking daily accomplishments in Obsidian.

The tool creates a small set of Markdown files in your Obsidian vault. Collect raw work notes throughout the day, and use LLMs to summarize the notes into a daily work log.

Then, aggregate the daily logs into a weekly log for more long-term tracking.

## Usage

1. Run `swelog init` to create a default config file

2. Update the config file with your Obsidian vault path

3. Run `swelog setup` to create the default swelog files in your Obsidian vault

4. Write notes in your work file throughout the day and run `swelog log` to generate a daily log

> Note: Please have Ollama downloaded and running on your machine before running `swelog log`.

## Contributing

### Prerequisites

Please have the following installed on your machine:

- Rust
- Just
- Ollama
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
