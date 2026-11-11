use super::*;

/// Props for the `euv_checkbox` component.
///
/// Defines the strongly-typed interface for a labeled checkbox.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvCheckboxProps {
    /// The unique identifier for the checkbox element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The HTML name attribute for the checkbox element.
    #[get(type(copy))]
    pub name: &'static str,
    /// The autocomplete hint for the browser.
    #[get(type(copy))]
    pub autocomplete: &'static str,
    /// The boolean signal bound to the checkbox checked state.
    #[get(type(copy))]
    pub checked: Signal<bool>,
    /// Whether the checkbox is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
    /// The label text displayed next to the checkbox.
    #[get(type(copy))]
    pub label: &'static str,
}
