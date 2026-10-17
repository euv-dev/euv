use super::*;

/// A reactive boolean toggle.
///
/// Constructed via `Toggle::new()` (Lombok `New`).
/// The initial value is the default (`false`); use
/// `set_true` / `set_false` / `toggle` / `set` to
/// change it. The current value is exposed as a
/// `Signal<bool>` accessed through `get_value()`. Any
/// `html!` body that reads `toggle.get_value().get()`
/// inside its render closure re-renders when the value
/// changes.
///
/// `Toggle::default()` produces the same value as
/// `Toggle::new()`.
#[derive(Clone, Data, Debug, Default, New)]
pub struct Toggle {
    /// The current value signal.
    ///
    /// Public because `ui/tests/toggle/fn.rs` reads it
    /// through `get_value()`.
    #[new(skip)]
    pub value: Signal<bool>,
}
