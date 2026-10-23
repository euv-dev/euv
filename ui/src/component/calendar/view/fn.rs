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
/// - `Option<Rc<dyn Fn(u32, bool)>>` - The callback a click on the cell runs.
///
/// # Returns
///
/// - `VirtualNode` - The day button.
fn calendar_day(
    day: EuvCalendarDay,
    selected: Signal<u32>,
    index: usize,
    on_day_click: Option<Rc<dyn Fn(u32, bool)>>,
) -> VirtualNode {
    let day_number: u32 = day.day;
    let day_text: String = day_number.to_string();
    let is_selected: bool = !day.muted && selected.get() == day_number;
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
    html! {
        button {
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
            key: index.to_string()
            onclick: handler
            day_text
        }
    }
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
    let weekday_nodes: Vec<VirtualNode> = weekdays
        .iter()
        .map(|weekday: &EuvCalendarWeekday| calendar_weekday(*weekday))
        .collect();
    let day_nodes: Vec<VirtualNode> = days
        .iter()
        .enumerate()
        .map(
            |(index, day): (usize, &calendar::view::r#struct::EuvCalendarDay)| {
                calendar_day(*day, selected, index, on_day_click.clone())
            },
        )
        .collect();
    html! {
        div {
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
                        title
                    }
                    div {
                        class: c_euv_calendar_nav()
                        button {
                            class: c_euv_calendar_day()
                            "‹"
                        }
                        button {
                            class: c_euv_calendar_day()
                            "›"
                        }
                    }
                }
            }
            div {
                class: c_euv_calendar_weekdays()
                weekday_nodes
            }
            div {
                class: c_euv_calendar_grid()
                day_nodes
            }
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
