use super::*;

/// Zero-overhead `Copy` handle to a fine-grained reactive signal stored in the arena.
#[derive(Debug)]
pub struct Signal<T>
where
    T: 'static,
{
    /// Backing arena identifier for this signal.
    pub(crate) id: SignalId,
    /// Type marker preserving static type safety without storing payload inline.
    pub(crate) marker: PhantomData<fn() -> T>,
}
