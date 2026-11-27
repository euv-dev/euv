<div align="center">

# euv-example

<img src="https://docs.ltpp.vip/img/euv-example.png" alt="euv-example logo" height="160">

[![Crates.io Version](https://img.shields.io/crates/v/euv-example.svg?style=flat-square)](https://crates.io/crates/euv-example)
[![Crates.io Downloads](https://img.shields.io/crates/d/euv-example.svg?style=flat-square)](https://crates.io/crates/euv-example)
[![Docs.rs](https://img.shields.io/docsrs/euv-example?style=flat-square)](https://docs.rs/euv-example)
[![CI Status](https://img.shields.io/github/actions/workflow/status/euv-dev/euv/Rust.yml?style=flat-square&label=CI)](https://github.com/euv-dev/euv/actions)
[![License: MIT](https://img.shields.io/crates/l/euv-example.svg?style=flat-square)](../LICENSE)

[API Docs](https://docs.rs/euv-example/latest/) · [Workspace README](../README.md)

</div>

> Full 15-route cross-platform reference showcase for `euv` + `euv-ui`, featuring Twitter/X-style bottom navigation, home module directory, horizontal swipe routing, live navbar FPS telemetry, gesture-draggable drawers & sheets, VS Code Eva Light/Dark Bold Markdown & Code Studio, 60 FPS 3D ray-traced sandbox, pure-Rust Media Studio, Dynamic WASM OTA, and an interactive 10-scenario Stress Lab.

## Running Locally

```shell
cargo run -p euv-example                     # Native desktop window
cargo run -p euv-example -- --headless       # Headless deterministic verification
../scripts/build/build-web.sh                # Zero-JS WebAssembly bundle
../scripts/build/build-theme-variants.sh     # All 17 theme-feature WebAssembly variants
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
