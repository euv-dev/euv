use super::*;

/// Implements the top-level static engine entry points on the `Engine` namespace.
///
/// Mirrors the role of `euv::App` - every public engine operation begins
/// with an `Engine::xxx` call, and `Engine` itself holds no state.
impl Engine {
    /// Creates a new engine handle bound to the given configuration.
    ///
    /// The handle is uninitialized; call `init_canvas` / `init_webgpu` and
    /// `start` on the returned handle to begin the game loop, or use
    /// `Engine::run` for the one-line equivalent.
    ///
    /// # Arguments
    ///
    /// - `EngineConfig` - The engine configuration.
    ///
    /// # Returns
    ///
    /// - `EngineHandle` - The new uninitialized engine handle.
    pub fn new_handle(config: EngineConfig) -> EngineHandle {
        EngineHandle::new(config, None, None, None, None)
    }

    /// Runs the engine through its complete lifecycle in a single async call.
    ///
    /// Equivalent to calling `Engine::new_handle(config)` followed by
    /// `init_canvas` / `init_webgpu` (matching the chosen backend) and
    /// `start(handler)`. The returned handle can still be used to stop
    /// the loop later via `handle.stop()`.
    ///
    /// # Arguments
    ///
    /// - `EngineConfig` - The engine configuration.
    /// - `TickHandlerRc` - The tick handler receiving update and render callbacks.
    ///
    /// # Returns
    ///
    /// - `EngineHandle` - A handle to the running engine. If renderer
    ///   initialization failed the handle's renderer field will be `None`
    ///   and `is_running` will be `false`. Initialization errors are not
    ///   logged inside the engine; callers should call `init_webgpu`
    ///   directly if they need access to the typed `WebGpuInitError`.
    pub async fn run(config: EngineConfig, handler: TickHandlerRc) -> EngineHandle {
        let mut handle: EngineHandle = Engine::new_handle(config);
        match handle.get_config().get_render().get_backend() {
            RenderBackendType::Canvas2D => {
                handle.init_canvas();
            }
            RenderBackendType::WebGpu => {
                let _: Result<WebGpuRenderer, WebGpuInitError> = handle.init_webgpu().await;
            }
            RenderBackendType::WebGl => {
                let _: Result<WebGlRenderer, WebGlInitError> = handle.init_webgl();
            }
        }
        handle.start(handler);
        handle
    }

    /// Returns the default engine configuration.
    ///
    /// Uses `RenderConfig::default()` (Canvas 2D backend) and the default
    /// scheduler configuration (60 Hz fixed timestep).
    ///
    /// # Returns
    ///
    /// - `EngineConfig` - The default engine configuration.
    pub fn default_config() -> EngineConfig {
        EngineConfig::default()
    }

    /// Creates a Canvas 2D renderer directly from a render configuration.
    ///
    /// Use this when you want a `CanvasRenderer` without going through
    /// the full `EngineHandle` lifecycle (for example, in a custom render
    /// loop that does not use the fixed-timestep scheduler).
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasRenderer>` - The renderer, or `None` if the canvas element was not found.
    pub fn canvas_renderer(config: &RenderConfig) -> Option<CanvasRenderer> {
        CanvasRenderer::from_selector(
            config.get_canvas_selector(),
            config.get_width(),
            config.get_height(),
        )
    }

    /// Creates a WebGPU renderer directly from a render configuration.
    ///
    /// Async because GPU adapter and device acquisition returns JavaScript
    /// Promises that must be awaited. Mirrors `Engine::canvas_renderer` for
    /// callers that want the renderer without the full engine handle.
    ///
    /// Failures are surfaced as `WebGpuInitError` so the caller can decide
    /// how to react (typically by logging via `Console::error` or by
    /// falling back to the Canvas 2D backend).
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGpuRenderer, WebGpuInitError>` - The initialized renderer,
    ///   or a typed error describing the specific failure.
    pub async fn webgpu_renderer(config: &RenderConfig) -> Result<WebGpuRenderer, WebGpuInitError> {
        WebGpuRenderer::init(config).await
    }

    /// Creates a WebGL 2 renderer directly from a render configuration.
    ///
    /// Unlike [`Engine::webgpu_renderer`] this is synchronous because WebGL
    /// context acquisition is a plain DOM call with no Promises involved.
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGlRenderer, WebGlInitError>` - The initialized renderer,
    ///   or a typed error describing the specific failure.
    pub fn webgl_renderer(config: &RenderConfig) -> Result<WebGlRenderer, WebGlInitError> {
        WebGlRenderer::init(config)
    }
}

/// Implements lifecycle management for `EngineHandle`.
///
/// All getter methods (`get_config`, `get_canvas_renderer`, `get_webgpu_renderer`)
/// are generated by the `Data` derive on the struct definition; this impl
/// only contributes lifecycle and accessor behavior that is not expressible
/// as a plain getter.
impl EngineHandle {
    /// Initializes the Canvas 2D rendering backend.
    ///
    /// On success, populates `canvas_renderer` and clears `webgpu_renderer`.
    /// On failure, both renderer fields remain `None`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the renderer was created successfully.
    pub fn init_canvas(&mut self) -> bool {
        let render_config: &RenderConfig = &self.get_config().get_render();
        let renderer: Option<CanvasRenderer> = CanvasRenderer::from_selector(
            render_config.get_canvas_selector(),
            render_config.get_width(),
            render_config.get_height(),
        );
        match renderer {
            Some(r) => {
                self.set_canvas_renderer(Some(r));
                self.set_webgpu_renderer(None);
                self.set_webgl_renderer(None);
                true
            }
            None => {
                self.set_canvas_renderer(None);
                self.set_webgpu_renderer(None);
                self.set_webgl_renderer(None);
                false
            }
        }
    }

    /// Initializes the WebGPU rendering backend.
    ///
    /// On success, populates `webgpu_renderer` and clears `canvas_renderer`.
    /// On failure, both renderer fields remain `None` and the typed error is
    /// returned so the caller can decide how to surface it (typically via
    /// `Console::error`).
    ///
    /// # Returns
    ///
    /// - `Result<WebGpuRenderer, WebGpuInitError>` - The initialized renderer,
    ///   or a typed error describing the specific failure.
    pub async fn init_webgpu(&mut self) -> Result<WebGpuRenderer, WebGpuInitError> {
        let render_config: &RenderConfig = &self.get_config().get_render();
        let renderer: WebGpuRenderer = WebGpuRenderer::init(render_config).await?;
        self.set_webgpu_renderer(Some(renderer.clone()));
        self.set_canvas_renderer(None);
        self.set_webgl_renderer(None);
        Ok(renderer)
    }

    /// Initializes the WebGL 2 rendering backend.
    ///
    /// On success, populates `webgl_renderer` and clears the other renderer
    /// fields. On failure, all renderer fields remain `None` and the typed
    /// error is returned so the caller can decide how to surface it.
    ///
    /// # Returns
    ///
    /// - `Result<WebGlRenderer, WebGlInitError>` - The initialized renderer,
    ///   or a typed error describing the specific failure.
    pub fn init_webgl(&mut self) -> Result<WebGlRenderer, WebGlInitError> {
        let render_config: &RenderConfig = &self.get_config().get_render();
        let renderer: WebGlRenderer = WebGlRenderer::init(render_config)?;
        self.set_webgl_renderer(Some(renderer.clone()));
        self.set_canvas_renderer(None);
        self.set_webgpu_renderer(None);
        Ok(renderer)
    }

    /// Attaches DOM input listeners and stores the shared input state.
    ///
    /// Resolves the canvas element from the render configuration's
    /// `canvas_selector` and routes DOM events into a shared [`InputState`]:
    /// keyboard events bind to `window`, mouse and touch events bind to the
    /// canvas. This works for both the Canvas 2D and WebGPU backends because
    /// either way the same `<canvas>` element receives the pointer events.
    ///
    /// Returns `None` (and stores nothing) if the canvas selector does not
    /// resolve at call time - for example when called before the canvas
    /// element is mounted into the DOM.
    ///
    /// The registered listeners stay alive for the lifetime of the document
    /// (mount-only convention; there is no matching `unregister_input`).
    ///
    /// # Returns
    ///
    /// - `Option<InputStateCell>` - The shared input state cell, or `None`
    ///   if the canvas element was not found.
    pub fn register_input(&mut self) -> Option<InputStateCell> {
        if let Some(existing) = self.try_get_input_cell() {
            return Some(existing.clone());
        }
        let window_value: Window = window()?;
        let document_value: Document = window_value.document()?;
        let render_config: &RenderConfig = &self.get_config().get_render();
        let canvas_selector: String = render_config.get_canvas_selector();
        let element: Element = document_value
            .query_selector(canvas_selector.as_ref())
            .ok()
            .flatten()?;
        let target: EventTarget = element.into();
        let cell: InputStateCell = Input::attach(
            Rc::new(EngineCell::new(InputState::default())),
            &window_value,
            &target,
        );
        self.set_input_cell(Some(cell.clone()));
        Some(cell)
    }

    /// Starts the game loop with the given tick handler.
    ///
    /// The scheduler configuration from `EngineConfig` controls the fixed
    /// timestep and maximum frame time. The scheduler handle is stored
    /// internally and can be stopped via `stop`.
    ///
    /// # Arguments
    ///
    /// - `TickHandlerRc` - The tick handler receiving update and render callbacks.
    pub fn start(&mut self, handler: TickHandlerRc) {
        let scheduler_config: SchedulerConfig = self.get_config().get_scheduler();
        self.set_scheduler_handle(Some(SchedulerHandle::start(scheduler_config, handler)));
    }

    /// Stops the game loop and cancels any pending animation frame request.
    pub fn stop(&self) {
        if let Some(handle) = self.try_get_scheduler_handle().as_ref() {
            handle.stop();
        }
    }

    /// Returns whether the game loop is currently running.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the scheduler is running.
    pub fn is_running(&self) -> bool {
        self.try_get_scheduler_handle()
            .as_ref()
            .is_some_and(|handle: &SchedulerHandle| handle.is_running())
    }
}
