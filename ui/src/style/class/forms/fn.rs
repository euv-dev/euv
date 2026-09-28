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
}
