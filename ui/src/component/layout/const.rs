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

/// Bright solar-gold energy highlight color for the `Euv Studio` Emperor-Armor Dragon wings and eyes (`#FFF099`).
pub(crate) const SHELL_LOGO_SOLAR_HI: Color = Color::hex(0xFFF099);

/// Gleaming imperial radiant gold armor color for the `Euv Studio` dragon head, antlers, and chest (`#FFD24D`).
pub(crate) const SHELL_LOGO_GOLD_BRIGHT: Color = Color::hex(0xFFD24D);

/// Deep royal gold mid-tone color for sculpted armor plates and wing spars (`#EAB308`).
pub(crate) const SHELL_LOGO_GOLD_ROYAL: Color = Color::hex(0xEAB308);

/// Warm burnished amber-gold feather color framing the wide holographic wings on both Light and Dark surfaces (`#D97706`).
pub(crate) const SHELL_LOGO_WING_AMBER: Color = Color::hex(0xD97706);

/// Matte obsidian-black mechanical chassis color grounding the Emperor-Armor Dragon silhouette (`#14171F`).
pub(crate) const SHELL_LOGO_OBSIDIAN: Color = Color::hex(0x14171F);

/// Vivid crimson ruby color for the chest core crystal, shoulder gem, forehead jewel, and wingtip beacons (`#E6192E`).
pub(crate) const SHELL_LOGO_CRIMSON: Color = Color::hex(0xE6192E);

/// Luminous rose-crimson specular facet highlight on the dragon's ruby crystals (`#FF6B81`).
pub(crate) const SHELL_LOGO_CRIMSON_HI: Color = Color::hex(0xFF6B81);

/// Crisp shaded steel-silver metallic color for armored talon outlines and bevels on Light and Dark surfaces (`#94A3B8`).
pub(crate) const SHELL_LOGO_SILVER: Color = Color::hex(0x94A3B8);

/// Bright mirror-silver specular edge highlight on the dragon's splayed talons (`#F1F5F9`).
pub(crate) const SHELL_LOGO_SILVER_HI: Color = Color::hex(0xF1F5F9);

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
