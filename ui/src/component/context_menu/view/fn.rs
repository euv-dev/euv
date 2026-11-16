use super::*;

/// Opens both pointer and touch gestures through the same position/focus path.
///
/// # Arguments
///
/// - `&web_sys::Element` - Shared reference to a `web_sys::Element`.
/// - `Signal<bool>` - A `Signal<bool>` parameter.
/// - `i32` - A 32-bit signed integer (`i32`).
/// - `i32` - A 32-bit signed integer (`i32`).
fn open_context_menu_at(trigger: &web_sys::Element, open: Signal<bool>, x: i32, y: i32) {
    set_context_menu_position(trigger, f64::from(x), f64::from(y));
    open.set(true);
    focus_context_menu_content_on_next_frame(trigger);
}

/// Starts a cancellable browser hold without preventing native scrolling.
///
/// # Arguments
///
/// - `&Event` - Shared reference to a `Event`.
/// - `&Rc<RefCell<ContextMenuLongPress>>` - Shared reference to a `Rc<RefCell<ContextMenuLongPress>>`.
/// - `Signal<bool>` - A `Signal<bool>` parameter.
/// - `bool` - A boolean (`bool`).
fn context_menu_touch_start(
    event: &Event,
    state: &Rc<RefCell<ContextMenuLongPress>>,
    open: Signal<bool>,
    disabled: bool,
) {
    let Some(touch_event) = event.dyn_ref::<web_sys::TouchEvent>() else {
        return;
    };
    let touches: web_sys::TouchList = touch_event.touches();
    let touch: Option<web_sys::Touch> = touches.item(0);
    state.borrow_mut().cancel();
    let Some(touch) = touch else {
        return;
    };
    state.borrow_mut().start(
        disabled,
        touches.length(),
        touch.client_x(),
        touch.client_y(),
        event.time_stamp(),
    );
    if state.borrow().origin.is_none() {
        return;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let Some(trigger) = context_menu_trigger_element(event) else {
            state.borrow_mut().cancel();
            return;
        };
        let Some(window) = web_sys::window() else {
            state.borrow_mut().cancel();
            return;
        };
        let weak: Weak<RefCell<ContextMenuLongPress>> = Rc::downgrade(state);
        let started: f64 = event.time_stamp();
        let callback: Closure<dyn FnMut()> = Closure::new(move || {
            let Some(state) = weak.upgrade() else {
                return;
            };
            let position: Option<(i32, i32)> = state
                .borrow_mut()
                .finish(started + f64::from(CONTEXT_MENU_LONG_PRESS_MS));
            if let Some((x, y)) = position
                && trigger.is_connected()
            {
                open_context_menu_at(&trigger, open, x, y);
            }
        });
        let timer: Result<i32, wasm_bindgen::JsValue> = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                CONTEXT_MENU_LONG_PRESS_MS,
            );
        if let Ok(timer) = timer {
            let mut pending: RefMut<'_, ContextMenuLongPress> = state.borrow_mut();
            pending.timer = Some(timer);
            pending.callback = Some(callback);
        } else {
            state.borrow_mut().cancel();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _: Signal<bool> = open;
}

/// Chooses a row for wraparound navigation or case-insensitive text search.
///
/// # Arguments
///
/// - `&[String]` - Enabled row labels in menu order.
/// - `Option<usize>` - The currently focused row index.
/// - `&str` - The navigation key or typeahead prefix.
///
/// # Returns
///
/// - `Option<usize>` - The row to focus, if any.
pub(crate) fn context_menu_next_index(
    labels: &[String],
    current: Option<usize>,
    key: &str,
) -> Option<usize> {
    let count: usize = labels.len();
    if count == 0 {
        return None;
    }
    match key {
        CONTEXT_MENU_DOWN_KEY => Some(current.map_or(0, |index: usize| (index + 1) % count)),
        CONTEXT_MENU_UP_KEY => {
            Some(current.map_or(count - 1, |index: usize| (index + count - 1) % count))
        }
        CONTEXT_MENU_HOME_KEY => Some(0),
        CONTEXT_MENU_END_KEY => Some(count - 1),
        _ => {
            let start: usize = current.map_or(0, |index: usize| (index + 1) % count);
            (0..count)
                .map(|offset: usize| (start + offset) % count)
                .find(|index: &usize| labels[*index].trim().to_lowercase().starts_with(key))
        }
    }
}

/// Resolves inherited CSS direction at the popup, including local direction scopes.
///
/// # Arguments
///
/// - `&web_sys::Element` - The popup whose inherited direction is used.
///
/// # Returns
///
/// - `bool` - Whether the popup's computed direction is right-to-left.
fn context_menu_is_rtl(popup: &web_sys::Element) -> bool {
    web_sys::window()
        .and_then(|window: web_sys::Window| window.get_computed_style(popup).ok().flatten())
        .and_then(|style: web_sys::CssStyleDeclaration| {
            style
                .get_property_value(CONTEXT_MENU_DIRECTION_PROPERTY)
                .ok()
        })
        .is_some_and(|direction: String| direction == CONTEXT_MENU_RTL)
}

/// Selects logical submenu entry and exit keys independently of collision flips.
///
/// # Arguments
///
/// - `bool` - Whether the popup direction is right-to-left.
///
/// # Returns
///
/// - `(&'static str, &'static str)` - The forward and backward keys.
pub(crate) fn context_menu_direction_keys(rtl: bool) -> (&'static str, &'static str) {
    if rtl {
        (CONTEXT_MENU_LEFT_KEY, CONTEXT_MENU_RIGHT_KEY)
    } else {
        (CONTEXT_MENU_RIGHT_KEY, CONTEXT_MENU_LEFT_KEY)
    }
}

/// Places a submenu on inline-end, flipping and shifting into the viewport.
///
/// # Arguments
///
/// - `(f64, f64, f64)` - The trigger's left, right and top viewport coordinates.
/// - `(f64, f64)` - The panel's measured border-box width and height.
/// - `(f64, f64)` - The viewport width and height.
/// - `bool` - Whether the inherited popup direction is right-to-left.
///
/// # Returns
///
/// - `(f64, f64)` - The collision-adjusted left and top viewport coordinates.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn context_menu_sub_position(
    anchor: (f64, f64, f64),
    panel: (f64, f64),
    viewport: (f64, f64),
    rtl: bool,
) -> (f64, f64) {
    let left_space: f64 = anchor.0.max(0.0);
    let right_space: f64 = (viewport.0 - anchor.1).max(0.0);
    let use_left: bool = if rtl {
        panel.0 <= left_space || left_space >= right_space
    } else {
        panel.0 > right_space && left_space > right_space
    };
    let left: f64 = if use_left {
        anchor.0 - panel.0
    } else {
        anchor.1
    };
    (
        left.clamp(0.0, (viewport.0 - panel.0).max(0.0)),
        anchor.2.clamp(0.0, (viewport.1 - panel.1).max(0.0)),
    )
}

/// Preserves the root popup's viewport position without a transformed containing block.
///
/// # Arguments
///
/// - `&web_sys::Element` - A descendant submenu whose fixed position must use the viewport.
/// - `(f64, f64)` - The viewport width and height.
#[cfg(target_arch = "wasm32")]
fn freeze_context_menu_root_position(panel: &web_sys::Element, viewport: (f64, f64)) {
    let Some(root) = panel.closest(CONTEXT_MENU_CONTENT_SELECTOR).ok().flatten() else {
        return;
    };
    let Some(element) = root.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    let rect: web_sys::DomRect = root.get_bounding_client_rect();
    let left: f64 = rect.left().clamp(0.0, (viewport.0 - rect.width()).max(0.0));
    let top: f64 = rect.top().clamp(0.0, (viewport.1 - rect.height()).max(0.0));
    let style: web_sys::CssStyleDeclaration = element.style();
    let _: Result<(), wasm_bindgen::JsValue> =
        style.set_property(CONTEXT_MENU_TRANSFORM_PROPERTY, CONTEXT_MENU_NO_TRANSFORM);
    let _: Result<(), wasm_bindgen::JsValue> =
        style.set_property(CONTEXT_MENU_LEFT_PROPERTY, &format!("{left}px"));
    let _: Result<(), wasm_bindgen::JsValue> =
        style.set_property(CONTEXT_MENU_TOP_PROPERTY, &format!("{top}px"));
}

/// Measures the mounted panel after rendering, like the Select positioning path.
///
/// # Arguments
///
/// - `NodeRef<web_sys::Element>` - The mounted submenu panel reference.
/// - `Signal<bool>` - The submenu open state, checked again in the scheduled frame.
#[cfg(target_arch = "wasm32")]
fn schedule_context_menu_sub_position(target: NodeRef<web_sys::Element>, open: Signal<bool>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let callback: js_sys::Function = Closure::once_into_js(move || {
        if !open.get() {
            return;
        }
        let Some(panel) = target.get_cloned() else {
            return;
        };
        if !panel.is_connected() {
            return;
        }
        let Some(owner) = panel.closest(CONTEXT_MENU_SUB_SELECTOR).ok().flatten() else {
            return;
        };
        let Some(trigger) = owner
            .query_selector(CONTEXT_MENU_SUB_TRIGGER_SELECTOR)
            .ok()
            .flatten()
        else {
            return;
        };
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(element) = panel.dyn_ref::<web_sys::HtmlElement>() else {
            return;
        };
        let style: web_sys::CssStyleDeclaration = element.style();
        for (property, value) in CONTEXT_MENU_SUB_POSITION_STYLES {
            let _: Result<(), wasm_bindgen::JsValue> = style.set_property(property, value);
        }
        let width: f64 = window
            .inner_width()
            .ok()
            .and_then(|value: wasm_bindgen::JsValue| value.as_f64())
            .unwrap_or_default();
        let height: f64 = window
            .inner_height()
            .ok()
            .and_then(|value: wasm_bindgen::JsValue| value.as_f64())
            .unwrap_or_default();
        freeze_context_menu_root_position(&panel, (width, height));
        let anchor: web_sys::DomRect = trigger.get_bounding_client_rect();
        let rect: web_sys::DomRect = panel.get_bounding_client_rect();
        let (left, top): (f64, f64) = context_menu_sub_position(
            (anchor.left(), anchor.right(), anchor.top()),
            (rect.width(), rect.height()),
            (width, height),
            context_menu_is_rtl(&panel),
        );
        let _: Result<(), wasm_bindgen::JsValue> =
            style.set_property(CONTEXT_MENU_LEFT_PROPERTY, &format!("{left}px"));
        let _: Result<(), wasm_bindgen::JsValue> =
            style.set_property(CONTEXT_MENU_TOP_PROPERTY, &format!("{top}px"));
    })
    .unchecked_into();
    let _: Result<i32, wasm_bindgen::JsValue> = window.request_animation_frame(&callback);
}

/// Clears measured coordinates when a submenu closes so reopening measures anew.
///
/// # Arguments
///
/// - `&NodeRef<web_sys::Element>` - Mounted submenu panel reference.
#[cfg(target_arch = "wasm32")]
fn reset_context_menu_sub_position(target: &NodeRef<web_sys::Element>) {
    if let Some(panel) = target.get_cloned()
        && let Some(element) = panel.dyn_ref::<web_sys::HtmlElement>()
    {
        let style: web_sys::CssStyleDeclaration = element.style();
        for (property, _) in CONTEXT_MENU_SUB_POSITION_STYLES {
            let _: Result<String, wasm_bindgen::JsValue> = style.remove_property(property);
        }
        for property in [CONTEXT_MENU_LEFT_PROPERTY, CONTEXT_MENU_TOP_PROPERTY] {
            let _: Result<String, wasm_bindgen::JsValue> = style.remove_property(property);
        }
    }
}

/// Handles keyboard events at the nearest popup, not at individual rows.
///
/// # Arguments
///
/// - `&Event` - The keyboard event from a menu descendant.
/// - `Signal<bool>` - The popup's controlled open state.
/// - `bool` - Whether this popup is a submenu.
fn context_menu_keydown(event: &Event, open: Signal<bool>, submenu: bool) {
    if !open.get() {
        return;
    }
    let Some(keyboard) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
        return;
    };
    if keyboard.is_composing() || keyboard.ctrl_key() || keyboard.alt_key() || keyboard.meta_key() {
        return;
    }
    let Some(target) = context_menu_target_element(event) else {
        return;
    };
    let Some(popup) = target.closest(CONTEXT_MENU_POPUP_SELECTOR).ok().flatten() else {
        return;
    };
    let key: String = keyboard.key();
    let rtl: bool = context_menu_is_rtl(&popup);
    let (forward, backward): (&str, &str) = context_menu_direction_keys(rtl);
    if key == CONTEXT_MENU_ESCAPE_KEY || (submenu && key == backward) {
        event.prevent_default();
        open.set(false);
        if submenu {
            if let Some(owner) = popup.closest(CONTEXT_MENU_SUB_SELECTOR).ok().flatten()
                && let Some(trigger) = owner
                    .query_selector(CONTEXT_MENU_SUB_TRIGGER_SELECTOR)
                    .ok()
                    .flatten()
                && let Some(element) = trigger.dyn_ref::<web_sys::HtmlElement>()
            {
                let _: Result<(), wasm_bindgen::JsValue> = element.focus();
            }
        } else {
            focus_context_menu_trigger(event);
        }
        return;
    }
    let Ok(nodes) = popup.query_selector_all(CONTEXT_MENU_ITEM_SELECTOR) else {
        return;
    };
    let mut rows: Vec<web_sys::Element> = Vec::new();
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Some(row) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };
        if row
            .closest(CONTEXT_MENU_POPUP_SELECTOR)
            .ok()
            .flatten()
            .as_ref()
            == Some(&popup)
            && row
                .get_attribute(CONTEXT_MENU_DISABLED_ATTRIBUTE)
                .as_deref()
                != Some(CONTEXT_MENU_ATTR_TRUE)
        {
            rows.push(row.clone());
        }
    }
    let current: Option<usize> = rows
        .iter()
        .position(|row: &web_sys::Element| row == &target || row.contains(Some(&target)));
    if key == CONTEXT_MENU_ENTER_KEY || key == " " || key == forward {
        if let Some(index) = current {
            let row: &web_sys::Element = &rows[index];
            if key != forward
                || row
                    .matches(CONTEXT_MENU_SUB_TRIGGER_SELECTOR)
                    .unwrap_or(false)
            {
                event.prevent_default();
                if !keyboard.repeat()
                    && let Some(element) = row.dyn_ref::<web_sys::HtmlElement>()
                {
                    element.click();
                }
            }
        }
        return;
    }
    let navigation: bool = [
        CONTEXT_MENU_DOWN_KEY,
        CONTEXT_MENU_UP_KEY,
        CONTEXT_MENU_HOME_KEY,
        CONTEXT_MENU_END_KEY,
    ]
    .contains(&key.as_str());
    let mut search: String = key.to_lowercase();
    if !navigation {
        if key.chars().count() != 1 {
            return;
        }
        let now: f64 = js_sys::Date::now();
        let previous_time: f64 = popup
            .get_attribute(CONTEXT_MENU_SEARCH_TIME_ATTRIBUTE)
            .and_then(|value: String| value.parse().ok())
            .unwrap_or(0.0);
        let previous: String = popup
            .get_attribute(CONTEXT_MENU_SEARCH_ATTRIBUTE)
            .unwrap_or_default();
        if now - previous_time < CONTEXT_MENU_SEARCH_RESET_MS && previous != search {
            search = format!("{previous}{search}");
        }
        let _: Result<(), wasm_bindgen::JsValue> =
            popup.set_attribute(CONTEXT_MENU_SEARCH_ATTRIBUTE, &search);
        let _: Result<(), wasm_bindgen::JsValue> =
            popup.set_attribute(CONTEXT_MENU_SEARCH_TIME_ATTRIBUTE, &now.to_string());
    } else {
        let _: Result<(), wasm_bindgen::JsValue> =
            popup.remove_attribute(CONTEXT_MENU_SEARCH_ATTRIBUTE);
    }
    event.prevent_default();
    let labels: Vec<String> = rows
        .iter()
        .map(|row: &web_sys::Element| row.text_content().unwrap_or_default())
        .collect();
    let selection: Option<usize> =
        context_menu_next_index(&labels, current, if navigation { &key } else { &search });
    if let Some(index) = selection
        && let Some(element) = rows[index].dyn_ref::<web_sys::HtmlElement>()
    {
        let _: Result<(), wasm_bindgen::JsValue> = element.focus();
    }
}

/// Resolves the trigger that received a context-menu event.
///
/// # Arguments
///
/// - `&Event` - The event dispatched by the trigger.
///
/// # Returns
///
/// - `Option<web_sys::Element>` - The trigger element that received the event.
fn context_menu_trigger_element(event: &Event) -> Option<web_sys::Element> {
    let target: web_sys::Element = context_menu_target_element(event)?;
    target.closest(CONTEXT_MENU_TRIGGER_SELECTOR).ok().flatten()
}

/// Resolves the element that received an event.
///
/// # Arguments
///
/// - `&Event` - The event dispatched by a context menu part.
///
/// # Returns
///
/// - `Option<web_sys::Element>` - The event target when it is an element.
fn context_menu_target_element(event: &Event) -> Option<web_sys::Element> {
    let event_target: Option<web_sys::EventTarget> = event.target();
    let event_target: web_sys::EventTarget = event_target?;
    let target_result: Result<web_sys::Element, web_sys::EventTarget> = event_target.dyn_into();
    target_result.ok()
}

/// Positions the context menu at a viewport coordinate.
///
/// # Arguments
///
/// - `&web_sys::Element` - The trigger whose root owns the menu.
/// - `f64` - The viewport x coordinate.
/// - `f64` - The viewport y coordinate.
fn set_context_menu_position(trigger: &web_sys::Element, left: f64, top: f64) {
    let Some(root_element) = trigger.closest(CONTEXT_MENU_ROOT_SELECTOR).ok().flatten() else {
        return;
    };
    let root_result: Result<web_sys::HtmlElement, web_sys::Element> = root_element.dyn_into();
    let Ok(root) = root_result else {
        return;
    };
    let style: web_sys::CssStyleDeclaration = root.style();
    let left_value: String = format!("{left}px");
    let top_value: String = format!("{top}px");
    let _: Result<(), wasm_bindgen::JsValue> =
        style.set_property(CONTEXT_MENU_POSITION_X_PROPERTY, &left_value);
    let _: Result<(), wasm_bindgen::JsValue> =
        style.set_property(CONTEXT_MENU_POSITION_Y_PROPERTY, &top_value);
    #[cfg(target_arch = "wasm32")]
    if let Some(content) = root
        .query_selector(CONTEXT_MENU_CONTENT_SELECTOR)
        .ok()
        .flatten()
        .and_then(|element: web_sys::Element| element.dyn_into::<web_sys::HtmlElement>().ok())
    {
        for property in [
            CONTEXT_MENU_LEFT_PROPERTY,
            CONTEXT_MENU_TOP_PROPERTY,
            CONTEXT_MENU_TRANSFORM_PROPERTY,
        ] {
            let _: Result<String, wasm_bindgen::JsValue> =
                content.style().remove_property(property);
        }
    }
}

/// Places a keyboard-opened context menu below its trigger.
///
/// # Arguments
///
/// - `&Event` - The keyboard event dispatched by the trigger.
fn set_context_menu_position_at_trigger(event: &Event) {
    let Some(trigger) = context_menu_trigger_element(event) else {
        return;
    };
    let trigger_rect: web_sys::DomRect = trigger.get_bounding_client_rect();
    set_context_menu_position(&trigger, trigger_rect.left(), trigger_rect.bottom());
}

/// Focuses the menu popup after the next render frame.
///
/// # Arguments
///
/// - `&web_sys::Element` - The trigger whose context menu should receive focus.
fn focus_context_menu_content_on_next_frame(trigger: &web_sys::Element) {
    let Some(root) = trigger.closest(CONTEXT_MENU_ROOT_SELECTOR).ok().flatten() else {
        return;
    };
    let Some(window) = web_sys::window() else {
        return;
    };
    let callback: js_sys::Function = Closure::once_into_js(move || {
        let content_result: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> =
            root.query_selector(CONTEXT_MENU_CONTENT_SELECTOR);
        let Some(content_element) = content_result.ok().flatten() else {
            return;
        };
        let content_result: Result<web_sys::HtmlElement, web_sys::Element> =
            content_element.dyn_into();
        let Ok(content) = content_result else {
            return;
        };
        let _: Result<(), wasm_bindgen::JsValue> = content.focus();
    })
    .unchecked_into();
    let _: Result<i32, wasm_bindgen::JsValue> = window.request_animation_frame(&callback);
}

/// Returns focus to the trigger that owns the event target.
///
/// # Arguments
///
/// - `&Event` - The event dispatched inside the context menu.
fn focus_context_menu_trigger(event: &Event) {
    let Some(target) = context_menu_target_element(event) else {
        return;
    };
    let Some(root) = target.closest(CONTEXT_MENU_ROOT_SELECTOR).ok().flatten() else {
        return;
    };
    let trigger_result: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> =
        root.query_selector(CONTEXT_MENU_TRIGGER_SELECTOR);
    let Some(trigger_element) = trigger_result.ok().flatten() else {
        return;
    };
    let trigger_result: Result<web_sys::HtmlElement, web_sys::Element> = trigger_element.dyn_into();
    let Ok(trigger) = trigger_result else {
        return;
    };
    let _: Result<(), wasm_bindgen::JsValue> = trigger.focus();
}

/// The frame that holds a context menu.
///
/// The frame owns the open state, so the trigger and the panel always
/// agree on whether the menu is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu virtual DOM tree.
#[component]
pub fn euv_context_menu(node: VirtualNode<EuvContextMenuProps>) -> VirtualNode {
    let EuvContextMenuProps { open }: EuvContextMenuProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let outside: UseClickOutside = use_click_outside();
    let outside_target: NodeRef<web_sys::Element> = outside.get_target().clone();
    let outside_triggered: Signal<bool> = outside.get_triggered();
    watch!(outside_triggered, |triggered: bool| {
        if triggered {
            if open.get() {
                open.set(false);
            }
            outside_triggered.set(false);
        }
    });
    let on_keydown: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let Some(keyboard) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
            return;
        };
        if keyboard.key() != CONTEXT_MENU_ESCAPE_KEY || !open.get() {
            return;
        }
        event.prevent_default();
        open.set(false);
        focus_context_menu_trigger(&event);
    }));
    html! {
        div {
            "data-slot": "context-menu"
            ref: outside_target
            class: c_euv_context_menu()
            onkeydown: on_keydown
            children
        }
    }
}

/// The surface the menu is opened from.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu trigger virtual DOM tree.
#[component]
pub fn euv_context_menu_trigger(node: VirtualNode<EuvContextMenuTriggerProps>) -> VirtualNode {
    let EuvContextMenuTriggerProps {
        open,
        label,
        disabled,
    }: EuvContextMenuTriggerProps = node.try_get_props().unwrap_or_default();
    let hold: Rc<RefCell<ContextMenuLongPress>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(ContextMenuLongPress::default())));
    let cleanup_hold: Rc<RefCell<ContextMenuLongPress>> = hold.clone();
    App::use_cleanup(move || cleanup_hold.borrow_mut().cancel());
    if disabled {
        hold.borrow_mut().cancel();
    }
    let start_hold: Rc<RefCell<ContextMenuLongPress>> = hold.clone();
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        context_menu_touch_start(&event, &start_hold, open, disabled);
    }));
    let move_hold: Rc<RefCell<ContextMenuLongPress>> = hold.clone();
    let on_touch_move: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let Some(touch_event) = event.dyn_ref::<web_sys::TouchEvent>() else {
            return;
        };
        let touches: web_sys::TouchList = touch_event.touches();
        if let Some(touch) = touches.item(0) {
            move_hold
                .borrow_mut()
                .moved(touches.length(), touch.client_x(), touch.client_y());
        } else {
            move_hold.borrow_mut().cancel();
        }
    }));
    let end_hold: Rc<RefCell<ContextMenuLongPress>> = hold.clone();
    let on_touch_end: Option<Rc<dyn Fn(Event)>> =
        Some(Rc::new(move |_: Event| end_hold.borrow_mut().cancel()));
    let on_touch_cancel: Option<Rc<dyn Fn(Event)>> =
        Some(Rc::new(move |_: Event| hold.borrow_mut().cancel()));
    let on_context_menu: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        if disabled {
            return;
        }
        event.prevent_default();
        if let Some(trigger) = context_menu_trigger_element(&event) {
            if let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() {
                open_context_menu_at(
                    &trigger,
                    open,
                    mouse_event.client_x(),
                    mouse_event.client_y(),
                );
                return;
            } else {
                set_context_menu_position_at_trigger(&event);
            }
            open.set(true);
            focus_context_menu_content_on_next_frame(&trigger);
        } else {
            open.set(true);
        }
    }));
    let on_keydown: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let Some(keyboard) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
            return;
        };
        if keyboard.key() == CONTEXT_MENU_ESCAPE_KEY && open.get() {
            event.prevent_default();
            open.set(false);
            return;
        }
        if !disabled
            && (keyboard.key() == CONTEXT_MENU_KEY
                || (keyboard.key() == CONTEXT_MENU_F10_KEY && keyboard.shift_key()))
        {
            event.prevent_default();
            set_context_menu_position_at_trigger(&event);
            open.set(true);
            if let Some(trigger) = context_menu_trigger_element(&event) {
                focus_context_menu_content_on_next_frame(&trigger);
            }
        }
    }));

    html! {
            div {
                role: "button"
                tabindex: "0"
                "aria-haspopup": "menu"
                "aria-expanded": if { open.get() } {
                    "true"
                } else {
                    "false"
                }
                "data-slot": "context-menu-trigger"
                class: c_euv_context_menu_trigger()

                "aria-disabled": if disabled {
                    "true"
                } else {
                    "false"
                }
                oncontextmenu: on_context_menu
                onkeydown: on_keydown
                ontouchstart: on_touch_start
                ontouchmove: on_touch_move
                ontouchend: on_touch_end
                ontouchcancel: on_touch_cancel
                {
                    label
                }
            }
    }
}

/// The panel that holds the rows of the menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu content virtual DOM tree.
#[component]
pub fn euv_context_menu_content(node: VirtualNode<EuvContextMenuContentProps>) -> VirtualNode {
    let EuvContextMenuContentProps { open }: EuvContextMenuContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "menu"
            "data-slot": "context-menu-content"
            onkeydown: move |event: Event| context_menu_keydown(&event, open, false)
            tabindex: "-1"
            class: c_euv_context_menu_content()
            class: if { open } {
                c_euv_context_menu_content_open()
            } else {
                c_euv_context_menu_content_closed()
            }
            children
        }
    }
}

/// The group of rows that belong to one heading.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu group virtual DOM tree.
#[component]
pub fn euv_context_menu_group(node: VirtualNode<EuvContextMenuGroupProps>) -> VirtualNode {
    let EuvContextMenuGroupProps { label }: EuvContextMenuGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "context-menu-group"
            "aria-label": label
            class: c_euv_context_menu_group()
            children
        }
    }
}

/// Creates lifecycle-owned selection state shared explicitly by all radio parts.
///
/// Controlled activation requests a value through the callback without mutating
/// the supplied signal. Uncontrolled activation updates the internal signal first.
/// Keep this hook unconditional, and give each item a unique value.
///
/// # Arguments
///
/// - `Option<Signal<String>>` - The caller-owned controlled value, if any.
/// - `&'static str` - Initial uncontrolled value, read only on first mount.
/// - `Option<Rc<dyn Fn(String)>>` - Callback for actual value-change requests.
///
/// # Returns
///
/// - `EuvContextMenuRadioGroupState` - The shared selection handle.
pub fn use_context_menu_radio_group(
    value: Option<Signal<String>>,
    default_value: &'static str,
    on_value_change: Option<Rc<dyn Fn(String)>>,
) -> EuvContextMenuRadioGroupState {
    let internal_value: Signal<String> = App::use_signal(move || default_value.to_owned());
    EuvContextMenuRadioGroupState {
        value: value.unwrap_or(internal_value),
        controlled: value.is_some(),
        on_value_change,
    }
}

/// Applies the shared mouse/keyboard radio activation contract without DOM state.
///
/// # Arguments
///
/// - `&EuvContextMenuRadioItemProps` - Item settings and the group's shared state.
///
/// # Returns
///
/// - `bool` - Whether the menu closed and focus should return to its trigger.
pub(crate) fn activate_context_menu_radio(props: &EuvContextMenuRadioItemProps) -> bool {
    if props.disabled {
        return false;
    }
    if props.selection.value.get() != props.value {
        let next: String = props.value.to_owned();
        if !props.selection.controlled {
            props.selection.value.set(next.clone());
        }
        if let Some(callback) = &props.selection.on_value_change {
            callback(next);
        }
    }
    if props.close_on_click {
        props.open.set(false);
    }
    props.close_on_click
}

/// The group of rows that share one radio selection.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuRadioGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu radio group virtual DOM tree.
#[component]
pub fn euv_context_menu_radio_group(
    node: VirtualNode<EuvContextMenuRadioGroupProps>,
) -> VirtualNode {
    let EuvContextMenuRadioGroupProps { label, selection }: EuvContextMenuRadioGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "data-slot": "context-menu-radio-group"
            "data-value": selection.value.get()
            "aria-label": label
            class: c_euv_context_menu_radio_group()
            children
        }
    }
}

/// Connects submenu state to an immediate owner with teardown-safe subscriptions.
///
/// # Arguments
///
/// - `Signal<bool>` - Immediate parent popup's controlled open state.
/// - `Signal<bool>` - This submenu's distinct controlled open state.
pub(crate) fn use_context_menu_sub_binding(parent_open: Signal<bool>, open: Signal<bool>) {
    let scope: Rc<ContextMenuSubBindingScope> = HookContext::use_hook(|| {
        Rc::new(ContextMenuSubBindingScope {
            slot: Rc::new(RefCell::new(None)),
        })
    });
    let binding: Rc<RefCell<Option<ContextMenuSubBinding>>> = scope.get_slot().clone();
    let cleanup: Rc<RefCell<Option<ContextMenuSubBinding>>> = binding.clone();
    App::use_cleanup(move || {
        cleanup.borrow_mut().take();
    });
    let unchanged: bool =
        binding
            .borrow()
            .as_ref()
            .is_some_and(|current: &ContextMenuSubBinding| {
                current.get_parent_open() == parent_open && current.get_open() == open
            });
    if unchanged {
        return;
    }
    binding.borrow_mut().take();
    if !parent_open.get() && open.get() {
        open.set(false);
    }
    let parent_subscription: usize = parent_open.subscribe(move || {
        if !parent_open.get() && open.get() {
            open.set(false);
        }
    });
    let open_subscription: usize = open.subscribe(move || {
        if open.get() && !parent_open.get() {
            open.set(false);
        }
    });
    #[cfg(target_arch = "wasm32")]
    let hover_owner: Rc<ContextMenuHoverOwner> = Rc::new(ContextMenuHoverOwner {
        parent: parent_open,
        controller: RefCell::new(None),
    });
    #[cfg(target_arch = "wasm32")]
    CONTEXT_MENU_SUB_PARENTS.with(
        |parents: &RefCell<HashMap<usize, Weak<ContextMenuHoverOwner>>>| {
            parents
                .borrow_mut()
                .insert(open.get_inner(), Rc::downgrade(&hover_owner));
        },
    );
    *binding.borrow_mut() = Some(ContextMenuSubBinding {
        parent_open,
        open,
        parent_subscription,
        open_subscription,
        #[cfg(target_arch = "wasm32")]
        hover_owner,
    });
}

/// The nested menu of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSubProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu sub virtual DOM tree.
#[component]
pub fn euv_context_menu_sub(node: VirtualNode<EuvContextMenuSubProps>) -> VirtualNode {
    let EuvContextMenuSubProps { open, parent_open }: EuvContextMenuSubProps =
        node.try_get_props().unwrap_or_default();
    use_context_menu_sub_binding(parent_open, open);
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "context-menu-sub"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_context_menu_sub()
            children
        }
    }
}

/// The row that opens a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSubTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu sub trigger virtual DOM tree.
#[component]
pub fn euv_context_menu_sub_trigger(
    node: VirtualNode<EuvContextMenuSubTriggerProps>,
) -> VirtualNode {
    let EuvContextMenuSubTriggerProps {
        label,
        open,
        disabled,
        open_on_hover,
        delay_ms,
        close_delay_ms,
    }: EuvContextMenuSubTriggerProps = node.try_get_props().unwrap_or_default();
    let trigger_ref: NodeRef<web_sys::Element> = App::use_node_ref();
    #[cfg(not(target_arch = "wasm32"))]
    let _: (bool, u32, u32) = (open_on_hover, delay_ms, close_delay_ms);
    #[cfg(target_arch = "wasm32")]
    use_context_menu_sub_hover(
        open,
        trigger_ref.clone(),
        (disabled, open_on_hover, delay_ms, close_delay_ms),
    );
    html! {
        button {
            type: "button"
            role: "menuitem"
            tabindex: "-1"
            "data-slot": "context-menu-sub-trigger"
            ref: trigger_ref
            disabled: disabled
            "aria-disabled": if disabled {
                "true"
            } else {
                "false"
            }
            onclick: move |event: Event| {
                if disabled {
                    return;
                }
                open.set(true);
                if let Some(target) = context_menu_target_element(&event)
                && let Some(owner) = target.closest(CONTEXT_MENU_SUB_SELECTOR).ok().flatten()
                && let Some(window) = web_sys::window() {
                    let
                    callback: js_sys::Function = Closure::once_into_js(move || {
                        if let Some(content) = owner.query_selector(CONTEXT_MENU_SUB_CONTENT_SELECTOR).ok().flatten() {
                            let
                            row: Option<web_sys::Element> = content.query_selector(CONTEXT_MENU_ITEM_SELECTOR).ok().flatten();
                            if let Some(element) = row.as_ref().unwrap_or(&content).dyn_ref::<web_sys::HtmlElement>() {
                                let
                                _: Result<(), wasm_bindgen::JsValue> = element.focus();
                            }
                        }
                    }).unchecked_into();
                    let
                    _: Result<i32, wasm_bindgen::JsValue> = window.request_animation_frame(&callback);
                }
            }
            "aria-haspopup": "menu"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            class: c_euv_context_menu_sub_trigger()
            {
                label
            }
        }
    }
}

/// The panel of a nested menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSubContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu sub content virtual DOM tree.
#[component]
pub fn euv_context_menu_sub_content(
    node: VirtualNode<EuvContextMenuSubContentProps>,
) -> VirtualNode {
    let EuvContextMenuSubContentProps { open }: EuvContextMenuSubContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let panel_ref: NodeRef<web_sys::Element> = App::use_node_ref();
    #[cfg(target_arch = "wasm32")]
    {
        let position_binding: Rc<RefCell<Option<(Signal<bool>, usize)>>> =
            HookContext::use_hook(|| Rc::new(RefCell::new(None)));
        let cleanup_binding: Rc<RefCell<Option<(Signal<bool>, usize)>>> = position_binding.clone();
        let cleanup_ref: NodeRef<web_sys::Element> = panel_ref.clone();
        App::use_cleanup(move || {
            if let Some((previous, subscription)) = cleanup_binding.borrow_mut().take() {
                previous.unsubscribe(subscription);
            }
            reset_context_menu_sub_position(&cleanup_ref);
        });
        let unchanged: bool = position_binding
            .borrow()
            .as_ref()
            .is_some_and(|(previous, _): &(Signal<bool>, usize)| *previous == open);
        if !unchanged {
            if let Some((previous, subscription)) = position_binding.borrow_mut().take() {
                previous.unsubscribe(subscription);
            }
            let position_ref: NodeRef<web_sys::Element> = panel_ref.clone();
            let subscription: usize = open.subscribe(move || {
                if open.get() {
                    schedule_context_menu_sub_position(position_ref.clone(), open);
                } else {
                    reset_context_menu_sub_position(&position_ref);
                }
            });
            *position_binding.borrow_mut() = Some((open, subscription));
        }
        let resize_ref: NodeRef<web_sys::Element> = panel_ref.clone();
        App::use_window_event(CONTEXT_MENU_RESIZE_EVENT, move || {
            if let Some((current, _)) = *position_binding.borrow()
                && current.get()
            {
                schedule_context_menu_sub_position(resize_ref.clone(), current);
            }
        });
        if open.get() {
            schedule_context_menu_sub_position(panel_ref.clone(), open);
        }
    }
    html! {
        div {
            role: "menu"
            "data-slot": "context-menu-sub-content"
            style: if { open.get() } {
                ""
            } else {
                "display:none;"
            }
            "aria-hidden": if { open.get() } {
                "false"
            } else {
                "true"
            }
            ref: panel_ref
            tabindex: "-1"
            onkeydown: move |event: Event| context_menu_keydown(&event, open, true)
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_context_menu_sub_content()
            children
        }
    }
}

/// One row of the menu.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu item virtual DOM tree.
#[component]
pub fn euv_context_menu_item(node: VirtualNode<EuvContextMenuItemProps>) -> VirtualNode {
    let EuvContextMenuItemProps {
        open,
        value,
        label,
        disabled,
        close_on_click,
        on_select,
    }: EuvContextMenuItemProps = node.try_get_props().unwrap_or_default();
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_context_menu_item_disabled
    } else {
        c_euv_context_menu_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |event: Event| {
            if let Some(callback) = &on_select {
                callback(value);
            }
            if close_on_click {
                open.set(false);
                focus_context_menu_trigger(&event);
            }
        }))
    };
    html! {
        div {
            role: "menuitem"
            tabindex: "-1"
            "data-slot": "context-menu-item"
            "aria-disabled": if disabled { "true" } else { "false" }
            class: item_class()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// A row that carries a box the reader ticks.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuCheckboxItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu checkbox item virtual DOM tree.
#[component]
pub fn euv_context_menu_checkbox_item(
    node: VirtualNode<EuvContextMenuCheckboxItemProps>,
) -> VirtualNode {
    let EuvContextMenuCheckboxItemProps {
        open,
        close_on_click,
        label,
        checked,
        on_select,
    }: EuvContextMenuCheckboxItemProps = node.try_get_props().unwrap_or_default();
    let is_checked: bool = checked.get();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        if let Some(callback) = &on_select {
            callback(!is_checked);
        }
        if close_on_click {
            open.set(false);
            focus_context_menu_trigger(&event);
        }
    }));
    html! {
        div {
            role: "menuitemcheckbox"
            tabindex: "-1"
            "data-slot": "context-menu-checkbox-item"
            "aria-checked": if is_checked { "true" } else { "false" }
            class: c_euv_context_menu_checkbox_item()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// A row that carries a dot the reader moves.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuRadioItemProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu radio item virtual DOM tree.
#[component]
pub fn euv_context_menu_radio_item(node: VirtualNode<EuvContextMenuRadioItemProps>) -> VirtualNode {
    let props: EuvContextMenuRadioItemProps = node.try_get_props().unwrap_or_default();
    let label: &'static str = props.label;
    let value: &'static str = props.value;
    let disabled: bool = props.disabled;
    let is_checked: bool = props.selection.value.get() == value;
    let item_class: fn() -> &'static Css = if disabled {
        c_euv_context_menu_item_disabled
    } else {
        c_euv_context_menu_radio_item
    };
    let on_click: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        Some(Rc::new(move |event: Event| {
            if activate_context_menu_radio(&props) {
                focus_context_menu_trigger(&event);
            }
        }))
    };
    html! {
        div {
            role: "menuitemradio"
            tabindex: "-1"
            "data-slot": "context-menu-radio-item"
            "data-value": value
            "aria-disabled": if disabled {
                "true"
            } else {
                "false"
            }
            onclick: on_click
            "aria-checked": if is_checked {
                "true"
            } else {
                "false"
            }
            class: item_class()
            {
                label
            }
        }
    }
}

/// The heading that names a group of rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuLabelProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu label virtual DOM tree.
#[component]
pub fn euv_context_menu_label(node: VirtualNode<EuvContextMenuLabelProps>) -> VirtualNode {
    let EuvContextMenuLabelProps { text }: EuvContextMenuLabelProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            "data-slot": "context-menu-label"
            class: c_euv_context_menu_label()
            {
                text
            }
        }
    }
}

/// The rule that divides one group of rows from the next.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuSeparatorProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu separator virtual DOM tree.
#[component]
pub fn euv_context_menu_separator(node: VirtualNode<EuvContextMenuSeparatorProps>) -> VirtualNode {
    let EuvContextMenuSeparatorProps { label }: EuvContextMenuSeparatorProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            role: "separator"
            "data-slot": "context-menu-separator"
            class: c_euv_context_menu_separator()
            "aria-label": label
        }
    }
}

/// The key hint shown at the trailing edge of a row.
///
/// # Arguments
///
/// - `VirtualNode<EuvContextMenuShortcutProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv context menu shortcut virtual DOM tree.
#[component]
pub fn euv_context_menu_shortcut(node: VirtualNode<EuvContextMenuShortcutProps>) -> VirtualNode {
    let EuvContextMenuShortcutProps { text }: EuvContextMenuShortcutProps =
        node.try_get_props().unwrap_or_default();
    html! {
        span {
            "data-slot": "context-menu-shortcut"
            class: c_euv_context_menu_shortcut()
            {
                text
            }
        }
    }
}

/// Accepts only hover-capable mouse and pen input for a live enabled owner.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `bool` - A boolean.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn context_menu_hover_eligible(
    pointer: &str,
    hover: bool,
    disabled: bool,
    enabled: bool,
    parent: bool,
) -> bool {
    matches!(
        pointer,
        CONTEXT_MENU_POINTER_MOUSE | CONTEXT_MENU_POINTER_PEN
    ) && hover
        && !disabled
        && enabled
        && parent
}

/// Chooses one cancellable branch intent without changing the owning root.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `Option<bool>` - `Some(...)` on success, `None` otherwise.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn context_menu_hover_intent(
    open: bool,
    trigger: bool,
    branch: bool,
    bridge: bool,
    eligible: bool,
) -> Option<bool> {
    if trigger {
        (!open && eligible).then_some(true)
    } else if open && !branch && !bridge {
        Some(false)
    } else {
        None
    }
}

/// Tests a bounded triangle to the actual facing popup edge, including side flips.
/// Rectangles are viewport left/top/right/bottom; no RTL assumption or grace timer is used.
///
/// # Arguments
///
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `(f64, f64, f64, f64)` - A `(f64, f64, f64, f64)` parameter.
/// - `(f64, f64, f64, f64)` - A `(f64, f64, f64, f64)` parameter.
///
/// # Returns
///
/// - `bool` - A boolean.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn context_menu_hover_bridge(
    point: (f64, f64),
    exit: (f64, f64),
    trigger: (f64, f64, f64, f64),
    panel: (f64, f64, f64, f64),
) -> bool {
    if panel.2 <= panel.0 || panel.3 <= panel.1 {
        return false;
    }
    let edge: f64 = if panel.0 + panel.2 >= trigger.0 + trigger.2 {
        panel.0
    } else {
        panel.2
    };
    let width: f64 = edge - exit.0;
    if width.abs() < f64::EPSILON {
        return false;
    }
    let fraction: f64 = (point.0 - exit.0) / width;
    if !(0.0..=1.0).contains(&fraction) {
        return false;
    }
    let top: f64 = exit.1 + fraction * (panel.1 - exit.1);
    let bottom: f64 = exit.1 + fraction * (panel.3 - exit.1);
    point.1 >= top.min(bottom) && point.1 <= top.max(bottom)
}

/// Resolves a live binding identity without reconstructing Signal handles from DOM.
///
/// # Arguments
/// - `Signal<bool>` - Controlled submenu state.
/// - `&Rc<ContextMenuHoverOwner>` - Expected binding identity.
///
/// # Returns
/// - `bool` - Whether this binding still owns the submenu.
#[cfg(target_arch = "wasm32")]
pub(super) fn context_menu_hover_owner_live(
    open: Signal<bool>,
    owner: &Rc<ContextMenuHoverOwner>,
) -> bool {
    CONTEXT_MENU_SUB_PARENTS.with(
        |parents: &RefCell<HashMap<usize, Weak<ContextMenuHoverOwner>>>| {
            parents
                .borrow()
                .get(&open.get_inner())
                .and_then(Weak::upgrade)
                .is_some_and(|current: Rc<ContextMenuHoverOwner>| Rc::ptr_eq(&current, owner))
        },
    )
}

/// Processes physical hover moves; sibling rows take precedence over the geometric bridge.
///
/// # Arguments
///
/// - `&ContextMenuSubHover` - Shared reference to a `ContextMenuSubHover`.
/// - `&Event` - Shared reference to a `Event`.
#[cfg(target_arch = "wasm32")]
fn context_menu_sub_hover_move(state: &ContextMenuSubHover, event: &Event) {
    let Some(trigger) = state
        .target
        .get_cloned()
        .filter(|trigger: &web_sys::Element| trigger.is_connected())
    else {
        state.cancel();
        return;
    };
    let pointer: String = js_sys::Reflect::get(
        event.as_ref(),
        &wasm_bindgen::JsValue::from_str(CONTEXT_MENU_POINTER_TYPE),
    )
    .ok()
    .and_then(|value: wasm_bindgen::JsValue| value.as_string())
    .unwrap_or_default();
    let hover: bool = web_sys::window()
        .and_then(|window: web_sys::Window| {
            window.match_media(CONTEXT_MENU_HOVER_MEDIA).ok().flatten()
        })
        .is_some_and(|query: web_sys::MediaQueryList| query.matches());
    if !matches!(
        pointer.as_str(),
        CONTEXT_MENU_POINTER_MOUSE | CONTEXT_MENU_POINTER_PEN
    ) || !hover
    {
        state.cancel();
        return;
    }
    if !state.parent.get_parent().get() {
        state.cancel();
        return;
    }
    let Some(mouse) = event.dyn_ref::<web_sys::MouseEvent>() else {
        return;
    };
    if pointer == CONTEXT_MENU_POINTER_PEN && mouse.buttons() != 0 {
        state.cancel();
        return;
    }
    let target: Option<web_sys::Element> = event
        .target()
        .and_then(|target: web_sys::EventTarget| target.dyn_into::<web_sys::Element>().ok());
    let on_trigger: bool = target
        .as_ref()
        .is_some_and(|target: &web_sys::Element| trigger.contains(Some(target)));
    let point: (f64, f64) = (f64::from(mouse.client_x()), f64::from(mouse.client_y()));
    if on_trigger {
        state.anchor.set(Some(point));
    }
    let owner: Option<web_sys::Element> = trigger.closest(CONTEXT_MENU_SUB_SELECTOR).ok().flatten();
    let panel: Option<web_sys::Element> = owner.as_ref().and_then(|owner: &web_sys::Element| {
        owner
            .query_selector(CONTEXT_MENU_SUB_CONTENT_SELECTOR)
            .ok()
            .flatten()
    });
    let in_branch: bool = state.open.get()
        && target.as_ref().is_some_and(|target: &web_sys::Element| {
            panel
                .as_ref()
                .is_some_and(|panel: &web_sys::Element| panel.contains(Some(target)))
        });
    if in_branch {
        if let Some(row) = target.as_ref().and_then(|target: &web_sys::Element| {
            target.closest(CONTEXT_MENU_ITEM_SELECTOR).ok().flatten()
        }) && row
            .get_attribute(CONTEXT_MENU_DISABLED_ATTRIBUTE)
            .as_deref()
            != Some(CONTEXT_MENU_ATTR_TRUE)
            && let Some(row) = row.dyn_ref::<web_sys::HtmlElement>()
        {
            let options: web_sys::FocusOptions = web_sys::FocusOptions::new();
            options.set_prevent_scroll(true);
            let _: Result<(), wasm_bindgen::JsValue> = row.focus_with_options(&options);
        }
    }
    let sibling: bool = target
        .as_ref()
        .and_then(|target: &web_sys::Element| {
            target.closest(CONTEXT_MENU_ITEM_SELECTOR).ok().flatten()
        })
        .is_some_and(|row: web_sys::Element| {
            row != trigger
                && row.closest(CONTEXT_MENU_POPUP_SELECTOR).ok().flatten()
                    == trigger.closest(CONTEXT_MENU_POPUP_SELECTOR).ok().flatten()
        });
    let bridge: bool = state.open.get()
        && !sibling
        && state.anchor.get().is_some_and(|exit: (f64, f64)| {
            panel.as_ref().is_some_and(|panel: &web_sys::Element| {
                let row: web_sys::DomRect = trigger.get_bounding_client_rect();
                let rect: web_sys::DomRect = panel.get_bounding_client_rect();
                context_menu_hover_bridge(
                    point,
                    exit,
                    (row.left(), row.top(), row.right(), row.bottom()),
                    (rect.left(), rect.top(), rect.right(), rect.bottom()),
                )
            })
        });
    let (disabled, enabled, _, _): (bool, bool, u32, u32) = state.options.get();
    let eligible: bool = context_menu_hover_eligible(
        &pointer,
        hover,
        disabled,
        enabled,
        state.parent.get_parent().get(),
    );
    state.schedule(context_menu_hover_intent(
        state.open.get(),
        on_trigger,
        in_branch,
        bridge,
        eligible,
    ));
}

/// Owns pointer listeners, subscriptions and a single cancellable deadline per trigger.
///
/// # Arguments
///
/// - `Signal<bool>` - A `Signal<bool>` parameter.
/// - `NodeRef<web_sys::Element>` - A `NodeRef<web_sys::Element>` parameter.
/// - `(bool, bool, u32, u32)` - A `(bool, bool, u32, u32)` parameter.
#[cfg(target_arch = "wasm32")]
fn use_context_menu_sub_hover(
    open: Signal<bool>,
    target: NodeRef<web_sys::Element>,
    options: (bool, bool, u32, u32),
) {
    let slot: Rc<RefCell<Option<Rc<ContextMenuSubHover>>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let cleanup_slot: Weak<RefCell<Option<Rc<ContextMenuSubHover>>>> = Rc::downgrade(&slot);
    App::use_cleanup(move || {
        if let Some(slot) = cleanup_slot.upgrade()
            && let Some(state) = slot.borrow_mut().take()
        {
            state.dispose();
        }
    });
    let parent: Option<Rc<ContextMenuHoverOwner>> = CONTEXT_MENU_SUB_PARENTS.with(
        |parents: &RefCell<HashMap<usize, Weak<ContextMenuHoverOwner>>>| {
            parents
                .borrow()
                .get(&open.get_inner())
                .and_then(Weak::upgrade)
        },
    );
    let Some(parent) = parent else {
        return;
    };
    if slot
        .borrow()
        .as_ref()
        .is_some_and(|state: &Rc<ContextMenuSubHover>| {
            state.open == open && state.mounted.get() && Rc::ptr_eq(&state.parent, &parent)
        })
    {
        if let Some(state) = slot.borrow().as_ref()
            && state.options.replace(options) != options
        {
            state.cancel();
            state.anchor.set(None);
        }
        return;
    }
    if let Some(state) = slot.borrow_mut().take() {
        state.dispose();
    }
    let Some(document) = web_sys::window().and_then(|window: web_sys::Window| window.document())
    else {
        return;
    };
    let state: Rc<ContextMenuSubHover> = Rc::new(ContextMenuSubHover {
        open,
        parent,
        target,
        options: Cell::new(options),
        mounted: Cell::new(true),
        timer: Cell::new(None),
        intent: Cell::new(None),
        anchor: Cell::new(None),
        callback: RefCell::new(None),
        cleanup: RefCell::new(None),
    });
    let weak: Weak<ContextMenuSubHover> = Rc::downgrade(&state);
    *state.callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if let Some(state) = weak.upgrade() {
            state.commit();
        }
    }) as Box<dyn FnMut()>));
    let weak: Weak<ContextMenuSubHover> = Rc::downgrade(&state);
    let listener: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        let Some(state) = weak.upgrade() else {
            return;
        };
        if event.type_() == CONTEXT_MENU_POINTER_MOVE {
            context_menu_sub_hover_move(&state, &event);
        } else if event.type_() == CONTEXT_MENU_POINTER_OUT {
            if event
                .dyn_ref::<web_sys::MouseEvent>()
                .is_some_and(|event: &web_sys::MouseEvent| event.related_target().is_none())
            {
                state.schedule(state.open.get().then_some(false));
            }
        } else {
            state.cancel();
        }
    }) as Box<dyn FnMut(Event)>);
    for name in CONTEXT_MENU_HOVER_EVENTS {
        let _: Result<(), wasm_bindgen::JsValue> = document
            .add_event_listener_with_callback_and_bool(
                name,
                listener.as_ref().unchecked_ref(),
                true,
            );
    }
    let parent_signal: Signal<bool> = state.parent.get_parent();
    let weak: Weak<ContextMenuSubHover> = Rc::downgrade(&state);
    let parent_id: usize = parent_signal.subscribe(move || {
        if let Some(state) = weak.upgrade() {
            state.cancel();
            state.anchor.set(None);
        }
    });
    let weak: Weak<ContextMenuSubHover> = Rc::downgrade(&state);
    let open_id: usize = open.subscribe(move || {
        if let Some(state) = weak.upgrade() {
            state.cancel();
        }
    });
    let weak: Weak<ContextMenuSubHover> = Rc::downgrade(&state);
    let disconnect_callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if let Some(state) = weak.upgrade()
            && !state
                .target
                .get_cloned()
                .is_some_and(|target: web_sys::Element| target.is_connected())
        {
            state.cancel();
            state.anchor.set(None);
        }
    }) as Box<dyn FnMut()>);
    let observer: Option<wasm_bindgen::JsValue> = js_sys::Reflect::get(
        &js_sys::global(),
        &wasm_bindgen::JsValue::from_str(CONTEXT_MENU_MUTATION_OBSERVER),
    )
    .ok()
    .and_then(|constructor: wasm_bindgen::JsValue| constructor.dyn_into::<js_sys::Function>().ok())
    .and_then(|constructor: js_sys::Function| {
        let arguments: js_sys::Array = js_sys::Array::new();
        arguments.push(disconnect_callback.as_ref());
        js_sys::Reflect::construct(&constructor, &arguments).ok()
    });
    if let Some(observer) = observer.as_ref() {
        let options: js_sys::Object = js_sys::Object::new();
        for property in CONTEXT_MENU_OBSERVER_OPTIONS {
            let _: Result<bool, wasm_bindgen::JsValue> = js_sys::Reflect::set(
                &options,
                &wasm_bindgen::JsValue::from_str(property),
                &wasm_bindgen::JsValue::TRUE,
            );
        }
        if let Ok(method) = js_sys::Reflect::get(
            observer,
            &wasm_bindgen::JsValue::from_str(CONTEXT_MENU_OBSERVE),
        ) && let Some(method) = method.dyn_ref::<js_sys::Function>()
        {
            let _: Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> =
                method.call2(observer, document.as_ref(), options.as_ref());
        }
    }
    let cleanup: Box<dyn FnOnce()> = Box::new(move || {
        if let Some(observer) = observer
            && let Ok(method) = js_sys::Reflect::get(
                &observer,
                &wasm_bindgen::JsValue::from_str(CONTEXT_MENU_DISCONNECT),
            )
            && let Some(method) = method.dyn_ref::<js_sys::Function>()
        {
            let _: Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> = method.call0(&observer);
        }
        drop(disconnect_callback);
        open.unsubscribe(open_id);
        parent_signal.unsubscribe(parent_id);
        for name in CONTEXT_MENU_HOVER_EVENTS {
            let _: Result<(), wasm_bindgen::JsValue> = document
                .remove_event_listener_with_callback_and_bool(
                    name,
                    listener.as_ref().unchecked_ref(),
                    true,
                );
        }
    });
    *state.cleanup.borrow_mut() = Some(cleanup);
    *state.parent.get_controller().borrow_mut() = Some(Rc::downgrade(&state));
    *slot.borrow_mut() = Some(state);
}
