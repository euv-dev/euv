use super::*;

/// A wrapper around an event callback.
///
/// Stores the event name and a shared reference to the callback closure.
/// Uses `Rc<UnsafeCell<>>` instead of `Rc<RefCell<>>` to avoid runtime
/// borrow checking overhead in the single-threaded WASM context.
/// The `Rc` provides automatic memory management (freed when last reference drops).
#[derive(Clone, CustomDebug, Data, New)]
#[new(pub(crate))]
pub struct NativeEventHandler {
    /// The name of the event (e.g., "click", "input").
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) event_name: &'static str,
    /// Shared reference to the callback closure.
    /// `UnsafeCell` allows mutable access without RefCell overhead.
    /// Safety: only accessed from the main thread in WASM single-threaded context.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: SharedEventCallback,
}
