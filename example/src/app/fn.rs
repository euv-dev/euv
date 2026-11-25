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
    app.attach_game_host(state.get_game_host().clone());
    app.attach_game_host(state.get_game3d_host().clone());
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

/// Steps one frame and writes the rendered RGBA framebuffer to `<out_dir>/<name>.rgba`.
///
/// # Arguments
///
/// - `&mut App` - Active headless application instance.
/// - `&Path` - Output directory path.
/// - `&str` - Frame file basename without extension.
/// - `&mut String` - Manifest accumulator receiving `<name> <width> <height>` lines.
/// - `&mut usize` - Running captured frame counter.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Success or platform/filesystem error.
#[cfg(not(target_arch = "wasm32"))]
fn dump_rgba_frame(
    app: &mut App,
    out_dir: &Path,
    name: &str,
    manifest: &mut String,
    count: &mut usize,
) -> Result<(), PlatformError> {
    let _: FrameStats = app.step_frame()?;
    for _ in 0..20 {
        if !app.has_active_animations() {
            break;
        }
        let _: FrameStats = app.step_game_frame(1.0 / 60.0)?;
    }
    if let Some(fb) = app.get_renderer().framebuffer() {
        let file_name: String = format!("{name}.rgba");
        let file_path: PathBuf = out_dir.join(&file_name);
        fs::write(&file_path, fb.get_pixels().as_slice())
            .map_err(|err: Error| PlatformError::SurfaceError(err.to_string()))?;
        manifest.push_str(&format!("{name} {} {}\n", fb.get_width(), fb.get_height()));
        *count += 1;
    }
    Ok(())
}

/// Renders all showcase routes, responsive viewports, dark mode, scroll positions, and overlays to raw RGBA files using native system fonts.
///
/// # Arguments
///
/// - `&Path` - Destination directory for captured `.rgba` frames and `manifest.txt`.
///
/// # Returns
///
/// - `Result<usize, PlatformError>` - Total number of captured frames.
#[cfg(not(target_arch = "wasm32"))]
pub fn capture_showcase_screenshots(out_dir: &Path) -> Result<usize, PlatformError> {
    reset_reactive_runtime();
    fs::create_dir_all(out_dir)
        .map_err(|err: Error| PlatformError::InitializationFailed(err.to_string()))?;
    let (mut app, state): (App, ShowcaseState) = create_showcase_app(true, RenderMode::Software)?;
    app.get_mut_renderer()
        .get_mut_software()
        .set_font_provider(NativeSystemFont::system());
    let wid: WindowId = app.get_window_state().get_id();
    let mut manifest: String = String::new();
    let mut count: usize = 0;
    app.queue_window_command(WindowCommand::SetSize(wid, 1200, 800));
    app.dispatch_window_event(WindowEvent::Resized(wid, 1200, 800, 2.0));
    let desktop_routes: [(&str, &str); 12] = [
        (ROUTE_OVERVIEW, "01_overview_desktop"),
        (ROUTE_REACTIVE, "02_reactive_desktop"),
        (ROUTE_RENDER, "03_render_desktop"),
        (ROUTE_WINDOW, "04_window_desktop"),
        (ROUTE_FORMS, "05_forms_desktop"),
        (ROUTE_THEMES, "06_themes_desktop"),
        (ROUTE_DATA, "07_data_desktop"),
        (ROUTE_GAME, "08_game_desktop"),
        (ROUTE_GAME_3D, "08b_game3d_desktop"),
        (ROUTE_MEDIA, "08c_media_desktop"),
        (ROUTE_OVERLAYS, "09_overlays_desktop"),
        (ROUTE_DYNAMIC, "10_dynamic_desktop"),
    ];
    for (route, label) in desktop_routes.iter() {
        let _: bool = state.get_route().set_if_changed((*route).to_string());
        if *route == ROUTE_OVERVIEW {
            app.dispatch_window_event(WindowEvent::PointerMove(
                wid,
                PointerEvent::mouse(100.0, 108.0),
            ));
        }
        dump_rgba_frame(&mut app, out_dir, label, &mut manifest, &mut count)?;
    }
    let _: bool = state.get_route().set_if_changed(ROUTE_FORMS.to_string());
    let _: FrameStats = app.step_frame()?;
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(600.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 420.0)));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "05b_forms_scrolled",
        &mut manifest,
        &mut count,
    )?;
    let _: bool = state.get_route().set_if_changed(ROUTE_DATA.to_string());
    let _: FrameStats = app.step_frame()?;
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(600.0, 400.0),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 460.0)));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "07b_data_vlist_scrolled",
        &mut manifest,
        &mut count,
    )?;
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(320.0, 120.0),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 720.0)));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "07c_data_bottom",
        &mut manifest,
        &mut count,
    )?;
    let _: bool = state.get_route().set_if_changed(ROUTE_OVERVIEW.to_string());
    let _: FrameStats = app.step_frame()?;
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(100.0, 108.0),
    ));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "01b_overview_hovered",
        &mut manifest,
        &mut count,
    )?;
    let _: bool = state.get_route().set_if_changed(ROUTE_OVERLAYS.to_string());
    let _: bool = state.get_modal_open().set_if_changed(true);
    dump_rgba_frame(
        &mut app,
        out_dir,
        "09b_overlays_modal",
        &mut manifest,
        &mut count,
    )?;
    let _: bool = state.get_modal_open().set_if_changed(false);
    let _: bool = state.get_alert_dialog_open().set_if_changed(true);
    dump_rgba_frame(
        &mut app,
        out_dir,
        "09c_overlays_alert",
        &mut manifest,
        &mut count,
    )?;
    let _: bool = state.get_alert_dialog_open().set_if_changed(false);
    let _: bool = state.get_hover_card_open().set_if_changed(true);
    dump_rgba_frame(
        &mut app,
        out_dir,
        "09d_overlays_hovercard",
        &mut manifest,
        &mut count,
    )?;
    let _: bool = state.get_hover_card_open().set_if_changed(false);
    let _: bool = state.get_route().set_if_changed(ROUTE_THEMES.to_string());
    state.toggle_theme_mode();
    dump_rgba_frame(
        &mut app,
        out_dir,
        "06b_themes_dark",
        &mut manifest,
        &mut count,
    )?;
    state.toggle_theme_mode();
    let _: bool = state.get_route().set_if_changed(ROUTE_OVERVIEW.to_string());
    app.queue_window_command(WindowCommand::SetSize(wid, 768, 1024));
    app.dispatch_window_event(WindowEvent::Resized(wid, 768, 1024, 2.0));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "11_overview_tablet",
        &mut manifest,
        &mut count,
    )?;
    app.queue_window_command(WindowCommand::SetSize(wid, 390, 844));
    app.dispatch_window_event(WindowEvent::Resized(wid, 390, 844, 2.0));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "12_overview_mobile",
        &mut manifest,
        &mut count,
    )?;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(350.0, 26.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::mouse(350.0, 26.0),
    ));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "12b_mobile_menu_open",
        &mut manifest,
        &mut count,
    )?;
    app.dispatch_window_event(WindowEvent::PointerDown(
        wid,
        PointerEvent::mouse(240.0, 26.0),
    ));
    app.dispatch_window_event(WindowEvent::PointerUp(
        wid,
        PointerEvent::mouse(240.0, 26.0),
    ));
    let _: bool = state.get_route().set_if_changed(ROUTE_FORMS.to_string());
    dump_rgba_frame(
        &mut app,
        out_dir,
        "13_forms_mobile",
        &mut manifest,
        &mut count,
    )?;
    fs::write(out_dir.join("manifest.txt"), manifest)
        .map_err(|err: Error| PlatformError::SurfaceError(err.to_string()))?;
    Ok(count)
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
        web_queue_pointer_move(x, y);
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
        web_queue_pointer_down(x, y);
    })
        as Box<dyn FnMut(MouseEvent)>);
    drop(canvas.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref()));
    on_down.forget();
    let canvas_up: HtmlCanvasElement = canvas.clone();
    let on_up: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |ev: MouseEvent| {
        let rect: DomRect = canvas_up.get_bounding_client_rect();
        let x: f32 = (ev.client_x() as f64 - rect.left()) as f32;
        let y: f32 = (ev.client_y() as f64 - rect.top()) as f32;
        web_queue_pointer_up(x, y);
    }) as Box<dyn FnMut(MouseEvent)>);
    drop(canvas.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref()));
    on_up.forget();
    let on_wheel: Closure<dyn FnMut(WheelEvent)> = Closure::wrap(Box::new(move |ev: WheelEvent| {
        ev.prevent_default();
        let mult: f32 = match ev.delta_mode() {
            1 => 20.0,
            2 => 320.0,
            _ => 1.0,
        };
        web_queue_scroll((ev.delta_x() as f32) * mult, (ev.delta_y() as f32) * mult);
    })
        as Box<dyn FnMut(WheelEvent)>);
    drop(canvas.add_event_listener_with_callback("wheel", on_wheel.as_ref().unchecked_ref()));
    on_wheel.forget();
    let on_key: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let key: String = ev.key();
            let code_str: String = ev.code();
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
            web_queue_key_down(&key, &code_str);
        }) as Box<dyn FnMut(KeyboardEvent)>);
    drop(win.add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref()));
    on_key.forget();
    let on_key_up: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let key: String = ev.key();
            let code_str: String = ev.code();
            web_queue_key_up(&key, &code_str);
        }) as Box<dyn FnMut(KeyboardEvent)>);
    drop(win.add_event_listener_with_callback("keyup", on_key_up.as_ref().unchecked_ref()));
    on_key_up.forget();
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
        web_queue_resize(lw, lh, dpr);
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
            let cur_route: String = euv_web_current_route();
            let on_game: bool = cur_route == ROUTE_GAME || cur_route == ROUTE_GAME_3D;
            let animating: bool = web_has_active_animations();
            let dirty: bool = WEB_NEEDS_FRAME.with(|flag: &Cell<bool>| flag.replace(false));
            if on_game || animating {
                let _: u64 = euv_web_tick(dt);
            } else if dirty {
                let _: u64 = web_flush_frame();
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

/// Maps a browser `KeyboardEvent.key` and physical `KeyboardEvent.code` into a cross-platform `KeyCode`.
///
/// # Arguments
///
/// - `&str` - Browser DOM `KeyboardEvent.key` string.
/// - `&str` - Browser DOM physical `KeyboardEvent.code` string (IME-independent).
///
/// # Returns
///
/// - `KeyCode` - Normalized `euv` key code.
#[cfg(target_arch = "wasm32")]
fn web_map_key(key: &str, code_str: &str) -> KeyCode {
    match code_str {
        "KeyW" => return KeyCode::Character('w'),
        "KeyA" => return KeyCode::Character('a'),
        "KeyS" => return KeyCode::Character('s'),
        "KeyD" => return KeyCode::Character('d'),
        "KeyE" => return KeyCode::Character('e'),
        "KeyR" => return KeyCode::Character('r'),
        "KeyP" => return KeyCode::Character('p'),
        "KeyJ" => return KeyCode::Character('j'),
        "Space" => return KeyCode::Space,
        "ArrowLeft" => return KeyCode::ArrowLeft,
        "ArrowRight" => return KeyCode::ArrowRight,
        "ArrowUp" => return KeyCode::ArrowUp,
        "ArrowDown" => return KeyCode::ArrowDown,
        _ => {}
    }
    match key {
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
    }
}

/// Returns `true` when the active WebAssembly app has running UI transition animations.
///
/// # Returns
///
/// - `bool` - Whether animation frames must continue ticking.
#[cfg(target_arch = "wasm32")]
fn web_has_active_animations() -> bool {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        match slot.try_borrow() {
            Ok(guard) => guard
                .as_ref()
                .map(|(app, _): &(App, ShowcaseState)| app.has_active_animations())
                .unwrap_or(false),
            Err(_) => false,
        }
    })
}

/// Marks the coalesced Web animation-frame render flag so the next `requestAnimationFrame` flushes visual changes.
#[cfg(target_arch = "wasm32")]
fn web_mark_needs_frame() {
    WEB_NEEDS_FRAME.with(|flag: &Cell<bool>| flag.set(true));
}

/// Enqueues a browser pointer move into the active WebAssembly runtime without synchronous full-frame rasterization.
///
/// # Arguments
///
/// - `f32` - Logical X coordinate.
/// - `f32` - Logical Y coordinate.
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_move(x: f32, y: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerMove(wid, PointerEvent::mouse(x, y)));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser pointer press into the active WebAssembly runtime for the next animation frame.
///
/// # Arguments
///
/// - `f32` - Logical X coordinate.
/// - `f32` - Logical Y coordinate.
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_down(x: f32, y: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerDown(wid, PointerEvent::mouse(x, y)));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser pointer release into the active WebAssembly runtime for the next animation frame.
///
/// # Arguments
///
/// - `f32` - Logical X coordinate.
/// - `f32` - Logical Y coordinate.
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_up(x: f32, y: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerUp(wid, PointerEvent::mouse(x, y)));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a normalized browser wheel scroll delta for coalesced VSync rendering.
///
/// # Arguments
///
/// - `f32` - Horizontal scroll delta.
/// - `f32` - Vertical scroll delta.
#[cfg(target_arch = "wasm32")]
fn web_queue_scroll(dx: f32, dy: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(dx, dy)));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser key-down and optional printable text event for the next animation frame.
///
/// # Arguments
///
/// - `&str` - DOM `KeyboardEvent.key` string.
/// - `&str` - DOM physical `KeyboardEvent.code` string.
#[cfg(target_arch = "wasm32")]
fn web_queue_key_down(key: &str, code_str: &str) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            let code: KeyCode = web_map_key(key, code_str);
            let key_ev: WindowEvent =
                WindowEvent::KeyDown(wid, KeyEvent::new(code, Modifiers::default(), false));
            app.dispatch_window_event(key_ev);
            if key.chars().count() == 1 {
                app.dispatch_window_event(WindowEvent::TextInput(wid, key.to_string()));
            }
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser key-up event into the active WebAssembly runtime and game host.
///
/// # Arguments
///
/// - `&str` - DOM `KeyboardEvent.key` string.
/// - `&str` - DOM physical `KeyboardEvent.code` string.
#[cfg(target_arch = "wasm32")]
fn web_queue_key_up(key: &str, code_str: &str) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            let code: KeyCode = web_map_key(key, code_str);
            app.dispatch_window_event(WindowEvent::KeyUp(
                wid,
                KeyEvent::new(code, Modifiers::default(), false),
            ));
        }
    });
}

/// Enqueues a browser resize event for immediate VSync coalesced redraw.
///
/// # Arguments
///
/// - `u32` - Logical viewport width.
/// - `u32` - Logical viewport height.
/// - `f32` - Active browser device pixel ratio.
#[cfg(target_arch = "wasm32")]
fn web_queue_resize(width: u32, height: u32, dpr: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
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
            web_mark_needs_frame();
        }
    });
}

/// Steps one coalesced WebAssembly frame during `requestAnimationFrame`.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
fn web_flush_frame() -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        app.step_frame()
            .map(|stats: FrameStats| stats.get_frame_hash())
            .unwrap_or(0)
    })
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
        let cur_route: String = state.get_route().get_untracked();
        let on_game: bool = cur_route == ROUTE_GAME || cur_route == ROUTE_GAME_3D;
        if on_game || app.has_active_animations() {
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

/// Executes an AI Schema v1 JSON query against the live WebAssembly showcase in dev builds only (stripped in release).
///
/// # Arguments
///
/// - `&str` - Serialized `AiSchemaRequest` JSON string.
///
/// # Returns
///
/// - `String` - Serialized `AiSchemaResponse` JSON string.
#[cfg(all(debug_assertions, target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn euv_dev_ai_schema(request_json: &str) -> String {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return String::new(),
        };
        let Some((app, _)) = borrow.as_mut() else {
            return String::new();
        };
        AiSchemaProtocol::handle_json(app, request_json)
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
