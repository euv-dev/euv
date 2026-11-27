---
title: Architecture & 7-Platform Backends
order: 2
section: guide
tagline: Direct OS API bindings, multi-mode rendering, and unified window abstractions.
---

## Unified Architecture

Every platform compiles the same `euv-core` runtime, layout engine, glyph rasterizer, and `DisplayList` paint pass:

- Web (`wasm32-unknown-unknown`): DOM Canvas 2D / WebGPU / WebGL2 via `web-sys`.
- macOS (`aarch64-apple-darwin`): Direct `libobjc` + AppKit `NSWindow` + CoreGraphics + Metal (`CAMetalLayer`).
- iOS and iPadOS (`aarch64-apple-ios`): Direct `libobjc` + UIKit (`UIDevice.userInterfaceIdiom` detection, SafeArea, Apple Pencil, Stage Manager) + Metal.
- Android (`aarch64-linux-android`): Direct NDK `ANativeWindow` surface locking + Vulkan / GLES.
- Windows (`x86_64-pc-windows-msvc`): Direct Win32 `user32`/`gdi32`/`kernel32` + Direct3D 12 HLSL shaders.
- Linux (`x86_64-unknown-linux-gnu`): Runtime `dlopen` Wayland and X11 bindings + Vulkan / OpenGL.

## Render Modes

Switch rendering backends at initialization or runtime with `app.set_render_mode(...)`:

- `RenderMode::Auto`: Selects hardware GPU when available and falls back seamlessly to software CPU rasterization.
- `RenderMode::Software`: Analytical SDF tile rasterizer with dirty-rectangle damage tracking, padding-box child clipping, exact `GlyphAtlas` caret alignment (`DrawCommand::TextCaret`), and deterministic pixel hashing.
- `RenderMode::Hardware`: Instanced quad/glyph/border batching emitting native WGSL, MSL, GLSL, or HLSL shaders.
- `RenderMode::Hybrid`: CPU vector rasterizer composited with GPU texture upload batches.

## Declarative Macros & IDE Navigation

`euv-macros` compiles `view!`, `style!`, `tokens!`, `watch!`, `computed!`, and `event!` directly to static Rust without a virtual DOM:

- **Named Component Props & Go-to-Definition Schemas**: Primitive `view!` tags/properties resolve to `euv::view_elements` and `style!` keys resolve to `euv::style_props`. Custom components derive `Props` and pass parameters as self-documenting named attributes (`EuvButton { label: "Save" tokens: tokens }`), expanding as `<ComponentPath as ::euv::ComponentProps>::init_props()` so IDE jump-to-definition lands directly on the component struct and each field declaration.
- **Clean `style!` Syntax**: One style rule per line with zero trailing semicolons; the compiler rejects stray `;` tokens or multiple rules packed on a single line.
- **Sibling Closure Discipline**: Keep multi-statement handlers and reactive text builders in sibling `let` bindings or outer helper `fn`s, then pass identifiers into `view!` (`text { { summary_text } }`, `on_click: on_save`).

## Route Lifecycle, Swipes & Interactive Overlays

- **Active / Frozen / Destroyed Route Lifecycle**: Routes only execute reactive work while active; inactive routes freeze or destroy cleanly based on their lifecycle policy, with cross-route state updates safely propagated via `GlobalState` and `event!`.
- **Twitter/X-Style Bottom Navigation & Horizontal Swipe Routing**: `EuvAppShell` pairs a persistent `EuvBottomNav`, live navbar FPS readout, and full-surface horizontal swipe routing across routes.
- **Zero-Overshoot Bidirectional Drag Drawers & Sheets**: `EuvAppShell` left drawers, `EuvDrawer` right drawers, and `EuvSheet` bottom sheets animate with strictly monotonic easing (`EaseOutQuint` / `EaseOutCubic`) and include floating grabber handles supporting drag-to-close and drag-to-expand.
