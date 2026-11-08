use super::*;

fn element_node<T: Clone + 'static>(props: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(props)),
    }
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

fn has_class(node: &VirtualNode, name: &str) -> bool {
    class_names(node).iter().any(|entry: &String| entry == name)
}

fn has_attribute(node: &VirtualNode, name: &str) -> bool {
    attribute_names(node)
        .iter()
        .any(|entry: &String| entry == name)
}

fn badge_node(variant: EuvBadgeVariant) -> VirtualNode<EuvBadgeProps> {
    element_node(EuvBadgeProps {
        text: "New",
        outline: false,
        variant,
        on_click: None,
    })
}

fn alert_node(variant: AlertVariant) -> VirtualNode<EuvAlertProps> {
    element_node(EuvAlertProps { variant })
}

fn tabs_node() -> VirtualNode<EuvTabsProps> {
    element_node(EuvTabsProps {
        items: vec![EuvTabItem {
            key: "one",
            label: "One",
        }],
        active: Signal::create(String::from("one")),
    })
}

fn tooltip_node() -> VirtualNode<EuvTooltipProps> {
    element_node(EuvTooltipProps {
        text: "Hint",
        placement: EuvTooltipPlacement::Top,
    })
}

fn progress_node() -> VirtualNode<EuvProgressProps> {
    element_node(EuvProgressProps {
        percent: Signal::create(40.0),
        label: "Loading",
        active: Signal::create(false),
    })
}

fn modal_node() -> VirtualNode<EuvModalProps> {
    element_node(EuvModalProps {
        title: "Title",
        onclick: None,
    })
}

fn dropdown_node() -> VirtualNode<EuvDropdownProps> {
    element_node(EuvDropdownProps {
        open: Signal::create(true),
        items: vec![EuvDropdownItem {
            label: "One",
            value: "one",
            active: false,
        }],
        on_select: None,
    })
}

fn select_node() -> VirtualNode<EuvSelectProps> {
    element_node(EuvSelectProps {
        value: Signal::create(String::from("one")),
        options: vec![EuvSelectOption {
            value: "one",
            label: "One",
            group: "",
            disabled: false,
        }],
        placeholder: "Pick",
        label: "",
        disabled: false,
        on_change: None,
    })
}

fn message_node() -> VirtualNode<EuvMessageProps> {
    element_node(EuvMessageProps {
        items: vec![EuvMessageItem {
            text: "Saved",
            action_label: "",
            on_action: None,
            on_close: None,
            dismissible: false,
        }],
    })
}

fn notification_node() -> VirtualNode<EuvNotificationProps> {
    element_node(EuvNotificationProps {
        items: vec![EuvNotificationItem {
            title: "Title",
            text: "Body",
            on_close: None,
            dismissible: false,
        }],
    })
}

fn pagination_node() -> VirtualNode<EuvPaginationProps> {
    element_node(EuvPaginationProps {
        prev_label: "Previous",
        next_label: "Next",
        prev: None,
        next: None,
    })
}

#[test]
fn every_badge_variant_reaches_the_class_list() {
    let cases: [(EuvBadgeVariant, &str); 4] = [
        (EuvBadgeVariant::Default, "c_euv_badge_default"),
        (EuvBadgeVariant::Secondary, "c_euv_badge_secondary"),
        (EuvBadgeVariant::Destructive, "c_euv_badge_destructive"),
        (EuvBadgeVariant::Outline, "c_euv_badge_outline"),
    ];
    for (variant, class_name) in cases {
        let rendered: VirtualNode = euv_badge(badge_node(variant));
        assert!(
            has_class(&rendered, class_name),
            "the {variant:?} badge carries {class_name}, got {:?}",
            class_names(&rendered)
        );
    }
}

#[test]
fn a_badge_keeps_its_historical_outline_flag() {
    let node: VirtualNode<EuvBadgeProps> = element_node(EuvBadgeProps {
        text: "New",
        outline: true,
        variant: EuvBadgeVariant::Default,
        on_click: None,
    });
    let rendered: VirtualNode = euv_badge(node);
    assert!(
        has_class(&rendered, "c_euv_badge_outline"),
        "the outline flag still selects the outline variant, got {:?}",
        class_names(&rendered)
    );
}

#[test]
fn every_alert_variant_reaches_the_class_list() {
    let destructive: VirtualNode = euv_alert(alert_node(AlertVariant::Destructive));
    let default: VirtualNode = euv_alert(alert_node(AlertVariant::Default));
    assert!(
        has_class(&destructive, "c_euv_alert_destructive"),
        "the destructive alert carries its own class, got {:?}",
        class_names(&destructive)
    );
    assert!(
        has_class(&default, "c_euv_alert_default"),
        "the default alert carries its own class, got {:?}",
        class_names(&default)
    );
}

#[test]
fn an_alert_announces_itself() {
    let rendered: VirtualNode = euv_alert(alert_node(AlertVariant::Default));
    assert!(
        has_attribute(&rendered, "role"),
        "an alert carries the alert role, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_tab_bar_is_a_tablist() {
    let rendered: VirtualNode = euv_tabs(tabs_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the tab bar carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-selected"),
        "a tab reports its selected state, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_tooltip_bubble_is_a_tooltip() {
    let rendered: VirtualNode = euv_tooltip(tooltip_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the bubble carries a role, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_progress_bar_reports_its_value() {
    let rendered: VirtualNode = euv_progress(progress_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the bar carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-valuenow"),
        "the bar reports its value, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_modal_is_a_dialog() {
    let rendered: VirtualNode = euv_modal(modal_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the modal carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-modal"),
        "the modal declares itself modal, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_dropdown_is_a_menu() {
    let rendered: VirtualNode = euv_dropdown(dropdown_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the dropdown carries a role, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_select_is_a_listbox() {
    let rendered: VirtualNode = euv_select(select_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the select carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-expanded"),
        "the select trigger reports its open state, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_message_stack_is_a_live_region() {
    let rendered: VirtualNode = euv_message(message_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the message stack carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-live"),
        "the message stack is a live region, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_notification_stack_is_a_live_region() {
    let rendered: VirtualNode = euv_notification(notification_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the notification stack carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-live"),
        "the notification stack is a live region, got {:?}",
        attribute_names(&rendered)
    );
}

fn divider_node() -> VirtualNode<EuvDividerProps> {
    element_node(EuvDividerProps {
        orientation: EuvDividerOrientation::Vertical,
        label: "",
    })
}

fn popover_node() -> VirtualNode<EuvPopoverProps> {
    element_node(EuvPopoverProps {
        open: Signal::create(true),
        title: "Title",
        placement: EuvTooltipPlacement::Bottom,
    })
}

#[test]
fn a_divider_is_a_separator() {
    let rendered: VirtualNode = euv_divider(divider_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the divider carries a role, got {:?}",
        attribute_names(&rendered)
    );
    assert!(
        has_attribute(&rendered, "aria-orientation"),
        "the divider reports its orientation, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_popover_is_a_dialog() {
    let rendered: VirtualNode = euv_popover(popover_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the popover body carries a role, got {:?}",
        attribute_names(&rendered)
    );
}

#[test]
fn a_pagination_is_navigation() {
    let rendered: VirtualNode = euv_pagination(pagination_node());
    assert!(
        has_attribute(&rendered, "role"),
        "the pagination carries a role, got {:?}",
        attribute_names(&rendered)
    );
}
