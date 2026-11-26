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

/// Instanced batch builder and execution methods for `HardwareRenderer`.
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
            readback: SoftwareRenderer::with_font_provider(width, height, config, font_provider),
            enable_readback,
        }
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
        for batch in self.get_batches().iter() {
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

    /// Compiles `display_list` into coalesced GPU instanced batches and submits them to the GPU driver.
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
        let mut clip_stack: Vec<Rect> = vec![full_clip];
        let mut clear_color: Color = COLOR_WHITE;
        self.get_mut_batches().clear();
        for cmd in display_list.get_commands().iter() {
            let active_clip: Rect = clip_stack.last().copied().unwrap_or(full_clip);
            match cmd {
                DrawCommand::Clear(c) => {
                    clear_color = *c;
                }
                DrawCommand::PushClip(rect) => {
                    let phys: Rect = Rect::new(
                        rect.get_x() * scale,
                        rect.get_y() * scale,
                        rect.get_width() * scale,
                        rect.get_height() * scale,
                    );
                    clip_stack.push(active_clip.intersect(phys).unwrap_or_default());
                }
                DrawCommand::PopClip => {
                    if clip_stack.len() > 1 {
                        clip_stack.pop();
                    }
                }
                DrawCommand::FillRoundedRect {
                    rect,
                    color,
                    radius,
                    border,
                } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            snapped.get_x(),
                            snapped.get_y(),
                            snapped.get_width(),
                            snapped.get_height(),
                        ],
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
                        stroke_or_blur: border.get_width() * scale,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::RoundedRect as u32,
                        reserved: 0,
                    };
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::LinearGradientRect {
                    rect,
                    gradient,
                    radius,
                } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            snapped.get_x(),
                            snapped.get_y(),
                            snapped.get_width(),
                            snapped.get_height(),
                        ],
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
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::BoxShadow {
                    rect,
                    radius,
                    shadow,
                } => {
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            (rect.get_x() + shadow.get_offset_x()) * scale,
                            (rect.get_y() + shadow.get_offset_y()) * scale,
                            rect.get_width() * scale,
                            rect.get_height() * scale,
                        ],
                        clip: [
                            active_clip.get_x(),
                            active_clip.get_y(),
                            active_clip.right(),
                            active_clip.bottom(),
                        ],
                        color: shadow.get_color().to_f32_array(),
                        secondary_color: [0.0; 4],
                        radii: [
                            radius.get_top_left() * scale,
                            radius.get_top_right() * scale,
                            radius.get_bottom_right() * scale,
                            radius.get_bottom_left() * scale,
                        ],
                        stroke_or_blur: shadow.get_blur_radius() * scale,
                        angle_rad: 0.0,
                        kind: GpuPrimitiveKind::Shadow as u32,
                        reserved: 0,
                    };
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::TextRun { rect, text, style } => {
                    let glyph_count: usize =
                        text.chars().filter(|c: &char| *c != ' ').count().max(1);
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            rect.get_x() * scale,
                            rect.get_y() * scale,
                            rect.get_width() * scale,
                            rect.get_height() * scale,
                        ],
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
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::Image { rect, radius, .. } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            snapped.get_x(),
                            snapped.get_y(),
                            snapped.get_width(),
                            snapped.get_height(),
                        ],
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
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::Circle {
                    center,
                    radius,
                    color,
                } => {
                    let r: f32 = radius * scale;
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            center.get_x() * scale - r,
                            center.get_y() * scale - r,
                            r * 2.0,
                            r * 2.0,
                        ],
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
                    self.push_instance(active_clip, inst);
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
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [x0, y0, w, h],
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
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::Triangle { p0, p1, p2, color } => {
                    let min_x: f32 = p0.get_x().min(p1.get_x()).min(p2.get_x()) * scale;
                    let min_y: f32 = p0.get_y().min(p1.get_y()).min(p2.get_y()) * scale;
                    let max_x: f32 = p0.get_x().max(p1.get_x()).max(p2.get_x()) * scale;
                    let max_y: f32 = p0.get_y().max(p1.get_y()).max(p2.get_y()) * scale;
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            min_x,
                            min_y,
                            (max_x - min_x).max(1.0),
                            (max_y - min_y).max(1.0),
                        ],
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
                    self.push_instance(active_clip, inst);
                }
                DrawCommand::SpriteRegion { rect, tint, .. } => {
                    let snapped: Rect = snap_physical_rect(*rect, scale);
                    let inst: GpuQuadInstance = GpuQuadInstance {
                        rect: [
                            snapped.get_x(),
                            snapped.get_y(),
                            snapped.get_width(),
                            snapped.get_height(),
                        ],
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
                    self.push_instance(active_clip, inst);
                }
            }
        }
        let Self {
            driver,
            batches,
            readback,
            enable_readback,
        } = self;
        let submitted: usize = driver.submit_batches(phys_w, phys_h, clear_color, batches)?;
        let pixels: usize = if *enable_readback {
            let (_, px): (usize, usize) = readback.render(display_list, damage);
            px
        } else {
            (phys_w as usize) * (phys_h as usize)
        };
        Ok((submitted, pixels))
    }

    /// Appends `instance` to the tail `GpuBatch` if scissor matches and batch is below capacity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hardware renderer.
    /// - `Rect` - Active scissor rectangle.
    /// - `GpuQuadInstance` - Instance data to batch.
    fn push_instance(&mut self, scissor: Rect, instance: GpuQuadInstance) {
        if let Some(last) = self.get_mut_batches().last_mut()
            && last.get_scissor() == scissor
            && last.get_instances().len() < MAX_QUADS_PER_BATCH
        {
            last.get_mut_instances().push(instance);
            return;
        }
        let mut batch: GpuBatch = GpuBatch::default();
        batch.set_scissor(scissor);
        batch.get_mut_instances().push(instance);
        self.get_mut_batches().push(batch);
    }
}
