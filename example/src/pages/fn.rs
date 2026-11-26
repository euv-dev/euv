use super::*;

style! {
    c_action_row {
        direction: row;
        width: Length::Percent(100.0);
        flex_wrap: FlexWrap::Wrap;
        align: AlignItems::Center;
        gap: SPACE_SM;
    }
    c_scheme_grid {
        columns: 4;
        width: Length::Percent(100.0);
        gap: SPACE_SM;
    }
}

/// Mounts the fully reactive showcase application shell rebuilding cleanly on route, theme, or viewport changes.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
///
/// # Returns
///
/// - `NodeId` - Mounted reactive root container handle.
pub fn mount_showcase_root(state: &ShowcaseState) -> NodeId {
    let host: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            height: Length::Percent(100.0)
        }
    };
    let state_c: ShowcaseState = state.clone();
    let mut ctx: BuildContext = current_build_context();
    ctx.bind_children(host, move || {
        let tokens: ThemeTokens = state_c.get_theme().get_tokens().get();
        let hub: DynamicUpdateHub = state_c.get_dynamic_hub().clone();
        let _: u64 = hub.routes_signal().get();
        let cmd_open: Signal<bool> = state_c.get_cmd_palette_open();
        let cmd_query: Signal<String> = state_c.get_cmd_palette_query();
        let cmd_items: Vec<String> = vec![
            "Go to Overview (Workspace)".to_string(),
            "Go to UI & Edge Lab (Workspace)".to_string(),
            "Go to 3D Engine (RT)".to_string(),
            "Go to Media Studio (Engine)".to_string(),
            "Go to Dynamic WASM (OTA)".to_string(),
            "Go to Window & OS (Runtime)".to_string(),
            "Go to Themes (17 Schemes)".to_string(),
            "Toggle Dark / Light Mode".to_string(),
        ];
        let cmd_sel: Signal<Option<usize>> = Signal::new(None);
        let cmd_route: Signal<String> = state_c.get_route();
        let cmd_state: ShowcaseState = state_c.clone();
        let cmd_list: NodeId = EuvCommand::new(cmd_items, cmd_sel, tokens)
            .query(cmd_query)
            .placeholder("Type a route or action...")
            .mount_current();
        let (cmd_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                if let Some(idx) = cmd_sel.get() {
                    match idx {
                        0 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_OVERVIEW.to_string());
                        }
                        1 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_LAB.to_string());
                        }
                        2 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_GAME_3D.to_string());
                        }
                        3 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_MEDIA.to_string());
                        }
                        4 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_DYNAMIC.to_string());
                        }
                        5 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_WINDOW.to_string());
                        }
                        6 => {
                            let _: bool = cmd_route.set_if_changed(ROUTE_THEMES.to_string());
                        }
                        _ => {
                            cmd_state.toggle_theme_mode();
                        }
                    }
                    let _: bool = cmd_sel.set_if_changed(None);
                    let _: bool = cmd_open.set_if_changed(false);
                }
            })
        });
        let cmd_modal: NodeId = EuvModal::new(
            cmd_open,
            "Command Palette".to_string(),
            cmd_list,
            tokens,
        )
        .mount_current();
        let rctx: BuildContext = current_build_context();
        rctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(cmd_modal, cmd_scope);
            },
            (),
        );
        let state_shell: ShowcaseState = state_c.clone();
        let hub_shell: DynamicUpdateHub = hub.clone();
        let mut global_slot_cfg: DynamicComponentSlot = DynamicComponentSlot::new(
            WASM_GLOBAL_ROOT_SLOT.to_string(),
            Rc::new(move || {
                let mut nav_items: Vec<EuvNavItem> = ShowcaseState::nav_items();
                for dyn_route in hub_shell.registered_routes().into_iter() {
                    if dyn_route == ROUTE_DYNAMIC_PANEL {
                        continue;
                    }
                    if !nav_items
                        .iter()
                        .any(|item: &EuvNavItem| item.get_path().as_str() == dyn_route.as_str())
                    {
                        let label: String = hub_shell
                            .page_title(&dyn_route)
                            .filter(|t: &String| !t.is_empty())
                            .unwrap_or_else(|| dyn_route.clone());
                        nav_items.push(EuvNavItem::new(dyn_route, label).section(NAV_SEC_RUNTIME));
                    }
                }
                let nav: NodeId =
                    EuvSidebarNav::new(nav_items, state_shell.get_route(), tokens).mount_current();
                let state_for_outlet: ShowcaseState = state_shell.clone();
                let router: EuvRouter = EuvRouter::new(&state_shell.get_route().get_untracked())
                    .with_dynamic_hub(hub_shell.clone());
                let route_sig: Signal<String> = state_shell.get_route();
                let breadcrumb_sig: Signal<String> =
                    Signal::new(ShowcaseState::breadcrumb_for_route(
                        &route_sig.get_untracked(),
                    ));
                let router_sync_in: EuvRouter = router.clone();
                let router_path_sig: Signal<String> = router.get_current_path();
                let mut sctx: BuildContext = current_build_context();
                let (sync_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                    let _: Effect = Effect::new(move || {
                        let target: String = route_sig.get();
                        let _: bool = breadcrumb_sig
                            .set_if_changed(ShowcaseState::breadcrumb_for_route(&target));
                        if router_sync_in.get_current_path().get_untracked() != target {
                            router_sync_in.push(&target);
                        }
                    });
                    Effect::new(move || {
                        let rp: String = router_path_sig.get();
                        if route_sig.get_untracked() != rp {
                            let _: bool = route_sig.set_if_changed(rp);
                        }
                    })
                });
                let page_node: NodeId = router
                    .outlet(move |matched: &EuvRouteMatch| {
                        mount_route_by_name(matched.get_path(), &state_for_outlet, tokens)
                    })
                    .mount(&mut sctx);
                sctx.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        tree.attach_scope(page_node, sync_scope);
                    },
                    (),
                );
                let st_dark_top: ShowcaseState = state_shell.clone();
                let st_scheme_top: ShowcaseState = state_shell.clone();
                let st_dark_mobile: ShowcaseState = state_shell.clone();
                let mode_label: String =
                    match state_shell.get_theme().get_mode().get_untracked() {
                        ThemeMode::Dark => "Light".to_string(),
                        ThemeMode::Light => "Dark".to_string(),
                    };
                let scheme_label: String =
                    format!("{:?}", state_shell.get_theme().get_scheme().get_untracked());
                let top_actions: NodeId = view! {
                    row {
                        gap: SPACE_SM
                        align: AlignItems::Center
                        { EuvButton::new("Search  Cmd+K".to_string(), tokens).outline().sm().on_click(move |_: &UiEvent| {
                            let _: bool = cmd_open.set_if_changed(true);
                        }) }
                        { EuvButton::new(scheme_label, tokens).secondary().sm().on_click(move |_: &UiEvent| {
                            st_scheme_top.next_scheme();
                        }) }
                        { EuvButton::new(mode_label.clone(), tokens).outline().sm().on_click(move |_: &UiEvent| {
                            st_dark_top.toggle_theme_mode();
                        }) }
                    }
                };
                let mobile_actions: NodeId = view! {
                    row {
                        gap: SPACE_XS
                        align: AlignItems::Center
                        { EuvButton::new("Search".to_string(), tokens).outline().sm().on_click(move |_: &UiEvent| {
                            let _: bool = cmd_open.set_if_changed(true);
                        }) }
                        { EuvButton::new(mode_label, tokens).ghost().sm().on_click(move |_: &UiEvent| {
                            st_dark_mobile.toggle_theme_mode();
                        }) }
                    }
                };
                EuvAppShell::new(SHOWCASE_BRAND.to_string(), nav, page_node, tokens)
                    .route(state_shell.get_route())
                    .breadcrumb(breadcrumb_sig)
                    .top_bar_actions(top_actions)
                    .mobile_header_actions(mobile_actions)
                    .mount_current()
            }),
        );
        global_slot_cfg.set_hub(Some(hub)).set_style(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0)),
        );
        let global_wasm_slot: NodeId = global_slot_cfg.mount_current();
        vec![global_wasm_slot, cmd_modal]
    });
    host
}

/// Mounts the showcase page corresponding to the active route in `state`.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active resolved design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted page container handle.
pub fn mount_active_route(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let route: String = state.get_route().get_untracked();
    mount_route_by_name(&route, state, tokens)
}

/// Dispatches a route string to its corresponding declarative showcase page builder.
///
/// # Arguments
///
/// - `&str` - Active route path.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active resolved design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted page container handle.
fn mount_route_by_name(route: &str, state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    match route {
        ROUTE_REACTIVE => mount_reactive_page(state, tokens),
        ROUTE_RENDER => mount_render_page(state, tokens),
        ROUTE_WINDOW => mount_window_page(state, tokens),
        ROUTE_FORMS => mount_forms_page(state, tokens),
        ROUTE_THEMES => mount_themes_page(state, tokens),
        ROUTE_DATA => mount_data_page(state, tokens),
        ROUTE_GAME => mount_game_page(state, tokens),
        ROUTE_GAME_3D => mount_game3d_page(state, tokens),
        ROUTE_MEDIA => mount_media_page(state, tokens),
        ROUTE_LAB => mount_lab_page(state, tokens),
        ROUTE_OVERLAYS => mount_overlays_page(state, tokens),
        ROUTE_DYNAMIC => mount_dynamic_page(state, tokens),
        _ => mount_overview_page(state, tokens),
    }
}

/// Mounts the overview dashboard page with KPI stat cards, quick navigation buttons, and the 7-platform matrix.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted overview page node.
pub fn mount_overview_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let route_sig: Signal<String> = state.get_route();
    let table: NodeId = EuvTable::new(
        vec![
            TABLE_COL_TARGET.to_string(),
            TABLE_COL_BACKEND.to_string(),
            TABLE_COL_GPU.to_string(),
        ],
        vec![
            vec![
                ROW_WEB.to_string(),
                ROW_WEB_API.to_string(),
                ROW_WEB_GPU.to_string(),
            ],
            vec![
                ROW_MACOS.to_string(),
                ROW_MACOS_API.to_string(),
                ROW_MACOS_GPU.to_string(),
            ],
            vec![
                ROW_IOS.to_string(),
                ROW_IOS_API.to_string(),
                ROW_MACOS_GPU.to_string(),
            ],
            vec![
                ROW_ANDROID.to_string(),
                ROW_ANDROID_API.to_string(),
                ROW_ANDROID_GPU.to_string(),
            ],
            vec![
                ROW_WINDOWS.to_string(),
                ROW_WINDOWS_API.to_string(),
                ROW_WINDOWS_GPU.to_string(),
            ],
            vec![
                ROW_LINUX.to_string(),
                ROW_LINUX_API.to_string(),
                ROW_ANDROID_GPU.to_string(),
            ],
        ],
        tokens,
    )
    .mount_current();
    let quick_row: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Try Reactivity".to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let _: bool = route_sig.set_if_changed(ROUTE_REACTIVE.to_string());
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Open Game Engine".to_string(), tokens)
                .secondary()
                .on_click(move |_: &UiEvent| {
                    let _: bool = route_sig.set_if_changed(ROUTE_GAME.to_string());
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Open OS Services".to_string(), tokens)
                .outline()
                .on_click(move |_: &UiEvent| {
                    let _: bool = route_sig.set_if_changed(ROUTE_WINDOW.to_string());
                })
                .mount_current(),
        )
        .mount_current();
    let card: NodeId = EuvCard::new(CARD_PLATFORMS_TITLE.to_string(), tokens)
        .subtitle(CARD_PLATFORMS_SUB)
        .child(quick_row)
        .child(table)
        .mount_current();
    let stats: NodeId = EuvStatGrid::new()
        .card(
            EuvStatCard::new(
                STAT_PLATFORMS_LABEL.to_string(),
                STAT_PLATFORMS_VAL.to_string(),
                STAT_PLATFORMS_CAP.to_string(),
                tokens,
            )
            .mount_current(),
        )
        .card(
            EuvStatCard::new(
                STAT_SCHEMES_LABEL.to_string(),
                STAT_SCHEMES_VAL.to_string(),
                STAT_SCHEMES_CAP.to_string(),
                tokens,
            )
            .mount_current(),
        )
        .card(
            EuvStatCard::new(
                STAT_RENDER_LABEL.to_string(),
                STAT_RENDER_VAL.to_string(),
                STAT_RENDER_CAP.to_string(),
                tokens,
            )
            .mount_current(),
        )
        .mount_current();
    EuvPage::new(
        OVERVIEW_TITLE.to_string(),
        OVERVIEW_SUBTITLE.to_string(),
        tokens,
    )
    .section(stats)
    .section(card)
    .mount_current()
}

/// Mounts the fine-grained reactivity showcase demonstrating `Signal`, `computed!`, `batch`, and `view!`.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted reactivity page node.
pub fn mount_reactive_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let counter: Signal<i32> = state.get_counter();
    let doubled: Memo<i32> = state.get_doubled();
    let mut batch_btn: EuvButton =
        EuvButton::new(BTN_BATCH_TEN.to_string(), tokens).on_click(move |_: &UiEvent| {
            batch(|| {
                for _ in 0..10 {
                    let _: bool = counter.update(|v: &mut i32| *v += 1);
                }
            });
        });
    batch_btn.set_variant(EuvButtonVariant::Secondary);
    let mut reset_btn: EuvButton =
        EuvButton::new(BTN_RESET.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = counter.set_if_changed(0);
        });
    reset_btn.set_variant(EuvButtonVariant::Outline);
    let reactive_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_INCREMENT.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let _: bool = counter.update(|v: &mut i32| *v += 1);
                })
                .mount_current(),
        )
        .child(batch_btn.mount_current())
        .child(reset_btn.mount_current())
        .mount_current();
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            text {
                font_size: 18.0
                font_weight: FontWeight::Bold
                color: p.get_foreground()
                { move || format!("{PREFIX_COUNTER}{}", counter.get()) }
            }
            text {
                font_size: 15.0
                color: p.get_primary()
                { move || format!("{PREFIX_DOUBLED}{}", doubled.get()) }
            }
            { reactive_actions }
        }
    };
    let card: NodeId = EuvCard::new(REACTIVE_CARD_TITLE.to_string(), tokens)
        .subtitle(REACTIVE_CARD_SUB)
        .child(body)
        .mount_current();
    EuvPage::new(
        REACTIVE_TITLE.to_string(),
        REACTIVE_SUBTITLE.to_string(),
        tokens,
    )
    .section(card)
    .mount_current()
}

/// Mounts the multi-mode rendering inspector with live mode switcher and vector `canvas` via `view!`.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted rendering page node.
pub fn mount_render_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let mode_idx: Signal<usize> = state.get_render_mode_index();
    let seg: NodeId = EuvSegmented::new(
        vec![
            MODE_AUTO.to_string(),
            MODE_SOFTWARE.to_string(),
            MODE_HARDWARE.to_string(),
            MODE_HYBRID.to_string(),
        ],
        mode_idx,
        tokens,
    )
    .mount_current();
    let preview: NodeId = view! {
        column {
            gap: SPACE_SM
            text {
                font_size: 13.0
                font_weight: FontWeight::Medium
                color: p.get_primary()
                { move || format!("Active Pipeline Selection: {}", match mode_idx.get() { 1 => MODE_SOFTWARE, 2 => MODE_HARDWARE, 3 => MODE_HYBRID, _ => MODE_AUTO }) }
            }
            text {
                font_size: 13.0
                font_weight: FontWeight::Medium
                color: p.get_muted_foreground()
                { CANVAS_LABEL }
            }
            canvas {
                width: Length::Percent(100.0)
                height: Length::Px(140.0)
                background: p.get_muted()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_md()
                painter: move |bounds: Rect, dl: &mut DisplayList| {
                    let bw: f32 = bounds.get_width().max(120.0);
                    let rect_w: f32 = (bw * 0.28).clamp(56.0, 160.0);
                    let circle_r: f32 = (bw * 0.08).clamp(20.0, 42.0);
                    let circle_x: f32 = bounds.get_x() + 16.0 + rect_w + circle_r + 20.0;
                    let line_x0: f32 = (circle_x + circle_r + 16.0).min(bounds.right() - 24.0);
                    let line_x1: f32 = (bounds.right() - 16.0).max(line_x0);
                    dl.push(DrawCommand::FillRoundedRect {
                        rect: Rect::new(bounds.get_x() + 16.0, bounds.get_y() + 16.0, rect_w, 108.0),
                        color: p.get_primary(),
                        radius: CornerRadius::all(14.0),
                        border: Border::default(),
                    });
                    dl.push(DrawCommand::Circle {
                        center: Point::new(circle_x.min(bounds.right() - circle_r - 8.0), bounds.get_y() + 70.0),
                        radius: circle_r,
                        color: p.get_success(),
                    });
                    dl.push(DrawCommand::Line {
                        from: Point::new(line_x0, bounds.get_y() + 28.0),
                        to: Point::new(line_x1, bounds.get_y() + 112.0),
                        color: p.get_warning(),
                        width: 3.0,
                    });
                }
            }
        }
    };
    let card: NodeId = EuvCard::new(RENDER_CARD_TITLE.to_string(), tokens)
        .subtitle(RENDER_CARD_SUB)
        .child(seg)
        .child(preview)
        .mount_current();
    EuvPage::new(
        RENDER_TITLE.to_string(),
        RENDER_SUBTITLE.to_string(),
        tokens,
    )
    .section(card)
    .mount_current()
}

/// Mounts the OS capabilities page with interactive A11y, Protocol Registration, Dynamic Island, Permissions, and Haptics.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted window & OS services page node.
pub fn mount_window_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let font_info: SystemFontInfo = NativeSystemFont::system().font_info();
    let gpu_driver: NativeGpuDriver = NativeGpuDriver::probe(GraphicsBackend::Metal);
    let gpu_info: &GpuAdapterInfo = gpu_driver.get_adapter();
    let platform: PlatformKind = current_platform();
    let caps: PlatformCapabilities = PlatformCapabilities::for_platform(platform);
    let hub: PlatformServiceHub = PlatformServiceHub::new(platform);
    let cam_key: &'static str = SystemPermission::Camera.native_key(platform);
    let island_active: Signal<bool> = state.get_island_active();
    let island_progress: Signal<u8> = state.get_island_progress();
    let cam_status: Signal<String> = state.get_camera_perm_status();
    let proto_status: Signal<String> = state.get_protocol_status();
    let a11y_count: Signal<u32> = state.get_a11y_announcements();
    let haptic_count: Signal<u32> = state.get_haptic_pulses();
    let bp_sig: Signal<Breakpoint> = use_breakpoint();
    let vp_sig: Signal<Size> = use_viewport_size();
    let summary: String = format!(
        "Platform: {:?} | Font: {} ({:?}) | GPU: {} (shader_ok={})",
        platform,
        font_info.get_family_name(),
        font_info.get_backend(),
        gpu_info.get_device_name(),
        gpu_info.get_shader_compiled()
    );
    let native_feedback: Signal<String> = Signal::new(
        "Ready — tap any OS capability below to trigger native platform feedback".to_string(),
    );
    let fs_sig: Signal<bool> = state.get_fullscreen();
    let mut proto_btn: EuvButton = EuvButton::new("Register euv:// Protocol".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let mut reg: ProtocolRegistry = ProtocolRegistry::new();
            let spec: ProtocolRegistration = ProtocolRegistration::new(
                "euv".to_string(),
                "euv Native Showcase".to_string(),
                SHOWCASE_BUNDLE_ID.to_string(),
            );
            let native_ok: bool =
                dispatch_native_action(NATIVE_ACTION_PROTOCOL, "euv", SHOWCASE_BUNDLE_ID);
            if let Ok(desc) = reg.register(spec, platform) {
                let tag: &str = if native_ok { "OS Bridged" } else { "Verified" };
                let msg: String = format!("Registered euv:// via {} ({tag})", desc.get_mechanism());
                let _: bool = proto_status.set_if_changed(msg.clone());
                let _: bool = native_feedback.set(msg);
            }
        });
    proto_btn.set_variant(EuvButtonVariant::Secondary);
    let mut perm_btn: EuvButton = EuvButton::new("Request Camera Permission".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let mut mgr: PermissionManager = PermissionManager::new();
            let desc: PermissionDescriptor = mgr.request(
                SystemPermission::Camera,
                "Native camera preview".to_string(),
                platform,
            );
            let msg: String = format!("{:?} ({})", desc.get_status(), desc.get_native_key());
            let _: bool = cam_status.set_if_changed(msg.clone());
            let _: bool = native_feedback.set(format!("Permission requested: {msg}"));
        });
    perm_btn.set_variant(EuvButtonVariant::Outline);
    let mut a11y_btn: EuvButton = EuvButton::new("Announce A11y + Haptic".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let mut svc: PlatformServiceHub = PlatformServiceHub::new(platform);
            let fired: bool = svc.trigger_haptic(HapticStyle::Medium);
            svc.announce_a11y(
                "euv native accessibility pulse".to_string(),
                A11yLiveRegion::Polite,
            );
            let _: bool = a11y_count.update(|v: &mut u32| *v += 1);
            let _: bool = haptic_count.update(|v: &mut u32| *v += 1);
            let _: bool = native_feedback.set(format!(
                "Dispatched Haptic pulse (ok={fired}) + A11y announcement #{}",
                a11y_count.get_untracked()
            ));
        });
    a11y_btn.set_variant(EuvButtonVariant::Outline);
    let win_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Toggle Dynamic Island".to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let next_active: bool = !island_active.get_untracked();
                    let _: bool = island_active.set_if_changed(next_active);
                    let next_pct: u8 = if next_active {
                        let cur: u8 = island_progress.get_untracked();
                        if cur >= 100 { 25 } else { (cur + 25).min(100) }
                    } else {
                        island_progress.get_untracked()
                    };
                    let _: bool = island_progress.set_if_changed(next_pct);
                    let mut svc: PlatformServiceHub = PlatformServiceHub::new(platform);
                    if next_active {
                        let mut content: LiveActivityContent = LiveActivityContent::new(
                            "Euv Live Activity".to_string(),
                            format!("Active ({next_pct}%) on {platform:?}"),
                        );
                        content.set_progress_percent(Some(next_pct));
                        let _ = svc.start_live_activity("euv-showcase-island".to_string(), content);
                        let _: bool = native_feedback.set(format!(
                            "Dynamic Island LIVE ({next_pct}%) dispatched to {platform:?}"
                        ));
                    } else {
                        let _: bool = dispatch_native_action(
                            NATIVE_ACTION_ISLAND,
                            "stop",
                            "Dynamic Island paused",
                        );
                        let _: bool =
                            native_feedback.set("Dynamic Island session ended".to_string());
                    }
                })
                .mount_current(),
        )
        .child(proto_btn.mount_current())
        .child(perm_btn.mount_current())
        .child(a11y_btn.mount_current())
        .mount_current();
    let ctx_fs: BuildContext = current_build_context();
    let prev_fs: Rc<Cell<bool>> = Rc::new(Cell::new(fs_sig.get_untracked()));
    let (fs_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            let now_fs: bool = fs_sig.get();
            if prev_fs.replace(now_fs) != now_fs {
                let arg: &str = if now_fs { "on" } else { "off" };
                let _: bool = dispatch_native_action(NATIVE_ACTION_FULLSCREEN, arg, "");
                let _: bool = native_feedback.set(format!("Immersive fullscreen mode: {arg}"));
            }
        })
    });
    let body: NodeId = view! {
        column {
            role: A11yRole::Region
            a11y_label: "Platform Telemetry"
            gap: SPACE_MD
            text {
                font_size: 13.0
                font_weight: FontWeight::SemiBold
                color: p.get_success()
                { move || native_feedback.get() }
            }
            text {
                font_size: 14.0
                font_weight: FontWeight::Medium
                color: p.get_foreground()
                { summary }
            }
            text {
                font_size: 13.0
                font_weight: FontWeight::SemiBold
                color: p.get_primary()
                { move || {
                    let vp: Size = vp_sig.get();
                    format!("Live Viewport: {:.0}x{:.0} | Breakpoint: {:?}", vp.get_width(), vp.get_height(), bp_sig.get())
                } }
            }
            text {
                font_size: 13.0
                color: p.get_primary()
                { move || format!("Dynamic Island: active={} ({}%) | Mode: {:?} | Biometrics: {:?}", island_active.get(), island_progress.get(), PlatformServiceHub::default_island_mode(platform), hub.biometric_kind()) }
            }
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                { move || format!("Protocol: {} | Camera: {} ({cam_key})", proto_status.get(), cam_status.get()) }
            }
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                { move || format!("A11y Bridge: {} (announcements={}) | Haptics: {} (pulses={})", caps.get_accessibility_bridge(), a11y_count.get(), caps.get_haptics(), haptic_count.get()) }
            }
            { win_actions }
            { EuvSwitch::new(fs_sig, SWITCH_FULLSCREEN.to_string(), tokens) }
            { EuvCodeBlock::new(CODE_WINDOW_SNIPPET.to_string(), tokens) }
        }
    };
    ctx_fs.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(body, fs_scope);
        },
        (),
    );
    let target_plat_idx: Signal<usize> = state.get_os_target_platform_idx();
    let island_mode_idx: Signal<usize> = state.get_island_mode_idx();
    let sys_notifs: Signal<Vec<(String, String)>> = state.get_system_notifications();
    let perm_statuses: Signal<Vec<PermissionStatus>> = state.get_permission_statuses();
    let all_platforms: [PlatformKind; 7] = [
        PlatformKind::Ios,
        PlatformKind::IpadOs,
        PlatformKind::Android,
        PlatformKind::MacOs,
        PlatformKind::Windows,
        PlatformKind::Linux,
        PlatformKind::Web,
    ];
    let all_island_modes: [IslandPresentationMode; 6] = [
        IslandPresentationMode::Compact,
        IslandPresentationMode::Minimal,
        IslandPresentationMode::Expanded,
        IslandPresentationMode::LockScreen,
        IslandPresentationMode::FallbackNotification,
        IslandPresentationMode::FallbackStatusBar,
    ];
    let all_perms: [SystemPermission; 14] = [
        SystemPermission::Camera,
        SystemPermission::Microphone,
        SystemPermission::LocationWhenInUse,
        SystemPermission::LocationAlways,
        SystemPermission::Notifications,
        SystemPermission::Photos,
        SystemPermission::Contacts,
        SystemPermission::Calendar,
        SystemPermission::Bluetooth,
        SystemPermission::Biometrics,
        SystemPermission::FileStorage,
        SystemPermission::ClipboardRead,
        SystemPermission::MotionSensors,
        SystemPermission::LocalNetwork,
    ];
    let plat_seg: NodeId = EuvSegmented::new(
        vec![
            "iOS".to_string(),
            "iPadOS".to_string(),
            LABEL_PLATFORM_ANDROID.to_string(),
            LABEL_PLATFORM_MACOS.to_string(),
            "Windows".to_string(),
            "Linux".to_string(),
            "Web".to_string(),
        ],
        target_plat_idx,
        tokens,
    )
    .mount_current();
    let mode_seg: NodeId = EuvSegmented::new(
        vec![
            "Compact".to_string(),
            "Minimal".to_string(),
            "Expanded".to_string(),
            "LockScreen".to_string(),
            "FallbackNotif".to_string(),
            "StatusBar".to_string(),
        ],
        island_mode_idx,
        tokens,
    )
    .mount_current();
    let mut ctx_win: BuildContext = current_build_context();
    let island_preview: NodeId = ctx_win.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .padding(Edges::all(SPACE_MD))
            .gap(SPACE_XS)
            .background(Color::hex(0x09090B))
            .radius(20.0),
    );
    let island_preview_txt: NodeId = ctx_win.reactive_text(
        Style::empty()
            .font_size(12.0)
            .font_weight(FontWeight::SemiBold)
            .color(Color::hex(0xF8FAFC)),
        move || {
            let tp: PlatformKind = all_platforms[target_plat_idx.get() % 7];
            let im: IslandPresentationMode = all_island_modes[island_mode_idx.get() % 6];
            let def_im: IslandPresentationMode = PlatformServiceHub::default_island_mode(tp);
            let active: bool = island_active.get();
            let pct: u8 = island_progress.get();
            let state_tag: &str = if active { "LIVE" } else { "IDLE" };
            format!(
                "[{state_tag}] Dynamic Island ({tp:?}) | Mode: {im:?} (OS Default: {def_im:?}) | Progress: {pct}%"
            )
        },
    );
    ctx_win.append(island_preview, island_preview_txt);
    let notif_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Trigger Alert Notification".to_string(), tokens)
                .sm()
                .on_click(move |_: &UiEvent| {
                    let tp: PlatformKind = all_platforms[target_plat_idx.get_untracked() % 7];
                    let mut svc: PlatformServiceHub = PlatformServiceHub::new(tp);
                    let n: SystemNotification = SystemNotification::new(
                        "euv-alert".to_string(),
                        format!("{tp:?} Alert Notification"),
                        "High-priority native OS alert dispatched cleanly".to_string(),
                    );
                    svc.send_notification(n.clone());
                    let _: bool = sys_notifs.update(|v: &mut Vec<(String, String)>| {
                        v.insert(0, (n.get_title().clone(), n.get_body().clone()));
                        v.truncate(5);
                    });
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Trigger Silent + Badge(3)".to_string(), tokens)
                .secondary()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let tp: PlatformKind = all_platforms[target_plat_idx.get_untracked() % 7];
                    let mut svc: PlatformServiceHub = PlatformServiceHub::new(tp);
                    let mut n: SystemNotification = SystemNotification::new(
                        "euv-sync".to_string(),
                        format!("{tp:?} Silent Sync (Badge 3)"),
                        "Background OTA sync completed silently".to_string(),
                    );
                    n.set_silent(true);
                    n.set_badge_count(Some(3));
                    svc.send_notification(n.clone());
                    let _: bool = sys_notifs.update(|v: &mut Vec<(String, String)>| {
                        v.insert(0, (n.get_title().clone(), n.get_body().clone()));
                        v.truncate(5);
                    });
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Clear Notifications".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = sys_notifs.set_if_changed(Vec::new());
                })
                .mount_current(),
        )
        .mount_current();
    let notif_deck: NodeId = EuvSonner::new(sys_notifs, tokens).mount_current();
    let perm_list_host: NodeId = ctx_win.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_XS),
    );
    ctx_win.bind_children(perm_list_host, move || {
        let tp: PlatformKind = all_platforms[target_plat_idx.get() % 7];
        let statuses: Vec<PermissionStatus> = perm_statuses.get();
        let mut rows: Vec<NodeId> = Vec::with_capacity(14);
        let mut bctx: BuildContext = current_build_context();
        for (idx, perm) in all_perms.iter().copied().enumerate() {
            let st: PermissionStatus = statuses
                .get(idx)
                .copied()
                .unwrap_or(PermissionStatus::NotDetermined);
            let native_key: &'static str = perm.native_key(tp);
            let row: NodeId = bctx.view(
                Style::empty()
                    .column()
                    .width(Length::Percent(100.0))
                    .padding(Edges::symmetric(SPACE_SM, SPACE_SM))
                    .gap(SPACE_XS)
                    .background(p.get_card())
                    .border(tokens.get_geometry().get_border_width(), p.get_border())
                    .radius(tokens.get_geometry().get_radius_sm()),
            );
            let lbl: NodeId = bctx.text(
                format!("{perm:?} [{st:?}] · {native_key}"),
                Style::empty()
                    .width(Length::Percent(100.0))
                    .font_size(12.0)
                    .color(p.get_foreground()),
            );
            let req_btn: NodeId = EuvButton::new("Request".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let mut mgr: PermissionManager = PermissionManager::new();
                    let desc: PermissionDescriptor =
                        mgr.request(perm, format!("Test {perm:?} access"), platform);
                    let _: bool = perm_statuses.update(|vec: &mut Vec<PermissionStatus>| {
                        if let Some(slot) = vec.get_mut(idx) {
                            *slot = desc.get_status();
                        }
                    });
                    let _: bool = native_feedback.set(format!(
                        "Requested {perm:?} ({}) -> {:?}",
                        perm.native_key(platform),
                        desc.get_status()
                    ));
                })
                .mount(&mut bctx);
            let deny_btn: NodeId = EuvButton::new("Deny".to_string(), tokens)
                .destructive()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = perm_statuses.update(|vec: &mut Vec<PermissionStatus>| {
                        if let Some(slot) = vec.get_mut(idx) {
                            *slot = PermissionStatus::Denied;
                        }
                    });
                    let _: bool = native_feedback.set(format!("Marked {perm:?} as Denied"));
                })
                .mount(&mut bctx);
            let btn_pair: NodeId = EuvActionRow::new()
                .child(req_btn)
                .child(deny_btn)
                .mount(&mut bctx);
            bctx.append(row, lbl);
            bctx.append(row, btn_pair);
            rows.push(row);
        }
        rows
    });
    let fps_limit_sig: Signal<Option<u32>> = ctx_win.get_fps_limit();
    let live_fps_sig: Signal<f32> = ctx_win.get_live_fps();
    let fps_idx: Signal<usize> = Signal::new(match fps_limit_sig.get_untracked() {
        Some(30) => 1,
        Some(60) => 2,
        Some(120) => 3,
        Some(144) => 4,
        Some(240) => 5,
        _ => 0,
    });
    let fps_seg: NodeId = EuvSegmented::new(
        vec![
            "Unlimited".to_string(),
            "30 FPS".to_string(),
            "60 FPS".to_string(),
            "120 FPS".to_string(),
            "144 FPS".to_string(),
            "240 FPS".to_string(),
        ],
        fps_idx,
        tokens,
    )
    .mount_current();
    let fps_readout: NodeId = ctx_win.reactive_text(
        Style::empty()
            .font_size(13.0)
            .font_weight(FontWeight::SemiBold)
            .color(p.get_primary()),
        move || {
            let cap_txt: String = match fps_limit_sig.get() {
                None => "Unlimited (Max FPS)".to_string(),
                Some(v) => format!("Capped at {v} FPS"),
            };
            format!(
                "Frame Rate Policy: {cap_txt} | Live Measured: {:.1} FPS",
                live_fps_sig.get()
            )
        },
    );
    let (fps_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            let next_cap: Option<u32> = match fps_idx.get() {
                1 => Some(30),
                2 => Some(60),
                3 => Some(120),
                4 => Some(144),
                5 => Some(240),
                _ => None,
            };
            let _: bool = fps_limit_sig.set_if_changed(next_cap);
        })
    });
    let fps_card: NodeId = EuvCard::new("Cross-Platform Frame Rate Limiter".to_string(), tokens)
        .subtitle(
            "Default Unlimited (Max FPS) on Desktop, Web, Android & iOS — adjust cap live anytime",
        )
        .child(fps_seg)
        .child(fps_readout)
        .mount_current();
    ctx_win.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(fps_card, fps_scope);
        },
        (),
    );
    let island_card: NodeId = EuvCard::new(
        "Cross-Platform Dynamic Island & Notifications".to_string(),
        tokens,
    )
    .subtitle("Test Live Activities across all 7 platforms and dispatch native OS notifications")
    .child(plat_seg)
    .child(mode_seg)
    .child(island_preview)
    .child(notif_actions)
    .child(notif_deck)
    .mount_current();
    let perm_card: NodeId = EuvCard::new(
        "Complete 14-Category OS Permissions Matrix".to_string(),
        tokens,
    )
    .subtitle("Inspect per-platform native entitlement keys and test Grant / Deny flows")
    .child(perm_list_host)
    .mount_current();
    let card: NodeId = EuvCard::new(WINDOW_CARD_TITLE.to_string(), tokens)
        .subtitle(WINDOW_CARD_SUB)
        .child(body)
        .mount_current();
    EuvPage::new(
        WINDOW_TITLE.to_string(),
        WINDOW_SUBTITLE.to_string(),
        tokens,
    )
    .section(card)
    .section(fps_card)
    .section(island_card)
    .section(perm_card)
    .mount_current()
}

/// Mounts the complete shadcn/ui component showcase and production edge-case regression lab.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted UI & Edge Lab page node.
pub fn mount_lab_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let mut ctx: BuildContext = current_build_context();
    let chart_data: Vec<EuvChartPoint> = vec![
        EuvChartPoint::new("Mon".to_string(), 60.0),
        EuvChartPoint::new("Tue".to_string(), 88.0),
        EuvChartPoint::new("Wed".to_string(), 118.0),
        EuvChartPoint::new("Thu".to_string(), 104.0),
        EuvChartPoint::new("Fri".to_string(), 120.0),
        EuvChartPoint::new("Sat".to_string(), 116.0),
    ];
    let bar_chart: NodeId = EuvChart::new(
        "Frame Rate Telemetry (Bar)".to_string(),
        EuvChartKind::Bar,
        chart_data.clone(),
        tokens,
    )
    .mount(&mut ctx);
    let area_chart: NodeId = EuvChart::new(
        "3D Ray-Tracing Throughput (Area)".to_string(),
        EuvChartKind::Area,
        chart_data,
        tokens,
    )
    .mount(&mut ctx);
    let combobox: NodeId = EuvCombobox::new(
        vec![
            "euv Native Rust".to_string(),
            "WebAssembly Zero-JS".to_string(),
            "Android ARM64 JNI".to_string(),
            "iOS CoreText Metal".to_string(),
            "macOS AppKit Retina".to_string(),
        ],
        state.get_lab_combobox_selected(),
        state.get_lab_combobox_open(),
        tokens,
    )
    .query(state.get_lab_combobox_query())
    .placeholder("Filter frameworks...")
    .mount(&mut ctx);
    let date_picker: NodeId = EuvDatePicker::new(
        LABEL_CALENDAR_MONTH.to_string(),
        state.get_lab_date_day(),
        state.get_lab_date_open(),
        tokens,
    )
    .mount(&mut ctx);
    let input_group: NodeId = EuvInputGroup::new(state.get_lab_input_group_val(), tokens)
        .prefix("https://")
        .placeholder("subdomain")
        .suffix(".euv.dev")
        .mount(&mut ctx);
    let native_sel: NodeId = EuvNativeSelect::new(
        vec![
            "60 Hz Standard".to_string(),
            "120 Hz ProMotion".to_string(),
            "Unlocked Benchmark".to_string(),
        ],
        state.get_lab_native_select_idx(),
        tokens,
    )
    .mount(&mut ctx);
    let nav_menu: NodeId = EuvNavigationMenu::new(
        vec![
            (
                "Architecture".to_string(),
                "Arena signals + deterministic Flexbox/Grid + retained scene graph.".to_string(),
            ),
            (
                "3D Engine".to_string(),
                "PBR metallic-roughness + BVH ray-traced shadows/reflections + FXAA.".to_string(),
            ),
            (
                "Media Suite".to_string(),
                "Pure-Rust Image, SVG, GIF, LIVE Photo, Video, WAV Audio, PDF, HTML.".to_string(),
            ),
        ],
        state.get_lab_nav_menu_active(),
        tokens,
    )
    .mount(&mut ctx);
    let data_table: NodeId = EuvDataTable::new(
        vec![
            "Target".to_string(),
            "Font Engine".to_string(),
            "Status".to_string(),
        ],
        vec![
            vec![
                LABEL_PLATFORM_ANDROID.to_string(),
                "Skia Paint AA".to_string(),
                "1:1 DPI".to_string(),
            ],
            vec![
                "iOS / iPadOS".to_string(),
                "CoreText @3x".to_string(),
                "ProMotion".to_string(),
            ],
            vec![
                LABEL_PLATFORM_MACOS.to_string(),
                "CoreText Retina".to_string(),
                "Live Resize".to_string(),
            ],
            vec![
                "WebAssembly".to_string(),
                "Canvas2D AA".to_string(),
                "Zero-JS".to_string(),
            ],
        ],
        tokens,
    )
    .filter(state.get_lab_table_filter())
    .sort_ascending(state.get_lab_table_sort_asc())
    .selected_row(state.get_lab_table_selected())
    .mount(&mut ctx);
    let left_split: NodeId = ctx.text(
        "Left Split Pane (Drag handle ->)".to_string(),
        Style::empty().font_size(13.0).color(p.get_foreground()),
    );
    let right_split: NodeId = ctx.text(
        "Right Split Pane (Zero rebuild on drag)".to_string(),
        Style::empty()
            .font_size(13.0)
            .color(p.get_muted_foreground()),
    );
    let resizable: NodeId =
        EuvResizable::new(left_split, right_split, state.get_lab_split_ratio(), tokens)
            .mount(&mut ctx);
    let item_row: NodeId = EuvItem::new("Production shadcn/ui Alignment".to_string(), tokens)
        .leading("EU")
        .description(
            "Unified layout grammar across all 17 design schemes with per-theme surface/radius tokens",
        )
        .trailing("v0.38")
        .mount(&mut ctx);
    let sep: NodeId =
        EuvSeparator::new("EDGE-CASE REGRESSION SUITE".to_string(), tokens).mount(&mut ctx);
    let rtl_sig: Signal<bool> = state.get_lab_rtl_mode();
    let dir_host: NodeId = ctx.view(Style::empty().column().width(Length::Percent(100.0)));
    ctx.bind_children(dir_host, move || {
        let dir: EuvTextDirection = if rtl_sig.get() {
            EuvTextDirection::Rtl
        } else {
            EuvTextDirection::Ltr
        };
        let mut bctx: BuildContext = current_build_context();
        let sample: NodeId = bctx.text(
            format!("Direction preview ({dir:?}): Unified layout alignment across LTR & RTL"),
            Style::empty().font_size(13.0).color(p.get_foreground()),
        );
        vec![EuvDirection::new(dir, sample, tokens).mount(&mut bctx)]
    });
    let block_scroll: Signal<bool> = state.get_lab_block_scroll_chain();
    let stop_bubble: Signal<bool> = state.get_lab_stop_click_bubble();
    let parent_clicks: Signal<u32> = state.get_lab_parent_clicks();
    let child_clicks: Signal<u32> = state.get_lab_child_clicks();
    let preserve_cnt: Signal<u32> = state.get_lab_resize_preserve_count();
    let modal_open: Signal<bool> = state.get_modal_open();
    let drawer_open: Signal<bool> = state.get_drawer_open();
    let nested_scroll_host: NodeId =
        ctx.view(Style::empty().column().width(Length::Percent(100.0)));
    ctx.bind_children(nested_scroll_host, move || {
        let locked: bool = block_scroll.get();
        let stop_clk: bool = stop_bubble.get();
        let mut bctx: BuildContext = current_build_context();
        let outer: NodeId = bctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(SPACE_MD))
                .gap(SPACE_SM)
                .background(p.get_muted())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_md()),
        );
        bctx.on_click(outer, move |_: &UiEvent| {
            let _: bool = parent_clicks.update(|v: &mut u32| *v += 1);
        });
        let inner_scroll: NodeId = bctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .height(Length::Px(110.0))
                .padding(Edges::all(SPACE_SM))
                .gap(SPACE_XS)
                .background(p.get_card())
                .border(g.get_border_width(), p.get_border())
                .radius(g.get_radius_sm())
                .stop_scroll_propagation(locked)
                .stop_event_propagation(stop_clk),
        );
        for row_i in 1..=8 {
            let line: NodeId = bctx.text(
                format!(
                    "Nested Scroll Row #{row_i} (scroll_lock={locked}, stop_bubble={stop_clk})"
                ),
                Style::empty().font_size(12.0).color(p.get_foreground()),
            );
            bctx.append(inner_scroll, line);
        }
        let inner_btn: NodeId = EuvButton::new("Click Inner Target".to_string(), tokens)
            .secondary()
            .sm()
            .on_click(move |_: &UiEvent| {
                let _: bool = child_clicks.update(|v: &mut u32| *v += 1);
            })
            .mount(&mut bctx);
        bctx.append(outer, inner_scroll);
        bctx.append(outer, inner_btn);
        vec![outer]
    });
    let bubble_stats: NodeId = ctx.reactive_text(
        Style::empty()
            .font_size(12.0)
            .color(p.get_muted_foreground()),
        move || {
            format!(
                "Inner Clicks: {} | Outer Parent Clicks: {} | Resize-Preserved Counter: {} | Overlay Stack Depth: {}",
                child_clicks.get(),
                parent_clicks.get(),
                preserve_cnt.get(),
                EuvOverlayStack::active_count()
            )
        },
    );
    let edge_controls: NodeId = EuvActionRow::new()
        .child(
            EuvSwitch::new(block_scroll, "Lock Scroll Penetration".to_string(), tokens)
                .mount_current(),
        )
        .child(
            EuvSwitch::new(stop_bubble, "Stop Click Bubbling".to_string(), tokens).mount_current(),
        )
        .child(EuvSwitch::new(rtl_sig, "RTL Direction".to_string(), tokens).mount_current())
        .child(
            EuvButton::new("Increment Preserved Counter".to_string(), tokens)
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = preserve_cnt.update(|v: &mut u32| *v += 1);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Open Stacked Modal + Drawer".to_string(), tokens)
                .secondary()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = modal_open.set_if_changed(true);
                    let _: bool = drawer_open.set_if_changed(true);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Dismiss Top Overlay (Back)".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = EuvOverlayStack::dismiss_top();
                })
                .mount_current(),
        )
        .mount_current();
    let trace_view: NodeId = EuvTrace::new(
        "Frame Pipeline Trace (1.42ms Total)".to_string(),
        vec![
            EuvTraceSpan::new(
                "layout.compute".to_string(),
                0.14,
                0.0,
                0.12,
                EuvTone::Primary,
            ),
            EuvTraceSpan::new(
                "bvh.raytrace_gi".to_string(),
                0.88,
                0.12,
                0.60,
                EuvTone::Success,
            ),
            EuvTraceSpan::new(
                "fxaa.tonemap".to_string(),
                0.24,
                0.72,
                0.16,
                EuvTone::Warning,
            ),
            EuvTraceSpan::new(
                "present.blit".to_string(),
                0.16,
                0.88,
                0.12,
                EuvTone::Neutral,
            ),
        ],
        tokens,
    )
    .mount(&mut ctx);
    let attach_vis: Signal<bool> = Signal::new(true);
    let attach_view: NodeId = EuvAttachment::new(
        "vice-boulevard-rt.wasm".to_string(),
        "48.2 KB · application/wasm · SHA-verified".to_string(),
        attach_vis,
        tokens,
    )
    .mount(&mut ctx);
    let bubble_in: NodeId = EuvBubble::new(
        "Euv Runtime".to_string(),
        "All 60+ shadcn/ui components rendered directly via Rust SceneTree with zero WebView."
            .to_string(),
        tokens,
    )
    .timestamp("09:41")
    .mount(&mut ctx);
    let bubble_out: NodeId = EuvBubble::new(
        "Developer".to_string(),
        "Verify multi-touch, text wrapping, blinking input caret & uncapped FPS live!".to_string(),
        tokens,
    )
    .timestamp("09:42")
    .outgoing(true)
    .mount(&mut ctx);
    let markers_row: NodeId = view! {
        row {
            style: c_action_row()
            { EuvMarker::new("01".to_string(), "Zero WebView".to_string(), EuvTone::Primary, tokens) }
            { EuvMarker::new("02".to_string(), "120+ FPS".to_string(), EuvTone::Success, tokens) }
            { EuvMarker::new("03".to_string(), "Multi-Touch".to_string(), EuvTone::Warning, tokens) }
            { EuvAvatar::new("EU".to_string(), 32.0, tokens) }
            { EuvKbd::new("Cmd+K".to_string(), tokens) }
            { EuvSpinner::new(18.0, tokens) }
        }
    };
    let q_sel: Signal<Option<usize>> = Signal::new(Some(0));
    let questionnaire: NodeId = EuvQuestionnaire::new(
        "Preferred native target deployment pipeline?".to_string(),
        vec![
            "Universal Native (Android + iOS + Desktop + Web)".to_string(),
            "Dynamic OTA .wasm Hot-Swap Modules".to_string(),
            "Embedded 3D Ray-Traced Engine Viewport".to_string(),
        ],
        q_sel,
        tokens,
    )
    .mount(&mut ctx);
    let extra_primitives: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_SM
            { EuvBreadcrumb::new(vec!["Euv Studio".to_string(), "Design System".to_string(), "Components".to_string()], tokens) }
            { EuvTypography::new("Unified shadcn/ui Typography & Surface Tokens".to_string(), EuvTypographyVariant::H3, tokens) }
            { EuvSkeleton::new(260.0, 18.0, tokens) }
            { EuvEmpty::new("No Pending Warnings".to_string(), "All layout, scroll, shadow and typography checks passed.".to_string(), tokens) }
        }
    };
    let touches_sig: Signal<Vec<(u64, Point)>> = ctx.get_active_touches();
    let touch_pad: NodeId = ctx.canvas(
        Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(170.0))
            .background(p.get_muted())
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md())
            .stop_scroll_propagation(true)
            .stop_event_propagation(true),
        move |bounds: Rect, dl: &mut DisplayList| {
            let pts: Vec<(u64, Point)> = touches_sig.get_untracked();
            let palette: [Color; 5] = [
                Color::hex(0x2563EB),
                Color::hex(0x16A34A),
                Color::hex(0xEA580C),
                Color::hex(0x9333EA),
                Color::hex(0xDC2626),
            ];
            for (idx, (_, pt)) in pts.iter().enumerate() {
                if bounds.contains(*pt) {
                    let col: Color = palette[idx % palette.len()];
                    dl.push(DrawCommand::Circle {
                        center: *pt,
                        radius: 26.0,
                        color: col.multiply_alpha(0.28),
                    });
                    dl.push(DrawCommand::Circle {
                        center: *pt,
                        radius: 10.0,
                        color: col,
                    });
                }
            }
            if pts.len() >= 2 && bounds.contains(pts[0].1) && bounds.contains(pts[1].1) {
                dl.push(DrawCommand::Line {
                    from: pts[0].1,
                    to: pts[1].1,
                    color: p.get_primary(),
                    width: 2.0,
                });
            }
        },
    );
    let touch_stats: NodeId = ctx.reactive_text(
        Style::empty()
            .font_size(12.5)
            .font_weight(FontWeight::Medium)
            .color(p.get_primary()),
        move || {
            let pts: Vec<(u64, Point)> = touches_sig.get();
            if pts.is_empty() {
                "Multi-Touch Pad Ready: place 1, 2, or more fingers inside the pad (sidebar & scroll isolated)".to_string()
            } else {
                let coords: String = pts
                    .iter()
                    .map(|(id, pt): &(u64, Point)| {
                        format!("#{id}({:.0},{:.0})", pt.get_x(), pt.get_y())
                    })
                    .collect::<Vec<String>>()
                    .join("  ");
                format!("Active Touches: {}  |  {coords}", pts.len())
            }
        },
    );
    let touch_handle: SceneHandle = ctx.get_scene().clone();
    let (touch_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            let _: Vec<(u64, Point)> = touches_sig.get();
            touch_handle.mark_dirty(touch_pad, DIRTY_PAINT);
        })
    });
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(touch_pad, touch_scope);
        },
        (),
    );
    let multitouch_card: NodeId = EuvCard::new("Multi-Touch & Gesture Isolation Pad".to_string(), tokens)
        .subtitle("Simultaneous multi-finger tracking with pointer IDs, pinch distance line, and gesture non-penetration")
        .child(touch_stats)
        .child(touch_pad)
        .mount_current();
    let shadcn_card: NodeId = EuvCard::new("Complete shadcn/ui Component Parity".to_string(), tokens)
        .subtitle("Trace, Attachment, Bubble, Marker, Questionnaire, Chart, Combobox, DataTable, DatePicker, InputGroup, Item, NativeSelect, NavigationMenu, Resizable, Separator, Direction, Avatar, Kbd, Spinner, Skeleton, Breadcrumb, Empty")
        .child(markers_row)
        .child(trace_view)
        .child(attach_view)
        .child(bubble_in)
        .child(bubble_out)
        .child(questionnaire)
        .child(nav_menu)
        .child(bar_chart)
        .child(area_chart)
        .child(combobox)
        .child(date_picker)
        .child(input_group)
        .child(native_sel)
        .child(data_table)
        .child(resizable)
        .child(item_row)
        .child(extra_primitives)
        .mount_current();
    let edge_card: NodeId = EuvCard::new("Boundary & Event Penetration Regression Lab".to_string(), tokens)
        .subtitle("Verify scroll lock, flexible event bubbling, overlay back stack, RTL alignment, and zero-rebuild resize preservation")
        .child(sep)
        .child(dir_host)
        .child(edge_controls)
        .child(bubble_stats)
        .child(nested_scroll_host)
        .mount_current();
    let ai_tick: Signal<u32> = Signal::new(0);
    let ai_fps_sig: Signal<f32> = ctx.get_live_fps();
    let ai_route_sig: Signal<String> = state.get_route();
    let ai_scheme_sig: Signal<DesignScheme> = state.get_theme().get_scheme();
    let ai_telemetry_txt: NodeId = ctx.reactive_text(
        Style::empty()
            .width(Length::Percent(100.0))
            .font_size(12.5)
            .color(p.get_foreground()),
        move || {
            let _: u32 = ai_tick.get();
            let rs: ReactiveStats = reactive_stats();
            #[cfg(debug_assertions)]
            let proto_label: &str = AI_SCHEMA_PROTOCOL_VERSION;
            #[cfg(not(debug_assertions))]
            let proto_label: &str = AI_SCHEMA_RELEASE_STRIPPED;
            let bridge_label: &str = if DevAutomationBridge::is_enabled() {
                AI_BRIDGE_STATUS_ACTIVE
            } else {
                AI_BRIDGE_STATUS_OFF
            };
            format!(
                "Protocol: {} ({}) | Route: {} | Scheme: {} | Live FPS: {:.1} | Signals: {} | Memos: {} | Effects: {} | Scopes: {} | Clock: {}",
                proto_label,
                bridge_label,
                ai_route_sig.get(),
                ai_scheme_sig.get().as_str(),
                ai_fps_sig.get(),
                rs.get_live_signals(),
                rs.get_live_memos(),
                rs.get_live_effects(),
                rs.get_live_scopes(),
                rs.get_clock()
            )
        },
    );
    let ai_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Refresh AI Telemetry".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = ai_tick.update(|v: &mut u32| *v = v.wrapping_add(1));
                })
                .mount_current(),
        )
        .mount_current();
    let ai_endpoints: NodeId = EuvCodeBlock::new(
        "GET /ai/schema | /ai/snapshot | /ai/a11y | /ai/health\nPOST /ai/query | /ai/action | /ai/script (dev-only; 0B in release)".to_string(),
        tokens,
    )
    .mount_current();
    let ai_card: NodeId = EuvCard::new("AI Schema v1 & Reactive Telemetry Bridge".to_string(), tokens)
        .subtitle("Live introspection of SceneTree bounds, A11y tree, reactive graph, FPS, and interactive agent actions")
        .child(ai_telemetry_txt)
        .child(ai_actions)
        .child(ai_endpoints)
        .mount_current();
    EuvPage::new(
        "UI & Edge-Case Lab".to_string(),
        "Full shadcn/ui component parity + multi-touch, propagation, overlay stack, AI Schema v1, and resize-preservation tests".to_string(),
        tokens,
    )
    .section(multitouch_card)
    .section(ai_card)
    .section(shadcn_card)
    .section(edge_card)
    .mount_current()
}

/// Mounts the comprehensive interactive forms showcase covering inputs, steppers, radios, sliders, ratings, toggles, switches, and checkboxes.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted forms page node.
pub fn mount_forms_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let username: Signal<String> = state.get_username();
    let stepper: Signal<i64> = state.get_stepper_value();
    let rating: Signal<usize> = state.get_rating_value();
    let cal_day: Signal<u32> = state.get_calendar_day();
    let input: NodeId =
        EuvInput::new(username, PLACEHOLDER_USERNAME.to_string(), tokens).mount_current();
    let mut b2: EuvButton = EuvButton::new(BTN_SECONDARY_LABEL.to_string(), tokens);
    b2.set_variant(EuvButtonVariant::Secondary);
    let mut b3: EuvButton = EuvButton::new(BTN_OUTLINE_LABEL.to_string(), tokens);
    b3.set_variant(EuvButtonVariant::Outline);
    let mut b4: EuvButton =
        EuvButton::new(BTN_DESTRUCTIVE_LABEL.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = username.set_if_changed(DEFAULT_USERNAME.to_string());
        });
    b4.set_variant(EuvButtonVariant::Destructive);
    let collapsible_body: NodeId = view! {
        text {
            font_size: 13.0
            color: p.get_muted_foreground()
            "shadcn/ui Collapsible disclosure panel adapts across Mobile, Tablet, and Desktop."
        }
    };
    let form_btns: NodeId = EuvActionRow::new()
        .child(EuvButton::new(BTN_PRIMARY_LABEL.to_string(), tokens).mount_current())
        .child(b2.mount_current())
        .child(b3.mount_current())
        .child(b4.mount_current())
        .mount_current();
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            { EuvTabs::new(vec![TAB_GENERAL.to_string(), TAB_GRAPHICS.to_string(), TAB_SECURITY.to_string()], state.get_active_tab(), tokens) }
            { EuvField::new(FIELD_USERNAME.to_string(), input, tokens) }
            { EuvTextarea::new(state.get_bio(), "Write notes...".to_string(), 54.0, tokens) }
            row {
                style: c_action_row()
                { EuvNumberInput::new(stepper, tokens).range(0, 20).step(1) }
                { EuvToggle::new(state.get_pin_toggled(), "Pin Config".to_string(), tokens) }
                { EuvRating::new(rating, 5, tokens) }
            }
            { EuvRadioGroup::new(vec!["Software CPU".to_string(), "Hardware GPU".to_string(), "Hybrid".to_string()], state.get_radio_index(), tokens) }
            { EuvSlider::new(state.get_slider_value(), 0.0, 100.0, tokens) }
            { EuvSelect::new(vec!["Metal".to_string(), "Vulkan".to_string(), "Direct3D 12".to_string(), "WebGPU".to_string()], state.get_select_index(), state.get_select_open(), tokens) }
            { EuvSwitch::new(state.get_hardware_accel(), SWITCH_HW_LABEL.to_string(), tokens) }
            { EuvCheckbox::new(state.get_agree_terms(), CHECKBOX_TERMS_LABEL.to_string(), tokens) }
            text {
                font_size: 12.0
                color: p.get_muted_foreground()
                { move || format!("Handle: {} | Stepper: {} | Stars: {}/5 | Day: {}", username.get(), stepper.get(), rating.get(), cal_day.get()) }
            }
            { form_btns }
        }
    };
    let shadcn_body: NodeId = view! {
        column {
            gap: SPACE_MD
            { EuvToggleGroup::new(vec!["Bold".to_string(), "Italic".to_string(), "Underline".to_string()], state.get_toggle_group_index(), tokens) }
            { EuvInputOtp::new(state.get_otp_code(), 6, tokens) }
            { EuvCalendar::new(LABEL_CALENDAR_MONTH.to_string(), 30, state.get_calendar_day(), tokens) }
            { EuvCollapsible::new(state.get_collapsible_open(), "Advanced Form Details".to_string(), collapsible_body, tokens) }
        }
    };
    let card: NodeId = EuvCard::new(FORMS_CARD_TITLE.to_string(), tokens)
        .subtitle(FORMS_CARD_SUB)
        .child(body)
        .mount_current();
    let shadcn_card: NodeId = EuvCard::new("shadcn/ui Extended Form Controls".to_string(), tokens)
        .subtitle("ToggleGroup, InputOTP, Calendar, and Collapsible")
        .child(shadcn_body)
        .mount_current();
    EuvPage::new(FORMS_TITLE.to_string(), FORMS_SUBTITLE.to_string(), tokens)
        .section(card)
        .section(shadcn_card)
        .mount_current()
}

/// Mounts the 17-scheme live design token gallery where every scheme button switches the active app theme live.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted themes page node.
pub fn mount_themes_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let scheme_sig: Signal<DesignScheme> = state.get_theme().get_scheme();
    let mode_sig: Signal<ThemeMode> = state.get_theme().get_mode();
    let state_for_cycle: ShowcaseState = state.clone();
    let state_for_mode: ShowcaseState = state.clone();
    let mut mode_btn: EuvButton =
        EuvButton::new(BTN_TOGGLE_MODE.to_string(), tokens).on_click(move |_: &UiEvent| {
            state_for_mode.toggle_theme_mode();
        });
    mode_btn.set_variant(EuvButtonVariant::Outline);
    let scheme_cols: u16 = match use_breakpoint().get_untracked() {
        Breakpoint::Compact => 2,
        Breakpoint::Medium => 3,
        Breakpoint::Expanded => 4,
    };
    let badges_grid: NodeId = view! {
        grid {
            style: c_scheme_grid()
            columns: scheme_cols
        }
    };
    let mut ctx: BuildContext = current_build_context();
    let all_schemes: [DesignScheme; 17] = DesignScheme::all();
    for scheme in all_schemes.iter().copied() {
        let preview_tokens: ThemeTokens = ThemeTokens::resolve(scheme, mode_sig.get_untracked());
        let mut swatch: EuvButton = EuvButton::new(scheme.as_str().to_string(), preview_tokens)
            .on_click(move |_: &UiEvent| {
                let _: bool = scheme_sig.set_if_changed(scheme);
            });
        swatch.set_size(EuvButtonSize::Small);
        ctx.append(badges_grid, swatch.mount_current());
    }
    let theme_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_NEXT_SCHEME.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    state_for_cycle.next_scheme();
                })
                .mount_current(),
        )
        .child(mode_btn.mount_current())
        .mount_current();
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            text {
                font_size: 15.0
                font_weight: FontWeight::SemiBold
                color: p.get_foreground()
                { move || format!("{PREFIX_ACTIVE_SCHEME}{} ({:?})", scheme_sig.get().as_str(), mode_sig.get()) }
            }
            { theme_actions }
            { badges_grid }
        }
    };
    let card: NodeId = EuvCard::new(THEMES_CARD_TITLE.to_string(), tokens)
        .subtitle(THEMES_CARD_SUB)
        .child(body)
        .mount_current();
    EuvPage::new(
        THEMES_TITLE.to_string(),
        THEMES_SUBTITLE.to_string(),
        tokens,
    )
    .section(card)
    .mount_current()
}

/// Mounts the reactive 10,000-row virtualized scrollable list plus pagination, steps, accordion, and progress.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted data display page node.
pub fn mount_data_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let scroll_y: Signal<f32> = state.get_list_scroll_y();
    let vlist: EuvVirtualList =
        EuvVirtualList::new(10_000, 32.0, scroll_y, tokens).viewport_height(224.0);
    let vlist_down: EuvVirtualList = vlist.clone();
    let list_node: NodeId = vlist.mount_with(move |idx: usize| {
        view! {
            row {
                width: Length::Percent(100.0)
                height: Length::Px(32.0)
                padding: Edges::symmetric(SPACE_XS, SPACE_MD)
                align: AlignItems::Center
                border: (g.get_border_width(), p.get_border())
                text {
                    font_size: 13.0
                    color: p.get_foreground()
                    { format!("{ROW_PREFIX}{idx}") }
                }
            }
        }
    });
    let mut top_btn: EuvButton =
        EuvButton::new(BTN_SCROLL_TOP.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = scroll_y.set_if_changed(0.0);
        });
    top_btn.set_variant(EuvButtonVariant::Outline);
    let aspect_inner: NodeId = view! {
        column {
            align: AlignItems::Center
            gap: SPACE_XS
            text {
                font_size: 14.0
                font_weight: FontWeight::SemiBold
                color: p.get_foreground()
                "16:9 shadcn AspectRatio"
            }
            text {
                font_size: 12.0
                color: p.get_muted_foreground()
                "Responsive container locked to 16:9 ratio"
            }
        }
    };
    let aspect_box: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            max_width: Length::Px(360.0)
            { EuvAspectRatio::new(16.0 / 9.0, aspect_inner, tokens) }
        }
    };
    let scroll_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_SCROLL_DOWN.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    vlist_down.scroll_by(160.0);
                })
                .mount_current(),
        )
        .child(top_btn.mount_current())
        .mount_current();
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            { EuvMenubar::new(vec!["File".to_string(), "Edit".to_string(), "View".to_string(), "Profiles".to_string()], state.get_menubar_active(), tokens) }
            row {
                style: c_action_row()
                { EuvBadge::new(BADGE_READY.to_string(), EuvTone::Success, tokens) }
                { EuvBadge::new(BADGE_LIVE.to_string(), EuvTone::Primary, tokens) }
            }
            { scroll_actions }
            text {
                font_size: 12.0
                color: p.get_muted_foreground()
                { move || format!("scroll_y={:.0}px | top_row=#{}", scroll_y.get(), (scroll_y.get() / 32.0).floor() as usize) }
            }
            { EuvProgress::new(0.84, tokens) }
            { list_node }
            { EuvCommand::new(vec!["Switch to Dark Mode".to_string(), "Open OTA Updates".to_string(), "Run Physics Tick".to_string()], state.get_command_selected(), tokens).query(state.get_command_query()).placeholder("Search commands...") }
            { EuvCarousel::new(vec!["Slide 1: Native Metal & Vulkan".to_string(), "Slide 2: Complete shadcn/ui Parity".to_string(), "Slide 3: Instant Responsive Resize".to_string()], state.get_carousel_index(), tokens) }
            { aspect_box }
            { EuvPagination::new(state.get_page_index(), 12, tokens) }
            { EuvSteps::new(vec!["Layout".to_string(), "Paint".to_string(), "Present".to_string()], state.get_step_index(), tokens) }
            { EuvAccordion::new(vec![EuvAccordionItem::new("Virtualization".to_string(), "Only visible rows + overscan are materialized in O(visible).".to_string())], state.get_accordion_open(), tokens) }
        }
    };
    let card: NodeId = EuvCard::new(DATA_CARD_TITLE.to_string(), tokens)
        .subtitle(DATA_CARD_SUB)
        .child(body)
        .mount_current();
    EuvPage::new(DATA_TITLE.to_string(), DATA_SUBTITLE.to_string(), tokens)
        .section(card)
        .mount_current()
}

/// Mounts the embedded 2D physics game engine viewport and reactive HUD overlay page.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted game engine showcase page node.
pub fn mount_game_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let g: ThemeGeometry = tokens.get_geometry();
    let p: ThemePalette = tokens.get_palette();
    let host: GameEngineHost = state.get_game_host().clone();
    let host_btn: GameEngineHost = host.clone();
    let host_wave: GameEngineHost = host.clone();
    let host_left: GameEngineHost = host.clone();
    let host_up: GameEngineHost = host.clone();
    let host_down: GameEngineHost = host.clone();
    let host_right: GameEngineHost = host.clone();
    let host_dash: GameEngineHost = host.clone();
    let viewport: NodeId = host.mount_viewport(
        Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(340.0))
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md()),
    );
    let top_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_STEP_PHYSICS.to_string(), tokens)
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: u32 = host_btn.step(0.051);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Spawn Enemy Wave".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    host_wave.ingest_event(&WindowEvent::KeyDown(
                        WindowId::new(1),
                        KeyEvent::new(KeyCode::Character('r'), Modifiers::default(), false),
                    ));
                    let _: u32 = host_wave.step(1.0 / 60.0);
                    host_wave.ingest_event(&WindowEvent::KeyUp(
                        WindowId::new(1),
                        KeyEvent::new(KeyCode::Character('r'), Modifiers::default(), false),
                    ));
                })
                .mount_current(),
        )
        .mount_current();
    let fs_2d_open: Signal<bool> = Signal::new(false);
    let fs_2d_portal: NodeId =
        mount_game_fullscreen_portal(fs_2d_open, &host, "2D Physics Arena", tokens);
    let touch_controls: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Left (A)".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    pulse_game_key(&host_left, KeyCode::ArrowLeft);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Up (W)".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    pulse_game_key(&host_up, KeyCode::ArrowUp);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Down (S)".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    pulse_game_key(&host_down, KeyCode::ArrowDown);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Right (D)".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    pulse_game_key(&host_right, KeyCode::ArrowRight);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Dash (E)".to_string(), tokens)
                .secondary()
                .sm()
                .on_click(move |_: &UiEvent| {
                    pulse_game_key(&host_dash, KeyCode::Character('e'));
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Fullscreen".to_string(), tokens)
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = fs_2d_open.set_if_changed(true);
                })
                .mount_current(),
        )
        .mount_current();
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            { top_actions }
            { viewport }
            { touch_controls }
            { fs_2d_portal }
        }
    };
    let card: NodeId = EuvCard::new(GAME_CARD_TITLE.to_string(), tokens)
        .subtitle(GAME_CARD_SUB)
        .child(body)
        .mount_current();
    EuvPage::new(GAME_TITLE.to_string(), GAME_SUBTITLE.to_string(), tokens)
        .section(card)
        .mount_current()
}

/// Mounts a reusable fullscreen portal overlay hosting any 2D/3D `GameEngineHost` with safe exit & Back support.
///
/// # Arguments
///
/// - `Signal<bool>` - Fullscreen visibility toggle signal.
/// - `&GameEngineHost` - Target game engine host.
/// - `&str` - Title shown in the fullscreen HUD bar.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted fullscreen portal handle.
fn mount_game_fullscreen_portal(
    open: Signal<bool>,
    host: &GameEngineHost,
    title: &str,
    tokens: ThemeTokens,
) -> NodeId {
    let mut ctx: BuildContext = current_build_context();
    let portal: NodeId = ctx.portal(
        Style::empty()
            .column()
            .display(Display::None)
            .position(Position::Fixed)
            .inset(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .background(Color::hex(0x09090B))
            .stop_scroll_propagation(true)
            .z_index(Z_INDEX_MODAL),
    );
    let top_bar: NodeId = ctx.view(
        Style::empty()
            .row()
            .width(Length::Percent(100.0))
            .justify(JustifyContent::SpaceBetween)
            .align(AlignItems::Center)
            .padding(Edges::symmetric(SPACE_SM, SPACE_MD))
            .background(Color::rgba(9, 9, 11, 220)),
    );
    let title_n: NodeId = ctx.text(
        format!("{title} · Fullscreen"),
        Style::empty()
            .font_size(14.0)
            .font_weight(FontWeight::SemiBold)
            .color(Color::hex(0xFAFAFA)),
    );
    let exit_btn: NodeId = EuvButton::new("Exit Fullscreen".to_string(), tokens)
        .secondary()
        .sm()
        .on_click(move |_: &UiEvent| {
            let _: bool = open.set_if_changed(false);
        })
        .mount(&mut ctx);
    ctx.append(top_bar, title_n);
    ctx.append(top_bar, exit_btn);
    let fs_viewport: NodeId = host.mount_viewport(
        Style::empty()
            .flex_grow(1.0)
            .width(Length::Percent(100.0))
            .height(Length::Percent(100.0))
            .stop_scroll_propagation(true)
            .stop_event_propagation(true),
    );
    ctx.append(portal, top_bar);
    ctx.append(portal, fs_viewport);
    let handle: SceneHandle = ctx.get_scene().clone();
    let safe_sig: Signal<SafeAreaInsets> = ctx.get_safe_area();
    let prev_fs: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        let _: Effect = EuvOverlayStack::bind_signal(open, "GameFullscreenPortal", Z_INDEX_MODAL);
        Effect::new(move || {
            let is_open: bool = open.get();
            let safe: SafeAreaInsets = safe_sig.get();
            if is_open != prev_fs.replace(is_open) {
                let _: bool = dispatch_native_action(
                    NATIVE_ACTION_FULLSCREEN,
                    if is_open { "on" } else { "off" },
                    "",
                );
            }
            handle.with_write(
                |tree: &mut SceneTree| {
                    if let Some(n) = tree.get_node_mut(portal) {
                        let mut st: Style = n.get_style();
                        st.get_mut_layout().set_display(if is_open {
                            Display::Flex
                        } else {
                            Display::None
                        });
                        n.set_style(st);
                    }
                    if let Some(n) = tree.get_node_mut(top_bar) {
                        let mut st: Style = n.get_style();
                        st.get_mut_layout().set_padding(Edges::new(
                            SPACE_SM + safe.get_top(),
                            SPACE_MD + safe.get_right(),
                            SPACE_SM,
                            SPACE_MD + safe.get_left(),
                        ));
                        n.set_style(st);
                    }
                    let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                    tree.mark_dirty(portal, dirty_l | DIRTY_PAINT);
                },
                (),
            );
        })
    });
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(portal, scope);
        },
        (),
    );
    portal
}

/// Mounts a press-and-hold cross-platform touch/mouse game control button driving `code` on `host`.
///
/// # Arguments
///
/// - `&str` - Button label.
/// - `&GameEngineHost` - Target game host.
/// - `KeyCode` - Virtual key bound to this button.
/// - `bool` - Whether to style as primary (`true`) or outline (`false`).
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted holdable button node.
fn mount_game_control_button(
    label: &str,
    host: &GameEngineHost,
    code: KeyCode,
    primary: bool,
    tokens: ThemeTokens,
) -> NodeId {
    let host_down: GameEngineHost = host.clone();
    let host_up: GameEngineHost = host.clone();
    let host_click: GameEngineHost = host.clone();
    let mut btn: EuvButton =
        EuvButton::new(label.to_string(), tokens)
            .sm()
            .on_click(move |_: &UiEvent| {
                pulse_game_key(&host_click, code);
            });
    if !primary {
        btn = btn.outline();
    }
    let node: NodeId = btn.mount_current();
    let mut ctx: BuildContext = current_build_context();
    ctx.on_pointer_down(node, move |_: &UiEvent| {
        host_down.press_key(code);
    });
    ctx.on_pointer_up(node, move |_: &UiEvent| {
        host_up.release_key(code);
    });
    node
}

/// Pulses a single directional or action key on `host` for mobile touch/button game controls.
///
/// # Arguments
///
/// - `&GameEngineHost` - Target game engine host.
/// - `KeyCode` - Virtual key code to pulse.
fn pulse_game_key(host: &GameEngineHost, code: KeyCode) {
    host.press_key(code);
    let _: u32 = host.step(0.08);
    host.release_key(code);
}

/// Mounts the AAA 3D Open-World Engine page with live BVH ray tracing, runtime graphics settings, and `.obj`/`.gltf` model import.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted 3D engine showcase page handle.
pub fn mount_game3d_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let g: ThemeGeometry = tokens.get_geometry();
    let p: ThemePalette = tokens.get_palette();
    let host: GameEngineHost = state.get_game3d_host().clone();
    let quality_sig: Signal<RenderQualityConfig> = state.get_engine3d_quality();
    let preset_idx: Signal<usize> = state.get_engine3d_preset_idx();
    let rt_idx: Signal<usize> = state.get_engine3d_rt_idx();
    let res_scale: Signal<f32> = state.get_engine3d_res_scale();
    let time_sig: Signal<f32> = state.get_engine3d_time_of_day();
    let import_sig: Signal<u64> = state.get_engine3d_import_queue();
    let status_sig: Signal<String> = state.get_engine3d_import_status();
    let cam_mode_sig: Signal<u8> = state.get_engine3d_camera_mode();
    let viewport: NodeId = host.mount_viewport(
        Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(340.0))
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md()),
    );
    let fs_3d_open: Signal<bool> = Signal::new(false);
    let fs_3d_portal: NodeId =
        mount_game_fullscreen_portal(fs_3d_open, &host, "Vice Boulevard 3D RT", tokens);
    let drive_controls: NodeId = EuvActionRow::new()
        .child(mount_game_control_button(
            "Steer Left (A)",
            &host,
            KeyCode::ArrowLeft,
            false,
            tokens,
        ))
        .child(mount_game_control_button(
            "Drive Fwd (W)",
            &host,
            KeyCode::ArrowUp,
            true,
            tokens,
        ))
        .child(mount_game_control_button(
            "Reverse (S)",
            &host,
            KeyCode::ArrowDown,
            false,
            tokens,
        ))
        .child(mount_game_control_button(
            "Steer Right (D)",
            &host,
            KeyCode::ArrowRight,
            false,
            tokens,
        ))
        .child(
            EuvButton::new("Switch Camera".to_string(), tokens)
                .secondary()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let next: u8 = (cam_mode_sig.get_untracked() + 1) % 4;
                    let _: bool = cam_mode_sig.set_if_changed(next);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Fullscreen 3D".to_string(), tokens)
                .sm()
                .on_click(move |_: &UiEvent| {
                    let _: bool = fs_3d_open.set_if_changed(true);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Import .OBJ Monument".to_string(), tokens)
                .secondary()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let cur: u64 = import_sig.get_untracked();
                    let next: u64 = if cur.is_multiple_of(2) {
                        cur + 1
                    } else {
                        cur + 2
                    };
                    let _: bool = import_sig.set_if_changed(next);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Import .glTF Tower".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let cur: u64 = import_sig.get_untracked();
                    let next: u64 = if !cur.is_multiple_of(2) {
                        cur + 1
                    } else {
                        cur + 2
                    };
                    let _: bool = import_sig.set_if_changed(next);
                })
                .mount_current(),
        )
        .mount_current();
    let mut ctx: BuildContext = current_build_context();
    let sync_host: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_MD),
    );
    let (sync_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            let p_idx: usize = preset_idx.get();
            let preset: QualityPreset = match p_idx {
                0 => QualityPreset::Low,
                1 => QualityPreset::Medium,
                3 => QualityPreset::Ultra,
                4 => QualityPreset::Cinematic,
                _ => QualityPreset::High,
            };
            let rt: RayTracingMode = match rt_idx.get() {
                0 => RayTracingMode::Off,
                1 => RayTracingMode::ShadowsOnly,
                3 => RayTracingMode::FullPathTracedGi,
                _ => RayTracingMode::ShadowsAndReflections,
            };
            let scale_pct: f32 = res_scale.get();
            let mut cfg: RenderQualityConfig = RenderQualityConfig::from_preset(preset);
            cfg.set_ray_tracing(rt);
            cfg.set_resolution_scale((scale_pct / 100.0).clamp(0.40, 1.25));
            let _: bool = quality_sig.set_if_changed(cfg);
        })
    });
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(sync_host, sync_scope);
        },
        (),
    );
    let preset_seg: NodeId = EuvSegmented::new(
        vec![
            "Low".to_string(),
            "Medium".to_string(),
            "High".to_string(),
            "Ultra".to_string(),
            "Cinematic".to_string(),
        ],
        preset_idx,
        tokens,
    )
    .mount_current();
    let rt_seg: NodeId = EuvSegmented::new(
        vec![
            "RT Off".to_string(),
            "RT Shadows".to_string(),
            "RT Refl".to_string(),
            "RT Full GI".to_string(),
        ],
        rt_idx,
        tokens,
    )
    .mount_current();
    let res_field: NodeId = EuvField::new(
        "Runtime Render Resolution Scale (%)".to_string(),
        EuvSlider::new(res_scale, 40.0, 125.0, tokens).mount_current(),
        tokens,
    )
    .mount_current();
    let sun_field: NodeId = EuvField::new(
        "Time of Day / Sun Lighting (6h Dawn -> 17h Sunset -> 22h Night)".to_string(),
        EuvSlider::new(time_sig, 6.0, 23.0, tokens).mount_current(),
        tokens,
    )
    .mount_current();
    let status_txt: NodeId = ctx.reactive_text(
        Style::empty()
            .font_size(12.0)
            .color(p.get_muted_foreground()),
        move || status_sig.get(),
    );
    ctx.append(sync_host, viewport);
    ctx.append(sync_host, drive_controls);
    ctx.append(sync_host, status_txt);
    ctx.append(sync_host, preset_seg);
    ctx.append(sync_host, rt_seg);
    ctx.append(sync_host, res_field);
    ctx.append(sync_host, sun_field);
    ctx.append(sync_host, fs_3d_portal);
    let card: NodeId = EuvCard::new(GAME3D_CARD_TITLE.to_string(), tokens)
        .subtitle(GAME3D_CARD_SUB)
        .child(sync_host)
        .mount_current();
    EuvPage::new(
        GAME3D_TITLE.to_string(),
        GAME3D_SUBTITLE.to_string(),
        tokens,
    )
    .section(card)
    .mount_current()
}

/// Mounts the overlays page featuring interactive `EuvModal`, `EuvDrawer`, `EuvSheet`, `EuvPopover`, `EuvDropdown`, `EuvToast`, `EuvNotification`, and `EuvLoading`.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted overlays page node.
pub fn mount_overlays_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let modal_open: Signal<bool> = state.get_modal_open();
    let drawer_open: Signal<bool> = state.get_drawer_open();
    let sheet_open: Signal<bool> = state.get_sheet_open();
    let toast_visible: Signal<bool> = state.get_toast_visible();
    let notif_visible: Signal<bool> = state.get_notif_visible();
    let mut toast_btn: EuvButton =
        EuvButton::new(BTN_TOGGLE_TOAST.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = toast_visible.update(|v: &mut bool| *v = !*v);
        });
    toast_btn.set_variant(EuvButtonVariant::Secondary);
    let mut drawer_btn: EuvButton =
        EuvButton::new("Toggle Drawer".to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = drawer_open.update(|v: &mut bool| *v = !*v);
        });
    drawer_btn.set_variant(EuvButtonVariant::Outline);
    let mut sheet_btn: EuvButton =
        EuvButton::new("Toggle Sheet".to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = sheet_open.update(|v: &mut bool| *v = !*v);
        });
    sheet_btn.set_variant(EuvButtonVariant::Outline);
    let mut notif_btn: EuvButton = EuvButton::new("Toggle Notification".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: bool = notif_visible.update(|v: &mut bool| *v = !*v);
        });
    notif_btn.set_variant(EuvButtonVariant::Outline);
    let mut close_btn: EuvButton =
        EuvButton::new(BTN_CLOSE_MODAL.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = modal_open.set_if_changed(false);
        });
    close_btn.set_variant(EuvButtonVariant::Outline);
    let alert_open: Signal<bool> = state.get_alert_dialog_open();
    let mut alert_btn: EuvButton =
        EuvButton::new("Alert Dialog".to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = alert_open.set_if_changed(true);
        });
    alert_btn.set_variant(EuvButtonVariant::Destructive);
    let actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_OPEN_MODAL.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let _: bool = modal_open.set_if_changed(true);
                })
                .mount_current(),
        )
        .child(alert_btn.mount_current())
        .child(toast_btn.mount_current())
        .child(drawer_btn.mount_current())
        .child(sheet_btn.mount_current())
        .child(notif_btn.mount_current())
        .mount_current();
    let dropdown: NodeId = EuvDropdown::new(
        "Quick Actions".to_string(),
        vec!["Export PNG".to_string(), "Copy DeepLink".to_string()],
        tokens,
    )
    .open(state.get_dropdown_open())
    .selected(state.get_dropdown_selected())
    .mount_current();
    let hover_card: NodeId = EuvHoverCard::new(
        "@euv-dev".to_string(),
        "euv 0.38 Native Engine".to_string(),
        tokens,
    )
    .open(state.get_hover_card_open())
    .description("Full ui.shadcn.com component alignment across all 7 native targets.")
    .mount_current();
    let tip_btn: NodeId = EuvButton::new("Hover Tooltip".to_string(), tokens)
        .outline()
        .sm()
        .mount_current();
    let tooltip_demo: NodeId =
        EuvTooltip::new("Zero-WebView floating tooltip".to_string(), tip_btn, tokens)
            .mount_current();
    let ctx_sel: Signal<Option<usize>> = Signal::new(None);
    let ctx_menu_demo: NodeId = EuvContextMenu::new(
        "Context Menu Surface".to_string(),
        vec![
            "Inspect Scene Node".to_string(),
            "Copy AI Selector".to_string(),
            "Capture Frame Hash".to_string(),
        ],
        tokens,
    )
    .selected(ctx_sel)
    .mount_current();
    let mut card: EuvCard = EuvCard::new(OVERLAYS_CARD_TITLE.to_string(), tokens)
        .child(actions)
        .child(dropdown)
        .child(hover_card)
        .child(tooltip_demo)
        .child(ctx_menu_demo);
    card.set_subtitle(OVERLAYS_CARD_SUB.to_string());
    let modal_body: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_MD
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                { MODAL_BODY_TEXT }
            }
            row {
                width: Length::Percent(100.0)
                justify: JustifyContent::End
                { close_btn }
            }
        }
    };
    let mut close_drawer_btn: EuvButton = EuvButton::new("Close Drawer".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: bool = drawer_open.set_if_changed(false);
        });
    close_drawer_btn.set_variant(EuvButtonVariant::Outline);
    let drawer_body: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_MD
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                "Native slide-over drawer surface"
            }
            row {
                width: Length::Percent(100.0)
                justify: JustifyContent::End
                { close_drawer_btn }
            }
        }
    };
    let mut close_sheet_btn: EuvButton = EuvButton::new("Close Sheet".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: bool = sheet_open.set_if_changed(false);
        });
    close_sheet_btn.set_variant(EuvButtonVariant::Outline);
    let sheet_body: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_MD
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                "Native bottom action sheet"
            }
            row {
                width: Length::Percent(100.0)
                justify: JustifyContent::End
                { close_sheet_btn }
            }
        }
    };
    let overlays_wrap: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            gap: SPACE_LG
            { EuvAlert::new(ALERT_TITLE.to_string(), ALERT_DESC.to_string(), EuvTone::Primary, tokens) }
            { card }
            { EuvModal::new(modal_open, MODAL_TITLE.to_string(), modal_body, tokens) }
            { EuvAlertDialog::new(alert_open, "Are you absolutely sure?".to_string(), state.get_alert_dialog_confirmed(), tokens).description("This shadcn AlertDialog confirms destructive operations cleanly.").labels("Cancel", "Continue") }
            { EuvDrawer::new(drawer_open, "Inspector Drawer".to_string(), drawer_body, tokens) }
            { EuvSheet::new(sheet_open, "Quick Sheet".to_string(), sheet_body, tokens) }
            { EuvToast::new(toast_visible, TOAST_TEXT.to_string(), EuvTone::Success, tokens) }
            { EuvNotification::new(notif_visible, "Native Sync".to_string(), "All 7 targets active.".to_string(), tokens).tone(EuvTone::Success) }
            { EuvLoading::new(state.get_loading_active(), "Syncing...".to_string(), tokens) }
        }
    };
    EuvPage::new(
        OVERLAYS_TITLE.to_string(),
        OVERLAYS_SUBTITLE.to_string(),
        tokens,
    )
    .section(overlays_wrap)
    .mount_current()
}

/// Mounts the interactive component-level and page-level dynamic OTA updates showcase page.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted dynamic updates page node.
pub fn mount_dynamic_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let hub: DynamicUpdateHub = state.get_dynamic_hub().clone();
    let sibling_counter: Signal<i32> = state.get_sibling_counter();
    let action_log: Signal<String> = state.get_dynamic_action_log();
    let route_sig: Signal<String> = state.get_route();
    let tel_sig: Signal<DynamicTelemetry> = hub.telemetry();
    let banner_rev: Signal<u64> = hub.component_revision_signal(SLOT_PROMO_BANNER);
    let metrics_rev: Signal<u64> = hub.component_revision_signal(SLOT_METRICS_WIDGET);
    let panel_rev: Signal<u64> = hub.page_revision_signal(ROUTE_DYNAMIC_PANEL);
    let hub_v1: DynamicUpdateHub = hub.clone();
    let hub_v2: DynamicUpdateHub = hub.clone();
    let hub_rb: DynamicUpdateHub = hub.clone();
    let hub_m1: DynamicUpdateHub = hub.clone();
    let hub_mrb: DynamicUpdateHub = hub.clone();
    let hub_p1: DynamicUpdateHub = hub.clone();
    let hub_bundle: DynamicUpdateHub = hub.clone();
    let hub_prb: DynamicUpdateHub = hub.clone();
    let hub_inject: DynamicUpdateHub = hub.clone();
    let hub_for_status: DynamicUpdateHub = hub.clone();
    let mut btn_banner_v2: EuvButton =
        EuvButton::new(BTN_PUSH_BANNER_V2.to_string(), tokens).on_click(move |_: &UiEvent| {
            let next_v: u64 = hub_v2.component_version(SLOT_PROMO_BANNER).saturating_add(1).max(2);
            let hub_act: DynamicUpdateHub = hub_v2.clone();
            let _: Result<u64, DynamicUpdateError> = hub_v2.swap_component_builder(
                SLOT_PROMO_BANNER.to_string(),
                next_v,
                move || {
                    let hub_click: DynamicUpdateHub = hub_act.clone();
                    view! {
                        row {
                            width: Length::Percent(100.0)
                            flex_wrap: FlexWrap::Wrap
                            padding: Edges::all(SPACE_MD)
                            gap: SPACE_MD
                            align: AlignItems::Center
                            background: Color::hex(0x064E3B)
                            radius: 8.0
                            text {
                                font_size: 14.0
                                font_weight: FontWeight::Bold
                                color: Color::hex(0xECFDF5)
                                { format!("Native Hot-Swapped Banner v{next_v} (Zero Restart)") }
                            }
                            { EuvButton::new(BTN_CLAIM_OFFER.to_string(), tokens).on_click(move |_: &UiEvent| {
                                hub_click.dispatch_action(DynamicActionEvent::new(
                                    SLOT_PROMO_BANNER.to_string(),
                                    "claim_offer".to_string(),
                                    format!("native-v{next_v}"),
                                ));
                            }) }
                        }
                    }
                },
            );
        });
    btn_banner_v2.set_variant(EuvButtonVariant::Secondary);
    let mut btn_banner_rb: EuvButton = EuvButton::new(BTN_ROLLBACK_BANNER.to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: Result<u64, DynamicUpdateError> = hub_rb.rollback_component(SLOT_PROMO_BANNER);
        });
    btn_banner_rb.set_variant(EuvButtonVariant::Outline);
    let mut btn_metrics_rb: EuvButton = EuvButton::new(BTN_ROLLBACK_METRICS.to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: Result<u64, DynamicUpdateError> =
                hub_mrb.rollback_component(SLOT_METRICS_WIDGET);
        });
    btn_metrics_rb.set_variant(EuvButtonVariant::Outline);
    let mut btn_sibling: EuvButton =
        EuvButton::new(BTN_INC_SIBLING.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = sibling_counter.update(|v: &mut i32| *v += 1);
        });
    btn_sibling.set_variant(EuvButtonVariant::Outline);
    let sibling_btn_row: NodeId = EuvActionRow::new()
        .child(btn_sibling.mount_current())
        .mount_current();
    let banner_btn_row: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_PUSH_BANNER_V1.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let next_v: u64 = hub_v1
                        .component_version(SLOT_PROMO_BANNER)
                        .saturating_add(1);
                    let mut badge: DynamicNodeSpec =
                        DynamicNodeSpec::badge(format!("OTA v{next_v}"));
                    badge
                        .get_mut_style()
                        .set_background_hex(Some(0x1D4ED8))
                        .set_color_hex(Some(0xFFFFFF));
                    let mut title: DynamicNodeSpec = DynamicNodeSpec::text(format!(
                        "Dynamic JSON Banner v{next_v}: Surgical Component Patch"
                    ));
                    title
                        .get_mut_style()
                        .set_bold(true)
                        .set_color_hex(Some(0x1E3A8A));
                    let mut offer_btn: DynamicNodeSpec = DynamicNodeSpec::button(
                        BTN_CLAIM_OFFER.to_string(),
                        "claim_offer".to_string(),
                    );
                    offer_btn.set_payload(format!("json-v{next_v}"));
                    let mut box_spec: DynamicNodeSpec = DynamicNodeSpec::row()
                        .child(badge)
                        .child(title)
                        .child(offer_btn);
                    box_spec
                        .get_mut_style()
                        .set_full_width(true)
                        .set_padding(Some(12.0))
                        .set_radius(Some(8.0))
                        .set_background_hex(Some(0xEFF6FF))
                        .set_border_width(Some(1.0))
                        .set_border_hex(Some(0x93C5FD));
                    let mut upd: DynamicComponentUpdate = DynamicComponentUpdate::new(
                        SLOT_PROMO_BANNER.to_string(),
                        next_v,
                        box_spec,
                    );
                    upd.sign();
                    if let Ok(json) = upd.to_json() {
                        let _: Result<u64, DynamicUpdateError> = hub_v1.apply_component_json(&json);
                    }
                })
                .mount_current(),
        )
        .child(btn_banner_v2.mount_current())
        .child(btn_banner_rb.mount_current())
        .mount_current();
    let metrics_btn_row: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_PATCH_METRICS.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let next_v: u64 = hub_m1.component_version(SLOT_METRICS_WIDGET).saturating_add(1);
                    let mut card_spec: DynamicNodeSpec = DynamicNodeSpec::card(format!("Live Metrics Widget v{next_v}"))
                        .child(DynamicNodeSpec::text(format!("Updated independently without touching promo_banner or sibling state (rev {next_v}).")));
                    card_spec
                        .get_mut_style()
                        .set_background_hex(Some(0xFEFCE8))
                        .set_border_width(Some(1.0))
                        .set_border_hex(Some(0xFACC15));
                    let mut upd: DynamicComponentUpdate = DynamicComponentUpdate::new(
                        SLOT_METRICS_WIDGET.to_string(),
                        next_v,
                        card_spec,
                    );
                    upd.sign();
                    let _: Result<u64, DynamicUpdateError> = hub_m1.apply_component(upd);
                })
                .mount_current(),
        )
        .child(btn_metrics_rb.mount_current())
        .mount_current();
    let comp_controls: NodeId = view! {
        column {
            gap: SPACE_MD
            { sibling_btn_row }
            text {
                font_size: 13.0
                font_weight: FontWeight::SemiBold
                color: p.get_primary()
                { move || format!("Sibling State Preserved: count={}", sibling_counter.get()) }
            }
            { banner_btn_row }
            slot {
                id: SLOT_PROMO_BANNER
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    text {
                        font_size: 13.0
                        color: p.get_foreground()
                        "Compiled Fallback Banner (v0) — click a button above to hot-update only this component."
                    }
                }
            }
            { metrics_btn_row }
            slot {
                id: SLOT_METRICS_WIDGET
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    text {
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        "Compiled Fallback Metrics Widget (v0) — independent surgical slot."
                    }
                }
            }
        }
    };
    let mut comp_card: EuvCard =
        EuvCard::new(DYNAMIC_COMP_CARD_TITLE.to_string(), tokens).child(comp_controls);
    comp_card.set_subtitle(DYNAMIC_COMP_CARD_SUB.to_string());
    let mut btn_bundle: EuvButton =
        EuvButton::new(BTN_APPLY_BUNDLE.to_string(), tokens).on_click(move |_: &UiEvent| {
            let next_comp_v: u64 = hub_bundle
                .component_version(SLOT_METRICS_WIDGET)
                .saturating_add(1);
            let next_page_v: u64 = hub_bundle
                .page_version(ROUTE_DYNAMIC_PANEL)
                .saturating_add(1);
            let mut comp_upd: DynamicComponentUpdate = DynamicComponentUpdate::new(
                SLOT_METRICS_WIDGET.to_string(),
                next_comp_v,
                DynamicNodeSpec::card(format!("Atomic Bundle Metrics v{next_comp_v}")).child(
                    DynamicNodeSpec::text(
                        "Committed together with page panel inside one atomic transaction."
                            .to_string(),
                    ),
                ),
            );
            comp_upd.sign();
            let mut page_upd: DynamicPageUpdate = DynamicPageUpdate::new(
                ROUTE_DYNAMIC_PANEL.to_string(),
                format!("Atomic Panel v{next_page_v}"),
                next_page_v,
                DynamicNodeSpec::card(format!("Atomic Bundle Page Panel v{next_page_v}")).child(
                    DynamicNodeSpec::text(
                        "Both component slot and page slot updated atomically via signed JSON."
                            .to_string(),
                    ),
                ),
            );
            page_upd.sign();
            let bundle: DynamicUpdateBundle =
                DynamicUpdateBundle::new("ota-atomic-bundle".to_string(), next_page_v)
                    .push_component(comp_upd)
                    .push_page(page_upd);
            if let Ok(json) = bundle.to_json() {
                let _: Result<DynamicTelemetry, DynamicUpdateError> =
                    hub_bundle.apply_bundle_json(&json);
            }
        });
    btn_bundle.set_variant(EuvButtonVariant::Secondary);
    let mut btn_page_rb: EuvButton = EuvButton::new(BTN_ROLLBACK_PAGE.to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: Result<u64, DynamicUpdateError> = hub_prb.rollback_page(ROUTE_DYNAMIC_PANEL);
        });
    btn_page_rb.set_variant(EuvButtonVariant::Outline);
    let mut btn_inject: EuvButton =
        EuvButton::new(BTN_INJECT_ROUTE.to_string(), tokens).on_click(move |_: &UiEvent| {
            let next_v: u64 = hub_inject.page_version(ROUTE_PROMO_LIVE).saturating_add(1);
            let mut back_btn: DynamicNodeSpec =
                DynamicNodeSpec::button(BTN_RETURN_DYNAMIC.to_string(), "return_dynamic".to_string());
            back_btn.set_payload(format!("from-promo-v{next_v}"));
            let page_spec: DynamicNodeSpec = DynamicNodeSpec::column()
                .child(
                    DynamicNodeSpec::card(format!("Runtime-Injected Page /promo-live (v{next_v})"))
                        .child(DynamicNodeSpec::badge("Zero Recompile Route".to_string()))
                        .child(DynamicNodeSpec::text(
                            "This entire route was injected dynamically at runtime via signed JSON manifest."
                                .to_string(),
                        ))
                        .child(back_btn),
                );
            let mut upd: DynamicPageUpdate = DynamicPageUpdate::new(
                ROUTE_PROMO_LIVE.to_string(),
                "Flash Promo (OTA)".to_string(),
                next_v,
                page_spec,
            );
            upd.sign();
            if let Ok(json) = upd.to_json()
                && hub_inject.apply_page_json(&json).is_ok()
            {
                let _: bool = route_sig.set_if_changed(ROUTE_PROMO_LIVE.to_string());
            }
        });
    btn_inject.set_variant(EuvButtonVariant::Secondary);
    let page_btn_row: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new(BTN_UPDATE_PAGE_V1.to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let next_v: u64 = hub_p1.page_version(ROUTE_DYNAMIC_PANEL).saturating_add(1);
                    let spec: DynamicNodeSpec = DynamicNodeSpec::card(format!(
                        "Hot-Updated Dynamic Page Panel v{next_v}"
                    ))
                    .child(DynamicNodeSpec::badge(format!("Page Revision v{next_v}")))
                    .child(DynamicNodeSpec::text(
                        "Page-level layout replaced live without restarting the application."
                            .to_string(),
                    ));
                    let mut upd: DynamicPageUpdate = DynamicPageUpdate::new(
                        ROUTE_DYNAMIC_PANEL.to_string(),
                        format!("Dynamic Panel v{next_v}"),
                        next_v,
                        spec,
                    );
                    upd.sign();
                    let _: Result<u64, DynamicUpdateError> = hub_p1.apply_page(upd);
                })
                .mount_current(),
        )
        .child(btn_bundle.mount_current())
        .child(btn_page_rb.mount_current())
        .child(btn_inject.mount_current())
        .mount_current();
    let page_controls: NodeId = view! {
        column {
            gap: SPACE_MD
            { page_btn_row }
            page_slot {
                route: ROUTE_DYNAMIC_PANEL
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    text {
                        font_size: 13.0
                        color: p.get_foreground()
                        "Compiled Fallback Page Panel (v0) — click Hot-Update Page or Apply Atomic Bundle."
                    }
                }
            }
            text {
                font_size: 12.0
                color: p.get_muted_foreground()
                { move || {
                    let _: u64 = banner_rev.get();
                    let _: u64 = metrics_rev.get();
                    let _: u64 = panel_rev.get();
                    let tel: DynamicTelemetry = tel_sig.get();
                    format!(
                        "Versions: banner=v{} metrics=v{} panel=v{} | Applied: comp={} page={} rollbacks={} rejected={} | Last Action: {}",
                        hub_for_status.component_version(SLOT_PROMO_BANNER),
                        hub_for_status.component_version(SLOT_METRICS_WIDGET),
                        hub_for_status.page_version(ROUTE_DYNAMIC_PANEL),
                        tel.get_components_applied(),
                        tel.get_pages_applied(),
                        tel.get_rollbacks(),
                        tel.get_rejected(),
                        action_log.get(),
                    )
                } }
            }
        }
    };
    let mut page_card: EuvCard =
        EuvCard::new(DYNAMIC_PAGE_CARD_TITLE.to_string(), tokens).child(page_controls);
    page_card.set_subtitle(DYNAMIC_PAGE_CARD_SUB.to_string());
    let wasm_mgr: WasmDynamicManager = state.get_wasm_manager().clone();
    let wasm_reg: RegistryWasmFetcher = state.get_wasm_registry().clone();
    let wasm_offline_sig: Signal<bool> = state.get_wasm_offline();
    let wasm_status_sig: Signal<String> = state.get_wasm_fault_status();
    let partial_scope: WasmDynamicScope = WasmDynamicScope::Partial {
        slot_id: SLOT_WASM_WIDGET.to_string(),
    };
    let partial_state_sig: Signal<WasmLoadState> = wasm_mgr.state_signal(&partial_scope);
    let global_state_sig: Signal<WasmLoadState> = wasm_mgr.state_signal(&WasmDynamicScope::Global);
    let wasm_tel_sig: Signal<WasmDynamicTelemetry> = wasm_mgr.telemetry();
    let m_p1: WasmDynamicManager = wasm_mgr.clone();
    let s_p1: Signal<String> = wasm_status_sig;
    let m_p2: WasmDynamicManager = wasm_mgr.clone();
    let s_p2: Signal<String> = wasm_status_sig;
    let m_prb: WasmDynamicManager = wasm_mgr.clone();
    let s_prb: Signal<String> = wasm_status_sig;
    let m_pun: WasmDynamicManager = wasm_mgr.clone();
    let s_pun: Signal<String> = wasm_status_sig;
    let r_off: RegistryWasmFetcher = wasm_reg.clone();
    let s_off: Signal<String> = wasm_status_sig;
    let m_g1: WasmDynamicManager = wasm_mgr.clone();
    let s_g1: Signal<String> = wasm_status_sig;
    let m_g2: WasmDynamicManager = wasm_mgr.clone();
    let s_g2: Signal<String> = wasm_status_sig;
    let m_gbad: WasmDynamicManager = wasm_mgr.clone();
    let s_gbad: Signal<String> = wasm_status_sig;
    let m_grb: WasmDynamicManager = wasm_mgr.clone();
    let s_grb: Signal<String> = wasm_status_sig;
    let m_f_retry: WasmDynamicManager = wasm_mgr.clone();
    let s_f_retry: Signal<String> = wasm_status_sig;
    let m_f_500: WasmDynamicManager = wasm_mgr.clone();
    let s_f_500: Signal<String> = wasm_status_sig;
    let m_f_corrupt: WasmDynamicManager = wasm_mgr.clone();
    let s_f_corrupt: Signal<String> = wasm_status_sig;
    let m_f_tamper: WasmDynamicManager = wasm_mgr.clone();
    let s_f_tamper: Signal<String> = wasm_status_sig;
    let m_f_sig: WasmDynamicManager = wasm_mgr.clone();
    let s_f_sig: Signal<String> = wasm_status_sig;
    let m_f_abi: WasmDynamicManager = wasm_mgr.clone();
    let s_f_abi: Signal<String> = wasm_status_sig;
    let m_f_exp: WasmDynamicManager = wasm_mgr.clone();
    let s_f_exp: Signal<String> = wasm_status_sig;
    let m_f_fuel: WasmDynamicManager = wasm_mgr.clone();
    let s_f_fuel: Signal<String> = wasm_status_sig;
    let m_f_trap: WasmDynamicManager = wasm_mgr.clone();
    let s_f_trap: Signal<String> = wasm_status_sig;
    let m_f_big: WasmDynamicManager = wasm_mgr.clone();
    let s_f_big: Signal<String> = wasm_status_sig;
    let m_f_cancel: WasmDynamicManager = wasm_mgr.clone();
    let s_f_cancel: Signal<String> = wasm_status_sig;
    let m_http: WasmDynamicManager = wasm_mgr.clone();
    let s_http: Signal<String> = wasm_status_sig;
    let wasm_partial_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Download Real HTTP .wasm".to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let bytes: Vec<u8> = build_partial_widget_wasm(
                            "Live HTTP-Downloaded .wasm Module v3",
                            3,
                            0x059669,
                        );
                        let bytes_srv: Vec<u8> = bytes.clone();
                        match TcpListener::bind("127.0.0.1:0") {
                            Ok(listener) => {
                                let port: u16 = listener
                                    .local_addr()
                                    .map(|addr: SocketAddr| addr.port())
                                    .unwrap_or(0);
                                drop(spawn(move || {
                                    if let Ok((mut stream, _)) = listener.accept() {
                                        let mut req_buf: [u8; 512] = [0; 512];
                                        let _: Result<usize, Error> = stream.read(&mut req_buf);
                                        let hdr: String = format!(
                                            "HTTP/1.1 200 OK\r\nContent-Type: application/wasm\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                            bytes_srv.len()
                                        );
                                        let _: Result<(), Error> = stream.write_all(hdr.as_bytes());
                                        let _: Result<(), Error> = stream.write_all(&bytes_srv);
                                        let _: Result<(), Error> = stream.flush();
                                    }
                                }));
                                let url: String =
                                    format!("http://127.0.0.1:{port}/live-module-v3.wasm");
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(
                                    url.clone(),
                                    SLOT_WASM_WIDGET.to_string(),
                                    3,
                                )
                                .with_checksum(compute_wasm_checksum(&bytes))
                                .with_signature(compute_wasm_signature(
                                    &bytes,
                                    3,
                                    WASM_SHOWCASE_SIGNING_KEY,
                                ))
                                .with_allow_downgrade(true);
                                match m_http.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                                    Ok(v) => {
                                        let _: bool = s_http.set(format!(
                                            "Downloaded {} bytes over real HTTP ({url}) -> executed & mounted .wasm v{v} natively!",
                                            bytes.len()
                                        ));
                                    }
                                    Err(e) => {
                                        let _: bool = s_http.set(format!("HTTP .wasm download failed: {e}"));
                                    }
                                }
                            }
                            Err(err) => {
                                let _: bool = s_http.set(format!("Local HTTP server bind error: {err}"));
                            }
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let bytes: Vec<u8> = build_partial_widget_wasm(TITLE_WASM_PARTIAL_V1, 1, 0x2563EB);
                        let req: WasmDownloadRequest = WasmDownloadRequest::partial(
                            URL_WASM_PARTIAL_V1.to_string(),
                            SLOT_WASM_WIDGET.to_string(),
                            1,
                        )
                        .with_checksum(compute_wasm_checksum(&bytes))
                        .with_signature(compute_wasm_signature(&bytes, 1, WASM_SHOWCASE_SIGNING_KEY))
                        .with_allow_downgrade(true);
                        match m_http.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                            Ok(v) => { let _: bool = s_http.set(format!("Fetched & executed .wasm v{v} natively in browser sandbox")); }
                            Err(e) => { let _: bool = s_http.set(format!("WASM fetch failed: {e}")); }
                        }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Download Partial WASM v1".to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let bytes: Vec<u8> = build_partial_widget_wasm(TITLE_WASM_PARTIAL_V1, 1, 0x2563EB);
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(
                        URL_WASM_PARTIAL_V1.to_string(),
                        SLOT_WASM_WIDGET.to_string(),
                        1,
                    )
                    .with_checksum(compute_wasm_checksum(&bytes))
                    .with_signature(compute_wasm_signature(&bytes, 1, WASM_SHOWCASE_SIGNING_KEY))
                    .with_allow_downgrade(true);
                    match m_p1.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                        Ok(v) => { let _: bool = s_p1.set(format!("Mounted Partial .wasm v{v} natively (verified checksum + signature)")); }
                        Err(e) => { let _: bool = s_p1.set(format!("Partial v1 failed: {e}")); }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Hot-Swap Partial WASM v2".to_string(), tokens)
                .secondary()
                .on_click(move |_: &UiEvent| {
                    let bytes: Vec<u8> = build_partial_widget_wasm("Hot-Swapped Partial WASM Widget v2", 2, 0x7C3AED);
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(
                        URL_WASM_PARTIAL_V2.to_string(),
                        SLOT_WASM_WIDGET.to_string(),
                        2,
                    )
                    .with_checksum(compute_wasm_checksum(&bytes))
                    .with_signature(compute_wasm_signature(&bytes, 2, WASM_SHOWCASE_SIGNING_KEY))
                    .with_allow_downgrade(true);
                    match m_p2.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                        Ok(v) => { let _: bool = s_p2.set(format!("Hot-swapped Partial .wasm v{v} natively (sibling UI untouched)")); }
                        Err(e) => { let _: bool = s_p2.set(format!("Partial v2 failed: {e}")); }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Toggle Offline (Cache Hit)".to_string(), tokens)
                .outline()
                .on_click(move |_: &UiEvent| {
                    let next: bool = !wasm_offline_sig.get_untracked();
                    let _: bool = wasm_offline_sig.set(next);
                    r_off.set_offline(next);
                    let _: bool = s_off.set(if next {
                        "Simulated OFFLINE enabled — re-downloading v1 will serve verified offline cache!".to_string()
                    } else {
                        "Simulated ONLINE restored — fetching live .wasm from CDN".to_string()
                    });
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Rollback Partial".to_string(), tokens)
                .outline()
                .on_click(move |_: &UiEvent| {
                    match m_prb.rollback_partial(SLOT_WASM_WIDGET, "manual rollback") {
                        Ok(v) => { let _: bool = s_prb.set(format!("Rolled back partial WASM slot to v{v}")); }
                        Err(e) => { let _: bool = s_prb.set(format!("Rollback: {e}")); }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Unmount Slot".to_string(), tokens)
                .ghost()
                .on_click(move |_: &UiEvent| {
                    m_pun.unmount_partial(SLOT_WASM_WIDGET);
                    let _: bool = s_pun.set("Unmounted partial WASM instance and restored compiled fallback".to_string());
                })
                .mount_current(),
        )
        .mount_current();
    let wasm_partial_body: NodeId = view! {
        column {
            gap: SPACE_MD
            { wasm_partial_actions }
            slot {
                id: SLOT_WASM_WIDGET
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    text {
                        font_size: 13.0
                        color: p.get_muted_foreground()
                        "Compiled Native Slot (v0) — click 'Download Partial WASM v1' to fetch, verify, execute .wasm in sandbox & render natively."
                    }
                }
            }
            text {
                font_size: 12.0
                color: p.get_muted_foreground()
                { move || {
                    let st: WasmLoadState = partial_state_sig.get();
                    let gst: WasmLoadState = global_state_sig.get();
                    let tel: WasmDynamicTelemetry = wasm_tel_sig.get();
                    format!(
                        "Partial={st:?} | Global={gst:?} | OK={}/{} Cache={} Retries={} Rollbacks={} Traps={} FuelStops={} Rejects={} Events={}",
                        tel.get_downloads_succeeded(),
                        tel.get_downloads_started(),
                        tel.get_cache_hits(),
                        tel.get_retries_performed(),
                        tel.get_rollbacks_executed(),
                        tel.get_sandbox_traps_caught(),
                        tel.get_fuel_exhaustions_caught(),
                        tel.get_integrity_rejections(),
                        tel.get_events_handled()
                    )
                } }
            }
        }
    };
    let wasm_partial_card: NodeId = EuvCard::new(
        "Scoped / Partial Dynamic WASM Execution".to_string(),
        tokens,
    )
    .subtitle("Download genuine .wasm binaries into a single native slot with checksum/signature verification, offline cache, interactive euv_on_event & rollback")
    .child(wasm_partial_body)
    .mount_current();
    let wasm_global_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Download Global WASM v1".to_string(), tokens)
                .on_click(move |_: &UiEvent| {
                    let bytes: Vec<u8> =
                        build_global_app_wasm("Global Dynamic WASM App Shell v1", 1);
                    let req: WasmDownloadRequest =
                        WasmDownloadRequest::global(URL_WASM_GLOBAL_V1.to_string(), 1)
                            .with_checksum(compute_wasm_checksum(&bytes))
                            .with_signature(compute_wasm_signature(
                                &bytes,
                                1,
                                WASM_SHOWCASE_SIGNING_KEY,
                            ))
                            .with_allow_downgrade(true);
                    match m_g1.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                        Ok(v) => {
                            let _: bool = s_g1
                                .set(format!("Global .wasm v{v} took over root shell natively!"));
                        }
                        Err(e) => {
                            let _: bool = s_g1.set(format!("Global v1 failed: {e}"));
                        }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Upgrade Global WASM v2".to_string(), tokens)
                .secondary()
                .on_click(move |_: &UiEvent| {
                    let bytes: Vec<u8> =
                        build_global_app_wasm("Global Dynamic WASM App Shell v2", 2);
                    let req: WasmDownloadRequest =
                        WasmDownloadRequest::global(URL_WASM_GLOBAL_V2.to_string(), 2)
                            .with_checksum(compute_wasm_checksum(&bytes))
                            .with_signature(compute_wasm_signature(
                                &bytes,
                                2,
                                WASM_SHOWCASE_SIGNING_KEY,
                            ))
                            .with_allow_downgrade(true);
                    match m_g2.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                        Ok(v) => {
                            let _: bool =
                                s_g2.set(format!("Upgraded Global .wasm to v{v} atomically"));
                        }
                        Err(e) => {
                            let _: bool = s_g2.set(format!("Global v2 failed: {e}"));
                        }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Faulty Global (Auto LKG Rollback)".to_string(), tokens)
                .destructive()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest =
                        WasmDownloadRequest::global(URL_WASM_TRAP.to_string(), 99);
                    match m_gbad.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                        Ok(v) => {
                            let _: bool = s_gbad.set(format!("Unexpected global mount v{v}"));
                        }
                        Err(e) => {
                            let _: bool =
                                s_gbad.set(format!("Global fault isolated & LKG preserved: {e}"));
                        }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Restore Native Shell".to_string(), tokens)
                .outline()
                .on_click(move |_: &UiEvent| {
                    m_grb.restore_native_global();
                    let _: bool = s_grb.set(STATUS_RESTORED_NATIVE_SHELL.to_string());
                })
                .mount_current(),
        )
        .mount_current();
    let wasm_global_body: NodeId = view! {
        column {
            gap: SPACE_MD
            { wasm_global_actions }
        }
    };
    let wasm_global_card: NodeId = EuvCard::new(
        "Global / Full-App Dynamic WASM Takeover".to_string(),
        tokens,
    )
    .subtitle("Atomically replace the entire native app shell from a downloaded .wasm release with automatic Last-Known-Good (LKG) rollback on any fault")
    .child(wasm_global_body)
    .mount_current();
    let wasm_fault_actions: NodeId = EuvActionRow::new()
        .child(
            EuvButton::new("Timeout -> Auto Retry".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_TIMEOUT_RETRY.to_string(), SLOT_WASM_WIDGET.to_string(), 10).with_allow_downgrade(true);
                    match m_f_retry.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                        Ok(v) => { let _: bool = s_f_retry.set(format!("PASS Timeout->Retry recovered and mounted v{v}!")); }
                        Err(e) => { let _: bool = s_f_retry.set(format!("Retry failed: {e}")); }
                    }
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("HTTP 500".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_HTTP_500.to_string(), SLOT_WASM_WIDGET.to_string(), 11).with_allow_downgrade(true);
                    let res: String = format!("Handled HTTP 500 -> {:?}", m_f_500.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_500.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Corrupted Magic".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_CORRUPTED.to_string(), SLOT_WASM_WIDGET.to_string(), 12).with_allow_downgrade(true);
                    let res: String = format!("Blocked Corrupted Magic -> {:?}", m_f_corrupt.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_corrupt.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Tampered Checksum".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let good: Vec<u8> = build_partial_widget_wasm(TITLE_WASM_PARTIAL_V1, 1, 0x2563EB);
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_TAMPERED.to_string(), SLOT_WASM_WIDGET.to_string(), 13)
                        .with_checksum(compute_wasm_checksum(&good))
                        .with_allow_downgrade(true);
                    let res: String = format!("Blocked Tampered Checksum -> {:?}", m_f_tamper.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_tamper.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Invalid Signature".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_PARTIAL_V1.to_string(), SLOT_WASM_WIDGET.to_string(), 14)
                        .with_signature("euv-sig-v1:forged".to_string())
                        .with_allow_downgrade(true);
                    let res: String = format!("Blocked Forged Signature -> {:?}", m_f_sig.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_sig.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("ABI Mismatch".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_BAD_ABI.to_string(), SLOT_WASM_WIDGET.to_string(), 15).with_allow_downgrade(true);
                    let res: String = format!("Blocked ABI Mismatch -> {:?}", m_f_abi.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_abi.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Missing Export".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_MISSING_EXPORT.to_string(), SLOT_WASM_WIDGET.to_string(), 16).with_allow_downgrade(true);
                    let res: String = format!("Blocked Missing Export -> {:?}", m_f_exp.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_exp.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Infinite-Loop Fuel".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_FUEL_BOMB.to_string(), SLOT_WASM_WIDGET.to_string(), 17).with_allow_downgrade(true);
                    let res: String = format!("Terminated Runaway Loop -> {:?}", m_f_fuel.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_fuel.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Runtime Trap".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_TRAP.to_string(), SLOT_WASM_WIDGET.to_string(), 18).with_allow_downgrade(true);
                    let res: String = format!("Isolated Runtime Trap -> {:?}", m_f_trap.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_trap.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Oversized Binary".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_OVERSIZED.to_string(), SLOT_WASM_WIDGET.to_string(), 19).with_allow_downgrade(true);
                    let res: String = format!("Blocked Oversized Binary -> {:?}", m_f_big.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                    let _: bool = s_f_big.set(res);
                })
                .mount_current(),
        )
        .child(
            EuvButton::new("Cancel / Stale Guard".to_string(), tokens)
                .outline()
                .sm()
                .on_click(move |_: &UiEvent| {
                    let gen_id: u64 = m_f_cancel.cancel_scope(&WasmDynamicScope::Partial { slot_id: SLOT_WASM_WIDGET.to_string() });
                    let _: bool = s_f_cancel.set(format!("Cancelled scope request cleanly (generation #{gen_id} blocks stale responses)"));
                })
                .mount_current(),
        )
        .mount_current();
    let wasm_fault_body: NodeId = view! {
        column {
            gap: SPACE_MD
            { wasm_fault_actions }
            text {
                font_size: 12.5
                font_weight: FontWeight::Medium
                color: p.get_primary()
                { move || wasm_status_sig.get() }
            }
        }
    };
    let wasm_fault_card: NodeId = EuvCard::new(
        "All-Case Security, Fault & Sandbox Verification Matrix".to_string(),
        tokens,
    )
    .subtitle("Verify every edge case live: auto-retry, offline cache, integrity rejection, ABI mismatch, infinite-loop fuel cutoff, trap isolation & stale cancel guard")
    .child(wasm_fault_body)
    .mount_current();
    EuvPage::new(
        DYNAMIC_TITLE.to_string(),
        DYNAMIC_SUBTITLE.to_string(),
        tokens,
    )
    .section(wasm_partial_card)
    .section(wasm_global_card)
    .section(wasm_fault_card)
    .section(comp_card.mount_current())
    .section(page_card.mount_current())
    .mount_current()
}

/// Mounts the complete cross-platform Media Studio page demonstrating Image, SVG, GIF, LIVE Photo, Video, Audio, PDF, and native HTML5.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted Media Studio page node.
pub fn mount_media_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let link_log: Signal<String> = state.get_media_html_link_log();
    let link_log_cb: Signal<String> = link_log;
    let photo_card: NodeId = EuvImage::new(
        ImageAsset::sample_photo("Hero Raster Photo (PNG/BMP/WebP)"),
        tokens,
    )
    .height(148.0)
    .mount_current();
    let svg_card: NodeId = EuvSvgView::new(
        SvgDocument::sample_svg(),
        "Vector SVG Badge".to_string(),
        tokens,
    )
    .mount_current();
    let gif_card: NodeId = EuvGifPlayer::new(state.get_media_gif(), tokens).mount_current();
    let live_card: NodeId = EuvLivePhoto::new(state.get_media_live(), tokens).mount_current();
    let video_card: NodeId = EuvVideoPlayer::new(state.get_media_video(), tokens).mount_current();
    let audio_card: NodeId = EuvAudioPlayer::new(state.get_media_audio(), tokens).mount_current();
    let pdf_card: NodeId = EuvPdfViewer::new(state.get_media_pdf(), tokens).mount_current();
    let html_card: NodeId = EuvHtmlView::new(HtmlDocument::sample_html(), tokens)
        .on_link(move |href: String| {
            let _: bool = link_log_cb.set_if_changed(format!("Opened native link: {href}"));
        })
        .mount_current();
    let link_status: NodeId = view! {
        text {
            font_size: 12.5
            color: p.get_muted_foreground()
            { move || link_log.get() }
        }
    };
    let media_grid_top: NodeId = mount_bento_pair(photo_card, svg_card);
    let media_grid_mid: NodeId = mount_bento_pair(gif_card, live_card);
    let media_grid_av: NodeId = mount_bento_pair(video_card, audio_card);
    let media_grid_doc: NodeId = mount_bento_pair(pdf_card, html_card);
    EuvPage::new(
        "Full Cross-Platform Media Studio".to_string(),
        "Pure-Rust aspect-preserving Image, SVG, animated GIF, LIVE Photo, Video, WAV Audio waveform, multi-page PDF, and sanitized zero-JS HTML5.".to_string(),
        tokens,
    )
    .section(media_grid_top)
    .section(media_grid_mid)
    .section(media_grid_av)
    .section(media_grid_doc)
    .section(link_status)
    .mount_current()
}

/// Mounts a responsive 2-column Bento card pair (2 columns on desktop `Expanded`, 1 column on tablet/mobile).
///
/// # Arguments
///
/// - `NodeId` - First card node.
/// - `NodeId` - Second card node.
///
/// # Returns
///
/// - `NodeId` - Mounted responsive Bento container node.
fn mount_bento_pair(left: NodeId, right: NodeId) -> NodeId {
    let mut ctx: BuildContext = current_build_context();
    let bp_sig: Signal<Breakpoint> = ctx.get_breakpoint();
    let init_cols: u16 = if bp_sig.get_untracked() == Breakpoint::Expanded {
        2
    } else {
        1
    };
    let grid: NodeId = ctx.view(
        Style::empty()
            .grid(init_cols)
            .width(Length::Percent(100.0))
            .gap(SPACE_MD),
    );
    ctx.append(grid, left);
    ctx.append(grid, right);
    let handle: SceneHandle = ctx.get_scene().clone();
    let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            let cols: u16 = if bp_sig.get() == Breakpoint::Expanded {
                2
            } else {
                1
            };
            handle.with_write(
                |tree: &mut SceneTree| {
                    if let Some(node) = tree.get_node_mut(grid) {
                        let mut st: Style = node.get_style();
                        st.get_mut_layout().set_grid_columns(cols);
                        node.set_style(st);
                    }
                    let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                    tree.mark_dirty(grid, dirty_l | DIRTY_PAINT);
                },
                (),
            );
        })
    });
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(grid, scope);
        },
        (),
    );
    grid
}
