use super::*;

// The Ant Design scheme.
//
// A dense enterprise system: a blue brand, 6px control corners, 14px body
// type and a 32px control height. Every role below has a counterpart in the
// other schemes, so the registry never reads an undefined token.
vars! {
    pub c_scheme_antdesign {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Ant Design rounds its controls to 6px and its surfaces to 8px.
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "2px";
        radius-sm: "6px";
        radius-md: "8px";
        radius-lg: "8px";
        radius-full: "50%";
        // ═══════════════════════════════════════════════════════════════════════
        // Typography
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
        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — 9px of vertical padding puts `space-sm` content in
        // the 36px box the CTA buttons already occupy: 9 + 24 line + 9 = 42.
        // Spelled as a name so the three families cannot drift apart again.
        // ═══════════════════════════════════════════════════════════════════════
        min-height-base: "32px";
        min-height-sm: "24px";
        control-row-pad: "4px";
        side-indent: "12px";
        side-indent-num: "12";
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
        role-surface-dark: "#141414";
        role-surface-container-light: "#ffffff";
        role-surface-container-dark: "#1f1f1f";
        role-surface-container-high-light: "#ffffff";
        role-surface-container-high-dark: "#262626";
        role-on-surface-light: "rgba(0, 0, 0, 0.88)";
        role-on-surface-dark: "rgba(255, 255, 255, 0.85)";
        role-on-surface-variant-light: "rgba(0, 0, 0, 0.65)";
        role-on-surface-variant-dark: "rgba(255, 255, 255, 0.65)";
        role-outline-light: "#d9d9d9";
        role-outline-dark: "#424242";
        role-primary-light: "#1677ff";
        role-primary-dark: "#1677ff";
        role-primary-hover-light: "#4096ff";
        role-primary-hover-dark: "#4096ff";
        role-on-primary-light: "#ffffff";
        role-on-primary-dark: "#ffffff";
        role-primary-container-light: "#e6f4ff";
        role-primary-container-dark: "#111a2c";
        role-fill-subtle-light: "rgba(0, 0, 0, 0.06)";
        role-fill-subtle-dark: "rgba(255, 255, 255, 0.08)";
        role-scrim-light: "rgba(0, 0, 0, 0.45)";
        role-scrim-dark: "rgba(0, 0, 0, 0.65)";
        role-focus-light: "#1677ff";
        role-focus-dark: "#1677ff";
        role-error-light: "#ff4d4f";
        role-error-dark: "#ff7875";
        role-on-error-light: "#ffffff";
        role-on-error-dark: "#000000";
        role-error-container-light: "#fff2f0";
        role-error-container-dark: "#2a1215";
        role-success-light: "#52c41a";
        role-success-dark: "#95de64";
        role-success-container-light: "#f6ffed";
        role-success-container-dark: "#111d0f";
        role-warning-light: "#faad14";
        role-warning-dark: "#ffc069";
        role-warning-container-light: "#fffbe6";
        role-warning-container-dark: "#2b1d00";
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
        role-shadow-control-light: "0 1px 2px rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px rgba(0, 0, 0, 0.02)";
        role-shadow-control-dark: "0 1px 2px rgba(0, 0, 0, 0.3), 0 1px 6px -1px rgba(0, 0, 0, 0.2)";
        role-shadow-raised-light: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        role-shadow-raised-dark: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        role-shadow-modal-light: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        role-shadow-modal-dark: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        role-shadow-drawer-light: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        role-shadow-drawer-dark: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)";
        role-shadow-console-button-light: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12)";
        role-shadow-console-button-dark: "0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12)";
        role-shadow-console-panel-light: "0 -8px 32px rgba(0, 0, 0, 0.06)";
        role-shadow-console-panel-dark: "0 -8px 32px rgba(0, 0, 0, 0.08)";
    }
}
