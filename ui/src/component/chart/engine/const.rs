/// Outer padding in logical pixels inside the chart canvas bounds.
pub(crate) const ENGINE_CANVAS_PAD: f32 = 12.0;

/// Interactive data-zoom slider strip height in logical pixels.
pub(crate) const ENGINE_SLIDER_HEIGHT: f32 = 28.0;

/// Interactive visual-map bar strip height in logical pixels.
pub(crate) const ENGINE_VISUAL_BAR_HEIGHT: f32 = 24.0;

/// Legend row height in logical pixels.
pub(crate) const ENGINE_LEGEND_ROW_H: f32 = 20.0;

/// Legend vertical column width in logical pixels when docked left or right.
pub(crate) const ENGINE_LEGEND_SIDE_W: f32 = 104.0;

/// Floating tooltip card maximum width in logical pixels.
pub(crate) const ENGINE_TOOLTIP_MAX_W: f32 = 220.0;

/// DataView header label for the primary category/label column.
pub(crate) const ENGINE_COL_CATEGORY: &str = "Category";

/// DataView header label for value columns when unlabeled.
pub(crate) const ENGINE_COL_VALUE: &str = "Value";

/// DataView header label for series name column.
pub(crate) const ENGINE_COL_SERIES: &str = "Series";

/// Default stack group key applied when MagicType switches to `Stack`.
pub(crate) const ENGINE_MAGIC_STACK_KEY: &str = "magic_stack";
