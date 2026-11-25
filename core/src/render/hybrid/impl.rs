use super::*;

/// Hybrid CPU-tile + GPU-composite rendering execution.
impl HybridRenderer {
    /// Creates a hybrid renderer targeting `gpu_backend` for final tile compositing.
    ///
    /// # Arguments
    ///
    /// - `GraphicsBackend` - GPU backend used for quad compositing.
    /// - `u32` - Initial physical width.
    /// - `u32` - Initial physical height.
    /// - `RenderConfig` - Rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized hybrid renderer.
    #[must_use]
    pub fn with_backend(
        gpu_backend: GraphicsBackend,
        width: u32,
        height: u32,
        config: RenderConfig,
    ) -> Self {
        let mut compositor: RecordingGpuDriver = RecordingGpuDriver::default();
        compositor.set_backend_kind(gpu_backend);
        Self {
            software: SoftwareRenderer::with_config(width, height, config),
            compositor,
        }
    }

    /// Rasterizes dirty tiles on CPU and submits a composite textured quad batch to the GPU driver.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hybrid renderer.
    /// - `&DisplayList` - Frame display list.
    /// - `DamageRegion` - Dirty damage bounds.
    ///
    /// # Returns
    ///
    /// - `Result<(usize, usize), PlatformError>` - Tuple of `(tiles_composited, pixels_shaded)`.
    pub fn render(
        &mut self,
        display_list: &DisplayList,
        damage: DamageRegion,
    ) -> Result<(usize, usize), PlatformError> {
        let (tiles, pixels): (usize, usize) = self.get_mut_software().render(display_list, damage);
        let w: u32 = self.get_software().get_framebuffer().get_width();
        let h: u32 = self.get_software().get_framebuffer().get_height();
        let full_rect: Rect = Rect::new(0.0, 0.0, w as f32, h as f32);
        let composite_quad: GpuQuadInstance = GpuQuadInstance {
            rect: [0.0, 0.0, w as f32, h as f32],
            clip: [0.0, 0.0, w as f32, h as f32],
            color: [1.0, 1.0, 1.0, 1.0],
            secondary_color: [0.0; 4],
            radii: [0.0; 4],
            stroke_or_blur: 0.0,
            angle_rad: 0.0,
            kind: GpuPrimitiveKind::TexturedQuad as u32,
            reserved: tiles as u32,
        };
        let mut batch: GpuBatch = GpuBatch::default();
        batch.set_scissor(full_rect);
        batch.get_mut_instances().push(composite_quad);
        let _: usize =
            self.get_mut_compositor()
                .submit_batches(w, h, COLOR_TRANSPARENT, &[batch])?;
        Ok((tiles.max(1), pixels))
    }
}
