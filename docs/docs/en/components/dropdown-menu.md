---
title: Dropdown Menu
order: 25
---

A full-width control dropdown menu (`EuvDropdown`) whose trigger matches standard `EuvSelect` / `EuvInput` control height, padding, border, and vector chevron, and whose popup list aligns 1:1 with the trigger width.

## Installation

```shell
cargo add euv euv-ui
```

## Declarative Macro Usage

```rust
view! {
    EuvDropdown {
        trigger: "Quick Actions"
        items: vec!["Export PNG".to_string(), "Copy DeepLink".to_string()]
        tokens: tokens
        open: dropdown_open
        selected: dropdown_selected
    }
}
```

## Layout Guarantees

- **Unified Control Spec**: Trigger uses `height: g.get_control_md()`, `padding: Edges::symmetric(0.0, SPACE_MD)`, `background: p.get_background()`, `radius: g.get_radius_md()`, and crisp vector chevron parity with `EuvSelect`, `EuvCombobox`, `EuvNativeSelect`, and `EuvDatePicker`.
- **1:1 Popup Width Parity**: Both the trigger bar and the dropdown menu span `width: Length::Percent(100.0)` inside their shared container so the list width always matches the trigger width exactly.
