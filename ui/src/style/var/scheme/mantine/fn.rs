use super::*;

// The Mantine scheme.
//
// Mantine 9.7.4 with the default theme object: the primary colour is `blue`
// with `primaryShade` 6 in light and 8 in dark, the radius is `md` and the
// font family is the system stack. Every colour below is a step of the
// published `blue`, `gray`, `dark`, `red`, `teal` and `yellow` palettes, with
// the dark-mode tints resolved with Mantine's own `darken()` rule.
vars! {
    pub c_scheme_mantine {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the `--mantine-radius-*` ladder, 2px to 32px, with `md` the
        // default every control, panel and paper takes. A pill is the 1000px
        // radius Mantine gives its badges, avatars and switches.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "8px";
        radius-lg: "16px";
        radius-xl: "32px";
        radius-full: "1000px";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the compiled `--mantine-shadow-xs..xl` ramp, unchanged
        // between the two modes.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), 0 0.0625rem 0.125rem rgba(0, 0, 0, 0.1)";
        shadow-sm: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 0.625rem 0.9375rem -0.3125rem, rgba(0, 0, 0, 0.04) 0 0.4375rem 0.4375rem -0.3125rem";
        shadow-md: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 1.25rem 1.5625rem -0.3125rem, rgba(0, 0, 0, 0.04) 0 0.625rem 0.625rem -0.3125rem";
        shadow-lg: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 1.75rem 1.4375rem -0.4375rem, rgba(0, 0, 0, 0.04) 0 0.75rem 0.75rem -0.4375rem";
        shadow-xl: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 2.25rem 1.75rem -0.4375rem, rgba(0, 0, 0, 0.04) 0 1.0625rem 1.0625rem -0.4375rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the focus ring is hard-coded in Mantine's global
        // stylesheet: `outline: 2px solid var(--mantine-primary-color-filled)`
        // with a 2px offset.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the default font family and the `fontSizes` ramp, with
        // the heading ladder filling the display steps. The theme publishes no
        // size above `h1`, so the three largest steps continue that ladder.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, Courier New, monospace";
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.25rem";
        font-xl: "1.375rem";
        font-2xl: "1.625rem";
        font-3xl: "2.125rem";
        font-4xl: "2.75rem";
        font-5xl: "3.5rem";
        font-6xl: "4.5rem";
        font-weight-normal: "400";
        font-weight-medium: "600";
        font-weight-semibold: "600";
        font-weight-bold: "700";
        letter-spacing-tight: "-0.01em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.025em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default button and input are the `sm` step,
        // 36px tall, and the compact one is the `xs` step. A row carries the
        // vertical padding of a Mantine menu item.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "36px";
        control-height-sm: "30px";
        control-row-pad: "6.67px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — a camera feed and the letterbox around a fullscreen
        // canvas are black, a drawing preview is white. They are pictures, not
        // chrome, so they do not follow the theme.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half. The page is `--mantine-color-body`, the
        // card the paper background and the floating surfaces the popover
        // background.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#ffffff";
        role-foreground-light: "#000000";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#000000";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The brand is the filled blue
        // step, its hover the next shade; the neutral fills are the gray steps
        // Mantine paints its menus and tables with.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#228be6";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#1c7ed6";
        role-secondary-light: "#f1f3f5";
        role-secondary-foreground-light: "#000000";
        role-muted-light: "#f8f9fa";
        role-muted-foreground-light: "#868e96";
        role-accent-light: "#f1f3f5";
        role-accent-foreground-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. The error is the global error colour, the
        // success the global success colour, and a container is the `light`
        // variant of the same hue.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#fa5252";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#099268";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#e6fcf5";
        role-warning-light: "#fab005";
        role-warning-foreground-light: "#ffffff";
        role-warning-container-light: "#fff3bf";
        role-error-container-light: "#fff5f5";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The hairline is the table and
        // paper border, the control border the input border, and the ring the
        // filled primary colour.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#dee2e6";
        role-input-light: "#ced4da";
        role-ring-light: "#228be6";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half, read off the gray ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#dee2e6";
        role-scrollbar-thumb-hover-light: "#ced4da";
        role-scrollbar-thumb-active-light: "#adb5bd";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. The page is the dark palette step 7, the
        // card and the floating surfaces step 6.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#242424";
        role-foreground-dark: "#C9C9C9";
        role-card-dark: "#2e2e2e";
        role-card-foreground-dark: "#C9C9C9";
        role-popover-dark: "#2e2e2e";
        role-popover-foreground-dark: "#C9C9C9";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. The brand moves to the dark
        // `primaryShade` step 8 and its hover to step 9; the neutral fills
        // come from the dark palette.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#1971c2";
        role-primary-foreground-dark: "#ffffff";
        role-primary-hover-dark: "#1864ab";
        role-secondary-dark: "#3b3b3b";
        role-secondary-foreground-dark: "#ffffff";
        role-muted-dark: "#2e2e2e";
        role-muted-foreground-dark: "#828282";
        role-accent-dark: "#424242";
        role-accent-foreground-dark: "#C9C9C9";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. The containers are the dark `light` variant
        // of each hue: the shade nine darkened by half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#e03131";
        role-destructive-foreground-dark: "#ffffff";
        role-success-dark: "#099268";
        role-success-foreground-dark: "#ffffff";
        role-success-container-dark: "#04402e";
        role-warning-dark: "#f08c00";
        role-warning-foreground-dark: "#ffffff";
        role-warning-container-dark: "#733c00";
        role-error-container-dark: "#651515";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#424242";
        role-input-dark: "#424242";
        role-ring-dark: "#1971c2";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half, read off the dark palette.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#424242";
        role-scrollbar-thumb-hover-dark: "#696969";
        role-scrollbar-thumb-active-dark: "#828282";
    }
}
