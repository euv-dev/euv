use super::*;

// The Geist UI scheme.
//
// Geist UI is Vercel's system: a single accent ramp (`accents_1`…`accents_8`)
// instead of a colour scale, a black or white page, and a blue that plays
// the role of both the brand colour and the success colour. The values here
// are the published `presets/default.ts` and `presets/dark.ts` palettes and
// expressiveness tokens. Geist publishes one radius, 6px, so the shape slots
// share it; the unit is 16px, which sets the type ramp and the control
// heights; and the font family is the published Inter stack.
vars! {
    pub c_scheme_geist {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Geist publishes a single radius, `theme.layout.radius` = 6px,
        // and every control, card and avatar sits on it. The tiny atom takes
        // half of it.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "3px";
        radius-sm: "6px";
        radius-md: "6px";
        radius-lg: "6px";
        radius-xl: "6px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the published dropdown shadow and the three published
        // shadows, small, medium and large. The dark theme replaces the
        // shadow with a one-pixel ring.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "none";
        shadow-sm: "0 4px 4px 0 rgba(0, 0, 0, 0.02)";
        shadow-md: "0 5px 10px rgba(0, 0, 0, 0.12)";
        shadow-lg: "0 8px 30px rgba(0, 0, 0, 0.12)";
        shadow-xl: "0 30px 60px rgba(0, 0, 0, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the hairline is the one-pixel border the card, the
        // input and the button publish. Geist publishes no focus ring, so it
        // takes the brand colour and a two-pixel width.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the published Inter stack and Geist's own mono stack.
        // The type ramp is the published scale, `SCALES.font(n) = n × 16px`,
        // anchored on the .875rem the components use and the 1rem the table
        // and the tabs use.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif";
        font-family-mono: "Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace";
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
        letter-spacing-tight: "-0.025em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.05em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — a button is `height(2.5)` tall, the input and the
        // select `height(2.25)`; a row pads itself by the published half gap.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "40px";
        control-height-sm: "36px";
        control-row-pad: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — a camera feed and the letterbox around a fullscreen
        // canvas are black, a drawing preview is white. They are pictures, not
        // chrome, so they do not follow the theme.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half. A Geist card is the page background with
        // a hairline border, so the card and the popover share the canvas.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#fff";
        role-foreground-light: "#111";
        role-card-light: "#fff";
        role-card-foreground-light: "#111";
        role-popover-light: "#fff";
        role-popover-foreground-light: "#111";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The brand colour is the
        // published blue, the quiet fill is the first accent, the second
        // neutral fill the second accent, and the muted ink the published
        // `palette.secondary`.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#0070f3";
        role-primary-foreground-light: "#fff";
        role-primary-hover-light: "#0761d1";
        role-secondary-light: "#eaeaea";
        role-secondary-foreground-light: "#111";
        role-muted-light: "#fafafa";
        role-muted-foreground-light: "#666";
        role-accent-light: "#fafafa";
        role-accent-foreground-light: "#111";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. Geist's own `success` is the blue, and its
        // containers are the published `…Lighter` tints.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#e00";
        role-destructive-foreground-light: "#fff";
        role-success-light: "#0070f3";
        role-success-foreground-light: "#fff";
        role-success-container-light: "#d3e5ff";
        role-warning-light: "#f5a623";
        role-warning-foreground-light: "#fff";
        role-warning-container-light: "#ffefcf";
        role-error-container-light: "#f7d4d6";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#eaeaea";
        role-input-light: "#eaeaea";
        role-ring-light: "#0070f3";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half, on the accent ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#999";
        role-scrollbar-thumb-hover-light: "#888";
        role-scrollbar-thumb-active-light: "#666";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#000";
        role-foreground-dark: "#fafafa";
        role-card-dark: "#000";
        role-card-foreground-dark: "#fafafa";
        role-popover-dark: "#000";
        role-popover-foreground-dark: "#fafafa";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#3291ff";
        role-primary-foreground-dark: "#000";
        role-primary-hover-dark: "#0070f3";
        role-secondary-dark: "#333";
        role-secondary-foreground-dark: "#fafafa";
        role-muted-dark: "#111";
        role-muted-foreground-dark: "#888";
        role-accent-dark: "#111";
        role-accent-foreground-dark: "#fafafa";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. The tints the light half publishes are too
        // pale for a black page, so the containers are the status colour
        // mixed into the published background.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#e00";
        role-destructive-foreground-dark: "#fff";
        role-success-dark: "#0070f3";
        role-success-foreground-dark: "#fff";
        role-success-container-dark: "color-mix(in oklab, #0070f3 25%, #000)";
        role-warning-dark: "#f5a623";
        role-warning-foreground-dark: "#fff";
        role-warning-container-dark: "color-mix(in oklab, #f5a623 25%, #000)";
        role-error-container-dark: "color-mix(in oklab, #e00 25%, #000)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#333";
        role-input-dark: "#333";
        role-ring-dark: "#3291ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half, on the accent ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#444";
        role-scrollbar-thumb-hover-dark: "#666";
        role-scrollbar-thumb-active-dark: "#888";
    }
}
