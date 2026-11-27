---
title: Sheet
order: 51
---

A zero-overshoot bottom sheet (`EuvSheet`) featuring an interactive top floating grabber pill that supports dragging downward to dismiss and dragging upward to expand to an aesthetic maximum height.

## Installation

```shell
cargo add euv euv-ui
```

## Declarative Macro Usage

```rust
let on_close = move |_: &UiEvent| {
    let _: bool = sheet_open.set_if_changed(false);
};
let sheet_body: NodeId = view! {
    column {
        width: Length::Percent(100.0)
        gap: SPACE_MD
        text {
            width: Length::Percent(100.0)
            "Hold the top grabber pill: drag down to close or drag up to expand."
        }
        EuvButton("Close Sheet".to_string(), tokens) {
            outline()
            on_click: on_close
        }
    }
};
view! {
    EuvSheet(sheet_open, "Quick Sheet".to_string(), sheet_body, tokens)
}
```

## Gesture & Animation Behavior

- **Drag Up to Expand**: Smoothly expands panel height up to `min(86% viewport, 760px)` with an internal scroll body.
- **Drag Down to Collapse / Dismiss**: Dragging down from expanded collapses to the compact resting height, and dragging further downward dismisses the sheet smoothly.
- **Zero Overshoot**: Uses strictly monotonic `Easing::EaseOutQuint` / `Easing::EaseOutCubic` transitions.
