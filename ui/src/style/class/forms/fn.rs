use super::*;

#[cfg(not(feature = "theme-custom"))]
class! {
    pub c_form_switch_off {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Form Elements
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_input_wrapper {
        width: "100%";
        margin: format!("{} 0px", var!(gap-element));
    }

    pub c_form_label {
        display: "block";
        margin-bottom: var!(space-sm);
        color: "inherit";
        font-weight: var!(font-weight-medium);
        font-size: var!(font-base);
    }

    pub c_inline_input_row {
        display: "flex";
        align-items: "center";
        gap: var!(gap-component);
    }

    pub c_euv_input {
        width: "100%";
        min-height: var!(control-height);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        // Hover and focus only swap the border and the fill, so they ease
        // between the two states instead of snapping.
        transition: format!(
        "border-color {} {}, background-color {} {}",
        var!(duration-fast),
        var!(ease-out),
        var!(duration-fast),
        var!(ease-out)
        );
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
            background: var!(muted);
        }
        border-radius: var!(radius-md);
    }

    pub c_euv_input_no_transition {
        width: "100%";
        min-height: var!(control-height);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        border-radius: var!(radius-md);
    }

    pub c_euv_input_error {
        width: "100%";
        min-height: var!(control-height);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(destructive));
        background: var!(error-container);
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(destructive);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
        border-radius: var!(radius-md);
    }

    pub c_form_checkbox {
        cursor: "pointer";
        width: var!(space-lg);
        height: var!(space-lg);
        border-radius: var!(radius-xs);
    }

    pub c_form_checkbox_label {
        font-size: var!(font-base);
        color: "inherit";
        cursor: "pointer";
    }

    pub c_form_checkbox_row {
        margin: format!("{} 0px", var!(gap-component));
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
    }

    pub c_select_input {
        width: "100%";
        min-height: var!(control-height);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        cursor: "pointer";
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        color: var!(foreground);
        vertical-align: "middle";
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='currentColor' d='M6 8L1 3h10z'/%3E%3C/svg%3E\")";
        background-repeat: "no-repeat";
        background-position: "right 14px center";
        :focus {
            outline: "none";
            border-color: var!(ring);
            background: var!(muted);
        }
        border-radius: var!(radius-md);
    }

    pub c_textarea_input {
        width: "100%";
        max-width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        resize: "vertical";
        overflow-x: "hidden";
        word-wrap: "break-word";
        font-family: "inherit";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        // Hover and focus only swap the border and the fill, so they ease
        // between the two states instead of snapping.
        transition: format!(
        "border-color {} {}, background-color {} {}",
        var!(duration-fast),
        var!(ease-out),
        var!(duration-fast),
        var!(ease-out)
        );
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
            background: var!(muted);
        }
        border-radius: var!(radius-md);
    }

    pub c_textarea_input_error {
        width: "100%";
        max-width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(destructive));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        resize: "vertical";
        overflow-x: "hidden";
        word-wrap: "break-word";
        font-family: "inherit";
        color: var!(destructive);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
        border-radius: var!(radius-md);
    }

    pub c_textarea_counter {
        text-align: "right";
        margin-top: var!(space-xs);
        margin-bottom: var!(gap-component);
    }

    pub c_textarea_counter_text {
        font-size: var!(font-sm);
        color: "inherit";
        opacity: "1";
    }

    pub c_field_error_text {
        color: var!(destructive);
        font-size: var!(font-base);
        margin-top: var!(space-xs);
        margin-bottom: var!(space-sm);
    }

    pub(crate) c_form_switch {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_form_switch_row {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-component);
        min-height: var!(control-height);
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_form_switch_on {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_form_switch_track {
        position: "relative";
        width: "44px";
        height: "24px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-full);
        background: var!(background);
        transition: format!("{} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_form_switch_thumb {
        position: "absolute";
        top: "3px";
        left: "3px";
        width: "16px";
        height: "16px";
        box-sizing: "border-box";
        border-radius: var!(radius-full);
        background: var!(foreground);
        transition: format!("{} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_form_switch_label {
        font-size: var!(font-base);
        font-weight: var!(font-weight-medium);
        color: var!(foreground);
    }

    pub(crate) c_euv_slider_input {
        flex: "1";
        width: "100%";
        height: "24px";
        padding: "0px";
        background: "transparent";
        cursor: "pointer";
        touch-action: "manipulation";
        -webkit-appearance: "none";
        appearance: "none";
        border: "none";
        outline: "none";
        ::-webkit-slider-runnable-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border: format!("{} solid {}", var!(border-width), var!(input));
            border-radius: var!(radius-full);
        }
        ::-webkit-slider-thumb {
            width: "16px";
            height: "16px";
            background: var!(background);
            border: format!("2px solid {}", var!(primary));
            border-radius: var!(radius-full);
            cursor: "pointer";
            -webkit-appearance: "none";
            // The thumb is taller than the track it rides on, so the margin
            // pulls it up until it sits centred on the track.
            margin-top: "-6px";
        }
        : active::-webkit-slider-thumb {
            transform: "scale(0.92)";
        }
        ::-moz-range-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border: format!("{} solid {}", var!(border-width), var!(input));
            border-radius: var!(radius-full);
        }
        ::-moz-range-thumb {
            width: "16px";
            height: "16px";
            background: var!(background);
            border: format!("2px solid {}", var!(primary));
            border-radius: var!(radius-full);
            cursor: "pointer";
        }
        :focus {
            outline: "none";
        }
    }

    pub(crate) c_euv_slider_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-md);
        margin-bottom: var!(space-sm);
    }

    pub(crate) c_euv_slider_header {
        display: "flex";
        align-items: "baseline";
        justify-content: "space-between";
        gap: var!(space-sm);
        margin-bottom: var!(space-xs);
    }

    pub(crate) c_euv_radio_group {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        border: "none";
        margin: "0px";
        padding: "0px";
    }

    pub(crate) c_euv_radio_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-height: var!(control-height);
        padding: format!("{} 0px", var!(space-xs));
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_radio_item_checked {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "16px";
        height: "16px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("2px solid {}", var!(primary));
        border-radius: var!(radius-full);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_radio_item_unchecked {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "16px";
        height: "16px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-full);
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_radio_input {
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

    pub(crate) c_progress_bar_fill {
        height: "100%";
        width: "0%";
        background: var!(primary);
        transition: format!("width {} {}", var!(duration-normal), var!(ease-out));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Select
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_select {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_select_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        appearance: "none";
        -webkit-appearance: "none";
        touch-action: "manipulation";
        // Hover and focus only swap the border and the fill, so they ease
        // between the two states instead of snapping.
        transition: format!(
        "border-color {} {}, background-color {} {}",
        var!(duration-fast),
        var!(ease-out),
        var!(duration-fast),
        var!(ease-out)
        );
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
        border-radius: var!(radius-md);
    }

    pub(crate) c_euv_select_value {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_euv_select_placeholder {
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_select_caret {
        flex-shrink: "0";
        font-size: var!(font-xs);
        line-height: "1";
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_select_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        right: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        max-height: "240px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        border-radius: var!(radius-md);
    }

    pub(crate) c_euv_select_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        right: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        max-height: "240px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_select_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        right: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        max-height: "240px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_select_option {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_select_option_active {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: "none";
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
        }
    }

    pub(crate) c_euv_select_option_disabled {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: "none";
        background: "transparent";
        color: var!(muted-foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "not-allowed";
        opacity: "0.5";
        box-sizing: "border-box";
    }

    pub(crate) c_euv_select_group {
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        letter-spacing: var!(letter-spacing-wide);
        text-transform: var!(text-transform-uppercase);
        cursor: "default";
    }

    pub(crate) c_euv_select_empty {
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Checkbox Group
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_checkbox_group {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        border: "none";
        margin: "0px";
        padding: "0px";
    }

    pub(crate) c_euv_checkbox_group_label {
        display: "block";
        margin-bottom: var!(space-sm);
        color: "inherit";
        font-weight: var!(font-weight-medium);
        font-size: var!(font-base);
    }

    pub(crate) c_euv_checkbox_group_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-height: var!(control-height);
        padding: format!("{} 0px", var!(space-xs));
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_checkbox_group_box {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "16px";
        height: "16px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-xs);
        background: var!(background);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_checkbox_group_box_checked {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "16px";
        height: "16px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("2px solid {}", var!(primary));
        border-radius: var!(radius-xs);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_checkbox_group_box_unchecked {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "16px";
        height: "16px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-xs);
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_checkbox_group_input {
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

    pub(crate) c_euv_checkbox_group_text {
        font-size: var!(font-base);
        color: "inherit";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Segmented
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_segmented {
        display: "inline-flex";
        align-items: "stretch";
        // A row of segments, not a text atom (§7): the labels stay on one
        // line and the row scrolls sideways instead of wrapping.
        white-space: "nowrap";
        border: format!("{} solid {}", var!(border-width), var!(input));
        box-sizing: "border-box";
        max-width: "100%";
        overflow-x: "auto";
    }

    pub(crate) c_euv_segmented_item {
        flex: "1 1 auto";
        min-width: "0px";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: "none";
        border-right: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        text-align: "center";
        white-space: "nowrap";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_segmented_item_active {
        flex: "1 1 auto";
        min-width: "0px";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: "none";
        border-right: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        text-align: "center";
        white-space: "nowrap";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Toggle
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_toggle {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        border-radius: var!(radius-md);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1";
        white-space: "nowrap";
        background: "transparent";
        color: var!(foreground);
        border: format!("{} solid transparent", var!(border-width));
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        transition: format!("{} {}", var!(duration-fast), var!(ease-out));
        :hover {
            background: var!(muted);
            color: var!(muted-foreground);
        }
        :disabled {
            opacity: "0.5";
            pointer-events: "none";
        }
    }

    pub(crate) c_euv_toggle_outline {
        border-color: var!(input);
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub(crate) c_euv_toggle_size_default {
        height: "40px";
        min-width: "40px";
        padding-inline: var!(space-md);
    }

    pub(crate) c_euv_toggle_size_small {
        height: var!(control-height);
        min-width: var!(control-height);
        padding-inline: "10px";
    }

    pub(crate) c_euv_toggle_size_large {
        // The reference large control is 44px: one step above the 40px
        // default, and the only control height outside the four-pixel
        // spacing scale.
        height: "44px";
        min-width: "44px";
        padding-inline: "20px";
    }

    pub(crate) c_euv_toggle_pressed {
        background: var!(accent);
        color: var!(accent-foreground);
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub(crate) c_euv_toggle_group {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-2xs);
        flex-wrap: "wrap";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Number Input
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_number_input {
        display: "flex";
        align-items: "stretch";
        width: "100%";
        min-height: var!(control-height);
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-sizing: "border-box";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        // The box is the wrapper, so the focus ring belongs to the wrapper:
        // the field inside carries no border of its own.
        :focus-within {
            box-shadow: format!("inset 0 0 0 1px {}", var!(ring));
        }
    }

    pub(crate) c_euv_number_input_field {
        flex: "1";
        min-width: "0px";
        min-height: var!(control-height);
        padding: format!("0px {}", var!(space-lg));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        box-sizing: "border-box";
        outline: "none";
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        // The wrapper draws the focus ring — the field has no border of its
        // own, so a ring inside it would sit 16px away from the frame the
        // user is looking at.
        :focus-visible {
            box-shadow: "none";
        }
    }

    pub(crate) c_euv_number_input_stepper {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-2xl);
        flex-shrink: "0";
        min-height: var!(control-height);
        padding: "0px";
        border: "none";
        border-left: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_number_input_stepper_disabled {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-2xl);
        flex-shrink: "0";
        min-height: var!(control-height);
        padding: "0px";
        border: "none";
        border-left: format!("{} solid {}", var!(border-width), var!(border));
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "not-allowed";
        opacity: "0.5";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Auto Complete
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_auto_complete {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_auto_complete_input {
        width: "100%";
        min-height: var!(control-height);
        padding: format!("0px {}", var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
    }

    pub(crate) c_euv_auto_complete_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        right: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        max-height: "240px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_auto_complete_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        right: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        max-height: "240px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_auto_complete_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        right: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        max-height: "240px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_auto_complete_option {
        display: "block";
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_auto_complete_option_active {
        display: "block";
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: "none";
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-base);
        font-weight: var!(font-weight-semibold);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
        }
    }

    pub(crate) c_euv_auto_complete_empty {
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Form
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_form {
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_form_actions {
        display: "flex";
        align-items: "center";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Date Picker
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_date_picker {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_date_picker_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
    }

    pub(crate) c_euv_date_picker_value {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_euv_date_picker_placeholder {
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_date_picker_caret {
        flex-shrink: "0";
        font-size: var!(font-xs);
        line-height: "1";
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_date_picker_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "280px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_date_picker_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "280px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_date_picker_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "280px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_date_picker_footer {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }

    // The month-step buttons of the picker's own header.
    //
    // They are square boxes like the calendar's day cells, but they are
    // not day cells: reusing the day class here would dress a month
    // step up as a date and would make the days impossible to count.
    pub(crate) c_euv_date_picker_nav {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "36px";
        min-width: "36px";
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Time Picker
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_time_picker {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_time_picker_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
    }

    pub(crate) c_euv_time_picker_value {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_euv_time_picker_placeholder {
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_time_picker_caret {
        flex-shrink: "0";
        font-size: var!(font-xs);
        line-height: "1";
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_time_picker_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "220px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_time_picker_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "220px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_time_picker_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "220px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_time_picker_grid {
        display: "flex";
        gap: var!(space-sm);
        padding: var!(space-sm);
    }

    pub(crate) c_euv_time_picker_column {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        flex: "1";
        min-width: "0px";
        max-height: "160px";
        overflow-y: "auto";
    }

    pub(crate) c_euv_time_picker_cell {
        display: "block";
        width: "100%";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        text-align: "center";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_time_picker_cell_active {
        display: "block";
        width: "100%";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: "none";
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        text-align: "center";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(primary);
            color: var!(primary-foreground);
        }
    }

    pub(crate) c_euv_time_picker_footer {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Transfer
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_transfer {
        display: "flex";
        align-items: "stretch";
        gap: var!(gap-component);
        width: "100%";
    }

    pub(crate) c_euv_transfer_panel {
        display: "flex";
        flex-direction: "column";
        flex: "1";
        min-width: "0px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_transfer_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
        font-size: var!(font-sm);
    }

    pub(crate) c_euv_transfer_list {
        display: "flex";
        flex-direction: "column";
        flex: "1";
        min-height: "0px";
        max-height: "240px";
        overflow-y: "auto";
        padding: var!(space-2xs);
    }

    pub(crate) c_euv_transfer_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        color: var!(foreground);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_transfer_item_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        color: var!(foreground);
        background: var!(muted);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_transfer_box {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "14px";
        height: "14px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-sm);
        background: var!(background);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_transfer_box_active {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: "14px";
        height: "14px";
        flex-shrink: "0";
        box-sizing: "border-box";
        border: format!("2px solid {}", var!(primary));
        border-radius: var!(radius-sm);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-xs);
        font-weight: var!(font-weight-bold);
        line-height: "1";
    }

    pub(crate) c_euv_transfer_actions {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Cascader
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_cascader {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_cascader_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
    }

    pub(crate) c_euv_cascader_value {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_euv_cascader_placeholder {
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_cascader_caret {
        flex-shrink: "0";
        font-size: var!(font-xs);
        line-height: "1";
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_cascader_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_cascader_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "flex";
        flex-direction: "row";
    }

    pub(crate) c_euv_cascader_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_cascader_columns {
        display: "flex";
        flex-direction: "row";
    }

    pub(crate) c_euv_cascader_column {
        display: "flex";
        flex-direction: "column";
        min-width: "140px";
        max-height: "240px";
        overflow-y: "auto";
        padding: var!(space-2xs);
        border-right: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub(crate) c_euv_cascader_option {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        color: var!(foreground);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_cascader_option_active {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        color: var!(foreground);
        background: var!(muted);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Tree Select
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_tree_select {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_tree_select_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
    }

    pub(crate) c_euv_tree_select_value {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_euv_tree_select_placeholder {
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_tree_select_caret {
        flex-shrink: "0";
        font-size: var!(font-xs);
        line-height: "1";
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_tree_select_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        max-height: "280px";
        overflow-y: "auto";
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_tree_select_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        max-height: "280px";
        overflow-y: "auto";
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "block";
    }

    pub(crate) c_euv_tree_select_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        max-height: "280px";
        overflow-y: "auto";
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Mentions
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_mentions {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        width: "100%";
    }

    pub(crate) c_euv_mentions_input {
        width: "100%";
        min-height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("{} solid {}", var!(border-width), var!(input));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :hover {
            border-color: var!(ring);
            background: var!(muted);
        }
        :focus {
            outline: "none";
            border-color: var!(ring);
        }
    }

    pub(crate) c_euv_mentions_panel {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "180px";
        max-height: "220px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_mentions_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "180px";
        max-height: "220px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "flex";
        flex-direction: "column";
    }

    pub(crate) c_euv_mentions_panel_closed {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "180px";
        max-height: "220px";
        overflow-y: "auto";
        border: format!("{} solid {}", var!(border-width), var!(input));
        background: var!(background);
        box-shadow: var!(shadow-md);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_mentions_option {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        color: var!(foreground);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    pub(crate) c_euv_mentions_option_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        font-weight: var!(font-weight-semibold);
        color: var!(foreground);
        background: var!(muted);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(muted);
            color: var!(primary);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Label
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_label {
        display: "block";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_label_disabled {
        display: "block";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1";
        opacity: "0.7";
        cursor: "not-allowed";
        user-select: "none";
        -webkit-user-select: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Textarea
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_textarea {
        display: "block";
        width: "100%";
        min-height: "80px";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        line-height: "1.5";
        box-sizing: "border-box";
        resize: "vertical";
        outline: "none";
        transition: format!("{} {}", var!(duration-fast), var!(ease-out));
        :focus-visible {
            border-color: var!(ring);
        }
        :disabled {
            opacity: "0.5";
            cursor: "not-allowed";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Button Group
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_button_group_horizontal {
        display: "inline-flex";
        align-items: "stretch";
        width: "fit-content";
        flex-direction: "row";
        border-radius: var!(radius-md);
        overflow: "hidden";
    }

    pub(crate) c_euv_button_group_vertical {
        display: "inline-flex";
        align-items: "stretch";
        width: "fit-content";
        flex-direction: "column";
        border-radius: var!(radius-md);
        overflow: "hidden";
    }

    pub(crate) c_euv_button_group_text {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(muted);
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
    }

    pub(crate) c_euv_button_group_separator_vertical {
        width: "1px";
        align-self: "stretch";
        margin: format!("{} 0px", var!(space-xs));
        background: var!(border);
    }

    pub(crate) c_euv_button_group_separator_horizontal {
        height: "1px";
        align-self: "stretch";
        margin: format!("0px {}", var!(space-xs));
        background: var!(border);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Input Group
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_input_group {
        display: "flex";
        align-items: "center";
        width: "100%";
        gap: var!(space-2xs);
        padding: var!(space-2xs);
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_input_group_disabled {
        display: "flex";
        align-items: "center";
        width: "100%";
        gap: var!(space-2xs);
        padding: var!(space-2xs);
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(muted);
        color: var!(muted-foreground);
        box-sizing: "border-box";
        opacity: "0.5";
    }

    pub(crate) c_euv_input_group_addon_inline_start {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        order: "-1";
        padding-inline-start: var!(space-sm);
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
    }

    pub(crate) c_euv_input_group_addon_inline_end {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-sm);
        order: "1";
        padding-inline-end: var!(space-sm);
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
    }

    pub(crate) c_euv_input_group_addon_block_start {
        display: "inline-flex";
        align-items: "center";
        justify-content: "flex-start";
        gap: var!(space-sm);
        order: "-1";
        width: "100%";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
    }

    pub(crate) c_euv_input_group_addon_block_end {
        display: "inline-flex";
        align-items: "center";
        justify-content: "flex-start";
        gap: var!(space-sm);
        order: "1";
        width: "100%";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
    }

    pub(crate) c_euv_input_group_button_xs {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-2xs);
        height: "24px";
        padding-inline: var!(space-xs);
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-xs);
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(muted);
        }
    }

    pub(crate) c_euv_input_group_button_sm {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-xs);
        height: "32px";
        padding-inline: var!(space-sm);
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(muted);
        }
    }

    pub(crate) c_euv_input_group_button_icon_xs {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: "24px";
        height: "24px";
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        cursor: "pointer";
        :hover {
            background: var!(muted);
        }
    }

    pub(crate) c_euv_input_group_button_icon_sm {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: "32px";
        height: "32px";
        padding: "0px";
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        cursor: "pointer";
        :hover {
            background: var!(muted);
        }
    }

    pub(crate) c_euv_input_group_text {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
    }

    pub(crate) c_euv_input_group_input {
        flex: "1";
        min-width: "0px";
        height: var!(control-height);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        box-sizing: "border-box";
        outline: "none";
    }

    pub(crate) c_euv_input_group_textarea {
        flex: "1";
        min-width: "0px";
        min-height: "80px";
        padding: var!(space-sm);
        border: "none";
        border-radius: var!(radius-sm);
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        line-height: "1.5";
        box-sizing: "border-box";
        resize: "none";
        outline: "none";
    }
    pub c_euv_radio_group_root {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub c_euv_radio_group_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: var!(space-xs) var!(space-sm);
        border: var!(border-width) solid var!(border);
        border-radius: var!(radius-sm);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        text-align: "left";
        cursor: "pointer";
    }

    pub c_euv_radio_group_item_checked {
        border-color: var!(primary);
        background: var!(primary);
        color: var!(primary-foreground);
    }

    pub c_euv_toast {
        position: "relative";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        width: "100%";
        padding: var!(space-md);
        border: var!(border-width) solid var!(border);
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_toast_destructive {
        border-color: var!(destructive);
        background: var!(destructive);
        color: var!(destructive-foreground);
    }

    pub c_euv_toast_title {
        font-size: var!(font-sm);
        font-weight: "500";
        line-height: "1.5";
    }

    pub c_euv_toast_description {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        line-height: "1.5";
    }

    pub c_euv_toast_action {
        display: "inline-flex";
        align-items: "center";
        align-self: "flex-start";
        padding: var!(space-2xs) var!(space-sm);
        border: var!(border-width) solid var!(border);
        border-radius: var!(radius-sm);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-sm);
        line-height: "1.5";
        white-space: "nowrap";
        cursor: "pointer";
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    pub c_euv_toast_close {
        position: "absolute";
        top: var!(space-sm);
        right: var!(space-sm);
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
    pub c_euv_input_otp {
        position: "relative";
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    pub c_euv_input_otp_input {
        position: "absolute";
        inset: "0px";
        width: "100%";
        height: "100%";
        padding: "0px";
        border: "none";
        background: "transparent";
        color: "transparent";
        caret-color: "transparent";
        font-family: "inherit";
        font-size: var!(font-sm);
        outline: "none";
    }

    pub c_euv_input_otp_group {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    pub c_euv_input_otp_slot {
        position: "relative";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-4xl);
        height: var!(space-4xl);
        border: var!(border-width) solid var!(input);
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        line-height: "1";
    }

    pub c_euv_input_otp_slot_active {
        border-color: var!(ring);
        box-shadow: format!("0 0 0 2px {}", var!(ring));
    }

    pub c_euv_input_otp_slot_idle {}

    pub c_euv_input_otp_separator {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        line-height: "1";
    }
}

#[cfg(feature = "theme-custom")]
class! {
    pub c_form_switch_off {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Form Elements
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_euv_input_wrapper {}

    pub c_form_label {}

    pub c_inline_input_row {}

    pub c_euv_input {}

    pub c_euv_input_no_transition {}

    pub c_euv_input_error {}

    pub c_form_checkbox {}

    pub c_form_checkbox_label {}

    pub c_form_checkbox_row {}

    pub c_select_input {}

    pub c_textarea_input {}

    pub c_textarea_input_error {}

    pub c_textarea_counter {}

    pub c_textarea_counter_text {}

    pub c_field_error_text {}

    pub(crate) c_form_switch {}

    pub(crate) c_form_switch_row {}

    pub(crate) c_form_switch_on {}

    pub(crate) c_form_switch_track {}

    pub(crate) c_form_switch_thumb {}

    pub(crate) c_form_switch_label {}

    pub(crate) c_euv_slider_input {}

    pub(crate) c_euv_slider_row {}

    pub(crate) c_euv_slider_header {}

    pub(crate) c_euv_radio_group {}

    pub(crate) c_euv_radio_item {}

    pub(crate) c_euv_radio_item_checked {}

    pub(crate) c_euv_radio_item_unchecked {}

    pub(crate) c_euv_radio_input {}

    pub(crate) c_progress_bar_fill {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Select
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_select {}

    pub(crate) c_euv_select_trigger {}

    pub(crate) c_euv_select_value {}

    pub(crate) c_euv_select_placeholder {}

    pub(crate) c_euv_select_caret {}

    pub(crate) c_euv_select_panel {}

    pub(crate) c_euv_select_panel_open {}

    pub(crate) c_euv_select_panel_closed {}

    pub(crate) c_euv_select_option {}

    pub(crate) c_euv_select_option_active {}

    pub(crate) c_euv_select_option_disabled {}

    pub(crate) c_euv_select_group {}

    pub(crate) c_euv_select_empty {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Checkbox Group
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_checkbox_group {}

    pub(crate) c_euv_checkbox_group_label {}

    pub(crate) c_euv_checkbox_group_item {}

    pub(crate) c_euv_checkbox_group_box {}

    pub(crate) c_euv_checkbox_group_box_checked {}

    pub(crate) c_euv_checkbox_group_box_unchecked {}

    pub(crate) c_euv_checkbox_group_input {}

    pub(crate) c_euv_checkbox_group_text {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Segmented
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_segmented {}

    pub(crate) c_euv_segmented_item {}

    pub(crate) c_euv_segmented_item_active {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Number Input
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_toggle {}

    pub(crate) c_euv_toggle_outline {}

    pub(crate) c_euv_toggle_size_default {}

    pub(crate) c_euv_toggle_size_small {}

    pub(crate) c_euv_toggle_size_large {}

    pub(crate) c_euv_toggle_pressed {}

    pub(crate) c_euv_toggle_group {}

    pub(crate) c_euv_number_input {}

    pub(crate) c_euv_number_input_field {}

    pub(crate) c_euv_number_input_stepper {}

    pub(crate) c_euv_number_input_stepper_disabled {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Auto Complete
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_auto_complete {}

    pub(crate) c_euv_auto_complete_input {}

    pub(crate) c_euv_auto_complete_panel {}

    pub(crate) c_euv_auto_complete_panel_open {}

    pub(crate) c_euv_auto_complete_panel_closed {}

    pub(crate) c_euv_auto_complete_option {}

    pub(crate) c_euv_auto_complete_option_active {}

    pub(crate) c_euv_auto_complete_empty {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Form
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_form {}

    pub(crate) c_euv_form_actions {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Date Picker
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_date_picker {}

    pub(crate) c_euv_date_picker_trigger {}

    pub(crate) c_euv_date_picker_value {}

    pub(crate) c_euv_date_picker_placeholder {}

    pub(crate) c_euv_date_picker_caret {}

    pub(crate) c_euv_date_picker_panel {}

    pub(crate) c_euv_date_picker_panel_open {}

    pub(crate) c_euv_date_picker_panel_closed {}

    pub(crate) c_euv_date_picker_footer {}

    // The month-step buttons of the picker's own header.
    //
    // They are square boxes like the calendar's day cells, but they are
    // not day cells: reusing the day class here would dress a month
    // step up as a date and would make the days impossible to count.
    pub(crate) c_euv_date_picker_nav {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Time Picker
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_time_picker {}

    pub(crate) c_euv_time_picker_trigger {}

    pub(crate) c_euv_time_picker_value {}

    pub(crate) c_euv_time_picker_placeholder {}

    pub(crate) c_euv_time_picker_caret {}

    pub(crate) c_euv_time_picker_panel {}

    pub(crate) c_euv_time_picker_panel_open {}

    pub(crate) c_euv_time_picker_panel_closed {}

    pub(crate) c_euv_time_picker_grid {}

    pub(crate) c_euv_time_picker_column {}

    pub(crate) c_euv_time_picker_cell {}

    pub(crate) c_euv_time_picker_cell_active {}

    pub(crate) c_euv_time_picker_footer {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Transfer
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_transfer {}

    pub(crate) c_euv_transfer_panel {}

    pub(crate) c_euv_transfer_header {}

    pub(crate) c_euv_transfer_list {}

    pub(crate) c_euv_transfer_item {}

    pub(crate) c_euv_transfer_item_active {}

    pub(crate) c_euv_transfer_box {}

    pub(crate) c_euv_transfer_box_active {}

    pub(crate) c_euv_transfer_actions {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Cascader
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_cascader {}

    pub(crate) c_euv_cascader_trigger {}

    pub(crate) c_euv_cascader_value {}

    pub(crate) c_euv_cascader_placeholder {}

    pub(crate) c_euv_cascader_caret {}

    pub(crate) c_euv_cascader_panel {}

    pub(crate) c_euv_cascader_panel_open {}

    pub(crate) c_euv_cascader_panel_closed {}

    pub(crate) c_euv_cascader_columns {}

    pub(crate) c_euv_cascader_column {}

    pub(crate) c_euv_cascader_option {}

    pub(crate) c_euv_cascader_option_active {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Tree Select
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_tree_select {}

    pub(crate) c_euv_tree_select_trigger {}

    pub(crate) c_euv_tree_select_value {}

    pub(crate) c_euv_tree_select_placeholder {}

    pub(crate) c_euv_tree_select_caret {}

    pub(crate) c_euv_tree_select_panel {}

    pub(crate) c_euv_tree_select_panel_open {}

    pub(crate) c_euv_tree_select_panel_closed {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Mentions
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_mentions {}

    pub(crate) c_euv_mentions_input {}

    pub(crate) c_euv_mentions_panel {}

    pub(crate) c_euv_mentions_panel_open {}

    pub(crate) c_euv_mentions_panel_closed {}

    pub(crate) c_euv_mentions_option {}

    pub(crate) c_euv_mentions_option_active {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Label
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_label {}

    pub(crate) c_euv_label_disabled {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Textarea
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_textarea {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Button Group
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_button_group_horizontal {}

    pub(crate) c_euv_button_group_vertical {}

    pub(crate) c_euv_button_group_text {}

    pub(crate) c_euv_button_group_separator_vertical {}

    pub(crate) c_euv_button_group_separator_horizontal {}

    // ═══════════════════════════════════════════════════════════════════════════
    // Input Group
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_input_group {}

    pub(crate) c_euv_input_group_disabled {}

    pub(crate) c_euv_input_group_addon_inline_start {}

    pub(crate) c_euv_input_group_addon_inline_end {}

    pub(crate) c_euv_input_group_addon_block_start {}

    pub(crate) c_euv_input_group_addon_block_end {}

    pub(crate) c_euv_input_group_button_xs {}

    pub(crate) c_euv_input_group_button_sm {}

    pub(crate) c_euv_input_group_button_icon_xs {}

    pub(crate) c_euv_input_group_button_icon_sm {}

    pub(crate) c_euv_input_group_text {}

    pub(crate) c_euv_input_group_input {}

    pub(crate) c_euv_input_group_textarea {}
    pub c_euv_radio_group_root {}

    pub c_euv_radio_group_item {}

    pub c_euv_radio_group_item_checked {}

    pub c_euv_toast {}

    pub c_euv_toast_destructive {}

    pub c_euv_toast_title {}

    pub c_euv_toast_description {}

    pub c_euv_toast_action {}

    pub c_euv_toast_close {}
    pub c_euv_input_otp {}

    pub c_euv_input_otp_input {}

    pub c_euv_input_otp_group {}

    pub c_euv_input_otp_slot {}

    pub c_euv_input_otp_slot_active {}

    pub c_euv_input_otp_slot_idle {}

    pub c_euv_input_otp_separator {}
}
