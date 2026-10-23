use super::*;

/// A standalone interactive ray-tracing demo page with three rendering
/// backends.
///
/// All three tabs render the *same* static scene through euv-engine's
/// shared `RayTraceRenderer`, so the tabs differ only in how the engine
/// reaches the pixels:
///
/// - **2D**: a CPU-side software ray tracer writing a persistent RGBA
///   framebuffer with a single `put_image_data` per frame, with 2x2
///   SSAA and an adaptive internal resolution ladder.
/// - **GL**: a WebGL 2 GLSL ES 3.00 fragment shader traversing the
///   engine's packed BVH out of a data texture.
/// - **GPU**: the same traversal expressed in WGSL on WebGPU.
///
/// The scene mixes every primitive kind the engine traces (an AABB, two
/// spheres, a triangle) and is lit by one directional sun, one point
/// fill, and one spot rim. It is built once and never rebuilt, because
/// nothing in it moves with the camera: the GPU backends upload it at
/// `init` and never again. This is shader BVH traversal, not native
/// hardware ray tracing.
///
/// The camera is an orbit camera whose yaw / pitch are shared across
/// tabs and driven by mouse drag, touch drag, or auto-rotation. The FPS
/// counter reports honest wall-clock rates on every tab, and each tab
/// surfaces the engine's own rejection text when a call fails.
///
/// # Arguments
///
/// - `VirtualNode<PageRaytraceProps>` - The page component node carrying the
///   page props.
///
/// # Returns
/// - `VirtualNode` - The raytrace page virtual DOM tree.
#[component]
pub(crate) fn page_raytrace(node: VirtualNode<PageRaytraceProps>) -> VirtualNode {
    let _page_raytrace_props: PageRaytraceProps = node.try_get_props().unwrap_or_default();
    let tab: Signal<RayTraceTab> = App::use_signal(RayTraceTab::default);
    let fullscreen: UseRayTraceFullscreen = use_raytrace_fullscreen_state();
    use_raytrace_fullscreen_popstate(fullscreen);
    let angles_store: Signal<RayTraceCameraAngles> = App::use_signal(RayTraceCameraAngles::default);
    let angles: RayTraceCameraAngles = angles_store.get();
    let last_pointer: PointerPositionSignal = App::use_signal(|| Rc::new(Cell::new(None)));
    let pointer_cell: Rc<Cell<Option<(f64, f64)>>> = last_pointer.get();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔦"
                title: RAYTRACE_HEADER_TITLE
                subtitle: RAYTRACE_HEADER_SUBTITLE
            }
            euv_card {
                title: match { tab.get() } {
                    RayTraceTab::Canvas2D => RAYTRACE_TAB_TITLE_CANVAS_2D,
                    RayTraceTab::WebGl => RAYTRACE_TAB_TITLE_WEBGL,
                    RayTraceTab::WebGpu => RAYTRACE_TAB_TITLE_WEBGPU,
                }
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == RayTraceTab::Canvas2D } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: raytrace_on_tab_select(tab, RayTraceTab::Canvas2D, fullscreen)
                        "2D"
                    }
                    div {
                        class: if { tab.get() == RayTraceTab::WebGl } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: raytrace_on_tab_select(tab, RayTraceTab::WebGl, fullscreen)
                        "GL"
                    }
                    div {
                        class: if { tab.get() == RayTraceTab::WebGpu } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: raytrace_on_tab_select(tab, RayTraceTab::WebGpu, fullscreen)
                        "GPU"
                    }
                }
                match { tab } {
                    RayTraceTab::Canvas2D => {
                        div {
                            raytrace_canvas_tab(angles.clone(), pointer_cell.clone(), fullscreen)
                        }
                    }
                    RayTraceTab::WebGl => {
                        div {
                            raytrace_webgl_tab(
                            use_raytrace_webgl_state(),
                            angles.clone(),
                            pointer_cell.clone(),
                            fullscreen,
                            )
                        }
                    }
                    RayTraceTab::WebGpu => {
                        div {
                            raytrace_webgpu_tab(
                            use_raytrace_webgpu_state(),
                            angles.clone(),
                            pointer_cell.clone(),
                            fullscreen,
                            )
                        }
                    }
                }
            }
            euv_card {
                title: RAYTRACE_BACKENDS_CARD_TITLE
                match { tab } {
                    RayTraceTab::Canvas2D => {
                        p {
                            class: c_game_description()
                            RAYTRACE_CANVAS_DESCRIPTION
                        }
                    }
                    RayTraceTab::WebGl => {
                        p {
                            class: c_game_description()
                            RAYTRACE_WEBGL_DESCRIPTION
                        }
                    }
                    RayTraceTab::WebGpu => {
                        p {
                            class: c_game_description()
                            RAYTRACE_WEBGPU_DESCRIPTION
                        }
                    }
                }
            }
        }
    }
}

/// Renders the Canvas 2D software raytracing tab content.
///
/// Contains the full Canvas 2D demo: stats bar (FPS, scene, adaptive
/// render scale, backend status), canvas, and controls.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
/// - `UseRayTraceFullscreen` - The page fullscreen state.
///
/// # Returns
///
/// - `VirtualNode` - The Canvas 2D tab virtual DOM tree.
fn raytrace_canvas_tab(
    angles: RayTraceCameraAngles,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
    fullscreen: UseRayTraceFullscreen,
) -> VirtualNode {
    let state: UseRayTrace = use_raytrace_state();
    let canvas_2d_fullscreen: Signal<bool> = fullscreen.get_canvas_2d();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_raytrace_loop(state, angles.clone());
    }
    let running: Signal<bool> = state.get_running();
    let auto_rotate: Signal<bool> = state.get_auto_rotate();
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = raytrace_on_toggle_pause(running);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_toggle_auto_rotate(auto_rotate);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = raytrace_on_reset_camera(angles.clone());
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_down(last_pointer.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_pointer_move(angles.clone(), auto_rotate, last_pointer.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_up(last_pointer.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_start(last_pointer.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_touch_move(angles.clone(), auto_rotate, last_pointer.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_end(last_pointer.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let scale_display: String = format!("{:.0}%", state.get_render_scale().get() * 100.0);
    let canvas_2d_loaded: bool = state.get_loaded().get();
    let canvas_2d_active: bool = state.get_active().get();
    let canvas_2d_error_code: &str = state.get_init_error_code().get();
    let canvas_2d_status: String = raytrace_status_text(
        canvas_2d_loaded,
        canvas_2d_active,
        canvas_2d_error_code,
        &state.get_error_detail().get(),
    );
    let pause_label: &str = if state.get_running().get() {
        RAYTRACE_PAUSE_LABEL
    } else {
        RAYTRACE_RESUME_LABEL
    };
    let auto_rotate_label: &str = if state.get_auto_rotate().get() {
        RAYTRACE_AUTO_ROTATE_ON_LABEL
    } else {
        RAYTRACE_AUTO_ROTATE_OFF_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    RAYTRACE_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_SCENE_SUMMARY
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_SCALE_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        scale_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_STATUS_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        canvas_2d_status
                    }
                }
            }
            div {
                class: if { canvas_2d_fullscreen.get() } {
                    c_game_container_fullscreen()
                } else {
                    c_game_canvas_wrapper()
                }
                div {
                    class: c_game_fullscreen_canvas_wrapper()
                    div {
                        class: c_game_fullscreen_canvas_letterbox()
                        canvas {
                            id: "raytrace-canvas"
                            class: if { canvas_2d_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                            onmousedown: on_pointer_down.clone()
                            onmousemove: on_pointer_move.clone()
                            onmouseup: on_pointer_up.clone()
                            onmouseleave: on_pointer_up.clone()
                            ontouchstart: on_touch_start.clone()
                            ontouchmove: on_touch_move.clone()
                            ontouchend: on_touch_end.clone()
                            ontouchcancel: on_touch_end.clone()
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "raytrace-loading-canvas"
                                class: c_game_loading_overlay()
                            }
                        }
                    }
                }
                if { canvas_2d_fullscreen.get() } {
                    div {
                        class: c_game_fullscreen_toolbar()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: RAYTRACE_EXIT_BUTTON_LABEL
                            onclick: raytrace_on_exit_fullscreen(canvas_2d_fullscreen)
                        }
                    }
                }
            }
            div {
                class: c_button_controls()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: pause_label
                    onclick: on_toggle_pause
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: auto_rotate_label
                    onclick: on_toggle_auto_rotate
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: RAYTRACE_RESET_CAMERA_BUTTON_LABEL
                    onclick: on_reset_camera
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: RAYTRACE_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: raytrace_on_enter_fullscreen(canvas_2d_fullscreen)
                }
            }
        }
    }
}

/// Maps a backend's init state plus the engine's stable error code to the
/// banner text shown next to "Status: ".
///
/// WebGL 2 is supported by every modern browser, so unlike the WebGPU
/// banner this does not need a full capability decision tree: an init
/// failure is almost always "browser too old" or a driver blocklist hit.
///
/// The `active` flag is checked *before* the error code on purpose. The
/// loop clears `active` on every rejected call, so a backend that failed
/// mid-run can never fall through to the "not supported" wording and claim
/// the browser lacks a feature it in fact ran.
///
/// # Arguments
///
/// - `bool` - Whether initialization has finished (success or failure).
/// - `&str` - The stable error code from the last rejected call.
///
/// # Returns
///
/// - `String` - The banner text, including the engine's own message when
///   there is one.
fn raytrace_status_text(loaded: bool, active: bool, init_error_code: &str, detail: &str) -> String {
    let status: &str = if !loaded {
        return RAYTRACE_INITIALIZING_STATUS.to_string();
    } else if active {
        RAYTRACE_WEBGL_ACTIVE_STATUS
    } else if init_error_code.is_empty() {
        RAYTRACE_WEBGL_NOT_SUPPORTED_STATUS
    } else {
        RAYTRACE_WEBGL_INIT_FAILED_STATUS
    };
    raytrace_status_with_detail(status, detail)
}

/// Appends the engine's rejection detail to a status line.
///
/// # Arguments
///
/// - `&str` - The base status text.
///
/// # Returns
///
/// - `String` - The status line.
pub(crate) fn raytrace_status_with_detail(status: &str, detail: &str) -> String {
    if detail.is_empty() {
        return status.to_string();
    }
    format!("{status}{RAYTRACE_STATUS_DETAIL_SEPARATOR}{detail}")
}

/// Renders the WebGL raytracing tab content.
///
/// Mirrors the Canvas 2D tab: the same scene, orbit camera, and
/// pointer/touch drag, rendered through the engine's GLSL ES 3.00 program
/// instead of the 2D context. Adds a WebGL status readout to the stats
/// bar.
///
/// # Arguments
///
/// - `UseRayTraceWebGl` - The WebGL backend state.
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
/// - `UseRayTraceFullscreen` - The page fullscreen state.
///
/// # Returns
///
/// - `VirtualNode` - The WebGL tab virtual DOM tree.
fn raytrace_webgl_tab(
    state: UseRayTraceWebGl,
    angles: RayTraceCameraAngles,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
    fullscreen: UseRayTraceFullscreen,
) -> VirtualNode {
    let web_gl_fullscreen: Signal<bool> = fullscreen.get_web_gl();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_raytrace_webgl_loop(state, angles.clone());
    }
    let running: Signal<bool> = state.get_running();
    let auto_rotate: Signal<bool> = state.get_auto_rotate();
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = raytrace_on_toggle_pause(running);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_toggle_auto_rotate(auto_rotate);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = raytrace_on_reset_camera(angles.clone());
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_down(last_pointer.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_pointer_move(angles.clone(), auto_rotate, last_pointer.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_up(last_pointer.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_start(last_pointer.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_touch_move(angles.clone(), auto_rotate, last_pointer.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_end(last_pointer.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: String = raytrace_status_text(
        loaded,
        active,
        init_error_code,
        &state.get_error_detail().get(),
    );
    let pause_label: &str = if state.get_running().get() {
        RAYTRACE_PAUSE_LABEL
    } else {
        RAYTRACE_RESUME_LABEL
    };
    let auto_rotate_label: &str = if state.get_auto_rotate().get() {
        RAYTRACE_AUTO_ROTATE_ON_LABEL
    } else {
        RAYTRACE_AUTO_ROTATE_OFF_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    RAYTRACE_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_SCENE_SUMMARY
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_STATUS_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        status_text
                    }
                }
            }
            div {
                class: if { web_gl_fullscreen.get() } {
                    c_game_container_fullscreen()
                } else {
                    c_game_canvas_wrapper()
                }
                div {
                    class: c_game_fullscreen_canvas_wrapper()
                    div {
                        class: c_game_fullscreen_canvas_letterbox()
                        canvas {
                            id: "raytrace-webgl-canvas"
                            class: if { web_gl_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                            onmousedown: on_pointer_down.clone()
                            onmousemove: on_pointer_move.clone()
                            onmouseup: on_pointer_up.clone()
                            onmouseleave: on_pointer_up.clone()
                            ontouchstart: on_touch_start.clone()
                            ontouchmove: on_touch_move.clone()
                            ontouchend: on_touch_end.clone()
                            ontouchcancel: on_touch_end.clone()
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "raytrace-webgl-loading-canvas"
                                class: c_game_loading_overlay()
                            }
                        }
                    }
                }
                if { web_gl_fullscreen.get() } {
                    div {
                        class: c_game_fullscreen_toolbar()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: RAYTRACE_EXIT_BUTTON_LABEL
                            onclick: raytrace_on_exit_fullscreen(web_gl_fullscreen)
                        }
                    }
                }
            }
            div {
                class: c_button_controls()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: pause_label
                    onclick: on_toggle_pause
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: auto_rotate_label
                    onclick: on_toggle_auto_rotate
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: RAYTRACE_RESET_CAMERA_BUTTON_LABEL
                    onclick: on_reset_camera
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: RAYTRACE_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: raytrace_on_enter_fullscreen(web_gl_fullscreen)
                }
            }
        }
    }
}

/// Renders the WebGPU raytracing tab content.
///
/// Mirrors the Canvas 2D tab: the same scene, orbit camera, and
/// pointer/touch drag, rendered through the engine's WGSL pipeline
/// instead of the 2D context. Adds a WebGPU status readout to the stats
/// bar.
///
/// # Arguments
///
/// - `UseRayTraceWebGpu` - The WebGPU backend state.
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
/// - `UseRayTraceFullscreen` - The page fullscreen state.
///
/// # Returns
///
/// - `VirtualNode` - The WebGPU tab virtual DOM tree.
fn raytrace_webgpu_tab(
    state: UseRayTraceWebGpu,
    angles: RayTraceCameraAngles,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
    fullscreen: UseRayTraceFullscreen,
) -> VirtualNode {
    let web_gpu_fullscreen: Signal<bool> = fullscreen.get_web_gpu();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_raytrace_webgpu_loop(state, angles.clone());
    }
    let running: Signal<bool> = state.get_running();
    let auto_rotate: Signal<bool> = state.get_auto_rotate();
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = raytrace_on_toggle_pause(running);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_toggle_auto_rotate(auto_rotate);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = raytrace_on_reset_camera(angles.clone());
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_down(last_pointer.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_pointer_move(angles.clone(), auto_rotate, last_pointer.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_up(last_pointer.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_start(last_pointer.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_touch_move(angles.clone(), auto_rotate, last_pointer.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_end(last_pointer.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    // `webgpu_status_text` resolves the *capability* question (no WebGPU at
    // all, adapter unavailable, device refused). A backend that acquired a
    // device and then failed is not a capability problem, so the engine's
    // own detail is appended to whatever wording that helper picked.
    let status_text: String = raytrace_status_with_detail(
        webgpu_status_text(loaded, active, init_error_code),
        &state.get_error_detail().get(),
    );
    let pause_label: &str = if state.get_running().get() {
        RAYTRACE_PAUSE_LABEL
    } else {
        RAYTRACE_RESUME_LABEL
    };
    let auto_rotate_label: &str = if state.get_auto_rotate().get() {
        RAYTRACE_AUTO_ROTATE_ON_LABEL
    } else {
        RAYTRACE_AUTO_ROTATE_OFF_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    RAYTRACE_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_SCENE_SUMMARY
                }
                span {
                    class: c_game_stats_label()
                    RAYTRACE_STATUS_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        status_text
                    }
                }
            }
            div {
                class: if { web_gpu_fullscreen.get() } {
                    c_game_container_fullscreen()
                } else {
                    c_game_canvas_wrapper()
                }
                div {
                    class: c_game_fullscreen_canvas_wrapper()
                    div {
                        class: c_game_fullscreen_canvas_letterbox()
                        canvas {
                            id: "raytrace-webgpu-canvas"
                            class: if { web_gpu_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                            onmousedown: on_pointer_down.clone()
                            onmousemove: on_pointer_move.clone()
                            onmouseup: on_pointer_up.clone()
                            onmouseleave: on_pointer_up.clone()
                            ontouchstart: on_touch_start.clone()
                            ontouchmove: on_touch_move.clone()
                            ontouchend: on_touch_end.clone()
                            ontouchcancel: on_touch_end.clone()
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "raytrace-webgpu-loading-canvas"
                                class: c_game_loading_overlay()
                            }
                        }
                    }
                }
                if { web_gpu_fullscreen.get() } {
                    div {
                        class: c_game_fullscreen_toolbar()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: RAYTRACE_EXIT_BUTTON_LABEL
                            onclick: raytrace_on_exit_fullscreen(web_gpu_fullscreen)
                        }
                    }
                }
            }
            div {
                class: c_button_controls()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: pause_label
                    onclick: on_toggle_pause
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: auto_rotate_label
                    onclick: on_toggle_auto_rotate
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: RAYTRACE_RESET_CAMERA_BUTTON_LABEL
                    onclick: on_reset_camera
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: RAYTRACE_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: raytrace_on_enter_fullscreen(web_gpu_fullscreen)
                }
            }
        }
    }
}
