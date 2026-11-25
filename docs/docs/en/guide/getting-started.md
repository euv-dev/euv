---
title: Getting Started
order: 1
section: guide
tagline: Install euv, build a reactive native UI, and target all 7 platforms from one Rust codebase.
---

## Install

```shell
cargo add euv euv-ui
```

## Minimal Cross-Platform App

```rust
use euv::*;
use euv_ui::*;

pub fn run() -> Result<(), PlatformError> {
    let config = AppConfig::default().with_window(
        WindowConfig::default()
            .with_title("Hello euv".to_string())
            .with_size(1024, 720)
            .with_render(RenderConfig::default()),
    );
    let mut app = App::new(config)?;
    let count = Signal::new(0);
    let tokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    app.mount(|ctx| {
        EuvButton::create("Increment".to_string(), tokens)
            .on_click(move |_| {
                count.update(|v| *v += 1);
            })
            .mount(ctx)
    });
    let _ = app.step_frame()?;
    Ok(())
}
```
