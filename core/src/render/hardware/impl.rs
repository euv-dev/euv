use super::*;

/// `GpuDriver` implementation for `RecordingGpuDriver`.
impl GpuDriver for RecordingGpuDriver {
    /// Returns the configured backend tag.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Driver reference.
    ///
    /// # Returns
    ///
    /// - `GraphicsBackend` - Active backend tag.
    fn backend(&self) -> GraphicsBackend {
        self.get_backend_kind()
    }

    /// Selects the canonical native shader source for this backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Driver reference.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Native shader source string.
    fn shader_source(&self) -> &'static str {
        match self.get_backend_kind() {
            GraphicsBackend::Metal => MSL_UI_SHADER,
            GraphicsBackend::Direct3D12 | GraphicsBackend::Direct3D11 => HLSL_UI_SHADER,
            GraphicsBackend::WebGl2 | GraphicsBackend::OpenGlEs => GLSL_ES3_UI_FRAGMENT_SHADER,
            GraphicsBackend::WebGpu
            | GraphicsBackend::Vulkan
            | GraphicsBackend::SoftwareCpu
            | GraphicsBackend::Canvas2d
            | GraphicsBackend::Headless => WGSL_UI_SHADER,
        }
    }

    /// Records submitted GPU batches and returns draw-call count.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable driver.
    /// - `u32` - Surface width.
    /// - `u32` - Surface height.
    /// - `Color` - Clear color.
    /// - `&[GpuBatch]` - Ordered batches.
    ///
    /// # Returns
    ///
    /// - `Result<usize, PlatformError>` - Submitted batch count.
    fn submit_batches(
        &mut self,
        _width: u32,
        _height: u32,
        clear_color: Color,
        batches: &[GpuBatch],
    ) -> Result<usize, PlatformError> {
        let count: usize = batches.len();
        let instances: usize = batches.iter().fold(0_usize, |acc: usize, b: &GpuBatch| {
            acc + b.get_instances().len()
        });
        self.set_last_clear(clear_color);
        self.set_last_instance_count(instances);
        self.set_total_draw_calls(self.get_total_draw_calls().saturating_add(count));
        Ok(count)
    }
}

/// Instanced batch builder, order-preserving coalescer, and telemetry methods for `HardwareRenderer`.
impl HardwareRenderer {
    /// Creates a hardware renderer bound to `backend` with optional deterministic pixel readback.
    ///
    /// # Arguments
    ///
    /// - `GraphicsBackend` - Selected GPU backend.
    /// - `u32` - Initial physical width.
    /// - `u32` - Initial physical height.
    /// - `RenderConfig` - Rendering configuration.
    /// - `bool` - Whether software readback mirror is enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured hardware renderer.
    #[must_use]
    pub fn with_backend(
        backend: GraphicsBackend,
        width: u32,
        height: u32,
        config: RenderConfig,
        enable_readback: bool,
    ) -> Self {
        Self::with_backend_and_font(
            backend,
            width,
            height,
            config,
            enable_readback,
            NativeSystemFont::deterministic(),
        )
    }

    /// Creates a hardware renderer bound to `backend` with an explicit font provider for readback.
    ///
    /// # Arguments
    ///
    /// - `GraphicsBackend` - Selected GPU backend.
    /// - `u32` - Initial physical width.
    /// - `u32` - Initial physical height.
    /// - `RenderConfig` - Rendering configuration.
    /// - `bool` - Whether software readback mirror is enabled.
    /// - `NativeSystemFont` - Font provider used by the software readback mirror.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured hardware renderer.
    #[must_use]
    pub fn with_backend_and_font(
        backend: GraphicsBackend,
        width: u32,
        height: u32,
        config: RenderConfig,
        enable_readback: bool,
        font_provider: NativeSystemFont,
    ) -> Self {
        let driver: NativeGpuDriver = NativeGpuDriver::probe(backend);
        Self {
            driver,
            batches: Vec::with_capacity(16),
            active_batches: 0,
            clip_stack: Vec::with_capacity(16),
            last_submitted_batches: 0,
            last_coalesced_merges: 0,
            last_culled_instances: 0,
            last_uploaded_instances: 0,
            last_uploaded_bytes: 0,
            readback: SoftwareRenderer::with_font_provider(width, height, config, font_provider),
            enable_readback,
        }
    }

    /// Returns latest-frame GPU batching and upload telemetry `(batches, coalesced_merges, culled_instances, uploaded_instances, uploaded_bytes)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hardware renderer reference.
    ///
    /// # Returns
    ///
    /// - `(usize, usize, usize, usize, usize)` - Latest frame batching and upload telemetry tuple.
    #[must_use]
    pub fn last_frame_telemetry(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.get_last_submitted_batches(),
            self.get_last_coalesced_merges(),
            self.get_last_culled_instances(),
            self.get_last_uploaded_instances(),
            self.get_last_uploaded_bytes(),
        )
    }

    /// Computes a deterministic 64-bit FNV-1a hash over all compiled GPU batch instances.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hardware renderer reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Deterministic batch geometry hash.
    #[must_use]
    pub fn batch_hash(&self) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        let active: usize = self.get_active_batches().min(self.get_batches().len());
        for batch in self.get_batches()[..active].iter() {
            for inst in batch.get_instances().iter() {
                for v in inst
                    .get_rect()
                    .iter()
                    .chain(inst.get_color().iter())
                    .chain(inst.get_radii().iter())
                {
                    hash ^= v.to_bits() as u64;
                    hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
        }
        hash
    }

    /// Returns `true` when physical quad `quad = [x, y, w, h]` lies completely inside `container`.
    ///
    /// # Arguments
    ///
    /// - `[f32; 4]` - Physical quad bounds `[x, y, width, height]`.
    /// - `Rect` - Enclosing physical clip rectangle.
    ///
    /// # Returns
    ///
    /// - `bool` - True when no fragment of `quad` extends outside `container`.
    #[inline(always)]
    #[must_use]
    fn quad_inside_rect(quad: [f32; 4], container: Rect) -> bool {
        let qx0: f32 = quad[0];
        let qy0: f32 = quad[1];
        let qx1: f32 = qx0 + quad[2];
        let qy1: f32 = qy0 + quad[3];
        let x_in: bool = qx0 >= container.get_x() && qx1 <= container.right();
        let y_in: bool = qy0 >= container.get_y() && qy1 <= container.bottom();
        container.get_width() > 0.0 && container.get_height() > 0.0 && x_in && y_in
    }

    /// Returns `true` when `quad` intersects `clip` with strictly positive area.
    ///
    /// # Arguments
    ///
    /// - `[f32; 4]` - Physical quad bounds `[x, y, width, height]`.
    /// - `Rect` - Active physical scissor clip.
    ///
    /// # Returns
    ///
    /// - `bool` - True when at least part of `quad` falls inside `clip`.
    #[inline(always)]
    #[must_use]
    fn quad_intersects_clip(quad: [f32; 4], clip: Rect) -> bool {
        let qw: f32 = quad[2];
        let qh: f32 = quad[3];
        let quad_empty: bool = qw <= 0.0 || qh <= 0.0;
        let clip_empty: bool = clip.get_width() <= 0.0 || clip.get_height() <= 0.0;
        if quad_empty || clip_empty {
            return false;
        }
        let qx0: f32 = quad[0];
        let qy0: f32 = quad[1];
        let qx1: f32 = qx0 + qw;
        let qy1: f32 = qy0 + qh;
        let x_hit: bool = qx1 > clip.get_x() && qx0 < clip.right();
        let y_hit: bool = qy1 > clip.get_y() && qy0 < clip.bottom();
        x_hit && y_hit
    }

    /// Returns `true` when `inner` lies completely inside `outer`.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Candidate inner rectangle.
    /// - `Rect` - Candidate outer enclosing rectangle.
    ///
    /// # Returns
    ///
    /// - `bool` - True when `inner` is contained inside `outer`.
    #[inline(always)]
    #[must_use]
    fn rect_inside_rect(inner: Rect, outer: Rect) -> bool {
        let x_in: bool = inner.get_x() >= outer.get_x() && inner.right() <= outer.right();
        let y_in: bool = inner.get_y() >= outer.get_y() && inner.bottom() <= outer.bottom();
        x_in && y_in
    }

    /// Returns the union bounding box of `existing` and physical quad `quad = [x, y, w, h]`.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Existing bounding box.
    /// - `[f32; 4]` - Newly appended quad bounds `[x, y, width, height]`.
    ///
    /// # Returns
    ///
    /// - `Rect` - Enclosing union rectangle.
    #[inline(always)]
    #[must_use]
    fn union_rect_quad(existing: Rect, quad: [f32; 4]) -> Rect {
        let x0: f32 = existing.get_x().min(quad[0]);
        let y0: f32 = existing.get_y().min(quad[1]);
        let x1: f32 = existing.right().max(quad[0] + quad[2]);
        let y1: f32 = existing.bottom().max(quad[1] + quad[3]);
        Rect::new(x0, y0, (x1 - x0).max(0.0), (y1 - y0).max(0.0))
    }

    /// Compiles `display_list` into order-preserving coalesced GPU instanced batches and submits them to the GPU driver.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hardware renderer.
    /// - `&DisplayList` - Backend-agnostic draw command stream.
    /// - `DamageRegion` - Active damage region for readback mirroring.
    ///
    /// # Returns
    ///
    /// - `Result<(usize, usize), PlatformError>` - Tuple of `(draw_batches, pixels_shaded)`.
    pub fn render(
        &mut self,
        display_list: &DisplayList,
        damage: DamageRegion,
    ) -> Result<(usize, usize), PlatformError> {
        let scale: f32 = display_list.get_scale_factor().max(0.5);
        let phys_w: u32 = ((display_list.get_viewport().get_width() * scale).round() as u32).max(1);
        let phys_h: u32 =
            ((display_list.get_viewport().get_height() * scale).round() as u32).max(1);
        let full_clip: Rect = Rect::new(0.0, 0.0, phys_w as f32, phys_h as f32);
        let full_entry: GpuClipEntry = GpuClipEntry {
            scissor: full_clip,
            safe_inner: full_clip,
        };
        let Self {
            driver,
            batches,
            active_batches,
            clip_stack,
            last_submitted_batches,
            last_coalesced_merges,
            last_culled_instances,
            last_uploaded_instances,
            last_uploaded_bytes,
            readback,
            enable_readback,
        } = self;
        clip_stack.clear();
        clip_stack.push(full_entry);
        *active_batches = 0;
        let mut tail_state: GpuBatchCoalesceState = GpuBatchCoalesceState::default();
        let mut coalesced_merges: usize = 0;
        let mut culled_instances: usize = 0;
        let mut uploaded_instances: usize = 0;
        let mut clear_color: Color = COLOR_WHITE;
        for cmd in display_list.get_commands().iter() {
            let clip_entry: GpuClipEntry = clip_stack.last().copied().unwrap_or(full_entry);
            let active_clip: Rect = clip_entry.scissor;
            match cmd {
                DrawCommand::Clear(c) => {
                    clear_color = *c;
                }
                DrawCommand::PushClip(rect) => {
                    let phys: Rect = snap_physical_rect(*rect, scale);
                    let next_scissor: Rect = active_clip.intersect(phys).unwrap_or_default();
                    let next_safe: Rect = clip_entry.safe_inner.intersect(phys).unwrap_or_default();
                    clip_stack.push(GpuClipEntry {
                        scissor: next_scissor,
                        safe_inner: next_safe,
                    });
                }
                DrawCommand::PushRoundedClip { rect, radius } => {
                    let phys: Rect = snap_physical_rect(*rect, scale);
                    let next_scissor: Rect = active_clip.intersect(phys).unwrap_or_default();
                    let max_r: f32 = (radius.max_radius() * scale).ceil();
                    let inset_w: f32 = (phys.get_width() - max_r * 2.0).max(0.0);
                    let inset_h: f32 = (phys.get_height() - max_r * 2.0).max(0.0);
                    let rounded_safe: Rect =
                        Rect::new(phys.get_x() + max_r, phys.get_y() + max_r, inset_w, inset_h);
                    let next_safe: Rect = clip_entry
                        .safe_inner
                        .intersect(rounded_safe)
                        .unwrap_or_default();
                    clip_stack.push(GpuClipEntry {
                        scissor: next_scissor,
                        safe_inner: next_safe,
                    });
                }
                DrawCommand::PopClip => {
                    if clip_stack.len() > 1 {
                        let _: Option<GpuClipEntry> = clip_stack.pop();
                    }
                }
                DrawCommand::FillRoundedRect {
                    rect,
                    color,
                    radius,
                    border,
                } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let quad: [f32; 4] = [
                        snapped.get_x(),
                        snapped.get_y(),
                        snapped.get_width(),
                        snapped.get_height(),
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let bw: f32 = if border.get_width() > 0.0 {
                        (border.get_width() * scale).max(1.0)
                    } else {
                        0.0
                    };
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: color.to_f32_array(),
                        secondary_color: border.get_color().to_f32_array(),
                        radii: [
                            radius.get_top_left() * scale,
                            radius.get_top_right() * scale,
                            radius.get_bottom_right() * scale,
                            radius.get_bottom_left() * scale,
                        ],
                        stroke_or_blur: bw,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::RoundedRect as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::LinearGradientRect {
                    rect,
                    gradient,
                    radius,
                } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let quad: [f32; 4] = [
                        snapped.get_x(),
                        snapped.get_y(),
                        snapped.get_width(),
                        snapped.get_height(),
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: gradient.get_start_color().to_f32_array(),
                        secondary_color: gradient.get_end_color().to_f32_array(),
                        radii: [
                            radius.get_top_left() * scale,
                            radius.get_top_right() * scale,
                            radius.get_bottom_right() * scale,
                            radius.get_bottom_left() * scale,
                        ],
                        stroke_or_blur: 0.0,
                        angle_rad: gradient.get_angle_deg().to_radians(),
                        kind: GpuPrimitiveKind::Gradient as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
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
                    let caster_x: f32 =
                        base_rect.get_x() + (shadow.get_offset_x() * scale).round() - spread;
                    let caster_y: f32 =
                        base_rect.get_y() + (shadow.get_offset_y() * scale).round() - spread;
                    let caster_w: f32 = (base_rect.get_width() + spread * 2.0).max(1.0);
                    let caster_h: f32 = (base_rect.get_height() + spread * 2.0).max(1.0);
                    let quad: [f32; 4] = [
                        caster_x - pad,
                        caster_y - pad,
                        caster_w + pad * 2.0,
                        caster_h + pad * 2.0,
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: shadow.get_color().to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [
                            (radius.get_top_left() * scale + spread).max(0.0),
                            (radius.get_top_right() * scale + spread).max(0.0),
                            (radius.get_bottom_right() * scale + spread).max(0.0),
                            (radius.get_bottom_left() * scale + spread).max(0.0),
                        ],
                        stroke_or_blur: sigma,
                        angle_rad: pad,
                        kind: GpuPrimitiveKind::Shadow as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::TextRun { rect, text, style } => {
                    let quad: [f32; 4] = [
                        rect.get_x() * scale,
                        rect.get_y() * scale,
                        rect.get_width() * scale,
                        rect.get_height() * scale,
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let glyph_count: usize = if text.is_ascii() {
                        text.as_bytes()
                            .iter()
                            .filter(|b: &&u8| **b != b' ')
                            .count()
                            .max(1)
                    } else {
                        text.chars().filter(|c: &char| *c != ' ').count().max(1)
                    };
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: style.get_color().to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [0.0; 4],
                        stroke_or_blur: style.get_font_size() * scale,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::Glyph as u32,
                        reserved: glyph_count as u32,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
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
                    if active_clip.get_width() <= 0.0 || active_clip.get_height() <= 0.0 {
                        culled_instances += 1;
                        continue;
                    }
                    let meas: TextLayoutResult = DeterministicTextMeasurer.measure(
                        text,
                        *style,
                        if *multiline {
                            Some(content_rect.get_width().max(1.0))
                        } else {
                            None
                        },
                    );
                    let last_w: f32 = meas
                        .get_lines()
                        .last()
                        .map(TextLine::get_width)
                        .unwrap_or(0.0);
                    let line_idx: usize = meas.get_lines().len().saturating_sub(1);
                    let trailing_gap: f32 = if last_w > 0.0 { 1.5 } else { 0.0 };
                    let min_x: f32 = content_rect.get_x();
                    let max_x: f32 = (content_rect.right() - 2.0).max(min_x);
                    let caret_x: f32 = (text_rect.get_x() + last_w + trailing_gap)
                        .min(max_x)
                        .max(min_x);
                    let caret_y: f32 = if *multiline {
                        (content_rect.get_y()
                            + (line_idx as f32) * meas.get_line_height_px()
                            + ((meas.get_line_height_px() - caret_height) * 0.5).max(0.0))
                        .min(content_rect.bottom() - caret_height)
                        .max(content_rect.get_y())
                    } else {
                        content_rect.get_y()
                            + ((content_rect.get_height() - caret_height) * 0.5).max(0.0)
                    };
                    let quad: [f32; 4] = [
                        caret_x * scale,
                        caret_y * scale,
                        (1.5 * scale).max(1.0),
                        (caret_height * scale).max(2.0),
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: color.to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [0.5 * scale; 4],
                        stroke_or_blur: 0.0,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::RoundedRect as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::Image { rect, radius, .. } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let quad: [f32; 4] = [
                        snapped.get_x(),
                        snapped.get_y(),
                        snapped.get_width(),
                        snapped.get_height(),
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: [1.0, 1.0, 1.0, 1.0],
                        secondary_color: [0.0; 4],
                        radii: [
                            radius.get_top_left() * scale,
                            radius.get_top_right() * scale,
                            radius.get_bottom_right() * scale,
                            radius.get_bottom_left() * scale,
                        ],
                        stroke_or_blur: 0.0,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::TexturedQuad as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::Circle {
                    center,
                    radius,
                    color,
                } => {
                    let r: f32 = radius * scale;
                    let quad: [f32; 4] = [
                        center.get_x() * scale - r,
                        center.get_y() * scale - r,
                        r * 2.0,
                        r * 2.0,
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: color.to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [r; 4],
                        stroke_or_blur: 0.0,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::Circle as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::Line {
                    from,
                    to,
                    color,
                    width,
                } => {
                    let dx: f32 = (to.get_x() - from.get_x()) * scale;
                    let dy: f32 = (to.get_y() - from.get_y()) * scale;
                    let x0: f32 = from.get_x().min(to.get_x()) * scale;
                    let y0: f32 = from.get_y().min(to.get_y()) * scale;
                    let w: f32 = dx.abs().max(width * scale);
                    let h: f32 = dy.abs().max(width * scale);
                    let quad: [f32; 4] = [x0, y0, w, h];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: color.to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [
                            from.get_x() * scale,
                            from.get_y() * scale,
                            to.get_x() * scale,
                            to.get_y() * scale,
                        ],
                        stroke_or_blur: width * scale,
                        angle_rad: dy.atan2(dx),
                        kind: GpuPrimitiveKind::Line as u32,
                        reserved: 0,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::Triangle { p0, p1, p2, color } => {
                    let min_x: f32 = p0.get_x().min(p1.get_x()).min(p2.get_x()) * scale;
                    let min_y: f32 = p0.get_y().min(p1.get_y()).min(p2.get_y()) * scale;
                    let max_x: f32 = p0.get_x().max(p1.get_x()).max(p2.get_x()) * scale;
                    let max_y: f32 = p0.get_y().max(p1.get_y()).max(p2.get_y()) * scale;
                    let quad: [f32; 4] = [
                        min_x,
                        min_y,
                        (max_x - min_x).max(1.0),
                        (max_y - min_y).max(1.0),
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: color.to_f32_array(),
                        secondary_color: [p0.get_x() * scale, p0.get_y() * scale, 0.0, 0.0],
                        radii: [
                            p1.get_x() * scale,
                            p1.get_y() * scale,
                            p2.get_x() * scale,
                            p2.get_y() * scale,
                        ],
                        stroke_or_blur: 0.0,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::Triangle as u32,
                        reserved: 3,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
                DrawCommand::SpriteRegion { rect, tint, .. } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let quad: [f32; 4] = [
                        snapped.get_x(),
                        snapped.get_y(),
                        snapped.get_width(),
                        snapped.get_height(),
                    ];
                    if !Self::quad_intersects_clip(quad, active_clip) {
                        culled_instances += 1;
                        continue;
                    }
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: quad,
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: tint.to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [0.0; 4],
                        stroke_or_blur: 0.0,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::TexturedQuad as u32,
                        reserved: 1,
                    };
                    Self::push_instance_coalesced(
                        batches,
                        active_batches,
                        &mut tail_state,
                        clip_entry,
                        inst,
                        &mut coalesced_merges,
                    );
                    uploaded_instances += 1;
                }
            }
        }
        let active_slice: &[GpuBatch] = &batches[..(*active_batches).min(batches.len())];
        let submitted: usize = driver.submit_batches(phys_w, phys_h, clear_color, active_slice)?;
        *last_submitted_batches = submitted;
        *last_coalesced_merges = coalesced_merges;
        *last_culled_instances = culled_instances;
        *last_uploaded_instances = uploaded_instances;
        *last_uploaded_bytes = uploaded_instances.saturating_mul(GPU_QUAD_INSTANCE_BYTES);
        let pixels: usize = if *enable_readback {
            let (_, px): (usize, usize) = readback.render(display_list, damage);
            px
        } else {
            (phys_w as usize) * (phys_h as usize)
        };
        Ok((submitted, pixels))
    }

    /// Appends `instance` in strict draw order into the tail `GpuBatch` when clip-compatible, or opens the next reusable batch slot.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<GpuBatch>` - Persistent batch pool vector.
    /// - `&mut usize` - Active batch watermark for the current frame.
    /// - `&mut GpuBatchCoalesceState` - Unclipped containment metadata for the active tail batch.
    /// - `GpuClipEntry` - Active physical scissor and corner-free interior safe clip rectangle.
    /// - `GpuQuadInstance` - Instance data to batch in exact back-to-front order.
    /// - `&mut usize` - Running counter of cross-clip batch splits avoided by coalescing.
    fn push_instance_coalesced(
        batches: &mut Vec<GpuBatch>,
        active_batches: &mut usize,
        tail_state: &mut GpuBatchCoalesceState,
        clip_entry: GpuClipEntry,
        instance: GpuQuadInstance,
        coalesced_merges: &mut usize,
    ) {
        let scissor: Rect = clip_entry.scissor;
        let inst_unclipped: bool = Self::quad_inside_rect(instance.rect, clip_entry.safe_inner);
        let count: usize = *active_batches;
        if count > 0 {
            let last: &mut GpuBatch = &mut batches[count - 1];
            if last.get_instances().len() < MAX_QUADS_PER_BATCH {
                let last_scissor: Rect = last.get_scissor();
                if last_scissor == scissor {
                    last.get_mut_instances().push(instance);
                    tail_state.all_unclipped = tail_state.all_unclipped && inst_unclipped;
                    tail_state.bounds = Self::union_rect_quad(tail_state.bounds, instance.rect);
                    return;
                }
                if inst_unclipped && Self::quad_inside_rect(instance.rect, last_scissor) {
                    last.get_mut_instances().push(instance);
                    tail_state.bounds = Self::union_rect_quad(tail_state.bounds, instance.rect);
                    *coalesced_merges += 1;
                    return;
                }
                if tail_state.all_unclipped && Self::rect_inside_rect(tail_state.bounds, scissor) {
                    last.set_scissor(scissor);
                    last.get_mut_instances().push(instance);
                    tail_state.all_unclipped = inst_unclipped;
                    tail_state.bounds = Self::union_rect_quad(tail_state.bounds, instance.rect);
                    *coalesced_merges += 1;
                    return;
                }
            }
        }
        if count < batches.len() {
            let slot: &mut GpuBatch = &mut batches[count];
            slot.set_scissor(scissor);
            slot.get_mut_instances().clear();
            slot.get_mut_instances().push(instance);
        } else {
            let mut batch: GpuBatch = GpuBatch::default();
            batch.set_scissor(scissor);
            batch.get_mut_instances().push(instance);
            batches.push(batch);
        }
        *active_batches = count + 1;
        tail_state.all_unclipped = inst_unclipped;
        tail_state.bounds = Rect::new(
            instance.rect[0],
            instance.rect[1],
            instance.rect[2],
            instance.rect[3],
        );
    }
}
