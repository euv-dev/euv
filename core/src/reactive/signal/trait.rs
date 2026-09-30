use super::*;

/// Type-erased handle to a `SignalInner<T>` slot stored in [`SIGNAL_SLAB`].
///
/// Each slot holds a heap-allocated `SignalInner<T>` for some concrete `T`,
/// boxed so that the slab can store heterogeneous signal types behind one
/// `Vec`. The trait exposes only what the slab needs: type-tag for safe
/// downcast, alive-flag access, and `Any` projections for typed getters
/// implemented in `impl.rs`.
pub(crate) trait AnySignalInner: Any {
    /// Projects the slot as `&mut dyn Any` for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Concrete typed view of a `SignalInner<T>` stored in the slab.
///
/// The slab stores `Box<dyn AnySignalInner>`. To obtain a typed
/// `&mut SignalInner<T>` the framework calls `AnySignalInner::as_any_mut`
/// and downcasts via `TypeId`. The TypeId check guards against accidental
/// cross-type access of the same slot index (impossible in practice because
/// each slot is created with one concrete `T`, but the assertion defends
/// against future refactors).
impl<T> AnySignalInner for SignalInner<T>
where
    T: Clone + PartialEq + 'static,
{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
