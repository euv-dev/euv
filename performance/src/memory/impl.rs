use super::*;

/// Inherent implementation of [`AllocationStats`].
impl AllocationStats {
    /// Captures this thread's counters, optionally starting a new peak window.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to reset the peak to the current live byte count.
    ///
    /// # Returns
    ///
    /// - `Self` - Current cumulative allocation counters.
    pub(crate) fn capture(reset_peak: bool) -> Self {
        #[cfg(feature = "allocation-tracking")]
        {
            COUNTERS
                .try_with(|counter: &Cell<Self>| {
                    let mut stats: Self = counter.get();
                    if reset_peak {
                        let live: u64 = stats.get_live();
                        stats.set_peak(live);
                        counter.set(stats);
                    }
                    stats
                })
                .unwrap_or_default()
        }
        #[cfg(not(feature = "allocation-tracking"))]
        {
            let _: bool = reset_peak;
            Self::default()
        }
    }

    /// Records a successful allocation or replacement allocation.
    ///
    /// # Arguments
    ///
    /// - `usize` - Number of requested bytes.
    /// - `bool` - Whether the allocation came from reallocation.
    #[cfg(feature = "allocation-tracking")]
    fn allocated(bytes: usize, reallocation: bool) {
        let _: Result<(), AccessError> = COUNTERS.try_with(|counter: &Cell<Self>| {
            let mut stats: Self = counter.get();
            *stats.get_mut_calls() += 1;
            *stats.get_mut_reallocations() += u64::from(reallocation);
            *stats.get_mut_allocated() += bytes as u64;
            *stats.get_mut_live() += bytes as u64;
            let peak: u64 = stats.get_peak().max(stats.get_live());
            stats.set_peak(peak);
            counter.set(stats);
        });
    }

    /// Records released bytes without counting a failed allocation.
    ///
    /// # Arguments
    ///
    /// - `usize` - Number of bytes released.
    #[cfg(feature = "allocation-tracking")]
    fn released(bytes: usize) {
        let _: Result<(), AccessError> = COUNTERS.try_with(|counter: &Cell<Self>| {
            let mut stats: Self = counter.get();
            *stats.get_mut_freed() += bytes as u64;
            let live: u64 = stats.get_live().saturating_sub(bytes as u64);
            stats.set_live(live);
            counter.set(stats);
        });
    }

    /// Calculates the counters belonging to one phase.
    ///
    /// # Arguments
    ///
    /// - `Self` - Cumulative counters before and after the phase.
    ///
    /// # Returns
    ///
    /// - `Option<AllocationDelta>` - Counters only in an allocation-tracking build.
    pub(crate) fn delta(before: Self, after: Self) -> Option<AllocationDelta> {
        if !cfg!(feature = "allocation-tracking") {
            return None;
        }
        Some(AllocationDelta::new(
            after.get_calls() - before.get_calls(),
            after.get_reallocations() - before.get_reallocations(),
            after.get_allocated() - before.get_allocated(),
            after.get_freed() - before.get_freed(),
            after.get_live() as i64 - before.get_live() as i64,
            after.get_peak().saturating_sub(before.get_live()),
        ))
    }
}

// SAFETY: every pointer and layout is forwarded unchanged to System. The
// thread-local counter uses const-initialized Cells and performs no allocation.
#[cfg(feature = "allocation-tracking")]
unsafe impl GlobalAlloc for CountingAllocator {
    /// Implements the `alloc` operation on [`CountingAllocator`].
    ///
    /// # Arguments
    ///
    /// - `Layout` - A `Layout` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut u8` - A `*mut u8` value.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer: *mut u8 = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            AllocationStats::allocated(layout.size(), false);
        }
        pointer
    }

    /// Implements the `alloc_zeroed` operation on [`CountingAllocator`].
    ///
    /// # Arguments
    ///
    /// - `Layout` - A `Layout` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut u8` - A `*mut u8` value.
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer: *mut u8 = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            AllocationStats::allocated(layout.size(), false);
        }
        pointer
    }

    /// Implements the `dealloc` operation on [`CountingAllocator`].
    ///
    /// # Arguments
    ///
    /// - `*mut u8` - A `*mut u8` parameter.
    /// - `Layout` - A `Layout` parameter.
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(pointer, layout);
        }
        AllocationStats::released(layout.size());
    }

    /// Implements the `realloc` operation on [`CountingAllocator`].
    ///
    /// # Arguments
    ///
    /// - `*mut u8` - A `*mut u8` parameter.
    /// - `Layout` - A `Layout` parameter.
    /// - `usize` - A non-negative integer (`usize`).
    ///
    /// # Returns
    ///
    /// - `*mut u8` - A `*mut u8` value.
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let replacement: *mut u8 = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            AllocationStats::released(layout.size());
            AllocationStats::allocated(new_size, true);
        }
        replacement
    }
}
