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

fn count_elements(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => {
            1 + children.iter().map(count_elements).sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(count_elements).sum(),
        _ => 0,
    }
}

fn count_with_class(node: &VirtualNode, class: &str, into: &mut usize) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class"
                    && attribute
                        .class_names()
                        .iter()
                        .any(|name: &String| name == class)
                {
                    *into += 1;
                }
            }
            for child in children {
                count_with_class(child, class, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                count_with_class(child, class, into);
            }
        }
        _ => {}
    }
}

fn count_class(node: &VirtualNode, class: &str) -> usize {
    let mut total: usize = 0;
    count_with_class(node, class, &mut total);
    total
}

fn count_any_class(node: &VirtualNode, classes: &[&str]) -> usize {
    let mut total: usize = 0;
    for class in classes {
        total += count_class(node, class);
    }
    total
}

fn assert_grows_by_one_per_item(
    small: &VirtualNode,
    large: &VirtualNode,
    small_count: usize,
    large_count: usize,
    per_item: usize,
    item_name: &str,
) {
    let small_size: usize = count_elements(small);
    let large_size: usize = count_elements(large);
    assert_eq!(
        large_size - small_size,
        (large_count - small_count) * per_item,
        "every {item_name} must build exactly {per_item} element(s) and no more, got \
         {small_size} for {small_count} and {large_size} for {large_count}"
    );
}

#[test]
fn a_select_builds_one_element_per_option() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let options: Vec<EuvSelectOption> = (0..count)
            .map(|_| EuvSelectOption {
                value: "v",
                label: "l",
                group: "",
                disabled: false,
            })
            .collect();
        euv_select(props(EuvSelectProps {
            value: Signal::create(String::new()),
            options,
            placeholder: "pick",
            label: "",
            disabled: false,
            on_change: None,
        }))
    };
    assert_grows_by_one_per_item(&render(50), &render(150), 50, 150, 1, "option");
    assert_eq!(
        count_class(&render(40), "c_euv_select_option"),
        40,
        "every option must be rendered once and only once"
    );
}

#[test]
fn a_checkbox_group_builds_four_elements_per_option() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let options: Vec<EuvCheckboxGroupOption> = (0..count)
            .map(|_| EuvCheckboxGroupOption {
                value: "v",
                label: "l",
                disabled: false,
            })
            .collect();
        euv_checkbox_group(props(EuvCheckboxGroupProps {
            options,
            values: Signal::create(Vec::new()),
            label: "",
            on_change: None,
        }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        4,
        "option row (the label, the hidden input, the box marker and the text)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_checkbox_group_item"),
        40,
        "every option row must be rendered once and only once"
    );
}

#[test]
fn a_segmented_control_builds_one_element_per_option() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let options: Vec<EuvSegmentedOption> = (0..count)
            .map(|_| EuvSegmentedOption {
                value: "v",
                label: "l",
                disabled: false,
            })
            .collect();
        euv_segmented(props(EuvSegmentedProps {
            options,
            value: Signal::create(String::new()),
            on_change: None,
        }))
    };
    assert_grows_by_one_per_item(&render(50), &render(150), 50, 150, 1, "segment");
}

#[test]
fn a_list_builds_three_elements_per_row() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let items: Vec<EuvListItem> = (0..count)
            .map(|_| EuvListItem {
                title: "t",
                desc: "",
                meta: "",
            })
            .collect();
        euv_list(props(EuvListProps {
            items,
            empty_text: "",
        }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        3,
        "list row (the row, its body and its title)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_list_item"),
        40,
        "every row must be rendered once and only once"
    );
}

#[test]
fn a_transfer_builds_three_elements_per_row() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let source: Vec<EuvTransferItem> = (0..count)
            .map(|_| EuvTransferItem {
                value: "v",
                label: "l",
                disabled: false,
            })
            .collect();
        euv_transfer(props(EuvTransferProps {
            source,
            target: Vec::new(),
            selected: Signal::create(Vec::new()),
            source_title: "s",
            target_title: "t",
            on_move_to_target: None,
            on_move_to_source: None,
        }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        3,
        "transfer row (the row, its box marker and its text)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_transfer_item"),
        40,
        "every row must be rendered once and only once"
    );
}

#[test]
fn a_tree_builds_four_elements_per_node() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let nodes: Vec<EuvTreeNode> = (0..count)
            .map(|_| EuvTreeNode {
                label: "l",
                value: "v",
                children: Vec::new(),
            })
            .collect();
        euv_tree(props(EuvTreeProps {
            nodes,
            selected: Signal::create(String::new()),
            on_select: None,
            default_expanded: false,
        }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        4,
        "tree node (the node, its row, its toggle and its label)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_tree_node_row"),
        40,
        "every node must render exactly one row"
    );
}

#[test]
fn a_cascader_builds_one_element_per_option() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let options: Vec<EuvCascaderOption> = (0..count)
            .map(|_| EuvCascaderOption {
                value: "v",
                label: "l",
                children: Vec::new(),
            })
            .collect();
        euv_cascader(props(EuvCascaderProps {
            options,
            value: Signal::create(Vec::new()),
            placeholder: "pick",
            label: "",
            on_select: None,
        }))
    };
    assert_grows_by_one_per_item(&render(50), &render(150), 50, 150, 1, "cascader option");
    assert_eq!(
        count_class(&render(40), "c_euv_cascader_option"),
        40,
        "every option must be rendered once and only once"
    );
}

#[test]
fn a_time_picker_renders_its_hours_and_minutes_once_each() {
    let rendered: VirtualNode = euv_time_picker(props(EuvTimePickerProps {
        value: Signal::create(String::new()),
        placeholder: "pick",
        label: "",
        min_hour: 0,
        max_hour: 23,
        minute_step: 15,
        on_change: None,
    }));
    assert_eq!(
        count_class(&rendered, "c_euv_time_picker_cell"),
        24 + 4,
        "a picker must render every hour once and every minute step once"
    );
}

#[test]
fn an_auto_complete_builds_one_element_per_suggestion() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let options: Vec<EuvAutoCompleteOption> = (0..count)
            .map(|_| EuvAutoCompleteOption {
                value: "v",
                label: "l",
            })
            .collect();
        euv_auto_complete(props(EuvAutoCompleteProps {
            options,
            value: Signal::create(String::new()),
            placeholder: "type",
            label: "",
            on_change: None,
            on_select: None,
        }))
    };
    assert_grows_by_one_per_item(&render(50), &render(150), 50, 150, 1, "suggestion");
    assert_eq!(
        count_any_class(
            &render(40),
            &[
                "c_euv_auto_complete_option",
                "c_euv_auto_complete_option_active"
            ]
        ),
        40,
        "every suggestion must be rendered once and only once"
    );
    assert_eq!(
        count_class(&render(40), "c_euv_auto_complete_option_active"),
        1,
        "exactly one suggestion may carry the active marker"
    );
}

#[test]
fn a_mentions_field_builds_one_element_per_suggestion() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let items: Vec<EuvMentionItem> = (0..count)
            .map(|_| EuvMentionItem {
                value: "l",
                label: "l",
            })
            .collect();
        euv_mentions(props(EuvMentionsProps {
            items,
            value: Signal::create(String::from("@l")),
            placeholder: "type",
            label: "",
            on_select: None,
        }))
    };
    assert_grows_by_one_per_item(&render(50), &render(150), 50, 150, 1, "suggestion");
    assert_eq!(
        count_any_class(
            &render(40),
            &["c_euv_mentions_option", "c_euv_mentions_option_active"]
        ),
        40,
        "every suggestion must be rendered once and only once"
    );
    assert_eq!(
        count_class(&render(40), "c_euv_mentions_option_active"),
        1,
        "exactly one suggestion may carry the active marker"
    );
}

#[test]
fn a_tree_select_builds_four_elements_per_node() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let nodes: Vec<EuvTreeNode> = (0..count)
            .map(|_| EuvTreeNode {
                label: "l",
                value: "v",
                children: Vec::new(),
            })
            .collect();
        euv_tree_select(props(EuvTreeSelectProps {
            nodes,
            value: Signal::create(String::new()),
            placeholder: "pick",
            label: "",
            on_select: None,
        }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        4,
        "tree node (the node, its row, its toggle and its label)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_tree_node_row"),
        40,
        "every node must render exactly one row"
    );
}

#[test]
fn a_descriptions_grid_builds_three_elements_per_pair() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let items: Vec<EuvDescriptionsItem> = (0..count)
            .map(|_| EuvDescriptionsItem {
                label: "l",
                value: "v",
            })
            .collect();
        euv_descriptions(props(EuvDescriptionsProps { items, columns: 1 }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        3,
        "description pair (the cell, its label and its value)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_descriptions_item"),
        40,
        "every pair must be rendered once and only once"
    );
}

#[test]
fn a_date_picker_holds_a_fixed_six_week_grid() {
    let rendered: VirtualNode = euv_date_picker(props(EuvDatePickerProps {
        value: Signal::create(String::new()),
        placeholder: "pick",
        label: "",
        min: "",
        max: "",
        on_change: None,
    }));
    assert_eq!(
        count_any_class(&rendered, &CALENDAR_DAY_CLASSES),
        42,
        "the day grid must always hold six weeks of seven days"
    );
}

const CALENDAR_DAY_CLASSES: [&str; 4] = [
    "c_euv_calendar_day",
    "c_euv_calendar_day_muted",
    "c_euv_calendar_day_today",
    "c_euv_calendar_day_selected",
];

fn day_cells(node: &VirtualNode, into: &mut Vec<(Vec<String>, Option<String>)>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            key,
            ..
        } => {
            let mut names: Vec<String> = Vec::new();
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    names = attribute.class_names();
                }
            }
            let is_day_cell: bool = names
                .iter()
                .any(|name: &String| CALENDAR_DAY_CLASSES.contains(&name.as_str()));
            if is_day_cell {
                into.push((names, key.clone()));
            }
            for child in children {
                day_cells(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                day_cells(child, into);
            }
        }
        _ => {}
    }
}

#[test]
fn a_calendar_day_carries_exactly_one_class_and_no_repeated_token() {
    let rendered: VirtualNode = euv_date_picker(props(EuvDatePickerProps {
        value: Signal::create(String::new()),
        placeholder: "pick",
        label: "",
        min: "",
        max: "",
        on_change: None,
    }));
    let mut cells: Vec<(Vec<String>, Option<String>)> = Vec::new();
    day_cells(&rendered, &mut cells);
    assert_eq!(cells.len(), 42, "every cell of the grid has to be found");
    for (names, _) in cells.iter() {
        let family_count: usize = names
            .iter()
            .filter(|name: &&String| CALENDAR_DAY_CLASSES.contains(&name.as_str()))
            .count();
        assert_eq!(
            family_count, 1,
            "a day cell carries exactly one class of the family, got {names:?}"
        );
        let mut seen: Vec<&String> = Vec::new();
        for name in names.iter() {
            assert!(
                !seen.contains(&name),
                "a day cell must not repeat a class token, got {names:?}"
            );
            seen.push(name);
        }
    }
}

#[test]
fn a_calendar_grid_keys_its_cells_uniquely() {
    let rendered: VirtualNode = euv_date_picker(props(EuvDatePickerProps {
        value: Signal::create(String::new()),
        placeholder: "pick",
        label: "",
        min: "",
        max: "",
        on_change: None,
    }));
    let mut cells: Vec<(Vec<String>, Option<String>)> = Vec::new();
    day_cells(&rendered, &mut cells);
    let mut keys: Vec<String> = Vec::new();
    for (_, key) in cells.iter() {
        let key: &String = key.as_ref().expect("a day cell must carry a key");
        assert!(
            !keys.contains(key),
            "a day cell key must be unique, {key} is used twice"
        );
        keys.push(key.clone());
    }
    assert_eq!(keys.len(), 42, "every cell of the grid has to be keyed");
}

#[test]
fn a_carousel_builds_three_elements_per_slide() {
    let render: fn(usize) -> VirtualNode = |count: usize| -> VirtualNode {
        let slides: Vec<EuvCarouselSlide> = (0..count)
            .map(|_| EuvCarouselSlide {
                title: "t",
                desc: "d",
            })
            .collect();
        euv_carousel(props(EuvCarouselProps {
            slides,
            active: Signal::create(0),
            on_change: None,
        }))
    };
    assert_grows_by_one_per_item(
        &render(50),
        &render(150),
        50,
        150,
        4,
        "carousel slide (the slide, its title, its description and its dot)",
    );
    assert_eq!(
        count_class(&render(40), "c_euv_carousel_slide"),
        40,
        "every slide must be rendered once and only once"
    );
}
