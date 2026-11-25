---
home: true
title: euv
order: 0
tagline: High-performance cross-platform native Rust UI framework targeting Web, Android, iOS, iPadOS, macOS, Windows, and Linux via direct system APIs.
---

## Why euv 0.38

euv 0.38 replaces Web-only DOM diffing with a unified native architecture:

- Direct OS API adapters across Web, Android, iOS, iPadOS, macOS, Windows, and Linux with zero WebView overhead.
- Fine-grained push-pull reactivity (`Signal`, `Memo`, `Effect`, `Selector`, `Scope`, `batch`) patching scene nodes in O(1).
- Deterministic Flexbox, Grid, Absolute, Fixed, Scroll, and SafeArea layout with O(1) clean-subtree caching.
- Complete multi-mode rendering (`Auto`, `Software`, `Hardware`, `Hybrid`) guaranteeing cross-platform pixel parity.
- 17 compiled design system presets in `euv-ui` with zero runtime CSS parsing.
