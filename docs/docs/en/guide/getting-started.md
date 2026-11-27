---
title: Getting Started
order: 1
section: guide
tagline: Install euv, compose zero-context reactive UIs with view! and newline-separated style!, and target all 7 platforms from one Rust codebase.
---

## Install

```shell
cargo add euv euv-ui
```

## Minimal Cross-Platform App

Define event handlers and reactive text closures as sibling `let` bindings (or outer helper functions) before `view!`, and place each rule inside `style!` on its own line without semicolons:

```rust
use {euv::*, euv_ui::*};

pub fn run() -> Result<(), PlatformError> {
    let mut window: WindowConfig = WindowConfig::default();
    window
        .set_title("Hello euv".to_string())
        .set_size(1024, 720)
        .set_render(RenderConfig::default());
    let mut config: AppConfig = AppConfig::default();
    config.set_window(window);
    let mut app: App = App::new(config)?;
    let count: Signal<i32> = Signal::new(0);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let _: NodeId = app.mount(move || {
        let p: ThemePalette = tokens.get_palette();
        let container_style: Style = style! {
            FlexDirection::Column
            Overflow::Hidden
            width: Length::Percent(100.0)
            padding: Edges::all(SPACE_LG)
            gap: SPACE_MD
            background: p.get_background()
        };
        let count_text = move || -> String { format!("Count: {}", count.get()) };
        let on_increment = move |_: &UiEvent| {
            let _: bool = count.update(|v: &mut i32| *v += 1);
        };
        view! {
            view {
                style: container_style
                text {
                    width: Length::Percent(100.0)
                    font_size: 18.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_foreground()
                    { count_text }
                }
                EuvButton {
                    label: "Increment"
                    tokens: tokens
                    on_click: on_increment
                }
            }
        }
    });
    let _: FrameStats = app.run_frames(Some(1))?;
    Ok(())
}
```

## IDE Go-to-Definition & Formatting

- **`Cmd+Click` / `F12` in `view!` & `style!`**: Built-in element tags (`column`, `row`, `text`, `input`, `canvas`, etc.) and properties jump directly to their schema docs in `euv::view_elements` / `euv::style_props`. Custom component names (`EuvButton`, `EuvCard`, etc.) jump directly to the component struct definition.
- **Formatting**: Run `cargo run -p euv-cli -- fmt` followed by `cargo fmt --all` to normalize both Rust code and all `view!` / `style!` / `tokens!` macro blocks idempotently.
