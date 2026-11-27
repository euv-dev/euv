use super::*;

/// Single disjoint horizontal framebuffer strip dispatched to a persistent rasterization worker thread.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Data, Debug, Default)]
pub(crate) struct BandTaskSlice {
    /// Raw mutable pixel buffer base address encoded as `usize` for `Send` transfer.
    #[get(type(copy))]
    pub(crate) pixels_ptr: usize,
    /// Byte length of the disjoint horizontal pixel strip.
    #[get(type(copy))]
    pub(crate) pixels_len: usize,
    /// Full framebuffer physical width in pixels.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Inclusive top physical Y row of this strip.
    #[get(type(copy))]
    pub(crate) y_start: u32,
    /// Number of physical rows inside this strip.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Optional raw source pointer for parallel base-scene backdrop restore (`0` when unused).
    #[get(type(copy))]
    pub(crate) base_src_ptr: usize,
}

/// Active rounded-rectangle clip mask in physical pixels applied during software band rasterization.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct RoundedClipMask {
    /// Physical clip rectangle bounds.
    #[get(type(copy))]
    pub(crate) rect: Rect,
    /// Physical corner radii.
    #[get(type(copy))]
    pub(crate) radius: CornerRadius,
}

/// Shared read-only frame dispatch state consumed lock-free by persistent band workers during a single `render` call.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct BandJob {
    /// Base address of the caller-owned `BandTaskSlice` slice.
    pub(crate) tasks_ptr: usize,
    /// Number of horizontal strips in the frame job.
    pub(crate) tasks_len: usize,
    /// Base address of `DisplayList::commands`.
    pub(crate) commands_ptr: usize,
    /// Length of `DisplayList::commands`.
    pub(crate) commands_len: usize,
    /// Base address of the pre-processed `PreppedCommand` slice.
    pub(crate) prepped_ptr: usize,
    /// Length of the pre-processed `PreppedCommand` slice.
    pub(crate) prepped_len: usize,
    /// Base address of the pre-measured `TextLayoutResult` slice.
    pub(crate) layouts_ptr: usize,
    /// Length of the pre-measured `TextLayoutResult` slice.
    pub(crate) layouts_len: usize,
    /// Base address of the per-frame active `RoundedClipMask` slice.
    pub(crate) rounded_clips_ptr: usize,
    /// Length of the per-frame active `RoundedClipMask` slice.
    pub(crate) rounded_clips_len: usize,
    /// Base address of the shared read-only `GlyphAtlas`.
    pub(crate) atlas_ptr: usize,
    /// Active physical DPI scale factor.
    pub(crate) scale: f32,
    /// Active anti-aliasing policy.
    pub(crate) aa: AntialiasingMode,
    /// Atomic work-stealing strip cursor shared by the caller and all worker threads.
    pub(crate) next_idx: AtomicUsize,
    /// Atomic count of worker threads that finished observing this epoch.
    pub(crate) workers_done: AtomicUsize,
    /// Atomic accumulated shaded pixel count across all strips.
    pub(crate) total_shaded: AtomicUsize,
}

/// Per-worker cache-line-isolated wake slot eliminating N-way mutex convoys and false sharing.
#[cfg(not(target_arch = "wasm32"))]
#[repr(align(128))]
#[derive(Debug, Default)]
pub(crate) struct BandWorkerSlot {
    /// Per-worker monotonically increasing dispatch epoch counter.
    pub(crate) epoch: AtomicUsize,
    /// Non-zero while this worker thread is parked inside `wake.wait`.
    pub(crate) parked: AtomicUsize,
    /// Private mutex guarding this worker's condvar sleep transition.
    pub(crate) lock: Mutex<()>,
    /// Condition variable waking only this worker on a new epoch or shutdown.
    pub(crate) wake: Condvar,
}

/// Lock-free atomic dispatch header and per-worker wake slots shared between the caller and persistent band workers.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
pub(crate) struct BandPoolShared {
    /// Raw pointer address of the active `BandJob` (`0` while idle).
    pub(crate) active_job: AtomicUsize,
    /// Number of background workers dispatched for the current epoch.
    pub(crate) active_workers: AtomicUsize,
    /// Non-zero when the calling thread is parked waiting for workers to finish.
    pub(crate) caller_parked: AtomicUsize,
    /// Non-zero when the worker pool is shutting down during `Drop`.
    pub(crate) shutdown: AtomicUsize,
    /// Mutex guarding caller parking transitions (bypassed on the spin fast-path).
    pub(crate) caller_lock: Mutex<()>,
    /// Condition variable waking a parked caller once all dispatched workers finish an epoch.
    pub(crate) wake_caller: Condvar,
    /// Contention-free per-worker wake slots indexed by worker ID `0..worker_count`.
    pub(crate) slots: Vec<BandWorkerSlot>,
}

/// Persistent native worker pool eliminating per-frame `pthread_create` overhead on high-refresh mobile and desktop targets.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct BandWorkerPool {
    /// Number of background worker threads (excluding the calling main thread).
    pub(crate) worker_count: usize,
    /// Non-blocking guard owning reusable horizontal band task slices across frames.
    pub(crate) dispatch_tasks: Mutex<Vec<BandTaskSlice>>,
    /// Shared atomic dispatch state and condition variables.
    pub(crate) shared: Arc<BandPoolShared>,
    /// Background worker join handles drained cleanly on `Drop`.
    pub(crate) handles: Vec<JoinHandle<()>>,
}

/// Preprocessed draw-command metadata computed once on the main thread before parallel band rasterization.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct PreppedCommand {
    /// Original index inside `DisplayList::commands`.
    #[get(type(copy))]
    pub(crate) cmd_index: usize,
    /// Active intersected physical clip rectangle for this command.
    #[get(type(copy))]
    pub(crate) active_clip: Rect,
    /// Inclusive top physical Y row affected by this command.
    #[get(type(copy))]
    pub(crate) y0: u32,
    /// Exclusive bottom physical Y row affected by this command.
    #[get(type(copy))]
    pub(crate) y1: u32,
    /// Optional opaque inner vertical-span cutout for a `BoxShadow` followed immediately by an opaque `FillRoundedRect`.
    #[get(type(copy))]
    pub(crate) opaque_cutout_h: Rect,
    /// Optional opaque inner horizontal-span cutout for a `BoxShadow` followed immediately by an opaque `FillRoundedRect`.
    #[get(type(copy))]
    pub(crate) opaque_cutout_v: Rect,
    /// Optional index into the per-frame pre-resolved `TextLayoutResult` slice (`u32::MAX` when unused).
    #[get(type(copy))]
    pub(crate) text_layout_idx: u32,
    /// Start index into the per-frame `RoundedClipMask` slice (`0` when unused).
    #[get(type(copy))]
    pub(crate) rounded_clip_start: u32,
    /// Count of active `RoundedClipMask` entries touching this command (`0` on fast path).
    #[get(type(copy))]
    pub(crate) rounded_clip_len: u16,
}

/// High-performance CPU tile and analytical-SDF software rasterizer with zero GPU dependencies.
#[derive(Clone, Data, Debug, Default)]
pub struct SoftwareRenderer {
    /// Persistent RGBA8 framebuffer reused across frames.
    pub(crate) framebuffer: Framebuffer,
    /// Shared glyph rasterization atlas.
    pub(crate) glyph_atlas: GlyphAtlas,
    /// Native or deterministic font rasterization provider.
    pub(crate) font_provider: NativeSystemFont,
    /// Deterministic text measurer matching `LayoutEngine` metrics.
    #[get(type(copy))]
    pub(crate) measurer: DeterministicTextMeasurer,
    /// Cached word-wrapped `TextLayoutResult` map eliminating per-frame text layout allocations during scrolling.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) text_layout_cache: HashMap<(u64, u32), TextLayoutResult>,
    /// Active physical clip-rectangle stack.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clip_stack: Vec<Rect>,
    /// Parallel stack of optional rounded-rectangle clip masks aligned 1:1 with `clip_stack`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rounded_clip_stack: Vec<Option<RoundedClipMask>>,
    /// Active anti-aliasing policy.
    #[get(type(copy))]
    pub(crate) antialiasing: AntialiasingMode,
    /// Tile edge length in physical pixels for damage tile accounting.
    #[get(type(copy))]
    pub(crate) tile_size: u32,
    /// Cached physical width of the static base-scene snapshot behind active portals.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) base_cache_width: u32,
    /// Cached physical height of the static base-scene snapshot behind active portals.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) base_cache_height: u32,
    /// Full base-layer command hash for the cached static backdrop (`0` when invalid).
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) base_cache_hash: u64,
    /// Base-layer command hash observed on the immediately preceding frame to avoid copying active scroll frames.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) prev_base_hash: u64,
    /// Saved RGBA8 pixels of the static base scene beneath active portals for fast overlay compositing.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) base_cache_pixels: Vec<u8>,
}
