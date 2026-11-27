/// Fallback single-character brand mark when the app brand title is empty.
pub(crate) const SHELL_DEFAULT_INITIAL: &str = "E";

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
