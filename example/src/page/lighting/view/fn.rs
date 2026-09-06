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
                title: "Phong Lighting"
                subtitle: "A Phong shading demo rendered three ways: a Canvas 2D software path with an ImageData fast path and adaptive internal resolution, a WebGL 2 GLSL fragment-shader path, and a WebGPU WGSL path. All backends shade the same scene (5 spheres + 1 ground line, 1 directional sun + 1 point lamp) with 2x2 sub-sample coverage, and every tab reports an honest wall-clock FPS. Click Enter Fullscreen for a larger view."
            }
            euv_card {
                title: "Lighting Demo"
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
                title: "Lighting Backends"
                match { tab } {
                    LightingTab::Canvas2D => {
                        p {
                            class: c_game_description()
                            "The Canvas 2D tab runs euv-engine's lighting module on the CPU: every frame, every sphere pixel reconstructs the surface normal from its screen-space position (dz = sqrt(r^2 - d^2)) and feeds it to LightingUniforms::shade together with one directional sun and one point lamp, and the ground line is shaded with the same pipeline using a fixed up-pointing normal. Finished frames are packed into a persistent RGBA buffer (gamma 1/2.2, alpha 0 outside the shapes so the theme background shows through) and uploaded with a single put_image_data call, and an EMA of the CPU frame time steps the internal resolution through a 320x240 .. 80x60 ladder (always 4:3, so the fullscreen letterbox holds) to protect the frame rate. The FPS counter measures unclamped wall-clock time."
                        }
                    }
                    LightingTab::WebGl => {
                        p {
                            class: c_game_description()
                            "The WebGL tab runs the identical scene inside a GLSL ES 3.00 fragment shader drawn on an attribute-less fullscreen triangle (gl_VertexID, no vertex buffers): the five circles, ground row, sun, and point lamp are hardcoded in the shader, and the canvas resolution plus computed background color are uploaded per frame as a vec4 uniform array. The fragment shader mirrors the engine's LightingUniforms::shade term for term (2x2 sub-sample coverage, gamma 1/2.2) and letterboxes the fixed 4:3 logical scene with a uniform scale so the spheres never stretch at any canvas size. Works in every modern browser with WebGL 2 support."
                        }
                    }
                    LightingTab::WebGpu => {
                        p {
                            class: c_game_description()
                            "The WebGPU tab runs the same shader logic expressed in WGSL: a fullscreen triangle generated from @builtin(vertex_index) and a fragment stage that shades the scene per pixel with 2x2 sub-sample coverage. Per-frame data arrives in a single 2-vec4 uniform buffer at @group(0) @binding(0) via WebGpuRenderer's create_render_pipeline / create_uniform_buffer / render_frame_with_bind_group helpers. Requires a WebGPU-capable browser (Chrome 113+, Edge 113+)."
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
    let pause_label: &str = if state.get_running().get() {
        "Pause"
    } else {
        "Resume"
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    "FPS: "
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    "Lights: 1 directional + 1 point"
                }
                span {
                    class: c_game_stats_label()
                    "Scale: "
                    span {
                        class: c_game_stats_count_value()
                        scale_display
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
                            id: LIGHTING_CANVAS_ID
                            class: if { canvas_2d_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                        }
                    }
                }
                if { canvas_2d_fullscreen.get() } {
                    div {
                        class: c_game_fullscreen_toolbar()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Exit"
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
                    label: "Enter Fullscreen"
                    onclick: lighting_on_enter_fullscreen(canvas_2d_fullscreen)
                }
            }
        }
    }
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
/// - `&str` - The `WebGlInitError::code()` from the last init attempt.
///
/// # Returns
///
/// - `&'static str` - The banner text.
fn lighting_webgl_status_text(loaded: bool, active: bool, init_error_code: &str) -> &'static str {
    if !loaded {
        return "Initializing...";
    }
    if active {
        return "WebGL Active";
    }
    if init_error_code.is_empty() {
        "WebGL not supported"
    } else {
        "WebGL init failed"
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
        "Pause"
    } else {
        "Resume"
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    "FPS: "
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    "Lights: 1 directional + 1 point"
                }
                span {
                    class: c_game_stats_label()
                    "Status: "
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
                            id: LIGHTING_WEBGL_CANVAS_ID
                            class: if { web_gl_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: LIGHTING_WEBGL_LOADING_CANVAS_ID
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
                            label: "Exit"
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
                    label: "Enter Fullscreen"
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
        "Pause"
    } else {
        "Resume"
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    "FPS: "
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    "Lights: 1 directional + 1 point"
                }
                span {
                    class: c_game_stats_label()
                    "Status: "
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
                            id: LIGHTING_WEBGPU_CANVAS_ID
                            class: if { web_gpu_fullscreen.get() } {
                                c_raytrace_canvas_fullscreen()
                            } else {
                                c_game_3d_canvas()
                            }
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: LIGHTING_WEBGPU_LOADING_CANVAS_ID
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
                            label: "Exit"
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
                    label: "Enter Fullscreen"
                    onclick: lighting_on_enter_fullscreen(web_gpu_fullscreen)
                }
            }
        }
    }
}
