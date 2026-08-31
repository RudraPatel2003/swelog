# swelog-cli docs

## Description

The documentation website for `swelog-cli`, built with
[Astro](https://astro.build) and [Starlight](https://starlight.astro.build).

## Recommended VSCode Extensions

- ESLint
- Prettier
- Astro
- markdownlint

## Prerequisites

- Node.js 22 or newer
- pnpm 11 or newer

## Local Development

Install the dependencies:

```sh
pnpm install
```

Start the development server:

```sh
pnpm dev
```

Run the linter:

```sh
pnpm lint
```

Fix lint and formatting issues:

```sh
pnpm lint:fix
```

Format the Markdown content:

```sh
pnpm format
```

Check for dead code:

```sh
pnpm knip
```

Build the production site:

```sh
pnpm build
```

Preview the production site:

```sh
pnpm preview
```

Each command is also available from the repository root as a `just` recipe, for
example `just docs-dev` and `just docs-build`.

## Deployment

The site is hosted on Vercel with the project root directory set to `docs`.
