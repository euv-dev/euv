use super::*;

/// Fallback single-character brand mark when the app brand title is empty.
pub(crate) const SHELL_DEFAULT_INITIAL: &str = "E";

/// Accessibility label assigned to static brand logo emblems in the sidebar and drawer.
pub(crate) const SHELL_BRAND_A11Y_LABEL: &str = "Euv Studio";

/// Shared square dimension in logical pixels for the unified navbar and sidebar brand logo mark.
pub(crate) const SHELL_BRAND_BADGE_SIZE: f32 = 26.0;

/// Minimum auto-fill child width in logical pixels inside `EuvActionRow`.
pub(crate) const ACTION_ROW_MIN_BTN_WIDTH: f32 = 76.0;

/// Maximum auto-synthesized child minimum width in logical pixels inside `EuvActionRow`.
pub(crate) const ACTION_ROW_MAX_BTN_WIDTH: f32 = 210.0;

/// Minimum auto-fill width in logical pixels for compound controls (switches/checkboxes) inside `EuvActionRow`.
pub(crate) const ACTION_ROW_SWITCH_MIN_WIDTH: f32 = 136.0;

/// Top gradient stop color for the obsidian-indigo `Euv Studio` emblem surface (`#192045`).
pub(crate) const SHELL_LOGO_BG_TOP: Color = Color::hex(0x192045);

/// Bottom gradient stop color for the deep obsidian `Euv Studio` emblem surface (`#070A16`).
pub(crate) const SHELL_LOGO_BG_BOTTOM: Color = Color::hex(0x070A16);

/// Luminous white stroke/fill color for the geometric 3-bar `E` monogram inside the emblem.
pub(crate) const SHELL_LOGO_MONOGRAM: Color = Color::hex(0xFFFFFF);

/// Reactive kinetic cyan chevron accent on the middle bar of the geometric `E` emblem (`#38BDF8`).
pub(crate) const SHELL_LOGO_SPARK: Color = Color::hex(0x38BDF8);

/// Bright cyan-ice leading tip highlight on the reactive middle chevron (`#67E8F9`).
pub(crate) const SHELL_LOGO_SPARK_HI: Color = Color::hex(0x67E8F9);

/// Subtle anodized glass rim highlight drawn around the `Euv Studio` emblem perimeter.
pub(crate) const SHELL_LOGO_RIM: Color = Color::rgba(224, 242, 254, 118);

/// Compact mobile header drawer toggle button label.
pub(crate) const SHELL_MENU_LABEL: &str = "Menu";

/// Overlay stack identifier for the `EuvAppShell` compact drawer.
pub(crate) const SHELL_DRAWER_OVERLAY_NAME: &str = "EuvAppShellDrawer";

/// Fixed height in logical pixels for the floating bottom navigation capsule.
pub(crate) const SHELL_BOTTOM_NAV_HEIGHT: f32 = 60.0;

/// Vertical float offset in logical pixels above the bottom edge on compact viewports.
pub(crate) const SHELL_BOTTOM_NAV_FLOAT_COMPACT: f32 = 14.0;

/// Vertical float offset in logical pixels above the bottom edge on medium/wide viewports.
pub(crate) const SHELL_BOTTOM_NAV_FLOAT_WIDE: f32 = 18.0;

/// Minimum horizontal side margin in logical pixels on both sides of the floating dock on compact screens.
pub(crate) const SHELL_BOTTOM_NAV_SIDE_MARGIN: f32 = 20.0;

/// Maximum capsule width in logical pixels for the horizontally centered floating bottom dock.
pub(crate) const SHELL_BOTTOM_NAV_MAX_WIDTH: f32 = 436.0;

/// Status text rendered in the shell FPS pill when no active presentation frames are running.
pub(crate) const SHELL_FPS_IDLE_LABEL: &str = "Idle";

/// Accessibility label assigned to the compact live FPS telemetry pill.
pub(crate) const SHELL_FPS_A11Y_LABEL: &str = "Live FPS";

/// Minimum measured FPS threshold above which the HUD pill reports active frame rate instead of idle.
pub(crate) const SHELL_FPS_ACTIVE_THRESHOLD: f32 = 0.05;
