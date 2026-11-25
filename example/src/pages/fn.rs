use super::*;

/// Mounts the complete showcase application shell and active page for `state`.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
///
/// # Returns
///
/// - `NodeId` - Mounted root application shell handle.
pub fn mount_showcase_root(ctx: &mut BuildContext, state: &ShowcaseState) -> NodeId {
    let tokens: ThemeTokens = state.get_theme().get_tokens().get();
    let nav: NodeId =
        EuvSidebarNav::create(ShowcaseState::nav_items(), state.get_route(), tokens).mount(ctx);
    let page_node: NodeId = mount_active_route(ctx, state, tokens);
    EuvAppShell::create(SHOWCASE_BRAND.to_string(), nav, page_node, tokens).mount(ctx)
}

/// Mounts the showcase page corresponding to the active route in `state`.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active resolved design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted page container handle.
pub fn mount_active_route(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let route: String = state.get_route().get_untracked();
    match route.as_str() {
        ROUTE_REACTIVE => mount_reactive_page(ctx, state, tokens),
        ROUTE_RENDER => mount_render_page(ctx, state, tokens),
        ROUTE_WINDOW => mount_window_page(ctx, state, tokens),
        ROUTE_FORMS => mount_forms_page(ctx, state, tokens),
        ROUTE_THEMES => mount_themes_page(ctx, state, tokens),
        ROUTE_DATA => mount_data_page(ctx, tokens),
        ROUTE_OVERLAYS => mount_overlays_page(ctx, state, tokens),
        _ => mount_overview_page(ctx, tokens),
    }
}

/// Mounts the overview dashboard page with KPI stat cards and the 7-platform capability matrix.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted overview page node.
pub fn mount_overview_page(ctx: &mut BuildContext, tokens: ThemeTokens) -> NodeId {
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId = EuvHeader::create(
        OVERVIEW_TITLE.to_string(),
        OVERVIEW_SUBTITLE.to_string(),
        tokens,
    )
    .mount(ctx);
    let stats_row: NodeId = ctx.view(
        Style::empty()
            .grid(3)
            .width(Length::Percent(100.0))
            .gap(SPACE_MD),
    );
    let s1: NodeId = EuvStatCard::create(
        STAT_PLATFORMS_LABEL.to_string(),
        STAT_PLATFORMS_VAL.to_string(),
        STAT_PLATFORMS_CAP.to_string(),
        tokens,
    )
    .mount(ctx);
    let s2: NodeId = EuvStatCard::create(
        STAT_SCHEMES_LABEL.to_string(),
        STAT_SCHEMES_VAL.to_string(),
        STAT_SCHEMES_CAP.to_string(),
        tokens,
    )
    .mount(ctx);
    let s3: NodeId = EuvStatCard::create(
        STAT_RENDER_LABEL.to_string(),
        STAT_RENDER_VAL.to_string(),
        STAT_RENDER_CAP.to_string(),
        tokens,
    )
    .mount(ctx);
    ctx.append(stats_row, s1);
    ctx.append(stats_row, s2);
    ctx.append(stats_row, s3);
    let table: NodeId = EuvTable::create(
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
    .mount(ctx);
    let card: NodeId = EuvCard::create(CARD_PLATFORMS_TITLE.to_string(), tokens)
        .subtitle(CARD_PLATFORMS_SUB.to_string())
        .child(table)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, stats_row);
    ctx.append(page, card);
    page
}

/// Mounts the fine-grained reactivity showcase demonstrating `Signal`, `Memo`, and `batch`.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted reactivity page node.
pub fn mount_reactive_page(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let counter: Signal<i32> = state.get_counter();
    let doubled: Memo<i32> = state.get_doubled();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId = EuvHeader::create(
        REACTIVE_TITLE.to_string(),
        REACTIVE_SUBTITLE.to_string(),
        tokens,
    )
    .mount(ctx);
    let counter_txt: NodeId = ctx.reactive_text(
        Style::empty()
            .font_size(18.0)
            .font_weight(FontWeight::Bold)
            .color(p.get_foreground()),
        move || format!("{PREFIX_COUNTER}{}", counter.get()),
    );
    let memo_txt: NodeId = ctx.reactive_text(
        Style::empty().font_size(15.0).color(p.get_primary()),
        move || format!("{PREFIX_DOUBLED}{}", doubled.get()),
    );
    let btn_row: NodeId = ctx.view(Style::empty().row().gap(SPACE_SM));
    let inc_btn: NodeId = EuvButton::create(BTN_INCREMENT.to_string(), tokens)
        .variant(EuvButtonVariant::Primary)
        .on_click(move |_: &UiEvent| {
            let _: bool = counter.update(|v: &mut i32| *v += 1);
        })
        .mount(ctx);
    let batch_btn: NodeId = EuvButton::create(BTN_BATCH_TEN.to_string(), tokens)
        .variant(EuvButtonVariant::Secondary)
        .on_click(move |_: &UiEvent| {
            batch(|| {
                for _ in 0..10 {
                    let _: bool = counter.update(|v: &mut i32| *v += 1);
                }
            });
        })
        .mount(ctx);
    let reset_btn: NodeId = EuvButton::create(BTN_RESET.to_string(), tokens)
        .variant(EuvButtonVariant::Outline)
        .on_click(move |_: &UiEvent| {
            let _: bool = counter.set_if_changed(0);
        })
        .mount(ctx);
    ctx.append(btn_row, inc_btn);
    ctx.append(btn_row, batch_btn);
    ctx.append(btn_row, reset_btn);
    let card: NodeId = EuvCard::create(REACTIVE_CARD_TITLE.to_string(), tokens)
        .subtitle(REACTIVE_CARD_SUB.to_string())
        .child(counter_txt)
        .child(memo_txt)
        .child(btn_row)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, card);
    page
}

/// Mounts the multi-mode rendering inspector with live mode switcher and vector `Canvas`.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted rendering page node.
pub fn mount_render_page(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId = EuvHeader::create(
        RENDER_TITLE.to_string(),
        RENDER_SUBTITLE.to_string(),
        tokens,
    )
    .mount(ctx);
    let seg: NodeId = EuvSegmented::create(
        vec![
            MODE_AUTO.to_string(),
            MODE_SOFTWARE.to_string(),
            MODE_HARDWARE.to_string(),
            MODE_HYBRID.to_string(),
        ],
        state.get_render_mode_index(),
        tokens,
    )
    .mount(ctx);
    let canvas_label: NodeId = ctx.text(
        CANVAS_LABEL.to_string(),
        Style::empty()
            .font_size(13.0)
            .font_weight(FontWeight::Medium)
            .color(p.get_muted_foreground()),
    );
    let canvas: NodeId = ctx.canvas(
        Style::empty()
            .width(Length::Percent(100.0))
            .height(Length::Px(140.0))
            .background(p.get_muted())
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_md()),
        move |bounds: Rect, dl: &mut DisplayList| {
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
        },
    );
    let card: NodeId = EuvCard::create(RENDER_CARD_TITLE.to_string(), tokens)
        .subtitle(RENDER_CARD_SUB.to_string())
        .child(seg)
        .child(canvas_label)
        .child(canvas)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, card);
    page
}

/// Mounts the window settings and platform capabilities inspector page.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted window inspector page node.
pub fn mount_window_page(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId = EuvHeader::create(
        WINDOW_TITLE.to_string(),
        WINDOW_SUBTITLE.to_string(),
        tokens,
    )
    .mount(ctx);
    let platform_line: NodeId = ctx.text(
        format!(
            "Platform: {:?} | Breakpoint: {:?}",
            ctx.get_platform(),
            ctx.get_breakpoint().get_untracked()
        ),
        Style::empty()
            .font_size(14.0)
            .font_weight(FontWeight::Medium)
            .color(p.get_foreground()),
    );
    let fs_switch: NodeId = EuvSwitch::create(
        state.get_fullscreen(),
        SWITCH_FULLSCREEN.to_string(),
        tokens,
    )
    .mount(ctx);
    let code: NodeId = EuvCodeBlock::create(CODE_WINDOW_SNIPPET.to_string(), tokens).mount(ctx);
    let card: NodeId = EuvCard::create(WINDOW_CARD_TITLE.to_string(), tokens)
        .subtitle(WINDOW_CARD_SUB.to_string())
        .child(platform_line)
        .child(fs_switch)
        .child(code)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, card);
    page
}

/// Mounts the interactive forms showcase with inputs, switches, checkboxes, tabs, and buttons.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted forms page node.
pub fn mount_forms_page(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let username: Signal<String> = state.get_username();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId =
        EuvHeader::create(FORMS_TITLE.to_string(), FORMS_SUBTITLE.to_string(), tokens).mount(ctx);
    let tabs: NodeId = EuvTabs::create(
        vec![
            TAB_GENERAL.to_string(),
            TAB_GRAPHICS.to_string(),
            TAB_SECURITY.to_string(),
        ],
        state.get_active_tab(),
        tokens,
    )
    .mount(ctx);
    let input: NodeId =
        EuvInput::create(username, PLACEHOLDER_USERNAME.to_string(), tokens).mount(ctx);
    let field: NodeId = EuvField::create(FIELD_USERNAME.to_string(), input, tokens).mount(ctx);
    let sw: NodeId = EuvSwitch::create(
        state.get_hardware_accel(),
        SWITCH_HW_LABEL.to_string(),
        tokens,
    )
    .mount(ctx);
    let cb: NodeId = EuvCheckbox::create(
        state.get_agree_terms(),
        CHECKBOX_TERMS_LABEL.to_string(),
        tokens,
    )
    .mount(ctx);
    let btn_row: NodeId = ctx.view(Style::empty().row().gap(SPACE_SM));
    let b1: NodeId = EuvButton::create(BTN_PRIMARY_LABEL.to_string(), tokens)
        .variant(EuvButtonVariant::Primary)
        .mount(ctx);
    let b2: NodeId = EuvButton::create(BTN_SECONDARY_LABEL.to_string(), tokens)
        .variant(EuvButtonVariant::Secondary)
        .mount(ctx);
    let b3: NodeId = EuvButton::create(BTN_OUTLINE_LABEL.to_string(), tokens)
        .variant(EuvButtonVariant::Outline)
        .mount(ctx);
    let b4: NodeId = EuvButton::create(BTN_DESTRUCTIVE_LABEL.to_string(), tokens)
        .variant(EuvButtonVariant::Destructive)
        .on_click(move |_: &UiEvent| {
            let _: bool = username.set_if_changed(DEFAULT_USERNAME.to_string());
        })
        .mount(ctx);
    ctx.append(btn_row, b1);
    ctx.append(btn_row, b2);
    ctx.append(btn_row, b3);
    ctx.append(btn_row, b4);
    let card: NodeId = EuvCard::create(FORMS_CARD_TITLE.to_string(), tokens)
        .subtitle(FORMS_CARD_SUB.to_string())
        .child(tabs)
        .child(field)
        .child(sw)
        .child(cb)
        .child(btn_row)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, card);
    page
}

/// Mounts the 17-scheme live design token gallery page.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted themes page node.
pub fn mount_themes_page(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let scheme_sig: Signal<DesignScheme> = state.get_theme().get_scheme();
    let mode_sig: Signal<ThemeMode> = state.get_theme().get_mode();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId = EuvHeader::create(
        THEMES_TITLE.to_string(),
        THEMES_SUBTITLE.to_string(),
        tokens,
    )
    .mount(ctx);
    let active_txt: NodeId = ctx.reactive_text(
        Style::empty()
            .font_size(15.0)
            .font_weight(FontWeight::SemiBold)
            .color(p.get_foreground()),
        move || {
            format!(
                "{PREFIX_ACTIVE_SCHEME}{} ({:?})",
                scheme_sig.get().as_str(),
                mode_sig.get()
            )
        },
    );
    let state_for_cycle: ShowcaseState = state.clone();
    let state_for_mode: ShowcaseState = state.clone();
    let actions: NodeId = ctx.view(Style::empty().row().gap(SPACE_SM));
    let cycle_btn: NodeId = EuvButton::create(BTN_NEXT_SCHEME.to_string(), tokens)
        .variant(EuvButtonVariant::Primary)
        .on_click(move |_: &UiEvent| {
            state_for_cycle.next_scheme();
        })
        .mount(ctx);
    let mode_btn: NodeId = EuvButton::create(BTN_TOGGLE_MODE.to_string(), tokens)
        .variant(EuvButtonVariant::Outline)
        .on_click(move |_: &UiEvent| {
            state_for_mode.toggle_theme_mode();
        })
        .mount(ctx);
    ctx.append(actions, cycle_btn);
    ctx.append(actions, mode_btn);
    let badges_grid: NodeId = ctx.view(
        Style::empty()
            .grid(4)
            .width(Length::Percent(100.0))
            .gap(SPACE_SM),
    );
    let all_schemes: [DesignScheme; 17] = DesignScheme::all();
    for scheme in all_schemes.iter() {
        let preview_tokens: ThemeTokens = ThemeTokens::resolve(*scheme, mode_sig.get_untracked());
        let badge: NodeId = EuvBadge::create(
            scheme.as_str().to_string(),
            EuvTone::Primary,
            preview_tokens,
        )
        .mount(ctx);
        ctx.append(badges_grid, badge);
    }
    let card: NodeId = EuvCard::create(THEMES_CARD_TITLE.to_string(), tokens)
        .subtitle(THEMES_CARD_SUB.to_string())
        .child(active_txt)
        .child(actions)
        .child(badges_grid)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, card);
    page
}

/// Mounts the 10,000-row virtualized list and progress display page.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted data display page node.
pub fn mount_data_page(ctx: &mut BuildContext, tokens: ThemeTokens) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let g: ThemeGeometry = tokens.get_geometry();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId =
        EuvHeader::create(DATA_TITLE.to_string(), DATA_SUBTITLE.to_string(), tokens).mount(ctx);
    let badge_row: NodeId = ctx.view(Style::empty().row().gap(SPACE_SM));
    let b1: NodeId = EuvBadge::create(BADGE_READY.to_string(), EuvTone::Success, tokens).mount(ctx);
    let b2: NodeId = EuvBadge::create(BADGE_LIVE.to_string(), EuvTone::Primary, tokens).mount(ctx);
    ctx.append(badge_row, b1);
    ctx.append(badge_row, b2);
    let progress: NodeId = EuvProgress::create(0.84, tokens).mount(ctx);
    let vlist: EuvVirtualList = EuvVirtualList::create(10_000, 32.0, 224.0, 640.0, tokens);
    let list_node: NodeId = vlist.mount_with(ctx, |row_ctx: &mut BuildContext, idx: usize| {
        let row: NodeId = row_ctx.view(
            Style::empty()
                .row()
                .width(Length::Percent(100.0))
                .height(Length::Px(32.0))
                .padding(Edges::symmetric(SPACE_XS, SPACE_MD))
                .align(AlignItems::Center)
                .border(g.get_border_width(), p.get_border()),
        );
        let txt: NodeId = row_ctx.text(
            format!("{ROW_PREFIX}{idx}"),
            Style::empty().font_size(13.0).color(p.get_foreground()),
        );
        row_ctx.append(row, txt);
        row
    });
    let card: NodeId = EuvCard::create(DATA_CARD_TITLE.to_string(), tokens)
        .subtitle(DATA_CARD_SUB.to_string())
        .child(badge_row)
        .child(progress)
        .child(list_node)
        .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, card);
    page
}

/// Mounts the overlays page featuring `EuvModal`, `EuvToast`, and `EuvAlert`.
///
/// # Arguments
///
/// - `&mut BuildContext` - Active build context.
/// - `&ShowcaseState` - Reactive showcase state.
/// - `ThemeTokens` - Active design tokens.
///
/// # Returns
///
/// - `NodeId` - Mounted overlays page node.
pub fn mount_overlays_page(
    ctx: &mut BuildContext,
    state: &ShowcaseState,
    tokens: ThemeTokens,
) -> NodeId {
    let p: ThemePalette = tokens.get_palette();
    let modal_open: Signal<bool> = state.get_modal_open();
    let toast_visible: Signal<bool> = state.get_toast_visible();
    let page: NodeId = ctx.view(
        Style::empty()
            .column()
            .width(Length::Percent(100.0))
            .gap(SPACE_LG),
    );
    let header: NodeId = EuvHeader::create(
        OVERLAYS_TITLE.to_string(),
        OVERLAYS_SUBTITLE.to_string(),
        tokens,
    )
    .mount(ctx);
    let alert: NodeId = EuvAlert::create(
        ALERT_TITLE.to_string(),
        ALERT_DESC.to_string(),
        EuvTone::Primary,
        tokens,
    )
    .mount(ctx);
    let actions: NodeId = ctx.view(Style::empty().row().gap(SPACE_SM));
    let open_btn: NodeId = EuvButton::create(BTN_OPEN_MODAL.to_string(), tokens)
        .variant(EuvButtonVariant::Primary)
        .on_click(move |_: &UiEvent| {
            let _: bool = modal_open.set_if_changed(true);
        })
        .mount(ctx);
    let toast_btn: NodeId = EuvButton::create(BTN_TOGGLE_TOAST.to_string(), tokens)
        .variant(EuvButtonVariant::Secondary)
        .on_click(move |_: &UiEvent| {
            let _: bool = toast_visible.update(|v: &mut bool| *v = !*v);
        })
        .mount(ctx);
    ctx.append(actions, open_btn);
    ctx.append(actions, toast_btn);
    let card: NodeId = EuvCard::create(OVERLAYS_CARD_TITLE.to_string(), tokens)
        .subtitle(OVERLAYS_CARD_SUB.to_string())
        .child(actions)
        .mount(ctx);
    let modal_body: NodeId = ctx.view(Style::empty().column().gap(SPACE_MD));
    let modal_copy: NodeId = ctx.text(
        MODAL_BODY_TEXT.to_string(),
        Style::empty()
            .font_size(13.0)
            .color(p.get_muted_foreground()),
    );
    let close_btn: NodeId = EuvButton::create(BTN_CLOSE_MODAL.to_string(), tokens)
        .variant(EuvButtonVariant::Outline)
        .on_click(move |_: &UiEvent| {
            let _: bool = modal_open.set_if_changed(false);
        })
        .mount(ctx);
    ctx.append(modal_body, modal_copy);
    ctx.append(modal_body, close_btn);
    let modal: NodeId =
        EuvModal::create(modal_open, MODAL_TITLE.to_string(), modal_body, tokens).mount(ctx);
    let toast: NodeId = EuvToast::create(
        toast_visible,
        TOAST_TEXT.to_string(),
        EuvTone::Success,
        tokens,
    )
    .mount(ctx);
    ctx.append(page, header);
    ctx.append(page, alert);
    ctx.append(page, card);
    ctx.append(page, modal);
    ctx.append(page, toast);
    page
}
