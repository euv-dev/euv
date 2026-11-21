use super::*;

class! {
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
        font-weight: var!(font-weight-bold);
        color: var!(primary);
        font-family: var!(font-family-mono);
    }

    pub c_virtual_list_card {
        flex: "1";
        padding: var!(card-padding);
        color: var!(foreground);
        display: "flex";
        flex-direction: "column";
        border-radius: var!(radius-xl);
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
        font-weight: var!(font-weight-semibold);
        color: var!(primary);
        font-family: var!(font-family-mono);
        flex-shrink: "0";
        display: "flex";
        align-items: "center";
    }

    pub c_virtual_list_row_label {
        font-size: var!(font-base);
        font-weight: var!(font-weight-medium);
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
        color: var!(primary);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
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
        border: format!("{} solid {}", var!(border-width), var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :hover {
            border-color: var!(primary);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(primary);
            background: var!(muted);
        }
        border-radius: var!(radius-md);
    }

    pub c_ws_message_time {
        color: var!(foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        flex-shrink: "0";
        margin-left: "auto";
    }

    pub(crate) c_euv_table_container {
        position: "relative";
        width: "100%";
        overflow-x: "auto";
    }

    pub(crate) c_euv_stat {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-lg);
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
        font-weight: var!(font-weight-bold);
        color: var!(primary);
        line-height: "1.1";
        font-family: var!(font-family-mono);
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
        border-radius: var!(radius-full);
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        line-height: "1";
    }

    pub(crate) c_euv_step_title {
        font-size: var!(font-base);
        font-weight: var!(font-weight-medium);
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
        border-top: format!("{} solid {}", var!(border-width), var!(border));
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
        border-radius: var!(radius-full);
        border: format!("{} solid {}", var!(border-width), var!(primary));
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
        font-weight: var!(font-weight-semibold);
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
        font-family: var!(font-family-mono);
    }

    pub(crate) c_euv_timeline_line {
        position: "absolute";
        left: "4px";
        top: "12px";
        bottom: format!("-{}", var!(space-lg));
        width: "0px";
        border: "none";
        border-left: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_virtual_list_container {
        flex: "1";
        height: "666px";
        max-height: "666px";
        overflow-y: "auto";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // List
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_list {
        display: "flex";
        flex-direction: "column";
        width: "100%";
        border: "none";
        margin: "0px";
        padding: "0px";
        list-style: "none";
    }

    pub(crate) c_euv_list_item {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-element);
        width: "100%";
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        box-sizing: "border-box";
        border-radius: var!(radius-sm);
    }

    pub(crate) c_euv_list_item_body {
        flex: "1";
        min-width: "0px";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    pub(crate) c_euv_list_item_title {
        font-size: var!(font-base);
        font-weight: var!(font-weight-medium);
        color: var!(foreground);
    }

    pub(crate) c_euv_list_item_desc {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        overflow-wrap: "anywhere";
    }

    pub(crate) c_euv_list_item_meta {
        flex-shrink: "0";
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        white-space: "nowrap";
    }

    pub(crate) c_euv_list_empty {
        padding: var!(space-xl);
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Descriptions
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_descriptions {
        display: "grid";
        grid-template-columns: "repeat(var(--descriptions-columns, 1), minmax(0, 1fr))";
        width: "100%";
        border-top: format!("{} solid {}", var!(border-width), var!(border));
        border-left: format!("{} solid {}", var!(border-width), var!(border));
        box-sizing: "border-box";
    }

    pub(crate) c_euv_descriptions_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-sm);
        border-right: format!("{} solid {}", var!(border-width), var!(border));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        box-sizing: "border-box";
        min-width: "0px";
    }

    pub(crate) c_euv_descriptions_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_descriptions_value {
        font-size: var!(font-base);
        color: var!(foreground);
        overflow-wrap: "anywhere";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Tree
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_tree {
        display: "flex";
        flex-direction: "column";
        width: "100%";
        font-size: var!(font-base);
        color: var!(foreground);
    }

    pub(crate) c_euv_tree_node {
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_tree_node_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-height: var!(control-height);
        padding: format!("{} 0px", var!(space-xs));
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        box-sizing: "border-box";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_tree_node_row_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-height: var!(control-height);
        padding: format!("{} 0px", var!(space-xs));
        background: var!(muted);
        color: var!(primary);
        font-weight: var!(font-weight-semibold);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        box-sizing: "border-box";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_tree_node_toggle {
        flex-shrink: "0";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        font-size: var!(font-sm);
        line-height: "1";
        color: var!(muted-foreground);
        text-align: "center";
    }

    pub(crate) c_euv_tree_node_toggle_open {
        flex-shrink: "0";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        font-size: var!(font-sm);
        line-height: "1";
        color: var!(muted-foreground);
        text-align: "center";
        transform: "rotate(90deg)";
    }

    pub(crate) c_euv_tree_node_toggle_leaf {
        flex-shrink: "0";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        font-size: var!(font-sm);
        line-height: "1";
        color: var!(muted-foreground);
        text-align: "center";
        opacity: "0";
    }

    pub(crate) c_euv_tree_node_label {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_euv_tree_node_children {
        display: "flex";
        flex-direction: "column";
        padding-left: var!(space-lg);
    }

    pub(crate) c_euv_tree_node_children_open {
        display: "flex";
        flex-direction: "column";
        padding-left: var!(space-lg);
    }

    pub(crate) c_euv_tree_node_children_closed {
        display: "none";
    }
    // ═══════════════════════════════════════════════════════════════════════════
    // Catalog
    //
    // The catalog surface is a grid of cards that describe a stored project:
    // its name, its size, its age and its digest. The styling lives here so
    // a consumer can render the catalog without shipping a stylesheet.
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_catalog {
        margin-block: format!("{} {}", var!(space-2xl), var!(space-3xl));
        min-width: "0px";
    }

    pub c_catalog_grid {
        display: "grid";
        grid-template-columns: "repeat(auto-fit, minmax(min(100%, 20rem), 1fr))";
        gap: var!(gap-component);
    }

    pub c_catalog_card {
        color: var!(card-foreground);
        background: var!(card);
        border: format!("{} solid {}", var!(surface-border-width), var!(border));
        border-radius: var!(radius-xl);
        box-shadow: var!(shadow-sm);
        padding: var!(space-2xl);
        box-sizing: "border-box";
        min-width: "0px";
    }

    pub c_catalog_card_title {
        margin: format!("{} 0px", var!(space-lg));
        color: var!(card-foreground);
        font-size: var!(font-lg);
        font-weight: var!(font-weight-semibold);
        letter-spacing: var!(letter-spacing-tight);
        @media ((max-width: 767px)) {
            font-size: var!(font-md);
        }
    }

    pub c_catalog_card_title_link {
        color: "inherit";
        font-weight: "inherit";
    }

    pub c_catalog_card_details {
        display: "grid";
        grid-template-columns: format!("auto minmax(0, 1fr)");
        gap: format!("{} {}", var!(space-sm), var!(space-lg));
        margin: "0px";
    }

    pub c_catalog_card_details_label {
        color: var!(muted-foreground);
    }

    pub c_catalog_card_details_value {
        margin: "0px";
        min-width: "0px";
        overflow-wrap: "anywhere";
    }

    pub c_catalog_card_digest {
        display: "block";
        overflow-wrap: "anywhere";
        white-space: "normal";
        user-select: "all";
        margin-block: var!(space-md);
    }

    pub c_catalog_card_summary {
        cursor: "pointer";
        font-family: var!(font-family-mono);
    }

    pub c_catalog_button {
        font: "inherit";
        color: var!(primary-foreground);
        background: var!(primary);
        border: format!("{} solid transparent", var!(border-width));
        border-radius: var!(radius-md);
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        cursor: "pointer";
    }

    pub c_catalog_focus_ring {
        outline: "none";
        box-shadow: format!("0 0 0 calc({} + {}) {}", var!(ring-width), var!(ring-offset), var!(ring));
        :focus-visible {
            outline: "none";
            box-shadow: format!("0 0 0 calc({} + {}) {}", var!(ring-width), var!(ring-offset), var!(ring));
        }
    }
}

#[cfg(not(feature = "theme-shadcn"))]
class! {
    pub(crate) c_euv_table {
        width: "100%";
        border-collapse: "collapse";
        table-layout: "auto";
        font-size: var!(font-base);
        color: var!(foreground);
        border-radius: var!(radius-sm);
    }

    pub(crate) c_euv_table_head {
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        letter-spacing: var!(letter-spacing-wide);
        text-transform: var!(text-transform-uppercase);
    }

    pub(crate) c_euv_table_body {
        font-size: var!(font-base);
    }

    pub(crate) c_euv_table_row {
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_table_row_zebra {
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(muted);
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
        font-weight: var!(font-weight-semibold);
        letter-spacing: var!(letter-spacing-wide);
        text-transform: var!(text-transform-uppercase);
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_table_caption {
        caption-side: "bottom";
        padding: var!(space-sm);
        color: var!(muted-foreground);
        text-align: "left";
    }

    pub(crate) c_euv_table_footer {
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
    }

    pub(crate) c_euv_skeleton {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        width: "100%";
    }

    pub(crate) c_euv_skeleton_line {
        display: "block";
        width: "100%";
        height: "12px";
        border-radius: var!(radius-sm);
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(muted);
        animation: format!("euv-skeleton-pulse 1.5s {} infinite", var!(ease-in-out));
    }

    pub(crate) c_euv_table_header_cell_center {}

    pub(crate) c_euv_table_header_cell_right {}
}

#[cfg(feature = "theme-shadcn")]
class! {
    pub(crate) c_euv_table {
        width: "100%";
        border-collapse: "collapse";
        table-layout: "auto";
        font-size: var!(font-base);
        color: var!(foreground);
        border-radius: var!(radius-sm);

        font-size: var!(font-sm);
    }

    pub(crate) c_euv_table_head {
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        letter-spacing: var!(letter-spacing-wide);
        text-transform: var!(text-transform-uppercase);

        height: "40px";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        letter-spacing: "normal";
        text-transform: "none";
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub(crate) c_euv_table_body {
        font-size: var!(font-base);

        font-size: var!(font-sm);
    }

    pub(crate) c_euv_table_row {
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));

        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        transition: format!("background-color {} {}", var!(duration-fast), var!(ease-out));
        :hover {
            background: format!("color-mix(in srgb, {} 50%, transparent)", var!(muted));
        }
        :last-child {
            border-bottom: "none";
        }
    }

    pub(crate) c_euv_table_row_zebra {
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(muted);

        background: format!("color-mix(in srgb, {} 50%, transparent)", var!(muted));
    }

    pub(crate) c_euv_table_cell {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";

        padding: var!(space-sm);
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub(crate) c_euv_table_cell_center {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "center";
        vertical-align: "middle";
        white-space: "nowrap";

        padding: var!(space-sm);
        text-align: "center";
    }

    pub(crate) c_euv_table_cell_right {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "right";
        vertical-align: "middle";
        white-space: "nowrap";

        padding: var!(space-sm);
        text-align: "right";
    }

    pub(crate) c_euv_table_header_cell {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";
        font-size: var!(font-xs);
        font-weight: var!(font-weight-semibold);
        letter-spacing: var!(letter-spacing-wide);
        text-transform: var!(text-transform-uppercase);
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));

        height: "40px";
        padding: format!("0px {}", var!(space-sm));
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        letter-spacing: "normal";
        text-transform: "none";
        text-align: "left";
        vertical-align: "middle";
        white-space: "nowrap";
    }

    pub(crate) c_euv_table_caption {
        caption-side: "bottom";
        padding: var!(space-sm);
        color: var!(muted-foreground);
        text-align: "left";

        margin-top: var!(space-lg);
        padding: "0px";
        font-size: var!(font-sm);
    }

    pub(crate) c_euv_table_footer {
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);

        background: format!("color-mix(in srgb, {} 50%, transparent)", var!(muted));
        border-top: format!("{} solid {}", var!(border-width), var!(border));
        font-weight: var!(font-weight-medium);
    }

    pub(crate) c_euv_skeleton {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        width: "100%";

        border: "none";
        border-radius: var!(radius-md);
    }

    pub(crate) c_euv_skeleton_line {
        display: "block";
        width: "100%";
        height: "12px";
        border-radius: var!(radius-sm);
        border: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(muted);
        animation: format!("euv-skeleton-pulse 1.5s {} infinite", var!(ease-in-out));

        border: "none";
        border-radius: var!(radius-md);
    }

    pub(crate) c_euv_table_header_cell_center {
        text-align: "center";
    }

    pub(crate) c_euv_table_header_cell_right {
        text-align: "right";
    }
}
