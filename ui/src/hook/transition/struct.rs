use super::*;

/// Configuration for a `TransitionState`.
///
/// Both fields are reactive (the underlying state holds a
/// `Signal<TransitionConfig>`), so consumers can adjust
/// the durations at runtime via `change_config` and any
/// pending timers driven by the old config will continue
/// using the old values until they next read the config
/// signal.
#[derive(Clone, Copy, Data, Debug, Eq, PartialEq)]
pub struct TransitionConfig {
    /// The duration of the enter animation, in
    /// milliseconds.
    #[get(type(copy))]
    pub enter_ms: u32,
    /// The duration of the exit animation, in
    /// milliseconds.
    #[get(type(copy))]
    pub exit_ms: u32,
}

/// The aggregate transition state.
///
/// Constructed once per animated element via
/// `App::use_transition(...)`. Cheap to `Clone` (each
/// internal signal is `Copy`-by-pointer). Drives the
/// state machine forward via `tick(elapsed_ms)` and
/// exposes the current phase / progress as reactive
/// signals so the consuming component's render closure
/// re-runs whenever either changes.
#[derive(Clone, Data, New)]
pub struct TransitionState {
    /// The current lifecycle phase.
    ///
    /// Public because `ui/tests/transition/fn.rs` and
    /// `ui/tests/hook_traits/fn.rs` read it through
    /// `get_phase()`.
    pub phase: Signal<TransitionPhase>,
    /// The current progress, in `[0.0, 1.0]`. Pinned to
    /// `1.0` for `Entered`, `0.0` for `Exited`, and
    /// intermediate for `Entering` / `Exiting`.
    ///
    /// Public because `ui/tests/transition/fn.rs` reads it
    /// through `get_progress()`.
    pub progress: Signal<f64>,
    /// The duration config (see `TransitionConfig`).
    ///
    /// Public because `ui/tests/transition/fn.rs` reads it
    /// through `get_config()`.
    pub config: Signal<TransitionConfig>,
}
