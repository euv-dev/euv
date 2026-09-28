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
}
