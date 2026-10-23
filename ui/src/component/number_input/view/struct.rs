use super::*;

/// Props for the [`euv_number_input`] component.
///
/// The value lives in a caller-owned [`Signal`] so the field takes part in
/// two-way binding: the steppers and the typed text both write back into it,
/// and the caller may write into it from the outside.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNumberInputProps {
    /// The current value of the field.
    pub value: Signal<i64>,
    /// The inclusive lower bound.
    ///
    /// `None` leaves the value unbounded below.
    pub min: Option<i64>,
    /// The inclusive upper bound.
    ///
    /// `None` leaves the value unbounded above.
    pub max: Option<i64>,
    /// The step size used by the two stepper buttons.
    ///
    /// A step of `0` would never move the value, so it is treated as `1`.
    pub step: i64,
    /// The label rendered above the field.
    ///
    /// An empty string renders the field without a label.
    pub label: &'static str,
    /// Whether the field and its steppers are inert.
    pub disabled: bool,
    /// The callback invoked with the new value after every change.
    ///
    /// It fires for stepper clicks and for typed text alike.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(i64)>>,
}
