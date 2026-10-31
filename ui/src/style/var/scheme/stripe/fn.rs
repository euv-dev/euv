use super::*;

// The Stripe scheme.
//
// Stripe's interface uses pale blue-gray space, navy ink and a vivid
// indigo action colour rather than a neutral black-and-white canvas.
// The palette follows the Stripe product-UI reference values, with
// related navy and status tints filling the semantic roles.
// Generous corners and layered navy-tinted shadows distinguish its surfaces.
vars! {
    pub c_scheme_stripe {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — The two-to-sixteen-pixel ladder gives payment forms
        // crisp small details and cards a more generous outer corner.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "8px";
        radius-lg: "12px";
        radius-xl: "16px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — Layered shadows use Stripe navy ink, rgba(50, 50, 93,
        // ...), rather than the neutral black ramp of the reference scheme.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px rgba(50, 50, 93, 0.04)";
        shadow-sm: "0 2px 5px rgba(50, 50, 93, 0.06), 0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-md: "0 4px 8px rgba(50, 50, 93, 0.10), 0 2px 4px rgba(0, 0, 0, 0.06)";
        shadow-lg: "0 13px 27px -5px rgba(50, 50, 93, 0.15), 0 8px 16px -8px rgba(0, 0, 0, 0.10)";
        shadow-xl: "0 30px 60px -12px rgba(50, 50, 93, 0.20), 0 18px 36px -18px rgba(0, 0, 0, 0.15)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — A one-pixel edge contains forms, with an offset
        // two-pixel indigo ring reserved for focus.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — The native sans stack keeps dashboard text familiar;
        // tight display tracking and a full display ramp support larger surfaces.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', monospace";
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
        letter-spacing-wide: "0.04em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — Forty-pixel controls and ten-pixel row padding
        // leave more breathing room around financial forms and table actions.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "40px";
        control-height-sm: "32px";
        control-row-pad: "10px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — Camera frames remain black and drawing previews
        // white; neither inherits the blue-gray dashboard chrome.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half. The signature pale blue-gray canvas surrounds
        // white cards and popovers, all carrying the same navy text.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#f6f9fc";
        role-foreground-light: "#1a1f36";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#1a1f36";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#1a1f36";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. Vivid indigo is reserved for actions;
        // secondary, muted and active surfaces continue the blue-gray palette.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#5b54eb";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#4a43d6";
        role-secondary-light: "#eef2f7";
        role-secondary-foreground-light: "#1a1f36";
        role-muted-light: "#f0f4f8";
        role-muted-foreground-light: "#626b7d";
        role-accent-light: "#e8edf5";
        role-accent-foreground-light: "#1a1f36";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. Emerald success, amber warnings and ruby errors
        // sit on pale related tints instead of a generic neutral container.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#df1b41";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#24b47e";
        role-success-foreground-light: "#081a2c";
        role-success-container-light: "#e3f8ef";
        role-warning-light: "#f5a623";
        role-warning-foreground-light: "#3d2b0b";
        role-warning-container-light: "#fff4dc";
        role-error-container-light: "#ffe7ed";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The pale blue decorative edge stays
        // subtle; input borders use a darker blue-gray to identify form controls.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#e3e9f0";
        role-input-light: "#7f8fa2";
        role-ring-light: "#635bff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half. The thumb moves down the blue-gray ramp
        // toward the muted navy ink during hover and drag.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#c1ccda";
        role-scrollbar-thumb-hover-light: "#98a7bb";
        role-scrollbar-thumb-active-light: "#697386";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. Deep navy is the page, not black; brighter navy
        // cards and popovers retain that blue identity.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#0a2540";
        role-foreground-dark: "#ffffff";
        role-card-dark: "#0f3157";
        role-card-foreground-dark: "#ffffff";
        role-popover-dark: "#0f3157";
        role-popover-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. The lighter indigo takes deep navy
        // button text for contrast, while neutral fills remain recognisably blue.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#a197ff";
        role-primary-foreground-dark: "#081a2c";
        role-primary-hover-dark: "#ada4ff";
        role-secondary-dark: "#103a6b";
        role-secondary-foreground-dark: "#ffffff";
        role-muted-dark: "#0c2c4b";
        role-muted-foreground-dark: "#a9c3d9";
        role-accent-dark: "#174471";
        role-accent-foreground-dark: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. Bright mint and pink stand out on navy, with
        // green-navy, amber-navy and ruby-navy containers for quieter notices.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#f2567a";
        role-destructive-foreground-dark: "#081a2c";
        role-success-dark: "#37d39b";
        role-success-foreground-dark: "#081a2c";
        role-success-container-dark: "#0d463f";
        role-warning-dark: "#f5a623";
        role-warning-foreground-dark: "#3d2b0b";
        role-warning-container-dark: "#4a3c26";
        role-error-container-dark: "#492b47";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half. Blue decorative edges separate navy
        // surfaces; lighter control edges remain visible against both fills.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#1d4470";
        role-input-dark: "#6689ac";
        role-ring-dark: "#7a73ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half. Thumb states brighten from medium navy to
        // the pale blue-gray used for quiet text.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#315c86";
        role-scrollbar-thumb-hover-dark: "#6689ac";
        role-scrollbar-thumb-active-dark: "#a9c3d9";
    }
}
