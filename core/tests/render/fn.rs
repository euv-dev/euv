use super::*;

#[test]
fn cross_platform_pixel_parity_across_all_seven_platforms_and_render_modes() {
    reset_reactive_runtime();
    let mut hashes: Vec<u64> = Vec::new();
    for platform in PlatformKind::all_targets().into_iter() {
        let backend: HeadlessPlatform =
            HeadlessPlatform::emulating(platform, 1.0, SafeAreaInsets::default());
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(240, 140).set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let mut app: App = App::with_platform(cfg, Box::new(backend)).unwrap();
        let _: NodeId = app.mount(|| {
            let mut ctx: BuildContext = current_build_context();
            let root: NodeId = ctx.view(
                Style::empty()
                    .column()
                    .width(Length::Px(240.0))
                    .height(Length::Px(140.0))
                    .padding(Edges::all(12.0))
                    .gap(8.0)
                    .background(Color::hex(0xF8FAFC)),
            );
            let card: NodeId = ctx.view(
                Style::empty()
                    .column()
                    .padding(Edges::all(10.0))
                    .background(Color::hex(0xFFFFFF))
                    .border(1.0, Color::hex(0xCBD5E1))
                    .radius(8.0),
            );
            let label: NodeId = ctx.text(
                "Cross-Platform Parity".to_string(),
                Style::empty()
                    .font_size(14.0)
                    .font_weight(FontWeight::Bold)
                    .color(Color::hex(0x0F172A)),
            );
            ctx.append(card, label);
            ctx.append(root, card);
            root
        });
        let sw_stats: FrameStats = app.step_frame().unwrap();
        assert_eq!(sw_stats.get_active_mode(), RenderMode::Software);
        assert_ne!(sw_stats.get_frame_hash(), 0);
        app.set_render_mode(RenderMode::Hybrid);
        let hybrid_stats: FrameStats = app.step_frame().unwrap();
        assert_eq!(hybrid_stats.get_active_mode(), RenderMode::Hybrid);
        assert_eq!(sw_stats.get_frame_hash(), hybrid_stats.get_frame_hash());
        hashes.push(sw_stats.get_frame_hash());
    }
    let expected: u64 = hashes[0];
    for hash in hashes.into_iter() {
        assert_eq!(hash, expected);
    }
}

#[test]
fn hardware_renderer_batches_primitives_and_selects_native_shaders() {
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let box_a: NodeId = tree.create_view(
        Style::empty()
            .width(Length::Px(80.0))
            .height(Length::Px(40.0))
            .background(Color::hex(0x2563EB))
            .radius(6.0),
    );
    let box_b: NodeId = tree.create_view(
        Style::empty()
            .width(Length::Px(80.0))
            .height(Length::Px(40.0))
            .gradient(LinearGradient::new(
                90.0,
                Color::hex(0x10B981),
                Color::hex(0x3B82F6),
            )),
    );
    tree.append_child(root, box_a);
    tree.append_child(root, box_b);
    let mut layout: LayoutEngine = LayoutEngine::default();
    let lstats: LayoutStats = layout.compute(&mut tree, Size::new(200.0, 120.0));
    let mut renderer: Renderer = Renderer::new(
        PlatformKind::MacOs,
        PlatformCapabilities::for_platform(PlatformKind::MacOs),
        200,
        120,
        RenderConfig::hardware(),
    );
    let stats: FrameStats = renderer
        .render_frame(
            &tree,
            Size::new(200.0, 120.0),
            1.0,
            COLOR_WHITE,
            lstats,
            DamageRegion::full(),
        )
        .unwrap();
    assert_eq!(stats.get_active_mode(), RenderMode::Hardware);
    assert_eq!(stats.get_active_backend(), GraphicsBackend::Metal);
    assert!(stats.get_draw_batches() >= 1);
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(
        renderer
            .get_hardware()
            .get_driver()
            .shader_source()
            .contains("metal_stdlib")
    );
}

#[test]
fn app_run_frames_steps_multiple_frames_and_exits_on_close_event() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(200, 120).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: BuildContext = current_build_context();
        ctx.text("Multi-frame loop".to_string(), Style::empty())
    });
    let stats: FrameStats = app.run_frames(Some(3)).unwrap();
    assert_eq!(stats.get_frame_index(), 3);
    assert_ne!(stats.get_frame_hash(), 0);
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::CloseRequested(wid));
    assert!(!app.get_running());
    let closed_stats: FrameStats = app.run_frames(None).unwrap();
    assert_eq!(closed_stats.get_frame_index(), 0);
}

#[test]
fn native_system_font_and_unicode_cjk_atlas_packing_work() {
    let sys_font: NativeSystemFont = NativeSystemFont::system();
    let info: SystemFontInfo = sys_font.font_info();
    assert!(!info.get_family_name().is_empty());
    assert!(info.get_supports_unicode());
    let mut atlas: GlyphAtlas = GlyphAtlas::default();
    let key_ascii: GlyphKey = GlyphKey::new('A', 180, false);
    let key_cjk_a: GlyphKey = GlyphKey::new('中', 180, false);
    let key_cjk_b: GlyphKey = GlyphKey::new('国', 180, false);
    let g_ascii: RasterizedGlyph = atlas.get_or_rasterize_with(key_ascii, &sys_font).clone();
    let g_cjk_a: RasterizedGlyph = atlas.get_or_rasterize_with(key_cjk_a, &sys_font).clone();
    let g_cjk_b: RasterizedGlyph = atlas.get_or_rasterize_with(key_cjk_b, &sys_font).clone();
    assert!(g_ascii.get_coverage().iter().any(|px: &u8| *px > 0));
    assert!(g_cjk_a.get_coverage().iter().any(|px: &u8| *px > 0));
    assert!(g_cjk_b.get_coverage().iter().any(|px: &u8| *px > 0));
    assert_ne!(g_cjk_a.get_coverage(), g_cjk_b.get_coverage());
    let packed: PackedGlyphAtlas = atlas.pack_rgba_atlas();
    assert_eq!(packed.get_regions().len(), 3);
    assert!(packed.get_pixels().iter().any(|px: &u8| *px > 0));
    assert!(is_wide_unicode('中'));
    assert!(!is_wide_unicode('A'));
}

#[test]
fn native_gpu_driver_probes_hardware_compiles_shaders_and_submits_batches() {
    let mut driver: NativeGpuDriver = NativeGpuDriver::probe(GraphicsBackend::Metal);
    let info: GpuAdapterInfo = driver.get_adapter().clone();
    assert!(!info.get_device_name().is_empty());
    let quad: GpuQuadInstance = GpuQuadInstance {
        rect: [10.0, 10.0, 120.0, 48.0],
        clip: [0.0, 0.0, 800.0, 600.0],
        color: [0.2, 0.5, 0.95, 1.0],
        secondary_color: [0.1, 0.2, 0.4, 1.0],
        radii: [8.0, 8.0, 8.0, 8.0],
        stroke_or_blur: 1.0,
        angle_rad: 0.0,
        kind: 0,
        reserved: 0,
    };
    let mut batch: GpuBatch = GpuBatch::default();
    batch.set_scissor(Rect::new(0.0, 0.0, 800.0, 600.0));
    batch.set_instances(vec![quad]);
    let calls: usize = driver
        .submit_batches(800, 600, COLOR_WHITE, &[batch])
        .unwrap();
    assert_eq!(calls, 1);
    assert_eq!(driver.get_submitted_frames(), 1);
    assert_eq!(driver.get_uploaded_instances(), 1);
    #[cfg(target_os = "macos")]
    {
        if info.get_hardware_verified() {
            assert!(info.get_shader_compiled());
        }
    }
}
