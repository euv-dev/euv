use super::*;

/// One suggestion offered by the [`euv_auto_complete`] component.
///
/// The `value` is what lands in the field when the suggestion is picked,
/// while the `label` is what the user reads in the suggestion list.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvAutoCompleteOption {
    /// The value written into the field when the option is picked.
    #[get(type(copy))]
    pub value: &'static str,
    /// The text shown in the suggestion list.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props for the [`euv_auto_complete`] component.
///
/// The text in the field lives in a caller-owned [`Signal`] so the component
/// takes part in two-way binding; the open state of the suggestion panel is
/// owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAutoCompleteProps {
    /// The suggestions offered to the user.
    pub options: Vec<EuvAutoCompleteOption>,
    /// The text currently held by the field.
    pub value: Signal<String>,
    /// The placeholder shown while the field is empty.
    pub placeholder: &'static str,
    /// The label rendered above the field.
    ///
    /// An empty string renders the field without a label.
    pub label: &'static str,
    /// The id used to associate the label with the input.
    ///
    /// An empty string uses the component-generated id.
    pub id: &'static str,
    /// The form field name submitted with the input value.
    pub name: &'static str,
    /// The id of the owning form; an empty value uses the nearest ancestor form.
    pub form: &'static str,
    /// The browser autofill hint; an empty value enables the browser default.
    pub autocomplete: &'static str,
    /// Whether the input participates in required form validation.
    pub required: bool,
    /// Whether the input is currently invalid.
    pub invalid: bool,
    /// Whether the input and suggestion list are disabled.
    pub disabled: bool,
    /// Whether the input text is read-only while suggestions remain selectable.
    pub read_only: bool,
    /// The accessible name used when no visible label is rendered.
    pub aria_label: &'static str,
    /// Space-separated ids naming the input's descriptive text.
    pub aria_describedby: &'static str,
    /// The callback invoked with the typed text after every change.
    ///
    /// The text is handed over as a `'static` slice, so the component
    /// leaks one small string per distinct value the user types.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
    /// The callback invoked with the value of the picked suggestion.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Render-time attributes and handlers for the native AutoComplete input.
pub(crate) struct EuvAutoCompleteInputProps {
    /// The native input id.
    pub(crate) id: String,
    /// The form field name.
    pub(crate) name: &'static str,
    /// The optional external form id.
    pub(crate) form: &'static str,
    /// The browser autocomplete hint.
    pub(crate) autocomplete: &'static str,
    /// The input's accessible name.
    pub(crate) aria_label: &'static str,
    /// The input's descriptive ID references.
    pub(crate) aria_describedby: &'static str,
    /// Whether the input is invalid.
    pub(crate) invalid: bool,
    /// Whether the input is required.
    pub(crate) required: bool,
    /// Whether the input is disabled.
    pub(crate) disabled: bool,
    /// Whether the input is read-only.
    pub(crate) read_only: bool,
    /// Whether the suggestion panel is open.
    pub(crate) panel_open: bool,
    /// The id of the controlled suggestion list.
    pub(crate) listbox_id: String,
    /// The currently active suggestion id.
    pub(crate) active_descendant_id: String,
    /// The input placeholder.
    pub(crate) placeholder: &'static str,
    /// The caller-owned input value.
    pub(crate) value: Signal<String>,
    /// The base input style.
    pub(crate) input_class: Css,
    /// The compile-selected default input geometry.
    pub(crate) default_class: Css,
    /// The invalid-state input style.
    pub(crate) invalid_class: Css,
    /// The disabled-state input style.
    pub(crate) disabled_class: Css,
    /// The input handler.
    pub(crate) on_input: Option<Rc<dyn Fn(Event)>>,
    /// The keyboard handler.
    pub(crate) on_keydown: Option<Rc<dyn Fn(Event)>>,
}
