use super::*;

class! {
    pub(crate) c_euv_tabs {
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_tabs_bar {
        display: "flex";
        border-bottom: format!("1px dashed {}", var!(border));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
        @media ((max-width: 767px)) {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_euv_tab_item {
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        border-bottom: "2px solid transparent";
        color: "inherit";
        background: "transparent";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        white-space: "nowrap";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_tab_item_active {
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        border-bottom: format!("2px solid {}", var!(accent));
        color: var!(text-on-accent);
        background: var!(accent);
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        white-space: "nowrap";
    }

    pub(crate) c_euv_tabs_panel {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub(crate) c_euv_collapse_item {
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub(crate) c_euv_collapse_header {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("{} 0px", var!(space-sm));
        cursor: "pointer";
        text-align: "left";
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
        background: "transparent";
        border: "none";
        outline: "none";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_collapse_header_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("{} 0px", var!(space-sm));
        cursor: "pointer";
        text-align: "left";
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        background: var!(accent-muted);
        border: "none";
        outline: "none";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_collapse_body {
        overflow: "hidden";
        font-size: var!(font-base);
        color: var!(foreground);
        transition: format!("max-height {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_euv_collapse_body_open {
        overflow: "hidden";
        max-height: "none";
        opacity: "1";
        padding-bottom: var!(space-md);
        font-size: var!(font-base);
        color: var!(foreground);
        transition: format!("max-height {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_euv_collapse_body_closed {
        overflow: "hidden";
        max-height: "0px";
        opacity: "0";
        font-size: var!(font-base);
        color: var!(foreground);
        transition: format!("max-height {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_euv_breadcrumb {
        display: "flex";
        align-items: "center";
        flex-wrap: "wrap";
        gap: var!(space-xs);
        font-size: var!(font-sm);
    }

    pub(crate) c_euv_breadcrumb_item {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        cursor: "pointer";
        text-decoration: "none";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_breadcrumb_sep {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_breadcrumb_current {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        cursor: "default";
    }

    pub(crate) c_euv_divider {
        display: "block";
        width: "100%";
    }

    pub(crate) c_euv_divider_horizontal {
        display: "block";
        width: "100%";
        height: "0px";
        border: "none";
        border-top: format!("1px solid {}", var!(border));
        margin: format!("{} 0px", var!(space-lg));
    }

    pub(crate) c_euv_divider_vertical {
        display: "block";
        width: "0px";
        height: "auto";
        align-self: "center";
        border: "none";
        border-left: format!("1px solid {}", var!(border));
        margin: format!("0px {}", var!(space-sm));
    }

    pub(crate) c_euv_divider_labeled {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        margin: format!("{} 0px", var!(space-lg));
    }

    pub(crate) c_euv_divider_label_text {
        font-size: var!(font-xs);
        font-weight: "500";
        color: var!(muted-foreground);
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Card Component
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_card {
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

    pub(crate) c_modal_close_button {
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

    pub(crate) c_euv_button_outline_md {
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

    pub(crate) c_badge {
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

    pub(crate) c_badge_outline {
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

    pub(crate) c_euv_tag_solid_black {
        display: "inline-block";
        vertical-align: "middle";
        text-align: "center";
        line-height: "1";
        color: var!(text-on-accent);
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        background: var!(accent);
        border: format!("1px solid {}", var!(accent));
        box-sizing: "border-box";
    }

    pub(crate) c_euv_tag_solid_white {
        display: "inline-block";
        vertical-align: "middle";
        text-align: "center";
        line-height: "1";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(accent));
        box-sizing: "border-box";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Euv Tag Variants (semantic colour tags)
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_tag_outline_black {
        display: "inline-block";
        vertical-align: "middle";
        text-align: "center";
        line-height: "1";
        color: var!(accent);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(accent));
        box-sizing: "border-box";
    }

    pub(crate) c_euv_tag_outline_white {
        display: "inline-block";
        vertical-align: "middle";
        text-align: "center";
        line-height: "1";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(border));
        box-sizing: "border-box";
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

    pub c_counter_value_row {
        margin-top: var!(space-sm);
        word-break: "break-all";
        overflow-wrap: "anywhere";
        font-size: var!(font-base);
        color: "inherit";
        font-weight: "400";
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
        // iOS WebKit: tap on a non-button element inside a scrollable
        // container can be silently dropped because iOS interprets the
        // touch as the start of a scroll gesture and never dispatches the
        // synthetic `click`. `touch-action: manipulation` tells iOS this
        // element only responds to taps and panning — no double-tap-zoom
        // wait, no scroll-gesture disambiguation. `user-select: none`
        // additionally suppresses the iOS text-selection bubble that
        // would otherwise intercept the tap. See ui/src/style/css/fn.rs
        // for the global reset and PR that documents the iOS repro.
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub c_tab_item_inactive {
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        cursor: "pointer";
        border-bottom: "2px solid transparent";
        color: "inherit";
        font-size: var!(font-base);
        font-weight: "500";
        // iOS WebKit fix — see c_tab_item_active above for the rationale.
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
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
    // euv_pagination
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_pagination {
        // CSS grid with two equal columns gives us perfect 50/50 widths
        // regardless of the link content length, which flex with `1 1
        // auto` cannot (each link starts at its own content size). The
        // mobile breakpoint stacks the grid to a single column so each
        // link sizes to its content.
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: var!(gap-component);
        // ~`space-2xl` (1.5rem) above the pagination — comfortable
        // breathing room without a huge gap before prev/next. The original
        // `space-4xl` (2.5rem) margin was doubled by a `padding-top` in the
        // docs app override, leaving ~100px of empty space above the
        // pagination boxes which made the page feel bottom-heavy.
        margin-top: var!(space-2xl);
        // `min-width: 0` lets prev/next text actually use ellipsis when the
        // container is narrow — without it the grid track blows out the row.
        min-width: "0px";
        @media ((max-width: 767px)) {
            grid-template-columns: "minmax(0, 1fr)";
        }
    }

    pub c_euv_pagination_link {
        // `min-width: 0` lets long labels ellipsize on the desktop row
        // layout. `overflow: hidden` is required to actually apply ellipsis
        // when content is wider than the link.
        min-width: "0px";
        border: format!("1px solid {}", var!(border));
        padding: var!(space-lg);
        cursor: "pointer";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        overflow: "hidden";
        :hover {
            border-color: var!(accent);
        }
    }

    pub(crate) c_euv_pagination_label {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        text-transform: "uppercase";
        letter-spacing: "0.08em";
    }

    pub(crate) c_euv_pagination_text {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        // Ellipsize when the link is too narrow for the page title. The
        // outer `.c_euv_pagination_link` already sets `overflow: hidden`
        // and `min-width: 0`, so a single `text-overflow: ellipsis` here
        // is enough — without `white-space: nowrap` the text wraps to
        // multiple lines and the ellipsis never kicks in.
        white-space: "nowrap";
        overflow: "hidden";
        text-overflow: "ellipsis";
        min-width: "0px";
    }

    pub(crate) c_euv_pagination_next {
        text-align: "right";
    }

    pub(crate) c_euv_pagination_spacer {
        flex: "1";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_markdown / euv_footer
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_markdown {
        width: "100%";
        min-width: "0px";
    }

    pub c_euv_footer {
        // Footer sits at the natural end of the article column. The user
        // reaches it by scrolling to the bottom of the article — no sticky
        // or fixed positioning. This matches the desktop behavior and
        // Avoid any layout interference on mobile (where a sticky footer
        // would overlap the last lines of content while scrolling).
        // `space-2xl` (1.5rem) above the footer matches the gap between
        // article and pagination, keeping a consistent rhythm in the
        // tail block. The previous `space-7xl` (5rem) was far beyond the
        // typography scale and made the footer feel detached from the
        // pagination.
        margin-top: var!(space-2xl);
        // The dashed rule is this element's own `border-top`, so the
        // padding below is the gap from the rule to the text and the
        // padding above is the gap from the preceding content to the rule.
        // Only one of them belongs to the band inside the rule, and it is
        // the top one — that is the space between the divider and the text.
        // `space-md` keeps the text close to the rule; the block is
        // `display: flex` with `align-items: center` so the line is
        // centred in the row the padding defines rather than riding its
        // top edge.
        padding: format!("{} 0px", var!(space-md));
        border-top: format!("1px dashed {}", var!(border));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        text-align: "center";
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_result / euv_doc_layout
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_result {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        text-align: "center";
        gap: var!(space-md);
        padding: format!("{} {}", var!(space-4xl), var!(space-md));
        // Half the available height, not half the viewport: the result block
        // lives inside a scroll container, so a viewport unit would be taller
        // than the visible area on short pages and leave the buttons below
        // the fold.
        min-height: "50%";
    }

    pub(crate) c_euv_result_code {
        font-size: var!(font-6xl);
        font-weight: "700";
        line-height: "1";
        letter-spacing: "-0.02em";
    }

    pub(crate) c_euv_result_title {
        font-size: var!(font-2xl);
        font-weight: "600";
        margin: "0px";
    }

    pub(crate) c_euv_result_description {
        color: var!(muted-foreground);
        margin: "0px";
    }

    pub(crate) c_euv_result_actions {
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
        // Fill the visible area of the scroll container (`c_app_main`,
        // `overflow: auto` with a definite `height: 100%`) so the content
        // column has room to push its tail to the bottom. A viewport unit
        // would be wrong here: `100vh` measures the whole window including
        // the app header, overshooting the visible area by the header
        // height and pushing pagination/footer one header past the fold.
        // Being a direct child of the scroll container, a percentage does
        // resolve against its content box — put it one level deeper and
        // the intermediate auto-height flex row makes `100%` degenerate to
        // `auto`.
        min-height: "100%";
        // As a flex item of the scroll container's column, the default
        // `flex-shrink: 1` lets this row be compressed to the `min-height`
        // floor — 824px here — even though its content is 12780px tall. A
        // sticky descendant can only travel inside its containing block, so
        // the clamped row made `c_euv_doc_toc` unpin after roughly one
        // viewport of scroll. Never shrink: the row must carry the article's
        // full height so the sticky range spans the whole page.
        flex-shrink: "0";
    }

    pub c_euv_doc_content {
        // Two parallel children — the article body and the
        // `c_euv_doc_tail` wrapper (pagination + footer). The flex column
        // distributes them at the two ends:
        //   - when the article is shorter than the column's intrinsic
        //     height, `justify-content: space-between` pushes the tail to
        //     the bottom edge;
        //   - when the article is longer, the tail sits right after the
        //     article in normal flow (because content height exceeds
        //     min-height, there's no remaining space to distribute).
        // The scroll container is `c_app_main` (`overflow: auto`), not the
        // window, so a viewport unit measures the whole window including the
        // app header. `100vh` therefore overshoots by the header height and
        // pushes the tail one header past the fold on a short page. Height
        // comes from `c_euv_doc_layout` (a direct child of the scroll
        // container, where a percentage resolves); here `align-items:
        // stretch` — the flex default — makes this column take that height
        // so `space-between` has the full column to distribute across.
        //   - article shorter than the column: `space-between` drops the
        //     tail to the bottom edge of the visible area;
        //   - article longer: the column grows with it and the tail follows
        //     the body in normal flow, scrolling with the page.
        flex: "1";
        min-width: "0px";
        max-width: var!(content-max-width);
        display: "flex";
        flex-direction: "column";
        justify-content: "space-between";
    }

    pub c_euv_doc_body {
        // Groups the page title and the rendered markdown into a single
        // flex item. Without this the column would hold the `<h1>` and
        // the `<article>` as two separate items, and
        // `justify-content: space-between` would push them apart — a wide
        // blank band between the title and the first paragraph on any
        // page whose body is shorter than the column.
        display: "flex";
        flex-direction: "column";
        // The body takes its natural height; only the tail is pushed to
        // the far end. `flex-shrink: 0` keeps a long body from being
        // compressed when the column is height-constrained.
        flex: "0 0 auto";
        min-width: "0px";
    }

    pub c_euv_doc_tail {
        // Plain block wrapper that groups pagination + footer into one
        // flex child. With `display: contents` the wrapper would be
        // skipped and the two siblings would behave as direct children
        // of `c_euv_doc_content` — breaking the two-parallel-containers
        // contract that `justify-content: space-between` relies on.
        display: "block";
    }

    pub c_euv_doc_toc {
        width: "200px";
        flex-shrink: "0";
        // Pin the TOC column to the viewport while the main content scrolls.
        // The sticky scroll container is `c_app_main` (the overflow:auto
        // wrapper), so this offset is measured from that container's top edge.
        // It tracks `padding-main-top` rather than a bare `0` so the pinned
        // column keeps the same breathing room below the container edge as the
        // article has below the same edge — the two columns stay on one
        // vertical rhythm instead of the TOC riding flush against the top.
        position: "sticky";
        top: var!(padding-main-top);
        @media ((max-width: 1100px)) {
            display: "none";
        }
    }

    // Makes the element's own box disappear while its children stay in the
    // parent's layout flow, so a wrapper can carry a key or a conditional
    // without contributing a box.
    //
    // The docs shell and page wrappers spelled this as an inline
    // `style: "display: contents"`; the rule is the same either way, but a
    // named class keeps the markup declarative and lets the browser cache it.
    pub c_euv_display_contents {
        display: "contents";
    }
}
