/// The title text for the header of the protective hooks page.
pub(crate) const HOOKS_PROTECT_HEADER_TITLE: &str = "Hooks — Protect";

/// The subtitle text for the header of the protective hooks page.
pub(crate) const HOOKS_PROTECT_HEADER_SUBTITLE: &str = "ErrorBoundary catches panics inside try_with; ProfilerHandle keeps a list of measurements without a global collector.";

/// The card heading for the `ErrorBoundary` demo.
pub(crate) const HOOKS_PROTECT_BOUNDARY_CARD_TITLE: &str = "ErrorBoundary";

/// The explanatory paragraph for the `ErrorBoundary` card.
pub(crate) const HOOKS_PROTECT_BOUNDARY_CARD_DESCRIPTION: &str = "try_with invokes the supplied closure inside a catch_unwind shim. If the closure panics, the boundary transitions to Caught and the caller gets an Err carrying the message.";

/// The button label that runs the closure that stays healthy.
pub(crate) const HOOKS_PROTECT_TRY_HEALTHY_LABEL: &str = "Healthy";

/// The button label that runs the closure that panics.
pub(crate) const HOOKS_PROTECT_TRY_PANIC_LABEL: &str = "Panic";

/// The button label that returns the boundary to its initial phase.
pub(crate) const HOOKS_PROTECT_BOUNDARY_RESET_LABEL: &str = "Reset";

/// The text prefix printed before the boundary phase readout.
pub(crate) const HOOKS_PROTECT_PHASE_PREFIX: &str = "phase: ";

/// The card heading for the `ProfilerHandle` demo.
pub(crate) const HOOKS_PROTECT_PROFILER_CARD_TITLE: &str = "Profiler";

/// The explanatory paragraph for the `ProfilerHandle` card.
pub(crate) const HOOKS_PROTECT_PROFILER_CARD_DESCRIPTION: &str = "Each render records a measurement via profiler_measure. Click to push more rows into the entries list.";

/// The button label that records one more measurement.
pub(crate) const HOOKS_PROTECT_PROFILER_MEASURE_LABEL: &str = "Measure";

/// The button label that empties the profiler entries list.
pub(crate) const HOOKS_PROTECT_PROFILER_CLEAR_LABEL: &str = "Clear";

/// The text prefix printed before the profiler entries count readout.
pub(crate) const HOOKS_PROTECT_ENTRIES_PREFIX: &str = "entries: ";

/// The text prefix printed before the current render trigger label readout.
pub(crate) const HOOKS_PROTECT_TRIGGER_LABEL_PREFIX: &str = "current render's trigger label: ";
