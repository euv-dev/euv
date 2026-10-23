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
}
