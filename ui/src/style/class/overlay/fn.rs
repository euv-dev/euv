use super::*;

#[cfg(not(feature = "theme-custom"))]
class! {
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
        border: format!("{} solid {}", var!(border-width), var!(border));
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
        font-weight: var!(font-weight-semibold);
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

    pub c_modal_actions {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(space-md);
        margin: format!("{} 0px", var!(space-md));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // File Upload
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_file_upload_input_hidden {
        position: "absolute";
        width: "1px";
        height: "1px";
        padding: "0px";
        // The visually hidden recipe: a 1px box pulled 1px out of the flow,
        // so it clips at the edge of its parent instead of taking a slot.
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
        font-weight: var!(font-weight-semibold);
        color: var!(primary);
        min-width: "24px";
    }

    pub c_file_upload_item_name {
        color: "inherit";
        word-break: "break-all";
    }

    pub(crate) c_euv_tooltip {
        position: "relative";
        display: "inline-flex";
        align-items: "center";
    }

    pub(crate) c_euv_tooltip_bubble {
        position: "absolute";
        z-index: "140";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
        line-height: "1.3";
        white-space: "nowrap";
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_euv_tooltip_bubble_top {
        position: "absolute";
        z-index: "140";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
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

    pub(crate) c_euv_tooltip_bubble_bottom {
        position: "absolute";
        z-index: "140";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
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

    pub(crate) c_euv_tooltip_bubble_left {
        position: "absolute";
        z-index: "140";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
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

    pub(crate) c_euv_tooltip_bubble_right {
        position: "absolute";
        z-index: "140";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
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

    pub(crate) c_euv_popover {
        position: "relative";
        display: "inline-flex";
        align-items: "center";
    }

    pub(crate) c_euv_popover_body {
        position: "absolute";
        z-index: "150";
        min-width: "200px";
        box-sizing: "border-box";
        border-radius: var!(radius-md);
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
        font-size: var!(font-base);
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub(crate) c_euv_popover_body_open {
        position: "absolute";
        z-index: "150";
        min-width: "200px";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        box-shadow: var!(shadow-md);
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        opacity: "1";
        visibility: "visible";
        pointer-events: "auto";
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub(crate) c_euv_popover_body_closed {
        position: "absolute";
        z-index: "150";
        min-width: "200px";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub(crate) c_euv_popover_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_popover_title {
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
        color: var!(foreground);
        margin: "0px";
    }

    pub(crate) c_euv_calendar {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-md);
        width: "100%";
        box-sizing: "border-box";
        padding: var!(space-md);
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
    }

    pub(crate) c_euv_calendar_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
    }

    pub(crate) c_euv_calendar_title {
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
        color: var!(foreground);
        margin: "0px";
    }

    pub(crate) c_euv_calendar_nav {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    pub(crate) c_euv_calendar_weekdays {
        display: "grid";
        grid-template-columns: "repeat(7, minmax(0px, 1fr))";
        gap: "0px";
        width: "100%";
    }

    pub(crate) c_euv_calendar_weekday {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "24px";
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        color: var!(muted-foreground);
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_calendar_grid {
        display: "flex";
        flex-direction: "column";
        gap: "0px";
        width: "100%";
    }

    pub(crate) c_euv_calendar_grid_row {
        display: "grid";
        grid-template-columns: "repeat(7, minmax(0px, 1fr))";
        gap: "0px";
        width: "100%";
    }

    pub(crate) c_euv_calendar_grid_cell {
        display: "flex";
        min-width: "0px";
        width: "100%";
    }

    pub(crate) c_euv_calendar_grid_cell_button {
        width: "100%";
        min-width: "0px";
    }

    pub(crate) c_euv_calendar_day {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: var!(font-weight-medium);
        color: var!(foreground);
        background: var!(background);
        border: format!("{} solid transparent", var!(border-width));
        border-radius: var!(radius-md);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_calendar_day_muted {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: var!(font-weight-normal);
        color: var!(muted-foreground);
        background: var!(background);
        border: format!("{} solid transparent", var!(border-width));
        border-radius: var!(radius-md);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_calendar_day_today {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: var!(font-weight-bold);
        color: var!(foreground);
        background: var!(background);
        border: format!("2px solid {}", var!(primary));
        border-radius: var!(radius-md);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_calendar_day_selected {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        box-sizing: "border-box";
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
        color: var!(primary-foreground);
        background: var!(primary);
        border: format!("2px solid {}", var!(primary));
        border-radius: var!(radius-md);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_upload {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-md);
        width: "100%";
    }

    pub(crate) c_euv_upload_input {
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

    pub(crate) c_euv_upload_drop_active {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(space-7xl);
        box-sizing: "border-box";
        padding: var!(space-2xl);
        border: format!("2px solid {}", var!(primary));
        background: var!(muted);
        color: var!(primary);
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

    pub(crate) c_euv_upload_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        width: "100%";
    }

    pub(crate) c_euv_upload_file {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(control-height);
        box-sizing: "border-box";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_upload_file_name {
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        color: var!(foreground);
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        min-width: "0px";
    }

    pub(crate) c_euv_upload_file_size {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        font-family: var!(font-family-mono);
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub(crate) c_euv_upload_file_status_pending {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-normal);
        color: var!(muted-foreground);
        border: format!("{} solid {}", var!(border-width), var!(border));
        padding: "0px";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub(crate) c_euv_upload_file_status_uploading {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
        color: var!(foreground);
        border: format!("{} solid {}", var!(border-width), var!(border));
        padding: "0px";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub(crate) c_euv_upload_file_status_done {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        color: var!(primary);
        border: format!("2px solid {}", var!(primary));
        padding: "0px";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub(crate) c_euv_upload_file_status_failed {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        color: var!(muted-foreground);
        border: format!("{} solid {}", var!(border-width), var!(foreground));
        padding: "0px";
        text-decoration: "line-through";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Modal
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_modal_overlay {
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

    pub(crate) c_modal_content {
        background: var!(popover);
        color: var!(popover-foreground);
        padding: "0px";
        max-width: "480px";
        width: "90%";
        animation: format!("euv-scale-in-modal {} {}", var!(duration-modal-content), var!(ease-bounce));
        overflow: "hidden";
        color: var!(foreground);
        box-sizing: "border-box";
        border-radius: var!(radius-xl);
        box-shadow: var!(shadow-xl);
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

    pub(crate) c_modal_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: format!("{} {} 0px {}", var!(space-md), var!(space-xl), var!(space-xl));
    }

    pub(crate) c_modal_title {
        margin: "0px";
        font-size: var!(font-xl);
        font-weight: var!(font-weight-semibold);
        color: "inherit";
    }

    pub(crate) c_modal_body {
        padding: format!("0px {} {} {}", var!(space-xl), var!(space-md), var!(space-xl));
        color: "inherit";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VConsole Panel
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_vconsole_badge {
        position: "absolute";
        top: "-6px";
        right: "-8px";
        min-width: "18px";
        height: "18px";
        background: var!(background);
        color: var!(foreground);
        font-size: "10px";
        font-weight: var!(font-weight-semibold);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: format!("0px {}", var!(space-xs));
        border: format!("{} solid {}", var!(border-width), var!(primary));
        pointer-events: "none";
        transition: format!("transform {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_vconsole_overlay {
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

    pub(crate) c_vconsole_overlay_hidden {
        opacity: "0";
        pointer-events: "none";
    }

    pub(crate) c_vconsole_panel {
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
        box-shadow: var!(shadow-lg);
        padding: format!("{} {}", var!(padding-main-top), var!(edge-gutter));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(space-md), var!(edge-gutter-mobile));
        }
    }

    pub(crate) c_vconsole_panel_closed {
        transform: "translateY(100%)";
        will-change: "auto";
    }

    pub(crate) c_vconsole_fab {
        position: "fixed";
        bottom: format!("calc({} + {})", var!(edge-gutter-bottom), var!(safe-area-inset-bottom));
        right: var!(edge-gutter);
        z-index: "9999";
        @media ((max-width: 767px)) {
            bottom: format!("calc({} + {})", var!(edge-gutter-mobile), var!(safe-area-inset-bottom));
            right: var!(edge-gutter-mobile);
        }
    }

    pub(crate) c_vconsole_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        flex-shrink: "0";
    }

    pub(crate) c_vconsole_title {
        color: var!(foreground);
        font-size: var!(font-md);
        font-weight: var!(font-weight-semibold);
        margin: "0px";
        letter-spacing: var!(letter-spacing-normal);
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub(crate) c_vconsole_title_dot {
        width: "8px";
        height: "8px";
        background: var!(primary);
        display: "inline-block";
        animation: "euv-pulse 2s ease-in-out infinite";
    }

    pub(crate) c_vconsole_header_actions {
        display: "flex";
        gap: var!(space-sm);
        align-items: "center";
    }

    pub(crate) c_vconsole_clear_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        cursor: "pointer";
        text-align: "center";
        // A button is a nowrap element (§7): the label never wraps, so the
        // border of the inline flex box stays where it belongs.
        white-space: "nowrap";
        border: format!("1.5px solid {}", var!(border));
        margin-left: "auto";
        :focus-visible {
            outline: "none";
        }
    }

    pub(crate) c_vconsole_close_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        cursor: "pointer";
        font-size: var!(font-sm);
        font-weight: var!(font-weight-normal);
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

    pub(crate) c_vconsole_body {
        flex: "1";
        overflow-y: "auto";
        contain: "content";
        padding-bottom: var!(space-md);
        font-family: var!(font-family-mono);
        font-size: var!(font-xs);
    }

    pub(crate) c_vconsole_log_item {
        display: "flex";
        align-items: "center";
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        color: var!(foreground);
        font-size: var!(font-xs);
        word-break: "break-all";
    }

    pub(crate) c_vconsole_empty {
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        text-align: "center";
        padding: format!("{} 0px", var!(space-4xl));
        overflow: "hidden";
    }

    pub(crate) c_vconsole_empty_hidden {
        height: "0";
        overflow: "hidden";
        padding: "0";
    }

    pub(crate) c_vconsole_log_list {
        overflow: "hidden";
    }

    pub(crate) c_vconsole_log_list_hidden {
        height: "0";
        overflow: "hidden";
    }

    pub(crate) c_vconsole_count {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-normal);
    }

    pub(crate) c_vconsole_filter_bar {
        display: "flex";
        gap: var!(space-sm);
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        flex-shrink: "0";
        align-items: "center";
    }

    pub(crate) c_vconsole_filter_badge {
        // The wrapping-atom recipe (§7): same kind of element as the tag and
        // the badge, so it gets the same box.
        display: "inline-block";
        vertical-align: "middle";
        text-align: "center";
        line-height: "1";
        box-sizing: "border-box";
        color: var!(primary-foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: format!("{} solid {}", var!(border-width), var!(primary));
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        cursor: "pointer";
        background: var!(primary);
        :focus-visible {
            outline: "none";
        }
    }

    pub(crate) c_vconsole_filter_badge_outline {
        display: "inline-block";
        vertical-align: "middle";
        text-align: "center";
        line-height: "1";
        box-sizing: "border-box";
        color: var!(foreground);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        cursor: "pointer";
        border: format!("1.5px solid {}", var!(border));
    }

    pub(crate) c_vconsole_level_badge {
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        margin-right: var!(space-sm);
        letter-spacing: var!(letter-spacing-wide);
        color: var!(foreground);
        border: format!("{} solid currentColor", var!(border-width));
        wrap: "nowrap";
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_dropdown
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_dropdown {
        position: "relative";
    }

    pub(crate) c_euv_dropdown_menu {
        position: "absolute";
        // Sits flush under the trigger. The row is 36px tall and the container
        // gives it no padding, so `top: 36px` is the trigger's bottom edge —
        // any larger value opens a gap that reads as a break between the
        // button and its menu. This was hardcoded to 44px, which is why the
        // menu floated 8px below the button it belongs to.
        top: "44px";
        right: "0px";
        // `width: 100%` so the menu matches the dropdown container (and
        // therefore the trigger button) width exactly. With only `min-width`,
        // a trigger wider than 140px (e.g. the nav-column locale switcher
        // button) leaves a large empty gap on the left of the menu, since
        // `right: 0` anchors only the right edge to the container.
        width: "100%";
        background: var!(popover);
        color: var!(popover-foreground);
        border: format!("{} solid {}", var!(border-width), var!(border));
        box-shadow: var!(shadow-md);
        flex-direction: "column";
        z-index: "101";
        border-radius: var!(radius-md);
    }

    pub(crate) c_euv_dropdown_menu_open {
        display: "flex";
    }

    pub(crate) c_euv_dropdown_menu_closed {
        display: "none";
    }

    pub(crate) c_euv_dropdown_item {
        // `control-row-pad` vertically, the same value the sidebar rows and the
        // CTA buttons use: 9 + 24 line + 9 + 3 left border = 42. The row is
        // only bordered on its left edge, so nothing stacks between neighbours
        // and the stack reads as one continuous column.
        //
        // It was `space-sm` (8px) for as long as the row wore a 1px border on
        // all four sides: 8 + 24 + 8 + 1 + 1 + 3 = 45 inside a 42px
        // border-box, so the browser gave back a fraction of the padding on
        // every row and the options came out with a hairline between them.
        // Dropping the stacked borders makes 9px the right number again.
        padding: format!("{} {}", var!(control-row-pad), var!(space-lg));
        font-size: var!(font-sm);
        text-align: "left";
        cursor: "pointer";
        // Rows must agree on height or the hover bar never lines up with the
        // active fill. `line-height: normal` is at the mercy of the glyphs:
        // English and Korean settle on a 35px row while Japanese takes 38px,
        // and the language switcher lists all three at once. Pinning the line
        // height is what makes the stack even, so the left bar can span a row
        // that matches the active background exactly. 24px is the tallest
        // of the three natural line heights *and* the body line height, so no
        // script is clipped and the row matches the sidebar and the buttons.
        line-height: "24px";
        text-box-trim: "none";
        text-box-edge: "none";
        // Hover signals through the left edge only — no background wash. The
        // menu sits over page content, so tinting the whole row made an
        // unselected option read as a highlighted page link and washed out
        // the label underneath it. The left border carries the affordance
        // without painting behind the text or boxing the row.
        //
        // Only the LEFT edge is bordered — 3px, transparent until hover, so
        // the hover bar needs no width change and the label never shifts.
        //
        // This used to be `border: 1px solid transparent` plus
        // `border-left-width: 3px`, i.e. a box on all four sides. In a stack
        // of rows every horizontal edge is shared between two neighbours, so
        // the transparent top and bottom of one row sat against those of the
        // next. Nothing is visible at rest, but each row then had to fit 8px +
        // 24px line + 8px + 1px + 1px + 3px = 45px of content into a 42px
        // border-box, and the browser reclaimed the overflow by trimming the
        // padding — leaving a hairline between every pair of options. Only the
        // left edge remains, so nothing stacks and 9 + 24 + 9 + 3 = 42 lands
        // exactly.
        border-left: "3px solid transparent";
        :hover {
            // Only the left edge reacts. The previous rule lit the whole
            // perimeter, which framed the option as a card floating over the
            // menu; the locale list is a plain vertical stack, so a box on
            // every side read as a selection rather than a pointer position.
            // The reserved left border carries the affordance on its own.
            border-left-color: var!(foreground);
        }
    }

    pub(crate) c_euv_dropdown_item_active {
        // Same vertical padding as `c_euv_dropdown_item` (`control-row-pad`) —
        // the accent fill has to be exactly as tall as the row it replaces.
        padding: format!("{} {}", var!(control-row-pad), var!(space-lg));
        font-size: var!(font-sm);
        text-align: "left";
        cursor: "pointer";
        font-weight: var!(font-weight-semibold);
        // The same pinned line height as `c_euv_dropdown_item`, for the same
        // reason: a hovered row's bar has to match this fill to the pixel.
        line-height: "24px";
        // Only the left edge, for the same reason as `c_euv_dropdown_item`:
        // a border on all four sides stacks between neighbouring rows. The
        // accent fill spans the full box either way, and reserving the same
        // 3px left edge keeps a hovered row's bar and an active row's fill
        // aligned edge for edge and at the same width.
        border-left: "3px solid transparent";
        // The selected item wears the accent fill and the text colour that
        // reads on top of it, the same pairing the sidebar's `_active` link
        // uses. The inset bar is dropped: the full-width fill already carries
        // the current-page signal, and a bar in the same monochrome accent
        // would be invisible against it.
        background: var!(primary);
        color: var!(primary-foreground);
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
            box-shadow: "none";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_drawer
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_drawer_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        bottom: "0px";
        background: var!(bg-overlay);
        z-index: "200";
        transition: format!("opacity {} {}, visibility {} {}", var!(duration-overlay), var!(ease-out), var!(duration-overlay), var!(ease-out));
    }

    pub(crate) c_euv_drawer_overlay_open {
        opacity: "1";
        visibility: "visible";
    }

    pub(crate) c_euv_drawer_overlay_closed {
        opacity: "0";
        visibility: "hidden";
    }

    pub(crate) c_euv_drawer {
        position: "fixed";
        top: "0px";
        left: "0px";
        bottom: "0px";
        width: "260px";
        background: var!(background);
        border-right: format!("{} solid {}", var!(border-width), var!(border));
        z-index: "201";
        overflow-y: "auto";
        padding: var!(space-xl);
        transition: format!("transform {} {}, visibility {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_euv_drawer_open {
        transform: "translateX(0px)";
        visibility: "visible";
    }

    pub(crate) c_euv_drawer_closed {
        transform: "translateX(-100%)";
        visibility: "hidden";
    }

    // The reference drawer is a sheet that rises from the bottom edge.
    // The panel sits across the full width, keeps its own rounded top
    // corners and carries the hairline that separates it from the page.
    pub(crate) c_euv_drawer_content {
        position: "fixed";
        left: "0px";
        right: "0px";
        bottom: "0px";
        display: "flex";
        flex-direction: "column";
        height: "auto";
        margin-top: var!(space-3xl);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-lg);
        background: var!(background);
        color: var!(foreground);
        z-index: "201";
        transition: format!("transform {} {}, visibility {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_euv_drawer_content_open {
        transform: "translateY(0px)";
        visibility: "visible";
    }

    pub(crate) c_euv_drawer_content_closed {
        transform: "translateY(100%)";
        visibility: "hidden";
    }

    // The handle is the small bar at the top of the sheet that says the
    // panel can be dragged.
    pub(crate) c_euv_drawer_handle {
        width: "100px";
        height: var!(space-xs);
        margin: format!("{} auto 0px", var!(space-lg));
        border-radius: var!(radius-full);
        background: var!(muted);
    }

    pub(crate) c_euv_drawer_trigger {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
    }

    pub(crate) c_euv_drawer_header {
        display: "grid";
        gap: var!(space-2xs);
        padding: var!(space-lg);
        text-align: "center";
    }

    pub(crate) c_euv_drawer_footer {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        margin-top: "auto";
        padding: var!(space-lg);
    }

    pub(crate) c_euv_drawer_title {
        font-size: var!(font-lg);
        font-weight: var!(font-weight-semibold);
        line-height: "1";
        letter-spacing: var!(letter-spacing-tight);
    }

    pub(crate) c_euv_drawer_description {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_drawer_close {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Message (toast)
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_message_stack {
        position: "fixed";
        top: var!(space-xl);
        left: "50%";
        transform: "translateX(-50%)";
        z-index: "1100";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        max-width: "calc(100% - 32px)";
        pointer-events: "none";
    }

    pub(crate) c_euv_message {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-width: "240px";
        max-width: "420px";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        box-shadow: var!(shadow-md);
        font-size: var!(font-sm);
        box-sizing: "border-box";
        pointer-events: "auto";
    }

    pub(crate) c_euv_message_text {
        flex: "1";
        min-width: "0px";
        overflow-wrap: "anywhere";
    }

    pub(crate) c_euv_message_action {
        flex-shrink: "0";
        padding: "0px";
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        text-decoration: "underline";
        cursor: "pointer";
        touch-action: "manipulation";
    }

    pub(crate) c_euv_message_close {
        flex-shrink: "0";
        width: var!(space-lg);
        height: var!(space-lg);
        padding: "0px";
        border: "none";
        background: "transparent";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        touch-action: "manipulation";
        :hover {
            color: var!(foreground);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Notification
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_notification_stack {
        position: "fixed";
        top: var!(space-xl);
        right: var!(space-xl);
        z-index: "1200";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        max-width: "calc(100% - 32px)";
        pointer-events: "none";
        @media ((max-width: 767px)) {
            right: var!(space-lg);
            left: var!(space-lg);
            max-width: "none";
        }
    }

    pub(crate) c_euv_notification {
        display: "flex";
        align-items: "flex-start";
        gap: var!(space-sm);
        width: "320px";
        max-width: "100%";
        padding: var!(space-md);
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        box-shadow: var!(shadow-md);
        font-size: var!(font-sm);
        box-sizing: "border-box";
        pointer-events: "auto";
        @media ((max-width: 767px)) {
            width: "100%";
        }
    }

    pub(crate) c_euv_notification_body {
        flex: "1";
        min-width: "0px";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub(crate) c_euv_notification_title {
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
        color: var!(foreground);
    }

    pub(crate) c_euv_notification_text {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        overflow-wrap: "anywhere";
    }

    pub(crate) c_euv_notification_close {
        flex-shrink: "0";
        width: var!(space-lg);
        height: var!(space-lg);
        padding: "0px";
        border: "none";
        background: "transparent";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        touch-action: "manipulation";
        :hover {
            color: var!(foreground);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Pop Confirm
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_popconfirm {
        position: "relative";
        display: "inline-flex";
        align-items: "center";
    }

    pub(crate) c_euv_popconfirm_body {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "220px";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        box-shadow: var!(shadow-md);
    }

    pub(crate) c_euv_popconfirm_body_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "220px";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        box-shadow: var!(shadow-md);
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_popconfirm_body_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "220px";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        box-shadow: var!(shadow-md);
        display: "none";
    }

    pub(crate) c_euv_popconfirm_text {
        padding: var!(space-md);
        color: var!(foreground);
        font-size: var!(font-sm);
        overflow-wrap: "anywhere";
    }

    pub(crate) c_euv_popconfirm_actions {
        display: "flex";
        align-items: "center";
        justify-content: "flex-end";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }
    pub c_euv_command {
        display: "flex";
        flex-direction: "column";
        width: "100%";
        height: "100%";
        overflow: "hidden";
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
    }

    pub(crate) c_euv_command_shadcn {
        border-radius: var!(radius-lg);
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        padding: var!(space-xs);
    }

    pub c_euv_command_input_wrapper {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("0px {}", var!(space-md));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_command_input_wrapper_shadcn {
        padding: var!(space-xs);
    }

    pub c_euv_command_input {
        display: "flex";
        width: "100%";
        height: var!(space-4xl);
        padding: var!(space-md);
        border: "none";
        border-radius: var!(radius-md);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        outline: "none";
    }

    pub c_euv_command_list {
        max-height: "300px";
        padding: var!(space-xs);
        overflow-x: "hidden";
        overflow-y: "auto";
    }

    pub c_euv_command_empty {
        padding: format!("{} 0px", var!(space-2xl));
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
    }

    pub c_euv_command_group {
        padding: var!(space-xs);
        overflow: "hidden";
        color: var!(foreground);
    }

    pub c_euv_command_group_heading {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: "500";
    }

    pub c_euv_command_item {
        position: "relative";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        font-size: var!(font-sm);
        cursor: "default";
        user-select: "none";
    }

    pub(crate) c_euv_command_item_shadcn {
        padding: var!(space-sm);
        border-bottom-right-radius: var!(radius-sm);
        border-bottom-left-radius: var!(radius-sm);
        border-top-right-radius: var!(radius-sm);
        border-top-left-radius: var!(radius-sm);
        padding-top: var!(space-xs-sm);
    }

    pub c_euv_command_item_selected {
        background: var!(accent);
        color: var!(accent-foreground);
    }

    pub c_euv_command_item_disabled {
        opacity: "0.5";
        pointer-events: "none";
    }

    pub c_euv_command_separator {
        height: "1px";
        margin: format!("0px calc(-1 * {})", var!(space-xs));
        background: var!(border);
    }

    pub c_euv_command_shortcut {
        margin-left: "auto";
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        letter-spacing: var!(letter-spacing-wide);
    }

    pub c_euv_combobox {
        position: "relative";
        display: "inline-flex";
        flex-direction: "column";
    }

    pub c_euv_combobox_trigger {
        display: "inline-flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        height: var!(space-2xl);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub(crate) c_euv_combobox_trigger_shadcn {
        height: var!(control-height-sm);
        min-height: var!(control-height-sm);
        padding: format!("{} {}", var!(space-sm), var!(control-pad-inline));
        border-radius: var!(radius-lg);
        font-size: var!(font-sm);
        line-height: "1.428571";
        gap: var!(space-xs-sm);
    }

    pub c_euv_combobox_input {
        display: "flex";
        width: "100%";
        height: var!(space-2xl);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        outline: "none";
    }

    pub(crate) c_euv_combobox_input_shadcn {
        height: var!(control-height-sm);
        min-height: var!(control-height-sm);
        padding: format!("{} {}", var!(space-xs), var!(control-pad-inline));
        border-radius: var!(radius-lg);
        font-size: var!(font-sm);
        line-height: "1.428571";
    }

    pub c_euv_combobox_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: 150;
        min-width: "180px";
        padding: var!(space-xs);
        overflow: "hidden";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub(crate) c_euv_combobox_content_shadcn {
        border-radius: var!(radius-lg);
    }

    pub c_euv_combobox_content_open {
        display: "block";
    }

    pub c_euv_combobox_content_closed {
        display: "none";
    }

    pub c_euv_combobox_list {
        max-height: "300px";
        overflow-y: "auto";
    }

    pub c_euv_combobox_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
    }

    pub(crate) c_euv_combobox_item_shadcn {
        padding: format!("{} calc({} + {})", var!(space-xs), var!(space-xs), var!(space-2xs));
        border-radius: var!(radius-md);
        font-size: var!(font-sm);
        line-height: "1.428571";
        :focus {
            outline: "none";
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_combobox_item_selected {
        background: var!(accent);
        color: var!(accent-foreground);
    }

    pub c_euv_combobox_item_disabled {
        opacity: "0.5";
        pointer-events: "none";
    }

    pub c_euv_combobox_group {
        padding: var!(space-xs);
        overflow: "hidden";
    }

    pub c_euv_combobox_label {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(muted-foreground);
        font-size: var!(font-xs);
    }

    pub c_euv_combobox_empty {
        padding: format!("{} 0px", var!(space-2xl));
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
    }

    pub c_euv_combobox_separator {
        height: "1px";
        margin: format!("{} 0px", var!(space-xs));
        background: var!(border);
    }

    pub c_euv_combobox_clear {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(muted-foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_sheet {
        position: "relative";
    }

    pub c_euv_sheet_trigger {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        height: var!(space-2xl);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub(crate) c_euv_sheet_trigger_shadcn {
        height: var!(control-height-sm);
        gap: var!(space-xs-sm);
        border-radius: var!(radius-lg);
        font-weight: var!(font-weight-medium);
        padding-left: var!(control-pad-inline);
    }

    pub c_euv_sheet_overlay {
        position: "fixed";
        inset: "0px";
        z-index: 1000;
        background: var!(bg-overlay);
    }

    pub c_euv_sheet_overlay_open {
        display: "block";
    }

    pub c_euv_sheet_overlay_closed {
        display: "none";
    }

    pub c_euv_sheet_content {
        position: "fixed";
        z-index: 1000;
        display: "flex";
        flex-direction: "column";
        gap: var!(space-lg);
        padding: var!(space-2xl);
        background: var!(background);
        color: var!(foreground);
        box-shadow: var!(shadow-lg);
    }

    pub c_euv_sheet_content_top {
        top: "0px";
        right: "0px";
        left: "0px";
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_sheet_content_bottom {
        right: "0px";
        bottom: "0px";
        left: "0px";
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_sheet_content_left {
        top: "0px";
        bottom: "0px";
        left: "0px";
        width: "75%";
        max-width: "384px";
        border-right: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_sheet_content_right {
        top: "0px";
        right: "0px";
        bottom: "0px";
        width: "75%";
        max-width: "384px";
        border-left: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_sheet_content_open {
        display: "flex";
    }

    pub c_euv_sheet_content_closed {
        display: "none";
    }

    pub c_euv_sheet_header {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        text-align: "center";
    }

    pub c_euv_sheet_footer {
        display: "flex";
        flex-direction: "column-reverse";
        gap: var!(space-sm);
    }

    pub c_euv_sheet_title {
        color: var!(foreground);
        font-size: var!(font-lg);
        font-weight: "600";
        line-height: "1";
    }

    pub c_euv_sheet_description {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
    }

    pub c_euv_sheet_close {
        position: "absolute";
        top: var!(space-lg);
        right: var!(space-lg);
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(muted-foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_alert_dialog {
        position: "relative";
    }

    pub c_euv_alert_dialog_trigger {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        height: var!(space-2xl);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub(crate) c_euv_alert_dialog_trigger_shadcn {
        height: var!(control-height-sm);
        gap: var!(space-xs-sm);
        border-radius: var!(radius-lg);
        font-weight: var!(font-weight-medium);
        padding-left: var!(control-pad-inline);
    }

    pub c_euv_alert_dialog_overlay {
        position: "fixed";
        inset: "0px";
        z-index: 1000;
        background: var!(bg-overlay);
    }

    pub c_euv_alert_dialog_overlay_open {
        display: "block";
    }

    pub c_euv_alert_dialog_overlay_closed {
        display: "none";
    }

    pub c_euv_alert_dialog_content {
        position: "fixed";
        top: "50%";
        left: "50%";
        z-index: 1000;
        display: "grid";
        gap: var!(space-lg);
        width: "100%";
        max-width: "384px";
        padding: var!(space-lg);
        transform: "translate(-50%, -50%)";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-xl);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-lg);
        overflow: "hidden";
        @media ((max-width: 767px)) {
            max-width: "320px";
        }
    }

    pub c_euv_alert_dialog_content_open {
        display: "grid";
    }

    pub c_euv_alert_dialog_content_closed {
        display: "none";
    }

    pub c_euv_alert_dialog_header {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        text-align: "left";
        @media ((max-width: 767px)) {
            text-align: "center";
        }
    }

    pub c_euv_alert_dialog_footer {
        display: "flex";
        flex-direction: "row";
        justify-content: "flex-end";
        gap: var!(space-sm);
        margin: format!(
        "0 calc(0px - {}) calc(0px - {})",
        var!(space-lg),
        var!(space-lg)
        );
        padding: var!(space-lg);
        border-top: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: format!("0 0 {} {}", var!(radius-xl), var!(radius-xl));
        background: format!("color-mix(in srgb, {} 50%, transparent)", var!(muted));
        @media ((max-width: 767px)) {
            flex-direction: "column-reverse";
        }
    }

    pub c_euv_alert_dialog_title {
        font-size: var!(font-lg);
        font-weight: "600";
        line-height: "1";
    }

    pub c_euv_alert_dialog_media {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-3xl);
        height: var!(space-3xl);
        flex-shrink: "0";
        color: var!(foreground);
    }

    pub c_euv_alert_dialog_description {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
    }

    pub c_euv_alert_dialog_action {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        height: var!(space-2xl);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_alert_dialog_cancel {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        height: var!(space-2xl);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_hover_card {
        position: "relative";
        display: "inline-flex";
    }

    pub c_euv_hover_card_trigger {
        display: "inline-flex";
        align-items: "center";
        font-family: "inherit";
        font-size: var!(font-sm);
        text-decoration: "underline";
        text-underline-offset: "4px";
        cursor: "pointer";
    }
    pub(crate) c_euv_hover_card_trigger_shadcn {
        border-bottom-right-radius: var!(radius-md);
        border-bottom-left-radius: var!(radius-md);
        border-top-right-radius: var!(radius-md);
        border-top-left-radius: var!(radius-md);
        gap: var!(space-xs);
        justify-content: "center";
    }


    pub c_euv_hover_card_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: 150;
        width: "256px";
        padding: var!(space-lg);
        border: format!("{} {} {}", var!(border-width), "solid", var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
        outline: "none";
    }

    pub c_euv_hover_card_content_open {
        display: "block";
    }

    pub c_euv_hover_card_content_closed {
        display: "none";
    }

    pub c_euv_context_menu {
        position: "relative";
        display: "inline-flex";
    }

    pub c_euv_context_menu_trigger {
        display: "inline-flex";
        align-items: "center";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: format!("{} {} {}", var!(border-width), "solid", var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub(crate) c_euv_context_menu_trigger_shadcn {
        border-radius: var!(radius-xl);
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        justify-content: "center";
    }

    pub c_euv_context_menu_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: 150;
        min-width: "180px";
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_context_menu_content_open {
        display: "flex";
        flex-direction: "column";
    }

    pub c_euv_context_menu_content_closed {
        display: "none";
    }

    pub c_euv_context_menu_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_context_menu_item_disabled {
        opacity: "0.5";
        pointer-events: "none";
    }

    pub c_euv_context_menu_checkbox_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_context_menu_radio_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_context_menu_label {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(muted-foreground);
        font-size: var!(font-xs);
    }

    pub c_euv_context_menu_separator {
        height: "1px";
        margin: format!("{} 0px", var!(space-xs));
        background: var!(border);
    }

    pub c_euv_context_menu_group {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    pub c_euv_context_menu_radio_group {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    pub c_euv_context_menu_sub {
        position: "relative";
        display: "flex";
        flex-direction: "column";
    }

    pub c_euv_context_menu_sub_trigger {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        text-align: "left";
        cursor: "pointer";
    }

    pub c_euv_context_menu_sub_content {
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

    pub c_euv_context_menu_shortcut {
        margin-left: "auto";
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        letter-spacing: var!(letter-spacing-wide);
    }

    pub c_euv_dialog_overlay {
        position: "fixed";
        inset: "0px";
        z-index: 1000;
        background: var!(bg-overlay);
    }

    pub c_euv_dialog_overlay_open {
        display: "block";
    }

    pub c_euv_dialog_overlay_closed {
        display: "none";
    }

    pub c_euv_dialog {
        position: "fixed";
        top: "50%";
        left: "50%";
        z-index: 1000;
        display: "grid";
        gap: var!(space-lg);
        width: "100%";
        max-width: "512px";
        padding: var!(space-2xl);
        transform: "translate(-50%, -50%)";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-lg);
        background: var!(background);
        color: var!(foreground);
        box-shadow: var!(shadow-lg);
    }

    pub c_euv_dialog_open {
        display: "grid";
    }

    pub c_euv_dialog_closed {
        display: "none";
    }

    pub c_euv_dialog_header {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        text-align: "center";
    }

    pub c_euv_dialog_title {
        font-size: var!(font-lg);
        font-weight: "600";
        line-height: "1";
        letter-spacing: var!(letter-spacing-tight);
    }

    pub c_euv_dialog_description {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
    }

    pub c_euv_dialog_footer {
        display: "flex";
        flex-direction: "column-reverse";
        gap: var!(space-sm);
    }

    pub c_euv_dialog_close {
        position: "absolute";
        top: var!(space-lg);
        right: var!(space-lg);
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(muted-foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    // The panel of a dialog. It is the only thing the reader can act on
    // while the dialog is open.
    pub c_euv_dialog_content {
        position: "fixed";
        top: "50%";
        left: "50%";
        z-index: 1000;
        display: "grid";
        gap: var!(space-lg);
        width: "100%";
        max-width: "512px";
        padding: var!(space-2xl);
        transform: "translate(-50%, -50%)";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-lg);
        background: var!(background);
        color: var!(foreground);
    }

    pub c_euv_dialog_content_open {
        display: "grid";
    }

    pub c_euv_dialog_content_closed {
        display: "none";
    }


    // The two indicators sit over the list edges, leaving scrolling to the native viewport.
    pub c_euv_select_scroll_up_button {
        position: "absolute";
        top: "0px";
        left: "0px";
        right: "0px";
        display: "flex";
        cursor: "default";
        align-items: "center";
        justify-content: "center";
        padding-top: var!(space-xs);
        padding-bottom: var!(space-xs);
        background: var!(background);
        color: var!(muted-foreground);
    }

    pub c_euv_select_scroll_down_button {
        position: "absolute";
        right: "0px";
        bottom: "0px";
        left: "0px";
        display: "flex";
        cursor: "default";
        align-items: "center";
        justify-content: "center";
        padding-top: var!(space-xs);
        padding-bottom: var!(space-xs);
        background: var!(background);
        color: var!(muted-foreground);
    }

    pub c_euv_select_scroll_icon {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        font-size: var!(font-sm);
        line-height: var!(space-lg);
    }
}

#[cfg(feature = "theme-custom")]
class! {
    pub c_euv_feature_grid {}

    pub c_euv_feature_card {}

    pub c_euv_feature_header {}

    pub c_euv_feature_icon {}

    pub c_euv_feature_name {}

    pub c_euv_feature_desc {}

    pub c_modal_actions {}

    // ═══════════════════════════════════════════════════════════════════════════
    // File Upload
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_file_upload_input_hidden {}

    pub c_file_upload_options {}

    pub c_file_upload_item {}

    pub c_file_upload_item_index {}

    pub c_file_upload_item_name {}

    pub(crate) c_euv_tooltip {}

    pub(crate) c_euv_tooltip_bubble {}

    pub(crate) c_euv_tooltip_bubble_top {}

    pub(crate) c_euv_tooltip_bubble_bottom {}

    pub(crate) c_euv_tooltip_bubble_left {}

    pub(crate) c_euv_tooltip_bubble_right {}

    pub(crate) c_euv_popover {}

    pub(crate) c_euv_popover_body {}

    pub(crate) c_euv_popover_body_open {}

    pub(crate) c_euv_popover_body_closed {}

    pub(crate) c_euv_popover_header {}

    pub(crate) c_euv_popover_title {}

    pub(crate) c_euv_calendar {}

    pub(crate) c_euv_calendar_header {}

    pub(crate) c_euv_calendar_title {}

    pub(crate) c_euv_calendar_nav {}

    pub(crate) c_euv_calendar_weekdays {}

    pub(crate) c_euv_calendar_weekday {}

    pub(crate) c_euv_calendar_grid {}

    pub(crate) c_euv_calendar_grid_row {}

    pub(crate) c_euv_calendar_grid_cell {}

    pub(crate) c_euv_calendar_grid_cell_button {}

    pub(crate) c_euv_calendar_day {}

    pub(crate) c_euv_calendar_day_muted {}

    pub(crate) c_euv_calendar_day_today {}

    pub(crate) c_euv_calendar_day_selected {}

    pub(crate) c_euv_upload {}

    pub(crate) c_euv_upload_input {}

    pub(crate) c_euv_upload_drop_active {}

    pub(crate) c_euv_upload_list {}

    pub(crate) c_euv_upload_file {}

    pub(crate) c_euv_upload_file_name {}

    pub(crate) c_euv_upload_file_size {}

    pub(crate) c_euv_upload_file_status_pending {}

    pub(crate) c_euv_upload_file_status_uploading {}

    pub(crate) c_euv_upload_file_status_done {}

    pub(crate) c_euv_upload_file_status_failed {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Modal
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_modal_overlay {}

    pub(crate) c_modal_content {}

    pub(crate) c_modal_header {}

    pub(crate) c_modal_title {}

    pub(crate) c_modal_body {}

    // ═══════════════════════════════════════════════════════════════════════════
    // VConsole Panel
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_vconsole_badge {}

    pub(crate) c_vconsole_overlay {}

    pub(crate) c_vconsole_overlay_hidden {}

    pub(crate) c_vconsole_panel {}

    pub(crate) c_vconsole_panel_closed {}

    pub(crate) c_vconsole_fab {}

    pub(crate) c_vconsole_header {}

    pub(crate) c_vconsole_title {}

    pub(crate) c_vconsole_title_dot {}

    pub(crate) c_vconsole_header_actions {}

    pub(crate) c_vconsole_clear_button {}

    pub(crate) c_vconsole_close_button {}

    pub(crate) c_vconsole_body {}

    pub(crate) c_vconsole_log_item {}

    pub(crate) c_vconsole_empty {}

    pub(crate) c_vconsole_empty_hidden {}

    pub(crate) c_vconsole_log_list {}

    pub(crate) c_vconsole_log_list_hidden {}

    pub(crate) c_vconsole_count {}

    pub(crate) c_vconsole_filter_bar {}

    pub(crate) c_vconsole_filter_badge {}

    pub(crate) c_vconsole_filter_badge_outline {}

    pub(crate) c_vconsole_level_badge {}

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_dropdown
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_dropdown {}

    pub(crate) c_euv_dropdown_menu {}

    pub(crate) c_euv_dropdown_menu_open {}

    pub(crate) c_euv_dropdown_menu_closed {}

    pub(crate) c_euv_dropdown_item {}

    pub(crate) c_euv_dropdown_item_active {}

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_drawer
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_drawer_overlay {}

    pub(crate) c_euv_drawer_overlay_open {}

    pub(crate) c_euv_drawer_overlay_closed {}

    pub(crate) c_euv_drawer {}

    pub(crate) c_euv_drawer_open {}

    pub(crate) c_euv_drawer_closed {}

    pub(crate) c_euv_drawer_content {}

    pub(crate) c_euv_drawer_content_open {}

    pub(crate) c_euv_drawer_content_closed {}

    pub(crate) c_euv_drawer_handle {}

    pub(crate) c_euv_drawer_trigger {}

    pub(crate) c_euv_drawer_header {}

    pub(crate) c_euv_drawer_footer {}

    pub(crate) c_euv_drawer_title {}

    pub(crate) c_euv_drawer_description {}

    pub(crate) c_euv_drawer_close {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Message (toast)
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_message_stack {}

    pub(crate) c_euv_message {}

    pub(crate) c_euv_message_text {}

    pub(crate) c_euv_message_action {}

    pub(crate) c_euv_message_close {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Notification
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_notification_stack {}

    pub(crate) c_euv_notification {}

    pub(crate) c_euv_notification_body {}

    pub(crate) c_euv_notification_title {}

    pub(crate) c_euv_notification_text {}

    pub(crate) c_euv_notification_close {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Pop Confirm
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_popconfirm {}

    pub(crate) c_euv_popconfirm_body {}

    pub(crate) c_euv_popconfirm_body_open {}

    pub(crate) c_euv_popconfirm_body_closed {}

    pub(crate) c_euv_popconfirm_text {}

    pub(crate) c_euv_popconfirm_actions {}
    pub c_euv_command {}

    pub c_euv_command_shadcn {}

    pub c_euv_command_input_wrapper {}

    pub c_euv_command_input_wrapper_shadcn {}

    pub c_euv_command_input {}

    pub c_euv_command_list {}

    pub c_euv_command_empty {}

    pub c_euv_command_group {}

    pub c_euv_command_group_heading {}

    pub c_euv_command_item {}

    pub c_euv_command_item_shadcn {}

    pub c_euv_command_item_selected {}

    pub c_euv_command_item_disabled {}

    pub c_euv_command_separator {}

    pub c_euv_command_shortcut {}

    pub c_euv_combobox {}

    pub c_euv_combobox_trigger {}

    pub(crate) c_euv_combobox_trigger_shadcn {}

    pub c_euv_combobox_input {}

    pub(crate) c_euv_combobox_input_shadcn {}

    pub c_euv_combobox_content {}

    pub(crate) c_euv_combobox_content_shadcn {}

    pub c_euv_combobox_content_open {}

    pub c_euv_combobox_content_closed {}

    pub c_euv_combobox_list {}

    pub c_euv_combobox_item {}

    pub(crate) c_euv_combobox_item_shadcn {}

    pub c_euv_combobox_item_selected {}

    pub c_euv_combobox_item_disabled {}

    pub c_euv_combobox_group {}

    pub c_euv_combobox_label {}

    pub c_euv_combobox_empty {}

    pub c_euv_combobox_separator {}

    pub c_euv_combobox_clear {}

    pub c_euv_sheet {}

    pub c_euv_sheet_trigger {}

    pub c_euv_sheet_trigger_shadcn {}

    pub c_euv_sheet_overlay {}

    pub c_euv_sheet_overlay_open {}

    pub c_euv_sheet_overlay_closed {}

    pub c_euv_sheet_content {}

    pub c_euv_sheet_content_top {}

    pub c_euv_sheet_content_bottom {}

    pub c_euv_sheet_content_left {}

    pub c_euv_sheet_content_right {}

    pub c_euv_sheet_content_open {}

    pub c_euv_sheet_content_closed {}

    pub c_euv_sheet_header {}

    pub c_euv_sheet_footer {}

    pub c_euv_sheet_title {}

    pub c_euv_sheet_description {}

    pub c_euv_sheet_close {}

    pub c_euv_alert_dialog {}

    pub c_euv_alert_dialog_trigger {}

    pub c_euv_alert_dialog_trigger_shadcn {}

    pub c_euv_alert_dialog_overlay {}

    pub c_euv_alert_dialog_overlay_open {}

    pub c_euv_alert_dialog_overlay_closed {}

    pub c_euv_alert_dialog_content {}

    pub c_euv_alert_dialog_content_open {}

    pub c_euv_alert_dialog_content_closed {}

    pub c_euv_alert_dialog_header {}

    pub c_euv_alert_dialog_footer {}

    pub c_euv_alert_dialog_title {}

    pub c_euv_alert_dialog_media {}

    pub c_euv_alert_dialog_description {}

    pub c_euv_alert_dialog_action {}

    pub c_euv_alert_dialog_cancel {}

    pub c_euv_hover_card {}

    pub c_euv_hover_card_trigger {}

    pub c_euv_hover_card_content {}

    pub c_euv_hover_card_content_open {}

    pub c_euv_hover_card_content_closed {}

    pub c_euv_context_menu {}

    pub c_euv_context_menu_trigger {}

    pub c_euv_context_menu_trigger_shadcn {}

    pub c_euv_context_menu_content {}

    pub c_euv_context_menu_content_open {}

    pub c_euv_context_menu_content_closed {}

    pub c_euv_context_menu_item {}

    pub c_euv_context_menu_item_disabled {}

    pub c_euv_context_menu_checkbox_item {}

    pub c_euv_context_menu_radio_item {}

    pub c_euv_context_menu_label {}

    pub c_euv_context_menu_separator {}

    pub c_euv_context_menu_group {}

    pub c_euv_context_menu_radio_group {}

    pub c_euv_context_menu_sub {}

    pub c_euv_context_menu_sub_trigger {}

    pub c_euv_context_menu_sub_content {}

    pub c_euv_context_menu_shortcut {}

    pub c_euv_dialog_overlay {}

    pub c_euv_dialog_overlay_open {}

    pub c_euv_dialog_overlay_closed {}
    pub c_euv_dialog_content {}
    pub c_euv_dialog_content_open {}
    pub c_euv_dialog_content_closed {}
    pub c_euv_select_scroll_up_button {}
    pub c_euv_select_scroll_down_button {}
    pub c_euv_select_scroll_icon {}

    pub c_euv_dialog {}

    pub c_euv_dialog_open {}

    pub c_euv_dialog_closed {}

    pub c_euv_dialog_header {}

    pub c_euv_dialog_title {}

    pub c_euv_dialog_description {}

    pub c_euv_dialog_footer {}

    pub c_euv_dialog_close {}
    pub(crate) c_euv_hover_card_trigger_shadcn {}
}
