use super::*;

tokens! {
    const TEST_BG: Color = Color::hex(0x0F172A);
    const TEST_FG: Color = Color::hex(0xF8FAFC);
}

style! {
    c_card {
        direction: column;
        padding: Edges::all(12.0);
        gap: 8.0;
        radius: 8.0;
        align: AlignItems::Start;
        justify: JustifyContent::Start;
        background: TEST_BG;
        border: (1.0, TEST_FG);
    }
    c_panel {
        direction: column;
        padding: Edges::all(12.0);
        gap: 8.0;
        radius: 8.0;
        overflow: Overflow::Hidden;
        background: TEST_BG;
        border: (1.0, TEST_FG);
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
        let panel_style: Style = Style::empty()
            .column()
            .padding(Edges::all(12.0))
            .gap(8.0)
            .radius(8.0)
            .overflow(Overflow::Hidden)
            .background(TEST_BG)
            .border(1.0, TEST_FG)
            .width(Length::Px(240.0));
        let mut ctx: BuildContext = current_build_context();
        let col: NodeId = ctx.view(panel_style);
        let txt: NodeId = ctx.text(
            "Native Macro Parity".to_string(),
            Style::empty().color(TEST_FG),
        );
        ctx.append(col, txt);
        col
    });
    let stats_macro: FrameStats = app_macro.step_frame().unwrap();
    let stats_builder: FrameStats = app_builder.step_frame().unwrap();
    assert_ne!(stats_macro.get_frame_hash(), 0);
    assert_eq!(stats_macro.get_frame_hash(), stats_builder.get_frame_hash());
}
