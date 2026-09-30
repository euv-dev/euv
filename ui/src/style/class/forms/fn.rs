use super::*;

class! {
    pub c_form_switch {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        cursor: "pointer";
        touch-action: "manipulation";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub c_form_switch_row {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-component);
        min-height: var!(min-height-base);
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("1px dashed {}", var!(border));
    }

    pub c_form_switch_on {
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

    pub c_form_switch_track {
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

    pub c_form_switch_thumb {
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

    pub c_form_switch_label {
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
    }

    pub c_euv_slider_input {
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
            margin-top: "-6px";
        }
        :active::-webkit-slider-thumb {
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

    pub c_euv_slider_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-md);
        margin-bottom: var!(space-sm);
    }

    pub c_euv_slider_header {
        display: "flex";
        align-items: "baseline";
        justify-content: "space-between";
        gap: var!(space-sm);
        margin-bottom: var!(space-xs);
    }

    pub c_euv_radio_group {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        border: "none";
        margin: "0px";
        padding: "0px";
    }

    pub c_euv_radio_item {
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

    pub c_euv_radio_item_checked {
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

    pub c_euv_radio_item_unchecked {
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

    pub c_euv_radio_input {
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

    pub c_progress_bar_fill {
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
}
