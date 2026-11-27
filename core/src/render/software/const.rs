use super::*;

/// Half-domain extent in standard deviations covered by `GAUSSIAN_ERFC_LUT` (`[-3.2σ, +3.2σ]`).
pub(crate) const GAUSSIAN_LUT_RANGE: f32 = 3.2;

/// Number of intervals inside `GAUSSIAN_ERFC_LUT` (`512` intervals, `513` sample points).
pub(crate) const GAUSSIAN_LUT_SEGMENTS: usize = 512;

/// Inverse spacing `(512.0 / 6.4) = 80.0` mapping `u + 3.2` into table index space.
pub(crate) const GAUSSIAN_LUT_STEP_INV: f32 =
    (GAUSSIAN_LUT_SEGMENTS as f32) / (GAUSSIAN_LUT_RANGE * 2.0);

/// Compile-time 2 KB analytical Gaussian `0.5 * erfc(u / sqrt(2))` lookup table eliminating per-pixel `exp()` calls.
pub(crate) const GAUSSIAN_ERFC_LUT: [f32; 513] = build_gaussian_erfc_lut();

/// Maximum cached `TextLayoutResult` entries retained by `SoftwareRenderer` between frames.
pub(crate) const TEXT_LAYOUT_CACHE_LIMIT: usize = 512;

/// Initial power-of-two slot table size for `TextLayoutCache` (load factor `<= 0.5` at `TEXT_LAYOUT_CACHE_LIMIT`).
pub(crate) const TEXT_LAYOUT_CACHE_SLOTS: usize = 1024;

/// 64-bit FxHash mixing multiplier for fast SipHash-free cache hashing and slot indexing.
pub(crate) const FX_HASH_K64: u64 = 0x517C_C1B7_2722_0A95;

/// Sentinel empty slot for `TextLayoutCache` open-addressed probing.
pub(crate) const EMPTY_TEXT_LAYOUT_SLOT: TextLayoutSlot = TextLayoutSlot {
    key_hash: 0,
    width_key: 0,
    entry_idx: u32::MAX,
};

/// Thread name prefix assigned to persistent horizontal-band rasterization workers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const BAND_WORKER_THREAD_PREFIX: &str = "euv-band";

/// Maximum total rasterizer threads (caller plus background workers) in `BandWorkerPool`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const BAND_WORKER_MAX_THREADS: usize = 16;

/// Short spin-loop iterations performed by idle band workers before parking on the condition variable.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const BAND_WORKER_SPIN_LIMIT: u32 = 32;

/// Short spin-loop iterations performed by the calling thread before parking on `wake_caller`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const BAND_CALLER_SPIN_LIMIT: u32 = 32;
