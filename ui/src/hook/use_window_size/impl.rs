use super::*;

/// Inherent implementation of [`UseWindowSize`].
impl UseWindowSize {
    /// Creates a handle with the size starting at zero.
    ///
    /// # Returns
    ///
    /// - `UseWindowSize` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            width: Signal::create(0),
            height: Signal::create(0),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current width as a snapshot.
    ///
    /// # Returns
    ///
    /// - `i32` - The viewport width in CSS pixels.
    pub fn current_width(&self) -> i32 {
        self.get_width().get()
    }

    /// Returns the current height as a snapshot.
    ///
    /// # Returns
    ///
    /// - `i32` - The viewport height in CSS pixels.
    pub fn current_height(&self) -> i32 {
        self.get_height().get()
    }

    /// Subscribes to the window's `resize` event and seeds the size
    /// with the browser's current values.
    ///
    /// On a host without a browser this is a no-op and the size stays at
    /// zero.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = browser_window() else {
                return;
            };
            let width: Signal<i32> = self.get_width();
            let height: Signal<i32> = self.get_height();
            let resized: Window = window.clone();
            let on_resize: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    width.set(read_dimension(&resized, true));
                    height.set(read_dimension(&resized, false));
                }));
            if window
                .add_event_listener_with_callback(EVENT_RESIZE, on_resize.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            width.set(read_dimension(&window, true));
            height.set(read_dimension(&window, false));
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
                    EVENT_RESIZE,
                    on_resize.as_ref().unchecked_ref(),
                );
            }));
        }
    }

    /// Removes the window listener.
    pub(crate) fn detach(&self) {
        let taken: Option<Box<dyn Fn()>> = self.get_teardown().borrow_mut().take();
        if let Some(teardown) = taken {
            teardown();
        }
    }
}
