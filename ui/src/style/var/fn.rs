use super::*;

vars! {
    pub(crate) c_theme_light {
        // ═══════════════════════════════════════════════════════════════════════
        // Monochrome Design Tokens (Black & White only)
        // ═══════════════════════════════════════════════════════════════════════

        // ─── Surface ───
        background: "#ffffff";
        foreground: "#000000";

        // ─── Secondary / Muted ───
        muted-foreground: "#000000";

        // ─── Accent ───
        accent: "#000000";

        // ─── Border & Input ───
        border: "#000000";
        ring: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Legacy Surface Aliases
        // ═══════════════════════════════════════════════════════════════════════
        bg-overlay: "rgba(0, 0, 0, 0.20)";

        // ─── Text Colors ───
        text-on-accent: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand / Accent (Black for minimalist)
        // ═══════════════════════════════════════════════════════════════════════
        accent: "#000000";
        accent-muted: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (shadcn/ui Tailwind spacing)
        // ═══════════════════════════════════════════════════════════════════════
        space-2xs: "2px";
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-7xl: "80px";

        // Sidebar nesting indent, in two forms. `side-indent` carries its own
        // `px` so it can be dropped into a plain `var()` position;
        // `side-indent-num` is the same number without a unit, for the places
        // that have to do arithmetic (`calc(100% + 9px)`, `-9px`) where
        // appending `px` to a var would produce invalid CSS. Keep the two in
        // step: `side-indent` is `space-sm + 1px` — the
        // `c_euv_sidebar_children` margin plus its dashed border.
        //
        // This value must NOT be the full nesting inset. It is only the
        // distance from a row's own left edge back to the dashed guide that
        // introduced it, which is `margin-left + border-left-width`
        // (8 + 1 = 9px). The row's own `padding-left` is *not* part of it:
        // an active row keeps its padding so the label does not move, and the
        // negative margin grows the fill leftward into that padding. Using
        // the full inset (17px) pushed the active fill 8px past the dashed
        // guide, breaking the tree's left rail.
        side-indent: "9px";
        side-indent-num: "9";

        // ═══════════════════════════════════════════════════════════════════════
        // Font Size Scale (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (shadcn/ui aligned)
        // ═══════════════════════════════════════════════════════════════════════
        safe-area-inset-top: "env(safe-area-inset-top, 0px)";
        safe-area-inset-right: "env(safe-area-inset-right, 0px)";
        safe-area-inset-bottom: "env(safe-area-inset-bottom, 0px)";
        safe-area-inset-left: "env(safe-area-inset-left, 0px)";
        padding-shell-top: var!(safe-area-inset-top);
        padding-shell-bottom: var!(safe-area-inset-bottom);
        padding-main-top: "24px";
        padding-main-top-mobile: "16px";
        // Edge Gutter — distance from a viewport/shell edge to the chrome
        // anchored there. Every edge-anchored element (mobile header left and
        // right, drawer header, desktop navbar, main column, vconsole FAB)
        // reads one of these tokens, so left/right offsets can never drift
        // apart. `edge-gutter-nav` is the nav column's own inner gutter
        // (desktop sidebar and mobile drawer share it).
        edge-gutter: "28px";
        edge-gutter-mobile: "16px";
        edge-gutter-nav: "20px";
        edge-gutter-bottom: "20px";
        padding-main-horizontal: var!(edge-gutter);
        padding-main-horizontal-mobile: var!(edge-gutter-mobile);
        gap-page-header: "16px";
        gap-page-title: "6px";
        min-height-base: "36px";
        min-height-sm: "36px";
        // Desktop sidebar adaptive width:
        //   `clamp(min, 22%, max)` lets the column grow with the available
        //   width up to a comfortable cap so long English titles (e.g.
        //   "Internationalization") no longer wrap. On the mobile drawer the
        //   same var is consumed with `min(100%, max)` instead — see
        //   `c_app_nav` / drawer classes.
        //   A percentage is used rather than a viewport width: the nav's
        //   containing block is the app root row, which is the full window
        //   width, so the two agree — but the percentage stays correct if a
        //   future shell constrains that row (e.g. a split view), and it
        //   keeps viewport units out of the codebase.
        nav-width: "clamp(248px, 22%, 320px)";
        nav-width-min: "248px";
        nav-width-max: "320px";
        content-max-width: "820px";
        mobile-header-height: "52px";

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale
        // ═══════════════════════════════════════════════════════════════════════
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Page Block Vertical Spacing
        // ═══════════════════════════════════════════════════════════════════════
        page-block-gap: "24px";
        page-block-gap-mobile: "20px";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows (black alpha only)
        // ═══════════════════════════════════════════════════════════════════════
        shadow-sm: "0 1px 3px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-modal: "0 25px 50px -12px rgba(0, 0, 0, 0.18)";
        shadow-drawer: "4px 0 20px rgba(0, 0, 0, 0.08)";
        shadow-accent-sm: "0 1px 3px rgba(0, 0, 0, 0.08)";
        shadow-accent-lg: "0 10px 15px -3px rgba(0, 0, 0, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(0, 0, 0, 0.15)";
        scrollbar-thumb-hover: "rgba(0, 0, 0, 0.30)";
        scrollbar-thumb-active: "rgba(0, 0, 0, 0.45)";

        // ═══════════════════════════════════════════════════════════════════════
        // Console / VConsole
        // ═══════════════════════════════════════════════════════════════════════
        shadow-console-button: "0 4px 14px rgba(0, 0, 0, 0.15)";
        shadow-console-panel: "0 -8px 32px rgba(0, 0, 0, 0.06)";
    }

    pub c_theme_dark {
        // ═══════════════════════════════════════════════════════════════════════
        // Monochrome Design Tokens (Black & White only)
        // ═══════════════════════════════════════════════════════════════════════

        // ─── Surface ───
        background: "#000000";
        foreground: "#ffffff";

        // ─── Secondary / Muted ───
        muted-foreground: "#ffffff";

        // ─── Accent ───
        accent: "#ffffff";

        // ─── Border & Input ───
        border: "#ffffff";
        ring: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Legacy Surface Aliases
        // ═══════════════════════════════════════════════════════════════════════
        bg-overlay: "rgba(255, 255, 255, 0.08)";

        // ─── Text Colors ───
        text-on-accent: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand / Accent (White for dark minimalist)
        // ═══════════════════════════════════════════════════════════════════════
        accent: "#ffffff";
        accent-muted: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        space-2xs: "2px";
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-7xl: "80px";

        // Sidebar nesting indent, in two forms. `side-indent` carries its own
        // `px` so it can be dropped into a plain `var()` position;
        // `side-indent-num` is the same number without a unit, for the places
        // that have to do arithmetic (`calc(100% + 9px)`, `-9px`) where
        // appending `px` to a var would produce invalid CSS. Keep the two in
        // step: `side-indent` is `space-sm + 1px` — the
        // `c_euv_sidebar_children` margin plus its dashed border.
        //
        // This value must NOT be the full nesting inset. It is only the
        // distance from a row's own left edge back to the dashed guide that
        // introduced it, which is `margin-left + border-left-width`
        // (8 + 1 = 9px). The row's own `padding-left` is *not* part of it:
        // an active row keeps its padding so the label does not move, and the
        // negative margin grows the fill leftward into that padding. Using
        // the full inset (17px) pushed the active fill 8px past the dashed
        // guide, breaking the tree's left rail.
        side-indent: "9px";
        side-indent-num: "9";

        // ═══════════════════════════════════════════════════════════════════════
        // Font Size Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        safe-area-inset-top: "env(safe-area-inset-top, 0px)";
        safe-area-inset-right: "env(safe-area-inset-right, 0px)";
        safe-area-inset-bottom: "env(safe-area-inset-bottom, 0px)";
        safe-area-inset-left: "env(safe-area-inset-left, 0px)";
        padding-shell-top: var!(safe-area-inset-top);
        padding-shell-bottom: var!(safe-area-inset-bottom);
        padding-main-top: "24px";
        padding-main-top-mobile: "16px";
        // Edge Gutter — distance from a viewport/shell edge to the chrome
        // anchored there. Every edge-anchored element (mobile header left and
        // right, drawer header, desktop navbar, main column, vconsole FAB)
        // reads one of these tokens, so left/right offsets can never drift
        // apart. `edge-gutter-nav` is the nav column's own inner gutter
        // (desktop sidebar and mobile drawer share it).
        edge-gutter: "28px";
        edge-gutter-mobile: "16px";
        edge-gutter-nav: "20px";
        edge-gutter-bottom: "20px";
        padding-main-horizontal: var!(edge-gutter);
        padding-main-horizontal-mobile: var!(edge-gutter-mobile);
        gap-page-header: "16px";
        gap-page-title: "6px";
        min-height-base: "36px";
        min-height-sm: "36px";
        // Desktop sidebar adaptive width:
        //   `clamp(min, 22%, max)` lets the column grow with the available
        //   width up to a comfortable cap so long English titles (e.g.
        //   "Internationalization") no longer wrap. On the mobile drawer the
        //   same var is consumed with `min(100%, max)` instead — see
        //   `c_app_nav` / drawer classes.
        //   A percentage is used rather than a viewport width: the nav's
        //   containing block is the app root row, which is the full window
        //   width, so the two agree — but the percentage stays correct if a
        //   future shell constrains that row (e.g. a split view), and it
        //   keeps viewport units out of the codebase.
        nav-width: "clamp(248px, 22%, 320px)";
        nav-width-min: "248px";
        nav-width-max: "320px";
        content-max-width: "820px";
        mobile-header-height: "52px";

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Page Block Vertical Spacing (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        page-block-gap: "24px";
        page-block-gap-mobile: "20px";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows (white alpha for dark)
        // ═══════════════════════════════════════════════════════════════════════
        shadow-sm: "0 1px 3px rgba(255, 255, 255, 0.12), 0 1px 2px rgba(255, 255, 255, 0.06)";
        shadow-modal: "0 25px 50px -12px rgba(255, 255, 255, 0.25)";
        shadow-drawer: "4px 0 20px rgba(255, 255, 255, 0.12)";
        shadow-accent-sm: "0 1px 3px rgba(255, 255, 255, 0.15)";
        shadow-accent-lg: "0 10px 15px -3px rgba(255, 255, 255, 0.18)";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(255, 255, 255, 0.18)";
        scrollbar-thumb-hover: "rgba(255, 255, 255, 0.35)";
        scrollbar-thumb-active: "rgba(255, 255, 255, 0.50)";

        // ═══════════════════════════════════════════════════════════════════════
        // Console / VConsole
        // ═══════════════════════════════════════════════════════════════════════
        shadow-console-button: "0 4px 14px rgba(255, 255, 255, 0.15)";
        shadow-console-panel: "0 -8px 32px rgba(255, 255, 255, 0.08)";
    }
}
