use super::*;

// The Cupertino scheme.
//
// Apple's Human Interface Guidelines: generous 10px-16px corners, 17px body
// type, a 44pt control height and translucent system fills. Every role below
// has a counterpart in the other schemes, so the registry never reads an
// undefined token.
vars! {
    pub c_scheme_cupertino {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Cupertino uses generous 10px to 16px corners throughout.
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "6px";
        radius-sm: "10px";
        radius-md: "12px";
        radius-lg: "16px";
        radius-full: "50%";
        // ═══════════════════════════════════════════════════════════════════════
        // Typography
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
        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — 9px of vertical padding puts `space-sm` content in
        // the 36px box the CTA buttons already occupy: 9 + 24 line + 9 = 42.
        // Spelled as a name so the three families cannot drift apart again.
        // ═══════════════════════════════════════════════════════════════════════
        min-height-base: "44px";
        min-height-sm: "28px";
        control-row-pad: "12px";
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
        role-surface-dark: "#000000";
        role-surface-container-light: "#ffffff";
        role-surface-container-dark: "#1c1c1e";
        role-surface-container-high-light: "#ffffff";
        role-surface-container-high-dark: "#2c2c2e";
        role-on-surface-light: "#000000";
        role-on-surface-dark: "#ffffff";
        role-on-surface-variant-light: "rgba(60, 60, 67, 0.60)";
        role-on-surface-variant-dark: "rgba(235, 235, 245, 0.60)";
        role-outline-light: "rgba(60, 60, 67, 0.29)";
        role-outline-dark: "rgba(84, 84, 88, 0.65)";
        role-primary-light: "#007aff";
        role-primary-dark: "#0a84ff";
        role-primary-hover-light: "#0071e8";
        role-primary-hover-dark: "#409cff";
        role-on-primary-light: "#ffffff";
        role-on-primary-dark: "#ffffff";
        role-primary-container-light: "rgba(0, 122, 255, 0.12)";
        role-primary-container-dark: "rgba(10, 132, 255, 0.20)";
        role-fill-subtle-light: "rgba(120, 120, 128, 0.16)";
        role-fill-subtle-dark: "rgba(120, 120, 128, 0.32)";
        role-scrim-light: "rgba(0, 0, 0, 0.40)";
        role-scrim-dark: "rgba(0, 0, 0, 0.60)";
        role-focus-light: "#007aff";
        role-focus-dark: "#0a84ff";
        role-error-light: "#ff3b30";
        role-error-dark: "#ff453a";
        role-on-error-light: "#ffffff";
        role-on-error-dark: "#ffffff";
        role-error-container-light: "#ffe5e4";
        role-error-container-dark: "#3a0a08";
        role-success-light: "#34c759";
        role-success-dark: "#30d158";
        role-success-container-light: "#e3f9e7";
        role-success-container-dark: "#0a2f14";
        role-warning-light: "#ff9500";
        role-warning-dark: "#ff9f0a";
        role-warning-container-light: "#fff2e0";
        role-warning-container-dark: "#3a2500";
        role-stroke-neutral-light: "rgba(60, 60, 67, 0.20)";
        role-stroke-neutral-strong-light: "rgba(60, 60, 67, 0.32)";
        role-stroke-neutral-strongest-light: "rgba(60, 60, 67, 0.45)";
        role-stroke-neutral-dark: "rgba(235, 235, 245, 0.24)";
        role-stroke-neutral-strong-dark: "rgba(235, 235, 245, 0.36)";
        role-stroke-neutral-strongest-dark: "rgba(235, 235, 245, 0.50)";
        // ═══════════════════════════════════════════════════════════════════════
        // Shadows — black alpha in light, white alpha in dark. Each role keeps
        // its own shape so the directional drawer and console shadows survive
        // the swap to another scheme.
        // ═══════════════════════════════════════════════════════════════════════
        role-shadow-control-light: "0 1px 2px rgba(0, 0, 0, 0.04)";
        role-shadow-control-dark: "0 1px 2px rgba(0, 0, 0, 0.40)";
        role-shadow-raised-light: "0 4px 16px rgba(0, 0, 0, 0.12)";
        role-shadow-raised-dark: "0 4px 16px rgba(0, 0, 0, 0.40)";
        role-shadow-modal-light: "0 12px 40px rgba(0, 0, 0, 0.18)";
        role-shadow-modal-dark: "0 12px 40px rgba(0, 0, 0, 0.50)";
        role-shadow-drawer-light: "-4px 0 24px rgba(0, 0, 0, 0.12)";
        role-shadow-drawer-dark: "-4px 0 24px rgba(0, 0, 0, 0.40)";
        role-shadow-console-button-light: "0 4px 16px rgba(0, 0, 0, 0.12)";
        role-shadow-console-button-dark: "0 4px 16px rgba(0, 0, 0, 0.40)";
        role-shadow-console-panel-light: "0 -4px 20px rgba(0, 0, 0, 0.10)";
        role-shadow-console-panel-dark: "0 -4px 20px rgba(0, 0, 0, 0.30)";
    }
}
