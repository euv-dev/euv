use super::*;

// The Microsoft Fluent 2 scheme.
//
// Office-grade neutrals under a single communicating blue: 4px control
// corners, 14px body type, a 32px control height and the eight-step Fluent
// shadow ramp. Every role below has a counterpart in the other schemes, so
// the registry never reads an undefined token.
vars! {
    pub c_scheme_fluent {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Fluent 2 keeps the sharpest corners of the six, 4px on controls.
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "6px";
        radius-lg: "8px";
        radius-full: "50%";
        // ═══════════════════════════════════════════════════════════════════════
        // Typography
        // ═══════════════════════════════════════════════════════════════════════
        font-family: "'Segoe UI Variable', 'Segoe UI', system-ui, -apple-system, sans-serif";
        font-family-mono: "Consolas, 'Cascadia Code', 'Courier New', monospace";
        font-xs: "10px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "16px";
        font-lg: "18px";
        font-xl: "20px";
        font-2xl: "24px";
        font-3xl: "28px";
        font-4xl: "32px";
        font-5xl: "40px";
        font-6xl: "48px";
        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — 9px of vertical padding puts `space-sm` content in
        // the 36px box the CTA buttons already occupy: 9 + 24 line + 9 = 42.
        // Spelled as a name so the three families cannot drift apart again.
        // ═══════════════════════════════════════════════════════════════════════
        min-height-base: "32px";
        min-height-sm: "24px";
        control-row-pad: "6px";
        side-indent: "16px";
        side-indent-num: "16";
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
        role-surface-dark: "#292929";
        role-surface-container-light: "#f5f5f5";
        role-surface-container-dark: "#1f1f1f";
        role-surface-container-high-light: "#fafafa";
        role-surface-container-high-dark: "#323232";
        role-on-surface-light: "#242424";
        role-on-surface-dark: "#ffffff";
        role-on-surface-variant-light: "#424242";
        role-on-surface-variant-dark: "#d6d6d6";
        role-outline-light: "#d1d1d1";
        role-outline-dark: "#404040";
        role-primary-light: "#0f6cbd";
        role-primary-dark: "#479ef5";
        role-primary-hover-light: "#115ea3";
        role-primary-hover-dark: "#62abf5";
        role-on-primary-light: "#ffffff";
        role-on-primary-dark: "#000000";
        role-primary-container-light: "#ebf3fc";
        role-primary-container-dark: "rgba(71, 158, 245, 0.20)";
        role-fill-subtle-light: "#f5f5f5";
        role-fill-subtle-dark: "#323232";
        role-scrim-light: "rgba(0, 0, 0, 0.40)";
        role-scrim-dark: "rgba(0, 0, 0, 0.60)";
        role-focus-light: "#0f6cbd";
        role-focus-dark: "#479ef5";
        role-error-light: "#c50f1f";
        role-error-dark: "#ff99a4";
        role-on-error-light: "#ffffff";
        role-on-error-dark: "#000000";
        role-error-container-light: "#fde7e9";
        role-error-container-dark: "#3a0a08";
        role-success-light: "#107c10";
        role-success-dark: "#54b054";
        role-success-container-light: "#dff6dd";
        role-success-container-dark: "#0a2f14";
        role-warning-light: "#f7630c";
        role-warning-dark: "#f79e1b";
        role-warning-container-light: "#fff4e4";
        role-warning-container-dark: "#3a2500";
        role-stroke-neutral-light: "rgba(0, 0, 0, 0.14)";
        role-stroke-neutral-strong-light: "rgba(0, 0, 0, 0.24)";
        role-stroke-neutral-strongest-light: "rgba(0, 0, 0, 0.36)";
        role-stroke-neutral-dark: "rgba(255, 255, 255, 0.20)";
        role-stroke-neutral-strong-dark: "rgba(255, 255, 255, 0.30)";
        role-stroke-neutral-strongest-dark: "rgba(255, 255, 255, 0.42)";
        // ═══════════════════════════════════════════════════════════════════════
        // Shadows — black alpha in light, white alpha in dark. Each role keeps
        // its own shape so the directional drawer and console shadows survive
        // the swap to another scheme.
        // ═══════════════════════════════════════════════════════════════════════
        role-shadow-control-light: "0 1px 2px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-control-dark: "0 1px 2px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-raised-light: "0 2px 4px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-raised-dark: "0 2px 4px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-modal-light: "0 14px 28px rgba(0, 0, 0, 0.24), 0 0 8px rgba(0, 0, 0, 0.20)";
        role-shadow-modal-dark: "0 14px 28px rgba(0, 0, 0, 0.24), 0 0 8px rgba(0, 0, 0, 0.20)";
        role-shadow-drawer-light: "0 8px 16px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-drawer-dark: "0 8px 16px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-console-button-light: "0 8px 16px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-console-button-dark: "0 8px 16px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-console-panel-light: "0 4px 8px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        role-shadow-console-panel-dark: "0 4px 8px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
    }
}
