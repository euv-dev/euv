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
    /// Whether a separator appears immediately before this option.
    #[get(type(copy))]
    pub separator_before: bool,
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
#[derive(Clone, CustomDebug)]
pub struct EuvSelectProps {
    /// The signal holding the currently selected option value.
    pub value: Signal<String>,
    /// The options rendered in the panel, in display order.
    pub options: Vec<EuvSelectOption>,
    /// The text shown in the trigger while no option is selected.
    pub placeholder: &'static str,
    /// The group label above the trigger; an empty label renders no label.
    pub label: &'static str,
    /// The trigger id used to connect an external label to the control.
    /// An empty id uses the generated Select id.
    pub id: &'static str,
    /// The form field name submitted by the hidden native control.
    pub name: &'static str,
    /// Whether the native form control is required.
    pub required: bool,
    /// Whether the control is invalid; the root and trigger expose that state
    /// for field styling and assistive technology.
    pub invalid: bool,
    /// Whether the whole control is disabled.
    pub disabled: bool,
    /// Whether the selected item aligns with the trigger while the panel is open.
    ///
    /// Defaults to `true` with the shadcn scheme and `false` with other schemes.
    pub align_item_with_trigger: bool,
    /// Optional callback receiving the value of the picked option.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
}

/// The reactive state shared by each rendered Select option row.
///
/// Keeping the row's handlers together avoids duplicating the selection and
/// panel signals across the helper's argument list.
#[derive(Clone, CustomDebug)]
pub(crate) struct EuvSelectOptionContext {
    /// The selection signal written when a row is picked.
    pub value: Signal<String>,
    /// The available options used for keyboard text navigation.
    pub options: Rc<Vec<EuvSelectOption>>,
    /// The open-state signal used to close the option panel.
    pub open: Signal<bool>,
    /// Whether the selected option should align with the trigger.
    pub align_item_with_trigger: bool,
    /// The CSS translation signal used by item alignment.
    pub align_offset: Signal<String>,
    /// Whether the panel can scroll toward earlier options.
    pub scroll_up_visible: Signal<bool>,
    /// Whether the panel can scroll down.
    pub scroll_down_visible: Signal<bool>,
    /// The current keyboard typeahead prefix.
    pub typeahead_buffer: Signal<String>,
    /// The generation that invalidates older typeahead reset timers.
    pub typeahead_generation: Signal<u32>,
    /// The optional callback invoked when a row is picked.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props for the [`euv_select_scroll_up_button`] component.
///
/// Displays a scroll-up indicator for a long option list. The Select panel's
/// native viewport remains responsible for scrolling.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSelectScrollUpButtonProps {}

/// Props for the [`euv_select_scroll_down_button`] component.
///
/// Displays a scroll-down indicator for a long option list. The Select panel's
/// native viewport remains responsible for scrolling.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSelectScrollDownButtonProps {}
