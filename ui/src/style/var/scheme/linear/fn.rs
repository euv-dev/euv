use super::*;

// The Linear scheme.
//
// Linear's issue-tracking interface pairs an almost-black workspace with
// cool neutral surfaces, compact controls and a restrained indigo accent.
// The palette and metrics follow the Linear product-UI reference values;
// the remaining roles continue those same cool gray and status ramps.
// Hairline surface edges, rather than heavy elevation, carry the hierarchy.
vars! {
    pub c_scheme_linear {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Compact two-to-twelve-pixel corners keep the issue list
        // and command surfaces crisp instead of pill-shaped.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "6px";
        radius-lg: "8px";
        radius-xl: "12px";
        radius-2xl: "14px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — A nearly flat ramp reserves a soft shadow for floating
        // surfaces; the hairline border still does the separating in dark mode.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "none";
        shadow-sm: "0 1px 2px rgba(0, 0, 0, 0.03)";
        shadow-md: "0 2px 4px rgba(0, 0, 0, 0.04)";
        shadow-lg: "0 4px 12px rgba(0, 0, 0, 0.05)";
        shadow-xl: "0 8px 24px rgba(0, 0, 0, 0.06)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — One-pixel edges frame surfaces; an offset indigo
        // ring identifies keyboard focus without adding visual weight at rest.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — Inter and tight tracking reproduce the dense product
        // UI; the display steps stay smaller than the reference scheme.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "Inter, ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', monospace";
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
        letter-spacing-tight: "-0.02em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.04em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — The default control is 36px, the compact issue-list
        // control 30px; eight-pixel row padding preserves the list rhythm.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "36px";
        control-height-sm: "30px";
        control-row-pad: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — Camera and drawing pixels keep their own black or
        // white backing rather than picking up the cool workspace tint.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half. The light half uses a cool off-white canvas and
        // white cards, with the same near-black ink on floating surfaces.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#f7f8f8";
        role-foreground-light: "#191a1f";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#191a1f";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#191a1f";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. Indigo marks actions; the quiet fills
        // move along the cool gray ramp rather than borrowing brand saturation.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#555fbd";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#4a54ab";
        role-secondary-light: "#eef0f2";
        role-secondary-foreground-light: "#191a1f";
        role-muted-light: "#f0f1f3";
        role-muted-foreground-light: "#6a6c73";
        role-accent-light: "#e9ebef";
        role-accent-foreground-light: "#191a1f";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. Green, amber and coral retain their distinct
        // signals; deep ink keeps text readable on the brighter status fills.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#eb5757";
        role-destructive-foreground-light: "#191a1f";
        role-success-light: "#2f9e6b";
        role-success-foreground-light: "#071f14";
        role-success-container-light: "#e7f5ee";
        role-warning-light: "#f0b429";
        role-warning-foreground-light: "#35260a";
        role-warning-container-light: "#fff6db";
        role-error-container-light: "#fdeced";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. Pale decorative hairlines stay quiet;
        // the input edge is stronger so a control can be identified by its edge.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#e6e8eb";
        role-input-light: "#8a8f98";
        role-ring-light: "#5e6ad2";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half. Thumb states walk the same cool-gray ramp
        // from a quiet resting fill toward the muted text colour.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#c8cbd1";
        role-scrollbar-thumb-hover-light: "#a6abb4";
        role-scrollbar-thumb-active-light: "#6c6e75";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. The almost-black canvas and raised charcoal
        // card are separated by an edge, not a heavy drop shadow.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#08090a";
        role-foreground-dark: "#f7f8f8";
        role-card-dark: "#0f1011";
        role-card-foreground-dark: "#f7f8f8";
        role-popover-dark: "#0f1011";
        role-popover-foreground-dark: "#f7f8f8";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. Indigo remains stable across modes;
        // secondary and active fills are separate near-black gray steps.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#6c7af1";
        role-primary-foreground-dark: "#0b0c10";
        role-primary-hover-dark: "#7b87f4";
        role-secondary-dark: "#16171a";
        role-secondary-foreground-dark: "#f7f8f8";
        role-muted-dark: "#131417";
        role-muted-foreground-dark: "#8a8f98";
        role-accent-dark: "#1c1d21";
        role-accent-foreground-dark: "#f7f8f8";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. Brighter green and coral survive the near-black
        // canvas; the containers are low-luminance tints of those same hues.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#f2555a";
        role-destructive-foreground-dark: "#191a1f";
        role-success-dark: "#4cb782";
        role-success-foreground-dark: "#071f14";
        role-success-container-dark: "#10291f";
        role-warning-dark: "#f0b429";
        role-warning-foreground-dark: "#35260a";
        role-warning-container-dark: "#33280e";
        role-error-container-dark: "#32171c";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half. The decorative edge is a charcoal
        // hairline; control edges and focus rings are deliberately more visible.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#1f2023";
        role-input-dark: "#5d6068";
        role-ring-dark: "#7079d8";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half. The thumb brightens through cool charcoal
        // toward the workspace muted ink as hover and drag states engage.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#34363d";
        role-scrollbar-thumb-hover-dark: "#555962";
        role-scrollbar-thumb-active-dark: "#8a8f98";
    }
}
