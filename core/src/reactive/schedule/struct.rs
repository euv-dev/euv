use super::*;

/// A cached `Function` wrapper around `window.queueMicrotask`.
///
/// OPT 7: resolving the `queueMicrotask` JS function used to take a
/// `Reflect::get(&window, &JsValue::from_str("queueMicrotask"))` call
/// (one JS round-trip + a string-to-JsValue conversion) and then a
/// `dyn_into::<Function>` call (another conversion) on every signal
/// update. The resolved handle is now looked up once on first use and
/// cached for the page's lifetime. Subsequent scheduling just
/// `Function::call1(window, dispatch_function)` — one JS round-trip
/// per dispatch instead of three.
///
/// Lives in a `thread_local!` `RefCell<MicrotaskCache>`. The previous
/// `MicrotaskCacheCell(UnsafeCell<MicrotaskCache>)` +
/// `unsafe impl Sync for MicrotaskCacheCell {}` pair existed only to smuggle
/// the cache through a `static`; with `thread_local!` the `RefCell` gives the
/// same lazy-populate-once behaviour while restoring the borrow check the
/// `unsafe` had bypassed. The payload is a plain `Option<Function>` — JS
/// object handles, not raw pointers — so nothing is lost by dropping the
/// wrapper.
#[derive(CustomDebug, Data)]
pub(crate) struct MicrotaskCache {
    /// The `window.queueMicrotask` function, resolved once on first
    /// call and reused across the page's lifetime. `None` if the
    /// browser does not expose `queueMicrotask` (the dispatch path
    /// then falls through to `setTimeout` / `requestAnimationFrame`).
    #[debug(skip)]
    pub(crate) queue_microtask: Option<Function>,
}

/// A zero-sized struct providing static methods for scheduling
/// signal update dispatches and batching.
///
/// All methods are crate-internal associated functions that manage
/// the global scheduling flags and dispatch closure.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Scheduler;
