<div align="center">

# euv-macros

<img src="https://docs.ltpp.vip/img/euv-macros.png" alt="euv-macros logo" height="160">

**Zero-Overhead Declarative UI, Style, Lifecycle & Reactive Proc Macros**

[![Crates.io Version](https://img.shields.io/crates/v/euv-macros.svg?style=flat-square)](https://crates.io/crates/euv-macros)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-macros.svg?style=flat-square)](https://crates.io/crates/euv-macros)
[![Docs.rs](https://img.shields.io/docsrs/euv-macros?style=flat-square)](https://docs.rs/euv-macros)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/euv-macros.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-macros/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Zero-overhead procedural macros for `euv`, compiling declarative UI (`view!`), newline-separated styles (`style!`), reactive bindings, component lifecycles, global/local state, and typed cross-component events directly into retained scene operations with full rust-analyzer Go-to-Definition support and zero macro-emitted `use` statements.

## Responsibilities & Module Layout

- `view! { ... }` — Zero-context declarative scene construction with IDE Go-to-Definition on built-in tags (`euv::view_elements`), properties, and custom component structs
- `style! { ... }` — Compile-time newline-separated `Style` builder (one rule per line, no `;`) with IDE Go-to-Definition via `euv::style_props`
- `#[component(retention = Freeze)]` — Splits `impl MyComp` into `Lifecycle` + `Component` with configurable retention (`Freeze`, `Destroy`, `Alive`)
- `#[event]` / `emit!` / `on_event!` — Typed cross-component `EventBus` declarations, dispatching, and scope-owned subscriptions
- `#[global_state_store]` / `global_state!` / `local_state!` — App-wide typed global state and scope-owned reactive local state
- `watch!` / `computed!` / `tokens!` / `euv_main!` — Concise reactive effects, memos, typed design tokens, and 7-platform app entry points

## Installation & Usage

```sh
cargo add euv-macros
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-macros --all-features --all-targets -- -D warnings
cargo test -p euv-macros --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
