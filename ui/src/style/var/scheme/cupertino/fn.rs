use super::*;

// The Cupertino scheme.
//
// Apple's Human Interface Guidelines: `systemBlue #007aff` on a
// `systemBackground` white page, 10-16px corners, 17px body type and the
// 44pt touch height. The dark half is the iOS dark palette, where the brand
// lifts to `#0a84ff`, the page drops to black and the system fills keep their
// translucency over the darker ground.
vars! {
    pub c_scheme_cupertino {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — HIG's continuous-corner ladder, from a 6px atom to a 16px
        // sheet.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "6px";
        radius-sm: "10px";
        radius-md: "10px";
        radius-lg: "12px";
        radius-xl: "16px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the soft, wide shadow ramp iOS gives floating surfaces.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-sm: "0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-md: "0 4px 16px rgba(0, 0, 0, 0.12)";
        shadow-lg: "0 12px 40px rgba(0, 0, 0, 0.18)";
        shadow-xl: "0 20px 60px rgba(0, 0, 0, 0.22)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "3px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the San Francisco stack, over the HIG point-size ramp.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "-apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Helvetica Neue', Helvetica, Arial, sans-serif";
        font-family-mono: "ui-monospace, 'SF Mono', Menlo, Consolas, monospace";
        font-xs: "11px";
        font-sm: "13px";
        font-base: "15px";
        font-md: "17px";
        font-lg: "17px";
        font-xl: "20px";
        font-2xl: "24px";
        font-3xl: "28px";
        font-4xl: "34px";
        font-5xl: "40px";
        font-6xl: "48px";
        font-weight-normal: "400";
        font-weight-medium: "500";
        font-weight-semibold: "600";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.02em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.04em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the 44pt minimum touch target, 28px for a compact
        // control.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "44px";
        control-height-sm: "28px";
        control-row-pad: "12px";

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

        role-primary-light: "#007aff";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#0071e8";
        role-secondary-light: "#f2f2f7";
        role-secondary-foreground-light: "#000000";
        role-muted-light: "#f2f2f7";
        role-muted-foreground-light: "rgba(60, 60, 67, 0.60)";
        role-accent-light: "rgba(120, 120, 128, 0.16)";
        role-accent-foreground-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#ff3b30";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#34c759";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#e3f9e7";
        role-warning-light: "#ff9500";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#fff2e0";
        role-error-container-light: "#ffe5e4";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "rgba(60, 60, 67, 0.29)";
        role-input-light: "rgba(60, 60, 67, 0.29)";
        role-ring-light: "#007aff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "rgba(60, 60, 67, 0.20)";
        role-scrollbar-thumb-hover-light: "rgba(60, 60, 67, 0.32)";
        role-scrollbar-thumb-active-light: "rgba(60, 60, 67, 0.45)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#000000";
        role-foreground-dark: "#ffffff";
        role-card-dark: "#1c1c1e";
        role-card-foreground-dark: "#ffffff";
        role-popover-dark: "#2c2c2e";
        role-popover-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#0a84ff";
        role-primary-foreground-dark: "#ffffff";
        role-primary-hover-dark: "#409cff";
        role-secondary-dark: "#1c1c1e";
        role-secondary-foreground-dark: "#ffffff";
        role-muted-dark: "#1c1c1e";
        role-muted-foreground-dark: "rgba(235, 235, 245, 0.60)";
        role-accent-dark: "rgba(120, 120, 128, 0.32)";
        role-accent-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ff453a";
        role-destructive-foreground-dark: "#ffffff";
        role-success-dark: "#30d158";
        role-success-foreground-dark: "#ffffff";
        role-success-container-dark: "#0a2f14";
        role-warning-dark: "#ff9f0a";
        role-warning-foreground-dark: "#ffffff";
        role-warning-container-dark: "#3a2500";
        role-error-container-dark: "#3a0a08";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "rgba(84, 84, 88, 0.65)";
        role-input-dark: "rgba(84, 84, 88, 0.65)";
        role-ring-dark: "#0a84ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "rgba(235, 235, 245, 0.24)";
        role-scrollbar-thumb-hover-dark: "rgba(235, 235, 245, 0.36)";
        role-scrollbar-thumb-active-dark: "rgba(235, 235, 245, 0.50)";
    }
}
