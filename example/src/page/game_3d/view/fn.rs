use super::*;

/// A 3D rotating cubes demo powered by the euv-engine 3D math library.
///
/// Displays multiple 3D cubes rendered on a 2D canvas using perspective
/// projection. The camera orbits around the scene and can be dragged
/// with the mouse or touch. Each cube rotates independently using
/// quaternion-based angular velocity integration. Features back-face
/// culling and painter's algorithm depth sorting.
///
/// A tab bar allows switching between the Canvas 2D backend and the
/// WebGPU backend for comparison.
///
/// # Arguments
///
/// - `VirtualNode<PageGame3DProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The 3D game demo page virtual DOM tree.
#[component]
pub(crate) fn page_game_3d(node: VirtualNode<PageGame3DProps>) -> VirtualNode {
    let _page_game_3d_props: PageGame3DProps = node.try_get_props().unwrap_or_default();
    let tab: Signal<Game3DTab> = App::use_signal(Game3DTab::default);
    let fullscreen: UseGame3DFullscreen = use_game_3d_fullscreen_state();
    use_game_3d_fullscreen_popstate(fullscreen);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎲"
                title: GAME_3D_HEADER_TITLE
                subtitle: GAME_3D_HEADER_SUBTITLE
            }
            euv_card {
                title: GAME_3D_CANVAS_CARD_TITLE
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == Game3DTab::Canvas2D } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: game_3d_on_tab_select(tab, Game3DTab::Canvas2D, fullscreen)
                        "2D"
                    }
                    div {
                        class: if { tab.get() == Game3DTab::WebGl } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: game_3d_on_tab_select(tab, Game3DTab::WebGl, fullscreen)
                        "GL"
                    }
                    div {
                        class: if { tab.get() == Game3DTab::WebGpu } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: game_3d_on_tab_select(tab, Game3DTab::WebGpu, fullscreen)
                        "GPU"
                    }
                }
                match { tab } {
                    Game3DTab::Canvas2D => {
                        div {
                            game_3d_canvas_tab(fullscreen)
                        }
                    }
                    Game3DTab::WebGl => {
                        div {
                            game_3d_webgl_tab(use_game_3d_webgl_state(), fullscreen)
                        }
                    }
                    Game3DTab::WebGpu => {
                        div {
                            game_3d_webgpu_tab(use_game_3d_webgpu_state(), fullscreen)
                        }
                    }
                }
            }
            euv_card {
                title: GAME_3D_FEATURES_CARD_TITLE
                match { tab } {
                    Game3DTab::Canvas2D => {
                        p {
                            class: c_game_description()
                            GAME_3D_CANVAS_DESCRIPTION
                        }
                    }
                    Game3DTab::WebGpu => {
                        p {
                            class: c_game_description()
                            GAME_3D_WEBGPU_DESCRIPTION
                        }
                    }
                    Game3DTab::WebGl => {
                        p {
                            class: c_game_description()
                            GAME_3D_WEBGL_DESCRIPTION
                        }
                    }
                }
            }
        }
    }
}

/// Renders the Canvas 2D rotating cubes demo tab content.
///
/// Contains the full Canvas 2D game with stats bar, canvas, and controls.
///
/// # Arguments
///
/// - `UseGame3DFullscreen` - The fullscreen controller hook.
///
/// # Returns
///
/// - `VirtualNode` - The Canvas 2D tab virtual DOM tree.
fn game_3d_canvas_tab(fullscreen: UseGame3DFullscreen) -> VirtualNode {
    let state: UseGame3D = use_game_3d_state();
    let canvas_2d_fullscreen: Signal<bool> = fullscreen.get_canvas_2d();
    let cubes_store: Signal<CubeStore> = App::use_signal(|| {
        let cubes: Rc<RefCell<Vec<Cube3D>>> = Rc::new(RefCell::new(create_initial_cubes()));
        CubeStore(cubes)
    });
    let cubes: Rc<RefCell<Vec<Cube3D>>> = cubes_store.get().0;
    let angles_store: Signal<CameraAngles> = App::use_signal(CameraAngles::default);
    let angles: CameraAngles = angles_store.get();
    let loop_started: Signal<bool> = App::use_signal(|| false);
    let last_pointer: PointerPositionSignal = App::use_signal(|| Rc::new(Cell::new(None)));
    if !loop_started.get() {
        loop_started.set(true);
        state.get_cube_count().set(cubes.borrow().len());
        start_game_3d_loop(state, cubes.clone(), angles.clone());
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_pause(state);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_auto_rotate(state);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = game_3d_on_reset_camera(angles.clone());
    let pointer_cell: Rc<Cell<Option<(f64, f64)>>> = last_pointer.get();
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_down(pointer_cell.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_pointer_move(angles.clone(), pointer_cell.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_up(pointer_cell.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_start(pointer_cell.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_touch_move(angles.clone(), pointer_cell.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_end(pointer_cell.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let cube_count: usize = state.get_cube_count().get();
    let pause_label: &str = if state.get_running().get() {
        GAME_3D_PAUSE_LABEL
    } else {
        GAME_3D_RESUME_LABEL
    };
    let auto_rotate_label: &str = if state.get_auto_rotate().get() {
        GAME_3D_AUTO_ROTATE_ON_LABEL
    } else {
        GAME_3D_AUTO_ROTATE_OFF_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    GAME_3D_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_3D_CUBES_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        cube_count
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
                            id: "game-3d-canvas"
                            class: if { canvas_2d_fullscreen.get() } {
                                c_game_3d_canvas_fullscreen()
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
                    }
                }
                if { canvas_2d_fullscreen.get() } {
                    div {
                        class: c_game_fullscreen_toolbar()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: GAME_3D_EXIT_BUTTON_LABEL
                            onclick: game_3d_on_exit_fullscreen(canvas_2d_fullscreen)
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
                    label: GAME_3D_RESET_CAMERA_BUTTON_LABEL
                    onclick: on_reset_camera
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: GAME_3D_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: game_3d_on_enter_fullscreen(fullscreen, canvas_2d_fullscreen)
                }
            }
        }
    }
}

/// Renders the WebGPU rotating cubes demo tab content for the 3D game page.
///
/// Mirrors the Canvas 2D tab: the same cubes, quaternion integration,
/// orbit camera, and pointer/touch drag, rendered through a WGSL pipeline
/// instead of the 2D context. Adds a WebGPU status readout to the stats
/// bar.
///
/// # Arguments
///
/// - `UseGame3DWebGpu` - The WebGPU backend controller hook.
/// - `UseGame3DFullscreen` - The fullscreen controller hook.
///
/// # Returns
///
/// - `VirtualNode` - The WebGPU tab virtual DOM tree.
fn game_3d_webgpu_tab(state: UseGame3DWebGpu, fullscreen: UseGame3DFullscreen) -> VirtualNode {
    let game: UseGame3D = use_game_3d_state();
    let web_gpu_fullscreen: Signal<bool> = fullscreen.get_web_gpu();
    let cubes_store: Signal<CubeStore> = App::use_signal(|| {
        let cubes: Rc<RefCell<Vec<Cube3D>>> = Rc::new(RefCell::new(create_initial_cubes()));
        CubeStore(cubes)
    });
    let cubes: Rc<RefCell<Vec<Cube3D>>> = cubes_store.get().0;
    let angles_store: Signal<CameraAngles> = App::use_signal(CameraAngles::default);
    let angles: CameraAngles = angles_store.get();
    let last_pointer: PointerPositionSignal = App::use_signal(|| Rc::new(Cell::new(None)));
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        game.get_cube_count().set(cubes.borrow().len());
        start_game_3d_webgpu_loop(state, game, cubes.clone(), angles.clone());
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_pause(game);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_auto_rotate(game);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = game_3d_on_reset_camera(angles.clone());
    let pointer_cell: Rc<Cell<Option<(f64, f64)>>> = last_pointer.get();
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_down(pointer_cell.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_pointer_move(angles.clone(), pointer_cell.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_up(pointer_cell.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_start(pointer_cell.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_touch_move(angles.clone(), pointer_cell.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_end(pointer_cell.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let cube_count: usize = game.get_cube_count().get();
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = webgpu_status_text(loaded, active, init_error_code);
    let pause_label: &str = if game.get_running().get() {
        GAME_3D_PAUSE_LABEL
    } else {
        GAME_3D_RESUME_LABEL
    };
    let auto_rotate_label: &str = if game.get_auto_rotate().get() {
        GAME_3D_AUTO_ROTATE_ON_LABEL
    } else {
        GAME_3D_AUTO_ROTATE_OFF_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    GAME_3D_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_3D_CUBES_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        cube_count
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_3D_STATUS_PREFIX
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
                            id: "game-3d-webgpu-canvas"
                            class: if { web_gpu_fullscreen.get() } {
                                c_game_3d_canvas_fullscreen()
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
                                id: "game-3d-webgpu-loading-canvas"
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
                            label: GAME_3D_EXIT_BUTTON_LABEL
                            onclick: game_3d_on_exit_fullscreen(web_gpu_fullscreen)
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
                    label: GAME_3D_RESET_CAMERA_BUTTON_LABEL
                    onclick: on_reset_camera
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: GAME_3D_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: game_3d_on_enter_fullscreen(fullscreen, web_gpu_fullscreen)
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
/// - `&str` - The `WebGl2InitError::code()` from the last init attempt.
///
/// # Returns
///
/// - `&'static str` - The banner text.
fn webgl_status_text(loaded: bool, active: bool, init_error_code: &str) -> &'static str {
    if !loaded {
        return GAME_3D_INITIALIZING_STATUS;
    }
    if active {
        return GAME_3D_WEBGL_ACTIVE_STATUS;
    }
    if init_error_code.is_empty() {
        GAME_3D_WEBGL_NOT_SUPPORTED_STATUS
    } else {
        GAME_3D_WEBGL_INIT_FAILED_STATUS
    }
}

/// Renders the WebGL rotating cubes demo tab content for the 3D game page.
///
/// Mirrors the Canvas 2D tab: the same cubes, quaternion integration,
/// orbit camera, and pointer/touch drag, rendered through a GLSL ES 3.00
/// program instead of the 2D context. Adds a WebGL status readout to the
/// stats bar.
///
/// # Arguments
///
/// - `UseGame3DWebGl` - The WebGL backend controller hook.
/// - `UseGame3DFullscreen` - The fullscreen controller hook.
///
/// # Returns
///
/// - `VirtualNode` - The WebGL tab virtual DOM tree.
fn game_3d_webgl_tab(state: UseGame3DWebGl, fullscreen: UseGame3DFullscreen) -> VirtualNode {
    let game: UseGame3D = use_game_3d_state();
    let web_gl_fullscreen: Signal<bool> = fullscreen.get_web_gl();
    let cubes_store: Signal<CubeStore> = App::use_signal(|| {
        let cubes: Rc<RefCell<Vec<Cube3D>>> = Rc::new(RefCell::new(create_initial_cubes()));
        CubeStore(cubes)
    });
    let cubes: Rc<RefCell<Vec<Cube3D>>> = cubes_store.get().0;
    let angles_store: Signal<CameraAngles> = App::use_signal(CameraAngles::default);
    let angles: CameraAngles = angles_store.get();
    let last_pointer: PointerPositionSignal = App::use_signal(|| Rc::new(Cell::new(None)));
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        game.get_cube_count().set(cubes.borrow().len());
        start_game_3d_webgl_loop(state, game, cubes.clone(), angles.clone());
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_pause(game);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_auto_rotate(game);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = game_3d_on_reset_camera(angles.clone());
    let pointer_cell: Rc<Cell<Option<(f64, f64)>>> = last_pointer.get();
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_down(pointer_cell.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_pointer_move(angles.clone(), pointer_cell.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_up(pointer_cell.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_start(pointer_cell.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_touch_move(angles.clone(), pointer_cell.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_end(pointer_cell.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let cube_count: usize = game.get_cube_count().get();
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = webgl_status_text(loaded, active, init_error_code);
    let pause_label: &str = if game.get_running().get() {
        GAME_3D_PAUSE_LABEL
    } else {
        GAME_3D_RESUME_LABEL
    };
    let auto_rotate_label: &str = if game.get_auto_rotate().get() {
        GAME_3D_AUTO_ROTATE_ON_LABEL
    } else {
        GAME_3D_AUTO_ROTATE_OFF_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    GAME_3D_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_3D_CUBES_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        cube_count
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_3D_STATUS_PREFIX
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
                            id: "game-3d-webgl-canvas"
                            class: if { web_gl_fullscreen.get() } {
                            c_game_3d_canvas_fullscreen()
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
                                id: "game-3d-webgl-loading-canvas"
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
                            label: GAME_3D_EXIT_BUTTON_LABEL
                            onclick: game_3d_on_exit_fullscreen(web_gl_fullscreen)
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
                    label: GAME_3D_RESET_CAMERA_BUTTON_LABEL
                    onclick: on_reset_camera
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: GAME_3D_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: game_3d_on_enter_fullscreen(fullscreen, web_gl_fullscreen)
                }
            }
        }
    }
}
