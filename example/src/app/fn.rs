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
    let host_plat_idx: usize = match app.platform_kind() {
        PlatformKind::Ios => 0,
        PlatformKind::IpadOs => 1,
        PlatformKind::Android => 2,
        PlatformKind::MacOs => 3,
        PlatformKind::Windows => 4,
        PlatformKind::Linux => 5,
        PlatformKind::Web | PlatformKind::Headless => 6,
    };
    let _: bool = state
        .get_os_target_platform_idx()
        .set_if_changed(host_plat_idx);
    state.attach_dynamic_hub(app.get_dynamic_hub().clone());
    app.attach_game_host(state.get_game_host().clone());
    app.attach_game_host(state.get_game3d_host().clone());
    let mount_state: ShowcaseState = state.clone();
    let _: NodeId = app.mount(move || mount_showcase_root(&mount_state));
    Ok((app, state))
}

/// Builds an `App` and reactive `ShowcaseState` reporting a specific host `PlatformKind` (for native mobile hosts).
///
/// # Arguments
///
/// - `PlatformKind` - Active native host platform identity (e.g. `PlatformKind::Android`, `PlatformKind::Ios`).
/// - `RenderMode` - Initial rendering pipeline mode.
///
/// # Returns
///
/// - `Result<(App, ShowcaseState), PlatformError>` - Initialized showcase app and state.
pub fn create_showcase_app_for_platform(
    platform: PlatformKind,
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
    let backend: Box<dyn PlatformBackend> = Box::new(HeadlessPlatform::emulating(
        platform,
        1.0,
        SafeAreaInsets::default(),
    ));
    let mut app: App = App::with_platform(app_cfg, backend)?;
    let mut state: ShowcaseState = ShowcaseState::new();
    let host_plat_idx: usize = match app.platform_kind() {
        PlatformKind::Ios => 0,
        PlatformKind::IpadOs => 1,
        PlatformKind::Android => 2,
        PlatformKind::MacOs => 3,
        PlatformKind::Windows => 4,
        PlatformKind::Linux => 5,
        PlatformKind::Web | PlatformKind::Headless => 6,
    };
    let _: bool = state
        .get_os_target_platform_idx()
        .set_if_changed(host_plat_idx);
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
    let mut window_cfg: WindowConfig = WindowConfig::default();
    window_cfg
        .set_title(SHOWCASE_WINDOW_TITLE.to_string())
        .set_size(1200, 800)
        .set_render(RenderConfig::software());
    let mut app_cfg: AppConfig = AppConfig::default();
    app_cfg.set_window(window_cfg);
    let backend: Box<dyn PlatformBackend> = Box::new(HeadlessPlatform::emulating(
        PlatformKind::Headless,
        2.0,
        SafeAreaInsets::default(),
    ));
    let mut app: App = App::with_platform(app_cfg, backend)?;
    let mut state: ShowcaseState = ShowcaseState::new();
    state.attach_dynamic_hub(app.get_dynamic_hub().clone());
    app.attach_game_host(state.get_game_host().clone());
    app.attach_game_host(state.get_game3d_host().clone());
    let mount_state: ShowcaseState = state.clone();
    let _: NodeId = app.mount(move || mount_showcase_root(&mount_state));
    app.get_mut_renderer()
        .get_mut_software()
        .set_font_provider(NativeSystemFont::system());
    let wid: WindowId = app.get_window_state().get_id();
    let mut manifest: String = String::new();
    let mut count: usize = 0;
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
    app.dispatch_window_event(WindowEvent::Resized(wid, 768, 1024, 2.0));
    dump_rgba_frame(
        &mut app,
        out_dir,
        "11_overview_tablet",
        &mut manifest,
        &mut count,
    )?;
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
    let Ok(Some(el)) = doc.query_selector(WEB_SELECTOR_CANVAS) else {
        return;
    };
    let Ok(canvas) = el.dyn_into::<HtmlCanvasElement>() else {
        return;
    };
    web_attach_ime_proxy(&doc);
    register_native_action_bridge(web_native_action_bridge);
    let _: bool = web_sync_exact_viewport(&win, &canvas);
    let canvas_move: HtmlCanvasElement = canvas.clone();
    let on_move: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::wrap(Box::new(move |ev: web_sys::PointerEvent| {
            let ptr: PointerEvent = web_pointer_from_dom(&ev, &canvas_move, None);
            web_queue_pointer_move_event(ptr);
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
    drop(canvas.add_event_listener_with_callback(
        WEB_EVENT_POINTER_MOVE,
        on_move.as_ref().unchecked_ref(),
    ));
    on_move.forget();
    let canvas_down: HtmlCanvasElement = canvas.clone();
    let on_down: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::wrap(Box::new(move |ev: web_sys::PointerEvent| {
            drop(canvas_down.set_pointer_capture(ev.pointer_id()));
            let ptr: PointerEvent = web_pointer_from_dom(&ev, &canvas_down, Some(true));
            web_queue_pointer_down_event(ptr);
            if !WEB_IME_ACTIVE.with(Cell::get) {
                drop(canvas_down.focus());
            }
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
    drop(canvas.add_event_listener_with_callback(
        WEB_EVENT_POINTER_DOWN,
        on_down.as_ref().unchecked_ref(),
    ));
    on_down.forget();
    let canvas_up: HtmlCanvasElement = canvas.clone();
    let on_up: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::wrap(Box::new(move |ev: web_sys::PointerEvent| {
            ev.stop_propagation();
            let ptr: PointerEvent = web_pointer_from_dom(&ev, &canvas_up, Some(false));
            web_queue_pointer_up_event(ptr);
            drop(canvas_up.release_pointer_capture(ev.pointer_id()));
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
    drop(
        canvas
            .add_event_listener_with_callback(WEB_EVENT_POINTER_UP, on_up.as_ref().unchecked_ref()),
    );
    on_up.forget();
    let on_cancel: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::wrap(Box::new(move |ev: web_sys::PointerEvent| {
            ev.stop_propagation();
            web_queue_pointer_cancel(web_pointer_id_from_dom(&ev));
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
    drop(canvas.add_event_listener_with_callback(
        WEB_EVENT_POINTER_CANCEL,
        on_cancel.as_ref().unchecked_ref(),
    ));
    drop(canvas.add_event_listener_with_callback(
        WEB_EVENT_LOST_POINTER_CAPTURE,
        on_cancel.as_ref().unchecked_ref(),
    ));
    on_cancel.forget();
    let canvas_win_up: HtmlCanvasElement = canvas.clone();
    let on_win_up: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::wrap(Box::new(move |ev: web_sys::PointerEvent| {
            let ptr: PointerEvent = web_pointer_from_dom(&ev, &canvas_win_up, Some(false));
            web_queue_pointer_up_event(ptr);
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
    drop(win.add_event_listener_with_callback(
        WEB_EVENT_POINTER_UP,
        on_win_up.as_ref().unchecked_ref(),
    ));
    on_win_up.forget();
    let on_win_cancel: Closure<dyn FnMut(web_sys::PointerEvent)> =
        Closure::wrap(Box::new(move |ev: web_sys::PointerEvent| {
            web_queue_pointer_cancel(web_pointer_id_from_dom(&ev));
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
    drop(win.add_event_listener_with_callback(
        WEB_EVENT_POINTER_CANCEL,
        on_win_cancel.as_ref().unchecked_ref(),
    ));
    on_win_cancel.forget();
    let on_blur: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_queue_pointer_cancel(0);
        web_dispatch_system_event(SystemEvent::FocusChanged(false));
    }) as Box<dyn FnMut(Event)>);
    drop(win.add_event_listener_with_callback(WEB_EVENT_BLUR, on_blur.as_ref().unchecked_ref()));
    on_blur.forget();
    web_attach_system_event_listeners(&win, &doc);
    let canvas_wheel: HtmlCanvasElement = canvas.clone();
    let on_wheel: Closure<dyn FnMut(WheelEvent)> = Closure::wrap(Box::new(move |ev: WheelEvent| {
        ev.prevent_default();
        let rect: DomRect = canvas_wheel.get_bounding_client_rect();
        let x: f32 = (ev.client_x() as f64 - rect.left()) as f32;
        let y: f32 = (ev.client_y() as f64 - rect.top()) as f32;
        let mult: f32 = match ev.delta_mode() {
            1 => 20.0,
            2 => 320.0,
            _ => 1.0,
        };
        web_queue_scroll_at(
            x,
            y,
            (ev.delta_x() as f32) * mult,
            (ev.delta_y() as f32) * mult,
        );
    })
        as Box<dyn FnMut(WheelEvent)>);
    drop(
        canvas.add_event_listener_with_callback(WEB_EVENT_WHEEL, on_wheel.as_ref().unchecked_ref()),
    );
    on_wheel.forget();
    let on_key: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let key: String = ev.key();
            let code_str: String = ev.code();
            let ime_active: bool = WEB_IME_ACTIVE.with(Cell::get);
            let should_prevent: bool = if ime_active {
                matches!(
                    key.as_str(),
                    WEB_KEY_ARROW_UP
                        | WEB_KEY_ARROW_DOWN
                        | WEB_KEY_ARROW_LEFT
                        | WEB_KEY_ARROW_RIGHT
                        | WEB_KEY_BACKSPACE
                )
            } else {
                matches!(
                    key.as_str(),
                    WEB_KEY_ARROW_UP
                        | WEB_KEY_ARROW_DOWN
                        | WEB_KEY_ARROW_LEFT
                        | WEB_KEY_ARROW_RIGHT
                        | WEB_KEY_BACKSPACE
                        | WEB_KEY_SPACE
                )
            };
            if should_prevent {
                ev.prevent_default();
            }
            web_queue_key_down(&key, &code_str);
        }) as Box<dyn FnMut(KeyboardEvent)>);
    drop(win.add_event_listener_with_callback(WEB_EVENT_KEY_DOWN, on_key.as_ref().unchecked_ref()));
    on_key.forget();
    let on_key_up: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let key: String = ev.key();
            let code_str: String = ev.code();
            web_queue_key_up(&key, &code_str);
        }) as Box<dyn FnMut(KeyboardEvent)>);
    drop(
        win.add_event_listener_with_callback(WEB_EVENT_KEY_UP, on_key_up.as_ref().unchecked_ref()),
    );
    on_key_up.forget();
    let win_resize: Window = win.clone();
    let canvas_resize: HtmlCanvasElement = canvas.clone();
    let on_resize: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        if web_sync_exact_viewport(&win_resize, &canvas_resize) {
            web_mark_needs_frame();
        }
    }) as Box<dyn FnMut(Event)>);
    drop(
        win.add_event_listener_with_callback(WEB_EVENT_RESIZE, on_resize.as_ref().unchecked_ref()),
    );
    on_resize.forget();
    let win_raf: Window = win.clone();
    let canvas_raf: HtmlCanvasElement = canvas.clone();
    WEB_RAF_CALLBACK.with(|slot: &RefCell<Option<WebRafCallback>>| {
        if let Ok(mut guard) = slot.try_borrow_mut() {
            *guard = Some(Closure::wrap(Box::new(move |ts: f64| {
                WEB_RAF_ID.with(|id_cell: &Cell<Option<i32>>| id_cell.set(None));
                if !web_should_pump_frames() {
                    WEB_RAF_LAST_TS.with(|ts_cell: &Cell<f64>| ts_cell.set(0.0));
                    WEB_RAF_ACCUM_SECS.with(|s_cell: &Cell<f32>| s_cell.set(0.0));
                    WEB_RAF_ACCUM_FRAMES.with(|f_cell: &Cell<u32>| f_cell.set(0));
                    return;
                }
                let prev_ts: f64 = WEB_RAF_LAST_TS.with(|ts_cell: &Cell<f64>| ts_cell.replace(ts));
                let mut dt: f32 = 1.0 / 60.0;
                let mut valid_step: bool = false;
                if prev_ts > 0.0 {
                    let raw_dt: f32 = ((ts - prev_ts) / 1000.0) as f32;
                    if (0.001..=0.1).contains(&raw_dt) {
                        valid_step = true;
                    }
                    dt = raw_dt.clamp(0.001, 0.05);
                }
                if valid_step {
                    let next_secs: f32 = WEB_RAF_ACCUM_SECS.with(Cell::get) + dt;
                    let next_frames: u32 = WEB_RAF_ACCUM_FRAMES.with(Cell::get).saturating_add(1);
                    if next_secs >= 0.35 && next_frames >= 8 {
                        let measured_hz: f32 = (next_frames as f32) / next_secs;
                        WEB_RAF_ACCUM_SECS.with(|s_cell: &Cell<f32>| s_cell.set(0.0));
                        WEB_RAF_ACCUM_FRAMES.with(|f_cell: &Cell<u32>| f_cell.set(0));
                        web_report_host_refresh_rate(measured_hz);
                    } else {
                        WEB_RAF_ACCUM_SECS.with(|s_cell: &Cell<f32>| s_cell.set(next_secs));
                        WEB_RAF_ACCUM_FRAMES.with(|f_cell: &Cell<u32>| f_cell.set(next_frames));
                    }
                }
                let resized_or_scaled: bool = web_sync_exact_viewport(&win_raf, &canvas_raf);
                let cur_route: String = euv_web_current_route();
                let on_game: bool = cur_route == ROUTE_GAME || cur_route == ROUTE_GAME_3D;
                let animating: bool = web_has_active_animations();
                let dirty: bool = WEB_NEEDS_FRAME.with(|flag: &Cell<bool>| flag.replace(false));
                let needs_continuous: bool = resized_or_scaled || on_game;
                let needs_redraw: bool = animating || dirty;
                if needs_continuous || needs_redraw {
                    let _: u64 = web_flush_frame(dt);
                }
                if web_should_pump_frames() {
                    web_start_raf_pump(&win_raf);
                }
            }) as Box<dyn FnMut(f64)>));
        }
    });
    if doc.hidden() {
        web_dispatch_system_event(SystemEvent::Background);
    } else {
        web_start_raf_pump(&win);
    }
}

/// Returns `true` when the active WebAssembly `App` should pump `requestAnimationFrame` frames.
///
/// # Returns
///
/// - `bool` - `false` while background-frozen or shutting down.
#[cfg(target_arch = "wasm32")]
fn web_should_pump_frames() -> bool {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        match slot.try_borrow() {
            Ok(guard) => guard
                .as_ref()
                .map(|(app, _): &(App, ShowcaseState)| app.should_pump_frames())
                .unwrap_or(false),
            Err(_) => false,
        }
    })
}

/// Schedules the retained browser `requestAnimationFrame` callback if no frame is already pending.
///
/// # Arguments
///
/// - `&Window` - Active browser `Window`.
#[cfg(target_arch = "wasm32")]
fn web_start_raf_pump(win: &Window) {
    if !web_should_pump_frames() {
        return;
    }
    let already_scheduled: bool =
        WEB_RAF_ID.with(|id_cell: &Cell<Option<i32>>| id_cell.get().is_some());
    if already_scheduled {
        return;
    }
    WEB_RAF_CALLBACK.with(|slot: &RefCell<Option<WebRafCallback>>| {
        if let Ok(cb_ref) = slot.try_borrow()
            && let Some(cb) = cb_ref.as_ref()
        {
            let func: &Function = cb.as_ref().unchecked_ref();
            if let Ok(req_id) = win.request_animation_frame(func) {
                WEB_RAF_ID.with(|id_cell: &Cell<Option<i32>>| id_cell.set(Some(req_id)));
            }
        }
    });
}

/// Cancels any pending browser `requestAnimationFrame` callback and resets rAF timing accumulators for zero-CPU background sleep.
#[cfg(target_arch = "wasm32")]
fn web_stop_raf_pump() {
    let pending_id: Option<i32> = WEB_RAF_ID.with(|id_cell: &Cell<Option<i32>>| id_cell.take());
    if let Some(req_id) = pending_id
        && let Some(win) = window()
    {
        drop(win.cancel_animation_frame(req_id));
    }
    WEB_RAF_LAST_TS.with(|ts_cell: &Cell<f64>| ts_cell.set(0.0));
    WEB_RAF_ACCUM_SECS.with(|s_cell: &Cell<f32>| s_cell.set(0.0));
    WEB_RAF_ACCUM_FRAMES.with(|f_cell: &Cell<u32>| f_cell.set(0));
}

/// Dispatches a browser-originated `SystemEvent` into the active WebAssembly `App` and pauses or restarts the rAF pump accordingly.
///
/// # Arguments
///
/// - `SystemEvent` - Cross-platform system or lifecycle event to normalize and dispatch.
#[cfg(target_arch = "wasm32")]
fn web_dispatch_system_event(event: SystemEvent) {
    let mut should_pump: bool = false;
    let mut handled: bool = false;
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            app.dispatch_system_event(event);
            should_pump = app.should_pump_frames();
            handled = true;
        }
    });
    if !handled {
        return;
    }
    if !should_pump {
        web_stop_raf_pump();
    } else {
        WEB_RAF_LAST_TS.with(|ts_cell: &Cell<f64>| ts_cell.set(0.0));
        web_mark_needs_frame();
        if let Some(win) = window() {
            web_start_raf_pump(&win);
        }
    }
}

/// Attaches browser DOM lifecycle, focus, connectivity, color-scheme, deep-link, and soft-keyboard listeners onto `win` and `doc`.
///
/// # Arguments
///
/// - `&Window` - Active browser `Window`.
/// - `&Document` - Active browser `Document`.
#[cfg(target_arch = "wasm32")]
fn web_attach_system_event_listeners(win: &Window, doc: &Document) {
    let doc_vis: Document = doc.clone();
    let on_vis: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        if doc_vis.hidden() {
            web_dispatch_system_event(SystemEvent::Background);
        } else {
            web_dispatch_system_event(SystemEvent::Foreground);
        }
    }) as Box<dyn FnMut(Event)>);
    drop(doc.add_event_listener_with_callback(
        WEB_EVENT_VISIBILITY_CHANGE,
        on_vis.as_ref().unchecked_ref(),
    ));
    on_vis.forget();
    let on_focus: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_dispatch_system_event(SystemEvent::FocusChanged(true));
    }) as Box<dyn FnMut(Event)>);
    drop(win.add_event_listener_with_callback(WEB_EVENT_FOCUS, on_focus.as_ref().unchecked_ref()));
    on_focus.forget();
    let on_pageshow: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_dispatch_system_event(SystemEvent::Foreground);
    }) as Box<dyn FnMut(Event)>);
    drop(win.add_event_listener_with_callback(
        WEB_EVENT_PAGE_SHOW,
        on_pageshow.as_ref().unchecked_ref(),
    ));
    on_pageshow.forget();
    let on_pagehide: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_dispatch_system_event(SystemEvent::Background);
    }) as Box<dyn FnMut(Event)>);
    drop(win.add_event_listener_with_callback(
        WEB_EVENT_PAGE_HIDE,
        on_pagehide.as_ref().unchecked_ref(),
    ));
    on_pagehide.forget();
    let on_unload: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_dispatch_system_event(SystemEvent::Terminate);
    }) as Box<dyn FnMut(Event)>);
    drop(win.add_event_listener_with_callback(
        WEB_EVENT_BEFORE_UNLOAD,
        on_unload.as_ref().unchecked_ref(),
    ));
    on_unload.forget();
    let on_online: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_dispatch_system_event(SystemEvent::NetworkStatusChanged(NetworkStatus::Online));
    }) as Box<dyn FnMut(Event)>);
    drop(
        win.add_event_listener_with_callback(WEB_EVENT_ONLINE, on_online.as_ref().unchecked_ref()),
    );
    on_online.forget();
    let on_offline: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        web_dispatch_system_event(SystemEvent::NetworkStatusChanged(NetworkStatus::Offline));
    }) as Box<dyn FnMut(Event)>);
    drop(
        win.add_event_listener_with_callback(
            WEB_EVENT_OFFLINE,
            on_offline.as_ref().unchecked_ref(),
        ),
    );
    on_offline.forget();
    if let Ok(Some(mql)) = win.match_media(WEB_MEDIA_PREFERS_DARK) {
        let mql_cb: MediaQueryList = mql.clone();
        let on_theme: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
            let next_theme: SystemTheme = if mql_cb.matches() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
            web_dispatch_system_event(SystemEvent::ThemeChanged(next_theme));
        })
            as Box<dyn FnMut(Event)>);
        drop(
            mql.add_event_listener_with_callback(
                WEB_EVENT_CHANGE,
                on_theme.as_ref().unchecked_ref(),
            ),
        );
        on_theme.forget();
    }
    let win_hash: Window = win.clone();
    let on_hash: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        if let Ok(href) = win_hash.location().href() {
            web_dispatch_system_event(SystemEvent::DeepLink(href));
        }
    }) as Box<dyn FnMut(Event)>);
    drop(
        win.add_event_listener_with_callback(
            WEB_EVENT_HASH_CHANGE,
            on_hash.as_ref().unchecked_ref(),
        ),
    );
    on_hash.forget();
    let vv_key: JsValue = JsValue::from_str(WEB_PROP_VISUAL_VIEWPORT);
    if let Ok(vv_val) = Reflect::get(win.as_ref(), &vv_key)
        && let Ok(vv_target) = vv_val.clone().dyn_into::<EventTarget>()
    {
        let win_vv: Window = win.clone();
        let vv_obj: JsValue = vv_val;
        let on_vv_resize: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
            let inner_h: f32 = win_vv
                .inner_height()
                .ok()
                .and_then(|v: JsValue| v.as_f64())
                .unwrap_or(0.0) as f32;
            let h_key: JsValue = JsValue::from_str(WEB_PROP_HEIGHT);
            let vv_h: f32 = Reflect::get(&vv_obj, &h_key)
                .ok()
                .and_then(|v: JsValue| v.as_f64())
                .unwrap_or(inner_h as f64) as f32;
            let kb_delta: f32 = (inner_h - vv_h).max(0.0);
            let visible: bool = kb_delta >= WEB_KEYBOARD_MIN_DELTA_PX;
            let height: f32 = if visible { kb_delta } else { 0.0 };
            web_dispatch_system_event(SystemEvent::KeyboardVisibilityChanged { visible, height });
        })
            as Box<dyn FnMut(Event)>);
        drop(vv_target.add_event_listener_with_callback(
            WEB_EVENT_RESIZE,
            on_vv_resize.as_ref().unchecked_ref(),
        ));
        on_vv_resize.forget();
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

/// Normalizes a browser `PointerEvent.pointerId` into a non-zero cross-platform pointer identifier.
///
/// # Arguments
///
/// - `&web_sys::PointerEvent` - Browser DOM pointer event.
///
/// # Returns
///
/// - `u64` - Stable non-zero pointer identifier.
#[cfg(target_arch = "wasm32")]
fn web_pointer_id_from_dom(ev: &web_sys::PointerEvent) -> u64 {
    (ev.pointer_id() as i64).unsigned_abs().saturating_add(1)
}

/// Converts a browser DOM `PointerEvent` into an `euv::PointerEvent` preserving pointer ID, touch/pen/mouse device, and button state.
///
/// # Arguments
///
/// - `&web_sys::PointerEvent` - Browser DOM pointer event.
/// - `&HtmlCanvasElement` - Target canvas element for local coordinate conversion.
/// - `Option<bool>` - Optional explicit pressed override (`Some(true)` on down, `Some(false)` on up, `None` to inspect `buttons()`).
///
/// # Returns
///
/// - `PointerEvent` - Cross-platform pointer event.
#[cfg(target_arch = "wasm32")]
fn web_pointer_from_dom(
    ev: &web_sys::PointerEvent,
    canvas: &HtmlCanvasElement,
    force_pressed: Option<bool>,
) -> PointerEvent {
    let rect: DomRect = canvas.get_bounding_client_rect();
    let x: f32 = (ev.client_x() as f64 - rect.left()) as f32;
    let y: f32 = (ev.client_y() as f64 - rect.top()) as f32;
    let pid: u64 = web_pointer_id_from_dom(ev);
    let ptype: String = ev.pointer_type();
    let device: PointerDevice = if ptype == WEB_PTR_TYPE_TOUCH {
        PointerDevice::Touch
    } else if ptype == WEB_PTR_TYPE_PEN {
        PointerDevice::Stylus
    } else {
        PointerDevice::Mouse
    };
    let is_pressed: bool =
        force_pressed.unwrap_or_else(|| ev.buttons() != 0 || device == PointerDevice::Touch);
    let pressure: f32 = if is_pressed { 1.0 } else { 0.0 };
    PointerEvent::new(
        pid,
        device,
        Point::new(x, y),
        PointerButton::Primary,
        pressure,
        Modifiers::default(),
    )
}

/// Enqueues a browser pointer move into the active WebAssembly runtime without synchronous full-frame rasterization.
///
/// # Arguments
///
/// - `PointerEvent` - Normalized pointer move event.
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_move_event(ptr: PointerEvent) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerMove(wid, ptr));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser pointer press into the active WebAssembly runtime for the next animation frame.
///
/// # Arguments
///
/// - `PointerEvent` - Normalized pointer press event.
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_down_event(ptr: PointerEvent) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerDown(wid, ptr));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser pointer release into the active WebAssembly runtime for the next animation frame.
///
/// # Arguments
///
/// - `PointerEvent` - Normalized pointer release event.
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_up_event(ptr: PointerEvent) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerUp(wid, ptr));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a browser pointer cancel (`0` cancels all active contacts) into the active WebAssembly runtime.
///
/// # Arguments
///
/// - `u64` - Cancelled pointer identifier (`0` cancels all active pointers).
#[cfg(target_arch = "wasm32")]
fn web_queue_pointer_cancel(pointer_id: u64) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerCancel(wid, pointer_id));
            web_mark_needs_frame();
        }
    });
}

/// Enqueues a normalized browser wheel scroll delta at `(x, y)` for coalesced VSync rendering.
///
/// # Arguments
///
/// - `f32` - Pointer X coordinate inside the canvas.
/// - `f32` - Pointer Y coordinate inside the canvas.
/// - `f32` - Horizontal scroll delta.
/// - `f32` - Vertical scroll delta.
#[cfg(target_arch = "wasm32")]
fn web_queue_scroll_at(x: f32, y: f32, dx: f32, dy: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::PointerMove(wid, PointerEvent::mouse(x, y)));
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
    if WEB_IME_COMPOSING.with(Cell::get) {
        return;
    }
    let ime_active: bool = WEB_IME_ACTIVE.with(Cell::get);
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            let code: KeyCode = web_map_key(key, code_str);
            let key_ev: WindowEvent =
                WindowEvent::KeyDown(wid, KeyEvent::new(code, Modifiers::default(), false));
            app.dispatch_window_event(key_ev);
            if !ime_active && key.chars().count() == 1 {
                app.dispatch_window_event(WindowEvent::TextInput(wid, key.to_string()));
            }
            web_mark_needs_frame();
        }
    });
}

/// Dispatches committed browser IME or virtual keyboard text into the active WebAssembly runtime.
///
/// # Arguments
///
/// - `String` - Committed UTF-8 text from the hidden IME proxy input.
#[cfg(target_arch = "wasm32")]
fn web_queue_text_input(text: String) {
    if text.is_empty() {
        return;
    }
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let wid: WindowId = app.get_window_state().get_id();
            app.dispatch_window_event(WindowEvent::TextInput(wid, text));
            web_mark_needs_frame();
        }
    });
}

/// Reads and clears the hidden browser IME proxy `<input>` value, dispatching any pending text into `euv`.
///
/// # Arguments
///
/// - `&HtmlElement` - Hidden browser IME proxy element.
#[cfg(target_arch = "wasm32")]
fn web_flush_ime_element(el: &HtmlElement) {
    let key: JsValue = JsValue::from_str(WEB_PROP_VALUE);
    if let Ok(val) = Reflect::get(el.as_ref(), &key)
        && let Some(text) = val.as_string()
        && !text.is_empty()
    {
        let _: Result<bool, JsValue> = Reflect::set(el.as_ref(), &key, &JsValue::from_str(""));
        web_queue_text_input(text);
    }
}

/// Attaches a visually hidden DOM `<input>` element so mobile browsers and CJK IMEs open native keyboards on input focus.
///
/// # Arguments
///
/// - `&Document` - Active browser DOM document.
#[cfg(target_arch = "wasm32")]
fn web_attach_ime_proxy(doc: &Document) {
    let Ok(raw_el) = doc.create_element(WEB_IME_INPUT_TAG) else {
        return;
    };
    let el: Element = raw_el;
    drop(el.set_attribute(WEB_IME_ATTR_ID, WEB_IME_INPUT_ID));
    drop(el.set_attribute(WEB_IME_ATTR_TYPE, WEB_IME_ATTR_TEXT));
    drop(el.set_attribute(WEB_IME_ATTR_AUTOCOMPLETE, WEB_IME_ATTR_OFF));
    drop(el.set_attribute(WEB_IME_ATTR_AUTOCORRECT, WEB_IME_ATTR_OFF));
    drop(el.set_attribute(WEB_IME_ATTR_AUTOCAPITALIZE, WEB_IME_ATTR_OFF));
    drop(el.set_attribute(WEB_IME_ATTR_SPELLCHECK, WEB_IME_ATTR_FALSE));
    drop(el.set_attribute(WEB_IME_ATTR_ARIA_LABEL, WEB_IME_ARIA_LABEL));
    drop(el.set_attribute(WEB_IME_ATTR_STYLE, WEB_IME_STYLE));
    let Ok(html_el) = el.dyn_into::<HtmlElement>() else {
        return;
    };
    if let Some(body) = doc.body() {
        drop(body.append_child(&html_el));
    }
    let comp_start: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        WEB_IME_COMPOSING.with(|c: &Cell<bool>| c.set(true));
    }) as Box<dyn FnMut(Event)>);
    drop(html_el.add_event_listener_with_callback(
        WEB_EVENT_COMPOSITION_START,
        comp_start.as_ref().unchecked_ref(),
    ));
    comp_start.forget();
    let comp_el: HtmlElement = html_el.clone();
    let comp_end: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        WEB_IME_COMPOSING.with(|c: &Cell<bool>| c.set(false));
        web_flush_ime_element(&comp_el);
    }) as Box<dyn FnMut(Event)>);
    drop(html_el.add_event_listener_with_callback(
        WEB_EVENT_COMPOSITION_END,
        comp_end.as_ref().unchecked_ref(),
    ));
    comp_end.forget();
    let inp_el: HtmlElement = html_el.clone();
    let on_input: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        if !WEB_IME_COMPOSING.with(Cell::get) {
            web_flush_ime_element(&inp_el);
        }
    }) as Box<dyn FnMut(Event)>);
    drop(
        html_el
            .add_event_listener_with_callback(WEB_EVENT_INPUT, on_input.as_ref().unchecked_ref()),
    );
    on_input.forget();
    let on_ime_blur: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |_: Event| {
        WEB_IME_ACTIVE.with(|a: &Cell<bool>| a.set(false));
        WEB_IME_COMPOSING.with(|c: &Cell<bool>| c.set(false));
        WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
            if let Ok(mut borrow) = slot.try_borrow_mut()
                && let Some((app, _)) = borrow.as_mut()
            {
                let wid: WindowId = app.get_window_state().get_id();
                app.dispatch_window_event(WindowEvent::Ime(wid, ImeEvent::Disabled));
            }
        });
    }) as Box<dyn FnMut(Event)>);
    drop(
        html_el
            .add_event_listener_with_callback(WEB_EVENT_BLUR, on_ime_blur.as_ref().unchecked_ref()),
    );
    on_ime_blur.forget();
    WEB_IME_ELEMENT.with(|slot: &RefCell<Option<HtmlElement>>| {
        if let Ok(mut guard) = slot.try_borrow_mut() {
            *guard = Some(html_el);
        }
    });
}

/// Handles `euv` native action requests in the browser, focusing or blurring the hidden IME proxy input.
///
/// # Arguments
///
/// - `&str` - Action category (e.g. `NATIVE_ACTION_IME`).
/// - `&str` - Action subcommand (`"show"` or `"hide"`).
/// - `&str` - Secondary payload argument.
///
/// # Returns
///
/// - `bool` - `true` when handled by the browser bridge.
#[cfg(target_arch = "wasm32")]
fn web_native_action_bridge(kind: &str, arg1: &str, _arg2: &str) -> bool {
    if kind != NATIVE_ACTION_IME {
        return false;
    }
    WEB_IME_ELEMENT.with(|slot: &RefCell<Option<HtmlElement>>| {
        let Ok(guard) = slot.try_borrow() else {
            return false;
        };
        let Some(el) = guard.as_ref() else {
            return false;
        };
        if arg1 == WEB_IME_ACTION_SHOW {
            WEB_IME_ACTIVE.with(|a: &Cell<bool>| a.set(true));
            if let Some(win) = window()
                && let Some(doc) = win.document()
                && let Some(active_el) = doc.active_element()
                && active_el.id() == WEB_IME_INPUT_ID
            {
                drop(el.blur());
            }
            drop(el.focus());
            true
        } else if arg1 == WEB_IME_ACTION_HIDE {
            WEB_IME_ACTIVE.with(|a: &Cell<bool>| a.set(false));
            WEB_IME_COMPOSING.with(|c: &Cell<bool>| c.set(false));
            let key: JsValue = JsValue::from_str(WEB_PROP_VALUE);
            let _: Result<bool, JsValue> = Reflect::set(el.as_ref(), &key, &JsValue::from_str(""));
            drop(el.blur());
            true
        } else {
            false
        }
    })
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

/// Synchronizes the WebAssembly `<canvas>` backing pixels and `App` viewport with exact 1:1 physical device pixels.
///
/// # Arguments
///
/// - `&Window` - Browser window object.
/// - `&HtmlCanvasElement` - Target full-viewport canvas element.
///
/// # Returns
///
/// - `bool` - `true` when physical dimensions or DPI scale changed and the viewport was updated.
#[cfg(target_arch = "wasm32")]
fn web_sync_exact_viewport(win: &Window, canvas: &HtmlCanvasElement) -> bool {
    let rect: DomRect = canvas.get_bounding_client_rect();
    let css_w: f64 = if rect.width() >= 320.0 {
        rect.width()
    } else {
        win.inner_width()
            .ok()
            .and_then(|v: JsValue| v.as_f64())
            .filter(|w: &f64| *w >= 320.0)
            .unwrap_or(1200.0)
    };
    let css_h: f64 = if rect.height() >= 320.0 {
        rect.height()
    } else {
        win.inner_height()
            .ok()
            .and_then(|v: JsValue| v.as_f64())
            .filter(|h: &f64| *h >= 320.0)
            .unwrap_or(800.0)
    };
    let dpr_raw: f64 = win.device_pixel_ratio();
    let dpr_f64: f64 = if dpr_raw.is_finite() && dpr_raw > 0.0 {
        dpr_raw.clamp(0.5, 4.5)
    } else {
        1.0
    };
    let dpr: f32 = dpr_f64 as f32;
    let phys_w: u32 = (css_w * dpr_f64).round().max(1.0) as u32;
    let phys_h: u32 = (css_h * dpr_f64).round().max(1.0) as u32;
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let Ok(mut borrow) = slot.try_borrow_mut() else {
            return false;
        };
        let Some((app, _)) = borrow.as_mut() else {
            return false;
        };
        let ws: WindowState = app.get_window_state();
        let size_diff: bool = ws.get_physical_width() != phys_w
            || ws.get_physical_height() != phys_h
            || (ws.get_scale_factor() - dpr).abs() > 0.005
            || canvas.width() != phys_w
            || canvas.height() != phys_h;
        if !size_diff {
            return false;
        }
        if canvas.width() != phys_w {
            canvas.set_width(phys_w);
        }
        if canvas.height() != phys_h {
            canvas.set_height(phys_h);
        }
        app.set_exact_physical_viewport(phys_w, phys_h, dpr);
        true
    })
}

/// Reports measured browser `requestAnimationFrame` refresh cadence into the active `App` runtime.
///
/// # Arguments
///
/// - `f32` - Measured browser presentation cadence in Hertz.
#[cfg(target_arch = "wasm32")]
fn web_report_host_refresh_rate(measured_hz: f32) {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        if let Ok(mut borrow) = slot.try_borrow_mut()
            && let Some((app, _)) = borrow.as_mut()
        {
            let prev_fps: f32 = app.get_context().get_live_fps().get_untracked();
            app.report_host_refresh_rate(measured_hz);
            let next_fps: f32 = app.get_context().get_live_fps().get_untracked();
            if (next_fps - prev_fps).abs() >= 1.0 {
                web_mark_needs_frame();
            }
        }
    });
}

/// Steps one coalesced WebAssembly frame during `requestAnimationFrame` with accurate real-time `dt_secs`.
///
/// # Arguments
///
/// - `f32` - Elapsed seconds since the previous animation frame.
///
/// # Returns
///
/// - `u64` - Rendered frame hash.
#[cfg(target_arch = "wasm32")]
fn web_flush_frame(dt_secs: f32) -> u64 {
    WEB_SHOWCASE_RUNTIME.with(|slot: &RefCell<Option<(App, ShowcaseState)>>| {
        let mut borrow: RefMut<'_, Option<(App, ShowcaseState)>> = match slot.try_borrow_mut() {
            Ok(guard) => guard,
            Err(_) => return 0,
        };
        let Some((app, _)) = borrow.as_mut() else {
            return 0;
        };
        app.step_game_frame(dt_secs)
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
            if let Some(win) = window()
                && let Ok(hash_str) = win.location().hash()
            {
                let cleaned: &str = hash_str.trim_start_matches('#');
                if cleaned.starts_with('/') {
                    let _: bool = state.get_route().set_if_changed(cleaned.to_string());
                }
            }
            let route_watch: Signal<String> = state.get_route();
            let (hash_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
                Effect::new(move || {
                    let r: String = route_watch.get();
                    if let Some(win) = window() {
                        let target_hash: String = format!("#{r}");
                        if win.location().hash().ok().as_deref() != Some(target_hash.as_str()) {
                            let _: Result<(), JsValue> = win.location().set_hash(&target_hash);
                        }
                    }
                })
            });
            let root_id: NodeId = app
                .get_scene()
                .with_read(SceneTree::get_root, NodeId::default());
            app.get_scene().with_write(
                |tree: &mut SceneTree| {
                    tree.attach_scope(root_id, hash_scope);
                },
                (),
            );
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
        if !app.should_pump_frames() {
            return app.get_renderer().get_last_stats().get_frame_hash();
        }
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
            dpr.clamp(0.5, 4.5)
        } else {
            1.0
        };
        let phys_w: u32 = ((w as f64) * (scale as f64)).round().max(1.0) as u32;
        let phys_h: u32 = ((h as f64) * (scale as f64)).round().max(1.0) as u32;
        if let Some(win) = window()
            && let Some(doc) = win.document()
            && let Ok(Some(el)) = doc.query_selector(WEB_SELECTOR_CANVAS)
            && let Ok(canvas) = el.dyn_into::<HtmlCanvasElement>()
        {
            if canvas.width() != phys_w {
                canvas.set_width(phys_w);
            }
            if canvas.height() != phys_h {
                canvas.set_height(phys_h);
            }
        }
        app.set_exact_physical_viewport(phys_w, phys_h, scale);
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
        let Some((app, state)) = borrow.as_mut() else {
            return String::new();
        };
        let hooks: AiAppHooks = state.build_ai_hooks();
        AiSchemaProtocol::handle_json_with_hooks(app, request_json, Some(&hooks))
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
