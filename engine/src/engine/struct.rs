use super::*;

/// A zero-sized namespace struct providing static engine entry points.
///
/// This struct follows the same pattern as `euv::App`, serving as the
/// single root namespace for all top-level engine operations: creating
/// engine handles, initializing rendering backends, and starting the
/// fixed-timestep game loop.
///
/// All engine usage flows through `Engine::xxx` static calls, mirroring
/// how `euv::App::use_signal` / `euv::App::mount` are the public entry
/// points of the core framework.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Engine;

/// A handle to a running or initialized engine instance.
///
/// Holds the engine configuration, an optional Canvas 2D renderer, an
/// optional WebGPU renderer, and the optional scheduler handle once the
/// game loop has started. Use `Engine::new_handle` to construct one, then
/// call `init_canvas` or `init_webgpu` (matching the backend chosen in
/// `RenderConfig`) and finally `start` to begin the game loop.
#[derive(Clone, Data, New)]
pub struct EngineHandle {
    /// The engine configuration containing render and scheduler settings.
    pub(crate) config: EngineConfig,
    /// The initialized Canvas 2D renderer, or `None` if the WebGPU backend is used or initialization has not yet happened.
    pub(crate) canvas_renderer: Option<CanvasRenderer>,
    /// The initialized WebGPU renderer, or `None` if the Canvas 2D backend is used or initialization has not yet happened.
    pub(crate) webgpu_renderer: Option<WebGpuRenderer>,
    /// The initialized WebGL 2 renderer, or `None` if another backend is used or initialization has not yet happened.
    pub(crate) webgl_renderer: Option<WebGlRenderer>,
    /// The shared input state once `register_input` has attached the DOM
    /// event listeners, or `None` before registration (or if the canvas
    /// selector did not resolve at registration time).
    #[new(skip)]
    pub(crate) input_cell: Option<InputStateCell>,
    /// The shared task registry holding every [`Updatable`] object the
    /// scheduler advances on each fixed step. Always present so callers can
    /// register tasks before `start` is called.
    #[new(skip)]
    pub(crate) tasks: TaskRegistryRc,
    /// The running scheduler handle, or `None` before `start` is called.
    pub(crate) scheduler_handle: Option<SchedulerHandle>,
}
