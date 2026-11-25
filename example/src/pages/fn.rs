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
}

class! {
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

/// Mounts the complete showcase application shell and active page for `state` using zero-`ctx` macros.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
///
/// # Returns
///
/// - `NodeId` - Mounted root application shell handle.
pub fn mount_showcase_root(state: &ShowcaseState) -> NodeId {
    let tokens: ThemeTokens = state.get_theme().get_tokens().get();
    let nav: NodeId =
        EuvSidebarNav::new(ShowcaseState::nav_items(), state.get_route(), tokens).mount_current();
    let page_node: NodeId = mount_active_route(state, tokens);
    EuvAppShell::new(SHOWCASE_BRAND.to_string(), nav, page_node, tokens).mount_current()
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
    match route.as_str() {
        ROUTE_REACTIVE => mount_reactive_page(state, tokens),
        ROUTE_RENDER => mount_render_page(state, tokens),
        ROUTE_WINDOW => mount_window_page(state, tokens),
        ROUTE_FORMS => mount_forms_page(state, tokens),
        ROUTE_THEMES => mount_themes_page(state, tokens),
        ROUTE_DATA => mount_data_page(state, tokens),
        ROUTE_GAME => mount_game_page(state, tokens),
        ROUTE_OVERLAYS => mount_overlays_page(state, tokens),
        _ => mount_overview_page(tokens),
    }
}

/// Mounts the overview dashboard page with KPI stat cards and the 7-platform capability matrix.
///
/// # Arguments
///
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted overview page node.
#[component]
pub fn mount_overview_page(tokens: ThemeTokens) -> NodeId {
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
    let mut card: EuvCard = EuvCard::new(CARD_PLATFORMS_TITLE.to_string(), tokens).child(table);
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
#[component]
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
    let body: NodeId = ui! {
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
#[component]
pub fn mount_render_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let seg: NodeId = EuvSegmented::new(
        vec![
            MODE_AUTO.to_string(),
            MODE_SOFTWARE.to_string(),
            MODE_HARDWARE.to_string(),
            MODE_HYBRID.to_string(),
        ],
        state.get_render_mode_index(),
        tokens,
    )
    .mount_current();
    let preview: NodeId = view! {
        column {
            gap: SPACE_SM
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

/// Mounts the window settings and platform capabilities inspector page.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted window inspector page node.
#[component]
pub fn mount_window_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let font_info: SystemFontInfo = NativeSystemFont::system().font_info();
    let gpu_driver: NativeGpuDriver = NativeGpuDriver::probe(GraphicsBackend::Metal);
    let gpu_info: &GpuAdapterInfo = gpu_driver.get_adapter();
    let platform: PlatformKind = current_platform();
    let caps: PlatformCapabilities = PlatformCapabilities::for_platform(platform);
    let mut hub: PlatformServiceHub = PlatformServiceHub::new(platform);
    let mut live_content: LiveActivityContent = LiveActivityContent::new(
        "euv Native Build".to_string(),
        "7-target pipeline active".to_string(),
    );
    live_content
        .set_compact_leading("euv".to_string())
        .set_compact_trailing("60fps".to_string())
        .set_progress_percent(Some(100));
    let island: LiveActivityHandle =
        hub.start_live_activity("showcase-live".to_string(), live_content);
    let proto_reg: ProtocolRegistration = ProtocolRegistration::new(
        "euv".to_string(),
        "euv Native Showcase".to_string(),
        "dev.euv.showcase".to_string(),
    );
    let proto_desc: ProtocolManifestEntry = proto_reg.platform_descriptor(platform);
    let cam_key: &str = SystemPermission::Camera.native_key(platform);
    let summary: String = format!(
        "Platform: {:?} | Breakpoint: {:?} | Font: {} ({:?}) | GPU: {} (shader_ok={})",
        platform,
        use_breakpoint().get_untracked(),
        font_info.get_family_name(),
        font_info.get_backend(),
        gpu_info.get_device_name(),
        gpu_info.get_shader_compiled()
    );
    let services_line: String = format!(
        "A11y: {} | Protocol: {} | Island: {:?} | Biometrics: {:?} | Camera Key: {}",
        caps.get_accessibility_bridge(),
        proto_desc.get_mechanism(),
        island.get_presentation(),
        hub.biometric_kind(),
        cam_key
    );
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
                color: p.get_muted_foreground()
                { services_line }
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

/// Mounts the interactive forms showcase with inputs, switches, checkboxes, tabs, and buttons.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted forms page node.
#[component]
pub fn mount_forms_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let username: Signal<String> = state.get_username();
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
            { EuvSwitch::new(state.get_hardware_accel(), SWITCH_HW_LABEL.to_string(), tokens) }
            { EuvCheckbox::new(state.get_agree_terms(), CHECKBOX_TERMS_LABEL.to_string(), tokens) }
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

/// Mounts the 17-scheme live design token gallery page.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted themes page node.
#[component]
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
    for scheme in all_schemes.iter() {
        let preview_tokens: ThemeTokens = ThemeTokens::resolve(*scheme, mode_sig.get_untracked());
        let badge: NodeId = EuvBadge::new(
            scheme.as_str().to_string(),
            EuvTone::Primary,
            preview_tokens,
        )
        .mount_current();
        ctx.append(badges_grid, badge);
    }
    let body: NodeId = ui! {
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

/// Mounts the reactive 10,000-row virtualized scrollable list and progress display page.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted data display page node.
#[component]
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
            { EuvProgress::new(0.84, tokens) }
            { list_node }
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
#[component]
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
                { EuvButton::new(BTN_STEP_PHYSICS.to_string(), tokens).on_click(move |_: &UiEvent| { let _: u32 = host_btn.step(0.05); }) }
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

/// Mounts the overlays page featuring `EuvModal`, `EuvToast`, and `EuvAlert`.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted overlays page node.
#[component]
pub fn mount_overlays_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let modal_open: Signal<bool> = state.get_modal_open();
    let toast_visible: Signal<bool> = state.get_toast_visible();
    let mut toast_btn: EuvButton =
        EuvButton::new(BTN_TOGGLE_TOAST.to_string(), tokens).on_click(move |_: &UiEvent| {
            let _: bool = toast_visible.update(|v: &mut bool| *v = !*v);
        });
    toast_btn.set_variant(EuvButtonVariant::Secondary);
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
        }
    };
    let mut card: EuvCard = EuvCard::new(OVERLAYS_CARD_TITLE.to_string(), tokens).child(actions);
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
    view! {
        column {
            style: c_page_column()
            { EuvHeader::new(OVERLAYS_TITLE.to_string(), OVERLAYS_SUBTITLE.to_string(), tokens) }
            { EuvAlert::new(ALERT_TITLE.to_string(), ALERT_DESC.to_string(), EuvTone::Primary, tokens) }
            { card }
            { EuvModal::new(modal_open, MODAL_TITLE.to_string(), modal_body, tokens) }
            { EuvToast::new(toast_visible, TOAST_TEXT.to_string(), EuvTone::Success, tokens) }
        }
    }
}
