use super::*;

/// A bounded reactive counter.
///
/// Constructed via `Counter::new(min, max, step)`
/// (Lombok `New`); the counter starts at `i32::default()`
/// (`0`). Use [`Counter::set`] (or one of the mutator
/// methods) to set the initial value. The current value
/// is exposed as a `Signal<i32>` accessed through
/// `get_value()`. Mutators (`increment`, `decrement`,
/// `set`, `reset`) write through that signal, so any
/// render closure that reads `counter.get_value().get()`
/// re-renders when the value changes.
#[derive(Clone, Data, Debug, Default, New)]
pub struct Counter {
    /// The current value signal. Defaults to
    /// `Signal::create(0)` because of `#[new(skip)]`.
    #[new(skip)]
    pub(crate) value: Signal<i32>,
    /// Optional minimum bound. `None` means unbounded below.
    #[get(type(copy))]
    pub(crate) min: Option<i32>,
    /// Optional maximum bound. `None` means unbounded above.
    #[get(type(copy))]
    pub(crate) max: Option<i32>,
    /// Step size used by `increment` / `decrement`.
    pub(crate) step: i32,
}
