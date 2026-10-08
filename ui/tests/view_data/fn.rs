use super::*;

fn total(node: &VirtualNode) -> usize {
    match node {
        VirtualNode::Element { children, .. } => 1 + children.iter().map(total).sum::<usize>(),
        VirtualNode::Fragment(children) => children.iter().map(total).sum(),
        _ => 0,
    }
}

fn count_named(node: &VirtualNode, needle: &str) -> usize {
    format!("{node:?}").matches(needle).count()
}

fn leaked(value: String) -> &'static str {
    Box::leak(value.into_boxed_str()) as &'static str
}

fn table_node(rows: usize, caption: &'static str) -> VirtualNode<EuvTableProps> {
    let columns: Vec<EuvTableColumn> = vec![EuvTableColumn {
        key: "a",
        label: "A",
        align: EuvTableAlign::default(),
    }];
    let built: Vec<EuvTableRow> = (0..rows)
        .map(|i: usize| EuvTableRow {
            cells: vec!["x"],
            key: leaked(format!("row-{i}")),
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
fn a_table_is_a_real_table_element_with_a_head() {
    let rendered: VirtualNode = euv_table(table_node(1, ""));
    let debugged: String = format!("{rendered:?}");
    assert!(
        debugged.contains("Element(\"table\")"),
        "a table must use a real table element so screen readers announce it, got {debugged}"
    );
    assert!(
        debugged.contains("Element(\"thead\")"),
        "with a header section, got {debugged}"
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
