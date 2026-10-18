use super::*;

/// A 3D cube instance with position, rotation, scale, and color.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cube3D {
    /// The world-space position of the cube center.
    pub(crate) position: Vector3D,
    /// The rotation as a quaternion.
    pub(crate) rotation: Quaternion,
    /// The angular velocity for continuous rotation.
    pub(crate) angular_velocity: Vector3D,
    /// The uniform scale factor.
    pub(crate) scale: f64,
    /// The CSS color string used to fill the cube faces.
    pub(crate) face_color: String,
    /// The CSS color string used to stroke the cube edges.
    pub(crate) edge_color: String,
}

/// Reactive state for the 3D game page.
///
/// Only signals that are read inside the `html!` render function belong here.
/// High-frequency values like camera angles use `CameraAngles` instead to
/// avoid triggering excessive re-renders through the `Data` reactivity macro.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame3D {
    /// Whether the game loop is currently running.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) running: Signal<bool>,
    /// The current frames-per-second measurement.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fps: Signal<f64>,
    /// The current number of cubes in the scene.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cube_count: Signal<usize>,
    /// Whether auto-rotation is enabled.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) auto_rotate: Signal<bool>,
    /// Whether the canvas has finished loading and is ready for interaction.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) loaded: Signal<bool>,
}

/// Non-reactive camera orbit angles persisted via a `Signal` wrapper.
///
/// The `Signal` is read once to obtain the `Rc` handles; all subsequent
/// reads and writes go through `Cell` which bypasses the reactivity system
/// entirely, preventing re-render storms during rapid mouse drag.
#[derive(Clone, Debug)]
pub(crate) struct CameraAngles {
    /// The orbit yaw angle in radians.
    pub(crate) yaw: Rc<Cell<f64>>,
    /// The orbit pitch angle in radians.
    pub(crate) pitch: Rc<Cell<f64>>,
}

/// A persistent wrapper for the cube list that survives component re-renders.
#[derive(Clone, Debug)]
pub(crate) struct CubeStore(pub(crate) Rc<RefCell<Vec<Cube3D>>>);

/// Reactive state for the 3D WebGPU demo page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame3DWebGpu {
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

/// Reactive state for the 3D WebGL demo page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame3DWebGl {
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

/// Reactive state for the 3D game fullscreen overlay.
///
/// Each rendering tab (Canvas 2D / WebGL / WebGPU) keeps an independent
/// `fullscreen` signal because the canvas DOM, the render loop, and the
/// 3D math / WebGPU device are all tab-specific. The three signals are
/// stacked into a single `UseGame3DFullscreen` so the page-level
/// `popstate` guard can be registered once and dispatch against whichever
/// tab is currently in fullscreen.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame3DFullscreen {
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
