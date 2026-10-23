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

fn number_input(
    value: Signal<i64>,
    min: Option<i64>,
    max: Option<i64>,
    step: i64,
    disabled: bool,
) -> VirtualNode {
    euv_number_input(props(EuvNumberInputProps {
        value,
        min,
        max,
        step,
        label: "Quantity",
        disabled,
        on_change: None,
    }))
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
fn a_stepper_moves_the_value_by_one_step() {
    let observed: Option<i64> = stepped_value(5, 1, None, None);
    assert_eq!(observed, Some(6), "one step up from five is six");
}

#[test]
fn a_negative_step_moves_the_value_down() {
    let observed: Option<i64> = stepped_value(5, -1, None, None);
    assert_eq!(observed, Some(4), "one step down from five is four");
}

#[test]
fn a_stepper_refuses_to_step_past_the_upper_bound() {
    let observed: Option<i64> = stepped_value(10, 1, None, Some(10));
    assert_eq!(
        observed, None,
        "the value already sits on the upper bound, so the stepper stays inert"
    );
}

#[test]
fn a_stepper_refuses_to_step_past_the_lower_bound() {
    let observed: Option<i64> = stepped_value(0, -1, Some(0), None);
    assert_eq!(
        observed, None,
        "the value already sits on the lower bound, so the stepper stays inert"
    );
}

#[test]
fn a_stepper_may_step_onto_a_bound() {
    let observed: Option<i64> = stepped_value(9, 1, Some(0), Some(10));
    assert_eq!(
        observed,
        Some(10),
        "a step that lands exactly on the bound is still a legal step"
    );
}

#[test]
fn a_step_of_zero_moves_the_value_by_one() {
    let observed: Option<i64> = stepped_value(5, 0, None, None);
    assert_eq!(
        observed,
        Some(6),
        "a zero step would leave the value where it is, so it counts as one"
    );
}

#[test]
fn a_stepper_clamps_a_value_that_sits_outside_the_bounds() {
    let observed: Option<i64> = stepped_value(12, -1, Some(0), Some(10));
    assert_eq!(
        observed,
        Some(10),
        "a value left above the bound is pulled back onto it"
    );
}

#[test]
fn a_stepper_that_cannot_move_the_value_reports_itself_as_disabled() {
    let blocked: bool = stepper_disabled(10, 1, None, Some(10));
    let free: bool = stepper_disabled(9, 1, None, Some(10));
    assert!(blocked, "a stepper on the bound must not look clickable");
    assert!(
        !free,
        "a stepper that can still move the value stays clickable"
    );
}

#[test]
fn a_number_input_renders_a_native_number_field_with_two_steppers() {
    let rendered: VirtualNode = number_input(Signal::create(4), None, None, 1, false);
    assert_eq!(
        count_tag(&rendered, "input"),
        1,
        "the field is a real input the browser can type into"
    );
    assert_eq!(
        count_tag(&rendered, "button"),
        2,
        "the field carries one stepper per direction"
    );
    assert!(
        has_classes(&rendered, &["c_euv_number_input"]),
        "the wrapper carries the number input class"
    );
    assert!(
        has_classes(&rendered, &["c_euv_number_input_field"]),
        "the field carries its own class"
    );
    assert!(
        has_classes(&rendered, &["c_euv_number_input_stepper"]),
        "the steppers carry the stepper class"
    );
}

#[test]
fn a_number_input_without_a_label_renders_no_label() {
    let bare: VirtualNode = euv_number_input(props(EuvNumberInputProps::default()));
    assert_eq!(
        count_tag(&bare, "label"),
        0,
        "an empty label renders the field without one"
    );
}
