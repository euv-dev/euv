use super::*;

/// Creates a click event handler that enters fullscreen drawing mode.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
pub(crate) fn canvas_on_draw(state: UseCanvas) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_fullscreen(state);
    }))
}

/// Creates a click event handler that clears the canvas drawing.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
pub(crate) fn canvas_on_clear(_state: UseCanvas) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        clear_canvas(CANVAS_DRAWING_SELECTOR);
    }))
}

/// Creates a click event handler that exits fullscreen drawing mode.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
pub(crate) fn canvas_on_exit_fullscreen(state: UseCanvas) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_fullscreen(state);
    }))
}

/// Creates the canvas drawing board reactive state signals wrapped
/// in a `UseCanvas` struct.
///
/// Initializes with default stroke color, line width, and an empty
/// snapshot data URL.
///
/// # Returns
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn use_canvas_state() -> UseCanvas {
    let initial_stroke_color: String = load_stroke_color();
    let initial_line_width: f64 = load_line_width();
    UseCanvas {
        drawing: App::use_signal(|| false),
        stroke_color: App::use_signal(move || initial_stroke_color.clone()),
        line_width: App::use_signal(move || initial_line_width),
        fullscreen: App::use_signal(|| false),
        snapshot_data_url: App::use_signal(String::new),
        last_x: App::use_signal(|| 0.0),
        last_y: App::use_signal(|| 0.0),
        touch_last_points: App::use_signal(HashMap::new),
    }
}

/// Captures the current canvas content as a data URL and stores it
/// in the `snapshot_data_url` signal.
///
/// Must be called while the canvas element is still in the DOM.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn update_snapshot(state: UseCanvas) {
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let data_url: String = canvas_element.to_data_url().unwrap_or_default();
    state.get_snapshot_data_url().set(data_url);
}

/// Begins a new drawing stroke on the canvas at the specified coordinates.
///
/// Sets the `drawing` signal to `true`, retrieves the 2D rendering context,
/// and starts a new path at the given point. Applies the current stroke
/// color and line width from the state signals. The line width is clamped
/// to a minimum of `CANVAS_MIN_LINE_WIDTH` to prevent rendering artifacts.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `f64` - The x coordinate of the starting point.
/// - `f64` - The y coordinate of the starting point.
pub(crate) fn start_drawing(state: UseCanvas, offset_x: f64, offset_y: f64) {
    state.get_drawing().set(true);
    state.get_last_x().set(offset_x);
    state.get_last_y().set(offset_y);
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element
        .get_context(CANVAS_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    CanvasRenderer::enable_smoothing_on(&context_2d);
    context_2d.begin_path();
    let _: Result<bool, JsValue> = Reflect::set(
        &context_2d,
        &JsValue::from_str(CANVAS_CONTEXT_PROPERTY_STROKE_STYLE),
        &JsValue::from_str(&state.get_stroke_color().get()),
    );
    let line_width: f64 = state.get_line_width().get().max(CANVAS_MIN_LINE_WIDTH);
    context_2d.set_line_width(line_width);
    context_2d.set_line_cap(CANVAS_LINE_CAP_ROUND);
    context_2d.set_line_join(CANVAS_LINE_JOIN_ROUND);
    context_2d.move_to(offset_x, offset_y);
    context_2d.line_to(offset_x, offset_y);
    context_2d.stroke();
}

/// Continues the current drawing stroke to the specified coordinates.
///
/// If the `drawing` signal is `true`, draws a line segment from the
/// current path position to the given point and strokes it.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `f64` - The x coordinate of the destination point.
/// - `f64` - The y coordinate of the destination point.
pub(crate) fn continue_drawing(state: UseCanvas, offset_x: f64, offset_y: f64) {
    if !state.get_drawing().get() {
        return;
    }
    let prev_x: f64 = state.get_last_x().get();
    let prev_y: f64 = state.get_last_y().get();
    state.get_last_x().set(offset_x);
    state.get_last_y().set(offset_y);
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element
        .get_context(CANVAS_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    CanvasRenderer::enable_smoothing_on(&context_2d);
    let _: Result<bool, JsValue> = Reflect::set(
        &context_2d,
        &JsValue::from_str(CANVAS_CONTEXT_PROPERTY_STROKE_STYLE),
        &JsValue::from_str(&state.get_stroke_color().get()),
    );
    let line_width: f64 = state.get_line_width().get().max(CANVAS_MIN_LINE_WIDTH);
    context_2d.set_line_width(line_width);
    context_2d.set_line_cap(CANVAS_LINE_CAP_ROUND);
    context_2d.set_line_join(CANVAS_LINE_JOIN_ROUND);
    context_2d.begin_path();
    context_2d.move_to(prev_x, prev_y);
    context_2d.line_to(offset_x, offset_y);
    context_2d.stroke();
}

/// Ends the current drawing stroke.
///
/// Sets the `drawing` signal to `false` and closes the current path.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn stop_drawing(state: UseCanvas) {
    state.get_drawing().set(false);
}

/// Clears the entire canvas and refills it with the white background color.
///
/// Retrieves the canvas element, clears the entire drawing area, and
/// applies the background fill. The canvas dimensions are preserved.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the `<canvas>` element to clear.
pub(crate) fn clear_canvas(canvas_selector: &str) {
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Some(element) = document_value
        .query_selector(canvas_selector)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let width: f64 = canvas_element.width() as f64;
    let height: f64 = canvas_element.height() as f64;
    let Some(context_object) = canvas_element
        .get_context(CANVAS_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    context_2d.clear_rect(0.0, 0.0, width, height);
    let _: Result<bool, JsValue> = Reflect::set(
        &context_2d,
        &JsValue::from_str(CANVAS_CONTEXT_PROPERTY_FILL_STYLE),
        &JsValue::from_str(CANVAS_BACKGROUND_COLOR),
    );
    context_2d.fill_rect(0.0, 0.0, width, height);
}

/// Extracts the pointer offset coordinates relative to the canvas element
/// from a mouse or touch event.
///
/// Reads `offsetX` and `offsetY` properties from the event via JavaScript
/// reflection. These properties provide coordinates relative to the target
/// element, which is ideal for canvas drawing.
///
/// # Arguments
///
/// - `&Event` - The mouse or touch event.
///
/// # Returns
///
/// - `(f64, f64)` - A tuple containing the `(offset_x, offset_y)` coordinates.
pub(crate) fn get_pointer_offset(event: &Event) -> (f64, f64) {
    let target: JsValue = event
        .target()
        .map_or(JsValue::NULL, |event_target: EventTarget| {
            event_target.into()
        });
    let offset_x: f64 = Reflect::get(&target, &JsValue::from_str(CANVAS_EVENT_PROPERTY_OFFSET_X))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    let offset_y: f64 = Reflect::get(&target, &JsValue::from_str(CANVAS_EVENT_PROPERTY_OFFSET_Y))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    (offset_x, offset_y)
}

/// Extracts the client coordinates from a mouse event.
///
/// Reads `clientX` and `clientY` properties from the event, which
/// represent the coordinates within the viewport. These are used
/// in fullscreen mode where CSS transforms make `offsetX`/`offsetY`
/// unreliable.
///
/// # Arguments
///
/// - `&Event` - The mouse event.
///
/// # Returns
///
/// - `(f64, f64)` - A tuple containing the `(client_x, client_y)` coordinates.
pub(crate) fn get_mouse_client(event: &Event) -> (f64, f64) {
    let client_x: f64 = Reflect::get(
        event.as_ref(),
        &JsValue::from_str(CANVAS_EVENT_PROPERTY_CLIENT_X),
    )
    .ok()
    .and_then(|value: JsValue| value.as_f64())
    .unwrap_or_default();
    let client_y: f64 = Reflect::get(
        event.as_ref(),
        &JsValue::from_str(CANVAS_EVENT_PROPERTY_CLIENT_Y),
    )
    .ok()
    .and_then(|value: JsValue| value.as_f64())
    .unwrap_or_default();
    (client_x, client_y)
}

/// Begins new drawing strokes for all active touch points.
///
/// Each touch point starts its own independent path on the canvas,
/// enabling simultaneous multi-finger drawing. The stroke color and
/// line width are applied from the current state signals.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `&Event` - The touchstart event.
/// - `bool` - Whether the canvas is in fullscreen mode.
pub(crate) fn start_drawing_multi_touch(state: UseCanvas, event: &Event, is_fullscreen: bool) {
    let points: Vec<NativeTouchPointF64> = NativeTouchPointF64::extract_all(event);
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element
        .get_context(CANVAS_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    CanvasRenderer::enable_smoothing_on(&context_2d);
    let canvas_rect: DomRect = canvas_element.get_bounding_client_rect();
    let stroke_color: String = state.get_stroke_color().get();
    let line_width: f64 = state.get_line_width().get().max(CANVAS_MIN_LINE_WIDTH);
    let _: Result<bool, JsValue> = Reflect::set(
        &context_2d,
        &JsValue::from_str(CANVAS_CONTEXT_PROPERTY_STROKE_STYLE),
        &JsValue::from_str(&stroke_color),
    );
    context_2d.set_line_width(line_width);
    context_2d.set_line_cap(CANVAS_LINE_CAP_ROUND);
    context_2d.set_line_join(CANVAS_LINE_JOIN_ROUND);
    state.get_drawing().set(true);
    let mut touch_last: HashMap<i32, (f64, f64)> = state.get_touch_last_points().get();
    for point in &points {
        let (mapped_x, mapped_y): (f64, f64) = if is_fullscreen {
            (
                point.get_client_x() - canvas_rect.left(),
                point.get_client_y() - canvas_rect.top(),
            )
        } else {
            (point.get_offset_x(), point.get_offset_y())
        };
        touch_last.insert(point.get_identifier(), (mapped_x, mapped_y));
        context_2d.begin_path();
        context_2d.move_to(mapped_x, mapped_y);
        context_2d.line_to(mapped_x, mapped_y);
        context_2d.stroke();
    }
    state.get_touch_last_points().set(touch_last);
}

/// Continues drawing strokes for all active touch points.
///
/// Each touch point draws a line segment from its current path
/// position to the new position, enabling simultaneous multi-finger
/// drawing.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `&Event` - The touchmove event.
/// - `bool` - Whether the canvas is in fullscreen mode.
pub(crate) fn continue_drawing_multi_touch(state: UseCanvas, event: &Event, is_fullscreen: bool) {
    if !state.get_drawing().get() {
        return;
    }
    let points: Vec<NativeTouchPointF64> = NativeTouchPointF64::extract_all(event);
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element
        .get_context(CANVAS_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    CanvasRenderer::enable_smoothing_on(&context_2d);
    let canvas_rect: DomRect = canvas_element.get_bounding_client_rect();
    let _: Result<bool, JsValue> = Reflect::set(
        &context_2d,
        &JsValue::from_str(CANVAS_CONTEXT_PROPERTY_STROKE_STYLE),
        &JsValue::from_str(&state.get_stroke_color().get()),
    );
    let line_width: f64 = state.get_line_width().get().max(CANVAS_MIN_LINE_WIDTH);
    context_2d.set_line_width(line_width);
    context_2d.set_line_cap(CANVAS_LINE_CAP_ROUND);
    context_2d.set_line_join(CANVAS_LINE_JOIN_ROUND);
    let mut touch_last: HashMap<i32, (f64, f64)> = state.get_touch_last_points().get();
    for point in &points {
        let (mapped_x, mapped_y): (f64, f64) = if is_fullscreen {
            (
                point.get_client_x() - canvas_rect.left(),
                point.get_client_y() - canvas_rect.top(),
            )
        } else {
            (point.get_offset_x(), point.get_offset_y())
        };
        let identifier: i32 = point.get_identifier();
        let (prev_x, prev_y): (f64, f64) = touch_last
            .get(&identifier)
            .copied()
            .unwrap_or((mapped_x, mapped_y));
        touch_last.insert(identifier, (mapped_x, mapped_y));
        context_2d.begin_path();
        context_2d.move_to(prev_x, prev_y);
        context_2d.line_to(mapped_x, mapped_y);
        context_2d.stroke();
    }
    state.get_touch_last_points().set(touch_last);
}

/// Ends drawing strokes for the touch points that were lifted.
///
/// Uses `changedTouches` to identify which specific fingers were
/// lifted, and closes only those paths. If no touches remain,
/// stops the overall drawing state.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `&Event` - The touchend or touchcancel event.
pub(crate) fn stop_drawing_multi_touch(state: UseCanvas, event: &Event) {
    let changed_points: Vec<NativeTouchPoint> = NativeTouchPoint::extract_changed(event);
    let remaining: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(event);
    let mut touch_last: HashMap<i32, (f64, f64)> = state.get_touch_last_points().get();
    for point in &changed_points {
        touch_last.remove(&point.get_identifier());
    }
    state.get_touch_last_points().set(touch_last);
    if remaining.is_empty() {
        state.get_drawing().set(false);
    }
}

/// Prevents the default browser behavior for the given event.
///
/// Used to prevent scrolling and other default touch behaviors
/// during canvas drawing.
///
/// # Arguments
///
/// - `&Event` - The event to prevent default on.
pub(crate) fn prevent_event_default(event: &Event) {
    event.prevent_default();
}

/// Maps screen coordinates to canvas-internal coordinates.
///
/// In fullscreen mode, calculates the position relative to the canvas
/// bounding rectangle using `clientX`/`clientY`. In normal mode,
/// returns `offsetX`/`offsetY` directly.
///
/// # Arguments
///
/// - `f64` - The `offsetX` value from the event (used in normal mode).
/// - `f64` - The `offsetY` value from the event (used in normal mode).
/// - `f64` - The `clientX` value from the event (used in fullscreen mode).
/// - `f64` - The `clientY` value from the event (used in fullscreen mode).
/// - `bool` - Whether the canvas is in fullscreen mode.
///
/// # Returns
///
/// - `(f64, f64)` - The mapped `(x, y)` coordinates in canvas space.
pub(crate) fn map_rotated_offset(
    offset_x: f64,
    offset_y: f64,
    client_x: f64,
    client_y: f64,
    is_fullscreen: bool,
) -> (f64, f64) {
    if !is_fullscreen {
        return (offset_x, offset_y);
    }
    let Some(window_value): Option<Window> = window() else {
        return (offset_x, offset_y);
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return (offset_x, offset_y);
    };
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return (offset_x, offset_y);
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let rect: DomRect = canvas_element.get_bounding_client_rect();
    let canvas_x: f64 = client_x - rect.left();
    let canvas_y: f64 = client_y - rect.top();
    (canvas_x, canvas_y)
}

/// Enters CSS fullscreen mode by setting the `fullscreen` signal to
/// `true` and pushing a browser history entry via `overlay_push_state`
/// so that the system back button will exit fullscreen instead of
/// navigating to the previous page.
///
/// After the DOM updates, the canvas is resized and initialized with
/// the content from `snapshot_data_url` (if any).
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn enter_fullscreen(state: UseCanvas) {
    state.get_fullscreen().set(true);
    Router::overlay_push_state();
    let snapshot_data_url: String = state.get_snapshot_data_url().get();
    let resize_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        resize_fullscreen_canvas(&snapshot_data_url);
    }));
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let _: Result<i32, JsValue> =
        window_value.request_animation_frame(resize_closure.as_ref().unchecked_ref());
    resize_closure.forget();
}

/// Calculates and applies the optimal pixel dimensions for the canvas
/// in fullscreen mode, then draws the snapshot content directly.
///
/// The canvas pixel buffer and CSS display size are both set to 9:16
/// (portrait) so that the canvas visually fills the portrait drawing
/// area without any CSS rotation. Drawing coordinates are in portrait
/// space. Since the snapshot shares the same portrait orientation,
/// it is drawn directly without any rotation transform.
///
/// # Arguments
///
/// - `&str` - The snapshot data URL to draw onto the canvas.
pub(crate) fn resize_fullscreen_canvas(snapshot_data_url: &str) {
    UseEuvLayout::apply_cached_insets();
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let device_pixel_ratio: f64 = Reflect::get(
        window_value.as_ref(),
        &JsValue::from_str(CANVAS_EVENT_PROPERTY_DEVICE_PIXEL_RATIO),
    )
    .ok()
    .and_then(|value: JsValue| value.as_f64())
    .unwrap_or(1.0);
    let Some(wrapper_element) = document_value
        .query_selector(CANVAS_FULLSCREEN_WRAPPER_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let Some(canvas_element_obj) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = canvas_element_obj.unchecked_into();
    let wrapper_width: i32 = wrapper_element.client_width();
    let wrapper_height: i32 = wrapper_element.client_height();
    let canvas_width: f64 = if (wrapper_height as f64) < (wrapper_width as f64) * 16.0 / 9.0 {
        wrapper_height as f64 * 9.0 / 16.0
    } else {
        wrapper_width as f64
    };
    let canvas_height: f64 = canvas_width * 16.0 / 9.0;
    canvas_element
        .style()
        .set_property(
            CANVAS_STYLE_PROPERTY_WIDTH,
            &format!("{}{}", canvas_width as i32, CANVAS_PIXEL_UNIT),
        )
        .unwrap_or(());
    canvas_element
        .style()
        .set_property(
            CANVAS_STYLE_PROPERTY_HEIGHT,
            &format!("{}{}", canvas_height as i32, CANVAS_PIXEL_UNIT),
        )
        .unwrap_or(());
    canvas_element.set_width((canvas_width * device_pixel_ratio) as u32);
    canvas_element.set_height((canvas_height * device_pixel_ratio) as u32);
    let Some(context_object) = canvas_element
        .get_context(CANVAS_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    CanvasRenderer::enable_smoothing_on(&context_2d);
    context_2d
        .scale(device_pixel_ratio, device_pixel_ratio)
        .unwrap_or(());
    let _: Result<bool, JsValue> = Reflect::set(
        &context_2d,
        &JsValue::from_str(CANVAS_CONTEXT_PROPERTY_FILL_STYLE),
        &JsValue::from_str(CANVAS_BACKGROUND_COLOR),
    );
    context_2d.fill_rect(0.0, 0.0, canvas_width, canvas_height);
    if snapshot_data_url.is_empty() {
        return;
    }
    let Ok(image): Result<HtmlImageElement, JsValue> = HtmlImageElement::new() else {
        return;
    };
    image.set_src(snapshot_data_url);
    let draw_image: HtmlImageElement = image.clone();
    let draw_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let _: Result<(), JsValue> = context_2d.draw_image_with_html_image_element_and_dw_and_dh(
            &draw_image,
            0.0,
            0.0,
            canvas_width,
            canvas_height,
        );
    }));
    image.set_onload(Some(draw_closure.as_ref().unchecked_ref()));
    draw_closure.forget();
}

/// Exits CSS fullscreen mode by capturing the canvas content as a
/// snapshot data URL, then setting the `fullscreen` signal to `false`
/// and consuming the browser history entry via `overlay_back`.
///
/// The canvas content is captured before the fullscreen signal changes
/// so the DOM element is still available for data extraction.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn exit_fullscreen(state: UseCanvas) {
    update_snapshot(state);
    state.get_fullscreen().set(false);
    UseEuvLayout::apply_cached_insets();
    Router::overlay_back(None);
}

/// Exits CSS fullscreen mode without consuming a browser history entry.
///
/// Used when the exit is triggered by a `popstate` event (system back
/// button), because the `popstate` itself has already consumed the
/// `pushState` entry that was created when entering fullscreen.
/// Calling `history.back()` again would incorrectly consume an extra
/// history entry.
///
/// The canvas content is captured before the fullscreen signal changes
/// so the DOM element is still available for data extraction.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn exit_fullscreen_from_popstate(state: UseCanvas) {
    update_snapshot(state);
    state.get_fullscreen().set(false);
    UseEuvLayout::apply_cached_insets();
}

/// Subscribes to browser `popstate` events to handle the system back
/// button while the canvas is in fullscreen mode.
///
/// When the user presses the system back button while in fullscreen,
/// this handler exits fullscreen instead of allowing the browser to
/// navigate to the previous page. The `popstate` event is triggered
/// by the `pushState` entry that was created when entering fullscreen.
/// Uses `exit_fullscreen_from_popstate` to avoid double-consuming
/// history entries.
///
///   Loads the persisted stroke color from localStorage.
///
/// Returns the stored color string if available and non-empty,
/// otherwise returns the default stroke color.
///
/// # Returns
///
/// - `String` - The persisted or default stroke color.
pub(crate) fn load_stroke_color() -> String {
    UseEuvBrowser::local_storage_get(CANVAS_STORAGE_KEY_STROKE_COLOR)
        .filter(|color: &String| !color.is_empty())
        .unwrap_or_else(|| CANVAS_DEFAULT_STROKE_COLOR.to_string())
}

/// Loads the persisted line width from localStorage.
///
/// Returns the stored line width if available and parseable,
/// otherwise returns the default line width.
///
/// # Returns
///
/// - `f64` - The persisted or default line width.
pub(crate) fn load_line_width() -> f64 {
    UseEuvBrowser::local_storage_get(CANVAS_STORAGE_KEY_LINE_WIDTH)
        .and_then(|width: String| width.parse::<f64>().ok())
        .unwrap_or(CANVAS_DEFAULT_LINE_WIDTH)
}

/// Persists the current stroke color to localStorage.
///
/// # Arguments
///
/// - `&str` - The stroke color value to persist.
pub(crate) fn save_stroke_color(color: &str) {
    UseEuvBrowser::local_storage_set(CANVAS_STORAGE_KEY_STROKE_COLOR, color);
}

/// Persists the current line width to localStorage.
///
/// # Arguments
///
/// - `f64` - The line width value to persist.
pub(crate) fn save_line_width(width: f64) {
    UseEuvBrowser::local_storage_set(CANVAS_STORAGE_KEY_LINE_WIDTH, width.to_string());
}

/// Creates an input event handler that updates the line width via
/// `requestAnimationFrame` throttling to ensure at most one signal
/// update per paint frame.
///
/// Instead of updating the signal on every `oninput` event (which can
/// fire many times per frame), stores the pending value and schedules
/// a single `requestAnimationFrame` callback. The callback reads the
/// latest pending value and applies it exactly once per paint frame,
/// then persists the value to localStorage.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the line width slider.
pub(crate) fn canvas_on_line_width_input(state: UseCanvas) -> Option<Rc<dyn Fn(Event)>> {
    let pending_value: Rc<Cell<f64>> = Rc::new(Cell::new(CANVAS_DEFAULT_LINE_WIDTH));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    Some(Rc::new(move |event: Event| {
        let new_width: f64 = Reflect::get(
            event.as_ref(),
            &JsValue::from_str(CANVAS_EVENT_PROPERTY_TARGET),
        )
        .ok()
        .and_then(|target: JsValue| {
            Reflect::get(&target, &JsValue::from_str(CANVAS_EVENT_PROPERTY_VALUE)).ok()
        })
        .and_then(|value: JsValue| value.as_string())
        .and_then(|string: String| string.parse::<f64>().ok())
        .unwrap_or(CANVAS_DEFAULT_LINE_WIDTH);
        if let Some(target) = event.target()
            && let Ok(input) = target.dyn_into::<HtmlInputElement>()
        {
            let min: f64 = input.min().parse::<f64>().unwrap_or(1.0);
            let max: f64 = input.max().parse::<f64>().unwrap_or(30.0);
            let range: f64 = max - min;
            let percent: f64 = if range > 0.0 {
                ((new_width - min) / range * 100.0).clamp(0.0, 100.0)
            } else {
                0.0
            };
            input
                .style()
                .set_property("--value", &format!("{percent}%"))
                .unwrap_or(());
        }
        pending_value.set(new_width);
        if raf_id.get().is_some() {
            return;
        }
        let pending_for_raf: Rc<Cell<f64>> = pending_value.clone();
        let raf_id_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let line_width_signal: Signal<f64> = state.get_line_width();
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            raf_id_clone.set(None);
            let current_width: f64 = pending_for_raf.get();
            line_width_signal.set(current_width);
            save_line_width(current_width);
        }));
        let id: i32 = window_value
            .request_animation_frame(raf_closure.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_id.set(Some(id));
        raf_closure.forget();
    }))
}

/// Registers a `popstate` guard that exits fullscreen drawing mode when the
/// system back button is pressed.
///
/// When the user presses the browser back button while the canvas page is in
/// fullscreen drawing mode, this guard exits fullscreen so the history entry
/// is consumed without leaving the UI in an inconsistent state. Returns `true`
/// to consume the `popstate` event only when the canvas was in fullscreen;
/// otherwise returns `false` to let other guards or the overlay stack handle it.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas state containing the fullscreen signal.
///
/// # Returns
///
/// - `usize` - A guard ID that can be passed to [`Router::unregister_popstate_guard`]
///   to remove the guard when the canvas page is unmounted.
pub(crate) fn use_fullscreen_popstate(state: UseCanvas) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_fullscreen().get() {
            exit_fullscreen_from_popstate(state);
            true
        } else {
            false
        }
    }))
}
