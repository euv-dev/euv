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
        width: var!(nav-width);
        background: var!(background);
        border-left: format!("2px solid {}", var!(border));
        display: "flex";
        flex-direction: "column";
        height: "100%";
        flex-shrink: "0";
        padding-top: var!(safe-area-inset-top);
        @media ((max-width: 767px)) {
            display: "none";
        }
    }

    pub c_nav_header {
        padding: format!("{} {}", var!(space-xl), var!(space-xl));
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
            left: var!(space-lg);
            right: var!(space-lg);
            height: "1px";
            background: format!("linear-gradient(90deg, transparent, {}, transparent)", var!(border));
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
        padding: format!("{} {} {} {}", var!(space-md), var!(space-xl), var!(space-xs), var!(space-xl));
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
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        flex-shrink: "0";
        margin-top: "auto";
        @media ((max-width: 767px)) {
            display: "none";
        }
    }

    // Optional row in the nav column / drawer for locale switchers or similar
    // widgets, placed between the brand header and the section label.
    pub c_nav_locale_row {
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        flex-shrink: "0";
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
        padding: format!("{} {}", var!(space-lg), var!(space-xl));
        position: "relative";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        flex-shrink: "0";
        text-decoration: "none";
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        cursor: "pointer";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
        opacity: "1";
        :hover {
            opacity: "1";
        }
    }

    pub c_nav_footer_divider {
        position: "absolute";
        top: "0";
        left: var!(space-lg);
        right: var!(space-lg);
        height: "1px";
        background: format!("linear-gradient(90deg, transparent, {}, transparent)", var!(border));
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
        padding: format!("{} {}", var!(space-md), var!(space-xl));
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
        padding: format!("{} {}", var!(space-md), var!(space-xl));
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
        padding: format!("{} {} {} {}", var!(padding-main-top), var!(padding-main-horizontal), var!(padding-main-bottom), var!(padding-main-horizontal));
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
            padding: format!("{} {} {} {}", var!(padding-main-top-mobile), var!(padding-main-horizontal-mobile), var!(padding-main-bottom), var!(padding-main-horizontal-mobile));
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
    // Page Banner (unified header with emoji icon)
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_page {
        position: "relative";
        text-align: "center";
        box-sizing: "border-box";
    }

    pub c_page_glow {
        position: "absolute";
        top: "-50%";
        left: "50%";
        transform: "translateX(-50%)";
        width: "400px";
        max-width: "100%";
        height: "400px";
        pointer-events: "none";
    }

    pub c_page_content {
        position: "relative";
        z-index: "1";
    }

    pub c_page_icon {
        font-size: "36px";
        padding-bottom: var!(space-md);
        @media ((max-width: 767px)) {
            font-size: "40px";
        }
    }

    pub c_page_title {
        font-size: var!(font-4xl);
        font-weight: "800";
        letter-spacing: "-0.03em";
        margin: "0px";
        color: var!(foreground);
        margin-bottom: var!(space-sm);
        @media ((max-width: 767px)) {
            font-size: var!(font-3xl);
        }
    }

    pub c_page_subtitle {
        font-size: var!(font-lg);
        color: var!(muted-foreground);
        margin: "0px auto";
        margin-bottom: var!(space-sm);
        max-width: "560px";
        opacity: "1";
        @media ((max-width: 767px)) {
            font-size: var!(font-base);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Card Component
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_card {
        color: var!(foreground);
        box-sizing: "border-box";
    }

    pub c_card_title {
        margin: format!("{} 0px", var!(gap-component));
        color: var!(foreground);
        font-size: var!(font-lg);
        font-weight: "600";
        padding-bottom: var!(gap-component);
        border-bottom: format!("1px dashed {}", var!(border));
        letter-spacing: "-0.01em";
        @media ((max-width: 767px)) {
            font-size: var!(font-md);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Buttons
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_primary_button {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        height: "42px";
        background: var!(accent);
        color: var!(text-on-accent);
        border: "1px solid transparent";
        padding: format!("0px {}", var!(space-2xl));
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        letter-spacing: "0.02em";
        text-align: "center";
        outline: "none";
        box-sizing: "border-box";
        white-space: "nowrap";
        overflow: "hidden";
        text-overflow: "ellipsis";
        user-select: "none";
        -webkit-user-select: "none";
        :focus-visible {
            outline: "none";
        }
        :disabled {
            background: var!(muted-foreground);
            cursor: "not-allowed";
            opacity: "1";
        }
        :hover {
            background: var!(accent);
        }
        :active {
            background: var!(accent);
            color: var!(text-on-accent);
            border-color: "transparent";
        }
        @media ((max-width: 767px)) {
            max-width: "100%";
            padding: format!("{} {}", var!(space-lg), var!(space-xl));
            font-size: var!(font-md);
        }
    }

    pub c_modal_close_button {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        color: "inherit";
        border: "none";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        cursor: "pointer";
        font-size: "20px";
        font-weight: "400";
        vertical-align: "middle";
        outline: "none";
        opacity: "1";
        flex-shrink: "0";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
        :hover {
            opacity: "1";
        }
        :active {
            opacity: "1";
        }
        :focus {
            outline: "none";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Button Controls Container
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_button_controls {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
        margin-top: var!(gap-component);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Euv Button Variants
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_button_primary_md {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        flex: "1 1 120px";
        height: "42px";
        padding: format!("0px {}", var!(space-xl));
        background: var!(accent);
        color: var!(text-on-accent);
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        outline: "none";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
        box-sizing: "border-box";
        :disabled {
            background: var!(muted-foreground);
            cursor: "not-allowed";
            opacity: "1";
        }
    }

    pub c_euv_button_outline_md {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        flex: "1 1 120px";
        height: "42px";
        padding: format!("0px {}", var!(space-xl));
        color: var!(foreground);
        border: format!("1px solid {}", var!(border));
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        outline: "none";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
        box-sizing: "border-box";
        :focus-visible {
            outline: "none";
        }
        :disabled {
            background: var!(muted-foreground);
            cursor: "not-allowed";
            opacity: "1";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Badges
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_badge {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(text-on-accent);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: format!("1px solid {}", var!(accent));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        text-align: "center";
        background: var!(accent);
        :focus-visible {
            outline: "none";
        }
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-2xs), var!(space-xs));
            font-size: var!(font-xs);
        }
    }

    pub c_badge_outline {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        text-align: "center";
        border: format!("1.5px solid {}", var!(border));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-2xs), var!(space-xs));
            font-size: var!(font-xs);
        }
    }

    pub c_euv_tag_solid_black {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        color: var!(text-on-accent);
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        background: var!(accent);
        border: format!("1px solid {}", var!(accent));
    }

    pub c_euv_tag_solid_white {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(accent));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Euv Tag Variants (semantic colour tags)
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_tag_outline_black {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(accent);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(accent));
    }

    pub c_euv_tag_outline_white {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(border));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Form Elements
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_input_wrapper {
        width: "100%";
        margin: format!("{} 0px", var!(gap-element));
    }

    pub c_form_label {
        display: "block";
        margin-bottom: var!(space-sm);
        color: "inherit";
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub c_inline_input_row {
        display: "flex";
        align-items: "center";
        gap: var!(gap-component);
    }

    pub c_euv_input {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
            background: var!(accent-muted);
        }
    }

    pub c_euv_input_no_transition {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
    }

    pub c_euv_input_error {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(foreground));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
    }

    pub c_form_checkbox {
        cursor: "pointer";
        width: var!(space-lg);
        height: var!(space-lg);
    }

    pub c_form_checkbox_label {
        font-size: var!(font-base);
        color: "inherit";
        cursor: "pointer";
    }

    pub c_form_checkbox_row {
        margin: format!("{} 0px", var!(gap-component));
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
    }

    pub c_select_input {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        cursor: "pointer";
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        color: var!(foreground);
        vertical-align: "middle";
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='currentColor' d='M6 8L1 3h10z'/%3E%3C/svg%3E\")";
        background-repeat: "no-repeat";
        background-position: "right 14px center";
        :focus {
            outline: "none";
            border-color: var!(accent);
            background: var!(accent-muted);
        }
    }

    pub c_textarea_input {
        width: "100%";
        max-width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        resize: "vertical";
        overflow-x: "hidden";
        word-wrap: "break-word";
        font-family: "inherit";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
            background: var!(accent-muted);
        }
    }

    pub c_textarea_input_error {
        width: "100%";
        max-width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border: format!("1px solid {}", var!(foreground));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        resize: "vertical";
        overflow-x: "hidden";
        word-wrap: "break-word";
        font-family: "inherit";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
    }

    pub c_textarea_counter {
        text-align: "right";
        margin-top: var!(space-xs);
        margin-bottom: var!(gap-component);
    }

    pub c_textarea_counter_text {
        font-size: var!(font-sm);
        color: "inherit";
        opacity: "1";
    }

    pub c_field_error_text {
        color: var!(foreground);
        font-size: var!(font-base);
        margin-top: var!(space-xs);
        margin-bottom: var!(space-sm);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Status Boxes
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_error_box {
        margin: format!("{} 0px", var!(gap-component));
        background: var!(accent-muted);
        color: var!(foreground);
        font-size: var!(font-base);
        box-sizing: "border-box";
    }

    pub c_success_box {
        margin: format!("{} 0px", var!(gap-component));
        background: var!(accent-muted);
        color: var!(foreground);
        font-size: var!(font-base);
        box-sizing: "border-box";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Counter / Signals Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_counter_row {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        align-items: "center";
        gap: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_counter_text {
        font-size: var!(font-base);
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    pub c_counter_value {
        font-weight: "700";
        color: var!(accent);
        font-size: var!(font-base);
    }

    pub c_badge_row {
        display: "flex";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
        align-items: "center";
    }

    pub c_badge_hint {
        margin-bottom: var!(gap-component);
        color: "inherit";
        opacity: "1";
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_text_ellipsis {
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_info_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-md);
        padding: format!("{} 0", var!(space-sm));
        overflow: "hidden";
    }

    pub c_info_label {
        font-size: var!(font-sm);
        font-weight: "500";
        color: var!(muted-foreground);
        min-width: "72px";
        flex-shrink: "0";
        letter-spacing: "0.02em";
    }

    pub c_info_value {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        font-family: "ui-monospace, monospace";
        flex: "1";
    }

    pub c_info_link {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex: "1";
        text-decoration: "none";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
        :hover {
            opacity: "1";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Camera Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_camera_video_container {
        margin: format!("{} 0", var!(space-lg));
    }

    pub c_camera_video_active {
        width: "100%";
        background: "#000";
        aspect-ratio: "16 / 9";
        object-fit: "cover";
        border: format!("2px solid {}", var!(accent));
        box-sizing: "border-box";
    }

    pub c_camera_video_hidden {
        width: "100%";
        aspect-ratio: "16 / 9";
        display: "none";
    }

    pub c_camera_video_placeholder {
        width: "100%";
        aspect-ratio: "16 / 9";
        border: format!("2px dashed {}", var!(border));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        box-sizing: "border-box";
    }

    pub c_camera_placeholder_content {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-md);
    }

    pub c_camera_placeholder_icon {
        font-size: var!(font-4xl);
        opacity: "1";
    }

    pub c_camera_placeholder_text {
        font-size: var!(font-base);
        color: "inherit";
        opacity: "1";
    }

    pub c_camera_error_box {
        margin: format!("{} 0px", var!(gap-component));
        color: var!(foreground);
        font-size: var!(font-base);
        word-break: "break-all";
        overflow-wrap: "break-word";
        box-sizing: "border-box";
    }

    pub c_camera_scan_result_box {
        margin-top: var!(gap-component);
        box-sizing: "border-box";
    }

    pub c_camera_scan_result_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        margin-bottom: var!(space-xs);
    }

    pub c_camera_scan_result_value {
        font-size: var!(font-base);
        color: "inherit";
        word-break: "break-all";
        overflow-wrap: "break-word";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Canvas Drawing Board
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_canvas_preview_container {
        margin: format!("{} 0", var!(space-lg));
        overflow: "hidden";
        border: format!("2px solid {}", var!(border));
        background: "#ffffff";
        aspect-ratio: "9 / 16";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        box-sizing: "border-box";
    }

    pub c_canvas_placeholder {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
        padding: var!(space-lg);
    }

    pub c_canvas_preview_image {
        width: "100%";
        height: "100%";
        object-fit: "cover";
        display: "block";
    }

    pub c_canvas_container_fullscreen {
        width: "100%";
        height: "100%";
        background: var!(background);
        display: "flex";
        flex-direction: "column";
        position: "fixed";
        top: "0";
        left: "0";
        z-index: "10002";
        padding: format!("{} {} {} {}", var!(padding-shell-top), var!(space-lg), var!(padding-shell-bottom), var!(space-lg));
        box-sizing: "border-box";
    }

    pub c_canvas_drawing_fullscreen_wrapper {
        flex: "1";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        overflow: "hidden";
        padding: var!(space-xs);
    }

    pub c_canvas_drawing_fullscreen {
        border: format!("2px solid {}", var!(border));
        display: "block";
        cursor: "crosshair";
        touch-action: "none";
    }

    pub c_canvas_fullscreen_toolbar {
        display: "flex";
        flex-direction: "column";
        align-items: "stretch";
        width: "100%";
        padding: format!("{} {}", var!(space-xs), "0px");
        flex-shrink: "0";
        gap: var!(space-xs);
    }

    pub c_canvas_fullscreen_toolbar_row_top {
        display: "flex";
        align-items: "center";
        height: var!(mobile-header-height);
        width: "100%";
        gap: var!(space-sm);
    }

    pub c_canvas_fullscreen_toolbar_color_wrapper {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex: "1";
        min-width: "0";
    }

    pub c_canvas_color_input_fullscreen {
        width: "100%";
        height: "42px";
        border: format!("1px solid {}", var!(border));
        cursor: "pointer";
        padding: format!("{}", var!(space-xs));
    }

    pub c_canvas_fullscreen_toolbar_row_bottom {
        display: "flex";
        align-items: "center";
        width: "100%";
    }

    pub c_canvas_fullscreen_range_input {
        flex: "1 1 0%";
        min-width: "120px";
        cursor: "pointer";
        -webkit-appearance: "none";
        appearance: "none";
        border: "none";
        outline: "none";
        height: "24px";
        ::-webkit-slider-runnable-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-webkit-slider-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
            -webkit-appearance: "none";
            margin-top: "-7px";
        }
        ::-moz-range-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-moz-range-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
        }
    }

    pub c_canvas_fullscreen_toolbar_button {
        flex: "0 0 auto";
        overflow: "hidden";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom Attrs Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_custom_attrs_demo {
        // Use the theme foreground token (not `inherit`) so the text inside
        // the dynamic demo block stays readable when the block itself
        // applies a user-supplied background (e.g. `background-color: #000000`)
        // that otherwise paints the inherited text the same colour.
        color: var!(foreground);
        box-sizing: "border-box";
    }

    pub c_demo_text {
        color: var!(foreground);
        margin-bottom: var!(gap-component);
    }

    pub c_demo_text_muted {
        color: var!(foreground);
        opacity: "1";
        font-size: var!(font-base);
        margin-bottom: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Async / Loading States
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_loading_container {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-component);
        margin: format!("{} 0px", var!(gap-component));
        box-sizing: "border-box";
    }

    pub c_spinner {
        width: "28px";
        height: "28px";
        border: format!("3px solid {}", var!(accent-muted));
        border-top: format!("3px solid {}", var!(accent));
        border-radius: "50%";
        flex-shrink: "0";
        animation: "euv-spin 0.8s linear infinite";
    }

    pub c_loading_text_col {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub c_loading_title {
        color: var!(foreground);
        font-size: var!(font-md);
        font-weight: "500";
    }

    pub c_loading_subtitle {
        color: var!(muted-foreground);
        font-size: var!(font-base);
    }

    pub c_loading_overlay(background: &str) {
        position: "absolute";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-component);
        background: background;
        z-index: "1";
        box-sizing: "border-box";
    }

    pub c_error_container {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        margin-top: var!(gap-component);
        box-sizing: "border-box";
    }

    pub c_error_icon {
        width: "20px";
        height: "20px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: "700";
    }

    pub c_error_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    pub c_data_box {
        margin: format!("{} 0px", var!(gap-component));
        box-sizing: "border-box";
    }

    pub c_data_pre {
        color: var!(foreground);
        font-size: var!(font-base);
        margin: "0px";
        white-space: "pre-wrap";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub c_fetch_hint {
        color: "inherit";
        opacity: "1";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Conditional Rendering Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_toggle_content {
        margin-top: var!(gap-component);
        color: "inherit";
    }

    pub c_toggle_title {
        margin-top: "0px";
        color: var!(accent);
        font-size: var!(font-md);
    }

    pub c_role_button_row {
        display: "flex";
        flex-wrap: "nowrap";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
    }

    pub c_role_guest_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    pub c_role_user_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    pub c_role_admin_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Tabs
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_tab_bar {
        display: "flex";
        border-bottom: format!("1px dashed {}", var!(border));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
        @media ((max-width: 767px)) {
            flex-wrap: "wrap";
        }
    }

    pub c_tab_item_active {
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        cursor: "pointer";
        border-bottom: format!("2px solid {}", var!(accent));
        color: var!(text-on-accent);
        background: var!(accent);
        font-size: var!(font-base);
        font-weight: "500";
    }

    pub c_tab_item_inactive {
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        cursor: "pointer";
        border-bottom: "2px solid transparent";
        color: "inherit";
        font-size: var!(font-base);
        font-weight: "500";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub c_tab_content {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub c_tab_text {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-element);
    }

    pub c_tab_text_muted {
        color: "inherit";
        opacity: "1";
        font-size: var!(font-base);
    }

    pub c_tab_text_input {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // List
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_list_input {
        width: "100%";
        flex: "1";
        height: "42px";
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
            background: var!(accent-muted);
        }
    }

    pub c_list_input_error {
        width: "100%";
        flex: "1";
        height: "42px";
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(foreground));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
    }

    pub c_list_error_text {
        color: var!(foreground);
        font-size: var!(font-base);
        margin: format!("{} 0px", var!(gap-element));
    }

    pub c_inline_input_button_wrap {
        flex-shrink: "0";
    }

    pub c_list_ul {
        list-style: "none";
        padding: "0px";
        margin: "0px";
        margin-top: var!(gap-component);
    }

    pub c_list_item {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        gap: var!(gap-component);
        min-height: var!(min-height-base);
        margin: format!("{} 0px", var!(gap-element));
    }

    pub c_list_item_text {
        flex: "1";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        font-size: var!(font-base);
        color: "inherit";
        min-width: "0";
    }

    pub c_list_item_button {
        max-width: "120px";
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Log / Console Display
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_log_container {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-base);
        margin-top: "0px";
        color: var!(foreground);
    }

    pub c_log_item {
        padding: format!("{} 0px", var!(space-sm));
        font-size: var!(font-base);
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 404 Not Found
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_render_count_text {
        font-size: var!(font-base);
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Event Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_event_result {
        font-size: var!(font-base);
        color: "inherit";
        margin-top: var!(gap-element);
    }

    pub c_event_highlight {
        font-weight: "600";
        color: var!(accent);
    }

    pub c_event_info_grid {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: var!(gap-element);
        margin-top: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
            gap: var!(gap-element);
        }
    }

    pub c_event_info_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        overflow: "hidden";
    }

    pub c_event_info_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        flex-shrink: "0";
    }

    pub c_event_info_value {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex: "1";
    }

    pub c_event_mouse_area {
        min-height: "120px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        cursor: "crosshair";
        text-align: "center";
        user-select: "none";
        color: "inherit";
    }

    pub c_event_drag_zone {
        min-height: "100px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        text-align: "center";
        user-select: "none";
        color: var!(foreground);
    }

    pub c_event_drag_zone_active {
        min-height: "100px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        text-align: "center";
        user-select: "none";
        color: var!(foreground);
    }

    pub c_event_drag_item {
        display: "inline-block";
        padding: format!("{} {}", var!(space-sm), var!(space-xl));
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "500";
        cursor: "grab";
        margin: var!(space-sm);
    }

    pub c_event_drop_zone {
        border: format!("2px dashed {}", var!(border));
        padding: format!("{} {}", var!(space-4xl), var!(space-xl));
        text-align: "center";
        cursor: "pointer";
    }

    pub c_event_drop_zone_active {
        border: format!("2px dashed {}", var!(accent));
        padding: format!("{} {}", var!(space-4xl), var!(space-xl));
        text-align: "center";
        cursor: "pointer";
        background: var!(accent-muted);
    }

    pub c_event_drop_icon {
        font-size: var!(font-5xl);
        display: "block";
        padding-bottom: var!(space-md);
    }

    pub c_event_drop_text {
        font-size: var!(font-lg);
        font-weight: "500";
        color: "inherit";
        padding-bottom: var!(space-md);
    }

    pub c_event_drop_hint {
        font-size: var!(font-base);
        color: "inherit";
        opacity: "1";
        margin: "0px";
    }

    pub c_event_wheel_zone {
        min-height: "120px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        text-align: "center";
        overflow: "auto";
        color: var!(foreground);
    }

    pub c_event_clipboard_area {
        color: var!(foreground);
    }

    pub c_event_touch_zone {
        min-height: "120px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(accent));
        text-align: "center";
        touch-action: "none";
        user-select: "none";
        color: var!(accent);
    }

    pub c_event_form_area {
        padding: format!("{} 0px", var!(space-lg));
        color: var!(foreground);
    }

    pub c_event_media_area {
        padding: format!("{} 0px", var!(space-lg));
        color: var!(foreground);
        max-width: "100%";
        overflow: "hidden";
    }

    pub c_event_audio {
        width: "100%";
        max-width: "100%";
    }

    pub c_event_video_area {
        color: var!(foreground);
        width: "100%";
        overflow: "hidden";
    }

    pub c_event_video {
        width: "100%";
        max-width: "100%";
    }

    pub c_event_image_area {
        padding: format!("{} 0px", var!(space-lg));
        color: "inherit";
        width: "100%";
        overflow: "hidden";
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-md);
    }

    pub c_event_image {
        width: "200px";
        max-width: "100%";
        object-fit: "contain";
        display: "block";
    }

    pub c_event_url_text {
        text-align: "center";
        font-size: var!(font-sm);
        color: "inherit";
        opacity: "1";
        font-family: "ui-monospace, monospace";
        word-break: "break-all";
        width: "100%";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Timer Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_timer_display {
        text-align: "center";
        padding: var!(space-xl);
        margin: format!("{} 0px", var!(gap-component));
        color: "inherit";
    }

    pub c_timer_value {
        font-size: var!(font-base);
        font-weight: "700";
        color: var!(accent);
        letter-spacing: "0.04em";
        font-family: "ui-monospace, monospace";
    }

    pub c_timer_controls {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
    }

    pub c_timer_done {
        text-align: "center";
        margin-top: var!(gap-component);
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Modal
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_modal_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        width: "100%";
        height: "100%";
        background: var!(bg-overlay);
        contain: "layout style paint";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        z-index: "1000";
        animation: format!("euv-fade-in {} {}", var!(duration-modal-overlay), var!(ease-out));
        @media ((max-width: 767px)) {
            align-items: "center";
            justify-content: "center";
        }
    }

    pub c_modal_content {
        background: var!(background);
        padding: "0px";
        max-width: "480px";
        width: "90%";
        animation: format!("euv-scale-in-modal {} {}", var!(duration-modal-content), var!(ease-bounce));
        overflow: "hidden";
        color: var!(foreground);
        box-sizing: "border-box";
        @media ((max-width: 767px)) {
            max-width: "100%";
            width: "calc(100% - 32px)";
            max-height: "85vh";
            overflow-y: "auto";
        }
    }

    pub c_modal_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: format!("{} {} 0px {}", var!(space-md), var!(space-xl), var!(space-xl));
    }

    pub c_modal_title {
        margin: "0px";
        font-size: var!(font-xl);
        font-weight: "600";
        color: "inherit";
    }

    pub c_modal_body {
        padding: format!("0px {} {} {}", var!(space-xl), var!(space-md), var!(space-xl));
        color: "inherit";
    }

    pub c_modal_actions {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(space-md);
        margin: format!("{} 0px", var!(space-md));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Animation Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_anim_fade_in {
        margin-top: var!(gap-component);
        animation: format!("euv-fade-in 0.5s {}", var!(ease-out));
        font-size: var!(font-base);
        color: "inherit";
    }

    pub c_anim_spin_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-lg));
        min-height: "80px";
    }

    pub c_anim_spin {
        font-size: var!(font-5xl);
        animation: "euv-spin 1.5s linear infinite";
        display: "inline-block";
        @media ((max-width: 767px)) {
            font-size: var!(font-4xl);
        }
    }

    pub c_anim_spin_stopped {
        font-size: var!(font-5xl);
        display: "inline-block";
        opacity: "1";
        @media ((max-width: 767px)) {
            font-size: var!(font-4xl);
        }
    }

    pub c_anim_pulse_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-lg));
        min-height: "80px";
    }

    pub c_anim_pulse {
        font-size: var!(font-5xl);
        animation: "euv-pulse 1.5s ease-in-out infinite";
        display: "inline-block";
        color: var!(foreground);
        @media ((max-width: 767px)) {
            font-size: var!(font-4xl);
        }
    }

    pub c_anim_pulse_stopped {
        font-size: var!(font-5xl);
        display: "inline-block";
        opacity: "1";
        color: var!(foreground);
        @media ((max-width: 767px)) {
            font-size: var!(font-4xl);
        }
    }

    pub c_progress_container {
        width: "100%";
        height: "12px";
        margin: format!("{} 0px", var!(space-lg));
        overflow: "hidden";
    }

    pub c_progress_bar_running {
        height: "100%";
        background: var!(accent);
        animation: format!("euv-progress 1.6s {} forwards", var!(ease-in-out));
    }

    pub c_progress_bar_stopped {
        height: "100%";
        background: var!(accent);
        width: "0%";
    }

    pub c_anim_scale_box {
        margin: format!("{} 0px", var!(gap-component));
        padding: var!(space-xl);
        background: var!(accent);
        cursor: "pointer";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Browser API Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_browser_api_row {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: var!(gap-component);
        margin-bottom: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_browser_api_actions {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
        margin: format!("{} 0px", var!(gap-component));
    }

    pub c_browser_result_box {
        margin-top: var!(gap-component);
        font-size: var!(font-base);
        word-break: "break-all";
        color: "inherit";
    }

    pub c_browser_result_label {
        font-weight: "600";
        color: var!(accent);
    }

    pub c_browser_result_value {
        color: "inherit";
    }

    pub c_browser_info_grid {
        display: "grid";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_browser_info_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        color: "inherit";
    }

    pub c_browser_info_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: "inherit";
        opacity: "1";
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub c_browser_info_value {
        font-size: var!(font-base);
        color: "inherit";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VConsole Panel
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_vconsole_badge {
        position: "absolute";
        top: "-6px";
        right: "-8px";
        min-width: "18px";
        height: "18px";
        background: var!(background);
        color: var!(foreground);
        font-size: "10px";
        font-weight: "600";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: format!("0px {}", var!(space-xs));
        border: format!("1px solid {}", var!(accent));
        pointer-events: "none";
        transition: format!("transform {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub c_vconsole_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        bottom: "0px";
        z-index: "10000";
        background: var!(bg-overlay);
        contain: "layout style paint";
        transition: format!("opacity {} {}", var!(duration-overlay), var!(ease-out));
    }

    pub c_vconsole_overlay_hidden {
        opacity: "0";
        pointer-events: "none";
    }

    pub c_vconsole_panel {
        position: "fixed";
        bottom: "0px";
        left: "0px";
        right: "0px";
        height: format!("calc(76vh - {})", var!(safe-area-inset-bottom));
        background: var!(background);
        z-index: "10001";
        display: "flex";
        flex-direction: "column";
        contain: "layout style paint";
        will-change: "transform";
        transition: format!("transform {} {}", var!(duration-overlay), var!(ease-out));
        overflow: "hidden";
        padding: format!("{} {}", var!(padding-main-top), var!(padding-main-horizontal));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-md), var!(padding-main-horizontal-mobile));
        }
    }

    pub c_vconsole_panel_closed {
        transform: "translateY(100%)";
        will-change: "auto";
    }

    pub c_vconsole_fab {
        position: "fixed";
        bottom: format!("calc(20px + {})", var!(safe-area-inset-bottom));
        right: var!(padding-main-horizontal);
        z-index: "9999";
        @media ((max-width: 767px)) {
            bottom: format!("calc(16px + {})", var!(safe-area-inset-bottom));
            right: var!(padding-main-horizontal-mobile);
        }
    }

    pub c_vconsole_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        flex-shrink: "0";
    }

    pub c_vconsole_title {
        color: var!(foreground);
        font-size: var!(font-md);
        font-weight: "600";
        margin: "0px";
        letter-spacing: "0.03em";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_vconsole_title_dot {
        width: "8px";
        height: "8px";
        background: var!(accent);
        display: "inline-block";
        animation: "euv-pulse 2s ease-in-out infinite";
    }

    pub c_vconsole_header_actions {
        display: "flex";
        gap: var!(space-sm);
        align-items: "center";
    }

    pub c_vconsole_clear_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        text-align: "center";
        border: format!("1.5px solid {}", var!(border));
        margin-left: "auto";
        :focus-visible {
            outline: "none";
        }
    }

    pub c_vconsole_close_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        cursor: "pointer";
        font-size: var!(font-sm);
        font-weight: "400";
        text-align: "center";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
        vertical-align: "middle";
        outline: "none";
        transition: format!("all {} {}", var!(duration-fast), var!(ease-out));
        :hover {
            border-color: var!(foreground);
            color: var!(foreground);
            background: var!(background);
        }
        :focus-visible {
            outline: "none";
        }
        :active {
            background: "transparent";
            color: var!(foreground);
            border-color: var!(border);
        }
    }

    pub c_vconsole_body {
        flex: "1";
        overflow-y: "auto";
        contain: "content";
        padding-bottom: var!(space-md);
        font-family: "ui-monospace, monospace";
        font-size: var!(font-xs);
    }

    pub c_vconsole_log_item {
        display: "flex";
        align-items: "center";
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("1px dashed {}", var!(border));
        color: var!(foreground);
        font-size: var!(font-xs);
        word-break: "break-all";
    }

    pub c_vconsole_empty {
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        text-align: "center";
        padding: format!("{} 0px", var!(space-4xl));
        overflow: "hidden";
    }

    pub c_vconsole_empty_hidden {
        height: "0";
        overflow: "hidden";
        padding: "0";
    }

    pub c_vconsole_log_list {
        overflow: "hidden";
    }

    pub c_vconsole_log_list_hidden {
        height: "0";
        overflow: "hidden";
    }

    pub c_vconsole_count {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: "400";
    }

    pub c_vconsole_filter_bar {
        display: "flex";
        gap: var!(space-sm);
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("1px dashed {}", var!(border));
        flex-shrink: "0";
        align-items: "center";
    }

    pub c_vconsole_filter_badge {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(text-on-accent);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: format!("1px solid {}", var!(accent));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        text-align: "center";
        background: var!(accent);
        :focus-visible {
            outline: "none";
        }
    }

    pub c_vconsole_filter_badge_outline {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        text-align: "center";
        border: format!("1.5px solid {}", var!(border));
    }

    pub c_vconsole_level_badge {
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        margin-right: var!(space-sm);
        letter-spacing: "0.05em";
        color: var!(foreground);
        border: "1px solid currentColor";
        wrap: "nowrap";
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Mobile Layout
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_mobile_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: format!("0px 0px 0px {}", var!(space-lg));
        height: var!(mobile-header-height);
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
        padding-top: "0px";
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
        padding: format!("0px {}", var!(space-lg));
        height: var!(mobile-header-height);
        flex-shrink: "0";
        position: "relative";
        ::after {
            content: "''";
            position: "absolute";
            bottom: "0px";
            left: var!(space-lg);
            right: var!(space-lg);
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
    // Keep-Alive Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_keep_alive_tab_bar {
        display: "flex";
        border-bottom: format!("1px dashed {}", var!(border));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
    }

    pub c_keep_alive_tab_panel {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub c_keep_alive_panel_title {
        margin-top: "0px";
        color: var!(accent);
        font-size: var!(font-md);
        margin-bottom: var!(gap-element);
    }

    pub c_keep_alive_demo_text {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    pub c_keep_alive_counter_display {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-xl));
    }

    pub c_keep_alive_counter_value {
        font-size: var!(font-base);
        font-weight: "700";
        font-variant-numeric: "tabular-nums";
        color: var!(accent);
        text-align: "center";
    }

    pub c_keep_alive_counter_controls {
        display: "flex";
        flex-wrap: "wrap";
        gap: format!("{}", var!(space-md));
    }

    pub c_keep_alive_form_group {
        margin-bottom: var!(gap-component);
    }

    pub c_keep_alive_form_preview {
        margin-top: var!(gap-component);
        background: var!(accent-muted);
    }

    pub c_keep_alive_preview_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        margin: format!("0px 0px {} 0px", var!(space-xs));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // File Upload
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_file_upload_input_hidden {
        position: "absolute";
        width: "1px";
        height: "1px";
        padding: "0px";
        margin: "-1px";
        overflow: "hidden";
        clip: "rect(0, 0, 0, 0)";
        white-space: "nowrap";
        border: "0px";
    }

    pub c_file_upload_options {
        margin: format!("{} 0px", var!(gap-component));
    }

    pub c_file_upload_item {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        margin: format!("{} 0px", var!(space-sm));
        font-size: var!(font-base);
        color: "inherit";
    }

    pub c_file_upload_item_index {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        min-width: "24px";
    }

    pub c_file_upload_item_name {
        color: "inherit";
        word-break: "break-all";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Binding Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_binding_child_box {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
    }

    pub c_binding_child_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub c_binding_parent_box {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
    }

    pub c_binding_section_title {
        margin-top: var!(gap-component);
        margin-bottom: var!(gap-element);
        color: var!(accent);
        font-size: var!(font-md);
        font-weight: "600";
    }

    pub c_binding_temp_converter {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-component);
        flex-wrap: "wrap";
        margin-top: var!(gap-component);
    }

    pub c_binding_temp_field {
        flex: "1";
        min-width: "120px";
    }

    pub c_binding_temp_arrow {
        font-size: "20px";
        font-weight: "700";
        color: var!(accent);
        padding-bottom: "10px";
    }

    pub c_binding_color_mixer {
        margin-top: var!(gap-component);
    }

    pub c_binding_color_preview {
        width: "100%";
        height: "80px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        margin-bottom: var!(gap-component);
    }

    pub c_binding_color_hex {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-xl);
        font-weight: "700";
        color: var!(text-on-accent);
        letter-spacing: "0.02em";
    }

    pub c_binding_slider_row {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        margin-bottom: var!(space-sm);
    }

    pub c_binding_slider_label {
        font-size: var!(font-base);
        font-weight: "700";
        min-width: var!(font-sm);
    }

    pub c_binding_slider {
        flex: "1";
        height: "24px";
        padding: "0px";
        cursor: "pointer";
        -webkit-appearance: "none";
        appearance: "none";
        border: "none";
        outline: "none";
        ::-webkit-slider-runnable-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-webkit-slider-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
            -webkit-appearance: "none";
            margin-top: "-7px";
        }
        : active::-webkit-slider-thumb {
            transform: "scale(0.92)";
        }
        ::-moz-range-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-moz-range-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
        }
        : active::-moz-range-thumb {
            transform: "scale(0.92)";
        }
    }

    pub c_binding_slider_value {
        font-size: var!(font-base);
        font-weight: "500";
        color: "inherit";
        min-width: "32px";
        text-align: "right";
        font-family: "ui-monospace, monospace";
    }

    pub c_binding_typed_prop_value {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        padding: format!("1px {}", var!(space-sm));
    }

    pub c_binding_typed_prop_group {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_binding_typed_warning {
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
        margin-bottom: "0px";
    }

    pub c_binding_form_label {
        display: "block";
        color: "inherit";
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub c_binding_demo_text {
        color: "inherit";
        margin: "0px"
    }

    pub c_binding_compact_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        height: "42px";
        padding: format!("0px {}", var!(space-md));
        background: var!(accent);
        color: var!(text-on-accent);
        border: format!("1px solid {}", var!(accent));
        cursor: "pointer";
        font-size: var!(font-sm);
        font-weight: "600";
        outline: "none";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
        box-sizing: "border-box";
        :hover {
            background: var!(accent);
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom Attrs - Dynamic Style
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_attrs_dynamic_demo(prop_key: &str, prop_value: &str) {
        {
            prop_key
        }
        : prop_value;
        // No layout defaults: this class lives on a `<p>` whose own margin /
        // spacing is controlled by its parent's typography rules, and pinning
        // any colour here would shadow the user-supplied dynamic property.
        // Users are expected to set `color` / `background` / `font-*` /
        // etc. via the two CSS Property Key / Value inputs above; whatever
        // they type becomes the literal CSS rule applied to the paragraph.
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Dynamic Component Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_dynamic_component_tab_bar {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
    }

    pub c_dynamic_component_panel {
        display: "block";
        min-height: var!(min-height-sm);
        margin-top: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Virtual List
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_virtual_list_status {
        display: "flex";
        gap: var!(gap-component);
        padding: format!("{} 0px", var!(gap-component));
        flex-wrap: "nowrap";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        @media ((max-width: 767px)) {
            gap: var!(gap-component-mobile);
        }
    }

    pub c_virtual_list_status_item {
        font-size: var!(font-base);
        color: "inherit";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_virtual_list_status_value {
        font-weight: "700";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
    }

    pub c_virtual_list_container {
        flex: "1";
        height: "666px";
        max-height: "666px";
        overflow-y: "auto";
    }

    pub c_virtual_list_card {
        flex: "1";
        padding: var!(space-xl);
        color: var!(foreground);
        display: "flex";
        flex-direction: "column";
        @media ((max-width: 767px)) {
            padding: var!(space-lg);
        }
    }

    pub c_virtual_list_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-lg);
        height: "100%";
        box-sizing: "border-box";
    }

    pub c_virtual_list_row_index {
        min-width: "36px";
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex-shrink: "0";
        display: "flex";
        align-items: "center";
    }

    pub c_virtual_list_row_label {
        font-size: var!(font-base);
        font-weight: "500";
        color: "inherit";
        min-width: "120px";
        flex-shrink: "0";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        display: "flex";
        align-items: "center";
    }

    pub c_virtual_list_row_description {
        font-size: var!(font-base);
        color: "inherit";
        opacity: "1";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        flex: "1";
        display: "flex";
        align-items: "center";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Unified Layout Primitives
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_element_stack {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    pub c_switcher {
        display: "flex";
        gap: var!(gap-component);
        flex-wrap: "wrap";
        @media ((max-width: 767px)) {
            flex-direction: "column";
            gap: var!(gap-component-mobile);
        }
    }

    pub c_switcher_col {
        flex: "1";
        min-width: "200px";
        @media ((max-width: 767px)) {
            min-width: "100%";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Unified Description / Hint Text
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_hint {
        color: "inherit";
        opacity: "1";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Network Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_net_messages_empty {
        color: "inherit";
        opacity: "1";
        padding: var!(space-xl);
        text-align: "center";
        font-size: var!(font-base);
    }

    pub c_net_messages_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub c_net_message_item {
        display: "flex";
        align-items: "baseline";
        gap: var!(gap-element);
        justify-content: "space-between";
    }

    pub c_net_message_index {
        color: var!(accent);
        font-size: var!(font-sm);
        font-weight: "600";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub c_net_message_data {
        color: "inherit";
        font-size: var!(font-base);
        word-break: "break-all";
        overflow-wrap: "break-word";
        flex: "1";
        min-width: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // WebSocket Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_ws_message_input {
        width: "100%";
        flex: "1";
        height: "42px";
        padding: format!("0px {}", var!(space-lg));
        color: var!(foreground);
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
            background: var!(accent-muted);
        }
    }

    pub c_ws_message_time {
        color: var!(foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        flex-shrink: "0";
        margin-left: "auto";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Sticky & CSS Effects Demo
    // ═══════════════════════════════════════════════════════════════════════════

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page — Section
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_home {
        position: "relative";
        text-align: "center";
        margin-bottom: var!(space-xl);
        flex: "1";
        display: "flex";
        flex-direction: "column";
        justify-content: "center";
        @media ((max-width: 767px)) {
            margin-bottom: var!(space-lg);
            flex: "0 0 auto";
            justify-content: "flex-start";
        }
    }

    pub c_home_content {
        position: "relative";
        z-index: "1";
    }

    pub c_home_badge_row {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        margin-bottom: var!(space-md);
    }

    pub c_home_badge {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(accent);
        font-size: var!(font-xs);
        font-weight: "600";
        letter-spacing: "0.05em";
        border: format!("1px solid {}", var!(accent));
    }

    pub c_home_title {
        font-size: var!(font-5xl);
        font-weight: "800";
        letter-spacing: "-0.03em";
        margin: "0px";
        color: var!(foreground);
        margin-bottom: var!(space-xs);
        @media ((max-width: 767px)) {
            font-size: var!(font-4xl);
        }
    }

    pub c_home_subtitle {
        font-size: var!(font-lg);
        color: var!(foreground);
        margin: "0px auto";
        max-width: "520px";
        margin-bottom: var!(space-lg);
        @media ((max-width: 767px)) {
            font-size: var!(font-base);
        }
    }

    pub c_home_actions {
        display: "flex";
        gap: var!(space-md);
        justify-content: "center";
        flex-wrap: "wrap";
    }

    pub c_home_btn_primary {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-2xl));
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "600";
        text-decoration: "none";
        border: "1.5px solid transparent";
        cursor: "pointer";
        letter-spacing: "0.01em";
        vertical-align: "middle";
        min-height: var!(min-height-sm);
        :focus-visible {
            outline: "none";
        }
        :active {
            background: var!(accent);
            color: var!(text-on-accent);
            border-color: "1.5px solid transparent";
        }
    }

    pub c_home_btn_secondary {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-2xl));
        color: var!(accent);
        font-size: var!(font-base);
        font-weight: "600";
        text-decoration: "none";
        border: format!("1.5px solid {}", var!(accent));
        cursor: "pointer";
        letter-spacing: "0.01em";
        vertical-align: "middle";
        min-height: var!(min-height-sm);
        :focus-visible {
            outline: "none";
        }
        :active {
            background: "transparent";
            color: var!(accent);
            border-color: var!(accent);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page — Stats Row
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_home_stats {
        display: "grid";
        grid-template-columns: "repeat(4, 1fr)";
        gap: var!(space-md);
        margin-bottom: var!(space-xl);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(2, 1fr)";
            gap: var!(space-sm);
        }
    }

    pub c_home_stat_card {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-xs);
    }

    pub c_home_stat_icon {
        font-size: var!(font-2xl);
    }

    pub c_home_stat_value {
        font-size: var!(font-xl);
        font-weight: "800";
        color: var!(foreground);
        letter-spacing: "-0.01em";
    }

    pub c_home_stat_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        font-weight: "500";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page — Section & Feature Cards
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_home_section_title {
        font-size: var!(font-2xl);
        font-weight: "700";
        color: var!(foreground);
        margin: "0px";
        margin: format!("{} 0px", var!(gap-component));
        letter-spacing: "-0.02em";
    }

    pub c_home_section_desc {
        font-size: var!(font-base);
        color: var!(foreground);
        margin: "0px";
        margin-bottom: var!(space-2xl);
    }

    pub c_home_feature_grid {
        display: "grid";
        grid-template-columns: "repeat(2, 1fr)";
        gap: var!(space-md);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_feature_card {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        padding: "0px";
        overflow: "hidden";
    }

    pub c_feature_header {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_feature_icon {
        font-size: var!(font-2xl);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        line-height: "1";
        flex-shrink: "0";
    }

    pub c_feature_name {
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_feature_desc {
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Global Utilities
    // ═══════════════════════════════════════════════════════════════════════════

    // Dev-only readouts produced by `euv_debug`. The base wrapper
    // establishes the inline-flex layout so the label and the value
    // share a baseline; the `code` / `pre` value elements apply
    // monospace font and wrap behaviour for the two modes
    // (`expanded: false` = single-line `code`, `expanded: true` =
    // multi-line `pre`).
    pub c_debug {
        display: "flex";
        flex-direction: "row";
        align-items: "baseline";
        gap: var!(space-sm);
        padding: var!(space-xs);
        border: format!("1px dashed {}", var!(accent-muted));
        border-radius: var!(radius-sm);
        background: var!(surface-muted);
        font-family: "ui-monospace, SFMono-Regular, monospace";
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: format!("{} 0px", var!(space-xs));
    }

    pub c_debug_label {
        font-weight: "600";
        color: var!(accent);
        flex-shrink: "0";
    }

    pub c_debug_value {
        font-family: "ui-monospace, SFMono-Regular, monospace";
        color: var!(foreground);
        margin: "0px";
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
        padding: format!("0px {}", var!(padding-main-horizontal));
        border-bottom: format!("1px solid {}", var!(border));
        background: var!(background);
        z-index: "100";
        @media ((max-width: 767px)) {
            padding: format!("0px {}", var!(padding-main-horizontal-mobile));
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
        hover {
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
        hover {
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
    // euv_dropdown
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_dropdown {
        position: "relative";
    }
    pub c_euv_dropdown_menu {
        position: "absolute";
        top: "44px";
        right: "0px";
        min-width: "140px";
        background: var!(background);
        border: format!("1px solid {}", var!(border));
        box-shadow: var!(shadow-accent-lg);
        flex-direction: "column";
        z-index: "101";
    }
    pub c_euv_dropdown_menu_open {
        display: "flex";
    }
    pub c_euv_dropdown_menu_closed {
        display: "none";
    }
    pub c_euv_dropdown_item {
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        font-size: var!(font-sm);
        text-align: "left";
        cursor: "pointer";
        hover {
            background: var!(accent-muted);
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
        padding: format!("{} {}", var!(space-sm), var!(space-xl));
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        text-align: "left";
        hover {
            background: var!(accent-muted);
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
    pub c_euv_sidebar_children {
        display: "flex";
        flex-direction: "column";
        padding-left: var!(space-md);
        border-left: format!("1px dashed {}", var!(border));
        margin-left: var!(space-sm);
        animation: format!("euv-fade-in {} {}", var!(duration-normal), var!(ease-out));
    }
    pub c_euv_sidebar_link {
        display: "block";
        padding: format!("{} {}", var!(space-sm), var!(space-xl));
        font-size: var!(font-sm);
        color: var!(foreground);
        cursor: "pointer";
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        hover {
            background: var!(accent-muted);
            color: var!(accent);
            box-shadow: format!("inset 4px 0px 0px {}", var!(accent));
        }
    }
    pub c_euv_sidebar_link_active {
        display: "block";
        padding: format!("{} {}", var!(space-sm), var!(space-xl));
        font-size: var!(font-sm);
        background: var!(accent);
        color: var!(text-on-accent);
        font-weight: "600";
        cursor: "pointer";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_toc
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_toc {
        position: "sticky";
        top: "76px";
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
        hover {
            color: var!(accent);
        }
    }
    pub c_euv_toc_link_nested {
        padding-left: var!(space-lg);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_pagination
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_pagination {
        display: "flex";
        justify-content: "space-between";
        gap: var!(gap-component);
        margin-top: var!(space-4xl);
        @media ((max-width: 767px)) {
            flex-direction: "column";
        }
    }
    pub c_euv_pagination_link {
        flex: "1";
        border: format!("1px solid {}", var!(border));
        padding: var!(space-lg);
        cursor: "pointer";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        hover {
            border-color: var!(accent);
        }
    }
    pub c_euv_pagination_label {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        text-transform: "uppercase";
        letter-spacing: "0.08em";
    }
    pub c_euv_pagination_text {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
    }
    pub c_euv_pagination_next {
        text-align: "right";
    }
    pub c_euv_pagination_spacer {
        flex: "1";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_drawer
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_drawer_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        bottom: "0px";
        background: var!(bg-overlay);
        z-index: "200";
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }
    pub c_euv_drawer_overlay_open {
        opacity: "1";
        visibility: "visible";
    }
    pub c_euv_drawer_overlay_closed {
        opacity: "0";
        visibility: "hidden";
    }
    pub c_euv_drawer {
        position: "fixed";
        top: "0px";
        left: "0px";
        bottom: "0px";
        width: "260px";
        background: var!(background);
        border-right: format!("1px solid {}", var!(border));
        z-index: "201";
        overflow-y: "auto";
        padding: var!(space-xl);
        transition: format!("transform {} {}, visibility {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }
    pub c_euv_drawer_open {
        transform: "translateX(0px)";
        visibility: "visible";
    }
    pub c_euv_drawer_closed {
        transform: "translateX(-100%)";
        visibility: "hidden";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_markdown / euv_footer
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_markdown {
        width: "100%";
        min-width: "0px";
    }
    pub c_euv_footer {
        margin-top: var!(space-7xl);
        padding: format!("{} 0px", var!(space-2xl));
        border-top: format!("1px dashed {}", var!(border));
        text-align: "center";
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_result / euv_doc_layout
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_result {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        text-align: "center";
        gap: var!(space-md);
        padding: format!("{} {}", var!(space-4xl), var!(space-md));
        min-height: "50vh";
    }
    pub c_euv_result_code {
        font-size: var!(font-6xl);
        font-weight: "700";
        line-height: "1";
        letter-spacing: "-0.02em";
    }
    pub c_euv_result_title {
        font-size: var!(font-2xl);
        font-weight: "600";
        margin: "0px";
    }
    pub c_euv_result_description {
        color: var!(muted-foreground);
        margin: "0px";
    }
    pub c_euv_result_actions {
        display: "flex";
        gap: var!(space-md);
        margin-top: var!(space-lg);
    }

    pub c_euv_doc_layout {
        display: "flex";
        gap: var!(space-4xl);
        width: "100%";
        min-width: "0px";
        max-width: "1080px";
        margin: "0px auto";
    }
    pub c_euv_doc_content {
        flex: "1";
        min-width: "0px";
        max-width: var!(content-max-width);
    }
    pub c_euv_doc_toc {
        width: "200px";
        flex-shrink: "0";
        @media ((max-width: 1100px)) {
            display: "none";
        }
    }
}
