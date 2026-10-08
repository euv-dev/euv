/// The title text for the header of the async hooks page.
pub(crate) const HOOKS_ASYNC_HEADER_TITLE: &str = "Hooks — Async";

/// The subtitle text for the header of the async hooks page.
pub(crate) const HOOKS_ASYNC_HEADER_SUBTITLE: &str = "AsyncState (use_async), lazy factory (use_lazy_component), and suspense phases (use_suspense).";

/// The card heading for the `use_async` row of the async hooks page.
pub(crate) const HOOKS_ASYNC_CARD_TITLE: &str = "use_async";

/// The explanatory paragraph for the `use_async` card.
pub(crate) const HOOKS_ASYNC_CARD_DESCRIPTION: &str = "The handle's state exposes an AsyncState machine. The Loading arm carries () by default; the Ok arm carries the resolved value; the Err arm the failure message.";

/// The button label that refetches the async handle.
pub(crate) const HOOKS_ASYNC_REFETCH_LABEL: &str = "Refetch";

/// The text prefix printed before the async state label readout.
pub(crate) const HOOKS_ASYNC_STATE_PREFIX: &str = "state: ";

/// The card heading for the `use_lazy_component` row of the async hooks page.
pub(crate) const HOOKS_ASYNC_LAZY_CARD_TITLE: &str = "use_lazy_component";

/// The explanatory paragraph for the `use_lazy_component` card.
pub(crate) const HOOKS_ASYNC_LAZY_CARD_DESCRIPTION: &str = "The factory is only invoked on first access. Click Load to run it once; Reset returns the component to the pending state so the next read invokes the factory again.";

/// The button label that runs the lazy component factory once.
pub(crate) const HOOKS_ASYNC_LAZY_LOAD_LABEL: &str = "Load";

/// The button label that returns a row back to its initial state, shared
/// by the lazy-component and suspense rows.
pub(crate) const HOOKS_ASYNC_RESET_LABEL: &str = "Reset";

/// The text prefix printed before the lazy component loaded readout.
pub(crate) const HOOKS_ASYNC_LAZY_LOADED_PREFIX: &str = "loaded:";

/// The text prefix printed before the lazy component pending readout.
pub(crate) const HOOKS_ASYNC_LAZY_PENDING_PREFIX: &str = "is_pending:";

/// The card heading for the `use_suspense` row of the async hooks page.
pub(crate) const HOOKS_ASYNC_SUSPENSE_CARD_TITLE: &str = "use_suspense";

/// The explanatory paragraph for the `use_suspense` card.
pub(crate) const HOOKS_ASYNC_SUSPENSE_CARD_DESCRIPTION: &str = "resolve_sync and fail flip the phase signal — the rendering code branches on the resulting Pending / Resolved / Failed variant.";

/// The button label that flips the suspense phase to resolved.
pub(crate) const HOOKS_ASYNC_SUSPENSE_RESOLVE_LABEL: &str = "Resolve";

/// The button label that flips the suspense phase to failed.
pub(crate) const HOOKS_ASYNC_SUSPENSE_FAIL_LABEL: &str = "Fail";

/// The text prefix printed before the suspense phase readout.
pub(crate) const HOOKS_ASYNC_PHASE_PREFIX: &str = "phase: ";
