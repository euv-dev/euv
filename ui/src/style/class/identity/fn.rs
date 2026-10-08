use super::*;

class! {
    pub(crate) c_euv_avatar {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: "40px";
        height: "40px";
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent-muted);
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: "600";
        line-height: "1";
        text-align: "center";
        overflow: "hidden";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_avatar_small {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: "24px";
        height: "24px";
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent-muted);
        color: var!(foreground);
        font-size: var!(font-xs);
        font-weight: "600";
        line-height: "1";
        text-align: "center";
        overflow: "hidden";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_avatar_medium {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: "32px";
        height: "32px";
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent-muted);
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: "600";
        line-height: "1";
        text-align: "center";
        overflow: "hidden";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_avatar_large {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: "40px";
        height: "40px";
        box-sizing: "border-box";
        border: format!("1px solid {}", var!(border));
        border-radius: "0px";
        background: var!(accent-muted);
        color: var!(foreground);
        font-size: var!(font-lg);
        font-weight: "600";
        line-height: "1";
        text-align: "center";
        overflow: "hidden";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_avatar_square {
        border-radius: "50%";
    }

    pub(crate) c_euv_icon {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        color: "inherit";
        line-height: "1";
        flex-shrink: "0";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_icon_label {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        color: "inherit";
        font-size: var!(font-sm);
        font-weight: "500";
        line-height: "1";
        flex-shrink: "0";
    }

    pub(crate) c_euv_space {
        display: "block";
    }

    pub(crate) c_euv_panel {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        width: "100%";
        box-sizing: "border-box";
    }

    pub(crate) c_euv_panel_plain {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        width: "100%";
        box-sizing: "border-box";
        border: "none";
        background: "transparent";
    }

    pub(crate) c_euv_panel_bordered {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        width: "100%";
        box-sizing: "border-box";
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
        background: var!(background);
    }

    pub(crate) c_euv_panel_dashed {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        width: "100%";
        box-sizing: "border-box";
        padding: var!(space-lg);
        border: format!("1px dashed {}", var!(border));
        background: var!(background);
    }

    pub(crate) c_euv_panel_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
    }

    pub(crate) c_euv_panel_title {
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
    }

    pub(crate) c_euv_panel_subtitle {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        margin: "0px";
    }

    pub(crate) c_euv_panel_body {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        font-size: var!(font-base);
        color: var!(foreground);
    }

    pub(crate) c_euv_rating {
        position: "relative";
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-2xs);
        font-size: var!(font-lg);
        line-height: "1";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_euv_rating_star {
        font-size: var!(font-lg);
        line-height: "1";
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_rating_fill {
        position: "absolute";
        left: "0px";
        top: "0px";
        display: "flex";
        align-items: "center";
        gap: var!(space-2xs);
        overflow: "hidden";
        white-space: "nowrap";
        font-size: var!(font-lg);
        line-height: "1";
        color: var!(accent);
        pointer-events: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Global Utilities
    // ═══════════════════════════════════════════════════════════════════════════

    // Dev-only readouts produced by `euv_debug`. The base wrapper
    // establishes the inline-flex layout so the label and the value
    // share a baseline; the `code` / `pre` value elements apply
    // monospace font and wrap behaviour for the two modes
    // (`expanded: false` = single-line `code`, `expanded: true` =
    // multi-line `pre`).
    pub(crate) c_debug {
        display: "flex";
        flex-direction: "row";
        align-items: "baseline";
        gap: var!(space-sm);
        padding: var!(space-xs);
        border: format!("1px dashed {}", var!(accent-muted));
        border-radius: var!(radius-sm);
        background: var!(surface-muted);
        font-family: "ui-monospace, SFMono-Regular, monospace";
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: format!("{} 0px", var!(space-xs));
    }

    pub(crate) c_debug_label {
        font-weight: "600";
        color: var!(accent);
        flex-shrink: "0";
    }

    pub(crate) c_debug_value {
        font-family: "ui-monospace, SFMono-Regular, monospace";
        color: var!(foreground);
        margin: "0px";
    }
}
