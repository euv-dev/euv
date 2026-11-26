use super::*;

/// Construction and helper methods for `ShowcaseState`.
impl ShowcaseState {
    /// Initializes a new reactive `ShowcaseState` with default values, derived `computed!` memos, and game host.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized reactive showcase state.
    #[must_use]
    pub fn new() -> Self {
        let counter: Signal<i32> = Signal::new(1);
        let doubled: Memo<i32> = computed!((counter) -> i32 => move |n: i32| n * 2);
        let arena: DemoPhysicsScene = DemoPhysicsScene::create_arena();
        let game_host: GameEngineHost = GameEngineHost::new(arena, 1.0 / 60.0);
        game_host.set_paused(true);
        let engine3d_quality: Signal<RenderQualityConfig> =
            Signal::new(RenderQualityConfig::from_preset(QualityPreset::Cinematic));
        let engine3d_preset_idx: Signal<usize> = Signal::new(4);
        let engine3d_rt_idx: Signal<usize> = Signal::new(3);
        let engine3d_res_scale: Signal<f32> = Signal::new(100.0);
        let engine3d_time_of_day: Signal<f32> = Signal::new(17.2);
        let engine3d_import_queue: Signal<u64> = Signal::new(0);
        let engine3d_import_status: Signal<String> = Signal::new(
            "Ready: Cinematic + RT Full GI active — import .OBJ or .glTF live".to_string(),
        );
        let engine3d_camera_mode: Signal<u8> = Signal::new(0);
        let scene3d: DemoOpenWorld3DScene = DemoOpenWorld3DScene::create_city(
            engine3d_quality,
            engine3d_time_of_day,
            engine3d_import_queue,
            engine3d_import_status,
            engine3d_camera_mode,
        );
        let game3d_host: GameEngineHost = GameEngineHost::new(scene3d, 1.0 / 60.0);
        game3d_host.set_paused(true);
        let router: EuvRouter =
            EuvRouter::new(ROUTE_OVERVIEW).with_default_retention(PageRetention::Freeze);
        let dynamic_hub: DynamicUpdateHub = DynamicUpdateHub::new();
        let wasm_registry: RegistryWasmFetcher = RegistryWasmFetcher::default();
        Self::populate_wasm_registry(&wasm_registry);
        let wasm_manager: WasmDynamicManager = WasmDynamicManager::new(
            dynamic_hub.clone(),
            Rc::new(wasm_registry.clone()),
            WasmSandboxConfig::default(),
        );
        Self {
            route: Signal::new(ROUTE_OVERVIEW.to_string()),
            counter,
            doubled,
            render_mode_index: Signal::new(0),
            theme: UiThemeState::new(active_feature_scheme(), ThemeMode::Light),
            username: Signal::new(DEFAULT_USERNAME.to_string()),
            bio: Signal::new("Zero-WebView native Rust UI".to_string()),
            stepper_value: Signal::new(4),
            radio_index: Signal::new(1),
            slider_value: Signal::new(65.0),
            select_index: Signal::new(0),
            select_open: Signal::new(false),
            rating_value: Signal::new(4),
            pin_toggled: Signal::new(false),
            hardware_accel: Signal::new(true),
            agree_terms: Signal::new(true),
            modal_open: Signal::new(false),
            drawer_open: Signal::new(false),
            sheet_open: Signal::new(false),
            popover_open: Signal::new(false),
            dropdown_open: Signal::new(false),
            dropdown_selected: Signal::new(None),
            toast_visible: Signal::new(false),
            notif_visible: Signal::new(false),
            loading_active: Signal::new(false),
            active_tab: Signal::new(0),
            page_index: Signal::new(1),
            step_index: Signal::new(1),
            accordion_open: Signal::new(Some(0)),
            fullscreen: Signal::new(false),
            list_scroll_y: Signal::new(640.0),
            island_active: Signal::new(false),
            island_progress: Signal::new(25),
            camera_perm_status: Signal::new("NotDetermined".to_string()),
            protocol_status: Signal::new("Unregistered".to_string()),
            a11y_announcements: Signal::new(0),
            haptic_pulses: Signal::new(0),
            game_host,
            game3d_host,
            engine3d_quality,
            engine3d_preset_idx,
            engine3d_rt_idx,
            engine3d_res_scale,
            engine3d_time_of_day,
            engine3d_import_queue,
            engine3d_import_status,
            engine3d_camera_mode,
            dynamic_hub,
            sibling_counter: Signal::new(0),
            dynamic_action_log: Signal::new("None yet".to_string()),
            alert_dialog_open: Signal::new(false),
            alert_dialog_confirmed: Signal::new(false),
            collapsible_open: Signal::new(true),
            toggle_group_index: Signal::new(0),
            otp_code: Signal::new("202642".to_string()),
            command_query: Signal::new(String::new()),
            command_selected: Signal::new(Some(0)),
            hover_card_open: Signal::new(false),
            calendar_day: Signal::new(25),
            menubar_active: Signal::new(None),
            carousel_index: Signal::new(0),
            media_gif: Signal::new(GifAnimation::sample_neon_gif()),
            media_live: Signal::new(LivePhotoAsset::sample_live_photo()),
            media_video: Signal::new(VideoTrack::sample_hdr_video()),
            media_audio: Signal::new(AudioTrack::sample_synth_wav()),
            media_pdf: Signal::new(PdfDocument::sample_pdf()),
            media_html_link_log: Signal::new("Click any native HTML link below".to_string()),
            os_target_platform_idx: Signal::new(0),
            island_mode_idx: Signal::new(0),
            system_notifications: Signal::new(vec![(
                "Euv System Ready".to_string(),
                "Native notification bus active across all 7 targets".to_string(),
            )]),
            permission_statuses: Signal::new(vec![PermissionStatus::NotDetermined; 14]),
            lab_combobox_query: Signal::new(String::new()),
            lab_combobox_selected: Signal::new(0),
            lab_combobox_open: Signal::new(false),
            lab_table_filter: Signal::new(String::new()),
            lab_table_sort_asc: Signal::new(true),
            lab_table_selected: Signal::new(Some(0)),
            lab_date_day: Signal::new(26),
            lab_date_open: Signal::new(false),
            lab_input_group_val: Signal::new("showcase".to_string()),
            lab_native_select_idx: Signal::new(0),
            lab_nav_menu_active: Signal::new(None),
            lab_split_ratio: Signal::new(0.45),
            lab_block_scroll_chain: Signal::new(true),
            lab_stop_click_bubble: Signal::new(true),
            lab_parent_clicks: Signal::new(0),
            lab_child_clicks: Signal::new(0),
            lab_resize_preserve_count: Signal::new(1),
            lab_rtl_mode: Signal::new(false),
            wasm_registry,
            wasm_manager,
            wasm_offline: Signal::new(false),
            wasm_fault_status: Signal::new(
                "Ready: test partial/global .wasm download, cache, retry, traps & rollback"
                    .to_string(),
            ),
            cmd_palette_open: Signal::new(false),
            cmd_palette_query: Signal::new(String::new()),
            picker_color: Signal::new(Color::hex(0x2563EB)),
            picker_time: Signal::new((14, 30)),
            tag_items: Signal::new(vec![
                DEFAULT_TAG_RUST.to_string(),
                DEFAULT_TAG_NATIVE.to_string(),
                DEFAULT_TAG_WASM.to_string(),
            ]),
            tree_selected: Signal::new(None),
            breadcrumb_active: Signal::new(2),
            stress_results: Signal::new(default_stress_results()),
            stress_summary: Signal::new(STRESS_SUMMARY_READY.to_string()),
            router,
            router_ready: Rc::new(Cell::new(false)),
            lifecycle_mounts: Signal::new(0),
            lifecycle_freezes: Signal::new(0),
            lifecycle_resumes: Signal::new(0),
            lifecycle_destroys: Signal::new(0),
            lifecycle_last_event: Signal::new("Booting lazy Freeze router...".to_string()),
            markdown_copy_log: Signal::new("Tap Copy on any Eva code block".to_string()),
            markdown_link_log: Signal::new("Tap any Markdown hyperlink below".to_string()),
            markdown_tasks_toggled: Signal::new(0),
            markdown_lang_tab: Signal::new(0),
        }
    }

    /// Registers genuine executable partial, global, and fault-case `.wasm` binaries into `registry`.
    ///
    /// # Arguments
    ///
    /// - `&RegistryWasmFetcher` - Target CDN registry fetcher.
    pub fn populate_wasm_registry(registry: &RegistryWasmFetcher) {
        let p_v1: Vec<u8> =
            build_partial_widget_wasm("Downloaded Partial WASM Widget v1", 1, 0x2563EB);
        let p_v2: Vec<u8> =
            build_partial_widget_wasm("Hot-Swapped Partial WASM Widget v2", 2, 0x7C3AED);
        let g_v1: Vec<u8> = build_global_app_wasm("Global Dynamic WASM App Shell v1", 1);
        let g_v2: Vec<u8> = build_global_app_wasm("Global Dynamic WASM App Shell v2", 2);
        registry.register(
            URL_WASM_PARTIAL_V1.to_string(),
            p_v1.clone(),
            "etag-p-v1".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_PARTIAL_V2.to_string(),
            p_v2,
            "etag-p-v2".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_GLOBAL_V1.to_string(),
            g_v1,
            "etag-g-v1".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_GLOBAL_V2.to_string(),
            g_v2,
            "etag-g-v2".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_TIMEOUT_RETRY.to_string(),
            p_v1.clone(),
            "etag-retry".to_string(),
            WasmFaultKind::TimeoutOnceThenSucceed,
        );
        registry.register(
            URL_WASM_HTTP_500.to_string(),
            p_v1.clone(),
            "etag-500".to_string(),
            WasmFaultKind::Http500,
        );
        registry.register(
            URL_WASM_CORRUPTED.to_string(),
            build_faulty_wasm(WasmFaultKind::CorruptedMagic),
            "etag-corrupt".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_TAMPERED.to_string(),
            p_v1,
            "etag-tamper".to_string(),
            WasmFaultKind::TamperedChecksum,
        );
        registry.register(
            URL_WASM_BAD_ABI.to_string(),
            build_faulty_wasm(WasmFaultKind::AbiMismatch),
            "etag-abi".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_MISSING_EXPORT.to_string(),
            build_faulty_wasm(WasmFaultKind::MissingExport),
            "etag-noexp".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_FUEL_BOMB.to_string(),
            build_faulty_wasm(WasmFaultKind::InfiniteLoopOutOfFuel),
            "etag-fuel".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_TRAP.to_string(),
            build_faulty_wasm(WasmFaultKind::ExecutionTrap),
            "etag-trap".to_string(),
            WasmFaultKind::None,
        );
        registry.register(
            URL_WASM_OVERSIZED.to_string(),
            build_faulty_wasm(WasmFaultKind::OversizedBinary),
            "etag-big".to_string(),
            WasmFaultKind::None,
        );
    }

    /// Formats a clean Linear/Vercel-style top bar breadcrumb for `route`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Active route path.
    ///
    /// # Returns
    ///
    /// - `String` - Formatted breadcrumb path (`"euv Studio  /  Category  /  Page"`).
    #[must_use]
    pub fn breadcrumb_for_route(route: &str) -> String {
        let (sec, title): (&str, &str) = match route {
            ROUTE_OVERVIEW => (NAV_SEC_WORKSPACE, NAV_LABEL_OVERVIEW),
            ROUTE_LAB => (NAV_SEC_WORKSPACE, NAV_LABEL_LAB),
            ROUTE_MARKDOWN => (NAV_SEC_DESIGN, NAV_LABEL_MARKDOWN),
            ROUTE_FORMS => (NAV_SEC_DESIGN, NAV_LABEL_FORMS),
            ROUTE_THEMES => (NAV_SEC_DESIGN, NAV_LABEL_THEMES),
            ROUTE_OVERLAYS => (NAV_SEC_DESIGN, NAV_LABEL_OVERLAYS),
            ROUTE_DATA => (NAV_SEC_DESIGN, NAV_LABEL_DATA),
            ROUTE_GAME_3D => (NAV_SEC_ENGINE, NAV_LABEL_GAME_3D),
            ROUTE_GAME => (NAV_SEC_ENGINE, NAV_LABEL_GAME),
            ROUTE_MEDIA => (NAV_SEC_ENGINE, NAV_LABEL_MEDIA),
            ROUTE_RENDER => (NAV_SEC_ENGINE, NAV_LABEL_RENDER),
            ROUTE_DYNAMIC => (NAV_SEC_RUNTIME, NAV_LABEL_DYNAMIC),
            ROUTE_STRESS => (NAV_SEC_RUNTIME, NAV_LABEL_STRESS),
            ROUTE_WINDOW => (NAV_SEC_RUNTIME, NAV_LABEL_WINDOW),
            ROUTE_REACTIVE => (NAV_SEC_RUNTIME, NAV_LABEL_REACTIVE),
            other => (NAV_SEC_RUNTIME, other),
        };
        format!("{sec} / {title}")
    }

    /// Binds `hub` from the active `App` and wires interactive dynamic actions and WASM manager callbacks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable showcase state.
    /// - `DynamicUpdateHub` - Shared dynamic update hub owned by `App`.
    pub fn attach_dynamic_hub(&mut self, hub: DynamicUpdateHub) {
        let route_sig: Signal<String> = self.get_route();
        let action_log: Signal<String> = self.get_dynamic_action_log();
        let wasm_status: Signal<String> = self.get_wasm_fault_status();
        let wasm_mgr: WasmDynamicManager = WasmDynamicManager::new(
            hub.clone(),
            Rc::new(self.get_wasm_registry().clone()),
            WasmSandboxConfig::default(),
        );
        let mgr_act: WasmDynamicManager = wasm_mgr.clone();
        hub.on_action(move |ev: &DynamicActionEvent| {
            let summary: String = format!(
                "{} from {} ({})",
                ev.get_action(),
                ev.get_source_id(),
                ev.get_payload()
            );
            let _: bool = action_log.set_if_changed(summary);
            if let Some(target_route) = ev.get_action().strip_prefix("navigate:") {
                let _: bool = route_sig.set_if_changed(target_route.to_string());
            } else if ev.get_action().as_str() == "return_dynamic" {
                let _: bool = route_sig.set_if_changed(ROUTE_DYNAMIC.to_string());
            } else if ev.get_action().as_str() == "wasm_interact" {
                let scope: WasmDynamicScope = WasmDynamicScope::Partial {
                    slot_id: SLOT_WASM_WIDGET.to_string(),
                };
                match mgr_act.dispatch_wasm_event(&scope, 1) {
                    Ok(next_st) => {
                        let _: bool = wasm_status.set(format!(
                            "WASM euv_on_event executed inside sandbox -> next_state={next_st} (native UI re-rendered)"
                        ));
                    }
                    Err(err) => {
                        let _: bool = wasm_status.set(format!("WASM event error: {err}"));
                    }
                }
            } else if ev.get_action().as_str() == "global_wasm_action" {
                match mgr_act.dispatch_wasm_event(&WasmDynamicScope::Global, 1) {
                    Ok(next_st) => {
                        let _: bool = wasm_status.set(format!(
                            "Global WASM euv_on_event executed live -> state={next_st} (full shell re-rendered)"
                        ));
                    }
                    Err(err) => {
                        let _: bool = wasm_status.set(format!("Global WASM event error: {err}"));
                    }
                }
            } else if ev.get_action().as_str() == "global_wasm_rollback" {
                match mgr_act.rollback_global("user rollback") {
                    Ok(ver) => {
                        let _: bool = wasm_status.set(format!(
                            "Rolled back Global WASM shell to v{ver} (0 = native shell)"
                        ));
                    }
                    Err(_) => {
                        mgr_act.restore_native_global();
                        let _: bool = wasm_status.set(STATUS_RESTORED_NATIVE_SHELL.to_string());
                    }
                }
            } else if ev.get_action().as_str() == "global_wasm_restore" {
                mgr_act.restore_native_global();
                let _: bool = wasm_status.set(STATUS_RESTORED_NATIVE_SHELL.to_string());
            }
        });
        let Self {
            dynamic_hub,
            wasm_manager,
            router,
            ..
        } = self;
        let _: EuvRouter = router.clone().with_dynamic_hub(hub.clone());
        *dynamic_hub = hub;
        *wasm_manager = wasm_mgr;
    }

    /// Builds `AiAppHooks` bridging route, theme, and 3D/2D game engine telemetry and controls to AI Schema v1.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive showcase state reference.
    ///
    /// # Returns
    ///
    /// - `AiAppHooks` - Configured AI automation hooks bound to this state.
    #[cfg(any(debug_assertions, test))]
    #[must_use]
    pub fn build_ai_hooks(&self) -> AiAppHooks {
        let route_get: Signal<String> = self.get_route();
        let route_set: Signal<String> = self.get_route();
        let theme_get: UiThemeState = self.get_theme().clone();
        let theme_set: UiThemeState = self.get_theme().clone();
        let preset_get: Signal<usize> = self.get_engine3d_preset_idx();
        let preset_set: Signal<usize> = self.get_engine3d_preset_idx();
        let rt_get: Signal<usize> = self.get_engine3d_rt_idx();
        let rt_set: Signal<usize> = self.get_engine3d_rt_idx();
        let res_get: Signal<f32> = self.get_engine3d_res_scale();
        let res_set: Signal<f32> = self.get_engine3d_res_scale();
        let time_get: Signal<f32> = self.get_engine3d_time_of_day();
        let time_set: Signal<f32> = self.get_engine3d_time_of_day();
        let cam_get: Signal<u8> = self.get_engine3d_camera_mode();
        let cam_set: Signal<u8> = self.get_engine3d_camera_mode();
        let import_set: Signal<u64> = self.get_engine3d_import_queue();
        let status_get: Signal<String> = self.get_engine3d_import_status();
        AiAppHooks::new()
            .on_route(move || route_get.get_untracked())
            .on_scheme(move || theme_get.get_scheme().get_untracked().as_str().to_string())
            .on_game_stats(move || {
                let raw: String = format!(
                    "{{\"preset_idx\":{},\"rt_idx\":{},\"resolution_scale_pct\":{:.1},\"time_of_day\":{:.2},\"camera_mode\":{},\"import_status\":{:?}}}",
                    preset_get.get_untracked(),
                    rt_get.get_untracked(),
                    res_get.get_untracked(),
                    time_get.get_untracked(),
                    cam_get.get_untracked(),
                    status_get.get_untracked()
                );
                AiSchemaProtocol::parse_json_value(&raw)
            })
            .on_navigate(move |target: &str| {
                let _: bool = route_set.set_if_changed(target.to_string());
                Ok(())
            })
            .on_set_theme(move |scheme_opt: Option<&str>, mode_opt: Option<&str>| {
                if let Some(s_name) = scheme_opt {
                    let matched: Option<DesignScheme> = DesignScheme::all()
                        .into_iter()
                        .find(|ds: &DesignScheme| ds.as_str().eq_ignore_ascii_case(s_name));
                    match matched {
                        Some(ds) => theme_set.select_scheme(ds),
                        None => return Err(format!("Unknown design scheme: {s_name}")),
                    }
                }
                if let Some(m_name) = mode_opt {
                    let next_mode: ThemeMode = if m_name.eq_ignore_ascii_case("Dark") {
                        ThemeMode::Dark
                    } else if m_name.eq_ignore_ascii_case("Light") {
                        ThemeMode::Light
                    } else {
                        return Err(format!("Unknown theme mode: {m_name}"));
                    };
                    let _: bool = theme_set.get_mode().set_if_changed(next_mode);
                }
                Ok(())
            })
            .on_game_control(move |cmd: &str, payload: &AiJsonValue| {
                let num_val: f64 = payload
                    .as_f64()
                    .or_else(|| {
                        payload
                            .get("value")
                            .and_then(|v: &AiJsonValue| v.as_f64())
                    })
                    .unwrap_or(0.0);
                match cmd {
                    "camera_mode" => {
                        let next_cam: u8 = (num_val.round() as u8).min(3);
                        let _: bool = cam_set.set_if_changed(next_cam);
                        Ok(AiSchemaProtocol::parse_json_value(&format!(
                            "{{\"camera_mode\":{next_cam}}}"
                        )))
                    }
                    "preset" => {
                        let next_p: usize = (num_val.round() as usize).min(4);
                        let _: bool = preset_set.set_if_changed(next_p);
                        Ok(AiSchemaProtocol::parse_json_value(&format!(
                            "{{\"preset_idx\":{next_p}}}"
                        )))
                    }
                    "ray_tracing" => {
                        let next_rt: usize = (num_val.round() as usize).min(3);
                        let _: bool = rt_set.set_if_changed(next_rt);
                        Ok(AiSchemaProtocol::parse_json_value(&format!(
                            "{{\"rt_idx\":{next_rt}}}"
                        )))
                    }
                    "resolution_scale" => {
                        let next_s: f32 = (num_val as f32).clamp(40.0, 125.0);
                        let _: bool = res_set.set_if_changed(next_s);
                        Ok(AiSchemaProtocol::parse_json_value(&format!(
                            "{{\"resolution_scale_pct\":{next_s:.1}}}"
                        )))
                    }
                    "time_of_day" => {
                        let next_t: f32 = (num_val as f32).clamp(6.0, 23.0);
                        let _: bool = time_set.set_if_changed(next_t);
                        Ok(AiSchemaProtocol::parse_json_value(&format!(
                            "{{\"time_of_day\":{next_t:.2}}}"
                        )))
                    }
                    "import_model" => {
                        let next_q: u64 = import_set.get_untracked().saturating_add(1);
                        let _: bool = import_set.set_if_changed(next_q);
                        Ok(AiSchemaProtocol::parse_json_value(&format!(
                            "{{\"import_queue\":{next_q}}}"
                        )))
                    }
                    other => Err(format!("Unknown game control command: {other}")),
                }
            })
    }

    /// Returns the categorized sidebar navigation items grouped in Linear/Vercel/shadcn hierarchy.
    ///
    /// # Returns
    ///
    /// - `Vec<EuvNavItem>` - Categorized list of showcase routes.
    #[must_use]
    pub fn nav_items() -> Vec<EuvNavItem> {
        vec![
            EuvNavItem::new(ROUTE_OVERVIEW.to_string(), NAV_LABEL_OVERVIEW.to_string())
                .section(NAV_SEC_WORKSPACE),
            EuvNavItem::new(ROUTE_LAB.to_string(), NAV_LABEL_LAB.to_string())
                .section(NAV_SEC_WORKSPACE),
            EuvNavItem::new(ROUTE_MARKDOWN.to_string(), NAV_LABEL_MARKDOWN.to_string())
                .section(NAV_SEC_DESIGN),
            EuvNavItem::new(ROUTE_FORMS.to_string(), NAV_LABEL_FORMS.to_string())
                .section(NAV_SEC_DESIGN),
            EuvNavItem::new(ROUTE_THEMES.to_string(), NAV_LABEL_THEMES.to_string())
                .section(NAV_SEC_DESIGN),
            EuvNavItem::new(ROUTE_OVERLAYS.to_string(), NAV_LABEL_OVERLAYS.to_string())
                .section(NAV_SEC_DESIGN),
            EuvNavItem::new(ROUTE_DATA.to_string(), NAV_LABEL_DATA.to_string())
                .section(NAV_SEC_DESIGN),
            EuvNavItem::new(ROUTE_GAME_3D.to_string(), NAV_LABEL_GAME_3D.to_string())
                .section(NAV_SEC_ENGINE),
            EuvNavItem::new(ROUTE_GAME.to_string(), NAV_LABEL_GAME.to_string())
                .section(NAV_SEC_ENGINE),
            EuvNavItem::new(ROUTE_MEDIA.to_string(), NAV_LABEL_MEDIA.to_string())
                .section(NAV_SEC_ENGINE),
            EuvNavItem::new(ROUTE_RENDER.to_string(), NAV_LABEL_RENDER.to_string())
                .section(NAV_SEC_ENGINE),
            EuvNavItem::new(ROUTE_DYNAMIC.to_string(), NAV_LABEL_DYNAMIC.to_string())
                .section(NAV_SEC_RUNTIME),
            EuvNavItem::new(ROUTE_STRESS.to_string(), NAV_LABEL_STRESS.to_string())
                .section(NAV_SEC_RUNTIME),
            EuvNavItem::new(ROUTE_WINDOW.to_string(), NAV_LABEL_WINDOW.to_string())
                .section(NAV_SEC_RUNTIME),
            EuvNavItem::new(ROUTE_REACTIVE.to_string(), NAV_LABEL_REACTIVE.to_string())
                .section(NAV_SEC_RUNTIME),
        ]
    }

    /// Resolves the currently selected `RenderMode` from `render_mode_index`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    ///
    /// # Returns
    ///
    /// - `RenderMode` - Active render pipeline mode.
    #[must_use]
    pub fn selected_render_mode(&self) -> RenderMode {
        match self.get_render_mode_index().get_untracked() {
            1 => RenderMode::Software,
            2 => RenderMode::Hardware,
            3 => RenderMode::Hybrid,
            _ => RenderMode::Auto,
        }
    }

    /// Cycles to the next `DesignScheme` in `DesignScheme::all()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    pub fn next_scheme(&self) {
        let all: [DesignScheme; 17] = DesignScheme::all();
        let current: DesignScheme = self.get_theme().get_scheme().get_untracked();
        let pos: usize = all
            .iter()
            .position(|s: &DesignScheme| *s == current)
            .unwrap_or(0);
        let next: DesignScheme = all[(pos + 1) % all.len()];
        let _: bool = self.get_theme().get_scheme().set_if_changed(next);
    }

    /// Toggles between light and dark theme modes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    pub fn toggle_theme_mode(&self) {
        let cur: ThemeMode = self.get_theme().get_mode().get_untracked();
        let next: ThemeMode = match cur {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        let _: bool = self.get_theme().get_mode().set_if_changed(next);
    }
}

/// Default `ShowcaseState` delegating to `ShowcaseState::new`.
impl Default for ShowcaseState {
    /// Creates a default `ShowcaseState`.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized state.
    fn default() -> Self {
        Self::new()
    }
}

/// Arena setup and gameplay helper methods for `DemoPhysicsScene`.
impl DemoPhysicsScene {
    /// Builds a playable 1280x720 tiled neon arena with player ship, obstacles, and initial enemy wave.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized playable arena scene.
    #[must_use]
    pub fn create_arena() -> Self {
        let mut tilemap: TileMap = TileMap::empty(24, 14, 48.0);
        let floor_a: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
            Color::hex(0x0B1120),
            Color::rgba(30, 41, 59, 90),
            false,
        ));
        let floor_b: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
            Color::hex(0x0F172A),
            Color::rgba(30, 41, 59, 90),
            false,
        ));
        let wall_tile: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
            Color::hex(0x1E293B),
            Color::hex(0x38BDF8),
            true,
        ));
        for row in 0..tilemap.get_rows() {
            for col in 0..tilemap.get_columns() {
                let is_border: bool = row == 0
                    || col == 0
                    || row + 1 == tilemap.get_rows()
                    || col + 1 == tilemap.get_columns();
                let tid: u16 = if is_border {
                    wall_tile
                } else if (row + col) % 2 == 0 {
                    floor_a
                } else {
                    floor_b
                };
                tilemap.set_tile(col, row, tid);
            }
        }
        let world_rect: Rect = tilemap.world_bounds();
        let inner_arena: Rect = Rect::new(
            56.0,
            56.0,
            (world_rect.get_width() - 112.0).max(200.0),
            (world_rect.get_height() - 112.0).max(200.0),
        );
        let mut world: GameWorld = GameWorld::with_capacity(256);
        let center: Point = Point::new(world_rect.get_width() * 0.5, world_rect.get_height() * 0.5);
        let player_id: EntityId = world.spawn(EntityTag::Player, center);
        if let Some(player) = world.get_entity_mut(player_id) {
            player.set_body(Some(RigidBody2D::dynamic(1.0, 0.4, 3.2)));
            player.set_collider(Some(Collider2D::circle(
                14.0,
                COLLISION_LAYER_PLAYER,
                COLLISION_MASK_PLAYER_DEFAULT,
                false,
            )));
            player.set_sprite(Some(Sprite2D::new(
                EntityVisualShape::Triangle,
                Size::new(30.0, 26.0),
                Color::hex(0x38BDF8),
                Color::hex(0xE0F2FE),
                10,
                0.0,
                true,
            )));
        }
        let pillars: [(f32, f32); 4] = [
            (280.0, 200.0),
            (860.0, 200.0),
            (280.0, 470.0),
            (860.0, 470.0),
        ];
        for (px, py) in pillars.into_iter() {
            let pid: EntityId = world.spawn(EntityTag::Obstacle, Point::new(px, py));
            if let Some(obs) = world.get_entity_mut(pid) {
                obs.set_body(Some(RigidBody2D::static_body(0.85)));
                obs.set_collider(Some(Collider2D::aabb(
                    52.0,
                    52.0,
                    COLLISION_LAYER_WORLD,
                    COLLISION_LAYER_ALL,
                    false,
                )));
                obs.set_sprite(Some(Sprite2D::new(
                    EntityVisualShape::Rect,
                    Size::new(52.0, 52.0),
                    Color::hex(0x1E293B),
                    Color::hex(0x6366F1),
                    2,
                    8.0,
                    true,
                )));
            }
        }
        let mut phys_cfg: PhysicsConfig = PhysicsConfig::default();
        phys_cfg.set_arena_bounds(Some(inner_arena));
        let mut camera: Camera2D = Camera2D::at(center, 1.0);
        camera.set_world_bounds(Some(world_rect));
        let mut scene: Self = Self {
            world,
            physics: PhysicsWorld2D::with_config(phys_cfg),
            camera,
            particles: ParticleSystem::with_capacity(1024),
            tilemap,
            actions: ActionMap::default(),
            audio: AudioBus::default(),
            player_id,
            fire_cooldown: 0.0,
            dash_cooldown: 0.0,
            score: 0,
            wave: 1,
            show_colliders: false,
            smoothed_fps: 60.0,
            ticks: 0,
        };
        scene.spawn_enemy_wave(8);
        scene
    }

    /// Spawns `count` patrolling enemy drones around the arena perimeter.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `usize` - Number of enemy drones to spawn.
    pub fn spawn_enemy_wave(&mut self, count: usize) {
        let bounds: Rect = self.get_tilemap().world_bounds();
        let cx: f32 = bounds.get_width() * 0.5;
        let cy: f32 = bounds.get_height() * 0.5;
        for idx in 0..count {
            let angle: f32 = ((idx as f32) / (count.max(1) as f32)) * 6.283_185_5
                + (self.get_wave() as f32) * 0.45;
            let radius: f32 = 220.0 + ((idx % 3) as f32) * 48.0;
            let sx: f32 = (cx + angle.cos() * radius).clamp(110.0, bounds.get_width() - 110.0);
            let sy: f32 = (cy + angle.sin() * radius).clamp(110.0, bounds.get_height() - 110.0);
            let speed: f32 = 95.0 + ((idx % 4) as f32) * 28.0;
            let eid: EntityId = self
                .get_mut_world()
                .spawn(EntityTag::Enemy, Point::new(sx, sy));
            if let Some(enemy) = self.get_mut_world().get_entity_mut(eid) {
                enemy.set_velocity(Velocity2D::new(
                    -angle.sin() * speed,
                    angle.cos() * speed,
                    1.5,
                ));
                enemy.set_body(Some(RigidBody2D::dynamic(1.2, 0.95, 0.05)));
                enemy.set_collider(Some(Collider2D::circle(
                    13.0,
                    COLLISION_LAYER_ENEMY,
                    COLLISION_LAYER_ALL,
                    false,
                )));
                enemy.set_sprite(Some(Sprite2D::new(
                    if idx % 2 == 0 {
                        EntityVisualShape::Diamond
                    } else {
                        EntityVisualShape::Circle
                    },
                    Size::new(26.0, 26.0),
                    if idx % 2 == 0 {
                        Color::hex(0xF43F5E)
                    } else {
                        Color::hex(0xF59E0B)
                    },
                    Color::hex(0xFEF08A),
                    6,
                    0.0,
                    true,
                )));
            }
        }
    }

    /// Fires a fast plasma projectile from the player ship along `aim_angle`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `Point` - Launch origin in world coordinates.
    /// - `f32` - Launch angle in radians.
    fn fire_projectile(&mut self, origin: Point, aim_angle: f32) {
        let dir_x: f32 = aim_angle.cos();
        let dir_y: f32 = aim_angle.sin();
        let launch: Point =
            Point::new(origin.get_x() + dir_x * 20.0, origin.get_y() + dir_y * 20.0);
        let pid: EntityId = self.get_mut_world().spawn(EntityTag::Projectile, launch);
        if let Some(proj) = self.get_mut_world().get_entity_mut(pid) {
            proj.get_mut_transform().set_rotation_rad(aim_angle);
            proj.set_velocity(Velocity2D::new(dir_x * 520.0, dir_y * 520.0, 0.0));
            proj.set_body(Some(RigidBody2D::dynamic(0.2, 0.9, 0.0)));
            proj.set_collider(Some(Collider2D::circle(
                5.0,
                COLLISION_LAYER_PROJECTILE,
                COLLISION_LAYER_ENEMY | COLLISION_LAYER_WORLD,
                true,
            )));
            proj.set_sprite(Some(Sprite2D::new(
                EntityVisualShape::Circle,
                Size::new(10.0, 10.0),
                Color::hex(0x34D399),
                Color::hex(0xFFFFFF),
                8,
                0.0,
                true,
            )));
            proj.set_ttl_secs(Some(1.4));
        }
        let mut muzzle: ParticleEmitterConfig = ParticleEmitterConfig::default();
        muzzle
            .set_base_angle(aim_angle)
            .set_spread_rad(0.45)
            .set_min_speed(80.0)
            .set_max_speed(180.0)
            .set_min_life(0.12)
            .set_max_life(0.26)
            .set_start_size(3.5)
            .set_start_color(Color::hex(0x34D399))
            .set_end_color(Color::rgba(56, 189, 248, 0));
        self.get_mut_particles().emit_burst(launch, 6, muzzle);
        self.get_mut_audio().play(SoundCue::Laser, 0.65, 0.0, 1.05);
    }
}

/// Deterministic fixed-timestep 2D game engine simulation and camera-driven HUD rendering for `DemoPhysicsScene`.
impl GameScene for DemoPhysicsScene {
    /// Steps player controls, ECS physics, collision combat, particles, and camera follow at 60Hz fixed cadence.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `f32` - Fixed simulation delta time in seconds.
    /// - `&InputState` - Real-time polled keyboard and pointer state.
    fn fixed_update(&mut self, dt: f32, input: &InputState) {
        let move_vec: Point = self.get_actions().movement_vector(input);
        let fire_pressed: bool = self.get_actions().is_active(GameAction::Fire, input);
        let dash_pressed: bool = self.get_actions().is_just_pressed(GameAction::Dash, input);
        let wave_pressed: bool = self
            .get_actions()
            .is_just_pressed(GameAction::ActionAlt, input);
        if wave_pressed {
            let next_wave: u32 = self.get_wave().saturating_add(1);
            self.set_wave(next_wave);
            self.spawn_enemy_wave(6);
        }
        let fire_cd: f32 = (self.get_fire_cooldown() - dt).max(0.0);
        let dash_cd: f32 = (self.get_dash_cooldown() - dt).max(0.0);
        self.set_fire_cooldown(fire_cd);
        self.set_dash_cooldown(dash_cd);
        let player_id: EntityId = self.get_player_id();
        let mut player_pos: Point = Point::new(576.0, 336.0);
        let mut aim_angle: f32 = 0.0;
        let mut dashed: bool = false;
        if let Some(player) = self.get_mut_world().get_entity_mut(player_id) {
            player_pos = player.get_transform().get_position();
            let accel: f32 = 1650.0;
            let mut vel: Velocity2D = player.get_velocity();
            vel.set_vx(vel.get_vx() + move_vec.get_x() * accel * dt);
            vel.set_vy(vel.get_vy() + move_vec.get_y() * accel * dt);
            let mut rot: f32 = player.get_transform().get_rotation_rad();
            if move_vec.get_x().abs() > 0.05 || move_vec.get_y().abs() > 0.05 {
                rot = move_vec.get_y().atan2(move_vec.get_x());
            } else {
                rot += dt * 0.8;
            }
            if dash_pressed && dash_cd <= 0.0 {
                vel.set_vx(vel.get_vx() + rot.cos() * 440.0);
                vel.set_vy(vel.get_vy() + rot.sin() * 440.0);
                dashed = true;
            }
            player.set_velocity(vel);
            player.get_mut_transform().set_rotation_rad(rot);
            aim_angle = rot;
        }
        if dashed {
            self.set_dash_cooldown(0.45);
            self.get_mut_camera().add_trauma(0.25);
            let mut burst: ParticleEmitterConfig = ParticleEmitterConfig::default();
            burst
                .set_base_angle(aim_angle + PI)
                .set_spread_rad(0.9)
                .set_start_color(Color::hex(0x38BDF8))
                .set_end_color(Color::rgba(56, 189, 248, 0));
            self.get_mut_particles().emit_burst(player_pos, 18, burst);
            self.get_mut_audio().play(SoundCue::Dash, 0.8, 0.0, 1.0);
        }
        if (fire_pressed || self.get_ticks().is_multiple_of(22)) && fire_cd <= 0.0 {
            self.fire_projectile(player_pos, aim_angle);
            self.set_fire_cooldown(0.14);
        }
        let collisions: Vec<CollisionEvent> = {
            let Self { world, physics, .. } = self;
            physics.step(world, dt)
        };
        let mut to_despawn: Vec<(EntityId, Point, bool)> = Vec::new();
        for ev in collisions.into_iter() {
            let tag_a: Option<EntityTag> = self
                .get_world()
                .get_entity(ev.get_entity_a())
                .map(EntityRecord::get_tag);
            let tag_b: Option<EntityTag> = self
                .get_world()
                .get_entity(ev.get_entity_b())
                .map(EntityRecord::get_tag);
            match (tag_a, tag_b) {
                (Some(EntityTag::Projectile), Some(EntityTag::Enemy)) => {
                    if let Some(e) = self.get_world().get_entity(ev.get_entity_b()) {
                        to_despawn.push((
                            ev.get_entity_b(),
                            e.get_transform().get_position(),
                            true,
                        ));
                    }
                    to_despawn.push((ev.get_entity_a(), Point::default(), false));
                }
                (Some(EntityTag::Enemy), Some(EntityTag::Projectile)) => {
                    if let Some(e) = self.get_world().get_entity(ev.get_entity_a()) {
                        to_despawn.push((
                            ev.get_entity_a(),
                            e.get_transform().get_position(),
                            true,
                        ));
                    }
                    to_despawn.push((ev.get_entity_b(), Point::default(), false));
                }
                (Some(EntityTag::Projectile), Some(EntityTag::Obstacle)) => {
                    to_despawn.push((ev.get_entity_a(), Point::default(), false));
                }
                (Some(EntityTag::Obstacle), Some(EntityTag::Projectile)) => {
                    to_despawn.push((ev.get_entity_b(), Point::default(), false));
                }
                (Some(EntityTag::Player), Some(EntityTag::Enemy))
                | (Some(EntityTag::Enemy), Some(EntityTag::Player)) => {
                    self.get_mut_camera().add_trauma(0.35);
                    self.get_mut_audio().play(SoundCue::Bounce, 0.7, 0.0, 0.85);
                }
                _ => {}
            }
        }
        for (eid, pos, is_enemy) in to_despawn.into_iter() {
            if self.get_mut_world().despawn(eid) && is_enemy {
                let next_score: u32 = self.get_score().saturating_add(100);
                self.set_score(next_score);
                self.get_mut_camera().add_trauma(0.32);
                let mut exp: ParticleEmitterConfig = ParticleEmitterConfig::default();
                exp.set_min_speed(70.0)
                    .set_max_speed(260.0)
                    .set_start_size(5.5)
                    .set_start_color(Color::hex(0xFB7185))
                    .set_end_color(Color::rgba(250, 204, 21, 0));
                self.get_mut_particles().emit_burst(pos, 22, exp);
                self.get_mut_audio()
                    .play(SoundCue::Explosion, 0.9, 0.0, 0.95);
            }
        }
        if self.get_world().count_by_tag(EntityTag::Enemy) < 3 {
            let next_wave: u32 = self.get_wave().saturating_add(1);
            self.set_wave(next_wave);
            self.spawn_enemy_wave(6);
        }
        let _: usize = self.get_mut_world().tick_lifetimes(dt);
        let _: usize = self.get_mut_particles().update(dt);
        let _: Vec<AudioCommand> = self.get_mut_audio().drain();
        if let Some(player) = self.get_world().get_entity(player_id) {
            let p: Point = player.get_transform().get_position();
            self.get_mut_camera()
                .follow(p, 7.5, dt, Size::new(820.0, 320.0));
        }
        self.set_ticks(self.get_ticks().saturating_add(1));
    }

    /// Updates smoothed frame-rate telemetry once per visual frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `f32` - Real frame delta time in seconds.
    /// - `&InputState` - Current polled input snapshot.
    fn update(&mut self, dt: f32, _input: &InputState) {
        if dt > 0.0001 {
            let inst_fps: f32 = (1.0 / dt).clamp(1.0, 9999.0);
            let next: f32 = self.get_smoothed_fps() * 0.85 + inst_fps * 0.15;
            self.set_smoothed_fps(next);
        }
    }

    /// Renders the camera-projected tilemap, ECS entities, raycast targeting laser, particles, and engine HUD.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `f32` - Interpolation alpha in `[0.0, 1.0]`.
    /// - `&mut GameRenderContext<'_>` - Immediate-mode game draw surface.
    fn render(&mut self, _alpha: f32, ctx: &mut GameRenderContext<'_>) {
        let size: Size = ctx.viewport_size();
        ctx.clear(Color::hex(0x020617));
        let cam: Camera2D = self.get_camera();
        ctx.render_tilemap(self.get_tilemap(), &cam);
        if let Some(player) = self.get_world().get_entity(self.get_player_id()) {
            let origin: Point = player.get_transform().get_position();
            let rot: f32 = player.get_transform().get_rotation_rad();
            let dir: Point = Point::new(rot.cos(), rot.sin());
            let end_world: Point = match PhysicsWorld2D::raycast(
                self.get_world(),
                origin,
                dir,
                340.0,
                COLLISION_LAYER_ENEMY | COLLISION_LAYER_WORLD,
            ) {
                Some(hit) => hit.get_point(),
                None => Point::new(
                    origin.get_x() + dir.get_x() * 340.0,
                    origin.get_y() + dir.get_y() * 340.0,
                ),
            };
            let s0: Point = cam.world_to_screen(origin, size);
            let s1: Point = cam.world_to_screen(end_world, size);
            ctx.draw_line(s0, s1, 1.5, Color::rgba(56, 189, 248, 120));
            ctx.fill_circle(s1, 3.5, Color::hex(0x38BDF8));
        }
        let drawn_entities: usize = ctx.render_world(self.get_world(), &cam);
        let drawn_particles: usize = ctx.render_particles(self.get_particles(), &cam);
        if self.get_show_colliders() {
            ctx.draw_debug_colliders(self.get_world(), &cam);
        }
        ctx.fill_rounded_rect(
            Rect::new(
                10.0,
                10.0,
                (size.get_width() - 20.0).clamp(180.0, 560.0),
                28.0,
            ),
            Color::rgba(15, 23, 42, 210),
            6.0,
        );
        ctx.draw_text(
            Point::new(18.0, 16.0),
            format!(
                "FPS {:.0} | Wave {} | Score {} | Ent {} | Part {} | SFX {}",
                self.get_smoothed_fps(),
                self.get_wave(),
                self.get_score(),
                drawn_entities,
                drawn_particles,
                self.get_audio().get_total_played()
            ),
            12.0,
            Color::hex(0xE2E8F0),
        );
        ctx.draw_text(
            Point::new(14.0, (size.get_height() - 22.0).max(40.0)),
            GAME_HUD_CONTROLS_HINT.to_string(),
            11.0,
            Color::rgba(148, 163, 184, 220),
        );
    }
}

/// Construction and runtime-config helpers for `DemoOpenWorld3DScene`.
impl DemoOpenWorld3DScene {
    /// Creates the live Vice Boulevard 3D open-world scene bound to reactive runtime settings signals.
    ///
    /// # Arguments
    ///
    /// - `Signal<RenderQualityConfig>` - Live quality/RT/resolution configuration signal.
    /// - `Signal<f32>` - Live time-of-day sun hour signal.
    /// - `Signal<u64>` - Monotonic model import request signal.
    /// - `Signal<String>` - Live status feedback signal.
    /// - `Signal<u8>` - Active 3D camera mode signal (`0=Chase, 1=Hood, 2=Orbit, 3=Aerial`).
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized 3D open-world scene.
    #[must_use]
    pub fn create_city(
        quality_signal: Signal<RenderQualityConfig>,
        time_signal: Signal<f32>,
        import_signal: Signal<u64>,
        status_signal: Signal<String>,
        camera_mode_signal: Signal<u8>,
    ) -> Self {
        Self {
            world: World3D::create_open_world_city(),
            renderer: Renderer3D::default(),
            actions: ActionMap::default(),
            quality_signal,
            time_signal,
            import_signal,
            last_import_seq: 0,
            status_signal,
            camera_mode_signal,
            smoothed_fps: 60.0,
        }
    }
}

/// Deterministic 3D open-world simulation and PBR + BVH ray-traced rendering for `DemoOpenWorld3DScene`.
impl GameScene for DemoOpenWorld3DScene {
    /// Steps vehicle physics, camera follow, spinning imported models, time-of-day sun, and queued model imports.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable 3D scene.
    /// - `f32` - Fixed simulation timestep in seconds.
    /// - `&InputState` - Real-time polled keyboard/pointer state.
    fn fixed_update(&mut self, dt: f32, input: &InputState) {
        let req_seq: u64 = self.get_import_signal().get_untracked();
        if req_seq != self.get_last_import_seq() {
            self.set_last_import_seq(req_seq);
            let is_obj: bool = req_seq % 2 == 1;
            let z_pos: f32 = -4.0 - ((req_seq % 5) as f32) * 8.0;
            let x_pos: f32 = if is_obj { -5.5 } else { 5.5 };
            let import_res: Result<ImportedModel3D, String> = if is_obj {
                self.get_mut_world().import_model(
                    ModelFormat::Obj,
                    SAMPLE_OBJ_MONUMENT,
                    Transform3D::new(Vec3::new(x_pos, 0.0, z_pos), 0.0, Vec3::splat(1.0)),
                    PbrMaterial::pbr(Color::hex(0xF59E0B), 0.9, 0.14, 0.72),
                    0.85,
                )
            } else {
                self.get_mut_world().import_model(
                    ModelFormat::GltfJson,
                    SAMPLE_GLTF_TOWER,
                    Transform3D::new(Vec3::new(x_pos, 0.0, z_pos), 0.0, Vec3::splat(1.0)),
                    PbrMaterial::pbr(Color::hex(0x38BDF8), 0.85, 0.12, 0.80),
                    -0.70,
                )
            };
            if let Ok(info) = import_res {
                let msg: String = format!(
                    "Imported {} ({:?}): {} verts, {} tris (LOD0+LOD1 + BVH rebuilt)",
                    info.get_name(),
                    info.get_format(),
                    info.get_total_vertices(),
                    info.get_total_triangles()
                );
                let _: bool = self.get_status_signal().set_if_changed(msg);
            }
        }
        let hour: f32 = self.get_time_signal().get_untracked();
        if (hour - self.get_world().get_sun().get_time_of_day()).abs() > 0.05 {
            self.get_mut_world()
                .set_sun(DirectionalLight3D::from_time_of_day(hour));
        }
        let cam_mode: u8 = self.get_camera_mode_signal().get_untracked();
        if cam_mode != self.get_world().get_camera_mode() {
            self.get_mut_world().set_camera_mode(cam_mode);
        }
        let actions: ActionMap = self.get_actions().clone();
        self.get_mut_world().step_simulation(dt, input, &actions);
    }

    /// Updates smoothed FPS telemetry for the 3D HUD overlay.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable 3D scene.
    /// - `f32` - Real frame delta time in seconds.
    /// - `&InputState` - Current polled input snapshot.
    fn update(&mut self, dt: f32, _input: &InputState) {
        if dt > 0.0001 {
            let inst_fps: f32 = (1.0 / dt).clamp(1.0, 9999.0);
            let next: f32 = self.get_smoothed_fps() * 0.85 + inst_fps * 0.15;
            self.set_smoothed_fps(next);
        }
    }

    /// Renders the 3D open-world city with live PBR + BVH ray tracing and overlays live telemetry HUD.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable 3D scene.
    /// - `f32` - Interpolation alpha.
    /// - `&mut GameRenderContext<'_>` - Immediate-mode viewport draw context.
    fn render(&mut self, _alpha: f32, ctx: &mut GameRenderContext<'_>) {
        let size: Size = ctx.viewport_size();
        let quality: RenderQualityConfig = self.get_quality_signal().get_untracked();
        let dpi_boost: f32 = match quality.get_preset() {
            QualityPreset::Low => 1.05,
            QualityPreset::Medium => 1.25,
            QualityPreset::High => 1.38,
            QualityPreset::Ultra => 1.45,
            QualityPreset::Cinematic => 1.52,
        };
        let vw: u32 = ((size.get_width() * dpi_boost).round() as u32).clamp(280, 960);
        let vh: u32 = ((size.get_height() * dpi_boost).round() as u32).clamp(180, 720);
        let frame_img: ImageBuffer = {
            let Self {
                world, renderer, ..
            } = self;
            renderer.render_world(world, vw, vh, &quality)
        };
        ctx.draw_image(
            Rect::new(0.0, 0.0, size.get_width(), size.get_height()),
            frame_img,
        );
        let stats: RenderStats3D = self.get_renderer().get_last_stats();
        let fps: f32 = self.get_smoothed_fps();
        let compact_hud: bool = size.get_width() < 520.0;
        let hud_text: String = if compact_hud {
            format!(
                "{fps:.0} FPS | {:?} | {}x{} | {} rays",
                quality.get_preset(),
                stats.get_render_width(),
                stats.get_render_height(),
                stats.get_rays_traced()
            )
        } else {
            format!(
                "{fps:.0} FPS | 3D {:?} | RT {:?} | {}x{} | Tris {} | Rays {} | BVH {}",
                quality.get_preset(),
                quality.get_ray_tracing(),
                stats.get_render_width(),
                stats.get_render_height(),
                stats.get_triangles_drawn(),
                stats.get_rays_traced(),
                stats.get_bvh_nodes()
            )
        };
        ctx.fill_rounded_rect(
            Rect::new(
                10.0,
                10.0,
                (size.get_width() - 20.0).clamp(200.0, 620.0),
                28.0,
            ),
            Color::rgba(9, 13, 24, 210),
            6.0,
        );
        ctx.draw_text(Point::new(18.0, 16.0), hud_text, 11.0, Color::hex(0xE2E8F0));
    }
}
