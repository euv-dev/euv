use super::*;

class! {

    // ═══════════════════════════════════════════════════════════════════════════
    // Layout Shell
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_app_root {
        display: "flex";
        height: "100%";
        padding-bottom: var!(safe-area-inset-bottom);
        font-family: "system-ui, -apple-system, sans-serif";
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
            border-radius: "0px";
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
        input, textarea, select, button {
            font: "inherit";
            font-family: "inherit";
            font-size: "inherit";
            line-height: "normal";
            margin: "0px";
            padding: "0px";
            border: "none";
            outline: "none";
            background: "transparent";
            color: "inherit";
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
        }
        input[type="text"], input[type="number"], input[type="email"], input[type="password"], input[type="search"], input[type="tel"], input[type="url"] {
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
            border-radius: "0px";
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
        }
        select {
            appearance: "none";
            -webkit-appearance: "none";
            -moz-appearance: "none";
            background-image: "none";
            border-radius: "0px";
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
            font-weight: "700";
        }
        em, i {
            font-style: "italic";
        }
        code, pre, kbd, samp {
            font-family: "ui-monospace, monospace";
        }
        @media ((max-width: 767px)) {
            input, select, textarea {
                font-size: "16px";
            }
        }
    }


    pub c_mobile_app_root {
        display: "flex";
        flex-direction: "column";
        width: "100%";
        height: "100%";
        font-family: "system-ui, -apple-system, sans-serif";
        background: var!(background);
        color: var!(foreground);
        padding: format!("0px {} {} {}", var!(safe-area-inset-right), var!(padding-shell-bottom), var!(safe-area-inset-left));
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: "0px";
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
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
        border-left: format!("2px solid {}", var!(border));
        display: "flex";
        flex-direction: "column";
        // Pin the sidebar to the viewport while the main column scrolls.
        // Without this the whole row flex container scrolls as a single
        // block and the nav appears to slide off-screen with the content.
        // `align-self: stretch` (the row flex default) is overridden by
        // `flex-start` so the sticky height is the natural 100vh and not
        // stretched to match the (much taller) main column.
        position: "sticky";
        top: "0px";
        align-self: "flex-start";
        height: "100vh";
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
        font-weight: "700";
        color: var!(foreground);
        letter-spacing: "-0.02em";
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
        font-weight: "700";
        color: var!(foreground);
        letter-spacing: "-0.02em";
    }


    pub c_euv_logo {
        display: "flex";
        background: var!(accent);
        align-items: "center";
        justify-content: "center";
        color: var!(text-on-accent);
        font-weight: "700";
        border: "none";
        cursor: "pointer";
        padding: "0px";
        flex-shrink: "0";
        position: "relative";
    }


    pub c_euv_logo_nav {
        width: "32px";
        height: "32px";
        font-size: var!(font-lg);
    }


    pub c_euv_logo_fab {
        width: "36px";
        height: "36px";
        font-size: var!(font-xl);
        @media ((max-width: 767px)) {
            width: "44px";
            height: "44px";
        }
    }


    pub c_nav_section_label {
        padding: format!("{} {} {} {}", var!(space-md), var!(edge-gutter-nav), var!(space-xs), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {} {} {}", var!(space-md), var!(edge-gutter-mobile), var!(space-xs), var!(edge-gutter-mobile));
        }
        margin: "0px";
        font-size: var!(font-xs);
        font-weight: "700";
        color: var!(foreground);
        text-transform: "uppercase";
        letter-spacing: "0.10em";
        flex-shrink: "0";
    }


    pub c_nav_items_scroll {
        flex: "1";
        overflow-y: "auto";
        contain: "content";
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: "0px";
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
        }
    }


    pub c_nav_theme_toggle {
        padding: format!("{} {}", var!(space-md), var!(edge-gutter-nav));
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
        border: format!("1px dashed {}", var!(border));
        background: "transparent";
        color: var!(foreground);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        // The trigger mirrors the sidebar's hover affordance: the dashed
        // border thickens to the foreground colour and a translucent wash
        // derived from the text colour appears behind the label. The wash is
        // `color-mix` on `foreground` rather than `accent-muted`, because in
        // the monochrome palette `accent-muted` resolves to the same value as
        // `background` (white on light, black on dark) and would be invisible.
        :hover {
            border-color: var!(foreground);
            background: format!("color-mix(in srgb, {} 8%, transparent)", var!(foreground));
        }
        :focus-visible {
            outline: "none";
        }
        // `:active` must not reset the hover styles: it only fires while the
        // pointer is held, and zeroing the background there is what made the
        // click look like nothing happened.
        :active {
            border-color: var!(foreground);
            background: format!("color-mix(in srgb, {} 12%, transparent)", var!(foreground));
        }
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
        border: format!("1px solid {}", var!(accent));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        // Background and text colour are a matched pair: the fill is the
        // accent, so the label must be the colour that reads on top of it.
        // Setting one without the other is what produces invisible text.
        background: var!(accent);
        color: var!(text-on-accent);
        :hover {
            background: var!(accent);
            color: var!(text-on-accent);
            border-color: var!(accent);
        }
        :focus-visible {
            outline: "none";
        }
        :active {
            background: var!(accent);
            color: var!(text-on-accent);
            border-color: var!(accent);
        }
    }


    pub c_nav_theme_button {
        width: "100%";
        height: "36px";
        padding: "0px";
        cursor: "pointer";
        outline: "none";
        border: format!("1px dashed {}", var!(border));
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
        // Asymmetric padding: the divider is painted at `top: 0` inside this
        // box, so the top padding is the gap between the divider and the
        // attribution text. The bottom padding is what separates the text
        // from the bottom of the sidebar — keep it to `space-md` so the
        // footer reads as sitting on the bottom edge instead of floating
        // above a wide empty band (the old `space-lg` bottom padding was
        // doubled by the row's own line-height).
        padding: format!("{} {} {}", var!(space-lg), var!(edge-gutter-nav), var!(space-md));
        @media ((max-width: 767px)) {
            padding: format!("{} {} {}", var!(space-lg), var!(edge-gutter-mobile), var!(space-md));
        }
        position: "relative";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        flex-shrink: "0";
        text-decoration: "none";
        display: "flex";
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
        font-weight: "400";
        letter-spacing: "0.02em";
    }


    pub c_nav_footer_brand {
        font-weight: "700";
        color: var!(accent);
    }


    pub c_nav_item_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-md), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-md), var!(edge-gutter-mobile));
        }
        text-decoration: "none";
        font-size: var!(font-base);
        color: var!(text-on-accent);
        font-weight: "600";
        background: var!(accent);
    }


    pub c_nav_item_inactive {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-md), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-md), var!(edge-gutter-mobile));
        }
        text-decoration: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        font-weight: "400";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
            box-shadow: format!("inset 4px 0 0 0 {}", var!(foreground));
        }
    }


    pub c_nav_item_icon {
        flex-shrink: "0";
        width: "20px";
        text-align: "center";
    }


    pub c_nav_item_label {
        flex: "1";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        color: "inherit";
    }


    // ═══════════════════════════════════════════════════════════════════════════
    // Main Content Area
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_app_main {
        flex: "1";
        height: "100%";
        overflow: "auto";
        padding: format!("{} {} {} {}", var!(padding-main-top), var!(edge-gutter), var!(padding-main-bottom), var!(edge-gutter));
        scrollbar-color: format!("{} {}", var!(scrollbar-thumb), var!(scrollbar-track));
        ::-webkit-scrollbar {
            width: "6px";
        }
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: "0px";
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
        ::-webkit-scrollbar-thumb:active {
            background: var!(scrollbar-thumb-active);
        }
        @media ((max-width: 767px)) {
            padding: format!("{} {} {} {}", var!(padding-main-top-mobile), var!(edge-gutter-mobile), var!(padding-main-bottom), var!(edge-gutter-mobile));
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
        border-bottom: format!("1px solid {}", var!(border));
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
        font-weight: "700";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(foreground);
        padding: "0px";
        :active {
            background: "transparent";
            color: var!(foreground);
        }
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
    }


    pub c_mobile_menu_button_active {
        width: "40px";
        height: "40px";
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        font-weight: "700";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(accent);
        padding: "0px";
        :active {
            background: var!(background);
            color: var!(accent);
        }
    }


    pub c_mobile_drawer_close_button {
        width: "32px";
        height: "32px";
        border: "none";
        cursor: "pointer";
        font-size: "18px";
        font-weight: "500";
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


    // ═══════════════════════════════════════════════════════════════════════════
    // euv_navbar
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_navbar {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        height: "56px";
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        padding: format!("0px {}", var!(edge-gutter));
        border-bottom: format!("1px solid {}", var!(border));
        background: var!(background);
        z-index: "100";
        @media ((max-width: 767px)) {
            padding: format!("0px {}", var!(edge-gutter-mobile));
        }
    }


    pub c_euv_navbar_brand {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        font-size: var!(font-lg);
        font-weight: "700";
        letter-spacing: "-0.02em";
        color: var!(foreground);
        cursor: "pointer";
        flex-shrink: "0";
    }


    pub c_euv_navbar_logo {
        width: "32px";
        height: "32px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-lg);
        flex-shrink: "0";
    }


    pub c_euv_navbar_links {
        display: "flex";
        align-items: "center";
        gap: var!(gap-section);
        margin-left: "auto";
        @media ((max-width: 767px)) {
            display: "none";
        }
    }


    pub c_euv_navbar_link {
        font-size: var!(font-sm);
        font-weight: "500";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-bottom: "2px solid transparent";
        cursor: "pointer";
        :hover {
            color: var!(accent);
        }
    }


    pub c_euv_navbar_link_active {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-bottom: format!("2px solid {}", var!(accent));
        cursor: "pointer";
    }


    pub c_euv_navbar_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        margin-left: var!(gap-section);
        @media ((max-width: 767px)) {
            margin-left: "auto";
        }
    }


    pub c_euv_navbar_icon_button {
        width: "36px";
        height: "36px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        border: format!("1px dashed {}", var!(border));
        cursor: "pointer";
        font-size: var!(font-base);
        :hover {
            background: var!(accent-muted);
        }
    }


    pub c_euv_navbar_menu_button {
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


    pub c_euv_navbar_menu_button_active {
        display: "none";
        width: "40px";
        height: "40px";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-xl);
        cursor: "pointer";
        background: var!(accent-muted);
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        @media ((max-width: 767px)) {
            display: "flex";
        }
    }


    // ═══════════════════════════════════════════════════════════════════════════
    // euv_sidebar
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_sidebar_group {
        margin-bottom: var!(space-xs);
    }


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
        padding: format!("{} {}", var!(space-md), var!(space-md));
        font-size: var!(font-base);
        font-weight: "400";
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
    pub c_euv_sidebar_group_title_root {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-md));
        font-size: var!(font-base);
        font-weight: "400";
        cursor: "pointer";
        text-align: "left";
        :hover {
            box-shadow: format!("inset 4px 0 0 0 {}", var!(foreground));
            background: var!(muted);
        }
    }


    pub c_euv_sidebar_group_title_root_active {
        background: var!(accent);
        color: var!(text-on-accent);
        font-weight: "600";
        :hover {
            box-shadow: "none";
            background: var!(accent);
        }
    }


    pub c_euv_sidebar_group_title_active {
        background: var!(accent);
        color: var!(text-on-accent);
        font-weight: "600";
        :hover {
            box-shadow: "none";
        }
    }


    pub c_euv_sidebar_group_arrow {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        transition: format!("transform {} {}", var!(duration-fast), var!(ease-out));
    }


    pub c_euv_sidebar_group_arrow_open {
        transform: "rotate(90deg)";
    }


    pub c_euv_sidebar_group_arrow_active {
        color: var!(text-on-accent);
    }


    pub c_euv_sidebar_children {
        display: "flex";
        flex-direction: "column";
        // Compact nesting: keep the dashed tree guide on the left of every
        // nesting level, but trim its indent so the child's text sits just
        // to the right of the dashed border instead of a full 33px gutter
        // away (the old `margin-left: 20px` + `padding-left: 12px` setup).
        // The numbers chosen (margin 8 + border 1 + padding 8 = 17px) are
        // large enough to read as a distinct level but small enough that
        // three-deep trees still fit in a 248–320px sidebar.
        margin-left: var!(space-sm);
        padding-left: var!(space-sm);
        border-left: format!("1px dashed {}", var!(border));
        animation: format!("euv-fade-in {} {}", var!(duration-normal), var!(ease-out));
    }


    pub c_euv_sidebar_link {
        display: "block";
        padding: format!("{} {}", var!(space-md), var!(space-md));
        font-size: var!(font-base);
        color: var!(foreground);
        font-weight: "400";
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
    }


    pub c_euv_sidebar_link_active {
        display: "block";
        padding: format!("{} {}", var!(space-md), var!(space-md));
        font-size: var!(font-base);
        background: var!(accent);
        color: var!(text-on-accent);
        font-weight: "600";
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
        border-left: format!("1px solid {}", var!(border));
        padding-left: var!(space-lg);
    }


    pub c_euv_toc_title {
        font-size: var!(font-xs);
        font-weight: "700";
        text-transform: "uppercase";
        letter-spacing: "0.08em";
        color: var!(muted-foreground);
        margin-bottom: var!(space-xs);
    }


    pub c_euv_toc_link {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        cursor: "pointer";
        line-height: "1.5";
        :hover {
            color: var!(accent);
        }
    }


    pub c_euv_toc_link_nested {
        c_euv_toc_link();
        padding-left: var!(space-lg);
    }
}
