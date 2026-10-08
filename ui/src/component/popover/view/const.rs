/// Inline `style` value placing the body above the trigger, centred on its
/// horizontal axis (used when the caller anchors the popover to the top).
pub(crate) const POPOVER_BODY_STYLE_TOP: &str =
    "bottom: calc(100% + 8px); left: 50%; transform: translateX(-50%);";

/// Inline `style` value placing the body below the trigger, centred on its
/// horizontal axis (used when the caller anchors the popover to the bottom).
pub(crate) const POPOVER_BODY_STYLE_BOTTOM: &str =
    "top: calc(100% + 8px); left: 50%; transform: translateX(-50%);";

/// Inline `style` value placing the body to the left of the trigger, centred
/// on its vertical axis.
pub(crate) const POPOVER_BODY_STYLE_LEFT: &str =
    "top: 50%; right: calc(100% + 8px); transform: translateY(-50%);";

/// Inline `style` value placing the body to the right of the trigger, centred
/// on its vertical axis.
pub(crate) const POPOVER_BODY_STYLE_RIGHT: &str =
    "top: 50%; left: calc(100% + 8px); transform: translateY(-50%);";
