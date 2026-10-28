use super::*;

// The Radix Themes scheme.
//
// Radix Themes 3.3.0 with its stock configuration: accent colour `indigo`,
// gray colour `slate` (what `grayColor: 'auto'` resolves to for an indigo
// accent), `radius="medium"` and `scaling=100%`. Every colour below is a step
// of the published indigo, slate, green, amber and red ramps, with the alpha
// steps resolved to their hex form, so no token points at another one. The
// roles Radix Themes does not publish itself — the status colours, the
// scrollbar and the border hairlines — are taken from the same palette
// family the library ships.
vars! {
    pub c_scheme_radix {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the `--radius-1..6` ladder at the default `radius="medium"`,
        // read off the components that use each step: the checkbox takes
        // `--radius-1`, the avatar `--radius-2`, the button and the text field
        // `--radius-3`, the card and the popover `--radius-4`/`--radius-5`.
        // A pill is `--radius-full`, the value Radix gives its own switch thumb.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "3px";
        radius-sm: "4px";
        radius-md: "6px";
        radius-lg: "8px";
        radius-xl: "12px";
        radius-full: "9999px";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the light `--shadow-1..6` ramp with the gray alpha steps
        // written out as hex. `--shadow-1` is the inset shadow a classic
        // control carries, `--shadow-4` the popover, `--shadow-6` the dialog.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "inset 0 0 0 1px #0009321f, inset 0 1.5px 2px 0 #00005506, inset 0 1.5px 2px 0 rgba(0, 0, 0, 0.1)";
        shadow-sm: "0 0 0 1px #0000330f, 0 0 0 0.5px rgba(0, 0, 0, 0.05), 0 1px 1px 0 #00005506, 0 2px 1px -1px rgba(0, 0, 0, 0.05), 0 1px 3px 0 rgba(0, 0, 0, 0.05)";
        shadow-md: "0 0 0 1px #0000330f, 0 2px 3px -2px #0000330f, 0 3px 12px -4px rgba(0, 0, 0, 0.1), 0 4px 16px -8px rgba(0, 0, 0, 0.1)";
        shadow-lg: "0 0 0 1px #0000330f, 0 8px 40px rgba(0, 0, 0, 0.05), 0 12px 32px -16px #0000330f";
        shadow-xl: "0 0 0 1px #0000330f, 0 12px 60px rgba(0, 0, 0, 0.15), 0 16px 64px #00005506, 0 16px 36px -20px #00062e32";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — Radix draws the focus ring as an outline, never as
        // a box shadow: `outline: 2px solid var(--focus-8)` with a 2px offset.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the `--default-font-family` stack and the nine-step
        // numeric scale (`--font-size-1..9`). The scale stops at 60px, so the
        // two largest steps continue its display end.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "-apple-system, BlinkMacSystemFont, 'Segoe UI (Custom)', Roboto, 'Helvetica Neue', 'Open Sans (Custom)', system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'";
        font-family-mono: "'Menlo', 'Consolas (Custom)', 'Bitstream Vera Sans Mono', monospace, 'Apple Color Emoji', 'Segoe UI Emoji'";
        font-xs: "12px";
        font-sm: "14px";
        font-base: "16px";
        font-md: "18px";
        font-lg: "20px";
        font-xl: "24px";
        font-2xl: "28px";
        font-3xl: "35px";
        font-4xl: "60px";
        font-5xl: "72px";
        font-6xl: "88px";
        font-weight-normal: "400";
        font-weight-medium: "500";
        font-weight-semibold: "600";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.025em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.025em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default button is `--space-7` tall, the small
        // one `--space-6`. A body row is the base 16px line on a 24px line
        // box plus the padding that lands it on the same 40px as the button.
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
        // Surfaces — the light half. The page is `--color-background`, the
        // card is the translucent `--color-panel` and the floating surfaces
        // sit on the solid `--color-panel-solid`.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#ffffff";
        role-foreground-light: "#1c2024";
        role-card-light: "rgba(255, 255, 255, 0.7)";
        role-card-foreground-light: "#1c2024";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#1c2024";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The brand is the indigo accent
        // step 9 with its hover one step down the ramp; the neutral fills are
        // the slate steps Radix uses for its hover surfaces.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#3e63dd";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#3358d4";
        role-secondary-light: "#f0f0f3";
        role-secondary-foreground-light: "#1c2024";
        role-muted-light: "#f9f9fb";
        role-muted-foreground-light: "#60646c";
        role-accent-light: "#f0f0f3";
        role-accent-foreground-light: "#1c2024";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. The green, amber and red ramps are the ones
        // Radix ships alongside the themes; a container is the soft, tinted
        // step the `soft` variant paints.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#e5484d";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#30a46c";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#00a43319";
        role-warning-light: "#ffc53d";
        role-warning-foreground-light: "#4f3422";
        role-warning-container-light: "#ffde003d";
        role-error-container-light: "#f3000d14";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The hairline is the slate alpha a
        // step Radix uses for its separators, the control border the alpha a7
        // it insets on every surface input, and the ring the accent step 8.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#0009321f";
        role-input-light: "#00062e32";
        role-ring-light: "#8da4ef";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half, read off the solid slate ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#b9bbc6";
        role-scrollbar-thumb-hover-light: "#8b8d98";
        role-scrollbar-thumb-active-light: "#80838d";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. The canvas is slate step 1, the solid
        // panel is step 2 and the translucent card the slate alpha a2.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#111113";
        role-foreground-dark: "#edeef0";
        role-card-dark: "#d8f4f609";
        role-card-foreground-dark: "#edeef0";
        role-popover-dark: "#18191b";
        role-popover-foreground-dark: "#edeef0";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. The brand keeps step 9 in both
        // modes and its hover moves one step up the dark ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#3e63dd";
        role-primary-foreground-dark: "#ffffff";
        role-primary-hover-dark: "#5472e4";
        role-secondary-dark: "#212225";
        role-secondary-foreground-dark: "#edeef0";
        role-muted-dark: "#18191b";
        role-muted-foreground-dark: "#b0b4ba";
        role-accent-dark: "#212225";
        role-accent-foreground-dark: "#edeef0";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. The status steps are mode independent in
        // Radix; only the tinted containers change with the mode.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#e5484d";
        role-destructive-foreground-dark: "#ffffff";
        role-success-dark: "#30a46c";
        role-success-foreground-dark: "#ffffff";
        role-success-container-dark: "#22ff991e";
        role-warning-dark: "#ffc53d";
        role-warning-foreground-dark: "#4f3422";
        role-warning-container-dark: "#fa820022";
        role-error-container-dark: "#ff173f2d";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#d9edfe25";
        role-input-dark: "#d9edff40";
        role-ring-dark: "#435db1";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half, read off the solid slate ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#5a6169";
        role-scrollbar-thumb-hover-dark: "#696e77";
        role-scrollbar-thumb-active-dark: "#777b84";
    }
}
