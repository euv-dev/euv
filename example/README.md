<div align="center">

# Euv-example

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="Euv-example logo" height="160">

**15-Route Cross-Platform Reference Showcase & Interactive Stress Lab**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-example)](https://crates.io/crates/euv-example)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-example)](https://crates.io/crates/euv-example)
[![Docs.rs](https://img.shields.io/docsrs/euv-example?style=flat-square)](https://docs.rs/euv-example)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-example/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Full 15-route cross-platform reference showcase for `euv` + `euv-ui`, featuring Twitter/X-style bottom navigation, home module directory, horizontal swipe routing, live navbar FPS telemetry, gesture-draggable drawers & sheets, VS Code Eva Light/Dark Bold Markdown & Code Studio, 60 FPS 3D ray-traced sandbox, pure-Rust Media Studio, Dynamic WASM OTA, and an interactive 10-scenario Stress Lab.

## Responsibilities & Module Layout

- `app` — 7-platform bootstrap, responsive shell, bottom navigation, and system-back double-tap exit guard
- `pages` — 15 lazy `PageRetention::Freeze` showcase pages covering UI, Forms, Overlays, Markdown/Code, Media, 2D/3D Engine, OTA, and Stress Lab
- `scripts/build/build-web.sh` & `build-theme-variants.sh` — Zero-JS WebAssembly bundle and 17-theme variant packaging

## Installation & Usage

```sh
cargo run -p euv-example                     # Native desktop window
cargo run -p euv-example -- --headless       # Headless deterministic verification
./scripts/build/build-web.sh                 # Zero-JS WebAssembly bundle
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-example --all-features --all-targets -- -D warnings
cargo test -p euv-example --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
