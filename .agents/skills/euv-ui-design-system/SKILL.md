---
name: euv-ui-design-system
description: 'Use when authoring euv-ui design tokens, 17 Light/Dark themes, layout grids, chrome, auto-fill rows, SafeArea/fullscreen, overlays, or rich components (EuvMarkdown, EuvEChart). Excludes mockup/video restoration.'
license: MIT
metadata:
  hermes:
    related_skills: [euv, euv-design-restoration, euv-ui-standards, rust-standards]
---

# Euv-ui Design System & Component Specification

Authoritative token, theme, geometry, layout, chrome, fullscreen, and component architecture standard for `euv-ui` across Web, Android, iOS, iPadOS, macOS, Windows, and Linux. (For pixel/keyframe restoration from mockups, images, or videos, use [`euv-design-restoration`](../euv-design-restoration/SKILL.md).)

## 1. Spatial Grid, Breakpoints & `EuvAppShell`

All surfaces snap to the **4px spatial grid** (`ui/src/theme/token/const.rs`):

| Token                  |   Value | Usage                                                                     |
| :--------------------- | ------: | :------------------------------------------------------------------------ |
| `SPACE_XS` / `SM`      |   `4/8` | Inline icon/label gaps (`4px`); action-row & form-control gaps (`8px`)    |
| `SPACE_MD` / `LG`      | `12/16` | Control horizontal padding & card gaps (`12px`); card/section pad (`16`)  |
| `SPACE_XL` / `2XL`     | `20/24` | Desktop canvas/overlay padding (`20px`); spacious empty states (`24px`)   |
| `MOBILE_HEADER_HEIGHT` |  `52px` | Compact top bar height (`52px + safe.top`); desktop bar matches `52px`    |
| `NAV_SIDEBAR_WIDTH`    | `240px` | Expanded sidebar width (`204px` on Medium rail); `CONTENT_MAX_WIDTH: 960` |

- **Breakpoints (`Breakpoint::from_width`)**: `Compact` (`< 768px`, top bar + `224..256px` left slide drawer expandable to `380px`, `SPACE_MD` canvas pad, 1-col KPI grid), `Medium` (`768px..1023px`, `204px` rail, `SPACE_LG` pad, 2-col KPI), `Expanded` (`>= 1024px`, `240px` rail, `SPACE_XL` pad, 3-col KPI).
- **Shell Chrome & Brand Identity**: `EuvAppShell` renders the unified `26x26px` `Euv Studio` Emperor-Armor Winged Golden Dragon vector emblem (imperial gold, obsidian chassis, wide solar-gold wings, crimson ruby core & wingtips) across navbar, sidebar, and drawer, optional centered floating capsule `EuvBottomNav` (`60px` high, `max-width: 436px`, float `14px` compact / `18px` wide + `safe.bottom`), optional live FPS pill (`show_fps`), and zero-teardown `UiThemeState` in-place restyling (`remap_subtree_theme_in_place`).
- **Reactive `SafeAreaInsets`**: Bind `EuvAppShell` (`page_padding`), `EuvRouterOutlet` double-back exit toast, `EuvDrawer`, `EuvSheet`, `EuvToast`, and `EuvNotification` reactively to `Context::get_safe_area()` (`Signal<SafeAreaInsets>`) so notches, Dynamic/Atomic Island pills, and OS bars reflow immediately.

## 2. Multi-Theme Tokens (`ThemePalette` & `ThemeGeometry`)

**Zero hardcoded color literals, control heights, border widths, or corner radii in business pages.** Resolve tokens via O(1) compile-time LUTs (`ThemeTokens::resolve(scheme, mode)`) and ambient `provide_theme` / `use_live_tokens` / `live_tokens_untracked`.

- **17 `DesignScheme` x 2 `ThemeMode` (`Light` / `Dark`)**: `Shadcn`, `Cupertino`, `Material`, `Fluent`, `AntDesign`, `Bootstrap`, `DaisyUi`, `Geist`, `HeroUi`, `Linear`, `Mantine`, `Monochrome`, `Notion`, `Park`, `Radix`, `Stripe`, `Supabase`.
- **`ThemePalette` Semantic Roles**: Surfaces (`background`, `card`, `muted`, `media_surface`, `overlay`), Ink (`foreground`, `card_foreground`, `muted_foreground`), Brand/Interactive (`primary`/`primary_foreground`, `secondary`/`secondary_foreground`, `accent`), Status (`destructive`, `success`, `warning` + `*_foreground`), Controls (`switch_track_off`, `switch_thumb`), Chrome (`border` `1px`, `ring` `2px`).
- **`ThemeGeometry` Unified 3-Tier Control Ramp & Typography**:
  - Heights: `control_sm` (`max(control_md - 6, 28)px`, `12px` text, `SPACE_SM` x-pad), `control_md` (`34..40px`, `36px` Shadcn, `13px` text, `SPACE_MD` x-pad), `control_lg` (`control_md + 8px`, `14px` text, `SPACE_XL` x-pad); `EuvButtonSize::Icon` is square `control_md x control_md`.
  - Type scale: `font_title` (`24px` Bold), `font_heading` (`16px` SemiBold), `font_body` (`14px` Regular), `13px` Medium controls/labels, `12px` muted captions, `11px` SemiBold badges/kbd.
  - Shadows: `shadow_xs` (controls/pills), `shadow_sm` (cards), `shadow_md` (popovers/dropdowns/toasts), `shadow_lg` (modals/drawers/sheets).
- **Disciplined Corner Radii**: `radius_sm` (`max(radius_md * 0.65, 2px)` — checkboxes, tags, kbd, segmented/tab pills), `radius_md` (buttons, inputs, dropdowns, popovers), `radius_lg` (cards, alerts, empty states), `radius_xl` (`radius_lg + 4px` — modals, sheets), full pill (switches, avatars, badges, radios, slider thumbs).

## 3. Declarative Composition, Chrome & Layout Invariants

1. **Macro & Props Discipline (`AGENTS.md` §2)**: Compose `euv-ui` components inside `view!` exclusively with named `#[derive(Props)]` attributes (`Component { prop: value }`); positional `Component(...)` calls are rejected at compile time. Bind event handlers, painters, and reactive closures to sibling `let` bindings or helper `fn`s before `view!`. Keep `style! { ... }` to one rule per line with zero semicolons.

  ```rust
  let on_save = move |_: &UiEvent| {
      let _: bool = saved.set_if_changed(true);
  };
  let save_btn: NodeId = view! {
      EuvButton {
          label: "Save".to_string()
          tokens: tokens
          fill: true
          on_click: on_save
      }
  };
  ```
2. **Compact Icon-Only Chrome**: Prefer crisp native vector `EuvIcon` glyphs via `EuvIconName` (`Copy`, `Check`, `Maximize`, `Minimize`, `Close`, `Search`, `Sun`, `Moon`, `Palette`, `Menu`, `Play`, `Pause`, `Refresh`, `Trash`, `ChevronDown`, `ArrowLeft`, `ArrowRight`) with explicit `a11y_label` and `test_id:` hints over verbose toolbar text. Keep navbar titles to 1–2 concise words with `TextOverflow::Ellipsis`.
3. **Auto-Fill Wrapped Button Rows**: Use `EuvActionRow` (sets `FlexWrap::Wrap`, `AlignItems::Center`, `SPACE_SM` gap, `flex_grow: 1.0`, and content-safe `min_width` clamped `76..210px` or `136px` for switches/checkboxes), `EuvButton { fill: true, min_width: ... }`, or `EuvButtonGroup { wrap_fill: true }` so every wrapped row fills 100% width with zero trailing empty gap. Allow at most one `EuvButtonVariant::Primary` per action group.
4. **Unified 1:1 Dropdowns**: `EuvDropdown`, `EuvSelect`, `EuvCombobox`, `EuvNativeSelect`, and `EuvDatePicker` MUST use the full-width `control_md` trigger (`Edges::symmetric(0.0, SPACE_MD)`, `p.get_background()`, vector `ChevronDown`) and match popup list width 1:1 to the trigger width.
5. **Zero Card Text Overflow**: All card, bubble, and alert containers (`EuvCard`, `EuvStatCard`, `ThemeTokens::card_style`) MUST set `overflow: Overflow::Hidden`, and wrapped `text` nodes inside cards MUST specify `width: Length::Percent(100.0)` (or `flex_shrink: 1.0` + `min_width: Length::Px(0.0)` inside rows) from `360px` mobile to `4K`.
6. **Zero-Boilerplate Page Scaffolding**: Compose screens with `EuvPage` (automatic `EuvHeader`, `max-width: 1040px`, `SPACE_LG` rhythm), `EuvStatGrid` (reactive 1/2/3-col KPI grid), `EuvCard`, and `EuvRouter` / `EuvRouterOutlet` (`PageRetention::Freeze`, `DOUBLE_BACK_EXIT_PROMPT`).

## 4. Z-Index Ladder, Edge-to-Edge Fullscreen & Overlay Motion

- **Z-Index Hierarchy**: `Z_INDEX_CONTENT` (`1`) < `Z_INDEX_CHROME` (`100`) < `Z_INDEX_TOOLTIP` (`140`) < `Z_INDEX_FLOATING` (`150`) < `Z_INDEX_OVERLAY` (`200`) < `Z_INDEX_DRAWER` (`201`) < `Z_INDEX_MODAL` (`1000`) < `Z_INDEX_TOAST` (`1100`) < `Z_INDEX_FULLSCREEN` (`1200`).
- **True Edge-to-Edge Fullscreen**: Fullscreen portals (`EuvCodeBlock`, `EuvMermaid`, game canvases) mount at `Z_INDEX_FULLSCREEN` (`1200`) with `0` outer margin, padding, and radius. Pair refcounted `enter_fullscreen()` / `exit_fullscreen()` (synced to `Context::enter_immersive_fullscreen` via `bind_fullscreen_context` and `use_fullscreen_active()`) so `EuvAppShell` chrome hides and restores cleanly.
- **Zero-Overshoot Overlays & Motion**: Animate transform/opacity only so layout is never recomputed mid-flight:
  - `EuvDrawer`, `EuvAppShell` mobile drawer, and `EuvSheet`: `220ms EaseOutCubic` scrim fade + `260ms EaseOutQuint` panel slide (strictly monotonic, never overshooting springs) with interactive floating grabber handles (drag-to-close / drag-to-expand).
  - `EuvRouterOutlet` coupled route slide: `260ms EaseOutCubic` (`220ms` cancel snap-back); `EuvBottomNav` active pill: `240ms EaseOutCubic` (`200ms` cancel).
  - `EuvModal`: `220ms EaseOutQuint` scrim + `NodeTransition::modal_enter`.
- **Zero-Stuck Overlay Dismissal**: Register every modal, drawer, sheet, popover, dropdown, and fullscreen portal in `EuvOverlayStack` (LIFO Back/Escape dismissal). On close, `on_pointer_cancel`, `on_cleanup`, or route change: cancel active transitions, immediately apply `Display::None` + `PointerEvents::None`, and clear descendant hover/drag state.

## 5. Rich Content Fidelity & Single-Owner Scroll Containment

- **`EuvCodeBlock` & `EuvMarkdown`**: Render GFM (`EuvMarkdown`: alerts, math, tables, `<details>`, footnotes, checklists) and native monospace `EuvCodeBlock` with `TextOverflow::Clip` (preserving leading indentation), VS Code `Eva Light Bold` (`#EBEEF5`) / `Eva Dark Bold` (`#282C34`) `CodeThemePalette`, H+V scrolling, copy action, and edge-to-edge fullscreen.
- **`EuvMermaid` & `EuvEChart`**: `EuvMermaid` renders native vector diagrams (flowchart, sequence, state, class, ER, pie, Gantt) with H+V scrolling and fullscreen. `EuvEChart` renders Apache ECharts via `EuvChartOption` across 20+ series (`EuvChartSeriesData`: `Line`, `Bar`, `Pie`, `Scatter`, `EffectScatter`, `Radar`, `Candlestick`, `Boxplot`, `Heatmap`, `Treemap`, `Sunburst`, `Sankey`, `Funnel`, `Gauge`, `Graph`, `Tree`, `Parallel`, `Lines`, `ThemeRiver`, `PictorialBar`, `Custom`, `GeoMap`) with cached vector `DisplayList`, tooltips, legends, DataZoom, VisualMap, and compact icon Toolbox (`EuvChart` covers lightweight Bar/Line/Area).
- **GestureArena Single-Owner Scroll Containment**: Honor exclusive `ActiveGesture` arbitration (`10dp` touch slop, `1.25x` axis hysteresis across `VerticalScroll`, `HorizontalScroll`, `RouteSwipe`, `DirectDrag`, `ScrollbarDrag`, `TextSelection`, `Pinch`). Horizontal scrollables (`EuvCodeBlock`, `EuvMarkdownTable`, `EuvMermaid`, `EuvTable`, `EuvTabs`, `EuvScrollArea`) isolate horizontal deltas without leaking into `EuvAppShell`/`EuvRouterOutlet` swipes or outer vertical scrolls.

## 6. Verification & Related Skills

```sh
cargo run -p euv-cli -- fmt && cargo run -p euv-cli -- fmt && cargo run -p euv-cli -- fmt --check
cargo clippy -p euv-ui --no-default-features --all-targets -- -D warnings
cargo clippy -p euv-ui --all-features --all-targets -- -D warnings
cargo test -p euv-ui --no-default-features && cargo test -p euv-ui --features theme-shadcn && cargo test -p euv-ui --all-features
```

- Companion skills: [Euv Core](../euv/SKILL.md) · [Design Restoration](../euv-design-restoration/SKILL.md) · [AI Automation](../euv-ai-automation/SKILL.md) · [E2E Replay](../euv-e2e-replay/SKILL.md) · [AGENTS Governance](../../../AGENTS.md)
