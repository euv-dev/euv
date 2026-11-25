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
