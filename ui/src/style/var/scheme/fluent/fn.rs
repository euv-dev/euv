use super::*;

// The Fluent 2 scheme.
//
// Microsoft Fluent 2: `colorBrandBackground #0f6cbd` over the neutral
// background ramp, Segoe UI Variable, a 4px control radius and a 32px control
// height. The dark half is Fluent's own dark theme, where the neutral ramp
// steps down through `#292929`, `#1f1f1f` and `#141414` and the brand lifts to
// `#479ef5`.
vars! {
    pub c_scheme_fluent {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Fluent's small-corner ladder, from a 2px atom to an 8px card.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "4px";
        radius-lg: "6px";
        radius-xl: "8px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the two-part Fluent shadow, a blurred key light over a
        // tight ambient one.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        shadow-sm: "0 2px 4px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        shadow-md: "0 4px 8px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        shadow-lg: "0 8px 16px rgba(0, 0, 0, 0.14), 0 0 2px rgba(0, 0, 0, 0.12)";
        shadow-xl: "0 14px 28px rgba(0, 0, 0, 0.24), 0 0 8px rgba(0, 0, 0, 0.20)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "3px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — Segoe UI Variable over Fluent's size ramp.
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
        font-weight-normal: "400";
        font-weight-medium: "600";
        font-weight-semibold: "600";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.01em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.04em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — Fluent's default control is a 32px box, the small
        // one 24px.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "32px";
        control-height-sm: "24px";
        control-row-pad: "6px";

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
        role-foreground-light: "#242424";
        role-card-light: "#fafafa";
        role-card-foreground-light: "#242424";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#242424";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#0f6cbd";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#115ea3";
        role-secondary-light: "#f5f5f5";
        role-secondary-foreground-light: "#242424";
        role-muted-light: "#f5f5f5";
        role-muted-foreground-light: "#424242";
        role-accent-light: "#f5f5f5";
        role-accent-foreground-light: "#242424";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#c50f1f";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#107c10";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#dff6dd";
        role-warning-light: "#f7630c";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#fff4e4";
        role-error-container-light: "#fde7e9";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#d1d1d1";
        role-input-light: "#d1d1d1";
        role-ring-light: "#0f6cbd";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "rgba(0, 0, 0, 0.14)";
        role-scrollbar-thumb-hover-light: "rgba(0, 0, 0, 0.24)";
        role-scrollbar-thumb-active-light: "rgba(0, 0, 0, 0.36)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#292929";
        role-foreground-dark: "#ffffff";
        role-card-dark: "#1f1f1f";
        role-card-foreground-dark: "#ffffff";
        role-popover-dark: "#323232";
        role-popover-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#479ef5";
        role-primary-foreground-dark: "#000000";
        role-primary-hover-dark: "#62abf5";
        role-secondary-dark: "#323232";
        role-secondary-foreground-dark: "#ffffff";
        role-muted-dark: "#1f1f1f";
        role-muted-foreground-dark: "#d6d6d6";
        role-accent-dark: "#323232";
        role-accent-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ff99a4";
        role-destructive-foreground-dark: "#000000";
        role-success-dark: "#54b054";
        role-success-foreground-dark: "#000000";
        role-success-container-dark: "#0a2f14";
        role-warning-dark: "#f79e1b";
        role-warning-foreground-dark: "#000000";
        role-warning-container-dark: "#3a2500";
        role-error-container-dark: "#3a0a08";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#404040";
        role-input-dark: "#404040";
        role-ring-dark: "#479ef5";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "rgba(255, 255, 255, 0.20)";
        role-scrollbar-thumb-hover-dark: "rgba(255, 255, 255, 0.30)";
        role-scrollbar-thumb-active-dark: "rgba(255, 255, 255, 0.42)";
    }
}
