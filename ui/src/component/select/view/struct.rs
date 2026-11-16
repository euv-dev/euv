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
/// The selection lives in caller-owned signals: `value` for single mode and
/// `values` when `multiple` is enabled. The panel open state is component-owned.
#[derive(Clone, CustomDebug)]
pub struct EuvSelectProps {
    /// The signal holding the currently selected option value.
    pub value: Signal<String>,
    /// Whether the Select permits more than one selected option.
    pub multiple: bool,
    /// The selected option values used when `multiple` is enabled.
    pub values: Signal<Vec<String>>,
    /// The size of the trigger.
    pub size: EuvSelectSize,
    /// The options rendered in the panel, in display order.
    pub options: Vec<EuvSelectOption>,
    /// The text shown in the trigger while no option is selected.
    pub placeholder: &'static str,
    /// The group label above the trigger; an empty label renders no label.
    pub label: &'static str,
    /// Accessible name for the trigger when no visible label is rendered.
    pub aria_label: String,
    /// Space-separated ids for the trigger's descriptive text and error.
    pub aria_describedby: &'static str,
    /// The trigger id used to connect an external label to the control.
    /// An empty id uses the generated Select id.
    pub id: &'static str,
    /// The form field name submitted by the hidden native control.
    pub name: &'static str,
    /// The id of the form that owns the hidden native control.
    /// An empty value uses the nearest ancestor form.
    pub form: &'static str,
    /// The browser autofill hint applied to the single-value native control.
    /// An empty value leaves the browser's default behavior enabled.
    pub auto_complete: &'static str,
    /// Whether the native form control is required.
    pub required: bool,
    /// Whether the control is invalid; the root and trigger expose that state
    /// for field styling and assistive technology.
    pub invalid: bool,
    /// Whether the whole control is disabled.
    pub disabled: bool,
    /// Whether the user can inspect options without changing the value.
    pub read_only: bool,
    /// The caller-owned open signal; `None` leaves open state uncontrolled.
    pub open: Option<Signal<bool>>,
    /// The initial open state used when `open` is not provided.
    pub default_open: bool,
    /// Whether the selected item aligns with the trigger while the panel is open.
    ///
    /// Defaults to `true` with the shadcn scheme and `false` with other schemes.
    pub align_item_with_trigger: bool,
    /// Optional callback receiving the value of the picked option.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
    /// Optional callback receiving the full multiple-selection value list.
    #[debug(skip)]
    pub on_change_values: Option<Rc<dyn Fn(Vec<String>)>>,
    /// Optional callback invoked when the Select requests an open-state change.
    #[debug(skip)]
    pub on_open_change: Option<Rc<dyn Fn(bool)>>,
}

/// The native form-control values and behavior consumed by Select rendering.
pub(crate) struct EuvSelectFormInputProps {
    /// The form field name submitted by the native input.
    pub(crate) name: &'static str,
    /// The single selected value.
    pub(crate) value: String,
    /// The selected values for a multiple Select.
    pub(crate) values: Vec<String>,
    /// Whether the Select submits multiple values.
    pub(crate) multiple: bool,
    /// Whether the form control participates in required validation.
    pub(crate) required: bool,
    /// Whether the native control is disabled.
    pub(crate) disabled: bool,
    /// The external form id; empty means ancestor association.
    pub(crate) form: &'static str,
    /// The browser autofill hint; empty means default behavior.
    pub(crate) auto_complete: &'static str,
}

/// The reactive state shared by each rendered Select option row.
///
/// Keeping the row's handlers together avoids duplicating the selection and
/// panel signals across the helper's argument list.
#[derive(Clone, CustomDebug)]
pub(crate) struct EuvSelectOptionContext {
    /// The selection signal written when a row is picked.
    pub value: Signal<String>,
    /// The signal holding all selected values in multiple mode.
    pub values: Signal<Vec<String>>,
    /// Whether option activation toggles membership instead of replacing a value.
    pub multiple: bool,
    /// The available options used for keyboard text navigation.
    pub options: Rc<Vec<EuvSelectOption>>,
    /// The open-state signal used to close the option panel.
    pub open: Signal<bool>,
    /// The callback invoked when the panel requests an open-state change.
    #[debug(skip)]
    pub on_open_change: Option<Rc<dyn Fn(bool)>>,
    /// Whether activating an option leaves the selected value unchanged.
    pub read_only: bool,
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
    /// The optional callback invoked after a multiple-selection change.
    #[debug(skip)]
    pub on_change_values: Option<Rc<dyn Fn(Vec<String>)>>,
}

/// Copyable signal handles used to align and focus the panel after it opens.
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy)]
pub(crate) struct EuvSelectOpenEffect {
    /// The signal containing the trigger id.
    pub(crate) trigger_id: Signal<String>,
    /// The inline style that positions the portaled panel against its trigger.
    pub(crate) panel_position_style: Signal<String>,
    /// The inline maximum height for the panel's scrollable list.
    pub(crate) viewport_position_style: Signal<String>,
    /// The trigger-to-panel gap measured when the panel first opens.
    pub(crate) panel_gap: Signal<f64>,
    /// The controlled or component-owned open state.
    pub(crate) open: Signal<bool>,
    /// The single-selection value.
    pub(crate) value: Signal<String>,
    /// The multiple-selection values.
    pub(crate) values: Signal<Vec<String>>,
    /// Whether the Select accepts multiple values.
    pub(crate) multiple: bool,
    /// Whether the selected item aligns with the trigger.
    pub(crate) align_item_with_trigger: bool,
    /// The computed panel alignment offset.
    pub(crate) align_offset: Signal<String>,
    /// Whether the viewport can scroll toward earlier options.
    pub(crate) scroll_up_visible: Signal<bool>,
    /// Whether the viewport can scroll toward later options.
    pub(crate) scroll_down_visible: Signal<bool>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
pub(crate) struct EuvSelectOpenEffect;

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
