---
title: Getting Started
order: 1
section: guide
tagline: Install euv, build a reactive native UI with zero-ctx macros, and target all 7 platforms from one Rust codebase.
---

## Install

```shell
cargo add euv euv-ui
```

## Minimal Cross-Platform App

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
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_LG)
                gap: SPACE_MD
                text {
                    font_size: 18.0
                    { move || format!("Count: {}", count.get()) }
                }
                { EuvButton::new("Increment".to_string(), tokens).on_click(move |_: &UiEvent| {
                    let _: bool = count.update(|v: &mut i32| *v += 1);
                }) }
            }
        }
    });
    let _: FrameStats = app.run_frames(Some(1))?;
    Ok(())
}
```
