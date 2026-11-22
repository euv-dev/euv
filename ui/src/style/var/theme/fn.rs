use super::*;

// Design tokens shared by every UI scheme.
//
// The two classes below carry the **semantic mapping** — the scheme's role
// primitives selected for the active mode — plus the **universal structure
// scale** (spacing, motion, layout) that every mainstream design system
// agrees on: they all space on a four-pixel base.
//
// A scheme owns the tokens that genuinely differ between design languages:
// the colour roles, the radius scale, the shadow ramp, the type ramp, the
// tracking and case treatments, and the control heights. The two sets are
// disjoint by construction, so a scheme class and a mode class can sit on
// the same root element without either shadowing the other.
//
// The role primitives are read through `var!("role-…")` so the generated CSS
// is `var(--role-background-light)` rather than a literal.
vars! {
    pub c_theme_light {
        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces
        // ═══════════════════════════════════════════════════════════════════════

        background: var!("role-background-light");
        foreground: var!("role-foreground-light");
        bg-overlay: "rgba(0, 0, 0, 0.45)";
        card: var!("role-card-light");
        card-foreground: var!("role-card-foreground-light");
        popover: var!("role-popover-light");
        popover-foreground: var!("role-popover-foreground-light");

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals
        // ═══════════════════════════════════════════════════════════════════════

        primary: var!("role-primary-light");
        primary-foreground: var!("role-primary-foreground-light");
        primary-hover: var!("role-primary-hover-light");
        secondary: var!("role-secondary-light");
        secondary-foreground: var!("role-secondary-foreground-light");
        muted: var!("role-muted-light");
        muted-foreground: var!("role-muted-foreground-light");
        accent: var!("role-accent-light");
        accent-foreground: var!("role-accent-foreground-light");

        // ═══════════════════════════════════════════════════════════════════════
        // Status
        // ═══════════════════════════════════════════════════════════════════════

        destructive: var!("role-destructive-light");
        destructive-foreground: var!("role-destructive-foreground-light");
        button-primary-hover-bg: format!("color-mix(in srgb, {} 90%, transparent)", var!(primary));
        button-secondary-hover-bg: format!("color-mix(in srgb, {} 80%, transparent)", var!(secondary));
        button-destructive-bg: format!("color-mix(in srgb, {} 10%, transparent)", var!(destructive));
        button-destructive-hover-bg: format!("color-mix(in srgb, {} 20%, transparent)", var!(destructive));
        button-destructive-focus-border: format!("color-mix(in srgb, {} 40%, transparent)", var!(destructive));
        button-destructive-focus-ring: format!("color-mix(in srgb, {} 20%, transparent)", var!(destructive));
        button-outline-bg: var!(background);
        button-outline-hover-bg: var!(accent);
        button-outline-hover-fg: var!(accent-foreground);
        button-ghost-hover-bg: var!(accent);
        button-ghost-hover-fg: var!(accent-foreground);
        success: var!("role-success-light");
        success-foreground: var!("role-success-foreground-light");
        success-container: var!("role-success-container-light");
        warning: var!("role-warning-light");
        warning-foreground: var!("role-warning-foreground-light");
        warning-container: var!("role-warning-container-light");
        error-container: var!("role-error-container-light");

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border: var!("role-border-light");
        input: var!("role-input-light");
        ring: var!("role-ring-light");

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════

        scrollbar-track: "transparent";
        scrollbar-thumb: var!("role-scrollbar-thumb-light");
        scrollbar-thumb-hover: var!("role-scrollbar-thumb-hover-light");
        scrollbar-thumb-active: var!("role-scrollbar-thumb-active-light");

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale — four pixels, the base every mainstream system uses.
        // ═══════════════════════════════════════════════════════════════════════

        control-pad-inline: "10px";
        control-pad-icon: "32px";
        tablist-inset: "3px";
        switch-track-height: "18.4px";
        space-2xs: "2px";
        space-xs: "4px";
        space-xs-sm: "6px";
        space-sm: "8px";
        space-sm-md: "10px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-7xl: "80px";

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale
        // ═══════════════════════════════════════════════════════════════════════

        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";
        gap-page-header: "16px";
        gap-page-title: "6px";

        // ═══════════════════════════════════════════════════════════════════════
        // Page Block Vertical Spacing
        // ═══════════════════════════════════════════════════════════════════════

        page-block-gap: "24px";
        page-block-gap-mobile: "20px";
        card-padding: var!(space-md);
        card-content-gap: var!(gap-component);
        nav-row-height: format!("calc({} + {})", var!(control-height), var!(space-lg));
        nav-row-padding: var!(space-md);
        nav-item-inset: var!(space-xs);
        // The width of the lock-in bar a table-of-contents row paints on
        // hover. It is narrower than the sidebar's 5px bar, because the
        // table of contents is a narrower column and a full-width bar
        // would read as a heavy rule rather than as a marker.
        toc-bar-width: "3px";
        side-indent: "9px";
        side-indent-num: "9";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations
        // ═══════════════════════════════════════════════════════════════════════

        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";
        // A press has to land before the finger leaves the surface, so the
        // press is the one transition that runs faster than `duration-fast`.
        duration-press: "0.1s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions
        // ═══════════════════════════════════════════════════════════════════════

        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";
        // A surface that enters decelerates into its resting place and one
        // that leaves accelerates away from it; a single `ease-out` makes
        // both directions read as sliding in from nowhere.
        ease-emphasized: "cubic-bezier(0.05, 0.7, 0.1, 1)";
        ease-accelerate: "cubic-bezier(0.3, 0, 0.8, 0.15)";

        // ═══════════════════════════════════════════════════════════════════════
        // Interaction
        // ═══════════════════════════════════════════════════════════════════════

        // Shared press depth; hover feedback never changes geometry.
        press-scale: "0.97";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout
        // ═══════════════════════════════════════════════════════════════════════

        safe-area-inset-top: "var(--euv-mobile-safe-top, 0px)";
        safe-area-inset-right: "var(--euv-safe-right, 0px)";
        safe-area-inset-bottom: "var(--euv-safe-bottom, 0px)";
        safe-area-inset-left: "var(--euv-safe-left, 0px)";
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
        nav-width: format!("clamp({}, 22%, {})", var!(nav-width-min), var!(nav-width-max));
        nav-width-min: "248px";
        nav-width-max: "320px";
        content-max-width: "820px";
        // The measure a body of text is set to — the line length a reader
        // can take in without losing the line. Narrower than the content
        // column, so prose stays readable inside a full-width card.
        text-measure: "360px";
        mobile-header-height: "52px";
    }

    pub c_theme_dark {
        // ═══════════════════════════════════════════════════════════════════════
        // Surfaces
        // ═══════════════════════════════════════════════════════════════════════

        background: var!("role-background-dark");
        foreground: var!("role-foreground-dark");
        bg-overlay: "rgba(0, 0, 0, 0.65)";
        card: var!("role-card-dark");
        card-foreground: var!("role-card-foreground-dark");
        popover: var!("role-popover-dark");
        popover-foreground: var!("role-popover-foreground-dark");

        // ═══════════════════════════════════════════════════════════════════════
        // Brand and Neutrals
        // ═══════════════════════════════════════════════════════════════════════

        primary: var!("role-primary-dark");
        primary-foreground: var!("role-primary-foreground-dark");
        primary-hover: var!("role-primary-hover-dark");
        secondary: var!("role-secondary-dark");
        secondary-foreground: var!("role-secondary-foreground-dark");
        muted: var!("role-muted-dark");
        muted-foreground: var!("role-muted-foreground-dark");
        accent: var!("role-accent-dark");
        accent-foreground: var!("role-accent-foreground-dark");

        // ═══════════════════════════════════════════════════════════════════════
        // Status
        // ═══════════════════════════════════════════════════════════════════════

        destructive: var!("role-destructive-dark");
        destructive-foreground: var!("role-destructive-foreground-dark");
        button-primary-hover-bg: format!("color-mix(in srgb, {} 90%, transparent)", var!(primary));
        button-secondary-hover-bg: format!("color-mix(in srgb, {} 80%, transparent)", var!(secondary));
        button-destructive-bg: format!("color-mix(in srgb, {} 20%, transparent)", var!(destructive));
        button-destructive-hover-bg: format!("color-mix(in srgb, {} 30%, transparent)", var!(destructive));
        button-destructive-focus-border: format!("color-mix(in srgb, {} 40%, transparent)", var!(destructive));
        button-destructive-focus-ring: format!("color-mix(in srgb, {} 40%, transparent)", var!(destructive));
        button-outline-bg: format!("color-mix(in srgb, {} 30%, transparent)", var!(input));
        button-outline-hover-bg: format!("color-mix(in srgb, {} 50%, transparent)", var!(input));
        button-outline-hover-fg: var!(foreground);
        button-ghost-hover-bg: format!("color-mix(in srgb, {} 50%, transparent)", var!(accent));
        button-ghost-hover-fg: var!(accent-foreground);
        success: var!("role-success-dark");
        success-foreground: var!("role-success-foreground-dark");
        success-container: var!("role-success-container-dark");
        warning: var!("role-warning-dark");
        warning-foreground: var!("role-warning-foreground-dark");
        warning-container: var!("role-warning-container-dark");
        error-container: var!("role-error-container-dark");

        // ═══════════════════════════════════════════════════════════════════════
        // Lines and Focus
        // ═══════════════════════════════════════════════════════════════════════

        border: var!("role-border-dark");
        input: var!("role-input-dark");
        ring: var!("role-ring-dark");

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════

        scrollbar-track: "transparent";
        scrollbar-thumb: var!("role-scrollbar-thumb-dark");
        scrollbar-thumb-hover: var!("role-scrollbar-thumb-hover-dark");
        scrollbar-thumb-active: var!("role-scrollbar-thumb-active-dark");

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════

        control-pad-inline: "10px";
        control-pad-icon: "32px";
        tablist-inset: "3px";
        switch-track-height: "18.4px";
        space-2xs: "2px";
        space-xs: "4px";
        space-xs-sm: "6px";
        space-sm: "8px";
        space-sm-md: "10px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-7xl: "80px";

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════

        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";
        gap-page-header: "16px";
        gap-page-title: "6px";

        // ═══════════════════════════════════════════════════════════════════════
        // Page Block Vertical Spacing (same as light)
        // ═══════════════════════════════════════════════════════════════════════

        page-block-gap: "24px";
        page-block-gap-mobile: "20px";
        card-padding: var!(space-md);
        card-content-gap: var!(gap-component);
        nav-row-height: format!("calc({} + {})", var!(control-height), var!(space-lg));
        nav-row-padding: var!(space-md);
        nav-item-inset: var!(space-xs);
        toc-bar-width: "3px";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (same as light)
        // ═══════════════════════════════════════════════════════════════════════

        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";
        // A press has to land before the finger leaves the surface, so the
        // press is the one transition that runs faster than `duration-fast`.
        duration-press: "0.1s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (same as light)
        // ═══════════════════════════════════════════════════════════════════════

        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";
        // A surface that enters decelerates into its resting place and one
        // that leaves accelerates away from it; a single `ease-out` makes
        // both directions read as sliding in from nowhere.
        ease-emphasized: "cubic-bezier(0.05, 0.7, 0.1, 1)";
        ease-accelerate: "cubic-bezier(0.3, 0, 0.8, 0.15)";

        // ═══════════════════════════════════════════════════════════════════════
        // Interaction
        // ═══════════════════════════════════════════════════════════════════════

        // Shared press depth; hover feedback never changes geometry.
        press-scale: "0.97";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (same as light)
        // ═══════════════════════════════════════════════════════════════════════

        safe-area-inset-top: "var(--euv-mobile-safe-top, 0px)";
        safe-area-inset-right: "var(--euv-safe-right, 0px)";
        safe-area-inset-bottom: "var(--euv-safe-bottom, 0px)";
        safe-area-inset-left: "var(--euv-safe-left, 0px)";
        padding-shell-top: var!(safe-area-inset-top);
        padding-shell-bottom: var!(safe-area-inset-bottom);
        padding-main-top: "24px";
        padding-main-top-mobile: "16px";
        edge-gutter: "28px";
        edge-gutter-mobile: "16px";
        edge-gutter-nav: "20px";
        edge-gutter-bottom: "20px";
        padding-main-horizontal: var!(edge-gutter);
        padding-main-horizontal-mobile: var!(edge-gutter-mobile);
        nav-width: format!("clamp({}, 22%, {})", var!(nav-width-min), var!(nav-width-max));
        nav-width-min: "248px";
        nav-width-max: "320px";
        content-max-width: "820px";
        text-measure: "360px";
        mobile-header-height: "52px";
    }
}

/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(feature = "theme-shadcn")]
fn selected_scheme_class() -> &'static str {
    c_scheme_shadcn().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), feature = "theme-monochrome"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_monochrome().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), feature = "theme-antdesign"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_antdesign().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), feature = "theme-material"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_material().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), feature = "theme-bootstrap"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_bootstrap().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), feature = "theme-cupertino"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_cupertino().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), feature = "theme-fluent"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_fluent().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), feature = "theme-radix"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_radix().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), feature = "theme-mantine"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_mantine().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), feature = "theme-daisyui"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_daisyui().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), feature = "theme-heroui"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_heroui().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), not(feature = "theme-heroui"), feature = "theme-park"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_park().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), not(feature = "theme-heroui"), not(feature = "theme-park"), feature = "theme-geist"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_geist().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), not(feature = "theme-heroui"), not(feature = "theme-park"), not(feature = "theme-geist"), feature = "theme-linear"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_linear().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), not(feature = "theme-heroui"), not(feature = "theme-park"), not(feature = "theme-geist"), not(feature = "theme-linear"), feature = "theme-stripe"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_stripe().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), not(feature = "theme-heroui"), not(feature = "theme-park"), not(feature = "theme-geist"), not(feature = "theme-linear"), not(feature = "theme-stripe"), feature = "theme-notion"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_notion().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-monochrome"), not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), not(feature = "theme-fluent"), not(feature = "theme-radix"), not(feature = "theme-mantine"), not(feature = "theme-daisyui"), not(feature = "theme-heroui"), not(feature = "theme-park"), not(feature = "theme-geist"), not(feature = "theme-linear"), not(feature = "theme-stripe"), not(feature = "theme-notion"), feature = "theme-supabase"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_supabase().get_name()
}

/// Returns the CSS class name for the built-in monochrome default.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the monochrome scheme tokens.
#[rustfmt::skip]
#[cfg(not(any(feature = "theme-shadcn", feature = "theme-monochrome", feature = "theme-antdesign", feature = "theme-material", feature = "theme-bootstrap", feature = "theme-cupertino", feature = "theme-fluent", feature = "theme-radix", feature = "theme-mantine", feature = "theme-daisyui", feature = "theme-heroui", feature = "theme-park", feature = "theme-geist", feature = "theme-linear", feature = "theme-stripe", feature = "theme-notion", feature = "theme-supabase")))]
fn selected_scheme_class() -> &'static str {
    c_scheme_monochrome().get_name()
}

/// Returns the CSS class name of the UI scheme compiled into this build.
///
/// The scheme is picked at compile time from the `theme-*` Cargo features.
/// More than one feature may be on at once — a workspace-wide `--all-features`
/// build turns them all on — so the order above decides which one wins.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
pub fn scheme_class_name() -> &'static str {
    selected_scheme_class()
}

/// Returns the name of the UI scheme compiled into this build.
///
/// # Returns
///
/// - `&'static str` - The scheme name, without its class prefix.
pub fn scheme_name() -> &'static str {
    let class: &str = scheme_class_name();
    class.strip_prefix(SCHEME_CLASS_PREFIX).unwrap_or(class)
}
