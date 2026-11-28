<div align="center">

# Euv-core

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="Euv-core logo" height="160">

**Cross-Platform Native Runtime, Layout, Rendering, 3D Engine & 7 OS Backends**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-core)](https://crates.io/crates/euv-core)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-core)](https://crates.io/crates/euv-core)
[![Docs.rs](https://img.shields.io/docsrs/euv-core?style=flat-square)](https://docs.rs/euv-core)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-core/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Core cross-platform native runtime for `euv`: generation-indexed reactive arena, deterministic Flexbox/Grid layout with O(1) clean-subtree caching, flat retained scene graph, zero-cost macro Go-to-Definition schemas (`view_elements`, `style_props`), native OS font & exact `TextCaret` rasterization, multi-mode Software/Hardware/Hybrid rendering, 2D/3D ray-traced game engine, pure-Rust media codecs, dynamic WASM OTA, and direct OS backends across 7 targets.

## Responsibilities & Module Layout

- `reactive` — `Signal`, `Memo`, `Effect`, `Selector`, `Scope` (with `freeze` / `resume`), `LocalState`, `GlobalState`, and typed `EventBus`
- `scene` — Flat `SceneTree`, ambient `Context`, `Component` + standalone `Lifecycle`, `NodeTransition`, `TextNodeContent`, and IDE Go-to-Definition `dsl::{view_elements, style_props}`
- `layout` — Deterministic Flexbox, Grid, Absolute/Fixed positioning, SafeArea insets, hit-testing, O(1) clean-subtree translation caching, and surgical partial-layout conservative bounds diffing
- `font` — Native OS proportional, monospace, bold, and italic glyph rasterization (CoreText, Android Skia Paint JNI, FreeType, Win32 GDI, Web Canvas2D) paired with bounded LRU `GlyphAtlas` & `TextLayoutCache`
- `render` — `DisplayList`, `DrawCommand::TextCaret`, analytical SDF software rasterizer, order-preserving Metal / Vulkan / D3D12 / WebGL2 GPU batching, and incremental ANSI cell-diff `TerminalRenderer` (interactive TUI & static plain-text CLI)
- `app` & `platform` — Opt-in `FrameTelemetry`, 7-platform `SystemEvent` lifecycle hooks, and 0% CPU / 0% GPU background freeze with lossless foreground restore across Web, Android, iOS, iPadOS, macOS, Windows, and Linux
- `game` & `media` — 2D physics ECS + 3D BVH ray-traced open-world engine (`FullPathTracedGi`) and pure-Rust Image, SVG, GIF, LIVE Photo, Video, Audio, PDF, and HTML5 decoders
- `dynamic` — Cryptographically signed component/page OTA bundles and WebAssembly hot-swap runtime

## Installation & Usage

```sh
cargo add euv-core
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-core --all-features --all-targets -- -D warnings
cargo test -p euv-core --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
