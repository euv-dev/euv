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
    let (max_x, max_y, has_selectable_code, fs_btn_pt, copy_btn_pt, card_h, closed_portal_ok): (
        f32,
        f32,
        bool,
        Option<Point>,
        Option<Point>,
        f32,
        bool,
    ) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut mx: f32 = 0.0;
            let mut my: f32 = 0.0;
            let mut sel_ok: bool = false;
            let mut fs_pt: Option<Point> = None;
            let mut cp_pt: Option<Point> = None;
            let mut root_card_h: f32 = 0.0;
            let mut portal_hidden: bool = false;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() {
                    continue;
                }
                let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                let lay: LayoutStyle = n.get_style().get_layout();
                if matches!(n.get_kind(), NodeKind::Portal) {
                    portal_hidden = lay.get_display() == Display::None
                        && lay.get_position() == Position::Fixed
                        && lay.get_z_index() == Z_INDEX_FULLSCREEN;
                }
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
                    if let Some(pid) = n.parent_id()
                        && let Some(pnode) = tree.get_node(pid)
                    {
                        root_card_h =
                            root_card_h.max(pnode.get_layout_box().get_bounds().get_height());
                    }
                }
                if let NodeKind::Text(t) = n.get_kind()
                    && t.contains("very_long_function_signature")
                    && tree.is_node_selectable(nid)
                {
                    sel_ok = true;
                }
                if let Some(a11y) = n.a11y_props_ref() {
                    if a11y.get_label() == "Fullscreen"
                        && let Some(b) = tree.node_screen_bounds(nid)
                    {
                        fs_pt = Some(Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        ));
                    }
                    if a11y.get_label() == "Copy"
                        && let Some(b) = tree.node_screen_bounds(nid)
                    {
                        cp_pt = Some(Point::new(
                            b.get_x() + b.get_width() * 0.5,
                            b.get_y() + b.get_height() * 0.5,
                        ));
                    }
                }
            }
            (mx, my, sel_ok, fs_pt, cp_pt, root_card_h, portal_hidden)
        },
        (0.0, 0.0, false, None, None, 0.0, false),
    );
    assert!(max_x > 20.0);
    assert!(max_y > 20.0);
    assert!(has_selectable_code);
    assert!(card_h > 40.0 && card_h < 280.0);
    assert!(closed_portal_ok);
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
    let (has_close_btn, open_portal_ok): (bool, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let close_ok: bool = tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive()
                    && n.a11y_props_ref()
                        .is_some_and(|a: &A11yProps| a.get_label() == "Close")
            });
            let portal_vis: bool = tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Portal)
                    && n.get_style().get_layout().get_display() == Display::Flex
                    && n.get_style().get_layout().get_position() == Position::Fixed
                    && n.get_style().get_layout().get_z_index() == Z_INDEX_FULLSCREEN
            });
            (close_ok, portal_vis)
        },
        (false, false),
    );
    assert!(has_close_btn);
    assert!(open_portal_ok);
    assert!(EuvOverlayStack::dismiss_top());
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(EuvOverlayStack::active_count(), 0);
    assert!(!is_fullscreen_active());
    let portal_rehidden: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Portal)
                    && n.get_style().get_layout().get_display() == Display::None
            })
        },
        false,
    );
    assert!(portal_rehidden);
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
    let (dock_radius, ind_node, ind_radius, ind_cy, tab_cy): (f32, NodeId, f32, f32, f32) =
        app.get_scene().with_read(
            |tree: &SceneTree| {
                let mut d_rad: f32 = 999.0;
                let mut i_id: NodeId = NodeId::default();
                let mut i_rad: f32 = 999.0;
                let mut i_cy: f32 = 0.0;
                let mut t_cy: f32 = 0.0;
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
                        i_cy = b.get_y() + b.get_height() * 0.5;
                        if let Some(&child_id) = n.get_children().first()
                            && let Some(child_n) = tree.get_node(child_id)
                        {
                            i_rad = child_n.get_style().get_paint().get_radius().get_top_left();
                        }
                    }
                    if lay.get_z_index() == 1 && (b.get_height() - 46.0).abs() < 1.0 && t_cy == 0.0
                    {
                        t_cy = b.get_y() + b.get_height() * 0.5;
                    }
                }
                (d_rad, i_id, i_rad, i_cy, t_cy)
            },
            (999.0, NodeId::default(), 999.0, 0.0, 0.0),
        );
    assert!(dock_radius <= 14.0);
    assert!(ind_radius <= 10.0);
    assert!((ind_cy - tab_cy).abs() <= 1.0);
    let expected_muted: Color = tokens.get_palette().get_muted_foreground();
    let expected_primary_fg: Color = tokens.get_palette().get_primary_foreground();
    let _: bool = app
        .get_context()
        .get_swipe_drag_offset()
        .set_if_changed(-480.0);
    let _: FrameStats = app.step_frame().unwrap();
    let home_color_mid_drag: Option<Color> = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().find_map(|n: &SceneNode| {
                if n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Text(t) if t == "Home")
                    && (n.get_style().get_text().get_font_size() - 10.5).abs() < 0.2
                {
                    Some(n.get_style().get_text().get_color())
                } else {
                    None
                }
            })
        },
        None,
    );
    assert_eq!(home_color_mid_drag, Some(expected_muted));
    let _: bool = app
        .get_context()
        .get_swipe_drag_offset()
        .set_if_changed(0.0);
    let _: FrameStats = app.step_frame().unwrap();
    let home_color_settled: Option<Color> = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().find_map(|n: &SceneNode| {
                if n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Text(t) if t == "Home")
                    && (n.get_style().get_text().get_font_size() - 10.5).abs() < 0.2
                {
                    Some(n.get_style().get_text().get_color())
                } else {
                    None
                }
            })
        },
        None,
    );
    assert_eq!(home_color_settled, Some(expected_primary_fg));
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

#[test]
fn bottom_nav_active_label_stays_contrasting_after_route_switch_and_dark_toggle() {
    reset_reactive_runtime();
    clear_provided_theme();
    let theme: UiThemeState = UiThemeState::new(DesignScheme::Shadcn, ThemeMode::Light);
    let init_tokens: ThemeTokens = untracked(|| theme.get_tokens().get());
    let route: Signal<String> = Signal::new("/".to_string());
    let theme_shell: UiThemeState = theme.clone();
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
                tokens: init_tokens
            }
        };
        let bottom: NodeId = view! {
            EuvBottomNav {
                items: vec![
                    EuvNavItem::new("/".to_string(), "Home".to_string()),
                    EuvNavItem::new("/markdown".to_string(), "Docs".to_string()),
                ]
                active_route: route
                tokens: init_tokens
            }
        };
        let body: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                text { "Theme Switch Body" }
            }
        };
        view! {
            EuvAppShell {
                brand: "ThemeDock".to_string()
                sidebar: nav
                main_content: body
                tokens: init_tokens
                theme: theme_shell
                route: route
                bottom_nav: bottom
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    let _: bool = route.set_if_changed("/markdown".to_string());
    let _: FrameStats = app.step_frame().unwrap();
    let _: bool = route.set_if_changed("/".to_string());
    let _: FrameStats = app.step_frame().unwrap();
    let _: bool = theme.get_mode().set_if_changed(ThemeMode::Dark);
    let _: FrameStats = app.step_frame().unwrap();
    let dark_tokens: ThemeTokens = untracked(|| theme.get_tokens().get());
    let expected_dark_fg: Color = dark_tokens.get_palette().get_primary_foreground();
    let expected_dark_primary: Color = dark_tokens.get_palette().get_primary();
    let expected_dark_dock_bg: Color = dark_tokens.get_palette().get_card().with_alpha(244);
    let (home_color, dock_bg, pill_bg): (Option<Color>, Option<Color>, Option<Color>) =
        app.get_scene().with_read(
            |tree: &SceneTree| {
                let mut h_col: Option<Color> = None;
                let mut d_bg: Option<Color> = None;
                let mut p_bg: Option<Color> = None;
                for n in tree.get_nodes().iter() {
                    if !n.get_alive() {
                        continue;
                    }
                    let lay: LayoutStyle = n.get_style().get_layout();
                    let b: Rect = n.get_layout_box().get_bounds();
                    if lay.get_position() == Position::Fixed
                        && lay.get_z_index() == Z_INDEX_CHROME
                        && (b.get_height() - 60.0).abs() < 1.0
                    {
                        d_bg = n.get_style().get_paint().background_color();
                    }
                    if lay.get_position() == Position::Absolute
                        && lay.get_z_index() == 0
                        && (b.get_height() - 46.0).abs() < 1.0
                        && let Some(&child_id) = n.get_children().first()
                        && let Some(child_n) = tree.get_node(child_id)
                    {
                        p_bg = child_n.get_style().get_paint().background_color();
                    }
                    if matches!(n.get_kind(), NodeKind::Text(t) if t == "Home")
                        && (n.get_style().get_text().get_font_size() - 10.5).abs() < 0.2
                    {
                        h_col = Some(n.get_style().get_text().get_color());
                    }
                }
                (h_col, d_bg, p_bg)
            },
            (None, None, None),
        );
    assert_eq!(home_color, Some(expected_dark_fg));
    assert_eq!(pill_bg, Some(expected_dark_primary));
    assert_eq!(dock_bg, Some(expected_dark_dock_bg));
    clear_provided_theme();
}

#[test]
fn mermaid_state_cycle_and_markdown_heading_emoji_and_def_list_render_cleanly() {
    reset_reactive_runtime();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let state_src: &str = "stateDiagram-v2\n    [*] --> Active\n    Active --> Frozen: Leave Route\n    Frozen --> Active: Resume Coalesced\n    Active --> [*]: Destroy\n";
    let md_src: &str = "# Markdown Studio :rocket:\n\n`PageRetention::Freeze`\n: Suspends inactive routes cleanly.\n\n$$x = 1$$\n";
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(800, 900).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                EuvMermaid {
                    source: state_src.to_string()
                    tokens: tokens
                }
                EuvMarkdown {
                    source: md_src.to_string()
                    tokens: tokens
                }
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    let (
        mermaid_cb,
        mermaid_bounds,
        has_rocket_h1,
        has_literal_rocket,
        has_clean_term,
        has_literal_tick,
    ): (Option<CanvasDrawCallback>, Rect, bool, bool, bool, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut cb_opt: Option<CanvasDrawCallback> = None;
            let mut cb_rect: Rect = Rect::default();
            let mut rocket_ok: bool = false;
            let mut lit_rocket: bool = false;
            let mut clean_term: bool = false;
            let mut lit_tick: bool = false;
            for n in tree.get_nodes().iter() {
                if !n.get_alive() {
                    continue;
                }
                if matches!(n.get_kind(), NodeKind::Canvas)
                    && n.get_layout_box().get_bounds().get_height() >= 120.0
                    && cb_opt.is_none()
                {
                    cb_opt = n.canvas_callback();
                    cb_rect = n.get_layout_box().get_bounds();
                }
                if let NodeKind::Text(t) = n.get_kind() {
                    if t.contains("Markdown Studio 🚀") {
                        rocket_ok = true;
                    }
                    if t.contains(":rocket:") {
                        lit_rocket = true;
                    }
                    if t == "PageRetention::Freeze" {
                        clean_term = true;
                    }
                    if t.contains("`PageRetention::Freeze`") {
                        lit_tick = true;
                    }
                }
            }
            (cb_opt, cb_rect, rocket_ok, lit_rocket, clean_term, lit_tick)
        },
        (None, Rect::default(), false, false, false, false),
    );
    assert!(has_rocket_h1);
    assert!(!has_literal_rocket);
    assert!(has_clean_term);
    assert!(!has_literal_tick);
    let paint_cb: CanvasDrawCallback = mermaid_cb.expect("mermaid canvas painter must exist");
    let mut dl: DisplayList = DisplayList::default();
    (paint_cb.borrow_mut())(mermaid_bounds, &mut dl);
    let mut active_r: Option<Rect> = None;
    let mut frozen_r: Option<Rect> = None;
    let mut leave_r: Option<Rect> = None;
    let mut resume_r: Option<Rect> = None;
    let mut destroy_r: Option<Rect> = None;
    for cmd in dl.get_commands().iter() {
        if let DrawCommand::TextRun { text, rect, .. } = cmd {
            match text.as_str() {
                "Active" => active_r = Some(*rect),
                "Frozen" => frozen_r = Some(*rect),
                "Leave Route" => leave_r = Some(*rect),
                "Resume Coalesced" => resume_r = Some(*rect),
                "Destroy" => destroy_r = Some(*rect),
                _ => {}
            }
        }
    }
    let a: Rect = active_r.expect("Active label must be drawn");
    let f: Rect = frozen_r.expect("Frozen label must be drawn");
    let l: Rect = leave_r.expect("Leave Route label must be drawn");
    let r: Rect = resume_r.expect("Resume Coalesced label must be drawn");
    let d: Rect = destroy_r.expect("Destroy label must be drawn");
    assert!(a.get_y() < f.get_y());
    for (x, y) in [(l, r), (l, d), (r, d), (d, a), (d, f)].into_iter() {
        let hit: bool = x.get_x() < y.right()
            && x.right() > y.get_x()
            && x.get_y() < y.bottom()
            && x.bottom() > y.get_y();
        assert!(!hit);
    }
}

#[derive(Clone, Debug, New)]
struct AppearTrackedPage {
    label: String,
    appear_hits: Rc<Cell<u32>>,
    disappear_hits: Rc<Cell<u32>>,
}

#[component(retention = Freeze)]
impl AppearTrackedPage {
    fn on_appear(&self) {
        self.appear_hits.set(self.appear_hits.get() + 1);
    }

    fn on_disappear(&self) {
        self.disappear_hits.set(self.disappear_hits.get() + 1);
    }

    fn render(&self) -> NodeId {
        let title: String = self.label.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                text {
                    { title }
                }
            }
        }
    }
}

#[test]
fn router_outlet_emits_subtree_appear_and_disappear_hooks_on_route_transitions() {
    reset_reactive_runtime();
    let theme: UiThemeState = UiThemeState::new(DesignScheme::Shadcn, ThemeMode::Light);
    let a_appear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let a_disappear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let b_comp_appear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let b_comp_disappear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let b_hook_appear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let b_hook_disappear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let c_appear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let c_disappear: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let aa: Rc<Cell<u32>> = Rc::clone(&a_appear);
    let ad: Rc<Cell<u32>> = Rc::clone(&a_disappear);
    let bca: Rc<Cell<u32>> = Rc::clone(&b_comp_appear);
    let bcd: Rc<Cell<u32>> = Rc::clone(&b_comp_disappear);
    let bha: Rc<Cell<u32>> = Rc::clone(&b_hook_appear);
    let bhd: Rc<Cell<u32>> = Rc::clone(&b_hook_disappear);
    let ca: Rc<Cell<u32>> = Rc::clone(&c_appear);
    let cd: Rc<Cell<u32>> = Rc::clone(&c_disappear);
    let router: EuvRouter = EuvRouter::new("/a")
        .with_default_retention(PageRetention::Freeze)
        .route_with_retention(
            "/a",
            "Page A",
            PageRetention::Freeze,
            move |_: &EuvRouteMatch| {
                let aa_in: Rc<Cell<u32>> = Rc::clone(&aa);
                let ad_in: Rc<Cell<u32>> = Rc::clone(&ad);
                let _: SystemEventSubscription = on_appear(move || {
                    aa_in.set(aa_in.get() + 1);
                });
                let _: SystemEventSubscription = on_disappear(move || {
                    ad_in.set(ad_in.get() + 1);
                });
                view! {
                    column {
                        text { "Page A" }
                    }
                }
            },
        )
        .route_with_retention(
            "/b",
            "Page B",
            PageRetention::Freeze,
            move |_: &EuvRouteMatch| {
                let bha_in: Rc<Cell<u32>> = Rc::clone(&bha);
                let bhd_in: Rc<Cell<u32>> = Rc::clone(&bhd);
                let _: SystemEventSubscription = on_appear(move || {
                    bha_in.set(bha_in.get() + 1);
                });
                let _: SystemEventSubscription = on_disappear(move || {
                    bhd_in.set(bhd_in.get() + 1);
                });
                AppearTrackedPage::new("Page B".to_string(), Rc::clone(&bca), Rc::clone(&bcd))
                    .mount()
            },
        )
        .route_with_retention(
            "/c",
            "Page C",
            PageRetention::Destroy,
            move |_: &EuvRouteMatch| {
                let ca_in: Rc<Cell<u32>> = Rc::clone(&ca);
                let cd_in: Rc<Cell<u32>> = Rc::clone(&cd);
                let _: SystemEventSubscription = on_appear(move || {
                    ca_in.set(ca_in.get() + 1);
                });
                let _: SystemEventSubscription = on_disappear(move || {
                    cd_in.set(cd_in.get() + 1);
                });
                view! {
                    column {
                        text { "Page C" }
                    }
                }
            },
        );
    let router_c: EuvRouter = router.clone();
    let theme_outlet: UiThemeState = theme.clone();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(640, 480).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        router_c
            .outlet(move |_: &EuvRouteMatch| view! { column {} })
            .with_theme(theme_outlet)
            .mount()
    });
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(a_appear.get(), 1);
    assert_eq!(a_disappear.get(), 0);
    assert_eq!(b_comp_appear.get(), 0);
    assert_eq!(b_hook_appear.get(), 0);
    router.push("/b");
    for _ in 0..20 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(a_appear.get(), 1);
    assert_eq!(a_disappear.get(), 1);
    assert_eq!(b_comp_appear.get(), 1);
    assert_eq!(b_hook_appear.get(), 1);
    assert_eq!(b_comp_disappear.get(), 0);
    assert_eq!(b_hook_disappear.get(), 0);
    let _: bool = theme.get_mode().set_if_changed(ThemeMode::Dark);
    let _: FrameStats = app.step_frame().unwrap();
    assert_eq!(b_comp_appear.get(), 1);
    assert_eq!(b_comp_disappear.get(), 0);
    router.push("/a");
    for _ in 0..20 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(b_comp_disappear.get(), 1);
    assert_eq!(b_hook_disappear.get(), 1);
    assert_eq!(a_appear.get(), 2);
    assert_eq!(a_disappear.get(), 1);
    router.push("/c");
    for _ in 0..20 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(a_disappear.get(), 2);
    assert_eq!(c_appear.get(), 1);
    assert_eq!(c_disappear.get(), 0);
    router.push("/a");
    for _ in 0..20 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    assert_eq!(c_disappear.get(), 1);
    assert_eq!(a_appear.get(), 3);
}

fn find_a11y_button_center(app: &App, label: &str, nth: usize) -> Option<Point> {
    app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut matched: usize = 0;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() || n.get_style().get_layout().get_display() == Display::None {
                    continue;
                }
                if n.a11y_props_ref()
                    .is_some_and(|a: &A11yProps| a.get_label() == label)
                {
                    let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                    if let Some(b) = tree.node_screen_bounds(nid)
                        && b.get_width() > 0.0
                        && b.get_height() > 0.0
                    {
                        if matched == nth {
                            return Some(Point::new(
                                b.get_x() + b.get_width() * 0.5,
                                b.get_y() + b.get_height() * 0.5,
                            ));
                        }
                        matched += 1;
                    }
                }
            }
            None
        },
        None,
    )
}

fn click_point_in_ui(app: &mut App, pt: Point) {
    let wid: WindowId = app.get_window_state().get_id();
    let ev: PointerEvent = PointerEvent::mouse(pt.get_x(), pt.get_y());
    app.dispatch_window_event(WindowEvent::PointerMove(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerDown(wid, ev));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, ev));
    let _: FrameStats = app.step_frame().unwrap();
}

#[test]
fn fullscreen_code_and_mermaid_edge_to_edge_hides_app_shell_chrome_and_restores_on_exit_and_dispose()
 {
    reset_reactive_runtime();
    EuvOverlayStack::clear();
    reset_fullscreen_state();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let route_sig: Signal<String> = Signal::new("/".to_string());
    let clicks: Signal<u32> = Signal::new(0);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(390, 844).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let clicks_cb: Signal<u32> = clicks;
    let _: NodeId = app.mount(move || {
        let on_ping = move |_: &UiEvent| {
            let _: bool = clicks_cb.update(|c: &mut u32| *c += 1);
        };
        let sidebar: NodeId = view! {
            EuvSidebarNav {
                items: vec![EuvNavItem::new("/".to_string(), "Home".to_string())]
                active_route: route_sig
                tokens: tokens
            }
        };
        let bottom_nav: NodeId = view! {
            EuvBottomNav {
                items: vec![
                    EuvNavItem::new("/".to_string(), "Home".to_string()).section("H"),
                    EuvNavItem::new("/docs".to_string(), "Docs".to_string()).section("D"),
                ]
                active_route: route_sig
                tokens: tokens
            }
        };
        let main_col: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                EuvButton {
                    label: "UnderlyingPing".to_string()
                    tokens: tokens
                    on_click: on_ping
                }
                EuvCodeBlock {
                    code: "fn main() {\n    println!(\"true edge-to-edge fullscreen\");\n}".to_string()
                    tokens: tokens
                    language: "rust"
                    line_numbers: true
                }
                EuvMermaid {
                    source: "flowchart LR\n    A[Start] --> B[End]".to_string()
                    tokens: tokens
                }
            }
        };
        view! {
            EuvAppShell {
                brand: "Euv Studio".to_string()
                sidebar: sidebar
                main_content: main_col
                tokens: tokens
                bottom_nav: bottom_nav
                show_fps: true
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    let badge_radii: Vec<(f32, f32, f32)> = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut out: Vec<(f32, f32, f32)> = Vec::new();
            for node in tree.get_nodes().iter() {
                if !node.get_alive() {
                    continue;
                }
                if matches!(node.get_kind(), NodeKind::Canvas)
                    && let Some(pid) = node.parent_id()
                    && let Some(pnode) = tree.get_node(pid)
                    && matches!(
                        pnode.a11y_props_ref().map(|a: &A11yProps| a.get_label().as_str()),
                        Some("Menu" | "Euv Studio")
                    )
                    && matches!(pnode.style_ref().get_layout().get_width(), Length::Px(w) if (w - 26.0).abs() < 0.5)
                {
                    let lay: LayoutStyle = pnode.style_ref().get_layout();
                    let rad: f32 = pnode.style_ref().get_paint().get_radius().get_top_left();
                    let w: f32 = match lay.get_width() {
                        Length::Px(px) => px,
                        _ => 0.0,
                    };
                    let h: f32 = match lay.get_height() {
                        Length::Px(px) => px,
                        _ => 0.0,
                    };
                    out.push((w, h, rad));
                }
            }
            out
        },
        Vec::new(),
    );
    assert!(badge_radii.len() >= 3);
    let first_badge: (f32, f32, f32) = badge_radii[0];
    assert!(first_badge.2 < 12.0);
    for entry in badge_radii.iter() {
        assert_eq!(*entry, first_badge);
    }
    let code_fs_pt: Point =
        find_a11y_button_center(&app, "Fullscreen", 0).expect("code fullscreen icon button");
    click_point_in_ui(&mut app, code_fs_pt);
    assert!(is_fullscreen_active());
    assert!(app.is_immersive_fullscreen());
    let (portal_bounds, chrome_hidden): (Option<Rect>, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut fs_rect: Option<Rect> = None;
            let mut any_visible_chrome: bool = false;
            for (idx, n) in tree.get_nodes().iter().enumerate() {
                if !n.get_alive() {
                    continue;
                }
                let lay: LayoutStyle = n.style_ref().get_layout();
                let paint: PaintStyle = n.style_ref().get_paint();
                if lay.get_z_index() == Z_INDEX_CHROME && lay.get_display() != Display::None {
                    any_visible_chrome = true;
                }
                if matches!(n.get_kind(), NodeKind::Portal)
                    && lay.get_z_index() == Z_INDEX_FULLSCREEN
                    && lay.get_display() == Display::Flex
                {
                    assert_eq!(lay.get_position(), Position::Fixed);
                    assert_eq!(lay.get_padding(), Edges::all(0.0));
                    assert_eq!(lay.get_margin(), Edges::all(0.0));
                    assert_eq!(paint.get_radius(), CornerRadius::all(0.0));
                    assert_eq!(paint.get_border().get_width(), 0.0);
                    let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                    fs_rect = tree.node_screen_bounds(nid);
                }
            }
            (fs_rect, !any_visible_chrome)
        },
        (None, false),
    );
    let fs_rect: Rect = portal_bounds.expect("edge-to-edge code fullscreen portal");
    assert!((fs_rect.get_x() - 0.0).abs() < 0.5);
    assert!((fs_rect.get_y() - 0.0).abs() < 0.5);
    assert!((fs_rect.get_width() - 390.0).abs() < 0.5);
    assert!((fs_rect.get_height() - 844.0).abs() < 0.5);
    assert!(chrome_hidden);
    let close_pt: Point =
        find_a11y_button_center(&app, "Close", 0).expect("code fullscreen close icon button");
    click_point_in_ui(&mut app, close_pt);
    assert!(!is_fullscreen_active());
    assert!(!app.is_immersive_fullscreen());
    assert_eq!(EuvOverlayStack::active_count(), 0);
    let (chrome_restored, any_open_fs_portal): (bool, bool) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let mut visible_chrome: usize = 0;
            let mut open_fs: bool = false;
            for n in tree.get_nodes().iter() {
                if !n.get_alive() {
                    continue;
                }
                let lay: LayoutStyle = n.style_ref().get_layout();
                if lay.get_z_index() == Z_INDEX_CHROME && lay.get_display() == Display::Flex {
                    visible_chrome += 1;
                }
                if matches!(n.get_kind(), NodeKind::Portal)
                    && lay.get_z_index() == Z_INDEX_FULLSCREEN
                    && lay.get_display() != Display::None
                {
                    open_fs = true;
                }
            }
            (visible_chrome >= 2, open_fs)
        },
        (false, true),
    );
    assert!(chrome_restored);
    assert!(!any_open_fs_portal);
    let ping_pt: Point = app
        .get_scene()
        .with_read(
            |tree: &SceneTree| {
                for (idx, n) in tree.get_nodes().iter().enumerate() {
                    if n.get_alive()
                        && matches!(n.get_kind(), NodeKind::Text(t) if t == "UnderlyingPing")
                    {
                        let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                        if let Some(b) = tree.node_screen_bounds(nid) {
                            return Some(Point::new(
                                b.get_x() + b.get_width() * 0.5,
                                b.get_y() + b.get_height() * 0.5,
                            ));
                        }
                    }
                }
                None
            },
            None,
        )
        .expect("underlying button must be hittable");
    click_point_in_ui(&mut app, ping_pt);
    assert_eq!(clicks.get_untracked(), 1);
    let mermaid_fs_pt: Point =
        find_a11y_button_center(&app, "Fullscreen", 1).expect("mermaid fullscreen icon button");
    click_point_in_ui(&mut app, mermaid_fs_pt);
    assert!(is_fullscreen_active());
    let mermaid_rect: Rect = app
        .get_scene()
        .with_read(
            |tree: &SceneTree| {
                for (idx, n) in tree.get_nodes().iter().enumerate() {
                    if n.get_alive()
                        && matches!(n.get_kind(), NodeKind::Portal)
                        && n.style_ref().get_layout().get_z_index() == Z_INDEX_FULLSCREEN
                        && n.style_ref().get_layout().get_display() == Display::Flex
                    {
                        let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                        return tree.node_screen_bounds(nid);
                    }
                }
                None
            },
            None,
        )
        .expect("edge-to-edge mermaid fullscreen portal");
    assert!((mermaid_rect.get_x() - 0.0).abs() < 0.5);
    assert!((mermaid_rect.get_y() - 0.0).abs() < 0.5);
    assert!((mermaid_rect.get_width() - 390.0).abs() < 0.5);
    assert!((mermaid_rect.get_height() - 844.0).abs() < 0.5);
    assert!(EuvOverlayStack::dismiss_top());
    let _: FrameStats = app.step_frame().unwrap();
    assert!(!is_fullscreen_active());
    assert_eq!(EuvOverlayStack::active_count(), 0);
    click_point_in_ui(&mut app, ping_pt);
    assert_eq!(clicks.get_untracked(), 2);
    let (disposable_scope, ()): (Scope, ()) = Scope::run(|_: Scope| {
        let sig: Signal<bool> = Signal::new(true);
        let _: Effect = EuvOverlayStack::bind_signal(sig, "DisposableFs", Z_INDEX_FULLSCREEN);
        enter_fullscreen();
        on_cleanup(move || {
            exit_fullscreen();
        });
    });
    assert!(is_fullscreen_active());
    assert_eq!(EuvOverlayStack::active_count(), 1);
    disposable_scope.dispose();
    assert!(!is_fullscreen_active());
    assert_eq!(EuvOverlayStack::active_count(), 0);
}

#[test]
fn reactive_safe_area_updates_app_shell_header_and_content_on_atomic_island_change() {
    reset_reactive_runtime();
    EuvOverlayStack::clear();
    reset_fullscreen_state();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let route_sig: Signal<String> = Signal::new("/".to_string());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(390, 844).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let _: NodeId = app.mount(move || {
        let sidebar: NodeId = view! {
            EuvSidebarNav {
                items: vec![EuvNavItem::new("/".to_string(), "Home".to_string())]
                active_route: route_sig
                tokens: tokens
            }
        };
        let content: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                text {
                    "SafeAreaMarker"
                }
            }
        };
        view! {
            EuvAppShell {
                brand: "SafeApp".to_string()
                sidebar: sidebar
                main_content: content
                tokens: tokens
                show_fps: true
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    fn find_y(tree: &SceneTree, needle: &str) -> f32 {
        for (idx, n) in tree.get_nodes().iter().enumerate() {
            if n.get_alive()
                && n.style_ref().get_layout().get_display() != Display::None
                && matches!(n.get_kind(), NodeKind::Text(t) if t == needle)
            {
                let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                if let Some(b) = tree.node_screen_bounds(nid) {
                    return b.get_y();
                }
            }
        }
        -1.0
    }
    let cold_content_y: f32 = app
        .get_scene()
        .with_read(|t: &SceneTree| find_y(t, "SafeAreaMarker"), -1.0);
    assert!(cold_content_y >= MOBILE_HEADER_HEIGHT);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        wid,
        SafeAreaInsets::new(56.0, 0.0, 34.0, 0.0),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    let (title_y_island, content_y_island): (f32, f32) = app.get_scene().with_read(
        |t: &SceneTree| (find_y(t, "SafeApp"), find_y(t, "SafeAreaMarker")),
        (-1.0, -1.0),
    );
    assert!(title_y_island >= 56.0);
    assert!(content_y_island >= MOBILE_HEADER_HEIGHT + 56.0);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(wid, SafeAreaInsets::default()));
    let _: FrameStats = app.step_frame().unwrap();
    let restored_content_y: f32 = app
        .get_scene()
        .with_read(|t: &SceneTree| find_y(t, "SafeAreaMarker"), -1.0);
    assert!((restored_content_y - cold_content_y).abs() < 1.0);
}

#[test]
fn rapid_overlays_and_interrupted_route_swipe_never_leave_ui_unclickable() {
    reset_reactive_runtime();
    EuvOverlayStack::clear();
    reset_fullscreen_state();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let modal_open: Signal<bool> = Signal::new(false);
    let drawer_open: Signal<bool> = Signal::new(false);
    let sheet_open: Signal<bool> = Signal::new(false);
    let alert_open: Signal<bool> = Signal::new(false);
    let alert_confirmed: Signal<bool> = Signal::new(false);
    let select_open: Signal<bool> = Signal::new(false);
    let select_idx: Signal<usize> = Signal::new(0);
    let dropdown_open: Signal<bool> = Signal::new(false);
    let clicks: Signal<u32> = Signal::new(0);
    let router: EuvRouter = EuvRouter::new("/a")
        .route("/a", "A", move |_: &EuvRouteMatch| {
            let on_click = move |_: &UiEvent| {
                let _: bool = clicks.update(|c: &mut u32| *c += 1);
            };
            view! {
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    EuvButton {
                        label: "ResponsiveTarget".to_string()
                        tokens: tokens
                        on_click: on_click
                    }
                    EuvDropdown {
                        label: "QuickMenu".to_string()
                        items: vec!["One".to_string(), "Two".to_string()]
                        tokens: tokens
                        open: dropdown_open
                    }
                    EuvSelect {
                        options: vec!["Alpha".to_string(), "Beta".to_string()]
                        selected: select_idx
                        open: select_open
                        tokens: tokens
                    }
                }
            }
        })
        .route("/b", "B", move |_: &EuvRouteMatch| {
            view! {
                column {
                    text { "RouteB" }
                }
            }
        });
    let router_m: EuvRouter = router.clone();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(420, 800).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(move || {
        let outlet: NodeId = router_m
            .outlet(move |_: &EuvRouteMatch| view! { column {} })
            .with_route_order(vec!["/a".to_string(), "/b".to_string()])
            .mount();
        let modal_body: NodeId = view! { text { "ModalBody" } };
        let drawer_body: NodeId = view! { text { "DrawerBody" } };
        let sheet_body: NodeId = view! { text { "SheetBody" } };
        view! {
            column {
                width: Length::Percent(100.0)
                height: Length::Percent(100.0)
                { outlet }
                EuvModal {
                    open: modal_open
                    title: "M".to_string()
                    body: modal_body
                    tokens: tokens
                }
                EuvDrawer {
                    open: drawer_open
                    title: "D".to_string()
                    body: drawer_body
                    tokens: tokens
                }
                EuvSheet {
                    open: sheet_open
                    title: "S".to_string()
                    body: sheet_body
                    tokens: tokens
                }
                EuvAlertDialog {
                    open: alert_open
                    confirmed: alert_confirmed
                    title: "A".to_string()
                    description: "Desc".to_string()
                    tokens: tokens
                }
            }
        }
    });
    let _: FrameStats = app.step_frame().unwrap();
    for _ in 0..8 {
        let _: bool = modal_open.set(true);
        let _: bool = drawer_open.set(true);
        let _: bool = sheet_open.set(true);
        let _: bool = alert_open.set(true);
        let _: bool = dropdown_open.set(true);
        let _: bool = select_open.set(true);
        let _: FrameStats = app.step_frame().unwrap();
        let _: bool = modal_open.set(false);
        let _: bool = drawer_open.set(false);
        let _: bool = sheet_open.set(false);
        let _: bool = alert_open.set(false);
        let _: bool = dropdown_open.set(false);
        let _: bool = select_open.set(false);
        let _: FrameStats = app.step_frame().unwrap();
    }
    assert_eq!(EuvOverlayStack::active_count(), 0);
    let any_visible_portal: bool = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_nodes().iter().any(|n: &SceneNode| {
                n.get_alive()
                    && matches!(n.get_kind(), NodeKind::Portal)
                    && n.style_ref().get_layout().get_display() != Display::None
            })
        },
        true,
    );
    assert!(!any_visible_portal);
    let _: bool = app.get_context().get_swipe_drag_offset().set(-140.0);
    let _: FrameStats = app.step_frame().unwrap();
    app.dispatch_window_event(WindowEvent::System(SystemEvent::Inactive));
    app.dispatch_window_event(WindowEvent::System(SystemEvent::Background));
    let _: bool = app.get_context().get_swipe_drag_offset().set(0.0);
    app.dispatch_window_event(WindowEvent::System(SystemEvent::Foreground));
    app.dispatch_window_event(WindowEvent::System(SystemEvent::Foreground));
    let _: FrameStats = app.step_frame().unwrap();
    let target_pt: Point = app
        .get_scene()
        .with_read(
            |tree: &SceneTree| {
                for (idx, n) in tree.get_nodes().iter().enumerate() {
                    if n.get_alive()
                        && matches!(n.get_kind(), NodeKind::Text(t) if t == "ResponsiveTarget")
                    {
                        let nid: NodeId = NodeId::new(idx as u32, n.get_generation());
                        if let Some(b) = tree.node_screen_bounds(nid) {
                            return Some(Point::new(
                                b.get_x() + b.get_width() * 0.5,
                                b.get_y() + b.get_height() * 0.5,
                            ));
                        }
                    }
                }
                None
            },
            None,
        )
        .expect("target button must be visible after interrupted swipe");
    click_point_in_ui(&mut app, target_pt);
    assert_eq!(clicks.get_untracked(), 1);
}

#[test]
fn horizontal_scroll_and_route_swipe_isolation_for_code_block_table_and_scroll_area() {
    reset_reactive_runtime();
    reset_fullscreen_state();
    EuvOverlayStack::clear();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(390, 844).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Dark);
    let code_a_scroll_ref: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let code_b_scroll_ref: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let table_scroll_ref: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let vscroll_ref: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let code_a_c: Rc<Cell<NodeId>> = Rc::clone(&code_a_scroll_ref);
    let code_b_c: Rc<Cell<NodeId>> = Rc::clone(&code_b_scroll_ref);
    let table_c: Rc<Cell<NodeId>> = Rc::clone(&table_scroll_ref);
    let vscroll_c: Rc<Cell<NodeId>> = Rc::clone(&vscroll_ref);
    let router: EuvRouter = EuvRouter::new("/page-a")
        .route("/page-a", "PageA", move |_: &EuvRouteMatch| {
            let wide_line: String =
                "let horizontal_isolation_probe_value: usize = compute_wide_expression_alpha(100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200);"
                    .to_string();
            let code_a: NodeId = EuvCodeBlock::new(wide_line.clone(), tokens)
                .language("rs")
                .mount();
            let code_b: NodeId = EuvCodeBlock::new(wide_line, tokens)
                .language("rs")
                .mount();
            let md_table: NodeId = EuvMarkdownTable::new(
                vec![
                    "ColumnOne".to_string(),
                    "ColumnTwo".to_string(),
                    "ColumnThree".to_string(),
                    "ColumnFour".to_string(),
                    "ColumnFive".to_string(),
                    "ColumnSix".to_string(),
                ],
                vec![EuvMarkdownAlign::Left; 6],
                vec![vec![
                    "Val1".to_string(),
                    "Val2".to_string(),
                    "Val3".to_string(),
                    "Val4".to_string(),
                    "Val5".to_string(),
                    "Val6".to_string(),
                ]],
                tokens,
            )
            .mount();
            let data_tbl: NodeId = EuvDataTable::new(
                vec![
                    "Name".to_string(),
                    "Role".to_string(),
                    "Status".to_string(),
                    "Score".to_string(),
                ],
                vec![vec![
                    "Engine".to_string(),
                    "Core".to_string(),
                    "Active".to_string(),
                    "120".to_string(),
                ]],
                tokens,
            )
            .mount();
            let mut area: EuvScrollArea = EuvScrollArea::new(110.0, tokens);
            for i in 0..8usize {
                let item: NodeId = view! {
                    text {
                        font_size: 13.0
                        color: tokens.get_palette().get_foreground()
                        { format!("ScrollItem-{i}-wide-content-line-for-horizontal-clamp-check") }
                    }
                };
                area = area.child(item);
            }
            let scroll_area: NodeId = area.mount();
            let ctx: Context = current_context();
            ctx.get_scene().with_read(
                |tree: &SceneTree| {
                    fn find_first_scroll(tree: &SceneTree, root_id: NodeId) -> NodeId {
                        let mut stack: Vec<NodeId> = vec![root_id];
                        while let Some(cur) = stack.pop() {
                            if let Some(n) = tree.get_node(cur) {
                                if n.style_ref().get_layout().get_overflow() == Overflow::Scroll {
                                    return cur;
                                }
                                for &ch in n.get_children().iter().rev() {
                                    stack.push(ch);
                                }
                            }
                        }
                        NodeId::default()
                    }
                    code_a_c.set(find_first_scroll(tree, code_a));
                    code_b_c.set(find_first_scroll(tree, code_b));
                    table_c.set(find_first_scroll(tree, md_table));
                    vscroll_c.set(find_first_scroll(tree, scroll_area));
                },
                (),
            );
            view! {
                column {
                    width: Length::Percent(100.0)
                    gap: SPACE_SM
                    { code_a }
                    { code_b }
                    { md_table }
                    { data_tbl }
                    { scroll_area }
                }
            }
        })
        .route("/page-b", "PageB", move |_: &EuvRouteMatch| {
            view! {
                column {
                    width: Length::Percent(100.0)
                    text {
                        font_size: 14.0
                        color: tokens.get_palette().get_foreground()
                        { "Page B" }
                    }
                }
            }
        });
    let r_mount: EuvRouter = router.clone();
    let _: NodeId = app.mount(move || {
        r_mount
            .outlet(move |_: &EuvRouteMatch| view! { column {} })
            .with_route_order(vec!["/page-a".to_string(), "/page-b".to_string()])
            .mount()
    });
    let _: FrameStats = app.step_frame().unwrap();
    let code_a_scroll: NodeId = code_a_scroll_ref.get();
    let code_b_scroll: NodeId = code_b_scroll_ref.get();
    let table_scroll: NodeId = table_scroll_ref.get();
    let vscroll_node: NodeId = vscroll_ref.get();
    assert_ne!(code_a_scroll, NodeId::default());
    assert_ne!(code_b_scroll, NodeId::default());
    assert_ne!(table_scroll, NodeId::default());
    assert_ne!(vscroll_node, NodeId::default());
    let (stops_ok, code_a_pt, vscroll_pt): (bool, Point, Point) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let stop_a: bool = tree
                .get_node(code_a_scroll)
                .map(|n: &SceneNode| n.style_ref().get_paint().get_stop_scroll_propagation())
                .unwrap_or(false);
            let stop_b: bool = tree
                .get_node(code_b_scroll)
                .map(|n: &SceneNode| n.style_ref().get_paint().get_stop_scroll_propagation())
                .unwrap_or(false);
            let stop_t: bool = tree
                .get_node(table_scroll)
                .map(|n: &SceneNode| n.style_ref().get_paint().get_stop_scroll_propagation())
                .unwrap_or(false);
            let stop_v: bool = tree
                .get_node(vscroll_node)
                .map(|n: &SceneNode| n.style_ref().get_paint().get_stop_scroll_propagation())
                .unwrap_or(false);
            let ca_b: Rect = tree.node_screen_bounds(code_a_scroll).unwrap_or_default();
            let vs_b: Rect = tree.node_screen_bounds(vscroll_node).unwrap_or_default();
            (
                stop_a && stop_b && stop_t && stop_v,
                Point::new(
                    ca_b.get_x() + ca_b.get_width() * 0.5,
                    ca_b.get_y() + ca_b.get_height() * 0.5,
                ),
                Point::new(
                    vs_b.get_x() + vs_b.get_width() * 0.5,
                    vs_b.get_y() + vs_b.get_height() * 0.5,
                ),
            )
        },
        (false, Point::default(), Point::default()),
    );
    assert!(stops_ok);
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(code_a_pt.get_x(), code_a_pt.get_y()),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(80.0, 0.0)));
    let _: FrameStats = app.step_frame().unwrap();
    let (ax1, bx1, tx1): (f32, f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let ax: f32 = tree
                .get_node(code_a_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            let bx: f32 = tree
                .get_node(code_b_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            let tx: f32 = tree
                .get_node(table_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            (ax, bx, tx)
        },
        (-1.0, -1.0, -1.0),
    );
    assert!(ax1 > 10.0);
    assert!(bx1.abs() < 0.01);
    assert!(tx1.abs() < 0.01);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::touch(9, code_a_pt.get_x(), code_a_pt.get_y()),
    ));
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::touch(9, code_a_pt.get_x() - 60.0, code_a_pt.get_y()),
    ));
    let swipe_during_code_drag: f32 = app.get_context().get_swipe_drag_offset().get_untracked();
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::touch(9, code_a_pt.get_x() - 60.0, code_a_pt.get_y()),
    ));
    let _: FrameStats = app.step_frame().unwrap();
    assert!(swipe_during_code_drag.abs() < 0.01);
    let (ax_touch, bx_touch): (f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let ax: f32 = tree
                .get_node(code_a_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            let bx: f32 = tree
                .get_node(code_b_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            (ax, bx)
        },
        (-1.0, -1.0),
    );
    assert!(ax_touch > ax1);
    assert!(bx_touch.abs() < 0.01);
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(vscroll_pt.get_x(), vscroll_pt.get_y()),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(60.0, 20.0)));
    let _: FrameStats = app.step_frame().unwrap();
    let vx: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(vscroll_node)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    assert!(vx.abs() < 0.01);
    for _ in 0..45 {
        let _: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    }
    let ax_before_swipe: f32 = app.get_scene().with_read(
        |tree: &SceneTree| {
            tree.get_node(code_a_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0)
        },
        -1.0,
    );
    let _: bool = app.get_context().get_swipe_drag_offset().set(-120.0);
    let _: FrameStats = app.step_frame().unwrap();
    let _: bool = app.get_context().get_swipe_drag_offset().set(0.0);
    let _: FrameStats = app.step_frame().unwrap();
    let (ax2, bx2): (f32, f32) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let ax: f32 = tree
                .get_node(code_a_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            let bx: f32 = tree
                .get_node(code_b_scroll)
                .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_x())
                .unwrap_or(-1.0);
            (ax, bx)
        },
        (-1.0, -1.0),
    );
    assert!((ax2 - ax_before_swipe).abs() < 0.01);
    assert!(bx2.abs() < 0.01);
}

#[test]
fn auto_fill_wrapped_button_row_packs_and_fills_every_row_without_trailing_gap() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(720, 640).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let tokens: ThemeTokens = ThemeTokens::resolve(DesignScheme::Shadcn, ThemeMode::Light);
    let narrow_row_ref: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let narrow_btns_ref: Rc<RefCell<Vec<NodeId>>> = Rc::new(RefCell::new(Vec::new()));
    let wide_btns_ref: Rc<RefCell<Vec<NodeId>>> = Rc::new(RefCell::new(Vec::new()));
    let group_btns_ref: Rc<RefCell<Vec<NodeId>>> = Rc::new(RefCell::new(Vec::new()));
    let probe_b_ref: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let nr_c: Rc<Cell<NodeId>> = Rc::clone(&narrow_row_ref);
    let nb_c: Rc<RefCell<Vec<NodeId>>> = Rc::clone(&narrow_btns_ref);
    let wb_c: Rc<RefCell<Vec<NodeId>>> = Rc::clone(&wide_btns_ref);
    let gb_c: Rc<RefCell<Vec<NodeId>>> = Rc::clone(&group_btns_ref);
    let pb_c: Rc<Cell<NodeId>> = Rc::clone(&probe_b_ref);
    let _: NodeId = app.mount(move || {
        let labels: [&str; 5] = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon"];
        let mut narrow_btns: Vec<NodeId> = Vec::with_capacity(5);
        for lbl in labels.iter() {
            narrow_btns.push(
                EuvButton::new((*lbl).to_string(), tokens)
                    .min_width(110.0)
                    .mount(),
            );
        }
        *nb_c.borrow_mut() = narrow_btns.clone();
        let mut action_narrow: EuvActionRow = EuvActionRow::default();
        for &cid in narrow_btns.iter() {
            action_narrow = action_narrow.child(cid);
        }
        let narrow_row_id: NodeId = action_narrow.mount();
        nr_c.set(narrow_row_id);
        let mut group_btns: Vec<NodeId> = Vec::with_capacity(5);
        for lbl in labels.iter() {
            group_btns.push(
                EuvButton::new((*lbl).to_string(), tokens)
                    .min_width(110.0)
                    .mount(),
            );
        }
        *gb_c.borrow_mut() = group_btns.clone();
        let group_id: NodeId = EuvButtonGroup::new(group_btns, tokens)
            .wrap_fill(true)
            .mount();
        let mut wide_btns: Vec<NodeId> = Vec::with_capacity(5);
        for lbl in labels.iter() {
            wide_btns.push(
                EuvButton::new((*lbl).to_string(), tokens)
                    .min_width(110.0)
                    .mount(),
            );
        }
        *wb_c.borrow_mut() = wide_btns.clone();
        let mut action_wide: EuvActionRow = EuvActionRow::default();
        for &cid in wide_btns.iter() {
            action_wide = action_wide.child(cid);
        }
        let wide_row_id: NodeId = action_wide.mount();
        let mut probe_b: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_MD
                column {
                    width: Length::Px(360.0)
                    { narrow_row_id }
                }
                column {
                    width: Length::Px(360.0)
                    { group_id }
                }
                column {
                    width: Length::Px(720.0)
                    { wide_row_id }
                }
                row {
                    width: Length::Px(360.0)
                    flex_wrap: FlexWrap::Wrap
                    gap: 8.0
                    view {
                        min_width: Length::Px(100.0)
                        height: Length::Px(24.0)
                        flex_grow: 1.0
                    }
                    view {
                        node_ref: probe_b
                        min_width: Length::Px(100.0)
                        height: Length::Px(24.0)
                        flex_grow: 1.0
                    }
                }
            }
        };
        pb_c.set(probe_b);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let narrow_row_id: NodeId = narrow_row_ref.get();
    let narrow_ids: Vec<NodeId> = narrow_btns_ref.borrow().clone();
    let wide_ids: Vec<NodeId> = wide_btns_ref.borrow().clone();
    let group_ids: Vec<NodeId> = group_btns_ref.borrow().clone();
    let probe_b_id: NodeId = probe_b_ref.get();
    let (row_style_ok, narrow_rects, wide_rects, group_rects, core_wrap_grows): (
        bool,
        Vec<Rect>,
        Vec<Rect>,
        Vec<Rect>,
        bool,
    ) = app.get_scene().with_read(
        |tree: &SceneTree| {
            let row_ok: bool = tree
                .get_node(narrow_row_id)
                .map(|n: &SceneNode| {
                    let lay: LayoutStyle = n.style_ref().get_layout();
                    lay.get_flex_direction() == FlexDirection::Row
                        && lay.get_flex_wrap() == FlexWrap::Wrap
                        && lay.get_width() == Length::Percent(100.0)
                })
                .unwrap_or(false);
            let all_grow: bool = narrow_ids
                .iter()
                .chain(group_ids.iter())
                .all(|id: &NodeId| {
                    tree.get_node(*id)
                        .map(|n: &SceneNode| {
                            let lay: LayoutStyle = n.style_ref().get_layout();
                            (lay.get_flex_grow() - 1.0).abs() < 0.01
                                && lay.get_min_width() == Length::Px(110.0)
                        })
                        .unwrap_or(false)
                });
            let n_r: Vec<Rect> = narrow_ids
                .iter()
                .map(|id: &NodeId| tree.node_screen_bounds(*id).unwrap_or_default())
                .collect();
            let w_r: Vec<Rect> = wide_ids
                .iter()
                .map(|id: &NodeId| tree.node_screen_bounds(*id).unwrap_or_default())
                .collect();
            let g_r: Vec<Rect> = group_ids
                .iter()
                .map(|id: &NodeId| tree.node_screen_bounds(*id).unwrap_or_default())
                .collect();
            let probe_r: Rect = tree.node_screen_bounds(probe_b_id).unwrap_or_default();
            let wrap_grow_active: bool =
                (probe_r.get_x() + probe_r.get_width() - 360.0).abs() <= 1.0;
            (row_ok && all_grow, n_r, w_r, g_r, wrap_grow_active)
        },
        (false, Vec::new(), Vec::new(), Vec::new(), false),
    );
    assert!(row_style_ok);
    assert_eq!(narrow_rects.len(), 5);
    assert_eq!(wide_rects.len(), 5);
    assert_eq!(group_rects.len(), 5);
    let y0: f32 = narrow_rects[0].get_y();
    assert!((narrow_rects[1].get_y() - y0).abs() < 1.0);
    assert!((narrow_rects[2].get_y() - y0).abs() < 1.0);
    let y1: f32 = narrow_rects[3].get_y();
    assert!(y1 > y0 + 20.0);
    assert!((narrow_rects[4].get_y() - y1).abs() < 1.0);
    let row1_right: f32 = narrow_rects[2].get_x() + narrow_rects[2].get_width();
    let row2_right: f32 = narrow_rects[4].get_x() + narrow_rects[4].get_width();
    assert!(row1_right <= 360.5);
    assert!(row2_right <= 360.5);
    let wy0: f32 = wide_rects[0].get_y();
    for wr in wide_rects.iter() {
        assert!((wr.get_y() - wy0).abs() < 1.0);
    }
    assert!(core_wrap_grows);
    assert!((row1_right - 360.0).abs() <= 1.0);
    assert!((row2_right - 360.0).abs() <= 1.0);
    let g_r1_right: f32 = group_rects[2].get_x() + group_rects[2].get_width();
    let g_r2_right: f32 = group_rects[4].get_x() + group_rects[4].get_width();
    assert!((g_r1_right - 356.0).abs() <= 2.0);
    assert!((g_r2_right - 356.0).abs() <= 2.0);
}
