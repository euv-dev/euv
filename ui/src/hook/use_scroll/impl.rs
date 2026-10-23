use super::*;

/// Inherent implementation of [`UseScroll`].
impl UseScroll {
    /// Creates a handle with both offsets starting at zero.
    ///
    /// # Returns
    ///
    /// - `UseScroll` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            x: Signal::create(0.0),
            y: Signal::create(0.0),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the vertical offset as a snapshot.
    ///
    /// # Returns
    ///
    /// - `f64` - The vertical scroll offset in CSS pixels.
    pub fn current_y(&self) -> f64 {
        self.get_y().get()
    }

    /// Returns the horizontal offset as a snapshot.
    ///
    /// # Returns
    ///
    /// - `f64` - The horizontal scroll offset in CSS pixels.
    pub fn current_x(&self) -> f64 {
        self.get_x().get()
    }

    /// Scrolls the page back to the top.
    ///
    /// The scroll is smooth when the browser supports it, so the
    /// reader sees the page travel rather than jump.
    ///
    /// On a host without a browser this is a no-op.
    pub fn scroll_to_top(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = browser_window() else {
                return;
            };
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    }

    /// Subscribes to the window's `scroll` event and seeds the offsets
    /// with the browser's current values.
    ///
    /// On a host without a browser this is a no-op and the offsets stay
    /// at zero.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = browser_window() else {
                return;
            };
            let x: Signal<f64> = self.get_x();
            let y: Signal<f64> = self.get_y();
            let scrolled: Window = window.clone();
            let on_scroll: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |_event: Event| {
                    x.set(scrolled.scroll_x().unwrap_or(0.0));
                    y.set(scrolled.scroll_y().unwrap_or(0.0));
                }));
            if window
                .add_event_listener_with_callback(EVENT_SCROLL, on_scroll.as_ref().unchecked_ref())
                .is_err()
            {
                return;
            }
            x.set(window.scroll_x().unwrap_or(0.0));
            y.set(window.scroll_y().unwrap_or(0.0));
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                let _: Result<(), JsValue> = window.remove_event_listener_with_callback(
                    EVENT_SCROLL,
                    on_scroll.as_ref().unchecked_ref(),
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
