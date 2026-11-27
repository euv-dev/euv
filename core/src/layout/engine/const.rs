/// Surgical layout damage succeeded without full-viewport fallback.
pub(crate) const LAYOUT_FALLBACK_NONE: u8 = 0;

/// Layout pass fell back to full redraw due to non-axis-aligned transforms or unsafe geometry.
pub(crate) const LAYOUT_FALLBACK_UNSAFE: u8 = 1;

/// Layout pass fell back to full redraw because a top-layer portal or fixed node moved or resized.
pub(crate) const LAYOUT_FALLBACK_PORTAL: u8 = 2;

/// Layout pass fell back to full redraw because touched nodes exceeded `DIRTY_PAINT_FULL_THRESHOLD`.
pub(crate) const LAYOUT_FALLBACK_OVERFLOW: u8 = 3;
