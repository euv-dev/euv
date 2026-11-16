use super::*;

fn bind_submenu(context: &mut HookContext, parent_open: Signal<bool>, open: Signal<bool>) {
    context.reset_index();
    HookContext::with(context.clone(), || {
        use_context_menu_sub_binding(parent_open, open)
    });
}

fn submenu_attribute(open: Signal<bool>, name: &str, panel: bool) -> Option<String> {
    let node: VirtualNode = if panel {
        euv_context_menu_sub_content(props_node(EuvContextMenuSubContentProps { open }))
    } else {
        euv_context_menu_sub_trigger(props_node(EuvContextMenuSubTriggerProps {
            open,
            label: "More actions",
            ..EuvContextMenuSubTriggerProps::default()
        }))
    };
    elements(&node).iter().find_map(|element: &VirtualNode| {
        if let VirtualNode::Element { attributes, .. } = element {
            attributes
                .iter()
                .find(|entry: &&AttributeEntry| entry.get_name() == name)
                .and_then(AttributeEntry::text_value)
        } else {
            None
        }
    })
}

#[test]
fn closing_a_controlled_parent_resets_every_descendant_and_reopening_keeps_them_closed() {
    let root: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let grandchild: Signal<bool> = Signal::create(true);
    let great_grandchild: Signal<bool> = Signal::create(true);
    let mut first_context: HookContext = HookContext::default();
    let mut second_context: HookContext = HookContext::default();
    let mut third_context: HookContext = HookContext::default();
    bind_submenu(&mut first_context, root, child);
    bind_submenu(&mut second_context, child, grandchild);
    bind_submenu(&mut third_context, grandchild, great_grandchild);
    for _ in 0..3 {
        root.set(false);
        for open in [child, grandchild, great_grandchild] {
            assert!(!open.get());
            assert_eq!(
                submenu_attribute(open, "aria-expanded", false).as_deref(),
                Some("false")
            );
            assert_eq!(
                submenu_attribute(open, "data-state", true).as_deref(),
                Some("closed")
            );
            open.set(true);
            assert!(
                !open.get(),
                "hidden descendants cannot reopen under a closed ancestor"
            );
        }
        root.set(true);
        for open in [child, grandchild, great_grandchild] {
            assert!(!open.get());
            open.set(true);
            assert!(open.get());
        }
    }
}

#[test]
fn closing_nearest_submenu_resets_only_its_branch_and_preserves_radio_selection() {
    let root: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let grandchild: Signal<bool> = Signal::create(true);
    let sibling: Signal<bool> = Signal::create(true);
    let mut first: HookContext = HookContext::default();
    let mut second: HookContext = HookContext::default();
    let mut other: HookContext = HookContext::default();
    bind_submenu(&mut first, root, child);
    bind_submenu(&mut second, child, grandchild);
    bind_submenu(&mut other, root, sibling);
    let selection: EuvContextMenuRadioGroupState = use_context_menu_radio_group(None, "grid", None);
    let props: EuvContextMenuRadioItemProps = EuvContextMenuRadioItemProps {
        open: root,
        selection: selection.clone(),
        value: "list",
        close_on_click: true,
        ..EuvContextMenuRadioItemProps::default()
    };
    child.set(false);
    assert!(root.get());
    assert!(sibling.get());
    assert!(!grandchild.get());
    child.set(true);
    grandchild.set(true);
    assert!(activate_context_menu_radio(&props));
    assert!(!root.get());
    assert!(!child.get());
    assert!(!grandchild.get());
    assert!(!sibling.get());
    root.set(true);
    assert_eq!(selection.value.get(), "list");
    assert!(!child.get());
}

#[test]
fn submenu_bindings_rebind_without_stale_listeners_and_cleanup_on_unmount() {
    let old_parent: Signal<bool> = Signal::create(true);
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let replacement: Signal<bool> = Signal::create(true);
    let mut context: HookContext = HookContext::default();
    for _ in 0..4 {
        bind_submenu(&mut context, old_parent, child);
    }
    assert!(child.get(), "unchanged rerenders must not close the branch");
    bind_submenu(&mut context, parent, replacement);
    assert!(!child.get(), "replaced branch closes");
    child.set(true);
    old_parent.set(false);
    assert!(child.get(), "old parent subscription was removed");
    assert!(replacement.get());
    context.switch_arm(1);
    assert!(!replacement.get(), "unmount closes branch");
    replacement.set(true);
    parent.set(false);
    assert!(
        replacement.get(),
        "unmounted parent subscription was removed"
    );
    replacement.set(false);
    replacement.set(true);
    assert!(replacement.get(), "unmounted child guard was removed");
}

#[test]
fn mounting_submenu_under_a_closed_parent_normalizes_real_state_before_render() {
    let parent_open: Signal<bool> = Signal::create(false);
    let open: Signal<bool> = Signal::create(true);
    let mut context: HookContext = HookContext::default();
    let node: VirtualNode = HookContext::with(context.clone(), || {
        euv_context_menu_sub(props_node(EuvContextMenuSubProps { open, parent_open }))
    });
    assert!(!open.get());
    assert!(has_attribute(&node, "data-state"));
    assert_eq!(
        submenu_attribute(open, "aria-expanded", false).as_deref(),
        Some("false")
    );
    context.switch_arm(1);
}

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

fn radio_attribute(props: &EuvContextMenuRadioItemProps, name: &str) -> Option<String> {
    let node: VirtualNode = euv_context_menu_radio_item(props_node(props.clone()));
    elements(&node).iter().find_map(|element: &VirtualNode| {
        if let VirtualNode::Element { attributes, .. } = element {
            attributes
                .iter()
                .find(|entry: &&AttributeEntry| entry.get_name() == name)
                .and_then(AttributeEntry::text_value)
        } else {
            None
        }
    })
}

#[test]
fn disabled_radio_uses_the_shared_disabled_row_style() {
    let node: VirtualNode = euv_context_menu_radio_item(props_node(EuvContextMenuRadioItemProps {
        disabled: true,
        ..EuvContextMenuRadioItemProps::default()
    }));
    assert!(has_class(&node, "c_euv_context_menu_item_disabled"));
}

#[test]
fn radio_group_excludes_siblings_preserves_other_groups_and_reports_changes_once() {
    let changes: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorded: Rc<RefCell<Vec<String>>> = changes.clone();
    let selection: EuvContextMenuRadioGroupState = use_context_menu_radio_group(
        None,
        "grid",
        Some(Rc::new(move |value: String| {
            recorded.borrow_mut().push(value)
        })),
    );
    let other: EuvContextMenuRadioGroupState = use_context_menu_radio_group(None, "other", None);
    let first: EuvContextMenuRadioItemProps = EuvContextMenuRadioItemProps {
        value: "grid",
        selection: selection.clone(),
        open: Signal::create(true),
        ..EuvContextMenuRadioItemProps::default()
    };
    let second: EuvContextMenuRadioItemProps = EuvContextMenuRadioItemProps {
        value: "list",
        ..first.clone()
    };
    assert_eq!(
        radio_attribute(&first, "aria-checked").as_deref(),
        Some("true")
    );
    assert_eq!(
        radio_attribute(&second, "aria-checked").as_deref(),
        Some("false")
    );
    assert!(!activate_context_menu_radio(&second));
    assert!(!activate_context_menu_radio(&second));
    assert_eq!(selection.value.get(), "list");
    assert_eq!(other.value.get(), "other");
    assert_eq!(changes.borrow().as_slice(), &["list"]);
    assert_eq!(
        radio_attribute(&first, "aria-checked").as_deref(),
        Some("false")
    );
    assert_eq!(
        radio_attribute(&second, "aria-checked").as_deref(),
        Some("true")
    );
    assert!(first.open.get());
    first.open.set(false);
    first.open.set(true);
    assert_eq!(
        radio_attribute(&second, "aria-checked").as_deref(),
        Some("true")
    );
}

#[test]
fn radio_disabled_activation_neither_changes_value_nor_closes_the_menu() {
    let props: EuvContextMenuRadioItemProps = EuvContextMenuRadioItemProps {
        value: "disabled",
        disabled: true,
        close_on_click: true,
        open: Signal::create(true),
        selection: use_context_menu_radio_group(
            None,
            "grid",
            Some(Rc::new(|_: String| panic!("disabled callback"))),
        ),
        ..EuvContextMenuRadioItemProps::default()
    };
    assert!(!activate_context_menu_radio(&props));
    assert_eq!(props.selection.value.get(), "grid");
    assert!(props.open.get());
    assert_eq!(
        radio_attribute(&props, "aria-disabled").as_deref(),
        Some("true")
    );
    assert_eq!(radio_attribute(&props, "tabindex").as_deref(), Some("-1"));
}

#[test]
fn radio_opt_in_close_also_closes_an_already_selected_choice_without_callback() {
    let props: EuvContextMenuRadioItemProps = EuvContextMenuRadioItemProps {
        value: "grid",
        close_on_click: true,
        open: Signal::create(true),
        selection: use_context_menu_radio_group(
            None,
            "grid",
            Some(Rc::new(|_: String| panic!("unchanged callback"))),
        ),
        ..EuvContextMenuRadioItemProps::default()
    };
    assert!(!EuvContextMenuRadioItemProps::default().close_on_click);
    assert!(activate_context_menu_radio(&props));
    assert!(!props.open.get());
    assert_eq!(props.selection.value.get(), "grid");
}

#[test]
fn controlled_radio_requests_changes_but_waits_for_the_owner_to_update_its_signal() {
    let value: Signal<String> = Signal::create("grid".to_owned());
    let changes: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let recorded: Rc<RefCell<Vec<String>>> = changes.clone();
    let selection: EuvContextMenuRadioGroupState = use_context_menu_radio_group(
        Some(value),
        "ignored",
        Some(Rc::new(move |next: String| {
            recorded.borrow_mut().push(next)
        })),
    );
    let props: EuvContextMenuRadioItemProps = EuvContextMenuRadioItemProps {
        value: "list",
        selection,
        ..EuvContextMenuRadioItemProps::default()
    };
    assert!(!activate_context_menu_radio(&props));
    assert_eq!(value.get(), "grid");
    assert_eq!(changes.borrow().as_slice(), &["list"]);
    value.set("list".to_owned());
    assert_eq!(
        radio_attribute(&props, "aria-checked").as_deref(),
        Some("true")
    );
    assert!(!activate_context_menu_radio(&props));
    assert_eq!(changes.borrow().len(), 1);
}

#[test]
fn long_press_opens_at_500ms_once_with_the_original_coordinates() {
    let mut hold: ContextMenuLongPress = ContextMenuLongPress::default();
    hold.start(false, 1, 120, 240, 100.0);
    hold.moved(1, 129, 250);
    assert_eq!(hold.finish(599.0), None);
    assert_eq!(hold.finish(600.0), Some((120, 240)));
    assert_eq!(hold.finish(900.0), None);
}

#[test]
fn long_press_cancels_on_either_axis_beyond_ten_pixels_or_multiple_touches() {
    for (count, x, y) in [
        (1, 111, 200),
        (1, 89, 200),
        (1, 100, 211),
        (1, 100, 189),
        (2, 100, 200),
        (0, 100, 200),
    ] {
        let mut hold: ContextMenuLongPress = ContextMenuLongPress::default();
        hold.start(false, 1, 100, 200, 0.0);
        hold.moved(count, x, y);
        hold.moved(1, 100, 200);
        assert_eq!(hold.finish(500.0), None);
    }
}

#[test]
fn long_press_rejects_disabled_and_non_single_touch_starts() {
    for (disabled, count) in [(true, 1), (false, 0), (false, 2)] {
        let mut hold: ContextMenuLongPress = ContextMenuLongPress::default();
        hold.start(false, 1, 10, 20, 0.0);
        hold.start(disabled, count, 10, 20, 100.0);
        assert_eq!(hold.finish(1000.0), None);
    }
}

#[test]
fn long_press_cancellation_clears_pending_state_and_new_holds_restart_the_delay() {
    let mut hold: ContextMenuLongPress = ContextMenuLongPress::default();
    hold.start(false, 1, 10, 20, 0.0);
    hold.cancel();
    assert_eq!(hold.finish(500.0), None);
    assert!(hold.timer.is_none());
    assert!(hold.callback.is_none());
    hold.start(false, 1, 30, 40, 400.0);
    assert_eq!(hold.finish(500.0), None);
    assert_eq!(hold.finish(900.0), Some((30, 40)));
}

#[test]
fn context_menu_triggers_bind_all_touch_lifecycle_events_without_changing_open_state() {
    for disabled in [false, true] {
        let open: Signal<bool> = Signal::create(false);
        let node: VirtualNode = euv_context_menu_trigger(props_node(EuvContextMenuTriggerProps {
            open,
            disabled,
            ..EuvContextMenuTriggerProps::default()
        }));
        for event in ["ontouchstart", "ontouchmove", "ontouchend", "ontouchcancel"] {
            assert!(has_attribute(&node, event));
        }
        assert!(has_attribute(&node, "aria-disabled"));
        assert!(!open.get());
    }
}

#[test]
fn context_menu_action_and_checkbox_have_distinct_dismissal_defaults() {
    let action: EuvContextMenuItemProps = EuvContextMenuItemProps::default();
    let checkbox: EuvContextMenuCheckboxItemProps = EuvContextMenuCheckboxItemProps::default();
    assert!(action.close_on_click);
    assert!(!action.disabled);
    assert!(!checkbox.close_on_click);
}

#[test]
fn context_menu_dismissal_options_preserve_controlled_state_during_render() {
    for close_on_click in [false, true] {
        let open: Signal<bool> = Signal::create(true);
        let checked: Signal<bool> = Signal::create(false);
        let action: VirtualNode = euv_context_menu_item(props_node(EuvContextMenuItemProps {
            open,
            close_on_click,
            ..EuvContextMenuItemProps::default()
        }));
        let checkbox: VirtualNode =
            euv_context_menu_checkbox_item(props_node(EuvContextMenuCheckboxItemProps {
                open,
                checked,
                close_on_click,
                ..EuvContextMenuCheckboxItemProps::default()
            }));
        assert!(has_attribute(&action, "onclick"));
        assert!(has_attribute(&checkbox, "onclick"));
        assert!(open.get());
        assert!(!checked.get());
    }
}

#[test]
fn a_disabled_context_menu_action_keeps_its_disabled_style_for_both_dismissal_options() {
    for close_on_click in [false, true] {
        let node: VirtualNode = euv_context_menu_item(props_node(EuvContextMenuItemProps {
            disabled: true,
            close_on_click,
            ..EuvContextMenuItemProps::default()
        }));
        assert!(has_class(&node, "c_euv_context_menu_item_disabled"));
        assert!(has_attribute(&node, "aria-disabled"));
    }
}

#[test]
fn a_context_menu_trigger_opens_from_mouse_and_keyboard_context_menu_events() {
    let node: VirtualNode = euv_context_menu_trigger(props_node(EuvContextMenuTriggerProps {
        open: Signal::create(false),
        label: "Actions",
        disabled: false,
    }));
    assert!(has_class(&node, "c_euv_context_menu_trigger"));
    assert!(has_attribute(&node, "oncontextmenu"));
    assert!(has_attribute(&node, "onkeydown"));
    assert!(has_attribute(&node, "tabindex"));
    assert!(!has_attribute(&node, "onclick"));
}

#[test]
fn a_context_menu_root_handles_outside_dismissal_and_keyboard_actions() {
    let node: VirtualNode = euv_context_menu(props_node(EuvContextMenuProps {
        open: Signal::create(false),
    }));
    assert!(has_attribute(&node, "ref"));
    assert!(has_attribute(&node, "onkeydown"));
}

#[test]
fn the_context_menu_panel_is_a_menu() {
    let node: VirtualNode = euv_context_menu_content(props_node(EuvContextMenuContentProps {
        open: Signal::create(true),
    }));
    assert!(has_class(&node, "c_euv_context_menu_content"));
    assert!(has_class(&node, "c_euv_context_menu_content_open"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "tabindex"));
}

#[test]
fn a_context_menu_row_is_a_menu_item() {
    let node: VirtualNode = euv_context_menu_item(props_node(EuvContextMenuItemProps {
        open: Signal::create(true),
        value: "copy",
        label: "Copy",
        disabled: false,
        close_on_click: true,
        on_select: None,
    }));
    assert!(has_class(&node, "c_euv_context_menu_item"));
    assert!(has_attribute(&node, "role"));
    assert!(has_attribute(&node, "onclick"));
}

#[test]
fn a_context_menu_carries_a_box_row_and_a_dot_row() {
    let checkbox: VirtualNode =
        euv_context_menu_checkbox_item(props_node(EuvContextMenuCheckboxItemProps {
            open: Signal::create(true),
            close_on_click: false,
            label: "Show grid",
            checked: Signal::create(true),
            on_select: None,
        }));
    assert!(has_class(&checkbox, "c_euv_context_menu_checkbox_item"));
    assert!(has_attribute(&checkbox, "aria-checked"));
    let radio: VirtualNode =
        euv_context_menu_radio_item(props_node(EuvContextMenuRadioItemProps {
            label: "Small",
            value: "small",
            ..EuvContextMenuRadioItemProps::default()
        }));
    assert!(has_class(&radio, "c_euv_context_menu_radio_item"));
    assert!(has_attribute(&radio, "aria-checked"));
}

#[test]
fn a_context_menu_carries_a_label_a_separator_and_a_shortcut() {
    let label: VirtualNode =
        euv_context_menu_label(props_node(EuvContextMenuLabelProps { text: "Actions" }));
    assert!(has_class(&label, "c_euv_context_menu_label"));
    let separator: VirtualNode =
        euv_context_menu_separator(props_node(EuvContextMenuSeparatorProps { label: "" }));
    assert!(has_class(&separator, "c_euv_context_menu_separator"));
    assert!(has_attribute(&separator, "role"));
    let shortcut: VirtualNode =
        euv_context_menu_shortcut(props_node(EuvContextMenuShortcutProps { text: "Ctrl+C" }));
    assert!(has_class(&shortcut, "c_euv_context_menu_shortcut"));
}

#[test]
fn context_menu_submenu_keys_follow_logical_direction() {
    assert_eq!(
        context_menu_direction_keys(false),
        ("ArrowRight", "ArrowLeft")
    );
    assert_eq!(
        context_menu_direction_keys(true),
        ("ArrowLeft", "ArrowRight")
    );
}

#[test]
fn context_menu_submenu_prefers_inline_end_and_flips_at_both_edges() {
    let viewport: (f64, f64) = (1000.0, 700.0);
    let panel: (f64, f64) = (160.0, 200.0);
    let anchor: (f64, f64, f64) = (400.0, 600.0, 100.0);
    assert_eq!(
        context_menu_sub_position(anchor, panel, viewport, false),
        (600.0, 100.0)
    );
    assert_eq!(
        context_menu_sub_position(anchor, panel, viewport, true),
        (240.0, 100.0)
    );
    for rtl in [false, true] {
        assert_eq!(
            context_menu_sub_position((10.0, 210.0, 600.0), panel, viewport, rtl),
            (210.0, 500.0)
        );
        assert_eq!(
            context_menu_sub_position((790.0, 990.0, 600.0), panel, viewport, rtl),
            (630.0, 500.0)
        );
    }
}

#[test]
fn context_menu_submenu_collision_positions_stay_within_available_viewport() {
    for rtl in [false, true] {
        for width in [80.0_f64, 160.0, 320.0, 1000.0] {
            for x in [-40.0, 0.0, 80.0, 280.0, 960.0] {
                for y in [-20.0, 0.0, 450.0, 700.0] {
                    let panel: (f64, f64) = (160.0_f64.min(width), 200.0);
                    let (left, top): (f64, f64) =
                        context_menu_sub_position((x, x + 200.0, y), panel, (width, 700.0), rtl);
                    assert!(left >= 0.0 && left + panel.0 <= width);
                    assert!(top >= 0.0 && top + panel.1 <= 700.0);
                }
            }
        }
    }
}

#[test]
fn context_menu_submenu_panel_has_a_mounted_positioning_reference() {
    let open: Signal<bool> = Signal::create(true);
    let node: VirtualNode =
        euv_context_menu_sub_content(props_node(EuvContextMenuSubContentProps { open }));
    assert!(has_attribute(&node, "ref"));
    assert!(has_attribute(&node, "onkeydown"));
    assert!(has_attribute(&node, "data-state"));
    assert!(open.get());
}

#[test]
fn context_menu_keyboard_navigation_wraps_and_has_endpoints() {
    let labels: Vec<String> = vec!["Copy".into(), "Show grid".into(), "More actions".into()];
    assert_eq!(context_menu_next_index(&labels, None, "ArrowDown"), Some(0));
    assert_eq!(context_menu_next_index(&labels, None, "ArrowUp"), Some(2));
    assert_eq!(
        context_menu_next_index(&labels, Some(2), "ArrowDown"),
        Some(0)
    );
    assert_eq!(
        context_menu_next_index(&labels, Some(0), "ArrowUp"),
        Some(2)
    );
    assert_eq!(context_menu_next_index(&labels, Some(1), "Home"), Some(0));
    assert_eq!(context_menu_next_index(&labels, Some(0), "End"), Some(2));
    assert_eq!(context_menu_next_index(&[], None, "ArrowDown"), None);
}

#[test]
fn context_menu_typeahead_cycles_matches_and_searches_prefixes() {
    let labels: Vec<String> = vec![
        "Copy".into(),
        "Copy and keep open".into(),
        " Show grid".into(),
    ];
    assert_eq!(context_menu_next_index(&labels, None, "c"), Some(0));
    assert_eq!(context_menu_next_index(&labels, Some(0), "c"), Some(1));
    assert_eq!(context_menu_next_index(&labels, Some(1), "c"), Some(0));
    assert_eq!(
        context_menu_next_index(&labels, Some(0), "copy and"),
        Some(1)
    );
    assert_eq!(context_menu_next_index(&labels, Some(1), "sh"), Some(2));
    assert_eq!(context_menu_next_index(&labels, Some(0), "missing"), None);
}

#[test]
fn context_menu_rows_leave_keyboard_delegation_to_the_popup() {
    let rows: Vec<VirtualNode> = vec![
        euv_context_menu_item(props_node(EuvContextMenuItemProps::default())),
        euv_context_menu_item(props_node(EuvContextMenuItemProps {
            disabled: true,
            ..EuvContextMenuItemProps::default()
        })),
        euv_context_menu_checkbox_item(props_node(EuvContextMenuCheckboxItemProps::default())),
        euv_context_menu_radio_item(props_node(EuvContextMenuRadioItemProps::default())),
        euv_context_menu_sub_trigger(props_node(EuvContextMenuSubTriggerProps::default())),
    ];
    for row in rows {
        assert!(has_attribute(&row, "tabindex"));
        assert!(!has_attribute(&row, "onkeydown"));
    }
    let popup: VirtualNode =
        euv_context_menu_content(props_node(EuvContextMenuContentProps::default()));
    let sub: VirtualNode =
        euv_context_menu_sub_content(props_node(EuvContextMenuSubContentProps::default()));
    assert!(has_attribute(&popup, "onkeydown"));
    assert!(has_attribute(&sub, "onkeydown"));
}

#[test]
fn context_submenu_hover_defaults_match_base_ui() {
    let props: EuvContextMenuSubTriggerProps = EuvContextMenuSubTriggerProps::default();
    assert!(!props.disabled);
    assert!(props.open_on_hover);
    assert_eq!(props.delay_ms, 100);
    assert_eq!(props.close_delay_ms, 0);
}

#[test]
fn context_hover_never_accepts_touch_or_disabled_or_closed_owners() {
    for pointer in ["mouse", "pen"] {
        assert!(context_menu_hover_eligible(
            pointer, true, false, true, true
        ));
        for (hover, disabled, enabled, parent) in [
            (false, false, true, true),
            (true, true, true, true),
            (true, false, false, true),
            (true, false, true, false),
        ] {
            assert!(!context_menu_hover_eligible(
                pointer, hover, disabled, enabled, parent
            ));
        }
    }
    for pointer in ["touch", "", "unknown"] {
        assert!(!context_menu_hover_eligible(
            pointer, true, false, true, true
        ));
    }
}

#[test]
fn context_hover_branch_intent_cancels_early_leave_and_dismisses_only_own_state() {
    assert_eq!(
        context_menu_hover_intent(false, true, false, false, true),
        Some(true)
    );
    assert_eq!(
        context_menu_hover_intent(false, false, false, false, true),
        None
    );
    assert_eq!(
        context_menu_hover_intent(false, true, false, false, false),
        None
    );
    assert_eq!(
        context_menu_hover_intent(true, false, true, false, true),
        None
    );
    assert_eq!(
        context_menu_hover_intent(true, false, false, true, true),
        None
    );
    assert_eq!(
        context_menu_hover_intent(true, false, false, false, true),
        Some(false)
    );
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let mut context: HookContext = HookContext::default();
    bind_submenu(&mut context, parent, child);
    child.set(false);
    assert!(parent.get());
}

#[test]
fn context_hover_bridge_follows_physical_edges_in_both_directions() {
    let row: (f64, f64, f64, f64) = (100.0, 100.0, 200.0, 130.0);
    assert!(context_menu_hover_bridge(
        (210.0, 115.0),
        (190.0, 115.0),
        row,
        (220.0, 90.0, 380.0, 200.0)
    ));
    assert!(context_menu_hover_bridge(
        (90.0, 115.0),
        (110.0, 115.0),
        row,
        (-80.0, 90.0, 80.0, 200.0)
    ));
    assert!(!context_menu_hover_bridge(
        (210.0, 250.0),
        (190.0, 115.0),
        row,
        (220.0, 90.0, 380.0, 200.0)
    ));
    assert!(!context_menu_hover_bridge(
        (400.0, 115.0),
        (190.0, 115.0),
        row,
        (220.0, 90.0, 380.0, 200.0)
    ));
    assert!(!context_menu_hover_bridge(
        (210.0, 115.0),
        (190.0, 115.0),
        row,
        (220.0, 90.0, 220.0, 90.0)
    ));
}

#[test]
fn context_closed_submenu_is_semantically_and_visually_hidden() {
    let open: Signal<bool> = Signal::create(false);
    assert_eq!(
        submenu_attribute(open, "aria-hidden", true).as_deref(),
        Some("true")
    );
    assert_eq!(
        submenu_attribute(open, "style", true).as_deref(),
        Some("display:none;")
    );
    open.set(true);
    assert_eq!(
        submenu_attribute(open, "aria-hidden", true).as_deref(),
        Some("false")
    );
    assert_eq!(submenu_attribute(open, "style", true).as_deref(), Some(""));
}

#[test]
fn context_binding_drop_without_cleanup_detaches_owner_and_closes_branch() {
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let mut context: HookContext = HookContext::default();
    bind_submenu(&mut context, parent, child);
    drop(context);
    assert!(!child.get());
    child.set(true);
    parent.set(false);
    assert!(child.get());
}
