use super::*;

// Design tokens shared by every UI scheme.

//

// The two classes below carry the **semantic mapping** — the scheme's role

// primitives selected for the active mode — plus the **universal structure

// scale** (spacing, motion, layout) that no design language changes.

//

// A scheme owns only the tokens that genuinely differ between design

// languages: the radius scale, the type ramp, the control heights and the

// colour roles themselves. Keeping the two sets disjoint is what lets a

// scheme class and a mode class sit on the same root element without either

// shadowing the other — the browser resolves each custom property from

// exactly one owner.

//

// The role primitives are read through `var!("role-…")` so the generated CSS

// is `var(--role-surface-light)` rather than a literal.

vars! {

    pub c_theme_light {

        // ═══════════════════════════════════════════════════════════════════════

        // Semantic Colours — the light half of the scheme's role primitives

        // ═══════════════════════════════════════════════════════════════════════

        background: var!("role-surface-light");

        foreground: var!("role-on-surface-light");

        muted-foreground: var!("role-on-surface-variant-light");

        accent: var!("role-primary-light");

        primary-hover: var!("role-primary-hover-light");

        border: var!("role-outline-light");

        ring: var!("role-focus-light");

        bg-overlay: var!("role-scrim-light");

        text-on-accent: var!("role-on-primary-light");

        accent-muted: var!("role-fill-subtle-light");

        surface-muted: var!("role-surface-container-light");

        surface-elevated: var!("role-surface-container-high-light");

        primary-container: var!("role-primary-container-light");

        error: var!("role-error-light");

        on-error: var!("role-on-error-light");

        error-container: var!("role-error-container-light");

        success: var!("role-success-light");

        success-container: var!("role-success-container-light");

        warning: var!("role-warning-light");

        warning-container: var!("role-warning-container-light");

        // ═══════════════════════════════════════════════════════════════════════

        // Elevation — the scheme's shadow ramp, flattened onto the roles the

        // registry actually reads.

        // ═══════════════════════════════════════════════════════════════════════

        shadow-sm: var!("role-shadow-control-light");

        shadow-modal: var!("role-shadow-modal-light");

        shadow-drawer: var!("role-shadow-drawer-light");

        shadow-accent-sm: var!("role-shadow-control-light");

        shadow-accent-lg: var!("role-shadow-raised-light");

        shadow-console-button: var!("role-shadow-console-button-light");

        shadow-console-panel: var!("role-shadow-console-panel-light");

        // ═══════════════════════════════════════════════════════════════════════

        // Scrollbar

        // ═══════════════════════════════════════════════════════════════════════

        scrollbar-track: "transparent";

        scrollbar-thumb: var!("role-stroke-neutral-light");

        scrollbar-thumb-hover: var!("role-stroke-neutral-strong-light");

        scrollbar-thumb-active: var!("role-stroke-neutral-strongest-light");

        // ═══════════════════════════════════════════════════════════════════════

        // Spacing Scale

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

        // ═══════════════════════════════════════════════════════════════════════

        // Transition Durations

        // ═══════════════════════════════════════════════════════════════════════

        duration-fast: "0.15s";

        duration-normal: "0.2s";

        duration-slower: "0.4s";

        duration-overlay: "0.2s";

        duration-modal-overlay: "0.15s";

        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════

        // Easing Functions

        // ═══════════════════════════════════════════════════════════════════════

        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";

        ease-in: "cubic-bezier(0.4, 0, 1, 1)";

        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";

        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";

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

        nav-width: "clamp(248px, 22%, 320px)";

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

        // Semantic Colours — the dark half of the scheme's role primitives

        // ═══════════════════════════════════════════════════════════════════════

        background: var!("role-surface-dark");

        foreground: var!("role-on-surface-dark");

        muted-foreground: var!("role-on-surface-variant-dark");

        accent: var!("role-primary-dark");

        primary-hover: var!("role-primary-hover-dark");

        border: var!("role-outline-dark");

        ring: var!("role-focus-dark");

        bg-overlay: var!("role-scrim-dark");

        text-on-accent: var!("role-on-primary-dark");

        accent-muted: var!("role-fill-subtle-dark");

        surface-muted: var!("role-surface-container-dark");

        surface-elevated: var!("role-surface-container-high-dark");

        primary-container: var!("role-primary-container-dark");

        error: var!("role-error-dark");

        on-error: var!("role-on-error-dark");

        error-container: var!("role-error-container-dark");

        success: var!("role-success-dark");

        success-container: var!("role-success-container-dark");

        warning: var!("role-warning-dark");

        warning-container: var!("role-warning-container-dark");

        // ═══════════════════════════════════════════════════════════════════════

        // Elevation

        // ═══════════════════════════════════════════════════════════════════════

        shadow-sm: var!("role-shadow-control-dark");

        shadow-modal: var!("role-shadow-modal-dark");

        shadow-drawer: var!("role-shadow-drawer-dark");

        shadow-accent-sm: var!("role-shadow-control-dark");

        shadow-accent-lg: var!("role-shadow-raised-dark");

        shadow-console-button: var!("role-shadow-console-button-dark");

        shadow-console-panel: var!("role-shadow-console-panel-dark");

        // ═══════════════════════════════════════════════════════════════════════

        // Scrollbar

        // ═══════════════════════════════════════════════════════════════════════

        scrollbar-track: "transparent";

        scrollbar-thumb: var!("role-stroke-neutral-dark");

        scrollbar-thumb-hover: var!("role-stroke-neutral-strong-dark");

        scrollbar-thumb-active: var!("role-stroke-neutral-strongest-dark");

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

        nav-width: "clamp(248px, 22%, 320px)";

        nav-width-min: "248px";

        nav-width-max: "320px";

        content-max-width: "820px";

        text-measure: "360px";

        mobile-header-height: "52px";

    }

}

/// Returns the CSS class name of the UI scheme compiled into this build.
///
/// The scheme is picked at compile time from the `theme-*` Cargo features.
/// More than one feature may be on at once — a workspace-wide `--all-features`
/// build turns them all on — so the order below decides which one wins.
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

/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(feature = "theme-antdesign")]
fn selected_scheme_class() -> &'static str {
    c_scheme_antdesign().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-antdesign"), feature = "theme-material"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_material().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-antdesign"), not(feature = "theme-material"), feature = "theme-bootstrap"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_bootstrap().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), feature = "theme-cupertino"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_cupertino().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(all(not(feature = "theme-antdesign"), not(feature = "theme-material"), not(feature = "theme-bootstrap"), not(feature = "theme-cupertino"), feature = "theme-fluent"))]
fn selected_scheme_class() -> &'static str {
    c_scheme_fluent().get_name()
}
/// Returns the CSS class name of the scheme this build selects.
///
/// # Returns
///
/// - `&'static str` - The class name that carries the scheme's tokens.
#[rustfmt::skip]
#[cfg(not(any(feature = "theme-antdesign", feature = "theme-material", feature = "theme-bootstrap", feature = "theme-cupertino", feature = "theme-fluent")))]
fn selected_scheme_class() -> &'static str {
    c_scheme_builtin().get_name()
}
