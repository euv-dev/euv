use super::*;

// The Material Design 3 scheme.
//
// The M3 baseline tokens: `primary #6750a4` over the `surface #fef7ff`
// baseline, the 4/8/12/16dp shape scale, Roboto and a 40dp control height.
// The dark half is M3's own dark scheme, where the primary inverts to
// `#d0bcff` and the surfaces step down through the container tones.
vars! {
    pub c_scheme_material {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the M3 shape scale, from a 4dp corner on the smallest atom
        // to a 16dp corner on a card.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "4px";
        radius-sm: "8px";
        radius-md: "12px";
        radius-lg: "12px";
        radius-xl: "16px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the M3 shadow ramp, where the key light is nearly opaque
        // black and the ambient light is a wide, faint blur.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px rgba(0, 0, 0, 0.30), 0 1px 3px 1px rgba(0, 0, 0, 0.15)";
        shadow-sm: "0 1px 2px rgba(0, 0, 0, 0.30), 0 1px 3px 1px rgba(0, 0, 0, 0.15)";
        shadow-md: "0 1px 2px rgba(0, 0, 0, 0.30), 0 2px 6px 2px rgba(0, 0, 0, 0.15)";
        shadow-lg: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.30)";
        shadow-xl: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.30)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        ring-width: "3px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — Roboto and the M3 type ramp in dp-sized pixels.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "Roboto, 'Helvetica Neue', Arial, sans-serif";
        font-family-mono: "'Roboto Mono', monospace";
        font-xs: "11px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "16px";
        font-lg: "16px";
        font-xl: "20px";
        font-2xl: "24px";
        font-3xl: "28px";
        font-4xl: "32px";
        font-5xl: "40px";
        font-6xl: "48px";
        font-weight-normal: "400";
        font-weight-medium: "500";
        font-weight-semibold: "500";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.01em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.05em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default control box is 40dp, the compact one
        // 32dp.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "40px";
        control-height-sm: "32px";
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

        role-background-light: "#fef7ff";
        role-foreground-light: "#1d1b20";
        role-card-light: "#f3edf7";
        role-card-foreground-light: "#1d1b20";
        role-popover-light: "#ece6f0";
        role-popover-foreground-light: "#1d1b20";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#6750a4";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#7f67be";
        role-secondary-light: "#e7e0ec";
        role-secondary-foreground-light: "#49454f";
        role-muted-light: "#f3edf7";
        role-muted-foreground-light: "#49454f";
        role-accent-light: "#e7e0ec";
        role-accent-foreground-light: "#49454f";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#b3261e";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#146c2e";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#c4eecd";
        role-warning-light: "#7d5700";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#ffdeab";
        role-error-container-light: "#f9dedc";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#cac4d0";
        role-input-light: "#cac4d0";
        role-ring-light: "#6750a4";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "rgba(0, 0, 0, 0.12)";
        role-scrollbar-thumb-hover-light: "rgba(0, 0, 0, 0.20)";
        role-scrollbar-thumb-active-light: "rgba(0, 0, 0, 0.30)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#141218";
        role-foreground-dark: "#e6e0e9";
        role-card-dark: "#211f26";
        role-card-foreground-dark: "#e6e0e9";
        role-popover-dark: "#2b2930";
        role-popover-foreground-dark: "#e6e0e9";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#d0bcff";
        role-primary-foreground-dark: "#381e72";
        role-primary-hover-dark: "#e9ddff";
        role-secondary-dark: "#cac4d0";
        role-secondary-foreground-dark: "#49454f";
        role-muted-dark: "#211f26";
        role-muted-foreground-dark: "#cac4d0";
        role-accent-dark: "#2b2930";
        role-accent-foreground-dark: "#e6e0e9";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#f2b8b5";
        role-destructive-foreground-dark: "#601410";
        role-success-dark: "#6dd58f";
        role-success-foreground-dark: "#0a2f14";
        role-success-container-dark: "#0c5226";
        role-warning-dark: "#f5bd4f";
        role-warning-foreground-dark: "#422800";
        role-warning-container-dark: "#5a4200";
        role-error-container-dark: "#8c1d18";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#49454f";
        role-input-dark: "#49454f";
        role-ring-dark: "#d0bcff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "rgba(255, 255, 255, 0.18)";
        role-scrollbar-thumb-hover-dark: "rgba(255, 255, 255, 0.26)";
        role-scrollbar-thumb-active-dark: "rgba(255, 255, 255, 0.36)";
    }
}
