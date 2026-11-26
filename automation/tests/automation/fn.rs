use super::*;

fn mount_test_app() -> (App, Signal<i32>, Signal<String>) {
    reset_reactive_runtime();
    let count: Signal<i32> = Signal::new(0);
    let typed: Signal<String> = Signal::new(String::new());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let c_click: Signal<i32> = count;
    let c_text: Signal<i32> = count;
    let t_in: Signal<String> = typed;
    let _: NodeId = app.mount(move || {
        let mut ctx: BuildContext = current_build_context();
        let root: NodeId = ctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(20.0))
                .gap(12.0)
                .background(COLOR_WHITE),
        );
        let btn: NodeId = ctx.view(
            Style::empty()
                .row()
                .padding(Edges::symmetric(8.0, 14.0))
                .background(Color::hex(0x18181B))
                .radius(6.0)
                .cursor(CursorIcon::Pointer),
        );
        let btn_lbl: NodeId = ctx.text(
            "Increment Counter".to_string(),
            Style::empty().font_size(14.0).color(COLOR_WHITE),
        );
        ctx.append(btn, btn_lbl);
        ctx.set_a11y(
            btn,
            A11yProps::new(A11yRole::Button, "Increment Action".to_string()),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            let _: bool = c_click.update(|v: &mut i32| *v += 1);
        });
        let status: NodeId = ctx.text(
            "Count=0".to_string(),
            Style::empty().font_size(14.0).color(Color::hex(0x18181B)),
        );
        let scene_h: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            scene_h.bind_text(status, move || format!("Count={}", c_text.get()))
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(status, scope);
            },
            (),
        );
        let inp: NodeId = ctx.input(
            InputNodeState::new(
                String::new(),
                "Enter handle".to_string(),
                0,
                false,
                false,
                false,
            ),
            Style::empty()
                .width(Length::Px(260.0))
                .height(Length::Px(36.0))
                .padding(Edges::symmetric(6.0, 10.0))
                .border(1.0, Color::hex(0xD4D4D8)),
        );
        ctx.on_change(inp, move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = t_in.set_if_changed(val.clone());
            }
        });
        ctx.append(root, btn);
        ctx.append(root, status);
        ctx.append(root, inp);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    (app, count, typed)
}

#[test]
fn ui_automator_queries_clicks_hovers_types_resizes_and_runs_scripts() {
    let (mut app, count, typed): (App, Signal<i32>, Signal<String>) = mount_test_app();
    let mut auto: UiAutomator<'_> = UiAutomator::new(&mut app);
    assert!(auto.exists(&UiSelector::text("Increment Counter")));
    assert!(auto.exists(&UiSelector::placeholder("Enter handle")));
    let _: FrameStats = auto.hover(&UiSelector::text("Increment Counter")).unwrap();
    let _: FrameStats = auto.click(&UiSelector::text("Increment Counter")).unwrap();
    assert_eq!(count.get_untracked(), 1);
    assert!(auto.exists(&UiSelector::text("Count=1")));
    let _: FrameStats = auto
        .type_text(&UiSelector::placeholder("Enter handle"), "euv-ai")
        .unwrap();
    assert_eq!(typed.get_untracked(), "euv-ai");
    assert!(auto.assert_no_horizontal_overflow().is_ok());
    assert!(auto.assert_no_text_overlap().is_ok());
    let script: UiTestScript = UiTestScript::new(
        "e2e_script".to_string(),
        vec![
            UiTestStep::Click {
                selector: UiSelector::text("Increment Counter"),
            },
            UiTestStep::AssertVisible {
                selector: UiSelector::text("Count=2"),
            },
            UiTestStep::Resize {
                width: 390,
                height: 844,
                scale: 2.0,
            },
            UiTestStep::AssertNoHorizontalOverflow,
            UiTestStep::AssertNoTextOverlap,
        ],
    );
    let report: UiTestReport = auto.run_script(&script);
    assert!(report.get_passed());
    assert_eq!(report.get_passed_steps(), 5);
    assert_eq!(report.get_failed_steps(), 0);
}

#[test]
fn ai_schema_v1_protocol_describes_snapshots_queries_hit_tests_and_executes_actions() {
    let (mut app, count, _): (App, Signal<i32>, Signal<String>) = mount_test_app();
    let desc_raw: String = AiSchemaProtocol::handle_json(&mut app, r#"{"op":"DescribeSchema"}"#);
    let desc: AiSchemaResponse = from_str(&desc_raw).unwrap();
    assert!(desc.get_ok());
    assert_eq!(desc.get_protocol_version(), AI_SCHEMA_PROTOCOL_VERSION);
    let snap: AiRuntimeSnapshot = AiSchemaProtocol::snapshot(&app, true);
    assert_eq!(snap.get_protocol_version(), AI_SCHEMA_PROTOCOL_VERSION);
    assert_eq!(snap.get_window().get_logical_width(), 640.0);
    assert!(snap.get_performance().get_live_node_count() >= 4);
    assert!(snap.root_layout_node().is_some());
    assert!(snap.get_anomalies().is_empty());
    let query_resp: AiSchemaResponse = AiSchemaProtocol::handle(
        &mut app,
        AiSchemaRequest::QueryNodes {
            selector: UiSelector::text("Increment Counter"),
        },
    );
    assert!(query_resp.get_ok());
    let matches: Vec<AiLayoutNode> = from_value(query_resp.get_data().clone()).unwrap();
    assert_eq!(matches.len(), 1);
    let center: Point = matches[0].get_screen_bounds().center();
    let hit: AiHitTestResult = AiSchemaProtocol::hit_test_at(&app, center.get_x(), center.get_y());
    assert!(hit.get_ancestor_indices().len() >= 2);
    let action_resp: AiSchemaResponse = AiSchemaProtocol::handle(
        &mut app,
        AiSchemaRequest::ExecuteAction {
            action: AiAction::Click {
                selector: UiSelector::text("Increment Counter"),
            },
        },
    );
    assert!(action_resp.get_ok());
    assert_eq!(count.get_untracked(), 1);
}

#[test]
fn dev_automation_bridge_serves_ai_schema_over_loopback_when_debug_assertions_enabled() {
    assert_eq!(DevAutomationBridge::is_enabled(), cfg!(debug_assertions));
    let (mut app, _, _): (App, Signal<i32>, Signal<String>) = mount_test_app();
    let mut bridge: DevAutomationBridge = DevAutomationBridge::start("127.0.0.1:0");
    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    {
        let addr: String = bridge.bound_address().unwrap().to_string();
        let mut client: TcpStream = TcpStream::connect(&addr).unwrap();
        client
            .write_all(b"GET /schema HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        assert_eq!(bridge.poll(&mut app), 1);
        let mut buf: String = String::new();
        let _: usize = client.read_to_string(&mut buf).unwrap();
        assert!(buf.contains(AI_SCHEMA_PROTOCOL_VERSION));
        let mut opt_client: TcpStream = TcpStream::connect(&addr).unwrap();
        opt_client
            .write_all(b"OPTIONS /ai/action HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        assert_eq!(bridge.poll(&mut app), 1);
        let mut opt_buf: String = String::new();
        let _: usize = opt_client.read_to_string(&mut opt_buf).unwrap();
        assert!(opt_buf.contains("204 No Content"));
        let action_json: &str =
            r#"{"kind":"Click","selector":{"type":"ByA11yLabel","value":"Increment Action"}}"#;
        let post_req: String = format!(
            "POST /ai/action HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            action_json.len(),
            action_json
        );
        let mut act_client: TcpStream = TcpStream::connect(&addr).unwrap();
        act_client.write_all(post_req.as_bytes()).unwrap();
        assert_eq!(bridge.poll(&mut app), 1);
        let mut act_buf: String = String::new();
        let _: usize = act_client.read_to_string(&mut act_buf).unwrap();
        assert!(act_buf.contains(r#""ok":true"#));
        let mut a11y_client: TcpStream = TcpStream::connect(&addr).unwrap();
        a11y_client
            .write_all(b"GET /ai/a11y HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        assert_eq!(bridge.poll(&mut app), 1);
        let mut a11y_buf: String = String::new();
        let _: usize = a11y_client.read_to_string(&mut a11y_buf).unwrap();
        assert!(a11y_buf.contains("Increment Action"));
    }
    #[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
    {
        assert!(bridge.bound_address().is_none());
        assert_eq!(bridge.poll(&mut app), 0);
    }
}

#[test]
fn ai_schema_hooks_gestures_clear_input_a11y_and_reactive_telemetry_work() {
    let (mut app, count, typed): (App, Signal<i32>, Signal<String>) = mount_test_app();
    let active_route: Signal<String> = Signal::new("/overview".to_string());
    let active_scheme: Signal<String> = Signal::new("Shadcn".to_string());
    let hooks: AiAppHooks = AiAppHooks::new()
        .on_route(move || active_route.get_untracked())
        .on_scheme(move || active_scheme.get_untracked())
        .on_game_stats(|| json!({"triangles": 4096, "fps_3d": 120.0}))
        .on_navigate(move |route: &str| {
            let _: bool = active_route.set(route.to_string());
            Ok(())
        })
        .on_set_theme(move |scheme: Option<&str>, _mode: Option<&str>| {
            if let Some(s) = scheme {
                let _: bool = active_scheme.set(s.to_string());
            }
            Ok(())
        })
        .on_game_control(|cmd: &str, payload: &Value| {
            Ok(json!({"executed": cmd, "payload": payload}))
        });
    let snap: AiRuntimeSnapshot = AiSchemaProtocol::snapshot_with_hooks(&app, true, Some(&hooks));
    assert_eq!(snap.get_window().active_route_value(), Some("/overview"));
    assert_eq!(snap.get_window().active_scheme_value(), Some("Shadcn"));
    assert!(snap.get_reactive().get_live_signals() >= 4);
    assert!(!snap.get_accessibility_tree().is_empty());
    assert!(snap.game_stats_value().is_some());
    let nav_resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
        &mut app,
        AiSchemaRequest::ExecuteAction {
            action: AiAction::Navigate {
                route: "/game3d".to_string(),
            },
        },
        Some(&hooks),
    );
    assert!(nav_resp.get_ok());
    assert_eq!(active_route.get_untracked(), "/game3d");
    let theme_resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
        &mut app,
        AiSchemaRequest::ExecuteAction {
            action: AiAction::SetTheme {
                scheme: Some("Cupertino".to_string()),
                mode: Some("Dark".to_string()),
            },
        },
        Some(&hooks),
    );
    assert!(theme_resp.get_ok());
    assert_eq!(active_scheme.get_untracked(), "Cupertino");
    let game_resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
        &mut app,
        AiSchemaRequest::ExecuteAction {
            action: AiAction::GameControl {
                command: "camera_mode".to_string(),
                payload: json!({"mode": "Orbit"}),
            },
        },
        Some(&hooks),
    );
    assert!(game_resp.get_ok());
    let mut auto: UiAutomator<'_> = UiAutomator::new(&mut app);
    let _: FrameStats = auto
        .click(&UiSelector::a11y_label("Increment Action"))
        .unwrap();
    assert_eq!(count.get_untracked(), 1);
    let _: FrameStats = auto
        .type_text(&UiSelector::placeholder("Enter handle"), "hello")
        .unwrap();
    assert_eq!(typed.get_untracked(), "hello");
    assert!(
        auto.assert_text_equals(&UiSelector::placeholder("Enter handle"), "hello")
            .is_ok()
    );
    let _: FrameStats = auto
        .clear_input(&UiSelector::placeholder("Enter handle"))
        .unwrap();
    assert_eq!(typed.get_untracked(), "");
    let _: FrameStats = auto
        .gesture(AiGestureKind::Swipe, 200.0, 200.0, 80.0, 0.0, 1.0)
        .unwrap();
    let _: FrameStats = auto
        .gesture(AiGestureKind::Pinch, 200.0, 200.0, 0.0, 0.0, 1.5)
        .unwrap();
    let _: FrameStats = auto.set_fps_limit(Some(60)).unwrap();
    assert_eq!(auto.app_ref().fps_limit(), Some(60));
}
