<div align="center">

# euv-macros

<img src="https://docs.ltpp.vip/img/euv-macros.png" alt="euv-macros logo" height="160">

[![Crates.io Version](https://img.shields.io/crates/v/euv-macros.svg?style=flat-square)](https://crates.io/crates/euv-macros)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-macros.svg?style=flat-square)](https://crates.io/crates/euv-macros)
[![Docs.rs](https://img.shields.io/docsrs/euv-macros?style=flat-square)](https://docs.rs/euv-macros)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/Rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![License: MIT](https://img.shields.io/crates/l/euv-macros.svg?style=flat-square)](../LICENSE)

[API Docs](https://docs.rs/euv-macros/latest/) · [Macro Authoring Guidelines](../AGENTS.md) · [Workspace README](../README.md)

</div>

> Zero-overhead procedural macros for `euv`, compiling declarative UI, newline-separated styles, reactive bindings, component lifecycles, global/local state, and typed cross-component events directly into retained scene operations with full rust-analyzer Go-to-Definition support.

## Provided Macros

| Macro                                                      | Purpose                                                                                                                                                 |
| :--------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `view! { ... }`                                            | Zero-context declarative scene construction with IDE Go-to-Definition on built-in tags (`euv::view_elements`), properties, and custom component structs |
| `style! { ... }`                                           | Compile-time newline-separated `Style` builder (one rule per line, no `;`) with IDE Go-to-Definition via `euv::style_props`                             |
| `#[component(retention = Freeze)]`                         | Splits `impl MyComp` into `Lifecycle` + `Component` with configurable retention (`Freeze`, `Destroy`, `Alive`)                                          |
| `#[event]` / `emit!` / `on_event!`                         | Typed cross-component `EventBus` declarations, dispatching, and scope-owned subscriptions                                                               |
| `#[global_state_store]` / `global_state!` / `local_state!` | App-wide typed global state and scope-owned reactive local state                                                                                        |
| `watch!` / `computed!` / `tokens!` / `euv_main!`           | Concise reactive effects, memos, typed design tokens, and 7-platform app entry points                                                                   |

## Installation

```shell
cargo add euv-macros
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
