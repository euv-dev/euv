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
            let mut ctx: Context = current_context();
            let root: NodeId = ctx.view(
                Style::default()
                    .column()
                    .width(Length::Px(240.0))
                    .height(Length::Px(140.0))
                    .padding(Edges::all(12.0))
                    .gap(8.0)
                    .background(Color::hex(0xF8FAFC)),
            );
            let card: NodeId = ctx.view(
                Style::default()
                    .column()
                    .padding(Edges::all(10.0))
                    .background(Color::hex(0xFFFFFF))
                    .border(1.0, Color::hex(0xCBD5E1))
                    .radius(8.0),
            );
            let label: NodeId = ctx.text(
                "Cross-Platform Parity".to_string(),
                Style::default()
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
        Style::default()
            .width(Length::Px(80.0))
            .height(Length::Px(40.0))
            .background(Color::hex(0x2563EB))
            .radius(6.0),
    );
    let box_b: NodeId = tree.create_view(
        Style::default()
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
        let mut ctx: Context = current_context();
        ctx.text("Multi-frame loop".to_string(), Style::default())
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
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(240.0))
                .height(Length::Px(120.0))
                .background(COLOR_WHITE),
        );
        let canvas_node: NodeId = ctx.canvas(
            Style::default()
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
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(16.0))
                .background(Color::hex(0xF8FAFC)),
        );
        let card: NodeId = ctx.view(
            Style::default()
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
    for sample_idx in -30_i32..=30_i32 {
        let dist: f32 = (sample_idx as f32) * 0.4;
        let exact: f32 = gaussian_shadow_falloff(dist, 4.0);
        let fast: f32 = fast_gaussian_shadow_falloff(dist, 0.25);
        assert!((exact - fast).abs() < 0.002);
    }
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

#[test]
fn translucent_rounded_rect_has_uniform_interior_and_no_nested_hover_tint() {
    reset_reactive_runtime();
    let mut renderer: SoftwareRenderer =
        SoftwareRenderer::with_config(240, 120, RenderConfig::software());
    let mut dl: DisplayList = DisplayList::default();
    dl.reset(Size::new(240.0, 120.0), 1.0);
    dl.push(DrawCommand::Clear(COLOR_WHITE));
    dl.push(DrawCommand::FillRoundedRect {
        rect: Rect::new(20.0, 20.0, 200.0, 44.0),
        color: Color::rgba(113, 113, 122, 42),
        radius: CornerRadius::all(8.0),
        border: Border::default(),
    });
    let _: (usize, usize) = renderer.render(&dl, DamageRegion::full());
    let read_px: fn(&Framebuffer, u32, u32) -> Color =
        |fb: &Framebuffer, x: u32, y: u32| -> Color {
            let idx: usize = ((y as usize) * (fb.get_width() as usize) + (x as usize)) * 4;
            let p: &[u8] = fb.get_pixels().as_slice();
            Color::new(p[idx], p[idx + 1], p[idx + 2], p[idx + 3])
        };
    let fb: &Framebuffer = renderer.get_framebuffer();
    let px_top: Color = read_px(fb, 120, 23);
    let px_mid: Color = read_px(fb, 120, 42);
    let px_bot: Color = read_px(fb, 120, 60);
    assert_eq!(px_top.get_r(), px_mid.get_r());
    assert_eq!(px_top.get_g(), px_mid.get_g());
    assert_eq!(px_top.get_b(), px_mid.get_b());
    assert_eq!(px_bot.get_r(), px_mid.get_r());
    assert_eq!(px_bot.get_g(), px_mid.get_g());
    assert_eq!(px_bot.get_b(), px_mid.get_b());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 200).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let mut ids: (NodeId, NodeId, NodeId) =
        (NodeId::default(), NodeId::default(), NodeId::default());
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .background(COLOR_WHITE),
        );
        let scrim: NodeId = ctx.portal(
            Style::default()
                .position(Position::Fixed)
                .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .background(Color::rgba(0, 0, 0, 120)),
        );
        ctx.on_click(scrim, |_: &UiEvent| {});
        let outer: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(220.0))
                .height(Length::Px(120.0))
                .padding(Edges::all(16.0))
                .background(COLOR_WHITE),
        );
        ctx.on_click(outer, |_: &UiEvent| {});
        let inner: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(160.0))
                .height(Length::Px(40.0))
                .radius(6.0)
                .background(COLOR_TRANSPARENT),
        );
        ctx.on_click(inner, |_: &UiEvent| {});
        ctx.append(outer, inner);
        ctx.append(scrim, outer);
        ctx.append(root, scrim);
        ids = (scrim, outer, inner);
        root
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    let baseline_scrim: Color = read_px(app.get_renderer().framebuffer().unwrap(), 280, 100);
    let baseline_outer: Color = read_px(app.get_renderer().framebuffer().unwrap(), 200, 100);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(60.0, 32.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let hovered_scrim: Color = read_px(app.get_renderer().framebuffer().unwrap(), 280, 100);
    let hovered_outer: Color = read_px(app.get_renderer().framebuffer().unwrap(), 200, 100);
    let hovered_inner: Color = read_px(app.get_renderer().framebuffer().unwrap(), 60, 32);
    assert_eq!(baseline_scrim, hovered_scrim);
    assert_eq!(baseline_outer, hovered_outer);
    assert_ne!(hovered_inner, COLOR_WHITE);
    let _: (NodeId, NodeId, NodeId) = ids;
}

#[test]
fn touch_scroll_thumb_lift_reversal_never_flings_backward() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 640).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let mut scroll_id: NodeId = NodeId::default();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let sc: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .gap(8.0)
                .background(COLOR_WHITE),
        );
        for _ in 0..40 {
            let item: NodeId = ctx.view(
                Style::default()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(56.0))
                    .background(Color::hex(0xF1F5F9)),
            );
            ctx.append(sc, item);
        }
        scroll_id = sc;
        sc
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, 180.0, 500.0),
    ));
    let mut y: f32 = 500.0;
    for _ in 0..8 {
        y -= 28.0;
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::touch(1, 180.0, y),
        ));
        let _: FrameStats = app.step_game_frame(1.0 / 120.0).unwrap();
    }
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(1, 180.0, y + 3.5),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(1, 180.0, y + 3.5),
    ));
    assert!(app.get_scroll_velocity().get_y() > 0.0);
    let before_inertia: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(0.0)
        },
        0.0,
    );
    let _: FrameStats = app.step_game_frame(1.0 / 120.0).unwrap();
    let after_inertia: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(0.0)
        },
        0.0,
    );
    assert!(after_inertia > before_inertia);
}

#[test]
fn keyboard_tab_cycling_and_enter_activation_drive_focus_and_scroll() {
    reset_reactive_runtime();
    let clicks_a: Signal<u32> = Signal::new(0);
    let clicks_b: Signal<u32> = Signal::new(0);
    let mut scroll_id: NodeId = NodeId::default();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(400, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let sc: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .gap(8.0)
                .background(COLOR_WHITE),
        );
        let btn_a: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(120.0))
                .height(Length::Px(36.0))
                .cursor(CursorIcon::Pointer),
        );
        ctx.on_click(btn_a, move |_: &UiEvent| {
            let _: bool = clicks_a.update(|v: &mut u32| *v += 1);
        });
        let btn_b: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(120.0))
                .height(Length::Px(36.0))
                .cursor(CursorIcon::Pointer),
        );
        ctx.on_click(btn_b, move |_: &UiEvent| {
            let _: bool = clicks_b.update(|v: &mut u32| *v += 1);
        });
        ctx.append(sc, btn_a);
        ctx.append(sc, btn_b);
        for _ in 0..20 {
            let filler: NodeId = ctx.view(
                Style::default()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(48.0)),
            );
            ctx.append(sc, filler);
        }
        scroll_id = sc;
        sc
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Tab, Modifiers::default(), false),
    ));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Enter, Modifiers::default(), false),
    ));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Tab, Modifiers::default(), false),
    ));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Space, Modifiers::default(), false),
    ));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::PageDown, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(clicks_a.get_untracked(), 1);
    assert_eq!(clicks_b.get_untracked(), 1);
    let scrolled_y: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(0.0)
        },
        0.0,
    );
    assert!(scrolled_y > 0.0);
}

#[test]
fn layout_single_pass_and_translation_cache_skip_clean_sibling_subtrees() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let page: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Px(960.0))
            .padding(Edges::all(12.0))
            .gap(8.0),
    );
    let mut target_leaf: NodeId = NodeId::default();
    for g_idx in 0..20usize {
        let grid: NodeId = ctx.view(
            Style::default()
                .grid(4)
                .width(Length::Percent(100.0))
                .gap(8.0),
        );
        for c_idx in 0..4usize {
            let card: NodeId = ctx.view(
                Style::default()
                    .column()
                    .width(Length::Percent(100.0))
                    .padding(Edges::all(8.0))
                    .gap(4.0),
            );
            let txt: NodeId = ctx.text(
                format!("Card {g_idx}-{c_idx}"),
                Style::default().font_size(13.0),
            );
            ctx.append(card, txt);
            ctx.append(grid, card);
            if g_idx == 10 && c_idx == 0 {
                target_leaf = txt;
            }
        }
        ctx.append(page, grid);
    }
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![page]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let viewport: Size = Size::new(960.0, 640.0);
    let init_stats: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, viewport),
        LayoutStats::default(),
    );
    assert!(init_stats.get_nodes_computed() < 400);
    let inc_stats: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| {
            tree.set_text(target_leaf, "Updated Leaf".to_string());
            engine.compute(tree, viewport)
        },
        LayoutStats::default(),
    );
    assert!(inc_stats.get_subtrees_skipped() >= 19);
    assert!(inc_stats.get_nodes_computed() <= 16);
}

#[test]
fn aspect_ratio_and_auto_height_centered_column_keep_children_visible_inside_bounds() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let page: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Px(360.0))
            .padding(Edges::all(16.0))
            .gap(12.0),
    );
    let auto_stage: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Percent(100.0))
            .padding(Edges::all(20.0))
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .overflow(Overflow::Hidden),
    );
    let stage_txt: NodeId = ctx.text("Slide 1".to_string(), Style::default().font_size(14.0));
    ctx.append(auto_stage, stage_txt);
    let aspect: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Percent(100.0))
            .aspect_ratio(16.0 / 9.0)
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
            .overflow(Overflow::Hidden),
    );
    let aspect_txt: NodeId = ctx.text("16:9 Ratio".to_string(), Style::default().font_size(14.0));
    ctx.append(aspect, aspect_txt);
    ctx.append(page, auto_stage);
    ctx.append(page, aspect);
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![page]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, Size::new(360.0, 900.0)),
        LayoutStats::default(),
    );
    let (sb, stb, ab, atb): (Rect, Rect, Rect, Rect) = scene.with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(auto_stage)
                    .unwrap()
                    .get_layout_box()
                    .get_bounds(),
                tree.get_node(stage_txt)
                    .unwrap()
                    .get_layout_box()
                    .get_bounds(),
                tree.get_node(aspect).unwrap().get_layout_box().get_bounds(),
                tree.get_node(aspect_txt)
                    .unwrap()
                    .get_layout_box()
                    .get_bounds(),
            )
        },
        (
            Rect::default(),
            Rect::default(),
            Rect::default(),
            Rect::default(),
        ),
    );
    assert!(stb.get_y() >= sb.get_y() && stb.bottom() <= sb.bottom() + 1.0);
    assert!(atb.get_y() >= ab.get_y() && atb.bottom() <= ab.bottom() + 1.0);
    assert!((ab.get_height() - ab.get_width() / (16.0 / 9.0)).abs() < 1.5);
}

#[test]
fn tap_jitter_and_right_edge_touch_cause_zero_scroll_drift_and_ime_focus_syncs() {
    reset_reactive_runtime();
    let clicks: Signal<u32> = Signal::new(0);
    let mut scroll_id: NodeId = NodeId::default();
    let mut btn_id: NodeId = NodeId::default();
    let mut inp_id: NodeId = NodeId::default();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 640).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let sc: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(16.0))
                .gap(12.0)
                .background(COLOR_WHITE),
        );
        let inp: NodeId = ctx.input(
            InputNodeState::new(
                String::new(),
                "Enter text...".to_string(),
                0,
                false,
                false,
                false,
            ),
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Px(40.0)),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Px(44.0))
                .cursor(CursorIcon::Pointer)
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            let _: bool = clicks.update(|c: &mut u32| *c += 1);
        });
        ctx.append(sc, inp);
        ctx.append(sc, btn);
        for _ in 0..25 {
            let row: NodeId = ctx.view(
                Style::default()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(56.0))
                    .background(Color::hex(0xF1F5F9)),
            );
            ctx.append(sc, row);
        }
        scroll_id = sc;
        btn_id = btn;
        inp_id = inp;
        sc
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    let btn_rect: Rect = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(btn_id)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                .unwrap_or_default()
        },
        Rect::default(),
    );
    let bx: f32 = btn_rect.get_x() + btn_rect.get_width() * 0.5;
    let by: f32 = btn_rect.get_y() + btn_rect.get_height() * 0.5;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, bx, by),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(1, bx + 4.0, by + 6.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(1, bx + 4.0, by + 6.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let scroll_after_tap: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    assert_eq!(clicks.get_untracked(), 1);
    assert!(
        scroll_after_tap.abs() < 0.01,
        "tap with 7.2dp jitter must cause 0px drift, got {scroll_after_tap}"
    );
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(2, 355.0, 420.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(2, 355.0, 420.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let scroll_after_edge_tap: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    assert!(
        scroll_after_edge_tap.abs() < 0.01,
        "right-edge touch tap must not teleport scroll offset, got {scroll_after_edge_tap}"
    );
    let inp_rect: Rect = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(inp_id)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                .unwrap_or_default()
        },
        Rect::default(),
    );
    let ix: f32 = inp_rect.get_x() + inp_rect.get_width() * 0.5;
    let iy: f32 = inp_rect.get_y() + inp_rect.get_height() * 0.5;
    assert!(!app.is_ime_active());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(3, ix, iy),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::touch(3, ix, iy)));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(app.is_ime_active());
    app.dispatch_window_event(WindowEvent::TextInput(wid, "ok".to_string()));
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(4, bx, by),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::touch(4, bx, by)));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!app.is_ime_active());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(5, ix, iy),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::touch(5, ix, iy)));
    assert!(app.is_ime_active());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Escape, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!app.is_ime_active());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(6, ix, iy),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::touch(6, ix, iy)));
    assert!(app.is_ime_active());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Enter, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!app.is_ime_active());
}

#[test]
fn portal_base_scene_cache_matches_uncached_full_redraw_bit_identically() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let mut panel_id: NodeId = NodeId::default();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(16.0))
                .gap(12.0)
                .background(Color::hex(0xF8FAFC)),
        );
        for idx in 0..4usize {
            let card: NodeId = ctx.view(
                Style::default()
                    .column()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(72.0))
                    .padding(Edges::all(12.0))
                    .background(COLOR_WHITE)
                    .radius(12.0)
                    .shadow(Shadow::new(
                        0.0,
                        4.0,
                        12.0,
                        0.0,
                        Color::rgba(15, 23, 42, 32),
                    )),
            );
            let txt: NodeId = ctx.text(
                format!("Base Card #{idx}"),
                Style::default().font_size(14.0).color(Color::hex(0x0F172A)),
            );
            ctx.append(card, txt);
            ctx.append(root, card);
        }
        let scrim: NodeId = ctx.portal(
            Style::default()
                .position(Position::Fixed)
                .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .z_index(100)
                .background(Color::rgba(15, 23, 42, 110)),
        );
        let panel: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(220.0))
                .height(Length::Percent(100.0))
                .background(COLOR_WHITE)
                .shadow(Shadow::new(
                    4.0,
                    0.0,
                    16.0,
                    0.0,
                    Color::rgba(15, 23, 42, 45),
                )),
        );
        let ptxt: NodeId = ctx.text(
            "Drawer Panel".to_string(),
            Style::default().font_size(15.0).color(Color::hex(0x0F172A)),
        );
        ctx.append(panel, ptxt);
        ctx.append(scrim, panel);
        ctx.append(root, scrim);
        panel_id = panel;
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.start_transition(
                NodeTransition::slide_fade(
                    panel_id,
                    Point::new(-220.0, 0.0),
                    Point::default(),
                    1.0,
                    1.0,
                    0.24,
                    false,
                )
                .with_easing(Easing::EaseOutQuint),
            );
        },
        (),
    );
    let cached_stats: FrameStats = app.step_game_frame(0.08).unwrap();
    let mut fresh_list: DisplayList = DisplayList::default();
    app.get_scene().with_read(
        |tree: &SceneTree| {
            PaintPass::build(
                tree,
                Size::new(320.0, 480.0),
                1.0,
                COLOR_WHITE,
                &mut fresh_list,
            );
        },
        (),
    );
    let mut fresh_sw: SoftwareRenderer =
        SoftwareRenderer::with_config(320, 480, RenderConfig::software());
    let _: (usize, usize) = fresh_sw.render(&fresh_list, DamageRegion::full());
    let fresh_hash: u64 = fresh_sw.get_framebuffer().fnv1a_hash();
    assert_eq!(
        cached_stats.get_frame_hash(),
        fresh_hash,
        "portal base-scene backdrop cache must remain 100% bit-identical to uncached rasterization"
    );
}
