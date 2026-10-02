use super::*;

/// One selectable entry of the `euv_radio_group`.
///
/// Carries the machine value submitted with the form and the label the
/// user reads; the group compares the machine value against its own
/// `value` signal to decide which item renders as checked.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvRadioOption {
    /// The machine value submitted when this option is selected.
    #[get(type(copy))]
    pub value: &'static str,
    /// The display label shown next to the radio control.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props for the `euv_radio` component.
///
/// Defines the strongly-typed interface for a radio group. The selection
/// is owned by the caller through a `Signal<String>` so the group can
/// take part in two-way binding with the rest of the page.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvRadioGroupProps {
    /// The HTML name attribute shared by every radio in the group.
    #[get(type(copy))]
    pub name: &'static str,
    /// The selectable options rendered in declaration order.
    pub options: Vec<EuvRadioOption>,
    /// The signal holding the currently selected option value.
    #[get(type(copy))]
    pub value: Signal<String>,
    /// The group label text displayed above the options.
    #[get(type(copy))]
    pub label: &'static str,
}
