use super::*;

/// Inherent implementation of [`WebGlRayTraceBackend`].
impl WebGlRayTraceBackend {
    /// Compiles the ray-trace program and binds the fullscreen vertex
    /// array.
    ///
    /// Both shader stages are compiled and linked synchronously and the
    /// driver's own info log is returned verbatim on failure, so an
    /// invalid program is an error rather than a link that silently
    /// produced no executable.
    ///
    /// The data textures are deliberately not allocated here. Each is
    /// created at the shape of the block it will hold, on the first
    /// upload that needs it, so an attach reserves two placeholder texels
    /// rather than the whole addressable budget of two textures.
    ///
    /// Every step that can fail cleans up the objects the earlier steps
    /// created: a refused vertex array, a failed second texture, or a
    /// failed initial resize deletes the program, the vertex array, and
    /// whichever texture already existed before returning.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - The canvas to draw into.
    /// - `u32` - The surface width in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<Self, JsValue>` - The ready backend, or the reason the
    ///   WebGL 2 path could not be set up.
    pub fn attach(canvas: &HtmlCanvasElement, width: u32, height: u32) -> Result<Self, JsValue> {
        let context: WebGl2RenderingContext = attach_context(canvas)?;
        if context.is_context_lost() {
            return Err(backend_error(
                ERROR_WEBGL_CONTEXT_LOST,
                ERROR_WEBGL_CONTEXT_LOST.to_string(),
            ));
        }
        let program: WebGlProgram = link_raytrace_program(&context)?;
        let uniforms: Result<(WebGlUniformLocation, WebGlUniformLocation), JsValue> =
            match resolve_sampler(&context, &program, RAYTRACE_GLSL_UNIFORM_SCENE) {
                Ok(scene_uniform) => {
                    match resolve_sampler(&context, &program, RAYTRACE_GLSL_UNIFORM_FRAME) {
                        Ok(frame_uniform) => Ok((scene_uniform, frame_uniform)),
                        Err(reason) => Err(reason),
                    }
                }
                Err(reason) => Err(reason),
            };
        let (scene_uniform, frame_uniform): (WebGlUniformLocation, WebGlUniformLocation) =
            match uniforms {
                Ok(pair) => pair,
                Err(reason) => {
                    release_partial(
                        &context,
                        &WebGlPartialAttach {
                            program: Some(&program),
                            vao: None,
                            scene_texture: None,
                            frame_texture: None,
                        },
                    );
                    return Err(reason);
                }
            };
        let Some(vao) = context.create_vertex_array() else {
            release_partial(
                &context,
                &WebGlPartialAttach {
                    program: Some(&program),
                    vao: None,
                    scene_texture: None,
                    frame_texture: None,
                },
            );
            return Err(backend_error(
                ERROR_WEBGL_PROGRAM,
                ERROR_WEBGL_VAO_REJECTED.to_string(),
            ));
        };
        context.bind_vertex_array(Some(&vao));
        let texture_limit: u32 = texture_size_limit(&context);
        let dimension_limit: u32 = texture_limit.min(viewport_dimension_limit(&context));
        let scene_texture: WebGlRayTraceTexture = match create_packed_texture(
            &context,
            WEBGL_TEXTURE_UNIT_SCENE,
            WEBGL_PLACEHOLDER_TEXELS,
            WEBGL_PLACEHOLDER_TEXELS,
        ) {
            Ok(texture) => texture,
            Err(reason) => {
                release_partial(
                    &context,
                    &WebGlPartialAttach {
                        program: Some(&program),
                        vao: Some(&vao),
                        scene_texture: None,
                        frame_texture: None,
                    },
                );
                return Err(reason);
            }
        };
        let frame_texture: WebGlRayTraceTexture = match create_packed_texture(
            &context,
            WEBGL_TEXTURE_UNIT_FRAME,
            WEBGL_PLACEHOLDER_TEXELS,
            WEBGL_PLACEHOLDER_TEXELS,
        ) {
            Ok(texture) => texture,
            Err(reason) => {
                release_partial(
                    &context,
                    &WebGlPartialAttach {
                        program: Some(&program),
                        vao: Some(&vao),
                        scene_texture: Some(&scene_texture),
                        frame_texture: None,
                    },
                );
                return Err(reason);
            }
        };
        let mut backend: WebGlRayTraceBackend = WebGlRayTraceBackend {
            context,
            program,
            vao,
            scene_texture,
            frame_texture,
            scene_uniform,
            frame_uniform,
            scene_scratch: Vec::new(),
            frame_scratch: Vec::new(),
            dimension_limit_value: dimension_limit,
        };
        let resized: Result<(), JsValue> = backend.resize(width, height);
        match resized {
            Ok(()) => Ok(backend),
            Err(reason) => {
                backend.release();
                Err(reason)
            }
        }
    }

    /// The largest surface dimension, in pixels, this context accepts.
    ///
    /// The value is the smaller of the implementation's
    /// `MAX_TEXTURE_SIZE` and its `MAX_VIEWPORT_DIMS` pair: a surface
    /// larger than either cannot be drawn into. It is a per-dimension
    /// pixel count, not a byte budget.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - The limit, or `None` when the context exposes
    ///   none to enforce.
    pub fn dimension_limit(&self) -> Option<u32> {
        match self.get_dimension_limit_value() {
            0 => None,
            limit => Some(limit),
        }
    }

    /// Validates a new surface size and applies the full-surface
    /// viewport.
    ///
    /// The size is rejected before any state changes when the context is
    /// lost or when either dimension exceeds the per-axis limit, so a
    /// failed resize leaves the viewport, and therefore the previous,
    /// still renderable, surface, exactly as it was. Neither data
    /// texture is touched: they are sized by the packed blocks, not by
    /// the surface, so a resize creates and destroys no GL object.
    ///
    /// # Arguments
    ///
    /// - `u32` - The surface width in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the size is accepted.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        validate_webgl_surface(
            self.get_context(),
            width,
            height,
            self.get_dimension_limit_value(),
        )?;
        apply_full_viewport(self.get_context())
    }

    /// Refills the frame texture and issues the fullscreen draw.
    ///
    /// Only the frame block is uploaded here. The scene block was
    /// uploaded by [`WebGlRayTraceBackend::upload_scene`] and is left
    /// untouched, so a steady frame moves no scene texel.
    ///
    /// The viewport is re-derived from the drawing buffer here rather
    /// than trusting the one set at resize, because the browser resets
    /// it whenever the canvas is resized — including the resize the
    /// renderer performs after this backend accepted the new size — and
    /// a stale viewport draws the fullscreen triangle into a corner.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The packed frame block.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the draw was issued.
    pub fn draw(&mut self, frame: &[f32]) -> Result<(), JsValue> {
        if self.get_context().is_context_lost() {
            return Err(backend_error(
                ERROR_WEBGL_CONTEXT_LOST,
                ERROR_WEBGL_CONTEXT_LOST.to_string(),
            ));
        }
        let context: WebGl2RenderingContext = self.get_context().clone();
        let mut scratch: Vec<f32> = mem::take(self.get_mut_frame_scratch());
        let upload: WebGlPackedUpload = WebGlPackedUpload {
            context: &context,
            unit: WEBGL_TEXTURE_UNIT_FRAME,
            packed: frame,
        };
        let outcome: Result<(), JsValue> =
            ensure_packed_texture(&upload, self.get_mut_frame_texture(), &mut scratch);
        self.set_frame_scratch(scratch);
        outcome?;
        apply_full_viewport(self.get_context())?;
        self.get_context().use_program(Some(self.get_program()));
        self.get_context().bind_vertex_array(Some(self.get_vao()));
        bind_data_texture(
            self.get_context(),
            WEBGL_TEXTURE_UNIT_SCENE,
            self.get_scene_texture().get_texture(),
        )?;
        bind_data_texture(
            self.get_context(),
            WEBGL_TEXTURE_UNIT_FRAME,
            self.get_frame_texture().get_texture(),
        )?;
        self.get_context()
            .uniform1i(Some(self.get_scene_uniform()), WEBGL_SAMPLER_UNIT_SCENE);
        self.get_context()
            .uniform1i(Some(self.get_frame_uniform()), WEBGL_SAMPLER_UNIT_FRAME);
        require_clear_gl_error(
            self.get_context(),
            ERROR_WEBGL_CALL,
            WEBGL_ERROR_SOURCE_BINDING,
        )?;
        self.get_context().draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            WEBGL_FIRST_VERTEX,
            WEBGL_VERTEX_COUNT,
        );
        require_clear_gl_error(self.get_context(), ERROR_WEBGL_CALL, "drawArrays")
    }

    /// Uploads the packed scene block into its data texture.
    ///
    /// The texture grows to the shape the block needs when it outgrows
    /// the current one, and keeps its allocation when it does not, so a
    /// repeated update at a fixed size issues one `texSubImage2D` and
    /// creates no GL object. Growth allocates the new texture, writes
    /// the whole block into it, swaps it in, and only then deletes the
    /// texture it replaced — a block the driver refuses leaves the
    /// previous, still valid, scene bound and readable.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The packed scene block.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the block is live on the GPU.
    pub fn upload_scene(&mut self, packed: &[f32]) -> Result<(), JsValue> {
        if self.get_context().is_context_lost() {
            return Err(backend_error(
                ERROR_WEBGL_CONTEXT_LOST,
                ERROR_WEBGL_CONTEXT_LOST.to_string(),
            ));
        }
        let context: WebGl2RenderingContext = self.get_context().clone();
        let mut scratch: Vec<f32> = mem::take(self.get_mut_scene_scratch());
        let upload: WebGlPackedUpload = WebGlPackedUpload {
            context: &context,
            unit: WEBGL_TEXTURE_UNIT_SCENE,
            packed,
        };
        let outcome: Result<(), JsValue> =
            ensure_packed_texture(&upload, self.get_mut_scene_texture(), &mut scratch);
        self.set_scene_scratch(scratch);
        outcome
    }

    /// Deletes every GL object the backend created.
    ///
    /// # Returns
    ///
    /// - `()` - Cleanup never fails; an object the driver already
    ///   reclaimed is simply not deleted again. A lost context has
    ///   already discarded every object, so none is deleted.
    pub fn release(&mut self) {
        if self.get_context().is_context_lost() {
            return;
        }
        self.get_context()
            .delete_texture(Some(self.get_scene_texture().get_texture()));
        self.get_context()
            .delete_texture(Some(self.get_frame_texture().get_texture()));
        self.get_context().delete_vertex_array(Some(self.get_vao()));
        self.get_context().delete_program(Some(self.get_program()));
    }
}
