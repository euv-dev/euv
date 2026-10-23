use super::*;

/// Inherent implementation of [`UseOnlineStatus`].
impl UseOnlineStatus {
    /// Creates a handle whose answer starts at `true`.
    ///
    /// # Returns
    ///
    /// - `UseOnlineStatus` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            online: Signal::create(true),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current answer as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the browser believes it is online.
    pub fn is_online(&self) -> bool {
        self.get_online().get()
    }

    /// Seeds the answer from `navigator.onLine` and subscribes to the
    /// window's `online` and `offline` events.
    ///
    /// On a host without a browser the answer stays `true`: a native
    /// build has no connection to lose.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = browser_window() else {
                return;
            };
            let online: Signal<bool> = self.get_online();
            let navigator: Navigator = window.navigator();
            online.set(navigator.on_line());
            let set_online: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    online.set(true);
                }));
            let set_offline: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    online.set(false);
                }));
            if window
                .add_event_listener_with_callback(EVENT_ONLINE, set_online.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            if window
                .add_event_listener_with_callback(
                    EVENT_OFFLINE,
                    set_offline.as_ref().unchecked_ref(),
                )
                .is_err()
            {
                return;
            }
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
                    EVENT_ONLINE,
                    set_online.as_ref().unchecked_ref(),
                );
                let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
                    EVENT_OFFLINE,
                    set_offline.as_ref().unchecked_ref(),
                );
            }));
        }
    }

    /// Removes the window listeners.
    pub(crate) fn detach(&self) {
        let taken: Option<Box<dyn Fn()>> = self.get_teardown().borrow_mut().take();
        if let Some(teardown) = taken {
            teardown();
        }
    }
}
