use super::*;

fn find_text_center(app: &App, label: &str) -> Point {
    app.get_scene().with_read(
        |tree: &SceneTree| {
            for (idx, node) in tree.get_nodes().iter().enumerate() {
                if !node.get_alive() || node.get_style().get_layout().get_display() == Display::None
                {
                    continue;
                }
                if let NodeKind::Text(text) = node.get_kind()
                    && text.as_str() == label
                {
                    let nid: NodeId = NodeId::new(idx as u32, node.get_generation());
                    let bounds: Rect = tree
                        .node_screen_bounds(nid)
                        .unwrap_or_else(|| node.get_layout_box().get_bounds());
                    if bounds.get_width() > 0.0 && bounds.get_height() > 0.0 {
                        return Point::new(
                            bounds.get_x() + bounds.get_width() * 0.5,
                            bounds.get_y() + bounds.get_height() * 0.5,
                        );
                    }
                }
            }
            Point::new(-1.0, -1.0)
        },
        Point::new(-1.0, -1.0),
    )
}

fn find_input_center_by_placeholder(app: &App, placeholder: &str) -> Point {
    app.get_scene().with_read(
        |tree: &SceneTree| {
            for node in tree.get_nodes().iter() {
                if !node.get_alive() || node.get_style().get_layout().get_display() == Display::None
                {
                    continue;
                }
                if let NodeKind::Input(inp) = node.get_kind()
                    && inp.get_placeholder().as_str() == placeholder
                {
                    let bounds: Rect = node.get_layout_box().get_bounds();
                    if bounds.get_width() > 0.0 && bounds.get_height() > 0.0 {
                        return Point::new(
                            bounds.get_x() + bounds.get_width() * 0.5,
                            bounds.get_y() + bounds.get_height() * 0.5,
                        );
                    }
                }
            }
            Point::new(-1.0, -1.0)
        },
        Point::new(-1.0, -1.0),
    )
}

fn click_at(app: &mut App, point: Point) -> FrameStats {
    assert!(
        point.get_x() >= 0.0 && point.get_y() >= 0.0,
        "target widget was not laid out on screen"
    );
    let wid: WindowId = app.get_window_state().get_id();
    let ev: PointerEvent = PointerEvent::mouse(point.get_x(), point.get_y());
    app.dispatch_window_event(WindowEvent::PointerMove(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
    app.step_frame().unwrap()
}

fn click_label(app: &mut App, label: &str) -> FrameStats {
    let pt: Point = find_text_center(app, label);
    assert!(
        pt.get_x() >= 0.0,
        "expected visible clickable label `{label}` in laid-out scene"
    );
    click_at(app, pt)
}

fn scroll_and_click_label(app: &mut App, label: &str) -> FrameStats {
    let mut pt: Point = find_text_center(app, label);
    assert!(
        pt.get_x() >= 0.0,
        "expected clickable label `{label}` in laid-out scene"
    );
    if pt.get_y() > 740.0 || pt.get_y() < 40.0 {
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerMove(
            wid,
            PointerEvent::mouse(600.0, 400.0),
        ));
        app.dispatch_window_event(WindowEvent::Scroll(
            wid,
            Point::new(0.0, pt.get_y() - 360.0),
        ));
        let _: FrameStats = app.step_frame().unwrap();
        pt = find_text_center(app, label);
    }
    click_at(app, pt)
}

#[test]
fn real_interactive_e2e_clicking_navigates_all_nine_routes_and_exercises_every_feature() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let frame_overview: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame_overview.get_frame_hash(), 0);
    let f_reactive_0: FrameStats = click_label(&mut app, "Reactivity");
    assert_eq!(state.get_route().get_untracked(), ROUTE_REACTIVE);
    assert_ne!(
        f_reactive_0.get_frame_hash(),
        frame_overview.get_frame_hash()
    );
    let f_inc: FrameStats = click_label(&mut app, "Increment (+1)");
    assert_eq!(state.get_counter().get_untracked(), 2);
    assert_eq!(state.get_doubled().get(), 4);
    assert_ne!(f_inc.get_frame_hash(), f_reactive_0.get_frame_hash());
    let f_batch: FrameStats = click_label(&mut app, "Batch (+10)");
    assert_eq!(state.get_counter().get_untracked(), 12);
    assert_eq!(state.get_doubled().get(), 24);
    assert_ne!(f_batch.get_frame_hash(), f_inc.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Reset");
    assert_eq!(state.get_counter().get_untracked(), 0);
    let f_render_0: FrameStats = click_label(&mut app, "Rendering");
    assert_eq!(state.get_route().get_untracked(), ROUTE_RENDER);
    let f_render_hw: FrameStats = click_label(&mut app, "Hardware");
    assert_eq!(state.get_render_mode_index().get_untracked(), 2);
    assert_ne!(f_render_hw.get_frame_hash(), f_render_0.get_frame_hash());
    let f_win_0: FrameStats = click_label(&mut app, "Window & OS");
    assert_eq!(state.get_route().get_untracked(), ROUTE_WINDOW);
    let f_island: FrameStats = click_label(&mut app, "Toggle Dynamic Island");
    assert!(state.get_island_active().get_untracked());
    assert_eq!(state.get_island_progress().get_untracked(), 50);
    assert_ne!(f_island.get_frame_hash(), f_win_0.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Register euv:// Protocol");
    assert!(
        state
            .get_protocol_status()
            .get_untracked()
            .contains("Registered")
    );
    let _: FrameStats = click_label(&mut app, "Request Camera Permission");
    assert!(
        state
            .get_camera_perm_status()
            .get_untracked()
            .contains("Granted")
    );
    let _: FrameStats = click_label(&mut app, "Announce A11y + Haptic");
    assert_eq!(state.get_a11y_announcements().get_untracked(), 1);
    assert_eq!(state.get_haptic_pulses().get_untracked(), 1);
    let a11y: A11yTree = app.a11y_tree();
    assert!(a11y.get_nodes().len() >= 10);
    let f_forms_0: FrameStats = click_label(&mut app, "Forms");
    assert_eq!(state.get_route().get_untracked(), ROUTE_FORMS);
    let inp_pt: Point = find_input_center_by_placeholder(&app, PLACEHOLDER_USERNAME);
    let _: FrameStats = click_at(&mut app, inp_pt);
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::TextInput(wid, "_ok!".to_string()));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Backspace, Modifiers::default(), false),
    ));
    let f_typed: FrameStats = app.step_frame().unwrap();
    assert!(state.get_username().get_untracked().ends_with("_ok"));
    assert_ne!(f_typed.get_frame_hash(), f_forms_0.get_frame_hash());
    let bio_pt: Point = find_input_center_by_placeholder(&app, "Write notes...");
    let _: FrameStats = click_at(&mut app, bio_pt);
    app.dispatch_window_event(WindowEvent::TextInput(wid, " + E2E".to_string()));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(state.get_bio().get_untracked().ends_with(" + E2E"));
    let _: FrameStats = click_label(&mut app, "Pin Config");
    assert!(state.get_pin_toggled().get_untracked());
    let _: FrameStats = click_label(&mut app, "Prefer Hardware GPU Pipeline");
    assert!(!state.get_hardware_accel().get_untracked());
    let _: FrameStats = click_label(&mut app, "Enable deterministic frame telemetry");
    assert!(!state.get_agree_terms().get_untracked());
    let f_themes_0: FrameStats = click_label(&mut app, "Themes (17)");
    assert_eq!(state.get_route().get_untracked(), ROUTE_THEMES);
    let initial_scheme: DesignScheme = state.get_theme().get_scheme().get_untracked();
    let f_scheme: FrameStats = click_label(&mut app, "Cycle Design Scheme");
    assert_ne!(
        state.get_theme().get_scheme().get_untracked(),
        initial_scheme
    );
    assert_ne!(f_scheme.get_frame_hash(), f_themes_0.get_frame_hash());
    let f_dark: FrameStats = click_label(&mut app, "Toggle Light / Dark");
    assert_eq!(
        state.get_theme().get_mode().get_untracked(),
        ThemeMode::Dark
    );
    assert_ne!(f_dark.get_frame_hash(), f_scheme.get_frame_hash());
    let f_data_0: FrameStats = click_label(&mut app, "Scroll List");
    assert_eq!(state.get_route().get_untracked(), ROUTE_DATA);
    let prev_scroll: f32 = state.get_list_scroll_y().get_untracked();
    let f_scroll: FrameStats = click_label(&mut app, "Scroll Down (+160px)");
    assert!((state.get_list_scroll_y().get_untracked() - (prev_scroll + 160.0)).abs() < 0.01);
    assert_ne!(f_scroll.get_frame_hash(), f_data_0.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Back to Top");
    assert!(state.get_list_scroll_y().get_untracked().abs() < 0.01);
    let f_game_0: FrameStats = click_label(&mut app, "Game Engine");
    assert_eq!(state.get_route().get_untracked(), ROUTE_GAME);
    let f_game_1: FrameStats = click_label(&mut app, "Step Physics (+3 Frames)");
    assert_ne!(f_game_1.get_frame_hash(), f_game_0.get_frame_hash());
    let f_ov_0: FrameStats = click_label(&mut app, "Overlays");
    assert_eq!(state.get_route().get_untracked(), ROUTE_OVERLAYS);
    let f_modal: FrameStats = click_label(&mut app, "Open Dialog Modal");
    assert!(state.get_modal_open().get_untracked());
    assert_ne!(f_modal.get_frame_hash(), f_ov_0.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Close Modal");
    assert!(!state.get_modal_open().get_untracked());
    let _: FrameStats = click_label(&mut app, "Toggle Toast Banner");
    assert!(state.get_toast_visible().get_untracked());
    let _: FrameStats = click_label(&mut app, "Toggle Drawer");
    assert!(state.get_drawer_open().get_untracked());
    let _: FrameStats = click_label(&mut app, "Close Drawer");
    assert!(!state.get_drawer_open().get_untracked());
    let _: FrameStats = click_label(&mut app, "Toggle Sheet");
    assert!(state.get_sheet_open().get_untracked());
    let _: FrameStats = click_label(&mut app, "Close Sheet");
    assert!(!state.get_sheet_open().get_untracked());
    let _: FrameStats = click_label(&mut app, "Alert Dialog");
    assert!(state.get_alert_dialog_open().get_untracked());
    let _: FrameStats = click_label(&mut app, "Continue");
    assert!(!state.get_alert_dialog_open().get_untracked());
    assert!(state.get_alert_dialog_confirmed().get_untracked());
    let _: FrameStats = click_label(&mut app, "@euv-dev");
    assert!(state.get_hover_card_open().get_untracked());
    let f_dyn_0: FrameStats = click_label(&mut app, "Dynamic WASM");
    assert_eq!(state.get_route().get_untracked(), ROUTE_DYNAMIC);
    let _: FrameStats = scroll_and_click_label(&mut app, "Download Partial WASM v1");
    assert_eq!(
        state.get_dynamic_hub().component_version(SLOT_WASM_WIDGET),
        1
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Increment Sibling (+1)");
    assert_eq!(state.get_sibling_counter().get_untracked(), 1);
    let f_dyn_banner_v1: FrameStats = scroll_and_click_label(&mut app, "Push Banner v1 (JSON)");
    assert_eq!(
        state.get_dynamic_hub().component_version(SLOT_PROMO_BANNER),
        1
    );
    assert_eq!(state.get_sibling_counter().get_untracked(), 1);
    assert_ne!(f_dyn_banner_v1.get_frame_hash(), f_dyn_0.get_frame_hash());
    let _: FrameStats = scroll_and_click_label(&mut app, "Claim Dynamic Offer");
    assert!(
        state
            .get_dynamic_action_log()
            .get_untracked()
            .contains("claim_offer")
    );
    let f_dyn_banner_v2: FrameStats = scroll_and_click_label(&mut app, "Push Banner v2 (Native)");
    assert_eq!(
        state.get_dynamic_hub().component_version(SLOT_PROMO_BANNER),
        2
    );
    assert_ne!(
        f_dyn_banner_v2.get_frame_hash(),
        f_dyn_banner_v1.get_frame_hash()
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Patch Metrics Widget");
    assert_eq!(
        state
            .get_dynamic_hub()
            .component_version(SLOT_METRICS_WIDGET),
        1
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Rollback Banner");
    assert_eq!(
        state.get_dynamic_hub().component_version(SLOT_PROMO_BANNER),
        1
    );
    let f_dyn_page_v1: FrameStats = scroll_and_click_label(&mut app, "Hot-Update Page (v1)");
    assert_eq!(state.get_dynamic_hub().page_version(ROUTE_DYNAMIC_PANEL), 1);
    assert_ne!(
        f_dyn_page_v1.get_frame_hash(),
        f_dyn_banner_v2.get_frame_hash()
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Apply Atomic Bundle");
    assert_eq!(
        state
            .get_dynamic_hub()
            .component_version(SLOT_METRICS_WIDGET),
        2
    );
    assert_eq!(state.get_dynamic_hub().page_version(ROUTE_DYNAMIC_PANEL), 2);
    let f_promo: FrameStats = scroll_and_click_label(&mut app, "Inject & Visit /promo-live");
    assert_eq!(state.get_route().get_untracked(), ROUTE_PROMO_LIVE);
    assert!(state.get_dynamic_hub().has_dynamic_page(ROUTE_PROMO_LIVE));
    assert_ne!(f_promo.get_frame_hash(), f_dyn_page_v1.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Return to Dynamic OTA");
    assert_eq!(state.get_route().get_untracked(), ROUTE_DYNAMIC);
    let _: FrameStats = click_label(&mut app, "Overview");
    assert_eq!(state.get_route().get_untracked(), ROUTE_OVERVIEW);
}

#[test]
fn showcase_supports_render_mode_switching_and_theme_cycling() {
    reset_reactive_runtime();
    let sw_stats: FrameStats = run_headless_smoke(RenderMode::Software).unwrap();
    let hw_stats: FrameStats = run_headless_smoke(RenderMode::Hardware).unwrap();
    let hy_stats: FrameStats = run_headless_smoke(RenderMode::Hybrid).unwrap();
    assert_ne!(sw_stats.get_frame_hash(), 0);
    assert!(hw_stats.get_draw_batches() >= 1);
    assert_eq!(sw_stats.get_frame_hash(), hy_stats.get_frame_hash());
    let state: ShowcaseState = ShowcaseState::new();
    let initial: DesignScheme = state.get_theme().get_scheme().get_untracked();
    state.next_scheme();
    assert_ne!(state.get_theme().get_scheme().get_untracked(), initial);
    assert_eq!(
        state.get_theme().get_mode().get_untracked(),
        ThemeMode::Light
    );
    state.toggle_theme_mode();
    assert_eq!(
        state.get_theme().get_mode().get_untracked(),
        ThemeMode::Dark
    );
    let _: bool = state.get_render_mode_index().set(2);
    assert_eq!(state.selected_render_mode(), RenderMode::Hardware);
}

#[test]
fn instant_responsive_resize_adapts_shell_and_all_ten_pages_without_horizontal_overflow() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let f_desk: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_context().get_breakpoint().get_untracked(),
        Breakpoint::Expanded
    );
    app.queue_window_command(WindowCommand::SetSize(wid, 768, 1024));
    app.dispatch_window_event(WindowEvent::Resized(wid, 768, 1024, 1.0));
    let f_tab: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_context().get_breakpoint().get_untracked(),
        Breakpoint::Medium
    );
    assert_ne!(f_tab.get_frame_hash(), f_desk.get_frame_hash());
    app.queue_window_command(WindowCommand::SetSize(wid, 390, 844));
    app.dispatch_window_event(WindowEvent::Resized(wid, 390, 844, 1.0));
    let f_mob: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        app.get_context().get_breakpoint().get_untracked(),
        Breakpoint::Compact
    );
    assert_ne!(f_mob.get_frame_hash(), f_tab.get_frame_hash());
    let f_menu_open: FrameStats = click_label(&mut app, "Menu");
    assert_ne!(f_menu_open.get_frame_hash(), f_mob.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Reactivity");
    assert_eq!(state.get_route().get_untracked(), ROUTE_REACTIVE);
    let routes: [&str; 15] = [
        ROUTE_OVERVIEW,
        ROUTE_MARKDOWN,
        ROUTE_REACTIVE,
        ROUTE_RENDER,
        ROUTE_WINDOW,
        ROUTE_FORMS,
        ROUTE_THEMES,
        ROUTE_DATA,
        ROUTE_GAME,
        ROUTE_GAME_3D,
        ROUTE_MEDIA,
        ROUTE_LAB,
        ROUTE_OVERLAYS,
        ROUTE_DYNAMIC,
        ROUTE_STRESS,
    ];
    let sizes: [(u32, u32, Breakpoint); 3] = [
        (390, 844, Breakpoint::Compact),
        (768, 1024, Breakpoint::Medium),
        (1280, 800, Breakpoint::Expanded),
    ];
    for (vw, vh, expected_bp) in sizes.into_iter() {
        app.queue_window_command(WindowCommand::SetSize(wid, vw, vh));
        app.dispatch_window_event(WindowEvent::Resized(wid, vw, vh, 1.0));
        let _: FrameStats = app.step_frame().unwrap();
        assert_eq!(
            app.get_context().get_breakpoint().get_untracked(),
            expected_bp
        );
        for route in routes.iter() {
            let _: bool = state.get_route().set_if_changed((*route).to_string());
            let stats: FrameStats = app.step_frame().unwrap();
            assert_ne!(stats.get_frame_hash(), 0);
            let overflows: Vec<(f32, f32, String)> = app.get_scene().with_read(
                |tree: &SceneTree| {
                    let mut out: Vec<(f32, f32, String)> = Vec::new();
                    for node in tree.get_nodes().iter() {
                        if !node.get_alive()
                            || node.get_style().get_layout().get_display() == Display::None
                        {
                            continue;
                        }
                        let b: Rect = node.get_layout_box().get_bounds();
                        if b.get_width() > 0.0 && b.right() > (vw as f32) + 1.5 {
                            let tag: String = match node.get_kind() {
                                NodeKind::Text(t) => format!("Text({t})"),
                                other => format!("{other:?}"),
                            };
                            out.push((b.get_x(), b.right(), tag));
                        }
                    }
                    out
                },
                Vec::new(),
            );
            assert!(
                overflows.is_empty(),
                "route `{route}` overflowed horizontally at {vw}x{vh}: {overflows:?}"
            );
        }
    }
}

#[test]
fn fast_wheel_scroll_scrollbar_slider_drag_animations_touch_scroll_and_game_keys_work_e2e() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    let _: FrameStats = click_label(&mut app, "Scroll List");
    assert_eq!(state.get_route().get_untracked(), ROUTE_DATA);
    assert!(app.has_active_animations());
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert!(!app.has_active_animations());
    let vlist_rect: Rect = app.get_scene().with_read(
        |tree: &SceneTree| {
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if n.get_alive() && n.virtual_scroll_metrics().is_some() {
                    let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                    if let Some(b) = tree.node_screen_bounds(nid) {
                        return b;
                    }
                }
            }
            Rect::default()
        },
        Rect::default(),
    );
    assert!(vlist_rect.get_width() > 100.0 && vlist_rect.get_height() > 100.0);
    let hover_pt: Point = Point::new(
        vlist_rect.get_x() + vlist_rect.get_width() * 0.5,
        vlist_rect.get_y() + vlist_rect.get_height() * 0.5,
    );
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(hover_pt.get_x(), hover_pt.get_y()),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let start_scroll: f32 = state.get_list_scroll_y().get_untracked();
    for _ in 0..25 {
        app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 96.0)));
        let _: FrameStats = app.step_frame().unwrap();
    }
    let end_scroll: f32 = state.get_list_scroll_y().get_untracked();
    assert!(
        end_scroll >= start_scroll + 2000.0,
        "fast wheel scroll stalled: start={start_scroll} end={end_scroll}"
    );
    let synced_metrics: Option<(f32, f32)> = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().find_map(|n: &SceneNode| {
                if n.get_alive() {
                    n.virtual_scroll_metrics()
                } else {
                    None
                }
            })
        },
        None,
    );
    assert!(
        synced_metrics
            .map(|(vy, total_h): (f32, f32)| (vy - end_scroll).abs() < 1.0 && total_h > 100_000.0)
            .unwrap_or(false)
    );
    let sb_x: f32 = vlist_rect.right() - 5.0;
    let sb_mid_y: f32 = vlist_rect.get_y() + vlist_rect.get_height() * 0.5;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(sb_x, sb_mid_y),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::mouse(sb_x, sb_mid_y),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let mid_scroll: f32 = state.get_list_scroll_y().get_untracked();
    assert!(
        mid_scroll > 140_000.0 && mid_scroll < 180_000.0,
        "virtual scrollbar mid-track click expected ~160k, got {mid_scroll}"
    );
    let _: FrameStats = click_label(&mut app, "Forms");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let slider_bounds: Rect = app.get_scene().with_read(
        |tree: &SceneTree| {
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Canvas)
                    && n.get_handlers()
                        .handler_for(&UiEventKind::PointerMove(Point::default()))
                        .is_some()
                {
                    let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                    if let Some(b) = tree.node_screen_bounds(nid) {
                        return b;
                    }
                }
            }
            Rect::default()
        },
        Rect::default(),
    );
    assert!(slider_bounds.get_width() > 80.0);
    let drag_start_x: f32 = slider_bounds.get_x() + slider_bounds.get_width() * 0.15;
    let drag_end_x: f32 = slider_bounds.get_x() + slider_bounds.get_width() * 0.88;
    let slider_mid_y: f32 = slider_bounds.get_y() + slider_bounds.get_height() * 0.5;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(drag_start_x, slider_mid_y),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let val_after_down: f32 = state.get_slider_value().get_untracked();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(drag_end_x, slider_mid_y + 42.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::mouse(drag_end_x, slider_mid_y + 42.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let val_after_drag: f32 = state.get_slider_value().get_untracked();
    assert!(
        val_after_down < 25.0 && val_after_drag > 80.0,
        "slider drag failed: down={val_after_down} drag={val_after_drag}"
    );
    let _: FrameStats = click_label(&mut app, "Overlays");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: FrameStats = click_label(&mut app, "Open Dialog Modal");
    assert!(app.has_active_animations());
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert!(!app.has_active_animations());
    let _: FrameStats = click_at(&mut app, Point::new(12.0, 12.0));
    assert!(!state.get_modal_open().get_untracked());
    assert!(app.has_active_animations());
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert!(!app.has_active_animations());
    let _: FrameStats = click_label(&mut app, "Alert Dialog");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert!(state.get_alert_dialog_open().get_untracked());
    let _: FrameStats = click_at(&mut app, Point::new(12.0, 12.0));
    assert!(!state.get_alert_dialog_open().get_untracked());
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: FrameStats = click_label(&mut app, "Game Engine");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Character('d'), Modifiers::default(), false),
    ));
    let f_game_a: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    let f_game_b: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    app.dispatch_window_event(WindowEvent::KeyUp(
        wid,
        KeyEvent::new(KeyCode::Character('d'), Modifiers::default(), false),
    ));
    assert_ne!(f_game_a.get_frame_hash(), f_game_b.get_frame_hash());
    app.queue_window_command(WindowCommand::SetSize(wid, 390, 844));
    app.dispatch_window_event(WindowEvent::Resized(wid, 390, 844, 3.0));
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        wid,
        SafeAreaInsets::new(59.0, 0.0, 34.0, 0.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let _: FrameStats = click_label(&mut app, "Menu");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: FrameStats = click_label(&mut app, "Forms");
    assert_eq!(state.get_route().get_untracked(), ROUTE_FORMS);
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, 195.0, 320.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(1, 195.0, 120.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(1, 195.0, 120.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let mobile_scrolled: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive() && n.get_layout_box().get_scroll_offset().get_y() > 50.0
            })
        },
        false,
    );
    assert!(
        mobile_scrolled,
        "touch pan drag failed to scroll mobile viewport"
    );
    app.queue_window_command(WindowCommand::SetSize(wid, 1280, 800));
    app.dispatch_window_event(WindowEvent::Resized(wid, 1280, 800, 1.0));
    let _: FrameStats = app.step_frame().unwrap();
    let _: FrameStats = click_label(&mut app, "3D Engine (RT)");
    assert_eq!(state.get_route().get_untracked(), ROUTE_GAME_3D);
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let f3d_before: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    let _: FrameStats = click_label(&mut app, "Switch Camera");
    let f3d_hood: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(state.get_engine3d_camera_mode().get_untracked(), 1);
    assert_ne!(f3d_before.get_frame_hash(), f3d_hood.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Switch Camera");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(state.get_engine3d_camera_mode().get_untracked(), 2);
    let _: FrameStats = click_label(&mut app, "Switch Camera");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(state.get_engine3d_camera_mode().get_untracked(), 3);
    let _: FrameStats = click_label(&mut app, "Switch Camera");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(state.get_engine3d_camera_mode().get_untracked(), 0);
    let _: FrameStats = click_label(&mut app, "Import .OBJ Monument");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert!(
        state
            .get_engine3d_import_status()
            .get_untracked()
            .contains("Imported")
    );
    let _: FrameStats = click_label(&mut app, "Import .glTF Tower");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    let _: FrameStats = scroll_and_click_label(&mut app, "RT Full GI");
    let _: FrameStats = scroll_and_click_label(&mut app, "Ultra");
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Character('w'), Modifiers::default(), false),
    ));
    let f3d_after: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    app.dispatch_window_event(WindowEvent::KeyUp(
        wid,
        KeyEvent::new(KeyCode::Character('w'), Modifiers::default(), false),
    ));
    assert_eq!(
        state
            .get_engine3d_quality()
            .get_untracked()
            .get_ray_tracing(),
        RayTracingMode::FullPathTracedGi
    );
    assert_ne!(f3d_before.get_frame_hash(), f3d_after.get_frame_hash());
    let _: FrameStats = click_label(&mut app, "Media Studio");
    assert_eq!(state.get_route().get_untracked(), ROUTE_MEDIA);
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: FrameStats = scroll_and_click_label(&mut app, "Next Frame");
    assert_eq!(state.get_media_gif().get_untracked().get_current_frame(), 1);
    let _: FrameStats = scroll_and_click_label(&mut app, "Trigger LIVE");
    assert!(state.get_media_live().get_untracked().get_active_motion());
    let _: FrameStats = scroll_and_click_label(&mut app, "Play Video");
    assert_eq!(
        state.get_media_video().get_untracked().get_state(),
        PlaybackState::Playing
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Play Audio");
    assert_eq!(
        state.get_media_audio().get_untracked().get_state(),
        PlaybackState::Playing
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Next Page");
    assert_eq!(state.get_media_pdf().get_untracked().get_current_page(), 1);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(10, 500.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(11, 560.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(11, 660.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(10, 500.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(11, 660.0, 400.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(state.get_media_pdf().get_untracked().get_zoom() > 1.0);
    assert_eq!(
        app.get_context()
            .get_gesture()
            .get_untracked()
            .map(|g: GestureEvent| g.get_kind()),
        Some(GestureEventKind::Pinch)
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Open euv Media Engine Spec");
    assert!(
        state
            .get_media_html_link_log()
            .get_untracked()
            .contains("euv://docs/media-engine")
    );
    let _: FrameStats = click_label(&mut app, "UI & Edge Lab");
    assert_eq!(state.get_route().get_untracked(), ROUTE_LAB);
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: FrameStats = click_label(&mut app, "Window & OS");
    assert_eq!(state.get_route().get_untracked(), ROUTE_WINDOW);
    let stuck_hover_count: usize = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes()
                .iter()
                .filter(|n: &&SceneNode| {
                    n.get_alive()
                        && n.get_interaction().get_hovered()
                        && n.get_style().get_paint().get_cursor() == CursorIcon::Pointer
                })
                .count()
        },
        0,
    );
    assert!(
        stuck_hover_count <= 1,
        "clicked sidebar items must never retain stuck hover states (found {stuck_hover_count})"
    );
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(state.get_route().get_untracked(), ROUTE_LAB);
    let _: FrameStats = click_label(&mut app, "Overlays");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: FrameStats = click_label(&mut app, "Open Dialog Modal");
    for _ in 0..18 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert!(state.get_modal_open().get_untracked());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(
        !state.get_modal_open().get_untracked(),
        "System Back must dismiss top EuvOverlayStack modal first"
    );
    assert_eq!(
        state.get_route().get_untracked(),
        ROUTE_OVERLAYS,
        "System Back consumed by modal must not pop the active route"
    );
    let _: FrameStats = click_label(&mut app, "UI & Edge Lab");
    let _: FrameStats = scroll_and_click_label(&mut app, "Increment Preserved Counter");
    let preserved_before: u32 = state.get_lab_resize_preserve_count().get_untracked();
    app.queue_window_command(WindowCommand::SetSize(wid, 390, 844));
    app.dispatch_window_event(WindowEvent::Resized(wid, 390, 844, 3.0));
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(
        state.get_lab_resize_preserve_count().get_untracked(),
        preserved_before,
        "Live viewport resize must update layout only without clearing data"
    );
}

#[test]
fn global_wasm_takeover_responds_to_clicks_and_restores_native_shell() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let _: FrameStats = app.step_frame().unwrap();
    let _: FrameStats = click_label(&mut app, "Dynamic WASM");
    assert_eq!(state.get_route().get_untracked(), ROUTE_DYNAMIC);
    let _: FrameStats = scroll_and_click_label(&mut app, "Download Global WASM v1");
    assert_eq!(
        state
            .get_wasm_manager()
            .global_version_signal()
            .get_untracked(),
        1
    );
    let _: FrameStats = click_label(&mut app, "Invoke Global WASM Action (+1)");
    assert!(
        state
            .get_wasm_fault_status()
            .get_untracked()
            .contains("state=1"),
        "expected Global WASM click to update state=1, got: {}",
        state.get_wasm_fault_status().get_untracked()
    );
    let has_state1: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive() && matches!(n.get_kind(), NodeKind::Text(t) if t.contains("STATE #1"))
            })
        },
        false,
    );
    assert!(
        has_state1,
        "expected Global WASM UI to re-render live STATE #1 after click"
    );
    let _: FrameStats = click_label(&mut app, "Return to Native Shell");
    assert_eq!(
        state
            .get_wasm_manager()
            .global_version_signal()
            .get_untracked(),
        0
    );
    let alive_texts: Vec<String> = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes()
                .iter()
                .filter(|n: &&SceneNode| n.get_alive())
                .filter_map(|n: &SceneNode| match n.get_kind() {
                    NodeKind::Text(t) => Some(t.clone()),
                    _ => None,
                })
                .collect()
        },
        Vec::new(),
    );
    let has_brand: bool = alive_texts.iter().any(|t: &String| t == SHOWCASE_BRAND);
    assert!(
        has_brand,
        "expected native EuvAppShell to be restored; alive_texts={alive_texts:?}"
    );
}

#[test]
fn responsive_five_widths_virtual_list_alignment_and_ai_schema_hooks_pass_zero_overflow() {
    let viewports: [(u32, u32); 5] = [
        (360, 800),
        (390, 844),
        (768, 1024),
        (1280, 800),
        (1440, 900),
    ];
    let routes: [&str; 15] = [
        ROUTE_OVERVIEW,
        ROUTE_MARKDOWN,
        ROUTE_LAB,
        ROUTE_REACTIVE,
        ROUTE_RENDER,
        ROUTE_WINDOW,
        ROUTE_FORMS,
        ROUTE_THEMES,
        ROUTE_DATA,
        ROUTE_GAME,
        ROUTE_GAME_3D,
        ROUTE_MEDIA,
        ROUTE_OVERLAYS,
        ROUTE_DYNAMIC,
        ROUTE_STRESS,
    ];
    for (vw, vh) in viewports.into_iter() {
        reset_reactive_runtime();
        let (mut app, state): (App, ShowcaseState) =
            create_showcase_app(true, RenderMode::Software).unwrap();
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::Resized(wid, vw, vh, 2.0));
        let _: FrameStats = app.step_frame().unwrap();
        let hooks: AiAppHooks = state.build_ai_hooks();
        for route in routes.into_iter() {
            let nav: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
                &mut app,
                AiSchemaRequest::ExecuteAction {
                    action: AiAction::Navigate {
                        route: route.to_string(),
                    },
                },
                Some(&hooks),
            );
            assert!(nav.get_ok());
            for _ in 0..8 {
                if !app.has_active_animations() {
                    break;
                }
                let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
            }
            let snap: AiRuntimeSnapshot =
                AiSchemaProtocol::snapshot_with_hooks(&app, true, Some(&hooks));
            assert_eq!(snap.get_window().active_route_value(), Some(route));
            assert!(snap.get_reactive().get_live_signals() > 0);
            assert!(!snap.get_accessibility_tree().is_empty());
            assert!(snap.game_stats_value().is_some());
            let overflows: Vec<String> = snap
                .get_anomalies()
                .iter()
                .filter(|a: &&AiLayoutAnomaly| a.get_kind() == AiAnomalyKind::HorizontalOverflow)
                .map(|a: &AiLayoutAnomaly| a.get_message().clone())
                .collect();
            assert!(
                overflows.is_empty(),
                "Horizontal overflow at {vw}x{vh} route {route}: {overflows:?}"
            );
        }
        if vw == 1440 {
            let _: bool = state.get_route().set_if_changed(ROUTE_DATA.to_string());
            let _: FrameStats = app.step_frame().unwrap();
            let row20_y: f32 = find_text_center(&app, "Virtual Row #20").get_y();
            let row17_y: f32 = find_text_center(&app, "Virtual Row #17").get_y();
            assert!(
                row20_y > row17_y && row20_y > 0.0,
                "Expected Virtual Row #20 aligned inside viewport below clipped overscan"
            );
            let theme_resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
                &mut app,
                AiSchemaRequest::ExecuteAction {
                    action: AiAction::SetTheme {
                        scheme: Some("Linear".to_string()),
                        mode: Some("Dark".to_string()),
                    },
                },
                Some(&hooks),
            );
            assert!(theme_resp.get_ok());
            assert_eq!(
                state.get_theme().get_scheme().get_untracked(),
                DesignScheme::Linear
            );
            assert_eq!(
                state.get_theme().get_mode().get_untracked(),
                ThemeMode::Dark
            );
            let game_resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
                &mut app,
                AiSchemaRequest::ExecuteAction {
                    action: AiAction::GameControl {
                        command: "camera_mode".to_string(),
                        payload: AiSchemaProtocol::parse_json_value("2"),
                    },
                },
                Some(&hooks),
            );
            assert!(game_resp.get_ok());
            assert_eq!(state.get_engine3d_camera_mode().get_untracked(), 2);
        }
    }
}

#[test]
fn multitouch_lifecycle_stale_cleanup_and_smooth_scroll_inertia_never_freeze() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    let _: bool = state.get_route().set_if_changed(ROUTE_LAB.to_string());
    let _: FrameStats = app.step_frame().unwrap();
    let touches_sig: Signal<Vec<(u64, Point)>> = app.get_context().get_active_touches();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, 500.0, 360.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(2, 580.0, 360.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(3, 640.0, 390.0),
    ));
    assert_eq!(touches_sig.get_untracked().len(), 3);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(2, 610.0, 370.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(2, 610.0, 370.0),
    ));
    assert_eq!(touches_sig.get_untracked().len(), 2);
    app.reconcile_active_touch_ids(&[1]);
    assert_eq!(touches_sig.get_untracked().len(), 1);
    assert_eq!(touches_sig.get_untracked()[0].0, 1);
    app.dispatch_window_event(WindowEvent::PointerCancel(wid, 0));
    assert!(touches_sig.get_untracked().is_empty());
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(10, 520.0, 360.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(11, 600.0, 360.0),
    ));
    app.dispatch_window_event(WindowEvent::Focused(wid, false));
    assert!(touches_sig.get_untracked().is_empty());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(520.0, 360.0),
    ));
    assert_eq!(touches_sig.get_untracked().len(), 1);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::new(
            0,
            PointerDevice::Mouse,
            Point::new(530.0, 360.0),
            PointerButton::Primary,
            0.0,
            Modifiers::default(),
        ),
    ));
    assert!(touches_sig.get_untracked().is_empty());
    app.dispatch_window_event(WindowEvent::Resized(wid, 390, 844, 2.0));
    let _: bool = state.get_route().set_if_changed(ROUTE_FORMS.to_string());
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, 195.0, 500.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(2, 240.0, 500.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(2, 240.0, 500.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(1, 195.0, 340.0),
    ));
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(1, 195.0, 340.0),
    ));
    let scroll_at_release: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes()
                .iter()
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .fold(0.0_f32, f32::max)
        },
        0.0,
    );
    assert!(
        scroll_at_release > 80.0,
        "expected multi-touch handoff drag to scroll without freeze, got {scroll_at_release}"
    );
    for _ in 0..12 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let scroll_after_fling: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes()
                .iter()
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .fold(0.0_f32, f32::max)
        },
        0.0,
    );
    assert!(
        scroll_after_fling > scroll_at_release,
        "expected smooth inertial fling glide after lift: release={scroll_at_release} after={scroll_after_fling}"
    );
    assert!(touches_sig.get_untracked().is_empty());
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(3, 195.0, 500.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(3, 195.0, 440.0),
    ));
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    for _ in 0..8 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(3, 195.0, 440.0),
    ));
    assert!(
        !app.has_active_animations(),
        "holding finger stationary before lift should cancel accidental fling"
    );
}

#[test]
fn interactive_stress_lab_buttons_execute_live_benchmarks_and_update_telemetry() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let _: FrameStats = app.step_frame().unwrap();
    let _: FrameStats = click_label(&mut app, "Stress Lab");
    assert_eq!(state.get_route().get_untracked(), ROUTE_STRESS);
    assert_eq!(state.get_stress_results().get_untracked().len(), 10);
    let _: FrameStats = scroll_and_click_label(&mut app, "Run 100k List");
    let _: FrameStats = scroll_and_click_label(&mut app, "Run 2k Layout");
    let _: FrameStats = scroll_and_click_label(&mut app, "Run Drawer Anim");
    let _: FrameStats = scroll_and_click_label(&mut app, "Run 240Hz Touch");
    let results: Vec<StressCaseResult> = state.get_stress_results().get_untracked();
    let ran_count: usize = results
        .iter()
        .filter(|r: &&StressCaseResult| r.get_runs() > 0 && r.get_iterations() > 0)
        .count();
    assert_eq!(ran_count, 4);
    assert!(
        state
            .get_stress_summary()
            .get_untracked()
            .contains("tap_drift=0.00px")
    );
    let _: FrameStats = scroll_and_click_label(&mut app, "Reset Telemetry");
    let reset_rows: Vec<StressCaseResult> = state.get_stress_results().get_untracked();
    assert!(
        reset_rows
            .iter()
            .all(|r: &StressCaseResult| r.get_runs() == 0)
    );
}

#[test]
fn showcase_freeze_lifecycle_markdown_studio_3d_max_defaults_and_double_back_exit_work() {
    reset_reactive_runtime();
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    let q3d: RenderQualityConfig = state.get_engine3d_quality().get_untracked();
    assert_eq!(q3d.get_preset(), QualityPreset::Cinematic);
    assert_eq!(q3d.get_ray_tracing(), RayTracingMode::FullPathTracedGi);
    assert!((q3d.get_resolution_scale() - 1.0).abs() < 0.01);
    assert!(state.get_game3d_host().get_paused().get());
    assert!(state.get_game_host().get_paused().get());
    let _: FrameStats = click_label(&mut app, "Markdown & Code");
    assert_eq!(state.get_route().get_untracked(), ROUTE_MARKDOWN);
    assert!(state.get_lifecycle_freezes().get_untracked() >= 1);
    assert!(state.get_router().get_frozen_count().get_untracked() >= 1);
    let _: FrameStats = click_label(&mut app, "Switch Eva Light / Dark Bold");
    assert_eq!(
        state.get_theme().get_mode().get_untracked(),
        ThemeMode::Dark
    );
    let _: FrameStats = click_label(&mut app, "Emit Custom Event");
    let banner: ShowcaseGlobalBanner = GlobalState::get::<ShowcaseGlobalBanner>().unwrap();
    assert_eq!(banner.get_pulses(), 1);
    assert_eq!(banner.get_last_source(), ROUTE_MARKDOWN);
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(state.get_route().get_untracked(), ROUTE_OVERVIEW);
    assert!(app.get_running());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert!(app.get_running());
    assert!(!app.get_context().get_exit_requested().get_untracked());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    assert!(app.get_context().get_exit_requested().get_untracked());
    assert!(!app.get_running());
}
