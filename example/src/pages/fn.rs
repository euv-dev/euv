use super::*;

style! {
    c_full_col {
        FlexDirection::Column
        width: Length::Percent(100.0)
        height: Length::Percent(100.0)
    }
    c_action_row {
        FlexDirection::Row
        FlexWrap::Wrap
        AlignItems::Center
        width: Length::Percent(100.0)
        gap: SPACE_SM
    }
    c_scheme_grid {
        columns: 4
        width: Length::Percent(100.0)
        gap: SPACE_SM
    }
}

/// Registers all 15 showcase routes lazily onto `router` using typed `#[component(retention = Freeze)]` pages.
///
/// # Arguments
///
/// - `&ShowcaseState` - Shared reactive showcase state.
fn ensure_showcase_routes_registered(state: &ShowcaseState) {
    if state.get_router_ready().replace(true) {
        return;
    }
    let all_routes: [(&'static str, &'static str); 15] = [
        (ROUTE_OVERVIEW, NAV_LABEL_OVERVIEW),
        (ROUTE_MARKDOWN, NAV_LABEL_MARKDOWN),
        (ROUTE_REACTIVE, NAV_LABEL_REACTIVE),
        (ROUTE_RENDER, NAV_LABEL_RENDER),
        (ROUTE_WINDOW, NAV_LABEL_WINDOW),
        (ROUTE_FORMS, NAV_LABEL_FORMS),
        (ROUTE_THEMES, NAV_LABEL_THEMES),
        (ROUTE_DATA, NAV_LABEL_DATA),
        (ROUTE_GAME, NAV_LABEL_GAME),
        (ROUTE_GAME_3D, NAV_LABEL_GAME_3D),
        (ROUTE_MEDIA, NAV_LABEL_MEDIA),
        (ROUTE_LAB, NAV_LABEL_LAB),
        (ROUTE_OVERLAYS, NAV_LABEL_OVERLAYS),
        (ROUTE_DYNAMIC, NAV_LABEL_DYNAMIC),
        (ROUTE_STRESS, NAV_LABEL_STRESS),
    ];
    let mut router: EuvRouter = state.get_router().clone();
    for (path, label) in all_routes.into_iter() {
        let st: ShowcaseState = state.clone();
        router = router.route_component(path, label, move |_: &EuvRouteMatch| {
            let tok: ThemeTokens = untracked(|| st.get_theme().get_tokens().get());
            ShowcasePage::new(path, st.clone(), tok)
        });
    }
    let _: EuvRouter = router;
}

/// Mounts the fully reactive showcase application shell with lazy `PageRetention::Freeze` routing and cross-component events.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
///
/// # Returns
///
/// - `NodeId` - Mounted reactive root container handle.
pub fn mount_showcase_root(state: &ShowcaseState) -> NodeId {
    provide_theme(state.get_theme().clone());
    ensure_showcase_routes_registered(state);
    let global_banner: Signal<ShowcaseGlobalBanner> =
        global_state!(ShowcaseGlobalBanner = ShowcaseGlobalBanner::default());
    let state_c: ShowcaseState = state.clone();
    let bg_state: ShowcaseState = state.clone();
    let fg_state: ShowcaseState = state.clone();
    let sys_log_sig: Signal<Vec<String>> = state.get_system_event_log();
    let sys_count_sig: Signal<u32> = state.get_system_event_count();
    let (ev_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        let _: SystemEventSubscription = on_background(move || {
            bg_state.freeze_for_background();
        });
        let _: SystemEventSubscription = on_foreground(move || {
            fg_state.restore_from_foreground();
        });
        let _: SystemEventSubscription = on_system_event(move |ev: &SystemEvent| {
            let next_seq: u32 = sys_count_sig.get_untracked().saturating_add(1);
            let _: bool = sys_count_sig.set(next_seq);
            let entry: String = format!("#{next_seq}: {ev:?}");
            let _: bool = sys_log_sig.update(|items: &mut Vec<String>| {
                items.insert(0, entry);
                if items.len() > SYS_EVENTS_LOG_LIMIT {
                    items.truncate(SYS_EVENTS_LOG_LIMIT);
                }
            });
        });
        on_event!(ShowcasePulseEvent => move |ev: &ShowcasePulseEvent| {
            let src: String = ev.get_source().clone();
            let _: bool = global_banner.update(|b: &mut ShowcaseGlobalBanner| {
                let next_pulses: u32 = b.get_pulses().saturating_add(1);
                b.set_pulses(next_pulses);
                b.set_last_source(src);
            });
        })
    });
    let root_node: NodeId = view! {
        column {
            width: Length::Percent(100.0)
            height: Length::Percent(100.0)
            bind_children: move || {
                let tokens: ThemeTokens = untracked(|| state_c.get_theme().get_tokens().get());
                let hub: DynamicUpdateHub = state_c.get_dynamic_hub().clone();
                let _: u64 = hub.routes_signal().get();
                let cmd_open: Signal<bool> = state_c.get_cmd_palette_open();
                let cmd_query: Signal<String> = state_c.get_cmd_palette_query();
                let cmd_items: Vec<String> = vec![
                    CMD_ITEM_OVERVIEW.to_string(),
                    CMD_ITEM_MARKDOWN.to_string(),
                    CMD_ITEM_LAB.to_string(),
                    CMD_ITEM_GAME_3D.to_string(),
                    CMD_ITEM_MEDIA.to_string(),
                    CMD_ITEM_DYNAMIC.to_string(),
                    CMD_ITEM_STRESS.to_string(),
                    CMD_ITEM_WINDOW.to_string(),
                    CMD_ITEM_THEMES.to_string(),
                    CMD_ITEM_TOGGLE_MODE.to_string(),
                ];
                let cmd_sel: Signal<Option<usize>> = Signal::new(None);
                let cmd_route: Signal<String> = state_c.get_route();
                let cmd_state: ShowcaseState = state_c.clone();
                let cmd_list: NodeId = view! {
                    EuvCommand {
                        items: cmd_items
                        selected: cmd_sel
                        tokens: tokens
                        query: cmd_query
                        placeholder: CMD_PLACEHOLDER
                    }
                };
                let (cmd_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                    Effect::new(move || {
                        if let Some(idx) = cmd_sel.get() {
                            match idx {
                                0 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_OVERVIEW.to_string());
                                }
                                1 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_MARKDOWN.to_string());
                                }
                                2 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_LAB.to_string());
                                }
                                3 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_GAME_3D.to_string());
                                }
                                4 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_MEDIA.to_string());
                                }
                                5 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_DYNAMIC.to_string());
                                }
                                6 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_STRESS.to_string());
                                }
                                7 => {
                                    let _: bool = cmd_route.set_if_changed(ROUTE_WINDOW.to_string());
                                }
                                8 => {
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
                let cmd_modal: NodeId = view! {
                    EuvModal {
                        open: cmd_open
                        title: CMD_MODAL_TITLE.to_string()
                        body: cmd_list
                        tokens: tokens
                    }
                };
                let rctx: Context = current_context();
                let cmd_theme_sig: Memo<ThemeTokens> = state_c.get_theme().get_tokens();
                let prev_cmd_tok: Rc<Cell<ThemeTokens>> = Rc::new(Cell::new(tokens));
                let cmd_theme_ctx: Context = rctx.clone();
                let (cmd_theme_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                    Effect::new(move || {
                        let next_tok: ThemeTokens = cmd_theme_sig.get();
                        let old_tok: ThemeTokens = prev_cmd_tok.replace(next_tok);
                        if old_tok == next_tok {
                            return;
                        }
                        cmd_theme_ctx.get_scene().with_write(
                            |tree: &mut SceneTree| {
                                remap_subtree_theme_in_place(tree, cmd_modal, old_tok, next_tok);
                            },
                            (),
                        );
                    })
                });
                rctx.get_scene().with_write(
                    |tree: &mut SceneTree| {
                        tree.attach_scope(cmd_modal, cmd_scope);
                        tree.attach_scope(cmd_modal, cmd_theme_scope);
                    },
                    (),
                );
                let state_shell: ShowcaseState = state_c.clone();
                let hub_shell: DynamicUpdateHub = hub.clone();
                let global_wasm_slot: NodeId = view! {
                    slot {
                        id: WASM_GLOBAL_ROOT_SLOT
                        hub: hub
                        style: c_full_col()
                        fallback: move || {
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
                            let nav: NodeId = view! {
                                EuvSidebarNav {
                                    items: nav_items
                                    active_route: state_shell.get_route()
                                    tokens: tokens
                                }
                            };
                            let bottom_items: Vec<EuvNavItem> = vec![
                                EuvNavItem::new(
                                    ROUTE_OVERVIEW.to_string(),
                                    BNAV_HOME_LABEL.to_string(),
                                )
                                .section(BNAV_HOME_ICON),
                                EuvNavItem::new(
                                    ROUTE_MARKDOWN.to_string(),
                                    BNAV_DOCS_LABEL.to_string(),
                                )
                                .section(BNAV_DOCS_ICON),
                                EuvNavItem::new(ROUTE_LAB.to_string(), BNAV_CASES_LABEL.to_string())
                                .section(BNAV_CASES_ICON),
                                EuvNavItem::new(
                                    ROUTE_STRESS.to_string(),
                                    BNAV_STRESS_LABEL.to_string(),
                                )
                                .section(BNAV_STRESS_ICON),
                                EuvNavItem::new(
                                    ROUTE_GAME_3D.to_string(),
                                    BNAV_3D_LABEL.to_string(),
                                )
                                .section(BNAV_3D_ICON),
                            ];
                            let bottom_nav: NodeId = view! {
                                EuvBottomNav {
                                    items: bottom_items
                                    active_route: state_shell.get_route()
                                    tokens: tokens
                                }
                            };
                            let swipe_order: Vec<String> = vec![
                                ROUTE_OVERVIEW.to_string(),
                                ROUTE_MARKDOWN.to_string(),
                                ROUTE_LAB.to_string(),
                                ROUTE_STRESS.to_string(),
                                ROUTE_GAME_3D.to_string(),
                                ROUTE_FORMS.to_string(),
                                ROUTE_MEDIA.to_string(),
                                ROUTE_OVERLAYS.to_string(),
                                ROUTE_THEMES.to_string(),
                                ROUTE_DATA.to_string(),
                                ROUTE_REACTIVE.to_string(),
                                ROUTE_RENDER.to_string(),
                                ROUTE_WINDOW.to_string(),
                                ROUTE_GAME.to_string(),
                                ROUTE_DYNAMIC.to_string(),
                            ];
                            let state_for_outlet: ShowcaseState = state_shell.clone();
                            let router: EuvRouter = state_shell
                            .get_router()
                            .clone()
                            .with_dynamic_hub(hub_shell.clone());
                            let route_sig: Signal<String> = state_shell.get_route();
                            let cur_target: String = route_sig.get_untracked();
                            if router.get_current_path().get_untracked() != cur_target {
                                router.replace(&cur_target);
                            }
                            let breadcrumb_sig: Signal<String> = Signal::new(
                                ShowcaseState::breadcrumb_for_route(&cur_target),
                            );
                            let page_pad_sig: Signal<Edges> = Signal::new(Edges::all(SPACE_LG));
                            let router_sync_in: EuvRouter = router.clone();
                            let router_path_sig: Signal<String> = router.get_current_path();
                            let sctx: Context = current_context();
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
                                let tok: ThemeTokens = untracked(|| {
                                    state_for_outlet.get_theme().get_tokens().get()
                                });
                                mount_route_by_name(matched.get_path(), &state_for_outlet, tok)
                            })
                            .with_theme(state_shell.get_theme().clone())
                            .with_page_padding(page_pad_sig)
                            .with_route_order(swipe_order.clone())
                            .with_double_back_exit(true)
                            .mount();
                            sctx.get_scene().with_write(
                                |tree: &mut SceneTree| {
                                    tree.attach_scope(page_node, sync_scope);
                                },
                                (),
                            );
                            let st_dark_top: ShowcaseState = state_shell.clone();
                            let st_scheme_top: ShowcaseState = state_shell.clone();
                            let st_dark_mobile: ShowcaseState = state_shell.clone();
                            let theme_mode_sig: Signal<ThemeMode> = state_shell.get_theme().get_mode();
                            let theme_scheme_sig: Signal<DesignScheme> = state_shell.get_theme().get_scheme();
                            let mode_text_desktop = move || -> String {
                                match theme_mode_sig.get() {
                                    ThemeMode::Dark => LABEL_MODE_LIGHT.to_string(),
                                    ThemeMode::Light => LABEL_MODE_DARK.to_string(),
                                }
                            };
                            let mode_text_mobile = move || -> String {
                                match theme_mode_sig.get() {
                                    ThemeMode::Dark => LABEL_MODE_LIGHT.to_string(),
                                    ThemeMode::Light => LABEL_MODE_DARK.to_string(),
                                }
                            };
                            let scheme_text_desktop = move || -> String {
                                format!("{:?}", theme_scheme_sig.get())
                            };
                            let scheme_btn_style = move || -> Style {
                                let live_tok: ThemeTokens = use_live_tokens(tokens);
                                let live_p: ThemePalette = live_tok.get_palette();
                                let live_g: ThemeGeometry = live_tok.get_geometry();
                                Style::default()
                                .row()
                                .height(Length::Px(live_g.get_control_sm()))
                                .padding(Edges::symmetric(0.0, SPACE_SM))
                                .justify(JustifyContent::Center)
                                .align(AlignItems::Center)
                                .background(live_p.get_secondary())
                                .border(live_g.get_border_width(), live_p.get_border())
                                .radius(live_g.get_radius_md())
                                .cursor(CursorIcon::Pointer)
                            };
                            let scheme_lbl_style = move || -> Style {
                                let live_p: ThemePalette = use_live_tokens(tokens).get_palette();
                                Style::default()
                                .font_size(12.5)
                                .font_weight(FontWeight::SemiBold)
                                .color(live_p.get_secondary_foreground())
                                .text_overflow(TextOverflow::Clip)
                            };
                            let mode_btn_style = move || -> Style {
                                let live_tok: ThemeTokens = use_live_tokens(tokens);
                                let live_p: ThemePalette = live_tok.get_palette();
                                let live_g: ThemeGeometry = live_tok.get_geometry();
                                Style::default()
                                .row()
                                .height(Length::Px(live_g.get_control_sm()))
                                .padding(Edges::symmetric(0.0, SPACE_SM))
                                .justify(JustifyContent::Center)
                                .align(AlignItems::Center)
                                .background(live_p.get_background())
                                .border(live_g.get_border_width(), live_p.get_border())
                                .radius(live_g.get_radius_md())
                                .cursor(CursorIcon::Pointer)
                            };
                            let mode_lbl_desktop_style = move || -> Style {
                                let live_p: ThemePalette = use_live_tokens(tokens).get_palette();
                                Style::default()
                                .font_size(12.5)
                                .font_weight(FontWeight::SemiBold)
                                .color(live_p.get_foreground())
                                .text_overflow(TextOverflow::Clip)
                            };
                            let mode_mobile_style = move || -> Style {
                                let live_tok: ThemeTokens = use_live_tokens(tokens);
                                let live_g: ThemeGeometry = live_tok.get_geometry();
                                Style::default()
                                .row()
                                .height(Length::Px(live_g.get_control_sm()))
                                .padding(Edges::symmetric(0.0, SPACE_SM))
                                .justify(JustifyContent::Center)
                                .align(AlignItems::Center)
                                .background(COLOR_TRANSPARENT)
                                .radius(live_g.get_radius_md())
                                .cursor(CursorIcon::Pointer)
                            };
                            let mode_lbl_mobile_style = move || -> Style {
                                let live_p: ThemePalette = use_live_tokens(tokens).get_palette();
                                Style::default()
                                .font_size(12.5)
                                .font_weight(FontWeight::SemiBold)
                                .color(live_p.get_foreground())
                                .text_overflow(TextOverflow::Clip)
                            };
                            let on_open_cmd_desktop = move |_: &UiEvent| {
                                let _: bool = cmd_open.set_if_changed(true);
                            };
                            let on_open_cmd_mobile = move |_: &UiEvent| {
                                let _: bool = cmd_open.set_if_changed(true);
                            };
                            let on_cycle_scheme = move |_: &UiEvent| {
                                st_scheme_top.next_scheme();
                            };
                            let on_toggle_mode_desktop = move |_: &UiEvent| {
                                st_dark_top.toggle_theme_mode();
                            };
                            let on_toggle_mode_mobile = move |_: &UiEvent| {
                                st_dark_mobile.toggle_theme_mode();
                            };
                            let top_actions: NodeId = view! {
                                row {
                                    gap: SPACE_SM
                                    align: AlignItems::Center
                                    EuvButton {
                                        label: BTN_SEARCH_DESKTOP.to_string()
                                        tokens: tokens
                                        outline()
                                        sm()
                                        on_click: on_open_cmd_desktop
                                    }
                                    row {
                                        bind_style: scheme_btn_style
                                        on_click: on_cycle_scheme
                                        text {
                                            bind_style: scheme_lbl_style
                                            { scheme_text_desktop }
                                        }
                                    }
                                    row {
                                        bind_style: mode_btn_style
                                        on_click: on_toggle_mode_desktop
                                        text {
                                            bind_style: mode_lbl_desktop_style
                                            { mode_text_desktop }
                                        }
                                    }
                                }
                            };
                            let mobile_actions: NodeId = view! {
                                row {
                                    gap: SPACE_XS
                                    align: AlignItems::Center
                                    EuvButton {
                                        label: BTN_SEARCH_MOBILE.to_string()
                                        tokens: tokens
                                        outline()
                                        sm()
                                        on_click: on_open_cmd_mobile
                                    }
                                    row {
                                        bind_style: mode_mobile_style
                                        on_click: on_toggle_mode_mobile
                                        text {
                                            bind_style: mode_lbl_mobile_style
                                            { mode_text_mobile }
                                        }
                                    }
                                }
                            };
                            view! {
                                EuvAppShell {
                                    brand: SHOWCASE_BRAND.to_string()
                                    sidebar: nav
                                    main_content: page_node
                                    tokens: tokens
                                    theme: state_shell.get_theme().clone()
                                    page_padding: page_pad_sig
                                    route: state_shell.get_route()
                                    breadcrumb: breadcrumb_sig
                                    top_bar_actions: top_actions
                                    mobile_header_actions: mobile_actions
                                    bottom_nav: bottom_nav
                                    swipe_routes: swipe_order
                                    show_fps: true
                                }
                            }
                        }
                    }
                };
                vec![global_wasm_slot, cmd_modal]
            }
        }
    };
    let root_ctx: Context = current_context();
    root_ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(root_node, ev_scope);
        },
        (),
    );
    root_node
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
pub(crate) fn mount_route_by_name(
    route: &str,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    match route {
        ROUTE_MARKDOWN => mount_markdown_page(state, tokens),
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
        ROUTE_STRESS => mount_stress_page(state, tokens),
        _ => mount_overview_page(state, tokens),
    }
}

/// Mounts the redesigned flagship overview dashboard with hero banner, KPI cards, live Freeze lifecycle monitor, cross-component event & state demo, and 7-platform matrix.
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
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let route_sig: Signal<String> = state.get_route();
    let frozen_sig: Signal<usize> = state.get_router().get_frozen_count();
    let mounts_sig: Signal<u32> = state.get_lifecycle_mounts();
    let freezes_sig: Signal<u32> = state.get_lifecycle_freezes();
    let resumes_sig: Signal<u32> = state.get_lifecycle_resumes();
    let last_ev_sig: Signal<String> = state.get_lifecycle_last_event();
    let entries_sig: Signal<Vec<EuvRouteLifecycleEntry>> =
        state.get_router().get_lifecycle_entries();
    let local_counter: LocalState<u32> = local_state!(1_u32);
    let global_banner: Signal<ShowcaseGlobalBanner> =
        global_state!(ShowcaseGlobalBanner = ShowcaseGlobalBanner::default());
    let dir_items: [(&str, &str, &str, &str); 14] = [
        (
            ROUTE_MARKDOWN,
            HOME_DIR_TAG_MD,
            NAV_LABEL_MARKDOWN,
            HOME_DIR_DESC_MD,
        ),
        (ROUTE_LAB, HOME_DIR_TAG_LB, NAV_LABEL_LAB, HOME_DIR_DESC_LB),
        (
            ROUTE_STRESS,
            HOME_DIR_TAG_ST,
            NAV_LABEL_STRESS,
            HOME_DIR_DESC_ST,
        ),
        (
            ROUTE_GAME_3D,
            HOME_DIR_TAG_3D,
            NAV_LABEL_GAME_3D,
            HOME_DIR_DESC_3D,
        ),
        (
            ROUTE_FORMS,
            HOME_DIR_TAG_FM,
            NAV_LABEL_FORMS,
            HOME_DIR_DESC_FM,
        ),
        (
            ROUTE_MEDIA,
            HOME_DIR_TAG_ME,
            NAV_LABEL_MEDIA,
            HOME_DIR_DESC_ME,
        ),
        (
            ROUTE_OVERLAYS,
            HOME_DIR_TAG_OV,
            NAV_LABEL_OVERLAYS,
            HOME_DIR_DESC_OV,
        ),
        (
            ROUTE_THEMES,
            HOME_DIR_TAG_TH,
            NAV_LABEL_THEMES,
            HOME_DIR_DESC_TH,
        ),
        (
            ROUTE_DATA,
            HOME_DIR_TAG_DT,
            NAV_LABEL_DATA,
            HOME_DIR_DESC_DT,
        ),
        (
            ROUTE_REACTIVE,
            HOME_DIR_TAG_RX,
            NAV_LABEL_REACTIVE,
            HOME_DIR_DESC_RX,
        ),
        (
            ROUTE_RENDER,
            HOME_DIR_TAG_RN,
            NAV_LABEL_RENDER,
            HOME_DIR_DESC_RN,
        ),
        (
            ROUTE_WINDOW,
            HOME_DIR_TAG_OS,
            NAV_LABEL_WINDOW,
            HOME_DIR_DESC_OS,
        ),
        (
            ROUTE_GAME,
            HOME_DIR_TAG_2D,
            NAV_LABEL_GAME,
            HOME_DIR_DESC_2D,
        ),
        (
            ROUTE_DYNAMIC,
            HOME_DIR_TAG_OT,
            NAV_LABEL_DYNAMIC,
            HOME_DIR_DESC_OT,
        ),
    ];
    let dir_rows: Vec<NodeId> = dir_items
        .iter()
        .map(|(path, tag, label, desc): &(&str, &str, &str, &str)| {
            let target: String = (*path).to_string();
            let badge_txt: String = (*tag).to_string();
            let title_txt: String = (*label).to_string();
            let desc_txt: String = (*desc).to_string();
            let on_dir_click = move |_: &UiEvent| {
                let _: bool = route_sig.set_if_changed(target.clone());
            };
            view! {
                row {
                    width: Length::Percent(100.0)
                    padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                    align: AlignItems::Center
                    justify: JustifyContent::SpaceBetween
                    gap: SPACE_SM
                    radius: g.get_radius_md()
                    background: p.get_background()
                    border: (g.get_border_width(), p.get_border())
                    cursor: CursorIcon::Pointer
                    on_click: on_dir_click
                    row {
                        flex_grow: 1.0
                        flex_shrink: 1.0
                        min_width: Length::Px(0.0)
                        align: AlignItems::Center
                        gap: SPACE_SM
                        overflow: Overflow::Hidden
                        row {
                            width: Length::Px(34.0)
                            height: Length::Px(34.0)
                            flex_shrink: 0.0
                            radius: 99.0
                            background: p.get_primary().with_alpha(26)
                            justify: JustifyContent::Center
                            align: AlignItems::Center
                            text {
                                font_size: 11.5
                                font_family: FontFamily::Monospace
                                font_weight: FontWeight::Bold
                                color: p.get_primary()
                                text_align: TextAlign::Center
                                text_overflow: TextOverflow::Clip
                                { badge_txt }
                            }
                        }
                        column {
                            flex_grow: 1.0
                            flex_shrink: 1.0
                            min_width: Length::Px(0.0)
                            gap: 2.0
                            overflow: Overflow::Hidden
                            text {
                                width: Length::Percent(100.0)
                                font_size: 14.0
                                font_weight: FontWeight::SemiBold
                                color: p.get_foreground()
                                overflow: Overflow::Hidden
                                text_overflow: TextOverflow::Ellipsis
                                { title_txt }
                            }
                            text {
                                width: Length::Percent(100.0)
                                font_size: 12.0
                                color: p.get_muted_foreground()
                                overflow: Overflow::Hidden
                                text_overflow: TextOverflow::Ellipsis
                                { desc_txt }
                            }
                        }
                    }
                    text {
                        flex_shrink: 0.0
                        font_size: 14.0
                        font_weight: FontWeight::Bold
                        color: p.get_muted_foreground()
                        text_overflow: TextOverflow::Clip
                        { HOME_DIR_ARROW }
                    }
                }
            }
        })
        .collect();
    let hero_style: Style = style! {
        FlexDirection::Column
        Overflow::Hidden
        width: Length::Percent(100.0)
        padding: Edges::all(SPACE_XL)
        gap: SPACE_MD
        background: p.get_card()
        border: (g.get_border_width().max(1.0), p.get_primary().with_alpha(96))
        radius: g.get_radius_xl()
        shadow: g.get_shadow_md()
    };
    let on_open_markdown = move |_: &UiEvent| {
        let _: bool = route_sig.set_if_changed(ROUTE_MARKDOWN.to_string());
    };
    let on_open_3d = move |_: &UiEvent| {
        let _: bool = route_sig.set_if_changed(ROUTE_GAME_3D.to_string());
    };
    let on_open_stress = move |_: &UiEvent| {
        let _: bool = route_sig.set_if_changed(ROUTE_STRESS.to_string());
    };
    let on_open_reactive = move |_: &UiEvent| {
        let _: bool = route_sig.set_if_changed(ROUTE_REACTIVE.to_string());
    };
    let lifecycle_stats_text = move || -> String {
        format!(
            "Frozen Pages: {} | Mounts: {} | Freezes: {} | Resumes: {}",
            frozen_sig.get(),
            mounts_sig.get(),
            freezes_sig.get(),
            resumes_sig.get()
        )
    };
    let last_hook_text = move || -> String { format!("Last Hook: {}", last_ev_sig.get()) };
    let cached_routes_text = move || -> String {
        let entries: Vec<EuvRouteLifecycleEntry> = entries_sig.get();
        let summary: Vec<String> = entries
            .iter()
            .map(|e: &EuvRouteLifecycleEntry| format!("{}={:?}", e.get_path(), e.get_phase()))
            .collect();
        if summary.is_empty() {
            LIFECYCLE_ACTIVE_DEFAULT.to_string()
        } else {
            format!("Cached Routes: {}", summary.join(" | "))
        }
    };
    let global_local_text = move || -> String {
        let gb: ShowcaseGlobalBanner = global_banner.get();
        let last_src: &str = if gb.get_last_source().is_empty() {
            SOURCE_NONE
        } else {
            gb.get_last_source().as_str()
        };
        format!(
            "GlobalState Pulses: {} (source: {}) | LocalState (preserved across Freeze): {}",
            gb.get_pulses(),
            last_src,
            local_counter.get()
        )
    };
    let on_emit_pulse = move |_: &UiEvent| {
        let seq: u32 = local_counter.get_untracked();
        emit!(ShowcasePulseEvent {
            source: ROUTE_OVERVIEW.to_string(),
            seq,
        });
    };
    let on_local_inc = move |_: &UiEvent| {
        let _: bool = local_counter.update(|v: &mut u32| *v += 1);
    };
    let on_open_game = move |_: &UiEvent| {
        let _: bool = route_sig.set_if_changed(ROUTE_GAME.to_string());
    };
    let on_open_window = move |_: &UiEvent| {
        let _: bool = route_sig.set_if_changed(ROUTE_WINDOW.to_string());
    };
    view! {
        EuvPage {
            title: OVERVIEW_TITLE.to_string()
            subtitle: OVERVIEW_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: HOME_DIR_TITLE.to_string()
                tokens: tokens
                subtitle: HOME_DIR_SUBTITLE
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    ..dir_rows
                }
            }
            view {
                style: hero_style
                row {
                    width: Length::Percent(100.0)
                    flex_wrap: FlexWrap::Wrap
                    align: AlignItems::Center
                    gap: SPACE_XS
                    EuvBadge {
                        label: HERO_BADGE_VER.to_string()
                        tone: EuvTone::Primary
                        tokens: tokens
                    }
                    EuvBadge {
                        label: HERO_BADGE_ZERO_WV.to_string()
                        tone: EuvTone::Success
                        tokens: tokens
                    }
                    EuvBadge {
                        label: HERO_BADGE_EVA.to_string()
                        tone: EuvTone::Neutral
                        tokens: tokens
                    }
                    EuvBadge {
                        label: HERO_BADGE_FREEZE.to_string()
                        tone: EuvTone::Warning
                        tokens: tokens
                    }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 20.0
                    font_weight: FontWeight::Bold
                    color: p.get_foreground()
                    { HERO_HEADLINE }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 13.5
                    color: p.get_muted_foreground()
                    { HERO_DESCRIPTION }
                }
                EuvActionRow {
                    EuvButton {
                        label: BTN_OPEN_MARKDOWN.to_string()
                        tokens: tokens
                        on_click: on_open_markdown
                    }
                    EuvButton {
                        label: BTN_OPEN_3D_RT.to_string()
                        tokens: tokens
                        secondary()
                        on_click: on_open_3d
                    }
                    EuvButton {
                        label: BTN_OPEN_STRESS_LAB.to_string()
                        tokens: tokens
                        outline()
                        on_click: on_open_stress
                    }
                    EuvButton {
                        label: BTN_TRY_REACTIVITY.to_string()
                        tokens: tokens
                        outline()
                        on_click: on_open_reactive
                    }
                }
            }
            EuvStatGrid {
                EuvStatCard {
                    label: STAT_PLATFORMS_LABEL.to_string()
                    value: STAT_PLATFORMS_VAL.to_string()
                    caption: STAT_PLATFORMS_CAP.to_string()
                    tokens: tokens
                }
                EuvStatCard {
                    label: STAT_SCHEMES_LABEL.to_string()
                    value: STAT_SCHEMES_VAL.to_string()
                    caption: STAT_SCHEMES_CAP.to_string()
                    tokens: tokens
                }
                EuvStatCard {
                    label: STAT_RENDER_LABEL.to_string()
                    value: STAT_RENDER_VAL.to_string()
                    caption: STAT_RENDER_CAP.to_string()
                    tokens: tokens
                }
            }
            EuvCard {
                title: CARD_LIFECYCLE_TITLE.to_string()
                tokens: tokens
                subtitle: CARD_LIFECYCLE_SUB
                text {
                    width: Length::Percent(100.0)
                    font_size: 13.5
                    font_weight: FontWeight::SemiBold
                    color: p.get_primary()
                    { lifecycle_stats_text }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    font_family: FontFamily::Monospace
                    color: p.get_muted_foreground()
                    { last_hook_text }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    color: p.get_foreground()
                    { cached_routes_text }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    font_weight: FontWeight::Medium
                    color: p.get_success()
                    { global_local_text }
                }
                EuvActionRow {
                    EuvButton {
                        label: BTN_EMIT_PULSE.to_string()
                        tokens: tokens
                        sm()
                        on_click: on_emit_pulse
                    }
                    EuvButton {
                        label: BTN_LOCAL_INC.to_string()
                        tokens: tokens
                        secondary()
                        sm()
                        on_click: on_local_inc
                    }
                    EuvButton {
                        label: BTN_OPEN_GAME_ENGINE.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: on_open_game
                    }
                    EuvButton {
                        label: BTN_OPEN_OS_SERVICES.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: on_open_window
                    }
                }
            }
            EuvCard {
                title: CARD_PLATFORMS_TITLE.to_string()
                tokens: tokens
                subtitle: CARD_PLATFORMS_SUB
                EuvTable {
                    headers: vec![
                        TABLE_COL_TARGET.to_string(),
                        TABLE_COL_BACKEND.to_string(),
                        TABLE_COL_GPU.to_string(),
                    ]
                    rows: vec![
                        vec![ROW_WEB.to_string(), ROW_WEB_API.to_string(), ROW_WEB_GPU.to_string()],
                        vec![ROW_MACOS.to_string(), ROW_MACOS_API.to_string(), ROW_MACOS_GPU.to_string()],
                        vec![ROW_IOS.to_string(), ROW_IOS_API.to_string(), ROW_MACOS_GPU.to_string()],
                        vec![ROW_ANDROID.to_string(), ROW_ANDROID_API.to_string(), ROW_ANDROID_GPU.to_string()],
                        vec![ROW_WINDOWS.to_string(), ROW_WINDOWS_API.to_string(), ROW_WINDOWS_GPU.to_string()],
                        vec![ROW_LINUX.to_string(), ROW_LINUX_API.to_string(), ROW_ANDROID_GPU.to_string()],
                    ]
                    tokens: tokens
                }
            }
        }
    }
}

/// Mounts the Markdown & Code Studio page showcasing exact VS Code `Eva Light Bold` / `Eva Dark Bold` `EuvCodeBlock` and the complete native GFM Markdown component suite.
///
/// # Arguments
///
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted Markdown & Code Studio page handle.
pub fn mount_markdown_page(state: &ShowcaseState, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let eva: CodeThemePalette = CodeThemePalette::resolve(tokens.get_mode());
    let st_toggle: ShowcaseState = state.clone();
    let theme_mode_sig: Signal<ThemeMode> = state.get_theme().get_mode();
    let lang_tab: Signal<usize> = state.get_markdown_lang_tab();
    let selectable_sig: Signal<bool> = state.get_markdown_selectable();
    let copy_log: Signal<String> = state.get_markdown_copy_log();
    let link_log: Signal<String> = state.get_markdown_link_log();
    let tasks_toggled: Signal<u32> = state.get_markdown_tasks_toggled();
    let copy_log_doc: Signal<String> = copy_log;
    let link_log_doc: Signal<String> = link_log;
    let tasks_doc: Signal<u32> = tasks_toggled;
    let active_theme_desc = move || -> String {
        match theme_mode_sig.get() {
            ThemeMode::Light => MD_THEME_DESC_LIGHT.to_string(),
            ThemeMode::Dark => MD_THEME_DESC_DARK.to_string(),
        }
    };
    let swatch_nodes: Vec<NodeId> = (0usize..6usize)
        .map(|slot: usize| {
            let swatch_style = move || -> Style {
                let live_eva: CodeThemePalette = CodeThemePalette::resolve(theme_mode_sig.get());
                let col: Color = match slot {
                    0 => live_eva.get_background(),
                    1 => live_eva.get_keyword(),
                    2 => live_eva.get_function(),
                    3 => live_eva.get_number(),
                    4 => live_eva.get_constant(),
                    _ => live_eva.get_property(),
                };
                Style::default()
                    .width(Length::Px(22.0))
                    .height(Length::Px(22.0))
                    .radius(6.0)
                    .background(col)
                    .border(1.0, live_eva.get_border())
            };
            view! {
                view {
                    bind_style: swatch_style
                }
            }
        })
        .collect();
    let on_toggle_eva = move |_: &UiEvent| {
        st_toggle.toggle_theme_mode();
    };
    let on_emit_pulse = move |_: &UiEvent| {
        emit!(ShowcasePulseEvent {
            source: ROUTE_MARKDOWN.to_string(),
            seq: 1,
        });
    };
    let build_code_tab = move || -> Vec<NodeId> {
        let tab: usize = lang_tab.get();
        let is_sel: bool = selectable_sig.get();
        let live_tok: ThemeTokens = live_tokens_untracked(tokens);
        let (src, lang, fname): (&str, &str, &str) = match tab {
            1 => (MD_SAMPLE_TS, MD_LANG_TS, MD_FILE_TS),
            2 => (MD_SAMPLE_JSON, MD_LANG_JSON, MD_FILE_JSON),
            3 => (MD_SAMPLE_DIFF, MD_LANG_DIFF, MD_FILE_DIFF),
            _ => (MD_SAMPLE_RUST, MD_LANG_RUST, MD_FILE_RUST),
        };
        let cb_log: Signal<String> = copy_log;
        let block: NodeId = EuvCodeBlock::new(src.to_string(), live_tok)
            .language(lang)
            .filename(fname)
            .line_numbers(true)
            .max_height(260.0)
            .selectable(is_sel)
            .on_copy(move |txt: &str| {
                let _: bool = cb_log.set(format!(
                    "Copied {} bytes ({lang}) via Eva Bold EuvCodeBlock",
                    txt.len()
                ));
            })
            .mount();
        vec![block]
    };
    let telemetry_text = move || -> String {
        format!(
            "Copy: {} | Link: {} | Tasks Toggled: {} | Selectable: {}",
            copy_log.get(),
            link_log.get(),
            tasks_toggled.get(),
            selectable_sig.get()
        )
    };
    let build_md_doc = move || -> Vec<NodeId> {
        let is_sel: bool = selectable_sig.get();
        let live_tok: ThemeTokens = live_tokens_untracked(tokens);
        let doc_node: NodeId = EuvMarkdown::new(MD_FULL_DOCUMENT.to_string(), live_tok)
            .selectable(is_sel)
            .on_link(move |url: &str| {
                let _: bool = link_log_doc.set(url.to_string());
            })
            .on_copy(move |code: &str| {
                let _: bool =
                    copy_log_doc.set(format!("Copied {} bytes from Markdown block", code.len()));
            })
            .on_toggle_task(move |_idx: usize, _checked: bool| {
                let _: bool = tasks_doc.update(|n: &mut u32| *n += 1);
            })
            .mount();
        vec![doc_node]
    };
    view! {
        EuvPage {
            title: MD_STUDIO_TITLE.to_string()
            subtitle: MD_STUDIO_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: MD_THEME_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: MD_THEME_CARD_SUB
                text {
                    font_size: 12.5
                    font_family: FontFamily::Monospace
                    font_weight: FontWeight::SemiBold
                    color: eva.get_keyword()
                    { active_theme_desc }
                }
                row {
                    width: Length::Percent(100.0)
                    align: AlignItems::Center
                    flex_wrap: FlexWrap::Wrap
                    gap: SPACE_SM
                    ..swatch_nodes
                    EuvButton {
                        label: BTN_TOGGLE_EVA_MODE.to_string()
                        tokens: tokens
                        sm()
                        on_click: on_toggle_eva
                    }
                    EuvButton {
                        label: BTN_EMIT_PULSE.to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: on_emit_pulse
                    }
                    EuvSwitch {
                        checked: selectable_sig
                        label: MD_SELECTABLE_LABEL.to_string()
                        tokens: tokens
                    }
                }
            }
            EuvCard {
                title: MD_CODE_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: MD_CODE_CARD_SUB
                EuvSegmented {
                    options: vec![
                        MD_TAB_RUST.to_string(),
                        MD_TAB_TS.to_string(),
                        MD_TAB_JSON.to_string(),
                        MD_TAB_DIFF.to_string(),
                    ]
                    selected: lang_tab
                    tokens: tokens
                }
                column {
                    width: Length::Percent(100.0)
                    bind_children: build_code_tab
                }
                text {
                    font_size: 12.0
                    font_family: FontFamily::Monospace
                    color: p.get_primary()
                    { telemetry_text }
                }
            }
            EuvCard {
                title: MD_DOC_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: MD_DOC_CARD_SUB
                column {
                    width: Length::Percent(100.0)
                    bind_children: build_md_doc
                }
            }
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
    view! {
        EuvPage {
            title: REACTIVE_TITLE.to_string()
            subtitle: REACTIVE_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: REACTIVE_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: REACTIVE_CARD_SUB
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
                    EuvActionRow {
                        EuvButton {
                            label: BTN_INCREMENT.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
                                let _: bool = counter.update(|v: &mut i32| *v += 1);
                            }
                        }
                        EuvButton {
                            label: BTN_BATCH_TEN.to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
                                batch(|| {
                                    for _ in 0..10 {
                                        let _: bool = counter.update(|v: &mut i32| *v += 1);
                                    }
                                });
                            }
                        }
                        EuvButton {
                            label: BTN_RESET.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = counter.set_if_changed(0);
                            }
                        }
                    }
                }
            }
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
    view! {
        EuvPage {
            title: RENDER_TITLE.to_string()
            subtitle: RENDER_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: RENDER_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: RENDER_CARD_SUB
                EuvSegmented {
                    options: vec![
                        MODE_AUTO.to_string(),
                        MODE_SOFTWARE.to_string(),
                        MODE_HARDWARE.to_string(),
                        MODE_HYBRID.to_string(),
                    ]
                    selected: mode_idx
                    tokens: tokens
                }
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
            }
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
    let ctx_win: Context = current_context();
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
    let fps_limit_sig: Signal<Option<u32>> = ctx_win.get_fps_limit();
    let live_fps_sig: Signal<f32> = ctx_win.get_live_fps();
    let native_hz_sig: Signal<f32> = ctx_win.get_display_refresh_rate();
    let fps_idx: Signal<usize> = Signal::new(match fps_limit_sig.get_untracked() {
        Some(30) => 1,
        Some(60) => 2,
        Some(120) => 3,
        Some(144) => 4,
        Some(240) => 5,
        _ => 0,
    });
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
    let mut body_id: NodeId = NodeId::default();
    let mut fps_card_id: NodeId = NodeId::default();
    let app_state_sig: Signal<AppState> = use_app_state();
    let sys_theme_sig: Signal<SystemTheme> = use_system_theme();
    let net_status_sig: Signal<NetworkStatus> = use_network_status();
    let kb_height_sig: Signal<f32> = use_keyboard_height();
    let safe_area_sig: Signal<SafeAreaInsets> = use_safe_area();
    let sys_log_sig: Signal<Vec<String>> = state.get_system_event_log();
    let sys_count_sig: Signal<u32> = state.get_system_event_count();
    let bg_frozen_sig: Signal<bool> = state.get_bg_frozen();
    let bg_freeze_cnt: Signal<u32> = state.get_bg_freeze_count();
    let bg_restore_cnt: Signal<u32> = state.get_bg_restore_count();
    let page_appear_count: Signal<u32> = Signal::new(0);
    let page_disappear_count: Signal<u32> = Signal::new(0);
    let _: SystemEventSubscription = on_appear(move || {
        let _: bool = page_appear_count.update(|n: &mut u32| *n = n.saturating_add(1));
    });
    let _: SystemEventSubscription = on_disappear(move || {
        let _: bool = page_disappear_count.update(|n: &mut u32| *n = n.saturating_add(1));
    });
    let sys_primary_line = move || -> String {
        format!(
            "AppState: {:?} | OS Theme: {:?} | Network: {:?}",
            app_state_sig.get(),
            sys_theme_sig.get(),
            net_status_sig.get()
        )
    };
    let sys_freeze_line = move || -> String {
        let mode: &str = if bg_frozen_sig.get() {
            BG_FREEZE_STATUS_FROZEN
        } else {
            BG_FREEZE_STATUS_ACTIVE
        };
        format!(
            "Background Freeze: {mode} | Freezes: {} | Restores: {}",
            bg_freeze_cnt.get(),
            bg_restore_cnt.get()
        )
    };
    let sys_secondary_line = move || -> String {
        let sa: SafeAreaInsets = safe_area_sig.get();
        format!(
            "Keyboard: {:.0}px | SafeArea: T{:.0} R{:.0} B{:.0} L{:.0} | Page Appear: {} / Disappear: {} | Total Events: {}",
            kb_height_sig.get(),
            sa.get_top(),
            sa.get_right(),
            sa.get_bottom(),
            sa.get_left(),
            page_appear_count.get(),
            page_disappear_count.get(),
            sys_count_sig.get()
        )
    };
    let build_sys_log_rows = move || -> Vec<NodeId> {
        let live_tok: ThemeTokens = live_tokens_untracked(tokens);
        let live_p: ThemePalette = live_tok.get_palette();
        sys_log_sig
            .get()
            .into_iter()
            .map(|entry: String| {
                view! {
                    text {
                        width: Length::Percent(100.0)
                        font_size: 12.0
                        font_family: FontFamily::Monospace
                        color: live_p.get_muted_foreground()
                        { entry }
                    }
                }
            })
            .collect()
    };
    let page: NodeId = view! {
        EuvPage {
            title: WINDOW_TITLE.to_string()
            subtitle: WINDOW_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: SYS_EVENTS_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: SYS_EVENTS_CARD_SUB
                selectable: true
                text {
                    width: Length::Percent(100.0)
                    font_size: 13.5
                    font_weight: FontWeight::SemiBold
                    color: p.get_primary()
                    { sys_primary_line }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    font_weight: FontWeight::Medium
                    color: p.get_success()
                    { sys_freeze_line }
                }
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    font_family: FontFamily::Monospace
                    color: p.get_foreground()
                    { sys_secondary_line }
                }
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_SM)
                    gap: 4.0
                    background: p.get_muted()
                    radius: tokens.get_geometry().get_radius_md()
                    overflow: Overflow::Hidden
                    bind_children: build_sys_log_rows
                }
            }
            EuvCard {
                title: WINDOW_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: WINDOW_CARD_SUB
                column {
                    node_ref: body_id
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
                    EuvActionRow {
                        EuvButton {
                            label: "Toggle Dynamic Island".to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
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
                                    let _: bool = native_feedback.set("Dynamic Island session ended".to_string());
                                }
                            }
                        }
                        EuvButton {
                            label: "Register euv:// Protocol".to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Request Camera Permission".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let mut mgr: PermissionManager = PermissionManager::new();
                                let desc: PermissionDescriptor = mgr.request(
                                    SystemPermission::Camera,
                                    "Native camera preview".to_string(),
                                    platform,
                                );
                                let msg: String = format!("{:?} ({})", desc.get_status(), desc.get_native_key());
                                let _: bool = cam_status.set_if_changed(msg.clone());
                                let _: bool = native_feedback.set(format!("Permission requested: {msg}"));
                            }
                        }
                        EuvButton {
                            label: "Announce A11y + Haptic".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                    }
                    EuvSwitch {
                        checked: fs_sig
                        label: SWITCH_FULLSCREEN.to_string()
                        tokens: tokens
                    }
                    EuvCodeBlock {
                        code: CODE_WINDOW_SNIPPET.to_string()
                        tokens: tokens
                    }
                }
            }
            EuvCard {
                title: "Cross-Platform Frame Rate Limiter".to_string()
                tokens: tokens
                node_ref: fps_card_id
                subtitle: "Prioritizes native hardware refresh rate (120Hz/144Hz/180Hz+) across Desktop, Web, Android & iOS"
                EuvSegmented {
                    options: vec![
                        "Native Auto".to_string(),
                        "30 FPS".to_string(),
                        "60 FPS".to_string(),
                        "120 FPS".to_string(),
                        "144 FPS".to_string(),
                        "240 FPS".to_string(),
                    ]
                    selected: fps_idx
                    tokens: tokens
                }
                text {
                    font_size: 13.0
                    font_weight: FontWeight::SemiBold
                    color: p.get_primary()
                    { move || {
                        let native_hz: f32 = native_hz_sig.get();
                        let cap_txt: String = match fps_limit_sig.get() {
                            None => format!("Native Auto ({native_hz:.0} Hz)"),
                            Some(v) => format!("Capped at {v} FPS"),
                        };
                        let live_fps: f32 = live_fps_sig.get();
                        if !live_fps.is_finite() || live_fps <= 0.05 {
                            format!("Frame Rate Policy: {cap_txt} | Status: Idle (0.0 FPS)")
                        } else {
                            format!(
                                "Frame Rate Policy: {cap_txt} | Live Measured: {live_fps:.1} FPS"
                            )
                        }
                    } }
                }
            }
            EuvCard {
                title: "Cross-Platform Dynamic Island & Notifications".to_string()
                tokens: tokens
                subtitle: "Test Live Activities across all 7 platforms and dispatch native OS notifications"
                EuvSegmented {
                    options: vec![
                        "iOS".to_string(),
                        "iPadOS".to_string(),
                        LABEL_PLATFORM_ANDROID.to_string(),
                        LABEL_PLATFORM_MACOS.to_string(),
                        "Windows".to_string(),
                        "Linux".to_string(),
                        "Web".to_string(),
                    ]
                    selected: target_plat_idx
                    tokens: tokens
                }
                EuvSegmented {
                    options: vec![
                        "Compact".to_string(),
                        "Minimal".to_string(),
                        "Expanded".to_string(),
                        "LockScreen".to_string(),
                        "FallbackNotif".to_string(),
                        "StatusBar".to_string(),
                    ]
                    selected: island_mode_idx
                    tokens: tokens
                }
                column {
                    width: Length::Percent(100.0)
                    padding: Edges::all(SPACE_MD)
                    gap: SPACE_XS
                    background: Color::hex(0x09090B)
                    radius: 20.0
                    text {
                        font_size: 12.0
                        font_weight: FontWeight::SemiBold
                        color: Color::hex(0xF8FAFC)
                        { move || {
                            let tp: PlatformKind = all_platforms[target_plat_idx.get() % 7];
                            let im: IslandPresentationMode = all_island_modes[island_mode_idx.get() % 6];
                            let def_im: IslandPresentationMode = PlatformServiceHub::default_island_mode(tp);
                            let active: bool = island_active.get();
                            let pct: u8 = island_progress.get();
                            let state_tag: &str = if active { "LIVE" } else { "IDLE" };
                            format!(
                                "[{state_tag}] Dynamic Island ({tp:?}) | Mode: {im:?} (OS Default: {def_im:?}) | Progress: {pct}%"
                            )
                        } }
                    }
                }
                EuvActionRow {
                    EuvButton {
                        label: "Trigger Alert Notification".to_string()
                        tokens: tokens
                        sm()
                        on_click: move |_: &UiEvent| {
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
                        }
                    }
                    EuvButton {
                        label: "Trigger Silent + Badge(3)".to_string()
                        tokens: tokens
                        secondary()
                        sm()
                        on_click: move |_: &UiEvent| {
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
                        }
                    }
                    EuvButton {
                        label: "Clear Notifications".to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = sys_notifs.set_if_changed(Vec::new());
                        }
                    }
                }
                EuvSonner {
                    messages: sys_notifs
                    tokens: tokens
                }
            }
            EuvCard {
                title: "Complete 14-Category OS Permissions Matrix".to_string()
                tokens: tokens
                subtitle: "Inspect per-platform native entitlement keys and test Grant / Deny flows"
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    bind_children: move || {
                        let tp: PlatformKind = all_platforms[target_plat_idx.get() % 7];
                        let statuses: Vec<PermissionStatus> = perm_statuses.get();
                        let g: ThemeGeometry = tokens.get_geometry();
                        all_perms
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|(idx, perm): (usize, SystemPermission)| {
                            let st: PermissionStatus = statuses
                            .get(idx)
                            .copied()
                            .unwrap_or(PermissionStatus::NotDetermined);
                            let native_key: &'static str = perm.native_key(tp);
                            view! {
                                column {
                                    width: Length::Percent(100.0)
                                    padding: Edges::symmetric(SPACE_SM, SPACE_SM)
                                    gap: SPACE_XS
                                    background: p.get_card()
                                    border: (g.get_border_width(), p.get_border())
                                    radius: g.get_radius_sm()
                                    text {
                                        width: Length::Percent(100.0)
                                        font_size: 12.0
                                        color: p.get_foreground()
                                        { format!("{perm:?} [{st:?}] · {native_key}") }
                                    }
                                    EuvActionRow {
                                        EuvButton {
                                            label: "Request".to_string()
                                            tokens: tokens
                                            outline()
                                            sm()
                                            on_click: move |_: &UiEvent| {
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
                                            }
                                        }
                                        EuvButton {
                                            label: "Deny".to_string()
                                            tokens: tokens
                                            destructive()
                                            sm()
                                            on_click: move |_: &UiEvent| {
                                                let _: bool = perm_statuses.update(|vec: &mut Vec<PermissionStatus>| {
                                                    if let Some(slot) = vec.get_mut(idx) {
                                                        *slot = PermissionStatus::Denied;
                                                    }
                                                });
                                                let _: bool = native_feedback.set(format!("Marked {perm:?} as Denied"));
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .collect()
                    }
                }
            }
        }
    };
    ctx_win.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(body_id, fs_scope);
            tree.attach_scope(fps_card_id, fps_scope);
        },
        (),
    );
    page
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
    let ctx: Context = current_context();
    let chart_data: Vec<EuvChartPoint> = vec![
        EuvChartPoint::new("Mon".to_string(), 60.0),
        EuvChartPoint::new("Tue".to_string(), 88.0),
        EuvChartPoint::new("Wed".to_string(), 118.0),
        EuvChartPoint::new("Thu".to_string(), 104.0),
        EuvChartPoint::new("Fri".to_string(), 120.0),
        EuvChartPoint::new("Sat".to_string(), 116.0),
    ];
    let left_split: NodeId = view! {
        text {
            font_size: 13.0
            color: p.get_foreground()
            "Left Split Pane (Drag handle ->)"
        }
    };
    let right_split: NodeId = view! {
        text {
            font_size: 13.0
            color: p.get_muted_foreground()
            "Right Split Pane (Zero rebuild on drag)"
        }
    };
    let rtl_sig: Signal<bool> = state.get_lab_rtl_mode();
    let block_scroll: Signal<bool> = state.get_lab_block_scroll_chain();
    let stop_bubble: Signal<bool> = state.get_lab_stop_click_bubble();
    let parent_clicks: Signal<u32> = state.get_lab_parent_clicks();
    let child_clicks: Signal<u32> = state.get_lab_child_clicks();
    let preserve_cnt: Signal<u32> = state.get_lab_resize_preserve_count();
    let modal_open: Signal<bool> = state.get_modal_open();
    let drawer_open: Signal<bool> = state.get_drawer_open();
    let attach_vis: Signal<bool> = Signal::new(true);
    let q_sel: Signal<Option<usize>> = Signal::new(Some(0));
    let touches_sig: Signal<Vec<(u64, Point)>> = ctx.get_active_touches();
    let pad_bounds_cell: Rc<Cell<Rect>> = Rc::new(Cell::new(Rect::default()));
    let pad_bounds_memo: Rc<Cell<Rect>> = Rc::clone(&pad_bounds_cell);
    let pad_bounds_paint: Rc<Cell<Rect>> = Rc::clone(&pad_bounds_cell);
    let touch_ids_memo: Memo<Vec<u64>> = Memo::new(move || {
        let pad_rect: Rect = pad_bounds_memo.get();
        if pad_rect.get_width() <= 0.0 || pad_rect.get_height() <= 0.0 {
            return Vec::new();
        }
        touches_sig
            .get()
            .into_iter()
            .filter(|(_, pt): &(u64, Point)| pad_rect.contains(*pt))
            .map(|(id, _): (u64, Point)| id)
            .collect()
    });
    let ai_tick: Signal<u32> = Signal::new(0);
    let ai_fps_sig: Signal<f32> = ctx.get_live_fps();
    let ai_route_sig: Signal<String> = state.get_route();
    let ai_scheme_sig: Signal<DesignScheme> = state.get_theme().get_scheme();
    let mut touch_pad_id: NodeId = NodeId::default();
    let page: NodeId = view! {
        EuvPage {
            title: "UI & Edge-Case Lab".to_string()
            subtitle: "Full shadcn/ui component parity + multi-touch, propagation, overlay stack, AI Schema v1, and resize-preservation tests".to_string()
            tokens: tokens
            EuvCard {
                title: "Multi-Touch & Gesture Isolation Pad".to_string()
                tokens: tokens
                subtitle: "Simultaneous multi-finger tracking with pointer IDs, pinch distance line, and gesture non-penetration"
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(22.0)
                    align: AlignItems::Center
                    overflow: Overflow::Hidden
                    text {
                        width: Length::Percent(100.0)
                        font_size: 12.5
                        font_weight: FontWeight::Medium
                        color: p.get_primary()
                        overflow: Overflow::Hidden
                        text_overflow: TextOverflow::Ellipsis
                        { move || {
                            let ids: Vec<u64> = touch_ids_memo.get();
                            if ids.is_empty() {
                                TOUCH_PAD_READY_TEXT.to_string()
                            } else {
                                let labels: String = ids
                                .iter()
                                .map(|id: &u64| format!("#{id}"))
                                .collect::<Vec<String>>()
                                .join(" ");
                                format!("Inside Pad: {} | Pointers: {labels}", ids.len())
                            }
                        } }
                    }
                }
                canvas {
                    node_ref: touch_pad_id
                    width: Length::Percent(100.0)
                    height: Length::Px(170.0)
                    background: p.get_muted()
                    border: (g.get_border_width(), p.get_border())
                    radius: g.get_radius_md()
                    stop_scroll_propagation: true
                    stop_event_propagation: true
                    painter: move |bounds: Rect, dl: &mut DisplayList| {
                        pad_bounds_paint.set(bounds);
                        let pts: Vec<(u64, Point)> = touches_sig.get_untracked();
                        let palette: [Color; 5] = [
                            Color::hex(0x2563EB),
                            Color::hex(0x16A34A),
                            Color::hex(0xEA580C),
                            Color::hex(0x9333EA),
                            Color::hex(0xDC2626),
                        ];
                        let inside_pts: Vec<(u64, Point)> = pts
                        .into_iter()
                        .filter(|(_, pt): &(u64, Point)| bounds.contains(*pt))
                        .collect();
                        if inside_pts.is_empty() {
                            let mut hint_style: TextStyle = TextStyle::default();
                            hint_style
                            .set_font_size(12.0)
                            .set_color(p.get_muted_foreground())
                            .set_align(TextAlign::Center);
                            dl.push(DrawCommand::TextRun {
                                rect: Rect::new(
                                    bounds.get_x() + 12.0,
                                    bounds.get_y() + (bounds.get_height() - 18.0) * 0.5,
                                    (bounds.get_width() - 24.0).max(1.0),
                                    20.0,
                                ),
                                text: TOUCH_PAD_CANVAS_HINT.to_string(),
                                style: hint_style,
                            });
                            return;
                        }
                        if inside_pts.len() >= 2 {
                            for pair in inside_pts.windows(2) {
                                dl.push(DrawCommand::Line {
                                    from: pair[0].1,
                                    to: pair[1].1,
                                    color: p.get_primary().multiply_alpha(0.75),
                                    width: 2.0,
                                });
                            }
                        }
                        for (idx, (pid, pt)) in inside_pts.iter().enumerate() {
                            let col: Color = palette[idx % palette.len()];
                            dl.push(DrawCommand::Circle {
                                center: *pt,
                                radius: 26.0,
                                color: col.multiply_alpha(0.24),
                            });
                            dl.push(DrawCommand::Circle {
                                center: *pt,
                                radius: 10.0,
                                color: col,
                            });
                            let mut tag_style: TextStyle = TextStyle::default();
                            tag_style
                            .set_font_size(11.0)
                            .set_weight(FontWeight::Bold)
                            .set_color(col);
                            dl.push(DrawCommand::TextRun {
                                rect: Rect::new(pt.get_x() + 14.0, pt.get_y() - 24.0, 96.0, 16.0),
                                text: format!("#{pid}({:.0},{:.0})", pt.get_x(), pt.get_y()),
                                style: tag_style,
                            });
                        }
                        let mut hud_style: TextStyle = TextStyle::default();
                        hud_style
                        .set_font_size(11.5)
                        .set_weight(FontWeight::Medium)
                        .set_color(p.get_foreground());
                        dl.push(DrawCommand::TextRun {
                            rect: Rect::new(
                                bounds.get_x() + 10.0,
                                bounds.get_y() + 8.0,
                                (bounds.get_width() - 20.0).max(1.0),
                                18.0,
                            ),
                            text: format!("Live Contacts Inside Pad: {}", inside_pts.len()),
                            style: hud_style,
                        });
                    }
                }
            }
            EuvCard {
                title: "AI Schema v1 & Reactive Telemetry Bridge".to_string()
                tokens: tokens
                subtitle: "Live introspection of SceneTree bounds, A11y tree, reactive graph, FPS, and interactive agent actions"
                text {
                    width: Length::Percent(100.0)
                    font_size: 12.5
                    color: p.get_foreground()
                    { move || {
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
                        let ai_fps: f32 = ai_fps_sig.get();
                        let fps_display: String = if !ai_fps.is_finite() || ai_fps <= 0.05 {
                            LABEL_FPS_IDLE.to_string()
                        } else {
                            format!("{ai_fps:.1}")
                        };
                        format!(
                            "Protocol: {} ({}) | Route: {} | Scheme: {} | Live FPS: {} | Signals: {} | Memos: {} | Effects: {} | Scopes: {} | Clock: {}",
                            proto_label,
                            bridge_label,
                            ai_route_sig.get(),
                            ai_scheme_sig.get().as_str(),
                            fps_display,
                            rs.get_live_signals(),
                            rs.get_live_memos(),
                            rs.get_live_effects(),
                            rs.get_live_scopes(),
                            rs.get_clock()
                        )
                    } }
                }
                EuvActionRow {
                    EuvButton {
                        label: "Refresh AI Telemetry".to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = ai_tick.update(|v: &mut u32| *v = v.wrapping_add(1));
                        }
                    }
                }
                EuvCodeBlock {
                    code: "GET /ai/schema | /ai/snapshot | /ai/a11y | /ai/health\nPOST /ai/query | /ai/action | /ai/script (dev-only; 0B in release)".to_string()
                    tokens: tokens
                }
            }
            EuvCard {
                title: "Complete shadcn/ui Component Parity".to_string()
                tokens: tokens
                subtitle: "Trace, Attachment, Bubble, Marker, Questionnaire, Chart, Combobox, DataTable, DatePicker, InputGroup, Item, NativeSelect, NavigationMenu, Resizable, Separator, Direction, Avatar, Kbd, Spinner, Skeleton, Breadcrumb, Empty"
                row {
                    style: c_action_row()
                    EuvMarker {
                        code: "01".to_string()
                        label: "Zero WebView".to_string()
                        tone: EuvTone::Primary
                        tokens: tokens
                    }
                    EuvMarker {
                        code: "02".to_string()
                        label: "120+ FPS".to_string()
                        tone: EuvTone::Success
                        tokens: tokens
                    }
                    EuvMarker {
                        code: "03".to_string()
                        label: "Multi-Touch".to_string()
                        tone: EuvTone::Warning
                        tokens: tokens
                    }
                    EuvAvatar {
                        initials: "EU".to_string()
                        size: 32.0
                        tokens: tokens
                    }
                    EuvKbd {
                        key: "Cmd+K".to_string()
                        tokens: tokens
                    }
                    EuvSpinner {
                        size: 18.0
                        tokens: tokens
                    }
                }
                EuvTrace {
                    title: "Frame Pipeline Trace (1.42ms Total)".to_string()
                    spans: vec![
                        EuvTraceSpan::new("layout.compute".to_string(), 0.14, 0.0, 0.12, EuvTone::Primary),
                        EuvTraceSpan::new("bvh.raytrace_gi".to_string(), 0.88, 0.12, 0.60, EuvTone::Success),
                        EuvTraceSpan::new("fxaa.tonemap".to_string(), 0.24, 0.72, 0.16, EuvTone::Warning),
                        EuvTraceSpan::new("present.blit".to_string(), 0.16, 0.88, 0.12, EuvTone::Neutral),
                    ]
                    tokens: tokens
                }
                EuvAttachment {
                    filename: "vice-boulevard-rt.wasm".to_string()
                    meta: "48.2 KB · application/wasm · SHA-verified".to_string()
                    visible: attach_vis
                    tokens: tokens
                }
                EuvBubble {
                    author: "Euv Runtime".to_string()
                    content: "All 60+ shadcn/ui components rendered directly via Rust SceneTree with zero WebView.".to_string()
                    tokens: tokens
                    timestamp: "09:41"
                }
                EuvBubble {
                    author: "Developer".to_string()
                    content: "Verify multi-touch, text wrapping, blinking input caret & uncapped FPS live!".to_string()
                    tokens: tokens
                    timestamp: "09:42"
                    outgoing: true
                }
                EuvQuestionnaire {
                    prompt: "Preferred native target deployment pipeline?".to_string()
                    options: vec![
                        "Universal Native (Android + iOS + Desktop + Web)".to_string(),
                        "Dynamic OTA .wasm Hot-Swap Modules".to_string(),
                        "Embedded 3D Ray-Traced Engine Viewport".to_string(),
                    ]
                    selected: q_sel
                    tokens: tokens
                }
                EuvNavigationMenu {
                    items: vec![
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
                    ]
                    active: state.get_lab_nav_menu_active()
                    tokens: tokens
                }
                EuvChart {
                    title: "Frame Rate Telemetry (Bar)".to_string()
                    kind: EuvChartKind::Bar
                    data: chart_data.clone()
                    tokens: tokens
                }
                EuvChart {
                    title: "3D Ray-Tracing Throughput (Area)".to_string()
                    kind: EuvChartKind::Area
                    data: chart_data
                    tokens: tokens
                }
                EuvCombobox {
                    options: vec![
                        "euv Native Rust".to_string(),
                        "WebAssembly Zero-JS".to_string(),
                        "Android ARM64 JNI".to_string(),
                        "iOS CoreText Metal".to_string(),
                        "macOS AppKit Retina".to_string(),
                    ]
                    selected: state.get_lab_combobox_selected()
                    open: state.get_lab_combobox_open()
                    tokens: tokens
                    query: state.get_lab_combobox_query()
                    placeholder: "Filter frameworks..."
                }
                EuvDatePicker {
                    month_label: LABEL_CALENDAR_MONTH.to_string()
                    selected_day: state.get_lab_date_day()
                    open: state.get_lab_date_open()
                    tokens: tokens
                }
                EuvInputGroup {
                    value: state.get_lab_input_group_val()
                    tokens: tokens
                    prefix: "https://"
                    placeholder: "subdomain"
                    suffix: ".euv.dev"
                }
                EuvNativeSelect {
                    options: vec![
                        "60 Hz Standard".to_string(),
                        "120 Hz ProMotion".to_string(),
                        "Unlocked Benchmark".to_string(),
                    ]
                    selected: state.get_lab_native_select_idx()
                    tokens: tokens
                }
                EuvDataTable {
                    headers: vec!["Target".to_string(), "Font Engine".to_string(), "Status".to_string()]
                    rows: vec![
                        vec![LABEL_PLATFORM_ANDROID.to_string(), "Skia Paint AA".to_string(), "1:1 DPI".to_string()],
                        vec!["iOS / iPadOS".to_string(), "CoreText @3x".to_string(), "ProMotion".to_string()],
                        vec![LABEL_PLATFORM_MACOS.to_string(), "CoreText Retina".to_string(), "Live Resize".to_string()],
                        vec!["WebAssembly".to_string(), "Canvas2D AA".to_string(), "Zero-JS".to_string()],
                    ]
                    tokens: tokens
                    filter: state.get_lab_table_filter()
                    sort_ascending: state.get_lab_table_sort_asc()
                    selected_row: state.get_lab_table_selected()
                }
                EuvResizable {
                    left: left_split
                    right: right_split
                    split_ratio: state.get_lab_split_ratio()
                    tokens: tokens
                }
                EuvItem {
                    title: "Production shadcn/ui Alignment".to_string()
                    tokens: tokens
                    leading: "EU"
                    description: "Unified layout grammar across all 17 design schemes with per-theme surface/radius tokens"
                    trailing: LAB_ITEM_VER
                }
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    EuvBreadcrumb {
                        segments: vec!["Euv Studio".to_string(), "Design System".to_string(), "Components".to_string()]
                        tokens: tokens
                        active: state.get_breadcrumb_active()
                    }
                    EuvTypography {
                        text: "Unified shadcn/ui Typography & Surface Tokens".to_string()
                        variant: EuvTypographyVariant::H3
                        tokens: tokens
                    }
                    EuvSkeleton {
                        width: 260.0
                        height: 18.0
                        tokens: tokens
                    }
                    EuvEmpty {
                        title: "No Pending Warnings".to_string()
                        description: "All layout, scroll, shadow and typography checks passed.".to_string()
                        tokens: tokens
                    }
                }
            }
            EuvCard {
                title: "Boundary & Event Penetration Regression Lab".to_string()
                tokens: tokens
                subtitle: "Verify scroll lock, flexible event bubbling, overlay back stack, RTL alignment, and zero-rebuild resize preservation"
                EuvSeparator {
                    label: "EDGE-CASE REGRESSION SUITE".to_string()
                    tokens: tokens
                }
                column {
                    width: Length::Percent(100.0)
                    bind_children: move || {
                        let dir: EuvTextDirection = if rtl_sig.get() {
                            EuvTextDirection::Rtl
                        } else {
                            EuvTextDirection::Ltr
                        };
                        vec![view! {
                            EuvDirection {
                                direction: dir
                                child: view! {
                                    text {
                                        font_size: 13.0
                                        color: p.get_foreground()
                                        { format!("Direction preview ({dir:?}): Unified layout alignment across LTR & RTL") }
                                    }
                                }
                                tokens: tokens
                            }
                        }]
                    }
                }
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_XS
                    EuvSwitch {
                        checked: block_scroll
                        label: "Lock Scroll Penetration".to_string()
                        tokens: tokens
                    }
                    EuvSwitch {
                        checked: stop_bubble
                        label: "Stop Click Bubbling".to_string()
                        tokens: tokens
                    }
                    EuvSwitch {
                        checked: rtl_sig
                        label: "RTL Direction".to_string()
                        tokens: tokens
                    }
                }
                EuvActionRow {
                    EuvButton {
                        label: "Increment Preserved Counter".to_string()
                        tokens: tokens
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = preserve_cnt.update(|v: &mut u32| *v += 1);
                        }
                    }
                    EuvButton {
                        label: "Open Stacked Modal + Drawer".to_string()
                        tokens: tokens
                        secondary()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = modal_open.set_if_changed(true);
                            let _: bool = drawer_open.set_if_changed(true);
                        }
                    }
                    EuvButton {
                        label: "Dismiss Top Overlay (Back)".to_string()
                        tokens: tokens
                        outline()
                        sm()
                        on_click: move |_: &UiEvent| {
                            let _: bool = EuvOverlayStack::dismiss_top();
                        }
                    }
                }
                text {
                    font_size: 12.0
                    color: p.get_muted_foreground()
                    { move || format!(
                        "Inner Clicks: {} | Outer Parent Clicks: {} | Resize-Preserved Counter: {} | Overlay Stack Depth: {}",
                        child_clicks.get(),
                        parent_clicks.get(),
                        preserve_cnt.get(),
                        EuvOverlayStack::active_count()
                    ) }
                }
                column {
                    width: Length::Percent(100.0)
                    bind_children: move || {
                        let locked: bool = block_scroll.get();
                        let stop_clk: bool = stop_bubble.get();
                        vec![view! {
                            column {
                                width: Length::Percent(100.0)
                                padding: Edges::all(SPACE_MD)
                                gap: SPACE_SM
                                background: p.get_muted()
                                border: (g.get_border_width(), p.get_border())
                                radius: g.get_radius_md()
                                on_click: move |_: &UiEvent| {
                                    let _: bool = parent_clicks.update(|v: &mut u32| *v += 1);
                                }
                                scroll {
                                    width: Length::Percent(100.0)
                                    height: Length::Px(110.0)
                                    padding: Edges::all(SPACE_SM)
                                    gap: SPACE_XS
                                    background: p.get_card()
                                    border: (g.get_border_width(), p.get_border())
                                    radius: g.get_radius_sm()
                                    stop_scroll_propagation: locked
                                    stop_event_propagation: stop_clk
                                    for row_i in 1..=8 {
                                        text {
                                            font_size: 12.0
                                            color: p.get_foreground()
                                            { format!("Nested Scroll Row #{row_i} (scroll_lock={locked}, stop_bubble={stop_clk})") }
                                        }
                                    }
                                }
                                EuvActionRow {
                                    EuvButton {
                                        label: "Click Inner Target".to_string()
                                        tokens: tokens
                                        secondary()
                                        sm()
                                        on_click: move |_: &UiEvent| {
                                            let _: bool = child_clicks.update(|v: &mut u32| *v += 1);
                                        }
                                    }
                                }
                            }
                        }]
                    }
                }
            }
        }
    };
    let touch_handle: SceneHandle = ctx.get_scene().clone();
    let (touch_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        Effect::new(move || {
            let _: Vec<(u64, Point)> = touches_sig.get();
            touch_handle.mark_dirty(touch_pad_id, DIRTY_PAINT);
        })
    });
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(touch_pad_id, touch_scope);
        },
        (),
    );
    page
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
    let picker_color: Signal<Color> = state.get_picker_color();
    let picker_time: Signal<(u8, u8)> = state.get_picker_time();
    let tag_items: Signal<Vec<String>> = state.get_tag_items();
    let input_node: NodeId = view! {
        EuvInput {
            value: username
            placeholder: PLACEHOLDER_USERNAME.to_string()
            tokens: tokens
        }
    };
    let collapsible_body: NodeId = view! {
        text {
            font_size: 13.0
            color: p.get_muted_foreground()
            "shadcn/ui Collapsible disclosure panel adapts across Mobile, Tablet, and Desktop."
        }
    };
    view! {
        EuvPage {
            title: FORMS_TITLE.to_string()
            subtitle: FORMS_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: FORMS_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: FORMS_CARD_SUB
                column {
                    gap: SPACE_MD
                    EuvTabs {
                        labels: vec![TAB_GENERAL.to_string(), TAB_GRAPHICS.to_string(), TAB_SECURITY.to_string()]
                        active: state.get_active_tab()
                        tokens: tokens
                    }
                    EuvField {
                        label: FIELD_USERNAME.to_string()
                        control: input_node
                        tokens: tokens
                    }
                    EuvTextarea {
                        value: state.get_bio()
                        placeholder: "Write notes...".to_string()
                        height: 54.0
                        tokens: tokens
                    }
                    row {
                        style: c_action_row()
                        EuvNumberInput {
                            value: stepper
                            tokens: tokens
                            range(0, 20)
                            step: 1
                        }
                        EuvToggle {
                            pressed: state.get_pin_toggled()
                            label: "Pin Config".to_string()
                            tokens: tokens
                        }
                        EuvRating {
                            value: rating
                            max_stars: 5
                            tokens: tokens
                        }
                    }
                    EuvRadioGroup {
                        options: vec!["Software CPU".to_string(), "Hardware GPU".to_string(), "Hybrid".to_string()]
                        selected: state.get_radio_index()
                        tokens: tokens
                    }
                    EuvSlider {
                        value: state.get_slider_value()
                        min: 0.0
                        max: 100.0
                        tokens: tokens
                    }
                    EuvSelect {
                        options: vec!["Metal".to_string(), "Vulkan".to_string(), "Direct3D 12".to_string(), "WebGPU".to_string()]
                        selected: state.get_select_index()
                        open: state.get_select_open()
                        tokens: tokens
                    }
                    EuvSwitch {
                        checked: state.get_hardware_accel()
                        label: SWITCH_HW_LABEL.to_string()
                        tokens: tokens
                    }
                    EuvCheckbox {
                        checked: state.get_agree_terms()
                        label: CHECKBOX_TERMS_LABEL.to_string()
                        tokens: tokens
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { move || format!("Handle: {} | Stepper: {} | Stars: {}/5 | Day: {}", username.get(), stepper.get(), rating.get(), cal_day.get()) }
                    }
                    EuvActionRow {
                        EuvButton {
                            label: BTN_PRIMARY_LABEL.to_string()
                            tokens: tokens
                        }
                        EuvButton {
                            label: BTN_SECONDARY_LABEL.to_string()
                            tokens: tokens
                            secondary() }
                        EuvButton {
                            label: BTN_OUTLINE_LABEL.to_string()
                            tokens: tokens
                            outline() }
                        EuvButton {
                            label: BTN_DESTRUCTIVE_LABEL.to_string()
                            tokens: tokens
                            destructive()
                            on_click: move |_: &UiEvent| {
                                let _: bool = username.set_if_changed(DEFAULT_USERNAME.to_string());
                            }
                        }
                    }
                }
            }
            EuvCard {
                title: "shadcn/ui Extended Form Controls".to_string()
                tokens: tokens
                subtitle: FORMS_EXT_CARD_SUB
                column {
                    gap: SPACE_MD
                    EuvToggleGroup {
                        options: vec!["Bold".to_string(), "Italic".to_string(), "Underline".to_string()]
                        selected: state.get_toggle_group_index()
                        tokens: tokens
                    }
                    EuvInputOtp {
                        value: state.get_otp_code()
                        length: 6
                        tokens: tokens
                    }
                    EuvColorPicker {
                        value: picker_color
                        tokens: tokens
                    }
                    EuvTimePicker {
                        value: picker_time
                        tokens: tokens
                    }
                    EuvTagInput {
                        tags: tag_items
                        tokens: tokens
                        placeholder: TAG_INPUT_PLACEHOLDER
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { move || {
                            let c: Color = picker_color.get();
                            let (hh, mm): (u8, u8) = picker_time.get();
                            format!(
                                "Color: #{:02X}{:02X}{:02X} | Time: {:02}:{:02} | Tags: {}",
                                c.get_r(),
                                c.get_g(),
                                c.get_b(),
                                hh,
                                mm,
                                tag_items.get().len()
                            )
                        } }
                    }
                    EuvCalendar {
                        month_label: LABEL_CALENDAR_MONTH.to_string()
                        days_in_month: 30
                        selected_day: state.get_calendar_day()
                        tokens: tokens
                    }
                    EuvCollapsible {
                        open: state.get_collapsible_open()
                        title: "Advanced Form Details".to_string()
                        content: collapsible_body
                        tokens: tokens
                    }
                }
            }
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
    let scheme_cols: u16 = match use_breakpoint().get_untracked() {
        Breakpoint::Compact => 2,
        Breakpoint::Medium => 3,
        Breakpoint::Expanded => 4,
    };
    let all_schemes: [DesignScheme; 17] = DesignScheme::all();
    let cur_mode: ThemeMode = mode_sig.get_untracked();
    view! {
        EuvPage {
            title: THEMES_TITLE.to_string()
            subtitle: THEMES_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: THEMES_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: THEMES_CARD_SUB
                column {
                    gap: SPACE_MD
                    text {
                        font_size: 15.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_foreground()
                        { move || format!("{PREFIX_ACTIVE_SCHEME}{} ({:?})", scheme_sig.get().as_str(), mode_sig.get()) }
                    }
                    EuvActionRow {
                        EuvButton {
                            label: BTN_NEXT_SCHEME.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
                                state_for_cycle.next_scheme();
                            }
                        }
                        EuvButton {
                            label: BTN_TOGGLE_MODE.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                state_for_mode.toggle_theme_mode();
                            }
                        }
                    }
                    grid {
                        style: c_scheme_grid()
                        columns: scheme_cols
                        for scheme in all_schemes.iter().copied() {
                            EuvButton {
                                label: scheme.as_str().to_string()
                                tokens: ThemeTokens::resolve(scheme, cur_mode)
                                sm()
                                on_click: move |_: &UiEvent| {
                                    let _: bool = scheme_sig.set_if_changed(scheme);
                                }
                            }
                        }
                    }
                }
            }
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
    let tree_sel: Signal<Option<String>> = state.get_tree_selected();
    view! {
        EuvPage {
            title: DATA_TITLE.to_string()
            subtitle: DATA_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: DATA_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: DATA_CARD_SUB
                column {
                    gap: SPACE_MD
                    EuvMenubar {
                        menus: vec!["File".to_string(), "Edit".to_string(), "View".to_string(), "Profiles".to_string()]
                        active_menu: state.get_menubar_active()
                        tokens: tokens
                    }
                    row {
                        style: c_action_row()
                        EuvBadge {
                            label: BADGE_READY.to_string()
                            tone: EuvTone::Success
                            tokens: tokens
                        }
                        EuvBadge {
                            label: BADGE_LIVE.to_string()
                            tone: EuvTone::Primary
                            tokens: tokens
                        }
                    }
                    EuvActionRow {
                        EuvButton {
                            label: BTN_SCROLL_DOWN.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
                                vlist_down.scroll_by(160.0);
                            }
                        }
                        EuvButton {
                            label: BTN_SCROLL_TOP.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = scroll_y.set_if_changed(0.0);
                            }
                        }
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { move || format!("scroll_y={:.0}px | top_row=#{}", scroll_y.get(), (scroll_y.get() / 32.0).floor() as usize) }
                    }
                    EuvProgress {
                        value: 0.84
                        tokens: tokens
                    }
                    { list_node }
                    EuvCommand {
                        items: vec!["Switch to Dark Mode".to_string(), "Open OTA Updates".to_string(), "Run Physics Tick".to_string()]
                        selected: state.get_command_selected()
                        tokens: tokens
                        query: state.get_command_query()
                        placeholder: "Search commands..."
                    }
                    EuvCarousel {
                        slides: vec![
                            "Slide 1: Native Metal & Vulkan".to_string(),
                            "Slide 2: Complete shadcn/ui Parity".to_string(),
                            "Slide 3: Instant Responsive Resize".to_string(),
                        ]
                        active_slide: state.get_carousel_index()
                        tokens: tokens
                    }
                    column {
                        width: Length::Percent(100.0)
                        max_width: Length::Px(360.0)
                        EuvAspectRatio {
                            ratio: 16.0 / 9.0
                            child: aspect_inner
                            tokens: tokens
                        }
                    }
                    EuvPagination {
                        current: state.get_page_index()
                        total_pages: 12
                        tokens: tokens
                    }
                    EuvSteps {
                        labels: vec!["Layout".to_string(), "Paint".to_string(), "Present".to_string()]
                        active: state.get_step_index()
                        tokens: tokens
                    }
                    EuvTree {
                        roots: vec![EuvTreeNode::new(
                            TREE_NODE_WORKSPACE.to_string(),
                            vec![
                                EuvTreeNode::new(
                                    TREE_NODE_CORE.to_string(),
                                    vec![
                                        EuvTreeNode::new(TREE_NODE_RENDER.to_string(), Vec::new()),
                                        EuvTreeNode::new(TREE_NODE_REACTIVE.to_string(), Vec::new()),
                                    ],
                                ),
                                EuvTreeNode::new(TREE_NODE_UI.to_string(), Vec::new()),
                                EuvTreeNode::new(TREE_NODE_PERF.to_string(), Vec::new()),
                            ],
                        )]
                        tokens: tokens
                        selected: state.get_tree_selected()
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { move || {
                            let cur: String = tree_sel
                            .get()
                            .unwrap_or_else(|| TREE_NONE_SELECTED.to_string());
                            format!("{TREE_SELECTED_PREFIX}: {cur}")
                        } }
                    }
                    EuvAccordion {
                        items: vec![EuvAccordionItem::new(
                            "Virtualization".to_string(),
                            "Only visible rows + overscan are materialized in O(visible).".to_string(),
                        )]
                        open_index: state.get_accordion_open()
                        tokens: tokens
                    }
                }
            }
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
    let host_wave: GameEngineHost = host.clone();
    let host_left: GameEngineHost = host.clone();
    let host_up: GameEngineHost = host.clone();
    let host_down: GameEngineHost = host.clone();
    let host_right: GameEngineHost = host.clone();
    let host_dash: GameEngineHost = host.clone();
    let viewport: NodeId = host.mount_viewport(
        Style::default()
            .width(Length::Percent(100.0))
            .height(Length::Px(340.0))
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md()),
    );
    let fs_2d_open: Signal<bool> = Signal::new(false);
    let fs_2d_portal: NodeId = mount_game_fullscreen_portal(
        fs_2d_open,
        &host,
        viewport,
        None,
        "2D Physics Arena",
        tokens,
    );
    view! {
        EuvPage {
            title: GAME_TITLE.to_string()
            subtitle: GAME_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: GAME_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: GAME_CARD_SUB
                column {
                    gap: SPACE_MD
                    EuvActionRow {
                        EuvButton {
                            label: BTN_STEP_PHYSICS.to_string()
                            tokens: tokens
                            sm()
                            on_click: move |_: &UiEvent| {
                                let _: u32 = host_btn.step(0.051);
                            }
                        }
                        EuvButton {
                            label: "Spawn Enemy Wave".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                host_wave.ingest_event(&WindowEvent::KeyDown(
                                    WindowId::new(1),
                                    KeyEvent::new(KeyCode::Character('r'), Modifiers::default(), false),
                                ));
                                let _: u32 = host_wave.step(1.0 / 60.0);
                                host_wave.ingest_event(&WindowEvent::KeyUp(
                                    WindowId::new(1),
                                    KeyEvent::new(KeyCode::Character('r'), Modifiers::default(), false),
                                ));
                            }
                        }
                    }
                    { viewport }
                    EuvActionRow {
                        EuvButton {
                            label: "Left (A)".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                pulse_game_key(&host_left, KeyCode::ArrowLeft);
                            }
                        }
                        EuvButton {
                            label: "Up (W)".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                pulse_game_key(&host_up, KeyCode::ArrowUp);
                            }
                        }
                        EuvButton {
                            label: "Down (S)".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                pulse_game_key(&host_down, KeyCode::ArrowDown);
                            }
                        }
                        EuvButton {
                            label: "Right (D)".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                pulse_game_key(&host_right, KeyCode::ArrowRight);
                            }
                        }
                        EuvButton {
                            label: "Dash (E)".to_string()
                            tokens: tokens
                            secondary()
                            sm()
                            on_click: move |_: &UiEvent| {
                                pulse_game_key(&host_dash, KeyCode::Character('e'));
                            }
                        }
                        EuvButton {
                            label: "Fullscreen".to_string()
                            tokens: tokens
                            sm()
                            on_click: move |_: &UiEvent| {
                                let _: bool = fs_2d_open.set_if_changed(true);
                            }
                        }
                    }
                    { fs_2d_portal }
                }
            }
        }
    }
}

/// Mounts a reusable fullscreen portal overlay hosting any 2D/3D `GameEngineHost` with safe exit, Back support, and touch controls.
///
/// # Arguments
///
/// - `Signal<bool>` - Fullscreen visibility toggle signal.
/// - `&GameEngineHost` - Target game engine host.
/// - `NodeId` - Embedded inline viewport hidden while fullscreen is active to avoid redundant rendering.
/// - `Option<Signal<u8>>` - Optional 3D camera mode signal for fullscreen camera toggling.
/// - `&str` - Title shown in the fullscreen HUD bar.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted fullscreen portal handle.
fn mount_game_fullscreen_portal(
    open: Signal<bool>,
    host: &GameEngineHost,
    inline_viewport: NodeId,
    cam_mode_opt: Option<Signal<u8>>,
    title: &str,
    tokens: ThemeTokens,
) -> NodeId {
    let ctx: Context = current_context();
    let fs_viewport: NodeId = host.mount_viewport(
        Style::default()
            .flex_grow(1.0)
            .flex_shrink(1.0)
            .min_height(Length::Px(0.0))
            .width(Length::Percent(100.0))
            .overflow(Overflow::Hidden)
            .stop_scroll_propagation(true)
            .stop_event_propagation(true),
    );
    let ctrl_left: NodeId =
        mount_game_control_button(FS_CTRL_LEFT, host, KeyCode::ArrowLeft, false, tokens);
    let ctrl_up: NodeId = mount_game_control_button(
        if cam_mode_opt.is_some() {
            FS_CTRL_DRIVE
        } else {
            FS_CTRL_UP
        },
        host,
        KeyCode::ArrowUp,
        true,
        tokens,
    );
    let ctrl_down: NodeId = mount_game_control_button(
        if cam_mode_opt.is_some() {
            FS_CTRL_REVERSE
        } else {
            FS_CTRL_DOWN
        },
        host,
        KeyCode::ArrowDown,
        false,
        tokens,
    );
    let ctrl_right: NodeId =
        mount_game_control_button(FS_CTRL_RIGHT, host, KeyCode::ArrowRight, false, tokens);
    let extra_ctrl: NodeId = if let Some(cam_sig) = cam_mode_opt {
        let on_cam = move |_: &UiEvent| {
            let next: u8 = (cam_sig.get_untracked() + 1) % 4;
            let _: bool = cam_sig.set_if_changed(next);
        };
        view! {
            EuvButton {
                label: FS_CTRL_CAMERA.to_string()
                tokens: tokens
                secondary()
                sm()
                on_click: on_cam
            }
        }
    } else {
        mount_game_control_button(FS_CTRL_DASH, host, KeyCode::Character('e'), false, tokens)
    };
    let on_exit = move |_: &UiEvent| {
        let _: bool = open.set_if_changed(false);
    };
    let mut top_bar_id: NodeId = NodeId::default();
    let mut bottom_bar_id: NodeId = NodeId::default();
    let portal: NodeId = view! {
        portal {
            style: Style::default().column()
            display: Display::None
            position: Position::Fixed
            inset: (Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            width: Length::Percent(100.0)
            height: Length::Percent(100.0)
            background: Color::hex(0x09090B)
            overflow: Overflow::Hidden
            stop_scroll_propagation: true
            z_index: Z_INDEX_MODAL
            row {
                node_ref: top_bar_id
                width: Length::Percent(100.0)
                flex_shrink: 0.0
                justify: JustifyContent::SpaceBetween
                align: AlignItems::Center
                padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                background: Color::rgba(9, 9, 11, 236)
                text {
                    flex_shrink: 1.0
                    min_width: Length::Px(0.0)
                    font_size: 14.0
                    font_weight: FontWeight::SemiBold
                    color: Color::hex(0xFAFAFA)
                    text_overflow: TextOverflow::Ellipsis
                    { format!("{title} · Fullscreen") }
                }
                EuvButton {
                    label: BTN_EXIT_FULLSCREEN.to_string()
                    tokens: tokens
                    secondary()
                    sm()
                    on_click: on_exit
                }
            }
            { fs_viewport }
            row {
                node_ref: bottom_bar_id
                width: Length::Percent(100.0)
                flex_shrink: 0.0
                flex_wrap: FlexWrap::Wrap
                justify: JustifyContent::Center
                align: AlignItems::Center
                gap: SPACE_SM
                padding: Edges::symmetric(SPACE_SM, SPACE_MD)
                background: Color::rgba(9, 9, 11, 232)
                { ctrl_left }
                { ctrl_up }
                { ctrl_down }
                { ctrl_right }
                { extra_ctrl }
            }
        }
    };
    let handle: SceneHandle = ctx.get_scene().clone();
    let safe_sig: Signal<SafeAreaInsets> = ctx.get_safe_area();
    let prev_fs: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let (scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
        let _: Effect = EuvOverlayStack::bind_signal(open, FS_OVERLAY_NAME, Z_INDEX_MODAL);
        Effect::new(move || {
            let is_open: bool = open.get();
            let safe: SafeAreaInsets = safe_sig.get();
            if is_open != prev_fs.replace(is_open) {
                let _: bool = dispatch_native_action(
                    NATIVE_ACTION_FULLSCREEN,
                    if is_open { FS_ACTION_ON } else { FS_ACTION_OFF },
                    "",
                );
            }
            handle.with_write(
                |tree: &mut SceneTree| {
                    if let Some(n) = tree.get_node_mut(inline_viewport) {
                        let mut st: Style = n.get_style();
                        st.get_mut_layout().set_display(if is_open {
                            Display::None
                        } else {
                            Display::Flex
                        });
                        n.set_style(st);
                        tree.mark_dirty(inline_viewport, DIRTY_LAYOUT | DIRTY_PAINT);
                    }
                    if let Some(n) = tree.get_node_mut(portal) {
                        let mut st: Style = n.get_style();
                        st.get_mut_layout().set_display(if is_open {
                            Display::Flex
                        } else {
                            Display::None
                        });
                        n.set_style(st);
                    }
                    if let Some(n) = tree.get_node_mut(top_bar_id) {
                        let mut st: Style = n.get_style();
                        st.get_mut_layout().set_padding(Edges::new(
                            SPACE_SM + safe.get_top(),
                            SPACE_MD + safe.get_right(),
                            SPACE_SM,
                            SPACE_MD + safe.get_left(),
                        ));
                        n.set_style(st);
                    }
                    if let Some(n) = tree.get_node_mut(bottom_bar_id) {
                        let mut st: Style = n.get_style();
                        st.get_mut_layout().set_padding(Edges::new(
                            SPACE_SM,
                            SPACE_MD + safe.get_right(),
                            SPACE_SM + safe.get_bottom(),
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
    let node: NodeId = if primary {
        view! {
            EuvButton {
                label: label.to_string()
                tokens: tokens
                sm()
                on_click: move |_: &UiEvent| {
                    pulse_game_key(&host_click, code);
                }
            }
        }
    } else {
        view! {
            EuvButton {
                label: label.to_string()
                tokens: tokens
                outline()
                sm()
                on_click: move |_: &UiEvent| {
                    pulse_game_key(&host_click, code);
                }
            }
        }
    };
    let mut ctx: Context = current_context();
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
        Style::default()
            .width(Length::Percent(100.0))
            .height(Length::Px(340.0))
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md()),
    );
    let fs_3d_open: Signal<bool> = Signal::new(false);
    let fs_3d_portal: NodeId = mount_game_fullscreen_portal(
        fs_3d_open,
        &host,
        viewport,
        Some(cam_mode_sig),
        "Vice Boulevard 3D RT",
        tokens,
    );
    let res_slider: NodeId = view! {
        EuvSlider {
            value: res_scale
            min: 40.0
            max: 125.0
            tokens: tokens
        }
    };
    let sun_slider: NodeId = view! {
        EuvSlider {
            value: time_sig
            min: 6.0
            max: 23.0
            tokens: tokens
        }
    };
    let ctx: Context = current_context();
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
    let mut sync_host_id: NodeId = NodeId::default();
    let page: NodeId = view! {
        EuvPage {
            title: GAME3D_TITLE.to_string()
            subtitle: GAME3D_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: GAME3D_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: GAME3D_CARD_SUB
                column {
                    node_ref: sync_host_id
                    width: Length::Percent(100.0)
                    gap: SPACE_MD
                    { viewport }
                    EuvActionRow {
                        { mount_game_control_button("Steer Left (A)", &host, KeyCode::ArrowLeft, false, tokens) }
                        { mount_game_control_button("Drive Fwd (W)", &host, KeyCode::ArrowUp, false, tokens) }
                        { mount_game_control_button("Reverse (S)", &host, KeyCode::ArrowDown, false, tokens) }
                        { mount_game_control_button("Steer Right (D)", &host, KeyCode::ArrowRight, false, tokens) }
                        EuvButton {
                            label: "Switch Camera".to_string()
                            tokens: tokens
                            secondary()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let next: u8 = (cam_mode_sig.get_untracked() + 1) % 4;
                                let _: bool = cam_mode_sig.set_if_changed(next);
                            }
                        }
                        EuvButton {
                            label: "Fullscreen 3D".to_string()
                            tokens: tokens
                            sm()
                            on_click: move |_: &UiEvent| {
                                let _: bool = fs_3d_open.set_if_changed(true);
                            }
                        }
                        EuvButton {
                            label: "Import .OBJ Monument".to_string()
                            tokens: tokens
                            secondary()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let cur: u64 = import_sig.get_untracked();
                                let next: u64 = if cur.is_multiple_of(2) { cur + 1 } else { cur + 2 };
                                let _: bool = import_sig.set_if_changed(next);
                            }
                        }
                        EuvButton {
                            label: "Import .glTF Tower".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let cur: u64 = import_sig.get_untracked();
                                let next: u64 = if !cur.is_multiple_of(2) { cur + 1 } else { cur + 2 };
                                let _: bool = import_sig.set_if_changed(next);
                            }
                        }
                    }
                    text {
                        font_size: 12.0
                        color: p.get_muted_foreground()
                        { move || status_sig.get() }
                    }
                    EuvSegmented {
                        options: vec![
                            "Low".to_string(),
                            "Medium".to_string(),
                            "High".to_string(),
                            "Ultra".to_string(),
                            "Cinematic".to_string(),
                        ]
                        selected: preset_idx
                        tokens: tokens
                    }
                    EuvSegmented {
                        options: vec![
                            "RT Off".to_string(),
                            "RT Shadows".to_string(),
                            "RT Refl".to_string(),
                            "RT Full GI".to_string(),
                        ]
                        selected: rt_idx
                        tokens: tokens
                    }
                    EuvField {
                        label: "Runtime Render Resolution Scale (%)".to_string()
                        control: res_slider
                        tokens: tokens
                    }
                    EuvField {
                        label: "Time of Day / Sun Lighting (6h Dawn -> 17h Sunset -> 22h Night)".to_string()
                        control: sun_slider
                        tokens: tokens
                    }
                    { fs_3d_portal }
                }
            }
        }
    };
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            tree.attach_scope(sync_host_id, sync_scope);
        },
        (),
    );
    page
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
    let alert_open: Signal<bool> = state.get_alert_dialog_open();
    let ctx_sel: Signal<Option<usize>> = Signal::new(None);
    let tip_btn: NodeId = view! {
        EuvButton {
            label: "Hover Tooltip".to_string()
            tokens: tokens
            outline()
            sm()
        }
    };
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
                EuvButton {
                    label: BTN_CLOSE_MODAL.to_string()
                    tokens: tokens
                    outline()
                    on_click: move |_: &UiEvent| {
                        let _: bool = modal_open.set_if_changed(false);
                    }
                }
            }
        }
    };
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
                EuvButton {
                    label: "Close Drawer".to_string()
                    tokens: tokens
                    outline()
                    on_click: move |_: &UiEvent| {
                        let _: bool = drawer_open.set_if_changed(false);
                    }
                }
            }
        }
    };
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
                EuvButton {
                    label: "Close Sheet".to_string()
                    tokens: tokens
                    outline()
                    on_click: move |_: &UiEvent| {
                        let _: bool = sheet_open.set_if_changed(false);
                    }
                }
            }
        }
    };
    view! {
        EuvPage {
            title: OVERLAYS_TITLE.to_string()
            subtitle: OVERLAYS_SUBTITLE.to_string()
            tokens: tokens
            column {
                width: Length::Percent(100.0)
                gap: SPACE_LG
                EuvAlert {
                    title: ALERT_TITLE.to_string()
                    description: ALERT_DESC.to_string()
                    tone: EuvTone::Primary
                    tokens: tokens
                }
                EuvCard {
                    title: OVERLAYS_CARD_TITLE.to_string()
                    tokens: tokens
                    subtitle: OVERLAYS_CARD_SUB
                    EuvActionRow {
                        EuvButton {
                            label: BTN_OPEN_MODAL.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
                                let _: bool = modal_open.set_if_changed(true);
                            }
                        }
                        EuvButton {
                            label: "Alert Dialog".to_string()
                            tokens: tokens
                            destructive()
                            on_click: move |_: &UiEvent| {
                                let _: bool = alert_open.set_if_changed(true);
                            }
                        }
                        EuvButton {
                            label: BTN_TOGGLE_TOAST.to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
                                let _: bool = toast_visible.update(|v: &mut bool| *v = !*v);
                            }
                        }
                        EuvButton {
                            label: "Toggle Drawer".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = drawer_open.update(|v: &mut bool| *v = !*v);
                            }
                        }
                        EuvButton {
                            label: "Toggle Sheet".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = sheet_open.update(|v: &mut bool| *v = !*v);
                            }
                        }
                        EuvButton {
                            label: "Toggle Notification".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = notif_visible.update(|v: &mut bool| *v = !*v);
                            }
                        }
                    }
                    EuvDropdown {
                        label: "Quick Actions".to_string()
                        items: vec!["Export PNG".to_string(), "Copy DeepLink".to_string()]
                        tokens: tokens
                        open: state.get_dropdown_open()
                        selected: state.get_dropdown_selected()
                    }
                    EuvHoverCard {
                        trigger_label: "@euv-dev".to_string()
                        title: HOVER_CARD_ENGINE_TITLE.to_string()
                        tokens: tokens
                        open: state.get_hover_card_open()
                        description: "Full ui.shadcn.com component alignment across all 7 native targets."
                    }
                    EuvTooltip {
                        text: "Zero-WebView floating tooltip".to_string()
                        trigger: tip_btn
                        tokens: tokens
                    }
                    EuvContextMenu {
                        trigger_label: "Context Menu Surface".to_string()
                        items: vec![
                            "Inspect Scene Node".to_string(),
                            "Copy AI Selector".to_string(),
                            "Capture Frame Hash".to_string(),
                        ]
                        tokens: tokens
                        selected: ctx_sel
                    }
                }
                EuvModal {
                    open: modal_open
                    title: MODAL_TITLE.to_string()
                    body: modal_body
                    tokens: tokens
                }
                EuvAlertDialog {
                    open: alert_open
                    title: "Are you absolutely sure?".to_string()
                    confirmed: state.get_alert_dialog_confirmed()
                    tokens: tokens
                    description: "This shadcn AlertDialog confirms destructive operations cleanly."
                    labels("Cancel", "Continue")
                }
                EuvDrawer {
                    open: drawer_open
                    title: "Inspector Drawer".to_string()
                    body: drawer_body
                    tokens: tokens
                }
                EuvSheet {
                    open: sheet_open
                    title: "Quick Sheet".to_string()
                    body: sheet_body
                    tokens: tokens
                }
                EuvToast {
                    visible: toast_visible
                    message: TOAST_TEXT.to_string()
                    tone: EuvTone::Success
                    tokens: tokens
                }
                EuvNotification {
                    visible: notif_visible
                    title: "Native Sync".to_string()
                    message: "All 7 targets active.".to_string()
                    tokens: tokens
                    tone: EuvTone::Success
                }
                EuvLoading {
                    active: state.get_loading_active()
                    label: "Syncing...".to_string()
                    tokens: tokens
                }
            }
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
    view! {
        EuvPage {
            title: DYNAMIC_TITLE.to_string()
            subtitle: DYNAMIC_SUBTITLE.to_string()
            tokens: tokens
            EuvCard {
                title: "Scoped / Partial Dynamic WASM Execution".to_string()
                tokens: tokens
                subtitle: "Download genuine .wasm binaries into a single native slot with checksum/signature verification, offline cache, interactive euv_on_event & rollback"
                column {
                    gap: SPACE_MD
                    EuvActionRow {
                        EuvButton {
                            label: "Download Real HTTP .wasm".to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Download Partial WASM v1".to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Hot-Swap Partial WASM v2".to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Toggle Offline (Cache Hit)".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let next: bool = !wasm_offline_sig.get_untracked();
                                let _: bool = wasm_offline_sig.set(next);
                                r_off.set_offline(next);
                                let _: bool = s_off.set(if next {
                                    "Simulated OFFLINE enabled — re-downloading v1 will serve verified offline cache!".to_string()
                                    } else {
                                        "Simulated ONLINE restored — fetching live .wasm from CDN".to_string()
                                });
                            }
                        }
                        EuvButton {
                            label: "Rollback Partial".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                match m_prb.rollback_partial(SLOT_WASM_WIDGET, "manual rollback") {
                                    Ok(v) => { let _: bool = s_prb.set(format!("Rolled back partial WASM slot to v{v}")); }
                                    Err(e) => { let _: bool = s_prb.set(format!("Rollback: {e}")); }
                                }
                            }
                        }
                        EuvButton {
                            label: "Unmount Slot".to_string()
                            tokens: tokens
                            ghost()
                            on_click: move |_: &UiEvent| {
                                m_pun.unmount_partial(SLOT_WASM_WIDGET);
                                let _: bool = s_pun.set("Unmounted partial WASM instance and restored compiled fallback".to_string());
                            }
                        }
                    }
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
            }
            EuvCard {
                title: "Global / Full-App Dynamic WASM Takeover".to_string()
                tokens: tokens
                subtitle: "Atomically replace the entire native app shell from a downloaded .wasm release with automatic Last-Known-Good (LKG) rollback on any fault"
                column {
                    gap: SPACE_MD
                    EuvActionRow {
                        EuvButton {
                            label: "Download Global WASM v1".to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Upgrade Global WASM v2".to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Faulty Global (Auto LKG Rollback)".to_string()
                            tokens: tokens
                            destructive()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: "Restore Native Shell".to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                m_grb.restore_native_global();
                                let _: bool = s_grb.set(STATUS_RESTORED_NATIVE_SHELL.to_string());
                            }
                        }
                    }
                }
            }
            EuvCard {
                title: "All-Case Security, Fault & Sandbox Verification Matrix".to_string()
                tokens: tokens
                subtitle: "Verify every edge case live: auto-retry, offline cache, integrity rejection, ABI mismatch, infinite-loop fuel cutoff, trap isolation & stale cancel guard"
                column {
                    gap: SPACE_MD
                    EuvActionRow {
                        EuvButton {
                            label: "Timeout -> Auto Retry".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_TIMEOUT_RETRY.to_string(), SLOT_WASM_WIDGET.to_string(), 10).with_allow_downgrade(true);
                                match m_f_retry.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY) {
                                    Ok(v) => { let _: bool = s_f_retry.set(format!("PASS Timeout->Retry recovered and mounted v{v}!")); }
                                    Err(e) => { let _: bool = s_f_retry.set(format!("Retry failed: {e}")); }
                                }
                            }
                        }
                        EuvButton {
                            label: "HTTP 500".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_HTTP_500.to_string(), SLOT_WASM_WIDGET.to_string(), 11).with_allow_downgrade(true);
                                let res: String = format!("Handled HTTP 500 -> {:?}", m_f_500.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_500.set(res);
                            }
                        }
                        EuvButton {
                            label: "Corrupted Magic".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_CORRUPTED.to_string(), SLOT_WASM_WIDGET.to_string(), 12).with_allow_downgrade(true);
                                let res: String = format!("Blocked Corrupted Magic -> {:?}", m_f_corrupt.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_corrupt.set(res);
                            }
                        }
                        EuvButton {
                            label: "Tampered Checksum".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let good: Vec<u8> = build_partial_widget_wasm(TITLE_WASM_PARTIAL_V1, 1, 0x2563EB);
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_TAMPERED.to_string(), SLOT_WASM_WIDGET.to_string(), 13)
                                .with_checksum(compute_wasm_checksum(&good))
                                .with_allow_downgrade(true);
                                let res: String = format!("Blocked Tampered Checksum -> {:?}", m_f_tamper.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_tamper.set(res);
                            }
                        }
                        EuvButton {
                            label: "Invalid Signature".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_PARTIAL_V1.to_string(), SLOT_WASM_WIDGET.to_string(), 14)
                                .with_signature("euv-sig-v1:forged".to_string())
                                .with_allow_downgrade(true);
                                let res: String = format!("Blocked Forged Signature -> {:?}", m_f_sig.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_sig.set(res);
                            }
                        }
                        EuvButton {
                            label: "ABI Mismatch".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_BAD_ABI.to_string(), SLOT_WASM_WIDGET.to_string(), 15).with_allow_downgrade(true);
                                let res: String = format!("Blocked ABI Mismatch -> {:?}", m_f_abi.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_abi.set(res);
                            }
                        }
                        EuvButton {
                            label: "Missing Export".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_MISSING_EXPORT.to_string(), SLOT_WASM_WIDGET.to_string(), 16).with_allow_downgrade(true);
                                let res: String = format!("Blocked Missing Export -> {:?}", m_f_exp.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_exp.set(res);
                            }
                        }
                        EuvButton {
                            label: "Infinite-Loop Fuel".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_FUEL_BOMB.to_string(), SLOT_WASM_WIDGET.to_string(), 17).with_allow_downgrade(true);
                                let res: String = format!("Terminated Runaway Loop -> {:?}", m_f_fuel.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_fuel.set(res);
                            }
                        }
                        EuvButton {
                            label: "Runtime Trap".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_TRAP.to_string(), SLOT_WASM_WIDGET.to_string(), 18).with_allow_downgrade(true);
                                let res: String = format!("Isolated Runtime Trap -> {:?}", m_f_trap.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_trap.set(res);
                            }
                        }
                        EuvButton {
                            label: "Oversized Binary".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let req: WasmDownloadRequest = WasmDownloadRequest::partial(URL_WASM_OVERSIZED.to_string(), SLOT_WASM_WIDGET.to_string(), 19).with_allow_downgrade(true);
                                let res: String = format!("Blocked Oversized Binary -> {:?}", m_f_big.load_and_mount(req, WASM_SHOWCASE_SIGNING_KEY));
                                let _: bool = s_f_big.set(res);
                            }
                        }
                        EuvButton {
                            label: "Cancel / Stale Guard".to_string()
                            tokens: tokens
                            outline()
                            sm()
                            on_click: move |_: &UiEvent| {
                                let gen_id: u64 = m_f_cancel.cancel_scope(&WasmDynamicScope::Partial { slot_id: SLOT_WASM_WIDGET.to_string() });
                                let _: bool = s_f_cancel.set(format!("Cancelled scope request cleanly (generation #{gen_id} blocks stale responses)"));
                            }
                        }
                    }
                    text {
                        font_size: 12.5
                        font_weight: FontWeight::Medium
                        color: p.get_primary()
                        { move || wasm_status_sig.get() }
                    }
                }
            }
            EuvCard {
                title: DYNAMIC_COMP_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: DYNAMIC_COMP_CARD_SUB
                column {
                    gap: SPACE_MD
                    EuvActionRow {
                        EuvButton {
                            label: BTN_INC_SIBLING.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: bool = sibling_counter.update(|v: &mut i32| *v += 1);
                            }
                        }
                    }
                    text {
                        font_size: 13.0
                        font_weight: FontWeight::SemiBold
                        color: p.get_primary()
                        { move || format!("Sibling State Preserved: count={}", sibling_counter.get()) }
                    }
                    EuvActionRow {
                        EuvButton {
                            label: BTN_PUSH_BANNER_V1.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: BTN_PUSH_BANNER_V2.to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
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
                                                EuvButton {
                                                    label: BTN_CLAIM_OFFER.to_string()
                                                    tokens: tokens
                                                    on_click: move |_: &UiEvent| {
                                                        hub_click.dispatch_action(DynamicActionEvent::new(
                                                            SLOT_PROMO_BANNER.to_string(),
                                                            "claim_offer".to_string(),
                                                            format!("native-v{next_v}"),
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    },
                                );
                            }
                        }
                        EuvButton {
                            label: BTN_ROLLBACK_BANNER.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: Result<u64, DynamicUpdateError> = hub_rb.rollback_component(SLOT_PROMO_BANNER);
                            }
                        }
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
                    EuvActionRow {
                        EuvButton {
                            label: BTN_PATCH_METRICS.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: BTN_ROLLBACK_METRICS.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: Result<u64, DynamicUpdateError> =
                                hub_mrb.rollback_component(SLOT_METRICS_WIDGET);
                            }
                        }
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
            }
            EuvCard {
                title: DYNAMIC_PAGE_CARD_TITLE.to_string()
                tokens: tokens
                subtitle: DYNAMIC_PAGE_CARD_SUB
                column {
                    gap: SPACE_MD
                    EuvActionRow {
                        EuvButton {
                            label: BTN_UPDATE_PAGE_V1.to_string()
                            tokens: tokens
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: BTN_APPLY_BUNDLE.to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
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
                            }
                        }
                        EuvButton {
                            label: BTN_ROLLBACK_PAGE.to_string()
                            tokens: tokens
                            outline()
                            on_click: move |_: &UiEvent| {
                                let _: Result<u64, DynamicUpdateError> = hub_prb.rollback_page(ROUTE_DYNAMIC_PANEL);
                            }
                        }
                        EuvButton {
                            label: BTN_INJECT_ROUTE.to_string()
                            tokens: tokens
                            secondary()
                            on_click: move |_: &UiEvent| {
                                let next_v: u64 = hub_inject.page_version(ROUTE_PROMO_LIVE).saturating_add(1);
                                let mut back_btn: DynamicNodeSpec = DynamicNodeSpec::button(
                                    BTN_RETURN_DYNAMIC.to_string(),
                                    "return_dynamic".to_string(),
                                );
                                back_btn.set_payload(format!("from-promo-v{next_v}"));
                                let page_spec: DynamicNodeSpec = DynamicNodeSpec::column().child(
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
                            }
                        }
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
            }
        }
    }
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
    let photo_card: NodeId = view! {
        EuvImage {
            asset: ImageAsset::sample_photo("Hero Raster Photo (PNG/BMP/WebP)")
            tokens: tokens
            height: 148.0
        }
    };
    let svg_card: NodeId = view! {
        EuvSvgView {
            document: SvgDocument::sample_svg()
            label: "Vector SVG Badge".to_string()
            tokens: tokens
        }
    };
    let gif_card: NodeId = view! {
        EuvGifPlayer {
            state: state.get_media_gif()
            tokens: tokens
        }
    };
    let live_card: NodeId = view! {
        EuvLivePhoto {
            state: state.get_media_live()
            tokens: tokens
        }
    };
    let video_card: NodeId = view! {
        EuvVideoPlayer {
            state: state.get_media_video()
            tokens: tokens
        }
    };
    let audio_card: NodeId = view! {
        EuvAudioPlayer {
            state: state.get_media_audio()
            tokens: tokens
        }
    };
    let pdf_card: NodeId = view! {
        EuvPdfViewer {
            state: state.get_media_pdf()
            tokens: tokens
        }
    };
    let html_card: NodeId = view! {
        EuvHtmlView {
            document: HtmlDocument::sample_html()
            tokens: tokens
            on_link: move |href: String| {
                let _: bool = link_log_cb.set_if_changed(format!("Opened native link: {href}"));
            }
        }
    };
    view! {
        EuvPage {
            title: "Full Cross-Platform Media Studio".to_string()
            subtitle: "Pure-Rust aspect-preserving Image, SVG, animated GIF, LIVE Photo, Video, WAV Audio waveform, multi-page PDF, and sanitized zero-JS HTML5.".to_string()
            tokens: tokens
            { mount_bento_pair(photo_card, svg_card) }
            { mount_bento_pair(gif_card, live_card) }
            { mount_bento_pair(video_card, audio_card) }
            { mount_bento_pair(pdf_card, html_card) }
            text {
                font_size: 12.5
                color: p.get_muted_foreground()
                { move || link_log.get() }
            }
        }
    }
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
    let bp_sig: Signal<Breakpoint> = use_breakpoint();
    view! {
        grid {
            bind_style: move || {
                let cols: u16 = if bp_sig.get() == Breakpoint::Expanded { 2 } else { 1 };
                Style::default()
                .grid(cols)
                .width(Length::Percent(100.0))
                .gap(SPACE_MD)
            }
            { left }
            { right }
        }
    }
}
