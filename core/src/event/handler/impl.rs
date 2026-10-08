use super::*;

/// Implementation of event handler construction and invocation.
impl NativeEventHandler {
    /// Creates a new event handler from a static event name string and callback.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name (e.g., "click", "input", "hashchange").
    /// - `F` - The callback to invoke when the event fires.
    ///
    /// # Returns
    ///
    /// - `Self` - A new event handler.
    pub fn create<F>(event_name: &'static str, callback: F) -> Self
    where
        F: FnMut(Event) + 'static,
    {
        let callback: SharedEventCallback = Rc::new(UnsafeCell::new(Box::new(callback)));
        Self::new(event_name, callback)
    }

    /// Invokes the underlying callback with the given event.
    ///
    /// # Safety
    ///
    /// Must only be called from the main thread. Guaranteed in WASM
    /// single-threaded context. No concurrent access is possible.
    ///
    /// # Arguments
    ///
    /// - `Event` - The event to pass to the callback.
    pub(crate) fn handle(&self, event: Event) {
        let callback: &mut Box<dyn FnMut(Event)> = unsafe { &mut *self.get_callback().get() };
        callback(event);
    }
}
