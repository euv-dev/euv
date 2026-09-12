use super::*;

/// Global typed signal slab. Single instance, lives for the program's
/// lifetime.
///
/// SAFETY: must only be accessed from the main thread (WASM single-threaded
/// context). The slab is append-only: slots are never recycled, so a stale
/// `Signal<T>` handle always resolves to its original (deactivated) slot.
pub(crate) static mut SIGNAL_SLAB: LazyLock<UnsafeCell<SignalSlab>> =
    LazyLock::new(|| UnsafeCell::new(SignalSlab::new()));
