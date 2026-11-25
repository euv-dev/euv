use super::*;

fn find_text_center(app: &App, label: &str) -> Point {
    app.get_scene().with_read(
        |tree: &SceneTree| {
            for node in tree.get_nodes().iter() {
                if !node.get_alive() || node.get_style().get_layout().get_display() == Display::None
                {
                    continue;
                }
                if let NodeKind::Text(text) = node.get_kind()
                    && text.as_str() == label
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
