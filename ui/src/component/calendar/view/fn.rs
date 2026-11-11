use super::*;

/// Renders one column heading of the calendar header row.
///
/// # Arguments
///
/// - `EuvCalendarWeekday` - The weekday this column stands for.
///
/// # Returns
///
/// - `VirtualNode` - The weekday heading cell.
fn calendar_weekday(weekday: EuvCalendarWeekday) -> VirtualNode {
    let label: &'static str = match weekday {
        EuvCalendarWeekday::Mon => "Mo",
        EuvCalendarWeekday::Tue => "Tu",
        EuvCalendarWeekday::Wed => "We",
        EuvCalendarWeekday::Thu => "Th",
        EuvCalendarWeekday::Fri => "Fr",
        EuvCalendarWeekday::Sat => "Sa",
        EuvCalendarWeekday::Sun => "Su",
    };
    html! {
        div {
            role: "columnheader"
            class: c_euv_calendar_weekday()
            key: label
            {
                label
            }
        }
    }
}

/// Renders one day cell of the calendar grid.
///
/// Every cell stays mounted; the cell picks exactly one class out of the
/// registry — `_muted`, `_selected`, `_today` or the plain day class — so the
/// class list of a cell never repeats a token and never leaves two
/// competing rules to be settled by stylesheet order. The registry entries
/// are complete cell styles rather than additive modifiers, so one class per
/// cell is what the cascade can actually honour.
///
/// A spill-over cell belongs to the neighbouring month, so it never carries
/// the selected marker: the day number it shows belongs to another month and
/// would mark the wrong cell.
///
/// # Arguments
///
/// - `EuvCalendarDay` - The cell to render.
/// - `Signal<u32>` - The selected-day signal driving the `_selected` modifier.
/// - `usize` - The position of the cell in the grid, used as its key.
/// - `usize` - The total number of day cells in the grid.
/// - `bool` - Whether this cell is the grid's roving tab stop.
/// - `Option<Rc<dyn Fn(u32, bool)>>` - The callback a click on the cell runs.
/// - `&str` - The month title used to form the accessible name.
///
/// # Returns
///
/// - `VirtualNode` - The grid cell containing the day button.
fn calendar_day(
    day: EuvCalendarDay,
    selected: Signal<u32>,
    index: usize,
    day_count: usize,
    is_roving_tab_stop: bool,
    on_day_click: Option<Rc<dyn Fn(u32, bool)>>,
    title: &str,
) -> VirtualNode {
    let day_number: u32 = day.day;
    let day_text: String = day_number.to_string();
    let is_selected: bool = !day.muted && selected.get() == day_number;
    let tabindex: &str = if is_roving_tab_stop { "0" } else { "-1" };
    let aria_selected: &str = if is_selected {
        CALENDAR_ATTR_TRUE
    } else {
        CALENDAR_ATTR_FALSE
    };
    let aria_current: &str = if day.today {
        CALENDAR_CURRENT_DATE
    } else {
        CALENDAR_ATTR_FALSE
    };
    // The cell does not resolve the click on its own: the caller owns the
    // date arithmetic, so the cell hands it the day number and whether the
    // cell belongs to a neighbouring month. Without a callback the cell
    // writes its day number into the selected signal, which is what a
    // calendar used on its own needs.
    let handler: Option<Rc<dyn Fn(Event)>> = match on_day_click {
        Some(callback) => {
            let callback: Rc<dyn Fn(u32, bool)> = Rc::clone(&callback);
            let muted: bool = day.muted;
            Some(Rc::new(move |_event: Event| {
                callback(day_number, muted);
            }))
        }
        None => on_calendar_day_click(selected, day_number),
    };
    let on_keydown: Option<Rc<dyn Fn(Event)>> = calendar_day_keydown(index, day_count);
    html! {
        div {
            role: "gridcell"
            "aria-selected": aria_selected
            "aria-current": aria_current
            class: c_euv_calendar_grid_cell()
            key: index.to_string()
            button {
                type: "button"
                "data-slot": "calendar-day-button"
                "data-day-index": index.to_string()
                "data-selected": if is_selected {
                    CALENDAR_ATTR_TRUE
                } else {
                    CALENDAR_ATTR_FALSE
                }
                "data-today": if day.today {
                    CALENDAR_ATTR_TRUE
                } else {
                    CALENDAR_ATTR_FALSE
                }
                "aria-label": format!("{} {}", title, day_number)
                tabindex: tabindex
                class: c_euv_calendar_grid_cell_button()
                class: if day.muted {
                    c_euv_calendar_day_muted()
                } else if is_selected {
                    c_euv_calendar_day_selected()
                } else if day.today {
                    c_euv_calendar_day_today()
                } else {
                    c_euv_calendar_day()
                }
                // The key has to be unique among the siblings. The day number
                // is not: the spill-over cells repeat the day numbers of the
                // month on display, and a duplicated key makes the keyed diff
                // hand a cell the DOM node of another one.
                onclick: handler
                onkeydown: on_keydown
                day_text
            }
        }
    }
}

/// Builds the roving-focus handler for a calendar day.
///
/// Arrow keys move by one day or one week; Home and End stay within the
/// current week. Clicking the focused cell reuses the caller's date logic.
///
/// # Arguments
///
/// - `usize` - The current cell index.
/// - `usize` - The number of cells in the calendar grid.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The day-navigation handler.
fn calendar_day_keydown(index: usize, day_count: usize) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        let week_start: usize = index / 7 * 7;
        let next_index: usize = match key.as_str() {
            CALENDAR_KEY_ARROW_LEFT if index > 0 => index - 1,
            CALENDAR_KEY_ARROW_RIGHT if index + 1 < day_count => index + 1,
            CALENDAR_KEY_ARROW_UP if index >= 7 => index - 7,
            CALENDAR_KEY_ARROW_DOWN if index + 7 < day_count => index + 7,
            CALENDAR_KEY_HOME => week_start,
            CALENDAR_KEY_END => (week_start + 6).min(day_count.saturating_sub(1)),
            _ => return,
        };
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Ok(Some(root)) = target.closest(CALENDAR_GRID_SELECTOR) else {
            return;
        };
        let Ok(buttons) = root.query_selector_all(CALENDAR_DAY_BUTTON_SELECTOR) else {
            return;
        };
        let Some(next) = buttons
            .item(next_index as u32)
            .and_then(|button: Node| button.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        keyboard_event.prevent_default();
        let _: Result<(), JsValue> = next.focus();
        next.click();
    }))
}

/// A pure-presentation month grid for date selection.
///
/// This component deliberately computes no dates: the caller resolves the
/// month, builds the `weekdays` header and the `days` cells (including
/// leading / trailing spill-over cells) and hands them in. Presentation-only
/// means the grid is reproducible from its props alone, so a test can assert
/// the rendered classes without touching the clock.
///
/// Day cells are always mounted; selection is a reactive class swap driven by
/// the `selected` signal, and clicking a cell writes the day number back
/// through [`on_calendar_day_click`]. The chevrons are inert — paging between
/// months belongs to the caller, which owns the date state this component
/// deliberately does not.
///
/// # Arguments
///
/// - `VirtualNode<EuvCalendarProps>` - The props node containing the month
///   caption, the weekday headings, the day cells and the selected day.
///
/// # Returns
///
/// - `VirtualNode` - The calendar virtual DOM tree.
#[component]
pub fn euv_calendar(node: VirtualNode<EuvCalendarProps>) -> VirtualNode {
    let EuvCalendarProps {
        title,
        weekdays,
        days,
        selected,
        on_day_click,
    }: EuvCalendarProps = node.try_get_props().unwrap_or_default();
    let selected_day: u32 = selected.get();
    let roving_index: usize = days
        .iter()
        .position(|day: &EuvCalendarDay| !day.muted && day.day == selected_day)
        .or_else(|| {
            days.iter()
                .position(|day: &EuvCalendarDay| !day.muted && day.today)
        })
        .or_else(|| days.iter().position(|day: &EuvCalendarDay| !day.muted))
        .unwrap_or(0);
    let weekday_nodes: Vec<VirtualNode> = weekdays
        .iter()
        .map(|weekday: &EuvCalendarWeekday| calendar_weekday(*weekday))
        .collect();
    let day_nodes: Vec<VirtualNode> = days
        .iter()
        .enumerate()
        .map(
            |(index, day): (usize, &calendar::view::r#struct::EuvCalendarDay)| {
                calendar_day(
                    *day,
                    selected,
                    index,
                    days.len(),
                    index == roving_index,
                    on_day_click.clone(),
                    title,
                )
            },
        )
        .collect();
    html! {
        div {
            "data-slot": "calendar"
            class: c_euv_calendar()
            // The header carries the month title and the two month
            // steps. A calendar that is embedded in a picker brings its
            // own header, so a calendar with no title renders no header
            // at all — a header with an empty title and a pair of month
            // steps that belong to nobody is worse than no header.
            if !title.is_empty() {
                div {
                    class: c_euv_calendar_header()
                    h3 {
                        class: c_euv_calendar_title()
                        "aria-live": "polite"
                        "aria-atomic": "true"
                        title
                    }
                    div {
                        class: c_euv_calendar_nav()
                        button {
                            type: "button"
                            "aria-label": CALENDAR_PREVIOUS_MONTH_LABEL
                            class: c_euv_calendar_day()
                            "‹"
                        }
                        button {
                            type: "button"
                            "aria-label": CALENDAR_NEXT_MONTH_LABEL
                            class: c_euv_calendar_day()
                            "›"
                        }
                    }
                }
            }
            div {
                "data-slot": "calendar-grid"
                role: "grid"
                "aria-label": if title.is_empty() {
                    CALENDAR_ARIA_LABEL
                } else {
                    title
                }
                div {
                    role: "row"
                    class: c_euv_calendar_weekdays()
                    weekday_nodes
                }
                div {
                    role: "rowgroup"
                    class: c_euv_calendar_grid()
                    for row in day_nodes.chunks(7) {
                        div {
                            role: "row"
                            class: c_euv_calendar_grid_row()
                            for day_node in row.iter() {
                                day_node.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The clickable cell of a day.
///
/// The cell reports the chosen day and the refused one the way the
/// reference does: the chosen day carries `data-selected` and
/// `aria-selected`, the refused one carries `data-disabled` and
/// `aria-disabled`.
///
/// # Arguments
///
/// - `VirtualNode<EuvCalendarDayButtonProps>` - The props node containing the
///   day number and the state of the cell.
///
/// # Returns
///
/// - `VirtualNode` - The euv calendar day button virtual DOM tree.
#[component]
pub fn euv_calendar_day_button(node: VirtualNode<EuvCalendarDayButtonProps>) -> VirtualNode {
    let EuvCalendarDayButtonProps {
        day,
        selected,
        disabled,
        on_select,
    }: EuvCalendarDayButtonProps = node.try_get_props().unwrap_or_default();
    let disabled_signal: Signal<bool> = Signal::create(disabled);
    let day_number: u32 = day;
    let day_text: String = day_number.to_string();
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_select {
                callback(day_number);
            }
        }))
    };
    html! {
        button {
            type: "button"
            "data-slot": "calendar-day-button"
            "data-selected": if selected {
                "true"
            } else {
                "false"
            }
            "data-disabled": if disabled {
                "true"
            } else {
                "false"
            }
            "aria-selected": if selected {
                "true"
            } else {
                "false"
            }
            "aria-disabled": if disabled {
                "true"
            } else {
                "false"
            }
            tabindex: if disabled {
                "-1"
            } else {
                "0"
            }
            disabled: disabled_signal
            class: if disabled {
                c_euv_calendar_day_button_disabled()
            } else if selected {
                c_euv_calendar_day_button_selected()
            } else {
                c_euv_calendar_day_button()
            }
            onclick: on_click
            day_text
        }
    }
}

/// Builds the click handler that selects a calendar day.
///
/// # Arguments
///
/// - `Signal<u32>` - The selected-day signal to write.
/// - `u32` - The day number of the clicked cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
pub fn on_calendar_day_click(selected: Signal<u32>, day: u32) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        selected.set(day);
    }))
}
