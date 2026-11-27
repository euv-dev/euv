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

/// Tag keyword for a transparent sibling fragment grouping container.
pub(crate) const TAG_FRAGMENT: &str = "fragment";

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

/// Property keyword for reactive style binding closure `move || -> Style`.
pub(crate) const PROP_BIND_STYLE: &str = "bind_style";

/// Property keyword for reactive children builder closure `move || -> Vec<NodeId>`.
pub(crate) const PROP_BIND_CHILDREN: &str = "bind_children";

/// Property keyword for capturing the created `NodeId` into a mutable local binding.
pub(crate) const PROP_NODE_REF: &str = "node_ref";

/// Property keyword for box display mode.
pub(crate) const PROP_DISPLAY: &str = "display";

/// Property keyword for positioning mode.
pub(crate) const PROP_POSITION: &str = "position";

/// Property keyword for absolute/fixed `(top, right, bottom, left)` inset tuple.
pub(crate) const PROP_INSET: &str = "inset";

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

/// Property keyword for pointer hit-test participation mode.
pub(crate) const PROP_POINTER_EVENTS: &str = "pointer_events";

/// Property keyword for blocking scroll propagation to ancestor scroll views.
pub(crate) const PROP_STOP_SCROLL_PROPAGATION: &str = "stop_scroll_propagation";

/// Property keyword for blocking pointer/click event bubbling to ancestors.
pub(crate) const PROP_STOP_EVENT_PROPAGATION: &str = "stop_event_propagation";

/// Property keyword for keyboard focus participation boolean.
pub(crate) const PROP_FOCUSABLE: &str = "focusable";

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

/// Property keyword for italic text slant.
pub(crate) const PROP_ITALIC: &str = "italic";

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

/// Property keyword for pointer-enter callback.
pub(crate) const PROP_ON_POINTER_ENTER: &str = "on_pointer_enter";

/// Property keyword for pointer-leave callback.
pub(crate) const PROP_ON_POINTER_LEAVE: &str = "on_pointer_leave";

/// Property keyword for pointer hover enter/leave callback.
pub(crate) const PROP_ON_HOVER: &str = "on_hover";

/// Property keyword for keyboard key-down callback.
pub(crate) const PROP_ON_KEY: &str = "on_key";

/// Property keyword for focus-gained callback.
pub(crate) const PROP_ON_FOCUS: &str = "on_focus";

/// Property keyword for focus-lost callback.
pub(crate) const PROP_ON_BLUR: &str = "on_blur";

/// Property keyword for input initial value.
pub(crate) const PROP_VALUE: &str = "value";

/// Property keyword for input placeholder text.
pub(crate) const PROP_PLACEHOLDER: &str = "placeholder";

/// Property keyword for input password masking boolean.
pub(crate) const PROP_PASSWORD: &str = "password";

/// Property keyword for input multiline boolean.
pub(crate) const PROP_MULTILINE: &str = "multiline";

/// Property keyword for input disabled boolean.
pub(crate) const PROP_DISABLED: &str = "disabled";

/// Property keyword for input initial caret offset.
pub(crate) const PROP_CARET: &str = "caret";

/// Property keyword for image buffer expression.
pub(crate) const PROP_SOURCE: &str = "source";

/// Property keyword for canvas painter closure.
pub(crate) const PROP_PAINTER: &str = "painter";

/// Property keyword for complete `A11yProps` expression.
pub(crate) const PROP_A11Y: &str = "a11y";

/// Property keyword for semantic accessibility role (`A11yRole`).
pub(crate) const PROP_ROLE: &str = "role";

/// Property keyword for accessible screen-reader label string.
pub(crate) const PROP_A11Y_LABEL: &str = "a11y_label";

/// Property keyword for accessible screen-reader usage hint string.
pub(crate) const PROP_A11Y_HINT: &str = "a11y_hint";

/// Property keyword for accessible current value string.
pub(crate) const PROP_A11Y_VALUE: &str = "a11y_value";

/// Property keyword for accessible disabled boolean.
pub(crate) const PROP_A11Y_DISABLED: &str = "a11y_disabled";

/// Property keyword for accessible checked state `Option<bool>`.
pub(crate) const PROP_A11Y_CHECKED: &str = "a11y_checked";

/// Property keyword for a component-level dynamic slot identifier (`slot { id: "..." }`).
pub(crate) const PROP_ID: &str = "id";

/// Property keyword for a page-level dynamic slot route (`page_slot { route: "/..." }`).
pub(crate) const PROP_ROUTE: &str = "route";

/// Property keyword for an optional explicit `DynamicUpdateHub` override on `slot` / `page_slot`.
pub(crate) const PROP_HUB: &str = "hub";

/// Property keyword for an explicit fallback builder closure `move || -> NodeId` on `slot` / `page_slot`.
pub(crate) const PROP_FALLBACK: &str = "fallback";

/// Component type name whose nested children map to `.section(...)`.
pub(crate) const COMP_EUV_PAGE: &str = "EuvPage";

/// Component type name whose nested children map to `.field(...)`.
pub(crate) const COMP_EUV_FORM: &str = "EuvForm";

/// Component type name whose nested children map to `.action(...)`.
pub(crate) const COMP_EUV_NAVBAR: &str = "EuvNavbar";

/// Component type name whose nested children map to `.card(...)`.
pub(crate) const COMP_EUV_STAT_GRID: &str = "EuvStatGrid";

/// Component fluent method name for page section children.
pub(crate) const METHOD_SECTION: &str = "section";

/// Component fluent method name for form field children.
pub(crate) const METHOD_FIELD: &str = "field";

/// Component fluent method name for navbar action children.
pub(crate) const METHOD_ACTION: &str = "action";

/// Component fluent method name for stat grid card children.
pub(crate) const METHOD_CARD: &str = "card";

/// Default component fluent method name for generic nested children.
pub(crate) const METHOD_CHILD: &str = "child";

/// Complete list of supported properties inside `view!` elements for typo diagnostics.
pub(crate) const KNOWN_VIEW_PROPS: &[&str] = &[
    PROP_STYLE,
    PROP_BIND_STYLE,
    PROP_BIND_CHILDREN,
    PROP_NODE_REF,
    PROP_DISPLAY,
    PROP_POSITION,
    PROP_INSET,
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
    PROP_POINTER_EVENTS,
    PROP_STOP_SCROLL_PROPAGATION,
    PROP_STOP_EVENT_PROPAGATION,
    PROP_FOCUSABLE,
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
    PROP_ITALIC,
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
    PROP_ON_POINTER_ENTER,
    PROP_ON_POINTER_LEAVE,
    PROP_ON_HOVER,
    PROP_ON_KEY,
    PROP_ON_FOCUS,
    PROP_ON_BLUR,
    PROP_VALUE,
    PROP_PLACEHOLDER,
    PROP_PASSWORD,
    PROP_MULTILINE,
    PROP_DISABLED,
    PROP_CARET,
    PROP_SOURCE,
    PROP_PAINTER,
    PROP_A11Y,
    PROP_ROLE,
    PROP_A11Y_LABEL,
    PROP_A11Y_HINT,
    PROP_A11Y_VALUE,
    PROP_A11Y_DISABLED,
    PROP_A11Y_CHECKED,
    PROP_ID,
    PROP_ROUTE,
    PROP_HUB,
    PROP_FALLBACK,
];

/// Error message when a caller passes a legacy explicit context argument to `view!(ctx, ...)` or `view!(in ctx, ...)`.
pub(crate) const ERR_LEGACY_VIEW_CTX: &str = "view! manages Context automatically; remove explicit `ctx` / `in ctx,` and wrap external scenes in `with_context(ctx, || view! { ... })`";

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

/// Error message when input-only properties are placed on a non-`input` element.
pub(crate) const ERR_INPUT_PROP_ONLY_ON_INPUT: &str = "`value:`, `placeholder:`, `password:`, `multiline:`, `disabled:`, and `caret:` are only valid on `input { ... }` elements";

/// Error message when child nodes are declared inside a leaf element (`input`, `image`, or `canvas`).
pub(crate) const ERR_LEAF_CHILDREN_FORBIDDEN: &str =
    "leaf elements (`input`, `image`, `canvas`) cannot contain child nodes";

/// Error message when `bind_children:` is combined with static children on the same element.
pub(crate) const ERR_BIND_CHILDREN_CONFLICT: &str = "`bind_children:` cannot be combined with inline static children because reactive updates replace all children";

/// Error message when `a11y:` is combined with individual `role:`/`a11y_*` properties.
pub(crate) const ERR_A11Y_CONFLICT: &str =
    "`a11y:` cannot be combined with individual `role:` or `a11y_*` properties on the same element";

/// Error message when `text` receives unsupported control-flow or multiple children.
pub(crate) const ERR_TEXT_CHILD_INVALID: &str =
    "`text` element accepts at most one string literal or `{ expr }` text expression";

/// Error message when `if` / `for` / `match` / `..spread` is used where a single root node is required without a container.
pub(crate) const ERR_SPREAD_IN_EXPR_CONTEXT: &str =
    "`..spread` can only appear inside a container, `fragment`, or component children block";

/// Error message prefix for duplicate property declarations on the same element.
pub(crate) const ERR_DUPLICATE_PROP_PREFIX: &str = "duplicate property";

/// Generated internal identifier for multi-root fragment container.
pub(crate) const IDENT_ROOT: &str = "__euv_root";

/// Generated internal identifier for control-flow fragment container.
pub(crate) const IDENT_FRAG: &str = "__euv_frag";

/// Generated internal identifier for mutable component builder local.
pub(crate) const IDENT_COMP: &str = "__euv_comp";

/// Generated internal identifier for newly constructed primitive node.
pub(crate) const IDENT_NODE: &str = "__euv_node";

/// Generated internal identifier for slot fallback root container.
pub(crate) const IDENT_FB_ROOT: &str = "__euv_fb_root";

/// Error message when an element tag has neither `{ ... }` body nor `(args...)`.
pub(crate) const ERR_EXPECTED_BODY_OR_ARGS: &str =
    "expected `{ ... }` body after element or component tag";

/// Error message when a PascalCase component passes positional `(...)` arguments instead of named attributes inside `{ ... }`.
pub(crate) const ERR_COMPONENT_POSITIONAL_ARGS: &str = "component parameters must be passed as named attributes inside `Component { prop: value }` instead of positional `(...)` arguments";

/// Legacy Web/HTML container tags rejected with native migration guidance.
pub(crate) const HTML_CONTAINER_TAGS: &[&str] = &[
    "div", "section", "main", "header", "footer", "nav", "article", "aside", "ul", "ol", "li",
    "form",
];

/// Legacy Web/HTML text tags rejected with native migration guidance.
pub(crate) const HTML_TEXT_TAGS: &[&str] = &[
    "span", "p", "h1", "h2", "h3", "h4", "h5", "h6", "label", "strong", "code",
];

/// Legacy Web/HTML interactive button/link tags rejected with native migration guidance.
pub(crate) const HTML_BUTTON_TAGS: &[&str] = &["button", "a"];

/// Legacy Web/HTML image tag rejected with native migration guidance.
pub(crate) const HTML_IMAGE_TAGS: &[&str] = &["img"];

/// Error message suffix for legacy HTML container tags.
pub(crate) const HTML_REPLACE_CONTAINER: &str = "`column`, `row`, `view`, or `grid`";

/// Error message suffix for legacy HTML text tags.
pub(crate) const HTML_REPLACE_TEXT: &str = "`text`";

/// Error message suffix for legacy HTML button/link tags.
pub(crate) const HTML_REPLACE_BUTTON: &str =
    "`EuvButton { label: \"...\" tokens: tokens }` or `view { on_click: ... }`";

/// Error message suffix for legacy HTML image tag.
pub(crate) const HTML_REPLACE_IMAGE: &str = "`image { source: ... }`";
