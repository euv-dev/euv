use super::*;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

thread_local! {
    pub(crate) static COUNTERS: Cell<AllocationStats> = const { Cell::new(AllocationStats {
        calls: 0, reallocations: 0, allocated: 0, freed: 0, live: 0, peak: 0,
    }) };
}
