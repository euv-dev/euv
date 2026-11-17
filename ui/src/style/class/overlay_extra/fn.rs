use super::*;

#[cfg(not(feature = "theme-custom"))]
class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Combobox — chip family
    // ═══════════════════════════════════════════════════════════════════════════





    pub c_euv_combobox_chip_list {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-xs);
    }

    pub c_euv_combobox_chip_disabled {
        opacity: "0.5";
        pointer-events: "none";
    }

    pub c_euv_combobox_chips_input {
        flex: "1 1 64px";
        min-width: "64px";
        padding: "0px";
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        outline: "none";
    }

    pub c_euv_combobox_value {
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Menubar — groups, labels, indicators and the nested menu
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_menubar_group {
        display: "flex";
        flex-direction: "column";
    }

    pub c_euv_menubar_shortcut {
        margin-left: "auto";
        color: var!(muted-foreground);
        font-family: "inherit";
        font-size: var!(font-xs);
        letter-spacing: var!(letter-spacing-wide);
    }

    pub c_euv_menubar_checkbox_item_indicator {
        position: "absolute";
        left: var!(space-sm);
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: "14px";
        height: "14px";
        color: var!(foreground);
        font-size: var!(font-sm);
        line-height: "1";
    }

    pub c_euv_menubar_radio_item_indicator {
        position: "absolute";
        left: var!(space-sm);
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: "14px";
        height: "14px";
    }

    pub c_euv_menubar_radio_item_indicator_dot {
        width: var!(space-xs);
        height: var!(space-xs);
        border-radius: var!(radius-full);
        background: "currentColor";
    }

    pub c_euv_menubar_radio_group {
        display: "flex";
        flex-direction: "column";
    }

    pub c_euv_menubar_sub {
        position: "relative";
        display: "flex";
        flex-direction: "column";
    }

    pub c_euv_menubar_sub_trigger_inset {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {} {} {}", var!(space-xs), var!(space-sm), var!(space-xs), var!(space-3xl));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub(crate) c_euv_menubar_sub_trigger_open {
        background: var!(accent);
        color: var!(accent-foreground);
    }

    pub c_euv_menubar_sub_trigger_indicator {
        margin-left: "auto";
        color: var!(muted-foreground);
        font-size: var!(font-base);
        line-height: "1";
    }

    pub c_euv_menubar_sub_content_open {
        display: "flex";
    }

    pub c_euv_menubar_sub_content_closed {
        display: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Carousel — viewport, slide and the two arrow buttons
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_carousel_content {
        overflow: "hidden";
    }

    pub c_euv_carousel_content_track {
        display: "flex";
        flex-direction: "row";
        margin-left: format!("calc(-1 * {})", var!(space-lg));
    }

    pub c_euv_carousel_content_track_vertical {
        flex-direction: "column";
        margin-top: format!("calc(-1 * {})", var!(space-lg));
        margin-left: "0px";
    }

    pub c_euv_carousel_item {
        min-width: "0px";
        flex-grow: "0";
        flex-shrink: "0";
        flex-basis: "100%";
        padding-left: var!(space-lg);
    }

    pub c_euv_carousel_item_vertical {
        padding-top: var!(space-lg);
        padding-left: "0px";
    }

















    // ═══════════════════════════════════════════════════════════════════════════
    // Calendar — the clickable day cell
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_calendar_day_button {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-2xs);
        width: "100%";
        min-width: var!(space-2xl);
        min-height: var!(space-2xl);
        aspect-ratio: "1";
        padding: "0px";
        border: format!("{} solid transparent", var!(border-width));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        font-weight: var!(font-weight-normal);
        line-height: "1";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
        }
    }

    pub c_euv_calendar_day_button_selected {
        background: var!(primary);
        color: var!(primary-foreground);
        font-weight: var!(font-weight-semibold);
    }

    pub c_euv_calendar_day_button_disabled {
        color: var!(muted-foreground);
        opacity: "0.5";
        pointer-events: "none";
    }
}

#[cfg(feature = "theme-custom")]
class! {
    pub c_euv_combobox_chips {}

    pub c_euv_combobox_chip_list {}

    pub c_euv_combobox_chip {}

    pub c_euv_combobox_chip_disabled {}

    pub c_euv_combobox_chip_remove {}

    pub c_euv_combobox_chips_input {}

    pub c_euv_combobox_value {}

    pub c_euv_menubar_group {}

    pub c_euv_menubar_separator {}

    pub c_euv_menubar_label {}

    pub c_euv_menubar_label_inset {}

    pub c_euv_menubar_shortcut {}

    pub c_euv_menubar_checkbox_item {}

    pub c_euv_menubar_checkbox_item_indicator {}

    pub c_euv_menubar_radio_item {}

    pub c_euv_menubar_radio_item_indicator {}

    pub c_euv_menubar_radio_group {}

    pub c_euv_menubar_sub {}

    pub c_euv_menubar_sub_trigger {}

    pub c_euv_menubar_sub_trigger_inset {}

    pub c_euv_menubar_sub_content {}

    pub c_euv_menubar_sub_content_open {}

    pub c_euv_menubar_sub_content_closed {}

    pub(crate) c_euv_menubar_radio_item_indicator_dot {}

    pub(crate) c_euv_menubar_sub_trigger_open {}

    pub(crate) c_euv_menubar_sub_trigger_indicator {}

    pub c_euv_carousel_content {}

    pub c_euv_carousel_content_track {}

    pub c_euv_carousel_content_track_vertical {}

    pub c_euv_carousel_item {}

    pub c_euv_carousel_item_vertical {}

    pub c_euv_carousel_next {}

    pub c_euv_carousel_next_vertical {}

    pub c_euv_carousel_previous {}

    pub c_euv_carousel_previous_vertical {}

    pub c_euv_calendar_day_button {}

    pub c_euv_calendar_day_button_selected {}

    pub c_euv_calendar_day_button_disabled {}
}

#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-custom")))]
class! {
    pub c_euv_combobox_chips {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-xs);
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
    }

    pub c_euv_combobox_chip {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-2xs);
        max-width: "100%";
        padding: format!("{} {}", var!(space-2xs), var!(space-xs));
        border-radius: var!(radius-sm);
        background: var!(secondary);
        color: var!(secondary-foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
    }

    pub c_euv_combobox_chip_remove {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-md);
        height: var!(space-md);
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

    pub c_euv_menubar_separator {
        height: var!(border-width);
        margin: format!("{} 0px", var!(space-xs));
        background: var!(border);
    }

    pub c_euv_menubar_label {
        padding: format!("calc({} + {}) {}", var!(space-xs), var!(space-2xs), var!(space-sm));
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1.5";
    }

    pub c_euv_menubar_label_inset {
        padding: format!("calc({} + {}) {} calc({} + {})", var!(space-xs), var!(space-2xs), var!(space-sm), var!(space-2xl), var!(space-sm));
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1.5";
    }

    pub c_euv_menubar_checkbox_item {
        position: "relative";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        // The reference pads the row with 6px (1.5 times the 4px base);
        // it is off the 4px scale, so it is spelled out here.
        padding: format!("{} {} {} {}", var!(space-xs-sm), var!(space-sm), var!(space-xs-sm), var!(space-3xl));
        border-radius: var!(radius-sm);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "default";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_menubar_radio_item {
        position: "relative";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        // The reference pads the row with 6px (1.5 times the 4px base);
        // it is off the 4px scale, so it is spelled out here.
        padding: format!("{} {} {} {}", var!(space-xs-sm), var!(space-sm), var!(space-xs-sm), var!(space-3xl));
        border-radius: var!(radius-sm);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "default";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_menubar_sub_trigger {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_menubar_sub_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        display: "flex";
        flex-direction: "column";
        min-width: "128px";
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_carousel_next {
        position: "absolute";
        right: "0px";
        top: "50%";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        height: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        padding: "0px";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-full);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-base);
        line-height: "1";
        white-space: "nowrap";
        transform: "translateY(-50%)";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            cursor: "not-allowed";
            opacity: "0.5";
        }
    }

    pub c_euv_carousel_next_vertical {
        right: "auto";
        top: "auto";
        // The reference parks the button 48px (12 times the 4px base)
        // outside the viewport; the offset is off the 4px scale.
        bottom: "-48px";
        left: "50%";
        transform: "translateX(-50%) rotate(90deg)";
    }

    pub c_euv_carousel_previous {
        position: "absolute";
        left: "0px";
        top: "50%";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        height: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        padding: "0px";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-full);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-base);
        line-height: "1";
        white-space: "nowrap";
        transform: "translateY(-50%)";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            cursor: "not-allowed";
            opacity: "0.5";
        }
    }

    pub c_euv_carousel_previous_vertical {
        left: "50%";
        // The reference parks the button 48px (12 times the 4px base)
        // outside the viewport; the offset is off the 4px scale.
        top: "-48px";
        transform: "translateX(-50%) rotate(90deg)";
    }
}

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
class! {
    pub c_euv_combobox_chips {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-xs);
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);

        border-radius: var!(radius-lg);
        background-color: "transparent";
        padding-left: var!(control-pad-inline);
    }

    pub c_euv_combobox_chip {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-2xs);
        max-width: "100%";
        padding: format!("{} {}", var!(space-2xs), var!(space-xs));
        border-radius: var!(radius-sm);
        background: var!(secondary);
        color: var!(secondary-foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";

        gap: var!(space-xs);
        font-size: var!(font-xs);
        color: var!(foreground);
        font-weight: var!(font-weight-medium);
        justify-content: "center";
        padding-left: var!(space-xs-sm);
    }

    pub c_euv_combobox_chip_remove {
        white-space: "nowrap";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-md);
        height: var!(space-md);
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
        opacity: "0.5";
    }

    pub c_euv_menubar_separator {
        height: var!(border-width);
        margin: format!("{} 0px", var!(space-xs));
        background: var!(border);

        margin-right: format!("calc(-1 * {})", var!(space-xs));
        margin-left: format!("calc(-1 * {})", var!(space-xs));
    }

    pub c_euv_menubar_label {
        padding: format!("calc({} + {}) {}", var!(space-xs), var!(space-2xs), var!(space-sm));
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1.5";

        font-weight: var!(font-weight-medium);
    }

    pub c_euv_menubar_label_inset {
        padding: format!("calc({} + {}) {} calc({} + {})", var!(space-xs), var!(space-2xs), var!(space-sm), var!(space-2xl), var!(space-sm));
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1.5";

        font-weight: var!(font-weight-medium);
    }

    pub c_euv_menubar_checkbox_item {
        position: "relative";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        // The reference pads the row with 6px (1.5 times the 4px base);
        // it is off the 4px scale, so it is spelled out here.
        padding: format!("{} {} {} {}", var!(space-xs-sm), var!(space-sm), var!(space-xs-sm), var!(space-3xl));
        border-radius: var!(radius-sm);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "default";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            pointer-events: "none";
            opacity: "0.5";
        }
    }

    pub c_euv_menubar_radio_item {
        position: "relative";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        // The reference pads the row with 6px (1.5 times the 4px base);
        // it is off the 4px scale, so it is spelled out here.
        padding: format!("{} {} {} {}", var!(space-xs-sm), var!(space-sm), var!(space-xs-sm), var!(space-3xl));
        border-radius: var!(radius-sm);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "default";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            pointer-events: "none";
            opacity: "0.5";
        }
    }

    pub c_euv_menubar_sub_trigger {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        cursor: "pointer";
        user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }

        border-radius: var!(radius-sm);
        :focus-visible {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_menubar_sub_content {
        position: "absolute";
        top: "100%";
        left: "0px";
        display: "flex";
        flex-direction: "column";
        min-width: "128px";
        padding: var!(space-xs);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);

        top: "0px";
        left: "100%";
        z-index: "101";
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_carousel_next {
        position: "absolute";
        right: "0px";
        top: "50%";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        height: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        padding: "0px";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-full);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-base);
        line-height: "1";
        white-space: "nowrap";
        transform: "translateY(-50%)";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            cursor: "not-allowed";
            opacity: "0.5";
        }

        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_carousel_next_vertical {
        right: "auto";
        top: "auto";
        // The reference parks the button 48px (12 times the 4px base)
        // outside the viewport; the offset is off the 4px scale.
        bottom: "-48px";
        left: "50%";
        transform: "translateX(-50%) rotate(90deg)";

        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_carousel_previous {
        position: "absolute";
        left: "0px";
        top: "50%";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        height: format!("calc({} - {})", var!(space-3xl), var!(space-xs));
        padding: "0px";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-full);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-base);
        line-height: "1";
        white-space: "nowrap";
        transform: "translateY(-50%)";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
        :disabled {
            cursor: "not-allowed";
            opacity: "0.5";
        }

        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_carousel_previous_vertical {
        left: "50%";
        // The reference parks the button 48px (12 times the 4px base)
        // outside the viewport; the offset is off the 4px scale.
        top: "-48px";
        transform: "translateX(-50%) rotate(90deg)";

        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
        font-weight: var!(font-weight-medium);
    }
}
