use super::*;

/// The glyph drawn on the trailing edge of the trigger.
const CARET_GLYPH: &str = "\u{25be}";

/// The label of the button that selects the current time.
const NOW_LABEL: &str = "Now";

/// The label of the button that empties the field.
const CLEAR_LABEL: &str = "Clear";

/// The zero-padded labels of the hours and of the minutes.
///
/// The labels never change, so they are written out once instead of being
/// formatted again for every cell of every render.
const TIME_LABELS: [&str; 60] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
    "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
];

/// Parses a `HH:MM` time.
///
/// A text that does not have that shape, or that names an hour or a minute
/// outside the day, is rejected rather than silently repaired.
///
/// # Arguments
///
/// - `&str` - The text to parse.
///
/// # Returns
///
/// - `Option<(u32, u32)>` - The hour and the minute, or `None` when the text
///   does not name a valid time.
pub fn parse_time(text: &str) -> Option<(u32, u32)> {
    let trimmed: &str = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts = trimmed.split(':');
    let hour_text: &str = parts.next()?;
    let minute_text: &str = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let hour: u32 = parse_clock_part(hour_text)?;
    let minute: u32 = parse_clock_part(minute_text)?;
    if hour > 23 || minute > 59 {
        return None;
    }
    Some((hour, minute))
}

/// Parses one part of a clock time.
///
/// A part is one or two decimal digits; anything else — a sign, a letter, a
/// longer number — is refused.
///
/// # Arguments
///
/// - `&str` - The text of the part.
///
/// # Returns
///
/// - `Option<u32>` - The number the part names, or `None` when the text is
///   not a clock part.
fn parse_clock_part(text: &str) -> Option<u32> {
    if text.is_empty() || text.len() > 2 {
        return None;
    }
    if !text.bytes().all(|byte: u8| byte.is_ascii_digit()) {
        return None;
    }
    text.parse::<u32>().ok()
}

/// Formats an hour and a minute as `HH:MM`.
///
/// # Arguments
///
/// - `u32` - The hour of the day, `0` through `23`.
///
/// # Returns
///
/// - `String` - The formatted time.
pub fn format_time(hour: u32, minute: u32) -> String {
    format!("{hour:02}:{minute:02}")
}

/// Clamps the hour bounds into a range the picker can offer.
///
/// Both bounds are clamped into the day, and a range whose bounds arrive in
/// the wrong order is flipped, so the picker always offers at least one hour.
///
/// # Arguments
///
/// - `u32` - The first hour the caller asked for.
///
/// # Returns
///
/// - `(u32, u32)` - The first and the last hour of the offered range.
pub fn clamp_hour_range(min_hour: u32, max_hour: u32) -> (u32, u32) {
    let low: u32 = min_hour.min(23);
    let high: u32 = max_hour.min(23);
    if low <= high {
        return (low, high);
    }
    (high, low)
}

/// Builds the hours the picker offers.
///
/// # Arguments
///
/// - `u32` - The first hour the caller asked for.
///
/// # Returns
///
/// - `Vec<u32>` - The offered hours, in display order.
pub fn hour_values(min_hour: u32, max_hour: u32) -> Vec<u32> {
    let (low, high) = clamp_hour_range(min_hour, max_hour);
    (low..=high).collect()
}

/// Builds the minutes the picker offers.
///
/// The minutes run from zero to fifty-nine in steps of the given size, so a
/// step of fifteen offers `0`, `15`, `30` and `45`. A step of zero is treated
/// as a step of one minute.
///
/// # Arguments
///
/// - `u32` - The step between two offered minutes.
///
/// # Returns
///
/// - `Vec<u32>` - The offered minutes, in display order.
pub fn minute_values(minute_step: u32) -> Vec<u32> {
    let step: u32 = minute_step.max(1);
    let mut minutes: Vec<u32> = Vec::with_capacity((60 / step) as usize + 1);
    let mut minute: u32 = 0;
    while minute < 60 {
        minutes.push(minute);
        minute = minute.saturating_add(step);
    }
    minutes
}

/// Returns the time that results from picking an hour.
///
/// The minutes of the current value are kept; a value that does not name a
/// time falls back to zero minutes.
///
/// # Arguments
///
/// - `&str` - The current value, formatted as `HH:MM`.
/// - `u32` - The picked hour.
///
/// # Returns
///
/// - `String` - The new time, formatted as `HH:MM`.
pub fn time_with_hour(current: &str, hour: u32) -> String {
    let minute: u32 = parse_time(current)
        .map(|(_, minute): (u32, u32)| minute)
        .unwrap_or(0);
    format_time(hour, minute)
}

/// Returns the time that results from picking a minute.
///
/// The hour of the current value is kept; a value that does not name a time
/// falls back to the given hour.
///
/// # Arguments
///
/// - `&str` - The current value, formatted as `HH:MM`.
/// - `u32` - The picked minute.
///   time.
///
/// # Returns
///
/// - `String` - The new time, formatted as `HH:MM`.
pub fn time_with_minute(current: &str, minute: u32, fallback_hour: u32) -> String {
    let hour: u32 = parse_time(current)
        .map(|(hour, _): (u32, u32)| hour)
        .unwrap_or(fallback_hour);
    format_time(hour, minute)
}

/// Returns the current time of the host.
///
/// The browser's clock is read on the web. A host build has no clock the
/// browser way, so it reports that no time is available and the "Now" button
/// stays inert there.
///
/// # Returns
///
/// - `Option<(u32, u32)>` - The hour and the minute of the current time, or
///   `None` when the host has no browser clock.
pub fn current_time() -> Option<(u32, u32)> {
    #[cfg(target_arch = "wasm32")]
    {
        let now: js_sys::Date = js_sys::Date::new_0();
        Some((now.get_hours(), now.get_minutes()))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Builds the click handler that opens and closes the panel.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<bool>` - The open state of the panel.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The toggle click handler.
pub fn time_picker_toggle_handler<E: 'static>(open: Signal<bool>) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_event: E| {
        let is_open: bool = open.get();
        open.set(!is_open);
    }))
}

/// Builds the click handler that picks an hour.
///
/// The handler writes the new time into the selection signal, reports it to
/// the change callback and closes the panel. The minutes of the current value
/// are kept.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the selected time.
/// - `Signal<bool>` - The open state of the panel.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the time.
/// - `u32` - The picked hour.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The pick click handler.
pub fn time_picker_pick_hour_handler<E: 'static>(
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
    hour: u32,
) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_event: E| {
        let current: String = value.get();
        let formatted: String = time_with_hour(&current, hour);
        let leaked: &'static str = Box::leak(formatted.clone().into_boxed_str());
        value.set(formatted);
        open.set(false);
        if let Some(callback) = &on_change {
            callback(leaked);
        }
    }))
}

/// Builds the click handler that picks a minute.
///
/// The handler writes the new time into the selection signal, reports it to
/// the change callback and closes the panel. The hour of the current value is
/// kept.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the selected time.
/// - `Signal<bool>` - The open state of the panel.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the time.
/// - `u32` - The picked minute.
///   time.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The pick click handler.
pub fn time_picker_pick_minute_handler<E: 'static>(
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
    minute: u32,
    fallback_hour: u32,
) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_event: E| {
        let current: String = value.get();
        let formatted: String = time_with_minute(&current, minute, fallback_hour);
        let leaked: &'static str = Box::leak(formatted.clone().into_boxed_str());
        value.set(formatted);
        open.set(false);
        if let Some(callback) = &on_change {
            callback(leaked);
        }
    }))
}

/// Builds the click handler of the "Now" button.
///
/// The handler selects the current time and closes the panel. A host build
/// has no browser clock, so the handler does nothing there.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the selected time.
/// - `Signal<bool>` - The open state of the panel.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the time.
///
/// # Returns
///
/// - `Rc<dyn Fn(E)>` - The click handler of the button.
pub fn time_picker_now_handler<E: 'static>(
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> Rc<dyn Fn(E)> {
    Rc::new(move |_event: E| {
        let Some((hour, minute)) = current_time() else {
            return;
        };
        let formatted: String = format_time(hour, minute);
        let leaked: &'static str = Box::leak(formatted.clone().into_boxed_str());
        value.set(formatted);
        open.set(false);
        if let Some(callback) = &on_change {
            callback(leaked);
        }
    })
}

/// Builds the click handler of the "Clear" button.
///
/// The handler empties the field and closes the panel.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the selected time.
/// - `Signal<bool>` - The open state of the panel.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the time.
///
/// # Returns
///
/// - `Rc<dyn Fn(E)>` - The click handler of the button.
pub fn time_picker_clear_handler<E: 'static>(
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> Rc<dyn Fn(E)> {
    Rc::new(move |_event: E| {
        value.set(String::new());
        open.set(false);
        if let Some(callback) = &on_change {
            callback("");
        }
    })
}

/// Renders one choice cell of a column.
///
/// The chosen cell carries the `c_euv_time_picker_cell_active` modifier on
/// top of the base `c_euv_time_picker_cell` class, the way the calendar
/// stacks its `_today` and `_selected` modifiers on a day cell.
///
/// The label is written as a borrowed text node, so a cell costs no
/// allocation for its own text.
///
/// # Arguments
///
/// - `&'static str` - The label of the cell.
/// - `bool` - Whether the cell is the chosen one.
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the cell.
///
/// # Returns
///
/// - `VirtualNode` - The cell button.
fn time_picker_cell(
    label: &'static str,
    is_active: bool,
    handler: Option<Rc<dyn Fn(Event)>>,
) -> VirtualNode {
    let cell_text: VirtualNode = VirtualNode::Text(TextNode::new(Cow::Borrowed(label), None));
    html! {
        button {
            type: "button"
            class: c_euv_time_picker_cell()
            class: if is_active {
                c_euv_time_picker_cell_active()
            } else {
                c_euv_time_picker_cell()
            }
            onclick: handler
            cell_text
        }
    }
}

/// A time-of-day picker with an always-mounted panel.
///
/// The component renders a `c_euv_time_picker` wrapper holding a
/// `c_euv_time_picker_trigger` button and the panel. The trigger shows the
/// selected time, or the placeholder while nothing is selected, and opens the
/// panel; the panel carries a grid of hours and minutes and a footer with a
/// "Now" and a "Clear" button.
///
/// The panel is always mounted: opening and closing it is a reactive class
/// swap between `c_euv_time_picker_panel_open` and
/// `c_euv_time_picker_panel_closed`, so the cells never have to be rebuilt
/// while the user looks at them.
///
/// The selected time lives in a caller-owned [`Signal`] formatted as `HH:MM`,
/// so the picker takes part in two-way binding. Picking an hour keeps the
/// minutes of the current value and picking a minute keeps its hour; either
/// way the new time is written into the signal, reported to the change
/// callback and the panel closes.
///
/// # Arguments
///
/// - `VirtualNode<EuvTimePickerProps>` - The props node carrying the selected
///   time, the offered hours, the minute step and the change callback.
///
/// # Returns
///
/// - `VirtualNode` - The time field with its hour and minute panel.
#[component]
pub fn euv_time_picker(node: VirtualNode<EuvTimePickerProps>) -> VirtualNode {
    let EuvTimePickerProps {
        value,
        placeholder,
        label: label_text,
        min_hour,
        max_hour,
        minute_step,
        on_change,
    }: EuvTimePickerProps = node.try_get_props().unwrap_or_default();
    let open: Signal<bool> = HookContext::use_hook(|| Signal::create(false));
    let selected_value: String = value.get();
    let has_value: bool = !selected_value.is_empty();
    let selected: Option<(u32, u32)> = parse_time(&selected_value);
    let trigger_text: String = if has_value {
        selected_value
    } else {
        String::from(placeholder)
    };
    let first_hour: u32 = clamp_hour_range(min_hour, max_hour).0;
    let hours: Vec<u32> = hour_values(min_hour, max_hour);
    let minutes: Vec<u32> = minute_values(minute_step);
    let selected_hour: Option<u32> = selected.map(|(hour, _): (u32, u32)| hour);
    let selected_minute: Option<u32> = selected.map(|(_, minute): (u32, u32)| minute);
    let hour_cells: Vec<VirtualNode> = hours
        .iter()
        .map(|hour: &u32| {
            time_picker_cell(
                TIME_LABELS[*hour as usize],
                selected_hour == Some(*hour),
                time_picker_pick_hour_handler(value, open, on_change.clone(), *hour),
            )
        })
        .collect();
    let minute_cells: Vec<VirtualNode> = minutes
        .iter()
        .map(|minute: &u32| {
            time_picker_cell(
                TIME_LABELS[*minute as usize],
                selected_minute == Some(*minute),
                time_picker_pick_minute_handler(
                    value,
                    open,
                    on_change.clone(),
                    *minute,
                    first_hour,
                ),
            )
        })
        .collect();
    html! {
        div {
            class: c_euv_time_picker()
            if !label_text.is_empty() {
                label {
                    class: c_form_label()
                    label_text
                }
            }
            button {
                type: "button"
                class: c_euv_time_picker_trigger()
                onclick: time_picker_toggle_handler(open)
                span {
                    class: if has_value {
                        c_euv_time_picker_value()
                    } else {
                        c_euv_time_picker_placeholder()
                    }
                    trigger_text
                }
                span {
                    class: c_euv_time_picker_caret()
                    CARET_GLYPH
                }
            }
            div {
                class: c_euv_time_picker_panel()
                class: if { open } {
                    c_euv_time_picker_panel_open()
                } else {
                    c_euv_time_picker_panel_closed()
                }
                div {
                    class: c_euv_time_picker_grid()
                    div {
                        class: c_euv_time_picker_column()
                        hour_cells
                    }
                    div {
                        class: c_euv_time_picker_column()
                        minute_cells
                    }
                }
                div {
                    class: c_euv_time_picker_footer()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: NOW_LABEL
                        onclick: Some(time_picker_now_handler(value, open, on_change.clone()))
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: CLEAR_LABEL
                        onclick: Some(time_picker_clear_handler(value, open, on_change.clone()))
                    }
                }
            }
        }
    }
}
