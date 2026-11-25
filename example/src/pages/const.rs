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
pub(crate) const CODE_WINDOW_SNIPPET: &str = "let mut w = WindowConfig::default(); w.set_title(\"euv\".into()).set_size(1280, 800).set_render(RenderConfig::default());";

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
