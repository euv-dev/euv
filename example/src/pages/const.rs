pub(crate) const LABEL_PLATFORM_ANDROID: &str = "Android";
pub(crate) const LABEL_PLATFORM_MACOS: &str = "macOS";
pub(crate) const LABEL_CALENDAR_MONTH: &str = "November 2026";
pub(crate) const OVERVIEW_TITLE: &str = "Cross-Platform Native Runtime";
pub(crate) const OVERVIEW_SUBTITLE: &str =
    "Unified retained scene graph and deterministic layout across 7 targets.";
pub(crate) const STAT_PLATFORMS_LABEL: &str = "Native Targets";
pub(crate) const STAT_PLATFORMS_VAL: &str = "7 OS / Web";
pub(crate) const STAT_PLATFORMS_CAP: &str = "Web, Android, iOS, iPadOS, macOS, Windows, Linux";
pub(crate) const STAT_SCHEMES_LABEL: &str = "Design Schemes";
pub(crate) const STAT_SCHEMES_VAL: &str = "17 Presets";
pub(crate) const STAT_SCHEMES_CAP: &str = "Zero-CSS compiled Light & Dark tokens";
pub(crate) const STAT_RENDER_LABEL: &str = "Render Modes";
pub(crate) const STAT_RENDER_VAL: &str = "Auto / SW / HW / Hybrid";
pub(crate) const STAT_RENDER_CAP: &str = "Bit-identical pixel parity";
pub(crate) const TABLE_COL_TARGET: &str = "Target";
pub(crate) const TABLE_COL_BACKEND: &str = "System API";
pub(crate) const TABLE_COL_GPU: &str = "GPU Shader";
pub(crate) const ROW_WEB: &str = "Web";
pub(crate) const ROW_WEB_API: &str = "DOM Canvas / WebGPU / WebGL2";
pub(crate) const ROW_WEB_GPU: &str = "WGSL / GLSL ES";
pub(crate) const ROW_MACOS: &str = "macOS";
pub(crate) const ROW_MACOS_API: &str = "AppKit / CoreGraphics / Metal";
pub(crate) const ROW_MACOS_GPU: &str = "MSL";
pub(crate) const ROW_IOS: &str = "iOS / iPadOS";
pub(crate) const ROW_IOS_API: &str = "UIKit / CAMetalLayer / Pencil";
pub(crate) const ROW_ANDROID: &str = "Android";
pub(crate) const ROW_ANDROID_API: &str = "NDK ANativeWindow / Vulkan";
pub(crate) const ROW_ANDROID_GPU: &str = "SPIR-V / GLES";
pub(crate) const ROW_WINDOWS: &str = "Windows";
pub(crate) const ROW_WINDOWS_API: &str = "Win32 / GDI / Direct3D 12";
pub(crate) const ROW_WINDOWS_GPU: &str = "HLSL";
pub(crate) const ROW_LINUX: &str = "Linux";
pub(crate) const ROW_LINUX_API: &str = "dlopen Wayland & X11 / Vulkan";
pub(crate) const CARD_PLATFORMS_TITLE: &str = "Platform Capability Matrix";
pub(crate) const CARD_PLATFORMS_SUB: &str = "Direct system API adapters with zero WebView wrappers";

pub(crate) const REACTIVE_TITLE: &str = "Fine-Grained Reactivity";
pub(crate) const REACTIVE_SUBTITLE: &str =
    "Glitch-free push-pull memos and surgical node patching.";
pub(crate) const REACTIVE_CARD_TITLE: &str = "Live Signal & Memo Graph";
pub(crate) const REACTIVE_CARD_SUB: &str =
    "Mutations update only subscribed text nodes without VDOM diffing.";
pub(crate) const BTN_INCREMENT: &str = "Increment (+1)";
pub(crate) const BTN_BATCH_TEN: &str = "Batch (+10)";
pub(crate) const BTN_RESET: &str = "Reset";
pub(crate) const PREFIX_COUNTER: &str = "Counter: ";
pub(crate) const PREFIX_DOUBLED: &str = "Memo (x2): ";

pub(crate) const RENDER_TITLE: &str = "Multi-Mode Rendering";
pub(crate) const RENDER_SUBTITLE: &str =
    "Switch cleanly between Auto, Software, Hardware, and Hybrid pipelines.";
pub(crate) const RENDER_CARD_TITLE: &str = "Pipeline Mode & Vector Canvas";
pub(crate) const RENDER_CARD_SUB: &str =
    "Analytical SDF software rasterizer paired with instanced GPU shaders.";
pub(crate) const MODE_AUTO: &str = "Auto";
pub(crate) const MODE_SOFTWARE: &str = "Software";
pub(crate) const MODE_HARDWARE: &str = "Hardware";
pub(crate) const MODE_HYBRID: &str = "Hybrid";
pub(crate) const CANVAS_LABEL: &str = "Retained 2D Canvas Preview";

pub(crate) const WINDOW_TITLE: &str = "Window & Platform Inspector";
pub(crate) const WINDOW_SUBTITLE: &str =
    "Unified window configuration, DPI scale, and safe area insets.";
pub(crate) const WINDOW_CARD_TITLE: &str = "Runtime Platform Telemetry";
pub(crate) const WINDOW_CARD_SUB: &str =
    "Identical API surface across desktop, tablet, mobile, and browser.";
pub(crate) const SWITCH_FULLSCREEN: &str = "Request Fullscreen Mode";
pub(crate) const SYS_EVENTS_CARD_TITLE: &str = "System Events & Lifecycle Monitor";
pub(crate) const SYS_EVENTS_CARD_SUB: &str = "Live normalized AppState, OS theme, network, keyboard, safe area, and Appear/Disappear hooks across all 7 targets.";
pub(crate) const SYS_EVENTS_LOG_LIMIT: usize = 8;
pub(crate) const CODE_WINDOW_SNIPPET: &str = "let mut w: WindowConfig = WindowConfig::default();\nw.set_title(\"euv\".into()).set_size(1280, 800);\nw.set_render(RenderConfig::default());";

pub(crate) const FORMS_TITLE: &str = "Interactive Form Controls";
pub(crate) const FORMS_SUBTITLE: &str =
    "Reactive text inputs, toggles, checkboxes, tabs, and button variants.";
pub(crate) const FORMS_CARD_TITLE: &str = "Profile Preferences";
pub(crate) const FORMS_CARD_SUB: &str =
    "Two-way signal bindings with focus ring and hit-tested keyboard input.";
pub(crate) const FIELD_USERNAME: &str = "Developer Handle";
pub(crate) const PLACEHOLDER_USERNAME: &str = "Enter handle...";
pub(crate) const SWITCH_HW_LABEL: &str = "Prefer Hardware GPU Pipeline";
pub(crate) const CHECKBOX_TERMS_LABEL: &str = "Enable deterministic frame telemetry";
pub(crate) const TAB_GENERAL: &str = "General";
pub(crate) const TAB_GRAPHICS: &str = "Graphics";
pub(crate) const TAB_SECURITY: &str = "Security";
pub(crate) const BTN_PRIMARY_LABEL: &str = "Save Changes";
pub(crate) const BTN_SECONDARY_LABEL: &str = "Secondary";
pub(crate) const BTN_OUTLINE_LABEL: &str = "Outline";
pub(crate) const BTN_DESTRUCTIVE_LABEL: &str = "Reset Profile";

pub(crate) const THEMES_TITLE: &str = "17 Compiled Design Schemes";
pub(crate) const THEMES_SUBTITLE: &str =
    "Zero-CSS compile-time palettes with Light and Dark theme tokens.";
pub(crate) const THEMES_CARD_TITLE: &str = "Active Theme Controls";
pub(crate) const THEMES_CARD_SUB: &str =
    "Switch design system presets or toggle Light/Dark mode dynamically.";
pub(crate) const BTN_NEXT_SCHEME: &str = "Cycle Design Scheme";
pub(crate) const BTN_TOGGLE_MODE: &str = "Toggle Light / Dark";
pub(crate) const PREFIX_ACTIVE_SCHEME: &str = "Active Scheme: ";

pub(crate) const DATA_TITLE: &str = "Virtualized Lists & Data Display";
pub(crate) const DATA_SUBTITLE: &str =
    "O(visible) virtualization for 10,000 rows plus progress and badges.";
pub(crate) const DATA_CARD_TITLE: &str = "10,000-Row Virtual Viewport";
pub(crate) const DATA_CARD_SUB: &str =
    "Only rows intersecting the scroll viewport plus overscan are mounted.";
pub(crate) const ROW_PREFIX: &str = "Virtual Row #";
pub(crate) const BADGE_READY: &str = "Deterministic";
pub(crate) const BADGE_LIVE: &str = "60-120 FPS";
pub(crate) const BTN_SCROLL_DOWN: &str = "Scroll Down (+160px)";
pub(crate) const BTN_SCROLL_TOP: &str = "Back to Top";

pub(crate) const GAME_TITLE: &str = "Real-Time Game Engine & HUD";
pub(crate) const GAME_SUBTITLE: &str = "Fixed-timestep FrameClock, InputState polling, GameScene viewport, and declarative HUD overlay.";
pub(crate) const GAME_CARD_TITLE: &str = "Embedded 60Hz Physics Arena";
pub(crate) const GAME_CARD_SUB: &str = "Step deterministic physics frames or overlay transparent euv HUD panels on top of any 2D/3D engine.";
pub(crate) const BTN_STEP_PHYSICS: &str = "Step Physics (+3 Frames)";
pub(crate) const BTN_EXIT_FULLSCREEN: &str = "Exit Fullscreen";
pub(crate) const FS_OVERLAY_NAME: &str = "GameFullscreenPortal";
pub(crate) const FS_ACTION_ON: &str = "on";
pub(crate) const FS_ACTION_OFF: &str = "off";
pub(crate) const FS_CTRL_LEFT: &str = "Left";
pub(crate) const FS_CTRL_UP: &str = "Up";
pub(crate) const FS_CTRL_DOWN: &str = "Down";
pub(crate) const FS_CTRL_RIGHT: &str = "Right";
pub(crate) const FS_CTRL_DASH: &str = "Dash";
pub(crate) const FS_CTRL_DRIVE: &str = "Drive";
pub(crate) const FS_CTRL_REVERSE: &str = "Rev";
pub(crate) const FS_CTRL_CAMERA: &str = "Camera";
pub(crate) const GAME3D_TITLE: &str = "AAA 3D Open-World Engine & Ray Tracing";
pub(crate) const GAME3D_SUBTITLE: &str = "BVH ray-traced shadows, glossy reflections, GI, LOD streaming, .OBJ/.glTF model import, and live runtime quality tuning.";
pub(crate) const GAME3D_CARD_TITLE: &str = "Vice Boulevard 3D Sandbox";
pub(crate) const GAME3D_CARD_SUB: &str = "Drive with WASD/Arrows or mobile controls; adjust preset, ray tracing, resolution scale, sun hour, or import 3D models live.";

pub(crate) const OVERLAYS_TITLE: &str = "Overlays, Modals & Toasts";
pub(crate) const OVERLAYS_SUBTITLE: &str =
    "Portal-mounted fixed layers with z-index stacking and scrim backdrops.";
pub(crate) const OVERLAYS_CARD_TITLE: &str = "Overlay Triggers";
pub(crate) const OVERLAYS_CARD_SUB: &str =
    "Toggle reactive portal layers without re-mounting the scene tree.";
pub(crate) const BTN_OPEN_MODAL: &str = "Open Dialog Modal";
pub(crate) const BTN_CLOSE_MODAL: &str = "Close Modal";
pub(crate) const BTN_TOGGLE_TOAST: &str = "Toggle Toast Banner";
pub(crate) const MODAL_TITLE: &str = "Cross-Platform Native Dialog";
pub(crate) const MODAL_BODY_TEXT: &str =
    "Rendered via euv retained portal layer with consistent geometry across all 7 platforms.";
pub(crate) const TOAST_TEXT: &str = "Render pipeline synchronized cleanly.";
pub(crate) const ALERT_TITLE: &str = "Zero-WebView Native Architecture";
pub(crate) const ALERT_DESC: &str = "Every pixel is computed by the unified Rust layout engine and rasterized or GPU-batched directly.";

pub(crate) const DYNAMIC_TITLE: &str = "Dynamic Component & Page OTA Updates";
pub(crate) const DYNAMIC_SUBTITLE: &str = "Surgical component-level hot updates, page-level live swaps, atomic JSON bundles, and runtime route injection.";
pub(crate) const DYNAMIC_COMP_CARD_TITLE: &str = "Component-Level Dynamic Slots";
pub(crate) const DYNAMIC_COMP_CARD_SUB: &str = "Update individual named slots via signed JSON or native Rust closures without resetting sibling state.";
pub(crate) const DYNAMIC_PAGE_CARD_TITLE: &str = "Page-Level Dynamic Updates & Route Injection";
pub(crate) const DYNAMIC_PAGE_CARD_SUB: &str = "Hot-swap entire page layouts, apply atomic multi-target bundles, or inject brand-new routes at runtime.";
pub(crate) const BTN_INC_SIBLING: &str = "Increment Sibling (+1)";
pub(crate) const BTN_PUSH_BANNER_V1: &str = "Push Banner v1 (JSON)";
pub(crate) const BTN_PUSH_BANNER_V2: &str = "Push Banner v2 (Native)";
pub(crate) const BTN_ROLLBACK_BANNER: &str = "Rollback Banner";
pub(crate) const BTN_PATCH_METRICS: &str = "Patch Metrics Widget";
pub(crate) const BTN_ROLLBACK_METRICS: &str = "Rollback Metrics";
pub(crate) const BTN_UPDATE_PAGE_V1: &str = "Hot-Update Page (v1)";
pub(crate) const BTN_APPLY_BUNDLE: &str = "Apply Atomic Bundle";
pub(crate) const BTN_ROLLBACK_PAGE: &str = "Rollback Page";
pub(crate) const BTN_INJECT_ROUTE: &str = "Inject & Visit /promo-live";
pub(crate) const BTN_CLAIM_OFFER: &str = "Claim Dynamic Offer";
pub(crate) const BTN_RETURN_DYNAMIC: &str = "Return to Dynamic OTA";
#[cfg(not(debug_assertions))]
pub(crate) const AI_SCHEMA_RELEASE_STRIPPED: &str = "stripped-in-release";
pub(crate) const AI_BRIDGE_STATUS_ACTIVE: &str = "dev-active";
pub(crate) const AI_BRIDGE_STATUS_OFF: &str = "0B-release";
pub(crate) const TOUCH_PAD_READY_TEXT: &str = "Pad Ready: touch inside (1-5 fingers, isolated)";
pub(crate) const TOUCH_PAD_CANVAS_HINT: &str = "Touch or drag with 1-5 fingers here";
pub(crate) const FORMS_EXT_CARD_SUB: &str =
    "ToggleGroup, InputOTP, ColorPicker, TimePicker, TagInput, Calendar, and Collapsible";
pub(crate) const TAG_INPUT_PLACEHOLDER: &str = "Add feature tag...";
pub(crate) const TREE_NODE_WORKSPACE: &str = "Workspace";
pub(crate) const TREE_NODE_CORE: &str = "euv-core";
pub(crate) const TREE_NODE_UI: &str = "euv-ui";
pub(crate) const TREE_NODE_PERF: &str = "euv-performance";
pub(crate) const TREE_NODE_RENDER: &str = "Renderer (SW/GPU)";
pub(crate) const TREE_NODE_REACTIVE: &str = "Reactive Signals";
pub(crate) const TREE_NONE_SELECTED: &str = "None";
pub(crate) const TREE_SELECTED_PREFIX: &str = "Selected Tree Node";
pub(crate) const HERO_BADGE_VER: &str = "euv 0.43";
pub(crate) const LAB_ITEM_VER: &str = "v0.43";
pub(crate) const HOVER_CARD_ENGINE_TITLE: &str = "euv 0.43 Native Engine";
pub(crate) const HERO_BADGE_ZERO_WV: &str = "Zero WebView";
pub(crate) const HERO_BADGE_EVA: &str = "VS Code Eva Bold";
pub(crate) const HERO_BADGE_FREEZE: &str = "Freeze Router";
pub(crate) const HERO_HEADLINE: &str =
    "Build Native Apps, 3D Worlds & Markdown Studios in Pure Rust";
pub(crate) const HERO_DESCRIPTION: &str = "Zero-context view! & style! macros, spring animations, lazy PageRetention::Freeze routing, cross-component events, and bit-identical rendering across 7 targets.";
pub(crate) const CARD_LIFECYCLE_TITLE: &str = "Live Page Lifecycle & Global State Monitor";
pub(crate) const CARD_LIFECYCLE_SUB: &str = "Every route mounts lazily on first visit and freezes on leave (PageRetention::Freeze) with cross-page coalesced sync.";
pub(crate) const BTN_OPEN_MARKDOWN: &str = "Markdown & Code (Eva)";
pub(crate) const BTN_OPEN_3D_RT: &str = "3D Engine (Full GI)";
pub(crate) const BTN_EMIT_PULSE: &str = "Emit Custom Event";
pub(crate) const BTN_LOCAL_INC: &str = "Local State (+1)";
pub(crate) const MD_STUDIO_TITLE: &str = "Markdown & Code Studio";
pub(crate) const MD_STUDIO_SUBTITLE: &str = "Complete GFM & Extended Markdown, native vector Mermaid diagrams, Unicode LaTeX math, selectable text, and scrollable fullscreen Eva Bold code blocks.";
pub(crate) const MD_THEME_CARD_TITLE: &str = "VS Code Eva Bold Theme & Selection Controls";
pub(crate) const MD_THEME_CARD_SUB: &str = "Toggle Light/Dark in-place without losing scroll position, or toggle selectable text across Markdown and Code blocks.";
pub(crate) const BTN_TOGGLE_EVA_MODE: &str = "Switch Eva Light / Dark Bold";
pub(crate) const MD_SELECTABLE_LABEL: &str = "Selectable Text";
pub(crate) const MD_CODE_CARD_TITLE: &str =
    "EuvCodeBlock Multi-Language Scrollable & Fullscreen Inspector";
pub(crate) const MD_CODE_CARD_SUB: &str = "Smooth horizontal + vertical scroll, Fullscreen modal portal, line numbers, highlighted lines, selectable code, and native clipboard Copy.";
pub(crate) const MD_DOC_CARD_TITLE: &str = "Complete Native GFM, Math & Mermaid Document";
pub(crate) const MD_DOC_CARD_SUB: &str = "Headings, inline styles, Emoji, Math, all 5 GitHub Alerts, Definition Lists, Footnotes, scrollable Tables, interactive Checklists, Disclosures, and Vector Mermaid.";
pub(crate) const MD_SAMPLE_RUST: &str = r#"#[component(retention = Freeze)]
impl MarkdownStudioPage {
    fn on_freeze(&self, _node: NodeId) {
        emit!(ShowcasePulseEvent { source: "markdown".into(), seq: 1 });
    }

    fn render(&self) -> NodeId {
        let count: LocalState<i32> = local_state!(42);
        let long_pipeline_description: &str = "Zero-WebView retained scene graph with coupled 1:1 edge-to-edge route transitions, O(1) clean-subtree caching, and 2D/3D hardware + software rendering";
        let box_style: Style = style! {
            AlignItems::Center
            Overflow::Hidden
            width: Length::Percent(100.0)
        };
        let on_copy_cb = move |copied_source: &str| {
            let _: usize = copied_source.len();
        };
        let label_fn = move || -> String {
            format!("Reactive count = {} | {}", count.get(), long_pipeline_description)
        };
        view! {
            column {
                style: box_style
                EuvCodeBlock {
                    code: source
                    tokens: tokens
                    language: "rust"
                    line_numbers: true
                    max_height: 260.0
                    selectable: true
                    on_copy: on_copy_cb
                }
                text {
                    { label_fn }
                }
            }
        }
    }
}"#;
pub(crate) const MD_SAMPLE_TS: &str = r#"// VS Code Eva Light Bold / Eva Dark Bold TypeScript sample
interface RenderPipeline {
  readonly target: "metal" | "vulkan" | "webgpu";
  maxFps: number;
  rayTracing: boolean;
}
export function createPipeline(fps: number): RenderPipeline {
  return { target: "metal", maxFps: fps, rayTracing: true };
}"#;
pub(crate) const MD_SAMPLE_JSON: &str = r##"{
  "theme": "Eva Light Bold / Eva Dark Bold",
  "background_light": "#EBEEF5",
  "background_dark": "#282C34",
  "retention": "Freeze",
  "ray_tracing": "FullPathTracedGi"
}"##;
pub(crate) const MD_SAMPLE_DIFF: &str = r##"@@ -1,4 +1,5 @@
-let style = Style::empty().align(AlignItems::Center).overflow(Overflow::Hidden);
+let style = style! {
+    AlignItems::Center
+    Overflow::Hidden
+};
-impl Mountable for MyPage { fn mount(self, ctx: &mut BuildContext) -> NodeId { ... } }
+#[component(retention = Freeze)]
+impl MyPage { fn render(&self) -> NodeId { view! { ... } } }"##;
pub(crate) const MD_FULL_DOCUMENT: &str = r##"# Native Rust Markdown & Vector Mermaid :rocket:

Welcome to **Euv Markdown Studio** :sparkles: — featuring *zero WebView*, ***bold italic typography***, ~~DOM bloat~~, ==highlighted callouts==, exponents 2^16^, chemical H~2~O, exact `Eva Bold` syntax colors, keyboard shortcuts like <kbd>Cmd+K</kbd>, inline math $O(n \log n)$ & $\alpha + \beta = \gamma$, [Documentation Links](https://euv.ltpp.vip/docs/markdown), [Reference Links][euv-ref], autolinks <https://euv.ltpp.vip>, and footnotes[^1][^2].

First line with explicit hard break  
Second line starts directly beneath via trailing two-space break. :check:

> [!NOTE]
> :info: Every Markdown block and Mermaid diagram below compiles directly to native `euv` scene nodes with bit-identical rendering across all 7 platforms.

> [!TIP]
> :bulb: Toggle **Light / Dark** above — the theme updates reactively in-place without tearing down pages or losing your scroll position!

> [!IMPORTANT]
> Inactive showcase routes use `PageRetention::Freeze` — reactive effects, scroll offsets, and 3D engines persist at 0% CPU until resumed.

> [!WARNING]
> :warning: Always place each style statement inside `style! { ... }` on its own line without semicolons.

> [!CAUTION]
> :fire: Legacy `Mountable` and `Style::empty()` APIs have been removed in favor of `Component`, `Lifecycle`, and `Style::default()`.

## Native Vector Mermaid Diagrams

### Architecture Flowchart

```mermaid
flowchart LR
    A[Reactive Signals] --> B(SceneTree Arena)
    B --> C{Render Mode}
    C -->|GPU| D([Metal / Vulkan / WebGPU])
    C -->|CPU| E([Software Rasterizer])
    D --> F((60-240Hz Frame))
    E --> F
```

### Coupled Transition Sequence

```mermaid
sequenceDiagram
    participant User
    participant Shell as EuvAppShell
    participant Router as EuvRouterOutlet
    User->>Shell: Swipe or Tap Tab
    Shell->>Router: Coupled 1:1 Slide
    Router-->>User: EaseOutCubic 0ms Gap
```

### State Machine & Telemetry Breakdown

```mermaid
stateDiagram-v2
    [*] --> Active
    Active --> Frozen: Leave Route
    Frozen --> Active: Resume Coalesced
    Active --> [*]: Destroy
```

```mermaid
pie title Native Frame Budget
    "Layout Cache" : 12
    "DisplayList Paint" : 28
    "GPU / Raster" : 48
    "Idle Headroom" : 12
```

```mermaid
gantt
    title Release Pipeline Milestones
    section Core
    Retained Scene Graph :done, 2026-01-01, 20d
    Coupled Transitions :active, 2026-01-21, 14d
    section UI
    Vector Mermaid + Math :2026-02-04, 10d
```

## Display Math & LaTeX Equations

$$L_o(p, \omega_o) = L_e(p, \omega_o) + \int_{\Omega} f_r(p, \omega_i, \omega_o) L_i(p, \omega_i) (\omega_i \cdot n) \, d\omega_i$$

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}, \quad \sum_{k=1}^{\infty} \frac{1}{k^2} = \frac{\pi^2}{6}$$

## Interactive GFM Checklist & Nested Lists

- [x] Standalone `Lifecycle` + `Component` traits and `#[component(retention = Freeze)]`
- [x] Coupled 1:1 edge-to-edge route slide transitions + sliding `EuvBottomNav` indicator
- [x] Scrollable & Fullscreen `EuvCodeBlock` with selectable text and native clipboard
- [ ] Tap any checkbox row to toggle completion state live

### Ordered Architecture Layers (Custom Start)

3. Fine-grained `Signal`, `LocalState`, `GlobalState`, and `EventBus`
4. Declarative `view!` & `style!` proc macros formatted by `euv fmt`
5. Persistent multi-core software + 3D BVH ray-traced renderer

## Definition Lists

`PageRetention::Freeze`
: Suspends reactive effects while preserving exact per-page scroll position and DOM/scene subtree cache.

`EuvMermaid`
: Pure-Rust native vector canvas renderer supporting flowcharts, sequence, state, class, ER, pie, and Gantt charts with fullscreen inspection.

## Cross-Platform Target Matrix

| Target | Graphics API | Retention Policy | Text Selection |
| :--- | :---: | :---: | ---: |
| **iOS & macOS** | `Metal / CoreGraphics` | Freeze | Native Clipboard |
| **Android** | `Vulkan / NDK` | Freeze | Native Clipboard |
| **WebAssembly** | `WebGPU / Canvas2D` | Freeze | Native Clipboard |
| **Windows & Linux** | `DX12 / Vulkan` | Freeze | Native Clipboard |

```rust
let global: Signal<ShowcaseGlobalBanner> = global_state!(ShowcaseGlobalBanner = ShowcaseGlobalBanner::default());
let _sub: Effect = on_event!(ShowcasePulseEvent => move |ev: &ShowcasePulseEvent| {
    let _: bool = global.update(|g: &mut ShowcaseGlobalBanner| g.set_pulses(g.get_pulses() + ev.get_seq()));
});
```

![Euv Unified Scene & BVH Pipeline](https://euv.ltpp.vip/assets/pipeline-2026.svg)

<details>
<summary>Expand Lifecycle & Cross-Page Sync Details</summary>

When leaving a route, `Scope::freeze()` suspends effects while marking dirty cross-page writes and preserving exact scroll offsets.

- Zero teardown on Light/Dark or 17-scheme switches
- Atomic single-flush resume when returning to a frozen page

</details>

---

> Outer quote highlighting zero-WebView performance.
> > Nested quote: *"Fast, lossless, and idiomatic — native Rust UI from 360px touchscreens to 4K displays."*

[euv-ref]: https://euv.ltpp.vip
[^1]: Exact VS Code `Eva Light Bold` (`#EBEEF5`) and `Eva Dark Bold` (`#282C34`) syntax color tokens.
[^2]: Zero JavaScript or WebView dependencies — every diagram, formula, and table draws directly onto `euv`'s retained scene graph.
"##;
pub(crate) const BTN_OPEN_STRESS_LAB: &str = "Open Stress Lab";
pub(crate) const BTN_TRY_REACTIVITY: &str = "Try Reactivity";
pub(crate) const BTN_OPEN_GAME_ENGINE: &str = "Open Game Engine";
pub(crate) const BTN_OPEN_OS_SERVICES: &str = "Open OS Services";
pub(crate) const LIFECYCLE_ACTIVE_DEFAULT: &str = "Active Cache: / = Active";
pub(crate) const SOURCE_NONE: &str = "none";
pub(crate) const MD_THEME_DESC_LIGHT: &str = "VS Code Eva Light Bold: bg #EBEEF5 | kw #7C4DFF | ctrl #C838C6 | str #53A053 | fn #437AED | ty #00BEC4";
pub(crate) const MD_THEME_DESC_DARK: &str = "VS Code Eva Dark Bold: bg #282C34 | kw #A78CFA | ctrl #CF68E1 | str #98C379 | fn #6495EE | ty #56B7C3";
pub(crate) const MD_TAB_RUST: &str = "Rust";
pub(crate) const MD_TAB_TS: &str = "TypeScript";
pub(crate) const MD_TAB_JSON: &str = "JSON";
pub(crate) const MD_TAB_DIFF: &str = "Diff";
pub(crate) const MD_LANG_RUST: &str = "rust";
pub(crate) const MD_LANG_TS: &str = "ts";
pub(crate) const MD_LANG_JSON: &str = "json";
pub(crate) const MD_LANG_DIFF: &str = "diff";
pub(crate) const MD_FILE_RUST: &str = "src/pages/markdown.rs";
pub(crate) const MD_FILE_TS: &str = "src/pipeline.ts";
pub(crate) const MD_FILE_JSON: &str = "eva-theme.json";
pub(crate) const MD_FILE_DIFF: &str = "refactor.patch";
pub(crate) const CMD_ITEM_OVERVIEW: &str = "Go to Overview (Workspace)";
pub(crate) const CMD_ITEM_MARKDOWN: &str = "Go to Markdown & Code (Eva Bold)";
pub(crate) const CMD_ITEM_LAB: &str = "Go to UI & Edge Lab (Workspace)";
pub(crate) const CMD_ITEM_GAME_3D: &str = "Go to 3D Engine (Full GI)";
pub(crate) const CMD_ITEM_MEDIA: &str = "Go to Media Studio (Engine)";
pub(crate) const CMD_ITEM_DYNAMIC: &str = "Go to Dynamic WASM (OTA)";
pub(crate) const CMD_ITEM_STRESS: &str = "Go to Stress Lab (Benchmarks)";
pub(crate) const CMD_ITEM_WINDOW: &str = "Go to Window & OS (Runtime)";
pub(crate) const CMD_ITEM_THEMES: &str = "Go to Themes (17 Schemes)";
pub(crate) const CMD_ITEM_TOGGLE_MODE: &str = "Toggle Dark / Light Mode";
pub(crate) const CMD_PLACEHOLDER: &str = "Type a route or action...";
pub(crate) const CMD_MODAL_TITLE: &str = "Command Palette";
pub(crate) const BTN_SEARCH_DESKTOP: &str = "Search  Cmd+K";
pub(crate) const BTN_SEARCH_MOBILE: &str = "Search";
pub(crate) const LABEL_MODE_LIGHT: &str = "Light";
pub(crate) const LABEL_MODE_DARK: &str = "Dark";
pub(crate) const BNAV_HOME_LABEL: &str = "Home";
pub(crate) const BNAV_HOME_ICON: &str = "H";
pub(crate) const BNAV_DOCS_LABEL: &str = "Docs";
pub(crate) const BNAV_DOCS_ICON: &str = "M";
pub(crate) const BNAV_CASES_LABEL: &str = "Cases";
pub(crate) const BNAV_CASES_ICON: &str = "L";
pub(crate) const BNAV_STRESS_LABEL: &str = "Stress";
pub(crate) const BNAV_STRESS_ICON: &str = "S";
pub(crate) const BNAV_3D_LABEL: &str = "3D";
pub(crate) const BNAV_3D_ICON: &str = "3D";
pub(crate) const HOME_DIR_TITLE: &str = "Explore Directory";
pub(crate) const HOME_DIR_SUBTITLE: &str =
    "Tap any module below or swipe left/right anywhere to switch routes directly";
pub(crate) const HOME_DIR_ARROW: &str = ">";
pub(crate) const HOME_DIR_TAG_MD: &str = "MD";
pub(crate) const HOME_DIR_TAG_LB: &str = "LB";
pub(crate) const HOME_DIR_TAG_ST: &str = "ST";
pub(crate) const HOME_DIR_TAG_3D: &str = "3D";
pub(crate) const HOME_DIR_TAG_FM: &str = "FM";
pub(crate) const HOME_DIR_TAG_ME: &str = "ME";
pub(crate) const HOME_DIR_TAG_OV: &str = "OV";
pub(crate) const HOME_DIR_TAG_TH: &str = "TH";
pub(crate) const HOME_DIR_TAG_DT: &str = "DT";
pub(crate) const HOME_DIR_TAG_RX: &str = "RX";
pub(crate) const HOME_DIR_TAG_RN: &str = "RN";
pub(crate) const HOME_DIR_TAG_OS: &str = "OS";
pub(crate) const HOME_DIR_TAG_2D: &str = "2D";
pub(crate) const HOME_DIR_TAG_OT: &str = "OT";
pub(crate) const HOME_DIR_DESC_MD: &str = "Eva Light/Dark Bold code blocks & full GFM suite";
pub(crate) const HOME_DIR_DESC_LB: &str = "Multi-touch isolation pad, AI Schema & shadcn parity";
pub(crate) const HOME_DIR_DESC_ST: &str = "10 interactive live performance stress benchmarks";
pub(crate) const HOME_DIR_DESC_3D: &str = "Cinematic + FullPathTracedGi 60 FPS open world";
pub(crate) const HOME_DIR_DESC_FM: &str = "Inputs with exact caret, IME, toggles & pickers";
pub(crate) const HOME_DIR_DESC_ME: &str = "Live Photo, SVG, GIF, Video, Audio, PDF & HTML";
pub(crate) const HOME_DIR_DESC_OV: &str = "Zero-overshoot Drawer, Sheet, Modal & Popover";
pub(crate) const HOME_DIR_DESC_TH: &str = "17 compiled design schemes with Light/Dark mode";
pub(crate) const HOME_DIR_DESC_DT: &str = "100k virtual scroll list, tables & charts";
pub(crate) const HOME_DIR_DESC_RX: &str = "Fine-grained Signals, Memos, Local & Global state";
pub(crate) const HOME_DIR_DESC_RN: &str = "Auto / Software / Hardware / Hybrid render modes";
pub(crate) const HOME_DIR_DESC_OS: &str = "Dynamic Island, permissions, haptics & safe area";
pub(crate) const HOME_DIR_DESC_2D: &str = "2D physics arena with particles & touch joystick";
pub(crate) const HOME_DIR_DESC_OT: &str = "Signed partial & global .wasm hot-swap runtime";

/// Label displayed in showcase telemetry readouts when no active frame presentation is occurring.
pub(crate) const LABEL_FPS_IDLE: &str = "Idle";
