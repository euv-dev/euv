use super::*;

/// Persistent work-stealing multi-core band pool methods for `BandWorkerPool`.
#[cfg(not(target_arch = "wasm32"))]
impl BandWorkerPool {
    /// Creates a persistent worker pool sized to the host CPU count (clamped to `1..=16` total threads including the caller).
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized worker pool with warm background threads.
    #[must_use]
    pub(crate) fn new() -> Self {
        let total: usize = available_parallelism()
            .map(usize::from)
            .unwrap_or(4)
            .clamp(1, BAND_WORKER_MAX_THREADS);
        let requested_workers: usize = total.saturating_sub(1);
        let mut slots: Vec<BandWorkerSlot> = Vec::with_capacity(requested_workers);
        for _ in 0..requested_workers {
            slots.push(BandWorkerSlot::default());
        }
        let shared: Arc<BandPoolShared> = Arc::new(BandPoolShared {
            active_job: AtomicUsize::new(0),
            active_workers: AtomicUsize::new(0),
            caller_parked: AtomicUsize::new(0),
            shutdown: AtomicUsize::new(0),
            caller_lock: Mutex::new(()),
            wake_caller: Condvar::new(),
            slots,
        });
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(requested_workers);
        for idx in 0..requested_workers {
            let shared_clone: Arc<BandPoolShared> = Arc::clone(&shared);
            let name: String = format!("{BAND_WORKER_THREAD_PREFIX}-{idx}");
            if let Ok(handle) = Builder::new().name(name).spawn(move || {
                let slot: &BandWorkerSlot = &shared_clone.slots[idx];
                let mut seen_epoch: usize = 0;
                loop {
                    let mut spins: u32 = 0;
                    let job_ptr: usize = loop {
                        let cur_epoch: usize = slot.epoch.load(Ordering::Acquire);
                        if cur_epoch != seen_epoch {
                            let ptr: usize = shared_clone.active_job.load(Ordering::Acquire);
                            if ptr != 0 {
                                seen_epoch = cur_epoch;
                                break ptr;
                            }
                        }
                        if shared_clone.shutdown.load(Ordering::Acquire) != 0 {
                            return;
                        }
                        if spins < BAND_WORKER_SPIN_LIMIT {
                            spins += 1;
                            spin_loop();
                            continue;
                        }
                        let Ok(mut guard) = slot.lock.lock() else {
                            return;
                        };
                        slot.parked.store(1, Ordering::SeqCst);
                        let parked_ptr: Option<usize> = loop {
                            let locked_epoch: usize = slot.epoch.load(Ordering::SeqCst);
                            if locked_epoch != seen_epoch {
                                let ptr: usize = shared_clone.active_job.load(Ordering::Acquire);
                                if ptr != 0 {
                                    seen_epoch = locked_epoch;
                                    break Some(ptr);
                                }
                            }
                            if shared_clone.shutdown.load(Ordering::SeqCst) != 0 {
                                break None;
                            }
                            let Ok(next_guard) = slot.wake.wait(guard) else {
                                break None;
                            };
                            guard = next_guard;
                        };
                        slot.parked.store(0, Ordering::SeqCst);
                        let Some(ptr) = parked_ptr else {
                            return;
                        };
                        break ptr;
                    };
                    // SAFETY: Caller keeps `BandJob` pinned on its stack until `workers_done >= active_workers`.
                    let job: &BandJob = unsafe { &*(job_ptr as *const BandJob) };
                    run_band_job_loop(job);
                    let expected: usize = shared_clone.active_workers.load(Ordering::Acquire);
                    let finished: usize = job.workers_done.fetch_add(1, Ordering::SeqCst) + 1;
                    if finished >= expected
                        && shared_clone.caller_parked.load(Ordering::SeqCst) != 0
                        && let Ok(_guard) = shared_clone.caller_lock.lock()
                    {
                        shared_clone.wake_caller.notify_one();
                    }
                }
            }) {
                handles.push(handle);
            } else {
                break;
            }
        }
        let worker_count: usize = handles.len();
        Self {
            worker_count,
            dispatch_tasks: Mutex::new(Vec::with_capacity(
                worker_count.saturating_add(1).saturating_mul(2),
            )),
            shared,
            handles,
        }
    }

    /// Executes a contiguous horizontal framebuffer slice serially on the calling thread with optional cached backdrop restore.
    ///
    /// # Arguments
    ///
    /// - `(&mut [u8], u32, u32, u32, usize)` - Tuple `(pixels, width, y_offset, height, start_byte_offset)`.
    /// - `&[DrawCommand]` - Display list commands.
    /// - `&[PreppedCommand]` - Pre-clipped command table.
    /// - `BandAuxSlices<'_>` - Pre-positioned glyphs, caret rects, rounded clips, and cached quadrant masks.
    /// - `&GlyphAtlas` - Shared read-only glyph atlas.
    /// - `(f32, AntialiasingMode, Option<&[u8]>)` - Tuple of `(scale, antialiasing, optional_base_restore)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count across the slice.
    fn execute_serial(
        slice_spec: (&mut [u8], u32, u32, u32, usize),
        commands: &[DrawCommand],
        prepped: &[PreppedCommand],
        aux: BandAuxSlices<'_>,
        atlas: &GlyphAtlas,
        render_opts: (f32, AntialiasingMode, Option<&[u8]>),
    ) -> usize {
        let (pixels, width, y_offset, height, start_byte): (&mut [u8], u32, u32, u32, usize) =
            slice_spec;
        let (scale, aa, base_restore): (f32, AntialiasingMode, Option<&[u8]>) = render_opts;
        if let Some(src_buf) = base_restore {
            let end_byte: usize = start_byte + pixels.len();
            if let Some(src_slice) = src_buf.get(start_byte..end_byte) {
                pixels.copy_from_slice(src_slice);
            }
        }
        let mut band: PixelBand<'_> = PixelBand {
            width,
            y_offset,
            height,
            pixels,
        };
        SoftwareRenderer::render_band(&mut band, commands, prepped, aux, atlas, (scale, aa))
    }

    /// Returns total participating rasterizer threads (background workers plus the calling main thread).
    ///
    /// # Returns
    ///
    /// - `usize` - Total active thread count in `1..=16`.
    #[must_use]
    pub(crate) fn total_threads(&self) -> usize {
        let Self { worker_count, .. } = self;
        worker_count.saturating_add(1)
    }

    /// Dispatches horizontal pixel strips across the persistent worker pool without per-frame task or band allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Worker pool reference.
    /// - `&mut Framebuffer` - Target mutable framebuffer.
    /// - `(u32, u32, usize, usize)` - Tuple `(clip_y0, clip_y1, strip_count, max_workers)`.
    /// - `(&[DrawCommand], &[PreppedCommand])` - Display list commands and pre-clipped command table.
    /// - `BandAuxSlices<'_>` - Pre-positioned glyphs, caret rects, rounded clips, and cached quadrant masks.
    /// - `&GlyphAtlas` - Shared read-only glyph atlas.
    /// - `(f32, AntialiasingMode, Option<&[u8]>)` - Tuple of `(scale, antialiasing, optional_base_restore)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count across all strips.
    pub(crate) fn execute(
        &self,
        framebuffer: &mut Framebuffer,
        band_range: (u32, u32, usize, usize),
        cmd_slices: (&[DrawCommand], &[PreppedCommand]),
        aux: BandAuxSlices<'_>,
        atlas: &GlyphAtlas,
        render_opts: (f32, AntialiasingMode, Option<&[u8]>),
    ) -> usize {
        let Self {
            worker_count,
            dispatch_tasks,
            shared,
            ..
        } = self;
        let (y0, y1, band_count, max_workers): (u32, u32, usize, usize) = band_range;
        let (commands, prepped): (&[DrawCommand], &[PreppedCommand]) = cmd_slices;
        let (positioned_glyphs, caret_rects, rounded_clips, corner_masks): (
            &[PositionedGlyph],
            &[Rect],
            &[RoundedClipMask],
            &[CachedCornerMask],
        ) = aux;
        let (scale, aa, base_restore): (f32, AntialiasingMode, Option<&[u8]>) = render_opts;
        let fb_w: u32 = framebuffer.get_width();
        let fb_h: u32 = framebuffer.get_height();
        let start_y: u32 = y0.min(fb_h);
        let end_y: u32 = y1.clamp(start_y, fb_h);
        let total_rows: u32 = end_y - start_y;
        if total_rows == 0 || fb_w == 0 {
            return 0;
        }
        let row_stride: usize = (fb_w as usize) * 4;
        let start_byte: usize = (start_y as usize) * row_stride;
        let end_byte: usize = (end_y as usize) * row_stride;
        let Some(slice) = framebuffer.get_mut_pixels().get_mut(start_byte..end_byte) else {
            return 0;
        };
        let strip_count: usize = band_count.max(1).min(total_rows as usize);
        let worker_budget: usize = (*worker_count).min(max_workers);
        if worker_budget == 0 || strip_count <= 1 {
            return Self::execute_serial(
                (slice, fb_w, start_y, total_rows, start_byte),
                commands,
                prepped,
                aux,
                atlas,
                (scale, aa, base_restore),
            );
        }
        let Ok(mut tasks) = dispatch_tasks.try_lock() else {
            return Self::execute_serial(
                (slice, fb_w, start_y, total_rows, start_byte),
                commands,
                prepped,
                aux,
                atlas,
                (scale, aa, base_restore),
            );
        };
        tasks.clear();
        let rows_per: usize = (total_rows as usize).div_ceil(strip_count);
        let chunk_bytes: usize = rows_per * row_stride;
        let mut cur_y: u32 = start_y;
        let mut cur_byte_off: usize = start_byte;
        for chunk in slice.chunks_mut(chunk_bytes) {
            let chunk_len: usize = chunk.len();
            let chunk_rows: u32 = (chunk_len / row_stride) as u32;
            let base_ptr: usize = if let Some(src_buf) = base_restore {
                let byte_end: usize = cur_byte_off + chunk_len;
                if byte_end <= src_buf.len() {
                    // SAFETY: `cur_byte_off <= byte_end <= src_buf.len()` stays within `src_buf`.
                    unsafe { src_buf.as_ptr().add(cur_byte_off) as usize }
                } else {
                    0
                }
            } else {
                0
            };
            tasks.push(BandTaskSlice {
                pixels_ptr: chunk.as_mut_ptr() as usize,
                pixels_len: chunk_len,
                width: fb_w,
                y_start: cur_y,
                height: chunk_rows,
                base_src_ptr: base_ptr,
            });
            cur_y += chunk_rows;
            cur_byte_off += chunk_len;
        }
        let job: BandJob = BandJob {
            tasks_ptr: tasks.as_ptr() as usize,
            tasks_len: tasks.len(),
            commands_ptr: commands.as_ptr() as usize,
            commands_len: commands.len(),
            prepped_ptr: prepped.as_ptr() as usize,
            prepped_len: prepped.len(),
            glyphs_ptr: positioned_glyphs.as_ptr() as usize,
            glyphs_len: positioned_glyphs.len(),
            caret_rects_ptr: caret_rects.as_ptr() as usize,
            caret_rects_len: caret_rects.len(),
            rounded_clips_ptr: rounded_clips.as_ptr() as usize,
            rounded_clips_len: rounded_clips.len(),
            corner_masks_ptr: corner_masks.as_ptr() as usize,
            corner_masks_len: corner_masks.len(),
            atlas_ptr: (atlas as *const GlyphAtlas) as usize,
            scale,
            aa,
            next_idx: AtomicUsize::new(0),
            workers_done: AtomicUsize::new(0),
            total_shaded: AtomicUsize::new(0),
        };
        let dispatch_workers: usize = worker_budget.min(tasks.len().saturating_sub(1));
        if dispatch_workers == 0 {
            run_band_job_loop(&job);
            return job.total_shaded.load(Ordering::Acquire);
        }
        shared
            .active_workers
            .store(dispatch_workers, Ordering::Relaxed);
        shared
            .active_job
            .store((&job as *const BandJob) as usize, Ordering::Release);
        let active_slots: &[BandWorkerSlot] = &shared.slots[..dispatch_workers];
        for slot in active_slots.iter() {
            let next_epoch: usize = slot.epoch.load(Ordering::Relaxed).wrapping_add(1);
            slot.epoch.store(next_epoch, Ordering::SeqCst);
        }
        for slot in active_slots.iter() {
            if slot.parked.load(Ordering::SeqCst) != 0
                && let Ok(_guard) = slot.lock.lock()
            {
                slot.wake.notify_one();
            }
        }
        run_band_job_loop(&job);
        let mut caller_spins: u32 = 0;
        while job.workers_done.load(Ordering::Acquire) < dispatch_workers {
            if caller_spins < BAND_CALLER_SPIN_LIMIT {
                caller_spins += 1;
                spin_loop();
                continue;
            }
            if let Ok(mut guard) = shared.caller_lock.lock() {
                shared.caller_parked.store(1, Ordering::SeqCst);
                while job.workers_done.load(Ordering::SeqCst) < dispatch_workers {
                    let Ok(next_guard) = shared.wake_caller.wait(guard) else {
                        break;
                    };
                    guard = next_guard;
                }
                shared.caller_parked.store(0, Ordering::SeqCst);
            } else {
                spin_loop();
            }
        }
        shared.active_job.store(0, Ordering::Release);
        job.total_shaded.load(Ordering::Acquire)
    }
}

/// Clean shutdown and thread join when a `BandWorkerPool` instance is dropped.
#[cfg(not(target_arch = "wasm32"))]
impl Drop for BandWorkerPool {
    /// Signals all background worker threads to exit and joins every handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dropping band worker pool.
    fn drop(&mut self) {
        let Self {
            shared, handles, ..
        } = self;
        shared.shutdown.store(1, Ordering::SeqCst);
        for slot in shared.slots.iter() {
            if let Ok(_guard) = slot.lock.lock() {
                slot.wake.notify_one();
            }
        }
        for handle in handles.drain(..) {
            let _: Result<(), Box<dyn Any + Send>> = handle.join();
        }
    }
}

/// Zero-clone SipHash-free open-addressed lookup operations for `TextLayoutCache`.
impl TextLayoutCache {
    /// Creates a pre-allocated cache with power-of-two slot capacity.
    ///
    /// # Arguments
    ///
    /// - `usize` - Requested slot table capacity (rounded up to a power of two).
    ///
    /// # Returns
    ///
    /// - `Self` - Pre-allocated empty text layout cache.
    #[must_use]
    pub(crate) fn with_capacity(slot_cap: usize) -> Self {
        let cap: usize = slot_cap.max(16).next_power_of_two();
        Self {
            slots: vec![EMPTY_TEXT_LAYOUT_SLOT; cap],
            entries: Vec::with_capacity(cap / 2),
        }
    }

    /// Returns the number of cached `TextLayoutResult` entries currently stored in the arena.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cache reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of cached layouts.
    #[must_use]
    pub(crate) fn len(&self) -> usize {
        let Self { entries, .. } = self;
        entries.len()
    }

    /// Returns the cached `TextLayoutResult` stored at arena index `idx`, if valid.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cache reference.
    /// - `u32` - Arena index returned by `get_or_insert_with`.
    ///
    /// # Returns
    ///
    /// - `Option<&TextLayoutResult>` - Borrowed cached layout entry.
    #[must_use]
    pub(crate) fn get(&self, idx: u32) -> Option<&TextLayoutResult> {
        let Self { entries, .. } = self;
        entries.get(idx as usize)
    }

    /// Clears all cached entries and resets the slot table while retaining bounded capacity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable cache reference.
    pub(crate) fn clear(&mut self) {
        let Self { slots, entries } = self;
        entries.clear();
        if slots.len() > TEXT_LAYOUT_CACHE_SLOTS {
            slots.resize(TEXT_LAYOUT_CACHE_SLOTS, EMPTY_TEXT_LAYOUT_SLOT);
        }
        slots.fill(EMPTY_TEXT_LAYOUT_SLOT);
    }

    /// Computes the initial power-of-two probe index for `(key_hash, width_key)`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Deterministic text+style hash.
    /// - `u32` - Quantized wrap width key.
    /// - `usize` - Power-of-two bitmask (`slots.len() - 1`).
    ///
    /// # Returns
    ///
    /// - `usize` - Starting probe index in `0..=mask`.
    #[inline(always)]
    #[must_use]
    fn slot_start(key_hash: u64, width_key: u32, mask: usize) -> usize {
        let mixed: u64 = (key_hash ^ (width_key as u64)).wrapping_mul(FX_HASH_K64);
        ((mixed ^ (mixed >> 32)) as usize) & mask
    }

    /// Rehashes all live slots into a new power-of-two table of size `new_cap`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable cache reference.
    /// - `usize` - Target power-of-two slot capacity.
    fn rehash(&mut self, new_cap: usize) {
        let cap: usize = new_cap.max(TEXT_LAYOUT_CACHE_SLOTS).next_power_of_two();
        let mask: usize = cap - 1;
        let mut next_slots: Vec<TextLayoutSlot> = vec![EMPTY_TEXT_LAYOUT_SLOT; cap];
        let Self { slots, .. } = self;
        for old in slots.iter().copied() {
            if old.entry_idx == u32::MAX {
                continue;
            }
            let mut idx: usize = Self::slot_start(old.key_hash, old.width_key, mask);
            while next_slots[idx].entry_idx != u32::MAX {
                idx = (idx + 1) & mask;
            }
            next_slots[idx] = old;
        }
        *slots = next_slots;
    }

    /// Looks up `(key_hash, width_key)` without cloning, inserting the result of `compute` on a miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable cache reference.
    /// - `u64` - Deterministic text+style hash.
    /// - `u32` - Quantized wrap width key (`u32::MAX` for unbounded lines).
    /// - `F` - Lazy layout computation invoked only on a cache miss.
    ///
    /// # Returns
    ///
    /// - `u32` - Stable arena index into `entries()` for the remainder of the current frame.
    pub(crate) fn get_or_insert_with<F>(&mut self, key_hash: u64, width_key: u32, compute: F) -> u32
    where
        F: FnOnce() -> TextLayoutResult,
    {
        let mut empty_slot: Option<usize> = None;
        {
            let Self { slots, .. } = self;
            if !slots.is_empty() {
                let mask: usize = slots.len() - 1;
                let mut idx: usize = Self::slot_start(key_hash, width_key, mask);
                loop {
                    let slot: TextLayoutSlot = slots[idx];
                    if slot.entry_idx == u32::MAX {
                        empty_slot = Some(idx);
                        break;
                    }
                    if slot.key_hash == key_hash && slot.width_key == width_key {
                        return slot.entry_idx;
                    }
                    idx = (idx + 1) & mask;
                }
            }
        }
        let measured: TextLayoutResult = compute();
        let needs_rehash: bool = {
            let Self { slots, entries } = self;
            slots.is_empty() || (entries.len() + 1).saturating_mul(2) > slots.len()
        };
        if needs_rehash {
            let next_cap: usize = {
                let Self { slots, .. } = self;
                if slots.is_empty() {
                    TEXT_LAYOUT_CACHE_SLOTS
                } else {
                    slots.len().saturating_mul(2)
                }
            };
            self.rehash(next_cap);
            empty_slot = None;
        }
        let Self { slots, entries } = self;
        let entry_idx: u32 = entries.len() as u32;
        entries.push(measured);
        let target_idx: usize = match empty_slot {
            Some(idx) => idx,
            None => {
                let mask: usize = slots.len() - 1;
                let mut idx: usize = Self::slot_start(key_hash, width_key, mask);
                while slots[idx].entry_idx != u32::MAX {
                    idx = (idx + 1) & mask;
                }
                idx
            }
        };
        slots[target_idx] = TextLayoutSlot {
            key_hash,
            width_key,
            entry_idx,
        };
        entry_idx
    }
}

/// Precomputed rounded-clip corner coverage cache operations for `CornerMaskCache`.
impl CornerMaskCache {
    /// Returns the number of cached quadrant coverage masks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cache reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of cached quadrant masks.
    #[must_use]
    pub(crate) fn len(&self) -> usize {
        let Self { entries } = self;
        entries.len()
    }

    /// Returns the read-only slice of cached quadrant masks shared across band workers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cache reference.
    ///
    /// # Returns
    ///
    /// - `&[CachedCornerMask]` - Contiguous cached corner mask slice.
    #[must_use]
    pub(crate) fn entries(&self) -> &[CachedCornerMask] {
        let Self { entries } = self;
        entries.as_slice()
    }

    /// Clears all cached corner masks when exceeding the bounded retention limit.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable cache reference.
    pub(crate) fn clear(&mut self) {
        let Self { entries } = self;
        entries.clear();
    }

    /// Builds an exact `0..=16` 4x4 RGSS coverage grid and per-row span table for one corner quadrant.
    ///
    /// # Arguments
    ///
    /// - `f32` - Clamped physical corner radius (`> 0.25`).
    /// - `u8` - Quadrant index (`0 = TL`, `1 = TR`, `2 = BR`, `3 = BL`).
    /// - `bool` - True when 4x4 RGSS anti-aliasing is active.
    ///
    /// # Returns
    ///
    /// - `CachedCornerMask` - Precomputed coverage grid and row spans.
    #[must_use]
    fn build_quadrant(radius: f32, quadrant: u8, aa_enabled: bool) -> CachedCornerMask {
        let dim: usize = (radius + 1.0).ceil().max(1.0) as usize;
        let dim_f: f32 = dim as f32;
        let half_extent: f32 = dim_f + 64.0;
        let cx_lim: f32 = half_extent - radius;
        let r_in: f32 = radius - 1.0;
        let r_in_sq: f32 = r_in * r_in;
        let r_out: f32 = radius + 1.0;
        let r_out_sq: f32 = r_out * r_out;
        let r_sq: f32 = radius * radius;
        let mut cov: Vec<u8> = vec![16; dim * dim];
        let mut row_spans: Vec<CornerMaskRowSpan> = Vec::with_capacity(dim);
        let is_left: bool = quadrant == 0 || quadrant == 3;
        let is_top: bool = quadrant == 0 || quadrant == 1;
        for my in 0..dim {
            let py: f32 = if is_top {
                (my as f32) + 0.5 - half_extent
            } else {
                (my as f32) + 0.5 + (half_extent - dim_f)
            };
            let qy: f32 = py.abs() - cx_lim;
            let row_off: usize = my * dim;
            for mx in 0..dim {
                let px: f32 = if is_left {
                    (mx as f32) + 0.5 - half_extent
                } else {
                    (mx as f32) + 0.5 + (half_extent - dim_f)
                };
                let qx: f32 = px.abs() - cx_lim;
                let hits: u8 = if qx <= 0.0 || qy <= 0.0 {
                    let dist: f32 = qx.max(qy) - radius;
                    if dist <= -1.0 {
                        16
                    } else if dist >= 1.0 {
                        0
                    } else if !aa_enabled {
                        if dist <= 0.0 { 16 } else { 0 }
                    } else {
                        let c: f32 = rounded_box_supersampled_coverage(
                            px,
                            py,
                            half_extent,
                            half_extent,
                            radius,
                            0.0,
                        )
                        .0;
                        (c * 16.0).round() as u8
                    }
                } else {
                    let d_sq: f32 = qx * qx + qy * qy;
                    if r_in >= 0.0 && d_sq <= r_in_sq {
                        16
                    } else if d_sq >= r_out_sq {
                        0
                    } else if !aa_enabled {
                        if d_sq <= r_sq { 16 } else { 0 }
                    } else {
                        let c: f32 = rounded_box_supersampled_coverage(
                            px,
                            py,
                            half_extent,
                            half_extent,
                            radius,
                            0.0,
                        )
                        .0;
                        (c * 16.0).round() as u8
                    }
                };
                cov[row_off + mx] = hits;
            }
            let row_slice: &[u8] = &cov[row_off..row_off + dim];
            let span: CornerMaskRowSpan = if is_left {
                let dirty_end: usize = row_slice
                    .iter()
                    .rposition(|&c: &u8| c < 16)
                    .map(|idx: usize| idx + 1)
                    .unwrap_or(0);
                let ext_end: usize = row_slice
                    .iter()
                    .take(dirty_end)
                    .take_while(|&&c: &&u8| c == 0)
                    .count();
                CornerMaskRowSpan {
                    ext_start: 0,
                    ext_end: ext_end as u16,
                    aa_start: ext_end as u16,
                    aa_end: dirty_end as u16,
                    dirty_start: 0,
                    dirty_end: dirty_end as u16,
                }
            } else {
                let dirty_start: usize = row_slice.iter().position(|&c: &u8| c < 16).unwrap_or(dim);
                let trailing_zeros: usize = row_slice[dirty_start..]
                    .iter()
                    .rev()
                    .take_while(|&&c: &&u8| c == 0)
                    .count();
                let ext_start: usize = dim - trailing_zeros;
                CornerMaskRowSpan {
                    ext_start: ext_start as u16,
                    ext_end: dim as u16,
                    aa_start: dirty_start as u16,
                    aa_end: ext_start.max(dirty_start) as u16,
                    dirty_start: dirty_start as u16,
                    dirty_end: dim as u16,
                }
            };
            row_spans.push(span);
        }
        CachedCornerMask {
            radius_bits: radius.to_bits(),
            quadrant,
            aa_enabled,
            dim: dim as u16,
            cov,
            row_spans,
        }
    }

    /// Returns the cached mask index for `(radius, quadrant, aa_enabled)`, building and caching it on a miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable cache reference.
    /// - `f32` - Clamped physical corner radius.
    /// - `u8` - Quadrant index (`0 = TL`, `1 = TR`, `2 = BR`, `3 = BL`).
    /// - `bool` - True when anti-aliasing is enabled.
    ///
    /// # Returns
    ///
    /// - `u32` - Index into `entries()`.
    fn get_or_build(&mut self, radius: f32, quadrant: u8, aa_enabled: bool) -> u32 {
        let bits: u32 = radius.to_bits();
        let Self { entries } = self;
        for (idx, entry) in entries.iter().enumerate() {
            if entry.radius_bits == bits
                && entry.quadrant == quadrant
                && entry.aa_enabled == aa_enabled
            {
                return idx as u32;
            }
        }
        let idx: u32 = entries.len() as u32;
        entries.push(Self::build_quadrant(radius, quadrant, aa_enabled));
        idx
    }

    /// Resolves cached quadrant mask IDs `[tl, tr, br, bl]` for an integer-snapped `RoundedClipMask`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable cache reference.
    /// - `Rect` - Physical clip rectangle.
    /// - `CornerRadius` - Physical corner radii.
    /// - `AntialiasingMode` - Active anti-aliasing policy.
    ///
    /// # Returns
    ///
    /// - `[u32; 4]` - Quadrant mask IDs (`u32::MAX` when inactive or uncached).
    pub(crate) fn resolve_quadrants(
        &mut self,
        rect: Rect,
        radius: CornerRadius,
        aa: AntialiasingMode,
    ) -> [u32; 4] {
        let mut ids: [u32; 4] = [u32::MAX; 4];
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let max_r: f32 = half_w.min(half_h).max(0.0);
        if max_r <= 0.25 {
            return ids;
        }
        let x0: f32 = rect.get_x();
        let y0: f32 = rect.get_y();
        let x1: f32 = rect.right();
        let y1: f32 = rect.bottom();
        if (x0 - x0.round()).abs() > 1e-4
            || (y0 - y0.round()).abs() > 1e-4
            || (x1 - x1.round()).abs() > 1e-4
            || (y1 - y1.round()).abs() > 1e-4
        {
            return ids;
        }
        let aa_enabled: bool = aa != AntialiasingMode::None;
        let radii: [f32; 4] = [
            radius.get_top_left().min(max_r),
            radius.get_top_right().min(max_r),
            radius.get_bottom_right().min(max_r),
            radius.get_bottom_left().min(max_r),
        ];
        for (q, r) in radii.into_iter().enumerate() {
            if r > 0.25 && r + 1.5 < half_w && r + 1.5 < half_h {
                let dim: u32 = (r + 1.0).ceil() as u32;
                if dim > 0 && dim <= CORNER_MASK_MAX_DIM {
                    ids[q] = self.get_or_build(r, q as u8, aa_enabled);
                }
            }
        }
        ids
    }
}

/// Multi-core lossless analytical-SDF and tile-band software rasterizer methods for `SoftwareRenderer`.
impl SoftwareRenderer {
    /// Creates a software renderer configured for `width x height` physical pixels.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical width.
    /// - `u32` - Physical height.
    /// - `RenderConfig` - Rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready-to-draw software renderer.
    #[must_use]
    pub fn with_config(width: u32, height: u32, config: RenderConfig) -> Self {
        Self::with_font_provider(width, height, config, NativeSystemFont::deterministic())
    }

    /// Creates a software renderer configured with an explicit `NativeSystemFont` provider.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical width.
    /// - `u32` - Physical height.
    /// - `RenderConfig` - Rendering configuration.
    /// - `NativeSystemFont` - Native OS or deterministic font provider.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready-to-draw software renderer.
    #[must_use]
    pub fn with_font_provider(
        width: u32,
        height: u32,
        config: RenderConfig,
        font_provider: NativeSystemFont,
    ) -> Self {
        Self {
            framebuffer: Framebuffer::allocate(width, height),
            glyph_atlas: GlyphAtlas::default(),
            font_provider,
            measurer: DeterministicTextMeasurer,
            text_layout_cache: TextLayoutCache::with_capacity(TEXT_LAYOUT_CACHE_SLOTS),
            clip_stack: Vec::with_capacity(8),
            rounded_clip_stack: Vec::with_capacity(8),
            antialiasing: config.get_antialiasing(),
            tile_size: config.get_tile_size().max(16),
            base_cache_width: 0,
            base_cache_height: 0,
            base_cache_hash: 0,
            prev_base_hash: 0,
            base_cache_pixels: Vec::new(),
            prepped_scratch: Vec::with_capacity(64),
            rounded_clips_scratch: Vec::with_capacity(16),
            positioned_glyphs_scratch: Vec::with_capacity(256),
            line_glyphs_scratch: Vec::with_capacity(64),
            caret_rects_scratch: Vec::with_capacity(4),
            corner_mask_cache: CornerMaskCache::default(),
        }
    }

    /// Rasterizes `display_list` into the internal `Framebuffer` clipped to `damage` using multi-core parallel horizontal bands and lossless portal backdrop caching.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `&DisplayList` - Backend-agnostic draw command stream.
    /// - `DamageRegion` - Optional dirty damage region for partial rasterization.
    ///
    /// # Returns
    ///
    /// - `(usize, usize)` - Tuple of `(tiles_updated, pixels_shaded)`.
    pub fn render(&mut self, display_list: &DisplayList, damage: DamageRegion) -> (usize, usize) {
        let scale: f32 = display_list.get_scale_factor().max(0.5);
        let phys_w: u32 = ((display_list.get_viewport().get_width() * scale).round() as u32).max(1);
        let phys_h: u32 =
            ((display_list.get_viewport().get_height() * scale).round() as u32).max(1);
        let resized: bool = self.get_mut_framebuffer().resize(phys_w, phys_h);
        if resized {
            self.set_base_cache_hash(0);
            self.set_prev_base_hash(0);
            self.set_base_cache_width(0);
            self.set_base_cache_height(0);
        }
        let effective_damage: DamageRegion = if resized {
            DamageRegion::full()
        } else {
            damage
        };
        let Some(root_clip) = effective_damage.resolve(phys_w, phys_h) else {
            return (0, 0);
        };
        let tile_sz: u32 = self.get_tile_size().max(16);
        let tiles_x: usize = ((root_clip.get_width().ceil() as u32).div_ceil(tile_sz)) as usize;
        let tiles_y: usize = ((root_clip.get_height().ceil() as u32).div_ceil(tile_sz)) as usize;
        let tiles_updated: usize = tiles_x.saturating_mul(tiles_y);
        let commands: &[DrawCommand] = display_list.get_commands().as_slice();
        let split: usize = display_list.get_portal_split_index().min(commands.len());
        let has_portals: bool = split > 0 && split < commands.len();
        let is_full_frame: bool = root_clip.get_x() <= 0.0
            && root_clip.get_y() <= 0.0
            && root_clip.get_width() >= (phys_w as f32)
            && root_clip.get_height() >= (phys_h as f32);
        if has_portals && is_full_frame {
            let base_hash: u64 = display_list.base_layer_hash();
            let fb_len: usize = self.get_framebuffer().get_pixels().len();
            let cache_hit: bool = base_hash != 0
                && self.get_base_cache_hash() == base_hash
                && self.get_base_cache_width() == phys_w
                && self.get_base_cache_height() == phys_h
                && self.get_base_cache_pixels().len() == fb_len;
            if cache_hit {
                self.prepare_frame_range(
                    display_list,
                    (split, commands.len()),
                    root_clip,
                    (phys_w, phys_h, scale),
                );
                let pixels_shaded: usize =
                    self.execute_prepared_bands(commands, root_clip, (phys_w, phys_h, scale), true);
                return (tiles_updated, pixels_shaded.max(1));
            }
            self.prepare_frame_range(display_list, (0, split), root_clip, (phys_w, phys_h, scale));
            let base_shaded: usize =
                self.execute_prepared_bands(commands, root_clip, (phys_w, phys_h, scale), false);
            let base_stable: bool = base_hash != 0 && self.get_prev_base_hash() == base_hash;
            self.set_prev_base_hash(base_hash);
            if base_stable {
                let Self {
                    framebuffer,
                    base_cache_pixels,
                    base_cache_width,
                    base_cache_height,
                    base_cache_hash,
                    ..
                } = self;
                let src_pixels: &[u8] = framebuffer.get_pixels().as_slice();
                if base_cache_pixels.len() != src_pixels.len() {
                    base_cache_pixels.resize(src_pixels.len(), 0);
                }
                base_cache_pixels.copy_from_slice(src_pixels);
                *base_cache_width = phys_w;
                *base_cache_height = phys_h;
                *base_cache_hash = base_hash;
            }
            self.prepare_frame_range(
                display_list,
                (split, commands.len()),
                root_clip,
                (phys_w, phys_h, scale),
            );
            let portal_shaded: usize =
                self.execute_prepared_bands(commands, root_clip, (phys_w, phys_h, scale), false);
            return (tiles_updated, base_shaded.saturating_add(portal_shaded));
        }
        if !has_portals {
            self.set_base_cache_hash(0);
            self.set_prev_base_hash(0);
        }
        self.prepare_frame_range(
            display_list,
            (0, commands.len()),
            root_clip,
            (phys_w, phys_h, scale),
        );
        let pixels_shaded: usize =
            self.execute_prepared_bands(commands, root_clip, (phys_w, phys_h, scale), false);
        (tiles_updated, pixels_shaded)
    }

    /// Executes prepared scratch buffers across horizontal bands with optional parallel restore from `base_cache_pixels`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer holding populated scratch buffers.
    /// - `&[DrawCommand]` - Full frame draw command slice.
    /// - `Rect` - Root physical damage clip rectangle.
    /// - `(u32, u32, f32)` - Tuple `(phys_w, phys_h, scale)`.
    /// - `bool` - Whether to restore `base_cache_pixels` into each strip before rasterizing.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count.
    fn execute_prepared_bands(
        &mut self,
        commands: &[DrawCommand],
        root_clip: Rect,
        dims: (u32, u32, f32),
        restore_base_cache: bool,
    ) -> usize {
        let (phys_w, phys_h, scale): (u32, u32, f32) = dims;
        let clip_y0: u32 = (root_clip.get_y().floor().max(0.0) as u32).min(phys_h);
        let clip_y1: u32 = (root_clip.bottom().ceil().max(0.0) as u32).min(phys_h);
        let clip_w: u32 = (root_clip.get_width().ceil().max(0.0) as u32).min(phys_w);
        let clip_h: u32 = clip_y1.saturating_sub(clip_y0);
        let Self {
            framebuffer,
            glyph_atlas,
            base_cache_pixels,
            prepped_scratch,
            positioned_glyphs_scratch,
            caret_rects_scratch,
            rounded_clips_scratch,
            corner_mask_cache,
            antialiasing,
            ..
        } = self;
        let aa: AntialiasingMode = *antialiasing;
        let prepped_slice: &[PreppedCommand] = prepped_scratch.as_slice();
        let glyphs_slice: &[PositionedGlyph] = positioned_glyphs_scratch.as_slice();
        let caret_rects_slice: &[Rect] = caret_rects_scratch.as_slice();
        let rounded_clips_slice: &[RoundedClipMask] = rounded_clips_scratch.as_slice();
        let corner_masks_slice: &[CachedCornerMask] = corner_mask_cache.entries();
        let atlas_ref: &GlyphAtlas = glyph_atlas;
        let base_restore: Option<&[u8]> = if restore_base_cache {
            Some(base_cache_pixels.as_slice())
        } else {
            None
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let pixel_area: usize = (clip_w as usize).saturating_mul(clip_h as usize);
            if pixel_area >= 65_536 && clip_h >= 64 {
                let pool: &BandWorkerPool = global_band_worker_pool();
                let effective_area: usize =
                    pixel_area.saturating_add(rounded_clips_slice.len().saturating_mul(16_384));
                let desired_threads: usize =
                    (effective_area / 65_536).clamp(1, pool.total_threads());
                let strip_count: usize = (desired_threads.saturating_mul(2))
                    .min((clip_h / 24) as usize)
                    .max(1);
                if strip_count > 1 && desired_threads > 1 {
                    return pool.execute(
                        framebuffer,
                        (
                            clip_y0,
                            clip_y1,
                            strip_count,
                            desired_threads.saturating_sub(1),
                        ),
                        (commands, prepped_slice),
                        (
                            glyphs_slice,
                            caret_rects_slice,
                            rounded_clips_slice,
                            corner_masks_slice,
                        ),
                        atlas_ref,
                        (scale, aa, base_restore),
                    );
                }
            }
            if let Some(src_buf) = base_restore
                && src_buf.len() == framebuffer.get_pixels().len()
            {
                framebuffer.get_mut_pixels().copy_from_slice(src_buf);
            }
            let mut band: PixelBand<'_> = framebuffer.as_band_mut();
            Self::render_band(
                &mut band,
                commands,
                prepped_slice,
                (
                    glyphs_slice,
                    caret_rects_slice,
                    rounded_clips_slice,
                    corner_masks_slice,
                ),
                atlas_ref,
                (scale, aa),
            )
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _: (u32, u32) = (clip_w, clip_h);
            if let Some(src_buf) = base_restore
                && src_buf.len() == framebuffer.get_pixels().len()
            {
                framebuffer.get_mut_pixels().copy_from_slice(src_buf);
            }
            let mut band: PixelBand<'_> = framebuffer.as_band_mut();
            Self::render_band(
                &mut band,
                commands,
                prepped_slice,
                (
                    glyphs_slice,
                    caret_rects_slice,
                    rounded_clips_slice,
                    corner_masks_slice,
                ),
                atlas_ref,
                (scale, aa),
            )
        }
    }

    /// Runs a lightweight main-thread pre-pass over `commands[range.0..range.1]` into reusable scratch buffers to resolve clips, opaque shadow cutouts, cached text layouts, and pre-positioned glyphs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `&DisplayList` - Incoming command stream.
    /// - `(usize, usize)` - Command slice `(start_idx, end_idx)` into `display_list.get_commands()`.
    /// - `Rect` - Root physical damage clip rectangle.
    /// - `(u32, u32, f32)` - Tuple `(phys_w, phys_h, scale)`.
    fn prepare_frame_range(
        &mut self,
        display_list: &DisplayList,
        range: (usize, usize),
        root_clip: Rect,
        dims: (u32, u32, f32),
    ) {
        let (start_idx, end_idx): (usize, usize) = range;
        let (phys_w, phys_h, scale): (u32, u32, f32) = dims;
        let commands: &[DrawCommand] = display_list.get_commands().as_slice();
        let clamped_end: usize = end_idx.min(commands.len());
        let clamped_start: usize = start_idx.min(clamped_end);
        let Self {
            glyph_atlas,
            font_provider,
            measurer,
            text_layout_cache,
            clip_stack,
            rounded_clip_stack,
            prepped_scratch: prepped,
            rounded_clips_scratch: rounded_clips,
            positioned_glyphs_scratch: positioned_glyphs,
            line_glyphs_scratch: line_scratch,
            caret_rects_scratch: caret_rects,
            corner_mask_cache,
            antialiasing,
            ..
        } = self;
        let aa: AntialiasingMode = *antialiasing;
        prepped.clear();
        prepped.reserve(clamped_end - clamped_start);
        rounded_clips.clear();
        positioned_glyphs.clear();
        caret_rects.clear();
        clip_stack.clear();
        clip_stack.push(root_clip);
        rounded_clip_stack.clear();
        rounded_clip_stack.push(None);
        if text_layout_cache.len() > TEXT_LAYOUT_CACHE_LIMIT {
            text_layout_cache.clear();
        }
        if corner_mask_cache.len() > CORNER_MASK_CACHE_LIMIT {
            corner_mask_cache.clear();
        }
        for (offset, cmd) in commands[clamped_start..clamped_end].iter().enumerate() {
            let cmd_index: usize = clamped_start + offset;
            let active_clip: Rect = clip_stack.last().copied().unwrap_or(root_clip);
            match cmd {
                DrawCommand::PushClip(rect) => {
                    let phys: Rect = snap_physical_rect(*rect, scale);
                    let next: Rect = active_clip.intersect(phys).unwrap_or_default();
                    clip_stack.push(next);
                    rounded_clip_stack.push(None);
                }
                DrawCommand::PushRoundedClip { rect, radius } => {
                    let phys: Rect = snap_physical_rect(*rect, scale);
                    let next: Rect = active_clip.intersect(phys).unwrap_or_default();
                    clip_stack.push(next);
                    let phys_r: CornerRadius = Self::scale_radius(*radius, scale);
                    if phys.get_width() > 0.0
                        && phys.get_height() > 0.0
                        && phys_r.max_radius() > 0.25
                    {
                        let corner_mask_ids: [u32; 4] =
                            corner_mask_cache.resolve_quadrants(phys, phys_r, aa);
                        rounded_clip_stack.push(Some(RoundedClipMask {
                            rect: phys,
                            radius: phys_r,
                            corner_mask_ids,
                        }));
                    } else {
                        rounded_clip_stack.push(None);
                    }
                }
                DrawCommand::PopClip => {
                    if clip_stack.len() > 1 {
                        clip_stack.pop();
                        let _: Option<Option<RoundedClipMask>> = rounded_clip_stack.pop();
                    }
                }
                DrawCommand::Clear(_) => {
                    let y0: u32 = (active_clip.get_y().floor().max(0.0) as u32).min(phys_h);
                    let y1: u32 = (active_clip.bottom().ceil().max(0.0) as u32).min(phys_h);
                    if y0 < y1 {
                        let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                            rounded_clip_stack.as_slice(),
                            rounded_clips,
                            active_clip,
                        );
                        prepped.push(PreppedCommand {
                            cmd_index,
                            active_clip,
                            y0,
                            y1,
                            opaque_cutout_h: Rect::default(),
                            opaque_cutout_v: Rect::default(),
                            glyph_run_start: 0,
                            glyph_run_len: 0,
                            caret_rect_idx: u32::MAX,
                            rounded_clip_start: rc_start,
                            rounded_clip_len: rc_len,
                        });
                    }
                }
                DrawCommand::FillRoundedRect { rect, .. }
                | DrawCommand::LinearGradientRect { rect, .. }
                | DrawCommand::Image { rect, .. }
                | DrawCommand::SpriteRegion { rect, .. } => {
                    let phys: Rect = snap_physical_rect(*rect, scale);
                    if let Some(vis) = phys.intersect(active_clip) {
                        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                        if y0 < y1 {
                            let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                                rounded_clip_stack.as_slice(),
                                rounded_clips,
                                vis,
                            );
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                glyph_run_start: 0,
                                glyph_run_len: 0,
                                caret_rect_idx: u32::MAX,
                                rounded_clip_start: rc_start,
                                rounded_clip_len: rc_len,
                            });
                        }
                    }
                }
                DrawCommand::BoxShadow { rect, shadow, .. } => {
                    if shadow.get_color().get_a() == 0 {
                        continue;
                    }
                    let spread: f32 = shadow.get_spread_radius() * scale;
                    let blur: f32 = (shadow.get_blur_radius().max(0.5) * scale).min(96.0);
                    let sigma: f32 = (blur * 0.5).max(0.75);
                    let pad: f32 = (sigma * 3.0).ceil();
                    let base_rect: Rect = snap_physical_rect(*rect, scale);
                    let caster_rect: Rect = Rect::new(
                        base_rect.get_x() + (shadow.get_offset_x() * scale).round() - spread,
                        base_rect.get_y() + (shadow.get_offset_y() * scale).round() - spread,
                        (base_rect.get_width() + spread * 2.0).max(1.0),
                        (base_rect.get_height() + spread * 2.0).max(1.0),
                    );
                    let shadow_rect: Rect = Rect::new(
                        caster_rect.get_x() - pad,
                        caster_rect.get_y() - pad,
                        caster_rect.get_width() + pad * 2.0,
                        caster_rect.get_height() + pad * 2.0,
                    );
                    if let Some(vis) = shadow_rect.intersect(active_clip) {
                        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                        if y0 < y1 {
                            let (cut_h, cut_v): (Rect, Rect) = Self::resolve_opaque_shadow_cutout(
                                commands.get(cmd_index + 1),
                                scale,
                            );
                            let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                                rounded_clip_stack.as_slice(),
                                rounded_clips,
                                vis,
                            );
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: cut_h,
                                opaque_cutout_v: cut_v,
                                glyph_run_start: 0,
                                glyph_run_len: 0,
                                caret_rect_idx: u32::MAX,
                                rounded_clip_start: rc_start,
                                rounded_clip_len: rc_len,
                            });
                        }
                    }
                }
                DrawCommand::TextRun { rect, text, style } => {
                    if text.is_empty() || style.get_color().get_a() == 0 {
                        continue;
                    }
                    let phys_rect: Rect = Self::scale_rect(*rect, scale);
                    let pad: f32 = style.get_font_size() * scale * 0.5;
                    let expanded: Rect = Rect::new(
                        phys_rect.get_x() - pad,
                        phys_rect.get_y() - pad,
                        phys_rect.get_width() + pad * 2.0,
                        phys_rect.get_height() + pad * 2.0,
                    );
                    let Some(vis) = expanded.intersect(active_clip) else {
                        continue;
                    };
                    let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                    let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                    if y0 >= y1 {
                        continue;
                    }
                    let max_w: f32 = rect.get_width().max(1.0);
                    let width_key: u32 = ((max_w * 10.0).round() as u32).min(u32::MAX - 1);
                    let key_hash: u64 = hash_text_layout_key(text, *style);
                    let layout_idx: u32 =
                        text_layout_cache.get_or_insert_with(key_hash, width_key, || {
                            measurer.measure(text, *style, Some(max_w))
                        });
                    let (glyph_run_start, glyph_run_len): (u32, u32) =
                        if let Some(layout) = text_layout_cache.get(layout_idx) {
                            Self::resolve_text_run_glyphs(
                                layout,
                                *rect,
                                *style,
                                (active_clip, phys_w, phys_h, scale),
                                (glyph_atlas, font_provider),
                                positioned_glyphs,
                                line_scratch,
                            )
                        } else {
                            (0, 0)
                        };
                    if glyph_run_len == 0 {
                        continue;
                    }
                    let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                        rounded_clip_stack.as_slice(),
                        rounded_clips,
                        vis,
                    );
                    prepped.push(PreppedCommand {
                        cmd_index,
                        active_clip,
                        y0,
                        y1,
                        opaque_cutout_h: Rect::default(),
                        opaque_cutout_v: Rect::default(),
                        glyph_run_start,
                        glyph_run_len,
                        caret_rect_idx: u32::MAX,
                        rounded_clip_start: rc_start,
                        rounded_clip_len: rc_len,
                    });
                }
                DrawCommand::TextCaret {
                    text_rect,
                    content_rect,
                    text,
                    style,
                    caret_height,
                    color,
                    multiline,
                } => {
                    if color.get_a() == 0 {
                        continue;
                    }
                    let phys_content: Rect = Self::scale_rect(*content_rect, scale);
                    let Some(vis) = phys_content.intersect(active_clip) else {
                        continue;
                    };
                    let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                    let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                    if y0 >= y1 {
                        continue;
                    }
                    let (max_w, width_key): (Option<f32>, u32) = if *multiline {
                        let w: f32 = content_rect.get_width().max(1.0);
                        (Some(w), ((w * 10.0).round() as u32).min(u32::MAX - 1))
                    } else {
                        (None, u32::MAX)
                    };
                    let key_hash: u64 = hash_text_layout_key(text, *style);
                    let layout_idx: u32 =
                        text_layout_cache.get_or_insert_with(key_hash, width_key, || {
                            measurer.measure(text, *style, max_w)
                        });
                    let caret_rect: Rect = Self::resolve_text_caret_rect(
                        text_layout_cache.get(layout_idx),
                        (*text_rect, *content_rect),
                        *style,
                        (*caret_height, scale, *multiline),
                        (glyph_atlas, font_provider),
                    );
                    let caret_rect_idx: u32 = caret_rects.len() as u32;
                    caret_rects.push(caret_rect);
                    let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                        rounded_clip_stack.as_slice(),
                        rounded_clips,
                        vis,
                    );
                    prepped.push(PreppedCommand {
                        cmd_index,
                        active_clip,
                        y0,
                        y1,
                        opaque_cutout_h: Rect::default(),
                        opaque_cutout_v: Rect::default(),
                        glyph_run_start: 0,
                        glyph_run_len: 0,
                        caret_rect_idx,
                        rounded_clip_start: rc_start,
                        rounded_clip_len: rc_len,
                    });
                }
                DrawCommand::Line {
                    from, to, width, ..
                } => {
                    let half_w: f32 = (width * scale).max(1.0) * 0.5 + 1.0;
                    let min_x: f32 = from.get_x().min(to.get_x()) * scale - half_w;
                    let min_y: f32 = from.get_y().min(to.get_y()) * scale - half_w;
                    let max_x: f32 = from.get_x().max(to.get_x()) * scale + half_w;
                    let max_y: f32 = from.get_y().max(to.get_y()) * scale + half_w;
                    if let Some(vis) =
                        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y).intersect(active_clip)
                    {
                        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                        if y0 < y1 {
                            let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                                rounded_clip_stack.as_slice(),
                                rounded_clips,
                                vis,
                            );
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                glyph_run_start: 0,
                                glyph_run_len: 0,
                                caret_rect_idx: u32::MAX,
                                rounded_clip_start: rc_start,
                                rounded_clip_len: rc_len,
                            });
                        }
                    }
                }
                DrawCommand::Circle { center, radius, .. } => {
                    let r: f32 = radius * scale + 1.0;
                    let bounds: Rect = Rect::new(
                        center.get_x() * scale - r,
                        center.get_y() * scale - r,
                        r * 2.0,
                        r * 2.0,
                    );
                    if let Some(vis) = bounds.intersect(active_clip) {
                        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                        if y0 < y1 {
                            let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                                rounded_clip_stack.as_slice(),
                                rounded_clips,
                                vis,
                            );
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                glyph_run_start: 0,
                                glyph_run_len: 0,
                                caret_rect_idx: u32::MAX,
                                rounded_clip_start: rc_start,
                                rounded_clip_len: rc_len,
                            });
                        }
                    }
                }
                DrawCommand::Triangle { p0, p1, p2, .. } => {
                    let min_x: f32 = p0.get_x().min(p1.get_x()).min(p2.get_x()) * scale - 1.0;
                    let min_y: f32 = p0.get_y().min(p1.get_y()).min(p2.get_y()) * scale - 1.0;
                    let max_x: f32 = p0.get_x().max(p1.get_x()).max(p2.get_x()) * scale + 1.0;
                    let max_y: f32 = p0.get_y().max(p1.get_y()).max(p2.get_y()) * scale + 1.0;
                    if let Some(vis) =
                        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y).intersect(active_clip)
                    {
                        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(phys_h);
                        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(phys_h);
                        if y0 < y1 {
                            let (rc_start, rc_len): (u32, u16) = Self::collect_active_rounded(
                                rounded_clip_stack.as_slice(),
                                rounded_clips,
                                vis,
                            );
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                glyph_run_start: 0,
                                glyph_run_len: 0,
                                caret_rect_idx: u32::MAX,
                                rounded_clip_start: rc_start,
                                rounded_clip_len: rc_len,
                            });
                        }
                    }
                }
            }
        }
    }

    /// Resolves and positions every visible glyph of a `TextRun` once on the main thread using direct ASCII slot tables.
    ///
    /// # Arguments
    ///
    /// - `&TextLayoutResult` - Cached wrapped lines for the text run.
    /// - `Rect` - Logical text bounding box.
    /// - `TextStyle` - Typography style.
    /// - `(Rect, u32, u32, f32)` - Tuple `(active_clip, phys_w, phys_h, scale)`.
    /// - `(&mut GlyphAtlas, &NativeSystemFont)` - Mutable glyph atlas and system font provider.
    /// - `&mut Vec<PositionedGlyph>` - Frame-wide flat output buffer for pre-positioned glyphs.
    /// - `&mut Vec<(u32, f32, u16, u16)>` - Reusable single-line scratch buffer for `Center`/`Right` alignment.
    ///
    /// # Returns
    ///
    /// - `(u32, u32)` - `(start_index, count)` range inside `out_glyphs`.
    fn resolve_text_run_glyphs(
        layout: &TextLayoutResult,
        rect: Rect,
        style: TextStyle,
        clip_dims: (Rect, u32, u32, f32),
        atlas_and_font: (&mut GlyphAtlas, &NativeSystemFont),
        out_glyphs: &mut Vec<PositionedGlyph>,
        line_scratch: &mut Vec<(u32, f32, u16, u16)>,
    ) -> (u32, u32) {
        let (clip, phys_w, phys_h, scale): (Rect, u32, u32, f32) = clip_dims;
        let (glyph_atlas, font_provider): (&mut GlyphAtlas, &NativeSystemFont) = atlas_and_font;
        let clip_x0: i32 = clip.get_x().ceil().max(0.0) as i32;
        let clip_y0: i32 = (clip.get_y().ceil().max(0.0) as i32).min(phys_h as i32);
        let clip_x1: i32 = (clip.right().floor().max(0.0) as i32).min(phys_w as i32);
        let clip_y1: i32 = (clip.bottom().floor().max(0.0) as i32).min(phys_h as i32);
        if clip_x0 >= clip_x1 || clip_y0 >= clip_y1 {
            return (0, 0);
        }
        let phys_font_size: f32 = style.get_font_size() * scale;
        let size_tenths: u16 = (phys_font_size.clamp(6.0, 256.0) * 10.0 + 0.5) as u16;
        let bold: bool = style.get_weight().is_bold();
        let monospace: bool = style.get_family() == FontFamily::Monospace;
        let italic: bool = style.get_italic();
        let key_meta: (u16, bool, bool, bool) = (size_tenths, bold, monospace, italic);
        let ascii_table_idx: usize =
            glyph_atlas.ascii_table_index(size_tenths, bold, monospace, italic);
        let line_h_phys: f32 = layout.get_line_height_px() * scale;
        let box_w_phys: f32 = rect.get_width() * scale;
        let letter_sp: f32 = style.get_letter_spacing() * scale;
        let space_step: f32 = measure_char_advance(' ', style) * scale + letter_sp;
        let base_x_phys: f32 = (rect.get_x() * scale).round();
        let base_y_phys: f32 = (rect.get_y() * scale).round();
        let clip_x1_f: f32 = clip_x1 as f32;
        let clip_y0_f: f32 = clip_y0 as f32;
        let clip_y1_f: f32 = clip_y1 as f32;
        let align: TextAlign = style.get_align();
        let run_start: u32 = out_glyphs.len() as u32;
        for (line_idx, line) in layout.get_lines().iter().enumerate() {
            let line_off_y: f32 = ((line_idx as f32) * line_h_phys).round();
            let pen_y: f32 = base_y_phys + line_off_y;
            if (pen_y + line_h_phys) < clip_y0_f || pen_y > clip_y1_f {
                continue;
            }
            let line_str: &str = line.get_text().as_str();
            if align == TextAlign::Left {
                let mut pen_x: f32 = base_x_phys;
                if line_str.is_ascii() {
                    for &byte in line_str.as_bytes() {
                        if byte == b' ' {
                            pen_x += space_step;
                            continue;
                        }
                        if pen_x > clip_x1_f {
                            break;
                        }
                        let slot: u32 = glyph_atlas.get_or_rasterize_ascii_slot_with(
                            ascii_table_idx,
                            byte,
                            key_meta,
                            font_provider,
                        );
                        let glyph: &RasterizedGlyph = glyph_atlas.glyph_by_slot(slot);
                        let adv_phys: f32 = (glyph.get_advance() as f32).max(1.0) + letter_sp;
                        let gx0: i32 = pen_x.round() as i32;
                        let gw: i32 = glyph.get_width() as i32;
                        if gx0 + gw > clip_x0 && gx0 < clip_x1 {
                            let gh: u32 = glyph.get_height();
                            let glyph_v_off: f32 = ((line_h_phys - (gh as f32)) * 0.5).round();
                            let gy0: i32 = (pen_y + glyph_v_off) as i32;
                            if gy0 < clip_y1 && gy0 + (gh as i32) > clip_y0 {
                                out_glyphs.push(PositionedGlyph {
                                    glyph_idx: slot,
                                    x: gx0,
                                    y: gy0,
                                    width: (gw as u32).min(u16::MAX as u32) as u16,
                                    height: gh.min(u16::MAX as u32) as u16,
                                });
                            }
                        }
                        pen_x += adv_phys;
                    }
                } else {
                    for ch in line_str.chars() {
                        if ch == ' ' {
                            pen_x += space_step;
                            continue;
                        }
                        if pen_x > clip_x1_f {
                            break;
                        }
                        let slot: u32 = if ch.is_ascii() {
                            glyph_atlas.get_or_rasterize_ascii_slot_with(
                                ascii_table_idx,
                                ch as u8,
                                key_meta,
                                font_provider,
                            )
                        } else {
                            let key: GlyphKey = GlyphKey {
                                codepoint: ch,
                                size_tenths,
                                bold,
                                monospace,
                                italic,
                            };
                            glyph_atlas.get_or_rasterize_slot_with(key, font_provider)
                        };
                        let glyph: &RasterizedGlyph = glyph_atlas.glyph_by_slot(slot);
                        let adv_phys: f32 = (glyph.get_advance() as f32).max(1.0) + letter_sp;
                        let gx0: i32 = pen_x.round() as i32;
                        let gw: i32 = glyph.get_width() as i32;
                        if gx0 + gw > clip_x0 && gx0 < clip_x1 {
                            let gh: u32 = glyph.get_height();
                            let glyph_v_off: f32 = ((line_h_phys - (gh as f32)) * 0.5).round();
                            let gy0: i32 = (pen_y + glyph_v_off) as i32;
                            if gy0 < clip_y1 && gy0 + (gh as i32) > clip_y0 {
                                out_glyphs.push(PositionedGlyph {
                                    glyph_idx: slot,
                                    x: gx0,
                                    y: gy0,
                                    width: (gw as u32).min(u16::MAX as u32) as u16,
                                    height: gh.min(u16::MAX as u32) as u16,
                                });
                            }
                        }
                        pen_x += adv_phys;
                    }
                }
            } else {
                line_scratch.clear();
                let mut line_w_phys: f32 = 0.0;
                if line_str.is_ascii() {
                    for &byte in line_str.as_bytes() {
                        if byte == b' ' {
                            line_w_phys += space_step;
                            line_scratch.push((u32::MAX, space_step, 0, 0));
                        } else {
                            let slot: u32 = glyph_atlas.get_or_rasterize_ascii_slot_with(
                                ascii_table_idx,
                                byte,
                                key_meta,
                                font_provider,
                            );
                            let glyph: &RasterizedGlyph = glyph_atlas.glyph_by_slot(slot);
                            let adv_phys: f32 = (glyph.get_advance() as f32).max(1.0) + letter_sp;
                            line_w_phys += adv_phys;
                            line_scratch.push((
                                slot,
                                adv_phys,
                                glyph.get_width().min(u16::MAX as u32) as u16,
                                glyph.get_height().min(u16::MAX as u32) as u16,
                            ));
                        }
                    }
                } else {
                    for ch in line_str.chars() {
                        if ch == ' ' {
                            line_w_phys += space_step;
                            line_scratch.push((u32::MAX, space_step, 0, 0));
                        } else {
                            let slot: u32 = if ch.is_ascii() {
                                glyph_atlas.get_or_rasterize_ascii_slot_with(
                                    ascii_table_idx,
                                    ch as u8,
                                    key_meta,
                                    font_provider,
                                )
                            } else {
                                let key: GlyphKey = GlyphKey {
                                    codepoint: ch,
                                    size_tenths,
                                    bold,
                                    monospace,
                                    italic,
                                };
                                glyph_atlas.get_or_rasterize_slot_with(key, font_provider)
                            };
                            let glyph: &RasterizedGlyph = glyph_atlas.glyph_by_slot(slot);
                            let adv_phys: f32 = (glyph.get_advance() as f32).max(1.0) + letter_sp;
                            line_w_phys += adv_phys;
                            line_scratch.push((
                                slot,
                                adv_phys,
                                glyph.get_width().min(u16::MAX as u32) as u16,
                                glyph.get_height().min(u16::MAX as u32) as u16,
                            ));
                        }
                    }
                }
                let align_x: f32 = if align == TextAlign::Center {
                    ((box_w_phys - line_w_phys) * 0.5).max(0.0)
                } else {
                    (box_w_phys - line_w_phys).max(0.0)
                };
                let mut pen_x: f32 = base_x_phys + align_x.round();
                for &(slot, adv_phys, gw_u16, gh_u16) in line_scratch.iter() {
                    if slot == u32::MAX {
                        pen_x += adv_phys;
                        continue;
                    }
                    if pen_x > clip_x1_f {
                        break;
                    }
                    let gx0: i32 = pen_x.round() as i32;
                    let gw: i32 = gw_u16 as i32;
                    let gh: i32 = gh_u16 as i32;
                    if gx0 + gw > clip_x0 && gx0 < clip_x1 {
                        let glyph_v_off: f32 = ((line_h_phys - (gh as f32)) * 0.5).round();
                        let gy0: i32 = (pen_y + glyph_v_off) as i32;
                        if gy0 < clip_y1 && gy0 + gh > clip_y0 {
                            out_glyphs.push(PositionedGlyph {
                                glyph_idx: slot,
                                x: gx0,
                                y: gy0,
                                width: gw_u16,
                                height: gh_u16,
                            });
                        }
                    }
                    pen_x += adv_phys;
                }
            }
        }
        let run_len: u32 = (out_glyphs.len() as u32).saturating_sub(run_start);
        (run_start, run_len)
    }

    /// Precomputes the exact physical pixel rectangle for `DrawCommand::TextCaret` using direct ASCII slot lookup.
    ///
    /// # Arguments
    ///
    /// - `Option<&TextLayoutResult>` - Cached wrapped text layout, if present.
    /// - `(Rect, Rect)` - Tuple `(text_rect, content_rect)` in logical pixels.
    /// - `TextStyle` - Typography style.
    /// - `(f32, f32, bool)` - Tuple `(caret_height, scale, multiline)`.
    /// - `(&mut GlyphAtlas, &NativeSystemFont)` - Mutable glyph atlas and system font provider.
    ///
    /// # Returns
    ///
    /// - `Rect` - Physical pixel caret rectangle ready for band rasterization.
    #[must_use]
    fn resolve_text_caret_rect(
        layout_opt: Option<&TextLayoutResult>,
        rects: (Rect, Rect),
        style: TextStyle,
        caret_opts: (f32, f32, bool),
        atlas_and_font: (&mut GlyphAtlas, &NativeSystemFont),
    ) -> Rect {
        let (text_rect, content_rect): (Rect, Rect) = rects;
        let (caret_height, scale, multiline): (f32, f32, bool) = caret_opts;
        let (glyph_atlas, font_provider): (&mut GlyphAtlas, &NativeSystemFont) = atlas_and_font;
        let phys_font_size: f32 = style.get_font_size() * scale;
        let size_tenths: u16 = (phys_font_size.clamp(6.0, 256.0) * 10.0 + 0.5) as u16;
        let bold: bool = style.get_weight().is_bold();
        let monospace: bool = style.get_family() == FontFamily::Monospace;
        let italic: bool = style.get_italic();
        let key_meta: (u16, bool, bool, bool) = (size_tenths, bold, monospace, italic);
        let letter_sp: f32 = style.get_letter_spacing() * scale;
        let space_step: f32 = measure_char_advance(' ', style) * scale + letter_sp;
        let (last_w_phys, line_idx, line_h_phys): (f32, usize, f32) =
            if let Some(layout) = layout_opt {
                let lh: f32 = layout.get_line_height_px() * scale;
                let idx: usize = layout.get_lines().len().saturating_sub(1);
                let w_phys: f32 = if let Some(line) = layout.get_lines().last() {
                    let ascii_table_idx: usize =
                        glyph_atlas.ascii_table_index(size_tenths, bold, monospace, italic);
                    let mut acc: f32 = 0.0;
                    for ch in line.get_text().chars() {
                        if ch == ' ' {
                            acc += space_step;
                        } else {
                            let slot: u32 = if ch.is_ascii() {
                                glyph_atlas.get_or_rasterize_ascii_slot_with(
                                    ascii_table_idx,
                                    ch as u8,
                                    key_meta,
                                    font_provider,
                                )
                            } else {
                                let key: GlyphKey = GlyphKey {
                                    codepoint: ch,
                                    size_tenths,
                                    bold,
                                    monospace,
                                    italic,
                                };
                                glyph_atlas.get_or_rasterize_slot_with(key, font_provider)
                            };
                            let adv: f32 =
                                (glyph_atlas.glyph_by_slot(slot).get_advance() as f32).max(1.0);
                            acc += adv + letter_sp;
                        }
                    }
                    acc
                } else {
                    0.0
                };
                (w_phys, idx, lh)
            } else {
                (
                    0.0,
                    0,
                    (style.get_font_size() * style.get_line_height()).max(1.0) * scale,
                )
            };
        let caret_h_phys: f32 = (caret_height * scale).max(2.0);
        let caret_w_phys: f32 = (1.5 * scale).max(1.5);
        let max_x: f32 =
            (content_rect.right() * scale - caret_w_phys - 1.0).max(content_rect.get_x() * scale);
        let caret_x_phys: f32 = (text_rect.get_x() * scale + last_w_phys + 1.5 * scale).min(max_x);
        let caret_y_phys: f32 = if multiline {
            let raw_y: f32 = content_rect.get_y() * scale
                + (line_idx as f32) * line_h_phys
                + ((line_h_phys - caret_h_phys) * 0.5).max(0.0);
            raw_y
                .min(content_rect.bottom() * scale - caret_h_phys)
                .max(content_rect.get_y() * scale)
        } else {
            content_rect.get_y() * scale
                + ((content_rect.get_height() * scale - caret_h_phys) * 0.5).max(0.0)
        };
        Rect::new(
            caret_x_phys.round(),
            caret_y_phys.round(),
            caret_w_phys.round().max(1.0),
            caret_h_phys.round().max(2.0),
        )
    }

    /// Collects active rounded clip masks intersecting `vis` into `out` and returns their `(start, len)` span.
    ///
    /// # Arguments
    ///
    /// - `&[Option<RoundedClipMask>]` - Current rounded clip stack.
    /// - `&mut Vec<RoundedClipMask>` - Frame-wide rounded clip output buffer.
    /// - `Rect` - Visible clipped physical bounds of the command.
    ///
    /// # Returns
    ///
    /// - `(u32, u16)` - `(start_index, mask_count)` slice range inside `out`.
    #[must_use]
    fn collect_active_rounded(
        stack: &[Option<RoundedClipMask>],
        out: &mut Vec<RoundedClipMask>,
        vis: Rect,
    ) -> (u32, u16) {
        let start: u32 = out.len() as u32;
        for entry in stack.iter().copied().flatten() {
            if rect_intersects_rounded_corners(vis, entry.get_rect(), entry.get_radius()) {
                out.push(entry);
            }
        }
        let len: u16 = (out.len() as u32)
            .saturating_sub(start)
            .min(u16::MAX as u32) as u16;
        (start, len)
    }

    /// Computes two strictly-interior opaque cutout rectangles `(horizontal_bar, vertical_bar)` if the next command is an opaque `FillRoundedRect`.
    ///
    /// # Arguments
    ///
    /// - `Option<&DrawCommand>` - Immediately following command in the display list.
    /// - `f32` - Active DPI scale factor.
    ///
    /// # Returns
    ///
    /// - `(Rect, Rect)` - Opaque inner rectangles safely skipped by `raster_shadow` with zero visual change.
    #[must_use]
    fn resolve_opaque_shadow_cutout(next_cmd: Option<&DrawCommand>, scale: f32) -> (Rect, Rect) {
        let Some(DrawCommand::FillRoundedRect {
            rect,
            color,
            radius,
            ..
        }) = next_cmd
        else {
            return (Rect::default(), Rect::default());
        };
        if color.get_a() != 255 {
            return (Rect::default(), Rect::default());
        }
        let phys: Rect = snap_physical_rect(*rect, scale);
        let r_max: f32 = Self::scale_radius(*radius, scale)
            .max_radius()
            .min(phys.get_width() * 0.5)
            .min(phys.get_height() * 0.5);
        let pad: f32 = 1.5;
        let h_w: f32 = (phys.get_width() - pad * 2.0).max(0.0);
        let h_h: f32 = (phys.get_height() - (r_max + pad) * 2.0).max(0.0);
        let cut_h: Rect = if h_w > 0.0 && h_h > 0.0 {
            Rect::new(phys.get_x() + pad, phys.get_y() + r_max + pad, h_w, h_h)
        } else {
            Rect::default()
        };
        let v_w: f32 = (phys.get_width() - (r_max + pad) * 2.0).max(0.0);
        let v_h: f32 = (phys.get_height() - pad * 2.0).max(0.0);
        let cut_v: Rect = if v_w > 0.0 && v_h > 0.0 {
            Rect::new(phys.get_x() + r_max + pad, phys.get_y() + pad, v_w, v_h)
        } else {
            Rect::default()
        };
        (cut_h, cut_v)
    }

    /// Saves only the non-interior (`cov < 16`) corner rows of a single cached `RoundedClipMask` via bulk row copies.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `RoundedClipMask` - Single active rounded clip mask.
    /// - `Rect` - Active band-clipped bounds.
    /// - `(u32, u32, u32)` - Tuple `(fb_w, band_y0, band_y1)`.
    /// - `&[CachedCornerMask]` - Read-only cached quadrant masks.
    /// - `&mut BandCornerScratch` - Reusable per-thread scratch storage.
    ///
    /// # Returns
    ///
    /// - `bool` - True when at least one dirty corner row was saved (or `false` if no dirty pixels intersect).
    fn save_single_cached_rounded_corners(
        band: &mut PixelBand<'_>,
        mask: RoundedClipMask,
        clip: Rect,
        band_bounds: (u32, u32, u32),
        corner_masks: &[CachedCornerMask],
        scratch: &mut BandCornerScratch,
    ) -> bool {
        let (fb_w, band_y0, band_y1): (u32, u32, u32) = band_bounds;
        let clip_x0: i32 = (clip.get_x().floor().max(0.0) as u32).min(fb_w) as i32;
        let clip_y0: i32 = (clip.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1) as i32;
        let clip_x1: i32 = (clip.right().ceil().max(0.0) as u32).min(fb_w) as i32;
        let clip_y1: i32 = (clip.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1) as i32;
        if clip_x0 >= clip_x1 || clip_y0 >= clip_y1 {
            return false;
        }
        let rect: Rect = mask.get_rect();
        let rx0_i: i32 = rect.get_x().round() as i32;
        let ry0_i: i32 = rect.get_y().round() as i32;
        let rx1_i: i32 = rect.right().round() as i32;
        let ry1_i: i32 = rect.bottom().round() as i32;
        let mask_ids: [u32; 4] = mask.get_corner_mask_ids();
        scratch.saved_bytes.clear();
        scratch.saved_rows.clear();
        for (q, &mask_id) in mask_ids.iter().enumerate() {
            if mask_id == u32::MAX {
                continue;
            }
            let Some(cm) = corner_masks.get(mask_id as usize) else {
                continue;
            };
            let dim_i: i32 = cm.dim as i32;
            let origin_x: i32 = if q == 0 || q == 3 {
                rx0_i
            } else {
                rx1_i - dim_i
            };
            let origin_y: i32 = if q == 0 || q == 1 {
                ry0_i
            } else {
                ry1_i - dim_i
            };
            let bx0: i32 = origin_x.max(clip_x0);
            let bx1: i32 = (origin_x + dim_i).min(clip_x1);
            let by0: i32 = origin_y.max(clip_y0);
            let by1: i32 = (origin_y + dim_i).min(clip_y1);
            if bx0 >= bx1 || by0 >= by1 {
                continue;
            }
            for y_i in by0..by1 {
                let my: usize = (y_i - origin_y) as usize;
                let span: CornerMaskRowSpan = cm.row_spans[my];
                if span.dirty_start >= span.dirty_end {
                    continue;
                }
                let sx0_i: i32 = (origin_x + (span.dirty_start as i32)).max(bx0);
                let sx1_i: i32 = (origin_x + (span.dirty_end as i32)).min(bx1);
                if sx0_i >= sx1_i {
                    continue;
                }
                let y: u32 = y_i as u32;
                let sx0: u32 = sx0_i as u32;
                let sx1: u32 = sx1_i as u32;
                if let Some(row) = band.row_slice_mut(y) {
                    let b0: usize = (sx0 as usize) * 4;
                    let b1: usize = (sx1 as usize) * 4;
                    let byte_off: usize = scratch.saved_bytes.len();
                    scratch.saved_bytes.extend_from_slice(&row[b0..b1]);
                    scratch.saved_rows.push(SavedCornerRow {
                        y,
                        sx0,
                        sx1,
                        byte_off,
                        mask_id,
                        origin_x,
                        origin_y,
                    });
                }
            }
        }
        !scratch.saved_rows.is_empty()
    }

    /// Restores exterior corner runs via bulk `copy_from_slice` and blends only the 1-3px anti-aliased boundary pixels.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&[CachedCornerMask]` - Read-only cached quadrant masks.
    /// - `&BandCornerScratch` - Saved corner bytes and row descriptors.
    fn restore_single_cached_rounded_corners(
        band: &mut PixelBand<'_>,
        corner_masks: &[CachedCornerMask],
        scratch: &BandCornerScratch,
    ) {
        let saved_bytes: &[u8] = scratch.saved_bytes.as_slice();
        for entry in scratch.saved_rows.iter() {
            let Some(row) = band.row_slice_mut(entry.y) else {
                continue;
            };
            let cm: &CachedCornerMask = &corner_masks[entry.mask_id as usize];
            let dim: usize = cm.dim as usize;
            let my: usize = ((entry.y as i32) - entry.origin_y) as usize;
            let span: CornerMaskRowSpan = cm.row_spans[my];
            let sx0_i: i32 = entry.sx0 as i32;
            let sx1_i: i32 = entry.sx1 as i32;
            if span.ext_start < span.ext_end {
                let ex0: i32 = (entry.origin_x + (span.ext_start as i32)).max(sx0_i);
                let ex1: i32 = (entry.origin_x + (span.ext_end as i32)).min(sx1_i);
                if ex0 < ex1 {
                    let dst_b0: usize = (ex0 as usize) * 4;
                    let dst_b1: usize = (ex1 as usize) * 4;
                    let src_b0: usize = entry.byte_off + ((ex0 - sx0_i) as usize) * 4;
                    let src_b1: usize = src_b0 + (dst_b1 - dst_b0);
                    row[dst_b0..dst_b1].copy_from_slice(&saved_bytes[src_b0..src_b1]);
                }
            }
            if span.aa_start < span.aa_end {
                let ax0: i32 = (entry.origin_x + (span.aa_start as i32)).max(sx0_i);
                let ax1: i32 = (entry.origin_x + (span.aa_end as i32)).min(sx1_i);
                if ax0 < ax1 {
                    let cov_row: &[u8] = &cm.cov[my * dim..(my + 1) * dim];
                    for x_i in ax0..ax1 {
                        let mx: usize = (x_i - entry.origin_x) as usize;
                        let cov_u8: u8 = cov_row[mx];
                        if cov_u8 >= 16 {
                            continue;
                        }
                        let dst_off: usize = (x_i as usize) * 4;
                        let src_off: usize = entry.byte_off + ((x_i - sx0_i) as usize) * 4;
                        let saved_rgba: [u8; 4] = [
                            saved_bytes[src_off],
                            saved_bytes[src_off + 1],
                            saved_bytes[src_off + 2],
                            saved_bytes[src_off + 3],
                        ];
                        if cov_u8 == 0 {
                            row[dst_off..dst_off + 4].copy_from_slice(&saved_rgba);
                        } else {
                            let cov: f32 = (cov_u8 as f32) * 0.0625;
                            lerp_pixel_slice_from_saved_linear(
                                &mut row[dst_off..dst_off + 4],
                                saved_rgba,
                                cov,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Executes all prepped commands overlapping `band`'s vertical slice `[y_offset, y_end)` completely lock-free.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Disjoint mutable horizontal pixel band.
    /// - `&[DrawCommand]` - Full frame draw command slice.
    /// - `&[PreppedCommand]` - Pre-clipped command list.
    /// - `BandAuxSlices<'_>` - Pre-positioned glyphs, caret rectangles, active rounded clip masks, and cached corner masks.
    /// - `&GlyphAtlas` - Immutable pre-warmed glyph atlas.
    /// - `(f32, AntialiasingMode)` - Tuple `(scale, antialiasing)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixels in this band.
    pub(crate) fn render_band(
        band: &mut PixelBand<'_>,
        commands: &[DrawCommand],
        prepped: &[PreppedCommand],
        aux: BandAuxSlices<'_>,
        glyph_atlas: &GlyphAtlas,
        render_opts: (f32, AntialiasingMode),
    ) -> usize {
        let (positioned_glyphs, caret_rects, rounded_clips, corner_masks): (
            &[PositionedGlyph],
            &[Rect],
            &[RoundedClipMask],
            &[CachedCornerMask],
        ) = aux;
        let (scale, aa): (f32, AntialiasingMode) = render_opts;
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let fb_w: u32 = band.width();
        let band_rect: Rect =
            Rect::new(0.0, band_y0 as f32, fb_w as f32, (band_y1 - band_y0) as f32);
        let has_any_rounded: bool = !rounded_clips.is_empty();
        let mut scratch: BandCornerScratch = if has_any_rounded {
            BAND_CORNER_SCRATCH
                .try_with(|cell: &RefCell<BandCornerScratch>| {
                    cell.try_borrow_mut()
                        .ok()
                        .map(|mut guard: RefMut<'_, BandCornerScratch>| take(&mut *guard))
                })
                .ok()
                .flatten()
                .unwrap_or_default()
        } else {
            BandCornerScratch::default()
        };
        let mut shaded: usize = 0;
        for item in prepped.iter() {
            if item.get_y1() <= band_y0 || item.get_y0() >= band_y1 {
                continue;
            }
            let Some(active_clip) = item.get_active_clip().intersect(band_rect) else {
                continue;
            };
            let rc_len: usize = item.get_rounded_clip_len() as usize;
            let item_masks: &[RoundedClipMask] = if rc_len > 0 {
                let start: usize = item.get_rounded_clip_start() as usize;
                let end: usize = start.saturating_add(rc_len).min(rounded_clips.len());
                &rounded_clips[start..end]
            } else {
                &[]
            };
            let mut corner_mode: u8 = 0;
            if item_masks.len() == 1 {
                let mask: RoundedClipMask = item_masks[0];
                let rect: Rect = mask.get_rect();
                let max_r: f32 = (rect.get_width() * 0.5)
                    .min(rect.get_height() * 0.5)
                    .max(0.0);
                let radius: CornerRadius = mask.get_radius();
                let radii: [f32; 4] = [
                    radius.get_top_left().min(max_r),
                    radius.get_top_right().min(max_r),
                    radius.get_bottom_right().min(max_r),
                    radius.get_bottom_left().min(max_r),
                ];
                let ids: [u32; 4] = mask.get_corner_mask_ids();
                let all_cached: bool = radii
                    .iter()
                    .zip(ids.iter())
                    .all(|(&r, &id): (&f32, &u32)| r <= 0.25 || (id as usize) < corner_masks.len());
                if all_cached {
                    if Self::save_single_cached_rounded_corners(
                        band,
                        mask,
                        active_clip,
                        (fb_w, band_y0, band_y1),
                        corner_masks,
                        &mut scratch,
                    ) {
                        corner_mode = 1;
                    }
                } else {
                    scratch.corner_boxes.clear();
                    scratch.saved_bytes.clear();
                    collect_rounded_clip_boxes(
                        mask,
                        active_clip,
                        (fb_w, band_y0, band_y1),
                        &mut scratch.corner_boxes,
                    );
                    if !scratch.corner_boxes.is_empty() {
                        for &(bx0, by0, bx1, by1) in scratch.corner_boxes.iter() {
                            let b0: usize = (bx0 as usize) * 4;
                            let b1: usize = (bx1 as usize) * 4;
                            for y in by0..by1 {
                                if let Some(row) = band.row_slice_mut(y) {
                                    scratch.saved_bytes.extend_from_slice(&row[b0..b1]);
                                }
                            }
                        }
                        corner_mode = 2;
                    }
                }
            } else if !item_masks.is_empty() {
                scratch.corner_boxes.clear();
                scratch.saved_bytes.clear();
                for mask in item_masks.iter().copied() {
                    collect_rounded_clip_boxes(
                        mask,
                        active_clip,
                        (fb_w, band_y0, band_y1),
                        &mut scratch.corner_boxes,
                    );
                }
                if !scratch.corner_boxes.is_empty() {
                    for &(bx0, by0, bx1, by1) in scratch.corner_boxes.iter() {
                        let b0: usize = (bx0 as usize) * 4;
                        let b1: usize = (bx1 as usize) * 4;
                        for y in by0..by1 {
                            if let Some(row) = band.row_slice_mut(y) {
                                scratch.saved_bytes.extend_from_slice(&row[b0..b1]);
                            }
                        }
                    }
                    corner_mode = 2;
                }
            }
            let cmd: &DrawCommand = &commands[item.get_cmd_index()];
            shaded += match cmd {
                DrawCommand::Clear(color) => {
                    if active_clip.get_x() <= 0.0
                        && active_clip.get_width() >= (band.width() as f32)
                        && active_clip.get_y() <= (band_y0 as f32)
                        && active_clip.bottom() >= (band_y1 as f32)
                    {
                        band.clear(*color);
                        (band.width() as usize) * ((band_y1 - band_y0) as usize)
                    } else {
                        Self::fill_rect_clipped(band, active_clip, *color, active_clip)
                    }
                }
                DrawCommand::FillRoundedRect {
                    rect,
                    color,
                    radius,
                    border,
                } => {
                    let phys_rect: Rect = snap_physical_rect(*rect, scale);
                    let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                    let bw: f32 = if border.get_width() > 0.0 {
                        (border.get_width() * scale).max(1.0)
                    } else {
                        0.0
                    };
                    let phys_border: Border = Border::new(bw, border.get_color());
                    Self::raster_rounded_rect(
                        band,
                        phys_rect,
                        *color,
                        phys_radius,
                        phys_border,
                        active_clip,
                        aa,
                    )
                }
                DrawCommand::LinearGradientRect {
                    rect,
                    gradient,
                    radius,
                } => {
                    let phys_rect: Rect = snap_physical_rect(*rect, scale);
                    let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                    Self::raster_gradient_rect(
                        band,
                        phys_rect,
                        *gradient,
                        phys_radius,
                        active_clip,
                        aa,
                    )
                }
                DrawCommand::BoxShadow {
                    rect,
                    radius,
                    shadow,
                } => {
                    let spread: f32 = shadow.get_spread_radius() * scale;
                    let blur: f32 = (shadow.get_blur_radius().max(0.5) * scale).min(96.0);
                    let sigma: f32 = (blur * 0.5).max(0.75);
                    let pad: f32 = (sigma * 3.0).ceil();
                    let base_rect: Rect = snap_physical_rect(*rect, scale);
                    let caster_rect: Rect = Rect::new(
                        base_rect.get_x() + (shadow.get_offset_x() * scale).round() - spread,
                        base_rect.get_y() + (shadow.get_offset_y() * scale).round() - spread,
                        (base_rect.get_width() + spread * 2.0).max(1.0),
                        (base_rect.get_height() + spread * 2.0).max(1.0),
                    );
                    let shadow_rect: Rect = Rect::new(
                        caster_rect.get_x() - pad,
                        caster_rect.get_y() - pad,
                        caster_rect.get_width() + pad * 2.0,
                        caster_rect.get_height() + pad * 2.0,
                    );
                    let raw_r: CornerRadius = Self::scale_radius(*radius, scale);
                    let phys_radius: CornerRadius = CornerRadius::new(
                        (raw_r.get_top_left() + spread).max(0.0),
                        (raw_r.get_top_right() + spread).max(0.0),
                        (raw_r.get_bottom_right() + spread).max(0.0),
                        (raw_r.get_bottom_left() + spread).max(0.0),
                    );
                    Self::raster_shadow(
                        band,
                        (shadow_rect, caster_rect, active_clip),
                        phys_radius,
                        sigma,
                        shadow.get_color(),
                        (item.get_opaque_cutout_h(), item.get_opaque_cutout_v()),
                    )
                }
                DrawCommand::TextRun { style, .. } => {
                    let g_start: usize = item.get_glyph_run_start() as usize;
                    let g_len: usize = item.get_glyph_run_len() as usize;
                    if let Some(run_slice) = positioned_glyphs.get(g_start..g_start + g_len) {
                        Self::raster_text_run_band(
                            band,
                            run_slice,
                            style.get_color(),
                            active_clip,
                            glyph_atlas,
                        )
                    } else {
                        0
                    }
                }
                DrawCommand::TextCaret { color, .. } => {
                    let cidx: usize = item.get_caret_rect_idx() as usize;
                    if let Some(&caret_rect) = caret_rects.get(cidx) {
                        Self::raster_rounded_rect(
                            band,
                            caret_rect,
                            *color,
                            CornerRadius::all(0.5 * scale),
                            Border::default(),
                            active_clip,
                            aa,
                        )
                    } else {
                        0
                    }
                }
                DrawCommand::Image {
                    rect,
                    buffer,
                    radius,
                } => {
                    let phys_rect: Rect = snap_physical_rect(*rect, scale);
                    let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                    Self::raster_image(band, phys_rect, buffer, phys_radius, active_clip, aa)
                }
                DrawCommand::Line {
                    from,
                    to,
                    color,
                    width,
                } => {
                    let p0: Point = Point::new(from.get_x() * scale, from.get_y() * scale);
                    let p1: Point = Point::new(to.get_x() * scale, to.get_y() * scale);
                    Self::raster_line(
                        band,
                        p0,
                        p1,
                        *color,
                        (width * scale).max(1.0),
                        active_clip,
                        aa,
                    )
                }
                DrawCommand::Circle {
                    center,
                    radius,
                    color,
                } => {
                    let c: Point = Point::new(center.get_x() * scale, center.get_y() * scale);
                    Self::raster_circle(band, c, radius * scale, *color, active_clip, aa)
                }
                DrawCommand::Triangle { p0, p1, p2, color } => {
                    let v0: Point = Point::new(p0.get_x() * scale, p0.get_y() * scale);
                    let v1: Point = Point::new(p1.get_x() * scale, p1.get_y() * scale);
                    let v2: Point = Point::new(p2.get_x() * scale, p2.get_y() * scale);
                    Self::raster_triangle(band, v0, v1, v2, *color, active_clip, aa)
                }
                DrawCommand::SpriteRegion {
                    rect,
                    buffer,
                    src_rect,
                    tint,
                } => {
                    let phys_rect: Rect = snap_physical_rect(*rect, scale);
                    Self::raster_sprite_region(
                        band,
                        phys_rect,
                        buffer,
                        *src_rect,
                        *tint,
                        active_clip,
                        aa,
                    )
                }
                DrawCommand::PushClip(_)
                | DrawCommand::PushRoundedClip { .. }
                | DrawCommand::PopClip => 0,
            };
            if corner_mode == 1 {
                Self::restore_single_cached_rounded_corners(band, corner_masks, &scratch);
            } else if corner_mode == 2 {
                let multi_mask: bool = item_masks.len() > 1;
                let saved_bytes: &[u8] = scratch.saved_bytes.as_slice();
                let mut byte_cursor: usize = 0;
                for (box_idx, &(bx0, by0, bx1, by1)) in scratch.corner_boxes.iter().enumerate() {
                    for y in by0..by1 {
                        let Some(row) = band.row_slice_mut(y) else {
                            byte_cursor += ((bx1 - bx0) as usize) * 4;
                            continue;
                        };
                        for x in bx0..bx1 {
                            let saved_rgba: [u8; 4] = [
                                saved_bytes[byte_cursor],
                                saved_bytes[byte_cursor + 1],
                                saved_bytes[byte_cursor + 2],
                                saved_bytes[byte_cursor + 3],
                            ];
                            byte_cursor += 4;
                            if multi_mask
                                && scratch.corner_boxes[..box_idx].iter().any(
                                    |&(px0, py0, px1, py1): &(u32, u32, u32, u32)| {
                                        x >= px0 && x < px1 && y >= py0 && y < py1
                                    },
                                )
                            {
                                continue;
                            }
                            let mut cov: f32 = 1.0;
                            for mask in item_masks.iter().copied() {
                                cov *= rounded_clip_coverage_cached(x, y, mask, corner_masks, aa);
                                if cov <= 0.0005 {
                                    break;
                                }
                            }
                            let dst_off: usize = (x as usize) * 4;
                            if cov <= 0.0005 {
                                row[dst_off..dst_off + 4].copy_from_slice(&saved_rgba);
                            } else if cov < 0.9995 {
                                lerp_pixel_slice_from_saved_linear(
                                    &mut row[dst_off..dst_off + 4],
                                    saved_rgba,
                                    cov,
                                );
                            }
                        }
                    }
                }
            }
        }
        if has_any_rounded {
            let _: bool = BAND_CORNER_SCRATCH
                .try_with(|cell: &RefCell<BandCornerScratch>| {
                    if let Ok(mut guard) = cell.try_borrow_mut() {
                        *guard = scratch;
                    }
                })
                .is_ok();
        }
        shaded
    }

    /// Scales a logical rectangle into physical pixels.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Logical rectangle.
    /// - `f32` - Scale factor.
    ///
    /// # Returns
    ///
    /// - `Rect` - Physical pixel rectangle.
    fn scale_rect(rect: Rect, scale: f32) -> Rect {
        Rect::new(
            rect.get_x() * scale,
            rect.get_y() * scale,
            rect.get_width() * scale,
            rect.get_height() * scale,
        )
    }

    /// Scales logical corner radii into physical pixel radii.
    ///
    /// # Arguments
    ///
    /// - `CornerRadius` - Logical corner radii.
    /// - `f32` - Scale factor.
    ///
    /// # Returns
    ///
    /// - `CornerRadius` - Physical corner radii.
    fn scale_radius(radius: CornerRadius, scale: f32) -> CornerRadius {
        CornerRadius::new(
            radius.get_top_left() * scale,
            radius.get_top_right() * scale,
            radius.get_bottom_right() * scale,
            radius.get_bottom_left() * scale,
        )
    }

    /// Fast axis-aligned solid rectangle fill clipped against `clip`.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Physical rectangle.
    /// - `Color` - Fill color.
    /// - `Rect` - Active clip rectangle.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of pixels shaded.
    fn fill_rect_clipped(band: &mut PixelBand<'_>, rect: Rect, color: Color, clip: Rect) -> usize {
        if color.get_a() == 0 {
            return 0;
        }
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        let span_len: usize = (x1 - x0) as usize;
        let rows: usize = (y1 - y0) as usize;
        let sa: u8 = color.get_a();
        if sa < 255 && span_len.saturating_mul(rows) >= 512 {
            let luts: ([u8; 256], [u8; 256], [u8; 256], [u8; 256]) = build_linear_blend_luts(color);
            for y in y0..y1 {
                band.fill_span_linear_luts(x0, x1, y, &luts);
            }
        } else {
            for y in y0..y1 {
                band.fill_span_linear(x0, x1, y, color);
            }
        }
        span_len * rows
    }

    /// Precomputes 1D horizontal edge samples for vertical side strip `x_start..x_end` into stack buffer `out`.
    ///
    /// # Arguments
    ///
    /// - `(u32, u32)` - Inclusive/exclusive horizontal span `(x_start, x_end)`.
    /// - `(f32, f32, f32, f32)` - Box geometry `(center_x, half_w, half_h, border_w)`.
    /// - `(Color, Border)` - Fill color and border specification.
    /// - `([f32; 4], [f32; 4])` - Precomputed linear fill and border tuples `(lin_r, lin_g, lin_b, alpha_norm)`.
    /// - `AntialiasingMode` - Active anti-aliasing policy.
    /// - `&mut [EdgeBlendSample; ROUNDED_EDGE_STRIP_CAP]` - Stack-allocated sample destination.
    ///
    /// # Returns
    ///
    /// - `(usize, usize)` - Tuple `(sample_count, shaded_count)` written to `out`.
    #[must_use]
    fn build_vertical_edge_strip(
        x_span: (u32, u32),
        geom: (f32, f32, f32, f32),
        colors: (Color, Border),
        lin_colors: ([f32; 4], [f32; 4]),
        aa: AntialiasingMode,
        out: &mut [EdgeBlendSample; ROUNDED_EDGE_STRIP_CAP],
    ) -> (usize, usize) {
        let (x_start, x_end): (u32, u32) = x_span;
        let (cx, half_w, half_h, bw): (f32, f32, f32, f32) = geom;
        let (color, border): (Color, Border) = colors;
        let (fill_lin, border_lin): ([f32; 4], [f32; 4]) = lin_colors;
        let has_border: bool = bw > 0.0 && border.get_color().get_a() > 0;
        let inner_hw: f32 = (half_w - bw).max(0.0);
        let inner_hh: f32 = (half_h - bw).max(0.0);
        let mut len: usize = 0;
        let mut shaded: usize = 0;
        for x in x_start..x_end {
            let px: f32 = (x as f32) + 0.5 - cx;
            let dist: f32 = px.abs() - half_w;
            if dist > 0.85 {
                continue;
            }
            if !has_border {
                if dist < -0.85 {
                    let sa: u8 = color.get_a();
                    if sa > 0 {
                        let a: f32 = (sa as f32) * (1.0 / 255.0);
                        out[len] = EdgeBlendSample {
                            x,
                            rgba: [color.get_r(), color.get_g(), color.get_b(), 255],
                            sr_a: fill_lin[0] * a,
                            sg_a: fill_lin[1] * a,
                            sb_a: fill_lin[2] * a,
                            inv_a: 1.0 - a,
                            sa_u32: sa as u32,
                            is_opaque: sa == 255,
                        };
                        len += 1;
                        shaded += 1;
                    }
                    continue;
                }
                let outer_cov: f32 = sdf_coverage(dist, aa);
                if outer_cov > 0.0 && color.get_a() > 0 {
                    let alpha_f: f32 = ((color.get_a() as f32) * (1.0 / 255.0)) * outer_cov;
                    shaded += 1;
                    if alpha_f > 0.0002 {
                        let a: f32 = alpha_f.clamp(0.0, 1.0);
                        out[len] = EdgeBlendSample {
                            x,
                            rgba: [color.get_r(), color.get_g(), color.get_b(), 255],
                            sr_a: fill_lin[0] * a,
                            sg_a: fill_lin[1] * a,
                            sb_a: fill_lin[2] * a,
                            inv_a: 1.0 - a,
                            sa_u32: (a * 255.0) as u32,
                            is_opaque: a >= 0.9995,
                        };
                        len += 1;
                    }
                }
                continue;
            }
            if dist < -(bw + 0.85) {
                let sa: u8 = color.get_a();
                if sa > 0 {
                    let a: f32 = (sa as f32) * (1.0 / 255.0);
                    out[len] = EdgeBlendSample {
                        x,
                        rgba: [color.get_r(), color.get_g(), color.get_b(), 255],
                        sr_a: fill_lin[0] * a,
                        sg_a: fill_lin[1] * a,
                        sb_a: fill_lin[2] * a,
                        inv_a: 1.0 - a,
                        sa_u32: sa as u32,
                        is_opaque: sa == 255,
                    };
                    len += 1;
                    shaded += 1;
                }
                continue;
            }
            let inner_dist: f32 = if inner_hw > 0.0 && inner_hh > 0.0 {
                px.abs() - inner_hw
            } else {
                2.0
            };
            if dist < -0.85 && inner_dist > 0.85 {
                let bc: Color = border.get_color();
                let sa: u8 = bc.get_a();
                let a: f32 = (sa as f32) * (1.0 / 255.0);
                out[len] = EdgeBlendSample {
                    x,
                    rgba: [bc.get_r(), bc.get_g(), bc.get_b(), 255],
                    sr_a: border_lin[0] * a,
                    sg_a: border_lin[1] * a,
                    sb_a: border_lin[2] * a,
                    inv_a: 1.0 - a,
                    sa_u32: sa as u32,
                    is_opaque: sa == 255,
                };
                len += 1;
                shaded += 1;
                continue;
            }
            let outer_cov: f32 = sdf_coverage(dist, aa);
            if outer_cov <= 0.0 {
                continue;
            }
            let inner_cov: f32 = if inner_hw > 0.0 && inner_hh > 0.0 {
                sdf_coverage(inner_dist, aa)
            } else {
                0.0
            };
            let mixed: Color =
                composite_fill_and_border_precomputed(fill_lin, border_lin, outer_cov, inner_cov);
            let sa: u8 = mixed.get_a();
            if sa > 0 {
                let a: f32 = (sa as f32) * (1.0 / 255.0);
                out[len] = EdgeBlendSample {
                    x,
                    rgba: [mixed.get_r(), mixed.get_g(), mixed.get_b(), 255],
                    sr_a: SRGB_TO_LIN_LUT[mixed.get_r() as usize] * a,
                    sg_a: SRGB_TO_LIN_LUT[mixed.get_g() as usize] * a,
                    sb_a: SRGB_TO_LIN_LUT[mixed.get_b() as usize] * a,
                    inv_a: 1.0 - a,
                    sa_u32: sa as u32,
                    is_opaque: sa == 255,
                };
                len += 1;
                shaded += 1;
            }
        }
        (len, shaded)
    }

    /// Rasterizes a rounded rectangle with optional border using edge-bounded 4x4 RGSS supersampling, precomputed 1D vertical edge strips, and linear-light spans.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Physical rectangle bounds.
    /// - `Color` - Fill color.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Border` - Physical border specification.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of pixels shaded.
    fn raster_rounded_rect(
        band: &mut PixelBand<'_>,
        rect: Rect,
        color: Color,
        radius: CornerRadius,
        border: Border,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let has_border: bool = border.get_width() > 0.0 && border.get_color().get_a() > 0;
        if radius.max_radius() <= 0.0 && !has_border {
            return Self::fill_rect_clipped(band, rect, color, clip);
        }
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let border_w: f32 = if has_border { border.get_width() } else { 0.0 };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        let effective_r: f32 = radius.max_radius().min(half_w).min(half_h);
        let safe_inset: f32 = effective_r.max(border_w) + 1.5;
        let edge_inset: f32 = border_w + 1.5;
        let iy0: u32 = ((rect.get_y() + safe_inset).ceil().max(0.0) as u32).clamp(y0, y1);
        let iy1: u32 = ((rect.bottom() - safe_inset).floor().max(0.0) as u32).clamp(y0, y1);
        let ix0: u32 = ((rect.get_x() + edge_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let ix1: u32 = ((rect.right() - edge_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let cx0: u32 = ((rect.get_x() + safe_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let cx1: u32 = ((rect.right() - safe_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_interior: bool = iy0 < iy1 && ix0 < ix1;
        let has_center_band: bool = cx0 < cx1;
        let fill_lin: [f32; 4] = [
            SRGB_TO_LIN_LUT[color.get_r() as usize],
            SRGB_TO_LIN_LUT[color.get_g() as usize],
            SRGB_TO_LIN_LUT[color.get_b() as usize],
            (color.get_a() as f32) / 255.0,
        ];
        let bc: Color = border.get_color();
        let border_lin: [f32; 4] = [
            SRGB_TO_LIN_LUT[bc.get_r() as usize],
            SRGB_TO_LIN_LUT[bc.get_g() as usize],
            SRGB_TO_LIN_LUT[bc.get_b() as usize],
            (bc.get_a() as f32) / 255.0,
        ];
        let interior_luts: Option<RgbaLutQuad> = if has_interior
            && color.get_a() > 0
            && color.get_a() < 255
            && ((ix1 - ix0) as usize).saturating_mul((iy1 - iy0) as usize) >= 512
        {
            Some(build_linear_blend_luts(color))
        } else {
            None
        };
        let mut shaded: usize = 0;
        if has_interior {
            for y in y0..iy0 {
                let py: f32 = (y as f32) + 0.5 - cy;
                let box_geom: (f32, f32, f32, f32) = (cx, py, half_w, half_h);
                if has_center_band {
                    if let Some(row) = band.row_slice_mut(y) {
                        for x in x0..cx0 {
                            shaded += Self::shade_rounded_pixel_row(
                                row,
                                x,
                                box_geom,
                                radius,
                                (color, border, fill_lin, border_lin),
                                aa,
                            );
                        }
                        for x in cx1..x1 {
                            shaded += Self::shade_rounded_pixel_row(
                                row,
                                x,
                                box_geom,
                                radius,
                                (color, border, fill_lin, border_lin),
                                aa,
                            );
                        }
                    }
                    let dist_y: f32 = py.abs() - half_h;
                    let outer_cov: f32 = sdf_coverage(dist_y, aa);
                    if outer_cov > 0.0 {
                        if has_border {
                            let inner_hh: f32 = (half_h - border_w).max(0.0);
                            let inner_cov: f32 = if inner_hh > 0.0 && half_w > border_w {
                                sdf_coverage(py.abs() - inner_hh, aa)
                            } else {
                                0.0
                            };
                            let mixed: Color = composite_fill_and_border_precomputed(
                                fill_lin, border_lin, outer_cov, inner_cov,
                            );
                            if mixed.get_a() > 0 {
                                band.fill_span_linear(cx0, cx1, y, mixed);
                            }
                            shaded += (cx1 - cx0) as usize;
                        } else if color.get_a() > 0 {
                            band.fill_span_linear(cx0, cx1, y, color.multiply_alpha(outer_cov));
                            shaded += (cx1 - cx0) as usize;
                        }
                    }
                } else if let Some(row) = band.row_slice_mut(y) {
                    for x in x0..x1 {
                        shaded += Self::shade_rounded_pixel_row(
                            row,
                            x,
                            box_geom,
                            radius,
                            (color, border, fill_lin, border_lin),
                            aa,
                        );
                    }
                }
            }
            let use_1d_strip: bool = ((ix0 - x0) as usize) <= ROUNDED_EDGE_STRIP_CAP
                && ((x1 - ix1) as usize) <= ROUNDED_EDGE_STRIP_CAP;
            if use_1d_strip {
                let mut left_strip: [EdgeBlendSample; ROUNDED_EDGE_STRIP_CAP] =
                    [EdgeBlendSample::default(); ROUNDED_EDGE_STRIP_CAP];
                let mut right_strip: [EdgeBlendSample; ROUNDED_EDGE_STRIP_CAP] =
                    [EdgeBlendSample::default(); ROUNDED_EDGE_STRIP_CAP];
                let (left_len, left_shaded): (usize, usize) = if x0 < ix0 {
                    Self::build_vertical_edge_strip(
                        (x0, ix0),
                        (cx, half_w, half_h, border_w),
                        (color, border),
                        (fill_lin, border_lin),
                        aa,
                        &mut left_strip,
                    )
                } else {
                    (0, 0)
                };
                let (right_len, right_shaded): (usize, usize) = if ix1 < x1 {
                    Self::build_vertical_edge_strip(
                        (ix1, x1),
                        (cx, half_w, half_h, border_w),
                        (color, border),
                        (fill_lin, border_lin),
                        aa,
                        &mut right_strip,
                    )
                } else {
                    (0, 0)
                };
                let left_slice: &[EdgeBlendSample] = &left_strip[..left_len];
                let right_slice: &[EdgeBlendSample] = &right_strip[..right_len];
                let edge_shaded: usize = left_shaded + right_shaded;
                let has_edges: bool = left_len > 0 || right_len > 0;
                let fill_visible: bool = color.get_a() > 0;
                let interior_span: usize = (ix1 - ix0) as usize;
                for y in iy0..iy1 {
                    if fill_visible {
                        if let Some(ref luts) = interior_luts {
                            band.fill_span_linear_luts(ix0, ix1, y, luts);
                        } else {
                            band.fill_span_linear(ix0, ix1, y, color);
                        }
                        shaded += interior_span;
                    }
                    if has_edges {
                        if let Some(row) = band.row_slice_mut(y) {
                            if left_len > 0 {
                                apply_edge_strip_row(row, left_slice);
                            }
                            if right_len > 0 {
                                apply_edge_strip_row(row, right_slice);
                            }
                        }
                        shaded += edge_shaded;
                    }
                }
            } else {
                let fill_visible: bool = color.get_a() > 0;
                let interior_span: usize = (ix1 - ix0) as usize;
                for y in iy0..iy1 {
                    let py: f32 = (y as f32) + 0.5 - cy;
                    let box_geom: (f32, f32, f32, f32) = (cx, py, half_w, half_h);
                    if fill_visible {
                        if let Some(ref luts) = interior_luts {
                            band.fill_span_linear_luts(ix0, ix1, y, luts);
                        } else {
                            band.fill_span_linear(ix0, ix1, y, color);
                        }
                        shaded += interior_span;
                    }
                    if let Some(row) = band.row_slice_mut(y) {
                        for x in x0..ix0 {
                            shaded += Self::shade_rounded_pixel_row(
                                row,
                                x,
                                box_geom,
                                radius,
                                (color, border, fill_lin, border_lin),
                                aa,
                            );
                        }
                        for x in ix1..x1 {
                            shaded += Self::shade_rounded_pixel_row(
                                row,
                                x,
                                box_geom,
                                radius,
                                (color, border, fill_lin, border_lin),
                                aa,
                            );
                        }
                    }
                }
            }
            for y in iy1..y1 {
                let py: f32 = (y as f32) + 0.5 - cy;
                let box_geom: (f32, f32, f32, f32) = (cx, py, half_w, half_h);
                if has_center_band {
                    if let Some(row) = band.row_slice_mut(y) {
                        for x in x0..cx0 {
                            shaded += Self::shade_rounded_pixel_row(
                                row,
                                x,
                                box_geom,
                                radius,
                                (color, border, fill_lin, border_lin),
                                aa,
                            );
                        }
                        for x in cx1..x1 {
                            shaded += Self::shade_rounded_pixel_row(
                                row,
                                x,
                                box_geom,
                                radius,
                                (color, border, fill_lin, border_lin),
                                aa,
                            );
                        }
                    }
                    let dist_y: f32 = py.abs() - half_h;
                    let outer_cov: f32 = sdf_coverage(dist_y, aa);
                    if outer_cov > 0.0 {
                        if has_border {
                            let inner_hh: f32 = (half_h - border_w).max(0.0);
                            let inner_cov: f32 = if inner_hh > 0.0 && half_w > border_w {
                                sdf_coverage(py.abs() - inner_hh, aa)
                            } else {
                                0.0
                            };
                            let mixed: Color = composite_fill_and_border_precomputed(
                                fill_lin, border_lin, outer_cov, inner_cov,
                            );
                            if mixed.get_a() > 0 {
                                band.fill_span_linear(cx0, cx1, y, mixed);
                            }
                            shaded += (cx1 - cx0) as usize;
                        } else if color.get_a() > 0 {
                            band.fill_span_linear(cx0, cx1, y, color.multiply_alpha(outer_cov));
                            shaded += (cx1 - cx0) as usize;
                        }
                    }
                } else if let Some(row) = band.row_slice_mut(y) {
                    for x in x0..x1 {
                        shaded += Self::shade_rounded_pixel_row(
                            row,
                            x,
                            box_geom,
                            radius,
                            (color, border, fill_lin, border_lin),
                            aa,
                        );
                    }
                }
            }
            return shaded;
        }
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            let box_geom: (f32, f32, f32, f32) = (cx, py, half_w, half_h);
            if has_center_band {
                if let Some(row) = band.row_slice_mut(y) {
                    for x in x0..cx0 {
                        shaded += Self::shade_rounded_pixel_row(
                            row,
                            x,
                            box_geom,
                            radius,
                            (color, border, fill_lin, border_lin),
                            aa,
                        );
                    }
                    for x in cx1..x1 {
                        shaded += Self::shade_rounded_pixel_row(
                            row,
                            x,
                            box_geom,
                            radius,
                            (color, border, fill_lin, border_lin),
                            aa,
                        );
                    }
                }
                let dist_y: f32 = py.abs() - half_h;
                let outer_cov: f32 = sdf_coverage(dist_y, aa);
                if outer_cov > 0.0 {
                    if has_border {
                        let inner_hh: f32 = (half_h - border_w).max(0.0);
                        let inner_cov: f32 = if inner_hh > 0.0 && half_w > border_w {
                            sdf_coverage(py.abs() - inner_hh, aa)
                        } else {
                            0.0
                        };
                        let mixed: Color = composite_fill_and_border_precomputed(
                            fill_lin, border_lin, outer_cov, inner_cov,
                        );
                        if mixed.get_a() > 0 {
                            band.fill_span_linear(cx0, cx1, y, mixed);
                        }
                        shaded += (cx1 - cx0) as usize;
                    } else if color.get_a() > 0 {
                        band.fill_span_linear(cx0, cx1, y, color.multiply_alpha(outer_cov));
                        shaded += (cx1 - cx0) as usize;
                    }
                }
            } else if let Some(row) = band.row_slice_mut(y) {
                for x in x0..x1 {
                    shaded += Self::shade_rounded_pixel_row(
                        row,
                        x,
                        box_geom,
                        radius,
                        (color, border, fill_lin, border_lin),
                        aa,
                    );
                }
            }
        }
        shaded
    }

    /// Evaluates fast early-rejected analytical SDF and edge-only 4x4 RGSS coverage directly onto a mutable scanline slice.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - Target scanline RGBA8 slice.
    /// - `u32` - Global pixel X coordinate.
    /// - `(f32, f32, f32, f32)` - Box geometry `(center_x, rel_py, half_w, half_h)`.
    /// - `CornerRadius` - Physical corner radii.
    /// - `(Color, Border, [f32; 4], [f32; 4])` - Fill color, border spec, and precomputed linear fill/border tuples.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - `1` if the pixel was shaded, `0` otherwise.
    #[inline(always)]
    fn shade_rounded_pixel_row(
        row: &mut [u8],
        x: u32,
        geom: (f32, f32, f32, f32),
        radius: CornerRadius,
        style_colors: (Color, Border, [f32; 4], [f32; 4]),
        aa: AntialiasingMode,
    ) -> usize {
        let (cx, py, half_w, half_h): (f32, f32, f32, f32) = geom;
        let (color, border, fill_lin, border_lin): (Color, Border, [f32; 4], [f32; 4]) =
            style_colors;
        let fill_rgb: [f32; 3] = [fill_lin[0], fill_lin[1], fill_lin[2]];
        let px: f32 = (x as f32) + 0.5 - cx;
        let corner_r: f32 = quadrant_corner_radius(radius, px, py)
            .min(half_w)
            .min(half_h);
        let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, corner_r);
        if dist > 0.85 {
            return 0;
        }
        let has_border: bool = border.get_width() > 0.0 && border.get_color().get_a() > 0;
        let bw: f32 = if has_border { border.get_width() } else { 0.0 };
        if !has_border {
            if dist < -0.85 {
                if color.get_a() > 0 {
                    blend_row_pixel_linear_precomputed(row, x, color, fill_rgb);
                    return 1;
                }
                return 0;
            }
            let in_corner_edge: bool = aa != AntialiasingMode::None
                && corner_r > 0.5
                && px.abs() > (half_w - corner_r - 1.5)
                && py.abs() > (half_h - corner_r - 1.5);
            let outer_cov: f32 = if in_corner_edge {
                rounded_box_supersampled_coverage(px, py, half_w, half_h, corner_r, 0.0).0
            } else {
                sdf_coverage(dist, aa)
            };
            if outer_cov > 0.0 && color.get_a() > 0 {
                let alpha_f: f32 = ((color.get_a() as f32) * (1.0 / 255.0)) * outer_cov;
                blend_row_pixel_f32_precomputed(row, x, color, fill_rgb, alpha_f);
                return 1;
            }
            return 0;
        }
        if dist < -(bw + 0.85) {
            if color.get_a() > 0 {
                blend_row_pixel_linear_precomputed(row, x, color, fill_rgb);
                return 1;
            }
            return 0;
        }
        let inner_hw: f32 = (half_w - bw).max(0.0);
        let inner_hh: f32 = (half_h - bw).max(0.0);
        let inner_r: f32 = (corner_r - bw).max(0.0);
        let inner_dist: f32 = if inner_hw > 0.0 && inner_hh > 0.0 {
            rounded_box_sdf(px, py, inner_hw, inner_hh, inner_r)
        } else {
            2.0
        };
        if dist < -0.85 && inner_dist > 0.85 {
            let border_rgb: [f32; 3] = [border_lin[0], border_lin[1], border_lin[2]];
            blend_row_pixel_linear_precomputed(row, x, border.get_color(), border_rgb);
            return 1;
        }
        let in_corner_zone: bool = aa != AntialiasingMode::None
            && corner_r > 0.5
            && px.abs() > (half_w - corner_r - 1.5)
            && py.abs() > (half_h - corner_r - 1.5);
        let (outer_cov, inner_cov): (f32, f32) = if in_corner_zone {
            rounded_box_supersampled_coverage(px, py, half_w, half_h, corner_r, bw)
        } else {
            let oc: f32 = sdf_coverage(dist, aa);
            if oc <= 0.0 {
                return 0;
            }
            let ic: f32 = if inner_hw > 0.0 && inner_hh > 0.0 {
                sdf_coverage(inner_dist, aa)
            } else {
                0.0
            };
            (oc, ic)
        };
        if outer_cov <= 0.0 {
            return 0;
        }
        let mixed: Color =
            composite_fill_and_border_precomputed(fill_lin, border_lin, outer_cov, inner_cov);
        if mixed.get_a() > 0 {
            let mixed_rgb: [f32; 3] = [
                SRGB_TO_LIN_LUT[mixed.get_r() as usize],
                SRGB_TO_LIN_LUT[mixed.get_g() as usize],
                SRGB_TO_LIN_LUT[mixed.get_b() as usize],
            ];
            blend_row_pixel_linear_precomputed(row, x, mixed, mixed_rgb);
            return 1;
        }
        0
    }

    /// Fills a constant-coverage horizontal span `[sx0, sx1)` on row `y` for a pure vertical gradient.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `(u32, u32, u32)` - Tuple `(sx0, sx1, y)` defining the horizontal span.
    /// - `Color` - Row-constant interpolated gradient color.
    /// - `f32` - Row-constant coverage in `(0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of pixels shaded across `[sx0, sx1)`.
    #[inline(always)]
    fn fill_vertical_gradient_span(
        band: &mut PixelBand<'_>,
        coords: (u32, u32, u32),
        sample: Color,
        cov: f32,
    ) -> usize {
        let (sx0, sx1, y): (u32, u32, u32) = coords;
        if sx0 >= sx1 || cov <= 0.0 {
            return 0;
        }
        let alpha_f: f32 = ((sample.get_a() as f32) * (1.0 / 255.0)) * cov;
        if alpha_f >= 0.9995 {
            band.fill_span(
                sx0,
                sx1,
                y,
                Color::new(sample.get_r(), sample.get_g(), sample.get_b(), 255),
            );
        } else if alpha_f > 0.0005 {
            band.blend_shadow_span_const_alpha(sx0, sx1, y, sample, alpha_f);
        } else if alpha_f > 0.0002
            && let Some(row) = band.row_slice_mut(y)
        {
            let y_term: f32 = (y as f32) * 0.005_837_15;
            for x in sx0..sx1 {
                blend_gradient_row_pixel(row, x, y_term, sample, cov);
            }
        }
        (sx1 - sx0) as usize
    }

    /// Fills a full-coverage or constant-coverage horizontal span `[sx0, sx1)` for a horizontal or diagonal gradient.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - Mutable target framebuffer scanline slice.
    /// - `(u32, u32, f32)` - Tuple `(sx0, sx1, y_term)` for span bounds and IGN phase.
    /// - `(f32, f32, f32)` - Tuple `(t_start, dt_dx, cov)` for incremental gradient stepping.
    /// - `(Color, Color)` - Gradient endpoints `(start_color, end_color)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of pixels shaded across `[sx0, sx1)`.
    #[inline(always)]
    fn fill_angled_gradient_span(
        row: &mut [u8],
        coords: (u32, u32, f32),
        step: (f32, f32, f32),
        stops: (Color, Color),
    ) -> usize {
        let (sx0, sx1, y_term): (u32, u32, f32) = coords;
        let (t_start, dt_dx, cov): (f32, f32, f32) = step;
        let (start_c, end_c): (Color, Color) = stops;
        if sx0 >= sx1 || cov <= 0.0 {
            return 0;
        }
        let byte_start: usize = (sx0 as usize) * 4;
        let byte_end: usize = (sx1 as usize) * 4;
        if byte_end > row.len() {
            return 0;
        }
        let sr_f: f32 = start_c.get_r() as f32;
        let sg_f: f32 = start_c.get_g() as f32;
        let sb_f: f32 = start_c.get_b() as f32;
        let er_f: f32 = end_c.get_r() as f32;
        let eg_f: f32 = end_c.get_g() as f32;
        let eb_f: f32 = end_c.get_b() as f32;
        let both_opaque: bool = start_c.get_a() == 255 && end_c.get_a() == 255;
        let mut t_cur: f32 = t_start;
        if both_opaque && cov >= 0.9995 {
            let chunks: &mut [[u8; 4]] = row[byte_start..byte_end].as_chunks_mut::<4>().0;
            for px in chunks.iter_mut() {
                let f: f32 = t_cur.clamp(0.0, 1.0);
                let inv: f32 = 1.0 - f;
                px[0] = (sr_f * inv + er_f * f + 0.5) as u8;
                px[1] = (sg_f * inv + eg_f * f + 0.5) as u8;
                px[2] = (sb_f * inv + eb_f * f + 0.5) as u8;
                px[3] = 255;
                t_cur += dt_dx;
            }
            return (sx1 - sx0) as usize;
        }
        let sa_f: f32 = start_c.get_a() as f32;
        let ea_f: f32 = end_c.get_a() as f32;
        for x in sx0..sx1 {
            let f: f32 = t_cur.clamp(0.0, 1.0);
            let inv: f32 = 1.0 - f;
            let sample: Color = Color::new(
                (sr_f * inv + er_f * f + 0.5) as u8,
                (sg_f * inv + eg_f * f + 0.5) as u8,
                (sb_f * inv + eb_f * f + 0.5) as u8,
                (sa_f * inv + ea_f * f + 0.5) as u8,
            );
            blend_gradient_row_pixel(row, x, y_term, sample, cov);
            t_cur += dt_dx;
        }
        (sx1 - sx0) as usize
    }

    /// Rasterizes a two-stop linear gradient inside an SDF rounded rectangle with vertical/interior fast paths and corner AA.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Physical bounds.
    /// - `LinearGradient` - Gradient angle and stops.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_gradient_rect(
        band: &mut PixelBand<'_>,
        rect: Rect,
        gradient: LinearGradient,
        radius: CornerRadius,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        let rad: f32 = gradient.get_angle_deg().to_radians();
        let dir_x: f32 = rad.sin();
        let dir_y: f32 = rad.cos();
        let start_c: Color = gradient.get_start_color();
        let end_c: Color = gradient.get_end_color();
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let inv_w: f32 = if rect.get_width() > 0.0 {
            1.0 / rect.get_width()
        } else {
            0.0
        };
        let inv_h: f32 = if rect.get_height() > 0.0 {
            1.0 / rect.get_height()
        } else {
            0.0
        };
        let r_max: f32 = radius.max_radius().min(half_w).min(half_h).max(0.0);
        let ix0: u32 = ((rect.get_x() + 1.0).ceil().max(0.0) as u32).clamp(x0, x1);
        let ix1: u32 = ((rect.right() - 1.0).floor().max(0.0) as u32).clamp(ix0, x1);
        let iy0: u32 = ((rect.get_y() + r_max + 1.0).ceil().max(0.0) as u32).clamp(y0, y1);
        let iy1: u32 = ((rect.bottom() - r_max - 1.0).floor().max(0.0) as u32).clamp(iy0, y1);
        let cx0: u32 = ((rect.get_x() + r_max + 1.0).ceil().max(0.0) as u32).clamp(x0, x1);
        let cx1: u32 = ((rect.right() - r_max - 1.0).floor().max(0.0) as u32).clamp(cx0, x1);
        let is_vertical: bool = dir_x.abs() < 1e-5;
        let dt_dx: f32 = inv_w * dir_x;
        let rect_x: f32 = rect.get_x();
        let rect_y: f32 = rect.get_y();
        let mut shaded: usize = 0;
        if is_vertical {
            for y in y0..iy0 {
                let py: f32 = (y as f32) + 0.5 - cy;
                let ny: f32 = ((y as f32) - rect_y) * inv_h;
                let t: f32 = ((ny - 0.5) * dir_y + 0.5).clamp(0.0, 1.0);
                let sample: Color = start_c.lerp(end_c, t);
                let dist_y: f32 = py.abs() - half_h;
                let mid_cov: f32 = if dist_y > 0.85 {
                    0.0
                } else if dist_y < -0.85 {
                    1.0
                } else {
                    sdf_coverage(dist_y, aa)
                };
                if cx0 < cx1 && mid_cov > 0.0 {
                    shaded +=
                        Self::fill_vertical_gradient_span(band, (cx0, cx1, y), sample, mid_cov);
                }
                if (x0 < cx0 || cx1 < x1)
                    && let Some(row) = band.row_slice_mut(y)
                {
                    let y_term: f32 = (y as f32) * 0.005_837_15;
                    for x in x0..cx0 {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cov: f32 =
                            gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                        if cov > 0.0 {
                            blend_gradient_row_pixel(row, x, y_term, sample, cov);
                            shaded += 1;
                        }
                    }
                    for x in cx1..x1 {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cov: f32 =
                            gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                        if cov > 0.0 {
                            blend_gradient_row_pixel(row, x, y_term, sample, cov);
                            shaded += 1;
                        }
                    }
                }
            }
            for y in iy0..iy1 {
                let ny: f32 = ((y as f32) - rect_y) * inv_h;
                let t: f32 = ((ny - 0.5) * dir_y + 0.5).clamp(0.0, 1.0);
                let sample: Color = start_c.lerp(end_c, t);
                if ix0 < ix1 {
                    shaded += Self::fill_vertical_gradient_span(band, (ix0, ix1, y), sample, 1.0);
                }
                if (x0 < ix0 || ix1 < x1)
                    && let Some(row) = band.row_slice_mut(y)
                {
                    let y_term: f32 = (y as f32) * 0.005_837_15;
                    for x in x0..ix0 {
                        let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                        if dist <= 0.85 {
                            let cov: f32 = if dist < -0.85 {
                                1.0
                            } else {
                                sdf_coverage(dist, aa)
                            };
                            if cov > 0.0 {
                                blend_gradient_row_pixel(row, x, y_term, sample, cov);
                                shaded += 1;
                            }
                        }
                    }
                    for x in ix1..x1 {
                        let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                        if dist <= 0.85 {
                            let cov: f32 = if dist < -0.85 {
                                1.0
                            } else {
                                sdf_coverage(dist, aa)
                            };
                            if cov > 0.0 {
                                blend_gradient_row_pixel(row, x, y_term, sample, cov);
                                shaded += 1;
                            }
                        }
                    }
                }
            }
            for y in iy1..y1 {
                let py: f32 = (y as f32) + 0.5 - cy;
                let ny: f32 = ((y as f32) - rect_y) * inv_h;
                let t: f32 = ((ny - 0.5) * dir_y + 0.5).clamp(0.0, 1.0);
                let sample: Color = start_c.lerp(end_c, t);
                let dist_y: f32 = py.abs() - half_h;
                let mid_cov: f32 = if dist_y > 0.85 {
                    0.0
                } else if dist_y < -0.85 {
                    1.0
                } else {
                    sdf_coverage(dist_y, aa)
                };
                if cx0 < cx1 && mid_cov > 0.0 {
                    shaded +=
                        Self::fill_vertical_gradient_span(band, (cx0, cx1, y), sample, mid_cov);
                }
                if (x0 < cx0 || cx1 < x1)
                    && let Some(row) = band.row_slice_mut(y)
                {
                    let y_term: f32 = (y as f32) * 0.005_837_15;
                    for x in x0..cx0 {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cov: f32 =
                            gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                        if cov > 0.0 {
                            blend_gradient_row_pixel(row, x, y_term, sample, cov);
                            shaded += 1;
                        }
                    }
                    for x in cx1..x1 {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cov: f32 =
                            gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                        if cov > 0.0 {
                            blend_gradient_row_pixel(row, x, y_term, sample, cov);
                            shaded += 1;
                        }
                    }
                }
            }
            return shaded;
        }
        for y in y0..iy0 {
            let Some(row) = band.row_slice_mut(y) else {
                continue;
            };
            let py: f32 = (y as f32) + 0.5 - cy;
            let ny: f32 = ((y as f32) - rect_y) * inv_h;
            let y_bias: f32 = (ny - 0.5) * dir_y + 0.5;
            let y_term: f32 = (y as f32) * 0.005_837_15;
            let dist_y: f32 = py.abs() - half_h;
            let mid_cov: f32 = if dist_y > 0.85 {
                0.0
            } else if dist_y < -0.85 {
                1.0
            } else {
                sdf_coverage(dist_y, aa)
            };
            if cx0 < cx1 && mid_cov > 0.0 {
                let t_start: f32 = (((cx0 as f32) - rect_x) * inv_w - 0.5) * dir_x + y_bias;
                shaded += Self::fill_angled_gradient_span(
                    row,
                    (cx0, cx1, y_term),
                    (t_start, dt_dx, mid_cov),
                    (start_c, end_c),
                );
            }
            for x in x0..cx0 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cov: f32 =
                    gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                if cov > 0.0 {
                    let nx: f32 = ((x as f32) - rect_x) * inv_w;
                    let t: f32 = ((nx - 0.5) * dir_x + y_bias).clamp(0.0, 1.0);
                    blend_gradient_row_pixel(row, x, y_term, start_c.lerp(end_c, t), cov);
                    shaded += 1;
                }
            }
            for x in cx1..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cov: f32 =
                    gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                if cov > 0.0 {
                    let nx: f32 = ((x as f32) - rect_x) * inv_w;
                    let t: f32 = ((nx - 0.5) * dir_x + y_bias).clamp(0.0, 1.0);
                    blend_gradient_row_pixel(row, x, y_term, start_c.lerp(end_c, t), cov);
                    shaded += 1;
                }
            }
        }
        for y in iy0..iy1 {
            let Some(row) = band.row_slice_mut(y) else {
                continue;
            };
            let ny: f32 = ((y as f32) - rect_y) * inv_h;
            let y_bias: f32 = (ny - 0.5) * dir_y + 0.5;
            let y_term: f32 = (y as f32) * 0.005_837_15;
            if ix0 < ix1 {
                let t_start: f32 = (((ix0 as f32) - rect_x) * inv_w - 0.5) * dir_x + y_bias;
                shaded += Self::fill_angled_gradient_span(
                    row,
                    (ix0, ix1, y_term),
                    (t_start, dt_dx, 1.0),
                    (start_c, end_c),
                );
            }
            for x in x0..ix0 {
                let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                if dist <= 0.85 {
                    let cov: f32 = if dist < -0.85 {
                        1.0
                    } else {
                        sdf_coverage(dist, aa)
                    };
                    if cov > 0.0 {
                        let nx: f32 = ((x as f32) - rect_x) * inv_w;
                        let t: f32 = ((nx - 0.5) * dir_x + y_bias).clamp(0.0, 1.0);
                        blend_gradient_row_pixel(row, x, y_term, start_c.lerp(end_c, t), cov);
                        shaded += 1;
                    }
                }
            }
            for x in ix1..x1 {
                let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                if dist <= 0.85 {
                    let cov: f32 = if dist < -0.85 {
                        1.0
                    } else {
                        sdf_coverage(dist, aa)
                    };
                    if cov > 0.0 {
                        let nx: f32 = ((x as f32) - rect_x) * inv_w;
                        let t: f32 = ((nx - 0.5) * dir_x + y_bias).clamp(0.0, 1.0);
                        blend_gradient_row_pixel(row, x, y_term, start_c.lerp(end_c, t), cov);
                        shaded += 1;
                    }
                }
            }
        }
        for y in iy1..y1 {
            let Some(row) = band.row_slice_mut(y) else {
                continue;
            };
            let py: f32 = (y as f32) + 0.5 - cy;
            let ny: f32 = ((y as f32) - rect_y) * inv_h;
            let y_bias: f32 = (ny - 0.5) * dir_y + 0.5;
            let y_term: f32 = (y as f32) * 0.005_837_15;
            let dist_y: f32 = py.abs() - half_h;
            let mid_cov: f32 = if dist_y > 0.85 {
                0.0
            } else if dist_y < -0.85 {
                1.0
            } else {
                sdf_coverage(dist_y, aa)
            };
            if cx0 < cx1 && mid_cov > 0.0 {
                let t_start: f32 = (((cx0 as f32) - rect_x) * inv_w - 0.5) * dir_x + y_bias;
                shaded += Self::fill_angled_gradient_span(
                    row,
                    (cx0, cx1, y_term),
                    (t_start, dt_dx, mid_cov),
                    (start_c, end_c),
                );
            }
            for x in x0..cx0 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cov: f32 =
                    gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                if cov > 0.0 {
                    let nx: f32 = ((x as f32) - rect_x) * inv_w;
                    let t: f32 = ((nx - 0.5) * dir_x + y_bias).clamp(0.0, 1.0);
                    blend_gradient_row_pixel(row, x, y_term, start_c.lerp(end_c, t), cov);
                    shaded += 1;
                }
            }
            for x in cx1..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cov: f32 =
                    gradient_rounded_pixel_coverage((px, py, half_w, half_h), radius, aa);
                if cov > 0.0 {
                    let nx: f32 = ((x as f32) - rect_x) * inv_w;
                    let t: f32 = ((nx - 0.5) * dir_x + y_bias).clamp(0.0, 1.0);
                    blend_gradient_row_pixel(row, x, y_term, start_c.lerp(end_c, t), cov);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Computes the horizontal opaque-interior skip interval `[cut_x0, cut_x1)` for shadow row `y`.
    ///
    /// # Arguments
    ///
    /// - `u32` - Global physical Y row.
    /// - `(Rect, Rect)` - Opaque cutout cross `(horizontal_bar, vertical_bar)`.
    /// - `u32` - Clamped row start X.
    /// - `u32` - Clamped row end X.
    ///
    /// # Returns
    ///
    /// - `(u32, u32)` - Opaque X span safely skippable on row `y`.
    #[must_use]
    fn shadow_row_cutout(y: u32, cutouts: (Rect, Rect), x0: u32, x1: u32) -> (u32, u32) {
        let (cut_h, cut_v): (Rect, Rect) = cutouts;
        let yf: f32 = (y as f32) + 0.5;
        if cut_h.get_width() > 0.0
            && cut_h.get_height() > 0.0
            && yf >= cut_h.get_y()
            && yf < cut_h.bottom()
        {
            let cx0: u32 = (cut_h.get_x().ceil().max(0.0) as u32).clamp(x0, x1);
            let cx1: u32 = (cut_h.right().floor().max(0.0) as u32).clamp(cx0, x1);
            return (cx0, cx1);
        }
        if cut_v.get_width() > 0.0
            && cut_v.get_height() > 0.0
            && yf >= cut_v.get_y()
            && yf < cut_v.bottom()
        {
            let cx0: u32 = (cut_v.get_x().ceil().max(0.0) as u32).clamp(x0, x1);
            let cx1: u32 = (cut_v.right().floor().max(0.0) as u32).clamp(cx0, x1);
            return (cx0, cx1);
        }
        (x1, x1)
    }

    /// Blends the constant-alpha horizontal middle span `[sx0, sx1)` on row `y` while skipping cutout `[cut0, cut1)`.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&ShadowRenderCtx` - Shared shadow geometry and color context.
    /// - `u32` - Global physical Y row coordinate.
    /// - `(u32, u32)` - Opaque cutout interval `(cut0, cut1)` on row `y`.
    /// - `f32` - Constant middle-span shadow opacity `mid_alpha`.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count across the middle span.
    #[inline(always)]
    fn raster_shadow_middle_span(
        band: &mut PixelBand<'_>,
        ctx: &ShadowRenderCtx,
        y: u32,
        cutout: (u32, u32),
        mid_alpha: f32,
    ) -> usize {
        if !ctx.has_mid_x || mid_alpha <= 0.0005 {
            return 0;
        }
        let (cut0, cut1): (u32, u32) = cutout;
        let sx0: u32 = ctx.lx1;
        let sx1: u32 = ctx.rx0;
        let mut shaded: usize = 0;
        if cut0 < cut1 {
            let m0_end: u32 = cut0.min(sx1);
            if sx0 < m0_end {
                band.blend_shadow_span_const_alpha(sx0, m0_end, y, ctx.color, mid_alpha);
                shaded += (m0_end - sx0) as usize;
            }
            let m1_start: u32 = cut1.max(sx0);
            if m1_start < sx1 {
                band.blend_shadow_span_const_alpha(m1_start, sx1, y, ctx.color, mid_alpha);
                shaded += (sx1 - m1_start) as usize;
            }
        } else {
            band.blend_shadow_span_const_alpha(sx0, sx1, y, ctx.color, mid_alpha);
            shaded += (sx1 - sx0) as usize;
        }
        shaded
    }

    /// Rasterizes a single uniform-radius shadow corner row using 2x horizontal left/right symmetry.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&ShadowRenderCtx` - Shared shadow geometry and color context.
    /// - `u32` - Global physical Y row coordinate.
    /// - `(Rect, Rect)` - Opaque cutout rectangles `(cut_h, cut_v)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count on row `y`.
    fn raster_shadow_corner_row_symmetric(
        band: &mut PixelBand<'_>,
        ctx: &ShadowRenderCtx,
        y: u32,
        cutouts: (Rect, Rect),
    ) -> usize {
        let py: f32 = (y as f32) + 0.5 - ctx.cy;
        let py_abs: f32 = py.abs();
        let (cut0, cut1): (u32, u32) = Self::shadow_row_cutout(y, cutouts, ctx.x0, ctx.x1);
        let mut shaded: usize = 0;
        if ctx.has_mid_x {
            let dist_y: f32 = py_abs - ctx.half_h;
            let mid_alpha: f32 = if dist_y < ctx.cutoff {
                ctx.base_alpha * fast_gaussian_shadow_falloff(dist_y, ctx.inv_sigma)
            } else {
                0.0
            };
            shaded += Self::raster_shadow_middle_span(band, ctx, y, (cut0, cut1), mid_alpha);
        }
        let (la0, la1): (u32, u32) = active_span_bounds(ctx.x0, ctx.lx1, cut0, cut1);
        let (ra0, ra1): (u32, u32) = active_span_bounds(ctx.rx0, ctx.x1, cut0, cut1);
        let mut u0: i32 = i32::MAX;
        let mut u1: i32 = i32::MIN;
        if la0 < la1 {
            u0 = u0.min(la0 as i32);
            u1 = u1.max(la1 as i32);
        }
        if ra0 < ra1 {
            u0 = u0.min(ctx.mirror_x_sum - (ra1 as i32) + 1);
            u1 = u1.max(ctx.mirror_x_sum - (ra0 as i32) + 1);
        }
        if u0 >= u1 {
            return shaded;
        }
        let Some(row) = band.row_slice_mut(y) else {
            return shaded;
        };
        let qy: f32 = py_abs - ctx.half_h + ctx.uniform_r;
        let qy_pos: f32 = qy.max(0.0);
        let qy_pos_sq: f32 = qy_pos * qy_pos;
        let sdf_geom: (f32, f32, f32, f32) =
            (ctx.half_w, ctx.uniform_r, ctx.cutoff, ctx.cutoff_r_safe_sq);
        let y_term: f32 = (y as f32) * 0.005_837_15;
        let x0_i: i32 = ctx.x0 as i32;
        let lx1_i: i32 = ctx.lx1 as i32;
        let rx0_i: i32 = ctx.rx0 as i32;
        let x1_i: i32 = ctx.x1 as i32;
        let cut0_i: i32 = cut0 as i32;
        let cut1_i: i32 = cut1 as i32;
        for xl_i in u0..u1 {
            let xr_i: i32 = ctx.mirror_x_sum - xl_i;
            let do_l: bool = xl_i >= x0_i && xl_i < lx1_i && (xl_i < cut0_i || xl_i >= cut1_i);
            let do_r: bool = xr_i >= rx0_i && xr_i < x1_i && (xr_i < cut0_i || xr_i >= cut1_i);
            if !do_l && !do_r {
                continue;
            }
            let px_abs: f32 = ((xl_i as f32) + 0.5 - ctx.cx).abs();
            let dist: f32 = shadow_corner_dist_fast(px_abs, qy, qy_pos_sq, sdf_geom);
            if dist >= ctx.cutoff {
                continue;
            }
            let alpha_f: f32 = ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
            if alpha_f <= 0.0005 {
                continue;
            }
            let a: f32 = alpha_f.clamp(0.0, 1.0);
            if a >= 0.9995 {
                if do_l {
                    let off: usize = (xl_i as usize) * 4;
                    row[off] = ctx.color_rgb[0];
                    row[off + 1] = ctx.color_rgb[1];
                    row[off + 2] = ctx.color_rgb[2];
                    row[off + 3] = 255;
                    shaded += 1;
                }
                if do_r {
                    let off: usize = (xr_i as usize) * 4;
                    row[off] = ctx.color_rgb[0];
                    row[off + 1] = ctx.color_rgb[1];
                    row[off + 2] = ctx.color_rgb[2];
                    row[off + 3] = 255;
                    shaded += 1;
                }
            } else {
                let inv_a: f32 = 1.0 - a;
                let sa_u32: u32 = (a * 255.0) as u32;
                let params: ShadowStripParams = ShadowStripParams {
                    sr_a: ctx.lin_rgb[0] * a,
                    sg_a: ctx.lin_rgb[1] * a,
                    sb_a: ctx.lin_rgb[2] * a,
                    inv_a,
                    sa_u32,
                    inv_a_u32: 255 - sa_u32,
                };
                let mut cache: ShadowPixelCache = ShadowPixelCache::default();
                if do_l {
                    blend_shadow_mirrored_pixel(row, xl_i as u32, y_term, params, &mut cache);
                    shaded += 1;
                }
                if do_r {
                    blend_shadow_mirrored_pixel(row, xr_i as u32, y_term, params, &mut cache);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes a vertically mirrored pair of uniform-radius shadow corner rows (`y_t < y_b`) with 4-quadrant SDF and LUT reuse.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&ShadowRenderCtx` - Shared shadow geometry and color context.
    /// - `u32` - Upper global physical Y row coordinate (`y_t < y_b`).
    /// - `u32` - Lower mirrored global physical Y row coordinate (`y_b > y_t`).
    /// - `(Rect, Rect)` - Opaque cutout rectangles `(cut_h, cut_v)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count across both rows `y_t` and `y_b`.
    fn raster_shadow_corner_pair_symmetric(
        band: &mut PixelBand<'_>,
        ctx: &ShadowRenderCtx,
        y_t: u32,
        y_b: u32,
        cutouts: (Rect, Rect),
    ) -> usize {
        let py: f32 = (y_t as f32) + 0.5 - ctx.cy;
        let py_abs: f32 = py.abs();
        let (cut0_t, cut1_t): (u32, u32) = Self::shadow_row_cutout(y_t, cutouts, ctx.x0, ctx.x1);
        let (cut0_b, cut1_b): (u32, u32) = Self::shadow_row_cutout(y_b, cutouts, ctx.x0, ctx.x1);
        let mut shaded: usize = 0;
        if ctx.has_mid_x {
            let dist_y: f32 = py_abs - ctx.half_h;
            let mid_alpha: f32 = if dist_y < ctx.cutoff {
                ctx.base_alpha * fast_gaussian_shadow_falloff(dist_y, ctx.inv_sigma)
            } else {
                0.0
            };
            if mid_alpha > 0.0005 {
                shaded +=
                    Self::raster_shadow_middle_span(band, ctx, y_t, (cut0_t, cut1_t), mid_alpha);
                shaded +=
                    Self::raster_shadow_middle_span(band, ctx, y_b, (cut0_b, cut1_b), mid_alpha);
            }
        }
        let (lat0, lat1): (u32, u32) = active_span_bounds(ctx.x0, ctx.lx1, cut0_t, cut1_t);
        let (rat0, rat1): (u32, u32) = active_span_bounds(ctx.rx0, ctx.x1, cut0_t, cut1_t);
        let (lab0, lab1): (u32, u32) = active_span_bounds(ctx.x0, ctx.lx1, cut0_b, cut1_b);
        let (rab0, rab1): (u32, u32) = active_span_bounds(ctx.rx0, ctx.x1, cut0_b, cut1_b);
        let mut u0: i32 = i32::MAX;
        let mut u1: i32 = i32::MIN;
        if lat0 < lat1 {
            u0 = u0.min(lat0 as i32);
            u1 = u1.max(lat1 as i32);
        }
        if rat0 < rat1 {
            u0 = u0.min(ctx.mirror_x_sum - (rat1 as i32) + 1);
            u1 = u1.max(ctx.mirror_x_sum - (rat0 as i32) + 1);
        }
        if lab0 < lab1 {
            u0 = u0.min(lab0 as i32);
            u1 = u1.max(lab1 as i32);
        }
        if rab0 < rab1 {
            u0 = u0.min(ctx.mirror_x_sum - (rab1 as i32) + 1);
            u1 = u1.max(ctx.mirror_x_sum - (rab0 as i32) + 1);
        }
        if u0 >= u1 {
            return shaded;
        }
        let Some((row_t, row_b)) = band.two_row_slices_mut(y_t, y_b) else {
            return shaded;
        };
        let qy: f32 = py_abs - ctx.half_h + ctx.uniform_r;
        let qy_pos: f32 = qy.max(0.0);
        let qy_pos_sq: f32 = qy_pos * qy_pos;
        let sdf_geom: (f32, f32, f32, f32) =
            (ctx.half_w, ctx.uniform_r, ctx.cutoff, ctx.cutoff_r_safe_sq);
        let y_term_t: f32 = (y_t as f32) * 0.005_837_15;
        let y_term_b: f32 = (y_b as f32) * 0.005_837_15;
        let x0_i: i32 = ctx.x0 as i32;
        let lx1_i: i32 = ctx.lx1 as i32;
        let rx0_i: i32 = ctx.rx0 as i32;
        let x1_i: i32 = ctx.x1 as i32;
        let cut0_ti: i32 = cut0_t as i32;
        let cut1_ti: i32 = cut1_t as i32;
        let cut0_bi: i32 = cut0_b as i32;
        let cut1_bi: i32 = cut1_b as i32;
        for xl_i in u0..u1 {
            let xr_i: i32 = ctx.mirror_x_sum - xl_i;
            let in_l: bool = xl_i >= x0_i && xl_i < lx1_i;
            let in_r: bool = xr_i >= rx0_i && xr_i < x1_i;
            let do_lt: bool = in_l && (xl_i < cut0_ti || xl_i >= cut1_ti);
            let do_rt: bool = in_r && (xr_i < cut0_ti || xr_i >= cut1_ti);
            let do_lb: bool = in_l && (xl_i < cut0_bi || xl_i >= cut1_bi);
            let do_rb: bool = in_r && (xr_i < cut0_bi || xr_i >= cut1_bi);
            if !do_lt && !do_rt && !do_lb && !do_rb {
                continue;
            }
            let px_abs: f32 = ((xl_i as f32) + 0.5 - ctx.cx).abs();
            let dist: f32 = shadow_corner_dist_fast(px_abs, qy, qy_pos_sq, sdf_geom);
            if dist >= ctx.cutoff {
                continue;
            }
            let alpha_f: f32 = ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
            if alpha_f <= 0.0005 {
                continue;
            }
            let a: f32 = alpha_f.clamp(0.0, 1.0);
            if a >= 0.9995 {
                if do_lt {
                    let off: usize = (xl_i as usize) * 4;
                    row_t[off] = ctx.color_rgb[0];
                    row_t[off + 1] = ctx.color_rgb[1];
                    row_t[off + 2] = ctx.color_rgb[2];
                    row_t[off + 3] = 255;
                    shaded += 1;
                }
                if do_rt {
                    let off: usize = (xr_i as usize) * 4;
                    row_t[off] = ctx.color_rgb[0];
                    row_t[off + 1] = ctx.color_rgb[1];
                    row_t[off + 2] = ctx.color_rgb[2];
                    row_t[off + 3] = 255;
                    shaded += 1;
                }
                if do_lb {
                    let off: usize = (xl_i as usize) * 4;
                    row_b[off] = ctx.color_rgb[0];
                    row_b[off + 1] = ctx.color_rgb[1];
                    row_b[off + 2] = ctx.color_rgb[2];
                    row_b[off + 3] = 255;
                    shaded += 1;
                }
                if do_rb {
                    let off: usize = (xr_i as usize) * 4;
                    row_b[off] = ctx.color_rgb[0];
                    row_b[off + 1] = ctx.color_rgb[1];
                    row_b[off + 2] = ctx.color_rgb[2];
                    row_b[off + 3] = 255;
                    shaded += 1;
                }
            } else {
                let inv_a: f32 = 1.0 - a;
                let sa_u32: u32 = (a * 255.0) as u32;
                let params: ShadowStripParams = ShadowStripParams {
                    sr_a: ctx.lin_rgb[0] * a,
                    sg_a: ctx.lin_rgb[1] * a,
                    sb_a: ctx.lin_rgb[2] * a,
                    inv_a,
                    sa_u32,
                    inv_a_u32: 255 - sa_u32,
                };
                let mut cache: ShadowPixelCache = ShadowPixelCache::default();
                if do_lt {
                    blend_shadow_mirrored_pixel(row_t, xl_i as u32, y_term_t, params, &mut cache);
                    shaded += 1;
                }
                if do_rt {
                    blend_shadow_mirrored_pixel(row_t, xr_i as u32, y_term_t, params, &mut cache);
                    shaded += 1;
                }
                if do_lb {
                    blend_shadow_mirrored_pixel(row_b, xl_i as u32, y_term_b, params, &mut cache);
                    shaded += 1;
                }
                if do_rb {
                    blend_shadow_mirrored_pixel(row_b, xr_i as u32, y_term_b, params, &mut cache);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes a general (asymmetric-radius or unaligned-center) shadow corner row directly into `row_slice_mut`.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&ShadowRenderCtx` - Shared shadow geometry and color context.
    /// - `CornerRadius` - Per-quadrant physical corner radii.
    /// - `u32` - Global physical Y row coordinate.
    /// - `(Rect, Rect)` - Opaque cutout rectangles `(cut_h, cut_v)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count on row `y`.
    fn raster_shadow_corner_row_general(
        band: &mut PixelBand<'_>,
        ctx: &ShadowRenderCtx,
        radius: CornerRadius,
        y: u32,
        cutouts: (Rect, Rect),
    ) -> usize {
        let py: f32 = (y as f32) + 0.5 - ctx.cy;
        let py_abs: f32 = py.abs();
        let (cut0, cut1): (u32, u32) = Self::shadow_row_cutout(y, cutouts, ctx.x0, ctx.x1);
        let spans: [(u32, u32); 2] = if cut0 < cut1 {
            [(ctx.x0, cut0), (cut1, ctx.x1)]
        } else {
            [(ctx.x0, ctx.x1), (ctx.x1, ctx.x1)]
        };
        let mut shaded: usize = 0;
        if ctx.has_mid_x {
            let dist_y: f32 = py_abs - ctx.half_h;
            let mid_alpha: f32 = if dist_y < ctx.cutoff {
                ctx.base_alpha * fast_gaussian_shadow_falloff(dist_y, ctx.inv_sigma)
            } else {
                0.0
            };
            shaded += Self::raster_shadow_middle_span(band, ctx, y, (cut0, cut1), mid_alpha);
        }
        let Some(row) = band.row_slice_mut(y) else {
            return shaded;
        };
        let y_term: f32 = (y as f32) * 0.005_837_15;
        let r_l: f32 = (if py < 0.0 {
            radius.get_top_left()
        } else {
            radius.get_bottom_left()
        })
        .min(ctx.half_w)
        .min(ctx.half_h)
        .max(0.0);
        let r_r: f32 = (if py < 0.0 {
            radius.get_top_right()
        } else {
            radius.get_bottom_right()
        })
        .min(ctx.half_w)
        .min(ctx.half_h)
        .max(0.0);
        let qy_l: f32 = py_abs - ctx.half_h + r_l;
        let qy_l_pos: f32 = qy_l.max(0.0);
        let qy_l_sq: f32 = qy_l_pos * qy_l_pos;
        let cut_l_r: f32 = ctx.cutoff + r_l + 1e-3;
        let geom_l: (f32, f32, f32, f32) = (ctx.half_w, r_l, ctx.cutoff, cut_l_r * cut_l_r);
        let qy_r: f32 = py_abs - ctx.half_h + r_r;
        let qy_r_pos: f32 = qy_r.max(0.0);
        let qy_r_sq: f32 = qy_r_pos * qy_r_pos;
        let cut_r_r: f32 = ctx.cutoff + r_r + 1e-3;
        let geom_r: (f32, f32, f32, f32) = (ctx.half_w, r_r, ctx.cutoff, cut_r_r * cut_r_r);
        for (rx0, rx1) in spans.into_iter() {
            if rx0 >= rx1 {
                continue;
            }
            if ctx.has_mid_x {
                let l_end: u32 = rx1.min(ctx.lx1);
                for x in rx0..l_end {
                    let px: f32 = (x as f32) + 0.5 - ctx.cx;
                    let dist: f32 = if px < 0.0 {
                        shadow_corner_dist_fast(px.abs(), qy_l, qy_l_sq, geom_l)
                    } else {
                        shadow_corner_dist_fast(px.abs(), qy_r, qy_r_sq, geom_r)
                    };
                    if dist < ctx.cutoff {
                        let alpha_f: f32 =
                            ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
                        if alpha_f > 0.0005 {
                            blend_shadow_row_pixel(
                                row,
                                x,
                                y_term,
                                ctx.color_rgb,
                                ctx.lin_rgb,
                                alpha_f,
                            );
                            shaded += 1;
                        }
                    }
                }
                let r_start: u32 = rx0.max(ctx.rx0);
                for x in r_start..rx1 {
                    let px: f32 = (x as f32) + 0.5 - ctx.cx;
                    let dist: f32 = if px < 0.0 {
                        shadow_corner_dist_fast(px.abs(), qy_l, qy_l_sq, geom_l)
                    } else {
                        shadow_corner_dist_fast(px.abs(), qy_r, qy_r_sq, geom_r)
                    };
                    if dist < ctx.cutoff {
                        let alpha_f: f32 =
                            ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
                        if alpha_f > 0.0005 {
                            blend_shadow_row_pixel(
                                row,
                                x,
                                y_term,
                                ctx.color_rgb,
                                ctx.lin_rgb,
                                alpha_f,
                            );
                            shaded += 1;
                        }
                    }
                }
            } else {
                for x in rx0..rx1 {
                    let px: f32 = (x as f32) + 0.5 - ctx.cx;
                    let dist: f32 = if px < 0.0 {
                        shadow_corner_dist_fast(px.abs(), qy_l, qy_l_sq, geom_l)
                    } else {
                        shadow_corner_dist_fast(px.abs(), qy_r, qy_r_sq, geom_r)
                    };
                    if dist >= ctx.cutoff {
                        continue;
                    }
                    let alpha_f: f32 =
                        ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
                    if alpha_f > 0.0005 {
                        blend_shadow_row_pixel(row, x, y_term, ctx.color_rgb, ctx.lin_rgb, alpha_f);
                        shaded += 1;
                    }
                }
            }
        }
        shaded
    }

    /// Rasterizes the vertical left/right shadow side strips across `sy0..sy1` using a precomputed 1D falloff LUT.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&ShadowRenderCtx` - Shared shadow geometry and color context.
    /// - `(u32, u32)` - Vertical interior row interval `(sy0, sy1)`.
    /// - `(Rect, Rect)` - Opaque cutout rectangles `(cut_h, cut_v)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count across `sy0..sy1`.
    fn raster_shadow_vertical_strips(
        band: &mut PixelBand<'_>,
        ctx: &ShadowRenderCtx,
        y_span: (u32, u32),
        cutouts: (Rect, Rect),
    ) -> usize {
        let (sy0, sy1): (u32, u32) = y_span;
        if sy0 >= sy1 {
            return 0;
        }
        let (cut_h, _): (Rect, Rect) = cutouts;
        let sy0_f: f32 = (sy0 as f32) + 0.5;
        let sy1_last_f: f32 = ((sy1 - 1) as f32) + 0.5;
        let const_cut: Option<(u32, u32)> = if cut_h.get_width() > 0.0
            && cut_h.get_height() > 0.0
            && sy0_f >= cut_h.get_y()
            && sy1_last_f < cut_h.bottom()
        {
            let cx0: u32 = (cut_h.get_x().ceil().max(0.0) as u32).clamp(ctx.x0, ctx.x1);
            let cx1: u32 = (cut_h.right().floor().max(0.0) as u32).clamp(cx0, ctx.x1);
            Some((cx0, cx1))
        } else {
            None
        };
        let (l_build0, l_build1): (u32, u32) = if let Some((c0, c1)) = const_cut {
            active_span_bounds(ctx.x0, ctx.lx1, c0, c1)
        } else {
            (ctx.x0, ctx.lx1)
        };
        let (r_build0, r_build1): (u32, u32) = if let Some((c0, c1)) = const_cut {
            active_span_bounds(ctx.rx0, ctx.x1, c0, c1)
        } else {
            (ctx.rx0, ctx.x1)
        };
        let mut shaded: usize = 0;
        let ran_tls: bool = BAND_SHADOW_SCRATCH
            .try_with(|cell: &RefCell<BandShadowScratch>| {
                if let Ok(mut scratch) = cell.try_borrow_mut() {
                    let BandShadowScratch {
                        left_hot,
                        left_params,
                        right_hot,
                        right_params,
                    } = &mut *scratch;
                    let (l_act0, l_act1): (u32, u32) = build_shadow_vertical_strip(
                        (l_build0, l_build1),
                        ctx,
                        left_hot,
                        left_params,
                    );
                    let (r_act0, r_act1): (u32, u32) = build_shadow_vertical_strip(
                        (r_build0, r_build1),
                        ctx,
                        right_hot,
                        right_params,
                    );
                    if l_act0 >= l_act1 && r_act0 >= r_act1 {
                        return;
                    }
                    let l_hot_s: &mut [ShadowStripHot] = left_hot.as_mut_slice();
                    let l_par_s: &[ShadowStripParams] = left_params.as_slice();
                    let r_hot_s: &mut [ShadowStripHot] = right_hot.as_mut_slice();
                    let r_par_s: &[ShadowStripParams] = right_params.as_slice();
                    for y in sy0..sy1 {
                        let Some(row) = band.row_slice_mut(y) else {
                            continue;
                        };
                        let y_term: f32 = (y as f32) * 0.005_837_15;
                        let (cut0, cut1): (u32, u32) = const_cut
                            .unwrap_or_else(|| Self::shadow_row_cutout(y, cutouts, ctx.x0, ctx.x1));
                        if l_act0 < l_act1 {
                            if cut0 < cut1 {
                                let a_end: u32 = cut0.min(l_act1);
                                if l_act0 < a_end {
                                    shaded += apply_shadow_strip_span(
                                        row,
                                        (l_act0, a_end, l_act0),
                                        y_term,
                                        ctx.color_rgb,
                                        l_hot_s,
                                        l_par_s,
                                    );
                                }
                                let b_start: u32 = cut1.max(l_act0);
                                if b_start < l_act1 {
                                    shaded += apply_shadow_strip_span(
                                        row,
                                        (b_start, l_act1, l_act0),
                                        y_term,
                                        ctx.color_rgb,
                                        l_hot_s,
                                        l_par_s,
                                    );
                                }
                            } else {
                                shaded += apply_shadow_strip_span(
                                    row,
                                    (l_act0, l_act1, l_act0),
                                    y_term,
                                    ctx.color_rgb,
                                    l_hot_s,
                                    l_par_s,
                                );
                            }
                        }
                        if r_act0 < r_act1 {
                            if cut0 < cut1 {
                                let a_end: u32 = cut0.min(r_act1);
                                if r_act0 < a_end {
                                    shaded += apply_shadow_strip_span(
                                        row,
                                        (r_act0, a_end, r_act0),
                                        y_term,
                                        ctx.color_rgb,
                                        r_hot_s,
                                        r_par_s,
                                    );
                                }
                                let b_start: u32 = cut1.max(r_act0);
                                if b_start < r_act1 {
                                    shaded += apply_shadow_strip_span(
                                        row,
                                        (b_start, r_act1, r_act0),
                                        y_term,
                                        ctx.color_rgb,
                                        r_hot_s,
                                        r_par_s,
                                    );
                                }
                            } else {
                                shaded += apply_shadow_strip_span(
                                    row,
                                    (r_act0, r_act1, r_act0),
                                    y_term,
                                    ctx.color_rgb,
                                    r_hot_s,
                                    r_par_s,
                                );
                            }
                        }
                    }
                }
            })
            .is_ok();
        if !ran_tls {
            for y in sy0..sy1 {
                let Some(row) = band.row_slice_mut(y) else {
                    continue;
                };
                let y_term: f32 = (y as f32) * 0.005_837_15;
                let (cut0, cut1): (u32, u32) = const_cut
                    .unwrap_or_else(|| Self::shadow_row_cutout(y, cutouts, ctx.x0, ctx.x1));
                let spans: [(u32, u32); 2] = if cut0 < cut1 {
                    [(ctx.x0, cut0), (cut1, ctx.x1)]
                } else {
                    [(ctx.x0, ctx.x1), (ctx.x1, ctx.x1)]
                };
                for (rx0, rx1) in spans.into_iter() {
                    let left_end: u32 = rx1.min(ctx.lx1);
                    for x in rx0..left_end {
                        let dist: f32 = ((x as f32) + 0.5 - ctx.cx).abs() - ctx.half_w;
                        if dist < ctx.cutoff {
                            let alpha_f: f32 =
                                ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
                            if alpha_f > 0.0005 {
                                blend_shadow_row_pixel(
                                    row,
                                    x,
                                    y_term,
                                    ctx.color_rgb,
                                    ctx.lin_rgb,
                                    alpha_f,
                                );
                                shaded += 1;
                            }
                        }
                    }
                    let right_start: u32 = rx0.max(ctx.rx0);
                    for x in right_start..rx1 {
                        let dist: f32 = ((x as f32) + 0.5 - ctx.cx).abs() - ctx.half_w;
                        if dist < ctx.cutoff {
                            let alpha_f: f32 =
                                ctx.base_alpha * fast_gaussian_shadow_falloff(dist, ctx.inv_sigma);
                            if alpha_f > 0.0005 {
                                blend_shadow_row_pixel(
                                    row,
                                    x,
                                    y_term,
                                    ctx.color_rgb,
                                    ctx.lin_rgb,
                                    alpha_f,
                                );
                                shaded += 1;
                            }
                        }
                    }
                }
            }
        }
        shaded
    }

    /// Rasterizes a lossless 2D analytical Gaussian `erfc` LUT drop shadow skipping strictly-opaque card interiors.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `(Rect, Rect, Rect)` - Tuple `(shadow_rect, caster_rect, clip)` in physical pixels.
    /// - `CornerRadius` - Per-quadrant physical corner radii.
    /// - `f32` - Gaussian standard deviation `sigma` in physical pixels.
    /// - `Color` - Shadow tint color.
    /// - `(Rect, Rect)` - Strictly-opaque interior cutout rectangles from immediately following opaque card.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_shadow(
        band: &mut PixelBand<'_>,
        rects: (Rect, Rect, Rect),
        radius: CornerRadius,
        sigma: f32,
        color: Color,
        cutouts: (Rect, Rect),
    ) -> usize {
        let (shadow_rect, caster_rect, clip): (Rect, Rect, Rect) = rects;
        if color.get_a() == 0 {
            return 0;
        }
        let base_alpha: f32 = (color.get_a() as f32) * (1.0 / 255.0);
        if base_alpha <= 0.0005 {
            return 0;
        }
        let Some(vis) = shadow_rect.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        let half_w: f32 = (caster_rect.get_width() * 0.5).max(0.5);
        let half_h: f32 = (caster_rect.get_height() * 0.5).max(0.5);
        let cx: f32 = caster_rect.get_x() + half_w;
        let cy: f32 = caster_rect.get_y() + half_h;
        let inv_sigma: f32 = 1.0 / sigma.max(0.5);
        let cutoff: f32 = sigma * 3.0;
        let r_max: f32 = radius.max_radius();
        let inner_inset: f32 = cutoff + r_max;
        let sy0: u32 = ((caster_rect.get_y() + inner_inset).ceil().max(0.0) as u32).clamp(y0, y1);
        let sy1: u32 = ((caster_rect.bottom() - inner_inset).floor().max(0.0) as u32).clamp(y0, y1);
        let sx0: u32 = ((caster_rect.get_x() + inner_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let sx1: u32 = ((caster_rect.right() - inner_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_mid_x: bool = sx0 < sx1;
        let has_mid_y: bool = sy0 < sy1;
        let r_tl: f32 = radius.get_top_left();
        let uniform_radius: bool = r_tl == radius.get_top_right()
            && r_tl == radius.get_bottom_right()
            && r_tl == radius.get_bottom_left();
        let two_cx_f: f32 = 2.0 * cx;
        let two_cx_i: i32 = two_cx_f.round() as i32;
        let x_symmetric: bool =
            uniform_radius && (two_cx_i as f32) == two_cx_f && two_cx_f.abs() < 16_777_216.0;
        let two_cy_f: f32 = 2.0 * cy;
        let two_cy_i: i32 = two_cy_f.round() as i32;
        let y_symmetric: bool =
            x_symmetric && (two_cy_i as f32) == two_cy_f && two_cy_f.abs() < 16_777_216.0;
        let uniform_r: f32 = r_tl.min(half_w).min(half_h).max(0.0);
        let cutoff_r_safe: f32 = cutoff + uniform_r + 1e-3;
        let (lx1, rx0): (u32, u32) = if has_mid_x {
            (sx0, sx1)
        } else if x_symmetric {
            let split_x: u32 = ((two_cx_i + 1).div_euclid(2).max(0) as u32).clamp(x0, x1);
            (split_x, split_x)
        } else {
            (x1, x1)
        };
        let color_rgb: [u8; 3] = [color.get_r(), color.get_g(), color.get_b()];
        let lin_rgb: [f32; 3] = [
            SRGB_TO_LIN_LUT[color_rgb[0] as usize],
            SRGB_TO_LIN_LUT[color_rgb[1] as usize],
            SRGB_TO_LIN_LUT[color_rgb[2] as usize],
        ];
        let ctx: ShadowRenderCtx = ShadowRenderCtx {
            color,
            color_rgb,
            lin_rgb,
            cx,
            cy,
            half_w,
            half_h,
            base_alpha,
            inv_sigma,
            cutoff,
            uniform_r,
            cutoff_r_safe_sq: cutoff_r_safe * cutoff_r_safe,
            x0,
            x1,
            lx1,
            rx0,
            mirror_x_sum: two_cx_i - 1,
            has_mid_x,
        };
        let mut shaded: usize = 0;
        if x_symmetric {
            let (top_end, bot_start): (u32, u32) = if has_mid_x && has_mid_y {
                (sy0, sy1)
            } else if y_symmetric {
                let split_y: u32 = ((two_cy_i + 1).div_euclid(2).max(0) as u32).clamp(y0, y1);
                (split_y, split_y)
            } else {
                (y1, y1)
            };
            let mirror_y_sum: i32 = two_cy_i - 1;
            let mut mirrored_b0: u32 = y1;
            let mut mirrored_b1: u32 = bot_start;
            for y_t in y0..top_end {
                let y_b_i: i32 = if y_symmetric {
                    mirror_y_sum - (y_t as i32)
                } else {
                    -1
                };
                if y_b_i >= (bot_start as i32) && y_b_i < (y1 as i32) {
                    let y_b: u32 = y_b_i as u32;
                    mirrored_b0 = mirrored_b0.min(y_b);
                    mirrored_b1 = mirrored_b1.max(y_b + 1);
                    shaded +=
                        Self::raster_shadow_corner_pair_symmetric(band, &ctx, y_t, y_b, cutouts);
                } else {
                    shaded += Self::raster_shadow_corner_row_symmetric(band, &ctx, y_t, cutouts);
                }
            }
            if has_mid_x && has_mid_y {
                shaded += Self::raster_shadow_vertical_strips(band, &ctx, (sy0, sy1), cutouts);
            }
            if mirrored_b0 < mirrored_b1 {
                for y_b in bot_start..mirrored_b0 {
                    shaded += Self::raster_shadow_corner_row_symmetric(band, &ctx, y_b, cutouts);
                }
                for y_b in mirrored_b1..y1 {
                    shaded += Self::raster_shadow_corner_row_symmetric(band, &ctx, y_b, cutouts);
                }
            } else {
                for y_b in bot_start..y1 {
                    shaded += Self::raster_shadow_corner_row_symmetric(band, &ctx, y_b, cutouts);
                }
            }
        } else if has_mid_x && has_mid_y {
            for y in y0..sy0 {
                shaded += Self::raster_shadow_corner_row_general(band, &ctx, radius, y, cutouts);
            }
            shaded += Self::raster_shadow_vertical_strips(band, &ctx, (sy0, sy1), cutouts);
            for y in sy1..y1 {
                shaded += Self::raster_shadow_corner_row_general(band, &ctx, radius, y, cutouts);
            }
        } else {
            for y in y0..y1 {
                shaded += Self::raster_shadow_corner_row_general(band, &ctx, radius, y, cutouts);
            }
        }
        shaded
    }

    /// Rasterizes a pre-positioned glyph slice into `band` using direct slot indexing with zero HashMap lookups.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `&[PositionedGlyph]` - Pre-positioned glyph instances for this text run.
    /// - `Color` - Foreground text color.
    /// - `Rect` - Active physical clip bounds.
    /// - `&GlyphAtlas` - Read-only pre-warmed glyph atlas.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_text_run_band(
        band: &mut PixelBand<'_>,
        positioned_glyphs: &[PositionedGlyph],
        color: Color,
        clip: Rect,
        glyph_atlas: &GlyphAtlas,
    ) -> usize {
        let fb_w: i32 = band.width() as i32;
        let band_y0: i32 = band.y_offset() as i32;
        let band_y1: i32 = band.y_end() as i32;
        let clip_x0: i32 = clip.get_x().ceil().max(0.0) as i32;
        let clip_y0: i32 = (clip.get_y().ceil().max(0.0) as i32).clamp(band_y0, band_y1);
        let clip_x1: i32 = (clip.right().floor().max(0.0) as i32).min(fb_w);
        let clip_y1: i32 = (clip.bottom().floor().max(0.0) as i32).clamp(band_y0, band_y1);
        if clip_x0 >= clip_x1 || clip_y0 >= clip_y1 {
            return 0;
        }
        let mut shaded: usize = 0;
        for pg in positioned_glyphs.iter() {
            let gy0: i32 = pg.y;
            let gh: i32 = pg.height as i32;
            if gy0 >= clip_y1 || gy0 + gh <= clip_y0 {
                continue;
            }
            let gx0: i32 = pg.x;
            let gw: i32 = pg.width as i32;
            if gx0 >= clip_x1 || gx0 + gw <= clip_x0 {
                continue;
            }
            let glyph: &RasterizedGlyph = glyph_atlas.glyph_by_slot(pg.glyph_idx);
            let py_start: i32 = (clip_y0 - gy0).max(0);
            let py_end: i32 = (clip_y1 - gy0).min(gh);
            let px_start: i32 = (clip_x0 - gx0).max(0);
            let px_end: i32 = (clip_x1 - gx0).min(gw);
            if py_start < py_end && px_start < px_end {
                let cov_bytes: &[u8] = glyph.get_coverage().as_slice();
                let gw_usize: usize = gw as usize;
                let dx: u32 = (gx0 + px_start) as u32;
                let start_u: usize = px_start as usize;
                let end_u: usize = px_end as usize;
                for py in py_start..py_end {
                    let dy: u32 = (gy0 + py) as u32;
                    let row_off: usize = (py as usize) * gw_usize;
                    let row_slice: &[u8] = &cov_bytes[row_off + start_u..row_off + end_u];
                    shaded += band.blend_glyph_row(dx, dy, row_slice, color);
                }
            }
        }
        shaded
    }

    /// Blits a full-coverage bilinear RGBA8 horizontal sub-span using precomputed 8.8 vertical and horizontal LUT entries.
    ///
    /// # Arguments
    ///
    /// - `(usize, usize, u32, u32)` - Vertical row parameters `(row0_px, row1_px, wy_256, iwy_256)`.
    /// - `usize` - Starting destination X pixel coordinate within `dst_row`.
    /// - `&[(usize, usize, u32)]` - Horizontal sample LUT slice `(sx0, sx1, wx_256)` for this span.
    /// - `&[u8]` - Source RGBA8 pixel slice.
    /// - `&mut [u8]` - Destination framebuffer row slice.
    #[inline(always)]
    fn blit_bilinear_span_precomputed(
        y_params: (usize, usize, u32, u32),
        dst_x0: usize,
        x_lut: &[(usize, usize, u32)],
        src_bytes: &[u8],
        dst_row: &mut [u8],
    ) {
        let (row0, row1, wy, iwy): (usize, usize, u32, u32) = y_params;
        let span_bytes_start: usize = dst_x0 * 4;
        let span_bytes_end: usize = (dst_x0 + x_lut.len()) * 4;
        if span_bytes_end > dst_row.len() {
            return;
        }
        let dst_chunks: &mut [[u8; 4]] = dst_row[span_bytes_start..span_bytes_end]
            .as_chunks_mut::<4>()
            .0;
        let src_len: usize = src_bytes.len();
        if wy == 0 {
            for (dp, &(sx0, sx1, wx)) in dst_chunks.iter_mut().zip(x_lut.iter()) {
                let i0: usize = (row0 + sx0) * 4;
                let i1: usize = (row0 + sx1) * 4;
                if i1 + 4 > src_len {
                    continue;
                }
                let iwx: u32 = 256 - wx;
                let a0: u32 = src_bytes[i0 + 3] as u32;
                let a1: u32 = src_bytes[i1 + 3] as u32;
                if (a0 & a1) == 255 {
                    dp[0] = (((src_bytes[i0] as u32) * iwx + (src_bytes[i1] as u32) * wx + 128)
                        >> 8) as u8;
                    dp[1] =
                        (((src_bytes[i0 + 1] as u32) * iwx + (src_bytes[i1 + 1] as u32) * wx + 128)
                            >> 8) as u8;
                    dp[2] =
                        (((src_bytes[i0 + 2] as u32) * iwx + (src_bytes[i1 + 2] as u32) * wx + 128)
                            >> 8) as u8;
                    dp[3] = 255;
                    continue;
                }
                let a_val: u32 = (a0 * iwx + a1 * wx + 128) >> 8;
                if a_val == 0 {
                    continue;
                }
                let r: u32 =
                    ((src_bytes[i0] as u32) * iwx + (src_bytes[i1] as u32) * wx + 128) >> 8;
                let g: u32 =
                    ((src_bytes[i0 + 1] as u32) * iwx + (src_bytes[i1 + 1] as u32) * wx + 128) >> 8;
                let b: u32 =
                    ((src_bytes[i0 + 2] as u32) * iwx + (src_bytes[i1 + 2] as u32) * wx + 128) >> 8;
                let inv_a: u32 = 255 - a_val;
                let vr: u32 = r * a_val + (dp[0] as u32) * inv_a + 128;
                let vg: u32 = g * a_val + (dp[1] as u32) * inv_a + 128;
                let vb: u32 = b * a_val + (dp[2] as u32) * inv_a + 128;
                dp[0] = ((vr + (vr >> 8)) >> 8) as u8;
                dp[1] = ((vg + (vg >> 8)) >> 8) as u8;
                dp[2] = ((vb + (vb >> 8)) >> 8) as u8;
                dp[3] = 255;
            }
            return;
        }
        for (dp, &(sx0, sx1, wx)) in dst_chunks.iter_mut().zip(x_lut.iter()) {
            let i00: usize = (row0 + sx0) * 4;
            let i10: usize = (row0 + sx1) * 4;
            let i01: usize = (row1 + sx0) * 4;
            let i11: usize = (row1 + sx1) * 4;
            if i11 + 4 > src_len {
                continue;
            }
            let iwx: u32 = 256 - wx;
            let w00: u32 = iwx * iwy;
            let w10: u32 = wx * iwy;
            let w01: u32 = iwx * wy;
            let w11: u32 = wx * wy;
            let a00: u32 = src_bytes[i00 + 3] as u32;
            let a10: u32 = src_bytes[i10 + 3] as u32;
            let a01: u32 = src_bytes[i01 + 3] as u32;
            let a11: u32 = src_bytes[i11 + 3] as u32;
            if (a00 & a10 & a01 & a11) == 255 {
                dp[0] = (((src_bytes[i00] as u32) * w00
                    + (src_bytes[i10] as u32) * w10
                    + (src_bytes[i01] as u32) * w01
                    + (src_bytes[i11] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dp[1] = (((src_bytes[i00 + 1] as u32) * w00
                    + (src_bytes[i10 + 1] as u32) * w10
                    + (src_bytes[i01 + 1] as u32) * w01
                    + (src_bytes[i11 + 1] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dp[2] = (((src_bytes[i00 + 2] as u32) * w00
                    + (src_bytes[i10 + 2] as u32) * w10
                    + (src_bytes[i01 + 2] as u32) * w01
                    + (src_bytes[i11 + 2] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dp[3] = 255;
                continue;
            }
            let a_val: u32 = (a00 * w00 + a10 * w10 + a01 * w01 + a11 * w11 + 32768) >> 16;
            if a_val == 0 {
                continue;
            }
            let r: u32 = ((src_bytes[i00] as u32) * w00
                + (src_bytes[i10] as u32) * w10
                + (src_bytes[i01] as u32) * w01
                + (src_bytes[i11] as u32) * w11
                + 32768)
                >> 16;
            let g: u32 = ((src_bytes[i00 + 1] as u32) * w00
                + (src_bytes[i10 + 1] as u32) * w10
                + (src_bytes[i01 + 1] as u32) * w01
                + (src_bytes[i11 + 1] as u32) * w11
                + 32768)
                >> 16;
            let b: u32 = ((src_bytes[i00 + 2] as u32) * w00
                + (src_bytes[i10 + 2] as u32) * w10
                + (src_bytes[i01 + 2] as u32) * w01
                + (src_bytes[i11 + 2] as u32) * w11
                + 32768)
                >> 16;
            let inv_a: u32 = 255 - a_val;
            let vr: u32 = r * a_val + (dp[0] as u32) * inv_a + 128;
            let vg: u32 = g * a_val + (dp[1] as u32) * inv_a + 128;
            let vb: u32 = b * a_val + (dp[2] as u32) * inv_a + 128;
            dp[0] = ((vr + (vr >> 8)) >> 8) as u8;
            dp[1] = ((vg + (vg >> 8)) >> 8) as u8;
            dp[2] = ((vb + (vb >> 8)) >> 8) as u8;
            dp[3] = 255;
        }
    }

    /// Executes bilinear image rasterization using reusable horizontal LUT buffer `x_lut`.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Physical destination bounds.
    /// - `(u32, u32, &[u8])` - Source dimensions and RGBA8 slice `(src_w, src_h, src_bytes)`.
    /// - `(u32, u32, u32, u32)` - Clipped visible pixel bounds `(x0, y0, x1, y1)`.
    /// - `CornerRadius` - Physical corner radii.
    /// - `AntialiasingMode` - Active AA policy.
    /// - `&mut Vec<(usize, usize, u32)>` - Reusable horizontal sample LUT buffer.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_image_bilinear_with_lut(
        band: &mut PixelBand<'_>,
        rect: Rect,
        src_info: (u32, u32, &[u8]),
        bounds: (u32, u32, u32, u32),
        radius: CornerRadius,
        aa: AntialiasingMode,
        x_lut: &mut Vec<(usize, usize, u32)>,
    ) -> usize {
        let (src_w, src_h, src_bytes): (u32, u32, &[u8]) = src_info;
        let (x0, y0, x1, y1): (u32, u32, u32, u32) = bounds;
        let rect_x: f32 = rect.get_x();
        let rect_y: f32 = rect.get_y();
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect_x + half_w;
        let cy: f32 = rect_y + half_h;
        let inv_dst_w: f32 = 1.0 / rect.get_width().max(1.0);
        let inv_dst_h: f32 = 1.0 / rect.get_height().max(1.0);
        let src_w_f: f32 = src_w as f32;
        let src_h_f: f32 = src_h as f32;
        let src_w_usize: usize = src_w as usize;
        let max_sx: usize = src_w_usize.saturating_sub(1);
        let max_sy: usize = (src_h as usize).saturating_sub(1);
        let has_radius: bool = radius.max_radius() > 0.0;
        x_lut.clear();
        for x in x0..x1 {
            let u: f32 = (((x as f32) + 0.5 - rect_x) * inv_dst_w).clamp(0.0, 1.0);
            let fx: f32 = (u * src_w_f - 0.5).clamp(0.0, max_sx as f32);
            let sx0: usize = (fx.floor() as usize).min(max_sx);
            let sx1: usize = (sx0 + 1).min(max_sx);
            let wx: u32 = ((fx - (sx0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
            x_lut.push((sx0, sx1, wx));
        }
        let x_lut_ref: &[(usize, usize, u32)] = x_lut.as_slice();
        if !has_radius {
            let x0_u: usize = x0 as usize;
            for y in y0..y1 {
                let Some(dst_row) = band.row_slice_mut(y) else {
                    continue;
                };
                let v: f32 = (((y as f32) + 0.5 - rect_y) * inv_dst_h).clamp(0.0, 1.0);
                let fy: f32 = (v * src_h_f - 0.5).clamp(0.0, max_sy as f32);
                let sy0: usize = (fy.floor() as usize).min(max_sy);
                let sy1: usize = (sy0 + 1).min(max_sy);
                let wy: u32 = ((fy - (sy0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
                let y_params: (usize, usize, u32, u32) =
                    (sy0 * src_w_usize, sy1 * src_w_usize, wy, 256 - wy);
                Self::blit_bilinear_span_precomputed(y_params, x0_u, x_lut_ref, src_bytes, dst_row);
            }
            return ((x1 - x0) as usize) * ((y1 - y0) as usize);
        }
        let r_max: f32 = radius.max_radius().min(half_w).min(half_h).max(0.0);
        let ix0: u32 = ((rect_x + 1.0).ceil().max(0.0) as u32).clamp(x0, x1);
        let ix1: u32 = ((rect.right() - 1.0).floor().max(0.0) as u32).clamp(ix0, x1);
        let iy0: u32 = ((rect_y + r_max + 1.0).ceil().max(0.0) as u32).clamp(y0, y1);
        let iy1: u32 = ((rect.bottom() - r_max - 1.0).floor().max(0.0) as u32).clamp(iy0, y1);
        let cx0: u32 = ((rect_x + r_max + 1.0).ceil().max(0.0) as u32).clamp(x0, x1);
        let cx1: u32 = ((rect.right() - r_max - 1.0).floor().max(0.0) as u32).clamp(cx0, x1);
        let mut shaded: usize = 0;
        for y in y0..iy0 {
            let Some(dst_row) = band.row_slice_mut(y) else {
                continue;
            };
            let v: f32 = (((y as f32) + 0.5 - rect_y) * inv_dst_h).clamp(0.0, 1.0);
            let fy: f32 = (v * src_h_f - 0.5).clamp(0.0, max_sy as f32);
            let sy0: usize = (fy.floor() as usize).min(max_sy);
            let sy1: usize = (sy0 + 1).min(max_sy);
            let wy: u32 = ((fy - (sy0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
            let y_params: (usize, usize, u32, u32) =
                (sy0 * src_w_usize, sy1 * src_w_usize, wy, 256 - wy);
            let py: f32 = (y as f32) + 0.5 - cy;
            let dist_y: f32 = py.abs() - half_h;
            let mid_cov: f32 = sdf_coverage(dist_y, aa);
            if cx0 < cx1 && mid_cov > 0.0 {
                let lut_s: usize = (cx0 - x0) as usize;
                let lut_e: usize = (cx1 - x0) as usize;
                if mid_cov >= 0.9995 {
                    Self::blit_bilinear_span_precomputed(
                        y_params,
                        cx0 as usize,
                        &x_lut_ref[lut_s..lut_e],
                        src_bytes,
                        dst_row,
                    );
                } else {
                    for x in cx0..cx1 {
                        let rgba: [u8; 4] = sample_bilinear_fixed(
                            src_bytes,
                            y_params,
                            x_lut_ref[(x - x0) as usize],
                        );
                        let off: usize = (x as usize) * 4;
                        blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, mid_cov);
                    }
                }
                shaded += (cx1 - cx0) as usize;
            }
            for x in x0..cx0 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cr: f32 = quadrant_corner_radius(radius, px, py);
                let cov: f32 = sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, cr), aa);
                if cov > 0.0 {
                    let rgba: [u8; 4] =
                        sample_bilinear_fixed(src_bytes, y_params, x_lut_ref[(x - x0) as usize]);
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, cov);
                    shaded += 1;
                }
            }
            for x in cx1..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cr: f32 = quadrant_corner_radius(radius, px, py);
                let cov: f32 = sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, cr), aa);
                if cov > 0.0 {
                    let rgba: [u8; 4] =
                        sample_bilinear_fixed(src_bytes, y_params, x_lut_ref[(x - x0) as usize]);
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, cov);
                    shaded += 1;
                }
            }
        }
        for y in iy0..iy1 {
            let Some(dst_row) = band.row_slice_mut(y) else {
                continue;
            };
            let v: f32 = (((y as f32) + 0.5 - rect_y) * inv_dst_h).clamp(0.0, 1.0);
            let fy: f32 = (v * src_h_f - 0.5).clamp(0.0, max_sy as f32);
            let sy0: usize = (fy.floor() as usize).min(max_sy);
            let sy1: usize = (sy0 + 1).min(max_sy);
            let wy: u32 = ((fy - (sy0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
            let y_params: (usize, usize, u32, u32) =
                (sy0 * src_w_usize, sy1 * src_w_usize, wy, 256 - wy);
            if ix0 < ix1 {
                let lut_s: usize = (ix0 - x0) as usize;
                let lut_e: usize = (ix1 - x0) as usize;
                Self::blit_bilinear_span_precomputed(
                    y_params,
                    ix0 as usize,
                    &x_lut_ref[lut_s..lut_e],
                    src_bytes,
                    dst_row,
                );
                shaded += (ix1 - ix0) as usize;
            }
            for x in x0..ix0 {
                let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                let cov: f32 = sdf_coverage(dist, aa);
                if cov > 0.0 {
                    let rgba: [u8; 4] =
                        sample_bilinear_fixed(src_bytes, y_params, x_lut_ref[(x - x0) as usize]);
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, cov);
                    shaded += 1;
                }
            }
            for x in ix1..x1 {
                let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                let cov: f32 = sdf_coverage(dist, aa);
                if cov > 0.0 {
                    let rgba: [u8; 4] =
                        sample_bilinear_fixed(src_bytes, y_params, x_lut_ref[(x - x0) as usize]);
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, cov);
                    shaded += 1;
                }
            }
        }
        for y in iy1..y1 {
            let Some(dst_row) = band.row_slice_mut(y) else {
                continue;
            };
            let v: f32 = (((y as f32) + 0.5 - rect_y) * inv_dst_h).clamp(0.0, 1.0);
            let fy: f32 = (v * src_h_f - 0.5).clamp(0.0, max_sy as f32);
            let sy0: usize = (fy.floor() as usize).min(max_sy);
            let sy1: usize = (sy0 + 1).min(max_sy);
            let wy: u32 = ((fy - (sy0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
            let y_params: (usize, usize, u32, u32) =
                (sy0 * src_w_usize, sy1 * src_w_usize, wy, 256 - wy);
            let py: f32 = (y as f32) + 0.5 - cy;
            let dist_y: f32 = py.abs() - half_h;
            let mid_cov: f32 = sdf_coverage(dist_y, aa);
            if cx0 < cx1 && mid_cov > 0.0 {
                let lut_s: usize = (cx0 - x0) as usize;
                let lut_e: usize = (cx1 - x0) as usize;
                if mid_cov >= 0.9995 {
                    Self::blit_bilinear_span_precomputed(
                        y_params,
                        cx0 as usize,
                        &x_lut_ref[lut_s..lut_e],
                        src_bytes,
                        dst_row,
                    );
                } else {
                    for x in cx0..cx1 {
                        let rgba: [u8; 4] = sample_bilinear_fixed(
                            src_bytes,
                            y_params,
                            x_lut_ref[(x - x0) as usize],
                        );
                        let off: usize = (x as usize) * 4;
                        blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, mid_cov);
                    }
                }
                shaded += (cx1 - cx0) as usize;
            }
            for x in x0..cx0 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cr: f32 = quadrant_corner_radius(radius, px, py);
                let cov: f32 = sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, cr), aa);
                if cov > 0.0 {
                    let rgba: [u8; 4] =
                        sample_bilinear_fixed(src_bytes, y_params, x_lut_ref[(x - x0) as usize]);
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, cov);
                    shaded += 1;
                }
            }
            for x in cx1..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cr: f32 = quadrant_corner_radius(radius, px, py);
                let cov: f32 = sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, cr), aa);
                if cov > 0.0 {
                    let rgba: [u8; 4] =
                        sample_bilinear_fixed(src_bytes, y_params, x_lut_ref[(x - x0) as usize]);
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(&mut dst_row[off..off + 4], rgba, cov);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Blits a decoded `ImageBuffer` into `band` with 1:1 scanline fast paths, reusable `x_lut` bilinear filtering, and corner-only SDF clipping.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Physical destination bounds.
    /// - `&ImageBuffer` - Source RGBA8 image.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_image(
        band: &mut PixelBand<'_>,
        rect: Rect,
        buffer: &ImageBuffer,
        radius: CornerRadius,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        if buffer.get_width() == 0 || buffer.get_height() == 0 {
            return 0;
        }
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let src_w: u32 = buffer.get_width();
        let src_h: u32 = buffer.get_height();
        let src_bytes: &[u8] = buffer.get_pixels().as_slice();
        let inv_dst_w: f32 = 1.0 / rect.get_width().max(1.0);
        let inv_dst_h: f32 = 1.0 / rect.get_height().max(1.0);
        let use_bilinear: bool = aa != AntialiasingMode::None
            && ((rect.get_width() - (src_w as f32)).abs() > 0.5
                || (rect.get_height() - (src_h as f32)).abs() > 0.5);
        let has_radius: bool = radius.max_radius() > 0.0;
        let max_sx: usize = (src_w as usize).saturating_sub(1);
        let max_sy: usize = (src_h as usize).saturating_sub(1);
        let src_w_usize: usize = src_w as usize;
        let is_exact_1to1: bool = !use_bilinear
            && (rect.get_width() - (src_w as f32)).abs() <= 0.01
            && (rect.get_height() - (src_h as f32)).abs() <= 0.01
            && (rect.get_x() - rect.get_x().round()).abs() <= 0.01
            && (rect.get_y() - rect.get_y().round()).abs() <= 0.01;
        let rx0_i: i32 = rect.get_x().round() as i32;
        let ry0_i: i32 = rect.get_y().round() as i32;
        if is_exact_1to1
            && !has_radius
            && (x0 as i32) >= rx0_i
            && (x1 as i32) <= rx0_i + (src_w as i32)
            && (y0 as i32) >= ry0_i
            && (y1 as i32) <= ry0_i + (src_h as i32)
        {
            let span_w: usize = (x1 - x0) as usize;
            let src_x_off: usize = ((x0 as i32) - rx0_i) as usize;
            let dst_byte0: usize = (x0 as usize) * 4;
            let dst_byte1: usize = (x1 as usize) * 4;
            for y in y0..y1 {
                let Some(dst_row) = band.row_slice_mut(y) else {
                    continue;
                };
                let sy: usize = ((y as i32) - ry0_i) as usize;
                let src_byte0: usize = (sy * src_w_usize + src_x_off) * 4;
                let src_byte1: usize = src_byte0 + span_w * 4;
                if src_byte1 <= src_bytes.len() && dst_byte1 <= dst_row.len() {
                    blit_1to1_rgba_span(
                        &src_bytes[src_byte0..src_byte1],
                        &mut dst_row[dst_byte0..dst_byte1],
                    );
                }
            }
            return span_w * ((y1 - y0) as usize);
        }
        if use_bilinear {
            let mut shaded: usize = 0;
            let ran_tls: bool = BAND_IMAGE_X_LUT
                .try_with(|cell: &RefCell<Vec<(usize, usize, u32)>>| {
                    if let Ok(mut guard) = cell.try_borrow_mut() {
                        shaded = Self::raster_image_bilinear_with_lut(
                            band,
                            rect,
                            (src_w, src_h, src_bytes),
                            (x0, y0, x1, y1),
                            radius,
                            aa,
                            &mut guard,
                        );
                    }
                })
                .is_ok();
            if !ran_tls {
                let mut fallback_lut: Vec<(usize, usize, u32)> =
                    Vec::with_capacity((x1 - x0) as usize);
                shaded = Self::raster_image_bilinear_with_lut(
                    band,
                    rect,
                    (src_w, src_h, src_bytes),
                    (x0, y0, x1, y1),
                    radius,
                    aa,
                    &mut fallback_lut,
                );
            }
            return shaded;
        }
        let mut shaded: usize = 0;
        let r_max: f32 = if has_radius {
            radius.max_radius().min(half_w).min(half_h).max(0.0)
        } else {
            0.0
        };
        let ix0: u32 = ((rect.get_x() + 1.0).ceil().max(0.0) as u32).clamp(x0, x1);
        let ix1: u32 = ((rect.right() - 1.0).floor().max(0.0) as u32).clamp(ix0, x1);
        let iy0: u32 = ((rect.get_y() + r_max + 1.0).ceil().max(0.0) as u32).clamp(y0, y1);
        let iy1: u32 = ((rect.bottom() - r_max - 1.0).floor().max(0.0) as u32).clamp(iy0, y1);
        let can_1to1_interior: bool = is_exact_1to1
            && (ix0 as i32) >= rx0_i
            && (ix1 as i32) <= rx0_i + (src_w as i32)
            && (iy0 as i32) >= ry0_i
            && (iy1 as i32) <= ry0_i + (src_h as i32);
        for y in y0..y1 {
            let Some(dst_row) = band.row_slice_mut(y) else {
                continue;
            };
            let v: f32 = (((y as f32) + 0.5 - rect.get_y()) * inv_dst_h).clamp(0.0, 1.0);
            let py: f32 = (y as f32) + 0.5 - cy;
            let sy: usize = ((v * (src_h as f32)).floor() as usize).min(max_sy);
            let row: usize = sy * src_w_usize;
            if has_radius && can_1to1_interior && y >= iy0 && y < iy1 && ix0 < ix1 {
                let src_off: usize = (row + (((ix0 as i32) - rx0_i) as usize)) * 4;
                let span_bytes: usize = ((ix1 - ix0) as usize) * 4;
                let dst_off: usize = (ix0 as usize) * 4;
                if src_off + span_bytes <= src_bytes.len() && dst_off + span_bytes <= dst_row.len()
                {
                    blit_1to1_rgba_span(
                        &src_bytes[src_off..src_off + span_bytes],
                        &mut dst_row[dst_off..dst_off + span_bytes],
                    );
                }
                shaded += (ix1 - ix0) as usize;
                for x in x0..ix0 {
                    let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                    let cov: f32 = sdf_coverage(dist, aa);
                    if cov > 0.0 {
                        let u: f32 =
                            (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                        let sx: usize = ((u * (src_w as f32)).floor() as usize).min(max_sx);
                        let idx: usize = (row + sx) * 4;
                        if idx + 4 <= src_bytes.len() {
                            let off: usize = (x as usize) * 4;
                            blend_rgba_pixel_with_cov(
                                &mut dst_row[off..off + 4],
                                [
                                    src_bytes[idx],
                                    src_bytes[idx + 1],
                                    src_bytes[idx + 2],
                                    src_bytes[idx + 3],
                                ],
                                cov,
                            );
                            shaded += 1;
                        }
                    }
                }
                for x in ix1..x1 {
                    let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                    let cov: f32 = sdf_coverage(dist, aa);
                    if cov > 0.0 {
                        let u: f32 =
                            (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                        let sx: usize = ((u * (src_w as f32)).floor() as usize).min(max_sx);
                        let idx: usize = (row + sx) * 4;
                        if idx + 4 <= src_bytes.len() {
                            let off: usize = (x as usize) * 4;
                            blend_rgba_pixel_with_cov(
                                &mut dst_row[off..off + 4],
                                [
                                    src_bytes[idx],
                                    src_bytes[idx + 1],
                                    src_bytes[idx + 2],
                                    src_bytes[idx + 3],
                                ],
                                cov,
                            );
                            shaded += 1;
                        }
                    }
                }
                continue;
            }
            for x in x0..x1 {
                let cov: f32 = if has_radius {
                    let px: f32 = (x as f32) + 0.5 - cx;
                    let cr: f32 = quadrant_corner_radius(radius, px, py);
                    sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, cr), aa)
                } else {
                    1.0
                };
                if cov <= 0.0 {
                    continue;
                }
                let u: f32 = (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                let sx: usize = ((u * (src_w as f32)).floor() as usize).min(max_sx);
                let idx: usize = (row + sx) * 4;
                if idx + 4 <= src_bytes.len() {
                    let off: usize = (x as usize) * 4;
                    blend_rgba_pixel_with_cov(
                        &mut dst_row[off..off + 4],
                        [
                            src_bytes[idx],
                            src_bytes[idx + 1],
                            src_bytes[idx + 2],
                            src_bytes[idx + 3],
                        ],
                        cov,
                    );
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes an anti-aliased line segment between `from` and `to`.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Point` - Physical start point.
    /// - `Point` - Physical end point.
    /// - `Color` - Stroke color.
    /// - `f32` - Physical stroke width.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_line(
        band: &mut PixelBand<'_>,
        from: Point,
        to: Point,
        color: Color,
        width: f32,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        let half_w: f32 = width * 0.5;
        let min_x: f32 = from.get_x().min(to.get_x()) - half_w - 1.0;
        let min_y: f32 = from.get_y().min(to.get_y()) - half_w - 1.0;
        let max_x: f32 = from.get_x().max(to.get_x()) + half_w + 1.0;
        let max_y: f32 = from.get_y().max(to.get_y()) + half_w + 1.0;
        let Some(vis) = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y).intersect(clip)
        else {
            return 0;
        };
        let dx: f32 = to.get_x() - from.get_x();
        let dy: f32 = to.get_y() - from.get_y();
        let len_sq: f32 = (dx * dx + dy * dy).max(1e-6);
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5;
            for x in x0..x1 {
                let px: f32 = (x as f32) + 0.5;
                let t: f32 = (((px - from.get_x()) * dx + (py - from.get_y()) * dy) / len_sq)
                    .clamp(0.0, 1.0);
                let proj_x: f32 = from.get_x() + t * dx;
                let proj_y: f32 = from.get_y() + t * dy;
                let dist: f32 =
                    ((px - proj_x) * (px - proj_x) + (py - proj_y) * (py - proj_y)).sqrt() - half_w;
                let cov: f32 = sdf_coverage(dist, aa);
                if cov > 0.0 {
                    let alpha_f: f32 = ((color.get_a() as f32) * (1.0 / 255.0)) * cov;
                    band.blend_pixel_f32_dithered(x, y, color, alpha_f, 0.0);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes an analytical SDF anti-aliased filled circle with 4x4 RGSS perimeter sampling.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Point` - Physical center point.
    /// - `f32` - Physical radius.
    /// - `Color` - Fill color.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_circle(
        band: &mut PixelBand<'_>,
        center: Point,
        radius: f32,
        color: Color,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        if radius <= 0.0 || color.get_a() == 0 {
            return 0;
        }
        let bounds: Rect = Rect::new(
            center.get_x() - radius - 1.0,
            center.get_y() - radius - 1.0,
            (radius + 1.0) * 2.0,
            (radius + 1.0) * 2.0,
        );
        let Some(vis) = bounds.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let outer_r: f32 = radius + 1.0;
        let inner_r: f32 = (radius - 1.0).max(0.0);
        let outer_r2: f32 = outer_r * outer_r;
        let inner_r2: f32 = inner_r * inner_r;
        let cx: f32 = center.get_x();
        let cy: f32 = center.get_y();
        let mut shaded: usize = 0;
        let base_alpha: f32 = (color.get_a() as f32) * (1.0 / 255.0);
        for y in y0..y1 {
            let dy: f32 = (y as f32) + 0.5 - cy;
            let dy2: f32 = dy * dy;
            if dy2 >= outer_r2 {
                continue;
            }
            let out_dx: f32 = (outer_r2 - dy2).sqrt();
            let row_x0: u32 = ((cx - out_dx).floor().max(0.0) as u32).clamp(x0, x1);
            let row_x1: u32 = ((cx + out_dx).ceil().max(0.0) as u32).clamp(x0, x1);
            if dy2 < inner_r2 && inner_r > 1.0 {
                let in_dx: f32 = (inner_r2 - dy2).sqrt();
                let in_x0: u32 = ((cx - in_dx).ceil().max(0.0) as u32).clamp(row_x0, row_x1);
                let in_x1: u32 = ((cx + in_dx).floor().max(0.0) as u32).clamp(row_x0, row_x1);
                for x in row_x0..in_x0 {
                    let dx: f32 = (x as f32) + 0.5 - cx;
                    let cov: f32 = if aa == AntialiasingMode::None {
                        sdf_coverage((dx * dx + dy2).sqrt() - radius, aa)
                    } else {
                        circle_supersampled_coverage(dx, dy, radius)
                    };
                    if cov > 0.0 {
                        band.blend_pixel_f32_dithered(x, y, color, base_alpha * cov, 0.0);
                        shaded += 1;
                    }
                }
                if in_x0 < in_x1 {
                    band.fill_span_linear(in_x0, in_x1, y, color);
                    shaded += (in_x1 - in_x0) as usize;
                }
                for x in in_x1..row_x1 {
                    let dx: f32 = (x as f32) + 0.5 - cx;
                    let cov: f32 = if aa == AntialiasingMode::None {
                        sdf_coverage((dx * dx + dy2).sqrt() - radius, aa)
                    } else {
                        circle_supersampled_coverage(dx, dy, radius)
                    };
                    if cov > 0.0 {
                        band.blend_pixel_f32_dithered(x, y, color, base_alpha * cov, 0.0);
                        shaded += 1;
                    }
                }
            } else {
                for x in row_x0..row_x1 {
                    let dx: f32 = (x as f32) + 0.5 - cx;
                    let cov: f32 = if aa == AntialiasingMode::None {
                        sdf_coverage((dx * dx + dy2).sqrt() - radius, aa)
                    } else {
                        circle_supersampled_coverage(dx, dy, radius)
                    };
                    if cov > 0.0 {
                        band.blend_pixel_f32_dithered(x, y, color, base_alpha * cov, 0.0);
                        shaded += 1;
                    }
                }
            }
        }
        shaded
    }

    /// Rasterizes a solid filled triangle using fast signed-edge classification and edge-only 4x4 RGSS.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Point` - First physical vertex.
    /// - `Point` - Second physical vertex.
    /// - `Point` - Third physical vertex.
    /// - `Color` - Fill color.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_triangle(
        band: &mut PixelBand<'_>,
        p0: Point,
        p1: Point,
        p2: Point,
        color: Color,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        if color.get_a() == 0 {
            return 0;
        }
        let area: f32 = (p1.get_x() - p0.get_x()) * (p2.get_y() - p0.get_y())
            - (p1.get_y() - p0.get_y()) * (p2.get_x() - p0.get_x());
        if area.abs() < 1e-5 {
            return 0;
        }
        let min_x: f32 = p0.get_x().min(p1.get_x()).min(p2.get_x()) - 1.0;
        let min_y: f32 = p0.get_y().min(p1.get_y()).min(p2.get_y()) - 1.0;
        let max_x: f32 = p0.get_x().max(p1.get_x()).max(p2.get_x()) + 1.0;
        let max_y: f32 = p0.get_y().max(p1.get_y()).max(p2.get_y()) + 1.0;
        let Some(vis) = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y).intersect(clip)
        else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let clip_x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let clip_x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let edges: [(Point, Point); 3] = [(p0, p1), (p1, p2), (p2, p0)];
        let sign: f32 = if area > 0.0 { 1.0 } else { -1.0 };
        let inv_l0: f32 = sign
            / (p1.get_x() - p0.get_x())
                .hypot(p1.get_y() - p0.get_y())
                .max(1e-5);
        let inv_l1: f32 = sign
            / (p2.get_x() - p1.get_x())
                .hypot(p2.get_y() - p1.get_y())
                .max(1e-5);
        let inv_l2: f32 = sign
            / (p0.get_x() - p2.get_x())
                .hypot(p0.get_y() - p2.get_y())
                .max(1e-5);
        let base_alpha: f32 = (color.get_a() as f32) * (1.0 / 255.0);
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5;
            if aa == AntialiasingMode::None {
                let mut min_ix: f32 = f32::INFINITY;
                let mut max_ix: f32 = f32::NEG_INFINITY;
                for (a, b) in edges.iter() {
                    let ay: f32 = a.get_y();
                    let by: f32 = b.get_y();
                    if (ay <= py && by > py) || (by <= py && ay > py) {
                        let t: f32 = (py - ay) / (by - ay);
                        let ix: f32 = a.get_x() + t * (b.get_x() - a.get_x());
                        min_ix = min_ix.min(ix);
                        max_ix = max_ix.max(ix);
                    }
                }
                if min_ix <= max_ix {
                    let sx0: u32 = (min_ix.floor().max(0.0) as u32).clamp(clip_x0, clip_x1);
                    let sx1: u32 = (max_ix.ceil().max(0.0) as u32).clamp(clip_x0, clip_x1);
                    if sx0 < sx1 {
                        band.fill_span_linear(sx0, sx1, y, color);
                        shaded += (sx1 - sx0) as usize;
                    }
                }
                continue;
            }
            for x in clip_x0..clip_x1 {
                let px: f32 = (x as f32) + 0.5;
                let d0: f32 = ((p1.get_x() - p0.get_x()) * (py - p0.get_y())
                    - (p1.get_y() - p0.get_y()) * (px - p0.get_x()))
                    * inv_l0;
                if d0 < -0.85 {
                    continue;
                }
                let d1: f32 = ((p2.get_x() - p1.get_x()) * (py - p1.get_y())
                    - (p2.get_y() - p1.get_y()) * (px - p1.get_x()))
                    * inv_l1;
                if d1 < -0.85 {
                    continue;
                }
                let d2: f32 = ((p0.get_x() - p2.get_x()) * (py - p2.get_y())
                    - (p0.get_y() - p2.get_y()) * (px - p2.get_x()))
                    * inv_l2;
                if d2 < -0.85 {
                    continue;
                }
                if d0 > 0.85 && d1 > 0.85 && d2 > 0.85 {
                    band.blend_pixel_linear_dithered(x, y, color, 0.0);
                    shaded += 1;
                    continue;
                }
                let cov: f32 = triangle_supersampled_coverage(px, py, p0, p1, p2);
                if cov >= 0.999 {
                    band.blend_pixel_linear_dithered(x, y, color, 0.0);
                    shaded += 1;
                } else if cov > 0.0 {
                    band.blend_pixel_f32_dithered(x, y, color, base_alpha * cov, 0.0);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Blits a cropped sub-rectangle of an `ImageBuffer` sprite sheet into `band` with bilinear filtering and optional tint.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Physical destination bounds.
    /// - `&ImageBuffer` - Source sprite-sheet RGBA8 buffer.
    /// - `Rect` - Source pixel sub-rectangle.
    /// - `Color` - Multiplicative tint color.
    /// - `Rect` - Active clip bounds.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_sprite_region(
        band: &mut PixelBand<'_>,
        rect: Rect,
        buffer: &ImageBuffer,
        src_rect: Rect,
        tint: Color,
        clip: Rect,
        aa: AntialiasingMode,
    ) -> usize {
        if buffer.get_width() == 0
            || buffer.get_height() == 0
            || tint.get_a() == 0
            || src_rect.get_width() <= 0.0
            || src_rect.get_height() <= 0.0
        {
            return 0;
        }
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let buf_w: u32 = buffer.get_width();
        let buf_h: u32 = buffer.get_height();
        let src_bytes: &[u8] = buffer.get_pixels().as_slice();
        let inv_w: f32 = 1.0 / rect.get_width().max(1.0);
        let inv_h: f32 = 1.0 / rect.get_height().max(1.0);
        let untinted: bool = tint == COLOR_WHITE;
        let tr: u32 = tint.get_r() as u32;
        let tg: u32 = tint.get_g() as u32;
        let tb: u32 = tint.get_b() as u32;
        let ta: u32 = tint.get_a() as u32;
        let use_bilinear: bool = aa != AntialiasingMode::None
            && ((rect.get_width() - src_rect.get_width()).abs() > 0.5
                || (rect.get_height() - src_rect.get_height()).abs() > 0.5);
        let max_sx: u32 = ((src_rect.right().ceil() as u32).saturating_sub(1)).min(buf_w - 1);
        let max_sy: u32 = ((src_rect.bottom().ceil() as u32).saturating_sub(1)).min(buf_h - 1);
        let min_sx: u32 = (src_rect.get_x().floor().max(0.0) as u32).min(max_sx);
        let min_sy: u32 = (src_rect.get_y().floor().max(0.0) as u32).min(max_sy);
        let buf_w_usize: usize = buf_w as usize;
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let v: f32 = (((y as f32) + 0.5 - rect.get_y()) * inv_h).clamp(0.0, 1.0);
            let fy: f32 = (src_rect.get_y() + v * src_rect.get_height() - 0.5)
                .clamp(min_sy as f32, max_sy as f32);
            let sy0: u32 = (fy.floor() as u32).clamp(min_sy, max_sy);
            let sy1: u32 = (sy0 + 1).min(max_sy);
            let wy: f32 = if use_bilinear { fy - (sy0 as f32) } else { 0.0 };
            let iwy: f32 = 1.0 - wy;
            let row0: usize = (sy0 as usize) * buf_w_usize;
            let row1: usize = (sy1 as usize) * buf_w_usize;
            for x in x0..x1 {
                let u: f32 = (((x as f32) + 0.5 - rect.get_x()) * inv_w).clamp(0.0, 1.0);
                let fx: f32 = (src_rect.get_x() + u * src_rect.get_width() - 0.5)
                    .clamp(min_sx as f32, max_sx as f32);
                let sx0: u32 = (fx.floor() as u32).clamp(min_sx, max_sx);
                let sx1: u32 = (sx0 + 1).min(max_sx);
                let (sr_u8, sg_u8, sb_u8, sa_u8): (u8, u8, u8, u8) = if use_bilinear {
                    let wx: f32 = fx - (sx0 as f32);
                    let iwx: f32 = 1.0 - wx;
                    let i00: usize = (row0 + (sx0 as usize)) * 4;
                    let i10: usize = (row0 + (sx1 as usize)) * 4;
                    let i01: usize = (row1 + (sx0 as usize)) * 4;
                    let i11: usize = (row1 + (sx1 as usize)) * 4;
                    if i11 + 4 > src_bytes.len() {
                        continue;
                    }
                    let w00: f32 = iwx * iwy;
                    let w10: f32 = wx * iwy;
                    let w01: f32 = iwx * wy;
                    let w11: f32 = wx * wy;
                    (
                        ((src_bytes[i00] as f32) * w00
                            + (src_bytes[i10] as f32) * w10
                            + (src_bytes[i01] as f32) * w01
                            + (src_bytes[i11] as f32) * w11)
                            .round() as u8,
                        ((src_bytes[i00 + 1] as f32) * w00
                            + (src_bytes[i10 + 1] as f32) * w10
                            + (src_bytes[i01 + 1] as f32) * w01
                            + (src_bytes[i11 + 1] as f32) * w11)
                            .round() as u8,
                        ((src_bytes[i00 + 2] as f32) * w00
                            + (src_bytes[i10 + 2] as f32) * w10
                            + (src_bytes[i01 + 2] as f32) * w01
                            + (src_bytes[i11 + 2] as f32) * w11)
                            .round() as u8,
                        ((src_bytes[i00 + 3] as f32) * w00
                            + (src_bytes[i10 + 3] as f32) * w10
                            + (src_bytes[i01 + 3] as f32) * w01
                            + (src_bytes[i11 + 3] as f32) * w11)
                            .round() as u8,
                    )
                } else {
                    let idx: usize = (row0 + (sx0 as usize)) * 4;
                    if idx + 4 > src_bytes.len() {
                        continue;
                    }
                    (
                        src_bytes[idx],
                        src_bytes[idx + 1],
                        src_bytes[idx + 2],
                        src_bytes[idx + 3],
                    )
                };
                if sa_u8 == 0 {
                    continue;
                }
                let sample: Color = if untinted {
                    Color::new(sr_u8, sg_u8, sb_u8, sa_u8)
                } else {
                    let rr: u32 = (sr_u8 as u32) * tr + 128;
                    let gg: u32 = (sg_u8 as u32) * tg + 128;
                    let bb: u32 = (sb_u8 as u32) * tb + 128;
                    let aa_v: u32 = (sa_u8 as u32) * ta + 128;
                    Color::new(
                        ((rr + (rr >> 8)) >> 8) as u8,
                        ((gg + (gg >> 8)) >> 8) as u8,
                        ((bb + (bb >> 8)) >> 8) as u8,
                        ((aa_v + (aa_v >> 8)) >> 8) as u8,
                    )
                };
                band.blend_pixel(x, y, sample);
                shaded += 1;
            }
        }
        shaded
    }
}
