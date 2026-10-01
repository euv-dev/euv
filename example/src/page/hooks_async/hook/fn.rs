use super::*;

/// Returns a click handler that triggers an `Ok("...")` future
/// and writes it into the supplied `UseAsyncHandle`.
/// # Arguments
///
/// - `UseAsyncHandle<String, ()>` - The UseAsyncHandle<String, ()> parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The Option<Rc<dyn Fn(Event)>> value.
pub(crate) fn hooks_async_refetch(handle: UseAsyncHandle<String, ()>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.set_state(AsyncState::<String, ()>::Ok(String::from(
            HOOKS_ASYNC_RESOLVED_VALUE,
        )));
    }))
}

/// Reads the current `AsyncState` and shapes it into a readable
/// string for the demo card.
/// # Arguments
///
/// - `UseAsyncHandle<String, ()>` - The UseAsyncHandle<String, ()> parameter.
///
/// # Returns
///
/// - `String` - The String value.
pub(crate) fn hooks_async_state_label(handle: UseAsyncHandle<String, ()>) -> String {
    match handle.state() {
        AsyncState::<String, ()>::Loading(_) => String::from(HOOKS_ASYNC_STATE_LOADING),
        AsyncState::<String, ()>::Ok(value) => format!("Ok({value:?})"),
        AsyncState::<String, ()>::Err(err) => format!("Err({err:?})"),
    }
}

/// Returns `true` while the lazy component has not produced a value
/// yet.
///
/// Reads `loaded()` (not `get()`) so the check never triggers the
/// factory as a side effect of rendering.
/// # Arguments
///
/// - `&LazyComponent<String>` - The &LazyComponent<String> parameter.
///
/// # Returns
///
/// - `bool` - The bool value.
pub(crate) fn hooks_async_lazy_is_pending(lazy: &LazyComponent<String>) -> bool {
    lazy.loaded().is_none()
}

/// Returns the loaded `LazyComponent` value as a `String` for the
/// demo card. Falls back to `"pending"` when the factory has not
/// fired yet. Never triggers the factory (uses `loaded()`).
/// # Arguments
///
/// - `&LazyComponent<String>` - The &LazyComponent<String> parameter.
///
/// # Returns
///
/// - `String` - The String value.
pub(crate) fn hooks_async_lazy_loaded_label(lazy: &LazyComponent<String>) -> String {
    lazy.loaded()
        .unwrap_or_else(|| String::from(HOOKS_ASYNC_LAZY_PENDING))
}

/// Builds the click handler that runs the lazy factory once
/// (Pending → Loading → Loaded in a single synchronous pass).
/// # Arguments
///
/// - `LazyComponent<String>` - The LazyComponent<String> parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The Option<Rc<dyn Fn(Event)>> value.
pub(crate) fn hooks_async_lazy_on_load(lazy: LazyComponent<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        lazy.prefetch();
    }))
}

/// Builds the click handler that resets the lazy component back to
/// `Pending` so the next read re-runs the factory.
/// # Arguments
///
/// - `LazyComponent<String>` - The LazyComponent<String> parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The Option<Rc<dyn Fn(Event)>> value.
pub(crate) fn hooks_async_lazy_on_reset(lazy: LazyComponent<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        lazy.reset();
    }))
}

/// Reads `SuspenseHandle`'s phase and shapes it into a readable
/// string for the demo card.
/// # Arguments
///
/// - `&SuspenseHandle<String>` - The &SuspenseHandle<String> parameter.
///
/// # Returns
///
/// - `String` - The String value.
pub(crate) fn hooks_async_suspense_phase_label(handle: &SuspenseHandle<String>) -> String {
    match handle.get_phase().get() {
        SuspensePhase::Pending => String::from(HOOKS_ASYNC_SUSPENSE_PENDING),
        SuspensePhase::Resolved(value) => format!("Resolved({value})"),
        SuspensePhase::Failed(message) => format!("Failed({message})"),
    }
}

/// Returns `true` when the suspense phase is `Resolved`.
/// # Arguments
///
/// - `&SuspenseHandle<String>` - The &SuspenseHandle<String> parameter.
///
/// # Returns
///
/// - `bool` - The bool value.
pub(crate) fn hooks_async_suspense_is_resolved(handle: &SuspenseHandle<String>) -> bool {
    matches!(handle.get_phase().get(), SuspensePhase::Resolved(_))
}

/// Returns `true` when the suspense phase is `Failed`.
/// # Arguments
///
/// - `&SuspenseHandle<String>` - The &SuspenseHandle<String> parameter.
///
/// # Returns
///
/// - `bool` - The bool value.
pub(crate) fn hooks_async_suspense_is_failed(handle: &SuspenseHandle<String>) -> bool {
    matches!(handle.get_phase().get(), SuspensePhase::Failed(_))
}

/// Builds the click handler that flips the suspense handle to
/// `Resolved`.
/// # Arguments
///
/// - `SuspenseHandle<String>` - The SuspenseHandle<String> parameter.
/// - `String` - The String parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The Option<Rc<dyn Fn(Event)>> value.
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
/// # Arguments
///
/// - `SuspenseHandle<String>` - The SuspenseHandle<String> parameter.
/// - `String` - The String parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The Option<Rc<dyn Fn(Event)>> value.
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
/// # Arguments
///
/// - `SuspenseHandle<String>` - The SuspenseHandle<String> parameter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The Option<Rc<dyn Fn(Event)>> value.
pub(crate) fn hooks_async_reset(handle: SuspenseHandle<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.reset();
    }))
}
