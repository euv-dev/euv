use super::*;

/// The string value the async page resolves into on `Resolve`.
pub(crate) const HOOKS_ASYNC_RESOLVED_VALUE: &str = "hello from use_async";

/// The error message the async page fails with on `Fail`.
pub(crate) const HOOKS_ASYNC_FAIL_MESSAGE: &str = "demo failure";

/// The value produced by the lazy-component factory when it runs.
///
/// Chosen to be self-explanatory on screen: the demo previously
/// loaded the bare number `7`, which read as a magic constant.
pub(crate) const HOOKS_ASYNC_LAZY_VALUE: &str = "lazy value (factory ran once)";

/// Returns a click handler that triggers an `Ok("...")` future
/// and writes it into the supplied `UseAsyncHandle`.
pub(crate) fn hooks_async_refetch(handle: UseAsyncHandle<String, ()>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.set_state(AsyncState::<String, ()>::Ok(String::from(
            HOOKS_ASYNC_RESOLVED_VALUE,
        )));
    }))
}

/// Reads the current `AsyncState` and shapes it into a readable
/// string for the demo card.
pub(crate) fn hooks_async_state_label(handle: UseAsyncHandle<String, ()>) -> String {
    match handle.state() {
        AsyncState::<String, ()>::Loading(_) => String::from("Loading"),
        AsyncState::<String, ()>::Ok(value) => format!("Ok({value:?})"),
        AsyncState::<String, ()>::Err(err) => format!("Err({err:?})"),
    }
}

/// Returns `true` when the async handle is currently in the `Ok`
/// state — used to highlight the Refetch button after a successful
/// refetch.
pub(crate) fn hooks_async_state_is_ok(handle: UseAsyncHandle<String, ()>) -> bool {
    matches!(handle.state(), AsyncState::<String, ()>::Ok(_))
}

/// Returns `true` while the lazy component has not produced a value
/// yet.
///
/// Reads `loaded()` (not `get()`) so the check never triggers the
/// factory as a side effect of rendering.
pub(crate) fn hooks_async_lazy_is_pending(lazy: &LazyComponent<String>) -> bool {
    lazy.loaded().is_none()
}

/// Returns `true` once the lazy component's factory has produced a
/// value — used to flip the Load button into its active state.
pub(crate) fn hooks_async_lazy_is_loaded(lazy: &LazyComponent<String>) -> bool {
    lazy.loaded().is_some()
}

/// Returns the loaded `LazyComponent` value as a `String` for the
/// demo card. Falls back to `"pending"` when the factory has not
/// fired yet. Never triggers the factory (uses `loaded()`).
pub(crate) fn hooks_async_lazy_loaded_label(lazy: &LazyComponent<String>) -> String {
    lazy.loaded().unwrap_or_else(|| String::from("pending"))
}

/// Builds the click handler that runs the lazy factory once
/// (Pending → Loading → Loaded in a single synchronous pass).
pub(crate) fn hooks_async_lazy_on_load(lazy: LazyComponent<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        lazy.prefetch();
    }))
}

/// Builds the click handler that resets the lazy component back to
/// `Pending` so the next read re-runs the factory.
pub(crate) fn hooks_async_lazy_on_reset(lazy: LazyComponent<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        lazy.reset();
    }))
}

/// Reads `SuspenseHandle`'s phase and shapes it into a readable
/// string for the demo card.
pub(crate) fn hooks_async_suspense_phase_label(handle: &SuspenseHandle<String>) -> String {
    match handle.get_phase().get() {
        SuspensePhase::Pending => String::from("Pending"),
        SuspensePhase::Resolved(value) => format!("Resolved({value})"),
        SuspensePhase::Failed(message) => format!("Failed({message})"),
    }
}

/// Returns `true` when the suspense phase is `Pending`.
pub(crate) fn hooks_async_suspense_is_pending(handle: &SuspenseHandle<String>) -> bool {
    matches!(handle.get_phase().get(), SuspensePhase::Pending)
}

/// Returns `true` when the suspense phase is `Resolved`.
pub(crate) fn hooks_async_suspense_is_resolved(handle: &SuspenseHandle<String>) -> bool {
    matches!(handle.get_phase().get(), SuspensePhase::Resolved(_))
}

/// Returns `true` when the suspense phase is `Failed`.
pub(crate) fn hooks_async_suspense_is_failed(handle: &SuspenseHandle<String>) -> bool {
    matches!(handle.get_phase().get(), SuspensePhase::Failed(_))
}

/// Builds the click handler that flips the suspense handle to
/// `Resolved`.
pub(crate) fn hooks_async_resolve(
    handle: SuspenseHandle<String>,
    value: String,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.resolve_sync(value.clone());
    }))
}

/// Builds the click handler that flips the suspense handle to
/// `Failed`.
pub(crate) fn hooks_async_fail(
    handle: SuspenseHandle<String>,
    message: String,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.fail(message.clone());
    }))
}

/// Builds the click handler that resets the suspense handle
/// back to `Pending`.
pub(crate) fn hooks_async_reset(handle: SuspenseHandle<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.reset();
    }))
}
