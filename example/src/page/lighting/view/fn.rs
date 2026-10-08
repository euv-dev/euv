use super::*;

/// A standalone Phong lighting demo page with three rendering backends.
///
/// All three tabs render the same static scene — five shaded spheres
/// plus a ground line, authored in a fixed 320x240 logical space and
/// lit by one directional sun and one point lamp — through different
/// backends:
///
/// - **2D**: a CPU-side per-pixel lighting pass writing a persistent
///   RGBA framebuffer with a single `put_image_data` per frame, with
///   2x2 sub-sample coverage and an adaptive internal resolution ladder.
/// - **GL**: a WebGL 2 GLSL ES 3.00 fragment shader mirroring the same
///   `LightingUniforms::shade` math.
/// - **GPU**: the same pipeline expressed in WGSL on WebGPU.
///
/// Every tab reports an honest wall-clock FPS.
///
/// # Arguments
///
/// - `VirtualNode<PageLightingProps>` - The page component node carrying the
///   page props.
///
/// # Returns
/// - `VirtualNode` - The lighting page virtual DOM tree.
#[component]
pub(crate) fn page_lighting(node: VirtualNode<PageLightingProps>) -> VirtualNode {
    let _page_lighting_props: PageLightingProps = node.try_get_props().unwrap_or_default();
    let tab: Signal<LightingTab> = App::use_signal(LightingTab::default);
    let fullscreen: UseLightingFullscreen = use_lighting_fullscreen_state();
    use_lighting_fullscreen_popstate(fullscreen);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "💡"
                title: LIGHTING_HEADER_TITLE
                subtitle: LIGHTING_HEADER_SUBTITLE
            }
            euv_card {
                title: LIGHTING_CANVAS_CARD_TITLE
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == LightingTab::Canvas2D } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: lighting_on_tab_select(tab, LightingTab::Canvas2D, fullscreen)
                        "2D"
                    }
                    div {
                        class: if { tab.get() == LightingTab::WebGl } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: lighting_on_tab_select(tab, LightingTab::WebGl, fullscreen)
                        "GL"
                    }
                    div {
                        class: if { tab.get() == LightingTab::WebGpu } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: lighting_on_tab_select(tab, LightingTab::WebGpu, fullscreen)
                        "GPU"
                    }
                }
                match { tab } {
                    LightingTab::Canvas2D => {
                        div {
                            lighting_canvas_tab(fullscreen)
                        }
                    }
                    LightingTab::WebGl => {
                        div {
                            lighting_webgl_tab(use_lighting_webgl_state(), fullscreen)
                        }
                    }
                    LightingTab::WebGpu => {
                        div {
                            lighting_webgpu_tab(use_lighting_webgpu_state(), fullscreen)
                        }
                    }
                }
            }
            euv_card {
                title: LIGHTING_BACKENDS_CARD_TITLE
                match { tab } {
                    LightingTab::Canvas2D => {
                        p {
                            class: c_game_description()
                            LIGHTING_CANVAS_DESCRIPTION
                        }
                    }
                    LightingTab::WebGl => {
                        p {
                            class: c_game_description()
                            LIGHTING_WEBGL_DESCRIPTION
                        }
                    }
                    LightingTab::WebGpu => {
                        p {
                            class: c_game_description()
                            LIGHTING_WEBGPU_DESCRIPTION
                        }
                    }
                }
            }
        }
    }
}

/// Renders the Canvas 2D software lighting tab content.
///
/// Contains the full Canvas 2D demo: stats bar (FPS, lights, adaptive
/// render scale), canvas, and controls.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The page fullscreen state.
///
/// # Returns
///
/// - `VirtualNode` - The Canvas 2D tab virtual DOM tree.
fn lighting_canvas_tab(fullscreen: UseLightingFullscreen) -> VirtualNode {
    let state: UseLighting = use_lighting_state();
    let canvas_2d_fullscreen: Signal<bool> = fullscreen.get_canvas_2d();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_lighting_loop(state);
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = lighting_on_toggle_pause(state.get_running());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let scale_display: String = format!("{:.0}%", state.get_render_scale().get() * 100.0);
    let active: bool = state.get_active().get();
    // Read the init error code even though `lighting_canvas_status_text`
    // does not consume it (the Canvas 2D tab has no async init that can
    // fail), so the signal is wired through and the three Lighting tabs
    // stay symmetric with each other. If a future Canvas 2D init step
    // becomes fallible, `lighting_canvas_status_text` can fold the code
    // in without any view-side changes.
    let _init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = lighting_canvas_status_text(active);
    let pause_label: &str = if state.get_running().get() {
        LIGHTING_PAUSE_LABEL
    } else {
        LIGHTING_RESUME_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    LIGHTING_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_LIGHTS_SUMMARY
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_SCALE_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        scale_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_STATUS_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        status_text
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
                            id: "lighting-canvas"
                            class: if { canvas_2d_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "lighting-loading-canvas"
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
                            label: LIGHTING_EXIT_BUTTON_LABEL
                            onclick: lighting_on_exit_fullscreen(canvas_2d_fullscreen)
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
                    label: LIGHTING_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: lighting_on_enter_fullscreen(canvas_2d_fullscreen)
                }
            }
        }
    }
}

/// Maps the Canvas 2D init state to the banner text shown next to "Status: ".
///
/// The Canvas 2D tab has no asynchronous init that can fail (the 2D
/// context acquire is synchronous in the first frame), so unlike the
/// WebGPU banner this does not need a full capability decision tree:
/// the only two states are "warmup" and "live".
///
/// # Arguments
///
/// - `bool` - Whether the renderer is active.
///
/// # Returns
///
/// - `&'static str` - The banner text.
fn lighting_canvas_status_text(active: bool) -> &'static str {
    let text: &'static str = if active {
        LIGHTING_CANVAS_2D_ACTIVE_STATUS
    } else {
        LIGHTING_INITIALIZING_STATUS
    };
    text
}

/// Maps the WebGL init state plus the engine's stable error code to the
/// banner text shown next to "Status: ".
///
/// WebGL 2 is supported by every modern browser, so unlike the WebGPU
/// banner this does not need a full capability decision tree: an init
/// failure is almost always "browser too old" or a driver blocklist hit.
///
/// # Arguments
///
/// - `bool` - Whether initialization has finished (success or failure).
/// - `bool` - Whether the renderer is active.
/// - `&str` - The `WebGl2InitError::code()` from the last init attempt.
///
/// # Returns
///
/// - `&'static str` - The banner text.
fn lighting_webgl_status_text(loaded: bool, active: bool, init_error_code: &str) -> &'static str {
    if !loaded {
        return LIGHTING_INITIALIZING_STATUS;
    }
    if active {
        return LIGHTING_WEBGL_ACTIVE_STATUS;
    }
    if init_error_code.is_empty() {
        LIGHTING_WEBGL_NOT_SUPPORTED_STATUS
    } else {
        LIGHTING_WEBGL_INIT_FAILED_STATUS
    }
}

/// Renders the WebGL lighting tab content.
///
/// Mirrors the Canvas 2D tab: the same static scene, rendered through a
/// GLSL ES 3.00 program instead of the 2D context. Adds a WebGL status
/// readout to the stats bar.
///
/// # Arguments
///
/// - `UseLightingWebGl` - The WebGL backend state.
/// - `UseLightingFullscreen` - The page fullscreen state.
///
/// # Returns
///
/// - `VirtualNode` - The WebGL tab virtual DOM tree.
fn lighting_webgl_tab(state: UseLightingWebGl, fullscreen: UseLightingFullscreen) -> VirtualNode {
    let web_gl_fullscreen: Signal<bool> = fullscreen.get_web_gl();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_lighting_webgl_loop(state);
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = lighting_on_toggle_pause(state.get_running());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = lighting_webgl_status_text(loaded, active, init_error_code);
    let pause_label: &str = if state.get_running().get() {
        LIGHTING_PAUSE_LABEL
    } else {
        LIGHTING_RESUME_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    LIGHTING_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_LIGHTS_SUMMARY
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_STATUS_PREFIX
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
                            id: "lighting-webgl-canvas"
                            class: if { web_gl_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "lighting-webgl-loading-canvas"
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
                            label: LIGHTING_EXIT_BUTTON_LABEL
                            onclick: lighting_on_exit_fullscreen(web_gl_fullscreen)
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
                    label: LIGHTING_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: lighting_on_enter_fullscreen(web_gl_fullscreen)
                }
            }
        }
    }
}

/// Renders the WebGPU lighting tab content.
///
/// Mirrors the Canvas 2D tab: the same static scene, rendered through a
/// WGSL pipeline instead of the 2D context. Adds a WebGPU status
/// readout to the stats bar.
///
/// # Arguments
///
/// - `UseLightingWebGpu` - The WebGPU backend state.
/// - `UseLightingFullscreen` - The page fullscreen state.
///
/// # Returns
///
/// - `VirtualNode` - The WebGPU tab virtual DOM tree.
fn lighting_webgpu_tab(state: UseLightingWebGpu, fullscreen: UseLightingFullscreen) -> VirtualNode {
    let web_gpu_fullscreen: Signal<bool> = fullscreen.get_web_gpu();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_lighting_webgpu_loop(state);
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = lighting_on_toggle_pause(state.get_running());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = webgpu_status_text(loaded, active, init_error_code);
    let pause_label: &str = if state.get_running().get() {
        LIGHTING_PAUSE_LABEL
    } else {
        LIGHTING_RESUME_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    LIGHTING_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_LIGHTS_SUMMARY
                }
                span {
                    class: c_game_stats_label()
                    LIGHTING_STATUS_PREFIX
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
                            id: "lighting-webgpu-canvas"
                            class: if { web_gpu_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "lighting-webgpu-loading-canvas"
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
                            label: LIGHTING_EXIT_BUTTON_LABEL
                            onclick: lighting_on_exit_fullscreen(web_gpu_fullscreen)
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
                    label: LIGHTING_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: lighting_on_enter_fullscreen(web_gpu_fullscreen)
                }
            }
        }
    }
}
