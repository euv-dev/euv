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
fn the_questionnaire_carries_its_slot() {
    let node: VirtualNode = euv_questionnaire(props_node(EuvQuestionnaireProps {}));
    assert!(has_class(&node, "c_euv_questionnaire"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_questionnaire_progress_carries_its_slot() {
    let node: VirtualNode =
        euv_questionnaire_progress(props_node(EuvQuestionnaireProgressProps {}));
    assert!(has_class(&node, "c_euv_questionnaire_progress"));
    assert!(has_attribute(&node, "data-slot"));
}

#[test]
fn the_questionnaire_choices_carry_their_kind_of_answer() {
    let node: VirtualNode = euv_questionnaire_choices(props_node(EuvQuestionnaireChoicesProps {
        choice_type: EuvQuestionnaireChoiceType::Checkbox,
    }));
    assert!(has_class(&node, "c_euv_questionnaire_choices"));
    assert!(has_attribute(&node, "data-type"));
}

#[test]
fn a_questionnaire_choice_reports_whether_it_is_ticked() {
    let node: VirtualNode = euv_questionnaire_choice(props_node(EuvQuestionnaireChoiceProps {
        choice_type: EuvQuestionnaireChoiceType::Radio,
        checked: true,
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_questionnaire_choice"));
    assert!(has_attribute(&node, "data-checked"));
    assert!(has_attribute(&node, "data-disabled"));
}

#[test]
fn the_questionnaire_error_is_announced() {
    let node: VirtualNode = euv_questionnaire_error(props_node(EuvQuestionnaireErrorProps {}));
    assert!(has_class(&node, "c_euv_questionnaire_error"));
    assert!(has_attribute(&node, "role"));
}

#[test]
fn the_questionnaire_actions_carry_their_slot() {
    let node: VirtualNode = euv_questionnaire_actions(props_node(EuvQuestionnaireActionsProps {}));
    assert!(has_class(&node, "c_euv_questionnaire_actions"));
    assert!(has_attribute(&node, "data-slot"));
}
