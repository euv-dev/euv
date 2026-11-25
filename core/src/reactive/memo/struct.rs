use super::*;

/// Zero-overhead `Copy` handle to a lazy, glitch-free memoized reactive computation.
#[derive(Debug)]
pub struct Memo<T>
where
    T: 'static,
{
    /// Backing arena identifier for this memo.
    pub(crate) id: MemoId,
    /// Type marker preserving static type safety.
    pub(crate) marker: PhantomData<fn() -> T>,
}
