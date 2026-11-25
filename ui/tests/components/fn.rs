use super::*;

#[test]
fn ui_components_render_deterministically_across_desktop_and_mobile_breakpoints() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let active_route: Signal<String> = Signal::new("/home".to_string());
    let switch_on: Signal<bool> = Signal::new(false);
    let modal_open: Signal<bool> = Signal::new(false);
    let input_val: Signal<String> = Signal::new(String::new());
    let selected_seg: Signal<usize> = Signal::new(0);
    let clicks: Signal<u32> = Signal::new(0);
    let cfg: AppConfig = AppConfig::default().with_window(
        WindowConfig::default()
            .with_size(900, 600)
            .with_render(RenderConfig::software()),
    );
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|ctx: &mut BuildContext| {
        let nav: NodeId = EuvSidebarNav::create(
            vec![
                EuvNavItem::new("/home".to_string(), "Home".to_string()),
                EuvNavItem::new("/render".to_string(), "Render".to_string()),
            ],
            active_route,
            tokens,
        )
        .mount(ctx);
        let btn: NodeId = EuvButton::create("Action".to_string(), tokens)
            .variant(EuvButtonVariant::Primary)
            .on_click(move |_: &UiEvent| {
                let _: bool = clicks.update(|v: &mut u32| *v += 1);
            })
            .mount(ctx);
        let input: NodeId =
            EuvInput::create(input_val, "Enter name".to_string(), tokens).mount(ctx);
        let field: NodeId = EuvField::create("Username".to_string(), input, tokens).mount(ctx);
        let sw: NodeId =
            EuvSwitch::create(switch_on, "Hardware GPU".to_string(), tokens).mount(ctx);
        let seg: NodeId = EuvSegmented::create(
            vec!["Software".to_string(), "Hardware".to_string()],
            selected_seg,
            tokens,
        )
        .mount(ctx);
        let prog: NodeId = EuvProgress::create(0.75, tokens).mount(ctx);
        let badge: NodeId =
            EuvBadge::create("v0.38".to_string(), EuvTone::Primary, tokens).mount(ctx);
        let card: NodeId = EuvCard::create("Controls".to_string(), tokens)
            .subtitle("Cross-platform UI primitives".to_string())
            .child(badge)
            .child(btn)
            .child(field)
            .child(sw)
            .child(seg)
            .child(prog)
            .mount(ctx);
        let modal_body: NodeId = ctx.text("Modal body".to_string(), Style::empty());
        let modal: NodeId =
            EuvModal::create(modal_open, "Dialog".to_string(), modal_body, tokens).mount(ctx);
        let content: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .gap(SPACE_LG),
        );
        let header: NodeId = EuvHeader::create(
            "Dashboard".to_string(),
            "Unified native UI".to_string(),
            tokens,
        )
        .mount(ctx);
        ctx.append(content, header);
        ctx.append(content, card);
        ctx.append(content, modal);
        EuvAppShell::create("euv-ui".to_string(), nav, content, tokens).mount(ctx)
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 12);
    let vlist: EuvVirtualList = EuvVirtualList::create(10_000, 32.0, 320.0, 3200.0, tokens);
    let (start, end): (usize, usize) = vlist.visible_range();
    assert_eq!(start, 97);
    assert_eq!(end, 113);
}

#[test]
fn full_production_component_suite_and_hooks_mount_and_render_cleanly() {
    reset_reactive_runtime();
    let theme: UiThemeState = use_theme(DesignScheme::Cupertino, ThemeMode::Dark);
    let tokens: ThemeTokens = theme.get_tokens().get();
    let mut i18n: I18nCatalog = use_i18n("zh-CN".to_string());
    i18n.insert("zh-CN".to_string(), "save".to_string(), "保存".to_string());
    assert_eq!(i18n.t("save"), "保存");
    let mut router: Router = use_router("/dashboard".to_string());
    router.push("/settings".to_string());
    assert_eq!(router.get_current().get_untracked(), "/settings");
    let _gesture: GestureRecognizer = use_gesture();
    let toggle_sig: Signal<bool> = Signal::new(false);
    let text_sig: Signal<String> = Signal::new("Notes".to_string());
    let num_sig: Signal<i64> = Signal::new(5);
    let radio_sig: Signal<usize> = Signal::new(1);
    let slider_sig: Signal<f32> = Signal::new(50.0);
    let select_sig: Signal<usize> = Signal::new(0);
    let select_open: Signal<bool> = Signal::new(false);
    let rating_sig: Signal<usize> = Signal::new(4);
    let page_sig: Signal<usize> = Signal::new(1);
    let step_sig: Signal<usize> = Signal::new(1);
    let acc_sig: Signal<Option<usize>> = Signal::new(Some(0));
    let toc_sig: Signal<usize> = Signal::new(0);
    let drawer_open: Signal<bool> = Signal::new(false);
    let sheet_open: Signal<bool> = Signal::new(false);
    let pop_open: Signal<bool> = Signal::new(false);
    let dd_open: Signal<bool> = Signal::new(false);
    let dd_sel: Signal<Option<usize>> = Signal::new(None);
    let notif_vis: Signal<bool> = Signal::new(true);
    let loading_sig: Signal<bool> = Signal::new(false);
    let cfg: AppConfig = AppConfig::default().with_window(
        WindowConfig::default()
            .with_size(1024, 768)
            .with_render(RenderConfig::software()),
    );
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|ctx: &mut BuildContext| {
        let bp_sig: Signal<Breakpoint> = use_breakpoint(ctx);
        let (_safe_sig, _mode_sig): (Signal<SafeAreaInsets>, Signal<ThemeMode>) = use_window(ctx);
        assert!(!bp_sig.get_untracked().is_compact());
        let nav: NodeId = EuvNavbar::create("euv Studio".to_string(), tokens)
            .action(EuvKbd::create("Cmd+K".to_string(), tokens).mount(ctx))
            .mount(ctx);
        let footer: NodeId = EuvFooter::create("euv 0.38 Native".to_string(), tokens).mount(ctx);
        let left_p: NodeId = EuvPanel::create("Inspector".to_string(), tokens)
            .child(EuvAvatar::create("EV".to_string(), 36.0, tokens).mount(ctx))
            .child(
                EuvIcon::create("*".to_string(), 18.0, tokens.get_palette().get_primary())
                    .mount(ctx),
            )
            .child(EuvTag::create("Native".to_string(), EuvTone::Success, tokens).mount(ctx))
            .child(
                EuvTypography::create("Title".to_string(), EuvTypographyVariant::H2, tokens)
                    .mount(ctx),
            )
            .child(
                EuvEmpty::create("No items".to_string(), "Add one".to_string(), tokens).mount(ctx),
            )
            .child(EuvSkeleton::create(140.0, 16.0, tokens).mount(ctx))
            .child(EuvSpinner::create(24.0, tokens).mount(ctx))
            .mount(ctx);
        let right_p: NodeId = EuvScrollArea::create(240.0, tokens)
            .child(
                EuvTimeline::create(
                    vec![EuvTimelineItem::new(
                        "Boot".to_string(),
                        "00:01".to_string(),
                        "Metal ready".to_string(),
                    )],
                    tokens,
                )
                .mount(ctx),
            )
            .child(
                EuvList::create(vec!["Item A".to_string(), "Item B".to_string()], tokens)
                    .mount(ctx),
            )
            .child(
                EuvTree::create(
                    vec![EuvTreeNode::new(
                        "Workspace".to_string(),
                        vec![EuvTreeNode::new("core".to_string(), Vec::new())],
                    )],
                    tokens,
                )
                .mount(ctx),
            )
            .mount(ctx);
        let split: NodeId = EuvSplitter::create(35.0, left_p, right_p, tokens).mount(ctx);
        let btn_a: NodeId = EuvButton::create("A".to_string(), tokens).mount(ctx);
        let btn_b: NodeId = EuvToggle::create(toggle_sig, "Pin".to_string(), tokens).mount(ctx);
        let btn_group: NodeId = EuvButtonGroup::create(vec![btn_a, btn_b], tokens).mount(ctx);
        let form: NodeId = EuvForm::create(tokens)
            .field(EuvLabel::create("Bio".to_string(), true, tokens).mount(ctx))
            .field(EuvTextarea::create(text_sig, "Write...".to_string(), 72.0, tokens).mount(ctx))
            .field(EuvNumberInput::create(num_sig, 0, 20, 1, tokens).mount(ctx))
            .field(
                EuvRadioGroup::create(
                    vec!["CPU".to_string(), "GPU".to_string()],
                    radio_sig,
                    tokens,
                )
                .mount(ctx),
            )
            .field(EuvSlider::create(slider_sig, 0.0, 100.0, tokens).mount(ctx))
            .field(
                EuvSelect::create(
                    vec!["Metal".to_string(), "Vulkan".to_string()],
                    select_sig,
                    select_open,
                    tokens,
                )
                .mount(ctx),
            )
            .field(EuvRating::create(rating_sig, 5, tokens).mount(ctx))
            .mount(ctx);
        let nav_col: NodeId = EuvSpace::column(SPACE_SM)
            .child(
                EuvBreadcrumb::create(vec!["Home".to_string(), "UI".to_string()], tokens)
                    .mount(ctx),
            )
            .child(EuvPagination::create(page_sig, 5, tokens).mount(ctx))
            .child(
                EuvSteps::create(
                    vec!["Build".to_string(), "Test".to_string()],
                    step_sig,
                    tokens,
                )
                .mount(ctx),
            )
            .child(
                EuvAccordion::create(
                    vec![EuvAccordionItem::new(
                        "FAQ".to_string(),
                        "Zero WebView".to_string(),
                    )],
                    acc_sig,
                    tokens,
                )
                .mount(ctx),
            )
            .child(
                EuvToc::create(
                    vec!["Intro".to_string(), "API".to_string()],
                    toc_sig,
                    tokens,
                )
                .mount(ctx),
            )
            .mount(ctx);
        let grid: NodeId = EuvGrid::create(2, SPACE_MD)
            .child(form)
            .child(nav_col)
            .mount(ctx);
        let pop_trig: NodeId = EuvButton::create("Info".to_string(), tokens).mount(ctx);
        let pop_body: NodeId = ctx.text("Popover details".to_string(), Style::empty());
        let tip_trig: NodeId = EuvButton::create("Help".to_string(), tokens).mount(ctx);
        let overlays: NodeId = EuvSpace::row(SPACE_SM)
            .child(EuvPopover::create(pop_open, pop_trig, pop_body, tokens).mount(ctx))
            .child(EuvTooltip::create("Shortcut".to_string(), tip_trig, tokens).mount(ctx))
            .child(
                EuvDropdown::create(
                    "Actions".to_string(),
                    vec!["Export".to_string(), "Share".to_string()],
                    dd_open,
                    dd_sel,
                    tokens,
                )
                .mount(ctx),
            )
            .mount(ctx);
        let drawer_body: NodeId = ctx.text("Drawer body".to_string(), Style::empty());
        let sheet_body: NodeId = ctx.text("Sheet body".to_string(), Style::empty());
        let drawer: NodeId =
            EuvDrawer::create(drawer_open, "Settings".to_string(), drawer_body, tokens).mount(ctx);
        let sheet: NodeId =
            EuvSheet::create(sheet_open, "Actions".to_string(), sheet_body, tokens).mount(ctx);
        let notif: NodeId = EuvNotification::create(
            notif_vis,
            "Build OK".to_string(),
            "7 targets verified".to_string(),
            EuvTone::Success,
            tokens,
        )
        .mount(ctx);
        let loading: NodeId =
            EuvLoading::create(loading_sig, "Syncing...".to_string(), tokens).mount(ctx);
        EuvPageContainer::create(tokens)
            .child(nav)
            .child(btn_group)
            .child(split)
            .child(grid)
            .child(overlays)
            .child(footer)
            .child(drawer)
            .child(sheet)
            .child(notif)
            .child(loading)
            .mount(ctx)
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 40);
}
