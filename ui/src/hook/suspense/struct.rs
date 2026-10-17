use super::*;

/// A handle that drives a `SuspenseState`.
///
/// Created via `SuspenseHandle::new()`. The handle owns
/// the underlying `Signal<SuspensePhase<T>>` and exposes
/// `state()` to read it. The handle itself is the only
/// thing that can mutate the phase, via
/// `resolve_sync` (every target) or `resolve_async`
/// (wasm-only).
#[derive(Clone, Data, Debug)]
pub struct SuspenseHandle<T: Clone + PartialEq + 'static> {
    /// The phase signal.
    ///
    /// Public because `ui/tests/suspense/fn.rs`,
    /// `ui/tests/no_dom_fallback/fn.rs` and
    /// `example/src/page/hooks_async` read it through
    /// `get_phase()`.
    pub phase: Signal<SuspensePhase<T>>,
}

/// `SuspenseHandle<T>` is `Copy` when `T` is — `Signal<...>` is
/// `Copy` for any `T: Clone + PartialEq + 'static`, so this
/// blanket impl is sound.
impl<T> Copy for SuspenseHandle<T> where T: Clone + PartialEq + 'static {}
