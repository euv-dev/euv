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
fn sheet_scroll_lease_nested_and_independent_roots_release_only_last_owner() {
    let restored: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let on_restore: Rc<Cell<usize>> = restored.clone();
    let parent: Rc<SheetScrollLease> = Rc::new(SheetScrollLease {
        cleanup: Some(Box::new(move || on_restore.set(on_restore.get() + 1))),
    });
    let child: Rc<SheetScrollLease> = parent.clone();
    let independent: Rc<SheetScrollLease> = parent.clone();
    drop(child);
    assert_eq!(restored.get(), 0);
    drop(parent);
    assert_eq!(restored.get(), 0);
    drop(independent);
    assert_eq!(restored.get(), 1);
}

#[test]
fn sheet_scroll_lease_duplicate_close_and_always_mounted_slot_do_not_unlock_parent() {
    let restored: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let on_restore: Rc<Cell<bool>> = restored.clone();
    let parent: Rc<SheetScrollLease> = Rc::new(SheetScrollLease {
        cleanup: Some(Box::new(move || on_restore.set(true))),
    });
    let mut child: Option<Rc<SheetScrollLease>> = Some(parent.clone());
    child.take();
    child.take();
    assert!(!restored.get());
    drop(parent);
    assert!(restored.get());
    assert!(child.is_none());
}

#[test]
fn sheet_scroll_lease_open_unmount_restores_and_weak_registry_cannot_keep_it_alive() {
    let restored: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let on_restore: Rc<Cell<bool>> = restored.clone();
    let root: Rc<SheetScrollLease> = Rc::new(SheetScrollLease {
        cleanup: Some(Box::new(move || on_restore.set(true))),
    });
    assert_eq!(Rc::weak_count(&root), 0);
    assert!(Rc::downgrade(&root).upgrade().is_some());
    drop(root);
    assert!(restored.get());
}

#[test]
fn sheet_scroll_lease_different_document_snapshots_restore_independently() {
    let first_restored: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let second_restored: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let first_callback: Rc<Cell<bool>> = first_restored.clone();
    let second_callback: Rc<Cell<bool>> = second_restored.clone();
    let first_document: Rc<SheetScrollLease> = Rc::new(SheetScrollLease {
        cleanup: Some(Box::new(move || first_callback.set(true))),
    });
    let second_document: Rc<SheetScrollLease> = Rc::new(SheetScrollLease {
        cleanup: Some(Box::new(move || second_callback.set(true))),
    });
    drop(first_document);
    assert!(first_restored.get());
    assert!(!second_restored.get());
    drop(second_document);
    assert!(second_restored.get());
}

#[test]
fn sheet_scroll_restore_policy_is_bounded_and_gutter_is_conditional() {
    for name in [
        "overflow",
        "overflow-x",
        "overflow-y",
        "overflow-block",
        "overflow-inline",
    ] {
        assert!(sheet_scroll_owns_property(name, false));
        assert!(sheet_scroll_owns_property(name, true));
    }
    assert!(!sheet_scroll_owns_property("scrollbar-gutter", false));
    assert!(sheet_scroll_owns_property("scrollbar-gutter", true));
    for name in [
        "color",
        "padding-right",
        "height",
        "position",
        "top",
        "--runtime",
        "overflow-anchor",
    ] {
        assert!(!sheet_scroll_owns_property(name, false));
        assert!(!sheet_scroll_owns_property(name, true));
    }
}

#[test]
fn sheet_parent_close_recursively_resets_always_mounted_descendants() {
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let deeper: Signal<bool> = Signal::create(true);
    let child_cleanup: Box<dyn FnOnce()> = sheet_parent_binding(child, Some(parent));
    let deeper_cleanup: Box<dyn FnOnce()> = sheet_parent_binding(deeper, Some(child));
    parent.set(false);
    assert!(!child.get());
    assert!(!deeper.get());
    parent.set(true);
    assert!(!child.get());
    assert!(!deeper.get());
    child_cleanup();
    deeper_cleanup();
}

#[test]
fn sheet_closed_parent_gates_initial_and_external_child_open() {
    let parent: Signal<bool> = Signal::create(false);
    let child: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = sheet_parent_binding(child, Some(parent));
    assert!(!child.get());
    child.set(true);
    assert!(!child.get());
    parent.set(true);
    child.set(true);
    assert!(child.get());
    parent.set(false);
    child.set(true);
    assert!(!child.get());
    cleanup();
}

#[test]
fn sheet_binding_cleanup_detaches_both_subscriptions() {
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = sheet_parent_binding(child, Some(parent));
    cleanup();
    assert!(!child.get());
    child.set(true);
    parent.set(false);
    assert!(child.get());
    child.set(false);
    child.set(true);
    assert!(child.get());
}

#[test]
fn sheet_child_close_preserves_parent_and_independent_root() {
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let independent: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = sheet_parent_binding(child, Some(parent));
    let independent_cleanup: Box<dyn FnOnce()> = sheet_parent_binding(independent, None);
    child.set(false);
    assert!(parent.get());
    assert!(independent.get());
    parent.set(false);
    assert!(independent.get());
    cleanup();
    independent_cleanup();
}

#[test]
fn sheet_escape_contract_rejects_handled_event_after_nearest_root_changes() {
    assert!(!sheet_handles_event(true, false, false));
    assert!(sheet_handles_event(true, false, true));
    assert!(!sheet_handles_event(true, true, true));
    assert!(!sheet_handles_event(false, false, true));
    assert!(sheet_handles_event(true, false, true));
}

#[test]
fn sheet_nested_vnode_remains_mounted_after_recursive_reset() {
    let parent: Signal<bool> = Signal::create(true);
    let child: Signal<bool> = Signal::create(true);
    let cleanup: Box<dyn FnOnce()> = sheet_parent_binding(child, Some(parent));
    parent.set(false);
    let panel: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: child,
        side: EuvSheetSide::Right,
    }));
    let trigger: VirtualNode = euv_sheet_trigger(props_node(EuvSheetTriggerProps {
        open: child,
        label: "Open nested sheet",
    }));
    assert_eq!(attribute(&panel, "data-state").as_deref(), Some("closed"));
    assert_eq!(attribute(&panel, "aria-hidden").as_deref(), Some("true"));
    assert_eq!(
        attribute(&trigger, "aria-expanded").as_deref(),
        Some("false")
    );
    assert!(!elements(&panel).is_empty());
    cleanup();
}

#[test]
fn sheet_parent_prop_defaults_to_independent_root() {
    assert!(EuvSheetProps::default().parent_open.is_none());
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
fn sheet_all_edges_report_the_actual_content_side() {
    for (side, name) in [
        (EuvSheetSide::Top, "top"),
        (EuvSheetSide::Right, "right"),
        (EuvSheetSide::Bottom, "bottom"),
        (EuvSheetSide::Left, "left"),
    ] {
        let panel: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
            open: Signal::create(true),
            side,
        }));
        assert_eq!(attribute(&panel, "data-side").as_deref(), Some(name));
        assert!(has_class(&panel, &format!("c_euv_sheet_content_{name}")));
        let classes: Vec<String> = class_names(&panel);
        assert_eq!(
            classes
                .iter()
                .filter(|name: &&String| { **name == "c_euv_sheet_content" })
                .count(),
            1
        );
    }
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

fn attribute(node: &VirtualNode, name: &str) -> Option<String> {
    elements(node).iter().find_map(|element: &VirtualNode| {
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

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn sheet_native_close_updates_only_its_shared_open_signal() {
    let open: Signal<bool> = Signal::create(true);
    let other: Signal<bool> = Signal::create(true);
    let close: VirtualNode = euv_sheet_close(props_node(EuvSheetCloseProps {
        open,
        label: "Close",
    }));
    assert!(has_attribute(&close, "onclick"));
    sheet_dismiss(open).unwrap()(JsValue::NULL.unchecked_into());
    assert!(!open.get());
    assert!(other.get());
    let trigger: VirtualNode = euv_sheet_trigger(props_node(EuvSheetTriggerProps {
        open,
        label: "Open",
    }));
    let content: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open,
        side: EuvSheetSide::Right,
    }));
    assert_eq!(
        attribute(&trigger, "aria-expanded").as_deref(),
        Some("false")
    );
    assert_eq!(attribute(&content, "aria-hidden").as_deref(), Some("true"));
    assert!(has_class(&content, "c_euv_sheet_content_closed"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn sheet_native_overlay_click_closes_shared_state() {
    let open: Signal<bool> = Signal::create(true);
    let overlay: VirtualNode = euv_sheet_overlay(props_node(EuvSheetOverlayProps { open }));
    assert!(has_attribute(&overlay, "onclick"));
    sheet_dismiss(open).unwrap()(JsValue::NULL.unchecked_into());
    assert!(!open.get());
    assert_eq!(attribute(&overlay, "aria-hidden").as_deref(), Some("true"));
}

#[test]
fn sheet_trigger_controls_its_own_stable_dialog_id() {
    let open: Signal<bool> = Signal::create(false);
    let trigger: VirtualNode = euv_sheet_trigger(props_node(EuvSheetTriggerProps {
        open,
        label: "Open",
    }));
    let content: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open,
        side: EuvSheetSide::Right,
    }));
    assert_eq!(
        attribute(&trigger, "aria-controls"),
        attribute(&content, "id")
    );
    assert_eq!(
        attribute(&trigger, "aria-haspopup").as_deref(),
        Some("dialog")
    );
    assert!(attribute(&trigger, "id").is_some());
    open.set(true);
    let reopened: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open,
        side: EuvSheetSide::Right,
    }));
    assert_eq!(attribute(&content, "id"), attribute(&reopened, "id"));
}

#[test]
fn sheet_dialog_ids_are_isolated_between_roots() {
    let first: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: Signal::create(false),
        side: EuvSheetSide::Right,
    }));
    let second: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open: Signal::create(false),
        side: EuvSheetSide::Right,
    }));
    assert_ne!(attribute(&first, "id"), attribute(&second, "id"));
}

#[test]
fn sheet_closed_state_keeps_content_mounted_but_hidden_from_accessibility() {
    let open: Signal<bool> = Signal::create(false);
    let closed: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open,
        side: EuvSheetSide::Right,
    }));
    assert_eq!(attribute(&closed, "aria-hidden").as_deref(), Some("true"));
    assert_eq!(attribute(&closed, "data-state").as_deref(), Some("closed"));
    assert_eq!(attribute(&closed, "tabindex").as_deref(), Some("-1"));
    assert!(has_class(&closed, "c_euv_sheet_content_closed"));
    open.set(true);
    let opened: VirtualNode = euv_sheet_content(props_node(EuvSheetContentProps {
        open,
        side: EuvSheetSide::Right,
    }));
    assert_eq!(attribute(&opened, "aria-hidden").as_deref(), Some("false"));
    assert_eq!(attribute(&opened, "data-state").as_deref(), Some("open"));
    assert_eq!(elements(&closed).len(), elements(&opened).len());
}

#[test]
fn sheet_close_renders_a_decorative_current_color_svg_x() {
    let close: VirtualNode = euv_sheet_close(props_node(EuvSheetCloseProps {
        open: Signal::create(false),
        label: "Dismiss sheet",
    }));
    assert_eq!(
        attribute(&close, "aria-label").as_deref(),
        Some("Dismiss sheet")
    );
    assert!(has_attribute(&close, "inner_html"));
    let markup: &str = crate::SHEET_CLOSE_MARKUP;
    for expected in [
        "xmlns=\"http://www.w3.org/2000/svg\"",
        "aria-hidden=\"true\"",
        "focusable=\"false\"",
        "width=\"16\"",
        "height=\"16\"",
        "stroke=\"currentColor\"",
        "M18 6 6 18M6 6l12 12",
    ] {
        assert!(markup.contains(expected), "{expected}");
    }
}

#[test]
fn sheet_trigger_and_close_preserve_composed_children() {
    let open: Signal<bool> = Signal::create(false);
    let mut trigger: VirtualNode<EuvSheetTriggerProps> = props_node(EuvSheetTriggerProps {
        open,
        label: "Open",
    });
    if let VirtualNode::Element { children, .. } = &mut trigger {
        children.push(html! { span { "Trigger child" } });
    }
    let mut close: VirtualNode<EuvSheetCloseProps> = props_node(EuvSheetCloseProps {
        open,
        label: "Close",
    });
    if let VirtualNode::Element { children, .. } = &mut close {
        children.push(html! { span { "Close child" } });
    }
    assert!(format!("{:?}", euv_sheet_trigger(trigger)).contains("Trigger child"));
    assert!(format!("{:?}", euv_sheet_close(close)).contains("Close child"));
}
