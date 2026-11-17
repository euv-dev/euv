use super::*;

/// Holds the panels of a resizable group.
///
/// The group lays the panels out in a row or in a column, and the
/// handles between them move the divider. The share of every panel is
/// carried by a signal the caller owns, so a drag moves one number and
/// the layout follows.
///
/// # Arguments
///
/// - `VirtualNode<EuvResizablePanelGroupProps>` - The props node
///   carrying the direction of the group.
///
/// # Returns
///
/// - `VirtualNode` - The euv resizable panel group virtual DOM tree.
#[component]
pub fn euv_resizable_panel_group(node: VirtualNode<EuvResizablePanelGroupProps>) -> VirtualNode {
    let EuvResizablePanelGroupProps { direction }: EuvResizablePanelGroupProps =
        node.try_get_props().unwrap_or_default();
    let direction_attr: &str = match direction {
        EuvResizableDirection::Horizontal => RESIZABLE_DIRECTION_HORIZONTAL,
        EuvResizableDirection::Vertical => RESIZABLE_DIRECTION_VERTICAL,
    };
    let direction_class: fn() -> &'static Css = match direction {
        EuvResizableDirection::Horizontal => c_euv_resizable_group_horizontal,
        EuvResizableDirection::Vertical => c_euv_resizable_group_vertical,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "resizable-panel-group"
            "data-panel-group-direction": direction_attr
            class: c_euv_resizable_group()
            class: direction_class()
            children
        }
    }
}

/// Holds one panel of a resizable group.
///
/// The panel takes the share of the group the caller gave it, so the
/// handle beside it and the panel itself always agree.
///
/// # Arguments
///
/// - `VirtualNode<EuvResizablePanelProps>` - The props node carrying the
///   share the panel takes.
///
/// # Returns
///
/// - `VirtualNode` - The euv resizable panel virtual DOM tree.
#[component]
pub fn euv_resizable_panel(node: VirtualNode<EuvResizablePanelProps>) -> VirtualNode {
    let EuvResizablePanelProps { size }: EuvResizablePanelProps =
        node.try_get_props().unwrap_or_default();
    let current: f64 = size.get();
    let panel_style: String = format!("flex: 0 1 {current}%;");
    let panel_ref: &str = panel_style.as_str();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "resizable-panel"
            style: panel_ref
            class: c_euv_resizable_panel()
            children
        }
    }
}

/// Moves the divider between two panels of a resizable group.
///
/// The handle is dragged with the mouse: pressing it starts a drag,
/// moving the pointer moves the divider, and releasing the button ends
/// it. The drag is measured against the group the handle sits in, so
/// the divider follows the pointer rather than drifting by a fixed
/// number of pixels.
///
/// # Arguments
///
/// - `VirtualNode<EuvResizableHandleProps>` - The props node carrying
///   the share the handle moves, the bounds of that share and the
///   direction of the group.
///
/// # Returns
///
/// - `VirtualNode` - The euv resizable handle virtual DOM tree.
#[component]
pub fn euv_resizable_handle(node: VirtualNode<EuvResizableHandleProps>) -> VirtualNode {
    let EuvResizableHandleProps {
        size,
        min,
        max,
        direction,
    }: EuvResizableHandleProps = node.try_get_props().unwrap_or_default();
    let direction_attr: &str = match direction {
        EuvResizableDirection::Horizontal => RESIZABLE_DIRECTION_HORIZONTAL,
        EuvResizableDirection::Vertical => RESIZABLE_DIRECTION_VERTICAL,
    };
    let direction_class: fn() -> &'static Css = match direction {
        EuvResizableDirection::Horizontal => c_euv_resizable_handle_horizontal,
        EuvResizableDirection::Vertical => c_euv_resizable_handle_vertical,
    };
    let current_size: f64 = size.get();
    let aria_value_now: String = current_size.to_string();
    let aria_value_min: String = min.to_string();
    let aria_value_max: String = max.to_string();
    let session: Rc<RefCell<Option<ResizableDragSession>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let on_keydown: Option<Rc<dyn Fn(Event)>> = on_resizable_keydown(size, min, max, direction);
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "resizable-handle"
            role: "separator"
            tabindex: "0"
            "aria-label": RESIZABLE_SEPARATOR_LABEL
            "aria-orientation": direction_attr
            "aria-valuenow": aria_value_now
            "aria-valuemin": aria_value_min
            "aria-valuemax": aria_value_max
            class: c_euv_resizable_handle()
            class: direction_class()
            onmousedown: on_resizable_drag(size, min, max, direction, session.clone())
            onkeydown: on_keydown
            children
        }
    }
}

/// Builds the handler that drags the divider of a resizable group.
///
/// The drag is measured against the group the handle sits in, so the
/// divider follows the pointer rather than drifting by a fixed number
/// of pixels. The share is clamped to the bounds the caller declared.
///
/// # Arguments
///
/// - `Signal<f64>` - The share of the panel, updated during the drag.
/// - `f64` - The smallest share the panel may take.
/// - `f64` - The largest share the panel may take.
/// - `EuvResizableDirection` - The direction of the group.
/// - `Rc<RefCell<Option<ResizableDragSession>>>` - The slot the live drag
///   handlers are held in.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press handler.
fn on_resizable_drag(
    size: Signal<f64>,
    min: f64,
    max: f64,
    direction: EuvResizableDirection,
    session: Rc<RefCell<Option<ResizableDragSession>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    #[cfg(target_arch = "wasm32")]
    {
        Some(Rc::new(move |event: Event| {
            let Some(document) = browser_document() else {
                return;
            };
            let Some(target) = event.target() else {
                return;
            };
            let Ok(element) = target.dyn_into::<Element>() else {
                return;
            };
            // The handle sits beside the panels it divides, so its
            // parent is the group the drag is measured against.
            let Some(group) = element.parent_element() else {
                return;
            };
            let Ok(mouse) = event.dyn_into::<MouseEvent>() else {
                return;
            };
            let rect: DomRect = group.get_bounding_client_rect();
            let extent: f64 = match direction {
                EuvResizableDirection::Horizontal => rect.width(),
                EuvResizableDirection::Vertical => rect.height(),
            };
            if extent <= 0.0 {
                return;
            }
            let start: f64 = match direction {
                EuvResizableDirection::Horizontal => mouse.client_x() as f64,
                EuvResizableDirection::Vertical => mouse.client_y() as f64,
            };
            let start_size: f64 = size.get();
            let move_size: Signal<f64> = size;
            let on_move: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let Ok(mouse) = event.dyn_into::<MouseEvent>() else {
                        return;
                    };
                    let current: f64 = match direction {
                        EuvResizableDirection::Horizontal => mouse.client_x() as f64,
                        EuvResizableDirection::Vertical => mouse.client_y() as f64,
                    };
                    let delta: f64 = current - start;
                    let next: f64 = start_size + (delta / extent) * 100.0;
                    move_size.set(next.clamp(min, max));
                }));
            // The end handler reads the live handlers back out of the
            // slot rather than capturing them, so it can take them off
            // the document without holding a reference to itself.
            let weak_session: Weak<RefCell<Option<ResizableDragSession>>> = Rc::downgrade(&session);
            let closing_document: Document = document.clone();
            let on_up: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_event: Event| {
                let Some(slot) = weak_session.upgrade() else {
                    return;
                };
                let taken: Option<ResizableDragSession> = slot.borrow_mut().take();
                let Some(drag) = taken else {
                    return;
                };
                let _: Result<(), JsValue> = closing_document.remove_event_listener_with_callback(
                    EVENT_MOUSEMOVE,
                    drag.on_move.as_ref().unchecked_ref(),
                );
                let _: Result<(), JsValue> = closing_document.remove_event_listener_with_callback(
                    EVENT_MOUSEUP,
                    drag.on_up.as_ref().unchecked_ref(),
                );
            }));
            if document
                .add_event_listener_with_callback(EVENT_MOUSEMOVE, on_move.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            if document
                .add_event_listener_with_callback(EVENT_MOUSEUP, on_up.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            *session.borrow_mut() = Some(ResizableDragSession { on_move, on_up });
        }))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: (Signal<f64>, f64, f64, EuvResizableDirection, _) =
            (size, min, max, direction, session);
        None
    }
}

/// Builds the keyboard handler that adjusts a resizable separator.
///
/// Arrow keys move the separator in its orientation; Home and End move to
/// the declared bounds. Holding Shift uses the larger adjustment step.
///
/// # Arguments
///
/// - `Signal<f64>` - The panel share the separator updates.
/// - `f64` - The smallest share the panel may take.
/// - `f64` - The largest share the panel may take.
/// - `EuvResizableDirection` - The separator orientation.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard adjustment handler.
fn on_resizable_keydown(
    size: Signal<f64>,
    min: f64,
    max: f64,
    direction: EuvResizableDirection,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        let movement: f64 = match direction {
            EuvResizableDirection::Horizontal if key == RESIZABLE_KEY_ARROW_LEFT => -1.0,
            EuvResizableDirection::Horizontal if key == RESIZABLE_KEY_ARROW_RIGHT => 1.0,
            EuvResizableDirection::Vertical if key == RESIZABLE_KEY_ARROW_UP => -1.0,
            EuvResizableDirection::Vertical if key == RESIZABLE_KEY_ARROW_DOWN => 1.0,
            _ => 0.0,
        };
        let recognized: bool =
            movement != 0.0 || key == RESIZABLE_KEY_HOME || key == RESIZABLE_KEY_END;
        if !recognized {
            return;
        }
        keyboard_event.prevent_default();
        let next: f64 = if key == RESIZABLE_KEY_HOME {
            min
        } else if key == RESIZABLE_KEY_END {
            max
        } else {
            let step: f64 = if keyboard_event.shift_key() {
                RESIZABLE_KEY_MODIFIED_STEP
            } else {
                RESIZABLE_KEY_STEP
            };
            size.get() + (movement * step)
        };
        size.set(next.clamp(min, max));
    }))
}
