<!-- markdownlint-disable MD033 -->

# swelog-cli

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

## Quick Start

### 1. Create the configuration file

```sh
swelog init
```

This creates `swelog/swelog.json` inside your operating system's configuration
directory. Run `swelog config` at any time to display the exact path.

### 2. Configure your Obsidian vault

Open `swelog.json` and set `obsidianVaultPath` to the absolute path of your
vault. You can also select a summarization provider and model at this point.

```json
{
  "obsidianVaultPath": "/absolute/path/to/your/vault",
  "swelogFolderName": "swelog",
  "workFileName": "WORK.md",
  "contextFileName": "CONTEXT.md",
  "dailyLogFolderName": "Daily",
  "weeklyLogFolderName": "Weekly",
  "llm": "ollama",
  "llmModel": "llama3.2",
  "linearUsername": null
}
```

### 3. Create the vault files

```sh
swelog setup
```

Swelog creates the work and context files plus the daily and weekly log
directories under `<obsidianVaultPath>/<swelogFolderName>`.

### 4. Add your context

Open `CONTEXT.md` and describe your role, team, systems, and priorities. Swelog
includes this context when generating daily and weekly summaries.

### 5. Record and summarize work

```sh
swelog log "Met with the payments team about retry failures"
swelog summarize
```

The first command adds a bullet to the `## Log` section of `WORK.md`. The second
creates today's daily log and resets `WORK.md` unless `--keep` is provided.

## Daily Workflow

1. Add notes manually in Obsidian or from the terminal:

   ```sh
   swelog log "Reviewed the checkout migration plan"
   ```

2. Optionally fetch activity from GitHub or Linear:

   ```sh
   swelog fetch github
   swelog fetch linear
   ```

3. Review and edit `WORK.md`. Integration sections are created only after the
   corresponding integration is used.

4. Generate the daily log:

   ```sh
   swelog summarize
   ```

5. Generate a weekly log when ready:

   ```sh
   swelog summarize week
   ```

## Backfilling a Missed Day

Every command that is tied to a day accepts `--date MM-DD-YYYY`, so a day you
missed can be reconstructed later. `WORK.md` is always the scratch pad the
summary is built from; the date flags only decide which day is fetched and which
file the summary is written to.

To backfill Monday, 08-17-2026:

1. Reset the work file so the day starts clean:

   ```sh
   swelog reset
   ```

2. Write Monday's notes in `WORK.md`, or add them from the terminal:

   ```sh
   swelog log "Debugged the retry storm in the payments worker"
   ```

3. Fetch Monday's integration activity:

   ```sh
   swelog fetch github --date 08-17-2026
   swelog fetch linear --date 08-17-2026
   ```

4. Summarize into Monday's daily log:

   ```sh
   swelog summarize day --date 08-17-2026
   ```

   This writes `Daily/08-17-2026.md` and resets `WORK.md`, so the next missed day
   can be backfilled straight away. Add `--force` to replace a daily log that
   already exists, or `--keep` to leave `WORK.md` alone.

5. Once the week's daily logs exist, summarize the week:

   ```sh
   swelog summarize week --week-of 08-17-2026
   ```

| Command | Backfill flag |
| --- | --- |
| `swelog fetch github` | `--date MM-DD-YYYY` |
| `swelog fetch linear` | `--date MM-DD-YYYY` |
| `swelog summarize day` | `--date MM-DD-YYYY` |
| `swelog summarize week` | `--week-of MM-DD-YYYY` |

`swelog log` and `swelog reset` need no date; they act on `WORK.md`, which holds
whichever day you are currently assembling.

Linear backfill is best-effort. See [Linear](#linear) for what it can and cannot
recover.

## Configuration

All generated files and directories live inside
`<obsidianVaultPath>/<swelogFolderName>`.

| Field | Required | Description |
| --- | --- | --- |
| `obsidianVaultPath` | Yes | Absolute path to the Obsidian vault. Commands fail until this is configured. |
| `swelogFolderName` | Yes | Folder created inside the vault for Swelog files. |
| `workFileName` | Yes | Markdown file used to collect the current day's raw notes. |
| `contextFileName` | Yes | Markdown file containing role, team, systems, and priority context. |
| `dailyLogFolderName` | Yes | Directory containing generated daily logs. |
| `weeklyLogFolderName` | Yes | Directory containing generated weekly logs. |
| `llm` | Yes | Summarization provider: `ollama`, `openAi`, or `openRouter`. |
| `llmModel` | Yes | Model identifier interpreted by the configured provider. |
| `linearUsername` | No | Exact Linear assignee name used by `swelog fetch linear`. Leave as `null` when Linear is not used. |

Display the current configuration and its location:

```sh
swelog config
```

## Summarization

Swelog uses the same configured model for daily and weekly summaries.

### Commands

| Command | Behavior |
| --- | --- |
| `swelog summarize` | Generate today's daily log. Alias for `swelog summarize day`. |
| `swelog summarize day` | Generate today's daily log and reset `WORK.md`. |
| `swelog summarize day --keep` | Generate the daily log without resetting `WORK.md`. |
| `swelog summarize day --force` | Replace an existing daily log for today. |
| `swelog summarize day --date MM-DD-YYYY` | Write the daily log for the supplied date instead of today. |
| `swelog summarize week` | Summarize available Monday-Friday daily logs for the default week. |
| `swelog summarize week --week-of MM-DD-YYYY` | Summarize the week beginning on the supplied Monday. |
| `swelog summarize week --force` | Replace an existing weekly log. |

Without `--week-of`, weekly summarization uses the current week's Monday. When
run on a Monday, it uses the previous Monday. The command skips missing weekday
logs but fails when no daily logs exist for the selected week.

Weekly summarization requires `WORK.md` to be empty/default so unfinished daily
notes are not accidentally excluded from a daily log.

Daily summarization always reads the current `WORK.md`. `--date` only chooses the
file the summary is written to, so it is what you use to backfill a missed day.
See [Backfilling a Missed Day](#backfilling-a-missed-day).

### Providers and Models

| Provider | `llm` | Example `llmModel` | Setup | Authentication | Notes |
| --- | --- | --- | --- | --- | --- |
| Ollama | `ollama` | `llama3.2` | Install Ollama, pull the model, and run the local service. | None | Swelog connects to Ollama at `localhost:11434`. |
| OpenAI | `openAi` | `gpt-5.4-mini` | Select an OpenAI model supported by the Responses API. | API key, prompted on first use | The model value is sent directly to OpenAI. |
| OpenRouter | `openRouter` | `openai/gpt-5.4-mini` | Select an OpenAI Responses-compatible model through OpenRouter. | API key, prompted on first use | The current implementation expects OpenAI Responses API output. |

Model names are examples, not a fixed allowlist. Set `llmModel` to a model that
is installed locally or available from the selected provider.

### Ollama Setup

1. Install and start Ollama.
2. Pull the configured model:

   ```sh
   ollama pull llama3.2
   ```

3. Configure Swelog:

   ```json
   {
     "llm": "ollama",
     "llmModel": "llama3.2"
   }
   ```

4. Run `swelog summarize`. Ensure Ollama remains available at
   `localhost:11434` while the command runs.

### OpenAI Setup

1. Configure the provider and model:

   ```json
   {
     "llm": "openAi",
     "llmModel": "gpt-5.4-mini"
   }
   ```

2. Run `swelog summarize`. The first run asks for your
   [OpenAI API key](https://platform.openai.com/api-keys) and stores it in your
   operating system keyring.

### OpenRouter Setup

1. Configure the provider and an OpenAI Responses-compatible model:

   ```json
   {
     "llm": "openRouter",
     "llmModel": "openai/gpt-5.4-mini"
   }
   ```

2. Run `swelog summarize`. The first run asks for your
   [OpenRouter API key](https://openrouter.ai/keys) and stores it in your
   operating system keyring.

Never store API keys in `swelog.json`. See [Authentication](#authentication) for
how Swelog stores and clears them.

## Integrations

Integrations are optional. The default work file contains only `## Focus` and
`## Log`; integration sections are created lazily when activity is fetched.

| Integration | Command | Configuration | Authentication | Work-file output |
| --- | --- | --- | --- | --- |
| GitHub | `swelog fetch github` | None | Personal access token, prompted on first use | Pull requests opened or merged on the selected date. |
| Linear | `swelog fetch linear` | `linearUsername` | Browser OAuth on first use | Assigned issues grouped by their current status. |

Swelog surrounds generated sections with invisible HTML markers. Treat the
content inside these managed blocks as generated data because future fetches
replace it.

### GitHub

GitHub fetching records pull requests you opened or merged.

1. Fetch today's activity:

   ```sh
   swelog fetch github
   ```

   The first run asks for a
   [GitHub personal access token](https://github.com/settings/tokens) with the
   `repo` scope and stores it in your operating system keyring.

2. To fetch a previous date, provide `MM-DD-YYYY`:

   ```sh
   swelog fetch github --date 08-17-2026
   ```

The command creates or updates a managed `## GitHub` section before `## Log`.
Never store the GitHub token in `swelog.json`.

### Linear

Linear fetching records issues assigned to a configured workspace user.
Without `--date` it records your currently active issues and omits completed and
canceled ones.

1. Set the exact Linear assignee name in `swelog.json`:

   ```json
   {
     "linearUsername": "rudrapatel"
   }
   ```

2. Run the fetch command:

   ```sh
   swelog fetch linear
   ```

3. On first use, Swelog prints a Linear authorization URL and attempts to open
   it in your browser. If the browser does not open, use the printed URL.

4. Complete authorization. Swelog stores the resulting OAuth credentials in
   your operating system keyring and reuses or refreshes them on later runs.

5. To backfill a previous date, provide `MM-DD-YYYY`:

   ```sh
   swelog fetch linear --date 08-17-2026
   ```

   This records the issues Linear shows activity on for that date, including the
   ones you completed or canceled, because finishing an issue is exactly what a
   past day's log should capture.

The generated section groups issues by status:

```markdown
## Linear
### In Progress
- [ENG-123](https://linear.app/...) Implement OAuth callback handling

### Todo
- [ENG-456](https://linear.app/...) Document the Linear integration
```

When no issues match, Swelog removes its managed Linear section.

#### What `--date` can and cannot recover

The Linear MCP server does not expose issue history, so Swelog can only match on
the timestamps Linear stores on the issue itself: `updatedAt`, `completedAt`,
`canceledAt`, `startedAt`, and `createdAt`. An issue is recorded for a date when
any of those lands on it, read in your local time zone.

`completedAt`, `canceledAt`, `startedAt`, and `createdAt` never move, so issues
you started, finished, canceled, or filed on a date are recovered no matter how
much later you backfill. `updatedAt` only holds the most recent change, so an
issue you touched on Monday and touched again on Tuesday is attributed to
Tuesday alone. Backfill soon after the fact for the most complete result, and
treat `--date` output as a prompt for your own notes rather than a full record.

To switch Linear accounts or organizations, update `linearUsername` when needed,
clear the stored authorization, and fetch again:

```sh
swelog auth clear linear
swelog fetch linear
```

The next fetch starts a new browser authorization flow.

## Authentication

Swelog stores every credential in your operating system's credential store —
Keychain on macOS, Credential Manager on Windows, and the Secret Service on
Linux. Nothing is written to `swelog.json` or any other file in your vault.

| Credential | Used by | How it is obtained |
| --- | --- | --- |
| GitHub token | `swelog fetch github` | Prompted on first use |
| OpenAI API key | `swelog summarize` with `"llm": "openAi"` | Prompted on first use |
| OpenRouter API key | `swelog summarize` with `"llm": "openRouter"` | Prompted on first use |
| Linear authorization | `swelog fetch linear` | Browser OAuth on first use |

Review what is stored — values are never printed:

```sh
swelog auth status
```

If a credential is revoked or rejected, clear it and run the command again to
enter a new one. Every authorization error names this command:

```sh
swelog auth clear github
swelog auth clear --all
```

### Environment variable overrides

`GITHUB_TOKEN`, `OPENAI_API_KEY`, and `OPENROUTER_API_KEY` take precedence over
the keyring when set, so CI and scripted runs work without a credential store.
When one of these is set, `swelog auth status` reports it instead of the stored
value.

In a non-interactive session — a pipe, a cron job, a CI job — Swelog never
prompts. It fails immediately with the name of the environment variable to set.

## Other Commands

| Command | Behavior |
| --- | --- |
| `swelog log "message"` | Append a bullet to the `## Log` section. |
| `swelog reset` | Reset `WORK.md` to its default Focus-and-Log template. |
| `swelog setup --force` | Recreate and overwrite configured Swelog files. |
| `swelog config` | Display the current configuration and config-file path. |
| `swelog auth status` | Show which credentials are stored, without printing them. |
| `swelog auth clear <credential>` | Remove a stored credential so the next command asks for it again. |

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

### Pull Request Process

Create a branch and open a pull request. Branch naming is flexible, but pull
request titles must use Conventional Commit style, for example:

```text
feat: add weekly log generation
```

All pull requests must pass `just pr`.
