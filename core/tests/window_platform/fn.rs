use super::*;

#[test]
fn all_seven_platforms_expose_accurate_capabilities_and_form_factors() {
    let targets: [PlatformKind; 7] = PlatformKind::all_targets();
    assert_eq!(targets.len(), 7);
    assert_eq!(PlatformKind::IpadOs.form_factor(), DeviceFormFactor::Tablet);
    assert_eq!(PlatformKind::Ios.form_factor(), DeviceFormFactor::Phone);
    assert_eq!(PlatformKind::MacOs.form_factor(), DeviceFormFactor::Desktop);
    let ipad_caps: PlatformCapabilities = PlatformCapabilities::for_platform(PlatformKind::IpadOs);
    assert!(ipad_caps.get_multi_window());
    assert!(ipad_caps.get_stylus());
    assert!(ipad_caps.get_hover_pointer());
    assert!(ipad_caps.get_touch());
    assert!(ipad_caps.get_safe_area_insets());
    let ios_caps: PlatformCapabilities = PlatformCapabilities::for_platform(PlatformKind::Ios);
    assert!(!ios_caps.get_multi_window());
    assert!(ios_caps.get_touch());
}

#[test]
fn pointer_click_input_typing_window_commands_and_router_work_end_to_end() {
    reset_reactive_runtime();
    let clicked: Signal<u32> = Signal::new(0);
    let typed: Signal<String> = Signal::new(String::new());
    let mut win: WindowConfig = WindowConfig::default();
    win.set_title("euv test".to_string())
        .set_size(400, 300)
        .set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: Context = current_context();
        let root: NodeId = ctx.view(
            Style::default()
                .column()
                .width(Length::Px(400.0))
                .height(Length::Px(300.0))
                .gap(8.0),
        );
        let btn: NodeId = ctx.view(
            Style::default()
                .width(Length::Px(120.0))
                .height(Length::Px(40.0))
                .background(Color::hex(0x111827)),
        );
        ctx.on_click(btn, move |_: &UiEvent| {
            let _: bool = clicked.update(|v: &mut u32| *v += 1);
        });
        let input: NodeId = ctx.input(
            InputNodeState::new(String::new(), "Type...".to_string(), 0, false, false, false),
            Style::default()
                .width(Length::Px(200.0))
                .height(Length::Px(36.0)),
        );
        ctx.on_change(input, move |ev: &UiEvent| {
            if let UiEventKind::Change(val) = ev.get_kind() {
                let _: bool = typed.set(val.clone());
            }
        });
        ctx.append(root, btn);
        ctx.append(root, input);
        root
    });
    let _: FrameStats = app.step_frame().unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(20.0, 20.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::mouse(20.0, 20.0)));
    app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::mouse(20.0, 20.0)));
    assert_eq!(clicked.get(), 1);
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(20.0, 60.0),
    ));
    app.dispatch_window_event(WindowEvent::TextInput(wid, "Hello".to_string()));
    app.dispatch_window_event(WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::Backspace, Modifiers::default(), false),
    ));
    assert_eq!(typed.get(), "Hell");
    app.queue_window_command(WindowCommand::SetSize(wid, 600, 480));
    let _: FrameStats = app.step_frame().unwrap();
    assert!((app.get_window_state().get_logical_width() - 600.0).abs() < 0.01);
    let mut router: Router = Router::default();
    router.push("/settings".to_string());
    assert_eq!(router.path(), "/settings");
    assert!(router.back());
    assert_eq!(router.path(), "/");
    assert!(router.forward());
    assert_eq!(router.path(), "/settings");
}
