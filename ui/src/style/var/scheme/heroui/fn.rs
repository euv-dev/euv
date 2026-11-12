use super::*;

// The HeroUI scheme.
//
// HeroUI (formerly NextUI) builds its theme on Tailwind scales: a zinc
// neutral ramp, a blue brand, and purple, green, yellow and red for the
// other roles. The values here are the published `@heroui/theme@2.4.26`
// semantic colours — `background`, `foreground`, `content1`…`content4`,
// `default`, `primary`, `secondary`, `success`, `warning` and `danger` —
// with the dark half taking the swapped scales. The layout tokens supply
// the radii (8px, 12px, 14px), the border widths, the shadow ramp and the
// type scale. HeroUI publishes no font family, so the family is the
// platform sans stack it inherits from Tailwind.
vars! {
    pub c_scheme_heroui {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — the published radii are 8px, 12px and 14px; the tiny atom
        // is the checkbox's own half-radius step.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "6px";
        radius-sm: "8px";
        radius-md: "12px";
        radius-lg: "12px";
        radius-xl: "14px";
        radius-2xl: "17px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — the published small, medium and large shadows; the two
        // ends of the ramp are interpolated in the same idiom.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "0px 0px 2px 0px rgb(0 0 0 / 0.04)";
        shadow-sm: "0px 0px 5px 0px rgb(0 0 0 / 0.02), 0px 2px 10px 0px rgb(0 0 0 / 0.06), 0px 0px 1px 0px rgb(0 0 0 / 0.3)";
        shadow-md: "0px 0px 15px 0px rgb(0 0 0 / 0.03), 0px 2px 30px 0px rgb(0 0 0 / 0.08), 0px 0px 1px 0px rgb(0 0 0 / 0.3)";
        shadow-lg: "0px 0px 30px 0px rgb(0 0 0 / 0.04), 0px 30px 60px 0px rgb(0 0 0 / 0.12), 0px 0px 1px 0px rgb(0 0 0 / 0.3)";
        shadow-xl: "0px 0px 60px 0px rgb(0 0 0 / 0.06), 0px 60px 120px 0px rgb(0 0 0 / 0.16), 0px 0px 1px 0px rgb(0 0 0 / 0.3)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the hairline is the small border width; the focus
        // ring is the medium border width.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "0px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — the published scale is tiny .75rem, small .875rem,
        // medium 1rem and large 1.125rem; the rest of the ramp continues on
        // Tailwind's scale.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'";
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
        letter-spacing-wide: "0.05em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — a button is h-10 tall and the small one h-8; a
        // menu row pads itself by py-1.5.
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
        // Surfaces — the light half. The card and the popover are `content1`,
        // the surface HeroUI's own modal and menu sit on.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#FFFFFF";
        role-foreground-light: "#11181C";
        role-card-light: "#FFFFFF";
        role-card-foreground-light: "#11181C";
        role-popover-light: "#FFFFFF";
        role-popover-foreground-light: "#11181C";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The quiet fill is the neutral
        // ramp at 100, the hover fill at 200, and the muted ink the ramp at
        // 500.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#006FEE";
        role-primary-foreground-light: "#FFFFFF";
        role-primary-hover-light: "rgba(0, 111, 238, 0.8)";
        role-secondary-light: "#7828C8";
        role-secondary-foreground-light: "#FFFFFF";
        role-muted-light: "#F4F4F5";
        role-muted-foreground-light: "#71717A";
        role-accent-light: "#E4E4E7";
        role-accent-foreground-light: "#11181C";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. The containers are the tint steps of the
        // same scales the status colours come from.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#F31260";
        role-destructive-foreground-light: "#FFFFFF";
        role-success-light: "#17C964";
        role-success-foreground-light: "#000000";
        role-success-container-light: "#E8FAF0";
        role-warning-light: "#F5A524";
        role-warning-foreground-light: "#000000";
        role-warning-container-light: "#FEFCE8";
        role-error-container-light: "#FEE7EF";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The border is the published
        // divider, the input the neutral ramp at 200, and the ring the
        // published focus colour.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "rgba(17, 17, 17, 0.15)";
        role-input-light: "#E4E4E7";
        role-ring-light: "#006FEE";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#A1A1AA";
        role-scrollbar-thumb-hover-light: "#71717A";
        role-scrollbar-thumb-active-light: "#52525B";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#000000";
        role-foreground-dark: "#ECEDEE";
        role-card-dark: "#18181B";
        role-card-foreground-dark: "#FAFAFA";
        role-popover-dark: "#18181B";
        role-popover-foreground-dark: "#FAFAFA";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half, on the swapped scales.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#006FEE";
        role-primary-foreground-dark: "#FFFFFF";
        role-primary-hover-dark: "rgba(0, 111, 238, 0.9)";
        role-secondary-dark: "#9353D3";
        role-secondary-foreground-dark: "#FFFFFF";
        role-muted-dark: "#27272A";
        role-muted-foreground-dark: "#A1A1AA";
        role-accent-dark: "#3F3F46";
        role-accent-foreground-dark: "#ECEDEE";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#F31260";
        role-destructive-foreground-dark: "#FFFFFF";
        role-success-dark: "#17C964";
        role-success-foreground-dark: "#000000";
        role-success-container-dark: "#052814";
        role-warning-dark: "#F5A524";
        role-warning-foreground-dark: "#000000";
        role-warning-container-dark: "#312107";
        role-error-container-dark: "#310413";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "rgba(255, 255, 255, 0.15)";
        role-input-dark: "#3F3F46";
        role-ring-dark: "#006FEE";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#52525B";
        role-scrollbar-thumb-hover-dark: "#71717A";
        role-scrollbar-thumb-active-dark: "#A1A1AA";
    }
}
