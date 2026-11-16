use super::*;

#[cfg(not(feature = "theme-custom"))]
class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // euv_sidebar — the composable parts of the reference sidebar.
    //
    // The sidebar has no theme tokens of its own, so the reference's
    // `--sidebar-*` colour roles are mapped onto the tokens every scheme
    // already publishes: the accent surface for a hovered row, the accent
    // foreground for its label, the muted surface for a skeleton and the
    // border colour for the separator and the guide of a nested menu.
    // ═══════════════════════════════════════════════════════════════════════════

    // The band that holds the title of the sidebar.
    pub c_euv_sidebar_header {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        padding: var!(space-sm);
    }

    // The scrollable middle of the sidebar: the menus live here.




    // The band that holds the footer of the sidebar.
    pub c_euv_sidebar_footer {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        padding: var!(space-sm);
    }

    // The container of one group of menu rows.
    //
    // The name carries the `_panel` suffix because `c_euv_sidebar_group`
    // already belongs to the tree renderer in `shell/fn.rs`, and two style
    // groups that export the same name make every use of that name
    // ambiguous.
    pub c_euv_sidebar_group_panel {
        position: "relative";
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        padding: var!(space-sm);
    }

    // The caption that names a group of menu rows.




    // The control at the end of a group caption.
    pub c_euv_sidebar_group_action {
        display: "flex";
        width: var!(space-lg);
        height: var!(space-lg);
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(muted-foreground);
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    // The rows of a group, below its caption.
    pub c_euv_sidebar_group_content {
        width: "100%";
    }

    // The list of menu rows.
    pub c_euv_sidebar_menu {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    // One row of the menu.
    pub c_euv_sidebar_menu_item {
        position: "relative";
    }

    // The clickable row of a menu item.




    // The row of the menu item the reader is on.




    // The control at the end of a menu row.
    pub c_euv_sidebar_menu_action {
        display: "flex";
        width: var!(space-lg);
        height: var!(space-lg);
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(muted-foreground);
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    // The count at the end of a menu row.
    pub c_euv_sidebar_menu_badge {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        height: var!(space-lg);
        min-width: var!(space-lg);
        padding: var!(space-2xs);
        border-radius: var!(radius-sm);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
        font-variant-numeric: "tabular-nums";
        color: var!(muted-foreground);
    }

    // The list of the rows nested under a menu row.
    pub c_euv_sidebar_menu_sub {
        display: "flex";
        min-width: "0px";
        flex-direction: "column";
        gap: var!(space-xs);
        // The gutter is 14px, the reference's own `mx-3.5`; it is off the
        // four-pixel scale because the nested guide has to clear the
        // parent's label by the same amount the reference clears it by.
        margin-left: "14px";
        // The list is a list only in name: the browser's own margin and
        // bullets would put a gap and a dot next to the nested rows.
        margin-top: "0px";
        margin-bottom: "0px";
        list-style: "none";
        padding-left: var!(space-md);
        border-left: format!("{} solid {}", var!(border-width), var!(border));
    }

    // One row of a nested menu.
    pub c_euv_sidebar_menu_sub_item {
        position: "relative";
    }

    // The clickable row of a nested menu item.
    pub c_euv_sidebar_menu_sub_button {
        display: "flex";
        min-width: "0px";
        align-items: "center";
        gap: var!(space-sm);
        height: var!(control-height-sm);
        padding: var!(space-sm);
        border: "none";
        border-radius: var!(radius-md);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        text-decoration: "none";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    // The row of the nested menu item the reader is on.
    pub c_euv_sidebar_menu_sub_button_active {
        c_euv_sidebar_menu_sub_button();
        background: var!(accent);
        color: var!(accent-foreground);
        font-weight: var!(font-weight-medium);
    }

    // The placeholder that stands in for a row while it loads.
    pub c_euv_sidebar_menu_skeleton {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        height: var!(control-height-sm);
        padding: var!(space-sm);
    }

    // The icon placeholder of a loading row.
    pub c_euv_sidebar_menu_skeleton_icon {
        width: var!(space-lg);
        height: var!(space-lg);
        flex-shrink: "0";
        border-radius: var!(radius-sm);
        background: var!(muted);
    }

    // The text placeholder of a loading row.
    pub c_euv_sidebar_menu_skeleton_text {
        flex: "1";
        height: var!(space-lg);
        border-radius: var!(radius-sm);
        background: var!(muted);
    }

    // The search field at the top of the sidebar.
    pub c_euv_sidebar_input {
        width: "100%";
        height: var!(control-height-sm);
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        :focus {
            border-color: var!(ring);
            outline: "none";
        }
    }

    // The main area beside the sidebar.
    pub c_euv_sidebar_inset {
        position: "relative";
        display: "flex";
        width: "100%";
        flex: "1";
        flex-direction: "column";
        background: var!(background);
    }

    pub c_euv_sidebar_provider {
        display: "flex";
        width: "100%";
        min-width: "0px";
        min-height: "100%";
    }

    // The thin edge that opens and closes the sidebar.
    pub c_euv_sidebar_rail {
        position: "absolute";
        top: "0px";
        right: "0px";
        bottom: "0px";
        width: var!(space-lg);
        padding: "0px";
        border: "none";
        background: "transparent";
        cursor: "pointer";
        :hover {
            background: var!(accent);
        }
    }

    // The rule that divides one band of the sidebar from the next.
    pub c_euv_sidebar_separator {
        width: "auto";
        height: "1px";
        margin: format!("{} {}", var!(space-sm), var!(space-md));
        border: "none";
        background: var!(border);
    }

    // The button that opens and closes the sidebar.
    pub c_euv_sidebar_trigger {
        display: "flex";
        width: var!(control-height-sm);
        height: var!(control-height-sm);
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        padding: "0px";
        border: "none";
        border-radius: var!(radius-md);
        background: "transparent";
        color: var!(foreground);
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_toast — the composable parts of the reference toast.
    // ═══════════════════════════════════════════════════════════════════════════

    // The provider owns the queue and the timing, so it adds no box of its
    // own: `display: contents` keeps it out of the layout while its
    // children stay in the flow of the page.
    pub c_euv_toast_provider {
        display: "contents";
    }

    // The fixed box the toasts stack inside.
    //
    // The viewport declares no z-index on purpose: the reference puts the
    // toast layer below the drawer and the modal, and a rule that carried
    // a z-index of its own would have to be registered in the layer
    // budget. Fixed positioning alone paints it above the page.
    pub c_euv_toast_viewport {
        position: "fixed";
        top: var!(space-xl);
        right: var!(space-xl);
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        width: "100%";
        max-width: "384px";
        outline: "none";
        @media ((max-width: 767px)) {
            right: var!(space-lg);
            left: var!(space-lg);
            max-width: "none";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_chart — the composable parts of the reference chart.
    // ═══════════════════════════════════════════════════════════════════════════

    // The box that holds the plot.
    pub c_euv_chart_container {
        display: "flex";
        width: "100%";
        aspect-ratio: "16 / 9";
        justify-content: "center";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The floating body of a tooltip.
    pub c_euv_chart_tooltip_content {
        display: "grid";
        min-width: "128px";
        align-items: "start";
        gap: var!(space-sm);
    }

    // The body of the key that names the series.
    pub c_euv_chart_legend_content {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-lg);
        padding-top: var!(space-md);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // euv_direction — the composable parts of the reference direction.
    // ═══════════════════════════════════════════════════════════════════════════

    // The provider sets the writing direction on a subtree, so it adds no
    // box of its own: `display: contents` keeps the subtree in the flow of
    // the page while the `dir` attribute reaches every child by
    // inheritance.
    pub c_euv_direction_provider {
        display: "contents";
    }
}

#[cfg(feature = "theme-custom")]
class! {
    pub c_euv_sidebar_header {}

    pub c_euv_sidebar_content {}

    pub c_euv_sidebar_footer {}

    pub c_euv_sidebar_group_panel {}

    pub c_euv_sidebar_group_label {}

    pub c_euv_sidebar_group_action {}

    pub c_euv_sidebar_group_content {}

    pub c_euv_sidebar_menu {}

    pub c_euv_sidebar_menu_item {}

    pub c_euv_sidebar_menu_button {}

    pub c_euv_sidebar_menu_button_active {}

    pub c_euv_sidebar_menu_action {}

    pub c_euv_sidebar_menu_badge {}

    pub c_euv_sidebar_menu_sub {}

    pub c_euv_sidebar_menu_sub_item {}

    pub c_euv_sidebar_menu_sub_button {}

    pub c_euv_sidebar_menu_sub_button_active {}

    pub c_euv_sidebar_menu_skeleton {}

    pub c_euv_sidebar_menu_skeleton_icon {}

    pub c_euv_sidebar_menu_skeleton_text {}

    pub c_euv_sidebar_input {}

    pub c_euv_sidebar_inset {}

    pub c_euv_sidebar_provider {}

    pub c_euv_sidebar_rail {}

    pub c_euv_sidebar_separator {}

    pub c_euv_sidebar_trigger {}

    pub c_euv_toast_provider {}

    pub c_euv_toast_viewport {}

    pub c_euv_chart_container {}

    pub c_euv_chart_tooltip_content {}

    pub c_euv_chart_legend_content {}

    pub c_euv_direction_provider {}
}

#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-custom")))]
class! {
    pub c_euv_sidebar_content {
        display: "flex";
        min-height: "0px";
        flex: "1";
        flex-direction: "column";
        gap: var!(space-sm);
        overflow: "auto";
    }

    pub c_euv_sidebar_group_label {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-sm);
        padding: var!(space-sm);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
        color: var!(muted-foreground);
    }

    pub c_euv_sidebar_menu_button {
        display: "flex";
        width: "100%";
        min-width: "0px";
        align-items: "center";
        gap: var!(space-sm);
        height: var!(control-height-sm);
        padding: var!(space-sm);
        border: "none";
        border-radius: var!(radius-md);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        text-decoration: "none";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_sidebar_menu_button_active {
        c_euv_sidebar_menu_button();
        background: var!(accent);
        color: var!(accent-foreground);
        font-weight: var!(font-weight-medium);
    }
}

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
class! {
    pub c_euv_sidebar_content {
        display: "flex";
        min-height: "0px";
        flex: "1";
        flex-direction: "column";
        gap: var!(space-sm);
        overflow: "auto";

        padding-left: var!(control-pad-inline);
    }

    pub c_euv_sidebar_group_label {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-sm);
        padding: var!(space-sm);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-medium);
        color: var!(muted-foreground);

        border-bottom-right-radius: var!(radius-md);
        border-bottom-left-radius: var!(radius-md);
        border-top-right-radius: var!(radius-md);
        border-top-left-radius: var!(radius-md);
    }

    pub c_euv_sidebar_menu_button {
        display: "flex";
        width: "100%";
        min-width: "0px";
        align-items: "center";
        gap: var!(space-sm);
        height: var!(control-height-sm);
        padding: var!(space-sm);
        border: "none";
        border-radius: var!(radius-md);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        text-decoration: "none";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_sidebar_menu_button_active {
        display: "flex";
        width: "100%";
        min-width: "0px";
        align-items: "center";
        gap: var!(space-sm);
        height: var!(control-height-sm);
        padding: var!(space-sm);
        border: "none";
        border-radius: var!(radius-md);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        text-decoration: "none";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        background: var!(accent);
        color: var!(accent-foreground);
        font-weight: var!(font-weight-medium);

        border-color: var!(border);
        border-bottom-right-radius: var!(radius-md);
        border-bottom-left-radius: var!(radius-md);
        border-top-right-radius: var!(radius-md);
        border-top-left-radius: var!(radius-md);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        font-weight: var!(font-weight-medium);
    }
}
