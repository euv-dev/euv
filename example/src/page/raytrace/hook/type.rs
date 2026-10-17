use super::*;

/// One scheduled `setTimeout` waiting to be cancelled or fired.
///
/// The handle and the Rust closure the browser will invoke travel together,
/// so teardown can cancel the timer and then drop the closure instead of
/// leaking it with `Closure::forget`.
pub(crate) type RayTracePendingTimer = (i32, Closure<dyn FnMut()>);

/// Borrowed view of the pending-timer list handed to a cleanup path.
pub(crate) type RayTracePendingTimerGuard<'a> = RefMut<'a, Vec<RayTracePendingTimer>>;

/// The Rust callback a live `ResizeObserver` invokes.
pub(crate) type RayTraceObserverCallback = Closure<dyn FnMut(js_sys::Array, ResizeObserver)>;

/// The Rust callback a debounced window-resize listener invokes.
pub(crate) type RayTraceDebounceCallback = Closure<dyn FnMut()>;

/// Borrowed observer-callback slot taken during teardown.
pub(crate) type RayTraceObserverCallbackSlot<'a> = RefMut<'a, Option<RayTraceObserverCallback>>;

/// Borrowed debounce-callback slot taken during teardown.
pub(crate) type RayTraceDebounceCallbackSlot<'a> = RefMut<'a, Option<RayTraceDebounceCallback>>;

/// Owner list for every `setTimeout` a RayTrace loop schedules.
///
/// Each entry pairs the timeout handle with the Rust closure the browser
/// invokes when it fires, so teardown can cancel the timer and then drop
/// the closure instead of leaking it with `Closure::forget`.
pub(crate) type RayTracePendingTimers = Rc<RefCell<Vec<RayTracePendingTimer>>>;

/// Sole owner of the debounced window-resize listener's Rust callback.
///
/// The listener outlives the function that registers it, so the closure
/// cannot live in a local. Cleanup releases it only after the tab is
/// switched away, because the listener stays able to fire until then.
pub(crate) type RayTraceDebounceOwner = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

/// Sole owner of a `ResizeObserver`'s Rust callback.
///
/// The observer is built asynchronously but keeps firing long after init
/// returns, so the closure is parked here and dropped only once cleanup
/// has disconnected the observer.
pub(crate) type RayTraceObserverClosureOwner =
    Rc<RefCell<Option<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>>>;
