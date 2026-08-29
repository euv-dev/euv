use super::*;

/// The internal state of a [`ThrottledValue`].
///
/// - `Idle` — no throttle window is active.
/// - `Cooldown(u64)` — the most recent `set` happened
///   at that millisecond timestamp. While in cooldown,
///   `set` calls are
///   accepted into the `pending` slot but do NOT update
///   the emitted value. When the cooldown expires
///   (next `tick` call at or after `start + interval_ms`),
///   any pending value is committed and the state returns
///   to `Idle`.
///
/// Timestamps are plain milliseconds (`u64`) instead of
/// `std::time::Instant` because `Instant::now()` panics on
/// `wasm32-unknown-unknown`.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum ThrottleState {
    /// No active cooldown.
    #[default]
    Idle,
    /// Cooldown is in effect since the given millisecond timestamp.
    Cooldown(u64),
}
