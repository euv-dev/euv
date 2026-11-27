---
# euv-docs site configuration (consumed by `euv-docs` build.rs).
# This frontmatter is the SINGLE source of truth for site-wide
# settings — replaces the legacy `docs/config.toml`.
site:
  title: euv-docs
  description: A VuePress-style documentation site powered by euv + euv-ui.
  logo: '📘'
locales:
  - prefix: /
    lang: en-US
    label: English
    title: euv-docs
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
    title: euv-docs
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

# euv-cli

<img src="https://docs.ltpp.vip/img/euv-cli.png" alt="euv-cli logo" height="160">

[![Crates.io Version](https://img.shields.io/crates/v/euv-cli.svg?style=flat-square)](https://crates.io/crates/euv-cli)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-cli.svg?style=flat-square)](https://crates.io/crates/euv-cli)
[![Docs.rs](https://img.shields.io/docsrs/euv-cli?style=flat-square)](https://docs.rs/euv-cli)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/Rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![License: MIT](https://img.shields.io/crates/l/euv-cli.svg?style=flat-square)](../LICENSE)

[API Docs](https://docs.rs/euv-cli/latest/) · [Workspace README](../README.md)

</div>

> Official CLI for the `euv` native Rust UI framework: 7-platform build orchestration, live SSE hot-reload dev server, zero-JS WebAssembly & static docs bundling, project scaffolding, toolchain doctor, and deterministic `euv fmt` macro indentation formatting.

## Commands

```shell
euv doctor                          # Inspect all 7 target toolchains
euv new my-app                      # Scaffold a cross-platform euv project
euv dev --target web --port 8080    # Live SSE hot-reload & OTA dev server
euv build --target all --release    # Build across Web, Android, iOS, macOS, Windows, Linux
euv docs build ./docs --out ./dist  # Build zero-JS documentation site
euv fmt --check                     # Format & verify Rust + view!/style!/tokens! macro blocks
```

## Installation

```shell
cargo install euv-cli
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
