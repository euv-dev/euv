use super::*;

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
    }
}

fn date_picker(value: &str, min: &'static str, max: &'static str) -> VirtualNode {
    HookContext::with(HookContext::default(), || {
        euv_date_picker(props(EuvDatePickerProps {
            value: Signal::create(String::from(value)),
            placeholder: "Pick a date",
            label: "Date",
            min,
            max,
            on_change: None,
        }))
    })
}

fn count_tag(node: &VirtualNode, tag: &str) -> usize {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            children,
            ..
        } => {
            let own: usize = usize::from(format!("{own_tag:?}") == format!("Element(\"{tag}\")"));
            own + children
                .iter()
                .map(|child: &VirtualNode| count_tag(child, tag))
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(|child: &VirtualNode| count_tag(child, tag)).sum(),
        _ => 0,
    }
}

fn carries_classes(entry: &AttributeEntry, names: &[&str]) -> bool {
    if names.len() == 1 {
        let single: AttributeEntry = AttributeEntry::new(
            Cow::Borrowed("class"),
            AttributeValue::Css(Css::new(
                String::from(names[0]),
                String::new(),
                Vec::new(),
                Vec::new(),
            )),
        );
        if entry == &single {
            return true;
        }
    }
    let merged: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(names.join(" ")),
    );
    entry == &merged
}

fn has_classes(node: &VirtualNode, names: &[&str]) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: bool = attributes
                .iter()
                .filter(|entry: &&AttributeEntry| entry.get_name() == "class")
                .any(|entry: &AttributeEntry| carries_classes(entry, names));
            own || children.iter().any(|child: &VirtualNode| has_classes(child, names))
        }
        VirtualNode::Fragment(children) => children.iter().any(|child: &VirtualNode| has_classes(child, names)),
        _ => false,
    }
}

#[test]
fn the_epoch_is_a_thursday() {
    let observed: u32 = weekday_index(1970, 1, 1);
    assert_eq!(
        observed, 3,
        "the first of January 1970 was a Thursday, which is index three"
    );
}

#[test]
fn a_civil_date_survives_the_round_trip_through_the_epoch() {
    let days: i64 = days_from_civil(2026, 10, 21);
    let observed: (i64, u32, u32) = civil_from_days(days);
    assert_eq!(
        observed,
        (2026, 10, 21),
        "the day count and the civil date are exact inverses"
    );
}

#[test]
fn the_epoch_day_count_is_zero() {
    let observed: i64 = days_from_civil(1970, 1, 1);
    assert_eq!(observed, 0, "the epoch counts as day zero");
}

#[test]
fn a_leap_year_has_a_longer_february() {
    let common: u32 = days_in_month(2026, 2);
    let leap: u32 = days_in_month(2024, 2);
    assert_eq!(common, 28, "2026 is not a leap year");
    assert_eq!(leap, 29, "2024 is a leap year");
    assert!(
        !is_leap_year(1900) && is_leap_year(2000),
        "a century is only a leap year when it is divisible by four hundred"
    );
}

#[test]
fn the_weekday_of_a_known_date_is_right() {
    let observed: u32 = weekday_index(2026, 10, 21);
    assert_eq!(
        observed, 2,
        "the twenty-first of October 2026 was a Wednesday"
    );
}

#[test]
fn a_month_grid_always_holds_forty_two_cells() {
    let grid: Vec<EuvCalendarDay> = month_grid(2026, 10, (2026, 10, 21));
    assert_eq!(
        grid.len(),
        42,
        "the grid is six weeks of seven days, so the panel keeps its height"
    );
}

#[test]
fn a_month_grid_starts_on_the_weekday_of_the_first() {
    let grid: Vec<EuvCalendarDay> = month_grid(2026, 10, (2026, 10, 21));
    let first_of_month: usize = grid
        .iter()
        .position(|cell: &EuvCalendarDay| !cell.muted)
        .unwrap_or_default();
    let muted: usize = grid
        .iter()
        .filter(|cell: &&EuvCalendarDay| cell.muted)
        .count();
    assert_eq!(
        first_of_month, 3,
        "October 2026 starts on a Thursday, so three cells spill over from September"
    );
    assert_eq!(
        muted - first_of_month,
        8,
        "and eight cells of November follow the thirty-one days of October"
    );
}

#[test]
fn a_month_grid_marks_today() {
    let grid: Vec<EuvCalendarDay> = month_grid(2026, 10, (2026, 10, 21));
    let today: Vec<&EuvCalendarDay> = grid.iter().filter(|cell: &EuvCalendarDay| cell.today).collect();
    assert_eq!(today.len(), 1, "exactly one cell carries the today marker");
    assert_eq!(today[0].day, 21, "and it is the day of the month");
}

#[test]
fn a_date_is_parsed_from_its_iso_text() {
    let observed: Option<(i64, u32, u32)> = parse_date("2026-10-21");
    assert_eq!(
        observed,
        Some((2026, 10, 21)),
        "the iso text names the date"
    );
}

#[test]
fn text_that_is_not_a_date_is_rejected() {
    assert_eq!(parse_date(""), None, "an empty string is not a date");
    assert_eq!(
        parse_date("2026-13-01"),
        None,
        "there is no thirteenth month"
    );
    assert_eq!(
        parse_date("2026-02-30"),
        None,
        "February never has a thirtieth day"
    );
    assert_eq!(parse_date("not a date"), None, "and neither is a sentence");
}

#[test]
fn a_date_is_formatted_back_into_its_iso_text() {
    let observed: String = format_date(2026, 10, 21);
    assert_eq!(
        observed, "2026-10-21",
        "the iso text is padded on both sides"
    );
}

#[test]
fn a_date_outside_the_bounds_is_refused() {
    let inside: bool = date_within_bounds((2026, 10, 21), "2026-01-01", "2026-12-31");
    let below: bool = date_within_bounds((2025, 12, 31), "2026-01-01", "2026-12-31");
    let above: bool = date_within_bounds((2027, 1, 1), "2026-01-01", "2026-12-31");
    let unbounded: bool = date_within_bounds((1999, 1, 1), "", "");
    assert!(inside, "a date between the bounds may be picked");
    assert!(!below, "a date before the lower bound may not");
    assert!(!above, "a date after the upper bound may not");
    assert!(unbounded, "empty bounds leave the date free");
}

#[test]
fn a_cell_of_the_month_resolves_to_that_day() {
    let observed: Option<(i64, u32, u32)> = resolve_clicked_date(2026, 10, 21, false);
    assert_eq!(
        observed,
        Some((2026, 10, 21)),
        "a cell of the month names a day of that month"
    );
}

#[test]
fn a_muted_cell_resolves_to_the_neighbouring_month() {
    let leading: Option<(i64, u32, u32)> = resolve_clicked_date(2026, 10, 30, true);
    let trailing: Option<(i64, u32, u32)> = resolve_clicked_date(2026, 10, 1, true);
    assert_eq!(
        leading,
        Some((2026, 9, 30)),
        "the cells in front of October carry the days of September"
    );
    assert_eq!(
        trailing,
        Some((2026, 11, 1)),
        "the cells behind October carry the days of November"
    );
}

#[test]
fn a_month_code_survives_the_round_trip() {
    let encoded: i64 = encode_month(2026, 10);
    let observed: (i64, u32) = decode_month(encoded);
    assert_eq!(
        observed,
        (2026, 10),
        "the month signal holds the whole month"
    );
    let january: (i64, u32) = decode_month(encode_month(2027, 1));
    assert_eq!(
        january,
        (2027, 1),
        "and it survives the turn of the year as well"
    );
}

#[test]
fn a_date_picker_renders_a_trigger_and_a_kept_mounted_panel() {
    let rendered: VirtualNode = date_picker("2026-10-21", "", "");
    assert_eq!(
        count_tag(&rendered, "button"),
        47,
        "the trigger, the two month chevrons, the forty-two day cells and the two \
         footer buttons are all mounted; the embedded calendar brings no second \
         pair of chevrons, because a calendar with no title renders no header"
    );
    assert!(
        has_classes(
            &rendered,
            &["c_euv_date_picker_panel", "c_euv_date_picker_panel_closed"]
        ),
        "the panel stays mounted and closed until the trigger opens it"
    );
    assert!(
        has_classes(&rendered, &["c_euv_date_picker_trigger"]),
        "the trigger carries the trigger class"
    );
    assert!(
        has_classes(&rendered, &["c_euv_date_picker_value"]),
        "a selected date is shown on the trigger"
    );
    assert!(
        has_classes(&rendered, &["c_euv_date_picker_footer"]),
        "the panel ends with the footer row"
    );
}

#[test]
fn a_date_picker_without_a_value_shows_the_placeholder() {
    let rendered: VirtualNode = date_picker("", "", "");
    assert!(
        has_classes(&rendered, &["c_euv_date_picker_placeholder"]),
        "an empty value shows the placeholder instead of a date"
    );
}

fn count_class(node: &VirtualNode, class: &str) -> usize {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: usize = usize::from(attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "class"
                    && attribute
                        .class_names()
                        .iter()
                        .any(|name: &String| name == class)
            }));
            own + children
                .iter()
                .map(|child: &VirtualNode| count_class(child, class))
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => {
            children.iter().map(|child: &VirtualNode| count_class(child, class)).sum()
        }
        _ => 0,
    }
}

fn element_with_class<'a>(node: &'a VirtualNode, class: &str) -> Option<&'a VirtualNode> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let carries: bool = attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "class"
                    && attribute
                        .class_names()
                        .iter()
                        .any(|name: &String| name == class)
            });
            if carries {
                return Some(node);
            }
            for child in children {
                if let Some(found) = element_with_class(child, class) {
                    return Some(found);
                }
            }
            None
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                if let Some(found) = element_with_class(child, class) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}

#[test]
fn an_empty_date_picker_marks_no_day_as_selected() {
    let rendered: VirtualNode = date_picker("", "", "");
    assert_eq!(
        count_class(&rendered, "c_euv_calendar_day_selected"),
        0,
        "an empty field has no selected day, so the grid must not mark one"
    );
}

#[test]
fn the_trigger_holds_its_text_as_a_text_child() {
    let rendered: VirtualNode = date_picker("", "", "");
    let span: &VirtualNode = element_with_class(&rendered, "c_euv_date_picker_placeholder")
        .expect("an empty picker shows the placeholder span");
    let VirtualNode::Element { children, .. } = span else {
        panic!("the placeholder span must be an element");
    };
    assert!(
        children
            .iter()
            .any(|child: &VirtualNode| matches!(child, VirtualNode::Text(_))),
        "the trigger text must be a text child: a dynamic child would keep the \
         text it was rendered with and never show the date that is picked"
    );
}

fn count_click_handling_day_cells(node: &VirtualNode) -> usize {
    const DAY_CLASSES: [&str; 4] = [
        "c_euv_calendar_day",
        "c_euv_calendar_day_muted",
        "c_euv_calendar_day_today",
        "c_euv_calendar_day_selected",
    ];
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let is_day_cell: bool = attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "class"
                    && attribute
                        .class_names()
                        .iter()
                        .any(|name: &String| DAY_CLASSES.contains(&name.as_str()))
            });
            let carries_handler: bool = attributes.iter().any(|attribute: &AttributeEntry| {
                attribute.get_name() == "onclick" && attribute.text_value().is_none()
            });
            let own: usize = usize::from(is_day_cell && carries_handler);
            own + children
                .iter()
                .map(count_click_handling_day_cells)
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => {
            children.iter().map(count_click_handling_day_cells).sum()
        }
        _ => 0,
    }
}

#[test]
fn every_day_cell_carries_its_own_click_handler() {
    let rendered: VirtualNode = date_picker("", "", "");
    assert_eq!(
        count_click_handling_day_cells(&rendered),
        42,
        "every day cell has to carry the handler that picks the date"
    );
}
