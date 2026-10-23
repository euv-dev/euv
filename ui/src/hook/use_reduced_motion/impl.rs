use super::*;

/// Inherent implementation of [`UseReducedMotion`].
impl UseReducedMotion {
    /// Creates a handle whose answer starts at `false`.
    ///
    /// # Returns
    ///
    /// - `UseReducedMotion` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            reduced: Signal::create(false),
            query: REDUCED_MOTION_QUERY,
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current answer as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the user asked for reduced motion.
    pub fn is_reduced(&self) -> bool {
        self.get_reduced().get()
    }

    /// Subscribes to the query's `change` event and seeds the answer
    /// with the browser's current one.
    ///
    /// On a host without a browser this is a no-op and the answer stays
    /// `false`.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = window() else {
                return;
            };
            let Ok(Some(list)) = window.match_media(self.get_query()) else {
                return;
            };
            let reduced: Signal<bool> = self.get_reduced();
            reduced.set(list.matches());
            let watched: MediaQueryList = list.clone();
            let closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    reduced.set(watched.matches());
                }));
            let Ok(()) = list.add_event_listener_with_callback(
                EVENT_REDUCED_MOTION_CHANGE,
                closure.as_ref().unchecked_ref(),
            ) else {
                return;
            };
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = list.remove_event_listener_with_callback(
                    EVENT_REDUCED_MOTION_CHANGE,
                    closure.as_ref().unchecked_ref(),
                );
            }));
        }
    }

    /// Removes the `change` listener.
    pub(crate) fn detach(&self) {
        let taken: Option<Box<dyn Fn()>> = self.get_teardown().borrow_mut().take();
        if let Some(teardown) = taken {
            teardown();
        }
    }
}
