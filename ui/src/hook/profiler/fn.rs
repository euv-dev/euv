use super::*;

/// Returns the current wall-clock time, in milliseconds.
///
/// On `wasm32` targets this is a thin wrapper around
/// [`js_sys::Date::now`] (which itself doubles as
/// `performance.now()`-based input if the browser exposes one).
/// Returns a `f64` because the upstream JavaScript value is also
/// `f64` and rounding to integer milliseconds throws away the
/// sub-millisecond precision the profiler relies on.
///
/// On native targets (`std::time::SystemTime` is available) we
/// fall back to system wall-clock time so the profiler's
/// `measure` / `begin` / `end` code paths — and the host-side
/// unit tests that exercise them — never invoke `js_sys::*`,
/// which would `SIGABRT` with "cannot call wasm-bindgen imported
/// functions on non-wasm targets". The two return values are not
/// bit-identical (the wasm path is monotonic since the JS context
/// was created; the native path uses UNIX epoch ms and may go
/// backwards if the system clock is adjusted), but every test
/// assertion on the native path is `>=`/`>`, which both
/// implementations satisfy in steady state.
///
/// The function is `pub(crate)` because the only intended consumer
/// lives in the same crate (the profiler's `measure` / `begin` /
/// `end` paths); downstream code does not need to call it directly.
///
/// # Returns
///
/// - `f64` - The current wall-clock time, in milliseconds.
pub fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration: std::time::Duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        duration.as_secs_f64() * 1_000.0
    }
}

/// Obtains a `ProfilerHandle` registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook`: the same `ProfilerHandle`
/// is returned on every render at the same hook index, so
/// measurements pushed onto its entries signal remain visible
/// across renders.
///
/// Use [`ProfilerHandle::measure`] for a single-shot
/// "label + closure" form or [`ProfilerHandle::begin`] /
/// [`ProfilerHandle::end`] for the split-timer form. Reads of
/// [`ProfilerHandle::entries`] inside a render closure subscribe
/// the render to new entries.
///
/// # Returns
///
/// - `ProfilerHandle` - The profiler handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_profiler() -> ProfilerHandle {
    HookContext::use_hook(ProfilerHandle::new_with_empty_entries)
}

/// Runs `body` and records the elapsed time under `label`.
///
/// Convenience helper that pairs with `App::use_interval`-style
/// re-renders: any render closure can call `profiler_measure(label, ...)`
/// on its hot path and the result lands in the same `ProfilerHandle`'s
/// entries signal.
///
/// # Arguments
///
/// - `&str` - The free-form label that identifies this measurement.
/// - `F: FnOnce() -> R` - The closure whose execution time is
///   measured.
///
/// # Returns
///
/// - `R` - The closure's return value, unchanged.
pub fn profiler_measure<F, R>(label: &str, body: F) -> R
where
    F: FnOnce() -> R,
{
    let profiler: ProfilerHandle = use_profiler();
    let start_ms: f64 = now_ms();
    let result: R = body();
    let elapsed_ms: f64 = now_ms() - start_ms;
    let timestamp_ms: f64 = now_ms();
    let entry: ProfileEntry = ProfileEntry {
        label: label.to_string(),
        elapsed_ms,
        timestamp_ms,
    };
    let next_entries: Vec<ProfileEntry> = {
        let mut next: Vec<ProfileEntry> = profiler.get_entries().get();
        next.push(entry);
        next
    };
    profiler.get_entries().set(next_entries);
    result
}
