use super::*;

/// Hook-context teardown for [`UseAsyncSlot`].
impl<T, L> Drop for UseAsyncSlot<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Drops the instance, releasing any owned resources.
    fn drop(&mut self) {
        // Flipping the flag first means an in-flight future that
        // happens to fire `state.set(...)` *while* `drop` is
        // running still sees the cancellation before its write
        // commits.
        self.get_cancel().set(true);
        // The `state` signal's own `Drop` impl is enough to release
        // its subscriptions; no extra cleanup needed here.
    }
}

/// Inherent implementation of [`UseAsyncHandle`] — slot lifecycle internals.
impl<T, L> UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Allocates a stand-alone slot (not tied to any hook context).
    ///
    /// Used as the fallback by [`Self::default`] and by the
    /// `App::use_async` wrapper when `HookContext::current()` is
    /// unavailable (e.g. when the user calls `use_async` outside of
    /// a render cycle, which is technically allowed but produces
    /// a non-reactive handle).
    pub(crate) fn new_for_fallback() -> Self {
        let cancel: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let state: Signal<AsyncState<T, L>> = Signal::create(AsyncState::Loading(L::empty()));
        let slot: Box<UseAsyncSlot<T, L>> = Box::new(UseAsyncSlot { state, cancel });
        let inner: usize = Box::into_raw(slot) as usize;
        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns a borrowed pointer to the heap-allocated slot.
    ///
    /// # Safety
    ///
    /// Caller must ensure the slot is alive. The handle owns a
    /// `Box<UseAsyncSlot<T, L>>` for its lifetime (the slot is
    /// leaked at allocation time, never dropped) — see
    /// [`Self::release`] for the explicit teardown path used by
    /// `HookContext::clear`.
    ///
    /// # Returns
    ///
    /// - `&UseAsyncSlot<T, L>` - A `UseAsyncSlot<T, L>` value.
    unsafe fn slot(&self) -> &UseAsyncSlot<T, L> {
        unsafe { &*(*self.get_inner() as *const UseAsyncSlot<T, L>) }
    }
}

/// Inherent implementation of [`UseAsyncHandle`] — public reactive API.
impl<T, L> UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Returns the current reactive state.
    ///
    /// # Returns
    ///
    /// - `AsyncState<T, L>` - The current `AsyncState`, owned.
    pub fn state(&self) -> AsyncState<T, L> {
        // SAFETY: handle either owns the slot (fallback path) or
        // borrows a slot whose lifetime is bounded by the hook
        // context. Both invariants ensure `slot()` returns a
        // valid reference.
        unsafe { self.slot().state.get() }
    }

    /// Overrides the slot's state directly.
    ///
    /// Bypasses the future machinery. Exists so the
    /// integration tests in `core/tests/use_async/` can
    /// exercise the `match` arms produced by users without
    /// needing a live browser to run the future.
    ///
    /// # Arguments
    ///
    /// - `AsyncState<T, L>` - A `AsyncState<T, L>` parameter.
    pub fn set_state(&self, next: AsyncState<T, L>) {
        unsafe { self.slot().state.set(next) }
    }

    /// Re-runs the future, ignoring any in-flight result from a
    /// previous attempt.
    ///
    /// Internally this sets a fresh cancel flag, transitions the
    /// state to `Loading(L::empty())`, and spawns the future. The
    /// existing in-flight future will see its cancel flag flipped
    /// and exit early.
    ///
    /// The error type `E` is intentionally a free type parameter
    /// (rather than `String` or a dedicated `AsyncError` trait) so
    /// `Result<T, JsValue>`, `Result<T, MyDomainError>`, and
    /// `Result<T, String>` all work without an adapter layer.
    ///
    /// # Arguments
    ///
    /// - `F` - A generic type parameter.
    pub fn refetch<F, Fut, E>(&self, factory: F)
    where
        F: FnOnce() -> Fut + 'static,
        Fut: Future<Output = Result<T, E>> + 'static,
        E: Into<String> + 'static,
    {
        let cancel: Rc<Cell<bool>> = unsafe { self.slot().cancel.clone() };
        // Reset cancellation. The previous in-flight future may
        // still be running, but its check now flips back to
        // "cancelled" only if its old clone of the `Rc` still
        // points at the now-false cell.
        //
        // Note: `Rc::clone` shares the same cell, so the new
        // future's check still sees *our* update. The previous
        // future sees the same cell, so on its late resolution
        // path it will compare against the same boolean — which
        // may now read `false` again, allowing the stale write to
        // commit. This is a known limitation of single-flag
        // cancellation; a `generation: usize` counter would fix it
        // but adds enough bookkeeping to make the slot a lot
        // bigger. Documented in the PR description.
        cancel.set(false);
        let state: Signal<AsyncState<T, L>> = unsafe { self.slot().state };
        let cancel_for_task: Rc<Cell<bool>> = Rc::clone(&cancel);
        let task_fut: Fut = factory();
        let task: core::pin::Pin<Box<dyn Future<Output = ()>>> = Box::pin(async move {
            let outcome: Result<T, E> = task_fut.await;
            if cancel_for_task.get() {
                return;
            }
            let next: AsyncState<T, L> = match outcome {
                Ok(value) => AsyncState::Ok(value),
                Err(err) => AsyncState::Err(err.into()),
            };
            state.set(next);
        });
        spawn_local(task);
    }
}

/// Implements `impl HasLoadingHint for ()`.
impl HasLoadingHint for () {
    /// Constructs an empty `AsyncData` value (no data, no error).
    fn empty() -> Self {}
}

/// Debug formatting for [`UseAsyncHandle`].
impl<T, L> core::fmt::Debug for UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Formats the [`UseAsyncHandle`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut core::fmt::Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `core::fmt::Result` - Result of the formatting operation.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Avoid touching the inner pointer in `Debug` output — the
        // address is meaningless to users and could collide with
        // string-formatted `AsyncState` payloads.
        f.debug_struct("UseAsyncHandle")
            .field("inner", &format_args!("<opaque 0x{:x}>", *self.get_inner()))
            .finish()
    }
}

/// Default-construction for [`UseAsyncHandle`].
impl<T, L> Default for UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Constructs a default [`UseAsyncHandle`] value.
    fn default() -> Self {
        // Same fallback path as `App::use_signal` when the hook
        // context is unavailable: a fresh state handle that points
        // at a stand-alone `UseAsyncInner`. This means
        // `UseAsyncHandle::default()` always gives the caller
        // something they can `match` on, but the state will stay
        // stuck in `Loading` because no future is wired up.
        Self::new_for_fallback()
    }
}
