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

/// Tag keyword for a surgically updatable component-level dynamic slot.
pub(crate) const TAG_SLOT: &str = "slot";

/// Tag keyword for a surgically updatable page-level dynamic slot.
pub(crate) const TAG_PAGE_SLOT: &str = "page_slot";

/// Property keyword for base style expression.
pub(crate) const PROP_STYLE: &str = "style";

/// Property keyword for box display mode.
pub(crate) const PROP_DISPLAY: &str = "display";

/// Property keyword for positioning mode.
pub(crate) const PROP_POSITION: &str = "position";

/// Property keyword for explicit width.
pub(crate) const PROP_WIDTH: &str = "width";

/// Property keyword for explicit height.
pub(crate) const PROP_HEIGHT: &str = "height";

/// Property keyword for minimum width constraint.
pub(crate) const PROP_MIN_WIDTH: &str = "min_width";

/// Property keyword for minimum height constraint.
pub(crate) const PROP_MIN_HEIGHT: &str = "min_height";

/// Property keyword for maximum width constraint.
pub(crate) const PROP_MAX_WIDTH: &str = "max_width";

/// Property keyword for maximum height constraint.
pub(crate) const PROP_MAX_HEIGHT: &str = "max_height";

/// Property keyword for aspect ratio constraint.
pub(crate) const PROP_ASPECT_RATIO: &str = "aspect_ratio";

/// Property keyword for uniform padding.
pub(crate) const PROP_PADDING: &str = "padding";

/// Property keyword for uniform margin.
pub(crate) const PROP_MARGIN: &str = "margin";

/// Property keyword for child gap.
pub(crate) const PROP_GAP: &str = "gap";

/// Property keyword for background color.
pub(crate) const PROP_BACKGROUND: &str = "background";

/// Property keyword for linear gradient fill.
pub(crate) const PROP_GRADIENT: &str = "gradient";

/// Property keyword for drop shadow.
pub(crate) const PROP_SHADOW: &str = "shadow";

/// Property keyword for focus ring border.
pub(crate) const PROP_FOCUS_RING: &str = "focus_ring";

/// Property keyword for cursor icon.
pub(crate) const PROP_CURSOR: &str = "cursor";

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

/// Property keyword for flex shrink weight.
pub(crate) const PROP_FLEX_SHRINK: &str = "flex_shrink";

/// Property keyword for flex wrapping mode.
pub(crate) const PROP_FLEX_WRAP: &str = "flex_wrap";

/// Property keyword for overflow clipping or scrolling.
pub(crate) const PROP_OVERFLOW: &str = "overflow";

/// Property keyword for font size.
pub(crate) const PROP_FONT_SIZE: &str = "font_size";

/// Property keyword for font weight.
pub(crate) const PROP_FONT_WEIGHT: &str = "font_weight";

/// Property keyword for font family.
pub(crate) const PROP_FONT_FAMILY: &str = "font_family";

/// Property keyword for horizontal text alignment.
pub(crate) const PROP_TEXT_ALIGN: &str = "text_align";

/// Property keyword for text overflow strategy.
pub(crate) const PROP_TEXT_OVERFLOW: &str = "text_overflow";

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

/// Property keyword for pointer-down callback.
pub(crate) const PROP_ON_POINTER_DOWN: &str = "on_pointer_down";

/// Property keyword for pointer-move callback.
pub(crate) const PROP_ON_POINTER_MOVE: &str = "on_pointer_move";

/// Property keyword for pointer-up callback.
pub(crate) const PROP_ON_POINTER_UP: &str = "on_pointer_up";

/// Property keyword for pointer hover enter/leave callback.
pub(crate) const PROP_ON_HOVER: &str = "on_hover";

/// Property keyword for keyboard key-down callback.
pub(crate) const PROP_ON_KEY: &str = "on_key";

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

/// Property keyword for a component-level dynamic slot identifier (`slot { id: "..." }`).
pub(crate) const PROP_ID: &str = "id";

/// Property keyword for a page-level dynamic slot route (`page_slot { route: "/..." }`).
pub(crate) const PROP_ROUTE: &str = "route";

/// Property keyword for an optional explicit `DynamicUpdateHub` override on `slot` / `page_slot`.
pub(crate) const PROP_HUB: &str = "hub";

/// Property keyword for an explicit fallback builder closure `move || -> NodeId` on `slot` / `page_slot`.
pub(crate) const PROP_FALLBACK: &str = "fallback";

/// Complete list of supported properties inside `view!` elements for typo diagnostics.
pub(crate) const KNOWN_VIEW_PROPS: &[&str] = &[
    PROP_STYLE,
    PROP_DISPLAY,
    PROP_POSITION,
    PROP_WIDTH,
    PROP_HEIGHT,
    PROP_MIN_WIDTH,
    PROP_MIN_HEIGHT,
    PROP_MAX_WIDTH,
    PROP_MAX_HEIGHT,
    PROP_ASPECT_RATIO,
    PROP_PADDING,
    PROP_MARGIN,
    PROP_GAP,
    PROP_BACKGROUND,
    PROP_GRADIENT,
    PROP_SHADOW,
    PROP_FOCUS_RING,
    PROP_CURSOR,
    PROP_COLOR,
    PROP_RADIUS,
    PROP_BORDER,
    PROP_ALIGN,
    PROP_JUSTIFY,
    PROP_FLEX_GROW,
    PROP_FLEX_SHRINK,
    PROP_FLEX_WRAP,
    PROP_OVERFLOW,
    PROP_FONT_SIZE,
    PROP_FONT_WEIGHT,
    PROP_FONT_FAMILY,
    PROP_TEXT_ALIGN,
    PROP_TEXT_OVERFLOW,
    PROP_Z_INDEX,
    PROP_OPACITY,
    PROP_COLUMNS,
    PROP_ON_CLICK,
    PROP_ON_CHANGE,
    PROP_ON_SCROLL,
    PROP_ON_POINTER_DOWN,
    PROP_ON_POINTER_MOVE,
    PROP_ON_POINTER_UP,
    PROP_ON_HOVER,
    PROP_ON_KEY,
    PROP_VALUE,
    PROP_PLACEHOLDER,
    PROP_SOURCE,
    PROP_PAINTER,
    PROP_ROLE,
    PROP_A11Y_LABEL,
    PROP_A11Y_HINT,
    PROP_ID,
    PROP_ROUTE,
    PROP_HUB,
    PROP_FALLBACK,
];

/// Error message when a caller passes a legacy explicit context argument to `view!(ctx, ...)`.
pub(crate) const ERR_LEGACY_VIEW_CTX: &str = "view! manages BuildContext automatically; remove the explicit context argument and write `view! { ... }`";

/// Error message when `image` lacks a `source` property.
pub(crate) const ERR_IMAGE_MISSING_SOURCE: &str =
    "image element requires a `source:` ImageBuffer expression";

/// Error message when `canvas` lacks a `painter` property.
pub(crate) const ERR_CANVAS_MISSING_PAINTER: &str =
    "canvas element requires a `painter:` closure `|bounds: Rect, dl: &mut DisplayList| { ... }`";

/// Error message when `slot` lacks an `id` property.
pub(crate) const ERR_SLOT_MISSING_ID: &str =
    "`slot` element requires an `id:` string identifying the dynamic component slot";

/// Error message when `page_slot` lacks a `route` property.
pub(crate) const ERR_PAGE_SLOT_MISSING_ROUTE: &str =
    "`page_slot` element requires a `route:` string identifying the dynamic page route";

/// Error message when `id:` is placed on a non-`slot` element.
pub(crate) const ERR_ID_ONLY_ON_SLOT: &str =
    "`id:` is only valid on `slot { id: \"...\" }` dynamic component slots";

/// Error message when `route:` is placed on a non-`page_slot` element.
pub(crate) const ERR_ROUTE_ONLY_ON_PAGE_SLOT: &str =
    "`route:` is only valid on `page_slot { route: \"/...\" }` dynamic page slots";

/// Error message when `hub:` or `fallback:` is placed on an element other than `slot` or `page_slot`.
pub(crate) const ERR_HUB_ONLY_ON_DYNAMIC_SLOT: &str =
    "`hub:` and `fallback:` are only valid on `slot` or `page_slot` dynamic elements";

/// Error message when `source:` is placed on a non-`image` element.
pub(crate) const ERR_SOURCE_ONLY_ON_IMAGE: &str =
    "`source:` is only valid on `image { source: ... }` elements";

/// Error message when `painter:` is placed on a non-`canvas` element.
pub(crate) const ERR_PAINTER_ONLY_ON_CANVAS: &str =
    "`painter:` is only valid on `canvas { painter: ... }` elements";

/// Error message when `value:` or `placeholder:` is placed on a non-`input` element.
pub(crate) const ERR_INPUT_PROP_ONLY_ON_INPUT: &str =
    "`value:` and `placeholder:` are only valid on `input { ... }` elements";
