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
        let nav: NodeId = view! {
            EuvSidebarNav(
                vec![
                    EuvNavItem::new("/home".to_string(), "Home".to_string()),
                    EuvNavItem::new("/render".to_string(), "Render".to_string()),
                ],
                active_route,
                tokens
            )
        };
        let content: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_LG
                EuvHeader("Dashboard".to_string(), "Unified native UI".to_string(), tokens)
                EuvCard("Controls".to_string(), tokens) {
                    subtitle: "Cross-platform UI primitives"
                    EuvBadge("v0.39".to_string(), EuvTone::Primary, tokens)
                    EuvButton("Action".to_string(), tokens) {
                        on_click: move |_: &UiEvent| {
                            let _: bool = clicks.update(|v: &mut u32| *v += 1);
                        }
                    }
                    EuvField(
                        "Username".to_string(),
                        view! { EuvInput(input_val, "Enter name".to_string(), tokens) },
                        tokens
                    )
                    EuvSwitch(switch_on, "Hardware GPU".to_string(), tokens)
                    EuvSegmented(
                        vec!["Software".to_string(), "Hardware".to_string()],
                        selected_seg,
                        tokens
                    )
                    EuvProgress(0.75, tokens)
                }
                EuvModal(
                    modal_open,
                    "Dialog".to_string(),
                    view! { text { "Modal body" } },
                    tokens
                )
            }
        };
        view! { EuvAppShell("euv-ui".to_string(), nav, content, tokens) }
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
        let left_p: NodeId = view! {
            EuvPanel("Inspector".to_string(), tokens) {
                EuvAvatar("EV".to_string(), 36.0, tokens)
                EuvIcon("*".to_string(), 18.0, tokens.get_palette().get_primary())
                EuvTag("Native".to_string(), EuvTone::Success, tokens)
                EuvTypography("Title".to_string(), EuvTypographyVariant::H2, tokens)
                EuvEmpty("No items".to_string(), "Add one".to_string(), tokens)
                EuvSkeleton(140.0, 16.0, tokens)
                EuvSpinner(24.0, tokens)
            }
        };
        let right_p: NodeId = view! {
            EuvScrollArea(240.0, tokens) {
                EuvTimeline(
                    vec![EuvTimelineItem::new(
                            "Boot".to_string(),
                            "00:01".to_string(),
                            "Metal ready".to_string(),
                    )],
                    tokens
                )
                EuvList(vec!["Item A".to_string(), "Item B".to_string()], tokens)
                EuvTree(
                    vec![EuvTreeNode::new(
                            "Workspace".to_string(),
                            vec![EuvTreeNode::new("core".to_string(), Vec::new())],
                    )],
                    tokens
                )
            }
        };
        let btn_a: NodeId = view! { EuvButton("A".to_string(), tokens) };
        let btn_b: NodeId = view! { EuvToggle(toggle_sig, "Pin".to_string(), tokens) };
        let pop_trig: NodeId = view! { EuvButton("Info".to_string(), tokens) };
        let pop_body: NodeId = view! { text { "Popover details" } };
        let tip_trig: NodeId = view! { EuvButton("Help".to_string(), tokens) };
        let drawer_body: NodeId = view! { text { "Drawer body" } };
        let sheet_body: NodeId = view! { text { "Sheet body" } };
        view! {
            EuvPageContainer(tokens) {
                EuvNavbar("euv Studio".to_string(), tokens) {
                    EuvKbd("Cmd+K".to_string(), tokens)
                }
                EuvButtonGroup(vec![btn_a, btn_b], tokens)
                EuvSplitter(35.0, left_p, right_p, tokens)
                EuvGrid(2, SPACE_MD) {
                    EuvForm(tokens) {
                        EuvLabel("Bio".to_string(), true, tokens)
                        EuvTextarea(text_sig, "Write...".to_string(), 72.0, tokens)
                        EuvNumberInput(num_sig, tokens) {
                            range(0, 20)
                            step: 1
                        }
                        EuvRadioGroup(
                            vec!["CPU".to_string(), "GPU".to_string()],
                            radio_sig,
                            tokens
                        )
                        EuvSlider(slider_sig, 0.0, 100.0, tokens)
                        EuvSelect(
                            vec!["Metal".to_string(), "Vulkan".to_string()],
                            select_sig,
                            select_open,
                            tokens
                        )
                        EuvRating(rating_sig, 5, tokens)
                    }
                    EuvSpace::column(SPACE_SM) {
                        EuvBreadcrumb(vec!["Home".to_string(), "UI".to_string()], tokens)
                        EuvPagination(page_sig, 5, tokens)
                        EuvSteps(
                            vec!["Build".to_string(), "Test".to_string()],
                            step_sig,
                            tokens
                        )
                        EuvAccordion(
                            vec![EuvAccordionItem::new(
                                    "FAQ".to_string(),
                                    "Zero WebView".to_string(),
                            )],
                            acc_sig,
                            tokens
                        )
                        EuvToc(vec!["Intro".to_string(), "API".to_string()], toc_sig, tokens)
                    }
                }
                EuvSpace::row(SPACE_SM) {
                    EuvPopover(pop_open, pop_trig, pop_body, tokens)
                    EuvTooltip("Shortcut".to_string(), tip_trig, tokens)
                    EuvDropdown(
                        "Actions".to_string(),
                        vec!["Export".to_string(), "Share".to_string()],
                        tokens
                    ) {
                        open: dd_open
                        selected: dd_sel
                    }
                }
                EuvFooter("euv 0.39 Native".to_string(), tokens)
                EuvDrawer(drawer_open, "Settings".to_string(), drawer_body, tokens)
                EuvSheet(sheet_open, "Actions".to_string(), sheet_body, tokens)
                EuvNotification(
                    notif_vis,
                    "Build OK".to_string(),
                    "7 targets verified".to_string(),
                    tokens
                ) {
                    tone: EuvTone::Success
                }
                EuvLoading(loading_sig, "Syncing...".to_string(), tokens)
            }
        }
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
                EuvButton("Increment".to_string(), tokens) {
                    node_ref: btn_id
                    on_click: move |_: &UiEvent| {
                        let _: bool = clicks.update(|v: &mut u32| *v += 1);
                    }
                }
                EuvSwitch(switch_on, "Hardware GPU".to_string(), tokens) {
                    node_ref: sw_id
                }
                EuvCheckbox(check_on, "Enable telemetry".to_string(), tokens) {
                    node_ref: chk_id
                }
                EuvToggle(toggle_on, "Pin panel".to_string(), tokens) {
                    node_ref: tog_id
                }
                EuvInput(input_val, "Type here".to_string(), tokens) {
                    node_ref: inp_id
                }
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
            let nav: NodeId = view! {
                EuvSidebarNav(
                    vec![EuvNavItem::new("/home".to_string(), "Home".to_string())],
                    route,
                    tokens
                )
            };
            let coll_body: NodeId = view! { text { "Collapsible content" } };
            let aspect_child: NodeId = view! { text { "16:9 Media" } };
            let body: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_MD
                    EuvMenubar(
                        vec!["File".to_string(), "Edit".to_string(), "View".to_string()],
                        menu_act,
                        tokens
                    )
                    EuvToggleGroup(
                        vec!["Left".to_string(), "Center".to_string(), "Right".to_string()],
                        tg_sel,
                        tokens
                    )
                    EuvInputOtp(otp, 6, tokens)
                    EuvCollapsible(coll_open, "Details".to_string(), coll_body, tokens)
                    EuvCommand(
                        vec!["Build".to_string(), "Deploy".to_string()],
                        cmd_sel,
                        tokens
                    ) {
                        query: cmd_q
                        placeholder: "Search..."
                    }
                    EuvHoverCard("@shadcn".to_string(), "shadcn/ui".to_string(), tokens) {
                        open: hover_open
                        description: "Design system parity."
                    }
                    EuvContextMenu(
                        "Canvas Surface".to_string(),
                        vec!["Inspect".to_string(), "Duplicate".to_string()],
                        tokens
                    ) {
                        open: ctx_open
                        selected: ctx_sel
                    }
                    EuvCalendar("November 2026".to_string(), 30, cal_day, tokens)
                    EuvAspectRatio(16.0 / 9.0, aspect_child, tokens)
                    EuvCarousel(
                        vec!["Slide A".to_string(), "Slide B".to_string()],
                        car_idx,
                        tokens
                    )
                    EuvAlertDialog(alert_open, "Confirm".to_string(), alert_confirmed, tokens) {
                        description: "Proceed?"
                        labels("Cancel", "Continue")
                    }
                }
            };
            view! { EuvAppShell("shadcn-euv".to_string(), nav, body, tokens) }
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
        view! {
            column {
                width: Length::Px(640.0)
                height: Length::Px(480.0)
                padding: Edges::all(24.0)
                gap: 16.0
                EuvButton("Hover Target".to_string(), tokens) {
                    node_ref: btn_node
                }
                EuvHoverCard("@hover".to_string(), "Preview".to_string(), tokens) {
                    open: hover_open
                    description: "Triggered on hover"
                }
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
        view! {
            EuvPage("Media Test".to_string(), "All 8 media types".to_string(), tokens) {
                EuvImage(ImageAsset::sample_photo("Sample"), tokens)
                EuvSvgView(SvgDocument::sample_svg(), "SVG".to_string(), tokens)
                EuvGifPlayer(gif_sig, tokens)
                EuvLivePhoto(live_sig, tokens)
                EuvVideoPlayer(video_sig, tokens)
                EuvAudioPlayer(audio_sig, tokens)
                EuvPdfViewer(pdf_sig, tokens)
                EuvHtmlView(HtmlDocument::sample_html(), tokens)
            }
        }
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
        let trig: NodeId = view! { EuvButton("Tip Trigger".to_string(), tokens) };
        view! {
            column {
                width: Length::Px(640.0)
                height: Length::Px(480.0)
                padding: Edges::all(20.0)
                gap: 12.0
                EuvRating(rating, 5, tokens)
                EuvInput(text_sig, "External sync".to_string(), tokens)
                EuvTooltip("Hover Hint".to_string(), trig, tokens) {
                    node_ref: tip_wrap
                }
                EuvContextMenu(
                    "Target".to_string(),
                    vec!["Copy".to_string(), "Paste".to_string()],
                    tokens
                ) {
                    open: ctx_open
                    selected: ctx_sel
                }
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
                EuvBreadcrumb(
                    vec!["Home".to_string(), "Forms".to_string(), "Pickers".to_string()],
                    tokens
                ) {
                    active: crumb_active
                }
                EuvColorPicker(color_sig, tokens)
                EuvTimePicker(time_sig, tokens)
                EuvTagInput(tags_sig, tokens) {
                    placeholder: "Add tag..."
                }
                EuvTree(
                    vec![EuvTreeNode::new(
                            "Root".to_string(),
                            vec![EuvTreeNode::new("Child".to_string(), Vec::new())],
                    )],
                    tokens
                ) {
                    selected: tree_sel
                }
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

#[test]
fn buttons_center_without_overflow_and_dropdowns_match_trigger_width() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let dd_open: Signal<bool> = Signal::new(true);
    let dd_sel: Signal<Option<usize>> = Signal::new(None);
    let sel_open: Signal<bool> = Signal::new(true);
    let sel_idx: Signal<usize> = Signal::new(0);
    let ctx_open: Signal<bool> = Signal::new(true);
    let ctx_sel: Signal<Option<usize>> = Signal::new(None);
    let date_open: Signal<bool> = Signal::new(true);
    let date_day: Signal<u32> = Signal::new(18);
    let mut grid_btn: NodeId = NodeId::default();
    let mut dd_node: NodeId = NodeId::default();
    let mut sel_node: NodeId = NodeId::default();
    let mut ctx_node: NodeId = NodeId::default();
    let mut date_node: NodeId = NodeId::default();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 844).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let b1: NodeId = EuvButton::new("Shadcn".to_string(), tokens)
            .outline()
            .sm()
            .mount();
        let b2: NodeId = EuvButton::new("Linear".to_string(), tokens)
            .outline()
            .sm()
            .mount();
        grid_btn = b1;
        let dd: NodeId = EuvDropdown::new(
            "Actions".to_string(),
            vec!["Duplicate".to_string(), "Archive".to_string()],
            tokens,
        )
        .open(dd_open)
        .selected(dd_sel)
        .mount();
        dd_node = dd;
        let sel: NodeId = EuvSelect::new(
            vec!["Software".to_string(), "Hardware".to_string()],
            sel_idx,
            sel_open,
            tokens,
        )
        .mount();
        sel_node = sel;
        let cm: NodeId = EuvContextMenu::new(
            "Target Surface".to_string(),
            vec!["Inspect".to_string(), "Copy".to_string()],
            tokens,
        )
        .open(ctx_open)
        .selected(ctx_sel)
        .mount();
        ctx_node = cm;
        let dp: NodeId =
            EuvDatePicker::new("Nov 2026".to_string(), date_day, date_open, tokens).mount();
        date_node = dp;
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(SPACE_MD)
                gap: SPACE_MD
                grid {
                    columns: 2
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    { b1 }
                    { b2 }
                }
                { dd }
                { sel }
                { cm }
                { dp }
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    let (btn_b, txt_b): (Rect, Rect) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let bn: &SceneNode = tree.get_node(grid_btn).unwrap();
            let tid: NodeId = bn.get_children()[0];
            let tn: &SceneNode = tree.get_node(tid).unwrap();
            (
                bn.get_layout_box().get_bounds(),
                tn.get_layout_box().get_bounds(),
            )
        },
        (Rect::default(), Rect::default()),
    );
    assert!(txt_b.get_x() >= btn_b.get_x() && txt_b.right() <= btn_b.right() + 1.0);
    assert!(txt_b.get_y() >= btn_b.get_y() && txt_b.bottom() <= btn_b.bottom() + 1.0);
    let btn_cx: f32 = btn_b.get_x() + btn_b.get_width() * 0.5;
    let txt_cx: f32 = txt_b.get_x() + txt_b.get_width() * 0.5;
    let btn_cy: f32 = btn_b.get_y() + btn_b.get_height() * 0.5;
    let txt_cy: f32 = txt_b.get_y() + txt_b.get_height() * 0.5;
    assert!(
        (btn_cx - txt_cx).abs() < 1.5 && (btn_cy - txt_cy).abs() < 1.5,
        "button text must be centered horizontally & vertically: btn={btn_b:?} txt={txt_b:?}"
    );
    let wrappers: [(&str, NodeId); 4] = [
        ("EuvDropdown", dd_node),
        ("EuvSelect", sel_node),
        ("EuvContextMenu", ctx_node),
        ("EuvDatePicker", date_node),
    ];
    for (name, wid_node) in wrappers.into_iter() {
        let (trig_w, menu_w): (f32, f32) = app.get_scene().with_read(
            |tree: &SceneTree| {
                let wnode: &SceneNode = tree.get_node(wid_node).unwrap();
                let ch: &[NodeId] = wnode.get_children();
                let tw: f32 = tree
                    .get_node(ch[0])
                    .unwrap()
                    .get_layout_box()
                    .get_bounds()
                    .get_width();
                let mw: f32 = tree
                    .get_node(ch[1])
                    .unwrap()
                    .get_layout_box()
                    .get_bounds()
                    .get_width();
                (tw, mw)
            },
            (0.0, 0.0),
        );
        assert!(
            trig_w > 100.0 && (trig_w - menu_w).abs() <= 1.0,
            "{name} dropdown menu width ({menu_w}) must align 1:1 with trigger width ({trig_w})"
        );
    }
}

#[test]
fn router_lazy_mounting_freeze_destroy_alive_and_double_back_exit_work() {
    reset_reactive_runtime();
    let home_builds: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let freeze_builds: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let destroy_builds: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let alive_builds: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let freeze_effect_runs: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let alive_effect_runs: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let shared_counter: Signal<u32> = Signal::new(1);
    let modal_open: Signal<bool> = Signal::new(false);
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let hb: Rc<Cell<u32>> = home_builds.clone();
    let fb: Rc<Cell<u32>> = freeze_builds.clone();
    let db: Rc<Cell<u32>> = destroy_builds.clone();
    let ab: Rc<Cell<u32>> = alive_builds.clone();
    let fe: Rc<Cell<u32>> = freeze_effect_runs.clone();
    let ae: Rc<Cell<u32>> = alive_effect_runs.clone();
    let router: EuvRouter = EuvRouter::new("/")
        .route_with_retention(
            "/",
            "Home",
            PageRetention::Freeze,
            move |_: &EuvRouteMatch| {
                hb.set(hb.get() + 1);
                view! {
                    column {
                        width: Length::Percent(100.0)
                        text { "Home Root" }
                    }
                }
            },
        )
        .route_with_retention(
            "/freeze",
            "Freeze",
            PageRetention::Freeze,
            move |_: &EuvRouteMatch| {
                fb.set(fb.get() + 1);
                let fe_in: Rc<Cell<u32>> = fe.clone();
                let _: Effect = Effect::new(move || {
                    let _: u32 = shared_counter.get();
                    fe_in.set(fe_in.get() + 1);
                });
                view! { text { "Frozen Page" } }
            },
        )
        .route_with_retention(
            "/destroy",
            "Destroy",
            PageRetention::Destroy,
            move |_: &EuvRouteMatch| {
                db.set(db.get() + 1);
                view! { text { "Destroyed Page" } }
            },
        )
        .route_with_retention(
            "/alive",
            "Alive",
            PageRetention::Alive,
            move |_: &EuvRouteMatch| {
                ab.set(ab.get() + 1);
                let ae_in: Rc<Cell<u32>> = ae.clone();
                let _: Effect = Effect::new(move || {
                    let _: u32 = shared_counter.get();
                    ae_in.set(ae_in.get() + 1);
                });
                view! { text { "Alive Page" } }
            },
        );
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(420, 800).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let router_m: EuvRouter = router.clone();
    let _: NodeId = app.mount(move || {
        let outlet: NodeId = router_m
            .outlet(|_: &EuvRouteMatch| view! { text { "Fallback" } })
            .with_double_back_exit(true)
            .mount();
        let modal: NodeId = view! {
            EuvModal(modal_open, "Overlay".to_string(), view! { text { "Body" } }, tokens)
        };
        view! {
            column {
                width: Length::Percent(100.0)
                { outlet }
                { modal }
            }
        }
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(home_builds.get(), 1);
    assert_eq!(freeze_builds.get(), 0);
    assert_eq!(destroy_builds.get(), 0);
    assert_eq!(alive_builds.get(), 0);
    router.push("/freeze");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(freeze_builds.get(), 1);
    assert_eq!(freeze_effect_runs.get(), 1);
    router.push("/alive");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(alive_builds.get(), 1);
    assert_eq!(alive_effect_runs.get(), 1);
    router.push("/destroy");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(destroy_builds.get(), 1);
    for v in 2..=12 {
        let _: bool = shared_counter.set(v);
    }
    assert_eq!(freeze_effect_runs.get(), 1);
    assert!(alive_effect_runs.get() >= 10);
    router.push("/freeze");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(freeze_builds.get(), 1);
    assert_eq!(freeze_effect_runs.get(), 2);
    router.push("/destroy");
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_eq!(destroy_builds.get(), 2);
    let _: bool = modal_open.set(true);
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    assert!(!modal_open.get_untracked());
    assert_eq!(router.get_current_path().get_untracked(), "/destroy");
    assert!(app.get_running());
    while router.get_can_go_back().get_untracked() {
        app.dispatch_window_event(WindowEvent::KeyDown(
            wid,
            KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
        ));
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(router.get_current_path().get_untracked(), "/");
    assert!(app.get_running());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert!(app.get_running());
    assert!(!app.get_context().get_exit_requested().get_untracked());
    let prompt_visible: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Text(t) if t == DOUBLE_BACK_EXIT_PROMPT)
                    && n.get_layout_box().get_bounds().get_width() > 0.0
                    && n.get_layout_box().get_bounds().get_height() > 0.0
            })
        },
        false,
    );
    assert!(prompt_visible);
    for _ in 0..50 {
        let _: FrameStats = app.step_game_frame(0.05).unwrap();
    }
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    let _: FrameStats = app.step_game_frame(0.05).unwrap();
    assert!(app.get_running());
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    assert!(app.get_context().get_exit_requested().get_untracked());
    assert!(!app.get_running());
}

#[test]
fn eva_code_block_full_markdown_suite_and_fullscreen_swipe_gesture_work() {
    reset_reactive_runtime();
    let light_eva: CodeThemePalette = CodeThemePalette::resolve(ThemeMode::Light);
    let dark_eva: CodeThemePalette = CodeThemePalette::resolve(ThemeMode::Dark);
    assert_eq!(light_eva.get_background(), Color::hex(0xEBEEF5));
    assert_eq!(light_eva.get_keyword(), Color::hex(0x7C4DFF));
    assert_eq!(light_eva.get_function(), Color::hex(0x437AED));
    assert_eq!(dark_eva.get_background(), Color::hex(0x282C34));
    assert_eq!(dark_eva.get_keyword(), Color::hex(0xA78CFA));
    assert_eq!(dark_eva.get_function(), Color::hex(0x6495EE));
    let tokens_dark: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Dark);
    let copied: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let copied_c: Rc<Cell<usize>> = copied.clone();
    let route: Signal<String> = Signal::new("/".to_string());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 800).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        let nav: NodeId = view! {
            EuvSidebarNav(
                vec![EuvNavItem::new("/".to_string(), "Home".to_string())],
                route,
                tokens_dark
            )
        };
        let md_src: String = "# Eva Studio\n\n> [!TIP]\n> Exact VS Code Eva Dark Bold.\n\n- [x] Ready\n- [ ] Pending\n\n| Key | Value |\n| :--- | ---: |\n| Theme | Eva Dark |\n\n```rust\nfn main() { let x: i32 = 42; }\n```\n\n<details>\n<summary>More</summary>\nDetails body\n</details>\n".to_string();
        let body: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                EuvCodeBlock("pub fn hello() -> i32 { 42 }".to_string(), tokens_dark) {
                    language: "rust"
                    filename: "lib.rs"
                    line_numbers: true
                    on_copy: move |txt: &str| {
                        copied_c.set(txt.len());
                    }
                }
                EuvMarkdown(md_src, tokens_dark)
            }
        };
        view! { EuvAppShell("Eva Docs".to_string(), nav, body, tokens_dark) }
    });
    let wid: WindowId = app.get_window_state().get_id();
    let f0: FrameStats = app.step_frame().unwrap();
    assert_ne!(f0.get_frame_hash(), 0);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(1, 220.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(1, 330.0, 404.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(1, 330.0, 404.0),
    ));
    let f1: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_ne!(f1.get_frame_hash(), f0.get_frame_hash());
    assert!(EuvOverlayStack::active_count() >= 1);
    assert!(EuvOverlayStack::dismiss_top());
}
