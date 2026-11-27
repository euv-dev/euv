<div align="center">

# euv-automation

**Dev-Only AI Schema v1 Bridge & Programmatic UI Automation**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-automation)](https://crates.io/crates/euv-automation)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-automation)](https://crates.io/crates/euv-automation)
[![Docs.rs](https://img.shields.io/docsrs/euv-automation?style=flat-square)](https://docs.rs/euv-automation)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-automation/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

`euv-automation` provides deterministic headless UI querying, pointer/keyboard/scroll interaction automation (`UiAutomator`), and the dev-only **AI Schema v1** JSON protocol (`AiSchemaProtocol` / `DevAutomationBridge`) for inspecting live `euv` scene trees, accessibility roles, hit-test bounds, and reactive telemetry. All dev automation endpoints are strictly gated behind `cfg(debug_assertions)` and compile down to **0 bytes** in release builds.

## Responsibilities & Module Layout

- `automator` — High-level `UiAutomator` driver for label/role queries, synthetic clicks, hovers, text input, resizes, and scripted flows
- `schema` — `AiSchemaRequest` / `AiSchemaResponse` protocol handlers (`describe`, `snapshot`, `query`, `hit_test`, `action`) plus optional `AiAppHooks`
- `bridge` — Local loopback HTTP dev bridge (`DevAutomationBridge`) serving `euv.ai.schema.v1` in debug builds only

## Installation & Usage

```sh
cargo add euv-automation
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-automation --all-features --all-targets -- -D warnings
cargo test -p euv-automation --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
