use super::*;

fn mount_orders_app(button_test_id: &str, button_works: bool, duplicate_button: bool) -> App {
    reset_reactive_runtime();
    let page_title: Signal<String> = Signal::new("Home".to_string());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let btn_id_str: String = button_test_id.to_string();
    let title_click: Signal<String> = page_title;
    let title_text: Signal<String> = page_title;
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .padding(Edges::all(16.0))
                .gap(10.0)
                .background(COLOR_WHITE),
        );
        let nav_btn: NodeId = ctx.view(
            Style::default()
                .row()
                .padding(Edges::symmetric(8.0, 14.0))
                .background(Color::hex(0x18181B))
                .cursor(CursorIcon::Pointer),
        );
        let nav_lbl: NodeId = ctx.text(
            "Open Orders".to_string(),
            Style::default().font_size(14.0).color(COLOR_WHITE),
        );
        ctx.append(nav_btn, nav_lbl);
        let mut nav_a11y: A11yProps = A11yProps::new(A11yRole::Button, "Orders Nav".to_string());
        nav_a11y.set_hint(format!("test_id:{btn_id_str}"));
        ctx.set_a11y(nav_btn, nav_a11y);
        if button_works {
            ctx.on_click(nav_btn, move |_: &UiEvent| {
                let _: bool = title_click.set_if_changed("Orders".to_string());
            });
        }
        let heading: NodeId = ctx.text(
            "Home".to_string(),
            Style::default().font_size(16.0).color(Color::hex(0x18181B)),
        );
        let mut head_a11y: A11yProps =
            A11yProps::new(A11yRole::Heading, "Page Heading".to_string());
        head_a11y.set_hint("test_id:orders.heading".to_string());
        ctx.set_a11y(heading, head_a11y);
        let scene_h: SceneHandle = ctx.get_scene().clone();
        let (scope, _): (Scope, Effect) =
            Scope::run(|_: Scope| scene_h.bind_text(heading, move || title_text.get()));
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(heading, scope);
            },
            (),
        );
        ctx.append(root, nav_btn);
        if duplicate_button {
            let dup_btn: NodeId = ctx.view(
                Style::default()
                    .row()
                    .padding(Edges::symmetric(8.0, 14.0))
                    .background(Color::hex(0x27272A))
                    .cursor(CursorIcon::Pointer),
            );
            let dup_lbl: NodeId = ctx.text(
                "Open Orders Copy".to_string(),
                Style::default().font_size(14.0).color(COLOR_WHITE),
            );
            ctx.append(dup_btn, dup_lbl);
            let mut dup_a11y: A11yProps =
                A11yProps::new(A11yRole::Button, "Orders Nav Dup".to_string());
            dup_a11y.set_hint(format!("test_id:{btn_id_str}"));
            ctx.set_a11y(dup_btn, dup_a11y);
            ctx.append(root, dup_btn);
        }
        ctx.append(root, heading);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    app
}

fn default_target() -> UiE2eTargetV1 {
    UiE2eTargetV1::new(
        "showcase".to_string(),
        "headless".to_string(),
        640,
        480,
        1.0,
    )
}

#[test]
fn e2e_recorder_merges_scroll_drag_redacts_secrets_and_ignores_idle_pauses() {
    let mut rec: UiE2eRecorder = UiE2eRecorder::new("checkout-flow", 1, default_target());
    rec.record_click(UiE2eLocator::TestId("login.email".to_string()));
    rec.record_idle_pause(4_500);
    rec.record_type_text(
        UiE2eLocator::TestId("login.email".to_string()),
        "alice@example.com",
        false,
    );
    rec.record_type_text(
        UiE2eLocator::TestId("login.password".to_string()),
        "TopSecretPass!123",
        true,
    );
    rec.record_scroll(UiE2eLocator::TestId("orders.list".to_string()), 0.0, 40.0);
    rec.record_scroll(UiE2eLocator::TestId("orders.list".to_string()), 0.0, 60.0);
    rec.record_drag_move(UiE2eLocator::TestId("slider.thumb".to_string()), 10.0, 0.0);
    rec.record_drag_move(UiE2eLocator::TestId("slider.thumb".to_string()), 25.0, 0.0);
    rec.record_wait_visible(UiE2eLocator::TestId("orders.heading".to_string()), 5_000);
    rec.record_assert_text_equals(UiE2eLocator::TestId("orders.heading".to_string()), "Orders");
    assert_eq!(rec.get_redacted_count(), 1);
    let flow: UiE2eFlowV1 = rec.export_flow();
    assert_eq!(flow.get_schema(), UI_E2E_FLOW_SCHEMA_V1);
    assert_eq!(flow.get_steps().len(), 7);
    let json_text: String = rec.export_json().unwrap();
    assert!(!json_text.contains("TopSecretPass!123"));
    assert!(json_text.contains("E2E_SECRET_LOGIN_PASSWORD"));
    let parsed: UiE2eFlowV1 = UiE2eFlowV1::from_json(&json_text).unwrap();
    assert_eq!(parsed, flow);
    match parsed.get_steps()[3].get_operation() {
        UiE2eStepOp::Do {
            action: UiE2eActionKind::Scroll,
            dy,
            ..
        } => assert_eq!(*dy, 100.0),
        other => panic!("expected merged scroll step, got {other:?}"),
    }
    match parsed.get_steps()[4].get_operation() {
        UiE2eStepOp::Do {
            action: UiE2eActionKind::Drag,
            dx,
            ..
        } => assert_eq!(*dx, 35.0),
        other => panic!("expected merged drag step, got {other:?}"),
    }
}

#[test]
fn e2e_strict_unique_locator_rejects_ambiguous_matches_without_clicking_first() {
    let mut app: App = mount_orders_app("nav.orders", true, true);
    let flow: UiE2eFlowV1 = UiE2eFlowV1::new(
        "ambiguous-flow",
        1,
        default_target(),
        vec![
            UiE2eStepV1::do_click(
                "open_orders",
                UiE2eLocator::TestId("nav.orders".to_string()),
            ),
            UiE2eStepV1::assert_text_equals(
                "heading",
                UiE2eLocator::TestId("orders.heading".to_string()),
                "Orders",
            ),
        ],
    );
    let replayer: UiE2eReplayer = UiE2eReplayer::for_build("build-dup");
    let res: UiE2eResultV1 = replayer.replay_once(&mut app, &flow, "run-ambig");
    assert_eq!(res.get_verdict(), UiE2eVerdict::Failed);
    assert_eq!(res.get_steps()[0].get_status(), UiE2eStepStatus::Failed);
    assert_eq!(
        res.get_steps()[0].code_value(),
        Some(E2E_ERR_AMBIGUOUS_LOCATOR)
    );
    assert_eq!(res.get_steps()[1].get_status(), UiE2eStepStatus::NotRun);
}

#[test]
fn e2e_fixture_a_renamed_locator_migration_proposal_preserves_original_assertion() {
    let flow: UiE2eFlowV1 = UiE2eFlowV1::new(
        "login-to-orders",
        3,
        default_target(),
        vec![
            UiE2eStepV1::do_launch("launch"),
            UiE2eStepV1::do_click("orders", UiE2eLocator::TestId("nav.orders".to_string())),
            UiE2eStepV1::assert_text_equals(
                "title",
                UiE2eLocator::TestId("orders.heading".to_string()),
                "Orders",
            ),
        ],
    );
    let mut app_new: App = mount_orders_app("nav.orders_v2", true, false);
    let replayer: UiE2eReplayer = UiE2eReplayer::for_build("build-v2");
    let mut failed_res: UiE2eResultV1 = replayer.replay_once(&mut app_new, &flow, "orders-052");
    assert_eq!(failed_res.get_schema(), UI_E2E_RESULT_SCHEMA_V1);
    assert_eq!(failed_res.get_verdict(), UiE2eVerdict::Failed);
    assert_eq!(
        failed_res.get_steps()[0].get_status(),
        UiE2eStepStatus::Passed
    );
    assert_eq!(
        failed_res.get_steps()[1].get_status(),
        UiE2eStepStatus::Failed
    );
    assert_eq!(
        failed_res.get_steps()[1].code_value(),
        Some(E2E_ERR_LOCATOR_NOT_FOUND)
    );
    assert!(failed_res.get_steps()[1].frame_id_value().is_some());
    assert_eq!(
        failed_res.get_steps()[2].get_status(),
        UiE2eStepStatus::NotRun
    );
    let mut clean_candidate_app: App = mount_orders_app("nav.orders_v2", true, false);
    let (proposal, candidate_flow): (UiE2eMigrationProposal, UiE2eFlowV1) =
        UiE2eMigrationProposal::propose_and_verify(
            &flow,
            "build-v1",
            "build-v2",
            vec![UiE2eLocatorChange::new(
                "orders".to_string(),
                UiE2eLocator::TestId("nav.orders".to_string()),
                UiE2eLocator::TestId("nav.orders_v2".to_string()),
                "Button renamed from nav.orders to nav.orders_v2".to_string(),
            )],
            &mut clean_candidate_app,
            "orders-053",
        )
        .unwrap();
    assert_eq!(proposal.get_schema(), UI_E2E_MIGRATION_SCHEMA_V1);
    assert!(!proposal.get_assertions_changed());
    assert_eq!(proposal.get_status(), UiE2eMigrationStatus::Proposed);
    assert_eq!(
        proposal.candidate_rerun_verdict_value(),
        Some(UiE2eVerdict::Passed)
    );
    assert!(UiE2eMigrationProposal::validate_preserves_assertions(&flow, &candidate_flow).is_ok());
    failed_res.attach_migration(proposal).unwrap();
    assert_eq!(failed_res.get_verdict(), UiE2eVerdict::Failed);
    assert!(failed_res.migration_proposal().is_some());
    let mut bad_app: App = mount_orders_app("nav.orders_v2", true, false);
    let mut_err: String = UiE2eMigrationProposal::propose_and_verify(
        &flow,
        "build-v1",
        "build-v2",
        vec![UiE2eLocatorChange::new(
            "title".to_string(),
            UiE2eLocator::TestId("orders.heading".to_string()),
            UiE2eLocator::TestId("orders.other".to_string()),
            "Attempted assertion edit".to_string(),
        )],
        &mut bad_app,
        "orders-bad",
    )
    .unwrap_err();
    assert!(mut_err.contains(E2E_ERR_ASSERTION_MUTATION_FORBIDDEN));
}

#[test]
fn e2e_fixture_b_broken_button_failure_marks_trailing_steps_not_run() {
    let mut app: App = mount_orders_app("nav.orders", false, false);
    let flow: UiE2eFlowV1 = UiE2eFlowV1::new(
        "orders-broken-btn",
        1,
        default_target(),
        vec![
            UiE2eStepV1::do_click("orders", UiE2eLocator::TestId("nav.orders".to_string())),
            UiE2eStepV1::assert_text_equals(
                "title",
                UiE2eLocator::TestId("orders.heading".to_string()),
                "Orders",
            ),
            UiE2eStepV1::assert_visible(
                "trailing_check",
                UiE2eLocator::TestId("nav.orders".to_string()),
            ),
        ],
    );
    let replayer: UiE2eReplayer = UiE2eReplayer::for_build("build-broken");
    let res: UiE2eResultV1 = replayer.replay_once(&mut app, &flow, "run-broken");
    assert_eq!(res.get_verdict(), UiE2eVerdict::Failed);
    assert_eq!(res.get_steps()[0].get_status(), UiE2eStepStatus::Passed);
    assert_eq!(res.get_steps()[1].get_status(), UiE2eStepStatus::Failed);
    assert_eq!(
        res.get_steps()[1].code_value(),
        Some(E2E_ERR_ASSERTION_FAILED)
    );
    assert_eq!(res.get_steps()[2].get_status(), UiE2eStepStatus::NotRun);
}

#[test]
fn e2e_fixture_c_approved_retired_flow_classification() {
    let mut app: App = mount_orders_app("nav.orders", true, false);
    let retired_flow: UiE2eFlowV1 = UiE2eFlowV1::new(
        "legacy-promo",
        2,
        default_target(),
        vec![UiE2eStepV1::do_click(
            "promo",
            UiE2eLocator::TestId("promo.removed".to_string()),
        )],
    )
    .with_retirement(UiE2eRetirementRecord::new(
        "product-lead".to_string(),
        "v0.45.0".to_string(),
        "Legacy promo drawer retired".to_string(),
        Some("modern-offers".to_string()),
    ));
    let replayer: UiE2eReplayer = UiE2eReplayer::for_build("v0.45.0");
    let retired_res: UiE2eResultV1 = replayer.replay_once(&mut app, &retired_flow, "run-retired");
    assert_eq!(retired_res.get_verdict(), UiE2eVerdict::Retired);
    assert_eq!(
        retired_res.get_steps()[0].get_status(),
        UiE2eStepStatus::NotRun
    );
    let unapproved_flow: UiE2eFlowV1 = UiE2eFlowV1::new(
        "unapproved-retire",
        2,
        default_target(),
        vec![UiE2eStepV1::do_click(
            "promo",
            UiE2eLocator::TestId("promo.removed".to_string()),
        )],
    )
    .with_retirement(UiE2eRetirementRecord::new(
        String::new(),
        "v0.45.0".to_string(),
        "Missing approval".to_string(),
        None,
    ));
    let unapproved_res: UiE2eResultV1 =
        replayer.replay_once(&mut app, &unapproved_flow, "run-unapproved");
    assert_eq!(unapproved_res.get_verdict(), UiE2eVerdict::Failed);
    assert_eq!(
        unapproved_res.get_steps()[0].code_value(),
        Some(E2E_ERR_LOCATOR_NOT_FOUND)
    );
    let blocked_replayer: UiE2eReplayer =
        UiE2eReplayer::for_build("v0.45.0").with_blocked_reason("fixture server offline");
    let blocked_res: UiE2eResultV1 =
        blocked_replayer.replay_once(&mut app, &unapproved_flow, "run-blocked");
    assert_eq!(blocked_res.get_verdict(), UiE2eVerdict::Blocked);
    let unsupported_replayer: UiE2eReplayer = UiE2eReplayer::for_build("v0.45.0")
        .with_supported_adapters(vec!["native-window".to_string()]);
    let unsupported_res: UiE2eResultV1 =
        unsupported_replayer.replay_once(&mut app, &unapproved_flow, "run-unsupported");
    assert_eq!(unsupported_res.get_verdict(), UiE2eVerdict::Unsupported);
}

#[test]
fn e2e_fixture_d_first_attempt_timeout_and_clean_retry_classified_flaky() {
    let flow: UiE2eFlowV1 = UiE2eFlowV1::new(
        "async-orders-load",
        1,
        default_target(),
        vec![
            UiE2eStepV1::do_click("orders", UiE2eLocator::TestId("nav.orders".to_string())),
            UiE2eStepV1::wait_text_equals(
                "wait_orders",
                UiE2eLocator::TestId("orders.heading".to_string()),
                "Orders",
                2_000,
            ),
            UiE2eStepV1::assert_text_equals(
                "verify_orders",
                UiE2eLocator::TestId("orders.heading".to_string()),
                "Orders",
            ),
        ],
    );
    let replayer: UiE2eReplayer = UiE2eReplayer::for_build("v0.45.0")
        .with_max_retries(2)
        .with_max_wait_frames(3);
    let result: UiE2eResultV1 =
        replayer.replay_with_clean_retries(&flow, "run-flaky-1", |attempt: u32| {
            mount_orders_app("nav.orders", attempt >= 2, false)
        });
    assert_eq!(result.get_verdict(), UiE2eVerdict::Flaky);
    assert_eq!(result.get_attempts().len(), 2);
    assert_eq!(result.get_attempts()[0].get_attempt(), 1);
    assert_eq!(result.get_attempts()[0].get_verdict(), UiE2eVerdict::Failed);
    assert_eq!(
        result.get_attempts()[0].first_failure_code_value(),
        Some(E2E_ERR_WAIT_TIMEOUT)
    );
    assert_eq!(result.get_attempts()[1].get_attempt(), 2);
    assert_eq!(result.get_attempts()[1].get_verdict(), UiE2eVerdict::Passed);
    let first_fail: &UiE2eStepResultV1 = result.first_failure_record().unwrap();
    assert_eq!(first_fail.get_id(), "wait_orders");
    assert_eq!(first_fail.code_value(), Some(E2E_ERR_WAIT_TIMEOUT));
    assert_eq!(result.get_steps()[1].get_status(), UiE2eStepStatus::Failed);
    assert_eq!(result.get_steps()[2].get_status(), UiE2eStepStatus::NotRun);
}
