use super::*;

fn find_label_center(app: &App, label: &str) -> Point {
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
                    let b: Rect = node.get_layout_box().get_bounds();
                    if b.get_width() > 0.0 && b.get_height() > 0.0 {
                        return Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        );
                    }
                }
            }
            Point::new(-1.0, -1.0)
        },
        Point::new(-1.0, -1.0),
    )
}

fn click_text(app: &mut App, label: &str) -> FrameStats {
    let pt: Point = find_label_center(app, label);
    assert!(
        pt.get_x() >= 0.0 && pt.get_y() >= 0.0,
        "expected visible dynamic label `{label}`"
    );
    let wid: WindowId = app.get_window_state().get_id();
    let ev: PointerEvent = PointerEvent::mouse(pt.get_x(), pt.get_y());
    app.dispatch_window_event(WindowEvent::PointerMove(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
    app.step_frame().unwrap()
}

fn make_headless_app() -> App {
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 400).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    App::headless(cfg).unwrap()
}

#[test]
fn component_level_dynamic_update_is_surgical_preserves_siblings_and_rolls_back() {
    reset_reactive_runtime();
    let mut app: App = make_headless_app();
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let sibling_state: Signal<i32> = Signal::new(42);
    let mounts_a: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mounts_b: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mounts_a_fb: Rc<Cell<u32>> = mounts_a.clone();
    let mounts_b_fb: Rc<Cell<u32>> = mounts_b.clone();
    let captured_actions: Rc<RefCell<Vec<DynamicActionEvent>>> = Rc::new(RefCell::new(Vec::new()));
    let captured_c: Rc<RefCell<Vec<DynamicActionEvent>>> = captured_actions.clone();
    hub.on_action(move |ev: &DynamicActionEvent| {
        if let Ok(mut list) = captured_c.try_borrow_mut() {
            list.push(ev.clone());
        }
    });
    let _: NodeId = app.mount(move || {
        let mut ctx: BuildContext = current_build_context();
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(12.0))
                .gap(8.0),
        );
        let slot_a: NodeId = DynamicComponentSlot::new(
            "slot_a".to_string(),
            Rc::new(move || {
                mounts_a_fb.set(mounts_a_fb.get() + 1);
                let mut c: BuildContext = current_build_context();
                c.text("Fallback A".to_string(), Style::empty().font_size(14.0))
            }),
        )
        .mount(&mut ctx);
        let slot_b: NodeId = DynamicComponentSlot::new(
            "slot_b".to_string(),
            Rc::new(move || {
                mounts_b_fb.set(mounts_b_fb.get() + 1);
                let mut c: BuildContext = current_build_context();
                c.text("Fallback B".to_string(), Style::empty().font_size(14.0))
            }),
        )
        .mount(&mut ctx);
        let sib: NodeId = ctx.reactive_text(Style::empty().font_size(13.0), move || {
            format!("Sibling={}", sibling_state.get())
        });
        ctx.append(root, slot_a);
        ctx.append(root, slot_b);
        ctx.append(root, sib);
        root
    });
    let f0: FrameStats = app.step_frame().unwrap();
    assert_ne!(f0.get_frame_hash(), 0);
    assert_eq!(mounts_a.get(), 1);
    assert_eq!(mounts_b.get(), 1);
    let mut btn_spec: DynamicNodeSpec =
        DynamicNodeSpec::button("Upgrade Pro".to_string(), "upgrade_pro".to_string());
    btn_spec.set_payload("tier-1".to_string());
    let spec_v1: DynamicNodeSpec = DynamicNodeSpec::card("Banner v1".to_string())
        .child(DynamicNodeSpec::badge("LIVE".to_string()))
        .child(btn_spec);
    let mut upd_v1: DynamicComponentUpdate =
        DynamicComponentUpdate::new("slot_a".to_string(), 1, spec_v1);
    upd_v1.sign();
    let json_v1: String = upd_v1.to_json().unwrap();
    let applied_v1: u64 = hub.apply_component_json(&json_v1).unwrap();
    assert_eq!(applied_v1, 1);
    let f1: FrameStats = app.step_frame().unwrap();
    assert_ne!(f1.get_frame_hash(), f0.get_frame_hash());
    assert_eq!(mounts_b.get(), 1);
    assert_eq!(sibling_state.get_untracked(), 42);
    let _: FrameStats = click_text(&mut app, "Upgrade Pro");
    let count_actions: usize = captured_actions
        .try_borrow()
        .map(|l: Ref<'_, Vec<DynamicActionEvent>>| l.len())
        .unwrap_or(0);
    assert_eq!(count_actions, 1);
    let last: DynamicActionEvent = hub.last_action().get_untracked().unwrap();
    assert_eq!(last.get_source_id().as_str(), "slot_a");
    assert_eq!(last.get_action().as_str(), "upgrade_pro");
    assert_eq!(last.get_payload().as_str(), "tier-1");
    let applied_v2: u64 = hub
        .swap_component_builder("slot_a".to_string(), 2, || {
            let mut c: BuildContext = current_build_context();
            c.text(
                "Native Hot-Swap v2".to_string(),
                Style::empty().font_size(16.0).color(Color::hex(0x059669)),
            )
        })
        .unwrap();
    assert_eq!(applied_v2, 2);
    assert_eq!(hub.component_history_len("slot_a"), 2);
    let f2: FrameStats = app.step_frame().unwrap();
    assert_ne!(f2.get_frame_hash(), f1.get_frame_hash());
    assert_eq!(mounts_b.get(), 1);
    let rb1: u64 = hub.rollback_component("slot_a").unwrap();
    assert_eq!(rb1, 1);
    let f_rb1: FrameStats = app.step_frame().unwrap();
    assert_eq!(f_rb1.get_frame_hash(), f1.get_frame_hash());
    let rb0: u64 = hub.rollback_component("slot_a").unwrap();
    assert_eq!(rb0, 0);
    let f_rb0: FrameStats = app.step_frame().unwrap();
    assert_eq!(f_rb0.get_frame_hash(), f0.get_frame_hash());
    assert_eq!(mounts_b.get(), 1);
    assert!(matches!(
        hub.rollback_component("slot_a"),
        Err(DynamicUpdateError::NoHistoryToRollback(_))
    ));
}

#[test]
fn page_level_dynamic_update_swaps_active_page_and_injects_new_runtime_routes() {
    reset_reactive_runtime();
    let mut app: App = make_headless_app();
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let mut router: Router = Router::at("/home".to_string());
    let route_sig: Signal<String> = router.get_current();
    let _: NodeId = app.mount(move || {
        let mut ctx: BuildContext = current_build_context();
        DynamicPageOutlet::new(
            route_sig,
            Rc::new(|path: &str| {
                let mut c: BuildContext = current_build_context();
                c.text(
                    format!("Compiled Fallback Route: {path}"),
                    Style::empty().font_size(15.0),
                )
            }),
        )
        .mount(&mut ctx)
    });
    let f_home_0: FrameStats = app.step_frame().unwrap();
    let mut home_v1: DynamicPageUpdate = DynamicPageUpdate::new(
        "/home".to_string(),
        "Home Live v1".to_string(),
        1,
        DynamicNodeSpec::card("Dynamic Home v1".to_string())
            .child(DynamicNodeSpec::text("Updated live in place".to_string())),
    );
    home_v1.sign();
    let home_json: String = home_v1.to_json().unwrap();
    assert_eq!(hub.apply_page_json(&home_json).unwrap(), 1);
    assert_eq!(hub.page_title("/home").as_deref(), Some("Home Live v1"));
    let f_home_1: FrameStats = app.step_frame().unwrap();
    assert_ne!(f_home_1.get_frame_hash(), f_home_0.get_frame_hash());
    let mut camp_btn: DynamicNodeSpec =
        DynamicNodeSpec::button("Launch Campaign".to_string(), "start_campaign".to_string());
    camp_btn.set_payload("autumn-sale".to_string());
    let mut camp_v1: DynamicPageUpdate = DynamicPageUpdate::new(
        "/campaign-live".to_string(),
        "Autumn Campaign".to_string(),
        1,
        DynamicNodeSpec::column().child(
            DynamicNodeSpec::card("Injected Campaign Page".to_string())
                .child(DynamicNodeSpec::divider())
                .child(camp_btn),
        ),
    );
    camp_v1.sign();
    assert_eq!(hub.apply_page(camp_v1).unwrap(), 1);
    assert!(hub.has_dynamic_page("/campaign-live"));
    assert_eq!(
        hub.registered_routes(),
        vec!["/campaign-live".to_string(), "/home".to_string()]
    );
    router.push("/campaign-live".to_string());
    let f_camp_1: FrameStats = app.step_frame().unwrap();
    assert_ne!(f_camp_1.get_frame_hash(), f_home_1.get_frame_hash());
    let _: FrameStats = click_text(&mut app, "Launch Campaign");
    let act: DynamicActionEvent = hub.last_action().get_untracked().unwrap();
    assert_eq!(act.get_source_id().as_str(), "/campaign-live");
    assert_eq!(act.get_action().as_str(), "start_campaign");
    assert_eq!(
        hub.swap_page_builder(
            "/campaign-live".to_string(),
            "Autumn Campaign v2".to_string(),
            2,
            || {
                let mut c: BuildContext = current_build_context();
                c.text(
                    "Native Campaign v2".to_string(),
                    Style::empty().font_size(18.0).color(Color::hex(0xDC2626)),
                )
            },
        )
        .unwrap(),
        2
    );
    let f_camp_2: FrameStats = app.step_frame().unwrap();
    assert_ne!(f_camp_2.get_frame_hash(), f_camp_1.get_frame_hash());
    assert_eq!(hub.rollback_page("/campaign-live").unwrap(), 1);
    assert_eq!(hub.rollback_page("/campaign-live").unwrap(), 0);
    assert!(!hub.has_dynamic_page("/campaign-live"));
}

#[test]
fn atomic_bundle_validates_checksums_versions_and_rejects_partial_commits() {
    reset_reactive_runtime();
    let hub: DynamicUpdateHub = DynamicUpdateHub::new();
    let mut comp_v1: DynamicComponentUpdate = DynamicComponentUpdate::new(
        "header_cta".to_string(),
        1,
        DynamicNodeSpec::button("Get Started".to_string(), "cta_go".to_string()),
    );
    comp_v1.sign();
    let mut page_v1: DynamicPageUpdate = DynamicPageUpdate::new(
        "/pricing-live".to_string(),
        "Pricing v1".to_string(),
        1,
        DynamicNodeSpec::card("Pricing Tiers".to_string()),
    );
    page_v1.sign();
    let bundle: DynamicUpdateBundle = DynamicUpdateBundle::new("release-001".to_string(), 1)
        .push_component(comp_v1)
        .push_page(page_v1);
    let bundle_json: String = bundle.to_json().unwrap();
    let tel: DynamicTelemetry = hub.apply_bundle_json(&bundle_json).unwrap();
    assert_eq!(tel.get_components_applied(), 1);
    assert_eq!(tel.get_pages_applied(), 1);
    assert_eq!(hub.component_version("header_cta"), 1);
    assert_eq!(hub.page_version("/pricing-live"), 1);
    let mut good_comp: DynamicComponentUpdate = DynamicComponentUpdate::new(
        "header_cta".to_string(),
        2,
        DynamicNodeSpec::button("Should Rollback".to_string(), "cta_v2".to_string()),
    );
    good_comp.sign();
    let mut bad_page: DynamicPageUpdate = DynamicPageUpdate::new(
        "/pricing-live".to_string(),
        "Corrupted".to_string(),
        2,
        DynamicNodeSpec::card("Tampered Payload".to_string()),
    );
    bad_page.set_checksum(0xDEAD_BEEF_CAFE_BABE);
    let bad_bundle: DynamicUpdateBundle = DynamicUpdateBundle::new("release-bad".to_string(), 2)
        .push_component(good_comp)
        .push_page(bad_page);
    let err: DynamicUpdateError = hub.apply_bundle(bad_bundle).unwrap_err();
    assert!(matches!(err, DynamicUpdateError::ChecksumMismatch { .. }));
    assert_eq!(hub.component_version("header_cta"), 1);
    assert_eq!(hub.page_version("/pricing-live"), 1);
    let mut regressed: DynamicComponentUpdate = DynamicComponentUpdate::new(
        "header_cta".to_string(),
        1,
        DynamicNodeSpec::text("Old v1".to_string()),
    );
    regressed.sign();
    assert!(matches!(
        hub.apply_component(regressed.clone()),
        Err(DynamicUpdateError::VersionRegression { .. })
    ));
    regressed.set_allow_downgrade(true);
    assert_eq!(hub.apply_component(regressed).unwrap(), 1);
    assert!(hub.telemetry().get_untracked().get_rejected() >= 2);
}

#[test]
fn dynamic_slot_disposes_previous_reactive_scope_and_cleanups_on_swap() {
    reset_reactive_runtime();
    let mut app: App = make_headless_app();
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let cleanups: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let cleanups_fb: Rc<Cell<u32>> = cleanups.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: BuildContext = current_build_context();
        DynamicComponentSlot::new(
            "scoped_widget".to_string(),
            Rc::new(move || {
                let c_flag: Rc<Cell<u32>> = cleanups_fb.clone();
                on_cleanup(move || {
                    c_flag.set(c_flag.get() + 1);
                });
                let mut c: BuildContext = current_build_context();
                c.text("Scoped Fallback".to_string(), Style::empty())
            }),
        )
        .mount(&mut ctx)
    });
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(cleanups.get(), 0);
    let cleanups_v1: Rc<Cell<u32>> = cleanups.clone();
    assert_eq!(
        hub.swap_component_builder("scoped_widget".to_string(), 1, move || {
            let c_flag: Rc<Cell<u32>> = cleanups_v1.clone();
            on_cleanup(move || {
                c_flag.set(c_flag.get() + 1);
            });
            let mut c: BuildContext = current_build_context();
            c.text("Scoped v1".to_string(), Style::empty())
        })
        .unwrap(),
        1
    );
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(cleanups.get(), 1);
    assert_eq!(hub.rollback_component("scoped_widget").unwrap(), 0);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(cleanups.get(), 2);
}

#[test]
fn dynamic_wasm_partial_and_global_cover_all_production_cases() {
    reset_reactive_runtime();
    let mut cfg: AppConfig = AppConfig::default();
    cfg.get_mut_window()
        .set_size(800, 600)
        .set_render(RenderConfig::software());
    let mut app: App = App::headless(cfg).unwrap();
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let registry: RegistryWasmFetcher = RegistryWasmFetcher::default();
    let signing_key: &str = "test-prod-key";
    let p_v1: Vec<u8> = build_partial_widget_wasm("Partial Widget v1", 1, 0x2563EB);
    let p_v2: Vec<u8> = build_partial_widget_wasm("Partial Widget v2", 2, 0x7C3AED);
    let g_v1: Vec<u8> = build_global_app_wasm("Global Shell v1", 1);
    let g_v2: Vec<u8> = build_global_app_wasm("Global Shell v2", 2);
    registry.register(
        "euv-cdn://p/v1.wasm".to_string(),
        p_v1.clone(),
        "e1".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://p/v2.wasm".to_string(),
        p_v2.clone(),
        "e2".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://g/v1.wasm".to_string(),
        g_v1.clone(),
        "g1".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://g/v2.wasm".to_string(),
        g_v2.clone(),
        "g2".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://f/retry.wasm".to_string(),
        p_v1.clone(),
        "fr".to_string(),
        WasmFaultKind::TimeoutOnceThenSucceed,
    );
    registry.register(
        "euv-cdn://f/500.wasm".to_string(),
        p_v1.clone(),
        "f5".to_string(),
        WasmFaultKind::Http500,
    );
    registry.register(
        "euv-cdn://f/corrupt.wasm".to_string(),
        build_faulty_wasm(WasmFaultKind::CorruptedMagic),
        "fc".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://f/tamper.wasm".to_string(),
        p_v1.clone(),
        "ft".to_string(),
        WasmFaultKind::TamperedChecksum,
    );
    registry.register(
        "euv-cdn://f/abi.wasm".to_string(),
        build_faulty_wasm(WasmFaultKind::AbiMismatch),
        "fa".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://f/noexp.wasm".to_string(),
        build_faulty_wasm(WasmFaultKind::MissingExport),
        "fe".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://f/fuel.wasm".to_string(),
        build_faulty_wasm(WasmFaultKind::InfiniteLoopOutOfFuel),
        "ff".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://f/trap.wasm".to_string(),
        build_faulty_wasm(WasmFaultKind::ExecutionTrap),
        "fp".to_string(),
        WasmFaultKind::None,
    );
    registry.register(
        "euv-cdn://f/big.wasm".to_string(),
        build_faulty_wasm(WasmFaultKind::OversizedBinary),
        "fb".to_string(),
        WasmFaultKind::None,
    );
    let mgr: WasmDynamicManager = WasmDynamicManager::new(
        hub.clone(),
        Rc::new(registry.clone()),
        WasmSandboxConfig::default(),
    );
    let hub_mount: DynamicUpdateHub = hub.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: BuildContext = current_build_context();
        let mut partial_cfg: DynamicComponentSlot = DynamicComponentSlot::new(
            "slot_a".to_string(),
            Rc::new(|| {
                let mut c: BuildContext = current_build_context();
                c.text("Native Fallback A".to_string(), Style::empty())
            }),
        );
        partial_cfg.set_hub(Some(hub_mount.clone()));
        let partial_slot: NodeId = partial_cfg.mount(&mut ctx);
        let mut global_cfg: DynamicComponentSlot = DynamicComponentSlot::new(
            WASM_GLOBAL_ROOT_SLOT.to_string(),
            Rc::new(move || partial_slot),
        );
        global_cfg.set_hub(Some(hub_mount));
        global_cfg.mount(&mut ctx)
    });
    let _: FrameStats = app.step_frame().unwrap();
    let req_p1: WasmDownloadRequest =
        WasmDownloadRequest::partial("euv-cdn://p/v1.wasm".to_string(), "slot_a".to_string(), 1)
            .with_checksum(compute_wasm_checksum(&p_v1))
            .with_signature(compute_wasm_signature(&p_v1, 1, signing_key));
    assert_eq!(mgr.load_and_mount(req_p1.clone(), signing_key).unwrap(), 1);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(hub.component_version("slot_a"), 1);
    let partial_scope: WasmDynamicScope = WasmDynamicScope::Partial {
        slot_id: "slot_a".to_string(),
    };
    assert_eq!(mgr.dispatch_wasm_event(&partial_scope, 1).unwrap(), 1);
    let _: FrameStats = app.step_frame().unwrap();
    let req_downgrade: WasmDownloadRequest =
        WasmDownloadRequest::partial("euv-cdn://p/v1.wasm".to_string(), "slot_a".to_string(), 1);
    assert!(matches!(
        mgr.load_and_mount(req_downgrade, signing_key),
        Err(WasmDynamicError::VersionRegression {
            current: 1,
            incoming: 1
        })
    ));
    let req_p2: WasmDownloadRequest =
        WasmDownloadRequest::partial("euv-cdn://p/v2.wasm".to_string(), "slot_a".to_string(), 2)
            .with_checksum(compute_wasm_checksum(&p_v2))
            .with_signature(compute_wasm_signature(&p_v2, 2, signing_key));
    assert_eq!(mgr.load_and_mount(req_p2, signing_key).unwrap(), 2);
    assert_eq!(mgr.rollback_partial("slot_a", "test").unwrap(), 1);
    registry.set_offline(true);
    assert_eq!(
        mgr.load_and_mount(req_p1.with_allow_downgrade(true), signing_key)
            .unwrap(),
        1
    );
    assert!(mgr.telemetry().get_untracked().get_cache_hits() >= 1);
    registry.set_offline(false);
    let req_retry: WasmDownloadRequest = WasmDownloadRequest::partial(
        "euv-cdn://f/retry.wasm".to_string(),
        "slot_a".to_string(),
        3,
    );
    assert_eq!(mgr.load_and_mount(req_retry, signing_key).unwrap(), 3);
    assert!(mgr.telemetry().get_untracked().get_retries_performed() >= 1);
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial("ftp://bad".to_string(), "slot_a".to_string(), 4),
            signing_key
        ),
        Err(WasmDynamicError::InvalidUrl(_))
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/500.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::HttpStatus { status: 500, .. })
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/corrupt.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::InvalidWasmBinary(_))
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/tamper.wasm".to_string(),
                "slot_a".to_string(),
                4
            )
            .with_checksum(compute_wasm_checksum(&p_v1)),
            signing_key
        ),
        Err(WasmDynamicError::ChecksumMismatch { .. })
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://p/v1.wasm".to_string(),
                "slot_a".to_string(),
                4
            )
            .with_signature("euv-sig-v1:invalid".to_string()),
            signing_key
        ),
        Err(WasmDynamicError::SignatureInvalid { .. })
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/abi.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::AbiMismatch {
            expected: 1,
            actual: 99
        })
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/noexp.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::MissingExport(_))
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/fuel.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::OutOfFuel { .. })
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/trap.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::ExecutionTrap(_))
    ));
    assert!(matches!(
        mgr.load_and_mount(
            WasmDownloadRequest::partial(
                "euv-cdn://f/big.wasm".to_string(),
                "slot_a".to_string(),
                4
            ),
            signing_key
        ),
        Err(WasmDynamicError::BinaryTooLarge { .. })
    ));
    let cancel_gen: u64 = mgr.cancel_scope(&partial_scope);
    assert!(cancel_gen > 0);
    mgr.unmount_partial("slot_a");
    assert_eq!(hub.component_version("slot_a"), 0);
    let req_g1: WasmDownloadRequest =
        WasmDownloadRequest::global("euv-cdn://g/v1.wasm".to_string(), 1)
            .with_checksum(compute_wasm_checksum(&g_v1))
            .with_signature(compute_wasm_signature(&g_v1, 1, signing_key));
    assert_eq!(mgr.load_and_mount(req_g1, signing_key).unwrap(), 1);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(mgr.global_version_signal().get_untracked(), 1);
    let req_g2: WasmDownloadRequest =
        WasmDownloadRequest::global("euv-cdn://g/v2.wasm".to_string(), 2)
            .with_checksum(compute_wasm_checksum(&g_v2))
            .with_signature(compute_wasm_signature(&g_v2, 2, signing_key));
    assert_eq!(mgr.load_and_mount(req_g2, signing_key).unwrap(), 2);
    assert!(
        mgr.load_and_mount(
            WasmDownloadRequest::global("euv-cdn://f/trap.wasm".to_string(), 3),
            signing_key
        )
        .is_err()
    );
    assert_eq!(mgr.global_version_signal().get_untracked(), 2);
    assert!(matches!(
        mgr.state_signal(&WasmDynamicScope::Global).get_untracked(),
        WasmLoadState::RolledBack {
            active_version: 2,
            ..
        }
    ));
    assert_eq!(mgr.rollback_global("manual").unwrap(), 1);
    mgr.restore_native_global();
    assert_eq!(mgr.global_version_signal().get_untracked(), 0);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn http_wasm_fetcher_downloads_real_wasm_over_local_tcp_and_renders_natively() {
    reset_reactive_runtime();
    let wasm_bytes: Vec<u8> = build_partial_widget_wasm("Live HTTP Downloaded WASM", 1, 0x059669);
    let body_clone: Vec<u8> = wasm_bytes.clone();
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let handle: JoinHandle<()> = spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut req_buf: [u8; 512] = [0; 512];
            let _: Result<usize, io::Error> = Read::read(&mut stream, &mut req_buf);
            let header: String = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/wasm\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body_clone.len()
            );
            let _: Result<(), io::Error> = Write::write_all(&mut stream, header.as_bytes());
            let _: Result<(), io::Error> = Write::write_all(&mut stream, &body_clone);
        }
    });
    let mut cfg: AppConfig = AppConfig::default();
    cfg.get_mut_window()
        .set_size(800, 600)
        .set_render(RenderConfig::software());
    let mut app: App = App::headless(cfg).unwrap();
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let mgr: WasmDynamicManager = WasmDynamicManager::new(
        hub.clone(),
        Rc::new(HttpWasmFetcher::default()),
        WasmSandboxConfig::default(),
    );
    let url: String = format!("http://127.0.0.1:{}/widget.wasm", addr.port());
    let req: WasmDownloadRequest = WasmDownloadRequest::partial(url, "http_slot".to_string(), 1)
        .with_checksum(compute_wasm_checksum(&wasm_bytes));
    assert_eq!(mgr.load_and_mount(req, "").unwrap(), 1);
    assert_eq!(hub.component_version("http_slot"), 1);
    let _: Result<(), Box<dyn Any + Send>> = handle.join();
    let _: FrameStats = app.step_frame().unwrap();
}
