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
        let mut ctx: BuildContext = current_build_context();
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Px(400.0))
                .height(Length::Px(240.0)),
        );
        btn_id = ctx.view(
            Style::empty()
                .width(Length::Px(120.0))
                .height(Length::Px(40.0)),
        );
        let mut btn_a11y: A11yProps = A11yProps::new(A11yRole::Button, "Save File".to_string());
        btn_a11y.set_hint("Saves active workspace".to_string());
        ctx.set_a11y(btn_id, btn_a11y);
        hidden_id = ctx.text("Decorative".to_string(), Style::empty());
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
