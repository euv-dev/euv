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
