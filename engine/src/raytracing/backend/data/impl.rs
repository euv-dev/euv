use super::*;

/// Implements construction, scene updates, rendering, resizing, and
/// teardown for [`RayTraceRenderer`].
///
/// Only [`RayTraceRenderer::init`] is asynchronous, because it is the
/// only step that awaits a browser promise. Every other call is
/// synchronous, so a render loop drives the renderer without awaiting.
///
/// Every step that can fail validates everything it can before it
/// mutates anything, and commits its own state change only after the
/// step that can fail has succeeded. A rejected call therefore leaves
/// the renderer exactly as it was, on the canvas as well as in memory.
impl RayTraceRenderer {
    /// Creates a renderer bound to one explicitly requested backend.
    ///
    /// The requested surface dimensions and the initial scene are both
    /// validated before any GPU resource is created, so an illegal
    /// input is rejected before a device, a pipeline, or two storage
    /// buffers are allocated and then thrown away.
    ///
    /// The backend is never substituted. If the requested backend is
    /// unavailable, if a shader fails to compile, or if the WebGPU
    /// pipeline fails validation, this returns the driver's own
    /// diagnostic — awaiting the WebGPU compilation info and error
    /// scopes, because WebGPU reports both asynchronously and a
    /// `catchJs` wrapper around the synchronous calls would report
    /// success for an invalid pipeline.
    ///
    /// # Arguments
    ///
    /// - `HtmlCanvasElement` - The canvas to draw into.
    /// - `RayTraceBackend` - The backend to drive, with no fallback.
    /// - `&RayTraceScene` - The initial scene, packed and uploaded here.
    ///
    /// # Returns
    ///
    /// - `Result<Self, JsValue>` - The ready renderer, or the reason it
    ///   could not be created.
    pub async fn init(
        canvas: HtmlCanvasElement,
        backend: RayTraceBackend,
        scene: &RayTraceScene,
    ) -> Result<Self, JsValue> {
        let width: u32 = canvas.width();
        let height: u32 = canvas.height();
        validate_surface(width, height, None)?;
        if let Err(reason) = scene.encode_gpu() {
            return Err(backend_error(ERROR_SCENE_PACK, reason));
        }
        let mut renderer: RayTraceRenderer = RayTraceRenderer {
            canvas,
            backend,
            adapter: None,
            scene: RayTraceScene::new(Vec::new()),
            packed_frame: Vec::new(),
            pixels: Vec::new(),
            width,
            height,
            disposed: false,
        };
        // The adapter owns the GPU resources, so a failure after this
        // point has to run its release. `adapter` is filled in before
        // the fallible steps below precisely so that the early return
        // releases it instead of leaking it.
        renderer.set_adapter(Some(
            RayTraceBackendAdapter::attach(renderer.get_canvas(), backend, width, height).await?,
        ));
        if let Err(reason) = renderer.resize(width, height) {
            renderer.release_backend();
            return Err(reason);
        }
        if let Err(reason) = renderer.update_scene(scene) {
            renderer.release_backend();
            return Err(reason);
        }
        Ok(renderer)
    }

    /// Replaces the scene, re-packing and re-uploading it exactly once.
    ///
    /// The update is transactional. The new scene is packed into a
    /// buffer staged outside the renderer, then uploaded; only once the
    /// upload has succeeded is the renderer's own scene mirror swapped
    /// in. A failed upload therefore leaves the previous scene both in
    /// the renderer and, because the adapter only rebinds a texture
    /// after the upload is accepted, still bound on the GPU. The
    /// staged block is the only allocation of the call.
    ///
    /// # Arguments
    ///
    /// - `&RayTraceScene` - The new scene.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the new scene is live.
    pub fn update_scene(&mut self, scene: &RayTraceScene) -> Result<(), JsValue> {
        if self.get_disposed() {
            return Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string()));
        }
        let staged: Vec<f32> = match scene.encode_gpu() {
            Ok(packed) => packed,
            Err(reason) => return Err(backend_error(ERROR_SCENE_PACK, reason)),
        };
        let outcome: Result<(), JsValue> = match self.get_mut_adapter().as_mut() {
            Some(adapter) => adapter.upload_scene(&staged),
            None => Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string())),
        };
        outcome?;
        self.set_scene(scene.clone());
        Ok(())
    }

    /// Traces one frame with the current scene at the given camera.
    ///
    /// A steady-state frame reuses every buffer it already owns. The
    /// Canvas path moves the pixel buffer out, traces into it, and moves
    /// it back whether the trace succeeded or failed, so a rejected
    /// frame still leaves the buffer allocated for the next attempt.
    /// The GPU paths refill the packed frame through its existing
    /// capacity and issue one draw.
    ///
    /// # Arguments
    ///
    /// - `&Camera3D` - The camera supplying the eye, target, up, and
    ///   field of view.
    /// - `&LightingUniforms` - The lights, ambient, and shading eye.
    /// - `&RayTraceSettings` - The sampling grid and bounce limit.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the frame is submitted.
    pub fn render(
        &mut self,
        camera: &Camera3D,
        lights: &LightingUniforms,
        settings: &RayTraceSettings,
    ) -> Result<(), JsValue> {
        if self.get_disposed() {
            return Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string()));
        }
        match self.get_backend() {
            RayTraceBackend::Canvas2D => self.render_canvas(camera, lights, settings),
            RayTraceBackend::WebGl2 | RayTraceBackend::WebGpu => {
                self.render_gpu(camera, lights, settings)
            }
        }
    }

    /// Traces the CPU frame and blits it through the reused image.
    ///
    /// # Arguments
    ///
    /// - `&Camera3D` - The camera supplying the eye, target, up, and
    ///   field of view.
    /// - `&LightingUniforms` - The lights, ambient, and shading eye.
    /// - `&RayTraceSettings` - The sampling grid and bounce limit.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the frame is blitted.
    fn render_canvas(
        &mut self,
        camera: &Camera3D,
        lights: &LightingUniforms,
        settings: &RayTraceSettings,
    ) -> Result<(), JsValue> {
        let mut pixels: Vec<u8> = mem::take(self.get_mut_pixels());
        let frame: RayTraceFrame<'_> = RayTraceFrame {
            camera,
            lights,
            settings,
            width: self.get_width(),
            height: self.get_height(),
        };
        let traced: Result<(), String> =
            render_raytrace_rgba(self.get_scene(), &frame, &mut pixels);
        let presented: Result<(), JsValue> = match self.get_mut_adapter().as_mut() {
            Some(adapter) => match traced {
                Ok(()) => adapter.present(&pixels),
                Err(reason) => Err(backend_error(ERROR_FRAME_PACK, reason)),
            },
            None => Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string())),
        };
        *self.get_mut_pixels() = pixels;
        presented
    }

    /// Refills the packed frame and issues the GPU draw.
    ///
    /// # Arguments
    ///
    /// - `&Camera3D` - The camera supplying the eye, target, up, and
    ///   field of view.
    /// - `&LightingUniforms` - The lights, ambient, and shading eye.
    /// - `&RayTraceSettings` - The sampling grid and bounce limit.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the frame is submitted.
    fn render_gpu(
        &mut self,
        camera: &Camera3D,
        lights: &LightingUniforms,
        settings: &RayTraceSettings,
    ) -> Result<(), JsValue> {
        let mut frame: Vec<f32> = mem::take(self.get_mut_packed_frame());
        let inputs: RayTraceFrame<'_> = RayTraceFrame {
            camera,
            lights,
            settings,
            width: self.get_width(),
            height: self.get_height(),
        };
        let packed: Result<(), String> = pack_raytrace_frame(&mut frame, &inputs);
        let drawn: Result<(), JsValue> = match (packed, self.get_mut_adapter().as_mut()) {
            (Ok(()), Some(adapter)) => adapter.draw(&frame),
            (Ok(()), None) => Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string())),
            (Err(reason), _) => Err(backend_error(ERROR_FRAME_PACK, reason)),
        };
        *self.get_mut_packed_frame() = frame;
        drawn
    }

    /// Resizes the surface, reallocating only what the new size needs.
    ///
    /// The call is transactional in both directions. The requested size
    /// is validated first, then the adapter applies it, because the
    /// adapter is the only step that can fail for a reason the shared
    /// layer cannot check: a lost context or a lost device. Only once
    /// that step has succeeded are the renderer's own size, the canvas
    /// backing store, and the CPU pixel buffer updated — so a rejected
    /// request leaves the canvas showing the previous, still valid,
    /// surface instead of one the renderer never adopted. The order
    /// within the commit does not matter, because nothing past the
    /// adapter can fail.
    ///
    /// A zero, overflowing, or oversized request is rejected before any
    /// state is touched. A size the hardware cannot address is an error
    /// rather than a silent clamp.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new width in pixels.
    /// - `u32` - The new height in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the new size is live.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        if self.get_disposed() {
            return Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string()));
        }
        let byte_length: usize = match self.get_mut_adapter().as_mut() {
            Some(adapter) => {
                let limit: Option<u32> = adapter.dimension_limit();
                let bytes: usize = validate_surface(width, height, limit)?;
                adapter.resize(width, height)?;
                bytes
            }
            None => {
                validate_surface(width, height, None)?;
                return Err(backend_error(ERROR_DISPOSED, ERROR_DISPOSED.to_string()));
            }
        };
        self.set_width(width);
        self.set_height(height);
        self.get_mut_canvas().set_width(width);
        self.get_mut_canvas().set_height(height);
        resize_pixel_buffer(self.get_mut_pixels(), byte_length);
        Ok(())
    }

    /// Releases every resource the renderer owns, idempotently.
    ///
    /// Calling this twice is safe: the second call finds the disposed
    /// flag already set and releases nothing. Cleanup also runs when the
    /// renderer is dropped, and the flag is set before the release so a
    /// renderer dropped right after an explicit dispose destroys nothing
    /// twice.
    pub fn dispose(&mut self) {
        if self.get_disposed() {
            return;
        }
        self.set_disposed(true);
        self.release_backend();
    }

    /// Destroys the adapter's resources at most once.
    pub(crate) fn release_backend(&mut self) {
        if let Some(mut adapter) = mem::take(self.get_mut_adapter()) {
            adapter.release();
        }
    }
}

impl Drop for RayTraceRenderer {
    /// Releases the backend resources the renderer owns.
    ///
    /// Cleanup is idempotent: a renderer that was explicitly disposed
    /// releases nothing here, and one dropped without being disposed
    /// releases exactly what it still owns, once.
    fn drop(&mut self) {
        if self.get_disposed() {
            return;
        }
        self.set_disposed(true);
        self.release_backend();
    }
}
