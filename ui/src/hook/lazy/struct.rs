use super::*;

/// A lazy component that defers factory invocation until
/// first access.
///
/// Use `LazyComponent::new(factory)` to construct, then
/// `get()` to read (which triggers the factory on first
/// call) or `prefetch()` to trigger without reading.
///
/// # Why use `Rc<dyn Fn() -> T>`?
///
/// Because the factory must be callable multiple times
/// (e.g. after a `reset()`), and `dyn Fn` lets the user
/// pass any closure that produces a `T`. The factory is
/// stored as `Rc<dyn Fn() -> T>` (not `Box<dyn Fn>`) so
/// `LazyComponent` can be cloned cheaply and shared
/// between hook contexts.
#[derive(Clone, Data)]
pub struct LazyComponent<T: Clone + PartialEq + 'static> {
    /// The reactive load state.
    ///
    /// Public because `ui/tests/lazy/fn.rs` reads it
    /// through `get_state()`.
    pub state: Signal<LoadState<T>>,
    /// The deferred value factory.
    ///
    /// `get_factory()` has no caller outside this crate
    /// (`impl.rs` invokes it internally), so the generated
    /// getter, mutable getter and setter are all
    /// `pub(crate)`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) factory: Rc<dyn Fn() -> T>,
}
