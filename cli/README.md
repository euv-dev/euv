---
# euv-docs site configuration (consumed by `euv-docs` build.rs).
# This frontmatter is the SINGLE source of truth for site-wide
# settings — replaces the legacy `docs/config.toml`.
site:
  title: Euv-docs
  description: A VuePress-style documentation site powered by euv + euv-ui.
  logo: '📘'
locales:
  - prefix: /
    lang: en-US
    label: English
    title: Euv-docs
    description: A VuePress-style documentation site powered by euv + euv-ui.
    footer: MIT Licensed | Built with euv + euv-ui
    toc_label: On this page
    prev_label: Previous
    next_label: Next
    navbar:
      - { text: Home, link: / }
      - { text: Guide, link: /guide/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
  - prefix: /zh/
    lang: zh-CN
    label: 简体中文
    title: Euv-docs
    description: 由 euv + euv-ui 驱动的 VuePress 风格文档站。
    footer: MIT 许可 | 基于 euv + euv-ui 构建
    toc_label: 本页目录
    prev_label: 上一页
    next_label: 下一页
    navbar:
      - { text: 首页, link: /zh/ }
      - { text: 指南, link: /zh/guide/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
---

<div align="center">

# Euv-cli

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="Euv-cli logo" height="160">

**7-Platform Build CLI, Dev Server & Layered Unified Formatter**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-cli)](https://crates.io/crates/euv-cli)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-cli)](https://crates.io/crates/euv-cli)
[![Docs.rs](https://img.shields.io/docsrs/euv-cli?style=flat-square)](https://docs.rs/euv-cli)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-cli/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Official CLI for the `euv` native Rust UI framework: 7-platform build orchestration, live SSE hot-reload dev server, zero-JS WebAssembly & static docs bundling, project scaffolding, toolchain doctor, and deterministic `euv fmt` (running `crate-cli` Prettier-aligned multi-format formatting first, then normalizing `view!`/`style!`/`tokens!` macro blocks and workspace `cargo fmt`).

## Responsibilities & Module Layout

- `fmt` — Layered formatter invoking `crate_cli::execute_unified_fmt` (`.md`, `.yml`/`.yaml`, `.json`, `.toml`, derives, `cargo fmt`) followed by `euv` macro indentation normalization
- `build` / `dev` / `docs` — 7-platform target orchestration, live SSE hot-reload server, and zero-JS static documentation bundling
- `doctor` / `new` — Cross-platform toolchain verification and project scaffolding

## Installation & Usage

```sh
cargo install euv-cli
euv doctor
euv fmt --check
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-cli --all-features --all-targets -- -D warnings
cargo test -p euv-cli --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
