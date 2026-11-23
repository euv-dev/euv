use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Layout Shell
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_app_root {
        display: "flex";
        height: "100%";
        padding-bottom: var!(safe-area-inset-bottom);
        font-family: var!(font-family);
        background: var!(background);
        color: var!(foreground);
        user-select: "none";
        -webkit-user-select: "none";
        -webkit-font-smoothing: "antialiased";
        -moz-osx-font-smoothing: "grayscale";
        text-rendering: "optimizeLegibility";
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: var!(radius-full);
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
        }

        // ═══════════════════════════════════════════════════════════════════════════
        // CSS Reset — Cross-browser normalization
        // ═══════════════════════════════════════════════════════════════════════════

        *, *::before, *::after {
            box-sizing: "border-box";
        }
        h1, h2, h3, h4, h5, h6, p, ul, ol, dl, dd, figure, blockquote, pre, hr {
            margin: "0px";
            padding: "0px";
        }
        ul, ol {
            list-style: "none";
            padding: "0px";
            margin: "0px";
        }
        a {
            color: "inherit";
            text-decoration: "none";
        }
        button {
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
            background: "transparent";
            border: "none";
            padding: "0px";
            margin: "0px";
            font: "inherit";
            color: "inherit";
            cursor: "pointer";
            outline: "none";
            font-family: "inherit";
            ::-moz-focus-inner {
                border: "0px";
                padding: "0px";
            }
        }
        input[type="text"], input[type="number"], input[type="email"], input[type="password"], input[type="search"], input[type="tel"], input[type="url"] {
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
            border-radius: var!(radius-sm);
        }
        input[type="search"]::-webkit-search-cancel-button, input[type="search"]::-webkit-search-decoration {
            -webkit-appearance: "none";
            appearance: "none";
        }
        input[type="number"] {
            -moz-appearance: "textfield";
        }
        input[type="number"]::-webkit-outer-spin-button, input[type="number"]::-webkit-inner-spin-button {
            -webkit-appearance: "none";
            appearance: "none";
            margin: "0px";
        }
        textarea {
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
            resize: "vertical";
            font-family: "inherit";
            border-radius: var!(radius-sm);
        }
        select {
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
            background-image: "none";
            border-radius: var!(radius-sm);
        }
        img, svg, video, canvas, audio, iframe, embed, object {
            display: "block";
            max-width: "100%";
        }
        table {
            border-collapse: "collapse";
            border-spacing: "0px";
        }
        hr {
            border: "none";
            margin: "0px";
        }
        :focus-visible {
            outline: "none";
        }
        : focus:not(: focus-visible) {
            outline: "none";
        }
        h1, h2, h3, h4, h5, h6 {
            font-weight: "inherit";
            font-size: "inherit";
        }
        strong, b {
            font-weight: var!(font-weight-bold);
        }
        em, i {
            font-style: "italic";
        }
        code, pre, kbd, samp {
            font-family: var!(font-family-mono);
        }
        @media ((max-width: 767px)) {
            input, select, textarea {
                font-size: "16px";
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Navigation - Desktop Sidebar
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_app_nav {
        // Adaptive sidebar width — `clamp()` keeps the column ≥ the historic
        // 248px floor (no regression on dense sidebar trees) and ≤ 320px on
        // roomy viewports, so longer English titles like "Internationalization"
        // stop wrapping at the historic fixed 248px.
        width: var!(nav-width);
        background: var!(background);
        border-right: format!("{} solid {}", var!(border-width), var!(border));
        display: "flex";
        flex-direction: "column";
        // Pin the sidebar to the viewport while the main column scrolls.
        // Without this the whole row flex container scrolls as a single
        // block and the nav appears to slide off-screen with the content.
        // `align-self: stretch` (the row flex default) is overridden by
        // `flex-start` so the sticky height is the natural full height and
        // not stretched to match the (much taller) main column.
        // `100%` rather than `100vh`: the app root is a `height: 100%`
        // flex column, so a percentage resolves against the real available
        // box (viewport minus any safe-area inset) instead of the whole
        // window — on a device with a notch the viewport unit would make the
        // nav taller than the screen and push its footer below the fold.
        position: "sticky";
        top: "0px";
        align-self: "flex-start";
        height: "100%";
        flex-shrink: "0";
        padding-top: var!(safe-area-inset-top);
        @media ((max-width: 767px)) {
            display: "none";
        }
    }

    pub c_nav_header {
        padding: format!("{} {}", var!(space-xl), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-xl), var!(edge-gutter-mobile));
        }
        width: "100%";
        box-sizing: "border-box";
        font-size: var!(font-xl);
        font-weight: var!(font-weight-bold);
        color: var!(foreground);
        letter-spacing: var!(letter-spacing-tight);
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        flex-shrink: "0";
        text-decoration: "none";
        cursor: "pointer";
        position: "relative";
        ::after {
            content: "''";
            position: "absolute";
            bottom: "0px";
            left: var!(edge-gutter-nav);
            right: var!(edge-gutter-nav);
            height: "1px";
            background: format!("linear-gradient(90deg, transparent, {}, transparent)", var!(border));
        }
        @media ((max-width: 767px)) {
            ::after {
                left: var!(edge-gutter-mobile);
                right: var!(edge-gutter-mobile);
            }
        }
    }

    pub c_nav_brand_title {
        font-size: var!(font-xl);
        font-weight: var!(font-weight-bold);
        color: var!(foreground);
        letter-spacing: var!(letter-spacing-tight);
    }

    pub c_nav_section_label {
        padding: format!("{} {} {} {}", var!(space-md), var!(edge-gutter-nav), var!(space-xs), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {} {} {}", var!(space-md), var!(edge-gutter-mobile), var!(space-xs), var!(edge-gutter-mobile));
        }
        margin: "0px";
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        color: var!(foreground);
        text-transform: var!(text-transform-uppercase);
        letter-spacing: var!(letter-spacing-wide);
        flex-shrink: "0";
    }

    pub c_nav_items_scroll {
        flex: "1";
        min-height: "0px";
        padding: format!("{} {}", var!(space-sm), var!(nav-item-inset));
        // `stable both-edges` reserves the scrollbar gutter on BOTH sides of
        // the column, which pushed every nav row 11px away from the sidebar
        // edge on the left as well. The column scrolls vertically, so the
        // gutter belongs on the right only; the left keeps just the column's
        // own padding, which is the breathing room the nav needs.
        scrollbar-gutter: "stable";
        scrollbar-width: "thin";
        ::-webkit-scrollbar {
            width: var!(space-xs-sm);
        }
        overflow-x: "hidden";
        overflow-y: "auto";
        contain: "content";
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: var!(radius-full);
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
        }
    }

    pub c_nav_theme_toggle {
        // The bottom padding here and the top padding of `c_nav_footer` add
        // up across the boundary between the two rows, while only the
        // footer's own bottom padding separates the text from the sidebar
        // edge. Keeping the toggle's vertical padding at `space-sm` puts
        // the attribution line in the middle of the band it sits in instead
        // of hugging the bottom, without moving the button itself.
        padding: format!("{} {}", var!(space-sm), var!(edge-gutter-nav));
        flex-shrink: "0";
        margin-top: "auto";
        @media ((max-width: 767px)) {
            display: "none";
        }
    }

    // Optional row in the nav column / drawer for locale switchers or similar
    // widgets, placed between the brand header and the section label.
    pub c_nav_locale_row {
        padding: format!("{} {}", var!(space-md), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-md), var!(edge-gutter-mobile));
        }
        flex-shrink: "0";
    }

    pub c_nav_locale_button {
        width: "100%";
        height: "36px";
        padding: "0px";
        cursor: "pointer";
        outline: "none";
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: "transparent";
        color: var!(foreground);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        // Hover changes the border only. A background wash was removed here
        // for the same reason it was removed from `c_euv_dropdown_item`: the
        // locale switcher sits directly above the sidebar, and a tinted
        // block under the label competed with the page's own scroll position
        // and made the control look like a selected page rather than a
        // pending click.
        :hover {
            border-color: var!(foreground);
        }
        :focus-visible {
            outline: "none";
        }
        // `:active` must not reset the hover border either: it only fires
        // while the pointer is held, and dropping back to the base border
        // there made the press look like the hover never registered.
        :active {
            border-color: var!(foreground);
        }
        border-radius: var!(radius-md);
    }

    // Open state of the locale switcher: the same solid treatment the sidebar
    // gives its current page, so "menu is open" reads as strongly as
    // "this is the page you are on".
    pub c_nav_locale_button_open {
        width: "100%";
        height: "36px";
        padding: "0px";
        cursor: "pointer";
        outline: "none";
        border: format!("{} solid {}", var!(border-width), var!(primary));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        // Background and text colour are a matched pair: the fill is the
        // accent, so the label must be the colour that reads on top of it.
        // Setting one without the other is what produces invisible text.
        background: var!(primary);
        color: var!(primary-foreground);
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
            border-color: var!(primary);
        }
        :focus-visible {
            outline: "none";
        }
        :active {
            background: var!(primary);
            color: var!(primary-foreground);
            border-color: var!(primary);
        }
    }

    pub c_nav_theme_button {
        width: "100%";
        height: "36px";
        padding: "0px";
        cursor: "pointer";
        outline: "none";
        border: format!("{} solid {}", var!(border-width), var!(border));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        :focus-visible {
            outline: "none";
        }
        :active {
            background: "transparent";
            border-color: var!(border);
        }
        border-radius: var!(radius-md);
    }

    pub c_theme_icon_sun {
        width: "20px";
        height: "20px";
        background-repeat: "no-repeat";
        background-position: "center";
        background-size: "20px 20px";
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='20' viewBox='0 0 24 24' fill='%23ffffff' stroke='%23ffffff' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'%3E%3Ccircle cx='12' cy='12' r='5'/%3E%3Cline x1='12' y1='1' x2='12' y2='4'/%3E%3Cline x1='12' y1='20' x2='12' y2='23'/%3E%3Cline x1='4.22' y1='4.22' x2='6.34' y2='6.34'/%3E%3Cline x1='17.66' y1='17.66' x2='19.78' y2='19.78'/%3E%3Cline x1='1' y1='12' x2='4' y2='12'/%3E%3Cline x1='20' y1='12' x2='23' y2='12'/%3E%3Cline x1='4.22' y1='19.78' x2='6.34' y2='17.66'/%3E%3Cline x1='17.66' y1='6.34' x2='19.78' y2='4.22'/%3E%3C/svg%3E\")";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
    }

    pub c_theme_icon_moon {
        width: "20px";
        height: "20px";
        background-repeat: "no-repeat";
        background-position: "center";
        background-size: "20px 20px";
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='20' viewBox='0 0 24 24' fill='%23000000' stroke='none' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M21 13A9 9 0 1 1 11 3a7 7 0 0 0 10 10z'/%3E%3C/svg%3E\")";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
    }

    pub c_nav_footer {
        // The attribution line sits between the divider painted at `top: 0`
        // and the bottom of the sidebar's visible area, so the two vertical
        // paddings are what decide whether it looks centred in that band.
        // `space-md` is the tighter of the two old values and keeps the row
        // compact against the bottom edge.
        padding: format!("{} {} {}", var!(space-md), var!(edge-gutter-nav), var!(space-md));
        // On mobile the top padding also carries the bottom safe-area inset.
        //
        // The band this line has to look centred in runs from the divider to
        // the *page* bottom, not to the bottom of the element that reserves
        // the safe area. On desktop that element is `c_app_root`, whose
        // `padding-bottom` lifts the whole sidebar and leaves the inset
        // region below the footer empty — the symmetric `space-md` pair is
        // already correct there. The mobile drawer cannot inherit that: it is
        // `position: fixed`, so it escapes `c_app_root`'s padded box entirely
        // and reserves `padding-shell-bottom` itself. The inset therefore
        // lands *below* the footer rather than above it, and a symmetric pair
        // would leave a gap under the line of `inset` px and `space-md` above
        // it — a 34px inset turned a 42px band into a visibly lopsided one.
        //
        // Adding the inset to the top padding restores the symmetry against
        // the page bottom at any inset value, which is the invariant that
        // matters: the band grows upward (the footer is the last row of a
        // column flex container, so it has room to) and the line stays put
        // relative to the bottom edge instead of drifting up as the inset
        // grows.
        @media ((max-width: 767px)) {
            padding-top: format!("calc({} + {})", var!(space-md), var!(safe-area-inset-bottom));
            padding-right: var!(edge-gutter-mobile);
            padding-bottom: var!(space-md);
        }
        position: "relative";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        flex-shrink: "0";
        text-decoration: "none";
        display: "flex";
        // Both axes: the row is a single line of text, so centring it
        // vertically inside the padded box is what puts the glyphs in the
        // middle of the band rather than riding the top edge.
        align-items: "center";
        // Center the "Built with X" attribution line inside the sidebar so
        // it visually balances with the centered brand header above and the
        // centered theme toggle divider. `justify-content: center` works
        // here because the link itself is a single row of inline text —
        // when it wraps to multiple lines on narrow sidebars the lines stay
        // grouped because `flex-direction` is the default `row`.
        justify-content: "center";
        gap: var!(space-xs);
        cursor: "pointer";
        text-align: "center";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
        opacity: "1";
        :hover {
            opacity: "1";
        }
    }

    pub c_nav_footer_divider {
        position: "absolute";
        top: "0";
        left: var!(edge-gutter-nav);
        right: var!(edge-gutter-nav);
        height: "1px";
        background: format!("linear-gradient(90deg, transparent, {}, transparent)", var!(border));
        @media ((max-width: 767px)) {
            left: var!(edge-gutter-mobile);
            right: var!(edge-gutter-mobile);
        }
    }

    pub c_nav_footer_text {
        font-weight: var!(font-weight-normal);
        letter-spacing: var!(letter-spacing-normal);
    }

    pub c_nav_footer_brand {
        font-weight: var!(font-weight-bold);
        color: var!(primary);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Main Content Area
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_app_main {
        flex: "1";
        height: "100%";
        overflow: "auto";
        padding: format!(
        "{} {} {} {}",
        var!(padding-main-top),
        var!(padding-main-horizontal),
        var!(edge-gutter-bottom),
        var!(edge-gutter)
        );
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar {
            width: "6px";
        }
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: var!(radius-full);
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
        }
        @media ((max-width: 767px)) {
            padding: format!(
            "{} {} {} {}",
            var!(padding-main-top-mobile),
            var!(padding-main-horizontal-mobile),
            var!(edge-gutter-bottom),
            var!(edge-gutter-mobile)
            );
            scrollbar-width: "none";
            ::-webkit-scrollbar {
                width: "0px";
            }
        }
    }

    pub c_page_router {
        flex: "1";
        display: "flex";
        flex-direction: "column";
    }

    pub c_page_container {
        width: "100%";
        margin: "0px auto";
        max-width: var!(content-max-width);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Mobile Layout
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_mobile_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: format!("var(--euv-mobile-safe-top, 0px) {} 0px {}", var!(edge-gutter-mobile), var!(edge-gutter-mobile));
        height: format!("calc({} + var(--euv-mobile-safe-top, 0px))", var!(mobile-header-height));
        flex-shrink: "0";
        position: "sticky";
        top: "0px";
        z-index: "100";
        contain: "content";
        background: var!(background);
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_mobile_header_left {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
    }

    pub c_mobile_header_logo {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-sm));
        text-decoration: "none";
        color: var!(foreground);
    }

    pub c_mobile_menu_button {
        width: "40px";
        height: "40px";
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        font-weight: var!(font-weight-bold);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(foreground);
        padding: "0px";
        :active {
            background: "transparent";
            color: var!(foreground);
        }
        border-radius: var!(radius-md);
    }

    pub c_mobile_theme_button {
        width: "40px";
        height: "40px";
        border: "none";
        cursor: "pointer";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: "0px";
        :active {
            background: "transparent";
        }
        border-radius: var!(radius-md);
    }

    pub c_mobile_menu_button_active {
        width: "40px";
        height: "40px";
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        font-weight: var!(font-weight-bold);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(primary);
        padding: "0px";
        :active {
            background: var!(background);
            color: var!(primary);
        }
    }

    pub c_mobile_drawer_close_button {
        width: "32px";
        height: "32px";
        border: "none";
        cursor: "pointer";
        font-size: "18px";
        font-weight: var!(font-weight-medium);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(muted-foreground);
        padding: "0px";
        transition: format!("color {} {}", var!(duration-fast), var!(ease-out));
        :hover {
            color: var!(foreground);
        }
        :active {
            color: var!(muted-foreground);
        }
        border-radius: var!(radius-md);
    }

    pub c_mobile_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        width: "100%";
        height: "100%";
        background: var!(bg-overlay);
        z-index: "200";
        contain: "layout style paint";
        transition: format!("opacity {} {}", var!(duration-overlay), var!(ease-out));
    }

    pub c_mobile_overlay_hidden {
        opacity: "0";
        pointer-events: "none";
    }

    pub c_mobile_nav_drawer {
        position: "fixed";
        top: "0px";
        left: "0px";
        width: "240px";
        max-width: "100%";
        height: "100%";
        background: var!(background);
        z-index: "201";
        display: "flex";
        flex-direction: "column";
        padding-top: "var(--euv-mobile-safe-top, 0px)";
        padding-bottom: var!(padding-shell-bottom);
        contain: "layout style paint";
        will-change: "transform";
        transition: format!("transform {} {}", var!(duration-overlay), var!(ease-out));
        overflow: "hidden";
        box-shadow: var!(shadow-lg);
    }

    pub c_mobile_nav_drawer_closed {
        transform: "translateX(-100%)";
    }

    pub c_mobile_nav_drawer_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: format!("0px {}", var!(edge-gutter-mobile));
        height: var!(mobile-header-height);
        flex-shrink: "0";
        position: "relative";
        ::after {
            content: "''";
            position: "absolute";
            bottom: "0px";
            left: var!(edge-gutter-mobile);
            right: var!(edge-gutter-mobile);
            height: "1px";
            background: format!("linear-gradient(90deg, transparent, {}, transparent)", var!(border));
        }
    }

    pub c_mobile_main {
        c_app_main();
        ::-webkit-scrollbar {
            width: "0px";
        }
    }

    pub c_euv_navbar_icon_button {
        width: "36px";
        height: "36px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        border: format!("{} solid {}", var!(border-width), var!(border));
        cursor: "pointer";
        font-size: var!(font-base);
        :hover {
            background: var!(muted);
        }
        border-radius: var!(radius-md);
    }

    pub c_euv_sidebar_children {
        display: "flex";
        flex-direction: "column";
        // The dashed border is this level's left edge, so it has to paint on
        // the text x of the parent title above it, at every depth. The parent
        // label sits one `space-md` right of this container's content-box
        // left, which is what `margin-left` places. `space-sm` left the guide
        // 4px short of the parent label at every depth (measured 8/12, 25/29,
        // 42/46 for guide/label at depth 0/1/2).
        //
        // `padding-left` stays `space-sm`: it is the guide-to-child gap and
        // half of the active row's pull-back, so widening it would move the
        // marker and the text together.
        margin-left: var!(space-md);
        padding-left: var!(space-sm);
        border-left: format!("{} solid {}", var!(border-width), var!(border));
        // A group of links expands into view, so it decelerates into place
        // rather than sliding in at a constant rate.
        animation: format!("euv-fade-in {} {}", var!(duration-normal), var!(ease-emphasized));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_toc
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_toc {
        // The outer `c_euv_doc_toc` column is already sticky in
        // `c_app_main`'s scroll context — adding another sticky offset here
        // would double the offset and the TOC would slide past the top of
        // the scrollable area instead of staying pinned. Keep the inner
        // toc purely as a layout container.
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        border: "none";
    }

    pub c_euv_toc_link {
        border: "none";
        box-shadow: "none";
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        cursor: "pointer";
        line-height: "1.5";
        // Regular weight by default: only the *active* entry carries weight,
        // so hovering an inactive entry must not bold it or the weight would
        // read as a false "you are here" cue on mouse-over alone.
        font-weight: var!(font-weight-normal);
        // The row is the positioning context of its own left bar, the same
        // mechanism the sidebar tree uses for the row under the pointer.
        position: "relative";
        // Hover paints a left bar and leaves the row's own background
        // transparent. The bar is an inset shadow rather than a border,
        // because a border takes part in the box's layout and would move
        // the text when the state changes; an inset shadow is painted
        // inside the existing padding box and moves nothing.
        //
        // The bar is narrower than the sidebar's, because the table of
        // contents is a narrower column and a full-width bar would read
        // as a heavy rule rather than as a marker.
        :hover {
            color: var!(primary);
            box-shadow: format!("inset {} 0 0 0 {}", var!(toc-bar-width), var!(foreground));
        }
        border-radius: "0";
    }

    pub c_euv_toc_link_active {
        c_euv_toc_link();
        // The active entry changes its fill, the same way the sidebar's
        // active row does, so the current section is unmistakable.
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-bold);
        box-shadow: format!("inset {} 0 0 0 {}", var!(toc-bar-width), var!(primary));
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
            box-shadow: format!("inset {} 0 0 0 {}", var!(toc-bar-width), var!(primary));
        }
    }

    pub(crate) c_euv_toc_link_separator {
        :is(.c_euv_toc_link_separator) {
            // The rows are separated by the lock-in bar and the active
            // fill, not by a rule: a solid line under every row read as a
            // table border, and the hover had nothing left to show.
            border-bottom: "none";
        }
    }

    pub c_euv_toc_link_nested {
        c_euv_toc_link();
        padding-left: var!(space-lg);
    }

    pub c_euv_toc_link_nested_active {
        c_euv_toc_link_nested();
        // The nested active row carries the same fill and the same bar as
        // the active row it nests, so the current section looks the same
        // at every depth.
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-bold);
        box-shadow: format!("inset {} 0 0 0 {}", var!(toc-bar-width), var!(primary));
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
            box-shadow: format!("inset {} 0 0 0 {}", var!(toc-bar-width), var!(primary));
        }
    }

    pub(crate) c_mobile_app_root {
        display: "flex";
        flex-direction: "column";
        width: "100%";
        height: "100%";
        font-family: var!(font-family);
        background: var!(background);
        color: var!(foreground);
        padding: format!("0px {} {} {}", var!(safe-area-inset-right), var!(padding-shell-bottom), var!(safe-area-inset-left));
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: var!(radius-full);
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
        }
    }

    pub(crate) c_euv_logo {
        display: "flex";
        background: var!(primary);
        align-items: "center";
        justify-content: "center";
        color: var!(primary-foreground);
        font-weight: var!(font-weight-bold);
        border: "none";
        cursor: "pointer";
        padding: "0px";
        flex-shrink: "0";
        position: "relative";
    }

    pub(crate) c_euv_logo_nav {
        width: "32px";
        height: "32px";
        font-size: var!(font-lg);
    }

    pub(crate) c_euv_logo_fab {
        width: "36px";
        height: "36px";
        font-size: var!(font-xl);
        @media ((max-width: 767px)) {
            width: "44px";
            height: "44px";
        }
        border-radius: var!(radius-md);
        box-shadow: var!(shadow-md);
    }

    pub(crate) c_nav_item_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        background: var!(accent);
        color: var!(accent-foreground);
        min-height: var!(nav-row-height);
        padding: format!("{} {}", var!(nav-row-padding), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(nav-row-padding), var!(edge-gutter-mobile));
        }
        text-decoration: "none";
        font-size: var!(font-base);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
        background: var!(primary);
        :focus-visible {
            outline: format!("{} solid {}", var!(ring-width), var!(ring));
            outline-offset: var!(ring-offset);
        }
        border-radius: var!(radius-md);
    }

    pub(crate) c_nav_item_inactive {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-height: var!(nav-row-height);
        padding: format!("{} {}", var!(nav-row-padding), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(nav-row-padding), var!(edge-gutter-mobile));
        }
        text-decoration: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        font-weight: var!(font-weight-normal);
        :hover {
            background: var!(muted);
            color: var!(primary);
            box-shadow: format!("inset 4px 0 0 0 {}", var!(foreground));
        }
        :focus-visible {
            outline: format!("{} solid {}", var!(ring-width), var!(ring));
            outline-offset: var!(ring-offset);
        }
        border-radius: var!(radius-md);
    }

    pub(crate) c_nav_item_icon {
        flex-shrink: "0";
        width: "20px";
        text-align: "center";
    }

    pub(crate) c_nav_item_label {
        flex: "1";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        color: "inherit";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_navbar
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_navbar {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        height: "56px";
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        padding: format!("0px {}", var!(edge-gutter));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        z-index: "100";
        @media ((max-width: 767px)) {
            padding: format!("0px {}", var!(edge-gutter-mobile));
        }
    }

    pub(crate) c_euv_navbar_brand {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        font-size: var!(font-lg);
        font-weight: var!(font-weight-bold);
        letter-spacing: var!(letter-spacing-tight);
        color: var!(foreground);
        cursor: "pointer";
        flex-shrink: "0";
    }

    pub(crate) c_euv_navbar_logo {
        width: "32px";
        height: "32px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-lg);
        flex-shrink: "0";
    }

    pub(crate) c_euv_navbar_links {
        display: "flex";
        align-items: "center";
        gap: var!(gap-section);
        margin-left: "auto";
        @media ((max-width: 767px)) {
            display: "none";
        }
    }

    pub(crate) c_euv_navbar_link {
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-bottom: "2px solid transparent";
        cursor: "pointer";
        :hover {
            color: var!(primary);
        }
    }

    pub(crate) c_euv_navbar_link_active {
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        color: var!(primary);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-bottom: format!("2px solid {}", var!(primary));
        cursor: "pointer";
    }

    pub(crate) c_euv_navbar_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        margin-left: var!(gap-section);
        @media ((max-width: 767px)) {
            margin-left: "auto";
        }
    }

    pub(crate) c_euv_navbar_menu_button {
        display: "none";
        width: "40px";
        height: "40px";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-xl);
        cursor: "pointer";
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        @media ((max-width: 767px)) {
            display: "flex";
        }
    }

    pub(crate) c_euv_navbar_menu_button_active {
        display: "none";
        width: "40px";
        height: "40px";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-xl);
        cursor: "pointer";
        background: var!(muted);
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        @media ((max-width: 767px)) {
            display: "flex";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_sidebar
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_sidebar_group {
        // The rows carry their own breathing room through `padding`, so a
        // margin here stacked a *second* gap on top of it. Every row already
        // met the next one through the pair of paddings; the extra 4px only
        // showed at group boundaries, where it cut the active fill away from
        // the row below it and read as a hole in the left rail.
        margin-bottom: "0px";
    }

    pub(crate) c_euv_toc_title {
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        text-transform: var!(text-transform-uppercase);
        letter-spacing: var!(letter-spacing-wide);
        color: var!(muted-foreground);
        margin-bottom: var!(space-xs);
    }
    pub c_euv_dropdown_menu_item {
        display: "flex";
        flex-shrink: "0";
        box-sizing: "border-box";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            pointer-events: "none";
            opacity: "0.5";
        }
    }

    pub c_euv_dropdown_menu_item_disabled {
        opacity: "0.5";
        cursor: "not-allowed";
        pointer-events: "none";
    }

    pub c_euv_dropdown_menu_item_label {
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_euv_dropdown_menu_label {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        line-height: "1.5";
    }

    pub c_euv_dropdown_menu_separator {
        height: "1px";
        margin: format!("{} {}", var!(space-xs), "0px");
        background: var!(border);
    }

    pub c_euv_dropdown_menu_group {
        display: "flex";
        flex-shrink: "0";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    pub c_euv_dropdown_menu_checkbox_item {
        c_euv_dropdown_menu_item();
        justify-content: "flex-start";
    }

    pub c_euv_dropdown_menu_checkbox_indicator {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: var!(space-md);
    }

    pub c_euv_dropdown_menu_radio_group {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    pub c_euv_dropdown_menu_radio_item {
        c_euv_dropdown_menu_item();
        justify-content: "flex-start";
    }

    pub c_euv_dropdown_menu_sub {
        position: "relative";
        display: "flex";
        flex-direction: "column";
    }

    // The sub-menu row carries the same gap as every other caret-bearing
    // trigger, so a nested menu opens its caret at the same distance from its
    // label as a select trigger opens its own.
    pub c_euv_dropdown_menu_sub_trigger {
        c_euv_dropdown_menu_item();
        justify-content: "flex-start";
        gap: var!(gap-caret);
    }

    pub(crate) c_euv_dropdown_menu_sub_caret {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: var!(space-lg);
        height: var!(space-lg);
        margin-inline-start: "auto";
        :dir(rtl) {
            transform: "rotate(180deg)";
        }
    }

    pub c_euv_dropdown_menu_sub_content {
        position: "absolute";
        top: "0px";
        left: "100%";
        z-index: "150";
        display: "flex";
        flex-direction: "column";
        min-width: "160px";
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-lg);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_dropdown_menu_shortcut {
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        letter-spacing: var!(letter-spacing-wide);
    }

    pub c_euv_menubar_menu {
        position: "relative";
        display: "flex";
        align-items: "center";
    }

    pub c_euv_menubar_menu_open {}

    pub c_euv_menubar_menu_closed {}

    pub c_euv_menubar_content_open {
        display: "flex";
    }

    pub c_euv_menubar_content_closed {
        display: "none";
    }

    pub c_euv_menu_panel {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        min-width: "180px";
        padding: var!(space-xs);
        border: format!("{} {} {}", var!(border-width), "solid", var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_scroll_area {
        position: "relative";
        overflow: "hidden";
    }
}

#[cfg(not(feature = "theme-shadcn"))]
class! {
    pub c_euv_sidebar_group_title {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        width: "100%";
        // Symmetric horizontal padding so the text and the dashed border of
        // any nested children line up on the left edge (see sidebar_children
        // below). Left padding equals `border-left-width + padding-left` of
        // the children container — that's the "text and the dashed border
        // share the same x" requirement.
        //
        // Vertical padding is `control-row-pad`, not `space-md`. Two adjacent
        // rows meet through the pair of their paddings, so 12px each left 24px
        // of blank between labels while the row box stayed 48px tall — the left
        // rail and the active fill then floated in twice the air the text
        // occupied. 9px puts every row on the 42px box the CTA buttons
        // (`c_home_btn_primary`) already use, so a sidebar row, a dropdown
        // item and a button all read as the same size control. Every sidebar
        // row class uses this value, so the rhythm stays even across the tree.
        padding: format!("{} {}", var!(control-row-pad), var!(space-md));
        font-size: var!(font-base);
        font-weight: var!(font-weight-normal);
        cursor: "pointer";
        text-align: "left";
        // Hover: inset 3px shadow bar between the dashed tree guide and the
        // title text. Using `box-shadow inset` instead of a real border
        // avoids any layout change so the text x-position never moves on
        // hover (see comment on `c_euv_sidebar_link:hover`).
        :hover {
            box-shadow: format!("inset 3px 0 0 0 {}", var!(foreground));
        }
    }

    // A top-level group title has no `c_euv_sidebar_children` ancestor, so
    // the base hover bar paints flush against the sidebar's own left edge
    // and reads as part of the sidebar border rather than as an affordance.
    // This variant insets the bar by the row's own left padding and doubles
    // its weight, so a first-level row gets the same "you can click this"
    // cue a nested row does, just with room around it. The offset lives on
    // the shadow rather than on `padding`, so the text never moves.
    //
    // Vertical padding is `control-row-pad` like every other sidebar row —
    // see `c_euv_sidebar_group_title`. A root title sits directly above a
    // nested one, so a taller row here left the two ends of the same pair
    // mismatched.
    pub c_euv_sidebar_group_title_root {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        width: "100%";
        padding: format!("{} {}", var!(control-row-pad), var!(space-md));
        font-size: var!(font-base);
        font-weight: var!(font-weight-normal);
        cursor: "pointer";
        text-align: "left";
        :hover {
            box-shadow: format!("inset 4px 0 0 0 {}", var!(foreground));
        }
    }

    pub c_euv_sidebar_group_title_root_active {
        c_euv_sidebar_group_title_root();
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
        :hover {
            box-shadow: "none";
            background: var!(primary);
        }
    }

    pub c_euv_sidebar_group_title_active {
        c_euv_sidebar_group_title();
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
        // Same flush fill as `c_euv_sidebar_link_active_flush`: a nested
        // active group title should read as one block with the sidebar, not
        // as a chip sitting in from the edge. The negative margin below pulls
        // the fill out to this level's dashed guide; the left padding pays it
        // straight back so the label keeps the exact x it has while inactive.
        //
        // The arithmetic runs through `calc()` on the *length* token
        // (`--side-indent`, which carries its own unit). Gluing a unit onto
        // a `var()` — `var(--side-indent-num)px` — is not a length: the
        // substitution leaves a number token next to an ident, the parser
        // drops the whole declaration, and the fill never reaches the guide.
        // A unitless token can only be used as a number inside `calc()`.
        margin-left: format!("calc(-1 * {})", var!(side-indent));
        width: format!("calc(100% + {})", var!(side-indent));
        padding-left: format!("calc({} + {})", var!(space-md), var!(side-indent));
        :hover {
            box-shadow: "none";
        }
    }

    pub c_euv_sidebar_link {
        display: "block";
        // `control-row-pad` vertically, matching `c_euv_sidebar_group_title`
        // and the two `_active` classes — see the note there on why the row is
        // 42px and not 48px.
        padding: format!("{} {}", var!(control-row-pad), var!(space-md));
        font-size: var!(font-base);
        color: var!(foreground);
        font-weight: var!(font-weight-normal);
        cursor: "pointer";
        // Hover: paint a 3px solid bar *between* the dashed tree guide and
        // the link's text. We use `inset box-shadow` rather than a real
        // `border-left` because borders participate in the box's layout
        // (they grow the border-edge and require compensating padding to
        // keep the text x-position stable — and `calc(12px + 3px - 3px)`
        // rounds inconsistently across browsers, causing sub-pixel text
        // shifts on hover). An inset shadow is painted *inside* the
        // existing padding box without changing layout, so the text never
        // moves. The dashed parent border is also untouched.
        :hover {
            box-shadow: format!("inset 3px 0 0 0 {}", var!(foreground));
        }
        border-radius: var!(radius-md);
    }

    pub c_euv_sidebar_link_active {
        display: "block";
        // Same vertical padding as `c_euv_sidebar_link` (`control-row-pad`) —
        // the accent fill has to be exactly as tall as the row it replaces.
        padding: format!("{} {}", var!(control-row-pad), var!(space-md));
        font-size: var!(font-base);
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
        cursor: "pointer";
        // Active link: when the cursor is over it, the accent background
        // must dominate. Drop the inset bar entirely so the accent fill
        // carries the visual weight; on dark accent, an inset bar in the
        // foreground color is invisible and only competes for the eye's
        // focus. The accent fill + bold text already announce "you are
        // here".
        :hover {
            box-shadow: "none";
        }
        border-radius: var!(radius-md);
    }

    // An active child needs the accent fill to read as one continuous block
    // against the sidebar, not as a chip floating inside its parent's gutter.
    // `c_euv_sidebar_children` offsets its rows from the dashed guide by its
    // own `padding-left` plus the 1px border. The active row pulls its fill
    // back out to the guide — `padding-left + border-left-width`, i.e.
    // `side-indent` (9px) — so the fill starts exactly at the dashed rail.
    // Compensating by the container's `margin-left` as well is wrong: it also
    // cancels the row's own `padding-left`, pushing the fill past the guide.
    // The negative margin is otherwise safe because the row is the full width
    // of its container: the fill grows leftward into the parent's padding and
    // nothing to its left moves.
    pub c_euv_sidebar_link_active_flush {
        display: "block";
        // Text must not move between the inactive and active states, so the
        // negative margin that pulls the fill out to the dashed guide is paid
        // back here as extra left padding. The two cancel for the label while
        // the background still reaches the guide. Only `padding-left` is
        // touched — the shorthand would drop the other three sides.
        //
        // Vertical padding matches `c_euv_sidebar_link` (`control-row-pad`),
        // see the note on that class for why the row is 42px and not 48px.
        padding: format!("{} {}", var!(control-row-pad), var!(space-md));
        padding-left: format!("calc({} + {})", var!(space-md), var!(side-indent));
        // The arithmetic runs through `calc()` on the *length* token
        // (`--side-indent`, which carries its own unit). Gluing a unit onto
        // a `var()` — `var(--side-indent-num)px` — is not a length: the
        // substitution leaves a number token next to an ident, the parser
        // drops the whole declaration, and the fill never reaches the guide.
        // A unitless token can only be used as a number inside `calc()`.
        margin-left: format!("calc(-1 * {})", var!(side-indent));
        width: format!("calc(100% + {})", var!(side-indent));
        font-size: var!(font-base);
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
        cursor: "pointer";
        :hover {
            box-shadow: "none";
        }
    }

    // The glyph itself is a 6px `▸`, far too small a target: the clickable
    // box wants to clear the ~24px that is comfortable to hit, without making
    // the triangle *look* bigger. So the glyph keeps its font size and the
    // extra area is built out of transparent padding plus negative margins
    // that cancel the padding back out — the row keeps its 42px height and
    // the glyph keeps its place at the right edge, but the invisible hit box
    // grows to roughly 24x42.
    //
    // `euv_sidebar_group` applies exactly ONE arrow class (see the
    // `(active, open)` match there), so these four variants do not stack and
    // each one has to repeat the base declarations — the same duplication
    // `c_euv_sidebar_group_title_root` already uses. Only the colour and the
    // rotation differ between them.
    //
    // `cursor: pointer` and `user-select: none` are load-bearing now that the
    // arrow is its own control: without them most of the enlarged area keeps
    // the inherited `default` cursor, and a double-click can select the glyph
    // as text instead of folding the group.
    pub(crate) c_euv_sidebar_group_arrow {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        // Invisible hit area: the 6px glyph plus 9px of padding on each side
        // is ~24px wide, and vertically the negative margins carry it to the
        // full row height. Written as a literal rather than through `var!`:
        // `format!` around `var!(...)` yields `-var(--x)`, which is invalid CSS
        // and the declaration is dropped silently — the row then stays 54px
        // tall instead of 42px.
        padding: var!(side-indent);
        // Vertical padding would otherwise leak into layout and grow every row
        // by 18px (42px -> 54px), so the negative top/bottom margins cancel
        // exactly the padding they accompany. The hit area is still the full
        // row height — it just overlaps the row's own padding instead of
        // pushing the row taller.
        margin-top: format!("calc(0px - {})", var!(side-indent));
        margin-bottom: format!("calc(0px - {})", var!(side-indent));
        margin-right: format!("calc(0px - {})", var!(side-indent));
        margin-left: format!("calc({} - {})", var!(space-xs), var!(side-indent));
        cursor: "pointer";
        user-select: "none";
        flex-shrink: "0";
        :hover {
            color: var!(foreground);
        }
    }

    pub(crate) c_euv_sidebar_group_arrow_open {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: var!(side-indent);
        // Vertical padding would otherwise leak into layout and grow every row
        // by 18px (42px -> 54px), so the negative top/bottom margins cancel
        // exactly the padding they accompany. The hit area is still the full
        // row height — it just overlaps the row's own padding instead of
        // pushing the row taller.
        margin-top: format!("calc(0px - {})", var!(side-indent));
        margin-bottom: format!("calc(0px - {})", var!(side-indent));
        margin-right: format!("calc(0px - {})", var!(side-indent));
        margin-left: format!("calc({} - {})", var!(space-xs), var!(side-indent));
        cursor: "pointer";
        user-select: "none";
        flex-shrink: "0";
        transform: "rotate(90deg)";
        :hover {
            color: var!(foreground);
        }
    }

    pub(crate) c_euv_sidebar_group_arrow_active {
        font-size: var!(font-xs);
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: var!(side-indent);
        // Vertical padding would otherwise leak into layout and grow every row
        // by 18px (42px -> 54px), so the negative top/bottom margins cancel
        // exactly the padding they accompany. The hit area is still the full
        // row height — it just overlaps the row's own padding instead of
        // pushing the row taller.
        margin-top: format!("calc(0px - {})", var!(side-indent));
        margin-bottom: format!("calc(0px - {})", var!(side-indent));
        margin-right: format!("calc(0px - {})", var!(side-indent));
        margin-left: format!("calc({} - {})", var!(space-xs), var!(side-indent));
        cursor: "pointer";
        user-select: "none";
        flex-shrink: "0";
        color: var!(primary-foreground);
        // An active row sits on the accent fill; the hover colour must stay
        // the on-accent ink or hovering the arrow flips it to the muted
        // foreground and it vanishes against the highlight.
        :hover {
            color: var!(primary-foreground);
        }
    }

    // The arrow's colour and its rotation are independent: `_active` only
    // recolours the glyph so it reads on the accent fill, while `_open` only
    // points it down. An active group can be open *or* collapsed, so it needs
    // both at once — without this the active branch in `euv_sidebar_group`
    // won the exclusive match and the arrow stopped following the toggle.
    pub(crate) c_euv_sidebar_group_arrow_active_open {
        font-size: var!(font-xs);
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: var!(side-indent);
        // Vertical padding would otherwise leak into layout and grow every row
        // by 18px (42px -> 54px), so the negative top/bottom margins cancel
        // exactly the padding they accompany. The hit area is still the full
        // row height — it just overlaps the row's own padding instead of
        // pushing the row taller.
        margin-top: format!("calc(0px - {})", var!(side-indent));
        margin-bottom: format!("calc(0px - {})", var!(side-indent));
        margin-right: format!("calc(0px - {})", var!(side-indent));
        margin-left: format!("calc({} - {})", var!(space-xs), var!(side-indent));
        cursor: "pointer";
        user-select: "none";
        flex-shrink: "0";
        color: var!(primary-foreground);
        transform: "rotate(90deg)";
        :hover {
            color: var!(primary-foreground);
        }
    }

    pub c_euv_menubar {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    pub c_euv_menubar_trigger {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-xs);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_menubar_trigger_open {
        background: var!(accent);
        color: var!(accent-foreground);
    }

    pub c_euv_menubar_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "101";
        display: "flex";
        flex-direction: "column";
        min-width: "180px";
        padding: var!(space-xs);
        border: format!("{} {} {}", var!(border-width), "solid", var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_menubar_item {
        display: "flex";
        box-sizing: "border-box";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            pointer-events: "none";
            opacity: "0.5";
        }
    }

    pub c_euv_scroll_area_viewport {
        width: "100%";
        height: "100%";
        overflow: "auto";
        border-radius: var!(radius-md);
        scrollbar-width: "none";
        ::-webkit-scrollbar {
            display: "none";
        }
    }

    pub c_euv_scroll_bar {
        position: "absolute";
        top: "0px";
        inset-inline-end: "0px";
        display: "flex";
        box-sizing: "border-box";
        width: "10px";
        height: "100%";
        // Shadcn's p-px leaves a seven-pixel thumb inside its ten-pixel bordered track.
        padding: var!(space-2xs);
        border-inline-start: format!("{} {} {}", var!(border-width), "solid", "transparent");
        touch-action: "none";
        user-select: "none";
    }

    pub c_euv_scroll_bar_thumb {
        position: "relative";
        flex: "0 0 auto";
        width: "100%";
        border-radius: "0px";
        background: var!(border);
    }
}

#[cfg(feature = "theme-shadcn")]
class! {
    pub c_euv_sidebar_group_title {
        c_euv_sidebar_menu_button();
        justify-content: "space-between";
    }

    pub c_euv_sidebar_group_title_root {
        c_euv_sidebar_menu_button();
        justify-content: "space-between";
    }

    pub c_euv_sidebar_group_title_root_active {
        c_euv_sidebar_menu_button_active();
        justify-content: "space-between";
    }

    pub c_euv_sidebar_group_title_active {
        c_euv_sidebar_menu_button_active();
        justify-content: "space-between";
    }

    pub c_euv_sidebar_link {
        c_euv_sidebar_menu_button();
    }

    pub c_euv_sidebar_link_active {
        c_euv_sidebar_menu_button_active();
    }

    pub c_euv_sidebar_link_active_flush {
        c_euv_sidebar_menu_button_active();
    }

    pub(crate) c_euv_sidebar_group_arrow {
        c_euv_sidebar_menu_action();
        font-size: var!(font-xs);
        user-select: "none";
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_euv_sidebar_group_arrow_open {
        c_euv_sidebar_menu_action();
        font-size: var!(font-xs);
        user-select: "none";
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        transform: "rotate(90deg)";
    }

    pub(crate) c_euv_sidebar_group_arrow_active {
        c_euv_sidebar_menu_action();
        font-size: var!(font-xs);
        user-select: "none";
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        color: var!(accent-foreground);
    }

    pub(crate) c_euv_sidebar_group_arrow_active_open {
        c_euv_sidebar_menu_action();
        font-size: var!(font-xs);
        user-select: "none";
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
        color: var!(accent-foreground);
        transform: "rotate(90deg)";
    }

    pub c_euv_menubar {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);

        height: var!(control-height);
        width: "fit-content";
        align-self: "flex-start";
        padding: var!(space-xs);
        gap: var!(space-xs);
        background: var!(background);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        box-shadow: var!(shadow-xs);

        border-radius: var!(radius-lg);
        gap: var!(space-2xs);
    }

    pub c_euv_menubar_trigger {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-xs);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        font-weight: var!(font-weight-medium);
        border-radius: var!(radius-sm);
        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        padding-left: var!(space-xs-sm);
    }

    pub c_euv_menubar_trigger_open {
        background: var!(accent);
        color: var!(accent-foreground);

        font-weight: var!(font-weight-medium);
        border-radius: var!(radius-sm);

        padding-left: var!(space-xs-sm);
    }

    pub c_euv_menubar_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "101";
        display: "flex";
        flex-direction: "column";
        min-width: "180px";
        padding: var!(space-xs);
        border: format!("{} {} {}", var!(border-width), "solid", var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);

        min-width: "12rem";
        padding: var!(space-xs);
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_menubar_item {
        display: "flex";
        box-sizing: "border-box";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            pointer-events: "none";
            opacity: "0.5";
        }

        padding: format!("calc({} + {}) {}", var!(space-xs), var!(space-2xs), var!(space-sm));
        border-radius: var!(radius-sm);
        cursor: "default";
        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            pointer-events: "none";
            opacity: "0.5";
        }
    }

    pub c_euv_scroll_area_viewport {
        width: "100%";
        height: "100%";
        overflow: "auto";
        border-radius: "inherit";
        scrollbar-width: "none";
        ::-webkit-scrollbar {
            display: "none";
        }
    }

    pub c_euv_scroll_bar {
        position: "absolute";
        top: "0px";
        inset-inline-end: "0px";
        display: "flex";
        box-sizing: "border-box";
        width: "10px";
        height: "100%";
        // Shadcn's p-px leaves a seven-pixel thumb inside its ten-pixel bordered track.
        padding: format!("calc({} / 2)", var!(space-2xs));
        border-inline-start: format!("{} {} {}", var!(border-width), "solid", "transparent");
        touch-action: "none";
        user-select: "none";
    }

    pub c_euv_scroll_bar_thumb {
        position: "relative";
        flex: "0 0 auto";
        width: "100%";
        border-radius: "9999px";
        background: var!(border);
    }
}
