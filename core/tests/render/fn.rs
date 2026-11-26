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

#[test]
fn transparent_canvas_hover_skips_gray_box_and_hardware_has_readback_framebuffer() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(240, 120).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: BuildContext = current_build_context();
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Px(240.0))
                .height(Length::Px(120.0))
                .background(COLOR_WHITE),
        );
        let canvas_node: NodeId = ctx.canvas(
            Style::empty()
                .width(Length::Px(180.0))
                .height(Length::Px(32.0))
                .cursor(CursorIcon::Pointer),
            |bounds: Rect, dl: &mut DisplayList| {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(
                        bounds.get_x(),
                        bounds.get_y() + 14.0,
                        bounds.get_width(),
                        4.0,
                    ),
                    color: Color::hex(0x2563EB),
                    radius: CornerRadius::all(2.0),
                    border: Border::default(),
                });
            },
        );
        ctx.append(root, canvas_node);
        root
    });
    let before: FrameStats = app.step_frame().unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(40.0, 16.0),
    ));
    let after: FrameStats = app.step_frame().unwrap();
    assert_eq!(before.get_frame_hash(), after.get_frame_hash());
    let hw_renderer: Renderer = Renderer::new(
        PlatformKind::Windows,
        PlatformCapabilities::for_platform(PlatformKind::Windows),
        240,
        120,
        RenderConfig::default(),
    );
    assert!(hw_renderer.framebuffer().is_some());
    assert!(WGSL_UI_SHADER.contains("border_a"));
    assert!(MSL_UI_SHADER.contains("border_a"));
    assert!(GLSL_ES3_UI_FRAGMENT_SHADER.contains("border_a"));
    assert!(HLSL_UI_SHADER.contains("border_a"));
    let cov_sdf: f32 = sdf_coverage(0.25, AntialiasingMode::AnalyticalSdf);
    let cov_msaa: f32 = sdf_coverage(0.25, AntialiasingMode::Msaa4x);
    let cov_ssaa: f32 = sdf_coverage(0.25, AntialiasingMode::Ssaa2x);
    assert!(cov_sdf > 0.0 && cov_msaa > 0.0 && cov_ssaa > 0.0);
    assert_ne!(cov_sdf.to_bits(), cov_ssaa.to_bits());
}

#[test]
fn lossless_multi_dpi_viewport_gaussian_shadow_and_supersampled_edges() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(411, 914).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: BuildContext = current_build_context();
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(16.0))
                .background(Color::hex(0xF8FAFC)),
        );
        let card: NodeId = ctx.view(
            Style::empty()
                .width(Length::Px(220.0))
                .height(Length::Px(96.0))
                .background(Color::hex(0xFFFFFF))
                .border(1.0, Color::hex(0xE2E8F0))
                .radius(12.0)
                .shadow(Shadow::new(0.0, 6.0, 20.0, 0.0, Color::new(15, 23, 42, 32))),
        );
        ctx.append(root, card);
        root
    });
    let dpi_cases: [(u32, u32, f32); 5] = [
        (1080, 2400, 2.625),
        (1366, 768, 1.25),
        (1920, 1080, 1.5),
        (1179, 2556, 3.0),
        (800, 600, 1.0),
    ];
    for (phys_w, phys_h, scale) in dpi_cases.into_iter() {
        app.set_exact_physical_viewport(phys_w, phys_h, scale);
        let stats: FrameStats = app.step_frame().unwrap();
        assert_ne!(stats.get_frame_hash(), 0);
        let fb: &Framebuffer = app.get_renderer().framebuffer().unwrap();
        assert_eq!(fb.get_width(), phys_w);
        assert_eq!(fb.get_height(), phys_h);
    }
    let f_neg: f32 = gaussian_shadow_falloff(-12.0, 4.0);
    let f_zero: f32 = gaussian_shadow_falloff(0.0, 4.0);
    let f_mid: f32 = gaussian_shadow_falloff(4.0, 4.0);
    let f_far: f32 = gaussian_shadow_falloff(12.0, 4.0);
    assert!(f_neg > 0.99);
    assert!((f_zero - 0.5).abs() < 0.01);
    assert!(f_mid > 0.10 && f_mid < 0.25);
    assert!(f_far < 0.01);
    assert!(f_neg > f_zero && f_zero > f_mid && f_mid > f_far);
    let (oc_tl, ic_tl): (f32, f32) =
        rounded_box_supersampled_coverage(-18.5, -18.5, 24.0, 24.0, 8.0, 1.5);
    let (oc_br, ic_br): (f32, f32) =
        rounded_box_supersampled_coverage(18.5, 18.5, 24.0, 24.0, 8.0, 1.5);
    assert!((oc_tl - oc_br).abs() < 0.001);
    assert!((ic_tl - ic_br).abs() < 0.001);
    assert!(oc_tl >= ic_tl);
    let tri_cov: f32 = triangle_supersampled_coverage(
        50.5,
        10.5,
        Point::new(0.0, 10.0),
        Point::new(100.0, 11.0),
        Point::new(50.0, 30.0),
    );
    assert!(tri_cov > 0.0 && tri_cov < 1.0);
    let snapped: Rect = snap_physical_rect(Rect::new(10.0, 10.0, 100.0, 40.0), 2.625);
    assert_eq!(snapped.get_x(), 26.0);
    assert_eq!(snapped.right(), 289.0);
    assert!(WGSL_UI_SHADER.contains("gaussian_falloff"));
    assert!(MSL_UI_SHADER.contains("euv_gaussian_falloff"));
    assert!(GLSL_ES3_UI_FRAGMENT_SHADER.contains("gaussian_falloff"));
    assert!(HLSL_UI_SHADER.contains("GaussianFalloff"));
}
