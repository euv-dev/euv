use super::*;

/// Inherent implementation of [`UseClickOutside`].
impl UseClickOutside {
    /// Creates a handle with the trigger starting at `false`.
    ///
    /// # Returns
    ///
    /// - `UseClickOutside` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            target: NodeRef::new(),
            triggered: Signal::create(false),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current trigger state as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the last press landed outside.
    pub fn is_triggered(&self) -> bool {
        self.get_triggered().get()
    }

    /// Clears the trigger, so the next outside press is a new event.
    pub fn reset(&self) {
        self.get_triggered().set(false);
    }

    /// Subscribes to the document's `mousedown` and `touchstart`
    /// events.
    ///
    /// A press inside the watched element is ignored. A press outside
    /// raises the trigger.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(document) = browser_document() else {
                return;
            };
            let triggered: Signal<bool> = self.get_triggered();
            let press_target: NodeRef<Element> = self.get_target().clone();
            let on_press: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    if press_is_outside(&press_target, &event) {
                        triggered.set(true);
                    }
                }));
            if document
                .add_event_listener_with_callback(
                    EVENT_MOUSEDOWN,
                    on_press.as_ref().unchecked_ref(),
                )
                .is_err()
            {
                return;
            }
            let touch_target: NodeRef<Element> = self.get_target().clone();
            let on_touch: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    if press_is_outside(&touch_target, &event) {
                        triggered.set(true);
                    }
                }));
            if document
                .add_event_listener_with_callback(
                    EVENT_TOUCHSTART,
                    on_touch.as_ref().unchecked_ref(),
                )
                .is_err()
            {
                return;
            }
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = document.remove_event_listener_with_callback(
                    EVENT_MOUSEDOWN,
                    on_press.as_ref().unchecked_ref(),
                );
                let _: Result<(), JsValue> = document.remove_event_listener_with_callback(
                    EVENT_TOUCHSTART,
                    on_touch.as_ref().unchecked_ref(),
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
