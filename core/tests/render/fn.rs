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

#[test]
fn focused_input_retap_reopens_ime_after_keyboard_dismissal_without_blurring() {
    reset_reactive_runtime();
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
                "hello".to_string(),
                "Type here".to_string(),
                5,
                false,
                false,
                false,
            ),
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Px(42.0)),
        );
        ctx.append(sc, inp);
        for _ in 0..20 {
            let filler: NodeId = ctx.view(
                Style::default()
                    .width(Length::Percent(100.0))
                    .height(Length::Px(48.0)),
            );
            ctx.append(sc, filler);
        }
        inp_id = inp;
        sc
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    let rect: Rect = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(inp_id)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                .unwrap_or_default()
        },
        Rect::default(),
    );
    let ix: f32 = rect.get_x() + rect.get_width() * 0.5;
    let iy: f32 = rect.get_y() + rect.get_height() * 0.5;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, ix, iy),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::touch(1, ix, iy)));
    assert!(app.is_ime_active());
    app.dispatch_window_event(WindowEvent::Ime(wid, ImeEvent::Disabled));
    assert!(!app.is_ime_active());
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!app.is_ime_active());
    let still_focused: bool = app.get_scene().with_read(
        |tree: &SceneTree| tree.get_focus().focused_node() == Some(inp_id),
        false,
    );
    assert!(still_focused);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(2, ix, iy),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(2, ix, iy + 36.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(2, ix, iy + 36.0),
    ));
    assert!(!app.is_ime_active());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(3, ix, iy),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::touch(3, ix, iy)));
    assert!(app.is_ime_active());
    app.dispatch_window_event(WindowEvent::Ime(wid, ImeEvent::Disabled));
    assert!(!app.is_ime_active());
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!app.is_ime_active());
    app.dispatch_window_event(WindowEvent::PointerDown(wid, PointerEvent::mouse(ix, iy)));
    assert!(app.is_ime_active());
}

#[test]
fn rounded_overflow_hidden_clips_child_corners_inside_parent_border() {
    reset_reactive_runtime();
    let bg_blue: Color = Color::hex(0x0000FF);
    let border_dark: Color = Color::hex(0x111111);
    let child_red: Color = Color::hex(0xFF0000);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(200, 120).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    cfg.set_clear_color(bg_blue);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(200.0))
                .height(Length::Px(120.0))
                .padding(Edges::all(20.0))
                .background(bg_blue),
        );
        let card: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(160.0))
                .height(Length::Px(80.0))
                .background(COLOR_WHITE)
                .border(2.0, border_dark)
                .radius(20.0)
                .overflow(Overflow::Hidden),
        );
        let fill_child: NodeId = ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .background(child_red),
        );
        ctx.append(card, fill_child);
        ctx.append(root, card);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let mut dl: DisplayList = DisplayList::default();
    app.get_scene().with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, Size::new(200.0, 120.0), 1.0, bg_blue, &mut dl);
        },
        (),
    );
    let has_rounded_clip: bool = dl
        .get_commands()
        .iter()
        .any(|c: &DrawCommand| matches!(c, DrawCommand::PushRoundedClip { .. }));
    assert!(has_rounded_clip);
    let fb: &Framebuffer = app.get_renderer().framebuffer().unwrap();
    let outer_corner: Color = fb.get_pixel(21, 21).unwrap();
    assert_eq!(outer_corner, bg_blue);
    let inner_center: Color = fb.get_pixel(100, 60).unwrap();
    assert_eq!(inner_center, child_red);
    let border_arc: Color = fb.get_pixel(26, 26).unwrap();
    assert!(
        border_arc.get_r() < 80,
        "child fill must not bleed over parent rounded border arc: got {border_arc:?}"
    );
}

#[test]
fn native_display_refresh_rate_updates_context_and_telemetry_above_60hz() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    assert!(app.display_refresh_rate_hz() >= 60.0);
    app.update_display_refresh_rate_hz(180.0);
    assert_eq!(app.display_refresh_rate_hz(), 180.0);
    assert_eq!(
        app.get_context().get_display_refresh_rate().get_untracked(),
        180.0
    );
    assert_eq!(app.get_context().get_live_fps().get_untracked(), 180.0);
    app.report_host_refresh_rate(144.0);
    assert_eq!(app.display_refresh_rate_hz(), 144.0);
    assert_eq!(app.get_context().get_live_fps().get_untracked(), 144.0);
    app.set_fps_limit(Some(120));
    app.report_host_refresh_rate(180.0);
    assert_eq!(app.display_refresh_rate_hz(), 180.0);
    assert_eq!(app.get_context().get_live_fps().get_untracked(), 120.0);
}

#[test]
fn fractional_scroll_keeps_card_height_and_text_offset_strictly_invariant_across_dpis() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let sc: NodeId = ctx.scroll_view(
        Style::default()
            .column()
            .width(Length::Px(360.0))
            .height(Length::Px(420.0))
            .padding(Edges::all(13.0))
            .gap(11.0)
            .background(COLOR_WHITE),
    );
    let card_bg: Color = Color::hex(0xE2E8F0);
    let card_a: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Percent(100.0))
            .height(Length::Px(63.4))
            .padding(Edges::symmetric(9.3, 12.0))
            .background(card_bg)
            .border(1.0, Color::hex(0x94A3B8))
            .radius(10.0),
    );
    let title_a: NodeId = ctx.text(
        "Coherent Scroll Card".to_string(),
        Style::default().font_size(14.5).color(COLOR_BLACK),
    );
    ctx.append(card_a, title_a);
    ctx.append(sc, card_a);
    for idx in 0..16u32 {
        let row: NodeId = ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Px(54.0))
                .background(Color::hex(0xF8FAFC)),
        );
        let lbl: NodeId = ctx.text(
            format!("Row #{idx}"),
            Style::default().font_size(13.0).color(COLOR_BLACK),
        );
        ctx.append(row, lbl);
        ctx.append(sc, row);
    }
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![sc]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let vp: Size = Size::new(360.0, 420.0);
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, vp),
        LayoutStats::default(),
    );
    let scales: [f32; 6] = [1.0, 1.25, 1.5, 2.0, 2.625, 3.0];
    let mut dl: DisplayList = DisplayList::default();
    for scale in scales.into_iter() {
        let mut expected_card_h: Option<i32> = None;
        let mut expected_text_rel_y: Option<i32> = None;
        let mut prev_card_y: Option<i32> = None;
        for step in 0..40u32 {
            let sy: f32 = (step as f32) * 0.73;
            scene.with_write(
                |tree: &mut SceneTree| {
                    tree.set_scroll_offset(sc, Point::new(0.0, sy));
                },
                (),
            );
            scene.with_read(
                |tree: &SceneTree| {
                    PaintPass::build(tree, vp, scale, COLOR_WHITE, &mut dl);
                },
                (),
            );
            let mut card_rect: Option<Rect> = None;
            let mut text_rect: Option<Rect> = None;
            for cmd in dl.get_commands().iter() {
                match cmd {
                    DrawCommand::FillRoundedRect { rect, color, .. } if *color == card_bg => {
                        card_rect = Some(snap_physical_rect(*rect, scale));
                    }
                    DrawCommand::TextRun { rect, text, .. } if text == "Coherent Scroll Card" => {
                        text_rect = Some(*rect);
                    }
                    _ => {}
                }
            }
            let cr: Rect = card_rect.expect("expected card draw command");
            let tr: Rect = text_rect.expect("expected text draw command");
            let orig_text_b: Rect = scene.with_read(
                |tree: &SceneTree| {
                    tree.get_node(title_a)
                        .unwrap()
                        .get_layout_box()
                        .get_content_bounds()
                },
                Rect::default(),
            );
            assert!((tr.get_width() - orig_text_b.get_width()).abs() < 1e-4);
            assert!((tr.get_height() - orig_text_b.get_height()).abs() < 1e-4);
            let card_y: i32 = cr.get_y().round() as i32;
            let card_h: i32 = cr.get_height().round() as i32;
            let text_y: i32 = (tr.get_y() * scale).round() as i32;
            if let Some(exp_h) = expected_card_h {
                assert_eq!(
                    card_h, exp_h,
                    "card height shimmered at scale={scale} step={step}: {card_h} != {exp_h}"
                );
            } else {
                expected_card_h = Some(card_h);
            }
            let rel_y: i32 = text_y - card_y;
            if let Some(exp_rel) = expected_text_rel_y {
                assert_eq!(
                    rel_y, exp_rel,
                    "text drifted relative to card at scale={scale} step={step}: {rel_y} != {exp_rel}"
                );
            } else {
                expected_text_rel_y = Some(rel_y);
            }
            if let Some(py) = prev_card_y {
                assert!(
                    card_y <= py,
                    "card jumped backwards during downward scroll at scale={scale} step={step}: {card_y} > {py}"
                );
            }
            prev_card_y = Some(card_y);
        }
    }
}

#[test]
fn hierarchical_transform_propagates_to_children_and_hit_test_in_pixel_lockstep() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let panel_bg: Color = Color::hex(0x1E293B);
    let btn_bg: Color = Color::hex(0x2563EB);
    let panel: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Px(220.0))
            .height(Length::Px(140.0))
            .padding(Edges::all(16.0))
            .background(panel_bg),
    );
    let btn: NodeId = ctx.view(
        Style::default()
            .width(Length::Px(120.0))
            .height(Length::Px(40.0))
            .background(btn_bg),
    );
    let btn_txt: NodeId = ctx.text(
        "Slide Action".to_string(),
        Style::default().font_size(14.0).color(COLOR_WHITE),
    );
    ctx.append(btn, btn_txt);
    ctx.append(panel, btn);
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![panel]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let vp: Size = Size::new(600.0, 400.0);
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, vp),
        LayoutStats::default(),
    );
    let scale: f32 = 2.0;
    let mut dl_base: DisplayList = DisplayList::default();
    scene.with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, vp, scale, COLOR_WHITE, &mut dl_base);
        },
        (),
    );
    let extract_positions: &dyn Fn(&DisplayList) -> (i32, i32, i32) =
        &|dl: &DisplayList| -> (i32, i32, i32) {
            let mut px: i32 = 0;
            let mut bx: i32 = 0;
            let mut tx: i32 = 0;
            for cmd in dl.get_commands().iter() {
                match cmd {
                    DrawCommand::FillRoundedRect { rect, color, .. } if *color == panel_bg => {
                        px = snap_physical_rect(*rect, scale).get_x().round() as i32;
                    }
                    DrawCommand::FillRoundedRect { rect, color, .. } if *color == btn_bg => {
                        bx = snap_physical_rect(*rect, scale).get_x().round() as i32;
                    }
                    DrawCommand::TextRun { rect, text, .. } if text == "Slide Action" => {
                        tx = (rect.get_x() * scale).round() as i32;
                    }
                    _ => {}
                }
            }
            (px, bx, tx)
        };
    let (p0, b0, t0): (i32, i32, i32) = extract_positions(&dl_base);
    let shift_dp: f32 = 143.7;
    scene.with_write(
        |tree: &mut SceneTree| {
            if let Some(n) = tree.get_node_mut(panel) {
                let mut st: Style = n.get_style();
                st.get_mut_paint()
                    .set_transform(Transform2D::new(shift_dp, 18.2, 1.0, 1.0, 0.0));
                n.set_style(st);
            }
            tree.mark_dirty(panel, DIRTY_PAINT);
        },
        (),
    );
    let mut dl_shifted: DisplayList = DisplayList::default();
    scene.with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, vp, scale, COLOR_WHITE, &mut dl_shifted);
        },
        (),
    );
    let (p1, b1, t1): (i32, i32, i32) = extract_positions(&dl_shifted);
    let expected_dx: i32 = (shift_dp * scale).round() as i32;
    assert_eq!(p1 - p0, expected_dx);
    assert_eq!(b1 - b0, expected_dx);
    assert_eq!(t1 - t0, expected_dx);
    let hit_shifted: Option<NodeId> = scene.with_read(
        |tree: &SceneTree| {
            let b: Rect = tree.node_screen_bounds(btn).unwrap();
            hit_test(
                tree,
                Point::new(
                    b.get_x() + b.get_width() * 0.5,
                    b.get_y() + b.get_height() * 0.5,
                ),
            )
        },
        None,
    );
    assert!(hit_shifted == Some(btn) || hit_shifted == Some(btn_txt));
    let hit_old: Option<NodeId> = scene.with_read(
        |tree: &SceneTree| hit_test(tree, Point::new(40.0, 30.0)),
        None,
    );
    assert_ne!(hit_old, Some(btn));
    assert_ne!(hit_old, Some(btn_txt));
}

#[test]
fn scroll_transitions_and_fixed_size_text_updates_trigger_zero_layout_recomputation() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let root_col: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Px(400.0))
            .height(Length::Px(400.0))
            .background(COLOR_WHITE),
    );
    let fps_txt: NodeId = ctx.text(
        "60 FPS".to_string(),
        Style::default()
            .width(Length::Px(50.0))
            .height(Length::Px(14.0))
            .font_size(11.0),
    );
    let sc: NodeId = ctx.scroll_view(
        Style::default()
            .column()
            .width(Length::Percent(100.0))
            .height(Length::Px(320.0)),
    );
    for _ in 0..20 {
        let item: NodeId = ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Px(48.0)),
        );
        ctx.append(sc, item);
    }
    ctx.append(root_col, fps_txt);
    ctx.append(root_col, sc);
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![root_col]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let vp: Size = Size::new(400.0, 400.0);
    let init: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, vp),
        LayoutStats::default(),
    );
    assert!(init.get_nodes_computed() > 0);
    scene.with_write(
        |tree: &mut SceneTree| {
            tree.set_scroll_offset(sc, Point::new(0.0, 84.5));
            tree.set_text(fps_txt, "180 FPS".to_string());
            tree.start_transition(NodeTransition::route_slide_enter(sc, 160.0));
        },
        (),
    );
    for _ in 0..10 {
        let step_layout: LayoutStats = scene.with_write(
            |tree: &mut SceneTree| {
                let _: bool = tree.tick_transitions(1.0 / 120.0);
                engine.compute(tree, vp)
            },
            LayoutStats::default(),
        );
        assert_eq!(step_layout.get_nodes_computed(), 0);
    }
}

#[test]
fn focused_empty_input_caret_precedes_placeholder_without_overlap_across_dpis() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let caret_color: Color = Color::hex(0xEF4444);
    let text_color: Color = Color::hex(0x0F172A);
    let mut inp_state: InputNodeState = InputNodeState::default();
    inp_state.set_placeholder("Search docs...".to_string());
    let input_id: NodeId = scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            let id: NodeId = tree.create_node(
                NodeKind::Input(inp_state),
                Style::default()
                    .width(Length::Px(240.0))
                    .height(Length::Px(38.0))
                    .padding(Edges::symmetric(8.0, 12.0))
                    .background(COLOR_WHITE)
                    .color(text_color)
                    .font_size(14.0),
            );
            tree.append_child(root, id);
            id
        },
        NodeId::default(),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let vp: Size = Size::new(320.0, 120.0);
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, vp),
        LayoutStats::default(),
    );
    let content_rect: Rect = scene.with_read(
        |tree: &SceneTree| {
            tree.get_node(input_id)
                .unwrap()
                .get_layout_box()
                .get_content_bounds()
        },
        Rect::default(),
    );
    let mut dl_unfocused: DisplayList = DisplayList::default();
    scene.with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, vp, 1.0, COLOR_WHITE, &mut dl_unfocused);
        },
        (),
    );
    let unfocused_caret: bool = dl_unfocused
        .get_commands()
        .iter()
        .any(|cmd: &DrawCommand| matches!(cmd, DrawCommand::TextCaret { .. }));
    assert!(!unfocused_caret);
    let unfocused_run_x: f32 = dl_unfocused
        .get_commands()
        .iter()
        .find_map(|cmd: &DrawCommand| match cmd {
            DrawCommand::TextRun { rect, text, .. } if text == "Search docs..." => {
                Some(rect.get_x())
            }
            _ => None,
        })
        .expect("unfocused placeholder run");
    assert!((unfocused_run_x - content_rect.get_x()).abs() < 0.01);
    let _: Vec<(UiEventHandler, UiEvent)> = scene.with_write(
        |tree: &mut SceneTree| {
            tree.set_caret_visible(true);
            tree.update_focus(Some(input_id))
        },
        Vec::new(),
    );
    let scales: [f32; 3] = [1.0, 2.0, 3.0];
    for scale in scales.into_iter() {
        let mut dl: DisplayList = DisplayList::default();
        scene.with_read(
            |tree: &SceneTree| {
                PaintPass::build(tree, vp, scale, COLOR_WHITE, &mut dl);
            },
            (),
        );
        let mut caret_cmd: Option<(Rect, String)> = None;
        let mut run_rect_opt: Option<Rect> = None;
        for cmd in dl.get_commands().iter() {
            match cmd {
                DrawCommand::TextCaret {
                    text_rect, text, ..
                } => {
                    caret_cmd = Some((*text_rect, text.clone()));
                }
                DrawCommand::TextRun { rect, text, .. } if text == "Search docs..." => {
                    run_rect_opt = Some(*rect);
                }
                _ => {}
            }
        }
        let (caret_text_rect, caret_text): (Rect, String) =
            caret_cmd.expect("focused empty input must emit TextCaret");
        let run_rect: Rect = run_rect_opt.expect("focused empty input must emit placeholder");
        assert!(caret_text.is_empty());
        assert!((caret_text_rect.get_x() - content_rect.get_x()).abs() < 0.01);
        assert!(
            run_rect.get_x() >= caret_text_rect.get_x() + 2.0,
            "placeholder must clear caret at scale={scale}: run_x={} caret_x={}",
            run_rect.get_x(),
            caret_text_rect.get_x()
        );
        let mut color_dl: DisplayList = DisplayList::default();
        color_dl.set_viewport(vp);
        color_dl.set_scale_factor(scale);
        color_dl.push(DrawCommand::Clear(COLOR_WHITE));
        for cmd in dl.get_commands().iter() {
            match cmd {
                DrawCommand::TextCaret {
                    text_rect,
                    content_rect,
                    text,
                    style,
                    caret_height,
                    multiline,
                    ..
                } => {
                    color_dl.push(DrawCommand::TextCaret {
                        text_rect: *text_rect,
                        content_rect: *content_rect,
                        text: text.clone(),
                        style: *style,
                        caret_height: *caret_height,
                        color: caret_color,
                        multiline: *multiline,
                    });
                }
                other => {
                    color_dl.push(other.clone());
                }
            }
        }
        let pw: u32 = (vp.get_width() * scale).round() as u32;
        let ph: u32 = (vp.get_height() * scale).round() as u32;
        let mut sw: SoftwareRenderer =
            SoftwareRenderer::with_config(pw, ph, RenderConfig::software());
        let _: (usize, usize) = sw.render(&color_dl, DamageRegion::full());
        let fb: &Framebuffer = sw.get_framebuffer();
        let mut caret_min_x: u32 = u32::MAX;
        let mut caret_max_x: u32 = 0;
        let mut glyph_min_x: u32 = u32::MAX;
        for py in 0..ph {
            for px in 0..pw {
                let c: Color = fb.get_pixel(px, py).unwrap_or(COLOR_WHITE);
                if c.get_r() > 210 && c.get_g() < 120 && c.get_b() < 120 {
                    caret_min_x = caret_min_x.min(px);
                    caret_max_x = caret_max_x.max(px);
                } else if c != COLOR_WHITE && c.get_r() < 235 && c.get_g() < 235 && c.get_b() < 235
                {
                    glyph_min_x = glyph_min_x.min(px);
                }
            }
        }
        assert!(
            caret_min_x <= caret_max_x,
            "caret pixels missing at scale={scale}"
        );
        assert!(
            glyph_min_x < u32::MAX,
            "placeholder pixels missing at scale={scale}"
        );
        let expected_caret_x: u32 = (content_rect.get_x() * scale).round() as u32;
        assert!(
            caret_min_x.abs_diff(expected_caret_x) <= 1,
            "caret must sit at content_x at scale={scale}: caret_min_x={caret_min_x} expected={expected_caret_x}"
        );
        assert!(
            caret_max_x < glyph_min_x,
            "caret overlapped placeholder at scale={scale}: caret_max_x={caret_max_x} glyph_min_x={glyph_min_x}"
        );
    }
    scene.with_write(
        |tree: &mut SceneTree| {
            if let Some(node) = tree.get_node_mut(input_id)
                && let NodeKind::Input(st) = node.get_mut_kind()
            {
                st.set_value("Hi".to_string());
            }
        },
        (),
    );
    let mut dl_non_empty: DisplayList = DisplayList::default();
    scene.with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, vp, 2.0, COLOR_WHITE, &mut dl_non_empty);
        },
        (),
    );
    let mut val_run_x: Option<f32> = None;
    let mut val_caret_text: Option<String> = None;
    for cmd in dl_non_empty.get_commands().iter() {
        match cmd {
            DrawCommand::TextRun { rect, text, .. } if text == "Hi" => {
                val_run_x = Some(rect.get_x());
            }
            DrawCommand::TextCaret { text, .. } => {
                val_caret_text = Some(text.clone());
            }
            _ => {}
        }
    }
    assert!((val_run_x.expect("value run") - content_rect.get_x()).abs() < 0.01);
    assert_eq!(val_caret_text.as_deref(), Some("Hi"));
}

#[test]
fn selectable_text_drag_double_triple_click_highlight_and_clipboard_copy() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(480, 300).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let sel_tint: Color = Color::rgba(37, 99, 235, 96);
    let (locked_txt, sel_txt, opt_out_txt): (NodeId, NodeId, NodeId) = app.get_scene().with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            let col: NodeId = tree.create_view(
                Style::default()
                    .column()
                    .width(Length::Px(440.0))
                    .padding(Edges::all(16.0))
                    .gap(12.0),
            );
            let locked: NodeId = tree.create_text(
                "Locked label".to_string(),
                Style::default().font_size(14.0).color(COLOR_BLACK),
            );
            let sel_box: NodeId = tree.create_view(
                Style::default()
                    .column()
                    .gap(8.0)
                    .selectable(true)
                    .selection_color(sel_tint),
            );
            let sel: NodeId = tree.create_text(
                "Hello Rust World".to_string(),
                Style::default().font_size(14.0).color(COLOR_BLACK),
            );
            let opt_out: NodeId = tree.create_text(
                "OptOut Label".to_string(),
                Style::default()
                    .font_size(14.0)
                    .color(COLOR_BLACK)
                    .selectable(false),
            );
            tree.append_child(sel_box, sel);
            tree.append_child(sel_box, opt_out);
            tree.append_child(col, locked);
            tree.append_child(col, sel_box);
            tree.replace_children(root, vec![col]);
            (locked, sel, opt_out)
        },
        (NodeId::default(), NodeId::default(), NodeId::default()),
    );
    let _: FrameStats = app.step_frame().unwrap();
    let (locked_ok, sel_ok, opt_out_ok): (bool, bool, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.is_node_selectable(locked_txt),
                tree.is_node_selectable(sel_txt),
                tree.is_node_selectable(opt_out_txt),
            )
        },
        (true, false, true),
    );
    assert!(!locked_ok);
    assert!(sel_ok);
    assert!(!opt_out_ok);
    let wid: WindowId = app.get_window_state().get_id();
    let locked_bounds: Rect = app.get_scene().node_screen_bounds(locked_txt).unwrap();
    let sel_bounds: Rect = app.get_scene().node_screen_bounds(sel_txt).unwrap();
    let opt_bounds: Rect = app.get_scene().node_screen_bounds(opt_out_txt).unwrap();
    let no_mods: Modifiers = Modifiers::default();
    let locked_y: f32 = locked_bounds.get_y() + locked_bounds.get_height() * 0.5;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(locked_bounds.get_x() + 2.0, locked_y),
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(locked_bounds.right() - 2.0, locked_y),
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(locked_bounds.right() - 2.0, locked_y),
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    assert_eq!(app.get_scene().get_selected_text(), None);
    let opt_y: f32 = opt_bounds.get_y() + opt_bounds.get_height() * 0.5;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(opt_bounds.get_x() + 2.0, opt_y),
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(opt_bounds.right() - 2.0, opt_y),
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(opt_bounds.right() - 2.0, opt_y),
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    assert_eq!(app.get_scene().get_selected_text(), None);
    let sel_y: f32 = sel_bounds.get_y() + sel_bounds.get_height() * 0.5;
    let mut ts14: TextStyle = TextStyle::default();
    ts14.set_font_size(14.0);
    let prefix_w: f32 = DeterministicTextMeasurer
        .measure("Hello Ru", ts14, None)
        .get_size()
        .get_width();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(sel_bounds.get_x() + 8.0, sel_y),
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    assert_eq!(app.get_last_cursor(), CursorIcon::Text);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(sel_bounds.get_x(), sel_y),
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(sel_bounds.get_x() + prefix_w, sel_y),
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(sel_bounds.get_x() + prefix_w, sel_y),
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    let dragged: String = app
        .get_scene()
        .get_selected_text()
        .expect("drag must select substring");
    assert!(!dragged.is_empty());
    assert!(dragged.len() < "Hello Rust World".len());
    assert!("Hello Rust World".starts_with(dragged.as_str()));
    let mut dl: DisplayList = DisplayList::default();
    app.get_scene().with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, Size::new(480.0, 300.0), 1.0, COLOR_WHITE, &mut dl);
        },
        (),
    );
    let mut highlight_idx: Option<usize> = None;
    let mut text_run_idx: Option<usize> = None;
    for (idx, cmd) in dl.get_commands().iter().enumerate() {
        match cmd {
            DrawCommand::FillRoundedRect { color, .. } if *color == sel_tint => {
                highlight_idx = Some(idx);
            }
            DrawCommand::TextRun { text, .. } if text == "Hello Rust World" => {
                text_run_idx = Some(idx);
            }
            _ => {}
        }
    }
    let h_idx: usize = highlight_idx.expect("selection highlight rect must be emitted");
    let t_idx: usize = text_run_idx.expect("selected TextRun must be emitted");
    assert!(h_idx < t_idx);
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(
            KeyCode::Character('c'),
            Modifiers::new(false, false, false, true),
            false,
        ),
    ));
    assert_eq!(app.read_clipboard().as_deref(), Some(dragged.as_str()));
    let rust_x: f32 = sel_bounds.get_x() + prefix_w;
    let rust_pt: Point = Point::new(rust_x, sel_y);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            rust_pt,
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            rust_pt,
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            rust_pt,
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            rust_pt,
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    assert_eq!(app.get_scene().get_selected_text().as_deref(), Some("Rust"));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(
            KeyCode::Character('c'),
            Modifiers::new(false, true, false, false),
            false,
        ),
    ));
    assert_eq!(app.read_clipboard().as_deref(), Some("Rust"));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            rust_pt,
            PointerButton::Primary,
            1.0,
            no_mods,
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            rust_pt,
            PointerButton::Primary,
            0.0,
            no_mods,
        ),
    ));
    assert_eq!(
        app.get_scene().get_selected_text().as_deref(),
        Some("Hello Rust World")
    );
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Escape, no_mods, false),
    ));
    assert_eq!(app.get_scene().get_selected_text(), None);
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(
            KeyCode::Character('a'),
            Modifiers::new(false, false, false, true),
            false,
        ),
    ));
    assert_eq!(
        app.get_scene().get_selected_text().as_deref(),
        Some("Hello Rust World")
    );
    assert_eq!(app.copy_selection().as_deref(), Some("Hello Rust World"));
    assert_eq!(app.read_clipboard().as_deref(), Some("Hello Rust World"));
}

#[test]
fn viewport_and_scroll_clip_culling_preserves_bit_exact_parity_and_emits_zero_offscreen_commands() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let root_col: NodeId = ctx.view(
        Style::default()
            .column()
            .width(Length::Px(360.0))
            .height(Length::Px(640.0))
            .background(Color::hex(0xF8FAFC)),
    );
    let scroll: NodeId = ctx.scroll_view(
        Style::default()
            .column()
            .width(Length::Px(360.0))
            .height(Length::Px(560.0))
            .padding(Edges::all(12.0))
            .gap(12.0),
    );
    for idx in 0..24usize {
        let is_bleed_shadow: bool = idx == 5;
        let is_transformed_up: bool = idx == 6;
        let mut card_style: Style = Style::default()
            .column()
            .width(Length::Percent(100.0))
            .height(Length::Px(100.0))
            .padding(Edges::all(12.0))
            .gap(6.0)
            .background(COLOR_WHITE)
            .border(1.0, Color::hex(0xCBD5E1))
            .radius(12.0)
            .overflow(Overflow::Hidden);
        if is_bleed_shadow {
            card_style = card_style.shadow(Shadow::new(
                0.0,
                -14.0,
                20.0,
                4.0,
                Color::rgba(15, 23, 42, 90),
            ));
        } else {
            card_style = card_style.shadow(Shadow::new(
                0.0,
                4.0,
                10.0,
                0.0,
                Color::rgba(15, 23, 42, 32),
            ));
        }
        if is_transformed_up {
            card_style
                .get_mut_paint()
                .set_transform(Transform2D::new(0.0, -180.0, 1.0, 1.0, 0.0));
        }
        let card: NodeId = ctx.view(card_style);
        let label_text: String = if idx >= 10 {
            format!("OFFSCREEN_CARD_{idx:02}")
        } else {
            format!("VISIBLE_CARD_{idx:02}")
        };
        let title: NodeId = ctx.text(
            label_text,
            Style::default().font_size(14.0).color(Color::hex(0x0F172A)),
        );
        ctx.append(card, title);
        if idx == 15 {
            let fixed_bar: NodeId = ctx.view(
                Style::default()
                    .row()
                    .position(Position::Fixed)
                    .inset(None, Some(0.0), Some(0.0), Some(0.0))
                    .width(Length::Px(360.0))
                    .height(Length::Px(52.0))
                    .padding(Edges::all(10.0))
                    .background(Color::hex(0x1E293B)),
            );
            let fixed_lbl: NodeId = ctx.text(
                "FIXED_ESCAPED".to_string(),
                Style::default().font_size(13.0).color(COLOR_WHITE),
            );
            ctx.append(fixed_bar, fixed_lbl);
            ctx.append(card, fixed_bar);
        }
        if idx == 18 {
            let portal_badge: NodeId = ctx.portal(
                Style::default()
                    .position(Position::Fixed)
                    .inset(Some(16.0), Some(16.0), None, None)
                    .width(Length::Px(120.0))
                    .height(Length::Px(32.0))
                    .padding(Edges::all(6.0))
                    .z_index(20)
                    .background(Color::hex(0x2563EB))
                    .radius(8.0),
            );
            let portal_lbl: NodeId = ctx.text(
                "PORTAL_ESCAPED".to_string(),
                Style::default().font_size(12.0).color(COLOR_WHITE),
            );
            ctx.append(portal_badge, portal_lbl);
            ctx.append(card, portal_badge);
        }
        ctx.append(scroll, card);
    }
    ctx.append(root_col, scroll);
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![root_col]);
        },
        (),
    );
    let vp: Size = Size::new(360.0, 640.0);
    let mut layout_engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| layout_engine.compute(tree, vp),
        LayoutStats::default(),
    );
    for scale in [1.0_f32, 2.0_f32].into_iter() {
        let mut dl_culled: DisplayList = DisplayList::default();
        let mut dl_full: DisplayList = DisplayList::default();
        scene.with_read(
            |tree: &SceneTree| {
                PaintPass::build(tree, vp, scale, Color::hex(0xF8FAFC), &mut dl_culled);
                PaintPass::build(
                    tree,
                    Size::new(360.0, 4800.0),
                    scale,
                    Color::hex(0xF8FAFC),
                    &mut dl_full,
                );
            },
            (),
        );
        let offscreen_cmds: usize = dl_culled
            .get_commands()
            .iter()
            .filter(|cmd: &&DrawCommand| {
                matches!(cmd, DrawCommand::TextRun { text, .. } if text.starts_with("OFFSCREEN_CARD_"))
            })
            .count();
        assert_eq!(offscreen_cmds, 0);
        let has_fixed_escaped: bool = dl_culled
            .get_commands()
            .iter()
            .any(|cmd: &DrawCommand| matches!(cmd, DrawCommand::TextRun { text, .. } if text == "FIXED_ESCAPED"));
        let has_portal_escaped: bool = dl_culled
            .get_commands()
            .iter()
            .any(|cmd: &DrawCommand| matches!(cmd, DrawCommand::TextRun { text, .. } if text == "PORTAL_ESCAPED"));
        let has_transformed_card: bool = dl_culled
            .get_commands()
            .iter()
            .any(|cmd: &DrawCommand| matches!(cmd, DrawCommand::TextRun { text, .. } if text == "VISIBLE_CARD_06"));
        let has_bleeding_shadow: bool = dl_culled.get_commands().iter().any(|cmd: &DrawCommand| {
            matches!(cmd, DrawCommand::BoxShadow { rect, shadow, .. } if rect.get_y() > 560.0 && shadow.get_offset_y() < -10.0)
        });
        assert!(has_fixed_escaped);
        assert!(has_portal_escaped);
        assert!(has_transformed_card);
        assert!(has_bleeding_shadow);
        dl_full.set_viewport(vp);
        let pw: u32 = (vp.get_width() * scale).round() as u32;
        let ph: u32 = (vp.get_height() * scale).round() as u32;
        let mut sw_culled: SoftwareRenderer =
            SoftwareRenderer::with_config(pw, ph, RenderConfig::software());
        let mut sw_full: SoftwareRenderer =
            SoftwareRenderer::with_config(pw, ph, RenderConfig::software());
        let _: (usize, usize) = sw_culled.render(&dl_culled, DamageRegion::full());
        let _: (usize, usize) = sw_full.render(&dl_full, DamageRegion::full());
        assert_eq!(
            sw_culled.get_framebuffer().fnv1a_hash(),
            sw_full.get_framebuffer().fnv1a_hash()
        );
    }
    scene.with_write(
        |tree: &mut SceneTree| {
            if let Some(sc_node) = tree.get_node_mut(scroll) {
                let mut st: Style = sc_node.get_style();
                st.get_mut_layout().set_overflow(Overflow::Visible);
                sc_node.set_style(st);
            }
        },
        (),
    );
    let mut dl_vp_culled: DisplayList = DisplayList::default();
    let mut dl_vp_full: DisplayList = DisplayList::default();
    scene.with_read(
        |tree: &SceneTree| {
            PaintPass::build(tree, vp, 1.0, Color::hex(0xF8FAFC), &mut dl_vp_culled);
            PaintPass::build(
                tree,
                Size::new(360.0, 4800.0),
                1.0,
                Color::hex(0xF8FAFC),
                &mut dl_vp_full,
            );
        },
        (),
    );
    assert!(dl_vp_culled.len() < dl_vp_full.len() / 2);
    dl_vp_full.set_viewport(vp);
    let mut sw_vp_culled: SoftwareRenderer =
        SoftwareRenderer::with_config(360, 640, RenderConfig::software());
    let mut sw_vp_full: SoftwareRenderer =
        SoftwareRenderer::with_config(360, 640, RenderConfig::software());
    let _: (usize, usize) = sw_vp_culled.render(&dl_vp_culled, DamageRegion::full());
    let _: (usize, usize) = sw_vp_full.render(&dl_vp_full, DamageRegion::full());
    assert_eq!(
        sw_vp_culled.get_framebuffer().fnv1a_hash(),
        sw_vp_full.get_framebuffer().fnv1a_hash()
    );
}

#[test]
fn lazy_native_font_fallback_skips_cjk_on_cold_ascii_and_renders_cjk_on_demand() {
    let handle: thread::JoinHandle<()> = thread::spawn(|| {
        let font: NativeSystemFont = NativeSystemFont::system();
        let before: usize = NativeSystemFont::thread_fallback_font_loads();
        assert_eq!(before, 0);
        let mut atlas: GlyphAtlas = GlyphAtlas::default();
        let ascii_sample: &str =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 .,:;-";
        for ch in ascii_sample.chars() {
            let key: GlyphKey = GlyphKey::new(ch, 160, false);
            let glyph: &RasterizedGlyph = atlas.get_or_rasterize_with(key, &font);
            if ch != ' ' {
                assert!(glyph.get_coverage().iter().any(|&a: &u8| a > 0));
            }
        }
        let after_ascii: usize = NativeSystemFont::thread_fallback_font_loads();
        assert_eq!(after_ascii, before);
        let mut cjk_bitmaps: Vec<Vec<u8>> = Vec::new();
        for ch in ['中', '国', '启', '动'].into_iter() {
            let key: GlyphKey = GlyphKey::new(ch, 160, false);
            let glyph: &RasterizedGlyph = atlas.get_or_rasterize_with(key, &font);
            assert!(glyph.get_coverage().iter().any(|&a: &u8| a > 0));
            cjk_bitmaps.push(glyph.get_coverage().clone());
        }
        assert_ne!(cjk_bitmaps[0], cjk_bitmaps[1]);
        if font.font_info().get_backend() != FontRenderBackend::DeterministicBitmap {
            let after_cjk: usize = NativeSystemFont::thread_fallback_font_loads();
            assert!(after_cjk > before);
            assert!(after_cjk <= before + 2);
        }
    });
    assert!(handle.join().is_ok());
}

#[test]
fn lazy_game_cold_start_constructors_allocate_on_first_active_step_and_render() {
    let mut particles: ParticleSystem = ParticleSystem::default();
    assert_eq!(particles.update(0.016), 0);
    assert!(particles.active_particle(0).is_none());
    particles.emit_burst(Point::new(10.0, 20.0), 12, ParticleEmitterConfig::default());
    assert_eq!(particles.get_active_count(), 12);
    assert!(particles.active_particle(0).is_some());
    assert_eq!(particles.update(0.016), 12);
    let mut physics: PhysicsWorld2D = PhysicsWorld2D::default();
    let mut world: GameWorld = GameWorld::default();
    let a: EntityId = world.spawn(EntityTag::Player, Point::new(0.0, 0.0));
    let b: EntityId = world.spawn(EntityTag::Enemy, Point::new(6.0, 0.0));
    if let Some(ent) = world.get_entity_mut(a) {
        ent.set_body(Some(RigidBody2D::dynamic(1.0, 0.5, 0.0)));
        ent.set_collider(Some(Collider2D::circle(
            8.0,
            COLLISION_LAYER_PLAYER,
            COLLISION_LAYER_ENEMY,
            false,
        )));
    }
    if let Some(ent) = world.get_entity_mut(b) {
        ent.set_body(Some(RigidBody2D::dynamic(1.0, 0.5, 0.0)));
        ent.set_collider(Some(Collider2D::circle(
            8.0,
            COLLISION_LAYER_ENEMY,
            COLLISION_LAYER_PLAYER,
            false,
        )));
    }
    let events: Vec<CollisionEvent> = physics.step(&mut world, 0.016);
    assert_eq!(events.len(), 1);
    let mut audio: AudioBus = AudioBus::default();
    assert!(audio.drain().is_empty());
    audio.play(SoundCue::Explosion, 0.8, 0.0, 1.0);
    assert_eq!(audio.drain().len(), 1);
    let empty_bvh: Bvh3D = Bvh3D::build(Vec::new());
    assert!(empty_bvh.get_nodes().is_empty());
    let mut renderer3d: Renderer3D = Renderer3D::default();
    let mut world3d: World3D = World3D::default();
    let low_q: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::Low);
    let img_low: ImageBuffer = renderer3d.render_world(&mut world3d, 240, 144, &low_q);
    assert_eq!(img_low.get_width(), 240);
    assert_eq!(img_low.get_height(), 144);
    let high_q: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::High);
    let img_high: ImageBuffer = renderer3d.render_world(&mut world3d, 240, 144, &high_q);
    assert_eq!(img_high.get_width(), 240);
    assert_eq!(img_high.get_height(), 144);
}

#[test]
fn flex_wrap_row_distributes_flex_grow_per_wrapped_line_without_trailing_gap() {
    reset_reactive_runtime();
    let scene: SceneHandle = SceneHandle::new_tree();
    let mut ctx: Context = Context::new(scene.clone(), PlatformKind::Headless);
    let wrap_row: NodeId = ctx.view(
        Style::default()
            .row()
            .wrap(FlexWrap::Wrap)
            .width(Length::Percent(100.0))
            .gap(8.0),
    );
    let mut buttons: Vec<NodeId> = Vec::new();
    for idx in 0..5usize {
        let btn: NodeId = ctx.view(
            Style::default()
                .flex_grow(1.0)
                .min_width(Length::Px(110.0))
                .height(Length::Px(36.0))
                .background(COLOR_WHITE),
        );
        let lbl: NodeId = ctx.text(
            format!("Btn {idx}"),
            Style::default().font_size(13.0).color(COLOR_BLACK),
        );
        ctx.append(btn, lbl);
        ctx.append(wrap_row, btn);
        buttons.push(btn);
    }
    scene.with_write(
        |tree: &mut SceneTree| {
            let root: NodeId = tree.get_root();
            tree.replace_children(root, vec![wrap_row]);
        },
        (),
    );
    let mut engine: LayoutEngine = LayoutEngine::default();
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, Size::new(360.0, 400.0)),
        LayoutStats::default(),
    );
    let bounds_narrow: Vec<Rect> = scene.with_read(
        |tree: &SceneTree| {
            buttons
                .iter()
                .map(|&id: &NodeId| tree.get_node(id).unwrap().get_layout_box().get_bounds())
                .collect()
        },
        Vec::new(),
    );
    assert!((bounds_narrow[0].get_y() - 0.0).abs() < 0.05);
    assert!((bounds_narrow[1].get_y() - 0.0).abs() < 0.05);
    assert!((bounds_narrow[2].get_y() - 0.0).abs() < 0.05);
    assert!((bounds_narrow[2].right() - 360.0).abs() < 0.1);
    assert!((bounds_narrow[3].get_y() - 44.0).abs() < 0.05);
    assert!((bounds_narrow[4].get_y() - 44.0).abs() < 0.05);
    assert!((bounds_narrow[4].right() - 360.0).abs() < 0.1);
    assert!((bounds_narrow[3].get_width() - 176.0).abs() < 0.1);
    assert!((bounds_narrow[4].get_width() - 176.0).abs() < 0.1);
    let _: LayoutStats = scene.with_write(
        |tree: &mut SceneTree| engine.compute(tree, Size::new(640.0, 400.0)),
        LayoutStats::default(),
    );
    let bounds_wide: Vec<Rect> = scene.with_read(
        |tree: &SceneTree| {
            buttons
                .iter()
                .map(|&id: &NodeId| tree.get_node(id).unwrap().get_layout_box().get_bounds())
                .collect()
        },
        Vec::new(),
    );
    for rect in bounds_wide.iter() {
        assert!((rect.get_y() - 0.0).abs() < 0.05);
    }
    assert!((bounds_wide[4].right() - 640.0).abs() < 0.1);
}
