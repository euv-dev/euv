use super::*;

/// Restores a signal-backed control after its owning form completes a reset.
///
/// The callback runs after the browser's native reset action, unless another
/// listener cancels the reset event. The element id resolves ancestor form
/// ownership; `form_id` resolves an externally associated form.
///
/// # Arguments
///
/// - `String` - The id of the control or group inside the form.
/// - `&'static str` - The external form id, or an empty string for ancestor association.
/// - `Rc<dyn Fn()>` - The callback that restores the control's initial state.
pub(crate) fn use_form_reset(element_id: String, form_id: &'static str, on_reset: Rc<dyn Fn()>) {
    #[cfg(target_arch = "wasm32")]
    {
        let listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> =
            HookContext::use_hook(|| Rc::new(RefCell::new(None)));
        let mounted: Rc<Cell<bool>> = HookContext::use_hook(|| Rc::new(Cell::new(true)));
        let document: Option<Document> = window().and_then(|window: Window| window.document());
        if listener.borrow().is_none() {
            if let Some(callback_document) = document.clone() {
                let event_document: Document = callback_document.clone();
                let callback_element_id: String = element_id.clone();
                let callback_form_id: String = String::from(form_id);
                let callback_on_reset: Rc<dyn Fn()> = on_reset.clone();
                let callback_mounted: Rc<Cell<bool>> = mounted.clone();
                let reset_listener: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        let Some(reset_form): Option<Element> = event
                            .target()
                            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
                        else {
                            return;
                        };
                        let owns_control: bool = if callback_form_id.is_empty() {
                            event_document
                                .get_element_by_id(&callback_element_id)
                                .and_then(|control: Element| {
                                    control.closest(FORM_RESET_FORM_SELECTOR).ok().flatten()
                                })
                                .is_some_and(|owner_form: Element| {
                                    let owner_node: Node = owner_form.into();
                                    let reset_node: Node = reset_form.clone().into();
                                    owner_node.is_same_node(Some(&reset_node))
                                })
                        } else {
                            reset_form.id() == callback_form_id
                        };
                        if !owns_control {
                            return;
                        }
                        let reset_event: Event = event.clone();
                        let reset_callback: Rc<dyn Fn()> = callback_on_reset.clone();
                        let reset_mounted: Rc<Cell<bool>> = callback_mounted.clone();
                        let Some(window) = window() else {
                            return;
                        };
                        let callback: Function = Closure::once_into_js(move || {
                            if reset_mounted.get() && !reset_event.default_prevented() {
                                reset_callback();
                            }
                        })
                        .unchecked_into();
                        let _: Result<i32, JsValue> = window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(&callback, 0);
                    }) as Box<dyn FnMut(Event)>);
                if callback_document
                    .add_event_listener_with_callback(
                        FORM_RESET_EVENT_NAME,
                        reset_listener.as_ref().unchecked_ref(),
                    )
                    .is_ok()
                {
                    *listener.borrow_mut() = Some(reset_listener);
                }
            }
        }
        App::use_cleanup({
            let cleanup_document: Option<Document> = document;
            let cleanup_listener: Rc<RefCell<Option<Closure<dyn FnMut(Event)>>>> = listener;
            let cleanup_mounted: Rc<Cell<bool>> = mounted;
            move || {
                cleanup_mounted.set(false);
                let callback: Option<Closure<dyn FnMut(Event)>> =
                    cleanup_listener.borrow_mut().take();
                if let (Some(document), Some(callback)) = (cleanup_document, callback) {
                    let _: Result<(), JsValue> = document.remove_event_listener_with_callback(
                        FORM_RESET_EVENT_NAME,
                        callback.as_ref().unchecked_ref(),
                    );
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: (String, &'static str, Rc<dyn Fn()>) = (element_id, form_id, on_reset);
    }
}
