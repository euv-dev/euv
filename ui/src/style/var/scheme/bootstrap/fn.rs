use super::*;

// The Bootstrap 5.3 scheme.
//
// Bootstrap 5.3's own published tokens: `--bs-primary #0d6efd` over a white
// `--bs-body-bg`, a 6px control radius, 16px body type and a 38px control
// height. The dark half is Bootstrap's own `data-bs-theme="dark"` colour mode,
// where the brand lightens to `#4dabf7`, the body darkens to `#212529` and the
// status colours run on their own tinted ramp.
vars! {
    pub c_scheme_bootstrap {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Bootstrap's radius ladder, from a 4px atom to a 12px card.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "4px";
        radius-sm: "4px";
        radius-md: "6px";
        radius-lg: "8px";
        radius-xl: "12px";
        radius-2xl: "14px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the `--bs-box-shadow` ramp, unchanged between the modes.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)";
        shadow-sm: "0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)";
        shadow-md: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        shadow-lg: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)";
        shadow-xl: "0 1rem 3rem rgba(0, 0, 0, 0.175)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "3px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the system stack Bootstrap ships, over its own rem ramp.
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
        font-weight-normal: "400";
        font-weight-medium: "500";
        font-weight-semibold: "600";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.01em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.05em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default `.btn` is a 38px box, the small one 31px.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "38px";
        control-height-sm: "31px";
        control-row-pad: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — a camera feed and the letterbox around a fullscreen
        // canvas are black, a drawing preview is white. They are pictures, not
        // chrome, so they do not follow the theme.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#ffffff";
        role-foreground-light: "#212529";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#212529";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#212529";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#0d6efd";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#0d6efd";
        role-secondary-light: "#e9ecef";
        role-secondary-foreground-light: "#212529";
        role-muted-light: "#f8f9fa";
        role-muted-foreground-light: "#6c757d";
        role-accent-light: "#e9ecef";
        role-accent-foreground-light: "#212529";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#dc3545";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#198754";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#d1e7dd";
        role-warning-light: "#ffc107";
        role-warning-foreground-light: "#212529";
        role-warning-container-light: "#fff3cd";
        role-error-container-light: "#f8d7da";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#dee2e6";
        role-input-light: "#dee2e6";
        role-ring-light: "#0d6efd";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "rgba(0, 0, 0, 0.16)";
        role-scrollbar-thumb-hover-light: "rgba(0, 0, 0, 0.26)";
        role-scrollbar-thumb-active-light: "rgba(0, 0, 0, 0.38)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#212529";
        role-foreground-dark: "#dee2e6";
        role-card-dark: "#2b3035";
        role-card-foreground-dark: "#dee2e6";
        role-popover-dark: "#343a40";
        role-popover-foreground-dark: "#dee2e6";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#4dabf7";
        role-primary-foreground-dark: "#000000";
        role-primary-hover-dark: "#74c0fc";
        role-secondary-dark: "#343a40";
        role-secondary-foreground-dark: "#dee2e6";
        role-muted-dark: "#2b3035";
        role-muted-foreground-dark: "#adb5bd";
        role-accent-dark: "#343a40";
        role-accent-foreground-dark: "#dee2e6";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ff6b6b";
        role-destructive-foreground-dark: "#000000";
        role-success-dark: "#75e3a4";
        role-success-foreground-dark: "#000000";
        role-success-container-dark: "#0d2c1a";
        role-warning-dark: "#ffda6a";
        role-warning-foreground-dark: "#000000";
        role-warning-container-dark: "#3a2f0b";
        role-error-container-dark: "#3c1c1f";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#495057";
        role-input-dark: "#495057";
        role-ring-dark: "#4dabf7";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "rgba(255, 255, 255, 0.20)";
        role-scrollbar-thumb-hover-dark: "rgba(255, 255, 255, 0.28)";
        role-scrollbar-thumb-active-dark: "rgba(255, 255, 255, 0.38)";
    }
}
