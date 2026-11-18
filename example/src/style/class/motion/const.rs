/// The example-only class that preserves component-owned motion.
pub(crate) const EXAMPLE_MOTION_POLICY_CLASS: &str = "c_example_motion_policy";

/// No theme, including the default build, disables component animations.
pub(crate) const EXAMPLE_MOTION_POLICY_STYLE: &str = "";

/// Consumer-owned motion and geometry for the headless custom feature.
#[cfg(feature = "theme-custom")]
pub(crate) const EXAMPLE_MOTION_POLICY_RULES: [(&str, &str); 6] = [
    (
        " .c_euv_logo_fab",
        "border-radius:var(--radius-md, 0px);transition:box-shadow var(--duration-fast, 150ms) ease-out;",
    ),
    (
        " .c_mobile_nav_drawer",
        "transition:transform var(--duration-normal, 250ms) ease-out,visibility var(--duration-normal, 250ms);",
    ),
    (
        " .c_mobile_overlay",
        "opacity:1;transition:opacity var(--duration-normal, 250ms),visibility var(--duration-normal, 250ms);",
    ),
    (" .c_mobile_overlay_hidden", "opacity:0;"),
    (
        " .c_vconsole_panel",
        "opacity:1;transition:opacity var(--duration-normal, 250ms),visibility var(--duration-normal, 250ms);",
    ),
    (" .c_vconsole_panel_closed", "opacity:0;"),
];

/// Styled themes retain their own motion and radius definitions.
#[cfg(not(feature = "theme-custom"))]
pub(crate) const EXAMPLE_MOTION_POLICY_RULES: [(&str, &str); 0] = [];
