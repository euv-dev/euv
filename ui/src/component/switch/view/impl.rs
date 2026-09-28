use super::*;

impl EuvSwitchState {
    /// Creates a state handle for the given checked signal.
    ///
    /// The handle is a thin wrapper: it never owns the signal, it only
    /// gives the view a place to hang [`Self::toggle`] and
    /// [`Self::on_toggle`] without threading bare signals around.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The checked-state signal to wrap.
    ///
    /// # Returns
    ///
    /// - `EuvSwitchState` - The state handle.
    pub fn new(checked: Signal<bool>) -> Self {
        Self { checked }
    }

    /// Flips the checked state of the switch.
    ///
    /// The click handler is a no-op while the switch is disabled, so a
    /// disabled switch can never be flipped by a stray click.
    pub fn toggle(&self) {
        let current: bool = self.get_checked().get();
        self.get_checked().set(!current);
    }

    /// Builds a click handler that toggles the switch.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The disabled signal consulted on every click.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler toggling the
    ///   checked signal, or ignoring the event while disabled.
    pub fn on_toggle(&self, disabled: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        let state: EuvSwitchState = *self;
        Some(Rc::new(move |_: Event| {
            if disabled.get() {
                return;
            }
            state.toggle();
        }))
    }
}
