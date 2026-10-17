use super::*;

/// A single bouncing ball in the 2D physics demo game.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Ball {
    /// The current world-space position of the ball center.
    pub(crate) position: Vector2D,
    /// The current linear velocity in pixels per second.
    pub(crate) velocity: Vector2D,
    /// The radius of the ball in pixels.
    pub(crate) radius: f64,
    /// The CSS color string used to fill the ball.
    pub(crate) color: String,
}

/// Reactive state for the 2D bouncing balls game page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame2D {
    /// Whether the 2D game loop is currently running.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) running: Signal<bool>,
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// The current number of balls on the canvas.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) ball_count: Signal<usize>,
    /// The total number of balls spawned since the 2D game started.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_spawned: Signal<usize>,
    /// Whether the canvas has finished loading and is ready for interaction.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
}

/// A persistent wrapper for the ball list that survives component re-renders.
#[derive(Clone, Debug)]
pub(crate) struct BallStore(pub(crate) Rc<RefCell<Vec<Ball>>>);

/// A cached reference to the 2D game canvas element used for efficient coordinate mapping.
#[derive(Clone, Debug)]
pub(crate) struct CanvasCache(pub(crate) Rc<RefCell<Option<HtmlCanvasElement>>>);

/// Reactive state for the 2D WebGPU demo page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame2DWebGpu {
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGPU renderer has finished initializing (success or failure).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGPU renderer is active and rendering.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGPU render loop has been kicked off in this component tree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loop_started: Signal<bool>,
    /// The most recent init error code as a stable string.
    ///
    /// Drives the diagnostic banner shown when `loaded` is true but
    /// `active` is false. The empty string means "no error" (i.e. init is
    /// still in flight or has not started). Storing a stable code rather
    /// than the full `WebGpuInitError` keeps this state `Copy` and avoids
    /// surfacing JS error detail into the reactive UI tree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) init_error_code: Signal<&'static str>,
}

/// Reactive state for the 2D WebGL demo page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame2DWebGl {
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGL renderer has finished initializing (success or failure).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGL renderer is active and rendering.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGL render loop has been kicked off in this component tree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loop_started: Signal<bool>,
    /// The most recent init error code as a stable string.
    ///
    /// Empty string means "no error" (init still in flight or not started).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) init_error_code: Signal<&'static str>,
}

/// Reactive state for the 2D game fullscreen overlay.
///
/// Each rendering tab (Canvas 2D / WebGL / WebGPU) keeps an independent
/// `fullscreen` signal because the canvas DOM, the render loop, and the
/// physics / GPU device are all tab-specific. The three signals are
/// stacked into a single `UseGame2DFullscreen` so the page-level
/// `popstate` guard can be registered once and dispatch against
/// whichever tab is currently in fullscreen.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame2DFullscreen {
    /// Whether the Canvas 2D tab is currently in landscape fullscreen.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas_2d: Signal<bool>,
    /// Whether the WebGL tab is currently in landscape fullscreen.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) web_gl: Signal<bool>,
    /// Whether the WebGPU tab is currently in landscape fullscreen.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) web_gpu: Signal<bool>,
}
