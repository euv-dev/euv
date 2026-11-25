use super::*;

#[test]
fn showcase_mounts_and_renders_all_eight_routes_cleanly() {
    reset_reactive_runtime();
    let routes: [&str; 8] = [
        ROUTE_OVERVIEW,
        ROUTE_REACTIVE,
        ROUTE_RENDER,
        ROUTE_WINDOW,
        ROUTE_FORMS,
        ROUTE_THEMES,
        ROUTE_DATA,
        ROUTE_OVERLAYS,
    ];
    let (mut app, state): (App, ShowcaseState) =
        create_showcase_app(true, RenderMode::Software).unwrap();
    for route in routes.iter() {
        let _: bool = state.get_route().set((*route).to_string());
        let mount_state: ShowcaseState = state.clone();
        let _: NodeId =
            app.mount(move |ctx: &mut BuildContext| mount_showcase_root(ctx, &mount_state));
        let stats: FrameStats = app.step_frame().unwrap();
        assert_ne!(stats.get_frame_hash(), 0);
        assert!(stats.get_draw_commands() >= 4);
    }
}

#[test]
fn showcase_supports_render_mode_switching_and_theme_cycling() {
    reset_reactive_runtime();
    let sw_stats: FrameStats = run_headless_smoke(RenderMode::Software).unwrap();
    let hw_stats: FrameStats = run_headless_smoke(RenderMode::Hardware).unwrap();
    let hy_stats: FrameStats = run_headless_smoke(RenderMode::Hybrid).unwrap();
    assert_ne!(sw_stats.get_frame_hash(), 0);
    assert!(hw_stats.get_draw_batches() >= 1);
    assert_eq!(sw_stats.get_frame_hash(), hy_stats.get_frame_hash());
    let state: ShowcaseState = ShowcaseState::create();
    let initial: DesignScheme = state.get_theme().get_scheme().get_untracked();
    state.next_scheme();
    assert_ne!(state.get_theme().get_scheme().get_untracked(), initial);
    assert_eq!(
        state.get_theme().get_mode().get_untracked(),
        ThemeMode::Light
    );
    state.toggle_theme_mode();
    assert_eq!(
        state.get_theme().get_mode().get_untracked(),
        ThemeMode::Dark
    );
    let _: bool = state.get_render_mode_index().set(2);
    assert_eq!(state.selected_render_mode(), RenderMode::Hardware);
}
