use super::*;

/// Reactive state for the RayTrace Canvas 2D software-rendering tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTrace {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the raytrace loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the raytrace loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    ///
    /// Dragging on the canvas disables auto-rotate for the rest of the
    /// session; the toolbar button re-enables it.
    #[get(type(copy))]
    pub(crate) auto_rotate: Signal<bool>,
    /// The current adaptive internal render scale (1.0 = full 320x240,
    /// 2.0 = 640x480 super-sampled).
    #[get(type(copy))]
    pub(crate) render_scale: Signal<f64>,
    /// Whether the RayTrace Canvas 2D tab has finished its first warmup
    /// frame and is ready to render.
    ///
    /// Mirrors the `loaded` signal exposed by the WebGL / WebGPU tabs
    /// so all three RayTrace tabs share the same loading-overlay UX:
    /// the view renders a `c_game_loading_overlay` canvas on top of the
    /// raytrace canvas while `loaded` is `false`, and the loop flips it
    /// to `true` after the SSAA wrapper is acquired and the first frame
    /// has been traced. Without this signal the Canvas 2D tab would
    /// show a blank / half-rendered canvas during the 200-400 ms
    /// warmup that the SSAA acquire + first per-pixel ray pass cost
    /// (scenes are precomputed but the offscreen canvas allocate +
    /// 2D-context acquire + initial put_image_data are not free).
    #[get(type(copy))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the Canvas 2D renderer is active and rendering. The
    /// WebGL / WebGPU tabs already carry an `active` signal; mirroring
    /// it here keeps the status banner code path identical across
    /// tabs and gives the future engine-rewrite a uniform status
    /// surface to read from.
    #[get(type(copy))]
    pub(crate) active: Signal<bool>,
}

/// Reactive state for the RayTrace WebGL backend tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceWebGl {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGL raytrace loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    #[get(type(copy))]
    pub(crate) auto_rotate: Signal<bool>,
    /// Whether the WebGL renderer has finished initializing (success or failure).
    #[get(type(copy))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGL renderer is active and rendering.
    #[get(type(copy))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGL render loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
    /// The most recent init error code as a stable string.
    ///
    /// Drives the diagnostic banner shown when `loaded` is true but
    /// `active` is false. The empty string means "no error" (i.e. init is
    /// still in flight or has not started). Storing a stable code rather
    /// than the full `WebGlInitError` keeps this state `Copy` and avoids
    /// surfacing JS error detail into the reactive UI tree.
    #[get(type(copy))]
    pub(crate) init_error_code: Signal<&'static str>,
}

/// Reactive state for the RayTrace WebGPU backend tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceWebGpu {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGPU raytrace loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    #[get(type(copy))]
    pub(crate) auto_rotate: Signal<bool>,
    /// Whether the WebGPU renderer has finished initializing (success or failure).
    #[get(type(copy))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGPU renderer is active and rendering.
    #[get(type(copy))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGPU render loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
    /// The most recent init error code as a stable string.
    ///
    /// Drives the diagnostic banner shown when `loaded` is true but
    /// `active` is false. The empty string means "no error" (i.e. init is
    /// still in flight or has not started). Storing a stable code rather
    /// than the full `WebGpuInitError` keeps this state `Copy` and avoids
    /// surfacing JS error detail into the reactive UI tree.
    #[get(type(copy))]
    pub(crate) init_error_code: Signal<&'static str>,
}

/// Reactive state for the RayTrace page fullscreen overlay.
///
/// Each rendering tab (Canvas 2D / WebGL / WebGPU) keeps an independent
/// `fullscreen` signal because the canvas DOM, the render loop, and the
/// GPU device are all tab-specific. The three signals are stacked into a
/// single `UseRayTraceFullscreen` so the page-level `popstate` guard can
/// be registered once and dispatch against whichever tab is currently in
/// fullscreen.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceFullscreen {
    /// Whether the Canvas 2D tab is currently in landscape fullscreen.
    #[get(type(copy))]
    pub(crate) canvas_2d: Signal<bool>,
    /// Whether the WebGL tab is currently in landscape fullscreen.
    #[get(type(copy))]
    pub(crate) web_gl: Signal<bool>,
    /// Whether the WebGPU tab is currently in landscape fullscreen.
    #[get(type(copy))]
    pub(crate) web_gpu: Signal<bool>,
}

/// Non-reactive camera orbit angles persisted via a `Signal` wrapper.
///
/// The `Signal` is read once to obtain the `Rc` handles; all subsequent
/// reads and writes go through `Cell` which bypasses the reactivity
/// system entirely, preventing re-render storms during rapid mouse
/// drag. `PartialEq` is derived so the type satisfies `Signal<T>`'s
/// `T: PartialEq` bound (the bound only matters for re-render skipping,
/// never for value equality, since the cell values change every frame).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RayTraceCameraAngles {
    /// The orbit yaw angle in radians.
    pub(crate) yaw: Rc<Cell<f64>>,
    /// The orbit pitch angle in radians.
    pub(crate) pitch: Rc<Cell<f64>>,
}
