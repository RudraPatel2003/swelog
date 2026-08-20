---
title: Authentication
description: How swelog stores, inspects, and clears your credentials.
---

Swelog stores every credential in your operating system's credential store —
Keychain on macOS, Credential Manager on Windows, and the Secret Service on
Linux. Nothing is written to `swelog.json` or any other file in your vault.

| Credential           | Used by                                       | How it is obtained         |
| -------------------- | --------------------------------------------- | -------------------------- |
| GitHub token         | `swelog fetch github`                         | Prompted on first use      |
| OpenAI API key       | `swelog summarize` with `"llm": "openAi"`     | Prompted on first use      |
| OpenRouter API key   | `swelog summarize` with `"llm": "openRouter"` | Prompted on first use      |
| Linear authorization | `swelog fetch linear`                         | Browser OAuth on first use |

## Inspecting stored credentials

Review what is stored — values are never printed:

```sh
swelog auth status
```

## Clearing a credential

If a credential is revoked or rejected, clear it and run the command again to
enter a new one. Every authorization error names this command:

```sh
swelog auth clear github
swelog auth clear --all
```

`swelog auth clear` accepts `github`, `open-ai`, `open-router`, and `linear`.

:::note
The values accepted by `swelog auth clear` are hyphenated (`open-ai`,
`open-router`), which differs from the camelCase spelling used for the `llm`
field in `swelog.json` (`openAi`, `openRouter`).
:::

## Environment variable overrides

`GITHUB_TOKEN`, `OPENAI_API_KEY`, and `OPENROUTER_API_KEY` take precedence over
the keyring when set, so CI and scripted runs work without a credential store.
When one of these is set, `swelog auth status` reports it instead of the stored
value.

Linear is authorized through the browser and has no environment variable
override.

In a non-interactive session — a pipe, a cron job, a CI job — Swelog never
prompts. It fails immediately with the name of the environment variable to set.
