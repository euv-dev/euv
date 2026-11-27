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
            EuvSidebarNav {
                items: vec![
                    EuvNavItem::new("/home".to_string(), "Home".to_string()),
                    EuvNavItem::new("/render".to_string(), "Render".to_string()),
                ]
                active_route: active_route
                tokens: tokens
            }
        };
        let content: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_LG
                EuvHeader {
                    title: "Dashboard".to_string()
                    subtitle: "Unified native UI".to_string()
                    tokens: tokens
                }
                EuvCard {
                    title: "Controls".to_string()
                    tokens: tokens
                    subtitle: "Cross-platform UI primitives"
                    EuvBadge {
                        label: "v0.39".to_string()
                        tone: EuvTone::Primary
                        tokens: tokens
                    }
                    EuvButton {
                        label: "Action".to_string()
                        tokens: tokens
                        on_click: move |_: &UiEvent| {
                            let _: bool = clicks.update(|v: &mut u32| *v += 1);
                        }
                    }
                    EuvField {
                        label: "Username".to_string()
                        control: view! { EuvInput {
                            value: input_val
                            placeholder: "Enter name".to_string()
                            tokens: tokens
                        } }
                        tokens: tokens
                    }
                    EuvSwitch {
                        checked: switch_on
                        label: "Hardware GPU".to_string()
                        tokens: tokens
                    }
                    EuvSegmented {
                        options: vec!["Software".to_string(), "Hardware".to_string()]
                        selected: selected_seg
                        tokens: tokens
                    }
                    EuvProgress {
                        value: 0.75
                        tokens: tokens
                    }
                }
                EuvModal {
                    open: modal_open
                    title: "Dialog".to_string()
                    body: view! { text { "Modal body" } }
                    tokens: tokens
                }
            }
        };
        view! {
            EuvAppShell {
                brand: "euv-ui".to_string()
                sidebar: nav
                main_content: content
                tokens: tokens
            }
        }
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
            EuvPanel {
                title: "Inspector".to_string()
                tokens: tokens
                EuvAvatar {
                    initials: "EV".to_string()
                    size: 36.0
                    tokens: tokens
                }
                EuvIcon {
                    symbol: "*".to_string()
                    size: 18.0
                    color: tokens.get_palette().get_primary()
                }
                EuvTag {
                    label: "Native".to_string()
                    tone: EuvTone::Success
                    tokens: tokens
                }
                EuvTypography {
                    text: "Title".to_string()
                    variant: EuvTypographyVariant::H2
                    tokens: tokens
                }
                EuvEmpty {
                    title: "No items".to_string()
                    description: "Add one".to_string()
                    tokens: tokens
                }
                EuvSkeleton {
                    width: 140.0
                    height: 16.0
                    tokens: tokens
                }
                EuvSpinner {
                    size: 24.0
                    tokens: tokens
                }
            }
        };
        let right_p: NodeId = view! {
            EuvScrollArea {
                height: 240.0
                tokens: tokens
                EuvTimeline {
                    items: vec![EuvTimelineItem::new(
                        "Boot".to_string(),
                        "00:01".to_string(),
                        "Metal ready".to_string(),
                    )]
                    tokens: tokens
                }
                EuvList {
                    items: vec!["Item A".to_string(), "Item B".to_string()]
                    tokens: tokens
                }
                EuvTree {
                    roots: vec![EuvTreeNode::new(
                        "Workspace".to_string(),
                        vec![EuvTreeNode::new("core".to_string(), Vec::new())],
                    )]
                    tokens: tokens
                }
            }
        };
        let btn_a: NodeId = view! {
            EuvButton {
                label: "A".to_string()
                tokens: tokens
            }
        };
        let btn_b: NodeId = view! {
            EuvToggle {
                pressed: toggle_sig
                label: "Pin".to_string()
                tokens: tokens
            }
        };
        let pop_trig: NodeId = view! {
            EuvButton {
                label: "Info".to_string()
                tokens: tokens
            }
        };
        let pop_body: NodeId = view! { text { "Popover details" } };
        let tip_trig: NodeId = view! {
            EuvButton {
                label: "Help".to_string()
                tokens: tokens
            }
        };
        let drawer_body: NodeId = view! { text { "Drawer body" } };
        let sheet_body: NodeId = view! { text { "Sheet body" } };
        view! {
            EuvPageContainer {
                tokens: tokens
                EuvNavbar {
                    brand: "euv Studio".to_string()
                    tokens: tokens
                    EuvKbd {
                        key: "Cmd+K".to_string()
                        tokens: tokens
                    }
                }
                EuvButtonGroup {
                    children: vec![btn_a, btn_b]
                    tokens: tokens
                }
                EuvSplitter {
                    ratio_percent: 35.0
                    primary: left_p
                    secondary: right_p
                    tokens: tokens
                }
                EuvGrid {
                    columns: 2
                    gap: SPACE_MD
                    EuvForm {
                        tokens: tokens
                        EuvLabel {
                            text: "Bio".to_string()
                            required: true
                            tokens: tokens
                        }
                        EuvTextarea {
                            value: text_sig
                            placeholder: "Write...".to_string()
                            height: 72.0
                            tokens: tokens
                        }
                        EuvNumberInput {
                            value: num_sig
                            tokens: tokens
                            range(0, 20)
                            step: 1
                        }
                        EuvRadioGroup {
                            options: vec!["CPU".to_string(), "GPU".to_string()]
                            selected: radio_sig
                            tokens: tokens
                        }
                        EuvSlider {
                            value: slider_sig
                            min: 0.0
                            max: 100.0
                            tokens: tokens
                        }
                        EuvSelect {
                            options: vec!["Metal".to_string(), "Vulkan".to_string()]
                            selected: select_sig
                            open: select_open
                            tokens: tokens
                        }
                        EuvRating {
                            value: rating_sig
                            max_stars: 5
                            tokens: tokens
                        }
                    }
                    EuvSpace::column(SPACE_SM) {
                        EuvBreadcrumb {
                            segments: vec!["Home".to_string(), "UI".to_string()]
                            tokens: tokens
                        }
                        EuvPagination {
                            current: page_sig
                            total_pages: 5
                            tokens: tokens
                        }
                        EuvSteps {
                            labels: vec!["Build".to_string(), "Test".to_string()]
                            active: step_sig
                            tokens: tokens
                        }
                        EuvAccordion {
                            items: vec![EuvAccordionItem::new(
                                "FAQ".to_string(),
                                "Zero WebView".to_string(),
                            )]
                            open_index: acc_sig
                            tokens: tokens
                        }
                        EuvToc {
                            headings: vec!["Intro".to_string(), "API".to_string()]
                            active: toc_sig
                            tokens: tokens
                        }
                    }
                }
                EuvSpace::row(SPACE_SM) {
                    EuvPopover {
                        open: pop_open
                        trigger: pop_trig
                        content: pop_body
                        tokens: tokens
                    }
                    EuvTooltip {
                        text: "Shortcut".to_string()
                        trigger: tip_trig
                        tokens: tokens
                    }
                    EuvDropdown {
                        label: "Actions".to_string()
                        items: vec!["Export".to_string(), "Share".to_string()]
                        tokens: tokens
                        open: dd_open
                        selected: dd_sel
                    }
                }
                EuvFooter {
                    text: "euv 0.39 Native".to_string()
                    tokens: tokens
                }
                EuvDrawer {
                    open: drawer_open
                    title: "Settings".to_string()
                    body: drawer_body
                    tokens: tokens
                }
                EuvSheet {
                    open: sheet_open
                    title: "Actions".to_string()
                    body: sheet_body
                    tokens: tokens
                }
                EuvNotification {
                    visible: notif_vis
                    title: "Build OK".to_string()
                    message: "7 targets verified".to_string()
                    tokens: tokens
                    tone: EuvTone::Success
                }
                EuvLoading {
                    active: loading_sig
                    label: "Syncing...".to_string()
                    tokens: tokens
                }
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
                EuvButton {
                    label: "Increment".to_string()
                    tokens: tokens
                    node_ref: btn_id
                    on_click: move |_: &UiEvent| {
                        let _: bool = clicks.update(|v: &mut u32| *v += 1);
                    }
                }
                EuvSwitch {
                    checked: switch_on
                    label: "Hardware GPU".to_string()
                    tokens: tokens
                    node_ref: sw_id
                }
                EuvCheckbox {
                    checked: check_on
                    label: "Enable telemetry".to_string()
                    tokens: tokens
                    node_ref: chk_id
                }
                EuvToggle {
                    pressed: toggle_on
                    label: "Pin panel".to_string()
                    tokens: tokens
                    node_ref: tog_id
                }
                EuvInput {
                    value: input_val
                    placeholder: "Type here".to_string()
                    tokens: tokens
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
                EuvSidebarNav {
                    items: vec![EuvNavItem::new("/home".to_string(), "Home".to_string())]
                    active_route: route
                    tokens: tokens
                }
            };
            let coll_body: NodeId = view! { text { "Collapsible content" } };
            let aspect_child: NodeId = view! { text { "16:9 Media" } };
            let body: NodeId = view! {
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_MD
                    EuvMenubar {
                        menus: vec!["File".to_string(), "Edit".to_string(), "View".to_string()]
                        active_menu: menu_act
                        tokens: tokens
                    }
                    EuvToggleGroup {
                        options: vec!["Left".to_string(), "Center".to_string(), "Right".to_string()]
                        selected: tg_sel
                        tokens: tokens
                    }
                    EuvInputOtp {
                        value: otp
                        length: 6
                        tokens: tokens
                    }
                    EuvCollapsible {
                        open: coll_open
                        title: "Details".to_string()
                        content: coll_body
                        tokens: tokens
                    }
                    EuvCommand {
                        items: vec!["Build".to_string(), "Deploy".to_string()]
                        selected: cmd_sel
                        tokens: tokens
                        query: cmd_q
                        placeholder: "Search..."
                    }
                    EuvHoverCard {
                        trigger_label: "@shadcn".to_string()
                        title: "shadcn/ui".to_string()
                        tokens: tokens
                        open: hover_open
                        description: "Design system parity."
                    }
                    EuvContextMenu {
                        trigger_label: "Canvas Surface".to_string()
                        items: vec!["Inspect".to_string(), "Duplicate".to_string()]
                        tokens: tokens
                        open: ctx_open
                        selected: ctx_sel
                    }
                    EuvCalendar {
                        month_label: "November 2026".to_string()
                        days_in_month: 30
                        selected_day: cal_day
                        tokens: tokens
                    }
                    EuvAspectRatio {
                        ratio: 16.0 / 9.0
                        child: aspect_child
                        tokens: tokens
                    }
                    EuvCarousel {
                        slides: vec!["Slide A".to_string(), "Slide B".to_string()]
                        active_slide: car_idx
                        tokens: tokens
                    }
                    EuvAlertDialog {
                        open: alert_open
                        title: "Confirm".to_string()
                        confirmed: alert_confirmed
                        tokens: tokens
                        description: "Proceed?"
                        labels("Cancel", "Continue")
                    }
                }
            };
            view! {
                EuvAppShell {
                    brand: "shadcn-euv".to_string()
                    sidebar: nav
                    main_content: body
                    tokens: tokens
                }
            }
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
                EuvButton {
                    label: "Hover Target".to_string()
                    tokens: tokens
                    node_ref: btn_node
                }
                EuvHoverCard {
                    trigger_label: "@hover".to_string()
                    title: "Preview".to_string()
                    tokens: tokens
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
            EuvPage {
                title: "Media Test".to_string()
                subtitle: "All 8 media types".to_string()
                tokens: tokens
                EuvImage {
                    asset: ImageAsset::sample_photo("Sample")
                    tokens: tokens
                }
                EuvSvgView {
                    document: SvgDocument::sample_svg()
                    label: "SVG".to_string()
                    tokens: tokens
                }
                EuvGifPlayer {
                    state: gif_sig
                    tokens: tokens
                }
                EuvLivePhoto {
                    state: live_sig
                    tokens: tokens
                }
                EuvVideoPlayer {
                    state: video_sig
                    tokens: tokens
                }
                EuvAudioPlayer {
                    state: audio_sig
                    tokens: tokens
                }
                EuvPdfViewer {
                    state: pdf_sig
                    tokens: tokens
                }
                EuvHtmlView {
                    document: HtmlDocument::sample_html()
                    tokens: tokens
                }
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
        let trig: NodeId = view! {
            EuvButton {
                label: "Tip Trigger".to_string()
                tokens: tokens
            }
        };
        view! {
            column {
                width: Length::Px(640.0)
                height: Length::Px(480.0)
                padding: Edges::all(20.0)
                gap: 12.0
                EuvRating {
                    value: rating
                    max_stars: 5
                    tokens: tokens
                }
                EuvInput {
                    value: text_sig
                    placeholder: "External sync".to_string()
                    tokens: tokens
                }
                EuvTooltip {
                    text: "Hover Hint".to_string()
                    trigger: trig
                    tokens: tokens
                    node_ref: tip_wrap
                }
                EuvContextMenu {
                    trigger_label: "Target".to_string()
                    items: vec!["Copy".to_string(), "Paste".to_string()]
                    tokens: tokens
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
                EuvBreadcrumb {
                    segments: vec!["Home".to_string(), "Forms".to_string(), "Pickers".to_string()]
                    tokens: tokens
                    active: crumb_active
                }
                EuvColorPicker {
                    value: color_sig
                    tokens: tokens
                }
                EuvTimePicker {
                    value: time_sig
                    tokens: tokens
                }
                EuvTagInput {
                    tags: tags_sig
                    tokens: tokens
                    placeholder: "Add tag..."
                }
                EuvTree {
                    roots: vec![EuvTreeNode::new(
                        "Root".to_string(),
                        vec![EuvTreeNode::new("Child".to_string(), Vec::new())],
                    )]
                    tokens: tokens
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
            EuvModal {
                open: modal_open
                title: "Overlay".to_string()
                body: view! { text { "Body" } }
                tokens: tokens
            }
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
            EuvSidebarNav {
                items: vec![EuvNavItem::new("/".to_string(), "Home".to_string())]
                active_route: route
                tokens: tokens_dark
            }
        };
        let md_src: String = "# Eva Studio\n\n> [!TIP]\n> Exact VS Code Eva Dark Bold.\n\n- [x] Ready\n- [ ] Pending\n\n| Key | Value |\n| :--- | ---: |\n| Theme | Eva Dark |\n\n```rust\nfn main() { let x: i32 = 42; }\n```\n\n<details>\n<summary>More</summary>\nDetails body\n</details>\n".to_string();
        let body: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                EuvCodeBlock {
                    code: "pub fn hello() -> i32 { 42 }".to_string()
                    tokens: tokens_dark
                    language: "rust"
                    filename: "lib.rs"
                    line_numbers: true
                    on_copy: move |txt: &str| {
                        copied_c.set(txt.len());
                    }
                }
                EuvMarkdown {
                    source: md_src
                    tokens: tokens_dark
                }
            }
        };
        view! {
            EuvAppShell {
                brand: "Eva Docs".to_string()
                sidebar: nav
                main_content: body
                tokens: tokens_dark
            }
        }
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

#[test]
fn floating_bottom_nav_is_centered_with_side_margins_and_bottom_lift() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let route: Signal<String> = Signal::new("/".to_string());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(1280, 800).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        let nav: NodeId = view! {
            EuvSidebarNav {
                items: vec![
                    EuvNavItem::new("/".to_string(), "Home".to_string()),
                    EuvNavItem::new("/markdown".to_string(), "Docs".to_string()),
                ]
                active_route: route
                tokens: tokens
            }
        };
        let bottom: NodeId = view! {
            EuvBottomNav {
                items: vec![
                    EuvNavItem::new("/".to_string(), "Home".to_string()),
                    EuvNavItem::new("/markdown".to_string(), "Docs".to_string()),
                    EuvNavItem::new("/lab".to_string(), "Cases".to_string()),
                    EuvNavItem::new("/stress".to_string(), "Stress".to_string()),
                    EuvNavItem::new("/game3d".to_string(), "3D RT".to_string()),
                ]
                active_route: route
                tokens: tokens
            }
        };
        let body: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                text { "Floating Dock Body" }
            }
        };
        view! {
            EuvAppShell {
                brand: "Floating Dock".to_string()
                sidebar: nav
                main_content: body
                tokens: tokens
                route: route
                bottom_nav: bottom
            }
        }
    });
    let wid: WindowId = app.get_window_state().get_id();
    for (vp_w, vp_h) in [(1280_u32, 800_u32), (390_u32, 844_u32)].into_iter() {
        app.dispatch_window_event(WindowEvent::Resized(wid, vp_w, vp_h, 1.0));
        let _: FrameStats = app.step_frame().unwrap();
        let dock_bounds: Option<Rect> = app.get_scene().with_read(
            |tree: &SceneTree| {
                tree.get_nodes().iter().find_map(|n: &SceneNode| {
                    let lay: LayoutStyle = n.get_style().get_layout();
                    let b: Rect = n.get_layout_box().get_bounds();
                    if n.get_alive()
                        && lay.get_position() == Position::Fixed
                        && lay.get_z_index() == Z_INDEX_CHROME
                        && (b.get_height() - 60.0).abs() < 1.0
                    {
                        Some(b)
                    } else {
                        None
                    }
                })
            },
            None,
        );
        let dock: Rect = dock_bounds.expect("floating bottom nav dock must exist");
        let w_f: f32 = vp_w as f32;
        let h_f: f32 = vp_h as f32;
        assert!(dock.get_width() < w_f - 32.0);
        assert!(dock.get_x() >= 18.0 && w_f - dock.right() >= 18.0);
        assert!(((dock.get_x() + dock.get_width() * 0.5) - w_f * 0.5).abs() <= 1.5);
        assert!(h_f - dock.bottom() >= 12.0);
    }
}

#[test]
fn coupled_route_transition_moves_outgoing_and_incoming_edge_to_edge_1_to_1() {
    reset_reactive_runtime();
    let theme: UiThemeState = UiThemeState::new(DesignScheme::Shadcn, ThemeMode::Light);
    let tokens: ThemeTokens = untracked(|| theme.get_tokens().get());
    let router: EuvRouter = EuvRouter::new("/").with_default_retention(PageRetention::Freeze);
    let router_c: EuvRouter = router.clone();
    let theme_c: UiThemeState = theme.clone();
    let order: Vec<String> = vec!["/".to_string(), "/docs".to_string(), "/lab".to_string()];
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 600).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let outlet_id: NodeId = app.mount(move || {
        router_c
            .outlet(move |m: &EuvRouteMatch| {
                let label: String = format!("Page {}", m.get_path());
                view! {
                    column {
                        width: Length::Percent(100.0)
                        height: Length::Px(900.0)
                        text {
                            font_size: 16.0
                            color: tokens.get_palette().get_foreground()
                            { label }
                        }
                    }
                }
            })
            .with_theme(theme_c)
            .with_route_order(order)
            .mount()
    });
    let _: FrameStats = app.step_frame().unwrap();
    let outlet_w: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(outlet_id)
                .map(|n: &SceneNode| n.get_layout_box().get_bounds().get_width())
                .unwrap_or(0.0)
        },
        0.0,
    );
    assert!((outlet_w - 800.0).abs() <= 1.0);
    router.push("/docs");
    for _ in 0..8 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let mid_fwd: Vec<(f32, f32)> = app.get_scene().with_read(
        |tree: &SceneTree| {
            let children: Vec<NodeId> = tree
                .get_node(outlet_id)
                .map(|n: &SceneNode| n.get_children().clone())
                .unwrap_or_default();
            children
                .into_iter()
                .filter_map(|cid: NodeId| {
                    let node: &SceneNode = tree.get_node(cid)?;
                    if !node.get_alive()
                        || node.get_style().get_layout().get_display() == Display::None
                    {
                        return None;
                    }
                    let paint: PaintStyle = node.get_style().get_paint();
                    Some((paint.get_transform().get_translate_x(), paint.get_opacity()))
                })
                .collect()
        },
        Vec::new(),
    );
    assert_eq!(mid_fwd.len(), 2);
    let mut xs_fwd: Vec<f32> = mid_fwd.iter().map(|p: &(f32, f32)| p.0).collect();
    xs_fwd.sort_by(|a: &f32, b: &f32| a.total_cmp(b));
    assert!(xs_fwd[0] < -10.0);
    assert!(xs_fwd[1] > 10.0);
    assert!(((xs_fwd[1] - xs_fwd[0]) - outlet_w).abs() <= 2.0);
    assert!(
        mid_fwd
            .iter()
            .all(|p: &(f32, f32)| (p.1 - 1.0).abs() < 0.01)
    );
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    router.push("/");
    for _ in 0..8 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let mid_back: Vec<f32> = app.get_scene().with_read(
        |tree: &SceneTree| {
            let children: Vec<NodeId> = tree
                .get_node(outlet_id)
                .map(|n: &SceneNode| n.get_children().clone())
                .unwrap_or_default();
            children
                .into_iter()
                .filter_map(|cid: NodeId| {
                    let node: &SceneNode = tree.get_node(cid)?;
                    if !node.get_alive()
                        || node.get_style().get_layout().get_display() == Display::None
                    {
                        return None;
                    }
                    Some(
                        node.get_style()
                            .get_paint()
                            .get_transform()
                            .get_translate_x(),
                    )
                })
                .collect()
        },
        Vec::new(),
    );
    assert_eq!(mid_back.len(), 2);
    let mut xs_back: Vec<f32> = mid_back;
    xs_back.sort_by(|a: &f32, b: &f32| a.total_cmp(b));
    assert!(xs_back[0] < -10.0);
    assert!(xs_back[1] > 10.0);
    assert!(((xs_back[1] - xs_back[0]) - outlet_w).abs() <= 2.0);
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let _: bool = app.get_context().get_swipe_drag_offset().set(-160.0);
    let _: FrameStats = app.step_frame().unwrap();
    let drag_xs: Vec<f32> = app.get_scene().with_read(
        |tree: &SceneTree| {
            let children: Vec<NodeId> = tree
                .get_node(outlet_id)
                .map(|n: &SceneNode| n.get_children().clone())
                .unwrap_or_default();
            children
                .into_iter()
                .filter_map(|cid: NodeId| {
                    let node: &SceneNode = tree.get_node(cid)?;
                    if !node.get_alive()
                        || node.get_style().get_layout().get_display() == Display::None
                    {
                        return None;
                    }
                    Some(
                        node.get_style()
                            .get_paint()
                            .get_transform()
                            .get_translate_x(),
                    )
                })
                .collect()
        },
        Vec::new(),
    );
    assert_eq!(drag_xs.len(), 2);
    let mut sorted_drag: Vec<f32> = drag_xs;
    sorted_drag.sort_by(|a: &f32, b: &f32| a.total_cmp(b));
    assert!((sorted_drag[0] - (-160.0)).abs() <= 1.0);
    assert!((sorted_drag[1] - (outlet_w - 160.0)).abs() <= 1.0);
}

#[test]
fn router_preserves_per_page_scroll_on_freeze_and_theme_toggle_and_resets_on_destroy() {
    reset_reactive_runtime();
    let theme: UiThemeState = UiThemeState::new(DesignScheme::Shadcn, ThemeMode::Light);
    let mounts_a: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let mounts_c: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let ma: Rc<Cell<u32>> = Rc::clone(&mounts_a);
    let mc: Rc<Cell<u32>> = Rc::clone(&mounts_c);
    let theme_route_c: UiThemeState = theme.clone();
    let router: EuvRouter = EuvRouter::new("/a")
        .with_default_retention(PageRetention::Freeze)
        .route_with_retention(
            "/c",
            "Destroy Page",
            PageRetention::Destroy,
            move |_: &EuvRouteMatch| {
                mc.set(mc.get() + 1);
                let tok: ThemeTokens = untracked(|| theme_route_c.get_tokens().get());
                let rows: Vec<NodeId> = (0usize..36usize)
                    .map(|idx: usize| {
                        view! {
                            row {
                                width: Length::Percent(100.0)
                                height: Length::Px(32.0)
                                background: tok.get_palette().get_card()
                                text {
                                    color: tok.get_palette().get_foreground()
                                    { format!("/c row {}", idx) }
                                }
                            }
                        }
                    })
                    .collect();
                view! {
                    column {
                        width: Length::Percent(100.0)
                        gap: 4.0
                        ..rows
                    }
                }
            },
        );
    let router_c: EuvRouter = router.clone();
    let theme_c: UiThemeState = theme.clone();
    let theme_outlet: UiThemeState = theme.clone();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 400).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let outlet_id: NodeId = app.mount(move || {
        router_c
            .outlet(move |m: &EuvRouteMatch| {
                if m.get_path() == "/a" {
                    ma.set(ma.get() + 1);
                }
                let tok: ThemeTokens = untracked(|| theme_c.get_tokens().get());
                let rows: Vec<NodeId> = (0usize..36usize)
                    .map(|idx: usize| {
                        view! {
                            row {
                                width: Length::Percent(100.0)
                                height: Length::Px(32.0)
                                background: tok.get_palette().get_card()
                                text {
                                    color: tok.get_palette().get_foreground()
                                    { format!("{} row {}", m.get_path(), idx) }
                                }
                            }
                        }
                    })
                    .collect();
                view! {
                    column {
                        width: Length::Percent(100.0)
                        gap: 4.0
                        ..rows
                    }
                }
            })
            .with_theme(theme_outlet)
            .mount()
    });
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(mounts_a.get(), 1);
    let scroll_a: NodeId = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(outlet_id)
                .and_then(|n: &SceneNode| n.get_children().first().copied())
                .unwrap()
        },
        NodeId::default(),
    );
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            if let Some(node) = tree.get_node_mut(scroll_a) {
                node.get_mut_layout_box()
                    .set_scroll_offset(Point::new(0.0, 220.0));
            }
        },
        (),
    );
    let _: FrameStats = app.step_frame().unwrap();
    let _: bool = theme.get_mode().set_if_changed(ThemeMode::Dark);
    theme.select_scheme(DesignScheme::AntDesign);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(mounts_a.get(), 1);
    let scroll_after_theme: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_a)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    assert!((scroll_after_theme - 220.0).abs() <= 1.0);
    router.push("/b");
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    router.push("/a");
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(mounts_a.get(), 1);
    let scroll_after_resume: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(scroll_a)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    assert!((scroll_after_resume - 220.0).abs() <= 1.0);
    router.push("/c");
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(mounts_c.get(), 1);
    let scroll_c1: NodeId = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(outlet_id)
                .and_then(|n: &SceneNode| {
                    n.get_children().iter().copied().find(|cid: &NodeId| {
                        tree.get_node(*cid)
                            .map(|cn: &SceneNode| {
                                cn.get_alive()
                                    && cn.get_style().get_layout().get_display() != Display::None
                            })
                            .unwrap_or(false)
                    })
                })
                .unwrap()
        },
        NodeId::default(),
    );
    app.get_scene().with_write(
        |tree: &mut SceneTree| {
            if let Some(node) = tree.get_node_mut(scroll_c1) {
                node.get_mut_layout_box()
                    .set_scroll_offset(Point::new(0.0, 180.0));
            }
        },
        (),
    );
    router.push("/a");
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    router.push("/c");
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(mounts_c.get(), 2);
    let scroll_c2_y: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(outlet_id)
                .and_then(|n: &SceneNode| {
                    n.get_children().iter().copied().find_map(|cid: NodeId| {
                        let cn: &SceneNode = tree.get_node(cid)?;
                        if cn.get_alive()
                            && cn.get_style().get_layout().get_display() != Display::None
                        {
                            Some(cn.get_layout_box().get_scroll_offset().get_y())
                        } else {
                            None
                        }
                    })
                })
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    assert!(scroll_c2_y.abs() <= 0.5);
}

#[test]
fn code_block_supports_hv_scroll_fullscreen_modal_clipboard_and_selectable() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Dark);
    let mut lines: Vec<String> = Vec::with_capacity(32);
    lines.push(
        "pub fn very_long_function_signature_demonstrating_horizontal_scroll(alpha: usize, beta: usize, gamma: usize, delta: usize) -> usize {"
            .to_string(),
    );
    for idx in 1..31 {
        lines.push(format!(
            "    let accumulator_{idx}: usize = alpha + beta + gamma + delta + {idx};"
        ));
    }
    lines.push("    accumulator_30".to_string());
    lines.push("}".to_string());
    let sample_code: String = lines.join("\n");
    let copied_len: Rc<Cell<usize>> = Rc::new(Cell::new(0));
    let copied_c: Rc<Cell<usize>> = Rc::clone(&copied_len);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(360, 600).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        view! {
            column {
                width: Length::Percent(100.0)
                padding: Edges::all(12.0)
                EuvCodeBlock {
                    code: sample_code
                    tokens: tokens
                    language: "rust"
                    filename: "long_scroll.rs"
                    line_numbers: true
                    max_height: 200.0
                    selectable: true
                    on_copy: move |s: &str| {
                        copied_c.set(s.len());
                    }
                }
            }
        }
    });
    let wid: WindowId = app.get_window_state().get_id();
    let _: FrameStats = app.step_frame().unwrap();
    let (max_x, max_y, has_selectable_code, fs_btn_pt, copy_btn_pt): (
        f32,
        f32,
        bool,
        Option<Point>,
        Option<Point>,
    ) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut mx: f32 = 0.0;
            let mut my: f32 = 0.0;
            let mut sel_ok: bool = false;
            let mut fs_pt: Option<Point> = None;
            let mut cp_pt: Option<Point> = None;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() {
                    continue;
                }
                let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                if matches!(n.get_kind(), NodeKind::ScrollView) {
                    let lb: LayoutBox = n.get_layout_box();
                    let cur_mx: f32 = (lb.get_content_size().get_width()
                        - lb.get_content_bounds().get_width())
                    .max(0.0);
                    let cur_my: f32 = (lb.get_content_size().get_height()
                        - lb.get_content_bounds().get_height())
                    .max(0.0);
                    mx = mx.max(cur_mx);
                    my = my.max(cur_my);
                }
                if let NodeKind::Text(t) = n.get_kind() {
                    if t.contains("very_long_function_signature") && tree.is_node_selectable(nid) {
                        sel_ok = true;
                    }
                    if t == "Fullscreen"
                        && let Some(b) = tree.node_screen_bounds(nid)
                    {
                        fs_pt = Some(Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        ));
                    }
                    if t == "Copy"
                        && let Some(b) = tree.node_screen_bounds(nid)
                    {
                        cp_pt = Some(Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        ));
                    }
                }
            }
            (mx, my, sel_ok, fs_pt, cp_pt)
        },
        (0.0, 0.0, false, None, None),
    );
    assert!(max_x > 20.0);
    assert!(max_y > 20.0);
    assert!(has_selectable_code);
    let cp_pt: Point = copy_btn_pt.expect("copy button must exist");
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(cp_pt.get_x(), cp_pt.get_y()),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::mouse(cp_pt.get_x(), cp_pt.get_y()),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(copied_len.get() > 100);
    let fs_pt: Point = fs_btn_pt.expect("fullscreen button must exist");
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(fs_pt.get_x(), fs_pt.get_y()),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::mouse(fs_pt.get_x(), fs_pt.get_y()),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(EuvOverlayStack::active_count() >= 1);
    let has_close_btn: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive() && matches!(n.get_kind(), NodeKind::Text(t) if t == "Close")
            })
        },
        false,
    );
    assert!(has_close_btn);
    assert!(EuvOverlayStack::dismiss_top());
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(EuvOverlayStack::active_count(), 0);
}

#[test]
fn full_markdown_and_vector_mermaid_parse_and_render_all_constructs() {
    reset_reactive_runtime();
    let inlines: Vec<EuvMarkdownInline> = parse_markdown_inlines(
        "**bold** *italic* ***both*** ~~del~~ ==mark== ^2^ ~2~ `code` $x^2 + \\alpha$ [^1] <kbd>K</kbd> :rocket: <https://euv.ltpp.vip>",
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Bold(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Italic(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::BoldItalic(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Strike(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Highlight(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Superscript(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Subscript(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Math(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::FootnoteRef(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Kbd(_)))
    );
    assert!(
        inlines
            .iter()
            .any(|i: &EuvMarkdownInline| matches!(i, EuvMarkdownInline::Link { .. }))
    );
    assert_eq!(resolve_emoji_shortcode("rocket"), Some("🚀"));
    let math_out: String = format_latex_math("\\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a} \\to \\infty");
    assert!(math_out.contains('±'));
    assert!(math_out.contains('√'));
    assert!(math_out.contains('→'));
    assert!(math_out.contains('∞'));
    assert!(matches!(
        parse_mermaid_diagram("flowchart LR\nA[Start] --> B{Decide}\nB -->|Yes| C((Done))"),
        MermaidDiagram::Flowchart { .. }
    ));
    assert!(matches!(
        parse_mermaid_diagram("sequenceDiagram\nAlice->>Bob: Hello\nBob-->>Alice: Hi"),
        MermaidDiagram::Sequence { .. }
    ));
    assert!(matches!(
        parse_mermaid_diagram("stateDiagram-v2\n[*] --> Idle\nIdle --> Running: go"),
        MermaidDiagram::State { .. }
    ));
    assert!(matches!(
        parse_mermaid_diagram("classDiagram\nclass Engine {\n+render()\n}\nEngine <|-- Native"),
        MermaidDiagram::Class { .. }
    ));
    assert!(matches!(
        parse_mermaid_diagram("erDiagram\nUSER ||--o{ ORDER : places"),
        MermaidDiagram::Er { .. }
    ));
    assert!(matches!(
        parse_mermaid_diagram("pie title Budget\n\"Layout\" : 20\n\"Paint\" : 80"),
        MermaidDiagram::Pie { .. }
    ));
    assert!(matches!(
        parse_mermaid_diagram("gantt\ntitle Plan\nsection Core\nTask A :done, 2026-01-01, 10d"),
        MermaidDiagram::Gantt { .. }
    ));
    let full_doc: &str = "# Title {#custom}\n\nParagraph with [Ref][r] and [^1].\n\n> [!NOTE]\n> Note line\n\n> [!WARNING]\n> Warn line\n\n$$\\sum_{k=1}^{\\infty} \\frac{1}{k^2} = \\frac{\\pi^2}{6}$$\n\nTerm\n: Definition line\n\n```mermaid\nflowchart TD\nA[Input] --> B[Output]\n```\n\n```mermaid\npie title Share\n\"A\" : 40\n\"B\" : 60\n```\n\n[r]: https://euv.ltpp.vip\n[^1]: Footnote body.\n";
    let blocks: Vec<EuvMarkdownBlock> = parse_markdown_blocks(full_doc);
    assert!(
        blocks
            .iter()
            .any(|b: &EuvMarkdownBlock| matches!(b, EuvMarkdownBlock::MathBlock(_)))
    );
    assert!(
        blocks
            .iter()
            .any(|b: &EuvMarkdownBlock| matches!(b, EuvMarkdownBlock::DefinitionList(_)))
    );
    assert!(
        blocks
            .iter()
            .any(|b: &EuvMarkdownBlock| matches!(b, EuvMarkdownBlock::Footnotes(_)))
    );
    assert_eq!(
        blocks
            .iter()
            .filter(|b: &&EuvMarkdownBlock| matches!(b, EuvMarkdownBlock::Mermaid { .. }))
            .count(),
        2
    );
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 900).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        view! {
            scroll {
                style: Style::default().column()
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                gap: SPACE_MD
                EuvCard {
                    title: "Selectable Card".to_string()
                    tokens: tokens
                    selectable: true
                    EuvAlert {
                        title: "Selectable Alert".to_string()
                        description: "Alert description text".to_string()
                        tone: EuvTone::Primary
                        tokens: tokens
                        selectable: true
                    }
                }
                EuvMarkdown {
                    source: full_doc.to_string()
                    tokens: tokens
                    selectable: true
                }
            }
        }
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 20);
    let (card_sel, alert_sel): (bool, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut c_ok: bool = false;
            let mut a_ok: bool = false;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() {
                    continue;
                }
                let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                if let NodeKind::Text(t) = n.get_kind() {
                    if t == "Selectable Card" && tree.is_node_selectable(nid) {
                        c_ok = true;
                    }
                    if t == "Alert description text" && tree.is_node_selectable(nid) {
                        a_ok = true;
                    }
                }
            }
            (c_ok, a_ok)
        },
        (false, false),
    );
    assert!(card_sel);
    assert!(alert_sel);
}

#[test]
fn bottom_nav_uses_disciplined_radii_and_slides_active_indicator() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let route: Signal<String> = Signal::new("/".to_string());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(1280, 800).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        let nav: NodeId = view! {
            EuvSidebarNav {
                items: vec![EuvNavItem::new("/".to_string(), "Home".to_string())]
                active_route: route
                tokens: tokens
            }
        };
        let bottom: NodeId = view! {
            EuvBottomNav {
                items: vec![
                    EuvNavItem::new("/".to_string(), "Home".to_string()),
                    EuvNavItem::new("/markdown".to_string(), "Docs".to_string()),
                    EuvNavItem::new("/lab".to_string(), "Cases".to_string()),
                    EuvNavItem::new("/stress".to_string(), "Stress".to_string()),
                    EuvNavItem::new("/game3d".to_string(), "3D".to_string()),
                ]
                active_route: route
                tokens: tokens
            }
        };
        let body: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                text { "Dock Test" }
            }
        };
        view! {
            EuvAppShell {
                brand: "Dock".to_string()
                sidebar: nav
                main_content: body
                tokens: tokens
                route: route
                bottom_nav: bottom
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    let (dock_radius, ind_node, ind_radius): (f32, NodeId, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut d_rad: f32 = 999.0;
            let mut i_id: NodeId = NodeId::default();
            let mut i_rad: f32 = 999.0;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() {
                    continue;
                }
                let lay: LayoutStyle = n.get_style().get_layout();
                let b: Rect = n.get_layout_box().get_bounds();
                if lay.get_position() == Position::Fixed
                    && lay.get_z_index() == Z_INDEX_CHROME
                    && (b.get_height() - 60.0).abs() < 1.0
                {
                    d_rad = n.get_style().get_paint().get_radius().get_top_left();
                }
                if lay.get_position() == Position::Absolute
                    && lay.get_z_index() == 0
                    && (b.get_height() - 46.0).abs() < 1.0
                {
                    i_id = NodeId::new(idx as u32, n.get_generation());
                    if let Some(&child_id) = n.get_children().first()
                        && let Some(child_n) = tree.get_node(child_id)
                    {
                        i_rad = child_n.get_style().get_paint().get_radius().get_top_left();
                    }
                }
            }
            (d_rad, i_id, i_rad)
        },
        (999.0, NodeId::default(), 999.0),
    );
    assert!(dock_radius <= 14.0);
    assert!(ind_radius <= 10.0);
    let _: bool = route.set_if_changed("/lab".to_string());
    for _ in 0..4 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let mid_tx: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(ind_node)
                .map(|n: &SceneNode| n.get_style().get_paint().get_transform().get_translate_x())
                .unwrap_or(0.0)
        },
        0.0,
    );
    assert!(mid_tx < -5.0);
    for _ in 0..24 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let end_tx: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(ind_node)
                .map(|n: &SceneNode| n.get_style().get_paint().get_transform().get_translate_x())
                .unwrap_or(99.0)
        },
        99.0,
    );
    assert!(end_tx.abs() <= 0.5);
}
