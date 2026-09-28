use super::*;

class! {
    pub c_euv_tabs {
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub c_euv_tabs_bar {
        display: "flex";
        border-bottom: format!("1px dashed {}", var!(border));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
        @media ((max-width: 767px)) {
            flex-wrap: "wrap";
        }
    }

    pub c_euv_tab_item {
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

    pub c_euv_tab_item_active {
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

    pub c_euv_tabs_panel {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub c_euv_collapse_item {
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub c_euv_collapse_header {
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

    pub c_euv_collapse_header_active {
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

    pub c_euv_collapse_body {
        overflow: "hidden";
        font-size: var!(font-base);
        color: var!(foreground);
        transition: format!("max-height {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub c_euv_collapse_body_open {
        overflow: "hidden";
        max-height: "none";
        opacity: "1";
        padding-bottom: var!(space-md);
        font-size: var!(font-base);
        color: var!(foreground);
        transition: format!("max-height {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub c_euv_collapse_body_closed {
        overflow: "hidden";
        max-height: "0px";
        opacity: "0";
        font-size: var!(font-base);
        color: var!(foreground);
        transition: format!("max-height {} {}, opacity {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
    }

    pub c_euv_breadcrumb {
        display: "flex";
        align-items: "center";
        flex-wrap: "wrap";
        gap: var!(space-xs);
        font-size: var!(font-sm);
    }

    pub c_euv_breadcrumb_item {
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

    pub c_euv_breadcrumb_sep {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub c_euv_breadcrumb_current {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        cursor: "default";
    }

    pub c_euv_divider {
        display: "block";
        width: "100%";
    }

    pub c_euv_divider_horizontal {
        display: "block";
        width: "100%";
        height: "0px";
        border: "none";
        border-top: format!("1px solid {}", var!(border));
        margin: format!("{} 0px", var!(space-lg));
    }

    pub c_euv_divider_vertical {
        display: "block";
        width: "0px";
        height: "auto";
        align-self: "center";
        border: "none";
        border-left: format!("1px solid {}", var!(border));
        margin: format!("0px {}", var!(space-sm));
    }

    pub c_euv_divider_labeled {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        margin: format!("{} 0px", var!(space-lg));
    }

    pub c_euv_divider_label_text {
        font-size: var!(font-xs);
        font-weight: "500";
        color: var!(muted-foreground);
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
    }
}
