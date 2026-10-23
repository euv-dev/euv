use super::*;

/// Inherent implementation of [`UseMediaQuery`].
impl UseMediaQuery {
    /// Creates a handle for `query` with the match result starting at
    /// `false`.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The media query text.
    ///
    /// # Returns
    ///
    /// - `UseMediaQuery` - The fresh handle.
    pub(crate) fn new(query: &'static str) -> Self {
        Self {
            matches: Signal::create(false),
            query,
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current match result as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the query currently matches.
    pub fn is_matching(&self) -> bool {
        self.get_matches().get()
    }

    /// Subscribes to the query's `change` event and seeds the match
    /// result with the browser's current answer.
    ///
    /// On a host without a browser this is a no-op and the match result
    /// stays `false`, so a component that reads the signal renders the
    /// same way in `cargo test` as it does in a browser that has no
    /// matching device.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = window() else {
                return;
            };
            let Ok(Some(list)) = window.match_media(self.get_query()) else {
                return;
            };
            let matches: Signal<bool> = self.get_matches();
            matches.set(list.matches());
            let watched: MediaQueryList = list.clone();
            let closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    matches.set(watched.matches());
                }));
            let Ok(()) = list.add_event_listener_with_callback(
                EVENT_MEDIA_QUERY_CHANGE,
                closure.as_ref().unchecked_ref(),
            ) else {
                return;
            };
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = list.remove_event_listener_with_callback(
                    EVENT_MEDIA_QUERY_CHANGE,
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
