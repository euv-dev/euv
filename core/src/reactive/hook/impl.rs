use super::*;

/// Implementation of hook context lifecycle and hook index management.
impl HookContext {
    /// Resets the hook index for a new render cycle.
    ///
    /// Sets the internal hook index back to zero so that subsequent
    /// `use_signal` calls start indexing from the beginning of the hook list.
    pub fn reset_index(&mut self) {
        if let Ok(mut inner) = self.get_inner().try_borrow_mut() {
            inner.set_hook_index(0);
        }
    }

    /// Notifies the hook context that a match arm is being entered.
    ///
    /// If the arm index has changed, all existing hooks and cleanups
    /// are cleared and re-initialized for the new arm. If the arm
    /// is unchanged, only the hook index is reset.
    ///
    /// # Arguments
    ///
    /// - `usize` - The index of the new match arm.
    pub fn switch_arm(&mut self, changed: usize) {
        let cleanups: Vec<Box<dyn FnOnce()>>;
        {
            let Ok(mut inner) = self.get_inner().try_borrow_mut() else {
                return;
            };
            if inner.get_arm_changed() == changed {
                drop(inner);
                self.reset_index();
                return;
            }
            cleanups = take(inner.get_mut_cleanups());
            inner.get_mut_hooks().clear();
            inner.set_arm_changed(changed);
        }
        for cleanup in cleanups {
            cleanup();
        }
        self.reset_index();
    }
}

/// Clones the hook context, sharing the same inner state.
///
/// All clones share the same underlying `Rc<RefCell<HookContextInner>>`,
/// so modifications through one clone are visible through all others.
///
/// # Returns
///
/// - `Self` - A new `HookContext` sharing the same inner state.
impl Clone for HookContext {
    fn clone(&self) -> Self {
        Self::new(self.get_inner().clone())
    }
}

/// Provides a default empty hook context.
///
/// Creates a fresh `Rc<RefCell<HookContextInner>>` with default values
/// (empty hook list, zero hook index, empty cleanup list).
///
/// # Returns
///
/// - `Self` - A new `HookContext` with default inner state.
impl Default for HookContext {
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(HookContextInner::default())))
    }
}

/// Implementation of interval handle lifecycle management.
impl IntervalHandle {
    /// Cancels the associated browser interval timer.
    ///
    /// Calls `window.clearInterval` with the stored interval ID.
    /// After calling this method the interval callback will no longer fire.
    ///
    /// # Panics
    ///
    /// Panics if `window()` is unavailable on the current platform.
    pub fn clear(&self) {
        if let Some(cleanup_window) = web_sys::window() {
            cleanup_window.clear_interval_with_handle(self.get_interval_id());
        }
    }
}

/// Associated functions for hook context management.
///
/// These are crate-internal static methods for managing the active hook
/// context, creating signals, registering cleanups, and scheduling intervals.
impl HookContext {
    /// Returns a shared reference to the current hook context global state.
    ///
    /// SAFETY: Must only be called from the main thread (WASM single-threaded context).
    #[allow(static_mut_refs)]
    fn try_get_current() -> &'static Option<HookContextRc> {
        unsafe { &*CURRENT_HOOK_CONTEXT.get_0().get() }
    }

    /// Returns a mutable reference to the current hook context global state.
    ///
    /// SAFETY: Must only be called from the main thread (WASM single-threaded context).
    #[allow(static_mut_refs)]
    fn try_get_mut_current() -> &'static mut Option<HookContextRc> {
        unsafe { &mut *CURRENT_HOOK_CONTEXT.get_0().get() }
    }

    /// Returns the currently active `HookContext`.
    ///
    /// If no hook context has been set, creates and stores a default one
    /// in the global `CURRENT_HOOK_CONTEXT` cell so subsequent calls
    /// return the same instance.
    ///
    /// # Returns
    ///
    /// - `HookContext` - The currently active hook context.
    pub(crate) fn current() -> HookContext {
        match Self::try_get_current() {
            Some(hook_context_rc) => HookContext::new(hook_context_rc.clone()),
            None => {
                let rc: HookContextRc = Rc::new(RefCell::new(HookContextInner::default()));
                *Self::try_get_mut_current() = Some(rc.clone());
                HookContext::new(rc)
            }
        }
    }

    /// Runs a closure with the given `HookContext` set as the active context.
    ///
    /// Saves the previous context, sets the new one, executes the closure,
    /// and restores the previous context afterward.
    ///
    /// # Arguments
    ///
    /// - `HookContext` - The hook context to set as active during closure execution.
    /// - `FnOnce() -> R` - The closure to execute with the given context.
    ///
    /// # Returns
    ///
    /// - `R` - The result of the closure execution.
    pub(crate) fn with<F, R>(context: HookContext, callback: F) -> R
    where
        F: FnOnce() -> R,
    {
        let previous: Option<HookContextRc> = Self::try_get_mut_current().take();
        *Self::try_get_mut_current() = Some(context.get_inner().clone());
        let result: R = callback();
        *Self::try_get_mut_current() = previous;
        result
    }

    /// Creates a new reactive signal with the given initial value.
    ///
    /// Uses the current `HookContext` to maintain signal identity across
    /// re-renders. On the first call at a given hook index, the signal
    /// is created with `init()` and stored. On subsequent re-renders,
    /// the existing signal at that index is returned unchanged.
    ///
    /// # Arguments
    ///
    /// - `FnOnce() -> T` - A closure that computes the initial value of the signal.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - A reactive signal containing the initialized or existing value.
    pub(crate) fn signal<T, F>(init: F) -> Signal<T>
    where
        T: Clone + PartialEq + 'static,
        F: FnOnce() -> T,
    {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return Signal::create(init());
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len()
            && let Some(existing) = inner.get_hooks()[index].downcast_ref::<Signal<T>>()
        {
            return *existing;
        }
        let signal: Signal<T> = Signal::create(init());
        inner
            .get_mut_cleanups()
            .push(Box::new(move || signal.deactivate()));
        if index < inner.get_hooks().len() {
            inner.get_mut_hooks()[index] = Box::new(signal);
        } else {
            inner.get_mut_hooks().push(Box::new(signal));
        }
        signal
    }

    /// Registers a cleanup callback that will be executed when the current
    /// hook context is cleared (e.g., when a `match` arm switches).
    ///
    /// This is useful for cleaning up side effects like intervals, timeouts,
    /// or subscriptions that are not automatically managed by signals.
    ///
    /// The cleanup callback is only registered once on the first render.
    /// On subsequent re-renders at the same hook index, this is a no-op.
    ///
    /// # Arguments
    ///
    /// - `FnOnce() + 'static` - The cleanup callback to execute on context teardown.
    pub(crate) fn cleanup<F>(cleanup: F)
    where
        F: FnOnce() + 'static,
    {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return;
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len() {
            return;
        }
        inner.get_mut_cleanups().push(Box::new(cleanup));
        inner.get_mut_hooks().push(Box::new(()));
    }

    /// Runs an effect closure once on the first render at this hook
    /// index. Subsequent renders at the same index are no-ops (matching
    /// the `use_cleanup` / `use_window_event` first-render-only
    /// contract documented on `App::use_effect`).
    ///
    /// The `FnOnce` body is consumed immediately when this hook
    /// commits; unlike `use_cleanup`, no cleanup closure is registered
    /// here, so callers that need a teardown should either return the
    /// cleanup via `effect_with_cleanup` or pair this with a separate
    /// `use_cleanup` call at the same hook index.
    pub(crate) fn effect<F>(effect: F)
    where
        F: FnOnce() + 'static,
    {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return;
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len() {
            // The effect body is a `FnOnce`, so we cannot re-invoke
            // it on subsequent renders — but we also cannot store the
            // partially-consumed closure. The pattern is "run once on
            // mount"; second renders simply do nothing.
            return;
        }
        effect();
        inner.get_mut_hooks().push(Box::new(()));
    }

    /// Runs an effect factory on the first render, captures its
    /// returned cleanup closure, and registers that cleanup for
    /// teardown when the hook context is cleared.
    ///
    /// If the effect factory panics before returning, no cleanup is
    /// registered (the panic propagates and the hook context is left
    /// untouched for this index).
    pub(crate) fn effect_with_cleanup<F, C>(effect: F)
    where
        F: FnOnce() -> C + 'static,
        C: FnOnce() + 'static,
    {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return;
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len() {
            // Same first-render-only contract as `effect` and
            // `cleanup`: the factory has already been consumed and
            // its cleanup registered on the original mount, so we
            // skip subsequent renders to avoid re-running setup
            // (which would typically also re-subscribe, leak
            // listeners, etc.).
            return;
        }
        let cleanup: C = effect();
        inner.get_mut_cleanups().push(Box::new(cleanup));
        inner.get_mut_hooks().push(Box::new(()));
    }

    /// Registers a `window.addEventListener` callback using event delegation,
    /// automatically removed when the hook context is cleared.
    ///
    /// Uses the global window event proxy registry so that only one
    /// `window.addEventListener` call is made per event name regardless of
    /// how many components listen to the same event. On cleanup, only the
    /// handler entry is removed from the proxy registry; the shared window
    /// listener remains active for other consumers.
    ///
    /// The event listener is only registered once on the first render.
    /// On subsequent re-renders at the same hook index, this is a no-op.
    ///
    /// # Arguments
    ///
    /// - `E: AsRef<str>` - The event name to listen for (e.g., "hashchange", "popstate", "resize").
    /// - `FnMut() + 'static` - The callback to invoke when the event fires.
    pub(crate) fn window_event<E, F>(event_name: E, callback: F)
    where
        E: AsRef<str>,
        F: FnMut() + 'static,
    {
        let event_name: &str = event_name.as_ref();
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return;
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len() {
            return;
        }
        let event_name_owned: String = event_name.to_owned();
        let handler_id: usize = Registry::register_window_event(event_name, callback);
        inner.get_mut_cleanups().push(Box::new(move || {
            Registry::unregister_window_event(&event_name_owned, handler_id);
        }));
        inner.get_mut_hooks().push(Box::new(()));
    }

    /// Creates a recurring interval that invokes the given closure at the
    /// specified period, returning an `IntervalHandle` that is automatically
    /// cleared when the hook context is cleared (i.e., when the component
    /// unmounts or a `match` arm switches).
    ///
    /// Unlike calling `set_interval_with_callback_and_timeout_and_arguments_0`
    /// + `Closure::forget()` manually, this hook ensures the interval is
    ///   properly cleaned up, preventing memory leaks and stale callbacks.
    ///
    /// The interval is only created once on the first render.
    /// On subsequent re-renders at the same hook index, the existing handle
    /// is returned unchanged.
    ///
    /// # Arguments
    ///
    /// - `i32` - The interval period in milliseconds.
    /// - `FnMut() + 'static` - The closure to invoke on each interval tick.
    ///
    /// # Returns
    ///
    /// - `IntervalHandle` - A handle that can be used to cancel the interval early.
    ///
    /// # Panics
    ///
    /// Panics if `window()` is unavailable on the current platform.
    pub(crate) fn interval<F>(millis: i32, callback: F) -> IntervalHandle
    where
        F: FnMut() + 'static,
    {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return IntervalHandle::new(0);
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len()
            && let Some(existing) = inner.get_hooks()[index].downcast_ref::<IntervalHandle>()
        {
            return *existing;
        }
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(callback));
        let window: Window = window().expect("no global window exists");
        let interval_id: i32 = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                millis,
            )
            .expect("failed to set interval");
        closure.forget();
        let handle: IntervalHandle = IntervalHandle::new(interval_id);
        inner.get_mut_cleanups().push(Box::new(move || {
            let Some(cleanup_window) = web_sys::window() else {
                return;
            };
            cleanup_window.clear_interval_with_handle(interval_id);
        }));
        if index < inner.get_hooks().len() {
            inner.get_mut_hooks()[index] = Box::new(handle);
        } else {
            inner.get_mut_hooks().push(Box::new(handle));
        }
        handle
    }
}
