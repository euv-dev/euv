use super::*;

/// Build a click handler that runs a healthy closure under the
/// supplied boundary. The boundary's phase transitions to
/// `Healthy` after the closure returns.
///
/// # Arguments
///
/// - `ErrorBoundary` - The boundary to run the healthy closure under.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for the healthy run.
pub(crate) fn hooks_protect_try_healthy(boundary: ErrorBoundary) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let result: Result<u32, String> = boundary.try_with(|| 7_u32);
        let _: Result<u32, String> = result;
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
///
/// # Arguments
///
/// - `ErrorBoundary` - The boundary to report the synthetic failure to.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for the panic demo.
pub(crate) fn hooks_protect_try_panic(boundary: ErrorBoundary) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let _: Result<u32, String> = boundary.try_with(|| 7_u32);
        boundary.report_error(HOOKS_PROTECT_DEMO_ERROR_MESSAGE);
    }))
}

/// Build a click handler that resets the boundary back to
/// `Healthy`.
///
/// # Arguments
///
/// - `ErrorBoundary` - The boundary to reset.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that resets the boundary.
pub(crate) fn hooks_protect_reset(boundary: ErrorBoundary) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        boundary.reset();
    }))
}

/// Build a click handler that records a deliberately-slow
/// measurement via the supplied profiler.
///
/// # Arguments
///
/// - `ProfilerHandle` - The profiler to record the slow measurement with.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that records the slow measurement.
pub(crate) fn hooks_protect_profile_slow(profiler: ProfilerHandle) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        profiler.measure(HOOKS_PROTECT_PROFILER_LABEL_SLOW_OP, || {
            // Tight loop ~ 1 ms; sufficient to show non-zero
            // elapsed time in the entries list.
            let mut accumulator: u64 = 0_u64;
            for index in 0_u64..1_000_000_u64 {
                accumulator = accumulator.wrapping_add(index);
            }
            let _: u64 = accumulator;
        });
    }))
}

/// Build a click handler that clears every recorded
/// measurement.
///
/// # Arguments
///
/// - `ProfilerHandle` - The profiler whose entries are cleared.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that clears the entries.
pub(crate) fn hooks_protect_profile_clear(profiler: ProfilerHandle) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        profiler.clear();
    }))
}

/// Returns the number of recorded entries, formatted as a
/// string for the demo readout.
///
/// # Arguments
///
/// - `ProfilerHandle` - The profiler whose entries are counted.
///
/// # Returns
///
/// - `usize` - The number of recorded profiler entries.
pub(crate) fn hooks_protect_entry_count(profiler: ProfilerHandle) -> usize {
    profiler.get_entries().get().len()
}

/// Returns `true` while the boundary phase is `Healthy` — used
/// to keep the "Try a healthy run" button in its active state.
///
/// # Arguments
///
/// - `&ErrorBoundary` - The boundary whose phase is inspected.
///
/// # Returns
///
/// - `bool` - True while the boundary phase is `Healthy`.
pub(crate) fn hooks_protect_is_healthy(boundary: &ErrorBoundary) -> bool {
    matches!(boundary.get_phase().get(), ErrorBoundaryPhase::Healthy)
}

/// Returns `true` while the boundary phase is `Caught` — used
/// to keep the "Try a panic" button in its active state.
///
/// # Arguments
///
/// - `&ErrorBoundary` - The boundary whose phase is inspected.
///
/// # Returns
///
/// - `bool` - True while the boundary phase is `Caught`.
pub(crate) fn hooks_protect_is_caught(boundary: &ErrorBoundary) -> bool {
    matches!(boundary.get_phase().get(), ErrorBoundaryPhase::Caught(_))
}

/// Reads the boundary's current phase and shapes it into a
/// readable string for the demo card.
///
/// # Arguments
///
/// - `&ErrorBoundary` - The boundary whose phase is rendered.
///
/// # Returns
///
/// - `String` - The readable phase label.
pub(crate) fn hooks_protect_phase_label(boundary: &ErrorBoundary) -> String {
    match boundary.get_phase().get() {
        ErrorBoundaryPhase::Healthy => String::from(HOOKS_PROTECT_PHASE_LABEL_HEALTHY),
        ErrorBoundaryPhase::Caught(message) => format!("Caught({message})"),
    }
}
