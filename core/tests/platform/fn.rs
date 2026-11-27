use super::*;

#[test]
fn cold_start_and_atomic_island_safe_area_never_regresses_to_zero() {
    let cold: SafeAreaInsets = resolve_mobile_safe_area(
        SafeAreaInsets::default(),
        SafeAreaInsets::default(),
        0.0,
        28.0,
        false,
    );
    assert!((cold.get_top() - 28.0).abs() < 0.01);
    let atomic_island: SafeAreaInsets = resolve_mobile_safe_area(
        SafeAreaInsets::new(36.0, 0.0, 24.0, 0.0),
        SafeAreaInsets::new(56.0, 0.0, 24.0, 0.0),
        54.0,
        28.0,
        false,
    );
    assert!((atomic_island.get_top() - 56.0).abs() < 0.01);
    assert!((atomic_island.get_bottom() - 24.0).abs() < 0.01);
    let cutout_dominant: SafeAreaInsets = resolve_mobile_safe_area(
        SafeAreaInsets::new(0.0, 0.0, 20.0, 0.0),
        SafeAreaInsets::new(32.0, 0.0, 20.0, 0.0),
        62.0,
        28.0,
        false,
    );
    assert!((cutout_dominant.get_top() - 62.0).abs() < 0.01);
    let fs_insets: SafeAreaInsets = resolve_mobile_safe_area(
        SafeAreaInsets::new(56.0, 0.0, 24.0, 0.0),
        SafeAreaInsets::new(56.0, 0.0, 24.0, 0.0),
        56.0,
        28.0,
        true,
    );
    assert_eq!(fs_insets, SafeAreaInsets::default());
    let px_resolved: SafeAreaInsets =
        resolve_safe_area_from_px([0, 0, 72, 0], [168, 0, 72, 0], 162, 108, 3.0, false);
    assert!((px_resolved.get_top() - 56.0).abs() < 0.01);
    assert!((px_resolved.get_bottom() - 24.0).abs() < 0.01);
    let px_cold_start: SafeAreaInsets =
        resolve_safe_area_from_px([0, 0, 0, 0], [0, 0, 0, 0], 0, 108, 3.0, false);
    assert!((px_cold_start.get_top() - 36.0).abs() < 0.01);
}

#[test]
fn high_refresh_display_mode_selection_and_frame_pacing_have_no_60hz_cap() {
    let current: DisplayModeInfo = DisplayModeInfo::new(1, 1440, 3200, 60.0);
    let supported: [DisplayModeInfo; 5] = [
        DisplayModeInfo::new(1, 1440, 3200, 60.0),
        DisplayModeInfo::new(2, 1440, 3200, 90.0),
        DisplayModeInfo::new(3, 1440, 3200, 120.0),
        DisplayModeInfo::new(4, 1440, 3200, 144.0),
        DisplayModeInfo::new(5, 1080, 2400, 240.0),
    ];
    let best: DisplayModeInfo = select_highest_refresh_mode(current, &supported);
    assert_eq!(best.mode_id(), 4);
    assert_eq!(best.physical_size(), (1440, 3200));
    assert!((best.refresh_rate_hz() - 144.0).abs() < 0.01);
    let rates: [(f32, f32, u64); 5] = [
        (60.0, 60.0, 16_666_667),
        (90.0, 60.0, 11_111_111),
        (120.0, 80.0, 8_333_333),
        (144.0, 80.0, 6_944_444),
        (240.0, 80.0, 4_166_667),
    ];
    for &(hz, expected_min, expected_ns) in rates.iter() {
        let pacing: FramePacingConfig = compute_frame_pacing(hz);
        assert!((pacing.preferred_hz() - hz).abs() < 0.01);
        assert!((pacing.max_hz() - hz).abs() < 0.01);
        assert!((pacing.min_hz() - expected_min).abs() < 0.01);
        assert!((pacing.frame_interval_secs() - (1.0 / hz)).abs() < 1e-6);
        assert_eq!(pacing.frame_interval_nanos(), expected_ns);
    }
    let fallback: FramePacingConfig = compute_frame_pacing(f32::NAN);
    assert!((fallback.preferred_hz() - FALLBACK_REFRESH_RATE_HZ).abs() < 0.01);
}

#[test]
fn android_and_ios_touch_cancellation_and_multi_touch_pointer_id_lifecycle() {
    let wid: WindowId = WindowId::new(1);
    let mut tracker: TouchPointerTracker = TouchPointerTracker::default();
    let down_ev: Vec<WindowEvent> =
        tracker.handle_android_motion(wid, ANDROID_ACTION_DOWN, 0, &[(4, 100.0, 120.0)]);
    assert_eq!(down_ev.len(), 1);
    assert_eq!(tracker.active_ids(), &[5]);
    let pdown_ev: Vec<WindowEvent> = tracker.handle_android_motion(
        wid,
        ANDROID_ACTION_POINTER_DOWN,
        1,
        &[(4, 100.0, 120.0), (7, 240.0, 180.0)],
    );
    assert_eq!(pdown_ev.len(), 1);
    assert_eq!(tracker.active_ids(), &[5, 8]);
    let pup_ev: Vec<WindowEvent> = tracker.handle_android_motion(
        wid,
        ANDROID_ACTION_POINTER_UP,
        1,
        &[(4, 102.0, 122.0), (7, 244.0, 184.0)],
    );
    assert_eq!(pup_ev.len(), 1);
    if let WindowEvent::PointerUp(_, ptr) = pup_ev[0] {
        assert_eq!(ptr.get_pointer_id(), 8);
    } else {
        panic!("expected PointerUp for actionIndex 1");
    }
    assert_eq!(tracker.active_ids(), &[5]);
    let up_ev: Vec<WindowEvent> =
        tracker.handle_android_motion(wid, ANDROID_ACTION_UP, 0, &[(4, 104.0, 124.0)]);
    assert_eq!(up_ev.len(), 1);
    assert!(matches!(up_ev[0], WindowEvent::PointerUp(_, _)));
    assert_eq!(tracker.active_count(), 0);
    let _: Vec<WindowEvent> =
        tracker.handle_android_motion(wid, ANDROID_ACTION_DOWN, 0, &[(1, 10.0, 20.0)]);
    let _: Vec<WindowEvent> = tracker.handle_android_motion(
        wid,
        ANDROID_ACTION_POINTER_DOWN,
        1,
        &[(1, 10.0, 20.0), (2, 30.0, 40.0)],
    );
    let cancel_ev: Vec<WindowEvent> = tracker.handle_android_motion(
        wid,
        ANDROID_ACTION_CANCEL,
        0,
        &[(1, 10.0, 20.0), (2, 30.0, 40.0)],
    );
    assert_eq!(cancel_ev.len(), 3);
    assert_eq!(cancel_ev[0], WindowEvent::PointerCancel(wid, 2));
    assert_eq!(cancel_ev[1], WindowEvent::PointerCancel(wid, 3));
    assert_eq!(cancel_ev[2], WindowEvent::PointerCancel(wid, 0));
    assert_eq!(tracker.active_count(), 0);
    let _: Vec<WindowEvent> =
        tracker.handle_ios_touches_began(wid, &[(101, 50.0, 60.0), (202, 150.0, 160.0)]);
    assert_eq!(tracker.active_ids(), &[101, 202]);
    let ios_partial: Vec<WindowEvent> = tracker.handle_ios_touches_cancelled(wid, &[101], &[202]);
    assert_eq!(ios_partial, vec![WindowEvent::PointerCancel(wid, 101)]);
    assert_eq!(tracker.active_ids(), &[202]);
    let focus_cancel: Vec<WindowEvent> = tracker.cancel_all_on_focus_loss(wid);
    assert_eq!(
        focus_cancel,
        vec![
            WindowEvent::PointerCancel(wid, 202),
            WindowEvent::PointerCancel(wid, 0),
        ]
    );
    assert_eq!(tracker.active_count(), 0);
}

#[test]
fn headless_platform_reports_high_refresh_and_handles_fullscreen_command() {
    let mut plat: HeadlessPlatform = HeadlessPlatform::emulating(
        PlatformKind::Android,
        3.0,
        SafeAreaInsets::new(48.0, 0.0, 24.0, 0.0),
    )
    .with_refresh_rate_hz(144.0);
    assert!((plat.display_refresh_rate_hz() - 144.0).abs() < 0.01);
    let cfg: WindowConfig = WindowConfig::default();
    let state: WindowState = plat.create_window(&cfg).unwrap();
    assert_eq!(state.get_display().get_refresh_rate_hz(), 144);
    assert!((state.get_safe_area().get_top() - 48.0).abs() < 0.01);
    let wid: WindowId = state.get_id();
    plat.apply_command(WindowCommand::SetFullscreen(
        wid,
        FullscreenMode::Borderless,
    ))
    .unwrap();
    assert_eq!(
        plat.window_state(wid).unwrap().get_fullscreen(),
        FullscreenMode::Borderless
    );
    plat.apply_command(WindowCommand::SetFullscreen(wid, FullscreenMode::Windowed))
        .unwrap();
    assert_eq!(
        plat.window_state(wid).unwrap().get_fullscreen(),
        FullscreenMode::Windowed
    );
}

#[test]
fn renderer_lazily_sizes_inactive_modes_and_supports_mode_switching() {
    let caps: PlatformCapabilities = PlatformCapabilities::for_platform(PlatformKind::Android);
    let mut renderer: Renderer = Renderer::new(
        PlatformKind::Android,
        caps,
        1440,
        3200,
        RenderConfig::software(),
    );
    assert_eq!(renderer.get_software().get_framebuffer().get_width(), 1440);
    assert_eq!(renderer.get_software().get_framebuffer().get_height(), 3200);
    assert_eq!(
        renderer
            .get_hybrid()
            .get_software()
            .get_framebuffer()
            .get_width(),
        1
    );
    assert_eq!(
        renderer
            .get_hybrid()
            .get_software()
            .get_framebuffer()
            .get_height(),
        1
    );
    let mut tree: SceneTree = SceneTree::default();
    let root: NodeId = tree.get_root();
    let child: NodeId = tree.create_node(
        NodeKind::View,
        Style::default()
            .width(Length::Px(120.0))
            .height(Length::Px(60.0))
            .background(Color::hex(0x4F46E5)),
    );
    tree.append_child(root, child);
    let mut engine: LayoutEngine = LayoutEngine::default();
    let lstats: LayoutStats = engine.compute(&mut tree, Size::new(360.0, 640.0));
    renderer.switch_mode(PlatformKind::Android, caps, RenderMode::Hybrid);
    let stats: FrameStats = renderer
        .render_frame(
            &tree,
            Size::new(360.0, 640.0),
            1.0,
            COLOR_WHITE,
            lstats,
            DamageRegion::full(),
        )
        .unwrap();
    assert!(stats.get_pixels_shaded() > 0);
    assert_eq!(
        renderer
            .get_hybrid()
            .get_software()
            .get_framebuffer()
            .get_width(),
        360
    );
    assert_eq!(
        renderer
            .get_hybrid()
            .get_software()
            .get_framebuffer()
            .get_height(),
        640
    );
}

#[test]
fn fifty_cycle_same_process_exit_reenter_keeps_sessions_and_ui_interactive() {
    let mut coord: MobileSessionLifecycle = MobileSessionLifecycle::default();
    let mut prev_owner: u64 = 0;
    let mut prev_gen: u64 = 0;
    let wid: WindowId = WindowId::new(1);
    for cycle in 1_u64..=50_u64 {
        let owner_id: u64 = cycle * 10;
        let session_gen: u64 = coord.begin_session(owner_id);
        assert_eq!(coord.active_session_generation(), cycle);
        assert!(coord.is_active_owner(owner_id, session_gen));
        if prev_owner != 0 {
            let stale_cleared: bool = coord.end_session(prev_owner, prev_gen);
            assert!(!stale_cleared);
            assert!(coord.is_active_owner(owner_id, session_gen));
        }
        coord.on_surface_created();
        assert!(coord.is_surface_attached());
        let loop_g1: u64 = coord.on_resume();
        assert!(coord.is_loop_running());
        assert!(coord.activate_session(owner_id, session_gen));
        let _: Vec<WindowEvent> = coord.touch_tracker_mut().handle_android_motion(
            wid,
            ANDROID_ACTION_DOWN,
            0,
            &[(0, 40.0, 40.0)],
        );
        assert_eq!(coord.touch_tracker().active_count(), 1);
        let pause_cancels: Vec<WindowEvent> = coord.on_pause(wid);
        assert!(!pause_cancels.is_empty());
        assert!(!coord.is_loop_running());
        assert_eq!(coord.touch_tracker().active_count(), 0);
        assert!(coord.loop_generation() > loop_g1);
        coord.on_background();
        let _: Vec<WindowEvent> = coord.on_surface_destroyed(wid);
        assert!(!coord.is_surface_attached());
        coord.on_terminate();
        assert!(coord.should_recreate_handle());
        if cycle % 2 == 0 {
            let ended: bool = coord.end_session(owner_id, session_gen);
            assert!(ended);
            assert_eq!(coord.active_owner_id(), 0);
            prev_owner = 0;
            prev_gen = 0;
        } else {
            prev_owner = owner_id;
            prev_gen = session_gen;
        }
    }
    if prev_owner != 0 {
        assert!(coord.end_session(prev_owner, prev_gen));
    }
    let mut total_clicks: u32 = 0;
    for cycle in 0_u32..50_u32 {
        reset_reactive_runtime();
        let clicked: Signal<u32> = Signal::new(0);
        let mut win: WindowConfig = WindowConfig::default();
        win.set_title("Euv Studio".to_string())
            .set_size(390, 844)
            .set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let backend: Box<dyn PlatformBackend> = Box::new(
            HeadlessPlatform::emulating(
                PlatformKind::Android,
                3.0,
                SafeAreaInsets::new(48.0, 0.0, 24.0, 0.0),
            )
            .with_refresh_rate_hz(120.0),
        );
        let mut app: App = App::with_platform(cfg, backend).unwrap();
        let _: NodeId = app.mount(|| {
            let mut ctx: Context = current_context();
            let root: NodeId = ctx.view(
                Style::default()
                    .column()
                    .width(Length::Px(390.0))
                    .height(Length::Px(844.0)),
            );
            let btn: NodeId = ctx.view(
                Style::default()
                    .width(Length::Px(160.0))
                    .height(Length::Px(48.0))
                    .background(Color::hex(0x4F46E5)),
            );
            ctx.on_click(btn, move |_: &UiEvent| {
                let _: bool = clicked.update(|v: &mut u32| *v += 1);
            });
            ctx.append(root, btn);
            root
        });
        let app_wid: WindowId = app.get_window_state().get_id();
        let _: FrameStats = app.step_game_frame(1.0 / 120.0).unwrap();
        app.dispatch_window_event(WindowEvent::PointerDown(
            app_wid,
            PointerEvent::touch(1, 40.0, 24.0),
        ));
        app.dispatch_window_event(WindowEvent::PointerUp(
            app_wid,
            PointerEvent::touch(1, 40.0, 24.0),
        ));
        assert_eq!(clicked.get(), 1, "cycle {cycle} button must respond");
        total_clicks += clicked.get();
        app.dispatch_system_event(SystemEvent::Inactive);
        app.dispatch_system_event(SystemEvent::Background);
        app.dispatch_system_event(SystemEvent::Appear);
        app.dispatch_system_event(SystemEvent::Foreground);
        let _: FrameStats = app.step_game_frame(1.0 / 120.0).unwrap();
        app.dispatch_window_event(WindowEvent::PointerDown(
            app_wid,
            PointerEvent::touch(2, 40.0, 24.0),
        ));
        app.dispatch_window_event(WindowEvent::PointerUp(
            app_wid,
            PointerEvent::touch(2, 40.0, 24.0),
        ));
        assert_eq!(
            clicked.get(),
            2,
            "cycle {cycle} button must stay interactive after background resume"
        );
        app.dispatch_system_event(SystemEvent::Terminate);
        drop(app);
    }
    assert_eq!(total_clicks, 50);
    reset_reactive_runtime();
}
