/// The CSS padding property each safe-area edge is written through.
///
/// The four edges are written and read back as a set, so they are named
/// as a family: a change to one edge is a change to all four.
pub(crate) const CSS_PROPERTY_PADDING_TOP: &str = "padding-top";
pub(crate) const CSS_PROPERTY_PADDING_RIGHT: &str = "padding-right";
pub(crate) const CSS_PROPERTY_PADDING_BOTTOM: &str = "padding-bottom";
pub(crate) const CSS_PROPERTY_PADDING_LEFT: &str = "padding-left";

/// The `env()` expression read for each safe-area edge, with the 0px
/// fallback that keeps the layout correct on a device without a notch.
pub(crate) const CSS_SAFE_AREA_INSET_TOP: &str = "env(safe-area-inset-top, 0px)";
pub(crate) const CSS_SAFE_AREA_INSET_RIGHT: &str = "env(safe-area-inset-right, 0px)";
pub(crate) const CSS_SAFE_AREA_INSET_BOTTOM: &str = "env(safe-area-inset-bottom, 0px)";
pub(crate) const CSS_SAFE_AREA_INSET_LEFT: &str = "env(safe-area-inset-left, 0px)";

/// The custom property each cached safe-area edge is written to, so the
/// value survives after the measuring element is gone.
pub(crate) const CSS_CUSTOM_PROPERTY_SAFE_AREA_TOP: &str = "--safe-area-inset-top";
pub(crate) const CSS_CUSTOM_PROPERTY_SAFE_AREA_RIGHT: &str = "--safe-area-inset-right";
pub(crate) const CSS_CUSTOM_PROPERTY_SAFE_AREA_BOTTOM: &str = "--safe-area-inset-bottom";
pub(crate) const CSS_CUSTOM_PROPERTY_SAFE_AREA_LEFT: &str = "--safe-area-inset-left";
