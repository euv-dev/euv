use super::*;

/// A 2D bouncing balls physics game demo powered by the euv_engine.
///
/// Click on the canvas to spawn balls. Each ball is affected by gravity,
/// bounces off walls with restitution, and collides with other balls
/// using impulse-based physics. The game loop runs at a fixed 60 Hz
/// timestep with interpolation via `requestAnimationFrame`.
///
/// A tab bar allows switching between the Canvas 2D, WebGL, and
/// WebGPU backends for comparison.
///
/// # Arguments
///
/// - `VirtualNode<PageGame2DProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The 2D game demo page virtual DOM tree.
#[component]
pub(crate) fn page_game_2d(node: VirtualNode<PageGame2DProps>) -> VirtualNode {
    let PageGame2DProps: PageGame2DProps = node.try_get_props().unwrap_or_default();
    let tab: Signal<Game2DTab> = App::use_signal(Game2DTab::default);
    let fullscreen: UseGame2DFullscreen = use_game_2d_fullscreen_state();
    use_game_2d_fullscreen_popstate(fullscreen);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎮"
                title: GAME_2D_HEADER_TITLE
                subtitle: GAME_2D_HEADER_SUBTITLE
            }
            euv_card {
                title: GAME_2D_CANVAS_CARD_TITLE
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == Game2DTab::Canvas2D } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: game_2d_on_tab_select(tab, Game2DTab::Canvas2D, fullscreen)
                        "2D"
                    }
                    div {
                        class: if { tab.get() == Game2DTab::WebGl } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: game_2d_on_tab_select(tab, Game2DTab::WebGl, fullscreen)
                        "GL"
                    }
                    div {
                        class: if { tab.get() == Game2DTab::WebGpu } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: game_2d_on_tab_select(tab, Game2DTab::WebGpu, fullscreen)
                        "GPU"
                    }
                }
                match { tab } {
                    Game2DTab::Canvas2D => {
                        div {
                            game_2d_canvas_tab(fullscreen)
                        }
                    }
                    Game2DTab::WebGl => {
                        div {
                            game_2d_webgl_tab(use_game_2d_webgl_state(), fullscreen)
                        }
                    }
                    Game2DTab::WebGpu => {
                        div {
                            game_2d_webgpu_tab(use_game_2d_webgpu_state(), fullscreen)
                        }
                    }
                }
            }
            euv_card {
                title: GAME_2D_FEATURES_CARD_TITLE
                match { tab } {
                    Game2DTab::Canvas2D => {
                        p {
                            class: c_game_description()
                            GAME_2D_CANVAS_DESCRIPTION
                        }
                    }
                    Game2DTab::WebGpu => {
                        p {
                            class: c_game_description()
                            GAME_2D_WEBGPU_DESCRIPTION
                        }
                    }
                    Game2DTab::WebGl => {
                        p {
                            class: c_game_description()
                            GAME_2D_WEBGL_DESCRIPTION
                        }
                    }
                }
            }
        }
    }
}

/// Renders the Canvas 2D bouncing balls demo tab content.
///
/// Contains the full Canvas 2D game with stats bar, canvas, and controls.
///
/// # Arguments
///
/// - `UseGame2DFullscreen` - The fullscreen controller hook.
///
/// # Returns
///
/// - `VirtualNode` - The Canvas 2D tab virtual DOM tree.
fn game_2d_canvas_tab(fullscreen: UseGame2DFullscreen) -> VirtualNode {
    let state: UseGame2D = use_game_2d_state();
    let canvas_2d_fullscreen: Signal<bool> = fullscreen.get_canvas_2d();
    let balls_store: Signal<BallStore> = App::use_signal(|| {
        let balls: Rc<RefCell<Vec<Ball>>> = Rc::new(RefCell::new(Vec::new()));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.5, 50.0)));
        balls.borrow_mut().push(create_ball(Vector2D::new(
            GAME_2D_CANVAS_WIDTH * 0.3,
            100.0,
        )));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.7, 80.0)));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.2, 60.0)));
        BallStore(balls)
    });
    let balls: Rc<RefCell<Vec<Ball>>> = balls_store.get().0;
    let canvas_cache: CanvasCache =
        App::use_signal(|| CanvasCache(Rc::new(RefCell::new(None)))).get();
    let loop_started: Signal<bool> = App::use_signal(|| false);
    if !loop_started.get() {
        loop_started.set(true);
        state.get_ball_count().set(balls.borrow().len());
        state.get_total_spawned().set(balls.borrow().len());
        start_game_2d_loop(state, balls.clone(), canvas_cache.clone());
    }
    let on_canvas_click: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_spawn_ball(state, balls.clone(), canvas_cache.clone());
    let on_canvas_touch: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_touch_spawn_ball(state, balls.clone(), canvas_cache.clone());
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_2d_on_toggle_pause(state);
    let on_clear: Option<Rc<dyn Fn(Event)>> = game_2d_on_clear(state, balls.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let ball_count: usize = state.get_ball_count().get();
    let total: usize = state.get_total_spawned().get();
    let pause_label: &str = if state.get_running().get() {
        GAME_2D_PAUSE_LABEL
    } else {
        GAME_2D_RESUME_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    GAME_2D_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_BALLS_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        ball_count
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_TOTAL_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        total
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
                            id: "game-2d-canvas"
                            class: if { canvas_2d_fullscreen.get() } {
                                c_game_2d_canvas_fullscreen()
                            } else {
                                c_game_2d_canvas()
                            }
                            onclick: on_canvas_click
                            ontouchstart: on_canvas_touch
                        }
                    }
                }
                if { canvas_2d_fullscreen.get() } {
                    div {
                        class: c_game_fullscreen_toolbar()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: GAME_2D_EXIT_BUTTON_LABEL
                            onclick: game_2d_on_exit_fullscreen(canvas_2d_fullscreen)
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
                    label: GAME_2D_CLEAR_BUTTON_LABEL
                    onclick: on_clear
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: GAME_2D_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: game_2d_on_enter_fullscreen(fullscreen, canvas_2d_fullscreen)
                }
            }
        }
    }
}

/// Renders the WebGPU bouncing balls demo tab content for the 2D game page.
///
/// Mirrors the Canvas 2D tab: the same balls, physics, and click/touch
/// spawning, rendered through a WGSL pipeline instead of the 2D context.
/// Adds a WebGPU status readout to the stats bar.
///
/// # Arguments
///
/// - `UseGame2DWebGpu` - The WebGPU backend controller hook.
/// - `UseGame2DFullscreen` - The fullscreen controller hook.
///
/// # Returns
///
/// - `VirtualNode` - The WebGPU tab virtual DOM tree.
fn game_2d_webgpu_tab(state: UseGame2DWebGpu, fullscreen: UseGame2DFullscreen) -> VirtualNode {
    let game: UseGame2D = use_game_2d_state();
    let web_gpu_fullscreen: Signal<bool> = fullscreen.get_web_gpu();
    let balls_store: Signal<BallStore> = App::use_signal(|| {
        let balls: Rc<RefCell<Vec<Ball>>> = Rc::new(RefCell::new(Vec::new()));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.5, 50.0)));
        balls.borrow_mut().push(create_ball(Vector2D::new(
            GAME_2D_CANVAS_WIDTH * 0.3,
            100.0,
        )));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.7, 80.0)));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.2, 60.0)));
        BallStore(balls)
    });
    let balls: Rc<RefCell<Vec<Ball>>> = balls_store.get().0;
    let canvas_cache: CanvasCache =
        App::use_signal(|| CanvasCache(Rc::new(RefCell::new(None)))).get();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        game.get_ball_count().set(balls.borrow().len());
        game.get_total_spawned().set(balls.borrow().len());
        start_game_2d_webgpu_loop(state, game, balls.clone(), canvas_cache.clone());
    }
    let on_canvas_click: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_spawn_ball(game, balls.clone(), canvas_cache.clone());
    let on_canvas_touch: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_touch_spawn_ball(game, balls.clone(), canvas_cache.clone());
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_2d_on_toggle_pause(game);
    let on_clear: Option<Rc<dyn Fn(Event)>> = game_2d_on_clear(game, balls.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let ball_count: usize = game.get_ball_count().get();
    let total: usize = game.get_total_spawned().get();
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = webgpu_status_text(loaded, active, init_error_code);
    let pause_label: &str = if game.get_running().get() {
        GAME_2D_PAUSE_LABEL
    } else {
        GAME_2D_RESUME_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    GAME_2D_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_BALLS_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        ball_count
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_TOTAL_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        total
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_STATUS_PREFIX
                    span {
                        class: c_game_stats_count_value()
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
                            id: "game-2d-webgpu-canvas"
                            class: if { web_gpu_fullscreen.get() } {
                                c_game_2d_canvas_fullscreen()
                            } else {
                                c_game_2d_canvas()
                            }
                            onclick: on_canvas_click
                            ontouchstart: on_canvas_touch
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "game-2d-webgpu-loading-canvas"
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
                            label: GAME_2D_EXIT_BUTTON_LABEL
                            onclick: game_2d_on_exit_fullscreen(web_gpu_fullscreen)
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
                    label: GAME_2D_CLEAR_BUTTON_LABEL
                    onclick: on_clear
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: GAME_2D_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: game_2d_on_enter_fullscreen(fullscreen, web_gpu_fullscreen)
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
        return GAME_2D_INITIALIZING_STATUS;
    }
    if active {
        return GAME_2D_WEBGL_ACTIVE_STATUS;
    }
    if init_error_code.is_empty() {
        GAME_2D_WEBGL_NOT_SUPPORTED_STATUS
    } else {
        GAME_2D_WEBGL_INIT_FAILED_STATUS
    }
}

/// Renders the WebGL bouncing balls demo tab content for the 2D game page.
///
/// Mirrors the Canvas 2D tab: the same balls, physics, and click/touch
/// spawning, rendered through a GLSL ES 3.00 program instead of the 2D
/// context. Adds a WebGL status readout to the stats bar.
///
/// # Arguments
///
/// - `UseGame2DWebGl` - The WebGL backend controller hook.
/// - `UseGame2DFullscreen` - The fullscreen controller hook.
///
/// # Returns
///
/// - `VirtualNode` - The WebGL tab virtual DOM tree.
fn game_2d_webgl_tab(state: UseGame2DWebGl, fullscreen: UseGame2DFullscreen) -> VirtualNode {
    let game: UseGame2D = use_game_2d_state();
    let web_gl_fullscreen: Signal<bool> = fullscreen.get_web_gl();
    let balls_store: Signal<BallStore> = App::use_signal(|| {
        let balls: Rc<RefCell<Vec<Ball>>> = Rc::new(RefCell::new(Vec::new()));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.5, 50.0)));
        balls.borrow_mut().push(create_ball(Vector2D::new(
            GAME_2D_CANVAS_WIDTH * 0.3,
            100.0,
        )));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.7, 80.0)));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.2, 60.0)));
        BallStore(balls)
    });
    let balls: Rc<RefCell<Vec<Ball>>> = balls_store.get().0;
    let canvas_cache: CanvasCache =
        App::use_signal(|| CanvasCache(Rc::new(RefCell::new(None)))).get();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        game.get_ball_count().set(balls.borrow().len());
        game.get_total_spawned().set(balls.borrow().len());
        start_game_2d_webgl_loop(state, game, balls.clone(), canvas_cache.clone());
    }
    let on_canvas_click: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_spawn_ball(game, balls.clone(), canvas_cache.clone());
    let on_canvas_touch: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_touch_spawn_ball(game, balls.clone(), canvas_cache.clone());
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_2d_on_toggle_pause(game);
    let on_clear: Option<Rc<dyn Fn(Event)>> = game_2d_on_clear(game, balls.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let ball_count: usize = game.get_ball_count().get();
    let total: usize = game.get_total_spawned().get();
    let loaded: bool = state.get_loaded().get();
    let active: bool = state.get_active().get();
    let init_error_code: &str = state.get_init_error_code().get();
    let status_text: &str = webgl_status_text(loaded, active, init_error_code);
    let pause_label: &str = if game.get_running().get() {
        GAME_2D_PAUSE_LABEL
    } else {
        GAME_2D_RESUME_LABEL
    };
    html! {
        div {
            div {
                class: c_game_stats_bar()
                span {
                    class: c_game_stats_label()
                    GAME_2D_FPS_PREFIX
                    span {
                        class: c_game_stats_fps_value()
                        fps_display
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_BALLS_PREFIX
                    span {
                        class: c_game_stats_count_value()
                        ball_count
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_TOTAL_PREFIX
                    span {
                        class: c_game_stats_total_value()
                        total
                    }
                }
                span {
                    class: c_game_stats_label()
                    GAME_2D_STATUS_PREFIX
                    span {
                        class: c_game_stats_count_value()
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
                            id: "game-2d-webgl-canvas"
                            class: if { web_gl_fullscreen.get() } {
                            c_game_2d_canvas_fullscreen()
                        } else {
                            c_game_2d_canvas()
                        }
                            onclick: on_canvas_click
                            ontouchstart: on_canvas_touch
                        }
                        if { !state.get_loaded().get() } {
                            canvas {
                                id: "game-2d-webgl-loading-canvas"
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
                            label: GAME_2D_EXIT_BUTTON_LABEL
                            onclick: game_2d_on_exit_fullscreen(web_gl_fullscreen)
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
                    label: GAME_2D_CLEAR_BUTTON_LABEL
                    onclick: on_clear
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: GAME_2D_ENTER_FULLSCREEN_BUTTON_LABEL
                    onclick: game_2d_on_enter_fullscreen(fullscreen, web_gl_fullscreen)
                }
            }
        }
    }
}
