# Contributing

## Prerequisites

- Rust
- Just (`cargo install just`)
- Cargo Nextest (`cargo install cargo-nextest`)
- Obsidian
- Ollama when testing the optional summarization feature
- Provider credentials when testing hosted models or integrations
- A Google Cloud OAuth client when testing the Google Calendar integration

## Development

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

Check the current version of the CLI when preparing for a release:

```sh
just list-release-version
```

Update the version of the CLI when preparing for a release:

```sh
just update-release-version <release-tag>
```

## Google Calendar OAuth Client

`swelog fetch google-calendar` authorizes against a Google OAuth client that is compiled into the binary.

The release binary gets it from the `SWELOG_GOOGLE_CLIENT_ID` and `SWELOG_GOOGLE_CLIENT_SECRET` repository secrets. Both are required to build the release binary. Debug builds do not need them, but will fail if you try to fetch Google Calendar data without them.

To test the integration locally, create your own client:

1. In the [Google Cloud Console](https://console.cloud.google.com/), create a
   project and enable the Google Calendar API.
2. On the OAuth consent screen, add the `https://www.googleapis.com/auth/calendar.events.readonly` scope, and add your own Google account as a test user.
3. Under Credentials, create an OAuth client ID of type Desktop app. Google treats the secret it issues as non-confidential, which is why it can ship in the binary.

Then export both variables in the shell you build from:

```sh
export SWELOG_GOOGLE_CLIENT_ID="<client-id>.apps.googleusercontent.com"
export SWELOG_GOOGLE_CLIENT_SECRET="<client-secret>"

just run fetch google-calendar
```

## Documentation Site

See [docs/README.md](./docs/README.md)

## Pull Request Process

Create a branch and open a pull request. Branch naming is flexible, but pull request titles must use Conventional Commit style, for example:

```text
feat: add weekly log generation
```
