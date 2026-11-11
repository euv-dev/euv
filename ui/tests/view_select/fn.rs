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

fn option(value: &'static str, label: &'static str, group: &'static str) -> EuvSelectOption {
    EuvSelectOption {
        value,
        label,
        group,
        separator_before: false,
        disabled: false,
    }
}

fn fruit_options() -> Vec<EuvSelectOption> {
    vec![
        option("apple", "Apple", ""),
        option("banana", "Banana", ""),
        option("cherry", "Cherry", ""),
    ]
}

fn select_with(value: &str, options: Vec<EuvSelectOption>) -> VirtualNode {
    select_with_invalid(value, options, false)
}

fn select_with_invalid(value: &str, options: Vec<EuvSelectOption>, invalid: bool) -> VirtualNode {
    euv_select(props(EuvSelectProps {
        value: Signal::create(String::from(value)),
        options,
        placeholder: "Pick a fruit",
        label: "",
        id: "",
        name: "",
        required: false,
        invalid,
        disabled: false,
        align_item_with_trigger: false,
        on_change: None,
    }))
}

fn carries_text(entry: &AttributeEntry, name: &str, value: &str) -> bool {
    entry.get_name() == name && entry.text_value().as_deref() == Some(value)
}

fn carries_classes(entry: &AttributeEntry, names: &[&str]) -> bool {
    let classes: Vec<String> = entry.class_names();
    names
        .iter()
        .all(|name: &&str| classes.iter().any(|class: &String| class == name))
}

fn classes_of_tag(node: &VirtualNode, tag: &str, into: &mut Vec<AttributeEntry>) {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            attributes,
            children,
            ..
        } => {
            if format!("{own_tag:?}") == format!("Element(\"{tag}\")") {
                for attribute in attributes {
                    if attribute.get_name() == "class" {
                        into.push(attribute.clone());
                    }
                }
            }
            for child in children {
                classes_of_tag(child, tag, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                classes_of_tag(child, tag, into);
            }
        }
        _ => {}
    }
}

fn button_classes(node: &VirtualNode) -> Vec<AttributeEntry> {
    let mut into: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(node, "button", &mut into);
    into
}

fn attributes_of_tag(node: &VirtualNode, tag: &str, into: &mut Vec<Vec<AttributeEntry>>) {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            attributes,
            children,
            ..
        } => {
            if format!("{own_tag:?}") == format!("Element(\"{tag}\")") {
                into.push(attributes.clone());
            }
            for child in children {
                attributes_of_tag(child, tag, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attributes_of_tag(child, tag, into);
            }
        }
        _ => {}
    }
}

fn carries_no_handler(entry: &AttributeEntry) -> bool {
    let empty: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("onclick"),
        AttributeValue::Text(String::new()),
    );
    entry == &empty
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

fn collect_attribute_names(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                into.push(attribute.get_name().to_string());
            }
            for child in children {
                collect_attribute_names(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                collect_attribute_names(child, into);
            }
        }
        _ => {}
    }
}

#[test]
fn a_select_panel_contains_a_dedicated_scroll_viewport() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let mut attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut attributes);
    assert!(
        attributes.iter().any(|entries: &Vec<AttributeEntry>| {
            entries
                .iter()
                .any(|entry: &AttributeEntry| carries_text(entry, "data-slot", "select-viewport"))
        }),
        "the option panel carries an internal select-viewport slot, got {rendered:?}"
    );
}

#[test]
fn a_short_select_list_has_no_scroll_indicators() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let mut attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut attributes);
    assert!(
        attributes.iter().all(|entries: &Vec<AttributeEntry>| {
            !entries.iter().any(|entry: &AttributeEntry| {
                carries_text(entry, "data-slot", "select-scroll-up-button")
                    || carries_text(entry, "data-slot", "select-scroll-down-button")
            })
        }),
        "a short list has no scroll affordances, got {rendered:?}"
    );
}

#[test]
fn a_select_with_no_options_renders_the_empty_row() {
    let rendered: VirtualNode = select_with("", Vec::new());
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    assert!(
        divs.iter()
            .any(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_select_empty"])),
        "a select without options must show the empty row, got {divs:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("No options available")),
        "the empty row still carries its message, got {rendered:?}"
    );
}

#[test]
fn a_select_without_a_selection_shows_the_placeholder() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let mut spans: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "span", &mut spans);
    assert!(
        spans
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_placeholder"]
            )),
        "an unselected select shows the placeholder, got {spans:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Pick a fruit")),
        "the placeholder text reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn select_item_alignment_default_tracks_the_active_theme() {
    assert_eq!(
        EuvSelectProps::default().align_item_with_trigger,
        cfg!(feature = "theme-shadcn")
    );
}

#[test]
fn an_invalid_select_marks_its_root_trigger_and_error_style() {
    let rendered: VirtualNode = select_with_invalid("", fruit_options(), true);
    let mut div_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut div_attributes);
    assert!(
        div_attributes
            .first()
            .is_some_and(|attributes: &Vec<AttributeEntry>| {
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "data-invalid", "true"))
            }),
        "an invalid Select root reports data-invalid=true, got {div_attributes:?}"
    );
    let mut button_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut button_attributes);
    assert!(
        button_attributes
            .first()
            .is_some_and(|attributes: &Vec<AttributeEntry>| {
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "aria-invalid", "true"))
            }),
        "an invalid Select trigger reports aria-invalid=true, got {button_attributes:?}"
    );
    assert!(
        button_classes(&rendered)
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_trigger_invalid"]
            )),
        "an invalid Select trigger carries the destructive border class, got {rendered:?}"
    );
}

#[test]
fn a_valid_select_reports_no_invalid_state() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let mut div_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut div_attributes);
    let mut button_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut button_attributes);
    assert!(
        div_attributes
            .first()
            .is_some_and(|attributes: &Vec<AttributeEntry>| {
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "data-invalid", "false"))
            }),
        "a valid Select root reports data-invalid=false, got {div_attributes:?}"
    );
    assert!(
        button_attributes
            .first()
            .is_some_and(|attributes: &Vec<AttributeEntry>| {
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "aria-invalid", "false"))
            }),
        "a valid Select trigger reports aria-invalid=false, got {button_attributes:?}"
    );
    assert!(
        button_classes(&rendered)
            .first()
            .is_some_and(|entry: &AttributeEntry| !carries_classes(
                entry,
                &["c_euv_select_trigger_invalid"]
            )),
        "a valid Select trigger has no invalid style, got {rendered:?}"
    );
}

#[test]
fn a_select_with_a_selection_shows_the_option_label() {
    let rendered: VirtualNode = select_with("banana", fruit_options());
    let mut spans: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "span", &mut spans);
    assert!(
        spans
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_select_value"])),
        "a selected select shows the value, got {spans:?}"
    );
    assert!(
        texts(&rendered)
            .iter()
            .any(|text: &String| text.contains("Banana")),
        "the selected option's label reaches the trigger, got {rendered:?}"
    );
}

#[test]
fn a_select_trigger_controls_its_listbox_by_id() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let mut button_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut button_attributes);
    let trigger_attributes: &Vec<AttributeEntry> = button_attributes
        .first()
        .expect("the Select renders its trigger before options");
    let controls_id: Option<String> = trigger_attributes
        .iter()
        .find(|entry: &&AttributeEntry| entry.get_name() == "aria-controls")
        .and_then(|entry: &AttributeEntry| entry.text_value());
    let trigger_role: bool = trigger_attributes
        .iter()
        .any(|entry: &AttributeEntry| carries_text(entry, "role", "combobox"));
    let mut div_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut div_attributes);
    let listbox_attributes: Option<&Vec<AttributeEntry>> =
        div_attributes
            .iter()
            .find(|attributes: &&Vec<AttributeEntry>| {
                attributes.iter().any(|entry: &AttributeEntry| {
                    carries_text(entry, "data-slot", "select-viewport")
                })
            });
    let listbox_id: Option<String> =
        listbox_attributes.and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|entry: &&AttributeEntry| entry.get_name() == "id")
                .and_then(|entry: &AttributeEntry| entry.text_value())
        });
    let listbox_role: bool = listbox_attributes.is_some_and(|attributes: &Vec<AttributeEntry>| {
        attributes
            .iter()
            .any(|entry: &AttributeEntry| carries_text(entry, "role", "listbox"))
    });
    assert!(
        trigger_role,
        "the trigger reports the combobox role, got {trigger_attributes:?}"
    );
    assert!(
        listbox_role,
        "the viewport reports the listbox role, got {listbox_attributes:?}"
    );
    assert!(
        controls_id.is_some(),
        "the trigger references its listbox, got {trigger_attributes:?}"
    );
    assert_eq!(
        controls_id, listbox_id,
        "aria-controls resolves to the listbox id"
    );
}

#[test]
fn typeahead_matches_labels_case_insensitively_and_skips_disabled_options() {
    let cherry: EuvSelectOption = option("cherry", "Cherry", "");
    let banana: EuvSelectOption = option("banana", "Banana", "");
    assert!(cherry.matches_typeahead("CH"));
    assert!(banana.matches_typeahead("b"));
    let mut disabled_banana: EuvSelectOption = option("disabled-banana", "Banana", "");
    disabled_banana.disabled = true;
    assert!(!disabled_banana.matches_typeahead("ba"));
}

#[test]
fn a_named_required_select_renders_a_native_form_control() {
    let rendered: VirtualNode = euv_select(props(EuvSelectProps {
        value: Signal::create(String::from("banana")),
        options: fruit_options(),
        placeholder: "Pick a fruit",
        label: "Fruit",
        id: "fruit-select",
        name: "fruit",
        required: true,
        invalid: false,
        disabled: false,
        align_item_with_trigger: false,
        on_change: None,
    }));
    let mut input_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "input", &mut input_attributes);
    let hidden_control_complete: bool =
        input_attributes
            .first()
            .is_some_and(|attributes: &Vec<AttributeEntry>| {
                let has_text_type: bool = attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "type", "text"));
                let has_name: bool = attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "name", "fruit"));
                let has_value: bool = attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "value", "banana"));
                let is_required: bool = attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "required", "true"));
                let is_hidden_from_accessibility: bool = attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "aria-hidden", "true"));
                has_text_type
                    && has_name
                    && has_value
                    && is_required
                    && is_hidden_from_accessibility
            });
    assert!(
        hidden_control_complete,
        "a named required Select carries its native form value, got inputs {input_attributes:?} and tree {rendered:?}"
    );
    let mut button_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut button_attributes);
    assert!(
        button_attributes
            .first()
            .is_some_and(|attributes: &Vec<AttributeEntry>| {
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "id", "fruit-select"))
                    && attributes
                        .iter()
                        .any(|entry: &AttributeEntry| carries_text(entry, "aria-required", "true"))
            }),
        "the Select trigger carries its id and required state, got {button_attributes:?}"
    );
    let default_rendered: VirtualNode = select_with("apple", fruit_options());
    let mut default_inputs: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&default_rendered, "input", &mut default_inputs);
    assert!(
        default_inputs.is_empty(),
        "an unnamed optional Select renders no form input"
    );
}

#[test]
fn a_select_value_carries_its_part_slot() {
    let rendered: VirtualNode = select_with("banana", fruit_options());
    let mut span_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "span", &mut span_attributes);
    assert!(
        span_attributes
            .iter()
            .any(|attributes: &Vec<AttributeEntry>| {
                attributes
                    .iter()
                    .any(|entry: &AttributeEntry| carries_text(entry, "data-slot", "select-value"))
            }),
        "the displayed value exposes the select-value slot, got {span_attributes:?}"
    );
}

#[test]
fn the_option_panel_stays_mounted_while_the_select_is_closed() {
    let rendered: VirtualNode = select_with("", fruit_options());
    let buttons: Vec<AttributeEntry> = button_classes(&rendered);
    assert_eq!(
        buttons.len(),
        4,
        "the trigger plus one button per option stay in the tree while the panel is \
         closed, got {rendered:?}"
    );
    assert!(
        buttons
            .first()
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_trigger"]
            )),
        "the first button is the trigger, got {buttons:?}"
    );
}

#[test]
fn the_selected_option_carries_the_active_class() {
    let rendered: VirtualNode = select_with("cherry", fruit_options());
    let buttons: Vec<AttributeEntry> = button_classes(&rendered);
    assert!(
        buttons
            .get(3)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_option_active"]
            )),
        "the picked option is filled with the accent, got {buttons:?}"
    );
    assert!(
        buttons
            .get(1)
            .is_some_and(|entry: &AttributeEntry| carries_classes(entry, &["c_euv_select_option"])),
        "the other options keep the neutral look, got {buttons:?}"
    );
}

#[test]
fn grouped_options_render_a_caption_row_for_each_group() {
    let options: Vec<EuvSelectOption> = vec![
        option("alpha", "Alpha", ""),
        option("beta", "Beta", "Letters"),
        option("gamma", "Gamma", "Letters"),
        option("delta", "Delta", "Greek"),
    ];
    let rendered: VirtualNode = select_with("", options);
    let mut divs: Vec<AttributeEntry> = Vec::new();
    classes_of_tag(&rendered, "div", &mut divs);
    let captions: usize = divs
        .iter()
        .filter(|entry: &&AttributeEntry| carries_classes(entry, &["c_euv_select_group"]))
        .count();
    assert_eq!(captions, 2, "one label row per group, got {divs:?}");
    let mut div_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut div_attributes);
    let groups: Vec<&Vec<AttributeEntry>> = div_attributes
        .iter()
        .filter(|attributes: &&Vec<AttributeEntry>| {
            attributes
                .iter()
                .any(|entry: &AttributeEntry| carries_text(entry, "data-slot", "select-group"))
        })
        .collect();
    let labels: usize = div_attributes
        .iter()
        .filter(|attributes: &&Vec<AttributeEntry>| {
            attributes
                .iter()
                .any(|entry: &AttributeEntry| carries_text(entry, "data-slot", "select-label"))
        })
        .count();
    let label_ids: Vec<String> = div_attributes
        .iter()
        .filter_map(|attributes: &Vec<AttributeEntry>| {
            let has_label_slot: bool = attributes
                .iter()
                .any(|entry: &AttributeEntry| carries_text(entry, "data-slot", "select-label"));
            if has_label_slot {
                attributes
                    .iter()
                    .find(|entry: &&AttributeEntry| entry.get_name() == "id")
                    .and_then(|entry: &AttributeEntry| entry.text_value())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(
        groups.len(),
        2,
        "one semantic group per category, got {groups:?}"
    );
    assert_eq!(
        labels, 2,
        "each group has a SelectLabel slot, got {div_attributes:?}"
    );
    assert!(
        groups.iter().all(|attributes: &&Vec<AttributeEntry>| {
            let labelled_by: Option<String> = attributes
                .iter()
                .find(|entry: &&AttributeEntry| entry.get_name() == "aria-labelledby")
                .and_then(|entry: &AttributeEntry| entry.text_value());
            attributes
                .iter()
                .any(|entry: &AttributeEntry| carries_text(entry, "role", "group"))
                && labelled_by.is_some_and(|id: String| label_ids.contains(&id))
        }),
        "each group references its own accessible label, got {groups:?}"
    );
    let rendered_texts: Vec<String> = texts(&rendered);
    assert!(
        rendered_texts
            .iter()
            .any(|text: &String| text.contains("Letters"))
            && rendered_texts
                .iter()
                .any(|text: &String| text.contains("Greek")),
        "both group captions reach the panel, got {rendered:?}"
    );
}

#[test]
fn an_option_can_be_preceded_by_an_accessible_separator() {
    let rendered: VirtualNode = select_with(
        "",
        vec![
            option("apple", "Apple", ""),
            EuvSelectOption {
                value: "banana",
                label: "Banana",
                group: "",
                separator_before: true,
                disabled: false,
            },
        ],
    );
    let mut div_attributes: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "div", &mut div_attributes);
    let separator_attributes: Option<&Vec<AttributeEntry>> =
        div_attributes
            .iter()
            .find(|attributes: &&Vec<AttributeEntry>| {
                attributes.iter().any(|attribute: &AttributeEntry| {
                    *attribute
                        == AttributeEntry::new(
                            Cow::Borrowed("data-slot"),
                            AttributeValue::Text(String::from("select-separator")),
                        )
                })
            });
    assert!(
        separator_attributes.is_some_and(|attributes: &Vec<AttributeEntry>| {
            attributes.iter().any(|attribute: &AttributeEntry| {
                *attribute
                    == AttributeEntry::new(
                        Cow::Borrowed("role"),
                        AttributeValue::Text(String::from("separator")),
                    )
            }) && attributes.iter().any(|attribute: &AttributeEntry| {
                *attribute
                    == AttributeEntry::new(
                        Cow::Borrowed("class"),
                        AttributeValue::Css(Css::new(
                            String::from("c_euv_select_separator"),
                            String::new(),
                            Vec::new(),
                            Vec::new(),
                        )),
                    )
            })
        }),
        "the requested separator is rendered with its slot, role and class, got {rendered:?}"
    );
}

#[test]
fn a_disabled_option_carries_the_disabled_class_and_cannot_be_picked() {
    let options: Vec<EuvSelectOption> = vec![
        option("apple", "Apple", ""),
        EuvSelectOption {
            value: "banana",
            label: "Banana",
            group: "",
            separator_before: false,
            disabled: true,
        },
    ];
    let rendered: VirtualNode = select_with("", options);
    let buttons: Vec<AttributeEntry> = button_classes(&rendered);
    assert!(
        buttons
            .get(2)
            .is_some_and(|entry: &AttributeEntry| carries_classes(
                entry,
                &["c_euv_select_option_disabled"]
            )),
        "a disabled option is drawn muted, got {buttons:?}"
    );
    let mut names: Vec<String> = Vec::new();
    collect_attribute_names(&rendered, &mut names);
    assert_eq!(
        names
            .iter()
            .filter(|name: &&String| name.as_str() == "onclick")
            .count(),
        3,
        "the trigger and every option carry a click slot, got {names:?}"
    );
    let mut buttons: Vec<Vec<AttributeEntry>> = Vec::new();
    attributes_of_tag(&rendered, "button", &mut buttons);
    let disabled_handler: Option<&AttributeEntry> =
        buttons.get(2).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        disabled_handler.is_some_and(carries_no_handler),
        "the disabled option carries no click handler, got {buttons:?}"
    );
    let enabled_handler: Option<&AttributeEntry> =
        buttons.get(1).and_then(|attributes: &Vec<AttributeEntry>| {
            attributes
                .iter()
                .find(|attribute: &&AttributeEntry| attribute.get_name() == "onclick")
        });
    assert!(
        enabled_handler.is_some_and(|entry: &AttributeEntry| !carries_no_handler(entry)),
        "the enabled option carries a click handler, got {buttons:?}"
    );
}

#[test]
fn a_select_without_props_falls_back_to_defaults() {
    let bare: VirtualNode<EuvSelectProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_select(bare);
    assert!(
        matches!(rendered, VirtualNode::Element { .. }),
        "defaults still render a select"
    );
}
