use super::*;

/// The terminal readiness state shared by cache waiters and resource requests.
pub(crate) type CacheReadiness = Result<Arc<Path>, String>;
