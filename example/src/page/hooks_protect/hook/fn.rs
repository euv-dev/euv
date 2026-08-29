use super::*;

/// Label used by the trigger-measurement call on every render.
pub(crate) const HOOKS_PROTECT_PROFILER_LABEL_TRIGGER: &str = "render-trigger";

/// A small value-payload returned by the trigger measurement —
/// kept short so the profiler entry row stays readable.
pub(crate) const HOOKS_PROTECT_TRIGGER_RENDER_VALUE: &str = "ok";

/// Synthetic error message used by the panic-demo button
/// without actually calling `std::panic!` (see
/// rust-standards R11.3 — demo code must not panic).
pub(crate) const HOOKS_PROTECT_DEMO_ERROR_MESSAGE: &str = "simulated failure";

/// Build a click handler that runs a healthy closure under the
/// supplied boundary. The boundary's phase transitions to
/// `Healthy` after the closure returns.
pub(crate) fn hooks_protect_try_healthy(boundary: ErrorBoundary) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let result: Result<u32, String> = boundary.try_with(|| 7_u32);
        let _ = result;
        boundary.reset();
    }))
}

/// Build a click handler that triggers a synthetic
/// failure under the boundary, leaving it in `Caught`
/// with the supplied message.
///
/// The hook's `try_with` API exists for genuine
/// panics; this demo deliberately avoids `panic!` so
/// the page stays inside rust-standards R11.3 (no
/// production panic). The boundary transitions
/// through [`ErrorBoundary::report_error`].
pub(crate) fn hooks_protect_try_panic(boundary: ErrorBoundary) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let _ = boundary.try_with(|| 7_u32);
        boundary.report_error(HOOKS_PROTECT_DEMO_ERROR_MESSAGE);
    }))
}

/// Build a click handler that resets the boundary back to
/// `Healthy`.
pub(crate) fn hooks_protect_reset(boundary: ErrorBoundary) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        boundary.reset();
    }))
}

/// Build a click handler that records a deliberately-slow
/// measurement via the supplied profiler.
pub(crate) fn hooks_protect_profile_slow(profiler: ProfilerHandle) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        profiler.measure("slow-op", || {
            // Tight loop ~ 1 ms; sufficient to show non-zero
            // elapsed time in the entries list.
            let mut accumulator: u64 = 0_u64;
            for index in 0_u64..1_000_000_u64 {
                accumulator = accumulator.wrapping_add(index);
            }
            let _ = accumulator;
        });
    }))
}

/// Build a click handler that clears every recorded
/// measurement.
pub(crate) fn hooks_protect_profile_clear(profiler: ProfilerHandle) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        profiler.clear();
    }))
}

/// Returns the number of recorded entries, formatted as a
/// string for the demo readout.
pub(crate) fn hooks_protect_entry_count(profiler: ProfilerHandle) -> usize {
    profiler.get_entries().get().len()
}

/// Returns `true` while the boundary phase is `Healthy` — used
/// to keep the "Try a healthy run" button in its active state.
pub(crate) fn hooks_protect_is_healthy(boundary: &ErrorBoundary) -> bool {
    matches!(boundary.get_phase().get(), ErrorBoundaryPhase::Healthy)
}

/// Returns `true` while the boundary phase is `Caught` — used
/// to keep the "Try a panic" button in its active state.
pub(crate) fn hooks_protect_is_caught(boundary: &ErrorBoundary) -> bool {
    matches!(boundary.get_phase().get(), ErrorBoundaryPhase::Caught(_))
}

/// Reads the boundary's current phase and shapes it into a
/// readable string for the demo card.
pub(crate) fn hooks_protect_phase_label(boundary: &ErrorBoundary) -> String {
    match boundary.get_phase().get() {
        ErrorBoundaryPhase::Healthy => String::from("Healthy"),
        ErrorBoundaryPhase::Caught(message) => format!("Caught({message})"),
    }
}
