use super::*;

/// A reference-counted, shared-mutable store of asset load callbacks.
///
/// Holds the `Closure` objects that keep `onload` / `onerror` handlers alive
/// for as long as their load is in flight.
///
/// ## Why the store is indexed rather than a plain `Vec`
///
/// A `wasm_bindgen::Closure` must not be dropped while JavaScript is
/// executing it — dropping frees the boxed Rust closure whose trampoline is
/// currently on the stack. A load callback therefore cannot free itself. The
/// callbacks instead mark their own slot `settled` in the parallel
/// `settled` vector, and [`AssetLoader::collect`] drops the settled slots
/// later, from a context that is provably not inside a callback.
///
/// ## Why the callbacks hold a `Weak` reference
///
/// The callbacks need to reach this store to mark their slot settled. A
/// strong `Rc` would make the store own the closures while the closures own
/// the store — a reference cycle leaking both halves. Holding `Weak` breaks
/// the cycle: once the owning [`AssetLoader`] is dropped the store is freed
/// and a still-pending callback's `Weak` upgrade simply returns `None`.
///
/// Held behind `EngineCell` (an `UnsafeCell`-backed `Sync` newtype) rather
/// than `RefCell` so the storage matches the rest of the engine's
/// `static mut` convention. Access via [`EngineCell::get_mut`] to obtain the
/// underlying store for push / take / drain operations.
pub type AssetClosures = Rc<EngineCell<AssetClosureStore>>;

/// A shared counter of loads that have been requested but not yet settled.
///
/// Held behind `EngineCell` so the `onload` / `onerror` closures — which own
/// only a `Weak` back-reference to the loader's state — can decrement it
/// without borrowing the loader. Making the counter itself the single source
/// of truth is what keeps `is_all_loaded` from sticking at `false`
/// forever: there is no separate `pending_count` field that a callback
/// could forget to update.
pub type AssetPending = Rc<EngineCell<u32>>;
