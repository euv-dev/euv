use super::*;

style! {
    c_page_column {
        direction: column;
        width: Length::Percent(100.0);
        gap: SPACE_LG;
    }
    c_action_row {
        direction: row;
        gap: SPACE_SM;
    }
    c_stats_grid {
        columns: 3;
        width: Length::Percent(100.0);
        gap: SPACE_MD;
    }
    c_scheme_grid {
        columns: 4;
        width: Length::Percent(100.0);
        gap: SPACE_SM;
    }
}

/// Mounts the fully reactive showcase application shell rebuilding cleanly on route or theme changes.
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
        let mut nav_items: Vec<EuvNavItem> = ShowcaseState::nav_items();
        for dyn_route in hub.registered_routes().into_iter() {
            if dyn_route == ROUTE_DYNAMIC_PANEL {
                continue;
            }
            if !nav_items
                .iter()
                .any(|item: &EuvNavItem| item.get_path().as_str() == dyn_route.as_str())
            {
                let label: String = hub
                    .page_title(&dyn_route)
                    .filter(|t: &String| !t.is_empty())
                    .unwrap_or_else(|| dyn_route.clone());
                nav_items.push(EuvNavItem::new(dyn_route, label));
            }
        }
        let nav: NodeId =
            EuvSidebarNav::new(nav_items, state_c.get_route(), tokens).mount_current();
        let state_for_outlet: ShowcaseState = state_c.clone();
        let mut outlet: DynamicPageOutlet = DynamicPageOutlet::new(
            state_c.get_route(),
            Rc::new(move |active_route: &str| {
                mount_route_by_name(active_route, &state_for_outlet, tokens)
            }),
        );
        outlet.set_hub(Some(hub));
        let page_node: NodeId = outlet.mount_current();
        let shell: NodeId =
            EuvAppShell::new(SHOWCASE_BRAND.to_string(), nav, page_node, tokens).mount_current();
        vec![shell]
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
    let mut game_btn: EuvButton =
        EuvButton::new("Open Game Engine".to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = route_sig.set_if_changed(ROUTE_GAME.to_string());
        });
    game_btn.set_variant(EuvButtonVariant::Secondary);
    let mut os_btn: EuvButton =
        EuvButton::new("Open OS Services".to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = route_sig.set_if_changed(ROUTE_WINDOW.to_string());
        });
    os_btn.set_variant(EuvButtonVariant::Outline);
    let quick_row: NodeId = view! {
        row {
            style: c_action_row()
            { EuvButton::new("Try Reactivity".to_string(), tokens).on_click(move |_: &UiEvent| { let _: bool = route_sig.set_if_changed(ROUTE_REACTIVE.to_string()); }) }
            { game_btn }
            { os_btn }
        }
    };
    let mut card: EuvCard = EuvCard::new(CARD_PLATFORMS_TITLE.to_string(), tokens)
        .child(quick_row)
        .child(table);
    card.set_subtitle(CARD_PLATFORMS_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(OVERVIEW_TITLE.to_string(), OVERVIEW_SUBTITLE.to_string(), tokens) }
            grid {
                style: c_stats_grid()
                { EuvStatCard::new(STAT_PLATFORMS_LABEL.to_string(), STAT_PLATFORMS_VAL.to_string(), STAT_PLATFORMS_CAP.to_string(), tokens) }
                { EuvStatCard::new(STAT_SCHEMES_LABEL.to_string(), STAT_SCHEMES_VAL.to_string(), STAT_SCHEMES_CAP.to_string(), tokens) }
                { EuvStatCard::new(STAT_RENDER_LABEL.to_string(), STAT_RENDER_VAL.to_string(), STAT_RENDER_CAP.to_string(), tokens) }
            }
            { card }
        }
    }
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
            row {
                style: c_action_row()
                { EuvButton::new(BTN_INCREMENT.to_string(), tokens).on_click(move |_: &UiEvent| { let _: bool = counter.update(|v: &mut i32| *v += 1); }) }
                { batch_btn }
                { reset_btn }
            }
        }
    };
    let mut card: EuvCard = EuvCard::new(REACTIVE_CARD_TITLE.to_string(), tokens).child(body);
    card.set_subtitle(REACTIVE_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(REACTIVE_TITLE.to_string(), REACTIVE_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
                    dl.push(DrawCommand::FillRoundedRect {
                        rect: Rect::new(bounds.get_x() + 16.0, bounds.get_y() + 16.0, 160.0, 108.0),
                        color: p.get_primary(),
                        radius: CornerRadius::all(14.0),
                        border: Border::default(),
                    });
                    dl.push(DrawCommand::Circle {
                        center: Point::new(bounds.get_x() + 260.0, bounds.get_y() + 70.0),
                        radius: 42.0,
                        color: p.get_success(),
                    });
                    dl.push(DrawCommand::Line {
                        from: Point::new(bounds.get_x() + 340.0, bounds.get_y() + 28.0),
                        to: Point::new(bounds.get_x() + 480.0, bounds.get_y() + 112.0),
                        color: p.get_warning(),
                        width: 3.0,
                    });
                }
            }
        }
    };
    let mut card: EuvCard = EuvCard::new(RENDER_CARD_TITLE.to_string(), tokens)
        .child(seg)
        .child(preview);
    card.set_subtitle(RENDER_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(RENDER_TITLE.to_string(), RENDER_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
    let summary: String = format!(
        "Platform: {:?} | Breakpoint: {:?} | Font: {} ({:?}) | GPU: {} (shader_ok={})",
        platform,
        use_breakpoint().get_untracked(),
        font_info.get_family_name(),
        font_info.get_backend(),
        gpu_info.get_device_name(),
        gpu_info.get_shader_compiled()
    );
    let mut proto_btn: EuvButton = EuvButton::new("Register euv:// Protocol".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let mut reg: ProtocolRegistry = ProtocolRegistry::new();
            let spec: ProtocolRegistration = ProtocolRegistration::new(
                "euv".to_string(),
                "euv Native Showcase".to_string(),
                "dev.euv.showcase".to_string(),
            );
            if let Ok(desc) = reg.register(spec, platform) {
                let _: bool = proto_status.set_if_changed(format!(
                    "Registered via {} | DeepLink Ready",
                    desc.get_mechanism()
                ));
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
            let _: bool = cam_status.set_if_changed(format!(
                "{:?} ({})",
                desc.get_status(),
                desc.get_native_key()
            ));
        });
    perm_btn.set_variant(EuvButtonVariant::Outline);
    let mut a11y_btn: EuvButton = EuvButton::new("Announce A11y + Haptic".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: bool = a11y_count.update(|v: &mut u32| *v += 1);
            let _: bool = haptic_count.update(|v: &mut u32| *v += 1);
        });
    a11y_btn.set_variant(EuvButtonVariant::Outline);
    let body: NodeId = view! {
        column {
            role: A11yRole::Region
            a11y_label: "Platform Telemetry"
            gap: SPACE_MD
            text {
                font_size: 14.0
                font_weight: FontWeight::Medium
                color: p.get_foreground()
                { summary }
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
            row {
                style: c_action_row()
                { EuvButton::new("Toggle Dynamic Island".to_string(), tokens).on_click(move |_: &UiEvent| { let _: bool = island_active.update(|v: &mut bool| *v = !*v); let _: bool = island_progress.update(|p: &mut u8| *p = (*p + 25).min(100)); }) }
                { proto_btn }
                { perm_btn }
                { a11y_btn }
            }
            { EuvSwitch::new(state.get_fullscreen(), SWITCH_FULLSCREEN.to_string(), tokens) }
            { EuvCodeBlock::new(CODE_WINDOW_SNIPPET.to_string(), tokens) }
        }
    };
    let mut card: EuvCard = EuvCard::new(WINDOW_CARD_TITLE.to_string(), tokens).child(body);
    card.set_subtitle(WINDOW_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(WINDOW_TITLE.to_string(), WINDOW_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            { EuvTabs::new(vec![TAB_GENERAL.to_string(), TAB_GRAPHICS.to_string(), TAB_SECURITY.to_string()], state.get_active_tab(), tokens) }
            { EuvField::new(FIELD_USERNAME.to_string(), input, tokens) }
            { EuvTextarea::new(state.get_bio(), "Write notes...".to_string(), 54.0, tokens) }
            row {
                style: c_action_row()
                { EuvNumberInput::new(stepper, 0, 20, 1, tokens) }
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
                { move || format!("Handle: {} | Stepper: {} | Stars: {}/5", username.get(), stepper.get(), rating.get()) }
            }
            row {
                style: c_action_row()
                { EuvButton::new(BTN_PRIMARY_LABEL.to_string(), tokens) }
                { b2 }
                { b3 }
                { b4 }
            }
        }
    };
    let mut card: EuvCard = EuvCard::new(FORMS_CARD_TITLE.to_string(), tokens).child(body);
    card.set_subtitle(FORMS_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(FORMS_TITLE.to_string(), FORMS_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
    let badges_grid: NodeId = view! {
        grid {
            style: c_scheme_grid()
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
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            text {
                font_size: 15.0
                font_weight: FontWeight::SemiBold
                color: p.get_foreground()
                { move || format!("{PREFIX_ACTIVE_SCHEME}{} ({:?})", scheme_sig.get().as_str(), mode_sig.get()) }
            }
            row {
                style: c_action_row()
                { EuvButton::new(BTN_NEXT_SCHEME.to_string(), tokens).on_click(move |_: &UiEvent| { state_for_cycle.next_scheme(); }) }
                { mode_btn }
            }
            { badges_grid }
        }
    };
    let mut card: EuvCard = EuvCard::new(THEMES_CARD_TITLE.to_string(), tokens).child(body);
    card.set_subtitle(THEMES_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(THEMES_TITLE.to_string(), THEMES_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
    let vlist: EuvVirtualList = EuvVirtualList::new(10_000, 32.0, 224.0, scroll_y, tokens);
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
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            row {
                style: c_action_row()
                { EuvBadge::new(BADGE_READY.to_string(), EuvTone::Success, tokens) }
                { EuvBadge::new(BADGE_LIVE.to_string(), EuvTone::Primary, tokens) }
                { EuvButton::new(BTN_SCROLL_DOWN.to_string(), tokens).on_click(move |_: &UiEvent| { vlist_down.scroll_by(160.0); }) }
                { top_btn }
            }
            text {
                font_size: 12.0
                color: p.get_muted_foreground()
                { move || format!("scroll_y={:.0}px | top_row=#{}", scroll_y.get(), (scroll_y.get() / 32.0).floor() as usize) }
            }
            { EuvProgress::new(0.84, tokens) }
            { list_node }
            { EuvPagination::new(state.get_page_index(), 12, tokens) }
            { EuvSteps::new(vec!["Layout".to_string(), "Paint".to_string(), "Present".to_string()], state.get_step_index(), tokens) }
            { EuvAccordion::new(vec![EuvAccordionItem::new("Virtualization".to_string(), "Only visible rows + overscan are materialized in O(visible).".to_string())], state.get_accordion_open(), tokens) }
        }
    };
    let mut card: EuvCard = EuvCard::new(DATA_CARD_TITLE.to_string(), tokens).child(body);
    card.set_subtitle(DATA_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(DATA_TITLE.to_string(), DATA_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
    let viewport: NodeId = host.mount_viewport(
        Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(220.0))
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md()),
    );
    let body: NodeId = view! {
        column {
            gap: SPACE_MD
            row {
                style: c_action_row()
                { EuvBadge::new("60Hz Fixed Timestep".to_string(), EuvTone::Success, tokens) }
                { EuvBadge::new("HUD Overlay Ready".to_string(), EuvTone::Primary, tokens) }
                { EuvButton::new(BTN_STEP_PHYSICS.to_string(), tokens).on_click(move |_: &UiEvent| { let _: u32 = host_btn.step(0.051); }) }
            }
            { viewport }
        }
    };
    let mut card: EuvCard = EuvCard::new(GAME_CARD_TITLE.to_string(), tokens).child(body);
    card.set_subtitle(GAME_CARD_SUB.to_string());
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(GAME_TITLE.to_string(), GAME_SUBTITLE.to_string(), tokens) }
            { card }
        }
    }
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
    let actions: NodeId = view! {
        row {
            style: c_action_row()
            { EuvButton::new(BTN_OPEN_MODAL.to_string(), tokens).on_click(move |_: &UiEvent| { let _: bool = modal_open.set_if_changed(true); }) }
            { toast_btn }
            { drawer_btn }
            { sheet_btn }
            { notif_btn }
        }
    };
    let dropdown: NodeId = EuvDropdown::new(
        "Quick Actions".to_string(),
        vec!["Export PNG".to_string(), "Copy DeepLink".to_string()],
        state.get_dropdown_open(),
        state.get_dropdown_selected(),
        tokens,
    )
    .mount_current();
    let mut card: EuvCard = EuvCard::new(OVERLAYS_CARD_TITLE.to_string(), tokens)
        .child(actions)
        .child(dropdown);
    card.set_subtitle(OVERLAYS_CARD_SUB.to_string());
    let modal_body: NodeId = view! {
        column {
            gap: SPACE_MD
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                { MODAL_BODY_TEXT }
            }
            { close_btn }
        }
    };
    let mut close_drawer_btn: EuvButton = EuvButton::new("Close Drawer".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: bool = drawer_open.set_if_changed(false);
        });
    close_drawer_btn.set_variant(EuvButtonVariant::Outline);
    let drawer_body: NodeId = view! {
        column {
            gap: SPACE_MD
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                "Native slide-over drawer surface"
            }
            { close_drawer_btn }
        }
    };
    let mut close_sheet_btn: EuvButton = EuvButton::new("Close Sheet".to_string(), tokens)
        .on_click(move |_: &UiEvent| {
            let _: bool = sheet_open.set_if_changed(false);
        });
    close_sheet_btn.set_variant(EuvButtonVariant::Outline);
    let sheet_body: NodeId = view! {
        column {
            gap: SPACE_MD
            text {
                font_size: 13.0
                color: p.get_muted_foreground()
                "Native bottom action sheet"
            }
            { close_sheet_btn }
        }
    };
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(OVERLAYS_TITLE.to_string(), OVERLAYS_SUBTITLE.to_string(), tokens) }
            { EuvAlert::new(ALERT_TITLE.to_string(), ALERT_DESC.to_string(), EuvTone::Primary, tokens) }
            { card }
            { EuvModal::new(modal_open, MODAL_TITLE.to_string(), modal_body, tokens) }
            { EuvDrawer::new(drawer_open, "Inspector Drawer".to_string(), drawer_body, tokens) }
            { EuvSheet::new(sheet_open, "Quick Sheet".to_string(), sheet_body, tokens) }
            { EuvToast::new(toast_visible, TOAST_TEXT.to_string(), EuvTone::Success, tokens) }
            { EuvNotification::new(notif_visible, "Native Sync".to_string(), "All 7 targets active.".to_string(), EuvTone::Success, tokens) }
            { EuvLoading::new(state.get_loading_active(), "Syncing...".to_string(), tokens) }
        }
    }
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
    let comp_controls: NodeId = view! {
        column {
            gap: SPACE_MD
            row {
                style: c_action_row()
                { btn_sibling }
                text {
                    font_size: 13.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_primary()
                    { move || format!("Sibling State Preserved: count={}", sibling_counter.get()) }
                }
            }
            row {
                style: c_action_row()
                { EuvButton::new(BTN_PUSH_BANNER_V1.to_string(), tokens).on_click(move |_: &UiEvent| {
                    let next_v: u64 = hub_v1.component_version(SLOT_PROMO_BANNER).saturating_add(1);
                    let mut badge: DynamicNodeSpec = DynamicNodeSpec::badge(format!("OTA v{next_v}"));
                    badge.get_mut_style().set_background_hex(Some(0x1D4ED8)).set_color_hex(Some(0xFFFFFF));
                    let mut title: DynamicNodeSpec = DynamicNodeSpec::text(format!("Dynamic JSON Banner v{next_v}: Surgical Component Patch"));
                    title.get_mut_style().set_bold(true).set_color_hex(Some(0x1E3A8A));
                    let mut offer_btn: DynamicNodeSpec = DynamicNodeSpec::button(BTN_CLAIM_OFFER.to_string(), "claim_offer".to_string());
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
                }) }
                { btn_banner_v2 }
                { btn_banner_rb }
            }
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
            row {
                style: c_action_row()
                { EuvButton::new(BTN_PATCH_METRICS.to_string(), tokens).on_click(move |_: &UiEvent| {
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
                }) }
                { btn_metrics_rb }
            }
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
    let page_controls: NodeId = view! {
        column {
            gap: SPACE_MD
            row {
                style: c_action_row()
                { EuvButton::new(BTN_UPDATE_PAGE_V1.to_string(), tokens).on_click(move |_: &UiEvent| {
                    let next_v: u64 = hub_p1.page_version(ROUTE_DYNAMIC_PANEL).saturating_add(1);
                    let spec: DynamicNodeSpec = DynamicNodeSpec::card(format!("Hot-Updated Dynamic Page Panel v{next_v}"))
                        .child(DynamicNodeSpec::badge(format!("Page Revision v{next_v}")))
                        .child(DynamicNodeSpec::text("Page-level layout replaced live without restarting the application.".to_string()));
                    let mut upd: DynamicPageUpdate = DynamicPageUpdate::new(
                        ROUTE_DYNAMIC_PANEL.to_string(),
                        format!("Dynamic Panel v{next_v}"),
                        next_v,
                        spec,
                    );
                    upd.sign();
                    let _: Result<u64, DynamicUpdateError> = hub_p1.apply_page(upd);
                }) }
                { btn_bundle }
                { btn_page_rb }
                { btn_inject }
            }
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
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(DYNAMIC_TITLE.to_string(), DYNAMIC_SUBTITLE.to_string(), tokens) }
            { comp_card }
            { page_card }
        }
    }
}
