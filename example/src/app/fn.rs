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
    let mut state: ShowcaseState = ShowcaseState::new();
    state.attach_dynamic_hub(app.get_dynamic_hub().clone());
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

/// Initializes the persistent WebAssembly showcase on the browser `<canvas>` and renders the first frame.
///
/// # Returns
///
/// - `u64` - Deterministic frame hash produced by the initial render, or `0` on startup failure.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_example_web_start() -> u64 {
    reset_reactive_runtime();
    match create_showcase_app(false, RenderMode::Software) {
        Ok((mut app, state)) => {
            let hash: u64 = app
                .step_frame()
                .map(|stats: FrameStats| stats.get_frame_hash())
                .unwrap_or(0);
            WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
                if let Ok(mut borrow) = slot.try_borrow_mut() {
                    *borrow = Some((app, state));
                }
            });
            hash
        }
        Err(_) => 0,
    }
}

/// Switches the active route in the persistent WebAssembly showcase and renders immediately.
///
/// # Arguments
///
/// - `&str` - Route path to navigate to.
///
/// # Returns
///
/// - `u64` - Deterministic frame hash after route navigation.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_example_render_route(route: &str) -> u64 {
    let has_runtime: bool = WEB_SHOWCASE_RUNTIME.with(
        |slot: &RefCell<Option<(App, ShowcaseState)>>| matches!(slot.try_borrow(), Ok(guard) if guard.is_some()),
    );
    if !has_runtime {
        let _: u64 = euv_example_web_start();
    }
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, state)) = borrow.as_mut() else {
            return 0;
        };
        let _: bool = state.get_route().set_if_changed(route.to_string());
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
}

/// Dispatches a browser pointer move event into the persistent WebAssembly showcase and re-renders.
///
/// # Arguments
///
/// - `f32` - Logical X coordinate inside the canvas.
/// - `f32` - Logical Y coordinate inside the canvas.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_pointer_move(x: f32, y: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerMove(wid, PointerEvent::mouse(x, y)));
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
}

/// Dispatches a browser pointer press event into the persistent WebAssembly showcase and re-renders.
///
/// # Arguments
///
/// - `f32` - Logical X coordinate inside the canvas.
/// - `f32` - Logical Y coordinate inside the canvas.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_pointer_down(x: f32, y: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerDown(wid, PointerEvent::mouse(x, y)));
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
}

/// Dispatches a browser pointer release (and click) event into the persistent WebAssembly showcase and re-renders.
///
/// # Arguments
///
/// - `f32` - Logical X coordinate inside the canvas.
/// - `f32` - Logical Y coordinate inside the canvas.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_pointer_up(x: f32, y: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::mouse(x, y)));
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
}

/// Dispatches a browser wheel scroll delta into the persistent WebAssembly showcase and re-renders.
///
/// # Arguments
///
/// - `f32` - Horizontal scroll delta.
/// - `f32` - Vertical scroll delta.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_scroll(dx: f32, dy: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        let wid: WindowId = app.get_window_state().get_id();
        app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(dx, dy)));
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
}

/// Dispatches a browser keyboard press and optional text input into the persistent WebAssembly showcase.
///
/// # Arguments
///
/// - `&str` - DOM `KeyboardEvent.key` string (e.g. `"Backspace"`, `"ArrowRight"`, `"a"`).
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_key_down(key: &str) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, state)) = borrow.as_mut() else {
            return 0;
        };
        let wid: WindowId = app.get_window_state().get_id();
        let code: KeyCode = match key {
            "Enter" => KeyCode::Enter,
            "Backspace" => KeyCode::Backspace,
            "Delete" => KeyCode::Delete,
            "Escape" => KeyCode::Escape,
            "Tab" => KeyCode::Tab,
            " " => KeyCode::Space,
            "ArrowLeft" => KeyCode::ArrowLeft,
            "ArrowRight" => KeyCode::ArrowRight,
            "ArrowUp" => KeyCode::ArrowUp,
            "ArrowDown" => KeyCode::ArrowDown,
            other => other
                .chars()
                .next()
                .map(KeyCode::Character)
                .unwrap_or(KeyCode::Unknown),
        };
        let key_ev: WindowEvent =
            WindowEvent::KeyDown(wid, KeyEvent::new(code, Modifiers::default(), false));
        state.get_game_host().ingest_event(&key_ev);
        app.dispatch_window_event(key_ev);
        if key.chars().count() == 1 {
            app.dispatch_window_event(WindowEvent::TextInput(wid, key.to_string()));
        }
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
}

/// Advances the game physics clock by `dt_secs` when viewing the Game Engine route and renders a frame.
///
/// # Arguments
///
/// - `f32` - Elapsed seconds since the previous animation frame.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_tick(dt_secs: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, state)) = borrow.as_mut() else {
            return 0;
        };
        if state.get_route().get_untracked() == ROUTE_GAME {
            let _: u32 = state.get_game_host().step(dt_secs);
            return app
                .step_game_frame(dt_secs)
                .map(|stats: FrameStats| stats.get_frame_hash())
                .unwrap_or(0);
        }
        app.get_renderer().get_last_stats().get_frame_hash()
    })
}

/// Returns the active route string of the persistent WebAssembly showcase for telemetry.
///
/// # Returns
///
/// - `String` - Current active route path.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_current_route() -> String {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        match slot.try_borrow() {
            Ok(guard) => guard
                .as_ref()
                .map(|(_, state): &(App, ShowcaseState)| state.get_route().get_untracked())
                .unwrap_or_default(),
            Err(_) => String::new(),
        }
    })
}
