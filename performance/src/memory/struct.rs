use super::*;

/// Delegates allocation to the system while recording requested heap bytes.
#[cfg(feature = "allocation-tracking")]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct CountingAllocator;

/// Per-thread allocator counters; no process RSS or GPU memory is inferred.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
#[new(pub(crate))]
pub(crate) struct AllocationStats {
    /// Successful allocation calls, including reallocations.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) calls: u64,
    /// Successful requests that replace an existing allocation.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) reallocations: u64,
    /// Cumulative requested allocation bytes on the current thread.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) allocated: u64,
    /// Cumulative released requested bytes on the current thread.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) freed: u64,
    /// Currently live requested bytes, excluding allocator overhead.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) live: u64,
    /// Requested-byte high-water mark in the current capture window.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) peak: u64,
}

/// Counters for one measured phase, including its signed live-byte change.
#[derive(Clone, Copy, Debug, Eq, Getter, Hash, New, Ord, PartialEq, PartialOrd, Serialize)]
#[new(pub(crate))]
pub(crate) struct AllocationDelta {
    /// Successful allocation calls belonging to this phase.
    #[get(pub(crate), type(copy))]
    pub(crate) calls: u64,
    /// Successful replacement allocations belonging to this phase.
    #[get(pub(crate), type(copy))]
    pub(crate) reallocations: u64,
    /// Full requested sizes of successful allocations and reallocations.
    #[get(pub(crate), type(copy))]
    pub(crate) allocated_bytes: u64,
    /// Requested bytes released by deallocation or replacement.
    #[get(pub(crate), type(copy))]
    pub(crate) freed_bytes: u64,
    /// Signed live-byte change between phase boundaries.
    #[get(pub(crate), type(copy))]
    pub(crate) live_delta_bytes: i64,
    /// Peak requested bytes above the phase's starting live-byte count.
    #[get(pub(crate), type(copy))]
    pub(crate) peak_extra_bytes: u64,
}
