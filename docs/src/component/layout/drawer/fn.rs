use super::*;

/// Tears down a responsive child scope before its old DOM is removed.
///
/// # Arguments
///
/// - `HookContext` - The child scope that owns native dropdown listeners.
/// - `Signal<bool>` - The responsive mode signal.
/// - `bool` - The mode that owns this scope.
pub(crate) fn use_docs_responsive_scope(context: HookContext, mobile: Signal<bool>, owner: bool) {
    let subscription: Rc<Cell<Option<usize>>> = HookContext::use_hook(move || {
        let handle: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));
        let callback_handle: Rc<Cell<Option<usize>>> = handle.clone();
        let id: usize = mobile.subscribe(move || {
            if mobile.get() != owner {
                if let Some(id) = callback_handle.take() {
                    mobile.unsubscribe(id);
                }
                let mut scope: HookContext = context.clone();
                scope.switch_arm(1);
            }
        });
        handle.set(Some(id));
        handle
    });
    App::use_cleanup(move || {
        if let Some(id) = subscription.take() {
            mobile.unsubscribe(id);
        }
    });
}

/// Installs one modal lifetime independently of reactive shell renders.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer, nested locale menu and mobile states.
pub(crate) fn use_docs_drawer_modal(
    drawer_open: Signal<bool>,
    locale_menu_open: Signal<bool>,
    mobile: Signal<bool>,
) {
    let teardown: Rc<dyn Fn()> = HookContext::use_hook(move || {
        bind_docs_drawer_modal(drawer_open, locale_menu_open, mobile)
    });
    App::use_cleanup(move || teardown());
}

/// Registers native listeners and cancellable post-render focus effects.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer, nested locale menu and mobile states.
///
/// # Returns
///
/// - `Rc<dyn Fn()>` - The subscription and listener teardown.
fn bind_docs_drawer_modal(
    drawer_open: Signal<bool>,
    locale_menu_open: Signal<bool>,
    mobile: Signal<bool>,
) -> Rc<dyn Fn()> {
    let Some(window): Option<Window> = window() else {
        return Rc::new(|| {});
    };
    let Some(document): Option<Document> = window.document() else {
        return Rc::new(|| {});
    };
    // DOM bookkeeping must not subscribe the app's render context to effects.
    let state: DocsDrawerModalState = Rc::new((
        Cell::new(false),
        RefCell::new(None),
        Cell::new(None),
        Cell::new(None),
    ));
    let focus_state: DocsDrawerModalState = state.clone();
    let finish: Rc<Closure<dyn FnMut(f64)>> = Rc::new(Closure::wrap(Box::new(move |_: f64| {
        focus_state.3.set(None);
        sync_docs_drawer_focus(drawer_open, mobile, &focus_state);
    }) as Box<dyn FnMut(f64)>));
    let frame_window: Window = window.clone();
    let frame_state: DocsDrawerModalState = state.clone();
    let after_render: Rc<Closure<dyn FnMut(f64)>> =
        Rc::new(Closure::wrap(Box::new(move |_: f64| {
            frame_state.2.set(None);
            if let Ok(id) =
                frame_window.request_animation_frame(finish.as_ref().as_ref().unchecked_ref())
            {
                frame_state.3.set(Some(id));
            }
        }) as Box<dyn FnMut(f64)>));
    let schedule_window: Window = window.clone();
    let schedule_state: DocsDrawerModalState = state.clone();
    let schedule: Rc<dyn Fn()> = Rc::new(move || {
        cancel_docs_drawer_frames(&schedule_window, &schedule_state);
        if let Ok(id) =
            schedule_window.request_animation_frame(after_render.as_ref().as_ref().unchecked_ref())
        {
            schedule_state.2.set(Some(id));
        }
    });
    let change_document: Document = document.clone();
    let change_schedule: Rc<dyn Fn()> = schedule.clone();
    let change_state: DocsDrawerModalState = state.clone();
    let drawer_subscription: usize = drawer_open.subscribe(move || {
        if drawer_open.get() && mobile.get() {
            *change_state.1.borrow_mut() = change_document
                .get_element_by_id(DOCS_DRAWER_TRIGGER_ID)
                .and_then(|element: Element| element.dyn_into::<HtmlElement>().ok());
            change_state.0.set(false);
        } else {
            locale_menu_open.set(false);
        }
        change_schedule();
    });
    let resize_schedule: Rc<dyn Fn()> = schedule.clone();
    let mobile_subscription: usize = mobile.subscribe(move || {
        if !mobile.get() && drawer_open.get() {
            dismiss_docs_drawer(drawer_open);
        }
        resize_schedule();
    });
    let key_listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        if drawer_open.get() && mobile.get() && !event.default_prevented() {
            docs_drawer_keydown(event, drawer_open, locale_menu_open);
        }
    }) as Box<dyn FnMut(Event)>);
    let focus_listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        if !drawer_open.get() || !mobile.get() {
            return;
        }
        let Some(drawer): Option<HtmlElement> = docs_drawer_element(DOCS_DRAWER_ID) else {
            return;
        };
        let target: Option<Node> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Node>().ok());
        if docs_drawer_element_available(&drawer) && !drawer.contains(target.as_ref()) {
            focus_docs_drawer_close();
        }
    }) as Box<dyn FnMut(Event)>);
    let _: Result<(), JsValue> = document.add_event_listener_with_callback(
        DOCS_DRAWER_KEY_EVENT,
        key_listener.as_ref().unchecked_ref(),
    );
    let _: Result<(), JsValue> = document.add_event_listener_with_callback_and_bool(
        DOCS_DRAWER_FOCUS_EVENT,
        focus_listener.as_ref().unchecked_ref(),
        true,
    );
    schedule();
    Rc::new(move || {
        drawer_open.unsubscribe(drawer_subscription);
        mobile.unsubscribe(mobile_subscription);
        cancel_docs_drawer_frames(&window, &state);
        let _: Result<(), JsValue> = document.remove_event_listener_with_callback(
            DOCS_DRAWER_KEY_EVENT,
            key_listener.as_ref().unchecked_ref(),
        );
        let _: Result<(), JsValue> = document.remove_event_listener_with_callback_and_bool(
            DOCS_DRAWER_FOCUS_EVENT,
            focus_listener.as_ref().unchecked_ref(),
            true,
        );
        let _: &Rc<dyn Fn()> = &schedule;
    })
}

/// Cancels both owned animation frames before releasing their callbacks.
///
/// # Arguments
///
/// - `&Window` - The owning window.
/// - `&DocsDrawerModalState` - Nonreactive modal bookkeeping.
fn cancel_docs_drawer_frames(window: &Window, state: &DocsDrawerModalState) {
    for frame in [&state.2, &state.3] {
        if let Some(id) = frame.take() {
            let _: Result<(), JsValue> = window.cancel_animation_frame(id);
        }
    }
}

/// Applies inert boundaries after patching and transfers or restores focus.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer and mobile states.
/// - `&DocsDrawerModalState` - Nonreactive focus and frame bookkeeping.
fn sync_docs_drawer_focus(
    drawer_open: Signal<bool>,
    mobile: Signal<bool>,
    state: &DocsDrawerModalState,
) {
    let open: bool = drawer_open.get() && mobile.get();
    if let Some(document) = window().and_then(|window: Window| window.document())
        && let Ok(background) = document.query_selector_all(DOCS_DRAWER_BACKGROUND_SELECTOR)
    {
        for index in 0..background.length() {
            if let Some(element) = background
                .item(index)
                .and_then(|node: Node| node.dyn_into::<Element>().ok())
            {
                let _: Result<bool, JsValue> =
                    element.toggle_attribute_with_force(DOCS_DRAWER_INERT_ATTRIBUTE, open);
            }
        }
    }
    if let Some(drawer) = docs_drawer_element(DOCS_DRAWER_ID) {
        let _: Result<bool, JsValue> =
            drawer.toggle_attribute_with_force(DOCS_DRAWER_INERT_ATTRIBUTE, !open);
        let _: Result<(), JsValue> = drawer.set_attribute(
            DOCS_DRAWER_HIDDEN_ATTRIBUTE,
            if open {
                DOCS_DRAWER_ARIA_FALSE
            } else {
                DOCS_DRAWER_ARIA_TRUE
            },
        );
    }
    if open {
        if !state.0.get()
            && let Some(close) = docs_drawer_element(DOCS_DRAWER_CLOSE_ID)
            && docs_drawer_element_available(&close)
        {
            state.0.set(true);
            let _: Result<(), JsValue> = close.focus();
        }
        return;
    }
    let previous: Option<HtmlElement> = state.1.borrow_mut().take();
    state.0.set(false);
    if mobile.get()
        && let Some(previous) = previous
        && docs_drawer_element(DOCS_DRAWER_TRIGGER_ID).as_ref() == Some(&previous)
        && docs_drawer_element_available(&previous)
    {
        let _: Result<(), JsValue> = previous.focus();
    }
}

/// Contains sequential focus and dismisses the innermost open surface.
///
/// # Arguments
///
/// - `Event` - The uncancelled keyboard event after descendant dispatch.
/// - `Signal<bool>` - The drawer and nested locale-menu states.
fn docs_drawer_keydown(event: Event, drawer_open: Signal<bool>, locale_menu_open: Signal<bool>) {
    let Ok(key): Result<KeyboardEvent, Event> = event.dyn_into::<KeyboardEvent>() else {
        return;
    };
    if key.is_composing() || key.alt_key() || key.ctrl_key() || key.meta_key() {
        return;
    }
    if key.key() == DOCS_DRAWER_KEY_ESCAPE {
        key.prevent_default();
        key.stop_propagation();
        if locale_menu_open.get() {
            locale_menu_open.set(false);
            if let Some(trigger) = docs_drawer_element(&format!(
                "euv-dropdown-{}-trigger",
                locale_menu_open.get_inner()
            )) {
                let _: Result<(), JsValue> = trigger.focus();
            }
        } else {
            dismiss_docs_drawer(drawer_open);
        }
        return;
    }
    if key.key() != DOCS_DRAWER_KEY_TAB {
        return;
    }
    let Some(drawer): Option<HtmlElement> = docs_drawer_element(DOCS_DRAWER_ID) else {
        return;
    };
    let controls: Vec<HtmlElement> = docs_drawer_controls(&drawer);
    let active: Option<Element> = drawer
        .owner_document()
        .and_then(|doc: Document| doc.active_element());
    let index: Option<usize> = controls.iter().position(|control: &HtmlElement| {
        let element: &Element = control.as_ref();
        active.as_ref() == Some(element)
    });
    let boundary: bool = index.is_none()
        || (key.shift_key() && index == Some(0))
        || (!key.shift_key() && index == controls.len().checked_sub(1));
    if boundary {
        key.prevent_default();
        let target: Option<&HtmlElement> = if key.shift_key() {
            controls.last()
        } else {
            controls.first()
        };
        let _: Result<(), JsValue> = target.unwrap_or(&drawer).focus();
    }
}

/// Lists available tab stops, including the locale menu portaled into the sheet.
///
/// # Arguments
///
/// - `&HtmlElement` - The modal drawer containing its owned portals.
///
/// # Returns
///
/// - `Vec<HtmlElement>` - The drawer's available controls in DOM order.
fn docs_drawer_controls(drawer: &HtmlElement) -> Vec<HtmlElement> {
    let Ok(nodes): Result<NodeList, JsValue> =
        drawer.query_selector_all(DOCS_DRAWER_FOCUSABLE_SELECTOR)
    else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|index: u32| nodes.item(index))
        .filter_map(|node: Node| node.dyn_into::<HtmlElement>().ok())
        .filter(|element: &HtmlElement| {
            element.tab_index() >= 0 && docs_drawer_element_available(element)
        })
        .collect()
}

/// Resolves a current DOM element instead of retaining removed shell nodes.
///
/// # Arguments
///
/// - `&str` - The stable control or drawer id.
///
/// # Returns
///
/// - `Option<HtmlElement>` - The current live HTML element.
fn docs_drawer_element(id: &str) -> Option<HtmlElement> {
    window()?
        .document()?
        .get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .ok()
}

/// Rejects disconnected, hidden, inert, disabled and visibility-hidden controls.
///
/// # Arguments
///
/// - `&HtmlElement` - The candidate control.
///
/// # Returns
///
/// - `bool` - Whether the control is available inside the current focus boundary.
fn docs_drawer_element_available(element: &HtmlElement) -> bool {
    element.is_connected()
        && element
            .closest(DOCS_DRAWER_UNAVAILABLE_SELECTOR)
            .ok()
            .flatten()
            .is_none()
        && element.get_bounding_client_rect().height() > 0.0
        && window()
            .and_then(|window: Window| window.get_computed_style(element).ok().flatten())
            .and_then(|style: CssStyleDeclaration| {
                style
                    .get_property_value(DOCS_DRAWER_VISIBILITY_PROPERTY)
                    .ok()
            })
            .is_none_or(|visibility: String| {
                !DOCS_DRAWER_HIDDEN_VISIBILITIES.contains(&visibility.as_str())
            })
}

/// Focuses the close control when focus attempts to leave the modal boundary.
fn focus_docs_drawer_close() {
    if let Some(close) = docs_drawer_element(DOCS_DRAWER_CLOSE_ID)
        && docs_drawer_element_available(&close)
    {
        let _: Result<(), JsValue> = close.focus();
    }
}

/// Dismisses the drawer through its existing overlay-history path.
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open state.
pub(crate) fn dismiss_docs_drawer(drawer_open: Signal<bool>) {
    if drawer_open.get() {
        Router::overlay_stack_close();
        drawer_open.set(false);
    }
}
