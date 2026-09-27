/// A reusable object pool with a free-list that preserves memory across
/// release/acquire cycles.
///
/// The pool keeps two disjoint collections of `T`: the free list of values
/// the pool owns and is ready to hand out again, and the count of values
/// currently checked out and tracked as active. Releasing a value never
/// destroys it — the caller's value is moved back onto the free list and
/// reused by the next acquire, so a per-frame spawn/despawn cycle stops
/// paying for `Vec` growth and reallocation.
///
/// ## Checkout order
///
/// The free list is **LIFO**: [`ObjectPool::acquire`] pops the most recently
/// released value. The most recently used value is therefore the next one
/// handed out, which keeps a recycling workload on the hottest allocations
/// instead of sweeping a cold list.
///
/// ## Why hand-written accessors
///
/// Lombok's `Data` derive is intentionally **not** applied here, matching the
/// precedent set by [`Tween`](crate::Tween) and
/// [`EngineCell`](crate::EngineCell): the derive does not propagate generic
/// bounds, so deriving on `ObjectPool<T>` would constrain `T` in ways the
/// pool must not constrain. The accessors below follow the same naming
/// contract as the Lombok-generated ones (`get_*` / `get_mut_*` / `set_*`).
pub struct ObjectPool<T> {
    /// The number of values currently checked out of the pool.
    ///
    /// Tracked as a count rather than a `Vec<T>` because acquire hands the
    /// value to the caller by move: the pool cannot also retain it without
    /// requiring `T: Clone`, which the pool deliberately avoids so that
    /// recycling a value never duplicates its allocation.
    pub(crate) active: usize,
    /// Values owned by the pool, ready to be handed out again.
    pub(crate) free: Vec<T>,
}
