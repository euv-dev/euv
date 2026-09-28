use super::*;

/// Props for the `euv_switch` component.
///
/// Defines the strongly-typed interface for a toggle switch. The checked
/// state is owned by the caller through a `Signal<bool>` so the switch can
/// participate in two-way binding with the rest of the page.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvSwitchProps {
    /// The unique identifier for the switch element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The HTML name attribute shared by switches in the same form group.
    #[get(type(copy))]
    pub name: &'static str,
    /// The boolean signal bound to the switch checked state.
    #[get(type(copy))]
    pub checked: Signal<bool>,
    /// The label text displayed next to the switch.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the switch is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
}

/// The reactive state handle for the `euv_switch` component.
///
/// Registered against the current hook context slot via
/// [`HookContext::use_hook`], so every render at the same hook index
/// observes the same `Signal<bool>` and the switch keeps its state
/// across re-renders.
#[derive(Clone, Copy, CustomDebug, Data, Default)]
pub struct EuvSwitchState {
    /// The signal holding the switch checked state.
    #[get(type(copy))]
    pub checked: Signal<bool>,
}

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

/// Obtains the switch state handle registered against the current hook
/// context slot.
///
/// Behaves like [`HookContext::use_hook`] — the same handle is returned
/// on every render at the same hook index, so the captured signal
/// survives re-renders. The factory is used directly when no hook
/// context is active (e.g. when called outside a render cycle).
///
/// # Arguments
///
/// - `Signal<bool>` - The checked-state signal owned by the caller.
///
/// # Returns
///
/// - `EuvSwitchState` - The switch state handle.
pub fn use_euv_switch_state(checked: Signal<bool>) -> EuvSwitchState {
    HookContext::use_hook(move || EuvSwitchState::new(checked))
}
