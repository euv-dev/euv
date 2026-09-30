/// The number of values a new pool starts with when no prewarm size is given.
pub(crate) const POOL_DEFAULT_PREWARM: usize = 0;

/// The struct name used by the `Debug` implementation of `ObjectPool`.
pub(crate) const POOL_DEBUG_NAME: &str = "ObjectPool";

/// The `Debug` field name of the number of checked-out values.
pub(crate) const POOL_FIELD_ACTIVE: &str = "active";

/// The `Debug` field name of the number of values on the free list.
pub(crate) const POOL_FIELD_AVAILABLE: &str = "available";
