---
title: Drawer
order: 24
---

A zero-overshoot side drawer (`EuvDrawer`) with an interactive vertical grabber pill supporting drag-to-close (drag toward edge) and drag-to-expand (drag inward up to an aesthetic maximum width).

## Installation

```shell
cargo add euv euv-ui
```

## Declarative Macro Usage

```rust
let on_close = move |_: &UiEvent| {
    let _: bool = drawer_open.set_if_changed(false);
};
let drawer_body: NodeId = view! {
    column {
        width: Length::Percent(100.0)
        gap: SPACE_MD
        text {
            width: Length::Percent(100.0)
            "Drag the left grabber bar inward to expand or outward to dismiss."
        }
        EuvButton {
            label: "Close Drawer"
            tokens: tokens
            outline()
            on_click: on_close
        }
    }
};
view! {
    EuvDrawer {
        open: drawer_open
        title: "Inspector Drawer"
        body: drawer_body
        tokens: tokens
    }
}
```

## Gesture & Animation Behavior

- **Monotonic Zero-Overshoot Easing**: Opens with `Easing::EaseOutQuint` and closes with `Easing::EaseOutCubic` — never leaves an edge gap or bounces back.
- **Interactive Floating Grabber**: Dragging inward expands the panel smoothly to `min(92% viewport, 560px)`; dragging outward collapses or dismisses the drawer.
- **System Back Stack**: Automatically registers with `EuvOverlayStack` so hardware/OS Back dismisses the drawer before popping routes.
