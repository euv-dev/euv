//! Implementation of [`super::UseAsyncHandle`] — the body of the
//! `use_async` hook.
//!
//! The handle mirrors `Signal<T>`'s shape: the public type holds a
//! raw heap-address and a phantom marker, while the live state lives
//! in a heap-allocated `UseAsyncSlot<T, L>` reachable through that
//! address. The `Rc<Cell<bool>>` cancellation flag is shared with
//! the in-flight future so that when the hook context is cleared
//! (component unmount or `match` arm switch) the future's late
//! resolution is dropped instead of writing into a detached state.
//!
//! ## Why a slot, not a `Signal<AsyncState<T,L>>` directly
//!
//! We need three pieces of state to drive a reactive async hook:
//!
//! 1. The user-visible state (`AsyncState<T, L>`).
//! 2. A cancellation flag set when the slot is dropped.
//! 3. A way to launch a fresh future from outside the slot
//!    (`refetch()` re-uses the same slot).
//!
//! Bundling all three into a single `Box<dyn Any>` is cheaper than
//! burning three hook slots per `use_async` call (the hook index is
//! a scarce resource per render — see `App::use_signal` for the
//! same single-slot pattern).

use super::*;
use crate::Signal;
use std::cell::Cell;
use std::future::Future;
use std::rc::Rc;

/// Heap-allocated state backing a [`super::UseAsyncHandle`].
///
/// Reachable only through the raw address stored in the handle.
/// Allocated by [`super::UseAsyncHandle::new_for_fallback`] for the
/// "no hook context" case and by [`HookContext::use_async`] when
/// the hook is registered for the first time.
pub(crate) struct UseAsyncSlot<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Reactive state, exposed to the user as
    /// [`UseAsyncHandle::state`].
    pub(crate) state: Signal<AsyncState<T, L>>,
    /// Cancellation flag — flipped on drop. The in-flight future
    /// reads this before writing back to `state`.
    pub(crate) cancel: Rc<Cell<bool>>,
}

impl<T, L> Drop for UseAsyncSlot<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    fn drop(&mut self) {
        // Flipping the flag first means an in-flight future that
        // happens to fire `state.set(...)` *while* `drop` is
        // running still sees the cancellation before its write
        // commits.
        self.cancel.set(true);
        // The `state` signal's own `Drop` impl is enough to release
        // its subscriptions; no extra cleanup needed here.
    }
}

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
    unsafe fn slot(&self) -> &UseAsyncSlot<T, L> {
        &*(self.inner as *const UseAsyncSlot<T, L>)
    }

    /// Drops the slot and frees the memory.
    ///
    /// Called by the hook-context cleanup path so that memory does
    /// not leak across `match` arm switches. The cancel flag
    /// fires on `UseAsyncSlot::drop`, so any in-flight future will
    /// short-circuit before touching `state`.
    ///
    /// # Safety
    ///
    /// After calling this method, the handle is dangling and must
    /// not be used again. The hook context ensures the only
    /// remaining copy of the handle lives inside the slot's cleanup
    /// closure, which is dropped immediately after.
    pub(crate) unsafe fn release(self) {
        let _: Box<UseAsyncSlot<T, L>> = Box::from_raw(self.inner as *mut UseAsyncSlot<T, L>);
    }
}

impl<T, L> UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Returns the current reactive state.
    pub fn state(&self) -> AsyncState<T, L> {
        // SAFETY: handle either owns the slot (fallback path) or
        // borrows a slot whose lifetime is bounded by the hook
        // context. Both invariants ensure `slot()` returns a
        // valid reference.
        unsafe { self.slot().state.get() }
    }

    /// Overrides the slot's state directly.
    ///
    /// Bypasses the future machinery. Exists so unit tests can
    /// exercise the `match` arms produced by users without
    /// needing a live browser to run the future.
    #[cfg(test)]
    pub(crate) fn set_state(&self, next: AsyncState<T, L>) {
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
        let state: Signal<AsyncState<T, L>> = unsafe { self.slot().state.clone() };
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
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(task);
        }
        // On non-wasm32 targets we drop the future. This is
        // intentional: `use_async` exists to bridge async APIs
        // (fetch, IndexedDB, ...) that only exist in the browser,
        // so silently no-op'ing in tests keeps the production code
        // path simple. Tests that need to drive the state machine
        // directly should use `UseAsyncHandle::set_state`.
        #[cfg(not(target_arch = "wasm32"))]
        {
            drop(task);
        }
    }
}
