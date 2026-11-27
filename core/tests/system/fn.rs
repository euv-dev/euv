use super::*;

#[test]
fn accessibility_tree_compiles_scene_and_maps_roles_across_all_seven_platforms() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(400, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let mut btn_id: NodeId = NodeId::default();
    let mut hidden_id: NodeId = NodeId::default();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(400.0))
                .height(Length::Px(240.0)),
        );
        btn_id = ctx.view(
            Style::default()
                .width(Length::Px(120.0))
                .height(Length::Px(40.0)),
        );
        let mut btn_a11y: A11yProps = A11yProps::new(A11yRole::Button, "Save File".to_string());
        btn_a11y.set_hint("Saves active workspace".to_string());
        ctx.set_a11y(btn_id, btn_a11y);
        hidden_id = ctx.text("Decorative".to_string(), Style::default());
        let mut hidden_a11y: A11yProps = A11yProps::new(A11yRole::Text, "Decorative".to_string());
        hidden_a11y.set_hidden(true);
        ctx.set_a11y(hidden_id, hidden_a11y);
        ctx.append(root, btn_id);
        ctx.append(root, hidden_id);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let tree: A11yTree = app.a11y_tree();
    let btn_node: &A11yNode = tree.find(btn_id).unwrap();
    assert_eq!(btn_node.get_props().get_role(), A11yRole::Button);
    assert_eq!(btn_node.get_props().get_label(), "Save File");
    assert_eq!(btn_node.get_props().get_hint(), "Saves active workspace");
    assert!(tree.find(hidden_id).is_none());
    for target in PlatformKind::all_targets().into_iter() {
        let native_role: &str = A11yRole::Button.native_role_name(target);
        assert!(!native_role.is_empty());
        let summary: String = tree.format_summary(target);
        assert!(summary.contains("Save File"));
    }
    app.get_mut_services()
        .announce_a11y("Saved successfully".to_string(), A11yLiveRegion::Polite);
    assert_eq!(app.get_services().get_announcements().len(), 1);
}

#[test]
fn protocol_registration_and_deep_link_dispatch_work_across_all_seven_platforms() {
    reset_reactive_runtime();
    let mut bad_reg: ProtocolRegistration = ProtocolRegistration::new(
        "javascript".to_string(),
        "Unsafe".to_string(),
        "dev.euv.bad".to_string(),
    );
    assert!(!bad_reg.is_valid_scheme());
    bad_reg.set_scheme("euv".to_string());
    bad_reg.set_universal_domains(vec!["app.euv.dev".to_string()]);
    assert!(bad_reg.is_valid_scheme());
    for target in PlatformKind::all_targets().into_iter() {
        let mut registry: ProtocolRegistry = ProtocolRegistry::new();
        let manifest: ProtocolManifestEntry = registry.register(bad_reg.clone(), target).unwrap();
        assert!(registry.is_registered("euv"));
        assert!(!manifest.get_mechanism().is_empty());
        assert!(manifest.get_descriptor().contains("euv"));
    }
    let parsed: DeepLinkEvent =
        DeepLinkEvent::parse("euv://studio/editor?tab=gpu&fps=120").unwrap();
    assert_eq!(parsed.get_scheme(), "euv");
    assert_eq!(parsed.get_host(), "studio");
    assert_eq!(parsed.get_path(), "/editor");
    assert_eq!(
        parsed.get_query().get("tab").map(String::as_str),
        Some("gpu")
    );
    assert_eq!(
        parsed.get_query().get("fps").map(String::as_str),
        Some("120")
    );
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(240, 120).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    app.dispatch_window_event(WindowEvent::DeepLink(parsed.clone()));
    assert_eq!(app.get_deep_link().get_untracked(), Some(parsed));
}

#[test]
fn dynamic_island_permissions_haptics_biometrics_and_tray_adapt_per_platform() {
    let mut ios_hub: PlatformServiceHub = PlatformServiceHub::new(PlatformKind::Ios);
    let mut content: LiveActivityContent =
        LiveActivityContent::new("Render Export".to_string(), "Metal encoding...".to_string());
    content
        .set_compact_leading("euv".to_string())
        .set_compact_trailing("42%".to_string())
        .set_minimal_badge("42".to_string())
        .set_progress_percent(Some(42));
    let handle: LiveActivityHandle =
        ios_hub.start_live_activity("export-1".to_string(), content.clone());
    assert_eq!(handle.get_presentation(), IslandPresentationMode::Compact);
    assert!(handle.get_active());
    content.set_progress_percent(Some(100));
    assert!(ios_hub.update_live_activity("export-1", content));
    assert!(ios_hub.end_live_activity("export-1"));
    assert!(ios_hub.trigger_haptic(HapticStyle::Success));
    assert_eq!(ios_hub.biometric_kind(), BiometricKind::FaceId);
    assert!(
        ios_hub
            .configure_tray(SystemTrayConfig::new("euv".to_string()))
            .is_err()
    );
    let mut mac_hub: PlatformServiceHub = PlatformServiceHub::new(PlatformKind::MacOs);
    let mac_handle: LiveActivityHandle = mac_hub.start_live_activity(
        "sync-1".to_string(),
        LiveActivityContent::new("Sync".to_string(), "Done".to_string()),
    );
    assert_eq!(
        mac_handle.get_presentation(),
        IslandPresentationMode::FallbackStatusBar
    );
    assert_eq!(mac_hub.biometric_kind(), BiometricKind::TouchId);
    assert!(
        mac_hub
            .configure_tray(SystemTrayConfig::new("euv menu".to_string()))
            .is_ok()
    );
    assert!(mac_hub.tray_config().is_some());
    let mut android_hub: PlatformServiceHub = PlatformServiceHub::new(PlatformKind::Android);
    let android_handle: LiveActivityHandle = android_hub.start_live_activity(
        "upload-1".to_string(),
        LiveActivityContent::new("Upload".to_string(), "Active".to_string()),
    );
    assert_eq!(
        android_handle.get_presentation(),
        IslandPresentationMode::FallbackNotification
    );
    assert_eq!(android_hub.biometric_kind(), BiometricKind::Fingerprint);
    for target in PlatformKind::all_targets().into_iter() {
        let mut perms: PermissionManager = PermissionManager::new();
        let cam: PermissionDescriptor = perms.request(
            SystemPermission::Camera,
            "Capture native video frames".to_string(),
            target,
        );
        assert_eq!(cam.get_status(), PermissionStatus::Granted);
        assert!(!cam.get_native_key().is_empty());
        assert_eq!(
            perms.check(SystemPermission::Camera, target),
            PermissionStatus::Granted
        );
        let hub: PlatformServiceHub = PlatformServiceHub::new(target);
        let share_res: String = hub
            .share(&SharePayload::new(
                "euv".to_string(),
                "Native Rust UI".to_string(),
            ))
            .unwrap();
        assert!(share_res.contains("euv"));
    }
    let caps_ios: PlatformCapabilities = PlatformCapabilities::for_platform(PlatformKind::Ios);
    assert!(caps_ios.get_dynamic_island());
    assert!(caps_ios.get_live_activities());
    assert!(caps_ios.get_accessibility_bridge());
    assert!(caps_ios.get_protocol_handler());
}

#[test]
fn system_event_normalizer_enforces_canonical_order_and_deduplicates_events() {
    let mut norm: SystemEventNormalizer = SystemEventNormalizer::new();
    assert_eq!(norm.state(), AppState::Active);
    assert!(norm.is_visible());
    let bg_events: Vec<SystemEvent> = norm.normalize(SystemEvent::Background);
    assert_eq!(
        bg_events,
        vec![
            SystemEvent::Inactive,
            SystemEvent::Disappear,
            SystemEvent::Background,
        ]
    );
    assert_eq!(norm.state(), AppState::Background);
    assert!(!norm.is_visible());
    assert!(norm.normalize(SystemEvent::Background).is_empty());
    assert!(norm.normalize(SystemEvent::Inactive).is_empty());
    assert!(norm.normalize(SystemEvent::Disappear).is_empty());
    let fg_events: Vec<SystemEvent> = norm.normalize(SystemEvent::Foreground);
    assert_eq!(
        fg_events,
        vec![SystemEvent::Appear, SystemEvent::Foreground]
    );
    assert_eq!(norm.state(), AppState::Active);
    assert!(norm.is_visible());
    assert!(norm.normalize(SystemEvent::Appear).is_empty());
    assert!(norm.normalize(SystemEvent::Foreground).is_empty());
    assert_eq!(
        norm.normalize(SystemEvent::Inactive),
        vec![SystemEvent::Inactive]
    );
    assert_eq!(norm.state(), AppState::Inactive);
    assert_eq!(
        norm.normalize(SystemEvent::Background),
        vec![SystemEvent::Disappear, SystemEvent::Background]
    );
    assert_eq!(
        norm.normalize(SystemEvent::Appear),
        vec![SystemEvent::Appear]
    );
    assert_eq!(norm.state(), AppState::Inactive);
    assert!(norm.is_visible());
    assert_eq!(
        norm.normalize(SystemEvent::Foreground),
        vec![SystemEvent::Foreground]
    );
    assert_eq!(norm.state(), AppState::Active);
    assert_eq!(
        norm.normalize(SystemEvent::Terminate),
        vec![
            SystemEvent::Inactive,
            SystemEvent::Disappear,
            SystemEvent::Terminate,
        ]
    );
    assert_eq!(norm.state(), AppState::Terminated);
    assert!(!norm.is_visible());
    assert!(norm.normalize(SystemEvent::Appear).is_empty());
    assert!(norm.normalize(SystemEvent::Foreground).is_empty());
    assert!(norm.normalize(SystemEvent::BackPressed).is_empty());
    let mut env_norm: SystemEventNormalizer = SystemEventNormalizer::new();
    assert_eq!(
        env_norm.normalize(SystemEvent::FocusChanged(true)),
        vec![SystemEvent::FocusChanged(true)]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::FocusChanged(true))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::FocusChanged(false)),
        vec![SystemEvent::FocusChanged(false)]
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::Resize {
            width: 390.0,
            height: 844.0,
            scale_factor: 3.0,
        }),
        vec![SystemEvent::Resize {
            width: 390.0,
            height: 844.0,
            scale_factor: 3.0,
        }]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::Resize {
                width: 390.0,
                height: 844.0,
                scale_factor: 3.0,
            })
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::Resize {
            width: 844.0,
            height: 390.0,
            scale_factor: 3.0,
        }),
        vec![
            SystemEvent::Resize {
                width: 844.0,
                height: 390.0,
                scale_factor: 3.0,
            },
            SystemEvent::OrientationChanged(ScreenOrientation::Landscape),
        ]
    );
    assert!(ScreenOrientation::Landscape.is_landscape());
    assert!(ScreenOrientation::Portrait.is_portrait());
    assert_eq!(
        ScreenOrientation::from_size(390.0, 844.0),
        ScreenOrientation::Portrait
    );
    assert_eq!(
        ScreenOrientation::from_size(844.0, 390.0),
        ScreenOrientation::Landscape
    );
    let insets: SafeAreaInsets = SafeAreaInsets::new(47.0, 0.0, 34.0, 0.0);
    assert_eq!(
        env_norm.normalize(SystemEvent::SafeAreaChanged(insets)),
        vec![SystemEvent::SafeAreaChanged(insets)]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::SafeAreaChanged(insets))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::ThemeChanged(SystemTheme::Dark)),
        vec![SystemEvent::ThemeChanged(SystemTheme::Dark)]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::ThemeChanged(SystemTheme::Dark))
            .is_empty()
    );
    assert!(
        env_norm
            .normalize(SystemEvent::LocaleChanged("   ".to_string()))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::LocaleChanged("en-US".to_string())),
        vec![SystemEvent::LocaleChanged("en-US".to_string())]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::LocaleChanged("en-US".to_string()))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::KeyboardVisibilityChanged {
            visible: true,
            height: 312.0,
        }),
        vec![SystemEvent::KeyboardVisibilityChanged {
            visible: true,
            height: 312.0,
        }]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::KeyboardVisibilityChanged {
                visible: true,
                height: 312.0,
            })
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::KeyboardVisibilityChanged {
            visible: false,
            height: 120.0,
        }),
        vec![SystemEvent::KeyboardVisibilityChanged {
            visible: false,
            height: 0.0,
        }]
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::LowPowerModeChanged(true)),
        vec![SystemEvent::LowPowerModeChanged(true)]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::LowPowerModeChanged(true))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::NetworkStatusChanged(NetworkStatus::Wifi)),
        vec![SystemEvent::NetworkStatusChanged(NetworkStatus::Wifi)]
    );
    assert!(NetworkStatus::Wifi.is_online());
    assert!(!NetworkStatus::Offline.is_online());
    assert!(
        env_norm
            .normalize(SystemEvent::NetworkStatusChanged(NetworkStatus::Wifi))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::MemoryWarning(MemoryPressure::Warning)),
        vec![SystemEvent::MemoryWarning(MemoryPressure::Warning)]
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::MemoryWarning(MemoryPressure::Critical)),
        vec![SystemEvent::MemoryWarning(MemoryPressure::Critical)]
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::BackPressed),
        vec![SystemEvent::BackPressed]
    );
    assert!(
        env_norm
            .normalize(SystemEvent::DeepLink(String::new()))
            .is_empty()
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::DeepLink("euv://open/home".to_string())),
        vec![SystemEvent::DeepLink("euv://open/home".to_string())]
    );
    assert_eq!(
        env_norm.normalize(SystemEvent::ClipboardChanged),
        vec![SystemEvent::ClipboardChanged]
    );
}

#[test]
fn scoped_system_event_hooks_reactive_signals_cleanup_and_freeze_work_end_to_end() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(400, 700).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let all_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let appear_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let disappear_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fg_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let bg_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let inactive_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let terminate_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let last_focus: Rc<Cell<Option<bool>>> = Rc::new(Cell::new(None));
    let last_resize: Rc<Cell<(f32, f32, f32)>> = Rc::new(Cell::new((0.0, 0.0, 0.0)));
    let last_safe: Rc<Cell<SafeAreaInsets>> = Rc::new(Cell::new(SafeAreaInsets::default()));
    let last_orient: Rc<Cell<Option<ScreenOrientation>>> = Rc::new(Cell::new(None));
    let last_theme: Rc<Cell<Option<SystemTheme>>> = Rc::new(Cell::new(None));
    let last_locale: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let last_kb: Rc<Cell<(bool, f32)>> = Rc::new(Cell::new((false, 0.0)));
    let last_mem: Rc<Cell<Option<MemoryPressure>>> = Rc::new(Cell::new(None));
    let last_low_power: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let last_net: Rc<Cell<Option<NetworkStatus>>> = Rc::new(Cell::new(None));
    let back_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let last_link: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let clip_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let frozen_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let active_terminate_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let manual_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let state_sig_cell: Rc<Cell<Option<Signal<AppState>>>> = Rc::new(Cell::new(None));
    let theme_sig_cell: Rc<Cell<Option<Signal<SystemTheme>>>> = Rc::new(Cell::new(None));
    let net_sig_cell: Rc<Cell<Option<Signal<NetworkStatus>>>> = Rc::new(Cell::new(None));
    let kb_sig_cell: Rc<Cell<Option<Signal<f32>>>> = Rc::new(Cell::new(None));
    let safe_sig_cell: Rc<Cell<Option<Signal<SafeAreaInsets>>>> = Rc::new(Cell::new(None));
    let scope_a_cell: Rc<Cell<Option<Scope>>> = Rc::new(Cell::new(None));
    let scope_b_cell: Rc<Cell<Option<Scope>>> = Rc::new(Cell::new(None));
    let manual_sub_cell: Rc<Cell<Option<SystemEventSubscription>>> = Rc::new(Cell::new(None));
    let all_c: Rc<Cell<u32>> = all_count.clone();
    let app_c: Rc<Cell<u32>> = appear_count.clone();
    let dis_c: Rc<Cell<u32>> = disappear_count.clone();
    let fg_c: Rc<Cell<u32>> = fg_count.clone();
    let bg_c: Rc<Cell<u32>> = bg_count.clone();
    let inact_c: Rc<Cell<u32>> = inactive_count.clone();
    let term_c: Rc<Cell<u32>> = terminate_count.clone();
    let foc_c: Rc<Cell<Option<bool>>> = last_focus.clone();
    let res_c: Rc<Cell<(f32, f32, f32)>> = last_resize.clone();
    let safe_c: Rc<Cell<SafeAreaInsets>> = last_safe.clone();
    let ori_c: Rc<Cell<Option<ScreenOrientation>>> = last_orient.clone();
    let thm_c: Rc<Cell<Option<SystemTheme>>> = last_theme.clone();
    let loc_c: Rc<RefCell<String>> = last_locale.clone();
    let kb_c: Rc<Cell<(bool, f32)>> = last_kb.clone();
    let mem_c: Rc<Cell<Option<MemoryPressure>>> = last_mem.clone();
    let lp_c: Rc<Cell<bool>> = last_low_power.clone();
    let net_c: Rc<Cell<Option<NetworkStatus>>> = last_net.clone();
    let back_c: Rc<Cell<u32>> = back_count.clone();
    let link_c: Rc<RefCell<String>> = last_link.clone();
    let clip_c: Rc<Cell<u32>> = clip_count.clone();
    let frz_c: Rc<Cell<u32>> = frozen_hits.clone();
    let act_term_c: Rc<Cell<u32>> = active_terminate_hits.clone();
    let man_c: Rc<Cell<u32>> = manual_hits.clone();
    let st_sig_c: Rc<Cell<Option<Signal<AppState>>>> = state_sig_cell.clone();
    let th_sig_c: Rc<Cell<Option<Signal<SystemTheme>>>> = theme_sig_cell.clone();
    let nt_sig_c: Rc<Cell<Option<Signal<NetworkStatus>>>> = net_sig_cell.clone();
    let kb_sig_c: Rc<Cell<Option<Signal<f32>>>> = kb_sig_cell.clone();
    let sf_sig_c: Rc<Cell<Option<Signal<SafeAreaInsets>>>> = safe_sig_cell.clone();
    let sa_cell_c: Rc<Cell<Option<Scope>>> = scope_a_cell.clone();
    let sb_cell_c: Rc<Cell<Option<Scope>>> = scope_b_cell.clone();
    let ms_cell_c: Rc<Cell<Option<SystemEventSubscription>>> = manual_sub_cell.clone();
    let _: NodeId = app.mount(move || {
        st_sig_c.set(Some(use_app_state()));
        th_sig_c.set(Some(use_system_theme()));
        nt_sig_c.set(Some(use_network_status()));
        kb_sig_c.set(Some(use_keyboard_height()));
        sf_sig_c.set(Some(use_safe_area()));
        let (scope_a, _): (Scope, ()) = Scope::run(move |_s: Scope| {
            let _: SystemEventSubscription = on_system_event(move |_ev: &SystemEvent| {
                all_c.set(all_c.get() + 1);
            });
            let _: SystemEventSubscription = on_appear(move || {
                app_c.set(app_c.get() + 1);
            });
            let _: SystemEventSubscription = on_disappear(move || {
                dis_c.set(dis_c.get() + 1);
            });
            let _: SystemEventSubscription = on_foreground(move || {
                fg_c.set(fg_c.get() + 1);
            });
            let _: SystemEventSubscription = on_background(move || {
                bg_c.set(bg_c.get() + 1);
            });
            let _: SystemEventSubscription = on_inactive(move || {
                inact_c.set(inact_c.get() + 1);
            });
            let _: SystemEventSubscription = on_terminate(move || {
                term_c.set(term_c.get() + 1);
            });
            let _: SystemEventSubscription = on_focus_change(move |focused: bool| {
                foc_c.set(Some(focused));
            });
            let _: SystemEventSubscription = on_resize(move |w: f32, h: f32, s: f32| {
                res_c.set((w, h, s));
            });
            let _: SystemEventSubscription = on_safe_area_change(move |insets: SafeAreaInsets| {
                safe_c.set(insets);
            });
            let _: SystemEventSubscription =
                on_orientation_change(move |orient: ScreenOrientation| {
                    ori_c.set(Some(orient));
                });
            let _: SystemEventSubscription = on_theme_change(move |theme: SystemTheme| {
                thm_c.set(Some(theme));
            });
            let _: SystemEventSubscription = on_locale_change(move |loc: &str| {
                *loc_c.borrow_mut() = loc.to_string();
            });
            let _: SystemEventSubscription = on_keyboard_change(move |vis: bool, h: f32| {
                kb_c.set((vis, h));
            });
            let _: SystemEventSubscription = on_memory_warning(move |p: MemoryPressure| {
                mem_c.set(Some(p));
            });
            let _: SystemEventSubscription = on_low_power_change(move |en: bool| {
                lp_c.set(en);
            });
            let _: SystemEventSubscription = on_network_change(move |st: NetworkStatus| {
                net_c.set(Some(st));
            });
            let _: SystemEventSubscription = on_system_back(move || {
                back_c.set(back_c.get() + 1);
            });
            let _: SystemEventSubscription = on_deep_link(move |uri: &str| {
                *link_c.borrow_mut() = uri.to_string();
            });
            let _: SystemEventSubscription = on_clipboard_change(move || {
                clip_c.set(clip_c.get() + 1);
            });
        });
        let (scope_b, _): (Scope, ()) = Scope::run(move |_s: Scope| {
            let _: SystemEventSubscription = on_system_back(move || {
                frz_c.set(frz_c.get() + 1);
            });
            let _: SystemEventSubscription = on_terminate(move || {
                act_term_c.set(act_term_c.get() + 1);
            });
        });
        let manual_sub: SystemEventSubscription = on_system_back(move || {
            man_c.set(man_c.get() + 1);
        });
        sa_cell_c.set(Some(scope_a));
        sb_cell_c.set(Some(scope_b));
        ms_cell_c.set(Some(manual_sub));
        let mut ctx: Context = current_context();
        ctx.view(Style::default())
    });
    let state_sig: Signal<AppState> = state_sig_cell.get().unwrap();
    let theme_sig: Signal<SystemTheme> = theme_sig_cell.get().unwrap();
    let net_sig: Signal<NetworkStatus> = net_sig_cell.get().unwrap();
    let kb_sig: Signal<f32> = kb_sig_cell.get().unwrap();
    let safe_sig: Signal<SafeAreaInsets> = safe_sig_cell.get().unwrap();
    assert_eq!(app.app_state(), AppState::Active);
    assert_eq!(app.app_state_signal().get_untracked(), AppState::Active);
    assert_eq!(state_sig.get_untracked(), AppState::Active);
    app.dispatch_window_event(PlatformEvent::System(SystemEvent::Background));
    assert_eq!(app.app_state(), AppState::Background);
    assert_eq!(state_sig.get_untracked(), AppState::Background);
    assert_eq!(inactive_count.get(), 1);
    assert_eq!(disappear_count.get(), 1);
    assert_eq!(bg_count.get(), 1);
    app.dispatch_window_event(WindowEvent::Lifecycle(LifecycleEvent::Resumed));
    assert_eq!(app.app_state(), AppState::Active);
    assert_eq!(appear_count.get(), 1);
    assert_eq!(fg_count.get(), 1);
    app.dispatch_window_event(WindowEvent::Focused(wid, false));
    assert_eq!(last_focus.get(), Some(false));
    app.dispatch_window_event(WindowEvent::Resized(wid, 800, 400, 2.0));
    assert_eq!(last_resize.get(), (800.0, 400.0, 2.0));
    app.dispatch_window_event(WindowEvent::Resized(wid, 400, 800, 2.0));
    assert_eq!(last_orient.get(), Some(ScreenOrientation::Portrait));
    let safe_insets: SafeAreaInsets = SafeAreaInsets::new(59.0, 0.0, 34.0, 0.0);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(wid, safe_insets));
    assert_eq!(last_safe.get(), safe_insets);
    assert_eq!(safe_sig.get_untracked(), safe_insets);
    app.dispatch_window_event(WindowEvent::ThemeChanged(wid, ThemeMode::Dark));
    assert_eq!(last_theme.get(), Some(SystemTheme::Dark));
    assert_eq!(theme_sig.get_untracked(), SystemTheme::Dark);
    app.dispatch_system_event(SystemEvent::LocaleChanged("ja-JP".to_string()));
    assert_eq!(last_locale.borrow().as_str(), "ja-JP");
    app.dispatch_system_event(SystemEvent::KeyboardVisibilityChanged {
        visible: true,
        height: 290.0,
    });
    assert_eq!(last_kb.get(), (true, 290.0));
    assert!((kb_sig.get_untracked() - 290.0).abs() < 0.01);
    app.dispatch_system_event(SystemEvent::MemoryWarning(MemoryPressure::Critical));
    assert_eq!(last_mem.get(), Some(MemoryPressure::Critical));
    app.dispatch_system_event(SystemEvent::LowPowerModeChanged(true));
    assert!(last_low_power.get());
    app.dispatch_system_event(SystemEvent::NetworkStatusChanged(NetworkStatus::Cellular));
    assert_eq!(last_net.get(), Some(NetworkStatus::Cellular));
    assert_eq!(net_sig.get_untracked(), NetworkStatus::Cellular);
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    assert_eq!(back_count.get(), 1);
    assert_eq!(frozen_hits.get(), 1);
    assert_eq!(manual_hits.get(), 1);
    manual_sub_cell.get().unwrap().unsubscribe();
    let scope_b: Scope = scope_b_cell.get().unwrap();
    scope_b.freeze();
    app.dispatch_system_event(SystemEvent::BackPressed);
    assert_eq!(back_count.get(), 2);
    assert_eq!(frozen_hits.get(), 1);
    assert_eq!(manual_hits.get(), 1);
    scope_b.resume();
    app.dispatch_system_event(SystemEvent::BackPressed);
    assert_eq!(back_count.get(), 3);
    assert_eq!(frozen_hits.get(), 2);
    let parsed_link: DeepLinkEvent = DeepLinkEvent::parse("euv://docs/lifecycle").unwrap();
    app.dispatch_window_event(WindowEvent::DeepLink(parsed_link));
    assert_eq!(last_link.borrow().as_str(), "euv://docs/lifecycle");
    app.write_clipboard("copied-by-test").unwrap();
    assert_eq!(clip_count.get(), 1);
    assert!(all_count.get() >= 18);
    let scope_a: Scope = scope_a_cell.get().unwrap();
    scope_a.dispose();
    let count_before_dispose_events: u32 = all_count.get();
    app.dispatch_system_event(SystemEvent::Terminate);
    assert_eq!(app.app_state(), AppState::Terminated);
    assert_eq!(terminate_count.get(), 0);
    assert_eq!(active_terminate_hits.get(), 1);
    assert_eq!(all_count.get(), count_before_dispose_events);
    scope_b.dispose();
}

struct SubtreePageProbe {
    label: &'static str,
    log: Rc<RefCell<Vec<String>>>,
}

impl Lifecycle for SubtreePageProbe {
    fn on_appear(&self) {
        let Self { label, log } = self;
        log.borrow_mut().push(format!("lc:{label}:appear"));
    }

    fn on_disappear(&self) {
        let Self { label, log } = self;
        log.borrow_mut().push(format!("lc:{label}:disappear"));
    }
}

#[test]
fn page_and_subtree_visibility_dispatch_isolates_target_subtree() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 240).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let page_a_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let page_b_cell: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let log_c: Rc<RefCell<Vec<String>>> = log.clone();
    let pa_c: Rc<Cell<NodeId>> = page_a_cell.clone();
    let pb_c: Rc<Cell<NodeId>> = page_b_cell.clone();
    let _: NodeId = app.mount(move || {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(Style::default().column());
        let log_a1: Rc<RefCell<Vec<String>>> = log_c.clone();
        let log_a2: Rc<RefCell<Vec<String>>> = log_c.clone();
        let (scope_a, page_a): (Scope, NodeId) = Scope::run(move |_s: Scope| {
            let _: SystemEventSubscription = on_appear(move || {
                log_a1.borrow_mut().push("hook:a:appear".to_string());
            });
            let _: SystemEventSubscription = on_disappear(move || {
                log_a2.borrow_mut().push("hook:a:disappear".to_string());
            });
            let mut inner: Context = current_context();
            inner.view(Style::default())
        });
        let log_b1: Rc<RefCell<Vec<String>>> = log_c.clone();
        let log_b2: Rc<RefCell<Vec<String>>> = log_c.clone();
        let (scope_b, page_b): (Scope, NodeId) = Scope::run(move |_s: Scope| {
            let _: SystemEventSubscription = on_appear(move || {
                log_b1.borrow_mut().push("hook:b:appear".to_string());
            });
            let _: SystemEventSubscription = on_disappear(move || {
                log_b2.borrow_mut().push("hook:b:disappear".to_string());
            });
            let mut inner: Context = current_context();
            inner.view(Style::default())
        });
        let probe_a: Rc<dyn Lifecycle> = Rc::new(SubtreePageProbe {
            label: "a",
            log: log_c.clone(),
        });
        let probe_b: Rc<dyn Lifecycle> = Rc::new(SubtreePageProbe {
            label: "b",
            log: log_c.clone(),
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(page_a, scope_a);
                tree.attach_lifecycle(page_a, probe_a);
                tree.attach_scope(page_b, scope_b);
                tree.attach_lifecycle(page_b, probe_b);
            },
            (),
        );
        ctx.append_many(root, &[page_a, page_b]);
        pa_c.set(page_a);
        pb_c.set(page_b);
        root
    });
    let page_a: NodeId = page_a_cell.get();
    let page_b: NodeId = page_b_cell.get();
    app.get_context().emit_subtree_disappear(page_a);
    app.get_context().emit_subtree_appear(page_b);
    assert_eq!(
        log.borrow().as_slice(),
        &[
            "lc:a:disappear".to_string(),
            "hook:a:disappear".to_string(),
            "lc:b:appear".to_string(),
            "hook:b:appear".to_string(),
        ]
    );
    log.borrow_mut().clear();
    app.get_context().emit_page_disappear();
    app.get_context().emit_page_appear();
    assert!(log.borrow().iter().any(|s: &String| s == "lc:a:disappear"));
    assert!(log.borrow().iter().any(|s: &String| s == "lc:b:appear"));
}
