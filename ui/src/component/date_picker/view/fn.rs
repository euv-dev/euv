use super::*;

/// The number of cells in the day grid: six weeks of seven days.
///
/// The grid always has this shape so the panel does not change height when
/// the user pages from a five-week month into a six-week one.
const GRID_CELLS: u32 = 42;

/// The number of days in a second.
#[cfg(not(target_arch = "wasm32"))]
const SECONDS_PER_DAY: i64 = 86_400;

/// The glyph drawn on the previous-month chevron.
const PREVIOUS_MONTH_GLYPH: &str = "\u{2039}";

/// The glyph drawn on the next-month chevron.
const NEXT_MONTH_GLYPH: &str = "\u{203a}";

/// The glyph drawn on the trailing edge of the trigger.
const CARET_GLYPH: &str = "\u{25be}";

/// The label of the button that selects today's date.
const TODAY_LABEL: &str = "Today";

/// The label of the button that empties the field.
const CLEAR_LABEL: &str = "Clear";

/// Converts a civil date into the number of days since the Unix epoch.
///
/// The algorithm is Howard Hinnant's `days_from_civil`, which is exact for
/// the proleptic Gregorian calendar and needs no leap-year special cases.
///
/// # Arguments
///
/// - `i64` - The year.
/// - `u32` - The month, `1` through `12`.
///
/// # Returns
///
/// - `i64` - The number of days between the epoch and the given date.
pub fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let shifted_year: i64 = if month <= 2 { year - 1 } else { year };
    let era: i64 = if shifted_year >= 0 {
        shifted_year
    } else {
        shifted_year - 399
    } / 400;
    let year_of_era: i64 = shifted_year - era * 400;
    let shifted_month: i64 = if month > 2 {
        month as i64 - 3
    } else {
        month as i64 + 9
    };
    let day_of_year: i64 = (153 * shifted_month + 2) / 5 + day as i64 - 1;
    let day_of_era: i64 = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// Converts a number of days since the Unix epoch into a civil date.
///
/// The algorithm is Howard Hinnant's `civil_from_days`, the exact inverse of
/// [`days_from_civil`].
///
/// # Arguments
///
/// - `i64` - The number of days between the epoch and the wanted date.
///
/// # Returns
///
/// - `(i64, u32, u32)` - The year, the month and the day of the month.
pub fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let shifted: i64 = days + 719_468;
    let era: i64 = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era: i64 = shifted - era * 146_097;
    let year_of_era: i64 =
        (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146_096) / 365;
    let year: i64 = year_of_era + era * 400;
    let day_of_year: i64 = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month: i64 = (5 * day_of_year + 2) / 153;
    let day: u32 = (day_of_year - (153 * shifted_month + 2) / 5 + 1) as u32;
    let month: u32 = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// Returns the weekday of a date, counting Monday as `0`.
///
/// The epoch itself was a Thursday, which is index `3` in a Monday-first
/// week.
///
/// # Arguments
///
/// - `i64` - The year.
/// - `u32` - The month, `1` through `12`.
///
/// # Returns
///
/// - `u32` - The weekday index, `0` for Monday through `6` for Sunday.
pub fn weekday_index(year: i64, month: u32, day: u32) -> u32 {
    ((days_from_civil(year, month, day) + 3).rem_euclid(7)) as u32
}

/// Returns whether a year is a leap year in the Gregorian calendar.
///
/// # Arguments
///
/// - `i64` - The year.
///
/// # Returns
///
/// - `bool` - `true` when the year has a 29th of February.
pub fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Returns the number of days in a month.
///
/// # Arguments
///
/// - `i64` - The year.
/// - `u32` - The month, `1` through `12`.
///
/// # Returns
///
/// - `u32` - The length of the month, or `0` for a month outside the year.
pub fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Returns the month that follows or precedes the given one.
///
/// # Arguments
///
/// - `i64` - The year of the month.
/// - `u32` - The month, `1` through `12`.
///
/// # Returns
///
/// - `(i64, u32)` - The year and the month that was walked to.
pub fn adjacent_month(year: i64, month: u32, delta: i64) -> (i64, u32) {
    let index: i64 = year * 12 + month as i64 - 1 + delta;
    (index.div_euclid(12), (index.rem_euclid(12) + 1) as u32)
}

/// Encodes a year and a month into a single number the month signal holds.
///
/// # Arguments
///
/// - `i64` - The year.
/// - `u32` - The month, `1` through `12`.
///
/// # Returns
///
/// - `i64` - The encoded month.
pub fn encode_month(year: i64, month: u32) -> i64 {
    year * 12 + month as i64 - 1
}

/// Decodes a month signal back into a year and a month.
///
/// # Arguments
///
/// - `i64` - The encoded month.
///
/// # Returns
///
/// - `(i64, u32)` - The year and the month.
pub fn decode_month(code: i64) -> (i64, u32) {
    (code.div_euclid(12), (code.rem_euclid(12) + 1) as u32)
}

/// Formats a date as `YYYY-MM-DD`.
///
/// # Arguments
///
/// - `i64` - The year.
/// - `u32` - The month, `1` through `12`.
///
/// # Returns
///
/// - `String` - The formatted date.
pub fn format_date(year: i64, month: u32, day: u32) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

/// Parses a `YYYY-MM-DD` date.
///
/// A date that does not have that shape, or that names a day the month does
/// not have, is rejected rather than silently repaired.
///
/// # Arguments
///
/// - `&str` - The text to parse.
///
/// # Returns
///
/// - `Option<(i64, u32, u32)>` - The year, the month and the day, or `None`
///   when the text is not a valid date.
pub fn parse_date(text: &str) -> Option<(i64, u32, u32)> {
    let trimmed: &str = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts = trimmed.split('-');
    let year_text: &str = parts.next()?;
    let month_text: &str = parts.next()?;
    let day_text: &str = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let year: i64 = year_text.parse().ok()?;
    let month: u32 = month_text.parse().ok()?;
    let day: u32 = day_text.parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    if day < 1 || day > days_in_month(year, month) {
        return None;
    }
    Some((year, month, day))
}

/// Returns whether a date sits inside the inclusive bounds.
///
/// An empty or unparsable bound means that side is not bounded at all.
///
/// # Arguments
///
/// - `(i64, u32, u32)` - The date under test.
/// - `&str` - The earliest selectable date, formatted as `YYYY-MM-DD`.
///
/// # Returns
///
/// - `bool` - `true` when the date may be selected.
pub fn date_within_bounds(date: (i64, u32, u32), min: &str, max: &str) -> bool {
    if let Some(minimum) = parse_date(min)
        && date < minimum
    {
        return false;
    }
    if let Some(maximum) = parse_date(max)
        && date > maximum
    {
        return false;
    }
    true
}

/// Builds the day grid of a month.
///
/// The grid always holds [`GRID_CELLS`] cells: the days of the month in the
/// middle, the trailing days of the previous month in front of them and the
/// leading days of the next month behind them. The spill-over cells are
/// marked as muted so the caller can tell them apart.
///
/// # Arguments
///
/// - `i64` - The year.
/// - `u32` - The month, `1` through `12`.
/// - `(i64, u32, u32)` - The current date, used to mark today's cell.
///
/// # Returns
///
/// - `Vec<EuvCalendarDay>` - The cells of the grid, in display order.
pub fn month_grid(year: i64, month: u32, today: (i64, u32, u32)) -> Vec<EuvCalendarDay> {
    let leading: u32 = weekday_index(year, month, 1);
    let month_days: u32 = days_in_month(year, month);
    let trailing: u32 = GRID_CELLS - leading - month_days;
    let (previous_year, previous_month) = adjacent_month(year, month, -1);
    let previous_days: u32 = days_in_month(previous_year, previous_month);
    let (next_year, next_month) = adjacent_month(year, month, 1);
    let mut cells: Vec<EuvCalendarDay> = Vec::with_capacity(GRID_CELLS as usize);
    for offset in 0..leading {
        let day: u32 = previous_days - leading + 1 + offset;
        cells.push(EuvCalendarDay {
            day,
            muted: true,
            today: (previous_year, previous_month, day) == today,
        });
    }
    for day in 1..=month_days {
        cells.push(EuvCalendarDay {
            day,
            muted: false,
            today: (year, month, day) == today,
        });
    }
    for day in 1..=trailing {
        cells.push(EuvCalendarDay {
            day,
            muted: true,
            today: (next_year, next_month, day) == today,
        });
    }
    cells
}

/// Resolves a clicked grid cell into the date it stands for.
///
/// A cell of the month itself resolves to that day of the month. A muted
/// cell belongs to the neighbouring month: the cells in front of the month
/// carry the trailing days of the previous month, the cells behind it the
/// leading days of the next one.
///
/// # Arguments
///
/// - `i64` - The year of the month on display.
/// - `u32` - The month on display, `1` through `12`.
/// - `bool` - Whether the clicked cell is one of the muted spill-over cells.
///
/// # Returns
///
/// - `Option<(i64, u32, u32)>` - The date the cell stands for, or `None`
///   when the cell does not name a real date.
pub fn resolve_clicked_date(
    year: i64,
    month: u32,
    day: u32,
    muted: bool,
) -> Option<(i64, u32, u32)> {
    if day == 0 {
        return None;
    }
    let month_days: u32 = days_in_month(year, month);
    if !muted {
        if day > month_days {
            return None;
        }
        return Some((year, month, day));
    }
    let leading: u32 = weekday_index(year, month, 1);
    let trailing: u32 = GRID_CELLS - leading - month_days;
    if day <= trailing {
        let (next_year, next_month) = adjacent_month(year, month, 1);
        return Some((next_year, next_month, day));
    }
    let (previous_year, previous_month) = adjacent_month(year, month, -1);
    if day > days_in_month(previous_year, previous_month) {
        return None;
    }
    Some((previous_year, previous_month, day))
}

/// Returns the current date of the host.
///
/// The browser's clock is read on the web and the system clock on the host,
/// so the same code answers with the same calendar day in both worlds.
///
/// # Returns
///
/// - `(i64, u32, u32)` - The year, the month and the day of the month.
fn current_date() -> (i64, u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let now: js_sys::Date = js_sys::Date::new_0();
        (
            now.get_full_year() as i64,
            now.get_month() + 1,
            now.get_date(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let seconds: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed: Duration| elapsed.as_secs() as i64)
            .unwrap_or_default();
        civil_from_days(seconds.div_euclid(SECONDS_PER_DAY))
    }
}

/// Returns the column headings of the day grid, Monday first.
///
/// # Returns
///
/// - `Vec<EuvCalendarWeekday>` - The headings, in display order.
fn calendar_weekdays() -> Vec<EuvCalendarWeekday> {
    vec![
        EuvCalendarWeekday::Mon,
        EuvCalendarWeekday::Tue,
        EuvCalendarWeekday::Wed,
        EuvCalendarWeekday::Thu,
        EuvCalendarWeekday::Fri,
        EuvCalendarWeekday::Sat,
        EuvCalendarWeekday::Sun,
    ]
}

/// Builds the click handler of the trigger.
///
/// The handler toggles the panel and, while opening it, marks the selected
/// day so the grid shows where the current value sits.
///
/// # Arguments
///
/// - `Signal<bool>` - The panel's open state.
/// - `Signal<u32>` - The day marked as selected in the grid.
/// - `Signal<String>` - The signal holding the selected date.
/// - `Signal<i64>` - The month on display.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the trigger.
fn on_trigger_click(
    open: Signal<bool>,
    selected: Signal<u32>,
    value: Signal<String>,
    month: Signal<i64>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        if open.get() {
            open.set(false);
            return;
        }
        open.set(true);
        let day: u32 = match parse_date(&value.get()) {
            Some((year, month_number, day)) if encode_month(year, month_number) == month.get() => {
                day
            }
            _ => 0,
        };
        selected.set(day);
    }))
}

/// Builds the click handler of the month chevrons.
///
/// The handler walks the month on display by the given number of months and
/// clears the selected day, because the day number of the previous month
/// means nothing in the new one.
///
/// # Arguments
///
/// - `Signal<i64>` - The month on display.
/// - `Signal<u32>` - The day marked as selected in the grid.
/// - `i64` - The number of months to walk.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the chevron.
fn on_shift_month(
    month: Signal<i64>,
    selected: Signal<u32>,
    delta: i64,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        month.set(month.get() + delta);
        selected.set(0);
    }))
}

/// Builds the callback a click on a day cell runs.
///
/// The callback receives the day number of the clicked cell and whether the
/// cell belongs to a neighbouring month, resolves the cell into a date and
/// publishes it. The click is handled by the cell itself rather than by a
/// handler on the panel: the delegated dispatch runs the handler of the
/// nearest marked ancestor, so a handler on the panel would never be reached
/// from a cell that carries a handler of its own.
///
/// A date outside the bounds is refused: the panel clears the selection and
/// leaves the value alone.
///
/// # Arguments
///
/// - `Signal<i64>` - The month on display.
/// - `Signal<u32>` - The day marked as selected in the grid.
/// - `Signal<String>` - The signal holding the selected date.
/// - `Signal<bool>` - The panel's open state.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the date.
/// - `&'static str` - The earliest selectable date.
///
/// # Returns
///
/// - `Rc<dyn Fn(u32, bool)>` - The callback of the day cells.
fn on_day_click(
    month: Signal<i64>,
    selected: Signal<u32>,
    value: Signal<String>,
    open: Signal<bool>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
    min: &'static str,
    max: &'static str,
) -> Rc<dyn Fn(u32, bool)> {
    Rc::new(move |day: u32, muted: bool| {
        let (year, month_number) = decode_month(month.get());
        let Some(picked) = resolve_clicked_date(year, month_number, day, muted) else {
            return;
        };
        if !date_within_bounds(picked, min, max) {
            selected.set(0);
            return;
        }
        let formatted: String = format_date(picked.0, picked.1, picked.2);
        let leaked: &'static str = Box::leak(formatted.clone().into_boxed_str());
        value.set(formatted);
        // The picked cell may belong to the neighbouring month, so the panel
        // walks to the month of the pick. The selected day is the day within
        // the month on display — keeping the number of the cell that was
        // clicked would mark a different day of the month walked to.
        month.set(encode_month(picked.0, picked.1));
        selected.set(picked.2);
        open.set(false);
        if let Some(callback) = &on_change {
            callback(leaked);
        }
    })
}

/// Builds the click handler of the "Today" button.
///
/// The handler selects today's date, walks the panel to the month of today
/// and closes the panel. A date outside the bounds is refused.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the selected date.
/// - `Signal<bool>` - The panel's open state.
/// - `Signal<i64>` - The month on display.
/// - `Signal<u32>` - The day marked as selected in the grid.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the date.
/// - `&'static str` - The earliest selectable date.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the button.
fn on_today(
    value: Signal<String>,
    open: Signal<bool>,
    month: Signal<i64>,
    selected: Signal<u32>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
    min: &'static str,
    max: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let (year, month_number, day) = current_date();
        if !date_within_bounds((year, month_number, day), min, max) {
            return;
        }
        let formatted: String = format_date(year, month_number, day);
        let leaked: &'static str = Box::leak(formatted.clone().into_boxed_str());
        value.set(formatted);
        month.set(encode_month(year, month_number));
        selected.set(day);
        open.set(false);
        if let Some(callback) = &on_change {
            callback(leaked);
        }
    }))
}

/// Builds the click handler of the "Clear" button.
///
/// The handler empties the field and closes the panel.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the selected date.
/// - `Signal<bool>` - The panel's open state.
/// - `Signal<u32>` - The day marked as selected in the grid.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The callback receiving the date.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the button.
fn on_clear(
    value: Signal<String>,
    open: Signal<bool>,
    selected: Signal<u32>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        value.set(String::new());
        selected.set(0);
        open.set(false);
        if let Some(callback) = &on_change {
            callback("");
        }
    }))
}

/// A date field backed by a calendar panel.
///
/// The component renders a `c_euv_date_picker` wrapper holding a trigger
/// button and the panel. The trigger shows the selected date, or the
/// placeholder while nothing is selected, and opens the panel; the panel
/// carries the month on display, the [`euv_calendar`] grid and a footer with
/// a "Today" and a "Clear" button.
///
/// The panel is always mounted: opening and closing it is a reactive class
/// swap between `c_euv_date_picker_panel_open` and
/// `c_euv_date_picker_panel_closed`, so the calendar never has to be
/// rebuilt while the user types in the field.
///
/// The month on display and the open state are owned by the component, while
/// the selected date lives in a caller-owned [`Signal`] formatted as
/// `YYYY-MM-DD`, so the picker takes part in two-way binding. Days outside
/// `min` and `max` cannot be picked.
///
/// The panel draws its own month caption and chevrons, because the calendar
/// it reuses is pure presentation and leaves paging to its caller; the
/// calendar keeps the inert chevrons of its own header as well.
///
/// # Arguments
///
/// - `VirtualNode<EuvDatePickerProps>` - The props node carrying the
///   selected date, the bounds and the change callback.
///
/// # Returns
///
/// - `VirtualNode` - The date field with its calendar panel.
#[component]
pub fn euv_date_picker(node: VirtualNode<EuvDatePickerProps>) -> VirtualNode {
    let EuvDatePickerProps {
        value,
        placeholder,
        label: label_text,
        min,
        max,
        on_change,
    }: EuvDatePickerProps = node.try_get_props().unwrap_or_default();
    let selected_value: String = value.get();
    let today: (i64, u32, u32) = current_date();
    let parsed: Option<(i64, u32, u32)> = parse_date(&selected_value);
    // An empty field has no selected day: the grid must not mark one. The
    // month on display still starts at the current month, so the panel opens
    // on the month the user is looking at.
    let initial_month: i64 = match parsed {
        Some((year, month_number, _)) => encode_month(year, month_number),
        None => encode_month(today.0, today.1),
    };
    let initial_selected: u32 = parsed.map(|(_, _, day): (i64, u32, u32)| day).unwrap_or(0);
    let is_empty: bool = selected_value.is_empty();
    let open: Signal<bool> = HookContext::use_hook(move || Signal::create(false));
    let month: Signal<i64> = HookContext::use_hook(move || Signal::create(initial_month));
    let selected: Signal<u32> = HookContext::use_hook(move || Signal::create(initial_selected));
    let caption_cache: MonthCaption = HookContext::use_hook(MonthCaption::default);
    let (year, month_number) = decode_month(month.get());
    let caption: &'static str = caption_cache.caption(year, month_number);
    let days: Vec<EuvCalendarDay> = month_grid(year, month_number, today);
    let day_click: Rc<dyn Fn(u32, bool)> =
        on_day_click(month, selected, value, open, on_change.clone(), min, max);
    let calendar_node: VirtualNode<EuvCalendarProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvCalendarProps {
            title: "",
            weekdays: calendar_weekdays(),
            days,
            selected,
            on_day_click: Some(day_click),
        })),
    };
    let calendar: VirtualNode = euv_calendar(calendar_node);
    html! {
        div {
            class: c_euv_date_picker()
            if !label_text.is_empty() {
                label {
                    class: c_form_label()
                    label_text
                }
            }
            button {
                type: "button"
                class: c_euv_date_picker_trigger()
                onclick: on_trigger_click(open, selected, value, month)
                span {
                    class: if is_empty {
                        c_euv_date_picker_placeholder()
                    } else {
                        c_euv_date_picker_value()
                    }
                    // The text has to be a child of the element, not a
                    // braced dynamic node: a braced node would capture the
                    // value it was rendered with and never learn about the
                    // pick that follows.
                    if is_empty {
                        placeholder
                    } else {
                        selected_value
                    }
                }
                span {
                    class: c_euv_date_picker_caret()
                    CARET_GLYPH
                }
            }
            div {
                class: c_euv_date_picker_panel()
                class: if { open } {
                    c_euv_date_picker_panel_open()
                } else {
                    c_euv_date_picker_panel_closed()
                }
                div {
                    class: c_euv_calendar_nav()
                    button {
                        type: "button"
                        class: c_euv_date_picker_nav()
                        onclick: on_shift_month(month, selected, -1)
                        PREVIOUS_MONTH_GLYPH
                    }
                    span {
                        class: c_euv_calendar_title()
                        caption
                    }
                    button {
                        type: "button"
                        class: c_euv_date_picker_nav()
                        onclick: on_shift_month(month, selected, 1)
                        NEXT_MONTH_GLYPH
                    }
                }
                calendar
                div {
                    class: c_euv_date_picker_footer()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: TODAY_LABEL
                        onclick: on_today(value, open, month, selected, on_change.clone(), min, max)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: CLEAR_LABEL
                        onclick: on_clear(value, open, selected, on_change.clone())
                    }
                }
            }
        }
    }
}
