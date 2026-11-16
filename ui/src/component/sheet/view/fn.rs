use super::*;

/// The frame that holds a sheet.
///
/// The frame owns the open state, so the trigger and the panel always
/// agree on whether the sheet is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet virtual DOM tree.
#[component]
pub fn euv_sheet(node: VirtualNode<EuvSheetProps>) -> VirtualNode {
    let EuvSheetProps {
        open,
        side: _side,
        parent_open,
    }: EuvSheetProps = node.try_get_props().unwrap_or_default();
    use_sheet_parent_binding(open, parent_open);
    let children: VirtualNode = node.get_children().into();
    let root: NodeRef<Element> = App::use_node_ref();
    #[cfg(target_arch = "wasm32")]
    use_sheet_modal(open, root.clone());
    #[cfg(not(target_arch = "wasm32"))]
    let _: Signal<bool> = open;
    html! {
        div {
            ref: root
            "data-slot": "sheet"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_sheet()
            children
        }
    }
}

/// The button that opens the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet trigger virtual DOM tree.
#[component]
pub fn euv_sheet_trigger(node: VirtualNode<EuvSheetTriggerProps>) -> VirtualNode {
    let EuvSheetTriggerProps { open, label }: EuvSheetTriggerProps =
        node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        open.set(true);
    }));
    let children: VirtualNode = node.get_children().into();
    let trigger_id: String = sheet_part_id(open, SHEET_TRIGGER_PART);
    let content_id: String = sheet_part_id(open, SHEET_CONTENT_PART);
    html! {
            button {
                type: "button"
                id: trigger_id
                "aria-controls": content_id
                "aria-haspopup": "dialog"
                "data-slot": "sheet-trigger"
                "aria-expanded": if { open.get() } {
                    "true"
                } else {
                    "false"
                }
                class: c_euv_sheet_trigger()

                onclick: on_click
                {
                    label
                }
                children
            }
    }
}

/// The veil that covers the page while the sheet is open.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetOverlayProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet overlay virtual DOM tree.
#[component]
pub fn euv_sheet_overlay(node: VirtualNode<EuvSheetOverlayProps>) -> VirtualNode {
    let EuvSheetOverlayProps { open }: EuvSheetOverlayProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "sheet-overlay"
            "aria-hidden": "true"
            onclick: sheet_dismiss(open)
            class: c_euv_sheet_overlay()
            class: if { open } {
                c_euv_sheet_overlay_open()
            } else {
                c_euv_sheet_overlay_closed()
            }
        }
    }
}

/// The panel of the sheet.
///
/// The panel is a dialog: it carries the role and the modal flag, so the
/// page behind it is announced as unreachable.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet content virtual DOM tree.
#[component]
pub fn euv_sheet_content(node: VirtualNode<EuvSheetContentProps>) -> VirtualNode {
    let EuvSheetContentProps { open, side }: EuvSheetContentProps =
        node.try_get_props().unwrap_or_default();
    let content_class: fn() -> &'static Css = { c_euv_sheet_content };
    let content_id: String = sheet_part_id(open, SHEET_CONTENT_PART);
    let side_class: fn() -> &'static Css = match side {
        EuvSheetSide::Top => c_euv_sheet_content_top,
        EuvSheetSide::Bottom => c_euv_sheet_content_bottom,
        EuvSheetSide::Left => c_euv_sheet_content_left,
        EuvSheetSide::Right => c_euv_sheet_content_right,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            id: content_id
            role: "dialog"
            tabindex: "-1"
            "aria-hidden": if { open.get() } {
                "false"
            } else {
                "true"
            }
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            "aria-modal": "true"
            "data-slot": "sheet-content"
            "data-side": match side {
                EuvSheetSide::Top => "top",
                EuvSheetSide::Right => "right",
                EuvSheetSide::Bottom => "bottom",
                EuvSheetSide::Left => "left",
            }
            class: content_class()
            class: side_class()
            class: if { open } {
                c_euv_sheet_content_open()
            } else {
                c_euv_sheet_content_closed()
            }
            children
        }
    }
}

/// The head of the sheet: its title and its description.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetHeaderProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet header virtual DOM tree.
#[component]
pub fn euv_sheet_header(node: VirtualNode<EuvSheetHeaderProps>) -> VirtualNode {
    let EuvSheetHeaderProps { label }: EuvSheetHeaderProps =
        node.try_get_props().unwrap_or_default();
    let header_class: fn() -> &'static Css = { c_euv_sheet_header };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            "data-slot": "sheet-header"
            class: header_class()
            children
        }
    }
}

/// The foot of the sheet: the actions that end the interaction.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetFooterProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet footer virtual DOM tree.
#[component]
pub fn euv_sheet_footer(node: VirtualNode<EuvSheetFooterProps>) -> VirtualNode {
    let EuvSheetFooterProps { label }: EuvSheetFooterProps =
        node.try_get_props().unwrap_or_default();
    let footer_class: fn() -> &'static Css = { c_euv_sheet_footer };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            "data-slot": "sheet-footer"
            class: footer_class()
            children
        }
    }
}

/// The title of the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet title virtual DOM tree.
#[component]
pub fn euv_sheet_title(node: VirtualNode<EuvSheetTitleProps>) -> VirtualNode {
    let EuvSheetTitleProps { text }: EuvSheetTitleProps = node.try_get_props().unwrap_or_default();
    let title_class: fn() -> &'static Css = { c_euv_sheet_title };
    html! {
        h2 {
            "data-slot": "sheet-title"
            class: title_class()
            {
                text
            }
        }
    }
}

/// The description of the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetDescriptionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet description virtual DOM tree.
#[component]
pub fn euv_sheet_description(node: VirtualNode<EuvSheetDescriptionProps>) -> VirtualNode {
    let EuvSheetDescriptionProps { text }: EuvSheetDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let description_class: fn() -> &'static Css = { c_euv_sheet_description };
    html! {
        p {
            "data-slot": "sheet-description"
            class: description_class()
            {
                text
            }
        }
    }
}

/// The button that closes the sheet.
///
/// # Arguments
///
/// - `VirtualNode<EuvSheetCloseProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv sheet close virtual DOM tree.
#[component]
pub fn euv_sheet_close(node: VirtualNode<EuvSheetCloseProps>) -> VirtualNode {
    let EuvSheetCloseProps { open, label }: EuvSheetCloseProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let close_class: fn() -> &'static Css = { c_euv_sheet_close };
    html! {
        button {
            type: "button"
            "aria-label": label
            "data-slot": "sheet-close"
            class: close_class()
            onclick: sheet_dismiss(open)
            inner_html: SHEET_CLOSE_MARKUP
            children
        }
    }
}

/// Keeps a nested sheet closed while its parent is closed.
///
/// # Arguments
/// - `Signal<bool>` - This sheet's caller-owned state.
/// - `Option<Signal<bool>>` - The immediate parent, if nested.
fn use_sheet_parent_binding(open: Signal<bool>, parent_open: Option<Signal<bool>>) {
    let binding: Rc<RefCell<Option<SheetParentBinding>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let cleanup_binding: Rc<RefCell<Option<SheetParentBinding>>> = binding.clone();
    App::use_cleanup(move || {
        if let Some((_, _, cleanup)) = cleanup_binding.borrow_mut().take() {
            cleanup();
        }
    });
    if binding
        .borrow()
        .as_ref()
        .is_some_and(|(previous, parent, _): &SheetParentBinding| {
            *previous == open && *parent == parent_open
        })
    {
        return;
    }
    if let Some((_, _, cleanup)) = binding.borrow_mut().take() {
        cleanup();
    }
    let cleanup: Box<dyn FnOnce()> = sheet_parent_binding(open, parent_open);
    *binding.borrow_mut() = Some((open, parent_open, cleanup));
}

/// Subscribes both directions; callbacks hold only Copy handles, never their owner.
///
/// # Arguments
/// - `Signal<bool>` - The child state.
/// - `Option<Signal<bool>>` - The immediate parent state.
///
/// # Returns
/// - `Box<dyn FnOnce()>` - Unsubscribes both guards and resets descendants on teardown.
pub(crate) fn sheet_parent_binding(
    open: Signal<bool>,
    parent_open: Option<Signal<bool>>,
) -> Box<dyn FnOnce()> {
    let Some(parent) = parent_open.filter(|parent: &Signal<bool>| *parent != open) else {
        return Box::new(|| {});
    };
    if !parent.get() {
        open.set(false);
    }
    let parent_id: usize = parent.subscribe(move || {
        if !parent.get() {
            open.set(false);
        }
    });
    let child_id: usize = open.subscribe(move || {
        if !parent.get() && open.get() {
            open.set(false);
        }
    });
    Box::new(move || {
        parent.unsubscribe(parent_id);
        open.unsubscribe(child_id);
        open.set(false);
    })
}

/// Decides event ownership before mutating any root's open state.
///
/// # Arguments
/// - `bool` - Whether this sheet is open.
/// - `bool` - Whether an earlier same-document capture listener handled the event.
/// - `bool` - Whether the target's nearest open sheet is this root.
///
/// # Returns
/// - `bool` - Whether this listener may handle the event.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn sheet_handles_event(open: bool, prevented: bool, nearest: bool) -> bool {
    open && !prevented && nearest
}

/// Derives a stable part ID from the caller-owned signal identity.
///
/// # Arguments
/// - `Signal<bool>` - The shared sheet state.
/// - `&str` - The part suffix.
///
/// # Returns
/// - `String` - An ID shared by matching trigger and panel props.
fn sheet_part_id(open: Signal<bool>, part: &str) -> String {
    format!("euv-sheet-{}-{part}", open.get_inner())
}

/// Builds the scoped close/overlay click handler.
///
/// # Arguments
/// - `Signal<bool>` - The shared sheet state.
///
/// # Returns
/// - `Option<Rc<dyn Fn(Event)>>` - The dismiss callback.
pub(crate) fn sheet_dismiss(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        #[cfg(target_arch = "wasm32")]
        if event.default_prevented() {
            return;
        }
        #[cfg(target_arch = "wasm32")]
        let root: Option<Element> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
            .and_then(|target: Element| target.closest(SHEET_ROOT_SELECTOR).ok().flatten());
        #[cfg(target_arch = "wasm32")]
        event.prevent_default();
        open.set(false);
        #[cfg(target_arch = "wasm32")]
        if let Some(root) = root {
            sheet_focus_trigger(&root);
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _: Event = event;
    }))
}

/// Finds a part belonging to this composition, not a nested sheet.
///
/// # Arguments
/// - `&Element` - The composition root.
/// - `&Element` - The subtree to search.
/// - `&str` - The part selector.
///
/// # Returns
/// - `Option<Element>` - The first matching own part.
#[cfg(target_arch = "wasm32")]
fn sheet_part(root: &Element, subtree: &Element, selector: &str) -> Option<Element> {
    let nodes: NodeList = subtree.query_selector_all(selector).ok()?;
    (0..nodes.length()).find_map(|index: u32| {
        nodes
            .item(index)
            .and_then(|node: Node| node.dyn_into::<Element>().ok())
            .filter(|part: &Element| {
                part.closest(SHEET_ROOT_SELECTOR).ok().flatten().as_ref() == Some(root)
            })
    })
}

/// Restores focus to this sheet's own trigger.
///
/// # Arguments
/// - `&Element` - The composition root.
#[cfg(target_arch = "wasm32")]
fn sheet_focus_trigger(root: &Element) {
    if let Some(trigger) = sheet_part(root, root, SHEET_TRIGGER_SELECTOR)
        .and_then(|part: Element| part.dyn_into::<HtmlElement>().ok())
    {
        let _: Result<(), JsValue> = trigger.focus();
    }
}

/// Collects enabled, visible tab stops in document order within this sheet.
///
/// # Arguments
/// - `&Element` - The own panel.
///
/// # Returns
/// - `Vec<HtmlElement>` - The live keyboard sequence.
#[cfg(target_arch = "wasm32")]
fn sheet_focusables(panel: &Element) -> Vec<HtmlElement> {
    let Ok(nodes) = panel.query_selector_all(SHEET_FOCUSABLE_SELECTOR) else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|index: u32| {
            nodes
                .item(index)
                .and_then(|node: Node| node.dyn_into::<HtmlElement>().ok())
        })
        .filter(|item: &HtmlElement| {
            item.tab_index() >= 0
                && !item.matches(SHEET_DISABLED_SELECTOR).unwrap_or(true)
                && item.closest(SHEET_HIDDEN_SELECTOR).ok().flatten().is_none()
                && item.offset_width() > 0
                && item.offset_height() > 0
                && window()
                    .and_then(|window: Window| window.get_computed_style(item).ok().flatten())
                    .is_some_and(|style: CssStyleDeclaration| {
                        style
                            .get_property_value(SHEET_VISIBILITY_PROPERTY)
                            .ok()
                            .as_deref()
                            == Some(SHEET_VISIBLE_VALUE)
                    })
        })
        .collect()
}

/// Synchronizes actual part relationships and focuses only on an opening transition.
///
/// # Arguments
/// - `&Element` - The mounted composition root.
/// - `Signal<bool>` - The shared sheet state.
/// - `bool` - Whether this frame should initialize panel focus.
#[cfg(target_arch = "wasm32")]
fn sheet_sync(root: &Element, open: Signal<bool>, focus: bool) {
    let Some(panel) = sheet_part(root, root, SHEET_CONTENT_SELECTOR) else {
        return;
    };
    let content_id: String = sheet_part_id(open, SHEET_CONTENT_PART);
    let _: Result<(), JsValue> = panel.set_attribute("id", &content_id);
    if let Some(trigger) = sheet_part(root, root, SHEET_TRIGGER_SELECTOR) {
        let _: Result<(), JsValue> = trigger.set_attribute("aria-controls", &content_id);
    }
    for (selector, suffix, attribute) in [
        (SHEET_TITLE_SELECTOR, "title", SHEET_LABEL_ATTRIBUTE),
        (
            SHEET_DESCRIPTION_SELECTOR,
            "description",
            SHEET_DESCRIPTION_ATTRIBUTE,
        ),
    ] {
        if let Some(part) = sheet_part(root, &panel, selector) {
            let id: String = format!("{content_id}-{suffix}");
            let _: Result<(), JsValue> = part.set_attribute("id", &id);
            let _: Result<(), JsValue> = panel.set_attribute(attribute, &id);
        } else if attribute == SHEET_LABEL_ATTRIBUTE {
            let _: Result<(), JsValue> =
                panel.set_attribute(attribute, &sheet_part_id(open, SHEET_TRIGGER_PART));
        } else {
            let _: Result<(), JsValue> = panel.remove_attribute(attribute);
        }
    }
    if open.get() {
        let _: Result<(), JsValue> = panel.remove_attribute(SHEET_INERT_ATTRIBUTE);
        if focus {
            let target: Option<HtmlElement> = sheet_focusables(&panel)
                .into_iter()
                .next()
                .or_else(|| panel.dyn_into::<HtmlElement>().ok());
            if let Some(target) = target {
                let _: Result<(), JsValue> = target.focus();
            }
        }
    } else {
        let _: Result<(), JsValue> = panel.set_attribute(SHEET_INERT_ATTRIBUTE, "");
    }
}

/// Restricts restoration to properties actually owned by this document lease.
///
/// # Arguments
/// - `&str` - A live inline property name.
/// - `bool` - Whether this acquisition reserves a classic scrollbar gutter.
///
/// # Returns
/// - `bool` - Whether this lease may snapshot, mutate and restore the property.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn sheet_scroll_owns_property(name: &str, gutter: bool) -> bool {
    SHEET_OVERFLOW_PROPERTIES.contains(&name) || (gutter && name == SHEET_GUTTER_PROPERTY)
}

/// Snapshots only declarations this lock may mutate, preserving declaration order.
///
/// # Arguments
/// - `&CssStyleDeclaration` - The element's live inline declaration.
/// - `bool` - Whether this element also owns a temporary stable gutter.
///
/// # Returns
/// - `SheetScrollSnapshot` - The prior values and priorities, never the whole style attribute.
#[cfg(target_arch = "wasm32")]
fn sheet_scroll_snapshot(style: &CssStyleDeclaration, gutter: bool) -> SheetScrollSnapshot {
    (0..style.length())
        .map(|index: u32| style.item(index))
        .filter(|name: &String| sheet_scroll_owns_property(name, gutter))
        .map(|name: String| {
            let value: String = style.get_property_value(&name).unwrap_or_default();
            let priority: String = style.get_property_priority(&name);
            (name, value, priority)
        })
        .collect()
}

/// Restores owned properties without overwriting unrelated runtime inline changes.
///
/// # Arguments
/// - `&CssStyleDeclaration` - The original html/body element's live declaration.
/// - `SheetScrollSnapshot` - The ordered original declarations.
/// - `bool` - Whether the gutter was temporarily changed.
#[cfg(target_arch = "wasm32")]
fn sheet_restore_scroll(style: &CssStyleDeclaration, snapshot: SheetScrollSnapshot, gutter: bool) {
    for property in SHEET_OVERFLOW_PROPERTIES {
        let _: Result<String, JsValue> = style.remove_property(property);
    }
    if gutter {
        let _: Result<String, JsValue> = style.remove_property(SHEET_GUTTER_PROPERTY);
    }
    for (property, value, priority) in snapshot {
        let _: Result<(), JsValue> = style.set_property_with_priority(&property, &value, &priority);
    }
}

/// Acquires one shared lease for this exact document, never another document's lock.
///
/// # Arguments
/// - `&Document` - The mounted sheet root's owner document.
///
/// # Returns
/// - `Option<Rc<SheetScrollLease>>` - A refcounted restore-on-final-release lease.
#[cfg(target_arch = "wasm32")]
fn sheet_document_scroll_lease(document: &Document) -> Option<Rc<SheetScrollLease>> {
    let existing: Option<Rc<SheetScrollLease>> =
        SHEET_SCROLL_LOCKS.with(|locks: &SheetScrollRegistry| {
            locks
                .borrow()
                .iter()
                .find_map(|(owner, lease): &(Document, Weak<SheetScrollLease>)| {
                    (owner == document).then(|| lease.upgrade()).flatten()
                })
        });
    if existing.is_some() {
        return existing;
    }
    let root: HtmlElement = document.document_element()?.dyn_into().ok()?;
    let body: HtmlElement = document.body()?;
    let root_style: CssStyleDeclaration = root.style();
    let body_style: CssStyleDeclaration = body.style();
    // Only classic, currently present scrollbars consume layout space. The app's
    // already-hidden document and overlay-scrollbar hosts must remain unchanged.
    let gutter: bool = document
        .default_view()
        .and_then(|window: Window| window.inner_width().ok())
        .and_then(|width: JsValue| width.as_f64())
        .is_some_and(|width: f64| width > f64::from(root.client_width()));
    let root_snapshot: SheetScrollSnapshot = sheet_scroll_snapshot(&root_style, gutter);
    let body_snapshot: SheetScrollSnapshot = sheet_scroll_snapshot(&body_style, false);
    if gutter {
        let symmetric: bool = document
            .default_view()
            .and_then(|window: Window| window.get_computed_style(&root).ok().flatten())
            .and_then(|style: CssStyleDeclaration| {
                style.get_property_value(SHEET_GUTTER_PROPERTY).ok()
            })
            .is_some_and(|value: String| value == SHEET_GUTTER_BOTH);
        let value: &str = if symmetric {
            SHEET_GUTTER_BOTH
        } else {
            SHEET_GUTTER_STABLE
        };
        let _: Result<(), JsValue> = root_style.set_property_with_priority(
            SHEET_GUTTER_PROPERTY,
            value,
            SHEET_SCROLL_PRIORITY,
        );
    }
    for style in [&root_style, &body_style] {
        for property in SHEET_OVERFLOW_PROPERTIES {
            let _: Result<(), JsValue> = style.set_property_with_priority(
                property,
                SHEET_SCROLL_HIDDEN,
                SHEET_SCROLL_PRIORITY,
            );
        }
    }
    let owner: Document = document.clone();
    let lease: Rc<SheetScrollLease> =
        Rc::new(SheetScrollLease {
            cleanup: Some(Box::new(move || {
                SHEET_SCROLL_LOCKS.with(|locks: &SheetScrollRegistry| {
                    locks.borrow_mut().retain(
                        |(document, _): &(Document, Weak<SheetScrollLease>)| document != &owner,
                    );
                });
                sheet_restore_scroll(&root_style, root_snapshot, gutter);
                sheet_restore_scroll(&body_style, body_snapshot, false);
            })),
        });
    SHEET_SCROLL_LOCKS.with(|locks: &SheetScrollRegistry| {
        locks
            .borrow_mut()
            .push((document.clone(), Rc::downgrade(&lease)));
    });
    Some(lease)
}

/// Makes each mounted, open root own at most one document lease.
///
/// # Arguments
/// - `&SheetScrollSlot` - Lifecycle-owned lease slot.
/// - `&NodeRef<Element>` - The live composition root, including detachment.
/// - `bool` - The current caller-owned open state.
#[cfg(target_arch = "wasm32")]
fn sheet_sync_scroll(slot: &SheetScrollSlot, target: &NodeRef<Element>, open: bool) {
    let document: Option<Document> = target
        .get_cloned()
        .filter(|root: &Element| open && root.is_connected())
        .and_then(|root: Element| root.owner_document());
    let same: bool = slot
        .borrow()
        .as_ref()
        .is_some_and(|(owner, _): &SheetScrollOwner| Some(owner) == document.as_ref());
    if same {
        return;
    }
    slot.borrow_mut().take();
    if let Some(document) = document
        && let Some(lease) = sheet_document_scroll_lease(&document)
    {
        *slot.borrow_mut() = Some((document, lease));
    }
}

/// Owns capture keyboard handling, the open subscription and a cancellable focus frame.
///
/// Capture is intentional: delegated renderer events stop at the first handler ancestor,
/// so a descendant input's own key handler must not bypass modal keyboard behavior.
///
/// # Arguments
/// - `Signal<bool>` - The shared sheet state.
/// - `NodeRef<Element>` - The lifecycle-managed composition reference.
#[cfg(target_arch = "wasm32")]
fn use_sheet_modal(open: Signal<bool>, target: NodeRef<Element>) {
    let teardown: Rc<RefCell<Option<Box<dyn FnOnce()>>>> = HookContext::use_hook(move || {
        let Some(window) = window() else {
            return Rc::new(RefCell::new(None));
        };
        let Some(document) = window.document() else {
            return Rc::new(RefCell::new(None));
        };
        let scroll: Rc<SheetScrollSlot> = Rc::new(RefCell::new(None));
        let frame_scroll: Weak<SheetScrollSlot> = Rc::downgrade(&scroll);
        let next_frame: Rc<SheetNextFrame> = Rc::new(RefCell::new(None));
        let frame_next: Rc<SheetNextFrame> = next_frame.clone();
        let frame: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
        let frame_id: Rc<Cell<Option<i32>>> = frame.clone();
        let frame_target: NodeRef<Element> = target.clone();
        let sync_pending: Rc<Cell<bool>> = Rc::new(Cell::new(true));
        let frame_sync_pending: Rc<Cell<bool>> = sync_pending.clone();
        let opening: Rc<Cell<bool>> = Rc::new(Cell::new(open.get()));
        let frame_opening: Rc<Cell<bool>> = opening.clone();
        let callback: Rc<Closure<dyn FnMut(f64)>> =
            Rc::new(Closure::wrap(Box::new(move |_: f64| {
                frame_id.set(None);
                if let Some(scroll) = frame_scroll.upgrade() {
                    sheet_sync_scroll(&scroll, &frame_target, open.get());
                }
                if open.get()
                    && let Some(schedule) = frame_next.borrow().as_ref().and_then(Weak::upgrade)
                {
                    schedule();
                }
                if let Some(root) = frame_target
                    .get_cloned()
                    .filter(|root: &Element| root.is_connected())
                    && frame_sync_pending.replace(false)
                {
                    sheet_sync(&root, open, frame_opening.replace(false));
                }
            }) as Box<dyn FnMut(f64)>));
        let schedule_frame: Rc<Cell<Option<i32>>> = frame.clone();
        let schedule_window: Window = window.clone();
        let schedule: Rc<dyn Fn()> = Rc::new(move || {
            if schedule_frame.get().is_none()
                && let Ok(id) = schedule_window
                    .request_animation_frame(callback.as_ref().as_ref().unchecked_ref())
            {
                schedule_frame.set(Some(id));
            }
        });
        *next_frame.borrow_mut() = Some(Rc::downgrade(&schedule));
        let subscription_scroll: Weak<SheetScrollSlot> = Rc::downgrade(&scroll);
        let subscription_target: NodeRef<Element> = target.clone();
        let subscription_schedule: Rc<dyn Fn()> = schedule.clone();
        let subscription: usize = open.subscribe(move || {
            opening.set(open.get());
            sync_pending.set(true);
            if let Some(scroll) = subscription_scroll.upgrade() {
                sheet_sync_scroll(&scroll, &subscription_target, open.get());
            }
            if let Some(root) = subscription_target
                .get_cloned()
                .filter(|root: &Element| root.is_connected())
            {
                sheet_sync(&root, open, false);
            }
            subscription_schedule();
        });
        let listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
            if !open.get() || event.default_prevented() {
                return;
            }
            let Some(root) = target
                .get_cloned()
                .filter(|root: &Element| root.is_connected())
            else {
                return;
            };
            let nearest: Option<Element> = event
                .target()
                .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
                .and_then(|target: Element| {
                    target.closest(SHEET_OPEN_ROOT_SELECTOR).ok().flatten()
                });
            if !sheet_handles_event(
                open.get(),
                event.default_prevented(),
                nearest.as_ref() == Some(&root),
            ) {
                return;
            }
            if event.type_() == SHEET_CLICK_EVENT {
                let dismiss: bool = event
                    .target()
                    .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
                    .and_then(|target: Element| {
                        target.closest(SHEET_DISMISS_SELECTOR).ok().flatten()
                    })
                    .is_some();
                if dismiss {
                    event.prevent_default();
                    open.set(false);
                    sheet_focus_trigger(&root);
                }
                return;
            }
            let Ok(keyboard) = event.dyn_into::<KeyboardEvent>() else {
                return;
            };
            let key: String = keyboard.key();
            if key == SHEET_ESCAPE_KEY {
                keyboard.prevent_default();
                keyboard.stop_propagation();
                open.set(false);
                sheet_focus_trigger(&root);
            } else if key == SHEET_TAB_KEY {
                let Some(panel) = sheet_part(&root, &root, SHEET_CONTENT_SELECTOR) else {
                    return;
                };
                let items: Vec<HtmlElement> = sheet_focusables(&panel);
                let active: Option<Element> = panel
                    .owner_document()
                    .and_then(|document: Document| document.active_element());
                if !active
                    .as_ref()
                    .is_some_and(|active: &Element| panel.contains(Some(active)))
                {
                    return;
                }
                let index: Option<usize> = items
                    .iter()
                    .position(|item: &HtmlElement| active.as_ref() == Some(item.as_ref()));
                let wrap: bool = index.is_none()
                    || if keyboard.shift_key() {
                        index == Some(0)
                    } else {
                        index == items.len().checked_sub(1)
                    };
                if wrap {
                    keyboard.prevent_default();
                    let destination: Option<HtmlElement> = if keyboard.shift_key() {
                        items.last().cloned()
                    } else {
                        items.first().cloned()
                    }
                    .or_else(|| panel.dyn_into::<HtmlElement>().ok());
                    if let Some(destination) = destination {
                        let _: Result<(), JsValue> = destination.focus();
                    }
                }
            }
        })
            as Box<dyn FnMut(Event)>);
        for event in [SHEET_KEYDOWN_EVENT, SHEET_CLICK_EVENT] {
            let _: Result<(), JsValue> = document.add_event_listener_with_callback_and_bool(
                event,
                listener.as_ref().unchecked_ref(),
                true,
            );
        }
        schedule();
        let cleanup: Box<dyn FnOnce()> = Box::new(move || {
            open.unsubscribe(subscription);
            scroll.borrow_mut().take();
            if let Some(id) = frame.take() {
                let _: Result<(), JsValue> = window.cancel_animation_frame(id);
            }
            for event in [SHEET_KEYDOWN_EVENT, SHEET_CLICK_EVENT] {
                let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
                    event,
                    listener.as_ref().unchecked_ref(),
                    true,
                );
            }
        });
        Rc::new(RefCell::new(Some(cleanup)))
    });
    App::use_cleanup(move || {
        if let Some(cleanup) = teardown.borrow_mut().take() {
            cleanup();
        }
    });
}
