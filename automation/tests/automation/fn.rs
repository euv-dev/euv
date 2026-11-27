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
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.scroll_view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(20.0))
                .gap(12.0)
                .background(COLOR_WHITE),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .row()
                .padding(Edges::symmetric(8.0, 14.0))
                .background(Color::hex(0x18181B))
                .radius(6.0)
                .cursor(CursorIcon::Pointer),
        );
        let btn_lbl: NodeId = ctx.text(
            "Increment Counter".to_string(),
            Style::default().font_size(14.0).color(COLOR_WHITE),
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
            Style::default().font_size(14.0).color(Color::hex(0x18181B)),
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
            Style::default()
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

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
fn send_http_and_poll(
    addr: &str,
    request_bytes: Vec<u8>,
    bridge: &mut DevAutomationBridge,
    app: &mut App,
) -> String {
    let target: String = addr.to_string();
    let done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let done_bg: Arc<AtomicBool> = Arc::clone(&done);
    let client_handle: JoinHandle<String> = thread::spawn(move || {
        let mut client: TcpStream = TcpStream::connect(&target).unwrap();
        client
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        client.write_all(&request_bytes).unwrap();
        let mut buf: String = String::new();
        let _: usize = client.read_to_string(&mut buf).unwrap();
        done_bg.store(true, Ordering::Release);
        buf
    });
    let deadline: Instant = Instant::now() + Duration::from_secs(3);
    while !done.load(Ordering::Acquire) && Instant::now() < deadline {
        let _: usize = bridge.poll_budgeted(app, 2_000);
        sleep(Duration::from_micros(200));
    }
    client_handle.join().unwrap()
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
        let buf: String = send_http_and_poll(
            &addr,
            b"GET /schema HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec(),
            &mut bridge,
            &mut app,
        );
        assert!(buf.contains(AI_SCHEMA_PROTOCOL_VERSION));
        let opt_buf: String = send_http_and_poll(
            &addr,
            b"OPTIONS /ai/action HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec(),
            &mut bridge,
            &mut app,
        );
        assert!(opt_buf.contains("204 No Content"));
        let health_buf: String = send_http_and_poll(
            &addr,
            b"GET /ai/health HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec(),
            &mut bridge,
            &mut app,
        );
        assert!(health_buf.contains(AI_SCHEMA_V2_PROTOCOL_VERSION));
        let action_json: &str =
            r#"{"kind":"Click","selector":{"type":"ByA11yLabel","value":"Increment Action"}}"#;
        let post_req: String = format!(
            "POST /ai/action HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            action_json.len(),
            action_json
        );
        let act_buf: String =
            send_http_and_poll(&addr, post_req.into_bytes(), &mut bridge, &mut app);
        assert!(act_buf.contains(r#""ok":true"#));
        let a11y_buf: String = send_http_and_poll(
            &addr,
            b"GET /ai/a11y HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec(),
            &mut bridge,
            &mut app,
        );
        assert!(a11y_buf.contains("Increment Action"));
        let v2_buf: String = send_http_and_poll(
            &addr,
            b"GET /ai/snapshot/v2 HTTP/1.1\r\nHost: localhost\r\n\r\n".to_vec(),
            &mut bridge,
            &mut app,
        );
        assert!(v2_buf.contains(AI_SCHEMA_V2_PROTOCOL_VERSION));
        assert!(bridge.requests_served() >= 5);
    }
    #[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
    {
        assert!(bridge.bound_address().is_none());
        assert_eq!(bridge.poll(&mut app), 0);
    }
}

#[test]
fn dev_automation_bridge_non_blocking_latency_and_backpressure_work() {
    let (mut app, _, _): (App, Signal<i32>, Signal<String>) = mount_test_app();
    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    {
        let mut bridge: DevAutomationBridge =
            DevAutomationBridge::start_with_capacity("127.0.0.1:0", 2);
        assert_eq!(bridge.queue_capacity(), 2);
        let addr: String = bridge.bound_address().unwrap().to_string();
        let mut latencies_us: Vec<u128> = Vec::with_capacity(200);
        for _ in 0..200usize {
            let started: Instant = Instant::now();
            let served: usize = bridge.poll_budgeted(&mut app, 500);
            let elapsed: u128 = started.elapsed().as_micros();
            assert_eq!(served, 0);
            latencies_us.push(elapsed);
        }
        latencies_us.sort();
        let p99_us: u128 = latencies_us[197];
        assert!(p99_us < 500, "expected UI poll p99 < 500us, got {p99_us}us");
        let mut handles: Vec<JoinHandle<String>> = Vec::with_capacity(8);
        for _ in 0..8usize {
            let target: String = addr.clone();
            handles.push(thread::spawn(move || {
                let mut stream: TcpStream = TcpStream::connect(&target).unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                stream
                    .write_all(b"GET /schema HTTP/1.1\r\nHost: localhost\r\n\r\n")
                    .unwrap();
                let mut reply: String = String::new();
                let _: usize = stream.read_to_string(&mut reply).unwrap();
                reply
            }));
        }
        let sat_deadline: Instant = Instant::now() + Duration::from_secs(2);
        while bridge.backpressure_rejected() == 0 && Instant::now() < sat_deadline {
            sleep(Duration::from_millis(1));
        }
        assert!(bridge.backpressure_rejected() >= 1);
        let drain_deadline: Instant = Instant::now() + Duration::from_secs(3);
        while bridge.requests_served() + bridge.backpressure_rejected() < 8
            && Instant::now() < drain_deadline
        {
            let _: usize = bridge.poll_budgeted(&mut app, 2_000);
            sleep(Duration::from_micros(250));
        }
        let mut ok_count: usize = 0;
        let mut rejected_count: usize = 0;
        for handle in handles.into_iter() {
            let reply: String = handle.join().unwrap();
            if reply.contains("429 Too Many Requests")
                && reply.contains(DEV_BRIDGE_ERR_BACKPRESSURE)
            {
                rejected_count += 1;
            } else if reply.contains("200 OK") {
                ok_count += 1;
            }
        }
        assert!(rejected_count >= 1);
        assert!(ok_count >= 2);
        assert_eq!(ok_count + rejected_count, 8);
    }
    #[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
    {
        let mut bridge: DevAutomationBridge = DevAutomationBridge::start("127.0.0.1:0");
        assert_eq!(bridge.poll_budgeted(&mut app, 500), 0);
    }
}

#[test]
fn ai_schema_v2_direct_query_parity_same_frame_layout_state_and_redaction() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let status_holder: Rc<RefCell<Option<NodeId>>> = Rc::new(RefCell::new(None));
    let status_holder_mount: Rc<RefCell<Option<NodeId>>> = Rc::clone(&status_holder);
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(16.0))
                .gap(8.0),
        );
        let title: NodeId = ctx.text(
            "Account Settings".to_string(),
            Style::default().font_size(16.0),
        );
        let mut title_a11y: A11yProps =
            A11yProps::new(A11yRole::Heading, "Settings Title".to_string());
        title_a11y.set_hint("test_id:settings.heading".to_string());
        ctx.set_a11y(title, title_a11y);
        let pwd: NodeId = ctx.input(
            InputNodeState::new(
                "SuperSecret99".to_string(),
                "Password".to_string(),
                13,
                true,
                false,
                false,
            ),
            Style::default()
                .width(Length::Px(220.0))
                .height(Length::Px(32.0)),
        );
        let token_text: NodeId = ctx.text("ghp_live_token_abc".to_string(), Style::default());
        let hidden_panel: NodeId = ctx.view(Style::default().display(Display::None));
        let hidden_text: NodeId = ctx.text("Hidden Detail".to_string(), Style::default());
        ctx.append(hidden_panel, hidden_text);
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                if let Some(pnode) = tree.get_node_mut(pwd) {
                    let _: &mut SceneNode = pnode.set_key(Some("login.password".to_string()));
                }
                if let Some(tnode) = tree.get_node_mut(token_text) {
                    let _: &mut SceneNode = tnode.set_key(Some("auth.api_token".to_string()));
                }
                if let Some(hnode) = tree.get_node_mut(hidden_panel) {
                    let _: &mut SceneNode = hnode.set_key(Some("panel.hidden".to_string()));
                }
            },
            (),
        );
        *status_holder_mount.borrow_mut() = Some(title);
        ctx.append(root, title);
        ctx.append(root, pwd);
        ctx.append(root, token_text);
        ctx.append(root, hidden_panel);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let capture: AiSameFrameCapture = AiSchemaProtocol::capture_same_frame(&app, true, true, None);
    assert!(capture.get_synced());
    assert!(capture.rgba_fnv1a_value().is_some());
    assert_eq!(
        capture.get_snapshot().get_schema_version(),
        AI_SCHEMA_V2_PROTOCOL_VERSION
    );
    assert_eq!(
        capture.get_frame_id(),
        capture.get_snapshot().get_frame_id()
    );
    assert_eq!(
        capture.get_frame_id(),
        capture.get_snapshot().get_performance().get_frame_index()
    );
    assert_eq!(
        capture.get_scene_revision(),
        capture.get_snapshot().get_scene_revision()
    );
    assert!(capture.get_snapshot().get_redacted_field_count() >= 2);
    let snap_json: String = to_string(capture.get_snapshot()).unwrap();
    assert!(!snap_json.contains("SuperSecret99"));
    assert!(!snap_json.contains("ghp_live_token_abc"));
    assert!(snap_json.contains(AI_SCHEMA_REDACTED_VALUE));
    let root_node: &AiLayoutNode = capture.get_snapshot().root_layout_node().unwrap();
    let mut stack: Vec<&AiLayoutNode> = vec![root_node];
    let mut hidden_matches: Vec<&AiLayoutNode> = Vec::new();
    while let Some(cur) = stack.pop() {
        if cur.test_id_value() == Some("panel.hidden") {
            hidden_matches.push(cur);
        }
        for child in cur.get_children().iter() {
            stack.push(child);
        }
    }
    assert_eq!(hidden_matches.len(), 1);
    assert_eq!(
        hidden_matches[0].get_layout_state(),
        AiLayoutState::Unmeasured
    );
    assert_eq!(hidden_matches[0].get_screen_bounds().get_width(), 0.0);
    let selectors: Vec<UiSelector> = vec![
        UiSelector::test_id("settings.heading"),
        UiSelector::stable_key("login.password"),
        UiSelector::test_id("auth.api_token"),
        UiSelector::placeholder("Password"),
        UiSelector::role("Heading"),
        UiSelector::Interactive,
        UiSelector::kind("Text").nth(0),
    ];
    let mut auto: UiAutomator<'_> = UiAutomator::new(&mut app);
    for sel in selectors.iter() {
        let direct: Vec<AiLayoutNode> = auto.query_all(sel);
        let from_snap: Vec<AiLayoutNode> = root_node.find_matches(sel);
        assert_eq!(direct, from_snap);
    }
    let pwd_matches: Vec<AiLayoutNode> = auto.query_all(&UiSelector::test_id("login.password"));
    assert_eq!(pwd_matches.len(), 1);
    assert!(pwd_matches[0].get_redacted());
    assert_eq!(pwd_matches[0].input_value(), Some(AI_SCHEMA_REDACTED_VALUE));
    let _: FrameStats = auto
        .clear_input(&UiSelector::test_id("login.password"))
        .unwrap();
    let cleared: Vec<AiLayoutNode> = auto.query_all(&UiSelector::test_id("login.password"));
    assert_eq!(cleared[0].input_value(), Some(""));
    let title_id: NodeId = status_holder.borrow().unwrap();
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.set_text(title_id, "Mutated Unrendered".to_string());
        },
        (),
    );
    let unsynced: AiSameFrameCapture =
        AiSchemaProtocol::capture_same_frame(&app, false, true, None);
    assert!(!unsynced.get_synced());
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
