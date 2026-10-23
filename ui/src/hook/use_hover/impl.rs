use super::*;

/// Inherent implementation of [`UseHover`].
impl UseHover {
    /// Creates a handle with the hover state starting at `false`.
    ///
    /// # Returns
    ///
    /// - `UseHover` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            target: NodeRef::new(),
            hovered: Signal::create(false),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current hover state as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the pointer is over the watched element.
    pub fn is_hovered(&self) -> bool {
        self.get_hovered().get()
    }

    /// Subscribes to the document's `mouseover` and `mouseout` events.
    ///
    /// Moving between two children of the watched element fires
    /// `mouseout` with a related target that is still inside the
    /// element, and that case must not clear the hover state — so the
    /// related target is checked rather than assuming every `mouseout`
    /// means "left the element".
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(document) = browser_document() else {
                return;
            };
            let hovered: Signal<bool> = self.get_hovered();
            let over_target: NodeRef<Element> = self.get_target().clone();
            let on_over: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    if pointer_is_inside(&over_target, &event) {
                        hovered.set(true);
                    }
                }));
            if document
                .add_event_listener_with_callback("mouseover", on_over.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            let out_target: NodeRef<Element> = self.get_target().clone();
            let on_out: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
                let Some(element) = out_target.get_cloned() else {
                    return;
                };
                let related: Option<Node> = event
                    .dyn_into::<MouseEvent>()
                    .ok()
                    .and_then(|mouse: MouseEvent| mouse.related_target())
                    .and_then(|related: EventTarget| related.dyn_into::<Node>().ok());
                if let Some(related) = related
                    && node_contains(&element, &related)
                {
                    return;
                }
                hovered.set(false);
            }));
            if document
                .add_event_listener_with_callback("mouseout", on_out.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = document.remove_event_listener_with_callback(
                    "mouseover",
                    on_over.as_ref().unchecked_ref(),
                );
                let _: Result<(), JsValue> = document.remove_event_listener_with_callback(
                    "mouseout",
                    on_out.as_ref().unchecked_ref(),
                );
            }));
        }
    }

    /// Removes the document listeners.
    pub(crate) fn detach(&self) {
        let taken: Option<Box<dyn Fn()>> = self.get_teardown().borrow_mut().take();
        if let Some(teardown) = taken {
            teardown();
        }
    }
}
