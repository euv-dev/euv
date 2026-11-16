use super::*;

fn total(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => 1 + children.iter().map(total).sum::<usize>(),
        VirtualNode::Fragment(children) => children.iter().map(total).sum(),
        _ => 0,
    }
}

fn class_values(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let mut values: Vec<String> = Vec::new();
            for attribute in attributes.iter() {
                if attribute.get_name() == "class" {
                    values.extend(attribute.class_names());
                }
            }
            for child in children.iter() {
                values.extend(class_values(child));
            }
            values
        }
        VirtualNode::Fragment(children) => children.iter().flat_map(class_values).collect(),
        _ => Vec::new(),
    }
}

fn count_named(node: &VirtualNode, needle: &str) -> usize {
    format!("{node:?}").matches(needle).count()
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    match node {
        VirtualNode::Element { children, .. } => {
            let mut out: Vec<VirtualNode> = vec![node.clone()];
            for child in children {
                out.extend(elements(child));
            }
            out
        }
        VirtualNode::Fragment(children) => {
            let mut out: Vec<VirtualNode> = Vec::new();
            for child in children {
                out.extend(elements(child));
            }
            out
        }
        _ => Vec::new(),
    }
}

fn slot_values(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "data-slot"
                    && let Some(value) = entry.text_value()
                {
                    out.push(value);
                }
            }
        }
    }
    out
}

fn values_for_attribute(node: &VirtualNode, name: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for attribute in attributes.iter() {
                if attribute.get_name() == name
                    && let Some(value) = attribute.text_value()
                {
                    out.push(value);
                }
            }
        }
    }
    out
}

fn leaked(value: String) -> &'static str {
    Box::leak(value.into_boxed_str()) as &'static str
}

fn table_node(rows: usize, caption: &'static str) -> VirtualNode<EuvTableProps> {
    let columns: Vec<EuvTableColumn> = vec![EuvTableColumn {
        key: "a",
        label: "A".into(),
        align: EuvTableAlign::default(),
    }];
    let built: Vec<EuvTableRow> = (0..rows)
        .map(|i: usize| EuvTableRow {
            cells: vec!["x".into()],
            key: leaked(format!("row-{i}")).into(),
        })
        .collect();
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTableProps {
            columns,
            rows: built,
            caption,
            zebra: false,
            class: Css::default(),
        })),
    }
}

#[cfg(feature = "theme-shadcn")]
fn aligned_table_node() -> VirtualNode<EuvTableProps> {
    let columns: Vec<EuvTableColumn> = vec![
        EuvTableColumn {
            key: "left",
            label: "Left".into(),
            align: EuvTableAlign::Left,
        },
        EuvTableColumn {
            key: "center",
            label: "Center".into(),
            align: EuvTableAlign::Center,
        },
        EuvTableColumn {
            key: "right",
            label: "Right".into(),
            align: EuvTableAlign::Right,
        },
    ];
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTableProps {
            columns,
            rows: vec![EuvTableRow {
                cells: vec!["a".into(), "b".into(), "c".into()],
                key: "aligned-row".into(),
            }],
            caption: "Alignment",
            zebra: false,
            class: Css::default(),
        })),
    }
}

fn steps_node(count: usize, current: usize) -> VirtualNode<EuvStepsProps> {
    let built: Vec<EuvStep> = (0..count)
        .map(|i: usize| EuvStep {
            title: leaked(format!("S{i}")),
            description: "",
        })
        .collect();
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvStepsProps {
            steps: built,
            current: Signal::create(current),
        })),
    }
}

fn radio_node(options: usize) -> VirtualNode<EuvRadioGroupProps> {
    let built: Vec<EuvRadioOption> = (0..options)
        .map(|i: usize| EuvRadioOption {
            value: leaked(format!("v{i}")),
            label: leaked(format!("L{i}")),
        })
        .collect();
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvRadioGroupProps {
            name: "group",
            options: built,
            value: Signal::create(String::from("v0")),
            label: "Pick",
        })),
    }
}

#[test]
fn a_table_accepts_application_content_and_classes() {
    let content: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("caption")),
        attributes: Vec::new(),
        children: vec![VirtualNode::from("Accessible account table")],
        key: None,
        props: None,
    };
    let node: VirtualNode<EuvTableProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: vec![content],
        key: None,
        props: Some(Box::new(EuvTableProps {
            class: Css {
                name: String::from("commerce-table"),
                ..Css::default()
            },
            ..EuvTableProps::default()
        })),
    };
    let rendered: VirtualNode = euv_table(node);
    let classes: Vec<String> = class_values(&rendered);
    let debugged: String = format!("{rendered:?}");
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "commerce-table")
    );
    assert!(debugged.contains("Accessible account table"));
}

#[test]
fn a_table_is_a_real_table_element_with_a_head() {
    let rendered: VirtualNode = euv_table(table_node(1, ""));
    let debugged: String = format!("{rendered:?}");
    let slots: Vec<String> = slot_values(&rendered);
    assert!(
        slots.iter().any(|slot: &String| slot == "table-container"),
        "the scroll wrapper must carry the reference's table container slot, got {slots:?}"
    );
    assert!(
        debugged.contains("Element(\"table\")")
            && slots.iter().any(|slot: &String| slot == "table-row"),
        "table rows carry the reference's interactive and border slot, got {slots:?}"
    );
    assert!(
        debugged.contains("Element(\"thead\")")
            && slots.iter().any(|slot: &String| slot == "table-header"),
        "with a header section, got {slots:?}"
    );
}

#[cfg(feature = "theme-shadcn")]
#[test]
fn the_shadcn_table_uses_reference_type_alignment_and_caption_classes() {
    let rendered: VirtualNode = euv_table(aligned_table_node());
    let classes: Vec<String> = class_values(&rendered);
    for class_name in [
        "c_euv_table",
        "c_euv_table_body",
        "c_euv_table_header_cell",
        "c_euv_table_header_cell_center",
        "c_euv_table_header_cell_right",
        "c_euv_table_cell",
        "c_euv_table_cell_center",
        "c_euv_table_cell_right",
        "c_euv_table_caption",
    ] {
        assert!(
            classes.iter().any(|class: &String| class == class_name),
            "the shadcn table must apply {class_name}, got {classes:?}"
        );
    }
    let footer: VirtualNode = euv_table_footer(VirtualNode::Empty);
    assert!(
        class_values(&footer)
            .iter()
            .any(|class: &String| class == "c_euv_table_footer")
    );
}

#[test]
fn each_row_adds_elements_to_the_table() {
    let none: VirtualNode = euv_table(table_node(0, ""));
    let one: VirtualNode = euv_table(table_node(1, ""));
    let two: VirtualNode = euv_table(table_node(2, ""));
    assert_ne!(none, one, "a row of data must reach the tree");
    assert!(
        total(&two) > total(&one) && total(&one) > total(&none),
        "each row adds elements, got {} / {} / {}",
        total(&none),
        total(&one),
        total(&two)
    );
}

#[test]
fn a_table_with_a_caption_differs_from_one_without() {
    let captioned: VirtualNode = euv_table(table_node(1, "Numbers"));
    let plain: VirtualNode = euv_table(table_node(1, ""));
    assert_ne!(
        captioned, plain,
        "a caption is a distinct element, not a class swap"
    );
}

#[test]
fn a_table_without_props_renders_its_header_only() {
    let bare: VirtualNode<EuvTableProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_table(bare);
    let debugged: String = format!("{rendered:?}");
    assert!(
        debugged.contains("Element(\"table\")"),
        "the table shell renders even with nothing to show, got {debugged}"
    );
    assert!(
        count_named(&rendered, "Element(\"tr\")") <= 1,
        "a header row and nothing else, got {debugged}"
    );
}

#[test]
fn a_step_renders_one_rail_per_step() {
    let rendered: VirtualNode = euv_steps(steps_node(2, 0));
    assert!(
        total(&rendered) > 0,
        "a stepper always renders, got {rendered:?}"
    );
}

#[test]
fn each_step_adds_elements_to_the_rail() {
    let one: VirtualNode = euv_steps(steps_node(1, 0));
    let two: VirtualNode = euv_steps(steps_node(2, 0));
    let three: VirtualNode = euv_steps(steps_node(3, 0));
    assert!(
        total(&two) > total(&one) && total(&three) > total(&two),
        "the rail grows with the step count, got {} / {} / {}",
        total(&one),
        total(&two),
        total(&three)
    );
}

#[test]
fn moving_the_current_step_changes_the_rendered_tree() {
    let first: VirtualNode = euv_steps(steps_node(2, 0));
    let second: VirtualNode = euv_steps(steps_node(2, 1));
    assert_ne!(
        first, second,
        "the progress marker moves, so the two states must be distinguishable"
    );
}

#[test]
fn a_stepper_without_props_renders_its_frame() {
    let bare: VirtualNode<EuvStepsProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_steps(bare);
    assert!(total(&rendered) > 0, "the rail renders even with no steps");
}

#[test]
fn a_radio_group_renders_one_control_per_option() {
    let one: VirtualNode = euv_radio(radio_node(1));
    let two: VirtualNode = euv_radio(radio_node(2));
    assert!(
        count_named(&two, "Element(\"input\")") > count_named(&one, "Element(\"input\")"),
        "each option adds a real radio input, got {two:?}"
    );
}

#[test]
fn a_radio_group_with_no_options_still_renders_its_label() {
    let rendered: VirtualNode = euv_radio(radio_node(0));
    assert!(
        format!("{rendered:?}").contains("Pick"),
        "the group label is a static text node and stays, got {rendered:?}"
    );
}

#[test]
fn a_radio_group_with_options_renders_more_than_an_empty_one() {
    let filled: VirtualNode = euv_radio(radio_node(2));
    let empty: VirtualNode = euv_radio(radio_node(0));
    assert!(
        total(&filled) > total(&empty),
        "options add their own controls, got {} vs {}",
        total(&filled),
        total(&empty)
    );
}

#[test]
fn a_radio_group_without_props_renders_without_panicking() {
    let bare: VirtualNode<EuvRadioGroupProps> = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = euv_radio(bare);
    assert!(
        total(&rendered) > 0,
        "the group frame renders, got {rendered:?}"
    );
}

fn tree_node(default_expanded: bool) -> VirtualNode<EuvTreeProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTreeProps {
            nodes: vec![EuvTreeNode {
                label: "Parent",
                value: "parent",
                children: vec![EuvTreeNode {
                    label: "Child",
                    value: "child",
                    children: Vec::new(),
                }],
            }],
            selected: Signal::create(String::new()),
            on_select: None,
            default_expanded,
        })),
    }
}

#[test]
fn a_tree_exposes_nested_roles_slots_and_a_single_roving_tab_stop() {
    let tree: VirtualNode<EuvTreeProps> = tree_node(true);
    let rendered: VirtualNode = euv_tree(tree);
    let roles: Vec<String> = values_for_attribute(&rendered, "role");
    let slots: Vec<String> = slot_values(&rendered);
    let tabindex_values: Vec<String> = values_for_attribute(&rendered, "tabindex");
    let levels: Vec<String> = values_for_attribute(&rendered, "aria-level");
    let expanded_values: Vec<String> = values_for_attribute(&rendered, "aria-expanded");
    let visible_values: Vec<String> = values_for_attribute(&rendered, "data-visible");
    assert!(roles.contains(&"tree".to_string()));
    assert!(roles.contains(&"group".to_string()));
    assert_eq!(
        roles
            .iter()
            .filter(|role: &&String| *role == "treeitem")
            .count(),
        2
    );
    for slot in [
        "tree",
        "tree-item",
        "tree-item-row",
        "tree-group",
        "tree-toggle",
    ] {
        assert!(
            slots.iter().any(|value: &String| value == slot),
            "missing {slot}"
        );
    }
    assert_eq!(
        tabindex_values
            .iter()
            .filter(|value: &&String| *value == "0")
            .count(),
        1
    );
    assert_eq!(tabindex_values.len(), 2);
    assert_eq!(levels, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(
        expanded_values,
        vec!["true".to_string(), "false".to_string()]
    );
    assert_eq!(visible_values, vec!["true".to_string(), "true".to_string()]);
}

#[test]
fn a_collapsed_tree_excludes_children_from_the_roving_sequence() {
    let rendered: VirtualNode = euv_tree(tree_node(false));
    let tabindex_values: Vec<String> = values_for_attribute(&rendered, "tabindex");
    let visible_values: Vec<String> = values_for_attribute(&rendered, "data-visible");
    let expanded_values: Vec<String> = values_for_attribute(&rendered, "aria-expanded");
    assert_eq!(tabindex_values, vec!["0".to_string(), "-1".to_string()]);
    assert_eq!(
        visible_values,
        vec!["true".to_string(), "false".to_string()]
    );
    assert_eq!(
        expanded_values,
        vec!["false".to_string(), "false".to_string()]
    );
}

#[test]
fn tree_selection_and_expansion_helpers_update_state() {
    let selected: Signal<String> = Signal::create(String::new());
    select_tree_node(selected, None, "child");
    assert_eq!(selected.get(), "child");
    let mut expanded: Vec<String> = vec!["parent".to_string()];
    toggle_expanded_value(&mut expanded, "parent");
    assert!(expanded.is_empty());
    toggle_expanded_value(&mut expanded, "parent");
    assert_eq!(expanded, vec!["parent".to_string()]);
}
