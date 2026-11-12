use super::*;

/// The reactive state the form demo page drives.
///
/// Every control on the page owns its selection through a signal, so
/// the page can read the current value back and show it in a summary
/// card.
#[derive(Clone, Debug)]
pub(crate) struct FormDemoState {
    /// The value the select is showing.
    pub(crate) select_value: Signal<String>,
    /// The values the checkbox group has checked.
    pub(crate) checked_values: Signal<Vec<String>>,
    /// The value the segmented control is showing.
    pub(crate) segmented_value: Signal<String>,
    /// The number the number input is showing.
    pub(crate) number_value: Signal<i64>,
    /// The text the auto-complete is showing.
    pub(crate) query: Signal<String>,
    /// The date the date picker is showing.
    pub(crate) date_value: Signal<String>,
    /// The values the toggle group has selected.
    pub(crate) toggle_values: Signal<Vec<String>>,
    /// Whether the standalone toggle is pressed.
    pub(crate) toggle_pressed: Signal<bool>,
    /// The values of the accordion sections that are open.
    pub(crate) accordion_open: Signal<Vec<String>>,
    /// The text the textarea is showing.
    pub(crate) notes: Signal<String>,
    /// The text the input group field is showing.
    pub(crate) group_query: Signal<String>,
    /// Whether the text fields on the page are disabled.
    pub(crate) field_disabled: Signal<bool>,
    /// Whether the first radio row is checked.
    pub(crate) radio_first: Signal<bool>,
    /// Whether the second radio row is checked.
    pub(crate) radio_second: Signal<bool>,
    /// Whether the menubar menu is open.
    pub(crate) menubar_open: Signal<bool>,
    /// Whether the second menubar menu is open.
    pub(crate) menubar_edit_open: Signal<bool>,
    /// Whether the menubar menu in the component-parts showcase is open.
    pub(crate) menubar_parts_open: Signal<bool>,
    /// Whether the command palette is open.
    pub(crate) command_query: Signal<String>,
    /// Whether the combobox is open.
    pub(crate) combobox_open: Signal<bool>,
    /// The value the combobox is showing.
    pub(crate) combobox_value: Signal<String>,
    /// Whether the sheet is open.
    pub(crate) sheet_open: Signal<bool>,
    /// Whether the alert dialog is open.
    pub(crate) alert_open: Signal<bool>,
    /// Whether the hover card is open.
    pub(crate) hover_open: Signal<bool>,
    /// Whether the context menu is open.
    pub(crate) context_open: Signal<bool>,
    /// The text the one time code field is showing.
    pub(crate) otp_value: Signal<String>,
    /// The index of the active carousel slide.
    pub(crate) carousel_active: Signal<u32>,
}
