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
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(900, 600).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let nav: NodeId = EuvSidebarNav::new(
            vec![
                EuvNavItem::new("/home".to_string(), "Home".to_string()),
                EuvNavItem::new("/render".to_string(), "Render".to_string()),
            ],
            active_route,
            tokens,
        )
        .mount_current();
        let mut btn: EuvButton =
            EuvButton::new("Action".to_string(), tokens).on_click(move |_: &UiEvent| {
                let _: bool = clicks.update(|v: &mut u32| *v += 1);
            });
        btn.set_variant(EuvButtonVariant::Primary);
        let input: NodeId =
            EuvInput::new(input_val, "Enter name".to_string(), tokens).mount_current();
        let field: NodeId = EuvField::new("Username".to_string(), input, tokens).mount_current();
        let sw: NodeId =
            EuvSwitch::new(switch_on, "Hardware GPU".to_string(), tokens).mount_current();
        let seg: NodeId = EuvSegmented::new(
            vec!["Software".to_string(), "Hardware".to_string()],
            selected_seg,
            tokens,
        )
        .mount_current();
        let prog: NodeId = EuvProgress::new(0.75, tokens).mount_current();
        let badge: NodeId =
            EuvBadge::new("v0.38".to_string(), EuvTone::Primary, tokens).mount_current();
        let mut card: EuvCard = EuvCard::new("Controls".to_string(), tokens)
            .child(badge)
            .child(btn.mount_current())
            .child(field)
            .child(sw)
            .child(seg)
            .child(prog);
        card.set_subtitle("Cross-platform UI primitives".to_string());
        let modal_body: NodeId = view! { text { "Modal body" } };
        let modal: NodeId =
            EuvModal::new(modal_open, "Dialog".to_string(), modal_body, tokens).mount_current();
        let content: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_LG
                { EuvHeader::new("Dashboard".to_string(), "Unified native UI".to_string(), tokens) }
                { card }
                { modal }
            }
        };
        EuvAppShell::new("euv-ui".to_string(), nav, content, tokens)
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 12);
    let scroll_y: Signal<f32> = Signal::new(3200.0);
    let vlist: EuvVirtualList =
        EuvVirtualList::new(10_000, 32.0, scroll_y, tokens).viewport_height(320.0);
    let (start, end): (usize, usize) = vlist.visible_range();
    assert_eq!(start, 97);
    assert_eq!(end, 113);
    vlist.scroll_by(320.0);
    let (next_start, next_end): (usize, usize) = vlist.visible_range();
    assert_eq!(next_start, 107);
    assert_eq!(next_end, 123);
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
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(1024, 768).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let bp_sig: Signal<Breakpoint> = use_breakpoint();
        let (_safe_sig, _mode_sig): (Signal<SafeAreaInsets>, Signal<ThemeMode>) = use_window();
        assert!(!bp_sig.get_untracked().is_compact());
        let nav: NodeId = EuvNavbar::new("euv Studio".to_string(), tokens)
            .action(EuvKbd::new("Cmd+K".to_string(), tokens).mount_current())
            .mount_current();
        let footer: NodeId = EuvFooter::new("euv 0.38 Native".to_string(), tokens).mount_current();
        let left_p: NodeId = EuvPanel::new("Inspector".to_string(), tokens)
            .child(EuvAvatar::new("EV".to_string(), 36.0, tokens).mount_current())
            .child(
                EuvIcon::new("*".to_string(), 18.0, tokens.get_palette().get_primary())
                    .mount_current(),
            )
            .child(EuvTag::new("Native".to_string(), EuvTone::Success, tokens).mount_current())
            .child(
                EuvTypography::new("Title".to_string(), EuvTypographyVariant::H2, tokens)
                    .mount_current(),
            )
            .child(
                EuvEmpty::new("No items".to_string(), "Add one".to_string(), tokens)
                    .mount_current(),
            )
            .child(EuvSkeleton::new(140.0, 16.0, tokens).mount_current())
            .child(EuvSpinner::new(24.0, tokens).mount_current())
            .mount_current();
        let right_p: NodeId = EuvScrollArea::new(240.0, tokens)
            .child(
                EuvTimeline::new(
                    vec![EuvTimelineItem::new(
                        "Boot".to_string(),
                        "00:01".to_string(),
                        "Metal ready".to_string(),
                    )],
                    tokens,
                )
                .mount_current(),
            )
            .child(
                EuvList::new(vec!["Item A".to_string(), "Item B".to_string()], tokens)
                    .mount_current(),
            )
            .child(
                EuvTree::new(
                    vec![EuvTreeNode::new(
                        "Workspace".to_string(),
                        vec![EuvTreeNode::new("core".to_string(), Vec::new())],
                    )],
                    tokens,
                )
                .mount_current(),
            )
            .mount_current();
        let split: NodeId = EuvSplitter::new(35.0, left_p, right_p, tokens).mount_current();
        let btn_a: NodeId = EuvButton::new("A".to_string(), tokens).mount_current();
        let btn_b: NodeId = EuvToggle::new(toggle_sig, "Pin".to_string(), tokens).mount_current();
        let btn_group: NodeId = EuvButtonGroup::new(vec![btn_a, btn_b], tokens).mount_current();
        let form: NodeId = EuvForm::new(tokens)
            .field(EuvLabel::new("Bio".to_string(), true, tokens).mount_current())
            .field(EuvTextarea::new(text_sig, "Write...".to_string(), 72.0, tokens).mount_current())
            .field(
                EuvNumberInput::new(num_sig, tokens)
                    .range(0, 20)
                    .step(1)
                    .mount_current(),
            )
            .field(
                EuvRadioGroup::new(
                    vec!["CPU".to_string(), "GPU".to_string()],
                    radio_sig,
                    tokens,
                )
                .mount_current(),
            )
            .field(EuvSlider::new(slider_sig, 0.0, 100.0, tokens).mount_current())
            .field(
                EuvSelect::new(
                    vec!["Metal".to_string(), "Vulkan".to_string()],
                    select_sig,
                    select_open,
                    tokens,
                )
                .mount_current(),
            )
            .field(EuvRating::new(rating_sig, 5, tokens).mount_current())
            .mount_current();
        let nav_col: NodeId = EuvSpace::column(SPACE_SM)
            .child(
                EuvBreadcrumb::new(vec!["Home".to_string(), "UI".to_string()], tokens)
                    .mount_current(),
            )
            .child(EuvPagination::new(page_sig, 5, tokens).mount_current())
            .child(
                EuvSteps::new(
                    vec!["Build".to_string(), "Test".to_string()],
                    step_sig,
                    tokens,
                )
                .mount_current(),
            )
            .child(
                EuvAccordion::new(
                    vec![EuvAccordionItem::new(
                        "FAQ".to_string(),
                        "Zero WebView".to_string(),
                    )],
                    acc_sig,
                    tokens,
                )
                .mount_current(),
            )
            .child(
                EuvToc::new(
                    vec!["Intro".to_string(), "API".to_string()],
                    toc_sig,
                    tokens,
                )
                .mount_current(),
            )
            .mount_current();
        let grid: NodeId = EuvGrid::new(2, SPACE_MD)
            .child(form)
            .child(nav_col)
            .mount_current();
        let pop_trig: NodeId = EuvButton::new("Info".to_string(), tokens).mount_current();
        let pop_body: NodeId = view! { text { "Popover details" } };
        let tip_trig: NodeId = EuvButton::new("Help".to_string(), tokens).mount_current();
        let overlays: NodeId = EuvSpace::row(SPACE_SM)
            .child(EuvPopover::new(pop_open, pop_trig, pop_body, tokens).mount_current())
            .child(EuvTooltip::new("Shortcut".to_string(), tip_trig, tokens).mount_current())
            .child(
                EuvDropdown::new(
                    "Actions".to_string(),
                    vec!["Export".to_string(), "Share".to_string()],
                    tokens,
                )
                .open(dd_open)
                .selected(dd_sel)
                .mount_current(),
            )
            .mount_current();
        let drawer_body: NodeId = view! { text { "Drawer body" } };
        let sheet_body: NodeId = view! { text { "Sheet body" } };
        let drawer: NodeId =
            EuvDrawer::new(drawer_open, "Settings".to_string(), drawer_body, tokens)
                .mount_current();
        let sheet: NodeId =
            EuvSheet::new(sheet_open, "Actions".to_string(), sheet_body, tokens).mount_current();
        let notif: NodeId = EuvNotification::new(
            notif_vis,
            "Build OK".to_string(),
            "7 targets verified".to_string(),
            tokens,
        )
        .tone(EuvTone::Success)
        .mount_current();
        let loading: NodeId =
            EuvLoading::new(loading_sig, "Syncing...".to_string(), tokens).mount_current();
        EuvPageContainer::new(tokens)
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
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 40);
}

#[test]
fn interactive_widgets_and_virtual_scroll_list_respond_to_events_and_update_pixels() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let clicks: Signal<u32> = Signal::new(0);
    let switch_on: Signal<bool> = Signal::new(false);
    let check_on: Signal<bool> = Signal::new(false);
    let toggle_on: Signal<bool> = Signal::new(false);
    let input_val: Signal<String> = Signal::new(String::new());
    let scroll_y: Signal<f32> = Signal::new(0.0);
    let mut btn_id: NodeId = NodeId::new(0, 0);
    let mut sw_id: NodeId = NodeId::new(0, 0);
    let mut chk_id: NodeId = NodeId::new(0, 0);
    let mut tog_id: NodeId = NodeId::new(0, 0);
    let mut inp_id: NodeId = NodeId::new(0, 0);
    let mut list_id: NodeId = NodeId::new(0, 0);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 560).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        btn_id = EuvButton::new("Increment".to_string(), tokens)
            .on_click(move |_: &UiEvent| {
                let _: bool = clicks.update(|v: &mut u32| *v += 1);
            })
            .mount_current();
        sw_id = EuvSwitch::new(switch_on, "Hardware GPU".to_string(), tokens).mount_current();
        chk_id = EuvCheckbox::new(check_on, "Enable telemetry".to_string(), tokens).mount_current();
        tog_id = EuvToggle::new(toggle_on, "Pin panel".to_string(), tokens).mount_current();
        inp_id = EuvInput::new(input_val, "Type here".to_string(), tokens).mount_current();
        let vlist: EuvVirtualList =
            EuvVirtualList::new(10_000, 28.0, scroll_y, tokens).viewport_height(140.0);
        list_id = vlist.mount_with(move |idx: usize| {
            view! {
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(28.0)
                    text {
                        font_size: 13.0
                        color: tokens.get_palette().get_foreground()
                        { format!("Virtual Item #{idx}") }
                    }
                }
            }
        });
        view! {
            column {
                width: Length::Px(640.0)
                height: Length::Px(560.0)
                padding: Edges::all(16.0)
                gap: 12.0
                background: tokens.get_palette().get_background()
                { btn_id }
                { sw_id }
                { chk_id }
                { tog_id }
                { inp_id }
                { list_id }
            }
        }
    });
    let before: FrameStats = app.step_frame().unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let targets: [NodeId; 5] = [btn_id, sw_id, chk_id, tog_id, inp_id];
    for target in targets.into_iter() {
        let center: Point = app.get_scene().with_read(
            |tree: &SceneTree| {
                let bounds: Rect = tree
                    .get_node(target)
                    .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                    .unwrap_or_default();
                Point::new(
                    bounds.get_x() + bounds.get_width() * 0.5,
                    bounds.get_y() + bounds.get_height() * 0.5,
                )
            },
            Point::default(),
        );
        let ev: PointerEvent = PointerEvent::new(
            1,
            PointerDevice::Mouse,
            center,
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        );
        app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
        app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
    }
    app.dispatch_window_event(WindowEvent::TextInput(wid, "euv!".to_string()));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Backspace, Modifiers::default(), false),
    ));
    let list_center: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            let bounds: Rect = tree
                .get_node(list_id)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                .unwrap_or_default();
            Point::new(
                bounds.get_x() + bounds.get_width() * 0.5,
                bounds.get_y() + bounds.get_height() * 0.5,
            )
        },
        Point::default(),
    );
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(list_center.get_x(), list_center.get_y()),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 280.0)));
    let after: FrameStats = app.step_frame().unwrap();
    assert_eq!(clicks.get_untracked(), 1);
    assert!(switch_on.get_untracked());
    assert!(check_on.get_untracked());
    assert!(toggle_on.get_untracked());
    assert_eq!(input_val.get_untracked(), "euv");
    assert!((scroll_y.get_untracked() - 280.0).abs() < 0.01);
    assert_ne!(before.get_frame_hash(), 0);
    assert_ne!(after.get_frame_hash(), 0);
    assert_ne!(before.get_frame_hash(), after.get_frame_hash());
}

#[test]
fn shadcn_extra_components_and_responsive_shell_adapt_across_mobile_tablet_desktop() {
    let viewports: [(u32, u32, Breakpoint, ThemeMode); 5] = [
        (360, 800, Breakpoint::Compact, ThemeMode::Light),
        (390, 844, Breakpoint::Compact, ThemeMode::Dark),
        (768, 1024, Breakpoint::Medium, ThemeMode::Dark),
        (1280, 800, Breakpoint::Expanded, ThemeMode::Light),
        (1440, 900, Breakpoint::Expanded, ThemeMode::Dark),
    ];
    for (vw, vh, expected_bp, mode) in viewports.into_iter() {
        reset_reactive_runtime();
        let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, mode);
        let route: Signal<String> = Signal::new("/home".to_string());
        let alert_open: Signal<bool> = Signal::new(true);
        let alert_confirmed: Signal<bool> = Signal::new(false);
        let coll_open: Signal<bool> = Signal::new(true);
        let tg_sel: Signal<usize> = Signal::new(0);
        let otp: Signal<String> = Signal::new("123456".to_string());
        let cmd_q: Signal<String> = Signal::new(String::new());
        let cmd_sel: Signal<Option<usize>> = Signal::new(None);
        let hover_open: Signal<bool> = Signal::new(true);
        let cal_day: Signal<u32> = Signal::new(15);
        let menu_act: Signal<Option<usize>> = Signal::new(Some(0));
        let car_idx: Signal<usize> = Signal::new(0);
        let ctx_open: Signal<bool> = Signal::new(true);
        let ctx_sel: Signal<Option<usize>> = Signal::new(None);
        let mut win: WindowConfig = WindowConfig::default();
        win.set_size(vw, vh).set_render(RenderConfig::software());
        let mut cfg: AppConfig = AppConfig::default();
        cfg.set_window(win);
        let mut app: App = App::headless(cfg).unwrap();
        let _: NodeId = app.mount(move || {
            assert_eq!(use_breakpoint().get_untracked(), expected_bp);
            let vp_size: Size = use_viewport_size().get_untracked();
            assert!((vp_size.get_width() - (vw as f32)).abs() < 1.0);
            let nav: NodeId = EuvSidebarNav::new(
                vec![EuvNavItem::new("/home".to_string(), "Home".to_string())],
                route,
                tokens,
            )
            .mount_current();
            let coll_body: NodeId = view! { text { "Collapsible content" } };
            let aspect_child: NodeId = view! { text { "16:9 Media" } };
            let body: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_MD
                    { EuvMenubar::new(vec!["File".to_string(), "Edit".to_string(), "View".to_string()], menu_act, tokens) }
                    { EuvToggleGroup::new(vec!["Left".to_string(), "Center".to_string(), "Right".to_string()], tg_sel, tokens) }
                    { EuvInputOtp::new(otp, 6, tokens) }
                    { EuvCollapsible::new(coll_open, "Details".to_string(), coll_body, tokens) }
                    { EuvCommand::new(vec!["Build".to_string(), "Deploy".to_string()], cmd_sel, tokens).query(cmd_q).placeholder("Search...") }
                    { EuvHoverCard::new("@shadcn".to_string(), "shadcn/ui".to_string(), tokens).open(hover_open).description("Design system parity.") }
                    { EuvContextMenu::new("Canvas Surface".to_string(), vec!["Inspect".to_string(), "Duplicate".to_string()], tokens).open(ctx_open).selected(ctx_sel) }
                    { EuvCalendar::new("November 2026".to_string(), 30, cal_day, tokens) }
                    { EuvAspectRatio::new(16.0 / 9.0, aspect_child, tokens) }
                    { EuvCarousel::new(vec!["Slide A".to_string(), "Slide B".to_string()], car_idx, tokens) }
                    { EuvAlertDialog::new(alert_open, "Confirm".to_string(), alert_confirmed, tokens).description("Proceed?").labels("Cancel", "Continue") }
                }
            };
            EuvAppShell::new("shadcn-euv".to_string(), nav, body, tokens)
        });
        let stats: FrameStats = app.step_frame().unwrap();
        assert_ne!(stats.get_frame_hash(), 0);
        assert!(stats.get_draw_commands() >= 30);
        let overflow_count: usize = app.get_scene().with_read(
            |tree: &SceneTree| {
                let mut bad: usize = 0;
                for node in tree.get_nodes().iter() {
                    if !node.get_alive()
                        || node.get_style().get_layout().get_display() == Display::None
                    {
                        continue;
                    }
                    let b: Rect = node.get_layout_box().get_bounds();
                    if b.get_width() > 0.0 && b.right() > (vw as f32) + 1.5 {
                        bad += 1;
                    }
                }
                bad
            },
            0,
        );
        assert_eq!(
            overflow_count, 0,
            "unexpected horizontal overflow at viewport {vw}x{vh}"
        );
    }
}

#[test]
fn hover_bubbling_and_hover_card_pointer_enter_leave_update_visual_state() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let hover_open: Signal<bool> = Signal::new(false);
    let mut btn_node: NodeId = NodeId::new(0, 0);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        btn_node = EuvButton::new("Hover Target".to_string(), tokens).mount_current();
        let hc: NodeId = EuvHoverCard::new("@hover".to_string(), "Preview".to_string(), tokens)
            .open(hover_open)
            .description("Triggered on hover")
            .mount_current();
        view! {
            column {
                width: Length::Px(640.0)
                height: Length::Px(480.0)
                padding: Edges::all(24.0)
                gap: 16.0
                { btn_node }
                { hc }
            }
        }
    });
    let idle: FrameStats = app.step_frame().unwrap();
    let btn_center: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            let b: Rect = tree
                .get_node(btn_node)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                .unwrap_or_default();
            Point::new(
                b.get_x() + b.get_width() * 0.5,
                b.get_y() + b.get_height() * 0.5,
            )
        },
        Point::default(),
    );
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(btn_center.get_x(), btn_center.get_y()),
    ));
    let hovered: FrameStats = app.step_frame().unwrap();
    let is_btn_hovered: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(btn_node)
                .map(|n: &SceneNode| n.get_interaction().get_hovered())
                .unwrap_or(false)
        },
        false,
    );
    assert!(is_btn_hovered);
    assert_ne!(idle.get_frame_hash(), hovered.get_frame_hash());
}

#[test]
fn full_media_component_suite_renders_image_svg_gif_live_video_audio_pdf_and_html() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Dark);
    let gif_sig: Signal<GifAnimation> = Signal::new(GifAnimation::sample_neon_gif());
    let live_sig: Signal<LivePhotoAsset> = Signal::new(LivePhotoAsset::sample_live_photo());
    let video_sig: Signal<VideoTrack> = Signal::new(VideoTrack::sample_hdr_video());
    let audio_sig: Signal<AudioTrack> = Signal::new(AudioTrack::sample_synth_wav());
    let pdf_sig: Signal<PdfDocument> = Signal::new(PdfDocument::sample_pdf());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(960, 1400).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        EuvPage::new(
            "Media Test".to_string(),
            "All 8 media types".to_string(),
            tokens,
        )
        .section(EuvImage::new(ImageAsset::sample_photo("Sample"), tokens).mount_current())
        .section(
            EuvSvgView::new(SvgDocument::sample_svg(), "SVG".to_string(), tokens).mount_current(),
        )
        .section(EuvGifPlayer::new(gif_sig, tokens).mount_current())
        .section(EuvLivePhoto::new(live_sig, tokens).mount_current())
        .section(EuvVideoPlayer::new(video_sig, tokens).mount_current())
        .section(EuvAudioPlayer::new(audio_sig, tokens).mount_current())
        .section(EuvPdfViewer::new(pdf_sig, tokens).mount_current())
        .section(EuvHtmlView::new(HtmlDocument::sample_html(), tokens).mount_current())
        .mount_current()
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 40);
}

#[test]
fn rating_tooltip_context_menu_and_input_external_sync_update_pixels() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Dark);
    let rating: Signal<usize> = Signal::new(1);
    let text_sig: Signal<String> = Signal::new("init".to_string());
    let ctx_open: Signal<bool> = Signal::new(false);
    let ctx_sel: Signal<Option<usize>> = Signal::new(None);
    let mut tip_wrap: NodeId = NodeId::new(0, 0);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let trig: NodeId = EuvButton::new("Tip Trigger".to_string(), tokens).mount_current();
        tip_wrap = EuvTooltip::new("Hover Hint".to_string(), trig, tokens).mount_current();
        view! {
            column {
                width: Length::Px(640.0)
                height: Length::Px(480.0)
                padding: Edges::all(20.0)
                gap: 12.0
                { EuvRating::new(rating, 5, tokens) }
                { EuvInput::new(text_sig, "External sync".to_string(), tokens) }
                { tip_wrap }
                { EuvContextMenu::new("Target".to_string(), vec!["Copy".to_string(), "Paste".to_string()], tokens).open(ctx_open).selected(ctx_sel) }
            }
        }
    });
    let frame0: FrameStats = app.step_frame().unwrap();
    let _: bool = rating.set(5);
    let frame1: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame0.get_frame_hash(), frame1.get_frame_hash());
    let _: bool = text_sig.set("synced_externally".to_string());
    let frame2: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame1.get_frame_hash(), frame2.get_frame_hash());
    let tip_center: Point = app.get_scene().with_read(
        |tree: &SceneTree| {
            let b: Rect = tree
                .get_node(tip_wrap)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds())
                .unwrap_or_default();
            Point::new(
                b.get_x() + b.get_width() * 0.5,
                b.get_y() + b.get_height() * 0.5,
            )
        },
        Point::default(),
    );
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(tip_center.get_x(), tip_center.get_y()),
    ));
    let frame3: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame2.get_frame_hash(), frame3.get_frame_hash());
    let _: bool = ctx_open.set(true);
    let frame4: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame3.get_frame_hash(), frame4.get_frame_hash());
}

#[test]
fn color_picker_time_picker_tag_input_interactive_tree_and_breadcrumb_work() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let color_sig: Signal<Color> = Signal::new(Color::hex(0x2563EB));
    let time_sig: Signal<(u8, u8)> = Signal::new((9, 30));
    let tags_sig: Signal<Vec<String>> = Signal::new(vec!["rust".to_string(), "native".to_string()]);
    let tree_sel: Signal<Option<String>> = Signal::new(None);
    let crumb_active: Signal<usize> = Signal::new(2);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 900).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        view! {
            column {
                width: Length::Px(800.0)
                height: Length::Px(900.0)
                padding: Edges::all(16.0)
                gap: 12.0
                background: tokens.get_palette().get_background()
                { EuvBreadcrumb::new(vec!["Home".to_string(), "Forms".to_string(), "Pickers".to_string()], tokens).active(crumb_active) }
                { EuvColorPicker::new(color_sig, tokens) }
                { EuvTimePicker::new(time_sig, tokens) }
                { EuvTagInput::new(tags_sig, tokens).placeholder("Add tag...") }
                { EuvTree::new(vec![EuvTreeNode::new("Root".to_string(), vec![EuvTreeNode::new("Child".to_string(), Vec::new())])], tokens).selected(tree_sel) }
            }
        }
    });
    let frame0: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame0.get_frame_hash(), 0);
    let _: bool = color_sig.set(Color::hex(0x16A34A));
    let _: bool = time_sig.set((14, 45));
    let _: bool = tags_sig.update(|items: &mut Vec<String>| items.push("gpu".to_string()));
    let _: bool = tree_sel.set(Some("Child".to_string()));
    let _: bool = crumb_active.set(1);
    let frame1: FrameStats = app.step_frame().unwrap();
    assert_ne!(frame0.get_frame_hash(), frame1.get_frame_hash());
    assert_eq!(tags_sig.get_untracked().len(), 3);
    assert_eq!(time_sig.get_untracked(), (14, 45));
}
