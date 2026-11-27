use super::*;

/// Lazily initialized process-wide persistent worker pool for zero-spawn multi-core horizontal band rasterization.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) static BAND_WORKER_POOL: OnceLock<BandWorkerPool> = OnceLock::new();

thread_local! {
    /// Per-thread reusable scratch buffers for rounded-clip corner save/restore eliminating per-band allocations.
    pub(crate) static BAND_CORNER_SCRATCH: RefCell<BandCornerScratch> =
        RefCell::new(BandCornerScratch::default());
    /// Per-thread reusable 1D horizontal shadow strip scratch buffers eliminating per-shadow allocations.
    pub(crate) static BAND_SHADOW_SCRATCH: RefCell<BandShadowScratch> =
        RefCell::new(BandShadowScratch::default());
}
