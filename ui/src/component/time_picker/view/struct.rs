use super::*;

/// Props for the [`euv_time_picker`] component.
///
/// The selected time lives in a caller-owned [`Signal`] formatted as `HH:MM`,
/// so the picker takes part in two-way binding with the rest of the page. The
/// open state of the panel is owned by the component itself, so the caller
/// never has to wire it up.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTimePickerProps {
    /// The selected time, formatted "HH:MM".
    ///
    /// An empty string means that nothing is selected yet.
    pub value: Signal<String>,
    /// The text shown when nothing is selected.
    pub placeholder: &'static str,
    /// The group label above the control.
    ///
    /// An empty string renders the picker without a label.
    pub label: &'static str,
    /// The first hour the picker offers.
    ///
    /// The bound is clamped into the day, so a value above `23` offers the
    /// last hour of the day.
    pub min_hour: u32,
    /// The last hour the picker offers (inclusive).
    ///
    /// The bound is clamped into the day, so a value above `23` offers the
    /// last hour of the day.
    pub max_hour: u32,
    /// The minute step the picker offers.
    ///
    /// The minutes run from `0` to `59` in steps of this size; a step of zero
    /// is treated as a step of one minute.
    pub minute_step: u32,
    /// The callback fired when a time is chosen.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(&'static str)>>,
}
