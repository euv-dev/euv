---
title: Getting Started
---

# Getting Started

## Prerequisites

- Rust 1.97+ with the `wasm32-unknown-unknown` target
- `euv-cli` installed (`cargo install euv-cli`)

## Use this template

Click **Use this template** on GitHub, or clone the repository:

```bash
git clone https://github.com/euv-dev/euv-docs.git my-docs
cd my-docs
```

## Write your first page

Create `docs/guide/hello.md`:

```markdown
# Hello

My first page.
```

The file becomes available at `#/guide/hello.html` and appears in the
sidebar automatically.

## Build

```bash
euv build -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack
```

The static site is emitted to `www/`.

## Dev server

```bash
euv run --dev --port 8080 -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack
```

Then open <http://localhost:8080>.

::: tip
The dev server watches `src/` and `docs/` — markdown edits rebuild
automatically.
:::
