use super::*;

/// Persistent work-stealing multi-core band pool methods for `BandWorkerPool`.
#[cfg(not(target_arch = "wasm32"))]
/// Inherent implementation of [`BandWorkerPool`].
impl BandWorkerPool {
    /// Creates a persistent worker pool sized to the host CPU count (clamped to `1..=8` total threads including the caller).
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized worker pool with warm background threads.
    #[must_use]
    pub(crate) fn new() -> Self {
        let total: usize = available_parallelism()
            .map(usize::from)
            .unwrap_or(4)
            .clamp(1, 8);
        let worker_count: usize = total.saturating_sub(1);
        let shared: Arc<(Mutex<BandPoolState>, Condvar, Condvar)> = Arc::new((
            Mutex::new(BandPoolState::default()),
            Condvar::new(),
            Condvar::new(),
        ));
        for idx in 0..worker_count {
            let shared_clone: Arc<(Mutex<BandPoolState>, Condvar, Condvar)> = Arc::clone(&shared);
            let name: String = format!("{BAND_WORKER_THREAD_PREFIX}-{idx}");
            let _: io::Result<JoinHandle<()>> = Builder::new().name(name).spawn(move || {
                let mut seen_epoch: u64 = 0;
                loop {
                    let job_ptr: usize = {
                        let (lock, wake_workers, _wake_caller): &(
                            Mutex<BandPoolState>,
                            Condvar,
                            Condvar,
                        ) = &shared_clone;
                        let Ok(mut state) = lock.lock() else {
                            break;
                        };
                        while state.get_epoch() == seen_epoch || state.get_active_job() == 0 {
                            let Ok(next_guard) = wake_workers.wait(state) else {
                                return;
                            };
                            state = next_guard;
                        }
                        seen_epoch = state.get_epoch();
                        state.get_active_job()
                    };
                    let job: &BandJob = unsafe { &*(job_ptr as *const BandJob) };
                    run_band_job_loop(job);
                    let finished: usize = job.workers_done.fetch_add(1, Ordering::AcqRel) + 1;
                    if finished >= worker_count {
                        let (lock, _wake_workers, wake_caller): &(
                            Mutex<BandPoolState>,
                            Condvar,
                            Condvar,
                        ) = &shared_clone;
                        let _guard: Result<
                            MutexGuard<'_, BandPoolState>,
                            PoisonError<MutexGuard<'_, BandPoolState>>,
                        > = lock.lock();
                        wake_caller.notify_one();
                    }
                }
            });
        }
        Self {
            worker_count,
            dispatch_lock: Mutex::new(()),
            shared,
        }
    }

    /// Executes bands serially on the calling thread with optional cached backdrop restore.
    ///
    /// # Arguments
    ///
    /// - `&mut [PixelBand<'_>]` - Disjoint horizontal framebuffer strips.
    /// - `&[DrawCommand]` - Display list commands.
    /// - `&[PreppedCommand]` - Pre-clipped command table.
    /// - `&[TextLayoutResult]` - Pre-measured text layouts.
    /// - `&GlyphAtlas` - Shared read-only glyph atlas.
    /// - `(f32, AntialiasingMode)` - Tuple of `(scale, antialiasing)`.
    /// - `Option<&[u8]>` - Optional cached base-scene RGBA8 pixel slice.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count across all strips.
    fn execute_serial(
        bands: &mut [PixelBand<'_>],
        commands: &[DrawCommand],
        prepped: &[PreppedCommand],
        layouts: &[TextLayoutResult],
        atlas: &GlyphAtlas,
        render_opts: (f32, AntialiasingMode),
        base_restore: Option<&[u8]>,
    ) -> usize {
        let (scale, aa): (f32, AntialiasingMode) = render_opts;
        let mut total: usize = 0;
        for band in bands.iter_mut() {
            if let Some(src_buf) = base_restore {
                let byte_off: usize = (band.y_offset as usize) * (band.width as usize) * 4;
                let byte_end: usize = byte_off + band.pixels.len();
                if let Some(src_slice) = src_buf.get(byte_off..byte_end) {
                    band.pixels.copy_from_slice(src_slice);
                }
            }
            total +=
                SoftwareRenderer::render_band(band, commands, prepped, layouts, atlas, scale, aa);
        }
        total
    }

    /// Returns total participating rasterizer threads (background workers plus the calling main thread).
    ///
    /// # Returns
    ///
    /// - `usize` - Total active thread count in `1..=6`.
    #[must_use]
    pub(crate) fn total_threads(&self) -> usize {
        let Self { worker_count, .. } = self;
        worker_count.saturating_add(1)
    }

    /// Dispatches horizontal pixel bands across the persistent worker pool with optional parallel backdrop restore and synchronous completion.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Worker pool reference.
    /// - `&mut [PixelBand<'_>]` - Disjoint horizontal framebuffer strips.
    /// - `&[DrawCommand]` - Display list commands.
    /// - `&[PreppedCommand]` - Pre-clipped command table.
    /// - `&[TextLayoutResult]` - Pre-measured text layouts.
    /// - `&GlyphAtlas` - Shared read-only glyph atlas.
    /// - `(f32, AntialiasingMode, Option<&[u8]>)` - Tuple of `(scale, antialiasing, optional_base_restore)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixel count across all strips.
    pub(crate) fn execute(
        &self,
        bands: &mut [PixelBand<'_>],
        commands: &[DrawCommand],
        prepped: &[PreppedCommand],
        layouts: &[TextLayoutResult],
        atlas: &GlyphAtlas,
        render_opts: (f32, AntialiasingMode, Option<&[u8]>),
    ) -> usize {
        let Self {
            worker_count,
            dispatch_lock,
            shared,
        } = self;
        let (scale, aa, base_restore): (f32, AntialiasingMode, Option<&[u8]>) = render_opts;
        if *worker_count == 0 || bands.len() <= 1 {
            return Self::execute_serial(
                bands,
                commands,
                prepped,
                layouts,
                atlas,
                (scale, aa),
                base_restore,
            );
        }
        let Ok(_dispatch_guard) = dispatch_lock.try_lock() else {
            return Self::execute_serial(
                bands,
                commands,
                prepped,
                layouts,
                atlas,
                (scale, aa),
                base_restore,
            );
        };
        let mut tasks: Vec<BandTaskSlice> = Vec::with_capacity(bands.len());
        for band in bands.iter_mut() {
            let base_ptr: usize = if let Some(src_buf) = base_restore {
                let byte_off: usize = (band.y_offset as usize) * (band.width as usize) * 4;
                let byte_end: usize = byte_off + band.pixels.len();
                if byte_end <= src_buf.len() {
                    unsafe { src_buf.as_ptr().add(byte_off) as usize }
                } else {
                    0
                }
            } else {
                0
            };
            tasks.push(BandTaskSlice {
                pixels_ptr: band.pixels.as_mut_ptr() as usize,
                pixels_len: band.pixels.len(),
                width: band.width,
                y_start: band.y_offset,
                height: band.height,
                base_src_ptr: base_ptr,
            });
        }
        let job: BandJob = BandJob {
            tasks_ptr: tasks.as_ptr() as usize,
            tasks_len: tasks.len(),
            commands_ptr: commands.as_ptr() as usize,
            commands_len: commands.len(),
            prepped_ptr: prepped.as_ptr() as usize,
            prepped_len: prepped.len(),
            layouts_ptr: layouts.as_ptr() as usize,
            layouts_len: layouts.len(),
            atlas_ptr: (atlas as *const GlyphAtlas) as usize,
            scale,
            aa,
            next_idx: AtomicUsize::new(0),
            workers_done: AtomicUsize::new(0),
            total_shaded: AtomicUsize::new(0),
        };
        let (lock, wake_workers, wake_caller): &(Mutex<BandPoolState>, Condvar, Condvar) = shared;
        {
            if let Ok(mut state) = lock.lock() {
                let next_epoch: u64 = state.get_epoch().wrapping_add(1);
                state.set_active_job((&job as *const BandJob) as usize);
                state.set_epoch(next_epoch);
                wake_workers.notify_all();
            } else {
                run_band_job_loop(&job);
                return job.total_shaded.load(Ordering::Acquire);
            }
        }
        run_band_job_loop(&job);
        let mut wait_guard: Option<MutexGuard<'_, BandPoolState>> = lock.lock().ok();
        while let Some(state) = wait_guard.take() {
            if job.workers_done.load(Ordering::Acquire) >= *worker_count {
                let mut done_state: MutexGuard<'_, BandPoolState> = state;
                done_state.set_active_job(0);
                break;
            }
            wait_guard = wake_caller.wait(state).ok();
        }
        job.total_shaded.load(Ordering::Acquire)
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
            text_layout_cache: HashMap::with_capacity(128),
            clip_stack: Vec::with_capacity(8),
            antialiasing: config.get_antialiasing(),
            tile_size: config.get_tile_size().max(16),
            base_cache_width: 0,
            base_cache_height: 0,
            base_cache_hash: 0,
            base_cache_pixels: Vec::new(),
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
                let (prepped, text_layouts): (Vec<PreppedCommand>, Vec<TextLayoutResult>) = self
                    .prepare_frame_range(
                        display_list,
                        split,
                        commands.len(),
                        root_clip,
                        scale,
                        phys_h,
                    );
                let pixels_shaded: usize = self.execute_prepared_bands(
                    commands,
                    prepped.as_slice(),
                    text_layouts.as_slice(),
                    root_clip,
                    (phys_w, phys_h, scale),
                    true,
                );
                return (tiles_updated, pixels_shaded.max(1));
            }
            let (base_prepped, base_layouts): (Vec<PreppedCommand>, Vec<TextLayoutResult>) =
                self.prepare_frame_range(display_list, 0, split, root_clip, scale, phys_h);
            let base_shaded: usize = self.execute_prepared_bands(
                commands,
                base_prepped.as_slice(),
                base_layouts.as_slice(),
                root_clip,
                (phys_w, phys_h, scale),
                false,
            );
            {
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
            let (portal_prepped, portal_layouts): (Vec<PreppedCommand>, Vec<TextLayoutResult>) =
                self.prepare_frame_range(
                    display_list,
                    split,
                    commands.len(),
                    root_clip,
                    scale,
                    phys_h,
                );
            let portal_shaded: usize = self.execute_prepared_bands(
                commands,
                portal_prepped.as_slice(),
                portal_layouts.as_slice(),
                root_clip,
                (phys_w, phys_h, scale),
                false,
            );
            return (tiles_updated, base_shaded.saturating_add(portal_shaded));
        }
        if !has_portals {
            self.set_base_cache_hash(0);
        }
        let (prepped, text_layouts): (Vec<PreppedCommand>, Vec<TextLayoutResult>) =
            self.prepare_frame_range(display_list, 0, commands.len(), root_clip, scale, phys_h);
        let pixels_shaded: usize = self.execute_prepared_bands(
            commands,
            prepped.as_slice(),
            text_layouts.as_slice(),
            root_clip,
            (phys_w, phys_h, scale),
            false,
        );
        (tiles_updated, pixels_shaded)
    }

    /// Executes prepped commands across horizontal bands with optional parallel restore from `base_cache_pixels`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `&[DrawCommand]` - Full frame draw command slice.
    /// - `&[PreppedCommand]` - Pre-clipped command slice.
    /// - `&[TextLayoutResult]` - Pre-measured text layouts.
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
        prepped_slice: &[PreppedCommand],
        layouts_slice: &[TextLayoutResult],
        root_clip: Rect,
        dims: (u32, u32, f32),
        restore_base_cache: bool,
    ) -> usize {
        let (phys_w, phys_h, scale): (u32, u32, f32) = dims;
        let aa: AntialiasingMode = self.get_antialiasing();
        let clip_y0: u32 = (root_clip.get_y().floor().max(0.0) as u32).min(phys_h);
        let clip_y1: u32 = (root_clip.bottom().ceil().max(0.0) as u32).min(phys_h);
        let clip_w: u32 = (root_clip.get_width().ceil().max(0.0) as u32).min(phys_w);
        let clip_h: u32 = clip_y1.saturating_sub(clip_y0);
        let Self {
            framebuffer,
            glyph_atlas,
            base_cache_pixels,
            ..
        } = self;
        let atlas_ref: &GlyphAtlas = glyph_atlas;
        let base_restore: Option<&[u8]> = if restore_base_cache {
            Some(base_cache_pixels.as_slice())
        } else {
            None
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            if (clip_w as usize).saturating_mul(clip_h as usize) >= 65_536 && clip_h >= 64 {
                let pool: &BandWorkerPool = global_band_worker_pool();
                let strip_count: usize = (pool.total_threads().saturating_mul(2))
                    .min((clip_h / 24) as usize)
                    .max(1);
                if strip_count > 1 && pool.total_threads() > 1 {
                    let mut bands: Vec<PixelBand<'_>> =
                        framebuffer.split_bands_mut(clip_y0, clip_y1, strip_count);
                    return pool.execute(
                        bands.as_mut_slice(),
                        commands,
                        prepped_slice,
                        layouts_slice,
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
                layouts_slice,
                atlas_ref,
                scale,
                aa,
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
                layouts_slice,
                atlas_ref,
                scale,
                aa,
            )
        }
    }

    /// Runs a lightweight main-thread pre-pass over `commands[start_idx..end_idx]` to resolve clips, opaque shadow cutouts, cached text layouts, and warm `GlyphAtlas`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `&DisplayList` - Incoming command stream.
    /// - `usize` - Inclusive start index into `display_list.get_commands()`.
    /// - `usize` - Exclusive end index into `display_list.get_commands()`.
    /// - `Rect` - Root physical damage clip rectangle.
    /// - `f32` - Active DPI scale factor.
    /// - `u32` - Framebuffer physical height.
    ///
    /// # Returns
    ///
    /// - `(Vec<PreppedCommand>, Vec<TextLayoutResult>)` - Band-ready commands and pre-measured text layouts.
    fn prepare_frame_range(
        &mut self,
        display_list: &DisplayList,
        start_idx: usize,
        end_idx: usize,
        root_clip: Rect,
        scale: f32,
        phys_h: u32,
    ) -> (Vec<PreppedCommand>, Vec<TextLayoutResult>) {
        let commands: &[DrawCommand] = display_list.get_commands().as_slice();
        let clamped_end: usize = end_idx.min(commands.len());
        let clamped_start: usize = start_idx.min(clamped_end);
        let mut prepped: Vec<PreppedCommand> = Vec::with_capacity(clamped_end - clamped_start);
        let mut text_layouts: Vec<TextLayoutResult> = Vec::new();
        let Self {
            glyph_atlas,
            font_provider,
            measurer,
            text_layout_cache,
            clip_stack,
            ..
        } = self;
        clip_stack.clear();
        clip_stack.push(root_clip);
        if text_layout_cache.len() > TEXT_LAYOUT_CACHE_LIMIT {
            text_layout_cache.clear();
        }
        for (offset, cmd) in commands[clamped_start..clamped_end].iter().enumerate() {
            let cmd_index: usize = clamped_start + offset;
            let active_clip: Rect = clip_stack.last().copied().unwrap_or(root_clip);
            match cmd {
                DrawCommand::PushClip(rect) => {
                    let phys: Rect = snap_physical_rect(*rect, scale);
                    let next: Rect = active_clip.intersect(phys).unwrap_or_default();
                    clip_stack.push(next);
                }
                DrawCommand::PopClip => {
                    if clip_stack.len() > 1 {
                        clip_stack.pop();
                    }
                }
                DrawCommand::Clear(_) => {
                    let y0: u32 = (active_clip.get_y().floor().max(0.0) as u32).min(phys_h);
                    let y1: u32 = (active_clip.bottom().ceil().max(0.0) as u32).min(phys_h);
                    if y0 < y1 {
                        prepped.push(PreppedCommand {
                            cmd_index,
                            active_clip,
                            y0,
                            y1,
                            opaque_cutout_h: Rect::default(),
                            opaque_cutout_v: Rect::default(),
                            text_layout_idx: u32::MAX,
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
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                text_layout_idx: u32::MAX,
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
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: cut_h,
                                opaque_cutout_v: cut_v,
                                text_layout_idx: u32::MAX,
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
                    let cache_key: (u64, u32) = (
                        hash_text_layout_key(text, *style),
                        (max_w * 10.0).round() as u32,
                    );
                    let layout: TextLayoutResult =
                        if let Some(cached) = text_layout_cache.get(&cache_key) {
                            cached.clone()
                        } else {
                            let measured: TextLayoutResult =
                                measurer.measure(text, *style, Some(max_w));
                            text_layout_cache.insert(cache_key, measured.clone());
                            measured
                        };
                    let phys_font_size: f32 = style.get_font_size() * scale;
                    for line in layout.get_lines().iter() {
                        for ch in line.get_text().chars() {
                            if ch != ' ' {
                                let key: GlyphKey =
                                    GlyphKey::from_style(ch, phys_font_size, style.get_weight());
                                let _: &RasterizedGlyph =
                                    glyph_atlas.get_or_rasterize_with(key, font_provider);
                            }
                        }
                    }
                    let layout_idx: u32 = text_layouts.len() as u32;
                    text_layouts.push(layout);
                    prepped.push(PreppedCommand {
                        cmd_index,
                        active_clip,
                        y0,
                        y1,
                        opaque_cutout_h: Rect::default(),
                        opaque_cutout_v: Rect::default(),
                        text_layout_idx: layout_idx,
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
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                text_layout_idx: u32::MAX,
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
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                text_layout_idx: u32::MAX,
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
                            prepped.push(PreppedCommand {
                                cmd_index,
                                active_clip,
                                y0,
                                y1,
                                opaque_cutout_h: Rect::default(),
                                opaque_cutout_v: Rect::default(),
                                text_layout_idx: u32::MAX,
                            });
                        }
                    }
                }
            }
        }
        (prepped, text_layouts)
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

    /// Executes all prepped commands overlapping `band`'s vertical slice `[y_offset, y_end)` completely lock-free.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Disjoint mutable horizontal pixel band.
    /// - `&[DrawCommand]` - Full frame draw command slice.
    /// - `&[PreppedCommand]` - Pre-clipped command list.
    /// - `&[TextLayoutResult]` - Pre-measured text layouts.
    /// - `&GlyphAtlas` - Immutable pre-warmed glyph atlas.
    /// - `f32` - Active DPI scale factor.
    /// - `AntialiasingMode` - Active anti-aliasing policy.
    ///
    /// # Returns
    ///
    /// - `usize` - Total shaded pixels in this band.
    pub(crate) fn render_band(
        band: &mut PixelBand<'_>,
        commands: &[DrawCommand],
        prepped: &[PreppedCommand],
        text_layouts: &[TextLayoutResult],
        glyph_atlas: &GlyphAtlas,
        scale: f32,
        aa: AntialiasingMode,
    ) -> usize {
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let band_rect: Rect = Rect::new(
            0.0,
            band_y0 as f32,
            band.width() as f32,
            (band_y1 - band_y0) as f32,
        );
        let mut shaded: usize = 0;
        for item in prepped.iter() {
            if item.get_y1() <= band_y0 || item.get_y0() >= band_y1 {
                continue;
            }
            let Some(active_clip) = item.get_active_clip().intersect(band_rect) else {
                continue;
            };
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
                DrawCommand::TextRun { rect, style, .. } => {
                    let lidx: usize = item.get_text_layout_idx() as usize;
                    if let Some(layout) = text_layouts.get(lidx) {
                        Self::raster_text_run_band(
                            band,
                            *rect,
                            layout,
                            *style,
                            scale,
                            active_clip,
                            glyph_atlas,
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
                DrawCommand::PushClip(_) | DrawCommand::PopClip => 0,
            };
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

    /// Rasterizes a rounded rectangle with optional border using edge-bounded 4x4 RGSS supersampling and linear-light spans.
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
        let border_w: f32 = border.get_width();
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let safe_inset: f32 = radius.max_radius().max(border_w) + 1.5;
        let edge_inset: f32 = border_w + 1.5;
        let iy0: u32 = ((rect.get_y() + safe_inset).ceil().max(0.0) as u32).clamp(y0, y1);
        let iy1: u32 = ((rect.bottom() - safe_inset).floor().max(0.0) as u32).clamp(y0, y1);
        let ix0: u32 = ((rect.get_x() + edge_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let ix1: u32 = ((rect.right() - edge_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let cx0: u32 = ((rect.get_x() + safe_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let cx1: u32 = ((rect.right() - safe_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_interior: bool = iy0 < iy1 && ix0 < ix1;
        let has_center_band: bool = cx0 < cx1;
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
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            let box_geom: (f32, f32, f32, f32) = (cx, py, half_w, half_h);
            if has_interior && y >= iy0 && y < iy1 {
                for x in x0..ix0 {
                    shaded += Self::shade_rounded_pixel(
                        band,
                        (x, y),
                        box_geom,
                        radius,
                        color,
                        border,
                        aa,
                    );
                }
                if color.get_a() > 0 {
                    if let Some(ref luts) = interior_luts {
                        band.fill_span_linear_luts(ix0, ix1, y, luts);
                    } else {
                        band.fill_span_linear(ix0, ix1, y, color);
                    }
                    shaded += (ix1 - ix0) as usize;
                }
                for x in ix1..x1 {
                    shaded += Self::shade_rounded_pixel(
                        band,
                        (x, y),
                        box_geom,
                        radius,
                        color,
                        border,
                        aa,
                    );
                }
            } else if has_center_band {
                for x in x0..cx0 {
                    shaded += Self::shade_rounded_pixel(
                        band,
                        (x, y),
                        box_geom,
                        radius,
                        color,
                        border,
                        aa,
                    );
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
                        let mixed: Color = composite_fill_and_border(
                            color,
                            border.get_color(),
                            outer_cov,
                            inner_cov,
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
                for x in cx1..x1 {
                    shaded += Self::shade_rounded_pixel(
                        band,
                        (x, y),
                        box_geom,
                        radius,
                        color,
                        border,
                        aa,
                    );
                }
            } else {
                for x in x0..x1 {
                    shaded += Self::shade_rounded_pixel(
                        band,
                        (x, y),
                        box_geom,
                        radius,
                        color,
                        border,
                        aa,
                    );
                }
            }
        }
        shaded
    }

    /// Evaluates fast early-rejected analytical SDF and edge-only 4x4 RGSS coverage for a single edge/corner pixel.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `(u32, u32)` - Global pixel `(x, y)` coordinates.
    /// - `(f32, f32, f32, f32)` - Box geometry `(center_x, rel_py, half_w, half_h)`.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Color` - Fill color.
    /// - `Border` - Border width and color.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - `1` if the pixel was shaded, `0` otherwise.
    fn shade_rounded_pixel(
        band: &mut PixelBand<'_>,
        pos: (u32, u32),
        geom: (f32, f32, f32, f32),
        radius: CornerRadius,
        color: Color,
        border: Border,
        aa: AntialiasingMode,
    ) -> usize {
        let (x, y): (u32, u32) = pos;
        let (cx, py, half_w, half_h): (f32, f32, f32, f32) = geom;
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
                    band.blend_pixel_linear_dithered(x, y, color, 0.0);
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
                band.blend_pixel_f32_dithered(x, y, color, alpha_f, 0.0);
                return 1;
            }
            return 0;
        }
        if dist < -(bw + 0.85) {
            if color.get_a() > 0 {
                band.blend_pixel_linear_dithered(x, y, color, 0.0);
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
            band.blend_pixel_linear_dithered(x, y, border.get_color(), 0.0);
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
            composite_fill_and_border(color, border.get_color(), outer_cov, inner_cov);
        if mixed.get_a() > 0 {
            band.blend_pixel_linear_dithered(x, y, mixed, 0.0);
            return 1;
        }
        0
    }

    /// Rasterizes a two-stop linear gradient inside an SDF rounded rectangle with edge-bounded RGSS and IGN dithering.
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
        let rad: f32 = gradient.get_angle_deg().to_radians();
        let dir_x: f32 = rad.sin();
        let dir_y: f32 = rad.cos();
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
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            let ny: f32 = ((y as f32) - rect.get_y()) * inv_h;
            for x in x0..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let corner_r: f32 = quadrant_corner_radius(radius, px, py)
                    .min(half_w)
                    .min(half_h);
                let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, corner_r);
                if dist > 0.85 {
                    continue;
                }
                let cov: f32 = if dist < -0.85 {
                    1.0
                } else {
                    let in_corner: bool = aa != AntialiasingMode::None
                        && corner_r > 0.5
                        && px.abs() > (half_w - corner_r - 1.5)
                        && py.abs() > (half_h - corner_r - 1.5);
                    if in_corner {
                        rounded_box_supersampled_coverage(px, py, half_w, half_h, corner_r, 0.0).0
                    } else {
                        sdf_coverage(dist, aa)
                    }
                };
                if cov <= 0.0 {
                    continue;
                }
                let nx: f32 = ((x as f32) - rect.get_x()) * inv_w;
                let t: f32 = ((nx - 0.5) * dir_x + (ny - 0.5) * dir_y + 0.5).clamp(0.0, 1.0);
                let base_sample: Color =
                    gradient.get_start_color().lerp(gradient.get_end_color(), t);
                let alpha_f: f32 = ((base_sample.get_a() as f32) * (1.0 / 255.0)) * cov;
                let dither: f32 = interleaved_gradient_noise(x, y);
                band.blend_pixel_f32_dithered(x, y, base_sample, alpha_f, dither);
                shaded += 1;
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
        let Some(vis) = shadow_rect.intersect(clip) else {
            return 0;
        };
        let half_w: f32 = (caster_rect.get_width() * 0.5).max(0.5);
        let half_h: f32 = (caster_rect.get_height() * 0.5).max(0.5);
        let cx: f32 = caster_rect.get_x() + half_w;
        let cy: f32 = caster_rect.get_y() + half_h;
        let base_alpha: f32 = (color.get_a() as f32) * (1.0 / 255.0);
        let inv_sigma: f32 = 1.0 / sigma.max(0.5);
        let cutoff: f32 = sigma * 3.0;
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
        let r_max: f32 = radius.max_radius();
        let inner_inset: f32 = cutoff + r_max;
        let sy0: u32 = ((caster_rect.get_y() + inner_inset).ceil().max(0.0) as u32).clamp(y0, y1);
        let sy1: u32 = ((caster_rect.bottom() - inner_inset).floor().max(0.0) as u32).clamp(y0, y1);
        let sx0: u32 = ((caster_rect.get_x() + inner_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let sx1: u32 = ((caster_rect.right() - inner_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_mid_x: bool = sx0 < sx1;
        let has_mid_y: bool = sy0 < sy1;
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            let (cut0, cut1): (u32, u32) = Self::shadow_row_cutout(y, cutouts, x0, x1);
            let spans: [(u32, u32); 2] = if cut0 < cut1 {
                [(x0, cut0), (cut1, x1)]
            } else {
                [(x0, x1), (x1, x1)]
            };
            if has_mid_x && has_mid_y && y >= sy0 && y < sy1 {
                for (rx0, rx1) in spans.into_iter() {
                    let left_end: u32 = rx1.min(sx0);
                    for x in rx0..left_end {
                        let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                        if dist < cutoff {
                            let alpha_f: f32 =
                                base_alpha * fast_gaussian_shadow_falloff(dist, inv_sigma);
                            if alpha_f > 0.0005 {
                                band.blend_pixel_f32_dithered(
                                    x,
                                    y,
                                    color,
                                    alpha_f,
                                    interleaved_gradient_noise(x, y),
                                );
                                shaded += 1;
                            }
                        }
                    }
                    let right_start: u32 = rx0.max(sx1);
                    for x in right_start..rx1 {
                        let dist: f32 = ((x as f32) + 0.5 - cx).abs() - half_w;
                        if dist < cutoff {
                            let alpha_f: f32 =
                                base_alpha * fast_gaussian_shadow_falloff(dist, inv_sigma);
                            if alpha_f > 0.0005 {
                                band.blend_pixel_f32_dithered(
                                    x,
                                    y,
                                    color,
                                    alpha_f,
                                    interleaved_gradient_noise(x, y),
                                );
                                shaded += 1;
                            }
                        }
                    }
                }
            } else if has_mid_x {
                let dist_y: f32 = py.abs() - half_h;
                let mid_alpha: f32 = if dist_y < cutoff {
                    base_alpha * fast_gaussian_shadow_falloff(dist_y, inv_sigma)
                } else {
                    0.0
                };
                for (rx0, rx1) in spans.into_iter() {
                    if rx0 >= rx1 {
                        continue;
                    }
                    let l_end: u32 = rx1.min(sx0);
                    for x in rx0..l_end {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cr: f32 = quadrant_corner_radius(radius, px, py);
                        let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, cr);
                        if dist < cutoff {
                            let alpha_f: f32 =
                                base_alpha * fast_gaussian_shadow_falloff(dist, inv_sigma);
                            if alpha_f > 0.0005 {
                                band.blend_pixel_f32_dithered(
                                    x,
                                    y,
                                    color,
                                    alpha_f,
                                    interleaved_gradient_noise(x, y),
                                );
                                shaded += 1;
                            }
                        }
                    }
                    let m_start: u32 = rx0.max(sx0);
                    let m_end: u32 = rx1.min(sx1);
                    if mid_alpha > 0.0005 && m_start < m_end {
                        band.blend_shadow_span_const_alpha(m_start, m_end, y, color, mid_alpha);
                        shaded += (m_end - m_start) as usize;
                    }
                    let r_start: u32 = rx0.max(sx1);
                    for x in r_start..rx1 {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cr: f32 = quadrant_corner_radius(radius, px, py);
                        let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, cr);
                        if dist < cutoff {
                            let alpha_f: f32 =
                                base_alpha * fast_gaussian_shadow_falloff(dist, inv_sigma);
                            if alpha_f > 0.0005 {
                                band.blend_pixel_f32_dithered(
                                    x,
                                    y,
                                    color,
                                    alpha_f,
                                    interleaved_gradient_noise(x, y),
                                );
                                shaded += 1;
                            }
                        }
                    }
                }
            } else {
                for (rx0, rx1) in spans.into_iter() {
                    for x in rx0..rx1 {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        let cr: f32 = quadrant_corner_radius(radius, px, py);
                        let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, cr);
                        if dist >= cutoff {
                            continue;
                        }
                        let alpha_f: f32 =
                            base_alpha * fast_gaussian_shadow_falloff(dist, inv_sigma);
                        if alpha_f > 0.0005 {
                            band.blend_pixel_f32_dithered(
                                x,
                                y,
                                color,
                                alpha_f,
                                interleaved_gradient_noise(x, y),
                            );
                            shaded += 1;
                        }
                    }
                }
            }
        }
        shaded
    }

    /// Rasterizes a pre-measured UTF-8 text run into `band` using the immutable pre-warmed `GlyphAtlas`.
    ///
    /// # Arguments
    ///
    /// - `&mut PixelBand<'_>` - Target pixel band.
    /// - `Rect` - Logical text bounding box.
    /// - `&TextLayoutResult` - Pre-measured wrapped lines.
    /// - `TextStyle` - Typography style.
    /// - `f32` - Active DPI scale factor.
    /// - `Rect` - Active physical clip bounds.
    /// - `&GlyphAtlas` - Read-only pre-warmed glyph atlas.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_text_run_band(
        band: &mut PixelBand<'_>,
        rect: Rect,
        layout: &TextLayoutResult,
        style: TextStyle,
        scale: f32,
        clip: Rect,
        glyph_atlas: &GlyphAtlas,
    ) -> usize {
        let phys_font_size: f32 = style.get_font_size() * scale;
        let line_h_phys: f32 = layout.get_line_height_px() * scale;
        let box_w_phys: f32 = rect.get_width() * scale;
        let color: Color = style.get_color();
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
        let letter_sp: f32 = style.get_letter_spacing() * scale;
        let space_adv: f32 = measure_char_advance(' ', style) * scale;
        let glyphs: &HashMap<GlyphKey, RasterizedGlyph> = glyph_atlas.get_glyphs();
        let mut shaded: usize = 0;
        for (line_idx, line) in layout.get_lines().iter().enumerate() {
            let pen_y: f32 = rect.get_y() * scale + (line_idx as f32) * line_h_phys;
            if (pen_y + line_h_phys) < (clip_y0 as f32) || pen_y > (clip_y1 as f32) {
                continue;
            }
            let align_x: f32 = match style.get_align() {
                TextAlign::Left => 0.0,
                TextAlign::Center | TextAlign::Right => {
                    let mut line_w_phys: f32 = 0.0;
                    for ch in line.get_text().chars() {
                        if ch == ' ' {
                            line_w_phys += space_adv + letter_sp;
                        } else {
                            let key: GlyphKey =
                                GlyphKey::from_style(ch, phys_font_size, style.get_weight());
                            let adv: f32 = glyphs
                                .get(&key)
                                .map(|g: &RasterizedGlyph| (g.get_advance() as f32).max(1.0))
                                .unwrap_or(space_adv);
                            line_w_phys += adv + letter_sp;
                        }
                    }
                    if style.get_align() == TextAlign::Center {
                        ((box_w_phys - line_w_phys) * 0.5).max(0.0)
                    } else {
                        (box_w_phys - line_w_phys).max(0.0)
                    }
                }
            };
            let mut pen_x: f32 = rect.get_x() * scale + align_x;
            for ch in line.get_text().chars() {
                if ch == ' ' {
                    pen_x += space_adv + letter_sp;
                    continue;
                }
                if pen_x > (clip_x1 as f32) {
                    break;
                }
                let key: GlyphKey = GlyphKey::from_style(ch, phys_font_size, style.get_weight());
                let Some(glyph) = glyphs.get(&key) else {
                    pen_x += space_adv + letter_sp;
                    continue;
                };
                let adv_phys: f32 = (glyph.get_advance() as f32).max(1.0) + letter_sp;
                let gx0: i32 = pen_x.round() as i32;
                let gy0: i32 =
                    (pen_y + (line_h_phys - (glyph.get_height() as f32)) * 0.5).round() as i32;
                let gw: i32 = glyph.get_width() as i32;
                let gh: i32 = glyph.get_height() as i32;
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
                pen_x += adv_phys;
            }
        }
        shaded
    }

    /// Blits a single bilinear-interpolated RGBA8 image scanline into `dst_row` using fixed-point 8.8 weights.
    ///
    /// # Arguments
    ///
    /// - `(usize, usize, f32, f32, f32)` - Row geometry `(y, x0, rect_y, inv_dst_h, src_h_f)`.
    /// - `(usize, usize)` - Source image dimensions `(src_w, max_sy)`.
    /// - `&[(usize, usize, u32)]` - Precomputed horizontal sample LUT `(sx0, sx1, wx_256)`.
    /// - `&[u8]` - Source RGBA8 pixel slice.
    /// - `&mut [u8]` - Destination framebuffer row slice.
    fn blit_bilinear_row(
        geom: (usize, usize, f32, f32, f32),
        src_dims: (usize, usize),
        x_lut: &[(usize, usize, u32)],
        src_bytes: &[u8],
        dst_row: &mut [u8],
    ) {
        let (y, x0, rect_y, inv_dst_h, src_h_f): (usize, usize, f32, f32, f32) = geom;
        let (src_w_usize, max_sy): (usize, usize) = src_dims;
        let v: f32 = (((y as f32) + 0.5 - rect_y) * inv_dst_h).clamp(0.0, 1.0);
        let fy: f32 = (v * src_h_f - 0.5).clamp(0.0, max_sy as f32);
        let sy0: usize = (fy.floor() as usize).min(max_sy);
        let sy1: usize = (sy0 + 1).min(max_sy);
        let wy: u32 = ((fy - (sy0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
        let iwy: u32 = 256 - wy;
        let row0: usize = sy0 * src_w_usize;
        let row1: usize = sy1 * src_w_usize;
        let src_len: usize = src_bytes.len();
        for (lx, &(sx0, sx1, wx)) in x_lut.iter().enumerate() {
            let i00: usize = (row0 + sx0) * 4;
            let i10: usize = (row0 + sx1) * 4;
            let i01: usize = (row1 + sx0) * 4;
            let i11: usize = (row1 + sx1) * 4;
            if i11 + 4 > src_len {
                continue;
            }
            let didx: usize = (x0 + lx) * 4;
            if didx + 4 > dst_row.len() {
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
                dst_row[didx] = (((src_bytes[i00] as u32) * w00
                    + (src_bytes[i10] as u32) * w10
                    + (src_bytes[i01] as u32) * w01
                    + (src_bytes[i11] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dst_row[didx + 1] = (((src_bytes[i00 + 1] as u32) * w00
                    + (src_bytes[i10 + 1] as u32) * w10
                    + (src_bytes[i01 + 1] as u32) * w01
                    + (src_bytes[i11 + 1] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dst_row[didx + 2] = (((src_bytes[i00 + 2] as u32) * w00
                    + (src_bytes[i10 + 2] as u32) * w10
                    + (src_bytes[i01 + 2] as u32) * w01
                    + (src_bytes[i11 + 2] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dst_row[didx + 3] = 255;
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
            dst_row[didx] = ((r * a_val + (dst_row[didx] as u32) * inv_a + 127) / 255) as u8;
            dst_row[didx + 1] =
                ((g * a_val + (dst_row[didx + 1] as u32) * inv_a + 127) / 255) as u8;
            dst_row[didx + 2] =
                ((b * a_val + (dst_row[didx + 2] as u32) * inv_a + 127) / 255) as u8;
            dst_row[didx + 3] = 255;
        }
    }

    /// Blits a decoded `ImageBuffer` into `band` with bilinear filtering and per-quadrant SDF rounded clip.
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
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let fb_w: u32 = band.width();
        let band_y0: u32 = band.y_offset();
        let band_y1: u32 = band.y_end();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).clamp(band_y0, band_y1);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).clamp(band_y0, band_y1);
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
        if !has_radius && use_bilinear && x0 < x1 && y0 < y1 {
            let span_w: usize = (x1 - x0) as usize;
            let mut x_lut: Vec<(usize, usize, u32)> = Vec::with_capacity(span_w);
            for x in x0..x1 {
                let u: f32 = (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                let fx: f32 = (u * (src_w as f32) - 0.5).clamp(0.0, max_sx as f32);
                let sx0: usize = (fx.floor() as usize).min(max_sx);
                let sx1: usize = (sx0 + 1).min(max_sx);
                let wx: u32 = ((fx - (sx0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
                x_lut.push((sx0, sx1, wx));
            }
            let x_lut_ref: &[(usize, usize, u32)] = x_lut.as_slice();
            let rect_y: f32 = rect.get_y();
            let src_h_f: f32 = src_h as f32;
            let x0_usize: usize = x0 as usize;
            for y in y0..y1 {
                if let Some(row_buf) = band.row_slice_mut(y) {
                    Self::blit_bilinear_row(
                        (y as usize, x0_usize, rect_y, inv_dst_h, src_h_f),
                        (src_w_usize, max_sy),
                        x_lut_ref,
                        src_bytes,
                        row_buf,
                    );
                }
            }
            return span_w * ((y1 - y0) as usize);
        }
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let v: f32 = (((y as f32) + 0.5 - rect.get_y()) * inv_dst_h).clamp(0.0, 1.0);
            let py: f32 = (y as f32) + 0.5 - cy;
            if use_bilinear {
                let fy: f32 = (v * (src_h as f32) - 0.5).clamp(0.0, max_sy as f32);
                let sy0: usize = (fy.floor() as usize).min(max_sy);
                let sy1: usize = (sy0 + 1).min(max_sy);
                let wy: f32 = fy - (sy0 as f32);
                let iwy: f32 = 1.0 - wy;
                let row0: usize = sy0 * src_w_usize;
                let row1: usize = sy1 * src_w_usize;
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
                    let fx: f32 = (u * (src_w as f32) - 0.5).clamp(0.0, max_sx as f32);
                    let sx0: usize = (fx.floor() as usize).min(max_sx);
                    let sx1: usize = (sx0 + 1).min(max_sx);
                    let wx: f32 = fx - (sx0 as f32);
                    let iwx: f32 = 1.0 - wx;
                    let i00: usize = (row0 + sx0) * 4;
                    let i10: usize = (row0 + sx1) * 4;
                    let i01: usize = (row1 + sx0) * 4;
                    let i11: usize = (row1 + sx1) * 4;
                    if i11 + 4 <= src_bytes.len() {
                        let w00: f32 = iwx * iwy;
                        let w10: f32 = wx * iwy;
                        let w01: f32 = iwx * wy;
                        let w11: f32 = wx * wy;
                        let r: u8 = ((src_bytes[i00] as f32) * w00
                            + (src_bytes[i10] as f32) * w10
                            + (src_bytes[i01] as f32) * w01
                            + (src_bytes[i11] as f32) * w11)
                            .round() as u8;
                        let g: u8 = ((src_bytes[i00 + 1] as f32) * w00
                            + (src_bytes[i10 + 1] as f32) * w10
                            + (src_bytes[i01 + 1] as f32) * w01
                            + (src_bytes[i11 + 1] as f32) * w11)
                            .round() as u8;
                        let b: u8 = ((src_bytes[i00 + 2] as f32) * w00
                            + (src_bytes[i10 + 2] as f32) * w10
                            + (src_bytes[i01 + 2] as f32) * w01
                            + (src_bytes[i11 + 2] as f32) * w11)
                            .round() as u8;
                        let a: u8 = ((src_bytes[i00 + 3] as f32) * w00
                            + (src_bytes[i10 + 3] as f32) * w10
                            + (src_bytes[i01 + 3] as f32) * w01
                            + (src_bytes[i11 + 3] as f32) * w11)
                            .round() as u8;
                        let sample: Color = Color::new(r, g, b, a).multiply_alpha(cov);
                        band.blend_pixel(x, y, sample);
                        shaded += 1;
                    }
                }
            } else {
                let sy: usize = ((v * (src_h as f32)).floor() as usize).min(max_sy);
                let row: usize = sy * src_w_usize;
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
                        let sample: Color = Color::new(
                            src_bytes[idx],
                            src_bytes[idx + 1],
                            src_bytes[idx + 2],
                            src_bytes[idx + 3],
                        )
                        .multiply_alpha(cov);
                        band.blend_pixel(x, y, sample);
                        shaded += 1;
                    }
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
