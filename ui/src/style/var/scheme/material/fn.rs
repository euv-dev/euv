use super::*;

// The Material Design 3 scheme.
//
// Google's tonal system: a violet seed colour, generous 4dp-28dp shape
// scale, Roboto and a 40px control height. Every role below has a
// counterpart in the other schemes, so the registry never reads an undefined
// token.
vars! {
    pub c_scheme_material {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Material 3 runs a generous 4dp to 16dp corner scale.
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "4px";
        radius-sm: "8px";
        radius-md: "12px";
        radius-lg: "16px";
        radius-full: "50%";
        // ═══════════════════════════════════════════════════════════════════════
        // Typography
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
        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — 9px of vertical padding puts `space-sm` content in
        // the 36px box the CTA buttons already occupy: 9 + 24 line + 9 = 42.
        // Spelled as a name so the three families cannot drift apart again.
        // ═══════════════════════════════════════════════════════════════════════
        min-height-base: "40px";
        min-height-sm: "32px";
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
        role-surface-light: "#fef7ff";
        role-surface-dark: "#141218";
        role-surface-container-light: "#f3edf7";
        role-surface-container-dark: "#211f26";
        role-surface-container-high-light: "#ece6f0";
        role-surface-container-high-dark: "#2b2930";
        role-on-surface-light: "#1d1b20";
        role-on-surface-dark: "#e6e0e9";
        role-on-surface-variant-light: "#49454f";
        role-on-surface-variant-dark: "#cac4d0";
        role-outline-light: "#79747e";
        role-outline-dark: "#938f99";
        role-primary-light: "#6750a4";
        role-primary-dark: "#d0bcff";
        role-primary-hover-light: "#7f67be";
        role-primary-hover-dark: "#e9ddff";
        role-on-primary-light: "#ffffff";
        role-on-primary-dark: "#381e72";
        role-primary-container-light: "#eaddff";
        role-primary-container-dark: "#4f378b";
        role-fill-subtle-light: "#ece6f0";
        role-fill-subtle-dark: "#2b2930";
        role-scrim-light: "rgba(0, 0, 0, 0.32)";
        role-scrim-dark: "rgba(0, 0, 0, 0.55)";
        role-focus-light: "#6750a4";
        role-focus-dark: "#d0bcff";
        role-error-light: "#b3261e";
        role-error-dark: "#f2b8b5";
        role-on-error-light: "#ffffff";
        role-on-error-dark: "#601410";
        role-error-container-light: "#f9dedc";
        role-error-container-dark: "#8c1d18";
        role-success-light: "#146c2e";
        role-success-dark: "#6dd58f";
        role-success-container-light: "#c4eecd";
        role-success-container-dark: "#0c5226";
        role-warning-light: "#7d5700";
        role-warning-dark: "#f5bd4f";
        role-warning-container-light: "#ffdeab";
        role-warning-container-dark: "#5a4200";
        role-stroke-neutral-light: "rgba(0, 0, 0, 0.12)";
        role-stroke-neutral-strong-light: "rgba(0, 0, 0, 0.20)";
        role-stroke-neutral-strongest-light: "rgba(0, 0, 0, 0.30)";
        role-stroke-neutral-dark: "rgba(255, 255, 255, 0.18)";
        role-stroke-neutral-strong-dark: "rgba(255, 255, 255, 0.26)";
        role-stroke-neutral-strongest-dark: "rgba(255, 255, 255, 0.36)";
        // ═══════════════════════════════════════════════════════════════════════
        // Shadows — black alpha in light, white alpha in dark. Each role keeps
        // its own shape so the directional drawer and console shadows survive
        // the swap to another scheme.
        // ═══════════════════════════════════════════════════════════════════════
        role-shadow-control-light: "0 1px 2px rgba(0, 0, 0, 0.3), 0 1px 3px 1px rgba(0, 0, 0, 0.15)";
        role-shadow-control-dark: "0 1px 2px rgba(0, 0, 0, 0.3), 0 1px 3px 1px rgba(0, 0, 0, 0.15)";
        role-shadow-raised-light: "0 1px 2px rgba(0, 0, 0, 0.3), 0 2px 6px 2px rgba(0, 0, 0, 0.15)";
        role-shadow-raised-dark: "0 1px 2px rgba(0, 0, 0, 0.3), 0 2px 6px 2px rgba(0, 0, 0, 0.15)";
        role-shadow-modal-light: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3)";
        role-shadow-modal-dark: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3)";
        role-shadow-drawer-light: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3)";
        role-shadow-drawer-dark: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3)";
        role-shadow-console-button-light: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3)";
        role-shadow-console-button-dark: "0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3)";
        role-shadow-console-panel-light: "0 1px 2px rgba(0, 0, 0, 0.3), 0 2px 6px 2px rgba(0, 0, 0, 0.15)";
        role-shadow-console-panel-dark: "0 1px 2px rgba(0, 0, 0, 0.3), 0 2px 6px 2px rgba(0, 0, 0, 0.15)";
    }
}
