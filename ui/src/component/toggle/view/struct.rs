use super::*;

/// Props for the `euv_toggle` component.
///
/// The pressed state lives in a caller-owned `Signal<bool>` so the control
/// takes part in two-way binding with the rest of the page.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvToggleProps {
    /// The signal holding the pressed state.
    #[get(type(copy))]
    pub pressed: Signal<bool>,
    /// The visual variant.
    #[get(type(copy))]
    pub variant: EuvToggleVariant,
    /// The control size.
    #[get(type(copy))]
    pub size: EuvToggleSize,
    /// The label shown inside the control.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the control is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
    /// Optional callback receiving the new pressed state.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(bool)>>,
}
