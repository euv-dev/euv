use super::*;

// The monochrome scheme.
//
// The built-in black-and-white system: square corners, one ink for every
// role and shadows built from black alpha only. Because the scheme has no
// colour of its own, the status roles reuse the same ink as the brand — the
// difference is carried by the border and by the surrounding surface.
vars! {
    pub c_scheme_monochrome {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the system is hard-edged, so every radius but the pill is
        // square.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "0px";
        radius-sm: "0px";
        radius-md: "0px";
        radius-lg: "0px";
        radius-xl: "0px";
        radius-2xl: "0px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — a ramp of pure black alpha, widening and deepening with
        // the height of the surface it sits under.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "none";
        shadow-sm: "0 1px 2px 0 rgba(0, 0, 0, 0.08)";
        shadow-md: "0 4px 8px 0 rgba(0, 0, 0, 0.10)";
        shadow-lg: "0 12px 24px 0 rgba(0, 0, 0, 0.12)";
        shadow-xl: "0 24px 48px 0 rgba(0, 0, 0, 0.14)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the system sans stack and a rem-based text ramp.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "system-ui, -apple-system, sans-serif";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace";
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.25rem";
        font-xl: "1.5rem";
        font-2xl: "1.875rem";
        font-3xl: "2.25rem";
        font-4xl: "3rem";
        font-5xl: "3.75rem";
        font-6xl: "4.5rem";
        font-weight-normal: "400";
        font-weight-medium: "500";
        font-weight-semibold: "600";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.02em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.08em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default control box is 36px, the compact one
        // 28px.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "36px";
        control-height-sm: "28px";
        control-row-pad: "9px";

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
        role-foreground-light: "#000000";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#000000";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#000000";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "rgba(0, 0, 0, 0.85)";
        role-secondary-light: "#ffffff";
        role-secondary-foreground-light: "#000000";
        role-muted-light: "#ffffff";
        role-muted-foreground-light: "#000000";
        role-accent-light: "#ffffff";
        role-accent-foreground-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. One ink fills every status, so the label
        // and the border carry the meaning.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#000000";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#000000";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#ffffff";
        role-warning-light: "#000000";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#ffffff";
        role-error-container-light: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#000000";
        role-input-light: "#000000";
        role-ring-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "rgba(0, 0, 0, 0.25)";
        role-scrollbar-thumb-hover-light: "rgba(0, 0, 0, 0.40)";
        role-scrollbar-thumb-active-light: "rgba(0, 0, 0, 0.55)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#000000";
        role-foreground-dark: "#ffffff";
        role-card-dark: "#000000";
        role-card-foreground-dark: "#ffffff";
        role-popover-dark: "#000000";
        role-popover-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#ffffff";
        role-primary-foreground-dark: "#000000";
        role-primary-hover-dark: "rgba(255, 255, 255, 0.85)";
        role-secondary-dark: "#000000";
        role-secondary-foreground-dark: "#ffffff";
        role-muted-dark: "#000000";
        role-muted-foreground-dark: "#ffffff";
        role-accent-dark: "#000000";
        role-accent-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ffffff";
        role-destructive-foreground-dark: "#000000";
        role-success-dark: "#ffffff";
        role-success-foreground-dark: "#000000";
        role-success-container-dark: "#000000";
        role-warning-dark: "#ffffff";
        role-warning-foreground-dark: "#000000";
        role-warning-container-dark: "#000000";
        role-error-container-dark: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#ffffff";
        role-input-dark: "#ffffff";
        role-ring-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "rgba(255, 255, 255, 0.25)";
        role-scrollbar-thumb-hover-dark: "rgba(255, 255, 255, 0.40)";
        role-scrollbar-thumb-active-dark: "rgba(255, 255, 255, 0.55)";
    }
}
