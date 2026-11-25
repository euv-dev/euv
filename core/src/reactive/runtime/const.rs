/// Initial preallocated capacity for reactive arena slots.
pub(crate) const INITIAL_ARENA_CAPACITY: usize = 256;

/// Initial capacity for subscriber vectors on each reactive node.
pub(crate) const INITIAL_SUBSCRIBER_CAPACITY: usize = 4;

/// Maximum flush iterations before breaking a cyclic reactive loop safely.
pub(crate) const MAX_FLUSH_ITERATIONS: usize = 1024;
