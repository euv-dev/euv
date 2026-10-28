use super::*;

// The daisyUI scheme.
//
// daisyUI is a Tailwind plugin whose whole theme is twenty-seven custom
// properties per theme. The values here are the published `light` and `dark`
// themes of daisyui@5.7.50: the `base` ramp becomes the page and card
// surfaces, the `primary`/`secondary`/`accent` triad becomes the brand
// roles, and the status triad becomes the status roles. The radius trio
// (`--radius-field`, `--radius-selector`, `--radius-box`) fills the shape
// slots, `--border` fills the hairline width, and the size steps of the
// component layer (`--size-field`) fix the control heights. daisyUI
// publishes no font family and no shadow ramp, so those are derived from
// the platform stack and from the two shadows it does publish.
vars! {
    pub c_scheme_daisyui {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — daisyUI publishes two radii: `--radius-field` (.25rem) for
        // the controls and `--radius-selector` / `--radius-box` (.5rem) for
        // the containers and the cards. The tiny atom takes half a step.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "4px";
        radius-lg: "8px";
        radius-xl: "8px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the alert shadow and the modal shadow are the two
        // daisyUI publishes; the steps between them are interpolated in the
        // same idiom, `oklch(0% 0 0 / α)`.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0 1px 2px 0 oklch(0% 0 0 / 0.08)";
        shadow-sm: "0 4px 3px -2px oklch(0% 0 0 / 0.08)";
        shadow-md: "0 10px 15px -3px oklch(0% 0 0 / 0.10)";
        shadow-lg: "0 20px 25px -5px oklch(0% 0 0 / 0.12)";
        shadow-xl: "0 25px 50px -12px oklch(0% 0 0 / 0.25)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — `--border` is the hairline; the card's own
        // two-pixel outline and its two-pixel offset are the focus ring.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — daisyUI inherits Tailwind's system sans stack and its
        // type ramp; the ramp is anchored on the sizes the component layer
        // publishes, .75rem for the small text and .875rem for the body.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace";
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.25rem";
        font-xl: "1.375rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
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
        // Control Metrics — a button is `calc(var(--size-field) * 10)` tall and
        // the small one `calc(var(--size-field) * 8)`; a menu row pads itself
        // by .375rem.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "40px";
        control-height-sm: "32px";
        control-row-pad: "6px";

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

        role-background-light: "oklch(100% 0 0)";
        role-foreground-light: "oklch(21% .006 285.885)";
        role-card-light: "oklch(100% 0 0)";
        role-card-foreground-light: "oklch(21% .006 285.885)";
        role-popover-light: "oklch(100% 0 0)";
        role-popover-foreground-light: "oklch(21% .006 285.885)";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "oklch(45% .24 277.023)";
        role-primary-foreground-light: "oklch(93% .034 272.788)";
        role-primary-hover-light: "oklch(45% .24 277.023 / 0.9)";
        role-secondary-light: "oklch(65% .241 354.308)";
        role-secondary-foreground-light: "oklch(94% .028 342.258)";
        role-muted-light: "oklch(98% 0 0)";
        role-muted-foreground-light: "oklch(21% .006 285.885 / 0.6)";
        role-accent-light: "oklch(77% .152 181.912)";
        role-accent-foreground-light: "oklch(38% .063 188.416)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "oklch(71% .194 13.428)";
        role-destructive-foreground-light: "oklch(27% .105 12.094)";
        role-success-light: "oklch(76% .177 163.223)";
        role-success-foreground-light: "oklch(37% .077 168.94)";
        role-success-container-light: "color-mix(in oklab, oklch(76% .177 163.223) 15%, oklch(100% 0 0))";
        role-warning-light: "oklch(82% .189 84.429)";
        role-warning-foreground-light: "oklch(41% .112 45.904)";
        role-warning-container-light: "color-mix(in oklab, oklch(82% .189 84.429) 18%, oklch(100% 0 0))";
        role-error-container-light: "color-mix(in oklab, oklch(71% .194 13.428) 15%, oklch(100% 0 0))";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The border is the divider mix, the
        // input is the `--input-color` mix of the content colour, and the ring
        // is the brand colour.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "oklch(21% .006 285.885 / 0.1)";
        role-input-light: "oklch(21% .006 285.885 / 0.2)";
        role-ring-light: "oklch(45% .24 277.023)";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "oklch(21% .006 285.885 / 0.2)";
        role-scrollbar-thumb-hover-light: "oklch(21% .006 285.885 / 0.3)";
        role-scrollbar-thumb-active-light: "oklch(21% .006 285.885 / 0.4)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "oklch(25.33% .016 252.42)";
        role-foreground-dark: "oklch(97.807% .029 256.847)";
        role-card-dark: "oklch(25.33% .016 252.42)";
        role-card-foreground-dark: "oklch(97.807% .029 256.847)";
        role-popover-dark: "oklch(25.33% .016 252.42)";
        role-popover-foreground-dark: "oklch(97.807% .029 256.847)";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "oklch(58% .233 277.117)";
        role-primary-foreground-dark: "oklch(96% .018 272.314)";
        role-primary-hover-dark: "oklch(58% .233 277.117 / 0.9)";
        role-secondary-dark: "oklch(65% .241 354.308)";
        role-secondary-foreground-dark: "oklch(94% .028 342.258)";
        role-muted-dark: "oklch(23.26% .014 253.1)";
        role-muted-foreground-dark: "oklch(97.807% .029 256.847 / 0.6)";
        role-accent-dark: "oklch(77% .152 181.912)";
        role-accent-foreground-dark: "oklch(38% .063 188.416)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "oklch(71% .194 13.428)";
        role-destructive-foreground-dark: "oklch(27% .105 12.094)";
        role-success-dark: "oklch(76% .177 163.223)";
        role-success-foreground-dark: "oklch(37% .077 168.94)";
        role-success-container-dark: "color-mix(in oklab, oklch(76% .177 163.223) 20%, oklch(25.33% .016 252.42))";
        role-warning-dark: "oklch(82% .189 84.429)";
        role-warning-foreground-dark: "oklch(41% .112 45.904)";
        role-warning-container-dark: "color-mix(in oklab, oklch(82% .189 84.429) 22%, oklch(25.33% .016 252.42))";
        role-error-container-dark: "color-mix(in oklab, oklch(71% .194 13.428) 20%, oklch(25.33% .016 252.42))";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "oklch(97.807% .029 256.847 / 0.1)";
        role-input-dark: "oklch(97.807% .029 256.847 / 0.2)";
        role-ring-dark: "oklch(58% .233 277.117)";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "oklch(97.807% .029 256.847 / 0.2)";
        role-scrollbar-thumb-hover-dark: "oklch(97.807% .029 256.847 / 0.3)";
        role-scrollbar-thumb-active-dark: "oklch(97.807% .029 256.847 / 0.4)";
    }
}
