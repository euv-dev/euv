use super::*;

class! {
    pub(crate) c_euv_table {
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

    pub(crate) c_euv_table_head {
        font-size: var!(font-xs);
        font-weight: "600";
        letter-spacing: "0.04em";
        text-transform: "uppercase";
    }

    pub(crate) c_euv_table_body {
        font-size: var!(font-base);
    }

    pub(crate) c_euv_table_row {
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub(crate) c_euv_table_row_zebra {
        border-bottom: format!("1px dashed {}", var!(border));
        background: var!(accent-muted);
    }

    pub(crate) c_euv_table_cell {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub(crate) c_euv_table_cell_center {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "center";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub(crate) c_euv_table_cell_right {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "right";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub(crate) c_euv_table_header_cell {
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

    pub(crate) c_euv_table_caption {
        caption-side: "bottom";
        padding-top: var!(space-sm);
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        text-align: "left";
    }

    pub(crate) c_euv_stat {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-lg);
        border: format!("1px dashed {}", var!(border));
    }

    pub(crate) c_euv_stat_icon {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        font-size: var!(font-2xl);
        line-height: "1";
        color: var!(foreground);
        flex-shrink: "0";
    }

    pub(crate) c_euv_stat_value {
        font-size: var!(font-2xl);
        font-weight: "700";
        color: var!(accent);
        line-height: "1.1";
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_euv_stat_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_stat_hint {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_steps {
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

    pub(crate) c_euv_step {
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

    pub(crate) c_euv_step_active {
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

    pub(crate) c_euv_step_done {
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

    pub(crate) c_euv_step_marker {
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

    pub(crate) c_euv_step_title {
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
        margin: "0px";
    }

    pub(crate) c_euv_step_desc {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        margin: "0px";
    }

    pub(crate) c_euv_step_line {
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

    pub(crate) c_euv_timeline {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-lg);
        width: "100%";
    }

    pub(crate) c_euv_timeline_item {
        position: "relative";
        display: "flex";
        flex-direction: "row";
        align-items: "flex-start";
        gap: var!(space-md);
        padding-left: var!(space-lg);
    }

    pub(crate) c_euv_timeline_marker {
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

    pub(crate) c_euv_timeline_content {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        flex: "1";
        min-width: "0";
    }

    pub(crate) c_euv_timeline_title {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
    }

    pub(crate) c_euv_timeline_desc {
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: "0px";
    }

    pub(crate) c_euv_timeline_time {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_euv_timeline_line {
        position: "absolute";
        left: "4px";
        top: "12px";
        bottom: format!("-{}", var!(space-lg));
        width: "0px";
        border: "none";
        border-left: format!("1px solid {}", var!(border));
    }

    pub(crate) c_euv_skeleton {
        display: "block";
        width: "100%";
        height: "16px";
        border-radius: "0px";
        border: format!("1px dashed {}", var!(border));
        background: var!(accent-muted);
        animation: format!("euv-pulse 1.5s {} infinite", var!(ease-in-out));
    }

    pub(crate) c_euv_skeleton_line {
        display: "block";
        width: "100%";
        height: "12px";
        border-radius: "0px";
        border: format!("1px dashed {}", var!(border));
        background: var!(accent-muted);
        animation: format!("euv-pulse 1.5s {} infinite", var!(ease-in-out));
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

    pub(crate) c_virtual_list_container {
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
}
