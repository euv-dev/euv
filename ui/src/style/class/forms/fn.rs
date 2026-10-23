use super::*;

class! {
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
        min-height: var!(min-height-base);
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("1px dashed {}", var!(border));
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

    pub c_form_switch_off {
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
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
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
        border-radius: "50%";
        background: var!(foreground);
        transition: format!("{} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_form_switch_label {
        font-size: var!(font-base);
        font-weight: "500";
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
            border: format!("1px solid {}", var!(border));
            border-radius: "0px";
        }
        ::-webkit-slider-thumb {
            width: "16px";
            height: "16px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
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
            border: format!("1px solid {}", var!(border));
            border-radius: "0px";
        }
        ::-moz-range-thumb {
            width: "16px";
            height: "16px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
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
        min-height: var!(min-height-base);
        padding: format!("{} 0px", var!(space-xs));
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
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
        border: format!("2px solid {}", var!(accent));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "700";
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
        border: format!("1px dashed {}", var!(border));
        border-radius: "0px";
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: "700";
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
        background: var!(accent);
        transition: format!("width {} {}", var!(duration-normal), var!(ease-out));
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
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub c_inline_input_row {
        display: "flex";
        align-items: "center";
        gap: var!(gap-component);
    }

    pub c_euv_input {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
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

    pub c_euv_input_no_transition {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
    }

    pub c_euv_input_error {
        width: "100%";
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(foreground));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        vertical-align: "middle";
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
    }

    pub c_form_checkbox {
        cursor: "pointer";
        width: var!(space-lg);
        height: var!(space-lg);
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
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
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
            border-color: var!(accent);
            background: var!(accent-muted);
        }
    }

    pub c_textarea_input {
        width: "100%";
        max-width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border: format!("1px solid {}", var!(border));
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

    pub c_textarea_input_error {
        width: "100%";
        max-width: "100%";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border: format!("1px solid {}", var!(foreground));
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
        :focus {
            outline: "none";
            border-color: var!(foreground);
        }
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
        color: var!(foreground);
        font-size: var!(font-base);
        margin-top: var!(space-xs);
        margin-bottom: var!(space-sm);
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
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        appearance: "none";
        -webkit-appearance: "none";
        touch-action: "manipulation";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
        }
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
        box-sizing: "border-box";
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
            background: var!(accent-muted);
            color: var!(accent);
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
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "600";
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(accent);
            color: var!(text-on-accent);
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
        letter-spacing: "0.08em";
        text-transform: "uppercase";
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
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub(crate) c_euv_checkbox_group_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        min-height: var!(min-height-base);
        padding: format!("{} 0px", var!(space-xs));
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
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
        border: format!("1px dashed {}", var!(border));
        border-radius: "0px";
        background: var!(background);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "700";
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
        border: format!("2px solid {}", var!(accent));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "700";
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
        border: format!("1px dashed {}", var!(border));
        border-radius: "0px";
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        font-weight: "700";
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
        border: format!("1px solid {}", var!(border));
        box-sizing: "border-box";
        max-width: "100%";
        overflow-x: "auto";
    }

    pub(crate) c_euv_segmented_item {
        flex: "1 1 auto";
        min-width: "0px";
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: "none";
        border-right: format!("1px solid {}", var!(border));
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
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_segmented_item_active {
        flex: "1 1 auto";
        min-width: "0px";
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: "none";
        border-right: format!("1px solid {}", var!(border));
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-sm);
        font-weight: "600";
        text-align: "center";
        white-space: "nowrap";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent);
            color: var!(text-on-accent);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Number Input
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_euv_number_input {
        display: "flex";
        align-items: "stretch";
        width: "100%";
        min-height: var!(min-height-base);
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-sizing: "border-box";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
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
        min-height: var!(min-height-base);
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
        min-height: var!(min-height-base);
        padding: "0px";
        border: "none";
        border-left: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        line-height: "1";
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_number_input_stepper_disabled {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-2xl);
        flex-shrink: "0";
        min-height: var!(min-height-base);
        padding: "0px";
        border: "none";
        border-left: format!("1px solid {}", var!(border));
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
        min-height: var!(min-height-base);
        padding: format!("0px {}", var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_auto_complete_option_active {
        display: "block";
        width: "100%";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        border: "none";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "600";
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(accent);
            color: var!(text-on-accent);
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
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_date_picker_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "280px";
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
        box-sizing: "border-box";
        display: "none";
    }

    pub(crate) c_euv_date_picker_footer {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border-top: format!("1px dashed {}", var!(border));
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
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_time_picker_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        width: "220px";
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_time_picker_cell_active {
        display: "block";
        width: "100%";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: "none";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-sm);
        font-weight: "600";
        text-align: "center";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            background: var!(accent);
            color: var!(text-on-accent);
        }
    }

    pub(crate) c_euv_time_picker_footer {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border-top: format!("1px dashed {}", var!(border));
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_transfer_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border-bottom: format!("1px dashed {}", var!(border));
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
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_transfer_item_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        background: var!(accent-muted);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
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
        border: format!("1px dashed {}", var!(border));
        border-radius: "0px";
        background: var!(background);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "700";
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
        border: format!("2px solid {}", var!(accent));
        border-radius: "0px";
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-xs);
        font-weight: "700";
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
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
        box-sizing: "border-box";
    }

    pub(crate) c_euv_cascader_panel_open {
        position: "absolute";
        top: "100%";
        left: "0px";
        z-index: "150";
        margin-top: var!(space-2xs);
        min-width: "240px";
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border-right: format!("1px dashed {}", var!(border));
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
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_cascader_option_active {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        background: var!(accent-muted);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
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
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-base);
        text-align: "left";
        cursor: "pointer";
        box-sizing: "border-box";
        touch-action: "manipulation";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        min-height: var!(min-height-base);
        padding: format!("{} {}", var!(space-xs), var!(space-lg));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-base);
        line-height: "normal";
        box-sizing: "border-box";
        outline: "none";
        color: var!(foreground);
        appearance: "none";
        -webkit-appearance: "none";
        -moz-appearance: "none";
        :hover {
            border-color: var!(accent);
            background: var!(accent-muted);
        }
        :focus {
            outline: "none";
            border-color: var!(accent);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
        border: format!("1px solid {}", var!(border));
        background: var!(background);
        box-shadow: var!(shadow-sm);
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
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_euv_mentions_option_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        background: var!(accent-muted);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }
}
