---
home: true
title: Euv
order: 0
tagline: High-performance cross-platform native Rust UI framework targeting Web, Android, iOS, iPadOS, macOS, Windows, and Linux via direct system APIs.
---

## Why Euv

`euv` replaces Web-only DOM diffing with a unified native Rust architecture:

- **Direct OS API Adapters**: Web, Android, iOS, iPadOS, macOS, Windows, and Linux with zero WebView overhead.
- **Zero-Context Macros + IDE Go-to-Definition**: `view!` & newline-separated `style!` compile straight to retained scene nodes while jumping to `euv::view_elements`, `euv::style_props`, and custom component structs in rust-analyzer.
- **Fine-Grained Push-Pull Reactivity & Freeze Router**: `Signal`, `Memo`, `Effect`, `Selector`, `LocalState`, `GlobalState`, `EventBus`, and lazy `PageRetention::Freeze` routing.
- **Adaptive Native UX**: Persistent Twitter/X-style `EuvBottomNav`, horizontal swipe routing, live FPS telemetry pill, zero-overshoot drag-to-close / drag-to-expand `EuvDrawer` & `EuvSheet`, unified 1:1 width dropdowns, and exact glyph-advance input carets.
- **17 Compiled Design Schemes & Eva Bold Markdown**: Zero runtime CSS parsing plus [Markdown Syntax & Eva Theme Showcase](./guide/markdown-syntax.md) and [Code Block](./components/code-block.md).
