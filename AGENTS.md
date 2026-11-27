# AGENTS.md — euv Workspace Engineering Guidelines

This file defines mandatory architectural, macro authoring, formatting, and quality rules for all contributors and AI coding agents working in the `euv` repository.

## 1. Declarative Macro Authoring Rules (`view!`, `style!`, `watch!`, `computed!`)

### 1.1 Never Embed Full Logic or Multi-Line Closures Directly Inside Macros

- **Do NOT write business logic, multi-statement event handlers, canvas painters, or reactive text/children builder closures inline inside `view! { ... }` or other `euv` macros.**
- **Instead**: define the handler or reactive closure as a sibling `let` binding immediately before the macro invocation (or as a standalone outer helper `fn`), and pass only the identifier into the macro.
- **Bad**:

  ```rust
  view! {
      EuvButton(BTN_SAVE.to_string(), tokens) {
          on_click: move |_: &UiEvent| {
              let next: String = draft.get_untracked().trim().to_string();
              if !next.is_empty() {
                  let _: bool = saved.set_if_changed(next);
              }
          }
      }
      text {
          { move || {
              let entries: Vec<EuvRouteLifecycleEntry> = entries_sig.get();
              format!("Cached: {}", entries.len())
          } }
      }
  }
  ```
- **Good**:

  ```rust
  let on_save = move |_: &UiEvent| {
      let next: String = draft.get_untracked().trim().to_string();
      if !next.is_empty() {
          let _: bool = saved.set_if_changed(next);
      }
  };
  let summary_text = move || -> String {
      let entries: Vec<EuvRouteLifecycleEntry> = entries_sig.get();
      format!("Cached: {}", entries.len())
  };
  view! {
      EuvButton(BTN_SAVE.to_string(), tokens) {
          on_click: on_save
      }
      text {
          { summary_text }
      }
  }
  ```

### 1.2 `style!` Macro Syntax: One Rule Per Line, No Semicolons

- Every statement inside `style! { ... }` must sit on its own line **without** trailing semicolons `;`.
- Placing two rules on the same line or adding trailing `;` is a compile-time error.
- **Good**:

  ```rust
  let card_style: Style = style! {
      FlexDirection::Column
      AlignItems::Center
      Overflow::Hidden
      width: Length::Percent(100.0)
      padding: Edges::all(SPACE_LG)
  };
  ```

### 1.3 IDE Go-to-Definition Support in Macros

- Built-in `view!` tags (`view`, `row`, `column`, `col`, `grid`, `scroll`, `portal`, `fragment`, `text`, `input`, `image`, `canvas`, `slot`, `page_slot`) and properties (`width`, `padding`, `on_click`, `value`, `painter`, etc.) resolve directly to their zero-cost schema definitions in `euv::view_elements` so `Cmd+Click` / `F12` jumps straight to all supported properties and documentation.
- `style!` property keys resolve directly to `euv::style_props`.
- Custom component invocations (`EuvButton(...)`, `EuvCard(...)`, etc.) preserve the type path span in constructor type position (`<#path>::new(...)`) and keep `::euv::Component::mount(...)` on `Span::call_site()` so `Cmd+Click` jumps directly to the component struct definition rather than the `Component` trait.

## 2. UI Component & Layout Standards

- **Zero-Overshoot Overlays & Gesture Drawers**: Drawers (`EuvDrawer`, `EuvAppShell` left drawer) and sheets (`EuvSheet`) must use strictly monotonic easing (`Easing::EaseOutQuint` / `Easing::EaseOutCubic`, never overshooting springs) and provide interactive floating grabber handles supporting drag-to-close and drag-to-expand up to an aesthetic maximum dimension.
- **Unified Dropdown & Trigger Widths**: Dropdowns (`EuvDropdown`, `EuvSelect`, `EuvCombobox`, `EuvNativeSelect`, `EuvDatePicker`) must use the unified full-width control trigger (`height: g.get_control_md()`, `padding: Edges::symmetric(0.0, SPACE_MD)`, `background: p.get_background()`, crisp vector chevron) and match popup list width 1:1 with the trigger width.
- **No Card Text Overflow**: All card/bubble/alert containers must set `overflow: Overflow::Hidden`, and wrapped `text` nodes inside cards must explicitly specify `width: Length::Percent(100.0)` (or `flex_shrink: 1.0` + `min_width: Length::Px(0.0)` inside rows) so text wraps cleanly across all viewport widths (`360px` to `4K`).
- **Strict Code & Theme Fidelity**: Code blocks (`EuvCodeBlock`) use real native monospace glyphs, preserve leading indentation via `TextOverflow::Clip`, and align strictly to VS Code `Eva Light Bold` / `Eva Dark Bold` palettes without unrequested gray line highlights.

## 3. Formatting, Linting & Quality Gates

- Always run `euv fmt` (`cargo run -p euv-cli -- fmt`) followed by `cargo fmt --all` twice and verify idempotency (`euv fmt --check` and `cargo fmt --all -- --check`).
- Enforce zero warnings with `cargo clippy --all-features --workspace --all-targets -- -D warnings`.
- Follow strict Rust module purity (`mod.rs`, `const.rs`, `enum.rs`, `struct.rs`, `trait.rs`, `impl.rs`, `fn.rs`, `type.rs`, `static.rs`), explicit types on `let` bindings and closure parameters, zero `#[allow(...)]`, zero `self.field` direct accesses, and zero `#[inline]` in WASM `cdylib` crates.
