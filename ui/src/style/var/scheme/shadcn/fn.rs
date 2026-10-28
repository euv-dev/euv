use super::*;

// The shadcn/ui scheme.
//
// The reference the other schemes are measured against: the `new-york` style
// on the neutral base colour, which is what the shadcn CLI writes today.
// A card is a hairline-bordered, softly shadowed surface; a control is a
// 1px-bordered box with an 8px corner; the overlays float on the popover
// surface with their own elevation.
vars! {
    pub c_scheme_shadcn {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the radius ladder the reference publishes, derived from
        // `--radius: 0.625rem` by its own multiplicative scale.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "6px";
        radius-md: "8px";
        radius-lg: "10px";
        radius-xl: "14px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the shadow ramp the reference inherits from its CSS
        // framework, unchanged between the two modes.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px 0 rgb(0 0 0 / 0.05)";
        shadow-sm: "0 1px 3px 0 rgb(0 0 0 / 0.10), 0 1px 2px -1px rgb(0 0 0 / 0.10)";
        shadow-md: "0 4px 6px -1px rgb(0 0 0 / 0.10), 0 2px 4px -2px rgb(0 0 0 / 0.10)";
        shadow-lg: "0 10px 15px -3px rgb(0 0 0 / 0.10), 0 4px 6px -4px rgb(0 0 0 / 0.10)";
        shadow-xl: "0 20px 25px -5px rgb(0 0 0 / 0.10), 0 8px 10px -6px rgb(0 0 0 / 0.10)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "3px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the system sans stack and the framework's text ramp.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace";
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
        letter-spacing-tight: "-0.025em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.05em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default button is a 36px box.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "36px";
        control-height-sm: "32px";
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
        role-foreground-light: "#0a0a0a";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#0a0a0a";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#0a0a0a";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#171717";
        role-primary-foreground-light: "#fafafa";
        role-primary-hover-light: "rgba(23, 23, 23, 0.90)";
        role-secondary-light: "#f5f5f5";
        role-secondary-foreground-light: "#171717";
        role-muted-light: "#f5f5f5";
        role-muted-foreground-light: "#737373";
        role-accent-light: "#f5f5f5";
        role-accent-foreground-light: "#171717";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#dc2626";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#16a34a";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#dcfce7";
        role-warning-light: "#d97706";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#fef3c7";
        role-error-container-light: "#fee2e2";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#e5e5e5";
        role-input-light: "#e5e5e5";
        role-ring-light: "#a3a3a3";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#d4d4d4";
        role-scrollbar-thumb-hover-light: "#a3a3a3";
        role-scrollbar-thumb-active-light: "#737373";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#0a0a0a";
        role-foreground-dark: "#fafafa";
        role-card-dark: "#171717";
        role-card-foreground-dark: "#fafafa";
        role-popover-dark: "#171717";
        role-popover-foreground-dark: "#fafafa";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#e5e5e5";
        role-primary-foreground-dark: "#171717";
        role-primary-hover-dark: "rgba(229, 229, 229, 0.90)";
        role-secondary-dark: "#262626";
        role-secondary-foreground-dark: "#fafafa";
        role-muted-dark: "#262626";
        role-muted-foreground-dark: "#a3a3a3";
        role-accent-dark: "#262626";
        role-accent-foreground-dark: "#fafafa";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ef4444";
        role-destructive-foreground-dark: "#ffffff";
        role-success-dark: "#22c55e";
        role-success-foreground-dark: "#052e16";
        role-success-container-dark: "#052e16";
        role-warning-dark: "#f59e0b";
        role-warning-foreground-dark: "#451a03";
        role-warning-container-dark: "#451a03";
        role-error-container-dark: "#450a0a";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "rgba(255, 255, 255, 0.10)";
        role-input-dark: "rgba(255, 255, 255, 0.15)";
        role-ring-dark: "#737373";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#404040";
        role-scrollbar-thumb-hover-dark: "#525252";
        role-scrollbar-thumb-active-dark: "#737373";
    }
}
