use super::*;

fn group_node(
    items: Vec<EuvToggleGroupItem>,
    values: Vec<String>,
) -> VirtualNode<EuvToggleGroupProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvToggleGroupProps {
            items,
            values: Signal::create(values),
            multiple: false,
            collapsible: false,
            variant: EuvToggleVariant::Default,
            size: EuvToggleSize::Default,
            on_change: None,
        })),
    }
}

fn item(value: &'static str, label: &'static str) -> EuvToggleGroupItem {
    EuvToggleGroupItem {
        value,
        label,
        disabled: false,
    }
}

fn render(items: Vec<EuvToggleGroupItem>, values: Vec<String>) -> VirtualNode {
    euv_toggle_group(group_node(items, values))
}

fn walk(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                walk(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                walk(child, out);
            }
        }
        _ => {}
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    walk(node, &mut out);
    out
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                out.push(entry.get_name().to_string());
            }
        }
    }
    out
}

fn class_names(node: &VirtualNode) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for element in elements(node) {
        if let VirtualNode::Element { attributes, .. } = &element {
            for entry in attributes.iter() {
                if entry.get_name() == "class" {
                    out.extend(entry.class_names());
                }
            }
        }
    }
    out
}

fn button_count(node: &VirtualNode) -> usize {
    let mut count: usize = 0;
    for element in elements(node) {
        if let VirtualNode::Element { tag, .. } = &element
            && format!("{tag:?}").contains("button")
        {
            count += 1;
        }
    }
    count
}

#[test]
fn a_group_renders_one_button_per_entry() {
    let rendered: VirtualNode = render(
        vec![item("left", "Left"), item("right", "Right")],
        Vec::new(),
    );
    assert_eq!(button_count(&rendered), 2, "one button per entry");
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "role"),
        "the group carries the group role"
    );
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "tabindex"),
        "the triggers participate in the roving tab stop"
    );
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "onkeydown"),
        "the triggers carry their keyboard navigation handler"
    );
    if cfg!(feature = "theme-shadcn") {
        assert!(
            class_names(&rendered)
                .iter()
                .any(|class: &String| class == "c_euv_toggle_group")
        );
    }
}

#[test]
fn a_selected_entry_is_pressed() {
    let rendered: VirtualNode = render(
        vec![item("left", "Left"), item("right", "Right")],
        vec![String::from("right")],
    );
    assert_eq!(
        class_names(&rendered)
            .iter()
            .filter(|name: &&String| name.as_str() == "c_euv_toggle_pressed")
            .count(),
        1,
        "only the selected entry is pressed"
    );
}

#[test]
fn an_empty_group_renders_no_entry() {
    let rendered: VirtualNode = render(Vec::new(), Vec::new());
    assert_eq!(
        button_count(&rendered),
        0,
        "a group without entries has no button"
    );
}

#[test]
fn a_disabled_entry_is_marked_disabled() {
    let rendered: VirtualNode = render(
        vec![EuvToggleGroupItem {
            value: "left",
            label: "Left",
            disabled: true,
        }],
        Vec::new(),
    );
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "disabled"),
        "a disabled entry says so"
    );
}
