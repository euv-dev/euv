use super::*;

/// The internal state of a [`DebouncedValue`].
///
/// - `Idle` — no value is pending. `get()` returns the
///   last emitted value.
/// - `Pending(u64, T)` — `set(T)` was called at
///   millisecond timestamp `u64`. If `tick()` is called before
///   `set_at + delay_ms`, nothing happens. If `tick()` is
///   called at or after `set_at + delay_ms`, the pending
///   value is emitted and the state returns to `Idle`.
///
/// Timestamps are plain milliseconds (`u64`) instead of
/// `std::time::Instant` because `Instant::now()` panics on
/// `wasm32-unknown-unknown`; callers on the web pass
/// `performance.now()` (or `Date::now()`) and tests pass
/// synthetic values.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum DebounceState<T> {
    /// No pending value.
    #[default]
    Idle,
    /// A pending value with the millisecond timestamp it was set at.
    Pending(u64, T),
}
