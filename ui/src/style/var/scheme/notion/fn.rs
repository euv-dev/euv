use super::*;

// The Notion scheme.
//
// Notion's document interface is warm paper and ink, with small corners
// and almost no elevation competing with the page content.
// The palette and metrics follow the Notion product-UI reference values;
// status containers use its pale green, amber and red page tints.
// Primary actions stay ink-coloured, and muted light text is nudged darker for contrast.
vars! {
    pub c_scheme_notion {
        // ═══════════════════════════════════════════════════════════════════════
        // Shape — Small two-to-eight-pixel corners echo document blocks
        // and compact inline tools rather than rounded application cards.
        // ═══════════════════════════════════════════════════════════════════════

        radius-xs: "2px";
        radius-sm: "3px";
        radius-md: "4px";
        radius-lg: "6px";
        radius-xl: "8px";
        radius-full: "50%";

        // ═══════════════════════════════════════════════════════════════════════
        // Elevation — Every step is flat: paper hierarchy comes from
        // hairline edges and warm fills, not a growing drop shadow.
        // ═══════════════════════════════════════════════════════════════════════

        shadow-xs: "none";
        shadow-sm: "none";
        shadow-md: "none";
        shadow-lg: "none";
        shadow-xl: "none";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — One-pixel edges keep document tools quiet;
        // a two-pixel offset ring makes keyboard focus explicit.
        // ═══════════════════════════════════════════════════════════════════════

        border-width: "1px";
        surface-border-width: "1px";
        ring-width: "2px";
        ring-offset: "2px";

        // ═══════════════════════════════════════════════════════════════════════
        // Typography — The native sans stack and restrained tracking keep
        // the page closer to a document than a branded marketing surface.
        // ═══════════════════════════════════════════════════════════════════════

        font-family: "ui-sans-serif, -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";
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
        letter-spacing-tight: "-0.015em";
        letter-spacing-normal: "0em";
        letter-spacing-wide: "0.03em";
        text-transform-none: "none";
        text-transform-uppercase: "uppercase";

        // ═══════════════════════════════════════════════════════════════════════
        // Control Metrics — The default control is 36px and the compact
        // toolbar control 32px, with eight-pixel row padding for document menus.
        // ═══════════════════════════════════════════════════════════════════════

        control-height: "36px";
        control-height-sm: "32px";
        control-row-pad: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Media Surfaces — Embedded camera and drawing content keeps a
        // black or white backing instead of adopting the paper palette.
        // ═══════════════════════════════════════════════════════════════════════

        surface-media: "#000000";
        surface-drawing: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the light half. White page, card and popover surfaces share
        // the warm #37352f document ink rather than a cool black.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-light: "#ffffff";
        role-foreground-light: "#37352f";
        role-card-light: "#ffffff";
        role-card-foreground-light: "#37352f";
        role-popover-light: "#ffffff";
        role-popover-foreground-light: "#37352f";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the light half. The primary button is ink, not a
        // brand hue; warm gray fills frame selections and quiet document tools.
        // Muted ink is slightly darker than #787774 to retain text contrast.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-light: "#37352f";
        role-primary-foreground-light: "#ffffff";
        role-primary-hover-light: "#2e2d28";
        role-secondary-light: "#f7f6f3";
        role-secondary-foreground-light: "#37352f";
        role-muted-light: "#f1f0ed";
        role-muted-foreground-light: "#6b6a66";
        role-accent-light: "#f7f6f3";
        role-accent-foreground-light: "#37352f";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the light half. Green, amber and red retain the familiar
        // Notion page tints, with green #edf3ec, amber #fbf3db and red #fdebec.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-light: "#d44c47";
        role-destructive-foreground-light: "#0d0c0a";
        role-success-light: "#448361";
        role-success-foreground-light: "#ffffff";
        role-success-container-light: "#edf3ec";
        role-warning-light: "#d9730d";
        role-warning-foreground-light: "#151410";
        role-warning-container-light: "#fbf3db";
        role-error-container-light: "#fdebec";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the light half. The warm decorative hairline avoids
        // boxing in the document; control edges use a more visible warm gray.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-light: "#e9e9e7";
        role-input-light: "#94938f";
        role-ring-light: "#787774";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the light half. Thumb states move through warm grays
        // instead of introducing a blue or indigo interaction colour.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-light: "#d3d1cb";
        role-scrollbar-thumb-hover-light: "#aaa8a2";
        role-scrollbar-thumb-active-light: "#787774";

        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces — the dark half. Charcoal paper replaces white without turning
        // the workspace into pure black; the text stays softly off-white.
        // ═══════════════════════════════════════════════════════════════════════

        role-background-dark: "#191919";
        role-foreground-dark: "#e9e9e8";
        role-card-dark: "#202020";
        role-card-foreground-dark: "#e9e9e8";
        role-popover-dark: "#202020";
        role-popover-foreground-dark: "#e9e9e8";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals — the dark half. The ink button reverses to paper
        // with dark text; secondary, muted and active fills are separate grays.
        // ═══════════════════════════════════════════════════════════════════════

        role-primary-dark: "#e9e9e8";
        role-primary-foreground-dark: "#191919";
        role-primary-hover-dark: "#cfcec9";
        role-secondary-dark: "#2a2a2a";
        role-secondary-foreground-dark: "#e9e9e8";
        role-muted-dark: "#242424";
        role-muted-foreground-dark: "#9b9b9b";
        role-accent-dark: "#2f2f2f";
        role-accent-foreground-dark: "#e9e9e8";

        // ═══════════════════════════════════════════════════════════════════════
        // Status — the dark half. Teal green, ochre and soft coral work on
        // charcoal paper; low-key tinted containers keep notices document-like.
        // ═══════════════════════════════════════════════════════════════════════

        role-destructive-dark: "#e8867f";
        role-destructive-foreground-dark: "#191919";
        role-success-dark: "#4dab9a";
        role-success-foreground-dark: "#191919";
        role-success-container-dark: "#203b34";
        role-warning-dark: "#cb912f";
        role-warning-foreground-dark: "#191919";
        role-warning-container-dark: "#3d3220";
        role-error-container-dark: "#422b2a";

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus — the dark half. The decorative charcoal edge stays
        // flat, while input edges and the quiet-gray ring identify controls.
        // ═══════════════════════════════════════════════════════════════════════

        role-border-dark: "#2f2f2f";
        role-input-dark: "#6d6d6d";
        role-ring-dark: "#9b9b9b";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar — the dark half. Neutral gray thumb states echo the
        // document tools, ending at the same gray as muted text.
        // ═══════════════════════════════════════════════════════════════════════

        role-scrollbar-thumb-dark: "#424242";
        role-scrollbar-thumb-hover-dark: "#6d6d6d";
        role-scrollbar-thumb-active-dark: "#9b9b9b";
    }
}
