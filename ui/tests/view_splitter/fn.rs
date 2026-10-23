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

fn classes_of(node: &VirtualNode, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "class" {
                    into.extend(attribute.class_names());
                }
            }
            for child in children {
                classes_of(child, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                classes_of(child, into);
            }
        }
        _ => {}
    }
}

fn attributes_of(node: &VirtualNode, name: &str, into: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == name {
                    into.extend(attribute.text_value());
                }
            }
            for child in children {
                attributes_of(child, name, into);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                attributes_of(child, name, into);
            }
        }
        _ => {}
    }
}

fn splitter_with(size: f64) -> VirtualNode {
    euv_splitter(props(EuvSplitterProps {
        size: Signal::create(size),
        min: 10.0,
        max: 90.0,
    }))
}

#[test]
fn a_splitter_renders_two_panes_and_a_divider() {
    let rendered: VirtualNode = splitter_with(40.0);
    let mut classes: Vec<String> = Vec::new();
    classes_of(&rendered, &mut classes);
    let panes: usize = classes
        .iter()
        .filter(|class: &&String| *class == "c_euv_splitter_pane")
        .count();
    assert_eq!(panes, 2, "a split view has two panes, got {classes:?}");
    assert!(
        classes
            .iter()
            .any(|class: &String| class == "c_euv_splitter_bar"),
        "the divider must be rendered, got {classes:?}"
    );
}

#[test]
fn a_splitter_sizes_the_first_pane_from_the_signal() {
    let rendered: VirtualNode = splitter_with(40.0);
    let mut styles: Vec<String> = Vec::new();
    attributes_of(&rendered, "style", &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("flex: 0 0 40%")),
        "the first pane must be sized from the signal, got {styles:?}"
    );
}

#[test]
fn a_splitter_clamps_the_first_pane_to_its_bounds() {
    let rendered: VirtualNode = splitter_with(500.0);
    let mut styles: Vec<String> = Vec::new();
    attributes_of(&rendered, "style", &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("flex: 0 0 90%")),
        "a size past the upper bound must be clamped, got {styles:?}"
    );
}

#[test]
fn a_splitter_clamps_a_too_small_pane_up() {
    let rendered: VirtualNode = splitter_with(0.0);
    let mut styles: Vec<String> = Vec::new();
    attributes_of(&rendered, "style", &mut styles);
    assert!(
        styles
            .iter()
            .any(|style: &String| style.contains("flex: 0 0 10%")),
        "a size below the lower bound must be clamped, got {styles:?}"
    );
}
