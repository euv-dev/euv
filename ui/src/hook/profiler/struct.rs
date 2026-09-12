use super::*;

/// A single recorded measurement.
///
/// Pushed onto the `ProfilerHandle`'s entries signal every time
/// the user calls `ProfilerHandle::measure(label, f)` (or
/// manually `begin` / `end`). Cheap to `Clone` (the inner
/// strings are small and `f64` is `Copy`).
///
/// Field-level semantics:
///
/// - `label`: free-form identifier — typically the call site name
///   (`"render-list"`, `"fetch-posts"`). Empty strings are
///   allowed but render as an empty chip in the UI, which makes
///   misconfigured measurements obvious in a profiler readout.
/// - `elapsed_ms`: wall-clock time between `begin()` and the
///   matching `end()` (or the duration of the measured closure),
///   in milliseconds. Always `>= 0.0` — `begin` is captured
///   before any user code runs, so the subtraction cannot
///   underflow.
/// - `timestamp_ms`: the wall-clock `now_ms()` value at the
///   instant the entry was recorded (NOT the start of the
///   measurement). This lets the UI sort / filter entries by
///   when they were committed, not by when the user started
///   the timer — which matters when entries are kept around
///   for "last N measurements" readouts.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct ProfileEntry {
    /// The free-form label passed to `measure` / `begin`.
    pub label: String,
    /// Duration of the measured operation, in milliseconds.
    #[get(type(copy))]
    pub elapsed_ms: f64,
    /// Wall-clock time at which the entry was recorded.
    #[get(type(copy))]
    pub timestamp_ms: f64,
}

/// A handle to the profiler registered against the current
/// hook context.
///
/// The handle owns the entries signal; calling
/// `ProfilerHandle::entries()` returns that signal so any
/// reactive read (`Signal::get()`) inside a closure subscribes
/// the enclosing render to new entries. The matching
/// measurement API is `ProfilerHandle::measure(label, f)`
/// (push-on-exit) or `ProfilerHandle::begin(label)` /
/// `ProfilerHandle::end()` (split-timer API for code paths
/// that don't fit inside a single closure).
///
/// # Lifecycle
///
/// The handle is obtained via `App::use_profiler()` (or
/// directly via `HookContext::profiler()`), which slots it into
/// the current hook context. On every render at the same hook
/// index, the same handle is returned — so measurements
/// recorded from a previous render remain visible in
/// `entries()`.
///
/// On hook-context teardown (component unmount, match-arm
/// switch, or explicit `clear()`), the handle is dropped and
/// its entries signal goes with it. If you need to keep
/// measurements alive past the lifetime of the component,
/// clone the entries vector out before the context is cleared.
#[derive(Clone, Data, New)]
pub struct ProfilerHandle {
    /// The reactive log of measurements. Every measurement
    /// pushes a fresh `ProfileEntry` into this vector via
    /// `.set(...)` — the `set` triggers the reactive update
    /// path, so any subscriber re-renders.
    pub(crate) entries: Signal<Vec<ProfileEntry>>,
}
/// A `begin()` marker — RAII guard that records the start
/// timestamp and the label so the matching `end()` call can
/// compute the elapsed time.
///
/// Created by `ProfilerHandle::begin(label)`. Consume with
/// `end()` to push a `ProfileEntry` into the entries signal.
/// Dropping the marker without calling `end()` discards the
/// measurement silently (we don't have a place to push a
/// half-finished entry, and panicking on drop is hostile).
#[derive(Data, New)]
pub struct ProfilerMark {
    /// The label this marker was created with. Copied out of
    /// the `&str` at construction time so the marker does not
    /// outlive any borrowed string.
    pub(crate) label: String,
    /// Wall-clock timestamp captured at `begin()`. Subtracted
    /// from the `end()` timestamp to compute `elapsed_ms`.
    pub(crate) started_ms: f64,
    /// Back-reference to the entries signal. Cloned (cheap —
    /// `Signal<T>` is `Copy`-by-pointer) so `end()` can push
    /// without re-borrowing the handle.
    pub(crate) entries: Signal<Vec<ProfileEntry>>,
}
