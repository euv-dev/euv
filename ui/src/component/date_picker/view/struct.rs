use super::*;

/// Props for the [`euv_date_picker`] component.
///
/// The selected date lives in a caller-owned [`Signal`] formatted as
/// `YYYY-MM-DD`; the month the panel shows and the open state of the panel
/// are owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDatePickerProps {
    /// The selected date, formatted as `YYYY-MM-DD`.
    ///
    /// An empty string means that nothing is selected yet.
    pub value: Signal<String>,
    /// The placeholder shown on the trigger while nothing is selected.
    pub placeholder: &'static str,
    /// The label rendered above the trigger.
    ///
    /// An empty string renders the picker without a label.
    pub label: &'static str,
    /// The earliest selectable date, formatted as `YYYY-MM-DD`.
    ///
    /// An empty string means that there is no lower bound.
    pub min: &'static str,
    /// The latest selectable date, formatted as `YYYY-MM-DD`.
    ///
    /// An empty string means that there is no upper bound.
    pub max: &'static str,
    /// The callback invoked with the new date after every change.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
}

/// The cache of one month's caption.
#[derive(Clone, Data, Debug, Default)]
pub(crate) struct MonthCaption {
    /// The cached `(year, month, caption)` triple.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    cached: CachedCaption,
}
