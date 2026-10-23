use super::*;

/// One entry of the [`euv_select`] option list.
///
/// Carries the machine value written into the selection signal, the label
/// the user reads, the caption of the group the option belongs to, and
/// whether the option can be picked at all.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvSelectOption {
    /// The machine value written into the selection signal when picked.
    #[get(type(copy))]
    pub value: &'static str,
    /// The display label shown for this option.
    #[get(type(copy))]
    pub label: &'static str,
    /// The caption of the group this option belongs to.
    ///
    /// An empty caption places the option in the ungrouped block at the
    /// top of the panel, which is rendered without a caption row.
    #[get(type(copy))]
    pub group: &'static str,
    /// Whether the option is disabled.
    ///
    /// A disabled option is drawn with the muted treatment and cannot be
    /// picked, so it never writes into the selection signal.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props for the [`euv_select`] component.
///
/// The selection lives in a caller-owned `Signal<String>` so the select
/// takes part in two-way binding with the rest of the page. The open state
/// of the option panel is owned by the component itself, so the caller
/// never has to wire it up.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSelectProps {
    /// The signal holding the currently selected option value.
    pub value: Signal<String>,
    /// The options rendered in the panel, in display order.
    pub options: Vec<EuvSelectOption>,
    /// The text shown in the trigger while no option is selected.
    pub placeholder: &'static str,
    /// The group label above the trigger; an empty label renders no label.
    pub label: &'static str,
    /// Whether the whole control is disabled.
    pub disabled: bool,
    /// Optional callback receiving the value of the picked option.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
}
