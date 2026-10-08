/// Label used by the trigger-measurement call on every render.
pub(crate) const HOOKS_PROTECT_PROFILER_LABEL_TRIGGER: &str = "render-trigger";

/// A small value-payload returned by the trigger measurement —
/// kept short so the profiler entry row stays readable.
pub(crate) const HOOKS_PROTECT_TRIGGER_RENDER_VALUE: &str = "ok";

/// Label used by the deliberately-slow measurement the demo
/// button records.
pub(crate) const HOOKS_PROTECT_PROFILER_LABEL_SLOW_OP: &str = "slow-op";

/// Synthetic error message used by the panic-demo button
/// without actually calling `std::panic!` (see
/// rust-standards R11.3 — demo code must not panic).
pub(crate) const HOOKS_PROTECT_DEMO_ERROR_MESSAGE: &str = "simulated failure";

/// Readable label for the boundary's healthy phase, shown in
/// the demo card's phase readout.
pub(crate) const HOOKS_PROTECT_PHASE_LABEL_HEALTHY: &str = "Healthy";
