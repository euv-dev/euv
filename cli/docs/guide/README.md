---
title: Introduction
order: -1
---

# Introduction

**euv-docs** turns a directory of markdown files into a complete
documentation site — just like VuePress, but rendered by a Rust/WASM
[euv](https://github.com/euv-dev/euv) application.

## How it works

1. You write markdown files under `docs/`.
2. `build.rs` parses them (frontmatter + VuePress-flavored markdown) and
   generates a Rust site model.
3. `euv build` compiles everything into a single WASM bundle.

## Directory layout

| Source file | Route |
| --- | --- |
| `docs/README.md` | `/` |
| `docs/guide/README.md` | `/guide/` |
| `docs/guide/getting-started.md` | `/guide/getting-started.html` |
| `docs/zh/README.md` | `/zh/` |

## Next steps

Head to [Getting Started](./getting-started.md) to build your own site.
