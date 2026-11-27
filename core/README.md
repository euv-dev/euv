<div align="center">

# euv-core

<img src="https://docs.ltpp.vip/img/euv-core.png" alt="euv-core logo" height="160">

[![Crates.io Version](https://img.shields.io/crates/v/euv-core.svg?style=flat-square)](https://crates.io/crates/euv-core)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-core.svg?style=flat-square)](https://crates.io/crates/euv-core)
[![Docs.rs](https://img.shields.io/docsrs/euv-core?style=flat-square)](https://docs.rs/euv-core)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/Rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![License: MIT](https://img.shields.io/crates/l/euv-core.svg?style=flat-square)](../LICENSE)

[API Docs](https://docs.rs/euv-core/latest/) · [Workspace README](../README.md)

</div>

> Core cross-platform native runtime for `euv`: generation-indexed reactive arena, deterministic Flexbox/Grid layout with O(1) clean-subtree caching, flat retained scene graph, zero-cost macro Go-to-Definition schemas (`view_elements`, `style_props`), native OS font & exact `TextCaret` rasterization, multi-mode Software/Hardware/Hybrid rendering, 2D/3D ray-traced game engine, pure-Rust media codecs, and direct OS backends across 7 targets.

## Key Modules

- **`reactive`**: `Signal`, `Memo`, `Effect`, `Selector`, `Scope` (with `freeze` / `resume`), `LocalState`, `GlobalState`, and typed `EventBus`.
- **`scene`**: Flat `SceneTree`, `Context`, `Component` + standalone `Lifecycle`, `NodeTransition`, and zero-cost IDE Go-to-Definition `dsl::{view_elements, style_props}`.
- **`layout`**: Deterministic Flexbox, Grid, Absolute/Fixed positioning, SafeArea insets, hit-testing, and clean-subtree translation caching.
- **`font`**: Cross-platform proportional, monospace, bold, and italic glyph rasterization (CoreText, Android Skia Paint JNI, FreeType, Win32 GDI, Web Canvas2D) paired with `GlyphAtlas`.
- **`render`**: `DisplayList`, `DrawCommand::TextCaret`, multi-core analytical SDF software rasterizer, and Metal / Vulkan / D3D12 / WebGL2 GPU pipelines.
- **`game` & `media`**: 2D physics ECS + 3D BVH ray-traced open-world engine (`FullPathTracedGi`) and pure-Rust Image, SVG, GIF, LIVE Photo, Video, Audio, PDF, and sanitized HTML5 decoders.
- **`dynamic`**: Signed component/page OTA bundles and WebAssembly hot-swap runtime.

## Installation

```shell
cargo add euv-core
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
