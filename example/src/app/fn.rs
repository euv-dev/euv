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

/// Automatically boots the WebAssembly showcase and attaches all browser canvas, input, resize, and frame listeners in pure Rust.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn euv_web_boot() {
    let _: u64 = euv_example_web_start();
    let Some(win): Option<Window> = window() else {
        return;
    };
    let Some(doc): Option<Document> = win.document() else {
        return;
    };
    let Ok(Some(el)) = doc.query_selector("canvas") else {
        return;
    };
    let Ok(canvas) = el.dyn_into::<HtmlCanvasElement>() else {
        return;
    };
    let init_w: u32 = win
        .inner_width()
        .ok()
        .and_then(|v: JsValue| v.as_f64())
        .map(|n: f64| n.floor() as u32)
        .unwrap_or(1200)
        .max(320);
    let init_h: u32 = win
        .inner_height()
        .ok()
        .and_then(|v: JsValue| v.as_f64())
        .map(|n: f64| n.floor() as u32)
        .unwrap_or(800)
        .max(320);
    let init_dpr: f32 = (win.device_pixel_ratio() as f32).clamp(1.0, 2.0);
    let _: u64 = euv_web_resize(init_w, init_h, init_dpr);
    let canvas_move: HtmlCanvasElement = canvas.clone();
    let on_move: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |ev: MouseEvent| {
        let rect: DomRect = canvas_move.get_bounding_client_rect();
        let x: f32 = (ev.client_x() as f64 - rect.left()) as f32;
        let y: f32 = (ev.client_y() as f64 - rect.top()) as f32;
        let _: u64 = euv_web_pointer_move(x, y);
    })
        as Box<dyn FnMut(MouseEvent)>);
    drop(canvas.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref()));
    on_move.forget();
    let canvas_down: HtmlCanvasElement = canvas.clone();
    let on_down: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |ev: MouseEvent| {
        drop(canvas_down.focus());
        let rect: DomRect = canvas_down.get_bounding_client_rect();
        let x: f32 = (ev.client_x() as f64 - rect.left()) as f32;
        let y: f32 = (ev.client_y() as f64 - rect.top()) as f32;
        let _: u64 = euv_web_pointer_down(x, y);
    })
        as Box<dyn FnMut(MouseEvent)>);
    drop(canvas.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref()));
    on_down.forget();
    let canvas_up: HtmlCanvasElement = canvas.clone();
    let on_up: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |ev: MouseEvent| {
        let rect: DomRect = canvas_up.get_bounding_client_rect();
        let x: f32 = (ev.client_x() as f64 - rect.left()) as f32;
        let y: f32 = (ev.client_y() as f64 - rect.top()) as f32;
        let _: u64 = euv_web_pointer_up(x, y);
    }) as Box<dyn FnMut(MouseEvent)>);
    drop(canvas.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref()));
    on_up.forget();
    let on_wheel: Closure<dyn FnMut(WheelEvent)> = Closure::wrap(Box::new(move |ev: WheelEvent| {
        ev.prevent_default();
        let _: u64 = euv_web_scroll(ev.delta_x() as f32, ev.delta_y() as f32);
    })
        as Box<dyn FnMut(WheelEvent)>);
    drop(canvas.add_event_listener_with_callback("wheel", on_wheel.as_ref().unchecked_ref()));
    on_wheel.forget();
    let on_key: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let key: String = ev.key();
            if matches!(
                key.as_str(),
                WEB_KEY_ARROW_UP
                    | WEB_KEY_ARROW_DOWN
                    | WEB_KEY_ARROW_LEFT
                    | WEB_KEY_ARROW_RIGHT
                    | WEB_KEY_BACKSPACE
                    | WEB_KEY_SPACE
            ) {
                ev.prevent_default();
            }
            let _: u64 = euv_web_key_down(&key);
        }) as Box<dyn FnMut(KeyboardEvent)>);
    drop(win.add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref()));
    on_key.forget();
    let win_resize: Window = win.clone();
    let on_resize: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        let lw: u32 = win_resize
            .inner_width()
            .ok()
            .and_then(|v: JsValue| v.as_f64())
            .map(|n: f64| n.floor() as u32)
            .unwrap_or(1200)
            .max(320);
        let lh: u32 = win_resize
            .inner_height()
            .ok()
            .and_then(|v: JsValue| v.as_f64())
            .map(|n: f64| n.floor() as u32)
            .unwrap_or(800)
            .max(320);
        let dpr: f32 = (win_resize.device_pixel_ratio() as f32).clamp(1.0, 2.0);
        let _: u64 = euv_web_resize(lw, lh, dpr);
    }) as Box<dyn FnMut(Event)>);
    drop(win.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref()));
    on_resize.forget();
    let raf_cell: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let raf_loop: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = raf_cell.clone();
    let win_raf: Window = win.clone();
    let last_ts: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    if let Ok(mut slot) = raf_cell.try_borrow_mut() {
        *slot = Some(Closure::wrap(Box::new(move |ts: f64| {
            let mut dt: f32 = 1.0 / 60.0;
            if let Ok(mut prev) = last_ts.try_borrow_mut() {
                if *prev > 0.0 {
                    dt = (((ts - *prev) / 1000.0) as f32).clamp(0.001, 0.05);
                }
                *prev = ts;
            }
            if euv_web_current_route() == ROUTE_GAME {
                let _: u64 = euv_web_tick(dt);
            }
            if let Ok(cb_ref) = raf_loop.try_borrow()
                && let Some(cb) = cb_ref.as_ref()
            {
                let func: &Function = cb.as_ref().unchecked_ref();
                drop(win_raf.request_animation_frame(func));
            }
        }) as Box<dyn FnMut(f64)>));
    }
    if let Ok(cb_ref) = raf_cell.try_borrow()
        && let Some(cb) = cb_ref.as_ref()
    {
        let func: &Function = cb.as_ref().unchecked_ref();
        drop(win.request_animation_frame(func));
    }
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
            WEB_KEY_ENTER => KeyCode::Enter,
            WEB_KEY_BACKSPACE => KeyCode::Backspace,
            WEB_KEY_DELETE => KeyCode::Delete,
            WEB_KEY_ESCAPE => KeyCode::Escape,
            WEB_KEY_TAB => KeyCode::Tab,
            WEB_KEY_SPACE => KeyCode::Space,
            WEB_KEY_ARROW_LEFT => KeyCode::ArrowLeft,
            WEB_KEY_ARROW_RIGHT => KeyCode::ArrowRight,
            WEB_KEY_ARROW_UP => KeyCode::ArrowUp,
            WEB_KEY_ARROW_DOWN => KeyCode::ArrowDown,
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

/// Resizes the persistent WebAssembly showcase canvas and viewport immediately and re-renders.
///
/// # Arguments
///
/// - `u32` - New logical viewport width in CSS pixels.
/// - `u32` - New logical viewport height in CSS pixels.
/// - `f32` - Active browser `window.devicePixelRatio`.
///
/// # Returns
///
/// - `u64` - Deterministic frame hash produced by the resized render.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn euv_web_resize(width: u32, height: u32, dpr: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        let w: u32 = width.max(320);
        let h: u32 = height.max(320);
        let scale: f32 = if dpr.is_finite() && dpr > 0.0 {
            dpr
        } else {
            1.0
        };
        let wid: WindowId = app.get_window_state().get_id();
        app.queue_window_command(WindowCommand::SetSize(wid, w, h));
        app.dispatch_window_event(WindowEvent::Resized(wid, w, h, scale));
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
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
