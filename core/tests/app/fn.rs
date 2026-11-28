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

#[test]
fn unresponsive_ui_recovers_from_swallowed_pointer_up_and_pointer_cancel() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let cancels: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let btn_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let clicks_c: Rc<Cell<u32>> = clicks.clone();
    let cancels_c: Rc<Cell<u32>> = cancels.clone();
    let btn_c: Rc<Cell<NodeId>> = btn_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(240.0))
                .padding(Edges::all(20.0)),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(140.0))
                .height(Length::Px(48.0))
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            clicks_c.set(clicks_c.get().saturating_add(1));
        });
        ctx.on_pointer_cancel(btn, move |_: &UiEvent| {
            cancels_c.set(cancels_c.get().saturating_add(1));
        });
        btn_c.set(btn);
        ctx.append(root, btn);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let btn_id: NodeId = btn_cell.get();
    let hit_pt: Point = Point::new(60.0, 44.0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 1);
    assert!(cancels.get() >= 1);
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            10,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    for _ in 0_u32..55 {
        let _: FrameStats = app.step_game_frame(0.05).unwrap();
    }
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            11,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            11,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 2);
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            20,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            hit_pt,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 3);
    let cancels_before: u32 = cancels.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            30,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerCancel(wid, 30));
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
    assert!(cancels.get() > cancels_before);
    assert_eq!(
        app.get_context().get_swipe_drag_offset().get_untracked(),
        0.0
    );
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            31,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerCancel(wid, 0));
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            32,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            32,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 4);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            33,
            PointerDevice::Touch,
            hit_pt,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.get_scene().mutate_style(btn_id, |s: &mut Style| {
        let _: &mut LayoutStyle = s.get_mut_layout().set_display(Display::None);
    });
    let _: FrameStats = app.step_frame().unwrap();
    let pressed_after_hide: Option<NodeId> = app
        .get_scene()
        .with_read(|tree: &SceneTree| tree.get_focus().pressed_node(), None);
    assert_eq!(pressed_after_hide, None);
}

#[test]
fn invisible_portals_never_swallow_clicks_while_visible_portal_enforces_tier0_barrier() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let bg_clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let modal_clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let portal_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let root_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let bg_c: Rc<Cell<u32>> = bg_clicks.clone();
    let modal_c: Rc<Cell<u32>> = modal_clicks.clone();
    let portal_c: Rc<Cell<NodeId>> = portal_cell.clone();
    let root_c: Rc<Cell<NodeId>> = root_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(240.0)),
        );
        let bg_btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(240.0))
                .background(Color::hex(0x0F172A)),
        );
        ctx.on_click(bg_btn, move |_: &UiEvent| {
            bg_c.set(bg_c.get().saturating_add(1));
        });
        let mut portal_style: Style = Style::default()
            .width(Length::Px(360.0))
            .height(Length::Px(240.0))
            .z_index(100)
            .background(Color::rgba(0, 0, 0, 128));
        portal_style
            .get_mut_layout()
            .set_position(Position::Fixed)
            .set_top(Some(0.0))
            .set_right(Some(0.0))
            .set_bottom(Some(0.0))
            .set_left(Some(0.0));
        let portal: NodeId = ctx.portal(portal_style);
        let modal_btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(120.0))
                .height(Length::Px(60.0))
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(modal_btn, move |_: &UiEvent| {
            modal_c.set(modal_c.get().saturating_add(1));
        });
        ctx.append(portal, modal_btn);
        ctx.append_many(root, &[bg_btn, portal]);
        portal_c.set(portal);
        root_c.set(root);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let portal_id: NodeId = portal_cell.get();
    let root_id: NodeId = root_cell.get();
    let inside_modal: Point = Point::new(40.0, 30.0);
    let outside_modal: Point = Point::new(260.0, 180.0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            outside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            outside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(bg_clicks.get(), 0);
    assert_eq!(modal_clicks.get(), 0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(modal_clicks.get(), 1);
    assert_eq!(bg_clicks.get(), 0);
    app.get_scene().mutate_style(portal_id, |s: &mut Style| {
        let _: &mut PaintStyle = s.get_mut_paint().set_opacity(0.0);
    });
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            3,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            3,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(bg_clicks.get(), 1);
    app.get_scene().mutate_style(portal_id, |s: &mut Style| {
        let _: &mut PaintStyle = s.get_mut_paint().set_opacity(0.0005);
    });
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            4,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            4,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(bg_clicks.get(), 2);
    app.get_scene().mutate_style(portal_id, |s: &mut Style| {
        let _: &mut PaintStyle = s.get_mut_paint().set_opacity(1.0);
        let _: &mut LayoutStyle = s.get_mut_layout().set_display(Display::None);
    });
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            5,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            5,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(bg_clicks.get(), 3);
    app.get_scene().mutate_style(portal_id, |s: &mut Style| {
        let _: &mut LayoutStyle = s.get_mut_layout().set_display(Display::Flex);
        let _: &mut PaintStyle = s.get_mut_paint().set_pointer_events(PointerEvents::None);
    });
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            6,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            6,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(bg_clicks.get(), 4);
    app.get_scene().mutate_style(portal_id, |s: &mut Style| {
        let _: &mut PaintStyle = s.get_mut_paint().set_pointer_events(PointerEvents::Auto);
    });
    app.get_scene().remove_and_dispose_child(root_id, portal_id);
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            7,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            7,
            PointerDevice::Touch,
            inside_modal,
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(bg_clicks.get(), 5);
}

#[test]
fn fifty_cycle_rapid_exit_terminate_background_reentry_keeps_ui_100_percent_interactive() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let click_sig: Signal<u32> = Signal::new(0);
    let effect_runs: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let scroll_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let eff_c: Rc<Cell<u32>> = effect_runs.clone();
    let sc_c: Rc<Cell<NodeId>> = scroll_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let exit_ctx: Context = ctx.clone();
        let _: SystemEventSubscription = on_system_back(move || {
            exit_ctx.request_exit();
        });
        let _: Effect = Effect::new(move || {
            let _: u32 = click_sig.get();
            eff_c.set(eff_c.get().saturating_add(1));
        });
        let scroll: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(320.0)),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(200.0))
                .height(Length::Px(60.0))
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            let _: bool = click_sig.update(|v: &mut u32| *v = v.saturating_add(1));
        });
        let spacer: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(1200.0)),
        );
        ctx.append_many(scroll, &[btn, spacer]);
        sc_c.set(scroll);
        scroll
    });
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(effect_runs.get(), 1);
    let scroll_id: NodeId = scroll_cell.get();
    for cycle in 0_u32..50 {
        let pid_base: u64 = u64::from(cycle) * 10 + 1;
        match cycle % 5 {
            0 => {
                app.dispatch_window_event(WindowEvent::PointerDown(
                    wid,
                    PointerEvent::new(
                        pid_base,
                        PointerDevice::Touch,
                        Point::new(80.0, 120.0),
                        PointerButton::Primary,
                        1.0,
                        Modifiers::default(),
                    ),
                ));
                app.dispatch_window_event(WindowEvent::PointerMove(
                    wid,
                    PointerEvent::new(
                        pid_base,
                        PointerDevice::Touch,
                        Point::new(80.0, 90.0),
                        PointerButton::Primary,
                        1.0,
                        Modifiers::default(),
                    ),
                ));
                app.dispatch_system_event(SystemEvent::Inactive);
                app.dispatch_system_event(SystemEvent::Background);
                app.dispatch_system_event(SystemEvent::Foreground);
            }
            1 => {
                app.dispatch_system_event(SystemEvent::BackPressed);
                app.dispatch_window_event(WindowEvent::CloseRequested(wid));
                app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Resumed));
            }
            2 => {
                app.dispatch_system_event(SystemEvent::Background);
                app.dispatch_system_event(SystemEvent::Background);
                app.dispatch_system_event(SystemEvent::Disappear);
                app.dispatch_system_event(SystemEvent::Terminate);
                app.dispatch_system_event(SystemEvent::Background);
                app.dispatch_system_event(SystemEvent::Appear);
                app.dispatch_system_event(SystemEvent::Foreground);
            }
            3 => {
                app.dispatch_window_event(WindowEvent::Focused(wid, true));
                app.dispatch_system_event(SystemEvent::Background);
                app.dispatch_window_event(WindowEvent::Focused(wid, true));
            }
            _ => {
                app.dispatch_window_event(WindowEvent::PointerDown(
                    wid,
                    PointerEvent::new(
                        pid_base,
                        PointerDevice::Touch,
                        Point::new(80.0, 150.0),
                        PointerButton::Primary,
                        1.0,
                        Modifiers::default(),
                    ),
                ));
                app.dispatch_window_event(WindowEvent::PointerMove(
                    wid,
                    PointerEvent::new(
                        pid_base,
                        PointerDevice::Touch,
                        Point::new(80.0, 110.0),
                        PointerButton::Primary,
                        1.0,
                        Modifiers::default(),
                    ),
                ));
                app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Paused));
                app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Terminated));
                app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Resumed));
            }
        }
        assert!(!app.is_background_frozen());
        assert!(app.get_running());
        assert!(app.should_pump_frames());
        assert_eq!(app.app_state(), AppState::Active);
        assert_eq!(app.active_gesture(), ActiveGesture::Idle);
        assert!(!app.get_context().get_exit_requested().get_untracked());
        app.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.set_scroll_offset(scroll_id, Point::new(0.0, 0.0));
            },
            (),
        );
        let _: FrameStats = app.step_frame().unwrap();
        let tap_pid: u64 = pid_base + 5;
        app.dispatch_window_event(WindowEvent::PointerDown(
            wid,
            PointerEvent::new(
                tap_pid,
                PointerDevice::Touch,
                Point::new(60.0, 30.0),
                PointerButton::Primary,
                1.0,
                Modifiers::default(),
            ),
        ));
        app.dispatch_window_event(WindowEvent::PointerUp(
            wid,
            PointerEvent::new(
                tap_pid,
                PointerDevice::Touch,
                Point::new(60.0, 30.0),
                PointerButton::Primary,
                0.0,
                Modifiers::default(),
            ),
        ));
        let _: FrameStats = app.step_frame().unwrap();
        let expected_clicks: u32 = cycle + 1;
        assert_eq!(click_sig.get_untracked(), expected_clicks);
        assert_eq!(effect_runs.get(), expected_clicks + 1);
        let drag_pid: u64 = pid_base + 7;
        app.dispatch_window_event(WindowEvent::PointerDown(
            wid,
            PointerEvent::new(
                drag_pid,
                PointerDevice::Touch,
                Point::new(120.0, 220.0),
                PointerButton::Primary,
                1.0,
                Modifiers::default(),
            ),
        ));
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::new(
                drag_pid,
                PointerDevice::Touch,
                Point::new(120.0, 140.0),
                PointerButton::Primary,
                1.0,
                Modifiers::default(),
            ),
        ));
        app.dispatch_window_event(WindowEvent::PointerUp(
            wid,
            PointerEvent::new(
                drag_pid,
                PointerDevice::Touch,
                Point::new(120.0, 140.0),
                PointerButton::Primary,
                0.0,
                Modifiers::default(),
            ),
        ));
        let scroll_y: f32 = app.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_node(scroll_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_y()
            },
            0.0,
        );
        assert!(scroll_y > 40.0);
        assert_eq!(click_sig.get_untracked(), expected_clicks);
    }
    assert_eq!(click_sig.get_untracked(), 50);
    assert_eq!(effect_runs.get(), 51);
}

#[test]
fn gesture_arena_vertical_scroll_cancels_pressed_button_and_suppresses_click() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let cancels: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let scroll_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let clicks_c: Rc<Cell<u32>> = clicks.clone();
    let cancels_c: Rc<Cell<u32>> = cancels.clone();
    let sc_c: Rc<Cell<NodeId>> = scroll_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let scroll: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(240.0)),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(320.0))
                .height(Length::Px(80.0))
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            clicks_c.set(clicks_c.get().saturating_add(1));
        });
        ctx.on_pointer_cancel(btn, move |_: &UiEvent| {
            cancels_c.set(cancels_c.get().saturating_add(1));
        });
        let filler: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(320.0))
                .height(Length::Px(1200.0)),
        );
        ctx.append_many(scroll, &[btn, filler]);
        sc_c.set(scroll);
        scroll
    });
    let _: FrameStats = app.step_frame().unwrap();
    let scroll_id: NodeId = scroll_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(80.0, 40.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(80.0, 45.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(80.0, 45.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 1);
    assert_eq!(cancels.get(), 0);
    assert_eq!(
        app.get_context()
            .get_gesture()
            .get_untracked()
            .map(|g: GestureEvent| g.get_kind()),
        Some(GestureEventKind::Tap)
    );
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(80.0, 60.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(80.0, 20.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(
        app.active_gesture(),
        ActiveGesture::VerticalScroll(scroll_id)
    );
    assert_eq!(cancels.get(), 1);
    let pressed_during_scroll: Option<NodeId> = app
        .get_scene()
        .with_read(|tree: &SceneTree| tree.get_focus().pressed_node(), None);
    assert_eq!(pressed_during_scroll, None);
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(80.0, 20.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 1);
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
    let scroll_y: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_y()
        },
        0.0,
    );
    assert!(scroll_y > 20.0);
}

#[test]
fn gesture_arena_horizontal_scroll_beats_route_swipe_while_non_scrollable_allows_route_swipe() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 260).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let code_clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let card_clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let h_scroll_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let code_c: Rc<Cell<u32>> = code_clicks.clone();
    let card_c: Rc<Cell<u32>> = card_clicks.clone();
    let hs_c: Rc<Cell<NodeId>> = h_scroll_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(260.0)),
        );
        let h_scroll: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(120.0)),
        );
        let wide_code: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(960.0))
                .height(Length::Px(100.0))
                .background(Color::hex(0x1E293B)),
        );
        ctx.on_click(wide_code, move |_: &UiEvent| {
            code_c.set(code_c.get().saturating_add(1));
        });
        ctx.append(h_scroll, wide_code);
        let plain_card: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(120.0))
                .background(Color::hex(0x334155)),
        );
        ctx.on_click(plain_card, move |_: &UiEvent| {
            card_c.set(card_c.get().saturating_add(1));
        });
        ctx.append_many(root, &[h_scroll, plain_card]);
        hs_c.set(h_scroll);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let h_scroll_id: NodeId = h_scroll_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(260.0, 50.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(140.0, 50.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(
        app.active_gesture(),
        ActiveGesture::HorizontalScroll(h_scroll_id)
    );
    assert_eq!(
        app.get_context().get_swipe_drag_offset().get_untracked(),
        0.0
    );
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(140.0, 50.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    let scroll_x: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(h_scroll_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_x()
        },
        0.0,
    );
    assert!(scroll_x > 80.0);
    assert_eq!(code_clicks.get(), 0);
    assert!(app.get_context().get_gesture().get_untracked().is_none());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(260.0, 180.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(130.0, 180.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(app.active_gesture(), ActiveGesture::RouteSwipe);
    assert!(app.get_context().get_swipe_drag_offset().get_untracked() < -100.0);
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(130.0, 180.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(
        app.get_context().get_swipe_drag_offset().get_untracked(),
        0.0
    );
    assert_eq!(card_clicks.get(), 0);
    assert_eq!(
        app.get_context()
            .get_gesture()
            .get_untracked()
            .map(|g: GestureEvent| g.get_kind()),
        Some(GestureEventKind::SwipeLeft)
    );
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
}

#[test]
fn gesture_arena_inertial_fling_tap_to_stop_arrests_fling_and_suppresses_click() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 260).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let scroll_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let clicks_c: Rc<Cell<u32>> = clicks.clone();
    let sc_c: Rc<Cell<NodeId>> = scroll_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let scroll: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(260.0)),
        );
        let tall_btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(2400.0))
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(tall_btn, move |_: &UiEvent| {
            clicks_c.set(clicks_c.get().saturating_add(1));
        });
        ctx.append(scroll, tall_btn);
        sc_c.set(scroll);
        scroll
    });
    let _: FrameStats = app.step_game_frame(0.016).unwrap();
    let scroll_id: NodeId = scroll_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(120.0, 220.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    let mut y_pos: f32 = 220.0;
    for _ in 0_u32..4 {
        y_pos -= 30.0;
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::new(
                1,
                PointerDevice::Touch,
                Point::new(120.0, y_pos),
                PointerButton::Primary,
                1.0,
                Modifiers::default(),
            ),
        ));
        let _: FrameStats = app.step_game_frame(0.016).unwrap();
    }
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(120.0, y_pos),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert!(app.get_scroll_velocity().get_y() > 100.0);
    let y_before_coast: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_y()
        },
        0.0,
    );
    let _: FrameStats = app.step_game_frame(0.016).unwrap();
    let y_after_coast: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_y()
        },
        0.0,
    );
    assert!(y_after_coast > y_before_coast + 2.0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(120.0, 120.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(app.get_scroll_velocity(), Point::default());
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(120.0, 120.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            3,
            PointerDevice::Touch,
            Point::new(120.0, 120.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            3,
            PointerDevice::Touch,
            Point::new(120.0, 120.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 1);
}

#[test]
fn gesture_arena_wheel_scroll_isolation_and_direct_drag_priority() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 260).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let clicks: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let cancels: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let drag_moves: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let scroll_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let drag_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let clicks_c: Rc<Cell<u32>> = clicks.clone();
    let cancels_c: Rc<Cell<u32>> = cancels.clone();
    let moves_c: Rc<Cell<u32>> = drag_moves.clone();
    let sc_c: Rc<Cell<NodeId>> = scroll_cell.clone();
    let dg_c: Rc<Cell<NodeId>> = drag_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let scroll: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(260.0)),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(320.0))
                .height(Length::Px(80.0))
                .background(Color::hex(0x2563EB)),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            clicks_c.set(clicks_c.get().saturating_add(1));
        });
        ctx.on_pointer_cancel(btn, move |_: &UiEvent| {
            cancels_c.set(cancels_c.get().saturating_add(1));
        });
        let slider: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(320.0))
                .height(Length::Px(60.0))
                .background(Color::hex(0x0EA5E9)),
        );
        ctx.on_pointer_down(slider, |_: &UiEvent| {});
        ctx.on_pointer_move(slider, move |_: &UiEvent| {
            moves_c.set(moves_c.get().saturating_add(1));
        });
        let filler: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(320.0))
                .height(Length::Px(1200.0)),
        );
        ctx.append_many(scroll, &[btn, slider, filler]);
        sc_c.set(scroll);
        dg_c.set(slider);
        scroll
    });
    let _: FrameStats = app.step_frame().unwrap();
    let scroll_id: NodeId = scroll_cell.get();
    let drag_id: NodeId = drag_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(80.0, 40.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 60.0)));
    assert_eq!(cancels.get(), 1);
    assert_eq!(
        app.get_context().get_swipe_drag_offset().get_untracked(),
        0.0
    );
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(80.0, 40.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(clicks.get(), 0);
    let after_wheel_y: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_y()
        },
        0.0,
    );
    assert!((after_wheel_y - 60.0).abs() < 0.5);
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.set_scroll_offset(scroll_id, Point::new(0.0, 0.0));
        },
        (),
    );
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(80.0, 110.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(140.0, 110.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(app.active_gesture(), ActiveGesture::DirectDrag(drag_id));
    assert!(drag_moves.get() >= 1);
    let scroll_after_drag: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_y()
        },
        0.0,
    );
    assert_eq!(scroll_after_drag, 0.0);
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            2,
            PointerDevice::Touch,
            Point::new(140.0, 110.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
}

#[test]
fn reactive_safe_area_delayed_atomic_island_and_immersive_fullscreen_commands() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(390, 844).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let safe_updates: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fs_updates: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let safe_c: Rc<Cell<u32>> = safe_updates.clone();
    let fs_c: Rc<Cell<u32>> = fs_updates.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let safe_sig: Signal<SafeAreaInsets> = ctx.safe_area_signal();
        let fs_sig: Signal<bool> = ctx.immersive_fullscreen_signal();
        let _: Effect = Effect::new(move || {
            let _: SafeAreaInsets = safe_sig.get();
            safe_c.set(safe_c.get().saturating_add(1));
        });
        let _: Effect = Effect::new(move || {
            let _: bool = fs_sig.get();
            fs_c.set(fs_c.get().saturating_add(1));
        });
        ctx.view(
            Style::default()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .background(Color::hex(0x0F172A)),
        )
    });
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.safe_area_signal().get_untracked(),
        SafeAreaInsets::default()
    );
    assert_eq!(safe_updates.get(), 1);
    assert!(!app.is_immersive_fullscreen());
    assert_eq!(fs_updates.get(), 1);
    let delayed: SafeAreaInsets = SafeAreaInsets::new(47.0, 0.0, 34.0, 0.0);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(wid, delayed));
    assert_eq!(app.get_context().safe_area(), delayed);
    assert_eq!(safe_updates.get(), 2);
    let relayout_stats: FrameStats = app.step_frame().unwrap();
    assert!(relayout_stats.get_layout_nodes() > 0);
    let island: SafeAreaInsets = SafeAreaInsets::new(59.0, 0.0, 34.0, 0.0);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(wid, island));
    assert_eq!(app.safe_area_signal().get_untracked(), island);
    assert_eq!(safe_updates.get(), 3);
    let malformed: SafeAreaInsets = SafeAreaInsets::new(-12.0, f32::NAN, 24.0, f32::INFINITY);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(wid, malformed));
    assert_eq!(
        app.get_context().safe_area(),
        SafeAreaInsets::new(0.0, 0.0, 24.0, 0.0)
    );
    app.get_context().enter_immersive_fullscreen();
    app.get_context().enter_immersive_fullscreen();
    assert!(app.is_immersive_fullscreen());
    assert!(app.get_context().is_immersive_fullscreen());
    assert_eq!(fs_updates.get(), 2);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_window_state().get_fullscreen(),
        FullscreenMode::Borderless
    );
    app.get_context().exit_immersive_fullscreen();
    assert!(app.is_immersive_fullscreen());
    app.get_context().exit_immersive_fullscreen();
    assert!(!app.is_immersive_fullscreen());
    assert_eq!(fs_updates.get(), 3);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_window_state().get_fullscreen(),
        FullscreenMode::Windowed
    );
    app.set_immersive_fullscreen(true);
    assert!(app.is_immersive_fullscreen());
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_window_state().get_fullscreen(),
        FullscreenMode::Borderless
    );
    app.set_immersive_fullscreen(false);
    assert!(!app.is_immersive_fullscreen());
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_window_state().get_fullscreen(),
        FullscreenMode::Windowed
    );
}

#[test]
fn horizontal_route_swipe_crossing_code_block_never_scrolls_code_block() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let code_scroll_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let cs_c: Rc<Cell<NodeId>> = code_scroll_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let page: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(320.0)),
        );
        let header: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(80.0))
                .background(Color::hex(0x0F172A)),
        );
        let code_box: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(140.0)),
        );
        let wide_code: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(1000.0))
                .height(Length::Px(120.0))
                .background(Color::hex(0x1E293B)),
        );
        ctx.append(code_box, wide_code);
        ctx.append_many(page, &[header, code_box]);
        cs_c.set(code_box);
        page
    });
    let _: FrameStats = app.step_frame().unwrap();
    let code_id: NodeId = code_scroll_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(300.0, 40.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(220.0, 40.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(app.active_gesture(), ActiveGesture::RouteSwipe);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(100.0, 140.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(100.0, 140.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    let code_scroll_x: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(code_id)
                .unwrap()
                .get_layout_box()
                .get_scroll_offset()
                .get_x()
        },
        0.0,
    );
    assert_eq!(code_scroll_x, 0.0);
    assert_eq!(app.active_gesture(), ActiveGesture::Idle);
}

#[test]
fn horizontal_code_block_drag_past_edge_never_triggers_route_swipe_or_parent_scroll() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let outer_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let inner_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let oc: Rc<Cell<NodeId>> = outer_cell.clone();
    let ic: Rc<Cell<NodeId>> = inner_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let outer: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(320.0)),
        );
        let wide_outer_row: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(900.0))
                .height(Length::Px(40.0)),
        );
        let code_box: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(140.0)),
        );
        let wide_code: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(500.0))
                .height(Length::Px(120.0))
                .background(Color::hex(0x1E293B)),
        );
        ctx.append(code_box, wide_code);
        ctx.append_many(outer, &[wide_outer_row, code_box]);
        oc.set(outer);
        ic.set(code_box);
        outer
    });
    let _: FrameStats = app.step_frame().unwrap();
    let outer_id: NodeId = outer_cell.get();
    let inner_id: NodeId = inner_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(320.0, 100.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(20.0, 100.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    assert_eq!(
        app.active_gesture(),
        ActiveGesture::HorizontalScroll(inner_id)
    );
    assert_eq!(
        app.get_context().get_swipe_drag_offset().get_untracked(),
        0.0
    );
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(20.0, 100.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    let (inner_x, outer_x): (f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(inner_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
                tree.get_node(outer_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
            )
        },
        (0.0, 0.0),
    );
    assert!(inner_x > 100.0);
    assert_eq!(outer_x, 0.0);
    assert_eq!(
        app.get_context().get_swipe_drag_offset().get_untracked(),
        0.0
    );
    assert!(app.get_context().get_gesture().get_untracked().is_none());
}

#[test]
fn diagonal_vertical_scroll_over_code_block_never_shifts_code_block_horizontally() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let page_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let code_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let pc: Rc<Cell<NodeId>> = page_cell.clone();
    let cc: Rc<Cell<NodeId>> = code_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let page: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(320.0)),
        );
        let code_box: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(140.0)),
        );
        let wide_code: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(1000.0))
                .height(Length::Px(120.0))
                .background(Color::hex(0x1E293B)),
        );
        ctx.append(code_box, wide_code);
        let filler: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(1200.0)),
        );
        ctx.append_many(page, &[code_box, filler]);
        pc.set(page);
        cc.set(code_box);
        page
    });
    let _: FrameStats = app.step_frame().unwrap();
    let page_id: NodeId = page_cell.get();
    let code_id: NodeId = code_cell.get();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(180.0, 100.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(174.0, 60.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(150.0, 20.0),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Touch,
            Point::new(150.0, 20.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    let (page_y, code_x): (f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(page_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_y(),
                tree.get_node(code_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
            )
        },
        (0.0, 0.0),
    );
    assert!(page_y > 50.0);
    assert_eq!(code_x, 0.0);
}

#[test]
fn wheel_horizontal_scroll_isolates_single_code_block_and_never_leaks_outside_hover() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 320).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let outer_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let code_a_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let code_b_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let oc: Rc<Cell<NodeId>> = outer_cell.clone();
    let ca: Rc<Cell<NodeId>> = code_a_cell.clone();
    let cb: Rc<Cell<NodeId>> = code_b_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let outer: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(320.0)),
        );
        let header: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(360.0))
                .height(Length::Px(60.0)),
        );
        let code_a: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(100.0)),
        );
        let wide_a: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(900.0))
                .height(Length::Px(80.0)),
        );
        ctx.append(code_a, wide_a);
        let code_b: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(100.0)),
        );
        let wide_b: NodeId = ctx.view(
            Style::default()
                .min_width(Length::Px(900.0))
                .height(Length::Px(80.0)),
        );
        ctx.append(code_b, wide_b);
        ctx.append_many(outer, &[header, code_a, code_b]);
        oc.set(outer);
        ca.set(code_a);
        cb.set(code_b);
        outer
    });
    let _: FrameStats = app.step_frame().unwrap();
    let outer_id: NodeId = outer_cell.get();
    let code_a_id: NodeId = code_a_cell.get();
    let code_b_id: NodeId = code_b_cell.get();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(100.0, 20.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(120.0, 0.0)));
    let (ax_0, bx_0, ox_0): (f32, f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(code_a_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
                tree.get_node(code_b_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
                tree.get_node(outer_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
            )
        },
        (0.0, 0.0, 0.0),
    );
    assert_eq!(ax_0, 0.0);
    assert_eq!(bx_0, 0.0);
    assert_eq!(ox_0, 0.0);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            1,
            PointerDevice::Mouse,
            Point::new(100.0, 100.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(90.0, 12.0)));
    let (ax_1, bx_1, oy_1): (f32, f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            (
                tree.get_node(code_a_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
                tree.get_node(code_b_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_x(),
                tree.get_node(outer_id)
                    .unwrap()
                    .get_layout_box()
                    .get_scroll_offset()
                    .get_y(),
            )
        },
        (0.0, 0.0, 0.0),
    );
    assert!((ax_1 - 90.0).abs() < 0.5);
    assert_eq!(bx_1, 0.0);
    assert_eq!(oy_1, 0.0);
}

#[test]
fn newly_created_subtree_text_updates_never_skip_frames_after_initial_layout() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let host_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let hc: Rc<Cell<NodeId>> = host_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let host: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(360.0))
                .height(Length::Px(240.0))
                .background(Color::hex(0xFFFFFF)),
        );
        hc.set(host);
        host
    });
    let _: FrameStats = app.step_frame().unwrap();
    let host_id: NodeId = host_cell.get();
    let txt_id: NodeId = app.get_scene().with_write(
        |tree: &mut SceneTree| {
            let label: NodeId = tree.create_text(
                "Counter: 1".to_string(),
                Style::default()
                    .width(Length::Px(220.0))
                    .height(Length::Px(32.0))
                    .font_size(18.0)
                    .color(Color::hex(0x0F172A)),
            );
            tree.append_child(host_id, label);
            label
        },
        NodeId::default(),
    );
    let f1: FrameStats = app.step_frame().unwrap();
    assert!(f1.get_pixels_shaded() > 0);
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.set_text(txt_id, "Counter: 2".to_string());
        },
        (),
    );
    let f2: FrameStats = app.step_frame().unwrap();
    assert!(f2.get_pixels_shaded() > 0);
    assert_ne!(f2.get_frame_hash(), f1.get_frame_hash());
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.set_text(txt_id, "Counter: 12".to_string());
        },
        (),
    );
    let f3: FrameStats = app.step_frame().unwrap();
    assert!(f3.get_pixels_shaded() > 0);
    assert_ne!(f3.get_frame_hash(), f2.get_frame_hash());
}
