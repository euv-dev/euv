use super::*;

struct TestBounceScene {
    x: f32,
    vx: f32,
    steps: u32,
}

impl GameScene for TestBounceScene {
    fn fixed_update(&mut self, dt: f32, input: &InputState) {
        let dir: f32 = input.axis(KeyCode::ArrowLeft, KeyCode::ArrowRight);
        self.x += (self.vx + dir * 120.0) * dt;
        self.steps += 1;
    }

    fn render(&mut self, _alpha: f32, ctx: &mut GameRenderContext<'_>) {
        ctx.clear(Color::hex(0x0B1120));
        ctx.fill_rounded_rect(
            Rect::new(self.x, 40.0, 28.0, 28.0),
            Color::hex(0x38BDF8),
            6.0,
        );
        ctx.fill_circle(Point::new(self.x + 14.0, 96.0), 12.0, Color::hex(0x34D399));
        let sprite: ImageBuffer = ImageBuffer::new(2, 2, vec![255; 16]);
        ctx.blit_framebuffer(Rect::new(12.0, 12.0, 24.0, 24.0), sprite);
    }
}

#[test]
fn game_engine_host_fixed_timestep_input_and_viewport_rendering_work() {
    reset_reactive_runtime();
    let host: GameEngineHost = GameEngineHost::new(
        TestBounceScene {
            x: 16.0,
            vx: 60.0,
            steps: 0,
        },
        1.0 / 60.0,
    );
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 200).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win).set_clear_color(COLOR_TRANSPARENT);
    let mut app: App = App::headless(cfg).unwrap();
    let host_mount: GameEngineHost = host.clone();
    let _: NodeId = app.mount(move || {
        host_mount.mount_viewport(
            Style::empty()
                .width(Length::Px(320.0))
                .height(Length::Px(200.0)),
        )
    });
    let frame_a: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let right_down: WindowEvent = WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::ArrowRight, Modifiers::default(), false),
    );
    host.ingest_event(&right_down);
    app.dispatch_window_event(right_down);
    assert!(app.get_input_state().is_key_down(KeyCode::ArrowRight));
    let fixed_steps: u32 = host.step(0.051);
    assert_eq!(fixed_steps, 3);
    let frame_b: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_ne!(frame_a.get_frame_hash(), 0);
    assert_ne!(frame_b.get_frame_hash(), 0);
    assert_ne!(frame_a.get_frame_hash(), frame_b.get_frame_hash());
}

#[test]
fn scroll_container_clamps_offset_and_dispatches_on_scroll_callback() {
    reset_reactive_runtime();
    let scrolled_dy: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
    let scrolled_c: Rc<Cell<f32>> = scrolled_dy.clone();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(300, 160).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let mut scroll_id: NodeId = NodeId::default();
    let _: NodeId = app.mount(|| {
        let mut ctx: BuildContext = current_build_context();
        scroll_id = ctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Px(300.0))
                .height(Length::Px(120.0)),
        );
        ctx.on_scroll(scroll_id, move |ev: &UiEvent| {
            if let UiEventKind::Scroll(delta) = ev.get_kind() {
                scrolled_c.set(scrolled_c.get() + delta.get_y());
            }
        });
        for i in 0..20_u32 {
            let row: NodeId = ctx.text(
                format!("Row #{i}"),
                Style::empty()
                    .width(Length::Px(280.0))
                    .height(Length::Px(24.0)),
            );
            ctx.append(scroll_id, row);
        }
        scroll_id
    });
    let frame_top: FrameStats = app.step_frame().unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(40.0, 40.0),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 96.0)));
    let frame_scrolled: FrameStats = app.step_frame().unwrap();
    assert!((scrolled_dy.get() - 96.0).abs() < 0.01);
    assert_ne!(frame_top.get_frame_hash(), frame_scrolled.get_frame_hash());
}
