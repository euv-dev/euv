use super::*;

tokens! {
    const TEST_BG: Color = Color::hex(0x0F172A);
    const TEST_FG: Color = Color::hex(0xF8FAFC);
}

style! {
    c_card {
        direction: column
        padding: Edges::all(12.0)
        gap: 8.0
        radius: 8.0
        align: AlignItems::Start
        justify: JustifyContent::Start
        background: TEST_BG
        border: (1.0, TEST_FG)
    }
    c_panel {
        direction: column
        padding: Edges::all(12.0)
        gap: 8.0
        radius: 8.0
        overflow: Overflow::Hidden
        background: TEST_BG
        border: (1.0, TEST_FG)
    }
}

fn demo_badge() -> NodeId {
    view! {
        row {
            padding: Edges::symmetric(4.0, 8.0)
            background: TEST_BG
            radius: 6.0
            text {
                color: TEST_FG
                "Native"
            }
        }
    }
}

#[test]
fn view_style_tokens_watch_and_computed_macros_work_end_to_end() {
    reset_reactive_runtime();
    let count: Signal<i32> = Signal::new(3);
    let doubled: Memo<i32> = computed!((count) -> i32 => move |n: i32| n * 2);
    let observed: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let observed_c: Rc<Cell<i32>> = observed.clone();
    let _eff: Effect = watch!((doubled) => move |v: i32| {
        observed_c.set(v);
    });
    assert_eq!(observed.get(), 6);
    let _: bool = count.set(5);
    assert_eq!(observed.get(), 10);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 200).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        view! {
            column {
                style: c_card()
                width: Length::Px(300.0)
                text {
                    color: TEST_FG
                    { move || format!("Value: {}", doubled.get()) }
                }
                demo_badge {}
            }
        }
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 3);
}

#[test]
fn style_macro_supports_full_layout_rules_and_matches_builder_api() {
    reset_reactive_runtime();
    let mut win_a: WindowConfig = WindowConfig::default();
    win_a
        .set_size(280, 160)
        .set_render(RenderConfig::software());
    let mut cfg_a: AppConfig = AppConfig::default();
    cfg_a.set_window(win_a);
    let cfg_b: AppConfig = cfg_a.clone();
    let mut app_macro: App = App::headless(cfg_a).unwrap();
    let _: NodeId = app_macro.mount(|| {
        view! {
            column {
                style: c_panel()
                width: Length::Px(240.0)
                text {
                    color: TEST_FG
                    "Native Macro Parity"
                }
            }
        }
    });
    let mut app_builder: App = App::headless(cfg_b).unwrap();
    let _: NodeId = app_builder.mount(|| {
        let panel_style: Style = Style::default()
            .column()
            .padding(Edges::all(12.0))
            .gap(8.0)
            .radius(8.0)
            .overflow(Overflow::Hidden)
            .background(TEST_BG)
            .border(1.0, TEST_FG)
            .width(Length::Px(240.0));
        let mut ctx: Context = current_context();
        let col: NodeId = ctx.view(panel_style);
        let txt: NodeId = ctx.text(
            "Native Macro Parity".to_string(),
            Style::default().color(TEST_FG),
        );
        ctx.append(col, txt);
        col
    });
    let stats_macro: FrameStats = app_macro.step_frame().unwrap();
    let stats_builder: FrameStats = app_builder.step_frame().unwrap();
    assert_ne!(stats_macro.get_frame_hash(), 0);
    assert_eq!(stats_macro.get_frame_hash(), stats_builder.get_frame_hash());
}

#[test]
fn view_slot_and_page_slot_macros_support_live_component_and_page_updates() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(400, 260).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let hub: DynamicUpdateHub = app.get_dynamic_hub().clone();
    let _: NodeId = app.mount(|| {
        view! {
            column {
                width: Length::Percent(100.0)
                gap: 8.0
                slot {
                    id: "macro_banner"
                    text {
                        color: TEST_FG
                        "Macro Fallback Banner"
                    }
                }
                page_slot {
                    route: "/macro-page"
                    text {
                        color: TEST_FG
                        "Macro Fallback Page"
                    }
                }
            }
        }
    });
    let f0: FrameStats = app.step_frame().unwrap();
    assert_ne!(f0.get_frame_hash(), 0);
    let mut comp_upd: DynamicComponentUpdate = DynamicComponentUpdate::new(
        "macro_banner".to_string(),
        1,
        DynamicNodeSpec::badge("Live Macro Banner v1".to_string()),
    );
    comp_upd.sign();
    assert_eq!(hub.apply_component(comp_upd).unwrap(), 1);
    let f1: FrameStats = app.step_frame().unwrap();
    assert_ne!(f1.get_frame_hash(), f0.get_frame_hash());
    let mut page_upd: DynamicPageUpdate = DynamicPageUpdate::new(
        "/macro-page".to_string(),
        "Macro Page v1".to_string(),
        1,
        DynamicNodeSpec::card("Live Macro Page v1".to_string()),
    );
    page_upd.sign();
    assert_eq!(hub.apply_page(page_upd).unwrap(), 1);
    let f2: FrameStats = app.step_frame().unwrap();
    assert_ne!(f2.get_frame_hash(), f1.get_frame_hash());
}

style! {
    c_extended {
        direction: row
        min_width: Length::Px(120.0)
        min_height: Length::Px(40.0)
        max_height: Length::Px(160.0)
        flex_shrink: 0.0
        aspect_ratio: 2.0
        cursor: CursorIcon::Pointer
        text_align: TextAlign::Center
        font_family: FontFamily::Monospace
        gradient: LinearGradient::new(90.0, TEST_BG, TEST_FG)
        shadow: Shadow::new(0.0, 2.0, 6.0, 0.0, Color::rgba(0, 0, 0, 40))
    }
}

#[test]
fn reactive_shorthand_and_extended_view_style_macros_work() {
    reset_reactive_runtime();
    let count: Signal<i32> = Signal::new(4);
    let tripled: Memo<i32> = computed!(move || count.get() * 3);
    let seen: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let seen_c: Rc<Cell<i32>> = seen.clone();
    let _eff: Effect = watch!(move || {
        seen_c.set(tripled.get());
    });
    assert_eq!(seen.get(), 12);
    let _: bool = count.set(7);
    assert_eq!(seen.get(), 21);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 180).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        view! {
            column {
                style: c_extended()
                min_width: Length::Px(160.0)
                flex_shrink: 0.0
                cursor: CursorIcon::Pointer
                on_hover: |_ev: &UiEvent| {}
                on_key: |_ev: &UiEvent| {}
                text {
                    font_family: FontFamily::Monospace
                    text_align: TextAlign::Center
                    color: TEST_FG
                    "Shorthand Ready"
                }
            }
        }
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
}

style! {
    c_param_pill(bg: Color, radius: f32) {
        direction: row
        padding: Edges::symmetric(6.0, 12.0)
        background: bg
        radius: radius
        pointer_events: PointerEvents::Auto
        stop_scroll_propagation: true
        stop_event_propagation: true
    }
}

#[derive(Props)]
struct MockCard {
    title: String,
    #[prop(custom)]
    subtitle: Option<String>,
    #[prop(custom)]
    elevated: bool,
    #[prop(custom)]
    children: Vec<NodeId>,
}

impl MockCard {
    fn elevated(mut self) -> Self {
        self.elevated = true;
        self
    }

    fn subtitle(mut self, sub: String) -> Self {
        self.subtitle = Some(sub);
        self
    }

    fn tag_pair(mut self, prefix: &str, suffix: &str) -> Self {
        self.subtitle = Some(format!("{prefix}-{suffix}"));
        self
    }

    fn child(mut self, id: NodeId) -> Self {
        self.children.push(id);
        self
    }
}

impl Lifecycle for MockCard {}

impl Component for MockCard {
    fn render(&self) -> NodeId {
        let title: String = self.title.clone();
        let subtitle: Option<String> = self.subtitle.clone();
        let elevated: bool = self.elevated;
        let children: Vec<NodeId> = self.children.clone();
        view! {
            column {
                padding: Edges::all(12.0)
                gap: 6.0
                background: TEST_BG
                radius: if elevated { 12.0 } else { 6.0 }
                text {
                    color: TEST_FG
                    { title }
                }
                if let Some(sub) = subtitle {
                    text {
                        color: TEST_FG
                        { sub }
                    }
                }
                ..children
            }
        }
    }
}

#[test]
fn view_v2_control_flow_spread_components_fragments_and_parity_work() {
    reset_reactive_runtime();
    let mut win_a: WindowConfig = WindowConfig::default();
    win_a
        .set_size(360, 520)
        .set_render(RenderConfig::software());
    let mut cfg_a: AppConfig = AppConfig::default();
    cfg_a.set_window(win_a);
    let cfg_b: AppConfig = cfg_a.clone();
    let mut app_macro: App = App::headless(cfg_a).unwrap();
    let active: Signal<bool> = Signal::new(false);
    let active_s: Signal<bool> = active;
    let active_t: Signal<bool> = active;
    let items_sig: Signal<Vec<&'static str>> = Signal::new(vec!["One", "Two"]);
    let items_c: Signal<Vec<&'static str>> = items_sig;
    let captured_input: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let captured_card: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let cap_in_c: Rc<Cell<NodeId>> = captured_input.clone();
    let cap_card_c: Rc<Cell<NodeId>> = captured_card.clone();
    let _: NodeId = app_macro.mount(move || {
        let tags: [&str; 2] = ["Alpha", "Beta"];
        let mode: u8 = 2;
        let extra: Vec<NodeId> = vec![view! {
            text {
                color: TEST_FG
                "SpreadItem"
            }
        }];
        let sibling_after_bare_comp: NodeId = view! {
            text {
                color: TEST_FG
                "AfterBareComp"
            }
        };
        let mut local_input: NodeId = NodeId::default();
        let mut local_card: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                width: Length::Px(340.0)
                gap: 8.0
                MockCard {
                    title: "Bare Sibling"
                }
                { sibling_after_bare_comp }
                MockCard {
                    title: "Macro Card"
                    elevated()
                    tag_pair("Typed", "Pair")
                    subtitle: "Typed Sub".to_string()
                    node_ref: local_card
                    for label in tags.iter().copied() {
                        row {
                            style: c_param_pill(TEST_BG, 8.0)
                            text {
                                color: TEST_FG
                                { label }
                            }
                        }
                    }
                    match mode {
                        1 => text { color: TEST_FG "Mode1" },
                        2 => fragment {
                            text { color: TEST_FG "Mode2A" }
                            text { color: TEST_FG "Mode2B" }
                        },
                        _ => text { color: TEST_FG "Other" },
                    }
                    if true {
                        ..extra
                    }
                }
                row {
                    bind_style: move || {
                        Style::default()
                        .row()
                        .padding(Edges::all(4.0))
                        .background(if active_s.get() { TEST_FG } else { TEST_BG })
                    }
                    bind_children: move || {
                        items_c
                        .get()
                        .into_iter()
                        .map(|label: &str| {
                            view! {
                                text {
                                    color: TEST_FG
                                    { label }
                                }
                            }
                        })
                        .collect()
                    }
                }
                text {
                    bind_style: move || {
                        Style::default().color(if active_t.get() { TEST_BG } else { TEST_FG })
                    }
                    { move || format!("Active={}", active_t.get()) }
                }
                input {
                    node_ref: local_input
                    value: "hello"
                    placeholder: "type..."
                    password: true
                    multiline: false
                    disabled: false
                    caret: 2usize
                    focusable: true
                    role: A11yRole::TextInput
                    a11y_label: "Secret"
                    a11y_hint: "Enter secret"
                    a11y_value: "hello"
                    a11y_disabled: false
                    a11y_checked: Some(true)
                    on_pointer_enter: |_ev: &UiEvent| {}
                    on_pointer_leave: |_ev: &UiEvent| {}
                    on_focus: |_ev: &UiEvent| {}
                    on_blur: |_ev: &UiEvent| {}
                }
            }
        };
        cap_in_c.set(local_input);
        cap_card_c.set(local_card);
        root
    });
    assert_ne!(captured_input.get(), NodeId::default());
    assert_ne!(captured_card.get(), NodeId::default());
    let f0: FrameStats = app_macro.step_frame().unwrap();
    assert_ne!(f0.get_frame_hash(), 0);
    let _: bool = active.set(true);
    let _: bool = items_sig.set(vec!["One", "Two", "Three"]);
    let f1: FrameStats = app_macro.step_frame().unwrap();
    assert_ne!(f1.get_frame_hash(), f0.get_frame_hash());
    let mut app_in_ctx: App = App::headless(cfg_b.clone()).unwrap();
    let _: NodeId = app_in_ctx.mount(|| {
        view! {
            row {
                padding: Edges::all(8.0)
                gap: 6.0
                background: TEST_BG
                inset: (Some(0.0), None, None, Some(0.0))
                text { color: TEST_FG "A" }
                text { color: TEST_FG "B" }
            }
        }
    });
    let mut app_builder: App = App::headless(cfg_b).unwrap();
    let _: NodeId = app_builder.mount(|| {
        let mut ctx: Context = current_context();
        let row_id: NodeId = ctx.view(
            Style::default()
                .row()
                .padding(Edges::all(8.0))
                .gap(6.0)
                .background(TEST_BG)
                .inset(Some(0.0), None, None, Some(0.0)),
        );
        let a: NodeId = ctx.text("A".to_string(), Style::default().color(TEST_FG));
        let b: NodeId = ctx.text("B".to_string(), Style::default().color(TEST_FG));
        ctx.append_many(row_id, &[a, b]);
        row_id
    });
    let f2: FrameStats = app_in_ctx.step_frame().unwrap();
    let f3: FrameStats = app_builder.step_frame().unwrap();
    assert_ne!(f2.get_frame_hash(), 0);
    assert_eq!(f2.get_frame_hash(), f3.get_frame_hash());
}

#[event]
#[derive(Debug, PartialEq)]
struct MacroTestEvent {
    value: u32,
}

#[global_state_store]
#[derive(Debug, Default, PartialEq)]
struct MacroTestStore {
    total: u32,
}

#[derive(Clone)]
struct MacroFreezeCard {
    mounted_hits: Rc<Cell<u32>>,
    frozen_hits: Rc<Cell<u32>>,
    resumed_hits: Rc<Cell<u32>>,
}

#[component(retention = Freeze)]
impl MacroFreezeCard {
    fn on_mount(&self, _node: NodeId) {
        self.mounted_hits.set(self.mounted_hits.get() + 1);
    }

    fn on_freeze(&self, _node: NodeId) {
        self.frozen_hits.set(self.frozen_hits.get() + 1);
    }

    fn on_resume(&self, _node: NodeId) {
        self.resumed_hits.set(self.resumed_hits.get() + 1);
    }

    fn render(&self) -> NodeId {
        let card_style: Style = style! {
            FlexDirection::Column
            AlignItems::Center
            Overflow::Hidden
            width: Length::Px(180.0)
            padding: Edges::all(12.0)
            background: TEST_BG
        };
        view! {
            view {
                style: card_style
                text {
                    color: TEST_FG
                    "Macro Component"
                }
            }
        }
    }
}

#[test]
fn inline_style_event_global_local_state_and_component_macros_work_end_to_end() {
    reset_reactive_runtime();
    let inline_st: Style = style! {
        AlignItems::Center
        Overflow::Hidden
        FlexDirection::Row
        width: Length::Px(120.0)
    };
    assert_eq!(inline_st.get_layout().get_align_items(), AlignItems::Center);
    assert_eq!(inline_st.get_layout().get_overflow(), Overflow::Hidden);
    assert_eq!(
        inline_st.get_layout().get_flex_direction(),
        FlexDirection::Row
    );
    assert_eq!(inline_st.get_layout().get_width(), Length::Px(120.0));
    let store_sig: Signal<MacroTestStore> =
        global_state!(MacroTestStore = MacroTestStore { total: 5 });
    let watched_total: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let watched_total_c: Rc<Cell<u32>> = watched_total.clone();
    let _g_watch: Option<Effect> = watch!(global MacroTestStore => move |_prev: &MacroTestStore, next: &MacroTestStore| {
        watched_total_c.set(next.total);
    });
    let _ev_sub: Effect = on_event!(MacroTestEvent => move |ev: &MacroTestEvent| {
        let _: bool = store_sig.update(|s: &mut MacroTestStore| s.total += ev.value);
    });
    let local_num: LocalState<u32> = local_state!(10_u32);
    emit!(MacroTestEvent {
        value: local_num.get_untracked(),
    });
    assert_eq!(store_sig.get_untracked().total, 15);
    assert_eq!(watched_total.get(), 15);
    let mounted_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let frozen_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let resumed_hits: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let card: MacroFreezeCard = MacroFreezeCard {
        mounted_hits: mounted_hits.clone(),
        frozen_hits: frozen_hits.clone(),
        resumed_hits: resumed_hits.clone(),
    };
    assert_eq!(card.retention_policy(), PageRetention::Freeze);
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 200).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let card_c: MacroFreezeCard = card.clone();
    let root_id: NodeId = app.mount(move || card_c.mount());
    assert_eq!(mounted_hits.get(), 1);
    app.get_scene().freeze_subtree(root_id);
    assert_eq!(frozen_hits.get(), 1);
    app.get_scene().resume_subtree(root_id);
    assert_eq!(resumed_hits.get(), 1);
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
}

#[test]
fn selectable_and_selection_color_work_in_style_and_view_macros() {
    reset_reactive_runtime();
    let tint: Color = Color::rgba(37, 99, 235, 96);
    let styled: Style = style! {
        selectable: true
        selection_color: tint
    };
    assert!(styled.get_text().get_selectable());
    assert_eq!(styled.get_text().selection_color(), Some(tint));
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 200).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let captured_txt: Rc<Cell<NodeId>> = Rc::new(Cell::new(NodeId::default()));
    let cap_c: Rc<Cell<NodeId>> = captured_txt.clone();
    let _: NodeId = app.mount(move || {
        let mut txt_id: NodeId = NodeId::default();
        let root: NodeId = view! {
            column {
                selectable: true
                selection_color: tint
                text {
                    node_ref: txt_id
                    color: TEST_FG
                    "Selectable Macro Text"
                }
            }
        };
        cap_c.set(txt_id);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let target: NodeId = captured_txt.get();
    let is_sel: bool = app
        .get_scene()
        .with_read(|tree: &SceneTree| tree.is_node_selectable(target), false);
    assert!(is_sel);
}
