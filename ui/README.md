<div align="center">

# Euv-ui

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="Euv-ui logo" height="160">

**60+ Adaptive Native Components, 17 Compiled Themes & Eva Bold Markdown Studio**

[![Crates.io Version](https://flat.badgen.net/crates/v/euv-ui)](https://crates.io/crates/euv-ui)
[![Crates.io Downloads](https://flat.badgen.net/crates/d/euv-ui)](https://crates.io/crates/euv-ui)
[![Docs.rs](https://img.shields.io/docsrs/euv-ui?style=flat-square)](https://docs.rs/euv-ui)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/euv-ui/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Adaptive native Rust UI component library featuring 60+ shadcn/ui-aligned `#[derive(Props)]` components, 17 zero-CSS compiled Light & Dark design schemes, lazy `PageRetention::Freeze` routing with directional dual-layer cross-slide transitions, floating horizontally-centered capsule `EuvBottomNav` + tuned horizontal swipe routing, live native-Hz navbar FPS telemetry, gesture-draggable zero-overshoot `EuvDrawer` & `EuvSheet`, unified 1:1 width dropdowns, and native `EuvMarkdown` + VS Code Eva Light/Dark Bold `EuvCodeBlock`.

## Responsibilities & Module Layout

- `theme` — 17 compile-time Light & Dark design token presets (Shadcn, Cupertino, Material, Fluent, AntDesign, Bootstrap, DaisyUI, Geist, HeroUI, Linear, Mantine, Monochrome, Notion, Park, Radix, Stripe, Supabase)
- `component::layout` & `component::navigation` — `EuvAppShell`, `EuvBottomNav`, `EuvRouter` (`PageRetention::Freeze`), horizontal swipe routing, and live FPS pill
- `component::overlay_extra` & `component::form_extra` — Zero-overshoot gesture-draggable `EuvDrawer` & `EuvSheet` and unified 1:1 width dropdowns (`EuvDropdown`, `EuvSelect`, `EuvCombobox`, `EuvNativeSelect`, `EuvDatePicker`)
- `component::markdown` — Full GFM `EuvMarkdown` and VS Code `Eva Light Bold` (`#EBEEF5`) / `Eva Dark Bold` (`#282C34`) `EuvCodeBlock` using native monospace/italic glyphs

## Installation & Usage

```sh
cargo add euv euv-ui
```

## Verification

```sh
cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-ui --all-features --all-targets -- -D warnings
cargo test -p euv-ui --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, macro, component, CLI, or module-boundary change per [`../AGENTS.md`](../AGENTS.md#4-unified-documentation--skill-maintenance-policy).
- Project standards and workflows live in [`../.agents/skills/euv/SKILL.md`](../.agents/skills/euv/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
