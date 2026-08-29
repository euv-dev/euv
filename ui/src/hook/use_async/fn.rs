use super::*;

/// Obtains the `UseAsyncHandle` registered against the current hook
/// context slot.
///
/// Behaves like `HookContext::use_hook`: the same handle (and therefore
/// the same `state` signal) is returned on every render at the same hook
/// index, so state written by `set_state` / `refetch` survives re-renders.
/// Without the hook context (e.g. when called outside a render cycle) the
/// factory falls back to a stand-alone handle — see
/// [`UseAsyncHandle::new_for_fallback`].
///
/// The returned handle is `Copy`, cheap to pass around, and exposes
/// `state()` / `set_state()` for non-async testing as well as
/// `refetch()` for the real wasm path. Render code subscribes to state
/// changes by reading `state()` inside a render closure.
///
/// # Returns
///
/// - `UseAsyncHandle<T, L>` - The async handle in the `Loading`
///   initial state, ready for an `refetch(...)` from the call site.
pub fn use_async<T, L>() -> UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    HookContext::use_hook(UseAsyncHandle::new_for_fallback)
}
