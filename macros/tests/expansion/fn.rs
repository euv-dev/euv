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
        background: TEST_BG;
        border: (1.0, TEST_FG);
    }
}

#[component]
fn demo_badge(ctx: &mut BuildContext) -> NodeId {
    view!(ctx, {
        row {
            padding: Edges::symmetric(4.0, 8.0)
            background: TEST_BG
            radius: 6.0
            text {
                color: TEST_FG
                "Native"
            }
        }
    })
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
    let cfg: AppConfig = AppConfig::default().with_window(
        WindowConfig::default()
            .with_size(320, 200)
            .with_render(RenderConfig::software()),
    );
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|ctx: &mut BuildContext| {
        view!(ctx, {
            column {
                style: c_card()
                width: Length::Px(300.0)
                text {
                    color: TEST_FG
                    { move || format!("Value: {}", doubled.get()) }
                }
                demo_badge {}
            }
        })
    });
    let stats: FrameStats = app.step_frame().unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 3);
}
