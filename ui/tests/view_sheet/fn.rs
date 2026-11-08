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
fn a_sheet_panel_is_a_modal_dialog() {
    let node: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: Signal::create(true),
        side: EuvSheetSide::Right,
    }));
    assert!(has_class(&node, "c_euv_sheet_content"));
    assert!(has_class(&node, "c_euv_sheet_content_right"));
    assert!(has_class(&node, "c_euv_sheet_content_open"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "aria-modal"));
}

#[test]
fn every_sheet_side_has_its_own_class() {
    let top: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: Signal::create(true),
        side: EuvSheetSide::Top,
    }));
    assert!(has_class(&top, "c_euv_sheet_content_top"));
    let bottom: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: Signal::create(true),
        side: EuvSheetSide::Bottom,
    }));
    assert!(has_class(&bottom, "c_euv_sheet_content_bottom"));
    let left: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: Signal::create(true),
        side: EuvSheetSide::Left,
    }));
    assert!(has_class(&left, "c_euv_sheet_content_left"));
}

#[test]
fn a_sheet_trigger_reports_whether_the_sheet_is_open() {
    let node: VirtualNode = euv_sheet_trigger(props_node(EuvSheetTriggerProps {
        open: Signal::create(false),
        label: "Open panel",
    }));
    assert!(has_class(&node, "c_euv_sheet_trigger"));
    assert!(has_attribute(&node, "aria-expanded"));
}

#[test]
fn a_sheet_carries_a_header_a_footer_a_title_and_a_description() {
    let header: VirtualNode = euv_sheet_header(props_node(EuvSheetHeaderProps { label: "Head" }));
    assert!(has_class(&header, "c_euv_sheet_header"));
    let footer: VirtualNode = euv_sheet_footer(props_node(EuvSheetFooterProps { label: "Foot" }));
    assert!(has_class(&footer, "c_euv_sheet_footer"));
    let title: VirtualNode = euv_sheet_title(props_node(EuvSheetTitleProps { text: "Panel" }));
    assert!(has_class(&title, "c_euv_sheet_title"));
    let description: VirtualNode =
        euv_sheet_description(props_node(EuvSheetDescriptionProps { text: "Body" }));
    assert!(has_class(&description, "c_euv_sheet_description"));
}
