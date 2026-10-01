/// The CSS selector used to query the drawing canvas element from the DOM.
pub(crate) const CANVAS_DRAWING_SELECTOR: &str = "#drawing-canvas";

/// The CSS selector used to query the fullscreen canvas drawing wrapper.
pub(crate) const CANVAS_FULLSCREEN_WRAPPER_SELECTOR: &str = "#fullscreen-canvas-wrapper";

/// The default stroke color for drawing.
pub(crate) const CANVAS_DEFAULT_STROKE_COLOR: &str = "#000000";

/// The default line width for drawing in pixels.
pub(crate) const CANVAS_DEFAULT_LINE_WIDTH: f64 = 3.0;

/// The canvas background fill color.
pub(crate) const CANVAS_BACKGROUND_COLOR: &str = "#ffffff";

/// The minimum allowed line width for drawing.
pub(crate) const CANVAS_MIN_LINE_WIDTH: f64 = 1.0;

/// The localStorage key for persisting the stroke color.
pub(crate) const CANVAS_STORAGE_KEY_STROKE_COLOR: &str = "euv-canvas-stroke-color";

/// The localStorage key for persisting the line width.
pub(crate) const CANVAS_STORAGE_KEY_LINE_WIDTH: &str = "euv-canvas-line-width";

/// The canvas 2D rendering context type identifier.
pub(crate) const CANVAS_CONTEXT_TYPE: &str = "2d";

/// The JavaScript property name for the device pixel ratio.
pub(crate) const CANVAS_EVENT_PROPERTY_DEVICE_PIXEL_RATIO: &str = "devicePixelRatio";

/// The JavaScript property name for the canvas stroke style.
pub(crate) const CANVAS_CONTEXT_PROPERTY_STROKE_STYLE: &str = "strokeStyle";

/// The JavaScript property name for the canvas fill style.
pub(crate) const CANVAS_CONTEXT_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The line cap style for smooth stroke endings.
pub(crate) const CANVAS_LINE_CAP_ROUND: &str = "round";

/// The line join style for smooth stroke corners.
pub(crate) const CANVAS_LINE_JOIN_ROUND: &str = "round";

/// The JavaScript property name for the event target element.
pub(crate) const CANVAS_EVENT_PROPERTY_TARGET: &str = "target";

/// The JavaScript property name for the input element value.
pub(crate) const CANVAS_EVENT_PROPERTY_VALUE: &str = "value";

/// The JavaScript property name for the pointer offset X coordinate.
pub(crate) const CANVAS_EVENT_PROPERTY_OFFSET_X: &str = "offsetX";

/// The JavaScript property name for the pointer offset Y coordinate.
pub(crate) const CANVAS_EVENT_PROPERTY_OFFSET_Y: &str = "offsetY";

/// The JavaScript property name for the client X coordinate.
pub(crate) const CANVAS_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate.
pub(crate) const CANVAS_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// The CSS style property name for width.
pub(crate) const CANVAS_STYLE_PROPERTY_WIDTH: &str = "width";

/// The CSS style property name for height.
pub(crate) const CANVAS_STYLE_PROPERTY_HEIGHT: &str = "height";

/// The pixel unit suffix for CSS property values.
pub(crate) const CANVAS_PIXEL_UNIT: &str = "px";
