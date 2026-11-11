use super::*;

// The Ant Design scheme.
//
// Ant Design's own published palette and metrics: `colorPrimary #1677ff`,
// 6px control corners, 14px body type and a 32px control height. The dark
// half is the colour algorithm run over the same brand colour, so the primary
// stays blue in both modes while the surfaces step down through the neutral
// greys Ant Design publishes.
vars! {
    pub c_scheme_antdesign {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Ant Design rounds its controls to 6px and its surfaces to 8px.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "6px";
        radius-md: "6px";
        radius-lg: "8px";
        radius-xl: "8px";
        radius-2xl: "10px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — Ant Design publishes one soft overlay shadow, so every
        // raised step shares it and only the smallest control keeps a hairline.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px rgba(0, 0, 0, 0.02)";
        shadow-sm: "none";
        shadow-md: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        shadow-lg: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        shadow-xl: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "0px";
        ring-width: "3px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the Ant Design font stack and its pixel type ramp, which
        // keeps 12px for both small steps and 14px for the body.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
        font-family-mono: "SFMono-Regular, Consolas, 'Liberation Mono', Menlo, Courier, monospace";
        font-xs: "12px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "16px";
        font-lg: "16px";
        font-xl: "18px";
        font-2xl: "20px";
        font-3xl: "24px";
        font-4xl: "28px";
        font-5xl: "32px";
        font-6xl: "38px";
        font-weight-normal: "400";
        font-weight-medium: "400";
        font-weight-semibold: "400";
        font-weight-bold: "600";
        letter-spacing-tight: "0em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.02em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default control box is 32px, the compact one
        // 24px.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "32px";
        control-height-sm: "24px";
        control-row-pad: "4px";

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
        role-foreground-light: "rgba(0, 0, 0, 0.88)";
        role-card-light: "#ffffff";
        role-card-foreground-light: "rgba(0, 0, 0, 0.88)";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "rgba(0, 0, 0, 0.88)";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#1677ff";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#4096ff";
        role-secondary-light: "#f5f5f5";
        role-secondary-foreground-light: "rgba(0, 0, 0, 0.88)";
        role-muted-light: "#f5f5f5";
        role-muted-foreground-light: "rgba(0, 0, 0, 0.65)";
        role-accent-light: "#f5f5f5";
        role-accent-foreground-light: "rgba(0, 0, 0, 0.88)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#ff4d4f";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#52c41a";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#f6ffed";
        role-warning-light: "#faad14";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#fffbe6";
        role-error-container-light: "#fff2f0";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#d9d9d9";
        role-input-light: "#d9d9d9";
        role-ring-light: "#1677ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "rgba(0, 0, 0, 0.12)";
        role-scrollbar-thumb-hover-light: "rgba(0, 0, 0, 0.20)";
        role-scrollbar-thumb-active-light: "rgba(0, 0, 0, 0.30)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#141414";
        role-foreground-dark: "rgba(255, 255, 255, 0.85)";
        role-card-dark: "#1f1f1f";
        role-card-foreground-dark: "rgba(255, 255, 255, 0.85)";
        role-popover-dark: "#262626";
        role-popover-foreground-dark: "rgba(255, 255, 255, 0.85)";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#1677ff";
        role-primary-foreground-dark: "#ffffff";
        role-primary-hover-dark: "#4096ff";
        role-secondary-dark: "#1f1f1f";
        role-secondary-foreground-dark: "rgba(255, 255, 255, 0.85)";
        role-muted-dark: "#141414";
        role-muted-foreground-dark: "rgba(255, 255, 255, 0.65)";
        role-accent-dark: "#262626";
        role-accent-foreground-dark: "rgba(255, 255, 255, 0.85)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ff7875";
        role-destructive-foreground-dark: "#000000";
        role-success-dark: "#95de64";
        role-success-foreground-dark: "#000000";
        role-success-container-dark: "#111d0f";
        role-warning-dark: "#ffc069";
        role-warning-foreground-dark: "#000000";
        role-warning-container-dark: "#2b1d00";
        role-error-container-dark: "#2a1215";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#424242";
        role-input-dark: "#424242";
        role-ring-dark: "#1677ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "rgba(255, 255, 255, 0.18)";
        role-scrollbar-thumb-hover-dark: "rgba(255, 255, 255, 0.26)";
        role-scrollbar-thumb-active-dark: "rgba(255, 255, 255, 0.36)";
    }
}
