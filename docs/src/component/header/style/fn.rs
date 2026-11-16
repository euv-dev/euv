use super::*;

class! {
    pub(crate) c_docs_header_link {
        color: var!(foreground);
    }
    pub(crate) c_docs_header_link_active {
        c_docs_header_link();
        background: var!(accent);
        color: var!(accent-foreground);
    }
    pub(crate) c_docs_header_search_panel {
        position: "absolute";
        top: format!("calc(100% + {})", var!(space-sm));
        right: "0";
        width: "100%";
        box-sizing: "border-box";
        max-height: format!("calc({} * 8)", var!(control-height));
        overflow-y: "auto";
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }
    pub(crate) c_docs_header_search_panel_closed {
        display: "none";
    }
    pub(crate) c_docs_header_search_option {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-sm);
        border-radius: var!(radius-sm);
        cursor: "pointer";
        overflow-wrap: "anywhere";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }
    pub(crate) c_docs_header_search_option_active {
        c_docs_header_search_option();
        background: var!(accent);
        color: var!(accent-foreground);
    }
    pub(crate) c_docs_header_search_route {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        overflow-wrap: "anywhere";
    }
    pub(crate) c_docs_header_search_status {
        padding: var!(space-sm);
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }
}
