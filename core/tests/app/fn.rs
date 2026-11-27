use super::*;

struct FreezeProbeGameScene {
    ticks: Rc<Cell<u32>>,
    renders: Rc<Cell<u32>>,
}

impl GameScene for FreezeProbeGameScene {
    fn fixed_update(&mut self, _dt: f32, _input: &InputState) {
        let Self { ticks, .. } = self;
        ticks.set(ticks.get().saturating_add(1));
    }

    fn render(&mut self, _alpha: f32, ctx: &mut GameRenderContext<'_>) {
        let Self { renders, ticks } = self;
        renders.set(renders.get().saturating_add(1));
        let phase: f32 = ((ticks.get() % 60) as f32) * 2.0;
        ctx.clear(Color::hex(0x0F172A));
        ctx.fill_rounded_rect(
            Rect::new(10.0 + phase, 20.0, 24.0, 24.0),
            Color::hex(0x38BDF8),
            4.0,
        );
    }
}

#[test]
fn background_freeze_suspends_logic_layout_paint_render_transitions_game_and_effects_across_120_frames()
 {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    app.set_telemetry_enabled(true);
    let wid: WindowId = app.get_window_state().get_id();
    let game_ticks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let game_renders: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let host: GameEngineHost = GameEngineHost::new(
        FreezeProbeGameScene {
            ticks: game_ticks.clone(),
            renders: game_renders.clone(),
        },
        1.0 / 60.0,
    );
    app.attach_game_host(host.clone());
    let active_sig: Signal<i32> = Signal::new(1);
    let frozen_page_sig: Signal<i32> = Signal::new(10);
    let active_effect_runs: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let frozen_effect_runs: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let bg_hook_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fg_hook_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let anim_node_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let frozen_scope_cell: Rc<Cell<Option<Scope>>> = Rc::new(Cell::new(None));
    let act_eff_c: Rc<Cell<u32>> = active_effect_runs.clone();
    let frz_eff_c: Rc<Cell<u32>> = frozen_effect_runs.clone();
    let bg_c: Rc<Cell<u32>> = bg_hook_count.clone();
    let fg_c: Rc<Cell<u32>> = fg_hook_count.clone();
    let anim_cell_c: Rc<Cell<NodeId>> = anim_node_cell.clone();
    let frz_scope_c: Rc<Cell<Option<Scope>>> = frozen_scope_cell.clone();
    let host_mount: GameEngineHost = host.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(240.0)),
        );
        let _: Effect = Effect::new(move || {
            let _: i32 = active_sig.get();
            act_eff_c.set(act_eff_c.get().saturating_add(1));
        });
        let _: SystemEventSubscription = on_background(move || {
            bg_c.set(bg_c.get().saturating_add(1));
        });
        let _: SystemEventSubscription = on_foreground(move || {
            fg_c.set(fg_c.get().saturating_add(1));
        });
        let anim_card: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(120.0))
                .height(Length::Px(40.0))
                .background(Color::hex(0x2563EB)),
        );
        anim_cell_c.set(anim_card);
        let viewport_node: NodeId = host_mount.mount_viewport(
            Style::default()
                .width(Length::Px(180.0))
                .height(Length::Px(120.0)),
        );
        let (inactive_scope, inactive_node): (Scope, NodeId) = Scope::run(move |_s: Scope| {
            let _: Effect = Effect::new(move || {
                let _: i32 = frozen_page_sig.get();
                frz_eff_c.set(frz_eff_c.get().saturating_add(1));
            });
            let mut inner_ctx: Context = current_context();
            inner_ctx.view(Style::default())
        });
        inactive_scope.freeze();
        frz_scope_c.set(Some(inactive_scope));
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(inactive_node, inactive_scope);
                tree.start_transition(NodeTransition::route_slide_enter(anim_card, 120.0));
            },
            (),
        );
        ctx.append_many(root, &[anim_card, viewport_node, inactive_node]);
        root
    });
    let initial: FrameStats = app.step_game_frame(0.02).unwrap();
    assert!(!app.is_background_frozen());
    assert!(app.should_pump_frames());
    assert!(!app.is_platform_frame_pump_paused());
    assert!(initial.get_layout_nodes() > 0);
    assert!(initial.get_draw_commands() > 0);
    assert!(initial.get_pixels_shaded() > 0);
    assert_eq!(active_effect_runs.get(), 1);
    assert_eq!(frozen_effect_runs.get(), 1);
    let second: FrameStats = app.step_game_frame(0.02).unwrap();
    assert!(second.get_pixels_shaded() > 0);
    assert!(game_ticks.get() >= 1);
    assert!(game_renders.get() >= 1);
    let anim_node: NodeId = anim_node_cell.get();
    let opacity_before_bg: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(anim_node)
                .map(|n: &SceneNode| n.get_style().get_paint().get_opacity())
                .unwrap_or(0.0)
        },
        0.0,
    );
    app.dispatch_window_event(WindowEvent::System(SystemEvent::Background));
    assert_eq!(bg_hook_count.get(), 1);
    assert_eq!(fg_hook_count.get(), 0);
    assert!(app.is_background_frozen());
    assert!(!app.should_pump_frames());
    assert!(app.is_platform_frame_pump_paused());
    assert!(!app.has_active_animations());
    assert_eq!(app.present_fps(), 0.0);
    assert!(app.is_idle());
    let clock_before_bg: u64 = app.get_context().elapsed_ms();
    let game_ticks_before_bg: u32 = game_ticks.get();
    let game_renders_before_bg: u32 = game_renders.get();
    let effect_runs_before_bg: u32 = active_effect_runs.get();
    let frozen_runs_before_bg: u32 = frozen_effect_runs.get();
    let hash_before_bg: u64 = second.get_frame_hash();
    for step_idx in 0_u32..60 {
        let _: bool = active_sig.set(100 + (step_idx as i32));
        let _: bool = frozen_page_sig.set(200 + (step_idx as i32));
        app.dispatch_window_event(WindowEvent::RedrawRequested(wid));
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::new(
                1,
                PointerDevice::Mouse,
                Point::new(50.0, 50.0),
                PointerButton::Primary,
                0.0,
                Modifiers::default(),
            ),
        ));
        let st: FrameStats = app.step_frame().unwrap();
        let tel: FrameTelemetry = app.last_frame_telemetry();
        assert_eq!(st.get_layout_nodes(), 0);
        assert_eq!(st.get_draw_commands(), 0);
        assert_eq!(st.get_draw_batches(), 0);
        assert_eq!(st.get_pixels_shaded(), 0);
        assert_eq!(st.get_frame_hash(), hash_before_bg);
        assert!(tel.get_is_idle());
        assert_eq!(tel.get_present_fps(), 0.0);
        assert_eq!(tel.get_layout_nodes(), 0);
        assert_eq!(tel.get_draw_commands(), 0);
        assert_eq!(tel.get_pixels_shaded(), 0);
        assert_eq!(tel.get_dirty_area_ratio(), 0.0);
    }
    for _ in 0_u32..60 {
        let st: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
        assert_eq!(st.get_layout_nodes(), 0);
        assert_eq!(st.get_draw_commands(), 0);
        assert_eq!(st.get_draw_batches(), 0);
        assert_eq!(st.get_pixels_shaded(), 0);
        assert_eq!(st.get_frame_hash(), hash_before_bg);
    }
    assert_eq!(active_effect_runs.get(), effect_runs_before_bg);
    assert_eq!(frozen_effect_runs.get(), frozen_runs_before_bg);
    assert_eq!(game_ticks.get(), game_ticks_before_bg);
    assert_eq!(game_renders.get(), game_renders_before_bg);
    assert_eq!(app.get_context().elapsed_ms(), clock_before_bg);
    let opacity_after_bg: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(anim_node)
                .map(|n: &SceneNode| n.get_style().get_paint().get_opacity())
                .unwrap_or(0.0)
        },
        0.0,
    );
    assert!((opacity_after_bg - opacity_before_bg).abs() < 1e-6);
    app.dispatch_window_event(WindowEvent::System(SystemEvent::Foreground));
    assert_eq!(fg_hook_count.get(), 1);
    assert!(!app.is_background_frozen());
    assert!(app.should_pump_frames());
    assert!(!app.is_platform_frame_pump_paused());
    assert_eq!(active_effect_runs.get(), effect_runs_before_bg + 1);
    assert_eq!(frozen_effect_runs.get(), frozen_runs_before_bg);
    assert!(frozen_scope_cell.get().unwrap().is_frozen());
    let restore_stats: FrameStats = app.step_game_frame(300.0).unwrap();
    assert!(restore_stats.get_draw_commands() > 0);
    assert!(restore_stats.get_pixels_shaded() > 0);
    assert_eq!(app.get_context().elapsed_ms(), clock_before_bg);
    assert_eq!(game_ticks.get(), game_ticks_before_bg);
    let opacity_on_restore: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(anim_node)
                .map(|n: &SceneNode| n.get_style().get_paint().get_opacity())
                .unwrap_or(0.0)
        },
        0.0,
    );
    assert!((opacity_on_restore - opacity_before_bg).abs() < 1e-6);
    let resumed_tick: FrameStats = app.step_game_frame(0.02).unwrap();
    assert!(resumed_tick.get_pixels_shaded() > 0);
    assert!(app.get_context().elapsed_ms() > clock_before_bg);
    assert!(game_ticks.get() > game_ticks_before_bg);
}

#[test]
fn background_freeze_processes_lifecycle_and_system_events_wakes_and_handles_terminate() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let term_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fg_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let term_c: Rc<Cell<u32>> = term_hits.clone();
    let fg_c: Rc<Cell<u32>> = fg_hits.clone();
    let _: NodeId = app.mount(move || {
        let _: SystemEventSubscription = on_foreground(move || {
            fg_c.set(fg_c.get().saturating_add(1));
        });
        let _: SystemEventSubscription = on_terminate(move || {
            term_c.set(term_c.get().saturating_add(1));
        });
        let mut ctx: Context = current_context();
        ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .background(Color::hex(0x1E293B)),
        )
    });
    let first: FrameStats = app.step_frame().unwrap();
    assert!(first.get_pixels_shaded() > 0);
    app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Paused));
    assert!(app.is_background_frozen());
    assert!(!app.should_pump_frames());
    app.dispatch_window_event(WindowEvent::ThemeChanged(wid, ThemeMode::Dark));
    app.dispatch_system_event(SystemEvent::LocaleChanged("de-DE".to_string()));
    app.dispatch_system_event(SystemEvent::LowPowerModeChanged(true));
    app.dispatch_system_event(SystemEvent::MemoryWarning(MemoryPressure::Warning));
    app.dispatch_window_event(WindowEvent::Resized(wid, 640, 480, 2.0));
    let bg_wait: FrameStats = app.wait_background_events().unwrap();
    assert!(app.is_background_frozen());
    assert_eq!(bg_wait.get_layout_nodes(), 0);
    assert_eq!(bg_wait.get_draw_commands(), 0);
    assert_eq!(bg_wait.get_pixels_shaded(), 0);
    assert_eq!(
        app.get_context().get_theme_mode().get_untracked(),
        ThemeMode::Dark
    );
    assert_eq!(app.get_context().get_locale().get_untracked(), "de-DE");
    assert!(app.get_context().get_low_power_mode().get_untracked());
    assert_eq!(
        app.get_context().get_viewport_size().get_untracked(),
        Size::new(640.0, 480.0)
    );
    app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Resumed));
    let woke: FrameStats = app.wait_background_events().unwrap();
    assert!(!app.is_background_frozen());
    assert!(app.should_pump_frames());
    assert_eq!(fg_hits.get(), 1);
    assert!(woke.get_pixels_shaded() > 0);
    app.dispatch_system_event(SystemEvent::Background);
    assert!(app.is_background_frozen());
    app.dispatch_system_event(SystemEvent::Terminate);
    assert_eq!(term_hits.get(), 1);
    assert!(!app.get_running());
    assert!(!app.should_pump_frames());
}

#[test]
fn platform_backends_pause_and_resume_frame_pumps_across_all_seven_targets() {
    for target in PlatformKind::all_targets().into_iter() {
        reset_reactive_runtime();
        let backend: HeadlessPlatform =
            HeadlessPlatform::emulating(target, 2.0, SafeAreaInsets::default());
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(320, 200).set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let mut app: App = App::with_platform(cfg, Box::new(backend)).unwrap();
        let _: NodeId = app.mount(|| {
            let mut ctx: Context = current_context();
            ctx.view(
                Style::default()
                    .width(Length::Px(320.0))
                    .height(Length::Px(200.0))
                    .background(Color::hex(0x0EA5E9)),
            )
        });
        let rendered: FrameStats = app.step_frame().unwrap();
        assert!(rendered.get_pixels_shaded() > 0);
        assert!(!app.is_platform_frame_pump_paused());
        app.dispatch_system_event(SystemEvent::Background);
        assert!(app.is_background_frozen());
        assert!(!app.should_pump_frames());
        assert!(app.is_platform_frame_pump_paused());
        for _ in 0_u32..100 {
            let bg: FrameStats = app.step_frame().unwrap();
            assert_eq!(bg.get_layout_nodes(), 0);
            assert_eq!(bg.get_draw_commands(), 0);
            assert_eq!(bg.get_pixels_shaded(), 0);
        }
        app.dispatch_system_event(SystemEvent::Foreground);
        assert!(!app.is_background_frozen());
        assert!(app.should_pump_frames());
        assert!(!app.is_platform_frame_pump_paused());
        let restored: FrameStats = app.step_frame().unwrap();
        assert!(restored.get_pixels_shaded() > 0);
    }
}
