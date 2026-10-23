use super::*;

/// Inherent implementation of [`UseKeyboard`].
impl UseKeyboard {
    /// Creates a handle with no key held down.
    ///
    /// # Returns
    ///
    /// - `UseKeyboard` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            pressed: Signal::create(String::new()),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the key currently held down as a snapshot.
    ///
    /// # Returns
    ///
    /// - `String` - The key, or an empty string when no key is down.
    pub fn current_key(&self) -> String {
        self.get_pressed().get()
    }

    /// Reports whether `key` is the key currently held down.
    ///
    /// # Arguments
    ///
    /// - `&str` - The key to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when that key is down.
    pub fn is_pressed(&self, key: &str) -> bool {
        self.get_pressed().get() == key
    }

    /// Subscribes to the window's `keydown` and `keyup` events.
    ///
    /// On a host without a browser this is a no-op and the key stays
    /// empty.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = browser_window() else {
                return;
            };
            let pressed: Signal<String> = self.get_pressed();
            let on_down: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let Ok(keyboard) = event.dyn_into::<KeyboardEvent>() else {
                        return;
                    };
                    pressed.set(keyboard.key());
                }));
            let on_up: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
                let Ok(keyboard) = event.dyn_into::<KeyboardEvent>() else {
                    return;
                };
                let released: String = keyboard.key();
                if pressed.get() == released {
                    pressed.set(String::new());
                }
            }));
            if window
                .add_event_listener_with_callback(EVENT_KEYDOWN, on_down.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            if window
                .add_event_listener_with_callback(EVENT_KEYUP, on_up.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
                    EVENT_KEYDOWN,
                    on_down.as_ref().unchecked_ref(),
                );
                let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
                    EVENT_KEYUP,
                    on_up.as_ref().unchecked_ref(),
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
