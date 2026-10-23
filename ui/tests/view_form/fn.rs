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

fn form_with(submit_label: &'static str, children: Vec<VirtualNode>) -> VirtualNode {
    let mut node: VirtualNode<EuvFormProps> = props(EuvFormProps {
        submit_label,
        on_submit: None,
    });
    if let VirtualNode::Element {
        children: own_children,
        ..
    } = &mut node
    {
        *own_children = children;
    }
    euv_form(node)
}

fn root_tag(node: &VirtualNode) -> String {
    match node {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        _ => "not an element".to_string(),
    }
}

fn attribute_names(node: &VirtualNode, into: &mut Vec<String>) {
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
                attribute_names(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attribute_names(child, into);
            }
        }
        _ => {}
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
fn a_form_renders_a_real_form_element() {
    let rendered: VirtualNode = form_with("Send", Vec::new());
    assert_eq!(
        root_tag(&rendered),
        "Element(\"form\")",
        "the component renders a form the browser understands"
    );
    assert!(
        has_classes(&rendered, &["c_euv_form"]),
        "the form carries the form class"
    );
}

#[test]
fn a_form_wraps_the_children_it_was_given() {
    let child: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("input")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let rendered: VirtualNode = form_with("Send", vec![child]);
    assert_eq!(
        count_tag(&rendered, "input"),
        1,
        "the caller's field ends up inside the form"
    );
}

#[test]
fn a_form_with_a_submit_label_renders_a_primary_submit_button() {
    let rendered: VirtualNode = form_with("Send", Vec::new());
    assert_eq!(
        count_tag(&rendered, "button"),
        1,
        "the submit label asks for exactly one button"
    );
    assert!(
        has_classes(&rendered, &["c_euv_button_primary_md"]),
        "the submit button is the primary button of the design system"
    );
    assert!(
        has_classes(&rendered, &["c_euv_form_actions"]),
        "the button sits in the actions row of the form"
    );
}

#[test]
fn a_form_without_a_submit_label_renders_no_actions_row() {
    let rendered: VirtualNode = form_with("", Vec::new());
    assert_eq!(
        count_tag(&rendered, "button"),
        0,
        "an empty submit label asks for no button at all"
    );
    assert!(
        !has_classes(&rendered, &["c_euv_form_actions"]),
        "an empty submit label asks for no actions row either"
    );
}

#[test]
fn a_form_cancels_the_native_submission() {
    let rendered: VirtualNode = form_with("Send", Vec::new());
    let mut names: Vec<String> = Vec::new();
    attribute_names(&rendered, &mut names);
    assert!(
        names.iter().any(|name: &String| name == "onsubmit"),
        "the form listens for the submit event so it can cancel the native one, got {names:?}"
    );
}
