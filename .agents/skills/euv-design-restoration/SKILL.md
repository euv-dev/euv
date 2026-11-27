---
name: euv-design-restoration
description: 'Use when restoring UI design mockups, static screenshots/images, or animated videos/recordings into native Rust euv and euv-ui apps with pixel, motion, and latency QA. Excludes headless AI bridge and E2E replay.'
license: MIT
metadata:
  hermes:
    related_skills: [euv, euv-ui-standards, euv-ai-automation, euv-e2e-replay, rust-standards]
---

# euv UI design, image, and video restoration

## Part 1: Static Design & Image Restoration (Figma, Sketch, Wireframes, Screenshots)

1. **Direct Visual & Token Decomposition**:
  - Inspect every reference mockup or screenshot directly (`read_file` on image assets) before writing layout code.
  - Map colors to `ThemePalette` semantic roles (`background`, `card`, `foreground`, `muted`, `primary`, `border`, `overlay`) rather than ad-hoc hex literals so Light and Dark modes stay coherent across all 17 schemes.
  - Derive disciplined radii, heights, shadows, and typography from `ThemeGeometry` (`radius_sm`/`md`/`lg`/`xl`, `control_sm`/`md`/`lg`, `font_body`/`heading`/`title`, `shadow_xs`/`sm`/`md`/`lg`).
  - Enforce compact icon-driven chrome: prefer crisp native vector `EuvIcon` icon buttons (each carrying explicit `a11y_label` and `test_id:` hint) over verbose toolbar text, and keep per-page navbar titles to 1–2 concise words with single-line `TextOverflow::Ellipsis`.

2. **Authoritative `euv-ui` Component Mapping**:
  - Shell & Navigation: `EuvAppShell` (unified sidebar/navbar brand mark), `EuvBottomNav`, `EuvRouter` / `EuvRouterOutlet` (`PageRetention::Freeze`).
  - Overlays & Pickers: `EuvDrawer`, `EuvSheet`, unified full-width `EuvDropdown` / `EuvSelect` / `EuvCombobox` / `EuvNativeSelect` / `EuvDatePicker` (trigger height `g.get_control_md()`, popup width matched 1:1 to trigger width).
  - Rich Content & Data Visualization: `EuvMarkdown` (GFM, alerts, math, footnotes), `EuvCodeBlock` (native monospace, `TextOverflow::Clip`, VS Code Eva Light/Dark Bold), `EuvMermaid` (vector diagrams), `EuvEChart` (20-series ECharts suite via `EuvChartOption`), and `EuvChart` (lightweight Bar/Line/Area).

3. **Mandatory Layout, Macro & Safe-Area Invariants (`AGENTS.md` §2–§3)**:
  - Bind every event handler, canvas painter, and reactive closure to a sibling `let` binding or outer helper `fn` before `view! { ... }`. Pass custom component props as named `Component { prop: value }` attributes (`#[derive(Props)]`). Keep `style! { ... }` to one rule per line with zero semicolons.
  - **Auto-Fill Wrapped Button Rows**: Configure multi-button control groups with `FlexDirection::Row`, `FlexWrap::Wrap`, `width: Length::Percent(100.0)`, `gap: SPACE_SM`, and child `flex_grow: 1.0` + content-safe `min_width` so every wrapped row (including the last) fills 100% width with zero trailing empty gap.
  - **Zero Card Text Overflow**: Set `overflow: Overflow::Hidden` on all cards, bubbles, and alerts, and set `width: Length::Percent(100.0)` on wrapped `text` nodes (or `flex_shrink: 1.0` + `min_width: Length::Px(0.0)` inside flex rows) from `360px` mobile to `4K`.
  - **Reactive SafeArea & Edge-to-Edge Fullscreen**: Bind top bars, bottom navigation, and fullscreen overlays to `Context::get_safe_area()` (`Signal<SafeAreaInsets>`) so notches, status/nav bars, and Dynamic/Atomic Island pills never clip content; toggle immersive fullscreen cleanly via `WindowCommand::SetFullscreen` / reactive fullscreen state.

4. **Side-by-Side Visual & Anomaly QA Gate**:
  - Render and inspect screenshots side-by-side against the reference via `read_file` across mobile (`390x844`) and desktop (`1280x800`) viewports in both `ThemeMode::Light` and `ThemeMode::Dark`.
  - Run `UiAutomator::new(&mut app)` / `AiSchemaProtocol::snapshot(&app, true)` across breakpoints (`360px`, `390px`, `768px`, `1280px`) and assert `anomalies.is_empty()` (zero `HorizontalOverflow`, `NegativeBounds`, `ZeroSizeVisible`, `TextOverlap`).

## Part 2: Video, Motion & Interaction Restoration

1. **Keyframe & Timing Decomposition**:
  - Extract keyframes from reference videos/recordings at rest (`t0`), gesture threshold/mid-flight (`25%`, `50%`, `75%`), and settled (`t1`) states.
  - Record exact motion invariants: translation axis, opacity/scrim curve, duration, drag-to-dismiss threshold, and post-release settle behavior.

2. **Monotonic Zero-Overshoot Easing & 1:1 Coupled Transitions**:
  - Use strictly monotonic curves (`Easing::EaseOutQuint` / `Easing::EaseOutCubic` for entrances, `Easing::EaseInCubic` for exits) on `EuvDrawer`, `EuvSheet`, and page transitions — never overshooting springs on edge-anchored overlays.
  - Couple edge-to-edge swipe/slide transitions 1:1 to finger displacement (`Context::get_swipe_drag_offset`), and include interactive floating grabbers on drawers/sheets supporting drag-to-close and drag-to-expand.

3. **GestureArena Single-Owner Scroll Containment**:
  - Respect exclusive gesture ownership (`ActiveGesture::{RouteSwipe, HorizontalScroll, VerticalScroll, DirectDrag, ScrollbarDrag, TextSelection, Pinch}`) and strict axis lock:
    - When `RouteSwipe` or `VerticalScroll` wins, zero horizontal deltas may leak into nested `EuvCodeBlock`, `EuvTable`, or horizontal `ScrollView` children.
    - When `HorizontalScroll` wins inside a nested scrollable, lock strictly to that child (`dy = 0.0`) so page swipe and outer vertical scroll never jitter.
    - Ensure `PointerCancel`, focus loss, `SystemEvent::Inactive`, and `SystemEvent::Background` always clear drag/swipe offsets and pressed state cleanly.

4. **Background 0% CPU/GPU Freeze**:
  - Wire animated pages, media playback, and continuous loops to suspend on `SystemEvent::Background` (`App::is_background_frozen() == true`, `layout_nodes == 0`, `draw_commands == 0`, `pixels_shaded == 0`) and resume losslessly on `SystemEvent::Foreground`.

5. **Multi-Timestamp Frame-Continuity & Tail-Latency Verification**:
  - Step deterministic frames via `App::step_game_frame(dt)` / `UiAutomator::step_frames(n)` and capture `AiSchemaProtocol::capture_same_frame(&app, true, true, hooks)` at `t0`, mid-transition, and settled frames; assert `synced == true`, monotonic geometry bounds, changing `rgba_fnv1a` during active motion, and stable `rgba_fnv1a` once settled (`has_active_animations == false`).
  - Enforce release frame budgets: **144Hz `<= 6.94ms`** and **120Hz `<= 8.33ms`** with `0` frames exceeding `6.94ms` in `--stress-matrix`.

## Verification Commands

```sh
cargo run -p euv-cli -- fmt
cargo run -p euv-cli -- fmt
cargo run -p euv-cli -- fmt --check
cargo test --all-features --workspace
./scripts/bench/stress-bench.sh --runs 3
cargo run -p euv-example -- --headless --software
```

## Related Skills

- [euv Core](../euv/SKILL.md) · [AI Automation](../euv-ai-automation/SKILL.md) · [E2E Replay](../euv-e2e-replay/SKILL.md) · [AGENTS Governance](../../../AGENTS.md)
