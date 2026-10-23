use super::*;

/// One entry of the [`euv_checkbox_group`] option list.
///
/// Carries the machine value stored in the checked-values signal, the label
/// the user reads, and whether the option can be toggled at all.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvCheckboxGroupOption {
    /// The machine value stored in the checked-values signal while ticked.
    #[get(type(copy))]
    pub value: &'static str,
    /// The display label shown next to the checkbox marker.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the option is disabled.
    ///
    /// A disabled option cannot be toggled, so it never writes into the
    /// checked-values signal.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props for the [`euv_checkbox_group`] component.
///
/// The checked values live in a caller-owned `Signal<Vec<String>>` so the
/// group takes part in two-way binding with the rest of the page; the
/// group reads that vector once per render to decide which markers are
/// ticked.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCheckboxGroupProps {
    /// The options rendered in the group, in display order.
    pub options: Vec<EuvCheckboxGroupOption>,
    /// The signal holding the currently checked option values.
    pub values: Signal<Vec<String>>,
    /// The group label above the options; an empty label renders no label.
    pub label: &'static str,
    /// Optional callback receiving the new set of checked values.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(Vec<String>)>>,
}
