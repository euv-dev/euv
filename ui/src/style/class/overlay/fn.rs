use super::*;

class! {
    pub c_euv_tooltip {
        position: "relative";
        display: "inline-flex";
        align-items: "center";
    }

    pub c_euv_tooltip_bubble {
        position: "absolute";
        z-index: "100";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "500";
        line-height: "1.3";
        white-space: "nowrap";
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    pub c_euv_tooltip_bubble_top {
        position: "absolute";
        z-index: "100";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "500";
        line-height: "1.3";
        white-space: "nowrap";
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        bottom: "calc(100% + 8px)";
        left: "50%";
        transform: "translateX(-50%)";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    pub c_euv_tooltip_bubble_bottom {
        position: "absolute";
        z-index: "100";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "500";
        line-height: "1.3";
        white-space: "nowrap";
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        top: "calc(100% + 8px)";
        left: "50%";
        transform: "translateX(-50%)";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    pub c_euv_tooltip_bubble_left {
        position: "absolute";
        z-index: "100";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "500";
        line-height: "1.3";
        white-space: "nowrap";
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        top: "50%";
        right: "calc(100% + 8px)";
        transform: "translateY(-50%)";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    pub c_euv_tooltip_bubble_right {
        position: "absolute";
        z-index: "100";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "500";
        line-height: "1.3";
        white-space: "nowrap";
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        top: "50%";
        left: "calc(100% + 8px)";
        transform: "translateY(-50%)";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    pub c_euv_popover {
        position: "relative";
        display: "inline-flex";
        align-items: "center";
    }

    pub c_euv_popover_body {
        position: "absolute";
        z-index: "100";
        min-width: "200px";
        box-sizing: "border-box";
        border-radius: "0px";
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub c_euv_popover_body_open {
        position: "absolute";
        z-index: "100";
        min-width: "200px";
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        box-shadow: var!(shadow-sm);
        border-radius: "0px";
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        opacity: "1";
        visibility: "visible";
        pointer-events: "auto";
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub c_euv_popover_body_closed {
        position: "absolute";
        z-index: "100";
        min-width: "200px";
        box-sizing: "border-box";
        border: format!("1px dashed {}", var!(border));
        border-radius: "0px";
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub c_euv_popover_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub c_euv_popover_title {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
    }

    pub c_euv_calendar {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-md);
        width: "100%";
        box-sizing: "border-box";
        padding: var!(space-md);
        border: format!("1px solid {}", var!(border));
        background: var!(background);
    }

    pub c_euv_calendar_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
    }

    pub c_euv_calendar_title {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
    }

    pub c_euv_calendar_nav {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    pub c_euv_calendar_weekdays {
        display: "grid";
        grid-template-columns: "repeat(7, 1fr)";
        gap: "0px";
        width: "100%";
    }

    pub c_euv_calendar_weekday {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "24px";
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(muted-foreground);
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub c_euv_calendar_grid {
        display: "grid";
        grid-template-columns: "repeat(7, 1fr)";
        gap: "0px";
        width: "100%";
    }

    pub c_euv_calendar_day {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
        background: var!(background);
        border: "1px solid transparent";
        border-radius: "0px";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub c_euv_calendar_day_muted {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: "400";
        color: var!(muted-foreground);
        background: var!(background);
        border: "1px dashed transparent";
        border-radius: "0px";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub c_euv_calendar_day_today {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: "700";
        color: var!(foreground);
        background: var!(background);
        border: format!("2px solid {}", var!(accent));
        border-radius: "0px";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub c_euv_calendar_day_selected {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(text-on-accent);
        background: var!(accent);
        border: format!("2px solid {}", var!(accent));
        border-radius: "0px";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub c_euv_upload {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-md);
        width: "100%";
    }

    pub c_euv_upload_input {
        position: "absolute";
        width: "1px";
        height: "1px";
        opacity: "0";
        pointer-events: "none";
        margin: "0px";
        padding: "0px";
        border: "none";
        outline: "none";
    }

    pub c_euv_upload_drop_active {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(space-7xl);
        box-sizing: "border-box";
        padding: var!(space-2xl);
        border: format!("2px solid {}", var!(accent));
        background: var!(accent-muted);
        color: var!(accent);
        font-size: var!(font-base);
        text-align: "center";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        @media ((max-width: 767px)) {
            padding: var!(space-lg);
        }
    }

    pub c_euv_upload_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        width: "100%";
    }

    pub c_euv_upload_file {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(min-height-base);
        box-sizing: "border-box";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("1px dashed {}", var!(border));
    }

    pub c_euv_upload_file_name {
        font-size: var!(font-sm);
        font-weight: "500";
        color: var!(foreground);
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        min-width: "0px";
    }

    pub c_euv_upload_file_size {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        font-family: "ui-monospace, monospace";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub c_euv_upload_file_status_pending {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: "400";
        color: var!(muted-foreground);
        border: format!("1px dashed {}", var!(border));
        padding: "0px";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub c_euv_upload_file_status_uploading {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: "500";
        color: var!(foreground);
        border: format!("1px solid {}", var!(border));
        padding: "0px";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub c_euv_upload_file_status_done {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(accent);
        border: format!("2px solid {}", var!(accent));
        padding: "0px";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub c_euv_upload_file_status_failed {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: "700";
        color: var!(muted-foreground);
        border: format!("1px dashed {}", var!(foreground));
        padding: "0px";
        text-decoration: "line-through";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub c_euv_feature_grid {
        display: "grid";
        grid-template-columns: "repeat(auto-fit, minmax(220px, 1fr))";
        gap: var!(space-md);
        width: "100%";
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_euv_feature_card {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        padding: var!(space-lg);
        overflow: "hidden";
        box-sizing: "border-box";
        border: format!("1px dashed {}", var!(border));
    }

    pub c_euv_feature_header {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_euv_feature_icon {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-2xl);
        line-height: "1";
        color: var!(foreground);
        flex-shrink: "0";
    }

    pub c_euv_feature_name {
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_euv_feature_desc {
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: "0px";
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
            // Percentage of the fixed overlay (which is already the full
            // window), so the sheet is capped by the screen it sits on
            // rather than by a viewport unit that ignores the safe-area
            // insets the overlay does not account for.
            max-height: "85%";
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
        // A fixed element with `bottom` anchored resolves percentage heights
        // against the initial containing block, so `76%` and `76vh` measure
        // the same box here — the percentage form is used to keep viewport
        // units out of the codebase. The safe-area inset is still subtracted
        // so the panel clears a home indicator.
        height: format!("calc(76% - {})", var!(safe-area-inset-bottom));
        background: var!(background);
        z-index: "10001";
        display: "flex";
        flex-direction: "column";
        contain: "layout style paint";
        will-change: "transform";
        transition: format!("transform {} {}", var!(duration-overlay), var!(ease-out));
        overflow: "hidden";
        padding: format!("{} {}", var!(padding-main-top), var!(edge-gutter));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-md), var!(edge-gutter-mobile));
        }
    }

    pub c_vconsole_panel_closed {
        transform: "translateY(100%)";
        will-change: "auto";
    }

    pub c_vconsole_fab {
        position: "fixed";
        bottom: format!("calc({} + {})", var!(edge-gutter-bottom), var!(safe-area-inset-bottom));
        right: var!(edge-gutter);
        z-index: "9999";
        @media ((max-width: 767px)) {
            bottom: format!("calc({} + {})", var!(edge-gutter-mobile), var!(safe-area-inset-bottom));
            right: var!(edge-gutter-mobile);
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
    // euv_dropdown
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_dropdown {
        position: "relative";
    }

    pub c_euv_dropdown_menu {
        position: "absolute";
        top: "44px";
        right: "0px";
    /* `width: 100%` instead of `min-width: 140px` so the menu
                                                                                                                                                           matches the dropdown container (and therefore the
                                                                                                                                                           trigger button) width exactly. With only `min-width`,
                                                                                                                                                           a trigger wider than 140px (e.g. the 206px-wide
                                                                                                                                                           nav-column locale switcher button) leaves a large
                                                                                                                                                           empty gap on the left of the menu, since `right: 0`
                                                                                                                                                           anchors only the right edge to the container. With
                                                                                                                                                           `width: 100%`, the menu's left edge lines up with the
                                                                                                                                                           trigger's left edge. The default minimum content width
                                                                                                                                                           is still guaranteed by the inner items' own padding,
                                                                                                                                                           so we keep the rule without an explicit min. */
        width: "100%";
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
        // Hover mirrors the sidebar's affordance (see `c_euv_sidebar_link`):
        // an inset 3px bar painted inside the padding box, which never
        // participates in layout and therefore never shifts the label. A
        // translucent wash is deliberately NOT used — `accent-muted` resolves
        // to the same value as `background` in the monochrome palette
        // (white on light, black on dark), so a background there is
        // invisible. `color-mix` derives the wash from the actual text colour
        // so it reads in both themes.
        :hover {
            background: format!("color-mix(in srgb, {} 8%, transparent)", var!(foreground));
            box-shadow: format!("inset 3px 0 0 0 {}", var!(foreground));
        }
    }

    pub c_euv_dropdown_item_active {
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        font-size: var!(font-sm);
        text-align: "left";
        cursor: "pointer";
        font-weight: "600";
        // The selected item wears the accent fill and the text colour that
        // reads on top of it, the same pairing the sidebar's `_active` link
        // uses. The inset bar is dropped: the full-width fill already carries
        // the current-page signal, and a bar in the same monochrome accent
        // would be invisible against it.
        background: var!(accent);
        color: var!(text-on-accent);
        :hover {
            background: var!(accent);
            color: var!(text-on-accent);
            box-shadow: "none";
        }
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
}
