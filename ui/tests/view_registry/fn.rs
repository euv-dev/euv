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

fn root_attribute_text(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(AttributeEntry::text_value),
        _ => None,
    }
}

fn badge_node(variant: EuvBadgeVariant) -> VirtualNode<EuvBadgeProps> {
    element_node(EuvBadgeProps {
        text: "New".into(),
        outline: false,
        variant,
        on_click: None,
        class: Css::default(),
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
        aria_label: "",
        activation_mode: EuvTabsActivationMode::Automatic,
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
            disabled: false,
            active: false,
        }],
        on_select: None,
    })
}

fn select_node() -> VirtualNode<EuvSelectProps> {
    element_node(EuvSelectProps {
        value: Signal::create(String::from("one")),
        multiple: false,
        values: Signal::create(Vec::new()),
        on_change_values: None,
        size: EuvSelectSize::Default,
        options: vec![EuvSelectOption {
            value: "one",
            label: "One",
            group: "",
            separator_before: false,
            disabled: false,
        }],
        placeholder: "Pick",
        label: "",
        aria_label: String::new(),
        aria_describedby: "",
        id: "",
        name: "",
        form: "",
        auto_complete: "",
        required: false,
        invalid: false,
        disabled: false,
        read_only: false,
        open: None,
        default_open: false,
        align_item_with_trigger: false,
        on_change: None,
        on_open_change: None,
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
    let cases: [(EuvBadgeVariant, &str); 6] = [
        (EuvBadgeVariant::Default, "c_euv_badge_default"),
        (EuvBadgeVariant::Secondary, "c_euv_badge_secondary"),
        (EuvBadgeVariant::Destructive, "c_euv_badge_destructive"),
        (EuvBadgeVariant::Outline, "c_euv_badge_outline"),
        (EuvBadgeVariant::Ghost, "c_euv_badge_ghost"),
        (EuvBadgeVariant::Link, "c_euv_badge_link"),
    ];
    for (variant, class_name) in cases {
        let rendered: VirtualNode = euv_badge(badge_node(variant));
        assert!(
            has_class(&rendered, class_name),
            "the {variant:?} badge carries {class_name}, got {:?}",
            class_names(&rendered)
        );
        let expected_data_variant: &str = match variant {
            EuvBadgeVariant::Default => BADGE_VARIANT_DEFAULT,
            EuvBadgeVariant::Secondary => BADGE_VARIANT_SECONDARY,
            EuvBadgeVariant::Destructive => BADGE_VARIANT_DESTRUCTIVE,
            EuvBadgeVariant::Outline => BADGE_VARIANT_OUTLINE,
            EuvBadgeVariant::Ghost => BADGE_VARIANT_GHOST,
            EuvBadgeVariant::Link => BADGE_VARIANT_LINK,
        };
        assert_eq!(
            root_attribute_text(&rendered, "data-variant").as_deref(),
            Some(expected_data_variant)
        );
        if cfg!(feature = "theme-shadcn") {
            assert!(has_class(&rendered, "c_euv_badge"));
            if variant == EuvBadgeVariant::Destructive {
                assert!(has_class(&rendered, "c_euv_badge_destructive"));
            }
        }
    }
}

#[test]
fn a_badge_composes_an_application_class_with_its_design_system_class() {
    let rendered: VirtualNode = euv_badge(element_node(EuvBadgeProps {
        class: Css {
            name: String::from("commerce-badge"),
            ..Css::default()
        },
        ..Default::default()
    }));
    assert!(has_class(&rendered, "commerce-badge"));
    assert!(has_class(&rendered, "c_euv_badge"));
}

#[test]
fn a_badge_keeps_its_historical_outline_flag() {
    let node: VirtualNode<EuvBadgeProps> = element_node(EuvBadgeProps {
        text: "New".into(),
        outline: true,
        variant: EuvBadgeVariant::Default,
        on_click: None,
        class: Css::default(),
    });
    let rendered: VirtualNode = euv_badge(node);
    let root_tag: String = match &rendered {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        other => format!("{other:?}"),
    };
    assert_eq!(root_tag, "Element(\"span\")");
    assert!(
        has_class(&rendered, "c_euv_badge_outline"),
        "the outline flag still selects the outline variant, got {:?}",
        class_names(&rendered)
    );
}

#[test]
fn a_clickable_badge_is_a_native_button() {
    let node: VirtualNode<EuvBadgeProps> = element_node(EuvBadgeProps {
        text: "New".into(),
        outline: false,
        variant: EuvBadgeVariant::Default,
        on_click: Some(Rc::new(|_: Event| {})),
        class: Css::default(),
    });
    let rendered: VirtualNode = euv_badge(node);
    let root_tag: String = match &rendered {
        VirtualNode::Element { tag, .. } => format!("{tag:?}"),
        other => format!("{other:?}"),
    };
    assert_eq!(root_tag, "Element(\"button\")");
    assert!(has_attribute(&rendered, "type"));
    assert!(has_class(&rendered, "c_euv_badge_clickable"));
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
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&destructive, "c_euv_alert_destructive"));
        assert!(has_class(&default, "c_euv_alert_default"));
    }
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
fn a_switch_carries_the_reference_switch_slot_on_its_control() {
    let rendered: VirtualNode = euv_switch(element_node(EuvSwitchProps {
        id: "switch",
        name: "mode",
        checked: Signal::create(false),
        label: "Mode",
        disabled: Signal::create(false),
    }));
    let VirtualNode::Element { children, .. } = &rendered else {
        panic!("the switch row is the root");
    };
    let switch_control: &VirtualNode = &children[0];
    assert_eq!(
        root_attribute_text(switch_control, "data-slot").as_deref(),
        Some("switch")
    );
    assert_eq!(
        root_attribute_text(switch_control, "data-size").as_deref(),
        Some("default")
    );
    assert!(has_attribute(switch_control, "aria-checked"));
    assert_eq!(
        root_attribute_text(switch_control, "data-unchecked").as_deref(),
        Some("")
    );
    assert_eq!(root_attribute_text(switch_control, "data-checked"), None);
    let checked: VirtualNode = euv_switch(element_node(EuvSwitchProps {
        id: "switch-checked",
        name: "mode-checked",
        checked: Signal::create(true),
        label: "Mode checked",
        disabled: Signal::create(false),
    }));
    let VirtualNode::Element { children, .. } = &checked else {
        panic!("the checked switch row is the root");
    };
    assert_eq!(
        root_attribute_text(&children[0], "data-checked").as_deref(),
        Some("")
    );
    assert_eq!(root_attribute_text(&children[0], "data-unchecked"), None);
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
    assert!(
        has_attribute(&rendered, "aria-label"),
        "the bar is named by its visible label, got {:?}",
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
    assert!(has_attribute(&rendered, "aria-haspopup"));
    assert!(has_attribute(&rendered, "onkeydown"));
    if cfg!(feature = "theme-shadcn") {
        assert!(has_class(&rendered, "c_euv_select_trigger"));
        assert!(has_class(&rendered, "c_euv_select_panel"));
        assert!(has_class(&rendered, "c_euv_select_option"));
        assert!(has_class(&rendered, "c_euv_select_option_active"));
    }
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

#[test]
fn commerce_heading_levels_reach_card_title_and_card() {
    let cases: [(EuvHeadingLevel, &str); 6] = [
        (EuvHeadingLevel::H1, "h1"),
        (EuvHeadingLevel::H2, "h2"),
        (EuvHeadingLevel::H3, "h3"),
        (EuvHeadingLevel::H4, "h4"),
        (EuvHeadingLevel::H5, "h5"),
        (EuvHeadingLevel::H6, "h6"),
    ];
    for (level, expected) in cases {
        let rendered: [VirtualNode; 2] = [
            euv_card_title(element_node(EuvCardTitleProps {
                text: format!("Title {expected}").into(),
                level: Some(level),
            })),
            euv_card(element_node(EuvCardProps {
                title: format!("Card {expected}").into(),
                level: Some(level),
            })),
        ];
        for node in rendered {
            let headings: Vec<String> = elements(&node)
                .iter()
                .filter_map(|element: &VirtualNode| match element {
                    VirtualNode::Element {
                        tag: Tag::Element(name),
                        ..
                    } if ["h1", "h2", "h3", "h4", "h5", "h6"].contains(&name.as_ref()) => {
                        Some(name.to_string())
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(headings, vec![expected.to_string()]);
        }
    }
}

#[test]
fn commerce_heading_defaults_remain_h3_for_cards() {
    let title: VirtualNode = euv_card_title(element_node(EuvCardTitleProps::default()));
    assert!(
        matches!(title, VirtualNode::Element { tag: Tag::Element(ref name), .. } if name == "h3")
    );
    let card: VirtualNode = euv_card(element_node(EuvCardProps {
        title: String::from("Account").into(),
        ..EuvCardProps::default()
    }));
    assert!(format!("{card:?}").contains("Element(\"h3\")"));
    let empty: VirtualNode = euv_card(element_node(EuvCardProps::default()));
    assert!(!format!("{empty:?}").contains("Element(\"h3\")"));
}

#[test]
fn commerce_card_title_children_override_owned_fallback_without_losing_level() {
    let mut node: VirtualNode<EuvCardTitleProps> = element_node(EuvCardTitleProps {
        text: String::from("unused fallback").into(),
        level: Some(EuvHeadingLevel::H2),
    });
    if let VirtualNode::Element { children, .. } = &mut node {
        children.push(VirtualNode::from(String::from("Visible child")));
    }
    let rendered: VirtualNode = euv_card_title(node);
    assert!(
        matches!(&rendered, VirtualNode::Element { tag: Tag::Element(name), .. } if name == "h2")
    );
    assert!(format!("{rendered:?}").contains("Visible child"));
    assert!(!format!("{rendered:?}").contains("unused fallback"));
    assert_eq!(
        root_attribute_text(&rendered, "data-slot").as_deref(),
        Some("card-title")
    );
    assert!(has_class(&rendered, "c_card_title"));
}

#[test]
fn commerce_button_submit_and_accessible_name_are_explicit() {
    for submit in [false, true] {
        let rendered: VirtualNode = euv_button(element_node(EuvButtonProps {
            label: "Pay",
            submit,
            aria_label: Some(String::from("Pay order 314")),
            ..Default::default()
        }));
        assert_eq!(
            root_attribute_text(&rendered, "type").as_deref(),
            Some(if submit { "submit" } else { "button" })
        );
        assert_eq!(
            root_attribute_text(&rendered, "aria-label").as_deref(),
            Some("Pay order 314")
        );
    }
}

#[test]
fn commerce_button_busy_state_is_published_on_each_render() {
    let busy: Signal<bool> = Signal::create(false);
    for expected in [false, true, false] {
        busy.set(expected);
        let rendered: VirtualNode = euv_button(element_node(EuvButtonProps {
            label: "Save",
            busy,
            ..Default::default()
        }));
        assert_eq!(
            root_attribute_text(&rendered, "aria-busy"),
            Some(expected.to_string())
        );
    }
}

#[test]
fn commerce_svg_icon_preserves_namespace_geometry_and_decorative_semantics() {
    let rendered: VirtualNode = euv_icon(element_node(EuvIconProps {
        name: "not the drawing",
        size: EuvIconSize::Lg,
        svg: Some("<path d=\"M2 3h4\"/>"),
        ..Default::default()
    }));
    let drawing: String = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\" focusable=\"false\"><path d=\"M2 3h4\"/></svg>",
    );
    let expected: VirtualNode = html! {
        span { class: c_euv_icon() style: "font-size: 24px; width: 24px; height: 24px;" inner_html: drawing.clone() }
    };
    assert_eq!(svg_glyph_markup("<path d=\"M2 3h4\"/>", 24), drawing);
    assert_eq!(class_names(&rendered), class_names(&expected));
    assert!(has_attribute(&rendered, "inner_html"));
    assert_eq!(
        root_attribute_text(&rendered, "style"),
        root_attribute_text(&expected, "style")
    );
}

#[test]
fn commerce_owned_display_text_survives_source_drop_and_distinct_renders() {
    for order in [314, 271] {
        let source: String = format!("Order {order}");
        let expected: String = source.clone();
        let rendered: [VirtualNode; 2] = [
            euv_badge(element_node(EuvBadgeProps {
                text: source.clone().into(),
                ..Default::default()
            })),
            VirtualNode::from(CowText::from(source.clone())),
        ];
        drop(source);
        for node in rendered {
            assert!(
                format!("{node:?}").contains(&expected),
                "owned label was lost: {node:?}"
            );
        }
    }
}

#[test]
fn commerce_textarea_accessible_name_and_keyboard_handler_reach_the_control() {
    let rendered: VirtualNode = euv_textarea(element_node(EuvTextareaProps {
        aria_label: "Order notes",
        onkeydown: Some(Rc::new(|_: Event| {})),
        ..Default::default()
    }));
    assert_eq!(
        root_attribute_text(&rendered, "aria-label").as_deref(),
        Some("Order notes")
    );
    assert!(has_attribute(&rendered, "onkeydown"));
    assert!(has_class(&rendered, "c_euv_textarea"));
}

#[test]
fn commerce_input_accessibility_readonly_and_disabled_reach_native_input() {
    for readonly in [false, true] {
        let rendered: VirtualNode = euv_input(element_node(EuvInputProps {
            aria_label: "Order reference",
            readonly,
            disabled: Signal::create(true),
            ..Default::default()
        }));
        let input: VirtualNode = elements(&rendered).into_iter().find(|node: &VirtualNode| {
            matches!(node, VirtualNode::Element { tag: Tag::Element(name), .. } if name == "input")
        }).expect("native input");
        assert_eq!(
            root_attribute_text(&input, "aria-label").as_deref(),
            Some("Order reference")
        );
        if let VirtualNode::Element { attributes, .. } = &input {
            let readonly_attribute: &AttributeEntry = attributes
                .iter()
                .find(|entry: &&AttributeEntry| entry.get_name() == "readonly")
                .expect("readonly attribute");
            assert_eq!(
                readonly_attribute,
                &AttributeEntry::new(Cow::Borrowed("readonly"), AttributeValue::from(readonly))
            );
            let disabled_attribute: &AttributeEntry = attributes
                .iter()
                .find(|entry: &&AttributeEntry| entry.get_name() == "disabled")
                .expect("disabled attribute");
            assert_eq!(
                disabled_attribute,
                &AttributeEntry::new(
                    Cow::Borrowed("disabled"),
                    AttributeValue::from(Signal::create(true))
                )
            );
        }
        assert!(has_class(&input, "c_euv_input"));
    }
}

#[test]
fn commerce_tabs_publish_the_named_tab_list() {
    let rendered: VirtualNode = euv_tabs(element_node(EuvTabsProps {
        aria_label: "Documentation panels",
        items: vec![EuvTabItem {
            key: "details",
            label: "Details",
        }],
        active: Signal::create(String::from("details")),
        activation_mode: EuvTabsActivationMode::Automatic,
    }));
    let tablist: VirtualNode = elements(&rendered)
        .into_iter()
        .find(|node: &VirtualNode| root_attribute_text(node, "role").as_deref() == Some("tablist"))
        .expect("semantic tab list");
    assert_eq!(
        root_attribute_text(&tablist, "aria-label").as_deref(),
        Some("Documentation panels")
    );
}

#[test]
fn commerce_button_test_id_does_not_replace_its_accessible_name() {
    let rendered: VirtualNode = euv_button(element_node(EuvButtonProps {
        label: "Sign in",
        data_testid: "login-submit",
        ..Default::default()
    }));
    assert_eq!(
        root_attribute_text(&rendered, "data-testid").as_deref(),
        Some("login-submit")
    );
    assert_eq!(
        root_attribute_text(&rendered, "aria-label").as_deref(),
        Some("Sign in")
    );
}

#[test]
fn commerce_upload_keeps_native_file_control_configuration() {
    let rendered: VirtualNode = euv_upload(element_node(EuvUploadProps {
        id: "invoice-file",
        label: "Attach invoice",
        accept: "application/pdf",
        disabled: Signal::create(true),
        onchange: Some(Rc::new(|_: Event| {})),
        ..Default::default()
    }));
    let input: VirtualNode = elements(&rendered)
        .into_iter()
        .find(|node: &VirtualNode| root_attribute_text(node, "type").as_deref() == Some("file"))
        .expect("native file input");
    assert_eq!(
        root_attribute_text(&input, "id").as_deref(),
        Some("invoice-file")
    );
    assert_eq!(
        root_attribute_text(&input, "accept").as_deref(),
        Some("application/pdf")
    );
    assert!(has_attribute(&input, "disabled"));
    assert!(has_attribute(&input, "onchange"));
}

#[test]
fn commerce_list_accepts_owned_row_text_and_keeps_distinct_owned_keys() {
    let items: Vec<EuvListItem> = (1..=2)
        .map(|index: usize| EuvListItem {
            title: format!("Plan {index}").into(),
            desc: format!("Description {index}").into(),
            meta: format!("${index} per month").into(),
        })
        .collect();
    let cloned: Vec<EuvListItem> = items.clone();
    assert_eq!(cloned[0].desc.as_str(), "Description 1");
    assert_eq!(cloned[1].meta.as_str(), "$2 per month");
    let rendered: VirtualNode = euv_list(element_node(EuvListProps {
        items,
        empty_text: "",
    }));
    let keys: Vec<String> = elements(&rendered)
        .iter()
        .filter_map(|node: &VirtualNode| node.key().map(str::to_string))
        .collect();
    assert_eq!(keys, vec![String::from("Plan 1"), String::from("Plan 2")]);
}
