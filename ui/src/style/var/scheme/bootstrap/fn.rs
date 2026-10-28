use super::*;

// The Bootstrap 5.3 scheme.
//
// The most widely deployed CSS framework: a 6px control radius, 16px body
// type, a 38px control height and a blue brand on a white page. Every role
// below has a counterpart in the other schemes, so the registry never reads an
// undefined token.
vars! {
    pub c_scheme_bootstrap {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Bootstrap rounds its controls to 6px and its cards to 12px.
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "4px";
        radius-sm: "6px";
        radius-md: "8px";
        radius-lg: "12px";
        radius-full: "50%";
        // ═══════════════════════════════════════════════════════════════════════
        // Typography
        // ═══════════════════════════════════════════════════════════════════════
        font-family: "system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', 'Noto Sans', 'Liberation Sans', Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
        font-family-mono: "SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace";
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.25rem";
        font-xl: "1.5rem";
        font-2xl: "1.75rem";
        font-3xl: "2rem";
        font-4xl: "2.5rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";
        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — 9px of vertical padding puts `space-sm` content in
        // the 36px box the CTA buttons already occupy: 9 + 24 line + 9 = 42.
        // Spelled as a name so the three families cannot drift apart again.
        // ═══════════════════════════════════════════════════════════════════════
        min-height-base: "38px";
        min-height-sm: "31px";
        control-row-pad: "8px";
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
        role-surface-dark: "#212529";
        role-surface-container-light: "#ffffff";
        role-surface-container-dark: "#2b3035";
        role-surface-container-high-light: "#ffffff";
        role-surface-container-high-dark: "#343a40";
        role-on-surface-light: "#212529";
        role-on-surface-dark: "#dee2e6";
        role-on-surface-variant-light: "#6c757d";
        role-on-surface-variant-dark: "#adb5bd";
        role-outline-light: "#dee2e6";
        role-outline-dark: "#495057";
        role-primary-light: "#0d6efd";
        role-primary-dark: "#4dabf7";
        role-primary-hover-light: "#0d6efd";
        role-primary-hover-dark: "#74c0fc";
        role-on-primary-light: "#ffffff";
        role-on-primary-dark: "#000000";
        role-primary-container-light: "#cfe2ff";
        role-primary-container-dark: "#031633";
        role-fill-subtle-light: "#e9ecef";
        role-fill-subtle-dark: "#343a40";
        role-scrim-light: "rgba(0, 0, 0, 0.50)";
        role-scrim-dark: "rgba(0, 0, 0, 0.75)";
        role-focus-light: "#0d6efd";
        role-focus-dark: "#4dabf7";
        role-error-light: "#dc3545";
        role-error-dark: "#ff6b6b";
        role-on-error-light: "#ffffff";
        role-on-error-dark: "#000000";
        role-error-container-light: "#f8d7da";
        role-error-container-dark: "#3c1c1f";
        role-success-light: "#198754";
        role-success-dark: "#75e3a4";
        role-success-container-light: "#d1e7dd";
        role-success-container-dark: "#0d2c1a";
        role-warning-light: "#ffc107";
        role-warning-dark: "#ffda6a";
        role-warning-container-light: "#fff3cd";
        role-warning-container-dark: "#3a2f0b";
        role-stroke-neutral-light: "rgba(0, 0, 0, 0.16)";
        role-stroke-neutral-strong-light: "rgba(0, 0, 0, 0.26)";
        role-stroke-neutral-strongest-light: "rgba(0, 0, 0, 0.38)";
        role-stroke-neutral-dark: "rgba(255, 255, 255, 0.20)";
        role-stroke-neutral-strong-dark: "rgba(255, 255, 255, 0.28)";
        role-stroke-neutral-strongest-dark: "rgba(255, 255, 255, 0.38)";
        // ═══════════════════════════════════════════════════════════════════════
        // Shadows — black alpha in light, white alpha in dark. Each role keeps
        // its own shape so the directional drawer and console shadows survive
        // the swap to another scheme.
        // ═══════════════════════════════════════════════════════════════════════
        role-shadow-control-light: "0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)";
        role-shadow-control-dark: "0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)";
        role-shadow-raised-light: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-raised-dark: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-modal-light: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-modal-dark: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-drawer-light: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-drawer-dark: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-console-button-light: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-console-button-dark: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-console-panel-light: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        role-shadow-console-panel-dark: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
    }
}
