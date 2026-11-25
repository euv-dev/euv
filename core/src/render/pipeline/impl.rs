use super::*;

/// Multi-mode pipeline construction, live mode switching, and frame rendering for `Renderer`.
impl Renderer {
    /// Creates a `Renderer` resolving the optimal backend for `platform` and `config`.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Active execution platform.
    /// - `PlatformCapabilities` - Hardware capabilities reported by the platform backend.
    /// - `u32` - Initial physical width.
    /// - `u32` - Initial physical height.
    /// - `RenderConfig` - Requested rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized multi-mode renderer facade.
    #[must_use]
    pub fn create(
        platform: PlatformKind,
        capabilities: PlatformCapabilities,
        width: u32,
        height: u32,
        config: RenderConfig,
    ) -> Self {
        let (active_mode, active_backend): (RenderMode, GraphicsBackend) =
            Self::resolve_mode_and_backend(platform, capabilities, config);
        let readback_mirror: bool =
            platform == PlatformKind::Headless || active_backend == GraphicsBackend::Headless;
        Self {
            config,
            active_mode,
            active_backend,
            software: SoftwareRenderer::with_config(width, height, config),
            hardware: HardwareRenderer::with_backend(
                active_backend,
                width,
                height,
                config,
                readback_mirror,
            ),
            hybrid: HybridRenderer::with_backend(active_backend, width, height, config),
            display_list: DisplayList::default(),
            frame_index: 0,
            last_stats: FrameStats::default(),
        }
    }

    /// Resolves `(RenderMode, GraphicsBackend)` deterministically from platform capabilities and user preference.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Target platform.
    /// - `PlatformCapabilities` - Hardware support flags.
    /// - `RenderConfig` - User configuration.
    ///
    /// # Returns
    ///
    /// - `(RenderMode, GraphicsBackend)` - Concrete resolved mode and backend.
    #[must_use]
    pub fn resolve_mode_and_backend(
        platform: PlatformKind,
        capabilities: PlatformCapabilities,
        config: RenderConfig,
    ) -> (RenderMode, GraphicsBackend) {
        let default_gpu: GraphicsBackend = match platform {
            PlatformKind::MacOs | PlatformKind::Ios | PlatformKind::IpadOs => {
                GraphicsBackend::Metal
            }
            PlatformKind::Windows => GraphicsBackend::Direct3D12,
            PlatformKind::Linux | PlatformKind::Android => GraphicsBackend::Vulkan,
            PlatformKind::Web => GraphicsBackend::WebGpu,
            PlatformKind::Headless => GraphicsBackend::Headless,
        };
        let mode: RenderMode = match config.get_mode() {
            RenderMode::Auto => {
                if capabilities.get_hardware_gpu() {
                    RenderMode::Hardware
                } else {
                    RenderMode::Software
                }
            }
            explicit => explicit,
        };
        let backend: GraphicsBackend = match mode {
            RenderMode::Software => match platform {
                PlatformKind::Web => GraphicsBackend::Canvas2d,
                _ => GraphicsBackend::SoftwareCpu,
            },
            RenderMode::Hardware | RenderMode::Hybrid => {
                config.preferred_backend_value().unwrap_or(default_gpu)
            }
            RenderMode::Auto => default_gpu,
        };
        (mode, backend)
    }

    /// Dynamically switches the active rendering mode at runtime without rebuilding the scene tree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable renderer.
    /// - `PlatformKind` - Active platform kind.
    /// - `PlatformCapabilities` - Platform hardware capabilities.
    /// - `RenderMode` - Target render mode.
    pub fn switch_mode(
        &mut self,
        platform: PlatformKind,
        capabilities: PlatformCapabilities,
        mode: RenderMode,
    ) {
        let mut next_cfg: RenderConfig = self.get_config();
        next_cfg.set_mode(mode);
        let (resolved_mode, resolved_backend): (RenderMode, GraphicsBackend) =
            Self::resolve_mode_and_backend(platform, capabilities, next_cfg);
        self.set_config(next_cfg);
        self.set_active_mode(resolved_mode);
        self.set_active_backend(resolved_backend);
        self.get_mut_hardware()
            .get_mut_driver()
            .set_backend_kind(resolved_backend);
        self.get_mut_hybrid()
            .get_mut_compositor()
            .set_backend_kind(resolved_backend);
    }

    /// Compiles `tree` into a `DisplayList`, executes the active render backend, and returns `FrameStats`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable renderer.
    /// - `&SceneTree` - Laid-out scene tree.
    /// - `Size` - Logical viewport dimensions.
    /// - `f32` - Active DPI scale factor.
    /// - `Color` - Surface clear color.
    /// - `LayoutStats` - Telemetry from the layout pass.
    /// - `DamageRegion` - Damaged screen region.
    ///
    /// # Returns
    ///
    /// - `Result<FrameStats, PlatformError>` - Frame performance and pixel-hash telemetry.
    pub fn render_frame(
        &mut self,
        tree: &SceneTree,
        viewport: Size,
        scale_factor: f32,
        clear_color: Color,
        layout_stats: LayoutStats,
        damage: DamageRegion,
    ) -> Result<FrameStats, PlatformError> {
        PaintPass::build(
            tree,
            viewport,
            scale_factor,
            clear_color,
            self.get_mut_display_list(),
        );
        let cmd_count: usize = self.get_display_list().len();
        let active_mode: RenderMode = self.get_active_mode();
        let active_backend: GraphicsBackend = self.get_active_backend();
        let Self {
            software,
            hardware,
            hybrid,
            display_list,
            frame_index,
            last_stats,
            ..
        } = self;
        let (batches, pixels, hash): (usize, usize, u64) = match active_mode {
            RenderMode::Software => {
                let (b, p): (usize, usize) = software.render(display_list, damage);
                (b, p, software.get_framebuffer().fnv1a_hash())
            }
            RenderMode::Hybrid => {
                let (b, p): (usize, usize) = hybrid.render(display_list, damage)?;
                (b, p, hybrid.get_software().get_framebuffer().fnv1a_hash())
            }
            RenderMode::Hardware | RenderMode::Auto => {
                let (b, p): (usize, usize) = hardware.render(display_list, damage)?;
                let h: u64 = if hardware.get_enable_readback() {
                    hardware.get_readback().get_framebuffer().fnv1a_hash()
                } else {
                    0
                };
                (b, p, h)
            }
        };
        *frame_index = frame_index.wrapping_add(1);
        let stats: FrameStats = FrameStats {
            frame_index: *frame_index,
            active_mode,
            active_backend,
            layout_nodes: layout_stats.get_nodes_computed(),
            layout_skipped: layout_stats.get_subtrees_skipped(),
            draw_commands: cmd_count,
            draw_batches: batches,
            pixels_shaded: pixels,
            frame_hash: hash,
        };
        *last_stats = stats;
        Ok(stats)
    }

    /// Returns a reference to the latest CPU-accessible `Framebuffer` for software/hybrid/readback presentation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Renderer reference.
    ///
    /// # Returns
    ///
    /// - `Option<&Framebuffer>` - Active RGBA8 framebuffer if available in host memory.
    #[must_use]
    pub fn framebuffer(&self) -> Option<&Framebuffer> {
        match self.get_active_mode() {
            RenderMode::Software => Some(self.get_software().get_framebuffer()),
            RenderMode::Hybrid => Some(self.get_hybrid().get_software().get_framebuffer()),
            RenderMode::Hardware | RenderMode::Auto => {
                if self.get_hardware().get_enable_readback() {
                    Some(self.get_hardware().get_readback().get_framebuffer())
                } else {
                    None
                }
            }
        }
    }
}
