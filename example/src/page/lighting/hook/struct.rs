use super::*;

/// A single shaded sphere rendered by the standalone Lighting demo.
///
/// Each sphere occupies a `(cx, cy)` centre in logical scene pixels and
/// a `radius` in logical pixels. The albedo and specular colour come
/// straight from `Material` so the same `lighting::compute_lambert` /
/// `lighting::compute_phong` routines used by the 3D engine can drive
/// the per-pixel fill pass.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LightingSphere {
    /// The horizontal centre of the sphere in logical scene pixels.
    pub(crate) cx: f64,
    /// The vertical centre of the sphere in logical scene pixels.
    pub(crate) cy: f64,
    /// The sphere radius in logical scene pixels.
    pub(crate) radius: f64,
    /// The surface material applied to every shaded pixel.
    pub(crate) material: Material,
}

/// Reactive state for the Lighting Canvas 2D software-rendering tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseLighting {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the lighting loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the lighting loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
    /// The current adaptive internal render scale (1.0 = full 320x240).
    #[get(type(copy))]
    pub(crate) render_scale: Signal<f64>,
}

/// Reactive state for the Lighting WebGL backend tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseLightingWebGl {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGL lighting loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
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

/// Reactive state for the Lighting WebGPU backend tab.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseLightingWebGpu {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGPU lighting loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
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

/// Reactive state for the Lighting page fullscreen overlay.
///
/// Each rendering tab (Canvas 2D / WebGL / WebGPU) keeps an independent
/// `fullscreen` signal because the canvas DOM, the render loop, and the
/// GPU device are all tab-specific. The three signals are stacked into a
/// single `UseLightingFullscreen` so the page-level `popstate` guard can
/// be registered once and dispatch against whichever tab is currently in
/// fullscreen.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseLightingFullscreen {
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
