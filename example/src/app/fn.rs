use super::*;

/// Builds an `App` and reactive `ShowcaseState` configured for either native OS or headless mode.
///
/// # Arguments
///
/// - `bool` - When `true`, uses deterministic `HeadlessPlatform`; otherwise uses the active native OS backend.
/// - `RenderMode` - Initial rendering pipeline mode.
///
/// # Returns
///
/// - `Result<(App, ShowcaseState), PlatformError>` - Initialized showcase app and state.
pub fn create_showcase_app(
    headless: bool,
    mode: RenderMode,
) -> Result<(App, ShowcaseState), PlatformError> {
    let render_cfg: RenderConfig = match mode {
        RenderMode::Software => RenderConfig::software(),
        RenderMode::Hardware => RenderConfig::hardware(),
        RenderMode::Hybrid => RenderConfig::hybrid(),
        RenderMode::Auto => RenderConfig::default(),
    };
    let mut window_cfg: WindowConfig = WindowConfig::default();
    window_cfg
        .set_title(SHOWCASE_WINDOW_TITLE.to_string())
        .set_size(SHOWCASE_WINDOW_WIDTH, SHOWCASE_WINDOW_HEIGHT)
        .set_render(render_cfg);
    let mut app_cfg: AppConfig = AppConfig::default();
    app_cfg.set_window(window_cfg);
    let mut app: App = if headless {
        App::headless(app_cfg)?
    } else {
        App::new(app_cfg)?
    };
    let state: ShowcaseState = ShowcaseState::new();
    let mount_state: ShowcaseState = state.clone();
    let _: NodeId = app.mount(move || mount_showcase_root(&mount_state));
    Ok((app, state))
}

/// Runs a complete headless smoke render of the showcase and returns frame telemetry.
///
/// # Arguments
///
/// - `RenderMode` - Rendering pipeline mode to verify.
///
/// # Returns
///
/// - `Result<FrameStats, PlatformError>` - Rendered frame statistics and deterministic pixel hash.
pub fn run_headless_smoke(mode: RenderMode) -> Result<FrameStats, PlatformError> {
    let (mut app, _): (App, ShowcaseState) = create_showcase_app(true, mode)?;
    app.step_frame()
}

/// WebAssembly entry point mounting the showcase onto the browser canvas and rendering the first frame.
///
/// # Returns
///
/// - `u64` - Deterministic frame hash produced by the initial render, or `0` on startup failure.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn euv_example_web_start() -> u64 {
    match create_showcase_app(false, RenderMode::Auto) {
        Ok((mut app, _)) => app
            .step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0),
        Err(_) => 0,
    }
}
