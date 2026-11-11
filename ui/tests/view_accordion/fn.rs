use super::*;

fn items() -> Vec<EuvAccordionItem> {
    vec![
        EuvAccordionItem {
            value: "one",
            trigger: "First section",
            content: "The body of the first section",
            disabled: false,
        },
        EuvAccordionItem {
            value: "two",
            trigger: "Second section",
            content: "The body of the second section",
            disabled: true,
        },
    ]
}

fn accordion_node(
    open: Vec<String>,
    accordion_type: EuvAccordionType,
    collapsible: bool,
) -> VirtualNode<EuvAccordionProps> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvAccordionProps {
            items: items(),
            open_values: Signal::create(open),
            accordion_type,
            collapsible,
            disabled: false,
            on_change: None,
        })),
    }
}

fn render(open: Vec<String>, accordion_type: EuvAccordionType, collapsible: bool) -> VirtualNode {
    euv_accordion(accordion_node(open, accordion_type, collapsible))
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

fn tag_name(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        _ => panic!("unexpected node variant: {node:?}"),
    }
}

fn attribute_names(node: &VirtualNode) -> Vec<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .map(|entry: &AttributeEntry| entry.get_name().to_string())
            .collect(),
        _ => Vec::new(),
    }
}

fn attribute_value(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
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

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn buttons(node: &VirtualNode) -> Vec<VirtualNode> {
    elements(node)
        .into_iter()
        .filter(|element: &VirtualNode| tag_name(element) == "Element(\"button\")")
        .collect()
}

#[test]
fn an_accordion_is_a_stack_of_sections() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    assert!(
        attribute_names(&rendered)
            .iter()
            .any(|name: &String| name == "data-slot"),
        "the stack names its slot"
    );
    assert!(
        has_class(&rendered, "c_euv_accordion"),
        "the stack carries the accordion class"
    );
}

#[test]
fn every_section_gets_a_header_button() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    assert_eq!(
        buttons(&rendered).len(),
        2,
        "every section is opened by a button"
    );
}

#[test]
fn a_header_button_never_submits_a_form() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    let first: Vec<VirtualNode> = buttons(&rendered);
    assert!(
        attribute_names(&first[0])
            .iter()
            .any(|name: &String| name == "type"),
        "the header button carries its type so it never submits the form around it"
    );
}

#[test]
fn a_header_button_reports_its_state() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    let first: Vec<VirtualNode> = buttons(&rendered);
    assert!(
        attribute_names(&first[0])
            .iter()
            .any(|name: &String| name == "aria-expanded"),
        "the header button reports whether its section is open"
    );
    assert!(
        attribute_names(&first[0])
            .iter()
            .any(|name: &String| name == "tabindex"),
        "the enabled trigger participates in the roving tab stop"
    );
    assert!(
        attribute_names(&first[0])
            .iter()
            .any(|name: &String| name == "onkeydown"),
        "the trigger carries its keyboard navigation handler"
    );
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&first[0], "c_euv_accordion_trigger_shadcn"));
        assert!(has_class(&rendered, "c_euv_accordion_content_shadcn"));
    }
}

#[test]
fn each_trigger_controls_and_names_its_content_region() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Multiple, true);
    let all: Vec<VirtualNode> = elements(&rendered);
    let triggers: Vec<VirtualNode> = all
        .iter()
        .filter(|node: &&VirtualNode| {
            attribute_value(node, "data-slot").as_deref() == Some("accordion-trigger")
        })
        .cloned()
        .collect();
    let contents: Vec<VirtualNode> = all
        .iter()
        .filter(|node: &&VirtualNode| {
            attribute_value(node, "data-slot").as_deref() == Some("accordion-content")
        })
        .cloned()
        .collect();
    assert_eq!(triggers.len(), 2);
    assert_eq!(contents.len(), 2);
    for (trigger, content) in triggers.iter().zip(contents.iter()) {
        assert_eq!(
            attribute_value(trigger, "aria-controls"),
            attribute_value(content, "id")
        );
        assert_eq!(
            attribute_value(content, "aria-labelledby"),
            attribute_value(trigger, "id")
        );
        assert_eq!(attribute_value(content, "role").as_deref(), Some("region"));
    }
    let items: Vec<VirtualNode> = all
        .iter()
        .filter(|node: &&VirtualNode| {
            attribute_value(node, "data-slot").as_deref() == Some("accordion-item")
        })
        .cloned()
        .collect();
    assert!(
        items.iter().all(|node: &VirtualNode| {
            attribute_value(node, "role").as_deref() != Some("region")
        })
    );
}

#[test]
fn an_open_section_carries_the_open_class() {
    let rendered: VirtualNode = render(vec!["one".to_string()], EuvAccordionType::Single, true);
    assert!(
        has_class(&rendered, "c_euv_accordion_trigger_open"),
        "the header of an open section is marked as open"
    );
    assert!(
        has_class(&rendered, "c_euv_accordion_content_open"),
        "the body of an open section is shown"
    );
}

#[test]
fn a_closed_section_hides_its_body() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    assert!(
        has_class(&rendered, "c_euv_accordion_content_closed"),
        "the body of a closed section is hidden"
    );
    assert!(
        !has_class(&rendered, "c_euv_accordion_trigger_open"),
        "the header of a closed section is not marked as open"
    );
}

#[test]
fn a_disabled_section_cannot_be_opened() {
    let rendered: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    let all: Vec<VirtualNode> = buttons(&rendered);
    assert!(
        attribute_names(&all[1])
            .iter()
            .any(|name: &String| name == "disabled"),
        "a disabled section cannot be opened"
    );
}

#[test]
fn the_accordion_carries_its_mode() {
    let single: VirtualNode = render(Vec::new(), EuvAccordionType::Single, true);
    let multiple: VirtualNode = render(Vec::new(), EuvAccordionType::Multiple, true);
    assert!(
        attribute_names(&single)
            .iter()
            .any(|name: &String| name == "data-type"),
        "the stack names the mode it was given"
    );
    assert!(
        attribute_names(&multiple)
            .iter()
            .any(|name: &String| name == "data-type"),
        "the stack names the mode it was given"
    );
}
