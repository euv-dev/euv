use super::*;

/// Reactive state for the RayTrace Canvas 2D software-rendering tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTrace {
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// Whether the raytrace loop is currently running.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) running: Signal<bool>,
    /// Whether the raytrace loop has been kicked off in this component tree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loop_started: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    ///
    /// Dragging on the canvas disables auto-rotate for the rest of the
    /// session; the toolbar button re-enables it.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) auto_rotate: Signal<bool>,
    /// The current adaptive internal render scale (1.0 = full 320x240,
    /// 2.0 = 640x480 super-sampled).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_scale: Signal<f64>,
    /// Whether the RayTrace Canvas 2D tab has finished its first render and
    /// is ready to present.
    ///
    /// Mirrors the `loaded` signal exposed by the WebGL / WebGPU tabs so all
    /// three RayTrace tabs share the same loading-overlay UX: the view
    /// renders a `c_game_loading_overlay` canvas on top of the raytrace
    /// canvas while `loaded` is `false`, and the loop flips it to `true`
    /// after a frame has actually been rendered. Without this signal the
    /// Canvas 2D tab would show a blank canvas during backend acquisition
    /// and the first per-pixel ray pass.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the Canvas 2D backend rendered a frame successfully and
    /// most recently still is.
    ///
    /// The view reads this for its status line, and the loop clears it on
    /// every rejected `render` call. That direction matters: a flag that
    /// only ever flips to `true` would keep reporting a healthy renderer
    /// after the backend lost its device or started rejecting frames.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active: Signal<bool>,
    /// The most recent engine rejection detail as a human-readable string.
    ///
    /// Empty when the backend is healthy. Shown verbatim in the tab's
    /// status line so a rejection is never reduced to a generic "failed".
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) error_detail: Signal<String>,
    /// The most recent rejected call site as a stable code.
    ///
    /// Empty when the backend is healthy. Distinguishes "the browser cannot
    /// host this backend" from "the backend died mid-run", which the raw
    /// detail string alone does not convey. Stored as a code rather than an
    /// error enum so the state stays `Copy`.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) init_error_code: Signal<&'static str>,
}

/// Reactive state for the RayTrace WebGL backend tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceWebGl {
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGL raytrace loop is currently running.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) running: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) auto_rotate: Signal<bool>,
    /// Whether the WebGL backend has finished initializing (success or
    /// failure).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGL backend rendered a frame successfully and most
    /// recently still is.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGL render loop has been kicked off in this component
    /// tree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loop_started: Signal<bool>,
    /// The most recent init error code as a stable string.
    ///
    /// Drives the diagnostic banner shown when `loaded` is true but
    /// `active` is false. The empty string means "no error".
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) init_error_code: Signal<&'static str>,
    /// The most recent engine rejection detail as a human-readable string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) error_detail: Signal<String>,
}

/// Reactive state for the RayTrace WebGPU backend tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceWebGpu {
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGPU raytrace loop is currently running.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) running: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) auto_rotate: Signal<bool>,
    /// Whether the WebGPU backend has finished initializing (success or
    /// failure).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGPU backend rendered a frame successfully and most
    /// recently still is.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGPU render loop has been kicked off in this component
    /// tree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loop_started: Signal<bool>,
    /// The most recent init error code as a stable string.
    ///
    /// Drives the diagnostic banner shown when `loaded` is true but
    /// `active` is false. The empty string means "no error".
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) init_error_code: Signal<&'static str>,
    /// The most recent engine rejection detail as a human-readable string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) error_detail: Signal<String>,
}

/// Reactive state for the RayTrace page fullscreen overlay.
///
/// Each rendering tab (Canvas 2D / WebGL / WebGPU) keeps an independent
/// `fullscreen` signal because the canvas DOM, the render loop, and the
/// GPU device are all tab-specific. The three signals are stacked into a
/// single `UseRayTraceFullscreen` so the page-level `popstate` guard can be
/// registered once and dispatch against whichever tab is currently in
/// fullscreen.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceFullscreen {
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
