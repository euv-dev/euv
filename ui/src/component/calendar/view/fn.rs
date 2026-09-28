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
            key: {
                label
            }
            {
                label
            }
        }
    }
}

/// Renders one day cell of the calendar grid.
///
/// Every cell stays mounted; the `_muted`, `_today` and `_selected` modifiers
/// stack on the base `c_euv_calendar_day` class so a cell can be muted *and*
/// today *and* selected at once without a combinatorial class explosion.
///
/// # Arguments
///
/// - `EuvCalendarDay` - The cell to render.
/// - `Signal<u32>` - The selected-day signal driving the `_selected` modifier.
///
/// # Returns
///
/// - `VirtualNode` - The day button.
fn calendar_day(day: EuvCalendarDay, selected: Signal<u32>) -> VirtualNode {
    let day_number: u32 = day.day;
    let day_text: String = day_number.to_string();
    let is_selected: bool = selected.get() == day_number;
    let base_class: fn() -> &'static Css = if day.muted {
        c_euv_calendar_day_muted
    } else {
        c_euv_calendar_day
    };
    html! {
        button {
            class: base_class()
            class: if day.today {
                c_euv_calendar_day_today()
            } else {
                c_euv_calendar_day()
            }
            class: if is_selected {
                c_euv_calendar_day_selected()
            } else {
                c_euv_calendar_day()
            }
            key: {
                day_number.to_string()
            }
            onclick: on_calendar_day_click(selected, day_number)
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
    }: EuvCalendarProps = node.try_get_props().unwrap_or_default();
    let weekday_nodes: Vec<VirtualNode> = weekdays
        .iter()
        .map(|weekday: &EuvCalendarWeekday| calendar_weekday(*weekday))
        .collect();
    let day_nodes: Vec<VirtualNode> = days
        .iter()
        .map(|day: &EuvCalendarDay| calendar_day(*day, selected))
        .collect();
    html! {
        div {
            class: c_euv_calendar()
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
