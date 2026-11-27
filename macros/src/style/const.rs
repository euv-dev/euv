/// Style property key for flex row/column direction.
pub(crate) const STYLE_KEY_DIRECTION: &str = "direction";

/// Direction value for horizontal flex row.
pub(crate) const STYLE_VAL_ROW: &str = "row";

/// Direction value for vertical flex column.
pub(crate) const STYLE_VAL_COLUMN: &str = "column";

/// Style property key for display mode.
pub(crate) const STYLE_KEY_DISPLAY: &str = "display";

/// Style property key for position mode.
pub(crate) const STYLE_KEY_POSITION: &str = "position";

/// Style property key for absolute/fixed `(top, right, bottom, left)` inset tuple.
pub(crate) const STYLE_KEY_INSET: &str = "inset";

/// Style property key for width.
pub(crate) const STYLE_KEY_WIDTH: &str = "width";

/// Style property key for height.
pub(crate) const STYLE_KEY_HEIGHT: &str = "height";

/// Style property key for minimum width constraint.
pub(crate) const STYLE_KEY_MIN_WIDTH: &str = "min_width";

/// Style property key for minimum height constraint.
pub(crate) const STYLE_KEY_MIN_HEIGHT: &str = "min_height";

/// Style property key for maximum width constraint.
pub(crate) const STYLE_KEY_MAX_WIDTH: &str = "max_width";

/// Style property key for maximum height constraint.
pub(crate) const STYLE_KEY_MAX_HEIGHT: &str = "max_height";

/// Style property key for aspect ratio constraint.
pub(crate) const STYLE_KEY_ASPECT_RATIO: &str = "aspect_ratio";

/// Style property key for padding.
pub(crate) const STYLE_KEY_PADDING: &str = "padding";

/// Style property key for margin.
pub(crate) const STYLE_KEY_MARGIN: &str = "margin";

/// Style property key for gap.
pub(crate) const STYLE_KEY_GAP: &str = "gap";

/// Style property key for background color.
pub(crate) const STYLE_KEY_BACKGROUND: &str = "background";

/// Style property key for linear gradient fill.
pub(crate) const STYLE_KEY_GRADIENT: &str = "gradient";

/// Style property key for drop shadow.
pub(crate) const STYLE_KEY_SHADOW: &str = "shadow";

/// Style property key for focus ring border.
pub(crate) const STYLE_KEY_FOCUS_RING: &str = "focus_ring";

/// Style property key for cursor icon.
pub(crate) const STYLE_KEY_CURSOR: &str = "cursor";

/// Style property key for pointer hit-test participation mode.
pub(crate) const STYLE_KEY_POINTER_EVENTS: &str = "pointer_events";

/// Style property key for blocking scroll propagation.
pub(crate) const STYLE_KEY_STOP_SCROLL_PROPAGATION: &str = "stop_scroll_propagation";

/// Style property key for blocking pointer/click event bubbling.
pub(crate) const STYLE_KEY_STOP_EVENT_PROPAGATION: &str = "stop_event_propagation";

/// Style property key for foreground color.
pub(crate) const STYLE_KEY_COLOR: &str = "color";

/// Style property key for corner radius.
pub(crate) const STYLE_KEY_RADIUS: &str = "radius";

/// Style property key for cross-axis item alignment.
pub(crate) const STYLE_KEY_ALIGN: &str = "align";

/// Style property key for main-axis content justification.
pub(crate) const STYLE_KEY_JUSTIFY: &str = "justify";

/// Style property key for flex grow weight.
pub(crate) const STYLE_KEY_FLEX_GROW: &str = "flex_grow";

/// Style property key for flex shrink weight.
pub(crate) const STYLE_KEY_FLEX_SHRINK: &str = "flex_shrink";

/// Style property key for flex wrap mode.
pub(crate) const STYLE_KEY_FLEX_WRAP: &str = "flex_wrap";

/// Style property key for overflow clipping or scrolling.
pub(crate) const STYLE_KEY_OVERFLOW: &str = "overflow";

/// Style property key for font size.
pub(crate) const STYLE_KEY_FONT_SIZE: &str = "font_size";

/// Style property key for font weight.
pub(crate) const STYLE_KEY_FONT_WEIGHT: &str = "font_weight";

/// Style property key for font family.
pub(crate) const STYLE_KEY_FONT_FAMILY: &str = "font_family";

/// Style property key for italic text slant.
pub(crate) const STYLE_KEY_ITALIC: &str = "italic";

/// Style property key for horizontal text alignment.
pub(crate) const STYLE_KEY_TEXT_ALIGN: &str = "text_align";

/// Style property key for text overflow clipping/ellipsis.
pub(crate) const STYLE_KEY_TEXT_OVERFLOW: &str = "text_overflow";

/// Style property key for enabling or disabling native text selection.
pub(crate) const STYLE_KEY_SELECTABLE: &str = "selectable";

/// Style property key for text selection highlight color.
pub(crate) const STYLE_KEY_SELECTION_COLOR: &str = "selection_color";

/// Style property key for opacity.
pub(crate) const STYLE_KEY_OPACITY: &str = "opacity";

/// Style property key for z-index.
pub(crate) const STYLE_KEY_Z_INDEX: &str = "z_index";

/// Style property key for border `(width, color)`.
pub(crate) const STYLE_KEY_BORDER: &str = "border";

/// Style property key for grid column count.
pub(crate) const STYLE_KEY_GRID: &str = "grid";

/// Alias style property key for grid column count.
pub(crate) const STYLE_KEY_COLUMNS: &str = "columns";

/// Error message prefix for duplicate style rule declarations.
pub(crate) const ERR_STYLE_DUPLICATE_KEY: &str = "duplicate style property";

/// Error message when a `style!` rule contains a disallowed semicolon.
pub(crate) const ERR_STYLE_UNEXPECTED_SEMI: &str =
    "unexpected `;` in `style!` — place each style rule on its own line without semicolons";

/// Error message when two `style!` statements appear on the same source line.
pub(crate) const ERR_STYLE_NEWLINE_REQUIRED: &str =
    "each style rule in `style!` must start on a new line";

/// Exhaustive list of supported properties inside `style!` blocks for typo diagnostics.
pub(crate) const KNOWN_STYLE_PROPS: &[&str] = &[
    STYLE_KEY_DIRECTION,
    STYLE_KEY_DISPLAY,
    STYLE_KEY_POSITION,
    STYLE_KEY_INSET,
    STYLE_KEY_WIDTH,
    STYLE_KEY_HEIGHT,
    STYLE_KEY_MIN_WIDTH,
    STYLE_KEY_MIN_HEIGHT,
    STYLE_KEY_MAX_WIDTH,
    STYLE_KEY_MAX_HEIGHT,
    STYLE_KEY_ASPECT_RATIO,
    STYLE_KEY_PADDING,
    STYLE_KEY_MARGIN,
    STYLE_KEY_GAP,
    STYLE_KEY_BACKGROUND,
    STYLE_KEY_GRADIENT,
    STYLE_KEY_SHADOW,
    STYLE_KEY_FOCUS_RING,
    STYLE_KEY_CURSOR,
    STYLE_KEY_POINTER_EVENTS,
    STYLE_KEY_STOP_SCROLL_PROPAGATION,
    STYLE_KEY_STOP_EVENT_PROPAGATION,
    STYLE_KEY_COLOR,
    STYLE_KEY_RADIUS,
    STYLE_KEY_ALIGN,
    STYLE_KEY_JUSTIFY,
    STYLE_KEY_FLEX_GROW,
    STYLE_KEY_FLEX_SHRINK,
    STYLE_KEY_FLEX_WRAP,
    STYLE_KEY_OVERFLOW,
    STYLE_KEY_FONT_SIZE,
    STYLE_KEY_FONT_WEIGHT,
    STYLE_KEY_FONT_FAMILY,
    STYLE_KEY_ITALIC,
    STYLE_KEY_TEXT_ALIGN,
    STYLE_KEY_TEXT_OVERFLOW,
    STYLE_KEY_SELECTABLE,
    STYLE_KEY_SELECTION_COLOR,
    STYLE_KEY_OPACITY,
    STYLE_KEY_Z_INDEX,
    STYLE_KEY_BORDER,
    STYLE_KEY_GRID,
    STYLE_KEY_COLUMNS,
];
