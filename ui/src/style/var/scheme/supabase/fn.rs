use super::*;

// The Supabase scheme.
//
// Supabase's dashboard pairs a vivid green accent with neutral grays
// and a dark-first, compact developer-tool layout.
// The palette and metrics follow the Supabase product-UI reference values;
// the green and neutral ramps also supply containers and scrollbar states.
// Light-mode green and muted ink are darkened enough to retain readable text.
vars! {
    pub c_scheme_supabase {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — The compact two-to-twelve-pixel ladder keeps database
        // controls tight while giving cards a little more room at their corners.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "4px";
        radius-md: "6px";
        radius-lg: "8px";
        radius-xl: "12px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — A restrained neutral-black ramp supports floating
        // tools without replacing the dark dashboard's hairline hierarchy.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "none";
        shadow-sm: "0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-md: "0 2px 4px rgba(0, 0, 0, 0.06)";
        shadow-lg: "0 4px 12px rgba(0, 0, 0, 0.08)";
        shadow-xl: "0 8px 24px rgba(0, 0, 0, 0.10)";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — One-pixel edges define the tool surfaces;
        // an offset green ring marks keyboard focus independently of hover.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — Inter supplies the developer dashboard text, with
        // a system mono stack for identifiers and a compact tracking treatment.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "Inter, ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif";
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
        // Control Metrics — Thirty-six-pixel controls and 32px compact
        // controls fit dense database forms on an eight-pixel row rhythm.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "36px";
        control-height-sm: "32px";
        control-row-pad: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — Camera pixels remain black-backed and drawing
        // previews white-backed, independent of the dashboard gray scale.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half. The near-white gray canvas surrounds white
        // cards, all using neutral charcoal ink without a blue tint.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#f8f9f9";
        role-foreground-light: "#1c1c1c";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#1c1c1c";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#1c1c1c";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The #24b47e brand green is deepened
        // for a white-label primary button; quieter surfaces stay neutral gray.
        // Muted ink is also deepened for the slightly off-white canvas.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#157b56";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#12704c";
        role-secondary-light: "#f1f3f4";
        role-secondary-foreground-light: "#1c1c1c";
        role-muted-light: "#f4f5f6";
        role-muted-foreground-light: "#676d73";
        role-accent-light: "#e9edef";
        role-accent-foreground-light: "#1c1c1c";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. Brand green also signals success; green
        // #e6f6ef, amber and red containers carry the lower-emphasis notices.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#e5484d";
        role-destructive-foreground-light: "#111111";
        role-success-light: "#24b47e";
        role-success-foreground-light: "#0a2b1f";
        role-success-container-light: "#e6f6ef";
        role-warning-light: "#f5a623";
        role-warning-foreground-light: "#33240b";
        role-warning-container-light: "#fff4dc";
        role-error-container-light: "#ffebec";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. Pale decorative hairlines preserve
        // the gray surface hierarchy; input edges and the green ring are stronger.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#e0e2e5";
        role-input-light: "#878e94";
        role-ring-light: "#16815a";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half. Neutral gray thumb states avoid making
        // routine scrolling look like a green success or primary-action state.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#c5cbd0";
        role-scrollbar-thumb-hover-light: "#9da5ac";
        role-scrollbar-thumb-active-light: "#6c7379";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. The dark-first canvas is neutral #1c1c1c,
        // with #242424 cards and popovers rather than a pure-black background.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#1c1c1c";
        role-foreground-dark: "#eeeeee";
        role-card-dark: "#242424";
        role-card-foreground-dark: "#eeeeee";
        role-popover-dark: "#242424";
        role-popover-foreground-dark: "#eeeeee";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. Bright #3ecf8e green takes dark
        // green ink; the gray secondary and active fills remain deliberately quiet.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#3ecf8e";
        role-primary-foreground-dark: "#0a2b1f";
        role-primary-hover-dark: "#4bdd9d";
        role-secondary-dark: "#2a2a2a";
        role-secondary-foreground-dark: "#eeeeee";
        role-muted-dark: "#242424";
        role-muted-foreground-dark: "#9b9b9b";
        role-accent-dark: "#2e2e2e";
        role-accent-foreground-dark: "#eeeeee";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. Bright green, amber and soft red separate the
        // statuses, with a deep #12291f success container on charcoal surfaces.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#ef6461";
        role-destructive-foreground-dark: "#111111";
        role-success-dark: "#3ecf8e";
        role-success-foreground-dark: "#0a2b1f";
        role-success-container-dark: "#12291f";
        role-warning-dark: "#f7b955";
        role-warning-foreground-dark: "#33240b";
        role-warning-container-dark: "#382b17";
        role-error-container-dark: "#3b2021";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half. A subtle gray decorative edge frames
        // surfaces; the input edge and green focus ring remain distinguishable.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#2e2e2e";
        role-input-dark: "#737373";
        role-ring-dark: "#3ecf8e";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half. The thumb brightens through neutral grays
        // to the muted text colour as hover and drag become active.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#454545";
        role-scrollbar-thumb-hover-dark: "#737373";
        role-scrollbar-thumb-active-dark: "#9b9b9b";
    }
}
