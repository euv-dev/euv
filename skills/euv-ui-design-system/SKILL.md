---
name: euv-ui-design-system
description: 'Canonical UI design system and engineering specification for euv and euv-ui applications: layout grid, spacing scale, unified component size ramp, alignment, typography, 17 multi-theme semantic color tokens, corner radius hierarchy, interactive behaviors (drag, scroll, hover, focus), motion/transition timings, and low-boilerplate page composition.'
license: MIT
metadata:
  euv:
    version: "0.38.0"
    applies_to:
      - ui
      - example
      - core/render
---

# euv UI Design System & Component Specification

This skill is the single source of truth for building consistent, refined, multi-theme cross-platform interfaces in `euv` and `euv-ui` across Web, macOS, iOS, iPadOS, Android, Windows, and Linux.

---

## 1. Layout & Responsive Grid

All surfaces and controls snap to a strict **4px spatial grid**:

| Token | Value | Usage |
| --- | ---: | --- |
| `SPACE_XS` | `4px` | Label-to-control gap, inline icon gap, tight segmented inner padding |
| `SPACE_SM` | `8px` | Horizontal action-row gap, checkbox/radio/switch label gap, compact card inner gap |
| `SPACE_MD` | `12px` | Form field spacing, card header-to-body gap, default control horizontal padding |
| `SPACE_LG` | `16px` | Standard card padding (`16px`), page section rhythm (`16px`–`20px`) |
| `SPACE_XL` | `20px` | Modal/drawer/sheet padding, desktop page vertical section gap |
| `SPACE_2XL` | `24px` | Expanded desktop main-canvas padding, empty-state vertical breathing room |

### Breakpoints & App Shell
- **Compact (`< 640px`, phones)**:
  - Fixed top chrome bar (`56px` height) with brand title + `Menu` trigger; respects OS safe-area insets.
  - Slide-in left navigation drawer (`82%` width, `max-width: 280px`) over dimmed scrim (`z-index: 201`) with smooth horizontal slide + fade animation.
  - Main scroll canvas padding: `16px`; KPI stat grids collapse to `1` column; scheme/card grids collapse to `2` columns.
- **Medium (`640px..1023px`, tablets & narrow windows)**:
  - Persistent left navigation rail (`192px` width, `12px` padding).
  - Main scroll canvas padding: `16px`; KPI stat grids use `2` columns; scheme grids use `3` columns.
- **Expanded (`>= 1024px`, desktop & large tablets)**:
  - Persistent left navigation rail (`240px` width, `16px` padding).
  - Main scroll canvas padding: `24px`; centered max content readability (`max-width: 1120px`); KPI grids use `3` columns; scheme grids use `4` columns.

---

## 2. Unified Component Size Scale

**Every interactive control MUST align on one unified 3-tier height ramp** derived from `ThemeGeometry`:

| Tier | Token | Default (Shadcn) | Font Size | Horizontal Padding | Radius |
| --- | --- | ---: | ---: | ---: | --- |
| **Small (`Sm`)** | `control_sm` | `30px` | `12px` | `12px` | `radius_sm` |
| **Medium (`Md`, default)** | `control_md` | `36px` | `13px` | `16px` | `radius_md` |
| **Large (`Lg`)** | `control_lg` | `44px` | `14px` | `20px` | `radius_md` |

### Component Geometry Invariants
- **Buttons (`EuvButton`)**: Exact `control_sm` / `control_md` / `control_lg` height; `Icon` button is a square `control_md × control_md`; text strictly single-line centered (`TextAlign::Center`, `TextOverflow::Clip`).
- **Text Inputs (`EuvInput`) & Selects (`EuvSelect`)**: Exact `control_md` height, `1px` hairline `border`, `radius_md`, `0 12px` horizontal padding, left-aligned `13px` text, `2px` focus ring on focus.
- **Number Stepper (`EuvNumberInput`)**: `-` button, numeric display box (`72px` wide), and `+` button MUST all share the exact same `control_sm` height and `AlignItems::Center`.
- **Segmented Control (`EuvSegmented`) & Tabs (`EuvTabs`)**: Outer pill container `min-height: control_md`, `4px` inner padding (`SPACE_XS`), `radius_md`, `muted` background; each option pill uses exact `control_sm` height (`30px` -> aligns cleanly inside `38px` bar), `0 12px` padding, `radius_sm`, active `card` surface + subtle `1px` drop shadow.
- **Switch (`EuvSwitch`)**: `44×24px` pill track (`radius: 12px`), `20×20px` elevated white thumb (`radius: 10px`), `2px` inner inset, `AlignItems::Center` with `13px` label.
- **Checkbox (`EuvCheckbox`) & Radio (`EuvRadioGroup`)**: Uniform `18×18px` indicator (`radius_sm` for checkbox, `9px` full circle for radio), `AlignItems::Center` with `SPACE_SM` (`8px`) gap to `13px` label.
- **Slider (`EuvSlider`)**: Full-width interactive hit track (`control_sm` height = `30px` vertical hit area for effortless drag!), `6px` centered rounded bar (`radius: 3px`), `18px` circular thumb (`9px` outer primary ring + `6.5px` inner background core), right-aligned fixed-width `40px` tabular numeric readout (`12px` muted foreground).
- **Badges (`EuvBadge`) & Tags (`EuvTag`)**: Uniform `22px` height (`2px 10px` padding, `11px` SemiBold text, pill `9999px` radius for badges / `radius_sm` for bordered tags).
- **Cards (`EuvCard`, `EuvStatCard`)**: `card` background, `1px solid border`, `radius_lg`, `16px` (`SPACE_LG`) uniform padding, `12px` (`SPACE_MD`) vertical gap, subtle `Shadow(0, 1, 2, 0, rgba(0,0,0,14))`.

---

## 3. Alignment & Typography Hierarchy

- **Horizontal Rows**: Any row containing mixed controls (buttons, inputs, badges, switches, steppers) MUST set `AlignItems::Center` and `FlexWrap::Wrap` with `gap: SPACE_SM` (`8px`) so wrapped items never collide or clip.
- **Text Alignment**: Headings, subtitles, form labels, descriptions, table cells, and list rows are left-aligned. Buttons, icon badges, avatars, and numeric stepper readouts are horizontally & vertically centered.
- **Type Scale**:
  - **Page Title (`H1` / `EuvHeader`)**: `font_title` (`24px`), `Bold`, `foreground`
  - **Section / Card Heading (`H2` / `EuvCard`)**: `font_heading` (`16px`), `SemiBold`, `card_foreground`
  - **Subheading (`H3`)**: `14px`, `SemiBold`, `foreground`
  - **Body Copy**: `font_body` (`14px`), `Regular`, `foreground`
  - **Control & Form Label**: `13px`, `Medium`, `foreground`
  - **Helper / Subtitle / Caption**: `12px`, `Regular`, `muted_foreground`
  - **Badge / Kbd / Micro Label**: `11px`, `SemiBold`, role foreground

---

## 4. Color Roles & Multi-Theme Architecture (17 Schemes × Light/Dark)

**Zero hardcoded color literals in business pages.** Every color MUST come from `ThemeTokens` (`ThemePalette`):
- **Surfaces**: `background` (app canvas), `card` (elevated surfaces & inputs), `muted` (subtle wells, tabs/segmented tracks, table headers), `overlay` (modal/drawer scrim).
- **Text**: `foreground`, `card_foreground`, `muted_foreground`.
- **Interactive & Brand**: `primary` / `primary_foreground`, `secondary` / `secondary_foreground`, `accent` (active nav item & hover surface).
- **Status**: `destructive` / `destructive_foreground`, `success`, `warning`.
- **Chrome**: `border` (`1px` structural hairline), `ring` (`2px` keyboard focus ring).
- **Button Hierarchy Rule**: At most **one `Primary`** button per action group; secondary actions use `Secondary` or `Outline`; dismiss/cancel uses `Ghost`; irreversible actions use `Destructive`.

---

## 5. Corner Radius Hierarchy

All radii derive strictly from the active scheme's `ThemeGeometry`:
- `radius_sm` — Checkboxes, tags, kbd caps, segmented/tab active pills, table headers, scrollbar thumbs.
- `radius_md` — Buttons, inputs, textareas, selects, dropdown menus, popovers, toasts, virtual lists, panels.
- `radius_lg` — Cards, stat cards, alerts, empty states, notifications.
- `radius_xl` — Modals, bottom sheets, top-level dialogs.
- Full pill (`height / 2` or `9999px`) — Switches, avatars, badges, radio circles, slider thumbs.

---

## 6. Interaction Logic & Motion Specification

### Interactive Contracts
- **Hover**: Paint-only tint (`interactive_tint` / `hover_paint`); never mutate layout geometry on hover so hover redraws remain confined to the single control's dirty rect.
- **Pressed**: Subtle `0.5px` downward micro-press feedback + deeper tint; pointer capture routes `PointerMove` / `PointerUp` to the pressed control even if the cursor leaves its bounding box.
- **Slider Dragging**: `PointerDown` seeks immediately to click X coordinate and captures pointer; `PointerMove` updates `Signal<f32>` continuously at frame rate; `PointerUp` releases capture.
- **Virtual & Native Scrolling**:
  - Fast wheel scrolling MUST re-resolve the live scrollable container under the current pointer position whenever the previously hovered child was unmounted/recycled during virtualization.
  - Virtualized lists (`EuvVirtualList`) MUST report `(virtual_scroll_y, virtual_total_height)` to the retained `LayoutBox` so the right-side overlay scrollbar thumb reflects true scroll progress (`0%..100%`) and supports direct thumb dragging.
- **Overlay Dismiss**: Clicking the dimmed backdrop scrim of a Modal, Drawer, Sheet, or Mobile Nav Drawer closes it cleanly (clicks inside the dialog/panel do not dismiss).

### Standard Motion & Transition Timings
All transitions animate **paint/transform properties only** (`opacity`, `translate_x`, `translate_y`, `scale_x`, `scale_y`) inside the core animation ticker so layout is never recomputed mid-animation and CPU drops to `0%` once settled:

| Surface | Enter Transition | Exit Transition | Duration & Curve |
| --- | --- | --- | --- |
| **Route Switch (`DynamicPageOutlet`)** | `opacity: 0 -> 1`, `translate_y: +12px -> 0` | Immediate swap into entering page | `220ms EaseOutCubic` |
| **Modal (`EuvModal`)** | Scrim `opacity: 0 -> 1`; dialog `opacity: 0 -> 1`, `scale: 0.95 -> 1.0`, `translate_y: +12px -> 0` | Reverse + `Display::None` on complete | `200ms EaseOutCubic` |
| **Right Drawer (`EuvDrawer`)** | Scrim `opacity: 0 -> 1`; panel `translate_x: +360px -> 0` | Slide out `+360px` + fade scrim | `240ms EaseOutCubic` |
| **Mobile Left Drawer (`EuvAppShell`)** | Scrim `opacity: 0 -> 1`; panel `translate_x: -280px -> 0` | Slide out `-280px` + fade scrim | `240ms EaseOutCubic` |
| **Bottom Sheet (`EuvSheet`)** | Scrim `opacity: 0 -> 1`; panel `translate_y: +240px -> 0` | Slide down `+240px` + fade scrim | `240ms EaseOutCubic` |
| **Popover / Dropdown / Select Menu** | `opacity: 0 -> 1`, `translate_y: -6px -> 0` | Fade out + `Display::None` | `160ms EaseOutCubic` |
| **Toast / Notification** | `opacity: 0 -> 1`, `translate_y: +16px -> 0` (or `translate_x: +24px -> 0`) | Slide/fade out + `Display::None` | `200ms EaseOutCubic` |

---

## 7. Zero-Boilerplate Business Page Development

Business pages MUST compose built-in multi-theme `euv-ui` primitives instead of hand-writing raw `Style::empty()` layout boilerplate:
1. Wrap pages with `EuvPage::new(title, subtitle, tokens)` (automatic `EuvHeader` + uniform `SPACE_LG` vertical rhythm).
2. Use `EuvActionRow::new()` for horizontal button/control bars (automatic wrap, `AlignItems::Center`, `SPACE_SM` gap).
3. Use `EuvStatGrid::responsive(tokens)` for adaptive 1/2/3-column KPI stat cards.
4. Chain fluent component builders: `EuvButton::new("Save", tokens).primary().md().on_click(...)`, `EuvButton::new("Cancel", tokens).outline().sm()`, `EuvCard::new("Section", tokens).subtitle("...").child(...)`.
5. Never hardcode colors, control heights, border widths, or radii inside application pages — let `ThemeTokens` drive all 17 schemes and Light/Dark modes automatically.
