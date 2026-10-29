<center>

## euv-ui

<img src="https://docs.ltpp.vip/img/euv-ui.png" alt="" height="160">

[![](https://img.shields.io/crates/v/euv-ui.svg)](https://crates.io/crates/euv-ui)
[![](https://img.shields.io/crates/d/euv-ui.svg)](https://img.shields.io/crates/d/euv-ui.svg)
[![](https://docs.rs/euv-ui/badge.svg)](https://docs.rs/euv-ui)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/euv-ui.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/euv-ui/latest/)

> Reusable UI component library for the euv framework, providing buttons, cards, modals, inputs, and more.

## Installation

To use this crate, you can run cmd:

```shell
cargo add euv-ui
```

## UI schemes

The library ships thirteen complete UI schemes. A scheme is a design language —
its colours, corner radii, type ramp, control heights and shadow ramp — and the
developer picks one at build time with a Cargo feature. Every scheme defines
the same token set, so no component falls back to a browser default when one is
selected.

| Feature | Design language |
| --- | --- |
| *(none)* | shadcn/ui — the reference: a hairline-bordered card, an 8px control, a 36px button |
| `theme-shadcn` | shadcn/ui, selected explicitly |
| `theme-monochrome` | The built-in black-and-white system — square corners, one ink for every role |
| `theme-antdesign` | Ant Design — a blue brand, 6px controls, 14px type |
| `theme-material` | Material Design 3 — violet tonal palette, 4dp-16dp shapes |
| `theme-bootstrap` | Bootstrap 5.3 — 6px controls, 16px type, a 38px control height |
| `theme-cupertino` | Apple Human Interface Guidelines — 44pt targets, translucent fills |
| `theme-fluent` | Microsoft Fluent 2 — Segoe UI Variable, a 4px control radius |
| `theme-radix` | Radix Themes — a twelve-step accent ramp, a 4px control |
| `theme-mantine` | Mantine — a teal brand, an 8px control, a 36px button |
| `theme-daisyui` | daisyUI — rounded fields, a bright brand, its own light and dark themes |
| `theme-heroui` | HeroUI — gradient accents, a dark-first palette |
| `theme-park` | Park UI — a zinc ramp and a soft, low-contrast surface |
| `theme-geist` | Geist UI — the Vercel scale, near-black neutrals |
| `theme-custom` | Bring your own styles — the built-in class registry ships empty, so the app is yours to paint |

```toml
[dependencies]
euv-ui = { version = "0.33", features = ["theme-antdesign"] }
```

Enable at most one scheme feature. The light and dark modes stay a runtime
toggle, so a scheme ships both halves.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@devhub.vip](mailto:root@devhub.vip).
