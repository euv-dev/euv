use super::*;

/// A confirm bubble anchored to its trigger.
///
/// The trigger is the component's children and the `c_euv_popconfirm`
/// wrapper anchors the bubble to it. The bubble body is always mounted:
/// opening and closing is a reactive class swap between
/// `c_euv_popconfirm_body_open` and `c_euv_popconfirm_body_closed`, never
/// a conditional that removes the bubble from the tree. The open state is
/// owned by the component through a hook, so the caller only supplies the
/// question, the two button labels and the two callbacks.
///
/// # Arguments
///
/// - `VirtualNode<EuvPopconfirmProps>` - The props node containing the question and the callbacks.
///
/// # Returns
///
/// - `VirtualNode` - The popconfirm virtual DOM tree.
#[component]
pub fn euv_popconfirm(node: VirtualNode<EuvPopconfirmProps>) -> VirtualNode {
    let EuvPopconfirmProps {
        title,
        ok_label,
        cancel_label,
        on_ok,
        on_cancel,
    }: EuvPopconfirmProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let open: Signal<bool> = HookContext::use_hook(|| Signal::create(false));
    let has_ok: bool = !ok_label.is_empty();
    let has_cancel: bool = !cancel_label.is_empty();
    let toggle_handler: Option<Rc<dyn Fn(Event)>> = popconfirm_toggle_handler(open);
    let ok_handler: Rc<dyn Fn(Event)> = popconfirm_ok_handler(on_ok, open);
    let cancel_handler: Rc<dyn Fn(Event)> = popconfirm_cancel_handler(on_cancel, open);
    html! {
        div {
            class: c_euv_popconfirm()
            span {
                onclick: toggle_handler
                children
            }
            div {
                class: c_euv_popconfirm_body()
                class: if { open } {
                    c_euv_popconfirm_body_open()
                } else {
                    c_euv_popconfirm_body_closed()
                }
                div {
                    class: c_euv_popconfirm_text()
                    title
                }
                div {
                    class: c_euv_popconfirm_actions()
                    if has_ok {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: ok_label
                            onclick: Some(ok_handler)
                        }
                    }
                    if has_cancel {
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            label: cancel_label
                            onclick: Some(cancel_handler)
                        }
                    }
                }
            }
        }
    }
}

/// Builds the click handler that toggles the bubble open and closed.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it
/// with [`Event`].
///
/// # Arguments
///
/// - `Signal<bool>` - The open state of the bubble.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The toggle click handler.
pub fn popconfirm_toggle_handler<E: 'static>(open: Signal<bool>) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_: E| {
        let is_open: bool = open.get();
        open.set(!is_open);
    }))
}

/// Builds the click handler that confirms the action and closes the bubble.
///
/// The callback is fired first, so the caller sees the confirmation
/// before the bubble is taken out of the open state.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn()>>` - The confirm callback, fired when supplied.
/// - `Signal<bool>` - The open state of the bubble.
///
/// # Returns
///
/// - `Rc<dyn Fn(E)>` - The confirm click handler.
pub fn popconfirm_ok_handler<E: 'static>(
    on_ok: Option<Rc<dyn Fn()>>,
    open: Signal<bool>,
) -> Rc<dyn Fn(E)> {
    Rc::new(move |_: E| {
        if let Some(callback) = &on_ok {
            callback();
        }
        open.set(false);
    })
}

/// Builds the click handler that cancels the action and closes the bubble.
///
/// The callback is fired first, so the caller sees the cancellation
/// before the bubble is taken out of the open state.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn()>>` - The cancel callback, fired when supplied.
/// - `Signal<bool>` - The open state of the bubble.
///
/// # Returns
///
/// - `Rc<dyn Fn(E)>` - The cancel click handler.
pub fn popconfirm_cancel_handler<E: 'static>(
    on_cancel: Option<Rc<dyn Fn()>>,
    open: Signal<bool>,
) -> Rc<dyn Fn(E)> {
    Rc::new(move |_: E| {
        if let Some(callback) = &on_cancel {
            callback();
        }
        open.set(false);
    })
}
