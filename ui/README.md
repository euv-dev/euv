<div align="center">

# euv-ui

<img src="https://docs.ltpp.vip/img/euv-ui.png" alt="euv-ui logo" height="160">

[![Crates.io Version](https://img.shields.io/crates/v/euv-ui.svg?style=flat-square)](https://crates.io/crates/euv-ui)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-ui.svg?style=flat-square)](https://crates.io/crates/euv-ui)
[![Docs.rs](https://img.shields.io/docsrs/euv-ui?style=flat-square)](https://docs.rs/euv-ui)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/Rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![License: MIT](https://img.shields.io/crates/l/euv-ui.svg?style=flat-square)](../LICENSE)

[API Docs](https://docs.rs/euv-ui/latest/) · [Component Catalog](../docs/docs/en/components/README.md) · [Workspace README](../README.md)

</div>

> Adaptive native Rust UI component library featuring 60+ shadcn/ui-aligned components, 17 zero-CSS compiled Light & Dark design schemes, lazy `PageRetention::Freeze` routing, Twitter/X-style `EuvBottomNav` + swipe routing, live navbar FPS telemetry, gesture-draggable zero-overshoot `EuvDrawer` & `EuvSheet`, unified 1:1 width dropdowns, and native `EuvMarkdown` + VS Code Eva Light/Dark Bold `EuvCodeBlock`.

## Highlights

- **17 Compiled Design Schemes**: Shadcn, Cupertino, Material, Fluent, AntDesign, Bootstrap, DaisyUI, Geist, HeroUI, Linear, Mantine, Monochrome, Notion, Park, Radix, Stripe, and Supabase — both Light & Dark modes.
- **Responsive Shell & Gestures**: `EuvAppShell` supports persistent `EuvBottomNav`, horizontal left/right swipe route switching, live real-time FPS telemetry pill, and interactive grabber-handle drag-to-close / drag-to-expand on left/right drawers and bottom sheets.
- **Markdown & VS Code Eva Bold Studio**: `EuvCodeBlock` and `EuvMarkdown` render exact VS Code `Eva Light Bold` (`#EBEEF5`) and `Eva Dark Bold` (`#282C34`) syntax tokens using native monospace/italic glyphs with preserved indentation and zero WebView.

## Installation

```shell
cargo add euv euv-ui
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
