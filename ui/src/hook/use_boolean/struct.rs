use super::*;

/// A reactive boolean toggle with a caller-chosen initial value.
///
/// The existing [`Toggle`] type always starts as `false`, which is the
/// right default for a switch but the wrong one for a dialog that opens
/// already expanded. This type is the same toggle with the initial
/// value chosen by the caller.
///
/// The current value is exposed as a `Signal<bool>` accessed through
/// `get_value()`. Any `html!` body that reads `toggle.get_value().get()`
/// inside its render closure re-renders when the value changes.
#[derive(Clone, Data, Debug, Default, New)]
pub struct UseBoolean {
    /// The current value signal.
    ///
    /// Defaults to `Signal::create(false)` via `#[new(skip)]`; the hook
    /// overwrites it with the requested initial value.
    #[new(skip)]
    #[get(type(copy))]
    #[set(skip)]
    pub value: Signal<bool>,
}
