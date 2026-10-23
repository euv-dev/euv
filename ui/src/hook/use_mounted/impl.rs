use super::*;

/// Inherent implementation of [`UseMounted`].
impl UseMounted {
    /// Creates a handle with the flag starting at `false`.
    ///
    /// # Returns
    ///
    /// - `UseMounted` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            mounted: Signal::create(false),
        }
    }

    /// Returns the current flag as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` once the DOM has been committed.
    pub fn is_mounted(&self) -> bool {
        self.get_mounted().get()
    }

    /// Raises the flag once the DOM has been committed.
    ///
    /// The wait is a macrotask rather than a microtask on purpose: the
    /// renderer commits a signal-driven update in a microtask, so a
    /// microtask scheduled from inside a render could run before that
    /// commit and would see a DOM that is not there yet.
    pub(crate) fn attach(&self) {
        let mounted: Signal<bool> = self.get_mounted();
        after_mount(move || {
            mounted.set(true);
        });
    }
}
