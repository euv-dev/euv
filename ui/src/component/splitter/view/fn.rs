use super::*;

/// A split view with a draggable divider.
///
/// Renders a `c_euv_splitter` row holding the first pane, the
/// `c_euv_splitter_bar` divider, and the second pane. The first pane's
/// width is driven by the `size` signal, so a drag updates one number
/// and the layout follows.
///
/// The divider is dragged with the mouse: pressing it starts a drag,
/// moving the pointer resizes the first pane, and releasing the button
/// ends it. The size is clamped to the `min` and `max` the caller
/// declared, and the drag is measured against the container's width so
/// the pane follows the pointer exactly.
///
/// # Arguments
///
/// - `VirtualNode<EuvSplitterProps>` - The props node containing the
///   split view configuration.
///
/// # Returns
///
/// - `VirtualNode` - The split view virtual DOM tree.
#[component]
pub fn euv_splitter(node: VirtualNode<EuvSplitterProps>) -> VirtualNode {
    let EuvSplitterProps { size, min, max }: EuvSplitterProps =
        node.try_get_props().unwrap_or_default();
    let children: Vec<VirtualNode> = node.get_children().to_vec();
    let first: VirtualNode = children.first().cloned().unwrap_or(VirtualNode::Empty);
    let rest: Vec<VirtualNode> = children.into_iter().skip(1).collect();
    let container_ref: NodeRef<Element> = App::use_node_ref();
    let session: Rc<RefCell<Option<DragSession>>> =
        HookContext::use_hook(|| Rc::new(RefCell::new(None)));
    let current: f64 = size.get().clamp(min, max);
    let pane_style: String = format!("flex: 0 0 {current}%;");
    let pane_ref: &str = pane_style.as_str();
    // The node ref is cloned before the tree is built: the `ref:`
    // attribute takes it by value, and the drag handler needs its own
    // handle to measure the container against.
    let measured_ref: NodeRef<Element> = container_ref.clone();
    App::use_cleanup({
        let session: Rc<RefCell<Option<DragSession>>> = session.clone();
        move || {
            session.borrow_mut().take();
        }
    });
    html! {
        div {
            class: c_euv_splitter()
            ref: container_ref
            div {
                class: c_euv_splitter_pane()
                style: pane_ref
                first
            }
            div {
                class: c_euv_splitter_bar()
                onmousedown: on_bar_mousedown(
                size,
                min,
                max,
                measured_ref,
                session.clone()
                )
            }
            div {
                class: c_euv_splitter_pane()
                rest
            }
        }
    }
}

/// Builds the handler that starts a drag on the divider.
///
/// The drag is measured against the container's width, so the pane
/// follows the pointer rather than drifting by a fixed number of
/// pixels.
///
/// # Arguments
///
/// - `Signal<f64>` - The size signal updated during the drag.
/// - `f64` - The smallest share the first pane may take.
/// - `NodeRef<Element>` - The container the drag is measured against.
/// - `Rc<RefCell<Option<DragSession>>>` - The slot the live drag
///   handlers are held in.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press handler.
fn on_bar_mousedown(
    size: Signal<f64>,
    min: f64,
    max: f64,
    container: NodeRef<Element>,
    session: Rc<RefCell<Option<DragSession>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    #[cfg(target_arch = "wasm32")]
    {
        Some(Rc::new(move |event: Event| {
            let Ok(mouse) = event.dyn_into::<MouseEvent>() else {
                return;
            };
            let Some(document) = browser_document() else {
                return;
            };
            let Some(element) = container.get_cloned() else {
                return;
            };
            let width: f64 = element.get_bounding_client_rect().width();
            if width <= 0.0 {
                return;
            }
            let start_x: f64 = mouse.client_x() as f64;
            let start_size: f64 = size.get();
            let move_size: Signal<f64> = size;
            let on_move: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let Ok(mouse) = event.dyn_into::<MouseEvent>() else {
                        return;
                    };
                    let delta: f64 = mouse.client_x() as f64 - start_x;
                    let next: f64 = start_size + (delta / width) * 100.0;
                    move_size.set(next.clamp(min, max));
                }));
            // The end handler reads the live handlers back out of the
            // slot rather than capturing them, so it can take them off
            // the document without holding a reference to itself. The
            // slot is reached through a weak pointer, so the handler
            // does not keep the slot — and through it the handler
            // itself — alive.
            let weak_session: Weak<RefCell<Option<DragSession>>> = Rc::downgrade(&session);
            let closing_document: Document = document.clone();
            let on_up: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_event: Event| {
                let Some(slot) = weak_session.upgrade() else {
                    return;
                };
                let taken: Option<DragSession> = slot.borrow_mut().take();
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
            *session.borrow_mut() = Some(DragSession { on_move, on_up });
        }))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: (Signal<f64>, f64, f64, NodeRef<_>, _) = (size, min, max, container, session);
        None
    }
}
