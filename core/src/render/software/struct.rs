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
    /// Horizontal byte offset within each row where backdrop restore starts.
    #[get(type(copy))]
    pub(crate) restore_x0_bytes: usize,
    /// Horizontal byte length restored per row (`width * 4` for full-width contiguous restore).
    #[get(type(copy))]
    pub(crate) restore_row_bytes: usize,
}

/// Optional base-scene backdrop restore and fused full-viewport scrim LUT specification for band dispatch.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BandRestoreSpec<'a> {
    /// Cached full-framebuffer RGBA8 base scene pixels, if restoring backdrop.
    pub(crate) base_pixels: Option<&'a [u8]>,
    /// Inclusive physical start X column to restore on each row.
    pub(crate) x0: u32,
    /// Exclusive physical end X column to restore on each row.
    pub(crate) x1: u32,
    /// Optional prebuilt 256-entry linear-light scrim LUT fused into backdrop restore.
    pub(crate) scrim_luts: Option<&'a RgbaLutQuad>,
}

/// Precomputed 1D horizontal edge pixel sample reused across every vertical interior scanline of a rounded rectangle.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct EdgeBlendSample {
    /// Physical X pixel coordinate inside the target scanline.
    pub(crate) x: u32,
    /// Direct RGBA8 bytes written verbatim when `is_opaque` is true.
    pub(crate) rgba: [u8; 4],
    /// Pre-multiplied linear red channel `SRGB_TO_LIN_LUT[r] * alpha`.
    pub(crate) sr_a: f32,
    /// Pre-multiplied linear green channel `SRGB_TO_LIN_LUT[g] * alpha`.
    pub(crate) sg_a: f32,
    /// Pre-multiplied linear blue channel `SRGB_TO_LIN_LUT[b] * alpha`.
    pub(crate) sb_a: f32,
    /// Complement opacity `1.0 - alpha`.
    pub(crate) inv_a: f32,
    /// Quantized source alpha in `0..=255` for destination alpha update.
    pub(crate) sa_u32: u32,
    /// True when the sample overwrites destination pixels without blending.
    pub(crate) is_opaque: bool,
}

/// Precomputed per-row exterior, anti-aliased, and dirty horizontal spans inside a cached rounded-clip corner mask.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CornerMaskRowSpan {
    /// Inclusive start X (in mask-local `0..dim`) of the contiguous exterior (`cov == 0`) run.
    pub(crate) ext_start: u16,
    /// Exclusive end X of the contiguous exterior (`cov == 0`) run.
    pub(crate) ext_end: u16,
    /// Inclusive start X of the anti-aliased (`1..=15`) transition run.
    pub(crate) aa_start: u16,
    /// Exclusive end X of the anti-aliased (`1..=15`) transition run.
    pub(crate) aa_end: u16,
    /// Inclusive start X of all non-interior (`cov < 16`) pixels requiring save/restore.
    pub(crate) dirty_start: u16,
    /// Exclusive end X of all non-interior (`cov < 16`) pixels requiring save/restore.
    pub(crate) dirty_end: u16,
}

/// Cached exact 4x4 RGSS coverage grid (`0..=16` hit counts) for one rounded-clip quadrant `(radius, quadrant, aa)`.
#[derive(Clone, Debug)]
pub(crate) struct CachedCornerMask {
    /// Bit representation of the clamped physical corner radius `r.to_bits()`.
    pub(crate) radius_bits: u32,
    /// Quadrant index (`0 = TL`, `1 = TR`, `2 = BR`, `3 = BL`).
    pub(crate) quadrant: u8,
    /// Whether 4x4 RGSS anti-aliasing is enabled (`false` for `AntialiasingMode::None`).
    pub(crate) aa_enabled: bool,
    /// Side length of the square coverage grid in physical pixels (`ceil(r + 1)`).
    pub(crate) dim: u16,
    /// Row-major exact coverage hit counts in `0..=16` (`0` = outside, `16` = fully inside).
    pub(crate) cov: Vec<u8>,
    /// Per-row horizontal exterior/AA/dirty span bounds (`len == dim`).
    pub(crate) row_spans: Vec<CornerMaskRowSpan>,
}

/// Persistent cache of precomputed rounded-clip corner coverage masks reused across cards and frames.
#[derive(Clone, Debug, Default)]
pub(crate) struct CornerMaskCache {
    /// Contiguous arena of cached quadrant coverage masks indexed by `u32` ID.
    pub(crate) entries: Vec<CachedCornerMask>,
}

/// Single saved horizontal row segment belonging to one cached rounded-clip corner.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SavedCornerRow {
    /// Global physical Y row coordinate.
    pub(crate) y: u32,
    /// Inclusive global physical start X of the saved segment.
    pub(crate) sx0: u32,
    /// Exclusive global physical end X of the saved segment.
    pub(crate) sx1: u32,
    /// Byte offset into `BandCornerScratch::saved_bytes` where this row's RGBA bytes start.
    pub(crate) byte_off: usize,
    /// Index into `CornerMaskCache::entries` for this quadrant.
    pub(crate) mask_id: u32,
    /// Mask-local X origin in global physical coordinates.
    pub(crate) origin_x: i32,
    /// Mask-local Y origin in global physical coordinates.
    pub(crate) origin_y: i32,
}

/// Per-thread reusable scratch buffers for rounded-clip corner save and restore in `render_band`.
#[derive(Debug, Default)]
pub(crate) struct BandCornerScratch {
    /// Bulk RGBA8 byte buffer storing pre-command corner pixels via `extend_from_slice`.
    pub(crate) saved_bytes: Vec<u8>,
    /// Saved row descriptors for the single-mask cached fast path.
    pub(crate) saved_rows: Vec<SavedCornerRow>,
    /// Clipped corner box bounds for the multi-mask / fallback path.
    pub(crate) corner_boxes: Vec<(u32, u32, u32, u32)>,
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
    /// Cached quadrant mask IDs `[tl, tr, br, bl]` into `CornerMaskCache::entries` (`u32::MAX` = uncached/inactive).
    #[get(type(copy))]
    pub(crate) corner_mask_ids: [u32; 4],
}

/// Pre-positioned glyph instance resolved once per frame on the main thread for zero-lookup band blitting.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct PositionedGlyph {
    /// Direct arena slot index into `GlyphAtlas::entries`.
    pub(crate) glyph_idx: u32,
    /// Rounded physical X coordinate of the glyph's left edge.
    pub(crate) x: i32,
    /// Rounded physical Y coordinate of the glyph's top edge.
    pub(crate) y: i32,
    /// Cached glyph bitmap width in physical pixels for fast band culling.
    pub(crate) width: u16,
    /// Cached glyph bitmap height in physical pixels for fast band culling.
    pub(crate) height: u16,
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
    /// Base address of the per-frame pre-positioned `PositionedGlyph` slice.
    pub(crate) glyphs_ptr: usize,
    /// Length of the per-frame pre-positioned `PositionedGlyph` slice.
    pub(crate) glyphs_len: usize,
    /// Base address of the per-frame pre-computed `TextCaret` rectangle slice.
    pub(crate) caret_rects_ptr: usize,
    /// Length of the per-frame pre-computed `TextCaret` rectangle slice.
    pub(crate) caret_rects_len: usize,
    /// Base address of the per-frame active `RoundedClipMask` slice.
    pub(crate) rounded_clips_ptr: usize,
    /// Length of the per-frame active `RoundedClipMask` slice.
    pub(crate) rounded_clips_len: usize,
    /// Base address of the read-only cached quadrant corner masks slice.
    pub(crate) corner_masks_ptr: usize,
    /// Length of the read-only cached quadrant corner masks slice.
    pub(crate) corner_masks_len: usize,
    /// Base address of the shared read-only `GlyphAtlas`.
    pub(crate) atlas_ptr: usize,
    /// Optional raw pointer to a stack-pinned `RgbaLutQuad` fused into backdrop restore (`0` when unused).
    pub(crate) scrim_luts_ptr: usize,
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
    /// Start index into the per-frame `PositionedGlyph` slice for `TextRun` commands (`0` when unused).
    #[get(type(copy))]
    pub(crate) glyph_run_start: u32,
    /// Count of pre-positioned glyphs for `TextRun` commands (`0` when unused).
    #[get(type(copy))]
    pub(crate) glyph_run_len: u32,
    /// Optional index into the per-frame pre-resolved `TextCaret` rectangle slice (`u32::MAX` when unused).
    #[get(type(copy))]
    pub(crate) caret_rect_idx: u32,
    /// Start index into the per-frame `RoundedClipMask` slice (`0` when unused).
    #[get(type(copy))]
    pub(crate) rounded_clip_start: u32,
    /// Count of active `RoundedClipMask` entries touching this command (`0` on fast path).
    #[get(type(copy))]
    pub(crate) rounded_clip_len: u16,
}

/// Single open-addressed slot in `TextLayoutCache` mapping `(key_hash, width_key)` to an arena index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TextLayoutSlot {
    /// Deterministic 64-bit FxHash of source text bytes and typography style.
    pub(crate) key_hash: u64,
    /// Quantized wrap width (`(max_w * 10.0).round() as u32`, or `u32::MAX` for unbounded lines).
    pub(crate) width_key: u32,
    /// Arena index into `TextLayoutCache::entries` (`u32::MAX` marks an empty slot).
    pub(crate) entry_idx: u32,
}

/// Zero-clone, SipHash-free open-addressed cache of wrapped `TextLayoutResult` entries referenced by arena index.
#[derive(Clone, Debug, Default)]
pub(crate) struct TextLayoutCache {
    /// Power-of-two linear-probing slot table (`entry_idx == u32::MAX` denotes an empty slot).
    pub(crate) slots: Vec<TextLayoutSlot>,
    /// Contiguous arena owning cached `TextLayoutResult` entries read lock-free across band workers.
    pub(crate) entries: Vec<TextLayoutResult>,
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
    /// Zero-clone open-addressed cache of wrapped `TextLayoutResult` entries eliminating per-frame cloning and SipHash.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) text_layout_cache: TextLayoutCache,
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
    /// Reusable pre-processed draw command buffer cleared at frame start to avoid per-frame allocations.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) prepped_scratch: Vec<PreppedCommand>,
    /// Reusable active rounded-clip mask buffer cleared at frame start to avoid per-frame allocations.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rounded_clips_scratch: Vec<RoundedClipMask>,
    /// Reusable flat buffer of pre-positioned glyphs emitted once per frame for zero-lookup band rasterization.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) positioned_glyphs_scratch: Vec<PositionedGlyph>,
    /// Reusable line buffer for single-lookup `Center`/`Right` text alignment positioning.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) line_glyphs_scratch: Vec<(u32, f32, u16, u16)>,
    /// Reusable buffer of pre-computed physical caret rectangles for `DrawCommand::TextCaret`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) caret_rects_scratch: Vec<Rect>,
    /// Persistent cache of precomputed rounded-clip corner coverage masks reused across cards and frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) corner_mask_cache: CornerMaskCache,
    /// Physical union bounds of the foreground portal commands painted on the previous cached frame.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) prev_portal_bounds: Option<Rect>,
    /// Full-viewport translucent scrim color active on the previous cached portal frame, if any.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) prev_portal_scrim: Option<Color>,
    /// Physical union bounds of all prepped commands emitted during the latest `prepare_frame_range`.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) prepped_bounds: Option<Rect>,
    /// Physical union bounds of `prepped_scratch[1..]` emitted during the latest `prepare_frame_range`.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) prepped_fg_bounds: Option<Rect>,
}

/// Compact 16-byte hot per-column cache entry reused across every vertical shadow strip scanline.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ShadowStripHot {
    /// Precomputed horizontal IGN phase term `(x as f32) * 0.067_110_56`.
    pub(crate) ign_x: f32,
    /// Last observed destination RGBA8 bytes packed as native-endian `u32`.
    pub(crate) cached_dst: u32,
    /// Cached quantized sRGB Q4 red output prior to IGN dither addition.
    pub(crate) q4_r: i16,
    /// Cached quantized sRGB Q4 green output prior to IGN dither addition.
    pub(crate) q4_g: i16,
    /// Cached quantized sRGB Q4 blue output prior to IGN dither addition.
    pub(crate) q4_b: i16,
    /// Cached composited output alpha byte.
    pub(crate) out_a: u8,
    /// Column blend state (`0` = inactive, `1` = uncached blend, `2` = cached blend, `3` = opaque).
    pub(crate) state: u8,
}

/// Precomputed linear-light blend factors for a single shadow column or mirrored corner sample.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ShadowStripParams {
    /// Premultiplied linear red channel `SRGB_TO_LIN_LUT[r] * alpha`.
    pub(crate) sr_a: f32,
    /// Premultiplied linear green channel `SRGB_TO_LIN_LUT[g] * alpha`.
    pub(crate) sg_a: f32,
    /// Premultiplied linear blue channel `SRGB_TO_LIN_LUT[b] * alpha`.
    pub(crate) sb_a: f32,
    /// Complement opacity `1.0 - alpha`.
    pub(crate) inv_a: f32,
    /// Quantized source alpha `(alpha * 255.0) as u32`.
    pub(crate) sa_u32: u32,
    /// Complement integer alpha `255 - sa_u32`.
    pub(crate) inv_a_u32: u32,
}

/// Single-pixel destination cache shared across 2x/4x symmetrically mirrored shadow corner writes.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ShadowPixelCache {
    /// Last observed destination RGBA8 bytes packed as native-endian `u32`.
    pub(crate) dst_u32: u32,
    /// Cached quantized sRGB Q4 red output prior to IGN dither addition.
    pub(crate) q4_r: i32,
    /// Cached quantized sRGB Q4 green output prior to IGN dither addition.
    pub(crate) q4_g: i32,
    /// Cached quantized sRGB Q4 blue output prior to IGN dither addition.
    pub(crate) q4_b: i32,
    /// Cached composited output alpha byte.
    pub(crate) out_a: u8,
    /// Whether `dst_u32` and `q4_*` have been populated for the current corner sample.
    pub(crate) valid: bool,
}

/// Per-thread reusable 1D horizontal shadow falloff strip buffers eliminating per-shadow allocations.
#[derive(Debug, Default)]
pub(crate) struct BandShadowScratch {
    /// Hot per-column cache entries for the left vertical shadow strip.
    pub(crate) left_hot: Vec<ShadowStripHot>,
    /// Cold linear blend parameters for the left vertical shadow strip.
    pub(crate) left_params: Vec<ShadowStripParams>,
    /// Hot per-column cache entries for the right vertical shadow strip.
    pub(crate) right_hot: Vec<ShadowStripHot>,
    /// Cold linear blend parameters for the right vertical shadow strip.
    pub(crate) right_params: Vec<ShadowStripParams>,
}

/// Precomputed geometry and hoisted linear tint constants shared across rows of one `raster_shadow` invocation.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ShadowRenderCtx {
    /// Shadow tint RGBA color passed to `blend_shadow_span_const_alpha`.
    pub(crate) color: Color,
    /// Shadow tint sRGB RGB bytes `[r, g, b]`.
    pub(crate) color_rgb: [u8; 3],
    /// Hoisted linear-light shadow tint `[sr, sg, sb]`.
    pub(crate) lin_rgb: [f32; 3],
    /// Caster horizontal center X in physical pixels.
    pub(crate) cx: f32,
    /// Caster vertical center Y in physical pixels.
    pub(crate) cy: f32,
    /// Caster half-width in physical pixels.
    pub(crate) half_w: f32,
    /// Caster half-height in physical pixels.
    pub(crate) half_h: f32,
    /// Normalized shadow tint alpha in `[0.0, 1.0]`.
    pub(crate) base_alpha: f32,
    /// Inverse Gaussian standard deviation `1.0 / sigma.max(0.5)`.
    pub(crate) inv_sigma: f32,
    /// Gaussian cutoff radius `3.0 * sigma`.
    pub(crate) cutoff: f32,
    /// Clamped uniform corner radius in physical pixels.
    pub(crate) uniform_r: f32,
    /// Conservative squared distance threshold `(cutoff + uniform_r + 1e-3)^2` for sqrt-free corner early reject.
    pub(crate) cutoff_r_safe_sq: f32,
    /// Clamped inclusive visible left X bound.
    pub(crate) x0: u32,
    /// Clamped exclusive visible right X bound.
    pub(crate) x1: u32,
    /// Exclusive end X of the left corner region (`sx0` when `has_mid_x`, else `split_x`).
    pub(crate) lx1: u32,
    /// Inclusive start X of the right corner region (`sx1` when `has_mid_x`, else `split_x`).
    pub(crate) rx0: u32,
    /// Horizontal mirror sum `(2 * cx).round() - 1` when `cx` is half-integer aligned.
    pub(crate) mirror_x_sum: i32,
    /// True when the shadow has a middle constant-alpha horizontal span (`sx0 < sx1`).
    pub(crate) has_mid_x: bool,
}
