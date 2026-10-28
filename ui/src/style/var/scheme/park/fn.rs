use super::*;

// The Park UI scheme.
//
// Park UI 0.43.1, built on Ark UI and Panda CSS: the `neutral` gray palette,
// the `blue` accent palette and the semantic `bg`/`fg`/`border` tokens the
// preset emits. The radii, the type ramp and the shadow ramp are the preset's
// own tokens, and the status colours come from the `green`, `amber` and
// `red` palettes that ship with it.
vars! {
    pub c_scheme_park {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the `radii` token ladder. The semantic radii `l1`, `l2` and
        // `l3` are the ones the components actually use: `l2` is the button,
        // the input and the dropdown, `l3` the card and the dialog, `l1` the
        // list item. A pill is the `full` radius.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "1px";
        radius-sm: "2px";
        radius-md: "4px";
        radius-lg: "6px";
        radius-xl: "12px";
        radius-full: "9999px";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the light `shadows` token ramp, with the gray alpha
        // steps written out as hex.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0px 1px 2px #0000001f, 0px 0px 1px #00000031";
        shadow-sm: "0px 2px 4px #0000000f, 0px 0px 1px #00000031";
        shadow-md: "0px 4px 8px #0000000f, 0px 0px 1px #00000031";
        shadow-lg: "0px 8px 16px #0000000f, 0px 0px 1px #00000031";
        shadow-xl: "0px 16px 24px #0000000f, 0px 0px 1px #00000031";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — a control is a one-pixel border and the focus
        // ring is an outline: `outline: 2px solid` with a 2px offset.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the `fonts` and `fontSizes` tokens, matched to the
        // `textStyles` ramp the components use.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
        font-family-mono: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace";
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
        letter-spacing-tight: "-0.025em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.025em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — the default button is the `md` step, 2.5rem tall,
        // and the small one the `sm` step. A row is the base line box on the
        // padding that lands it on the same 40px as the button.
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
        // Surfaces — the light half. The page is `bg.canvas`, the card and the
        // floating surfaces `bg.default`.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#fcfcfc";
        role-foreground-light: "#202020";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#202020";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#202020";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The brand is the accent
        // palette's solid fill with its hover one step down the ramp; the
        // neutral fills are `bg.subtle` and `bg.muted`.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#0090ff";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#0588f0";
        role-secondary-light: "#f9f9f9";
        role-secondary-foreground-light: "#202020";
        role-muted-light: "#f0f0f0";
        role-muted-foreground-light: "#646464";
        role-accent-light: "#0000000f";
        role-accent-foreground-light: "#202020";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. A status colour is the palette's solid fill
        // and a container its subtle variant, the tinted fill behind it.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#e5484d";
        role-destructive-foreground-light: "#ffffff";
        role-success-light: "#30a46c";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#00a43319";
        role-warning-light: "#ffc53d";
        role-warning-foreground-light: "#202020";
        role-warning-container-light: "#ffde003d";
        role-error-container-light: "#f3000d14";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The hairline is the global
        // `border.subtle` every element carries, the control border the
        // `border.default` the inputs use, and the ring the accent fill.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#e8e8e8";
        role-input-light: "#cecece";
        role-ring-light: "#0090ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half, read off the neutral ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#d9d9d9";
        role-scrollbar-thumb-hover-light: "#cecece";
        role-scrollbar-thumb-active-light: "#bbbbbb";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. The page is `bg.canvas`, the card and the
        // floating surfaces `bg.default`.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#111111";
        role-foreground-dark: "#eeeeee";
        role-card-dark: "#191919";
        role-card-foreground-dark: "#eeeeee";
        role-popover-dark: "#191919";
        role-popover-foreground-dark: "#eeeeee";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. The accent fill is mode
        // independent and its hover moves one step up the dark ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#0090ff";
        role-primary-foreground-dark: "#ffffff";
        role-primary-hover-dark: "#3b9eff";
        role-secondary-dark: "#222222";
        role-secondary-foreground-dark: "#eeeeee";
        role-muted-dark: "#2a2a2a";
        role-muted-foreground-dark: "#b4b4b4";
        role-accent-dark: "#ffffff12";
        role-accent-foreground-dark: "#eeeeee";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. The status fills are mode independent; only
        // the tinted containers change with the mode.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#e5484d";
        role-destructive-foreground-dark: "#ffffff";
        role-success-dark: "#30a46c";
        role-success-foreground-dark: "#ffffff";
        role-success-container-dark: "#22ff991e";
        role-warning-dark: "#ffc53d";
        role-warning-foreground-dark: "#fcfcfc";
        role-warning-container-dark: "#fa820022";
        role-error-container-dark: "#ff173f2d";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#2a2a2a";
        role-input-dark: "#484848";
        role-ring-dark: "#0090ff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half, read off the neutral ramp.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#3a3a3a";
        role-scrollbar-thumb-hover-dark: "#484848";
        role-scrollbar-thumb-active-dark: "#606060";
    }
}
