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
- `RenderMode::Software`: Analytical SDF tile rasterizer with dirty-rectangle damage tracking and deterministic pixel hashing.
- `RenderMode::Hardware`: Instanced quad/glyph/border batching emitting native WGSL, MSL, GLSL, or HLSL shaders.
- `RenderMode::Hybrid`: CPU vector rasterizer composited with GPU texture upload batches.
