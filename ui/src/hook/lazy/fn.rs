use super::*;

/// Obtains a `LazyComponent` registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `LazyComponent` is
/// returned on every render at the same hook index, preserving the
/// load state across renders. The factory closure is invoked on first
/// access via [`LazyComponent::get`] / [`LazyComponent::loaded`] /
/// [`LazyComponent::prefetch`].
///
/// # Arguments
///
/// - `Rc<dyn Fn() -> T>` - The factory that produces the underlying
///   value on demand. Wrapped in `Rc` so the `LazyComponent` can be
///   cloned cheaply and the factory can be invoked multiple times
///   after a [`LazyComponent::reset`] (when the load state is reset).
///
/// # Returns
///
/// - `LazyComponent<T>` - The lazy component handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_lazy_component<T, F>(factory: F) -> LazyComponent<T>
where
    T: Clone + PartialEq + Debug + 'static,
    F: Fn() -> T + 'static,
{
    HookContext::use_hook(move || LazyComponent::<T>::new(factory))
}
