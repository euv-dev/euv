<div align="center">

# euv-automation

**Dev-Only Async AI Schema v2 Bridge, `euv.ui-e2e.v1` & Programmatic UI Automation**

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

`euv-automation` provides deterministic headless UI querying, pointer/keyboard/scroll interaction automation (`UiAutomator`), the `euv.ui-e2e.v1` / `euv.ui-e2e.result.v1` recorder & replayer, and the dev-only async **AI Schema v2** JSON protocol (`AiSchemaProtocol` / `DevAutomationBridge`) for inspecting live `euv` scene trees, accessibility roles, hit-test bounds, password redaction, same-frame captures, and reactive telemetry. All dev automation endpoints are strictly gated behind `cfg(debug_assertions)` and compile down to **0 bytes** in release builds.

## Responsibilities & Module Layout

- `automator` — High-level `UiAutomator` driver with zero-snapshot direct `SceneTree` queries, synthetic clicks, hovers, text input, resizes, and scripted flows
- `schema` — `AiSchemaRequest` / `AiSchemaResponse` protocol handlers (`describe`, `snapshot`, `query`, `hit_test`, `action`, `capture_same_frame`) with secret redaction plus optional `AiAppHooks`
- `bridge` — Resident background-thread loopback HTTP dev bridge (`DevAutomationBridge`) with bounded backpressure queue and `poll_budgeted` in debug builds only
- `e2e` — `UiE2eFlowV1`, `UiE2eRecorder`, `UiE2eReplayer`, `UiE2eResultV1`, and `UiE2eMigrationProposal` deterministic E2E testing contracts

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
