use super::*;

/// Creates keyboard event state signals wrapped in a `UseKeyboardEvent` struct.
///
/// # Returns
///
/// - `UseKeyboardEvent` - The keyboard event demo state.
pub(crate) fn use_keyboard_event() -> UseKeyboardEvent {
    let mut state: UseKeyboardEvent = UseKeyboardEvent::default();
    state.set_last_key(App::use_signal(|| "None".to_string()));
    state.set_last_key_code(App::use_signal(|| "None".to_string()));
    state.set_last_key_up(App::use_signal(|| "None".to_string()));
    state.set_key_repeat(App::use_signal(|| false));
    state.set_modifier(App::use_signal(|| "None".to_string()));
    state
}

/// Creates mouse event state signals wrapped in a `UseMouseEvent` struct.
///
/// # Returns
///
/// - `UseMouseEvent` - The mouse event demo state.
pub(crate) fn use_mouse_event() -> UseMouseEvent {
    let mut state: UseMouseEvent = UseMouseEvent::default();
    state.set_click_count(App::use_signal(|| 0));
    state.set_double_click_count(App::use_signal(|| 0));
    state.set_mouse_pos(App::use_signal(|| "(0, 0)".to_string()));
    state.set_mouse_screen_pos(App::use_signal(|| "(0, 0)".to_string()));
    state.set_mouse_button(App::use_signal(|| "None".to_string()));
    state.set_mouse_buttons(App::use_signal(|| "0".to_string()));
    state.set_mouse_enter_count(App::use_signal(|| 0));
    state.set_mouse_leave_count(App::use_signal(|| 0));
    state.set_mouse_over_count(App::use_signal(|| 0));
    state.set_mouse_out_count(App::use_signal(|| 0));
    state.set_mouse_down_count(App::use_signal(|| 0));
    state.set_mouse_up_count(App::use_signal(|| 0));
    state
}

/// Creates focus event state signals wrapped in a `UseFocusEvent` struct.
///
/// # Returns
///
/// - `UseFocusEvent` - The focus event demo state.
pub(crate) fn use_focus_event() -> UseFocusEvent {
    let mut state: UseFocusEvent = UseFocusEvent::default();
    state.set_focus_status(App::use_signal(|| "Not focused".to_string()));
    state.set_focus_in_count(App::use_signal(|| 0));
    state.set_focus_out_count(App::use_signal(|| 0));
    state
}

/// Creates drag event state signals wrapped in a `UseDragEvent` struct.
///
/// # Returns
///
/// - `UseDragEvent` - The drag event demo state.
pub(crate) fn use_drag_event() -> UseDragEvent {
    let mut state: UseDragEvent = UseDragEvent::default();
    state.set_drag_status(App::use_signal(|| "Idle".to_string()));
    state.set_drag_pos(App::use_signal(|| "(-, -)".to_string()));
    state.set_drag_types(App::use_signal(|| "None".to_string()));
    state.set_drag_enter_counter(App::use_signal(|| 0));
    state.set_drag_pending_pos(App::use_signal(String::new));
    state.set_drag_raf_id(App::use_signal(|| -1));
    state
}

/// Creates wheel event state signals wrapped in a `UseWheelEvent` struct.
///
/// # Returns
///
/// - `UseWheelEvent` - The wheel event demo state.
pub(crate) fn use_wheel_event() -> UseWheelEvent {
    let mut state: UseWheelEvent = UseWheelEvent::default();
    state.set_wheel_delta(App::use_signal(|| "(0, 0)".to_string()));
    state.set_wheel_total(App::use_signal(|| 0.0));
    state
}

/// Creates clipboard event state signals wrapped in a `UseClipboardEvent` struct.
///
/// # Returns
///
/// - `UseClipboardEvent` - The clipboard event demo state.
pub(crate) fn use_clipboard_event() -> UseClipboardEvent {
    let mut state: UseClipboardEvent = UseClipboardEvent::default();
    state.set_clipboard_data(App::use_signal(|| "None".to_string()));
    state.set_clipboard_event_type(App::use_signal(|| "None".to_string()));
    state
}

/// Creates touch event state signals wrapped in a `UseTouchEvent` struct.
///
/// # Returns
///
/// - `UseTouchEvent` - The touch event demo state.
pub(crate) fn use_touch_event() -> UseTouchEvent {
    let mut state: UseTouchEvent = UseTouchEvent::default();
    state.set_touch_info(App::use_signal(|| "No touch".to_string()));
    state
}

/// Creates form event state signals wrapped in a `UseFormEvent` struct.
///
/// # Returns
///
/// - `UseFormEvent` - The form event demo state.
pub(crate) fn use_form_event() -> UseFormEvent {
    let mut state: UseFormEvent = UseFormEvent::default();
    state.set_euv_input_value(App::use_signal(String::new));
    state.set_form_change_value(App::use_signal(|| "None".to_string()));
    state.set_form_checkbox(App::use_signal(|| false));
    state.set_form_select_value(App::use_signal(|| "None".to_string()));
    state.set_submit_count(App::use_signal(|| 0));
    state
}

/// Creates audio media event state signals wrapped in a `UseMediaEvent` struct.
///
/// # Returns
///
/// - `UseMediaEvent` - The audio media event demo state.
pub(crate) fn use_media_event() -> UseMediaEvent {
    let mut state: UseMediaEvent = UseMediaEvent::default();
    state.set_media_status(App::use_signal(|| "Not started".to_string()));
    state.set_media_event_log(App::use_signal(|| "None".to_string()));
    state
}

/// Creates video event state signals wrapped in a `UseVideoEvent` struct.
///
/// # Returns
///
/// - `UseVideoEvent` - The video event demo state.
pub(crate) fn use_video_event() -> UseVideoEvent {
    let mut state: UseVideoEvent = UseVideoEvent::default();
    state.set_video_status(App::use_signal(|| "Not loaded".to_string()));
    state.set_video_event_log(App::use_signal(|| "None".to_string()));
    state.set_video_current_time(App::use_signal(|| "0.00".to_string()));
    state.set_video_duration(App::use_signal(|| "0.00".to_string()));
    state.set_video_buffered(App::use_signal(|| "0%".to_string()));
    state.set_video_playback_rate(App::use_signal(|| "1.0".to_string()));
    state
}

/// Creates image event state signals wrapped in a `UseImageEvent` struct.
///
/// # Returns
///
/// - `UseImageEvent` - The image event demo state.
pub(crate) fn use_image_event() -> UseImageEvent {
    let mut state: UseImageEvent = UseImageEvent::default();
    state.set_image_status(App::use_signal(|| "Not loaded".to_string()));
    state.set_image_event_log(App::use_signal(|| "None".to_string()));
    state.set_image_natural_size(App::use_signal(|| "N/A".to_string()));
    state
}

/// Reads the current browser URL without query parameters and hash fragment.
///
/// Constructs the full address from `origin` + `pathname` only,
/// stripping any `?search` and `#hash` parts.
///
/// # Returns
///
/// - `String` - The current complete URL without parameters and hash.
pub(crate) fn current_url_without_params() -> String {
    let Some(window): Option<Window> = window() else {
        return String::new();
    };
    let location: Location = window.location();
    let origin: String = location
        .origin()
        .unwrap_or_else(|_error: JsValue| "Unknown".to_string());
    let pathname: String = location
        .pathname()
        .unwrap_or_else(|_error: JsValue| "/".to_string());
    format!("{origin}{pathname}")
}

/// URL-encodes special characters in a string for safe embedding in a data URL.
///
/// Replaces `%`, `#`, `"`, `'`, `<`, `>`, `&`, `{`, and `}` with their
/// percent-encoded equivalents.
///
/// # Arguments
///
/// - `&str` - The raw string to encode.
///
/// # Returns
///
/// - `String` - The URL-encoded string.
fn encode_svg_for_data_url(raw: &str) -> String {
    raw.chars()
        .map(|character: char| match character {
            character if SVG_ESCAPED_CHARS.contains(&character) => {
                format!("%{:02X}", character as u8)
            }
            _ => character.to_string(),
        })
        .collect()
}

/// Generates a QR code SVG data URL for the given content string.
///
/// Encodes the provided string into a QR code using the `qrcode` crate
/// with SVG rendering, then wraps the SVG XML into a `data:image/svg+xml`
/// URI suitable for use as an `img` `src` attribute.
///
/// # Arguments
///
/// - `&str` - The content to encode into the QR code.
///
/// # Returns
///
/// - `String` - A URL-encoded SVG data URL of the QR code.
pub(crate) fn generate_qr_code_data_url(content: &str) -> String {
    let initial_result: Result<QrCode, QrError> = QrCode::new(content);
    let code: QrCode = match initial_result {
        Ok(qr) => qr,
        Err(_) => match QrCode::new("error") {
            Ok(qr) => qr,
            Err(_) => return String::new(),
        },
    };
    let svg_string: String = code
        .render::<svg::Color>()
        .min_dimensions(QR_CODE_MIN_DIMENSION, QR_CODE_MIN_DIMENSION)
        .quiet_zone(true)
        .build();
    let encoded_svg: String = encode_svg_for_data_url(&svg_string);
    format!("{SVG_DATA_URL_PREFIX}{encoded_svg}")
}
