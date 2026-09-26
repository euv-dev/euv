use super::*;

/// `use_async`'s reactive handle, stored in the hook context slot and
/// returned to the user on each render.
///
/// The handle exposes three fields:
///
/// - `state` - the current `AsyncState<T, L>` (matches what the user
///   should `match` on in `html!`).
/// - `refetch` - triggers the future to run again, regardless of
///   whether the previous attempt completed or is still in flight.
/// - `cancel` - drops the in-flight future (if any) and prevents its
///   `Ok`/`Err` branches from mutating the state. Subsequent renders
///   will still call the future again on the next mount.
///
/// Cloning a handle is cheap — `UseAsyncHandle` is `Copy` if its
/// generic parameters are. Use it from event handlers the same way
/// you'd use a `Signal<T>`.
#[derive(Clone, Data)]
pub struct UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    /// Address of the heap-allocated `UseAsyncInner<T, L>` state.
    pub(crate) inner: usize,
    /// `Copy` marker so `UseAsyncHandle` itself is `Copy`.
    pub(crate) _marker: core::marker::PhantomData<fn() -> (T, L)>,
}

/// Blanket `Copy` for any generic instance — both fields are
/// themselves `Copy` (`usize`, `PhantomData<fn pointer>`).
/// The `where` clause must be repeated because a separate impl
/// block cannot inherit bounds from the type declaration.
impl<T, L> core::marker::Copy for UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
}

/// Heap-allocated state backing a [`super::UseAsyncHandle`].
///
/// Reachable only through the raw address stored in the handle.
/// Allocated by [`super::UseAsyncHandle::new_for_fallback`] for the
/// "no hook context" case and by [`HookContext::use_async`] when
/// the hook is registered for the first time.
#[derive(Clone, Data)]
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
