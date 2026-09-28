use super::*;

class! {
    pub c_euv_table {
        width: "100%";
        border-collapse: "collapse";
        table-layout: "auto";
        font-size: var!(font-base);
        color: var!(foreground);
        overflow-x: "auto";
        display: "block";
        @media ((max-width: 767px)) {
            overflow-x: "auto";
        }
    }

    pub c_euv_table_head {
        font-size: var!(font-xs);
        font-weight: "600";
        letter-spacing: "0.04em";
        text-transform: "uppercase";
    }

    pub c_euv_table_body {
        font-size: var!(font-base);
    }

    pub c_euv_table_row {
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub c_euv_table_row_zebra {
        border-bottom: format!("1px dashed {}", var!(border));
        background: var!(accent-muted);
    }

    pub c_euv_table_cell {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub c_euv_table_cell_center {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "center";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub c_euv_table_cell_right {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "right";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub c_euv_table_header_cell {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";
        font-size: var!(font-xs);
        font-weight: "600";
        letter-spacing: "0.04em";
        text-transform: "uppercase";
        border-bottom: format!("1px solid {}", var!(border));
    }

    pub c_euv_table_caption {
        caption-side: "bottom";
        padding-top: var!(space-sm);
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        text-align: "left";
    }

    pub c_euv_stat {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-lg);
        border: format!("1px dashed {}", var!(border));
    }

    pub c_euv_stat_icon {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-2xl);
        line-height: "1";
        color: var!(foreground);
        flex-shrink: "0";
    }

    pub c_euv_stat_value {
        font-size: var!(font-2xl);
        font-weight: "700";
        color: var!(accent);
        line-height: "1.1";
        font-family: "ui-monospace, monospace";
    }

    pub c_euv_stat_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub c_euv_stat_hint {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    pub c_euv_steps {
        display: "flex";
        flex-direction: "row";
        align-items: "flex-start";
        gap: var!(space-lg);
        width: "100%";
        @media ((max-width: 767px)) {
            flex-direction: "column";
            gap: var!(space-md);
        }
    }

    pub c_euv_step {
        display: "flex";
        flex-direction: "row";
        align-items: "flex-start";
        gap: var!(space-sm);
        flex: "1";
        min-width: "0";
        font-size: var!(font-base);
        color: var!(muted-foreground);
        @media ((max-width: 767px)) {
            width: "100%";
        }
    }

    pub c_euv_step_active {
        display: "flex";
        flex-direction: "row";
        align-items: "flex-start";
        gap: var!(space-sm);
        flex: "1";
        min-width: "0";
        font-size: var!(font-base);
        color: var!(foreground);
        @media ((max-width: 767px)) {
            width: "100%";
        }
    }

    pub c_euv_step_done {
        display: "flex";
        flex-direction: "row";
        align-items: "flex-start";
        gap: var!(space-sm);
        flex: "1";
        min-width: "0";
        font-size: var!(font-base);
        color: var!(foreground);
        @media ((max-width: 767px)) {
            width: "100%";
        }
    }

    pub c_euv_step_marker {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "24px";
        height: "24px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border-radius: "50%";
        border: format!("1px dashed {}", var!(border));
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: "600";
        line-height: "1";
    }

    pub c_euv_step_title {
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
        margin: "0px";
    }

    pub c_euv_step_desc {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        margin: "0px";
    }

    pub c_euv_step_line {
        flex: "1";
        height: "0px";
        align-self: "center";
        border: "none";
        border-top: format!("1px dashed {}", var!(border));
        min-width: var!(space-lg);
        @media ((max-width: 767px)) {
            display: "none";
        }
    }

    pub c_euv_timeline {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-lg);
        width: "100%";
    }

    pub c_euv_timeline_item {
        position: "relative";
        display: "flex";
        flex-direction: "row";
        align-items: "flex-start";
        gap: var!(space-md);
        padding-left: var!(space-lg);
    }

    pub c_euv_timeline_marker {
        position: "absolute";
        left: "0px";
        top: "0px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "10px";
        height: "10px";
        box-sizing: "border-box";
        border-radius: "50%";
        border: format!("1px solid {}", var!(accent));
        background: var!(background);
    }

    pub c_euv_timeline_content {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        flex: "1";
        min-width: "0";
    }

    pub c_euv_timeline_title {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
    }

    pub c_euv_timeline_desc {
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: "0px";
    }

    pub c_euv_timeline_time {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        font-family: "ui-monospace, monospace";
    }

    pub c_euv_timeline_line {
        position: "absolute";
        left: "4px";
        top: "12px";
        bottom: format!("-{}", var!(space-lg));
        width: "0px";
        border: "none";
        border-left: format!("1px solid {}", var!(border));
    }

    pub c_euv_skeleton {
        display: "block";
        width: "100%";
        height: "16px";
        border-radius: "0px";
        border: format!("1px dashed {}", var!(border));
        background: var!(accent-muted);
        animation: format!("euv-pulse 1.5s {} infinite", var!(ease-in-out));
    }

    pub c_euv_skeleton_line {
        display: "block";
        width: "100%";
        height: "12px";
        border-radius: "0px";
        border: format!("1px dashed {}", var!(border));
        background: var!(accent-muted);
        animation: format!("euv-pulse 1.5s {} infinite", var!(ease-in-out));
    }
}
