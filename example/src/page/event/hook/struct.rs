use super::*;

/// Reactive state for the keyboard events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseKeyboardEvent {
    /// The last key pressed down.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_key: Signal<String>,
    /// The last key code pressed down.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_key_code: Signal<String>,
    /// The last key released up.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_key_up: Signal<String>,
    /// Whether the key press is a repeat.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) key_repeat: Signal<bool>,
    /// The active modifier keys string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) modifier: Signal<String>,
}

/// Reactive state for the mouse events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseMouseEvent {
    /// The total click count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) click_count: Signal<i32>,
    /// The total double-click count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) double_click_count: Signal<i32>,
    /// The client position string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_pos: Signal<String>,
    /// The screen position string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_screen_pos: Signal<String>,
    /// The mouse button name.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_button: Signal<String>,
    /// The mouse buttons bitmask string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_buttons: Signal<String>,
    /// The mouse enter count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_enter_count: Signal<i32>,
    /// The mouse leave count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_leave_count: Signal<i32>,
    /// The mouse over count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_over_count: Signal<i32>,
    /// The mouse out count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_out_count: Signal<i32>,
    /// The mouse down count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_down_count: Signal<i32>,
    /// The mouse up count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mouse_up_count: Signal<i32>,
}

/// Reactive state for the focus events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseFocusEvent {
    /// The current focus status string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) focus_status: Signal<String>,
    /// The focus-in count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) focus_in_count: Signal<i32>,
    /// The focus-out count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) focus_out_count: Signal<i32>,
}

/// Reactive state for the drag events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseDragEvent {
    /// The current drag status string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) drag_status: Signal<String>,
    /// The drag position string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) drag_pos: Signal<String>,
    /// The drag data types string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) drag_types: Signal<String>,
    /// The counter used to track whether the cursor is inside the drop zone.
    ///
    /// Incremented on `dragenter`, decremented on `dragleave`.
    /// Non-zero means the cursor is inside the zone, avoiding false
    /// triggers when moving between child elements.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) drag_enter_counter: Signal<i32>,
    /// The pending drag position to be flushed on the next animation frame.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) drag_pending_pos: Signal<String>,
    /// The request-animation-frame ID used for throttling drag position updates.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) drag_raf_id: Signal<i32>,
}

/// Reactive state for the wheel event demo.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub(crate) struct UseWheelEvent {
    /// The wheel delta string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) wheel_delta: Signal<String>,
    /// The accumulated vertical scroll total.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) wheel_total: Signal<f64>,
}

/// Reactive state for the clipboard events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseClipboardEvent {
    /// The clipboard data content.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clipboard_data: Signal<String>,
    /// The clipboard event type name.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clipboard_event_type: Signal<String>,
}

/// Reactive state for the touch events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseTouchEvent {
    /// The touch information string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) touch_info: Signal<String>,
}

/// Reactive state for the form events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseFormEvent {
    /// The input value from oninput.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) euv_input_value: Signal<String>,
    /// The change value from onchange.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) form_change_value: Signal<String>,
    /// The checkbox checked state.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) form_checkbox: Signal<bool>,
    /// The select value from onchange.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) form_select_value: Signal<String>,
    /// The form submit count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) submit_count: Signal<i32>,
}

/// Reactive state for the audio media events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseMediaEvent {
    /// The media playback status.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) media_status: Signal<String>,
    /// The last media event log.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) media_event_log: Signal<String>,
}

/// Reactive state for the video events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseVideoEvent {
    /// The video playback status.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) video_status: Signal<String>,
    /// The last video event log.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) video_event_log: Signal<String>,
    /// The current playback time string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) video_current_time: Signal<String>,
    /// The video duration string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) video_duration: Signal<String>,
    /// The buffered percentage string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) video_buffered: Signal<String>,
    /// The playback rate string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) video_playback_rate: Signal<String>,
}

/// Reactive state for the image events demo.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseImageEvent {
    /// The image load status.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image_status: Signal<String>,
    /// The last image event log.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image_event_log: Signal<String>,
    /// The image natural size string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image_natural_size: Signal<String>,
}
