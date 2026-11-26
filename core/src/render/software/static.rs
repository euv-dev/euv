use super::*;

/// Lazily initialized process-wide persistent worker pool for zero-spawn multi-core horizontal band rasterization.
pub(crate) static BAND_WORKER_POOL: OnceLock<BandWorkerPool> = OnceLock::new();
