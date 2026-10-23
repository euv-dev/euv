use super::*;

/// One entry of the [`euv_segmented`] control.
///
/// Carries the machine value written into the selection signal, the label
/// the user reads, and whether the entry can be picked at all.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvSegmentedOption {
    /// The machine value written into the selection signal when picked.
    #[get(type(copy))]
    pub value: &'static str,
    /// The display label shown on the segment.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the segment is disabled.
    ///
    /// A disabled segment is marked `disabled` and cannot be picked, so it
    /// never writes into the selection signal.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props for the [`euv_segmented`] component.
///
/// The selection lives in a caller-owned `Signal<String>` so the control
/// takes part in two-way binding with the rest of the page.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSegmentedProps {
    /// The options rendered in the control, in display order.
    pub options: Vec<EuvSegmentedOption>,
    /// The signal holding the currently selected option value.
    pub value: Signal<String>,
    /// Optional callback receiving the value of the picked option.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
}
