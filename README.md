<center>

## euv

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="" height="160">

[![](https://img.shields.io/crates/v/euv.svg)](https://crates.io/crates/euv)
[![](https://img.shields.io/crates/d/euv.svg)](https://crates.io/crates/euv)
[![](https://docs.rs/euv/badge.svg)](https://docs.rs/euv)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/euv.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/euv/latest/)

> A high-performance cross-platform native UI framework for Rust targeting Web, Android, iOS, iPadOS, macOS, Windows, and Linux via direct system APIs, fine-grained reactivity, zero-VDOM macros, and multi-mode rendering.

## Highlights

- **7 Target Platforms via Direct OS APIs (Zero WebView)**:
  - **Web** (`wasm32-unknown-unknown`): DOM Canvas / WebGPU / WebGL2
  - **Android** (`aarch64-linux-android`): Direct NDK `ANativeWindow` + Vulkan / GLES
  - **iOS & iPadOS** (`aarch64-apple-ios`): Direct `libobjc` + UIKit (`UIDevice.userInterfaceIdiom`, SafeArea, Stage Manager, Apple Pencil) + Metal
  - **macOS** (`aarch64-apple-darwin`): Direct `libobjc` + AppKit `NSWindow` + CoreGraphics + Metal
  - **Windows** (`x86_64-pc-windows-msvc`): Direct Win32 `user32`/`gdi32`/`kernel32` + Direct3D 12
  - **Linux** (`x86_64-unknown-linux-gnu`): Runtime `dlopen` Wayland & X11 + Vulkan / OpenGL
- **Fine-Grained Push-Pull Reactivity**: Generation-indexed arena `Signal`, `Memo`, `Effect`, `Selector`, `Scope`, and `batch` (~660 ns diamond updates, ~217 ns O(1) selector switch).
- **Deterministic Cross-Platform Layout & Rendering**: Retained scene graph with O(1) clean-subtree caching (~23 ns cached pass), built-in deterministic glyph atlas, and `RenderMode::{Auto, Software, Hardware, Hybrid}` with bit-identical FNV-1a pixel parity across all platforms.
- **Unified Window Controls & SafeArea**: `WindowConfig`, `WindowState`, `WindowCommand`, `SafeAreaInsets`, DPI scaling, fullscreen, and orientation locks.
- **`euv-ui` Component Library**: 17 zero-CSS compiled design system presets (Light & Dark) and adaptive desktop/tablet/mobile primitives.

## Installation

```shell
cargo add euv euv-ui
```

## Quick Commands

```shell
cargo run -p euv-cli -- doctor
cargo run -p euv-cli -- build --target all --plan
cargo run -p euv-example -- --headless --software
cargo run -p euv-docs -- --headless
cargo run -p euv-performance
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@devhub.vip](mailto:root@devhub.vip).
