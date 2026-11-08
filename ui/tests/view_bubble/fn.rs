use super::*;

fn props_node<T: Clone + 'static>(props: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    }
}

fn elements(node: &VirtualNode) -> Vec<VirtualNode> {
    let mut out: Vec<VirtualNode> = Vec::new();
    collect(node, &mut out);
    out
}

fn collect(node: &VirtualNode, out: &mut Vec<VirtualNode>) {
    match node {
        VirtualNode::Element { children, .. } => {
            out.push(node.clone());
            for child in children.iter() {
                collect(child, out);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children.iter() {
                collect(child, out);
            }
        }
        _ => {}
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

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    attribute_names(node)
        .iter()
        .any(|entry: &String| entry == name)
}

#[test]
fn a_bubble_carries_its_variant_and_alignment() {
    let node: VirtualNode = euv_bubble(props_node(EuvBubbleProps {
        variant: EuvBubbleVariant::Tinted,
        align: EuvBubbleAlign::End,
    }));
    assert!(has_class(&node, "c_euv_bubble"));
    assert!(has_class(&node, "c_euv_bubble_tinted"));
    assert!(has_class(&node, "c_euv_bubble_align_end"));
    assert!(has_attribute(&node, "data-variant"));
    assert!(has_attribute(&node, "data-align"));
}

#[test]
fn a_destructive_bubble_is_drawn_in_the_destructive_colour() {
    let node: VirtualNode = euv_bubble(props_node(EuvBubbleProps {
        variant: EuvBubbleVariant::Destructive,
        align: EuvBubbleAlign::Start,
    }));
    assert!(has_class(&node, "c_euv_bubble_destructive"));
    assert!(has_class(&node, "c_euv_bubble_align_start"));
}

#[test]
fn the_bubble_group_carries_its_slot() {
    let node: VirtualNode = euv_bubble_group(props_node(EuvBubbleGroupProps {}));
    assert!(has_class(&node, "c_euv_bubble_group"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_bubble_content_carries_its_slot() {
    let node: VirtualNode = euv_bubble_content(props_node(EuvBubbleContentProps {}));
    assert!(has_class(&node, "c_euv_bubble_content"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_bubble_reactions_carry_their_side() {
    let node: VirtualNode = euv_bubble_reactions(props_node(EuvBubbleReactionsProps {
        side: EuvBubbleSide::Top,
        align: EuvBubbleAlign::Start,
    }));
    assert!(has_class(&node, "c_euv_bubble_reactions_top"));
    assert!(has_class(&node, "c_euv_bubble_reactions_start"));
    assert!(has_attribute(&node, "data-side"));
}
