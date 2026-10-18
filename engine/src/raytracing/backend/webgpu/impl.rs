use super::*;

/// Wraps an allocation in a validation error scope, then reports what
/// the scope caught.
///
/// WebGPU reports allocation failures asynchronously, so the scope is
/// popped and read before the resource is handed back. An allocation
/// that the device rejected is an error, never a silently unusable
/// handle.
///
/// The allocation's own outcome is kept separate from the scope's: a
/// throw here must stay an `Err` even though the scope still has to be
/// popped first, so the device is not left with an unbalanced stack of
/// error scopes. Popping is delegated to
/// [`pop_validation_scope`](super::pop_validation_scope), so a scope
/// this helper cannot read is an error rather than a clean result.
///
/// An error scope cannot cover an asynchronous creation. The WebGPU
/// specification resolves `createRenderPipelineAsync` with a
/// `GPUPipelineError` and reports **no** `GPUError` to any error scope,
/// so a scope popped before the promise settles is guaranteed to be
/// empty whether the pipeline was accepted or rejected. Callers whose
/// allocation returns a promise must therefore await it themselves;
/// only a synchronous allocation may pass through this helper.
///
/// # Arguments
///
/// - `&JsValue` - The device owning the scope.
/// - `F` - The allocation to perform, bound to return the resource the
///   scope must then vouch for.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The allocated resource, or the
///   validation message the scope reported.
async fn scoped<F>(device: &JsValue, allocate: F) -> Result<JsValue, JsValue>
where
    F: FnOnce() -> Result<JsValue, JsValue>,
{
    let scope_argument: JsValue = JsValue::from_str(BACKEND_WEBGPU_SCOPE_VALIDATION);
    let pushed: Result<JsValue, JsValue> =
        call1(device, BACKEND_WEBGPU_METHOD_PUSH_SCOPE, &scope_argument);
    pushed?;
    let allocation: Result<JsValue, JsValue> = allocate();
    let observed: Result<Option<String>, JsValue> = pop_validation_scope(device).await;
    let resource: JsValue = allocation?;
    let scope: Option<String> = observed?;
    match scope {
        Some(message) => Err(backend_error(ERROR_WEBGPU_CALL, message)),
        None => Ok(resource),
    }
}

/// Creates the render pipeline and awaits its asynchronous creation.
///
/// `createRenderPipelineAsync` returns a promise rather than a
/// pipeline, so the promise *is* the pipeline as far as every later
/// call is concerned: handing `setPipeline` the pending promise fails
/// with `parameter 1 is not of type 'GPURenderPipeline'` while the
/// rejection lands in an unhandled rejection instead of as an `Err`.
///
/// The promise is therefore awaited here, before the descriptor's
/// error scope is popped, for two reasons. Awaiting attaches the
/// rejection handler in the same synchronous step that created the
/// promise, so a browser that rejects during the same task still finds
/// a handler and never raises an unhandled rejection. Popping the
/// scope first would leave an await point between the creation and its
/// handler, which is exactly the window in which an unobserved
/// rejection is reported.
///
/// The scope is still popped either way: a rejection leaves the
/// device's scope stack unbalanced otherwise, and the inner `Err` is
/// kept rather than replaced by whatever the scope reported. Popping
/// is therefore not best-effort — the `popErrorScope` call, the
/// promise it returns, and the `GPUError` that promise resolves to are
/// each checked, because a scope this backend pushed and then failed
/// to read is indistinguishable from one the device never validated.
///
/// Both layers matter for different failures. A throw from
/// `popErrorScope` means the scope was never closed, and a rejected
/// `popErrorScope` promise means it was closed but its result is
/// lost; either way the scope's own verdict on the creation is
/// unknown, so a pipeline accepted on a device that could not report
/// is never treated as validated. The pipeline's own outcome still
/// wins, because a `GPUPipelineError` names the real failure while a
/// scope message may describe any validation the descriptor provoked.
///
/// # Arguments
///
/// - `&JsValue` - The device to create the pipeline on.
/// - `&JsValue` - The compiled WGSL module.
/// - `&JsValue` - The layout both packed buffers bind against.
/// - `&str` - The swap-chain format the fragment stage must target.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The resolved pipeline, or the reason
///   the browser rejected creating it.
async fn create_render_pipeline(
    device: &JsValue,
    module: &JsValue,
    layout: &JsValue,
    format: &str,
) -> Result<JsValue, JsValue> {
    let layout_list: Array = Array::new();
    layout_list.push(layout.as_ref());
    let layout_descriptor: Object = descriptor(&[(
        BACKEND_WEBGPU_PROPERTY_BIND_GROUP_LAYOUTS,
        layout_list.into(),
    )])?;
    let pipeline_layout: JsValue = call1(
        device,
        BACKEND_WEBGPU_METHOD_CREATE_PIPELINE_LAYOUT,
        layout_descriptor.as_ref(),
    )?;
    let stage_descriptor: Object = descriptor(&[
        (BACKEND_WEBGPU_PROPERTY_MODULE, module.clone()),
        (
            BACKEND_WEBGPU_PROPERTY_ENTRY_POINT,
            JsValue::from_str(BACKEND_WEBGPU_ENTRY_VERTEX),
        ),
    ])?;
    let stage: JsValue = stage_descriptor.into();
    let target_descriptor: Object = descriptor(&[(
        BACKEND_WEBGPU_PROPERTY_FORMAT_TARGET,
        JsValue::from_str(format),
    )])?;
    let targets: Array = Array::new();
    targets.push(target_descriptor.as_ref());
    let fragment_descriptor: Object = descriptor(&[
        (BACKEND_WEBGPU_PROPERTY_MODULE, module.clone()),
        (
            BACKEND_WEBGPU_PROPERTY_ENTRY_POINT,
            JsValue::from_str(BACKEND_WEBGPU_ENTRY_FRAGMENT),
        ),
        (BACKEND_WEBGPU_PROPERTY_TARGETS, targets.into()),
    ])?;
    let fragment: JsValue = fragment_descriptor.into();
    let primitive_descriptor: Object = descriptor(&[(
        BACKEND_WEBGPU_PROPERTY_TOPOLOGY,
        JsValue::from_str(BACKEND_WEBGPU_TOPOLOGY_TRIANGLE_LIST),
    )])?;
    let primitive: JsValue = primitive_descriptor.into();
    let pairs: [(&str, JsValue); 4] = [
        (BACKEND_WEBGPU_PROPERTY_LAYOUT, pipeline_layout),
        (BACKEND_WEBGPU_PROPERTY_VERTEX, stage),
        (BACKEND_WEBGPU_PROPERTY_FRAGMENT, fragment),
        (BACKEND_WEBGPU_PROPERTY_PRIMITIVE, primitive),
    ];
    let scope_argument: JsValue = JsValue::from_str(BACKEND_WEBGPU_SCOPE_VALIDATION);
    let pushed: Result<JsValue, JsValue> =
        call1(device, BACKEND_WEBGPU_METHOD_PUSH_SCOPE, &scope_argument);
    pushed?;
    let created: Result<JsValue, JsValue> = match descriptor(&pairs) {
        Ok(pipeline_descriptor) => call1(
            device,
            BACKEND_WEBGPU_METHOD_CREATE_PIPELINE,
            pipeline_descriptor.as_ref(),
        ),
        Err(reason) => Err(reason),
    };
    let resolved: Result<JsValue, JsValue> = match created {
        Ok(pending) => await_promise(pending).await,
        Err(reason) => Err(reason),
    };
    let observed: Result<Option<String>, JsValue> = pop_validation_scope(device).await;
    let pipeline: JsValue = resolved?;
    if pipeline.is_undefined() || pipeline.is_null() {
        return Err(backend_error(
            BACKEND_WEBGPU_ERROR_PIPELINE,
            "createRenderPipelineAsync returned nothing".to_string(),
        ));
    }
    let validation: Option<String> = observed?;
    if let Some(message) = validation {
        return Err(backend_error(ERROR_WEBGPU_CALL, message));
    }
    Ok(pipeline)
}

/// Creates one storage buffer under a popped-and-inspected error scope.
///
/// # Arguments
///
/// - `&JsValue` - The device to allocate from.
/// - `u32` - The byte size to allocate.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The buffer, or the validation message
///   the scope reported.
async fn create_storage_buffer(device: &JsValue, bytes: u32) -> Result<JsValue, JsValue> {
    let usage: u32 = BACKEND_WEBGPU_USAGE_STORAGE | BACKEND_WEBGPU_USAGE_COPY_DST;
    let pairs: [(&str, JsValue); 2] = [
        (
            BACKEND_WEBGPU_PROPERTY_SIZE,
            JsValue::from_f64(bytes as f64),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_USAGE_FLAGS,
            JsValue::from_f64(usage as f64),
        ),
    ];
    match scoped(device, || -> Result<JsValue, JsValue> {
        let descriptor: Object = descriptor(&pairs)?;
        call1(
            device,
            BACKEND_WEBGPU_METHOD_CREATE_BUFFER,
            descriptor.as_ref(),
        )
    })
    .await
    {
        Ok(buffer) if !buffer.is_undefined() => Ok(buffer),
        Ok(_) => Err(backend_error(
            ERROR_WEBGPU_CALL,
            "createBuffer returned nothing".to_string(),
        )),
        Err(reason) => Err(reason),
    }
}

/// Creates the shader module and awaits its compilation info.
///
/// A WGSL module compiles into an object even when it is invalid, so the
/// compiler's own errors are read from `getCompilationInfo` rather than
/// inferred from the absence of a throw.
///
/// # Arguments
///
/// - `&JsValue` - The device to compile on.
/// - `&str` - The WGSL source.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The compiled module, or the compiler's
///   own errors.
async fn create_shader_module(device: &JsValue, source: &str) -> Result<JsValue, JsValue> {
    let pairs: [(&str, JsValue); 2] = [
        (BACKEND_WEBGPU_PROPERTY_CODE, JsValue::from_str(source)),
        (
            BACKEND_WEBGPU_PROPERTY_LABEL,
            JsValue::from_str(BACKEND_WEBGPU_SHADER_LABEL),
        ),
    ];
    let module: JsValue = scoped(device, || -> Result<JsValue, JsValue> {
        let module_descriptor: Object = descriptor(&pairs)?;
        call1(
            device,
            BACKEND_WEBGPU_METHOD_CREATE_MODULE,
            module_descriptor.as_ref(),
        )
    })
    .await?;
    if module.is_undefined() || module.is_null() {
        return Err(backend_error(
            BACKEND_WEBGPU_ERROR_SHADER,
            "createShaderModule returned nothing".to_string(),
        ));
    }
    let pending: JsValue = call0(&module, BACKEND_WEBGPU_METHOD_COMPILATION_INFO)?;
    let resolved: JsValue = await_promise(pending).await?;
    let errors: String = compilation_messages(&resolved);
    if !errors.is_empty() {
        return Err(backend_error(BACKEND_WEBGPU_ERROR_SHADER, errors));
    }
    Ok(module)
}

/// Requests the WebGPU adapter, refusing to fall back.
///
/// # Arguments
///
/// - `&JsValue` - The `GPU` object.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The adapter, or the reason none is
///   available.
async fn request_adapter(gpu: &JsValue) -> Result<JsValue, JsValue> {
    let pending: JsValue = call0(gpu, BACKEND_WEBGPU_METHOD_REQUEST_ADAPTER)?;
    let resolved: JsValue = await_promise(pending).await?;
    if resolved.is_null() || resolved.is_undefined() {
        return Err(backend_error(
            ERROR_WEBGPU_ADAPTER,
            ERROR_WEBGPU_NO_ADAPTER.to_string(),
        ));
    }
    Ok(resolved)
}

/// Requests the device from an adapter.
///
/// # Arguments
///
/// - `&JsValue` - The adapter to request from.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The device, or the reason the request
///   produced none.
async fn request_device(adapter: &JsValue) -> Result<JsValue, JsValue> {
    let empty: Object = Object::new();
    let pending: JsValue = call1(
        adapter,
        BACKEND_WEBGPU_METHOD_REQUEST_DEVICE,
        empty.as_ref(),
    )?;
    let resolved: JsValue = await_promise(pending).await?;
    if resolved.is_null() || resolved.is_undefined() {
        return Err(backend_error(
            ERROR_WEBGPU_DEVICE,
            ERROR_WEBGPU_DEVICE.to_string(),
        ));
    }
    Ok(resolved)
}

impl WebGpuRayTraceBackend {
    /// Acquires the device, compiles the shader, and builds the
    /// pipeline, bind group, and both packed buffers.
    ///
    /// Everything WebGPU validates asynchronously is awaited here: the
    /// shader's compilation info, the pipeline creation, and every
    /// allocation's error scope. An invalid shader or pipeline is an
    /// error returned from `attach` rather than a black frame.
    ///
    /// Both buffers start small and grow to the geometry the scene
    /// actually needs. Reserving the whole packed budget up front
    /// would ask for more than the storage-binding limit most adapters
    /// report and fail the first allocation outright.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - The canvas to draw into.
    ///
    /// # Returns
    ///
    /// - `Result<Self, JsValue>` - The ready backend, or the reason the
    ///   WebGPU path could not be set up.
    pub async fn attach(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let gpu: JsValue = navigator_gpu()?;
        let adapter: JsValue = request_adapter(&gpu).await?;
        let device: JsValue = request_device(&adapter).await?;
        let owned: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let failure: Rc<RefCell<Option<String>>> = Rc::clone(&owned);
        let mut context: Option<JsValue> = None;
        let mut scene_buffer: Option<JsValue> = None;
        let mut frame_buffer: Option<JsValue> = None;
        let mut listener: Option<JsValue> = None;
        let built: Result<Self, JsValue> = {
            let observed: Result<JsValue, JsValue> = observe_device(&device, &failure);
            async {
                let error_listener: JsValue = observed?;
                listener = Some(error_listener.clone());
                let format_name: String = preferred_format(&gpu);
                let surface: JsValue = configure_context(canvas, &device, &format_name)?;
                context = Some(surface.clone());
                let queue: JsValue = attribute(&device, BACKEND_WEBGPU_PROPERTY_QUEUE)?;
                let layout: JsValue = packed_layout(&device)?;
                let module: JsValue = create_shader_module(&device, raytrace_wgsl()).await?;
                let pipeline: JsValue =
                    create_render_pipeline(&device, &module, &layout, &format_name).await?;
                let first: JsValue =
                    create_storage_buffer(&device, BACKEND_WEBGPU_INITIAL_BUFFER_BYTES).await?;
                scene_buffer = Some(first.clone());
                let second: JsValue =
                    create_storage_buffer(&device, BACKEND_WEBGPU_INITIAL_BUFFER_BYTES).await?;
                frame_buffer = Some(second.clone());
                let bind_group: JsValue = bind_group_of(&device, &first, &second, &layout)?;
                Ok(WebGpuRayTraceBackend {
                    context: surface,
                    device: device.clone(),
                    queue,
                    layout,
                    scene_buffer: first,
                    frame_buffer: second,
                    bind_group,
                    pipeline,
                    scene_capacity: BACKEND_WEBGPU_INITIAL_BUFFER_BYTES,
                    frame_capacity: BACKEND_WEBGPU_INITIAL_BUFFER_BYTES,
                    storage_binding_limit: device_limit(
                        &device,
                        BACKEND_WEBGPU_LIMIT_STORAGE_BINDING,
                    ),
                    buffer_size_limit: device_limit(&device, BACKEND_WEBGPU_LIMIT_BUFFER_SIZE),
                    texture_dimension_limit: device_limit(&device, BACKEND_WEBGPU_LIMIT_TEXTURE_2D),
                    failure: owned,
                    error_listener: listener.take(),
                    released: false,
                })
            }
            .await
        };
        match built {
            Ok(backend) => Ok(backend),
            Err(reason) => {
                // A partially built backend registered the listener
                // before it could fail, so the listener is removed here
                // for the same reason `release` removes it: the device is
                // about to be destroyed and must not keep calling into
                // a backend that will never exist.
                if let Some(registered) = listener.as_ref() {
                    unregister_error_listener(&device, registered);
                }
                abandon(
                    &device,
                    scene_buffer.as_ref(),
                    frame_buffer.as_ref(),
                    context.as_ref(),
                );
                Err(reason)
            }
        }
    }

    /// The largest surface edge in pixels this device accepts.
    ///
    /// The limit counts pixels, not bytes: a surface is bounded per
    /// edge, so this is the only number a width or a height is checked
    /// against.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - The device's per-edge pixel limit, or `None`
    ///   when the device reports none.
    pub fn dimension_limit(&self) -> Option<u32> {
        match self.get_texture_dimension_limit() {
            0 => None,
            limit => Some(limit),
        }
    }

    /// Validates a requested surface size against the device's edge
    /// limit.
    ///
    /// The canvas is not touched here: the renderer commits its own
    /// dimensions only after this succeeds, so a rejected size leaves
    /// the previous, still valid, surface intact.
    ///
    /// # Arguments
    ///
    /// - `u32` - The requested width in pixels.
    /// - `u32` - The requested height in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` when both edges fit the device.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        rejection(self.pending_failure())?;
        if width == 0 || height == 0 {
            return Err(backend_error(
                ERROR_ZERO_DIMENSION,
                format!("requested {width}x{height}"),
            ));
        }
        let ceiling: u32 = self.get_texture_dimension_limit();
        if ceiling != 0 && (width > ceiling || height > ceiling) {
            return Err(backend_error(
                ERROR_GPU_LIMIT,
                format!("{width}x{height} exceeds the {ceiling} pixel edge limit"),
            ));
        }
        Ok(())
    }

    /// Grows one packed buffer when a block no longer fits.
    ///
    /// The device limit is checked before the allocation, so an
    /// oversized block is rejected synchronously and the previous, still
    /// valid, buffer stays bound. Growth then allocates, rebinds, and
    /// only destroys the old buffer once every step that could fail has
    /// succeeded — the reverse order would leave the backend with no
    /// scene buffer at all if the rebinding failed.
    ///
    /// The new bind group is created against the persistent layout, not
    /// a fresh one: a bind group records the layout it was made with, so
    /// a different layout would be rejected by the pipeline at draw
    /// time.
    ///
    /// A failure between allocating the new buffer and swapping it in is
    /// transactional in both directions: the new buffer is released, so
    /// nothing is left bound to nothing, and the old buffer, capacity,
    /// and bind group are all left untouched, so the previous scene
    /// still renders and a later call may retry. The error reported is
    /// the one the failing step produced, never the outcome of the
    /// release that follows it.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether this is the scene buffer rather than the frame
    ///   buffer.
    /// - `u32` - The byte size the block needs.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the buffer can hold the
    ///   block.
    fn ensure_capacity(&mut self, scene: bool, bytes: u32) -> Result<(), JsValue> {
        let current: u32 = if scene {
            self.get_scene_capacity()
        } else {
            self.get_frame_capacity()
        };
        if bytes <= current {
            return Ok(());
        }
        let ceiling: u32 = self.byte_ceiling();
        if ceiling != 0 && bytes > ceiling {
            return Err(backend_error(
                ERROR_GPU_LIMIT,
                format!("a packed block of {bytes} bytes exceeds the device limit {ceiling}"),
            ));
        }
        if bytes == 0 {
            return Err(backend_error(
                ERROR_WEBGPU_CALL,
                "a packed block of zero bytes cannot back a storage buffer".to_string(),
            ));
        }
        let usage: u32 = BACKEND_WEBGPU_USAGE_STORAGE | BACKEND_WEBGPU_USAGE_COPY_DST;
        let buffer_descriptor: Object = descriptor(&[
            (
                BACKEND_WEBGPU_PROPERTY_SIZE,
                JsValue::from_f64(bytes as f64),
            ),
            (
                BACKEND_WEBGPU_PROPERTY_USAGE_FLAGS,
                JsValue::from_f64(usage as f64),
            ),
        ])?;
        let device: JsValue = self.get_device();
        let grown: JsValue = call1(
            &device,
            BACKEND_WEBGPU_METHOD_CREATE_BUFFER,
            buffer_descriptor.as_ref(),
        )?;
        if grown.is_undefined() || grown.is_null() {
            return Err(backend_error(
                ERROR_WEBGPU_CALL,
                "createBuffer returned nothing while growing a packed buffer".to_string(),
            ));
        }
        let layout: JsValue = self.get_layout();
        let frame_buffer: JsValue = self.get_frame_buffer();
        let scene_buffer: JsValue = self.get_scene_buffer();
        let bound: Result<JsValue, JsValue> = if scene {
            bind_group_of(&device, &grown, &frame_buffer, &layout)
        } else {
            bind_group_of(&device, &scene_buffer, &grown, &layout)
        };
        let group: JsValue = match bound {
            Ok(built) => built,
            Err(reason) => {
                // The grown buffer was allocated before the bind group
                // was built, and the caller keeps the previous buffer
                // bound: the old scene stays valid and a later retry
                // allocates again. The new buffer is therefore
                // unreachable from here on, so it is destroyed before
                // the original error propagates. Its own destruction
                // outcome is discarded — a cleanup failure must not
                // replace the real one, which is the only thing the
                // caller can act on.
                let _: Result<(), JsValue> = destroy_buffer(&grown);
                return Err(reason);
            }
        };
        let previous: JsValue = if scene { scene_buffer } else { frame_buffer };
        if scene {
            self.set_scene_buffer(grown);
            self.set_scene_capacity(bytes);
        } else {
            self.set_frame_buffer(grown);
            self.set_frame_capacity(bytes);
        }
        self.set_bind_group(group);
        // The old buffer is unbound and already replaced, so it can
        // only now be destroyed. Destroying it earlier would, if the
        // rebinding had failed, leave the backend with neither.
        let _: Result<(), JsValue> = destroy_buffer(&previous);
        Ok(())
    }

    /// Uploads the packed scene block into its storage buffer.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The packed scene block.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the upload was issued.
    pub fn upload_scene(&mut self, packed: &[f32]) -> Result<(), JsValue> {
        rejection(self.pending_failure())?;
        let bytes: u32 = packed_byte_length(packed.len());
        self.ensure_capacity(true, bytes)?;
        let queue: JsValue = self.get_queue();
        let buffer: JsValue = self.get_scene_buffer();
        write_packed(&queue, &buffer, packed)
    }

    /// Refills the frame buffer and submits the fullscreen draw.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The packed frame block.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the frame is submitted.
    pub fn draw(&mut self, frame: &[f32]) -> Result<(), JsValue> {
        rejection(self.pending_failure())?;
        let bytes: u32 = packed_byte_length(frame.len());
        self.ensure_capacity(false, bytes)?;
        let queue: JsValue = self.get_queue();
        let frame_buffer: JsValue = self.get_frame_buffer();
        write_packed(&queue, &frame_buffer, frame)?;
        let device: JsValue = self.get_device();
        let context: JsValue = self.get_context();
        let texture: JsValue = call0(&context, BACKEND_WEBGPU_METHOD_CURRENT_TEXTURE)?;
        if texture.is_undefined() || texture.is_null() {
            return Err(backend_error(
                BACKEND_WEBGPU_ERROR_TEXTURE,
                "getCurrentTexture returned nothing".to_string(),
            ));
        }
        // A colour attachment takes a view, not a texture: handing it
        // the texture itself invalidates the render pass with nothing
        // thrown, which is exactly the silent black frame this backend
        // must not produce.
        let view: JsValue = call0(&texture, BACKEND_WEBGPU_METHOD_CREATE_VIEW)?;
        if view.is_undefined() || view.is_null() {
            return Err(backend_error(
                BACKEND_WEBGPU_ERROR_VIEW,
                "createView returned nothing".to_string(),
            ));
        }
        let attachment: Object = clear_attachment(&view)?;
        let attachments: Array = Array::new();
        attachments.push(attachment.as_ref());
        let pass_descriptor: Object = descriptor(&[(
            BACKEND_WEBGPU_PROPERTY_COLOR_ATTACHMENTS,
            attachments.into(),
        )])?;
        let encoder: JsValue = call0(&device, BACKEND_WEBGPU_METHOD_CREATE_ENCODER)?;
        let pass: JsValue = call1(
            &encoder,
            BACKEND_WEBGPU_METHOD_BEGIN_PASS,
            pass_descriptor.as_ref(),
        )?;
        let pipeline: JsValue = self.get_pipeline();
        call1(&pass, BACKEND_WEBGPU_METHOD_SET_PIPELINE, &pipeline)?;
        let group: JsValue = self.get_bind_group();
        let index: JsValue = JsValue::from_f64(BACKEND_WEBGPU_BIND_GROUP_INDEX as f64);
        call2(&pass, BACKEND_WEBGPU_METHOD_SET_BIND_GROUP, &index, &group)?;
        let args: DrawArgs = DrawArgs {
            vertex_count: BACKEND_WEBGPU_VERTEX_COUNT,
            instance_count: BACKEND_WEBGPU_INSTANCE_COUNT,
            first_vertex: BACKEND_WEBGPU_FIRST_VERTEX,
            first_instance: BACKEND_WEBGPU_FIRST_INSTANCE,
        };
        draw_vertices(&pass, &args)?;
        call0(&pass, BACKEND_WEBGPU_METHOD_END)?;
        let command: JsValue = call0(&encoder, BACKEND_WEBGPU_METHOD_FINISH)?;
        let batch: Array = Array::new();
        batch.push(command.as_ref());
        call1(&queue, BACKEND_WEBGPU_METHOD_SUBMIT, batch.as_ref())?;
        rejection(self.pending_failure())
    }

    /// Destroys every GPU resource the backend created, including its
    /// device.
    ///
    /// The device is destroyed because this backend requested it: it is
    /// the backend's own, not a shared one borrowed from a caller, and
    /// leaving it alive would pin every buffer, texture, and pipeline
    /// the browser had already reclaimed. A second call finds the
    /// released flag and destroys nothing twice.
    ///
    /// The `uncapturederror` listener is removed while the device is
    /// still alive: it owns a Rust callback, and destroying the device
    /// leaves the device free to stop calling it, but the reference it
    /// holds has to be handed back explicitly. The `device.lost`
    /// callback is not removed — destroying the device settles that
    /// promise, which is exactly what frees its one-shot callback.
    ///
    /// # Returns
    ///
    /// - `()` - Cleanup never fails and is never repeated.
    pub fn release(&mut self) {
        if self.get_released() {
            return;
        }
        self.set_released(true);
        let device: JsValue = self.get_device();
        if let Some(listener) = self.get_error_listener() {
            unregister_error_listener(&device, &listener);
            self.set_error_listener(None);
        }
        let context: JsValue = self.get_context();
        let _: Result<JsValue, JsValue> = call0(&context, BACKEND_WEBGPU_METHOD_UNCONFIGURE);
        let scene: JsValue = self.get_scene_buffer();
        let _: Result<(), JsValue> = destroy_buffer(&scene);
        let frame: JsValue = self.get_frame_buffer();
        let _: Result<(), JsValue> = destroy_buffer(&frame);
        destroy_device(&device);
    }
}

impl Drop for WebGpuRayTraceBackend {
    /// Releases the device when the backend is dropped without an
    /// explicit release.
    ///
    /// # Returns
    ///
    /// - `()` - Cleanup never fails.
    fn drop(&mut self) {
        self.release();
    }
}

impl WebGpuRayTraceBackend {
    /// Reports the failure the browser recorded, if any.
    ///
    /// A device loss is sticky: it is never drained, because a lost
    /// device cannot recover and clearing the record would let a later
    /// `resize` or `update_scene` report success on a device whose
    /// every subsequent submission is silently dropped. An
    /// `uncapturederror` is transient and *is* drained, so the frame
    /// that follows the invalid command fails and later frames work
    /// again — that is the contract the validation-error path relies
    /// on.
    ///
    /// A concurrent borrow is the frame whose handler is still
    /// recording, not a failure: it yields the placeholder rather than
    /// panicking, because a lost device must report its reason instead
    /// of aborting the page.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The recorded diagnostic, or `None` when the
    ///   device is healthy.
    pub(crate) fn pending_failure(&self) -> Option<String> {
        let mut cell: RefMut<Option<String>> = match self.get_failure().try_borrow_mut() {
            Ok(borrowed) => borrowed,
            Err(_) => return Some(WEBGPU_FAILURE_BUSY.to_string()),
        };
        match cell.as_ref() {
            Some(recorded) if is_device_lost(recorded) => Some(recorded.clone()),
            Some(_) => cell.take(),
            None => None,
        }
    }

    /// The device's largest storage binding and buffer sizes in bytes,
    /// as the smaller of the two.
    ///
    /// A binding may not exceed either limit, and taking one number
    /// means the capacity check and the allocation check cannot
    /// disagree about which is the binding's ceiling.
    ///
    /// # Returns
    ///
    /// - `u32` - The byte ceiling, or zero when the device reports
    ///   neither limit.
    pub(crate) fn byte_ceiling(&self) -> u32 {
        let binding: u32 = self.get_storage_binding_limit();
        let buffer: u32 = self.get_buffer_size_limit();
        match (binding, buffer) {
            (0, 0) => 0,
            (0, size) => size,
            (size, 0) => size,
            (first, second) => first.min(second),
        }
    }
}
