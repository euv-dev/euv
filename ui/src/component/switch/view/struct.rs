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
