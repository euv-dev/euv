/// Tag keyword for a generic container view.
pub(crate) const TAG_VIEW: &str = "view";

/// Tag keyword for a horizontal flex row.
pub(crate) const TAG_ROW: &str = "row";

/// Tag keyword for a vertical flex column.
pub(crate) const TAG_COLUMN: &str = "column";

/// Short alias tag keyword for a vertical flex column.
pub(crate) const TAG_COL: &str = "col";

/// Tag keyword for a grid container.
pub(crate) const TAG_GRID: &str = "grid";

/// Tag keyword for a scrollable container.
pub(crate) const TAG_SCROLL: &str = "scroll";

/// Tag keyword for a top-layer portal container.
pub(crate) const TAG_PORTAL: &str = "portal";

/// Tag keyword for a text run node.
pub(crate) const TAG_TEXT: &str = "text";

/// Tag keyword for an editable text input node.
pub(crate) const TAG_INPUT: &str = "input";

/// Tag keyword for a bitmap image node.
pub(crate) const TAG_IMAGE: &str = "image";

/// Tag keyword for a custom 2D canvas node.
pub(crate) const TAG_CANVAS: &str = "canvas";

/// Property keyword for base style expression.
pub(crate) const PROP_STYLE: &str = "style";

/// Property keyword for explicit width.
pub(crate) const PROP_WIDTH: &str = "width";

/// Property keyword for explicit height.
pub(crate) const PROP_HEIGHT: &str = "height";

/// Property keyword for uniform padding.
pub(crate) const PROP_PADDING: &str = "padding";

/// Property keyword for uniform margin.
pub(crate) const PROP_MARGIN: &str = "margin";

/// Property keyword for child gap.
pub(crate) const PROP_GAP: &str = "gap";

/// Property keyword for background color.
pub(crate) const PROP_BACKGROUND: &str = "background";

/// Property keyword for foreground text color.
pub(crate) const PROP_COLOR: &str = "color";

/// Property keyword for uniform corner radius.
pub(crate) const PROP_RADIUS: &str = "radius";

/// Property keyword for border `(width, color)` tuple.
pub(crate) const PROP_BORDER: &str = "border";

/// Property keyword for cross-axis item alignment.
pub(crate) const PROP_ALIGN: &str = "align";

/// Property keyword for main-axis content justification.
pub(crate) const PROP_JUSTIFY: &str = "justify";

/// Property keyword for flex grow weight.
pub(crate) const PROP_FLEX_GROW: &str = "flex_grow";

/// Property keyword for overflow clipping or scrolling.
pub(crate) const PROP_OVERFLOW: &str = "overflow";

/// Property keyword for font size.
pub(crate) const PROP_FONT_SIZE: &str = "font_size";

/// Property keyword for font weight.
pub(crate) const PROP_FONT_WEIGHT: &str = "font_weight";

/// Property keyword for z-index.
pub(crate) const PROP_Z_INDEX: &str = "z_index";

/// Property keyword for node opacity.
pub(crate) const PROP_OPACITY: &str = "opacity";

/// Property keyword for grid column count.
pub(crate) const PROP_COLUMNS: &str = "columns";

/// Property keyword for click callback.
pub(crate) const PROP_ON_CLICK: &str = "on_click";

/// Property keyword for input change callback.
pub(crate) const PROP_ON_CHANGE: &str = "on_change";

/// Property keyword for scroll callback.
pub(crate) const PROP_ON_SCROLL: &str = "on_scroll";

/// Property keyword for input initial value.
pub(crate) const PROP_VALUE: &str = "value";

/// Property keyword for input placeholder text.
pub(crate) const PROP_PLACEHOLDER: &str = "placeholder";

/// Property keyword for image buffer expression.
pub(crate) const PROP_SOURCE: &str = "source";

/// Property keyword for canvas painter closure.
pub(crate) const PROP_PAINTER: &str = "painter";

/// Property keyword for semantic accessibility role (`A11yRole`).
pub(crate) const PROP_ROLE: &str = "role";

/// Property keyword for accessible screen-reader label string.
pub(crate) const PROP_A11Y_LABEL: &str = "a11y_label";

/// Property keyword for accessible screen-reader usage hint string.
pub(crate) const PROP_A11Y_HINT: &str = "a11y_hint";

/// Error message for unknown property inside `view!`.
pub(crate) const ERR_UNKNOWN_VIEW_PROP: &str = "unsupported property inside view! element";

/// Error message when `image` lacks a `source` property.
pub(crate) const ERR_IMAGE_MISSING_SOURCE: &str = "image element requires a `source:` ImageBuffer";

/// Error message when `canvas` lacks a `painter` property.
pub(crate) const ERR_CANVAS_MISSING_PAINTER: &str = "canvas element requires a `painter:` closure";
