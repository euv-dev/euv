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

fn time_picker(value: &str, min_hour: u32, max_hour: u32, minute_step: u32) -> VirtualNode {
    HookContext::with(HookContext::default(), || {
        euv_time_picker(props(EuvTimePickerProps {
            value: Signal::create(String::from(value)),
            placeholder: "Pick a time",
            label: "Time",
            min_hour,
            max_hour,
            minute_step,
            on_change: None,
        }))
    })
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

fn elements_with_class<'a>(node: &'a VirtualNode, class: &str, into: &mut Vec<&'a VirtualNode>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let carries: bool = attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == "class"
                    && entry
                        .class_names()
                        .iter()
                        .any(|name: &String| name.as_str() == class)
            });
            if carries {
                into.push(node);
            }
            for child in children {
                elements_with_class(child, class, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                elements_with_class(child, class, into);
            }
        }
        _ => {}
    }
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

fn texts_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Text(text_node) => {
            into.push(format!("{text_node:?}"));
        }
        VirtualNode::Element { children, .. } => {
            for child in children {
                texts_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                texts_of(child, into);
            }
        }
        _ => {}
    }
}

fn texts(node: &VirtualNode) -> Vec<String> {
    let mut into: Vec<String> = Vec::new();
    texts_of(node, &mut into);
    into
}

#[test]
fn a_time_is_parsed_from_its_clock_text() {
    let observed: Option<(u32, u32)> = parse_time("09:30");
    assert_eq!(
        observed,
        Some((9, 30)),
        "the clock text names the hour and the minute"
    );
}

#[test]
fn text_that_is_not_a_time_is_rejected() {
    assert_eq!(parse_time(""), None, "an empty string is not a time");
    assert_eq!(parse_time("24:00"), None, "there is no twenty-fourth hour");
    assert_eq!(parse_time("12:60"), None, "there is no sixtieth minute");
    assert_eq!(parse_time("noon"), None, "and neither is a word");
    assert_eq!(parse_time("12:00:00"), None, "a time has two parts only");
}

#[test]
fn a_time_is_formatted_back_into_its_clock_text() {
    let observed: String = format_time(9, 5);
    assert_eq!(observed, "09:05", "the clock text is padded on both sides");
}

#[test]
fn the_offered_hours_run_from_the_first_to_the_last_bound() {
    let observed: Vec<u32> = hour_values(9, 12);
    assert_eq!(
        observed,
        vec![9, 10, 11, 12],
        "the last hour of the range is offered as well"
    );
}

#[test]
fn the_hour_bounds_are_clamped_into_the_day() {
    assert_eq!(
        clamp_hour_range(30, 40),
        (23, 23),
        "a bound above the last hour of the day is clamped to it"
    );
    assert_eq!(
        clamp_hour_range(5, 2),
        (2, 5),
        "bounds that arrive in the wrong order are flipped, so the range is never empty"
    );
}

#[test]
fn the_offered_minutes_run_in_steps() {
    let observed: Vec<u32> = minute_values(15);
    assert_eq!(
        observed,
        vec![0, 15, 30, 45],
        "the minutes run from zero to fifty-nine in steps of the given size"
    );
}

#[test]
fn a_minute_step_of_zero_offers_every_minute() {
    let observed: Vec<u32> = minute_values(0);
    assert_eq!(
        observed.len(),
        60,
        "a step of zero is treated as a step of one minute"
    );
}

#[test]
fn picking_an_hour_keeps_the_current_minutes() {
    let observed: String = time_with_hour("09:30", 14);
    assert_eq!(
        observed, "14:30",
        "the minutes of the current value survive the picked hour"
    );
}

#[test]
fn picking_a_minute_keeps_the_current_hour() {
    let observed: String = time_with_minute("09:30", 45, 8);
    assert_eq!(
        observed, "09:45",
        "the hour of the current value survives the picked minute"
    );
}

#[test]
fn a_pick_falls_back_when_nothing_is_selected_yet() {
    assert_eq!(
        time_with_hour("", 14),
        "14:00",
        "an hour picked without a value starts the hour"
    );
    assert_eq!(
        time_with_minute("", 45, 8),
        "08:45",
        "a minute picked without a value starts at the first offered hour"
    );
}

#[test]
fn a_time_picker_renders_a_trigger_and_a_kept_mounted_panel() {
    let rendered: VirtualNode = time_picker("09:30", 0, 23, 15);
    let mut wrappers: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker", &mut wrappers);
    assert_eq!(wrappers.len(), 1, "the picker renders one wrapper");
    let mut triggers: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_trigger", &mut triggers);
    assert_eq!(triggers.len(), 1, "the wrapper holds one trigger");
    let mut grids: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_grid", &mut grids);
    assert_eq!(grids.len(), 1, "the panel holds one grid");
    let mut columns: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_column", &mut columns);
    assert_eq!(
        columns.len(),
        2,
        "the grid holds an hour and a minute column"
    );
    let mut footers: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_footer", &mut footers);
    assert_eq!(footers.len(), 1, "the panel ends with the footer row");
    assert!(
        has_classes(
            &rendered,
            &["c_euv_time_picker_panel", "c_euv_time_picker_panel_closed"]
        ),
        "the panel stays mounted and closed until the trigger opens it"
    );
}

#[test]
fn the_two_columns_list_the_hours_and_the_minutes() {
    let rendered: VirtualNode = time_picker("09:30", 9, 12, 15);
    let mut columns: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_column", &mut columns);
    let hour_column: &VirtualNode = columns.first().copied().unwrap_or(&rendered);
    let minute_column: &VirtualNode = columns.get(1).copied().unwrap_or(&rendered);
    assert_eq!(
        count_tag(hour_column, "button"),
        4,
        "the hours from nine to twelve are four cells"
    );
    assert_eq!(
        count_tag(minute_column, "button"),
        4,
        "the minutes in steps of fifteen are four cells"
    );
    assert!(
        texts(hour_column)
            .iter()
            .any(|text: &String| text.contains("\"09\"")),
        "the hour column carries the hour it names, got {:?}",
        texts(hour_column)
    );
    assert!(
        texts(minute_column)
            .iter()
            .any(|text: &String| text.contains("\"30\"")),
        "the minute column carries the minute it names, got {:?}",
        texts(minute_column)
    );
}

#[test]
fn the_chosen_cell_carries_the_active_class() {
    let rendered: VirtualNode = time_picker("09:30", 0, 23, 15);
    let mut active: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_cell_active", &mut active);
    assert_eq!(
        active.len(),
        2,
        "the chosen hour and the chosen minute carry the active class"
    );
    let mut cells: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_cell", &mut cells);
    assert_eq!(
        cells.len(),
        28,
        "the twenty-four hours and the four stepped minutes are cells"
    );
    assert!(
        has_classes(
            &rendered,
            &["c_euv_time_picker_cell", "c_euv_time_picker_cell_active"]
        ),
        "the active modifier stacks on top of the base cell class"
    );
}

#[test]
fn a_time_picker_without_a_value_shows_the_placeholder() {
    let rendered: VirtualNode = time_picker("", 0, 23, 15);
    let mut placeholders: Vec<&VirtualNode> = Vec::new();
    elements_with_class(
        &rendered,
        "c_euv_time_picker_placeholder",
        &mut placeholders,
    );
    assert_eq!(
        placeholders.len(),
        1,
        "an empty value shows the placeholder instead of a time"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Pick a time")),
        "the placeholder text reaches the trigger, got {rendered:?}"
    );
    let mut active: Vec<&VirtualNode> = Vec::new();
    elements_with_class(&rendered, "c_euv_time_picker_cell_active", &mut active);
    assert!(
        active.is_empty(),
        "no cell is chosen while nothing is selected"
    );
}

#[test]
fn the_footer_offers_a_now_and_a_clear_button() {
    let rendered: VirtualNode = time_picker("09:30", 0, 23, 15);
    let rendered_texts: Vec<String> = texts(&rendered);
    assert!(
        rendered_texts
            .iter()
            .any(|text: &String| text.contains("\"Now\"")),
        "the footer selects the current time, got {rendered:?}"
    );
    assert!(
        rendered_texts
            .iter()
            .any(|text: &String| text.contains("\"Clear\"")),
        "the footer empties the field, got {rendered:?}"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_host_build_has_no_browser_clock() {
    assert!(
        current_time().is_none(),
        "a host build has no clock the browser way, so the Now button stays inert"
    );
}

#[test]
fn picking_an_hour_writes_the_time_and_closes_the_panel() {
    let value: Signal<String> = Signal::create(String::from("09:30"));
    let open: Signal<bool> = Signal::create(true);
    let seen: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<String>>> = seen.clone();
    let callback: Rc<dyn Fn(&'static str)> = Rc::new(move |time: &'static str| {
        recorder.borrow_mut().push(String::from(time));
    });
    let handler: Rc<dyn Fn(u32)> = time_picker_pick_hour_handler(value, open, Some(callback), 14)
        .expect("the hour cell always carries a pick handler");
    handler(0);
    assert_eq!(
        value.get(),
        "14:30",
        "the picked hour is written into the value and keeps the minutes"
    );
    assert!(!open.get(), "picking a cell closes the panel");
    assert_eq!(
        seen.borrow().len(),
        1,
        "the change callback fires exactly once, got {:?}",
        seen.borrow()
    );
    assert_eq!(seen.borrow()[0], "14:30", "and it reports the new time");
}

#[test]
fn picking_a_minute_writes_the_time_and_closes_the_panel() {
    let value: Signal<String> = Signal::create(String::from("09:30"));
    let open: Signal<bool> = Signal::create(true);
    let seen: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<String>>> = seen.clone();
    let callback: Rc<dyn Fn(&'static str)> = Rc::new(move |time: &'static str| {
        recorder.borrow_mut().push(String::from(time));
    });
    let handler: Rc<dyn Fn(u32)> =
        time_picker_pick_minute_handler(value, open, Some(callback), 45, 8)
            .expect("the minute cell always carries a pick handler");
    handler(0);
    assert_eq!(
        value.get(),
        "09:45",
        "the picked minute is written into the value and keeps the hour"
    );
    assert!(!open.get(), "picking a cell closes the panel");
    assert_eq!(
        seen.borrow()[0],
        "09:45",
        "the change callback reports the new time"
    );
}

#[test]
fn the_clear_handler_empties_the_field_and_closes_the_panel() {
    let value: Signal<String> = Signal::create(String::from("09:30"));
    let open: Signal<bool> = Signal::create(true);
    let seen: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder: Rc<RefCell<Vec<String>>> = seen.clone();
    let callback: Rc<dyn Fn(&'static str)> = Rc::new(move |time: &'static str| {
        recorder.borrow_mut().push(String::from(time));
    });
    let handler: Rc<dyn Fn(u32)> = time_picker_clear_handler(value, open, Some(callback));
    handler(0);
    assert_eq!(value.get(), "", "clearing empties the field");
    assert!(!open.get(), "clearing closes the panel");
    assert_eq!(
        seen.borrow()[0],
        "",
        "the change callback reports the empty time"
    );
}

#[test]
fn the_toggle_handler_opens_and_closes_the_panel() {
    let open: Signal<bool> = Signal::create(false);
    let handler: Rc<dyn Fn(u32)> = time_picker_toggle_handler::<u32>(open)
        .expect("the trigger always carries a toggle handler");
    handler(0);
    assert!(open.get(), "the first click opens the panel");
    handler(0);
    assert!(!open.get(), "the second click closes it again");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_now_handler_leaves_the_value_alone_without_a_browser_clock() {
    let value: Signal<String> = Signal::create(String::from("09:30"));
    let open: Signal<bool> = Signal::create(true);
    let handler: Rc<dyn Fn(u32)> = time_picker_now_handler::<u32>(value, open, None);
    handler(0);
    assert_eq!(
        value.get(),
        "09:30",
        "a host build has no clock, so the Now button leaves the value alone"
    );
    assert!(open.get(), "and it leaves the panel open as well");
}

#[test]
fn a_time_picker_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvTimePickerProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = HookContext::with(HookContext::default(), || euv_time_picker(bare));
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a time picker"
    );
}
