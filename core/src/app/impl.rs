use super::*;

/// Implementation of core framework APIs for the `App` struct.
///
/// This implementation block provides static methods that delegate to the
/// corresponding framework functions. All methods are available directly
/// on the `App` type without requiring an instance.
impl App {
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
    pub fn use_signal<T, F>(init: F) -> Signal<T>
    where
        T: Clone + PartialEq + 'static,
        F: FnOnce() -> T,
    {
        HookContext::signal(init)
    }

    /// Batches signal updates within a closure, deferring DOM dispatch until the
    /// outermost batch completes.
    ///
    /// Sets `SUPPRESS_SCHEDULE` to `true` so that any `Signal::set()` calls
    /// inside the closure mark their dependents dirty precisely but do not
    /// queue a microtask dispatch. When the outermost batch completes,
    /// a single dispatch is scheduled if any dirty slots were accumulated
    /// during the batch, ensuring that all pending updates are processed.
    ///
    /// Unlike the legacy full-broadcast approach, this uses precise dependency
    /// tracking: only the dynamic nodes that actually depend on the changed
    /// signals are marked dirty and re-rendered.
    ///
    /// # Arguments
    ///
    /// - `FnOnce() -> R` - The closure to execute with batched updates.
    ///
    /// # Returns
    ///
    /// - `R` - The result of the closure execution.
    pub fn batch<F, R>(callback: F) -> R
    where
        F: FnOnce() -> R,
    {
        Scheduler::batch(callback)
    }

    /// Mounts the given virtual DOM tree to a specific element matched by a CSS selector.
    ///
    /// Supported selector syntax:
    /// - `"#id"` — select by element ID
    /// - `".class"` — select by class name (uses the first match)
    /// - `"tag"` — select by tag name (uses the first match)
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - A CSS selector string to locate the target element.
    /// - `FnOnce() -> VirtualNode + 'static` - A closure that returns the virtual DOM tree to render.
    pub fn mount<S, F>(selector: S, render_fn: F)
    where
        S: AsRef<str>,
        F: FnOnce() -> VirtualNode,
    {
        Mount::setup(selector, render_fn)
    }

    /// Schedules a deferred signal update with precise dirty marking.
    ///
    /// Marks only the specified dynamic node IDs as dirty, then queues a
    /// single microtask dispatch if one is not already pending. When
    /// `SUPPRESS_SCHEDULE` is `true`, slots are still marked dirty but no
    /// dispatch is scheduled, allowing `batch` to batch
    /// precise dirty marks without triggering premature DOM updates.
    ///
    /// # Arguments
    ///
    /// - `&[usize]` - Dynamic node IDs to mark dirty.
    pub fn schedule_update(dependents: &[usize]) {
        Scheduler::update(dependents)
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
    pub fn use_cleanup<F>(cleanup: F)
    where
        F: FnOnce() + 'static,
    {
        HookContext::cleanup(cleanup)
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
    pub fn use_interval<F>(millis: i32, callback: F) -> IntervalHandle
    where
        F: FnMut() + 'static,
    {
        HookContext::interval(millis, callback)
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
    pub fn use_window_event<E, F>(event_name: E, callback: F)
    where
        E: AsRef<str>,
        F: FnMut() + 'static,
    {
        HookContext::window_event(event_name, callback)
    }

    /// Creates a reactive handle that the renderer will populate with the
    /// DOM element after the corresponding virtual node is mounted.
    ///
    /// The returned [`NodeRef<T>`] is empty (`None`) until a virtual node
    /// in the same render carries a `ref:` attribute pointing to a clone
    /// of this handle, at which point the renderer calls
    /// [`NodeRef::set`] with the underlying `JsValue`. On unmount the
    /// handle is cleared so subsequent reads return `None` again.
    ///
    /// The handle is stored in the current hook context, so subsequent
    /// re-renders at the same hook index return the same instance. This
    /// means cloning a ref into an event handler closure will keep the
    /// ref pointing at the live DOM element across re-renders.
    ///
    /// # Arguments
    ///
    /// - No arguments; the element type is inferred from how the handle is
    ///   used (`NodeRef<HtmlInputElement>`, `NodeRef<HtmlDivElement>`, …).
    ///
    /// # Returns
    ///
    /// - `NodeRef<T>` - A reactive handle to the eventual DOM element.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let input_ref: NodeRef<HtmlInputElement> = App::use_node_ref();
    /// html! {
    ///     input { ref: input_ref.clone() type: "text" }
    /// }
    /// // Later, in an event handler or another component:
    /// input_ref.get_cloned().map(|el| el.focus());
    /// ```
    pub fn use_node_ref<T: ?Sized + 'static>() -> NodeRef<T> {
        HookContext::noderef()
    }
}
