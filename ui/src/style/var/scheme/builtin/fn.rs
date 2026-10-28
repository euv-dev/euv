use super::*;

// The built-in monochrome scheme.
//
// A hard-edged black-and-white system: square corners, one ink for every
// role, and shadows built from black alpha only. It is the reference the
// other schemes are measured against — every token below has a counterpart
// in each of them, so swapping the `theme-*` feature never leaves a role
// undefined.
vars! {
    pub c_scheme_builtin {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the built-in system is hard-edged, so every radius is square.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "0px";
        radius-sm: "0px";
        radius-md: "0px";
        radius-lg: "0px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "system-ui, -apple-system, sans-serif";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace";
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — 9px of vertical padding puts `space-sm` content in
        // the 36px box the CTA buttons already occupy: 9 + 24 line + 9 = 42.
        // Spelled as a name so the three families cannot drift apart again.
        // ═══════════════════════════════════════════════════════════════════════

        min-height-base: "36px";
        min-height-sm: "36px";
        control-row-pad: "9px";
        side-indent: "9px";
        side-indent-num: "9";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — a camera feed and the letterbox around a fullscreen
        // canvas are black, a drawing preview is white. They are pictures, not
        // chrome, so they do not follow the theme.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Colour Roles — one ink fills every role in the light half, and its
        // inverse fills every role in the dark half. The scheme is monochrome
        // by construction, so "error" is the same ink as "primary".
        // ═══════════════════════════════════════════════════════════════════════

        role-surface-light: "#ffffff";
        role-surface-dark: "#000000";
        role-surface-container-light: "#ffffff";
        role-surface-container-dark: "#000000";
        role-surface-container-high-light: "#ffffff";
        role-surface-container-high-dark: "#000000";
        role-on-surface-light: "#000000";
        role-on-surface-dark: "#ffffff";
        role-on-surface-variant-light: "#000000";
        role-on-surface-variant-dark: "#ffffff";
        role-outline-light: "#000000";
        role-outline-dark: "#ffffff";
        role-primary-light: "#000000";
        role-primary-dark: "#ffffff";
        role-primary-hover-light: "#000000";
        role-primary-hover-dark: "#ffffff";
        role-on-primary-light: "#ffffff";
        role-on-primary-dark: "#000000";
        role-primary-container-light: "#ffffff";
        role-primary-container-dark: "#000000";
        role-fill-subtle-light: "#ffffff";
        role-fill-subtle-dark: "#000000";
        role-scrim-light: "rgba(0, 0, 0, 0.20)";
        role-scrim-dark: "rgba(255, 255, 255, 0.08)";
        role-focus-light: "#000000";
        role-focus-dark: "#ffffff";
        role-error-light: "#000000";
        role-error-dark: "#ffffff";
        role-on-error-light: "#ffffff";
        role-on-error-dark: "#000000";
        role-error-container-light: "#ffffff";
        role-error-container-dark: "#000000";
        role-success-light: "#000000";
        role-success-dark: "#ffffff";
        role-success-container-light: "#ffffff";
        role-success-container-dark: "#000000";
        role-warning-light: "#000000";
        role-warning-dark: "#ffffff";
        role-warning-container-light: "#ffffff";
        role-warning-container-dark: "#000000";
        role-stroke-neutral-light: "rgba(0, 0, 0, 0.15)";
        role-stroke-neutral-strong-light: "rgba(0, 0, 0, 0.30)";
        role-stroke-neutral-strongest-light: "rgba(0, 0, 0, 0.45)";
        role-stroke-neutral-dark: "rgba(255, 255, 255, 0.18)";
        role-stroke-neutral-strong-dark: "rgba(255, 255, 255, 0.35)";
        role-stroke-neutral-strongest-dark: "rgba(255, 255, 255, 0.50)";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows — black alpha in light, white alpha in dark. Each role keeps
        // its own shape so the directional drawer and console shadows survive
        // the swap to another scheme.
        // ═══════════════════════════════════════════════════════════════════════

        role-shadow-control-light: "0 1px 3px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)";
        role-shadow-control-dark: "0 1px 3px rgba(255, 255, 255, 0.12), 0 1px 2px rgba(255, 255, 255, 0.06)";
        role-shadow-raised-light: "0 10px 15px -3px rgba(0, 0, 0, 0.12)";
        role-shadow-raised-dark: "0 10px 15px -3px rgba(255, 255, 255, 0.18)";
        role-shadow-modal-light: "0 25px 50px -12px rgba(0, 0, 0, 0.18)";
        role-shadow-modal-dark: "0 25px 50px -12px rgba(255, 255, 255, 0.25)";
        role-shadow-drawer-light: "4px 0 20px rgba(0, 0, 0, 0.08)";
        role-shadow-drawer-dark: "4px 0 20px rgba(255, 255, 255, 0.12)";
        role-shadow-console-button-light: "0 4px 14px rgba(0, 0, 0, 0.15)";
        role-shadow-console-button-dark: "0 4px 14px rgba(255, 255, 255, 0.15)";
        role-shadow-console-panel-light: "0 -8px 32px rgba(0, 0, 0, 0.06)";
        role-shadow-console-panel-dark: "0 -8px 32px rgba(255, 255, 255, 0.08)";
    }
}
