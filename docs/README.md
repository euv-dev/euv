---
# Site-level configuration for the euv-docs demo home page
# (consumed by build.rs when EUV_DOCS_SRC_DIR is unset, e.g.
# during `cargo clippy` / `cargo test` in the euv-docs crate itself).
site:
  title: Euv-docs
  description: A documentation site generator powered by euv + euv-ui.
  logo: '📘'
locales:
  - prefix: /
    # Content directory under docs/docs/. Required: the build refuses to
    # guess a locale's content location, and fails loudly when it is empty
    # instead of emitting a one-page site.
    dir: en
    lang: en-US
    label: English
    title: Euv-docs
    description: A documentation site generator powered by euv + euv-ui.
    footer: MIT Licensed | Built with euv + euv-ui
    toc_label: On this page
    prev_label: Previous
    next_label: Next
    navbar:
      - { text: Home, link: / }
      - { text: Guide, link: /guide/ }
      - { text: Components, link: /components/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
---

<div align="center">

# Euv-docs

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="Euv-docs logo" height="160">

**Zero-JS Static Documentation Site Generator Powered by euv + euv-ui**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-docs)](https://crates.io/crates/euv-docs)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-docs)](https://crates.io/crates/euv-docs)
[![Docs.rs](https://img.shields.io/docsrs/euv-docs?style=flat-square)](https://docs.rs/euv-docs)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-docs/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Zero-JS static documentation site generator powered by `euv` + `euv-ui` (Rust/WASM), featuring YAML frontmatter site configuration, localized navigation, live client-side search, footer item declarations, and native VS Code Eva Bold syntax highlighting.

## Responsibilities & Module Layout

- `build.rs` — Reads YAML frontmatter from site `README.md` and compiles Markdown trees with zero guessing
- `site` / `theme` — Responsive sidebar, table of contents, locale switcher, footer items (`footer_link`, `footer_items`), and live search
- `docs/docs/en/` — Canonical framework guides, component catalog, and GFM + Eva Bold syntax verification pages

## Footer Items

The footer is one row: the locale's `footer` text, then the items the site declares. Every part of the content comes from the site configuration (`footer_link`, `footer_items` with `text`, `link`, optional `icon`, and `alt`). The row never wraps; an item that does not fit is cut with an ellipsis so filing notices remain legible and copyable.

## Installation & Usage

```sh
cargo install euv-docs
cargo run -p euv-docs -- --generate docs/docs/en target/docs-preview
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-docs --all-features --all-targets -- -D warnings
cargo test -p euv-docs --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
