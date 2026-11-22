use super::*;

class! {
    pub(crate) c_docs_header_link {
        color: var!(foreground);
        font-weight: var!(font-weight-medium);
        // Hover paints the same lock-in bar the sidebar tree paints on the
        // row under the pointer, and leaves the row's own background
        // transparent.
        //
        // The bar is an inset shadow rather than a border: a border takes
        // part in the box's layout, so the text would move when the state
        // changes, and an inset shadow is painted inside the existing
        // padding box and moves nothing.
        //
        // The bar is the foreground colour, so it follows the link's own
        // text colour in every scheme.
        :hover {
            background: "transparent";
            color: var!(foreground);
            box-shadow: format!("inset 3px 0 0 0 {}", var!(foreground));
        }
    }
    pub(crate) c_docs_header_link_active {
        c_docs_header_link();
        // The active link changes its fill, the way the sidebar's active
        // project row does. The fill is the primary colour rather than the
        // accent: inside the app root the accent is the header's own
        // background, so an accent fill would leave the active link
        // invisible.
        //
        // The active link drops the bar on hover, because the accent fill
        // already carries the weight.
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
            box-shadow: "none";
        }
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
        // Hover paints a left bar and leaves the row's own background
        // transparent, the same way the sidebar tree marks the row
        // under the pointer.
        //
        // The bar is an inset shadow rather than a border, because a
        // border takes part in the box's layout and would move the
        // text when the state changes; an inset shadow is painted
        // inside the existing padding box and moves nothing.
        //
        // The bar is the foreground colour, so it follows the row's
        // text colour in every scheme.
        :hover {
            background: "transparent";
            color: var!(foreground);
            box-shadow: format!("inset 5px 0 0 0 {}", var!(foreground));
        }
    }
    pub(crate) c_docs_header_search_option_active {
        c_docs_header_search_option();
        // The active row changes its fill, the same way the sidebar's
        // active row does. The fill is the primary colour rather than
        // the accent: inside the app root the accent is the panel's
        // own background, so an accent fill would leave the active row
        // invisible.
        background: var!(primary);
        color: var!(primary-foreground);
        box-shadow: format!("inset 5px 0 0 0 {}", var!(primary));
        // The route keeps its muted colour at rest, but on the active row it
        // must follow the row's own text colour: the muted foreground equals
        // the primary fill in the monochrome scheme in both themes, so the
        // route would otherwise be painted in the fill and disappear.
        > .c_docs_header_search_route {
            color: var!(primary-foreground);
        }
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
            box-shadow: format!("inset 5px 0 0 0 {}", var!(primary));
        }
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
