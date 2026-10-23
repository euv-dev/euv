use super::*;

/// Inherent implementation of [`WebGpuChartBackend`].
impl WebGpuChartBackend {
    /// Returns `true` when the host browser exposes WebGPU.
    ///
    /// The check is synchronous and cheap: it only looks for
    /// `navigator.gpu`. It says nothing about whether an adapter can
    /// actually be acquired — headless browsers, GPU-blacklisted devices,
    /// and sandboxed frames all expose the entry point while failing to
    /// hand out a device. For the definitive answer, create the backend:
    /// [`WebGpuChartBackend::create`] asks for an adapter and a device for
    /// real, and returns `None` when either fails.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_supported() -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };
        let navigator: Navigator = window.navigator();
        match js_sys::Reflect::get(navigator.as_ref(), &JsValue::from_str(WEBGPU_PROPERTY_GPU)) {
            Ok(value) => !value.is_undefined() && !value.is_null(),
            Err(_) => false,
        }
    }

    /// Creates a backend around the given canvas.
    ///
    /// The whole acquisition is asynchronous: `requestAdapter` and
    /// `requestDevice` return JavaScript promises. Every failure mode —
    /// no `navigator.gpu`, no adapter, no device, a canvas that already
    /// has a context of another kind — returns `None`, so the backend
    /// factory can fall back to WebGL2 and then to Canvas2D.
    ///
    /// The GPU buffers are not allocated here. They are created on the
    /// first render, once the size of the glyph atlas is known, and then
    /// kept until the display list changes.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - The `&HtmlCanvasElement` value.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The `Option<Self>` value.
    pub async fn create(canvas: &HtmlCanvasElement) -> Option<Self> {
        let gpu: JsValue = Self::gpu_object()?;
        let adapter: JsValue = Self::request_adapter(&gpu).await?;
        let device: JsValue = Self::request_device(&adapter).await?;
        let queue: JsValue = gpu_queue(&device)?;
        let context: JsValue = Self::canvas_context(canvas)?;
        let format: String = Self::preferred_format(&gpu)?;
        Self::configure_context(&context, &device, &format)?;
        let api: WebGpuApi = WebGpuApi::new(&device, &queue, &context)?;

        let uniform_layout: JsValue = Self::create_uniform_layout(&device)?;
        let texture_layout: JsValue = Self::create_texture_layout(&device)?;
        let solid_layout: JsValue =
            Self::create_pipeline_layout(&device, from_ref(&uniform_layout))?;
        let textured_layout: JsValue = Self::create_pipeline_layout(
            &device,
            &[uniform_layout.clone(), texture_layout.clone()],
        )?;

        let solid_shader: JsValue = Self::create_shader_module(&device, WEBGPU_SOLID_SHADER)?;
        let textured_shader: JsValue = Self::create_shader_module(&device, WEBGPU_TEXTURED_SHADER)?;
        let solid_pipeline: JsValue =
            Self::create_render_pipeline(&device, &solid_layout, &solid_shader, &format, false)?;
        let textured_pipeline: JsValue = Self::create_render_pipeline(
            &device,
            &textured_layout,
            &textured_shader,
            &format,
            true,
        )?;

        let uniform_buffer: JsValue = Self::create_buffer(
            &device,
            WEBGPU_UNIFORM_SIZE,
            WEBGPU_BUFFER_USAGE_UNIFORM | WEBGPU_BUFFER_USAGE_COPY_DST,
        )?;
        let uniform_bind_group: JsValue = Self::create_bind_group(
            &device,
            &uniform_layout,
            &[gpu_object(&[
                (WEBGPU_KEY_BINDING, &JsValue::from_f64(0.0)),
                (
                    WEBGPU_KEY_RESOURCE,
                    &gpu_object(&[(WEBGPU_KEY_BUFFER, &uniform_buffer)]),
                ),
            ])],
        )?;

        let sampler: JsValue = Self::create_sampler(&device)?;
        let (pass_descriptor, pass_attachment) = Self::create_pass_descriptor();

        Some(Self {
            canvas: canvas.clone(),
            device,
            queue,
            context,
            format,
            api,
            encoder_api: None,
            pass_api: None,
            solid_pipeline,
            textured_pipeline,
            texture_layout,
            sampler,
            uniform_buffer,
            uniform_bind_group,
            pass_descriptor,
            pass_attachment,
            atlas: WebGpuAtlas {
                texture: None,
                bind_group: None,
                width: 0,
                height: 0,
            },
            solid_buffer: WebGpuBuffer {
                buffer: None,
                capacity: 0,
            },
            solid_index_buffer: WebGpuBuffer {
                buffer: None,
                capacity: 0,
            },
            textured_buffer: WebGpuBuffer {
                buffer: None,
                capacity: 0,
            },
            textured_index_buffer: WebGpuBuffer {
                buffer: None,
                capacity: 0,
            },
            batches: Vec::new(),
            images: Vec::new(),
            next_image_id: 1,
            atlas_glyphs: 0,
            fingerprint: None,
            tessellator: Tessellator::new(),
            width: 0.0,
            height: 0.0,
            dpr: 1.0,
            solid_vertices: Vec::new(),
            solid_indices: Vec::new(),
            textured_vertices: Vec::new(),
            textured_indices: Vec::new(),
            clips: Vec::new(),
        })
    }

    /// Returns the canvas the backend draws into.
    ///
    /// # Returns
    ///
    /// - `&HtmlCanvasElement` - A `HtmlCanvasElement` value.
    pub fn canvas(&self) -> &HtmlCanvasElement {
        self.get_canvas()
    }

    /// Returns the device pixel ratio the backend renders at.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn dpr(&self) -> f64 {
        self.get_dpr()
    }

    /// Returns the swap-chain texture format the backend renders into.
    ///
    /// # Returns
    ///
    /// - `&str` - A `str` value.
    pub fn format(&self) -> &str {
        self.get_format()
    }

    /// Returns the number of draw calls the last frame used.
    ///
    /// One batch is one draw call, so this is the number the chart pays
    /// per frame. A chart that has not changed keeps the same number.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn batch_count(&self) -> usize {
        self.get_batches()
            .iter()
            .filter(|batch: &&WebGpuBatch| batch.index_count > 0)
            .count()
    }

    /// Forces the backend to rebuild its geometry on the next render.
    ///
    /// The backend notices a changed display list on its own, by
    /// fingerprinting it. This is for the cases the fingerprint cannot
    /// see — a web font that finished loading, for instance, changes the
    /// metrics of the text without changing the display list.
    pub fn invalidate(&mut self) {
        self.set_fingerprint(None);
    }

    /// Returns the `navigator.gpu` object, if the host has one.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn gpu_object() -> Option<JsValue> {
        let window: Window = web_sys::window()?;
        let navigator: Navigator = window.navigator();
        let gpu: JsValue =
            js_sys::Reflect::get(navigator.as_ref(), &JsValue::from_str(WEBGPU_PROPERTY_GPU))
                .ok()?;
        if gpu.is_undefined() || gpu.is_null() {
            return None;
        }
        Some(gpu)
    }

    /// Requests an adapter from the `GPU` object.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    async fn request_adapter(gpu: &JsValue) -> Option<JsValue> {
        let method: Function = gpu_method(gpu, WEBGPU_METHOD_REQUEST_ADAPTER)?;
        let options: JsValue = gpu_object(&[(
            "powerPreference",
            &JsValue::from_str(WEBGPU_POWER_PREFERENCE),
        )]);
        let promise: js_sys::Promise = method.call1(gpu, &options).ok()?.unchecked_into();
        let adapter: JsValue = JsFuture::from(promise).await.ok()?;
        if adapter.is_undefined() || adapter.is_null() {
            return None;
        }
        Some(adapter)
    }

    /// Requests a device from an adapter.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    async fn request_device(adapter: &JsValue) -> Option<JsValue> {
        let method: Function = gpu_method(adapter, WEBGPU_METHOD_REQUEST_DEVICE)?;
        let promise: js_sys::Promise = method.call0(adapter).ok()?.unchecked_into();
        let device: JsValue = JsFuture::from(promise).await.ok()?;
        if device.is_undefined() || device.is_null() {
            return None;
        }
        Some(device)
    }

    /// Asks the canvas for a WebGPU context.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - Shared reference to a `HtmlCanvasElement`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn canvas_context(canvas: &HtmlCanvasElement) -> Option<JsValue> {
        let context: Object = canvas.get_context(WEBGPU_CONTEXT_ID).ok()??;
        Some(context.into())
    }

    /// Asks the `GPU` object for the preferred swap-chain format.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - `Some(...)` on success, `None` otherwise.
    fn preferred_format(gpu: &JsValue) -> Option<String> {
        let method: Function = gpu_method(gpu, WEBGPU_METHOD_GET_PREFERRED_CANVAS_FORMAT)?;
        let format: JsValue = method.call0(gpu).ok()?;
        format.as_string()
    }

    /// Configures the canvas context for the given device and format.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<()>` - `Some(...)` on success, `None` otherwise.
    fn configure_context(context: &JsValue, device: &JsValue, format: &str) -> Option<()> {
        let method: Function = gpu_method(context, WEBGPU_METHOD_CONFIGURE)?;
        let descriptor: JsValue = gpu_object(&[
            ("device", device),
            (WEBGPU_KEY_FORMAT, &JsValue::from_str(format)),
            ("alphaMode", &JsValue::from_str(WEBGPU_ALPHA_MODE)),
        ]);
        method.call1(context, &descriptor).ok()?;
        Some(())
    }

    /// Creates a shader module.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_shader_module(device: &JsValue, code: &str) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_SHADER_MODULE)?;
        let descriptor = gpu_object(&[("code", &JsValue::from_str(code))]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates a buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_buffer(device: &JsValue, size: u32, usage: u32) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_BUFFER)?;
        let descriptor: JsValue = gpu_object(&[
            (WEBGPU_KEY_SIZE, &JsValue::from_f64(f64::from(size))),
            (WEBGPU_KEY_USAGE, &JsValue::from_f64(f64::from(usage))),
        ]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates a texture.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_texture(
        device: &JsValue,
        width: u32,
        height: u32,
        format: &str,
        usage: u32,
    ) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_TEXTURE)?;
        let descriptor: JsValue = gpu_object(&[
            (
                WEBGPU_KEY_SIZE,
                &gpu_object(&[
                    (WEBGPU_KEY_WIDTH, &JsValue::from_f64(f64::from(width))),
                    (WEBGPU_KEY_HEIGHT, &JsValue::from_f64(f64::from(height))),
                    (WEBGPU_KEY_DEPTH_OR_ARRAY_LAYERS, &JsValue::from_f64(1.0)),
                ]),
            ),
            (WEBGPU_KEY_FORMAT, &JsValue::from_str(format)),
            (WEBGPU_KEY_USAGE, &JsValue::from_f64(f64::from(usage))),
        ]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates a view of a texture.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_view(texture: &JsValue) -> Option<JsValue> {
        gpu_method(texture, WEBGPU_METHOD_CREATE_VIEW)?
            .call0(texture)
            .ok()
    }

    /// Destroys a GPU resource.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    fn destroy(resource: &JsValue) {
        if let Some(method) = gpu_method(resource, WEBGPU_METHOD_DESTROY) {
            let _: Result<JsValue, JsValue> = method.call0(resource);
        }
    }

    /// Creates a linear sampler with clamped addressing.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_sampler(device: &JsValue) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_SAMPLER)?;
        let descriptor: JsValue = gpu_object(&[
            ("magFilter", &JsValue::from_str(WEBGPU_FILTER_LINEAR)),
            ("minFilter", &JsValue::from_str(WEBGPU_FILTER_LINEAR)),
            (
                "addressModeU",
                &JsValue::from_str(WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE),
            ),
            (
                "addressModeV",
                &JsValue::from_str(WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE),
            ),
        ]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates the bind group layout that holds the clip-space transform.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_uniform_layout(device: &JsValue) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_BIND_GROUP_LAYOUT)?;
        let entries: JsValue = gpu_array(&[gpu_object(&[
            (WEBGPU_KEY_BINDING, &JsValue::from_f64(0.0)),
            (
                WEBGPU_KEY_VISIBILITY,
                &JsValue::from_f64(f64::from(WEBGPU_SHADER_STAGE_VERTEX)),
            ),
            (
                WEBGPU_KEY_BUFFER,
                &gpu_object(&[(WEBGPU_KEY_TYPE, &JsValue::from_str("uniform"))]),
            ),
        ])]);
        let descriptor: JsValue = gpu_object(&[(WEBGPU_KEY_ENTRIES, &entries)]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates the bind group layout that holds the texture and the sampler.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_texture_layout(device: &JsValue) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_BIND_GROUP_LAYOUT)?;
        let entries: JsValue = gpu_array(&[
            gpu_object(&[
                (WEBGPU_KEY_BINDING, &JsValue::from_f64(0.0)),
                (
                    WEBGPU_KEY_VISIBILITY,
                    &JsValue::from_f64(f64::from(WEBGPU_SHADER_STAGE_FRAGMENT)),
                ),
                (
                    "sampler",
                    &gpu_object(&[(WEBGPU_KEY_TYPE, &JsValue::from_str("filtering"))]),
                ),
            ]),
            gpu_object(&[
                (WEBGPU_KEY_BINDING, &JsValue::from_f64(1.0)),
                (
                    WEBGPU_KEY_VISIBILITY,
                    &JsValue::from_f64(f64::from(WEBGPU_SHADER_STAGE_FRAGMENT)),
                ),
                (
                    WEBGPU_KEY_TEXTURE,
                    &gpu_object(&[("sampleType", &JsValue::from_str("float"))]),
                ),
            ]),
        ]);
        let descriptor: JsValue = gpu_object(&[(WEBGPU_KEY_ENTRIES, &entries)]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates a pipeline layout out of bind group layouts.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&[JsValue]` - Shared reference to a `[JsValue]`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_pipeline_layout(device: &JsValue, layouts: &[JsValue]) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_PIPELINE_LAYOUT)?;
        let descriptor = gpu_object(&[("bindGroupLayouts", &gpu_array(layouts))]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates a bind group.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&[JsValue]` - Shared reference to a `[JsValue]`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_bind_group(
        device: &JsValue,
        layout: &JsValue,
        entries: &[JsValue],
    ) -> Option<JsValue> {
        let method: Function = gpu_method(device, WEBGPU_METHOD_CREATE_BIND_GROUP)?;
        let descriptor: JsValue = gpu_object(&[
            (WEBGPU_KEY_LAYOUT, layout),
            (WEBGPU_KEY_ENTRIES, &gpu_array(entries)),
        ]);
        method.call1(device, &descriptor).ok()
    }

    /// Creates a render pipeline.
    ///
    /// The two pipelines differ only in their vertex layout: the solid one
    /// reads a position and a colour, the textured one reads a position,
    /// a texture coordinate, and a colour.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&str` - Shared reference to a `str`.
    /// - `bool` - A boolean (`bool`).
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn create_render_pipeline(
        device: &JsValue,
        layout: &JsValue,
        shader: &JsValue,
        format: &str,
        textured: bool,
    ) -> Option<JsValue> {
        let attributes: JsValue = if textured {
            gpu_array(&[
                gpu_object(&[
                    (WEBGPU_KEY_SHADER_LOCATION, &JsValue::from_f64(0.0)),
                    (WEBGPU_KEY_OFFSET, &JsValue::from_f64(0.0)),
                    (
                        WEBGPU_KEY_FORMAT,
                        &JsValue::from_str(WEBGPU_VERTEX_FORMAT_FLOAT32X2),
                    ),
                ]),
                gpu_object(&[
                    (WEBGPU_KEY_SHADER_LOCATION, &JsValue::from_f64(1.0)),
                    (WEBGPU_KEY_OFFSET, &JsValue::from_f64(8.0)),
                    (
                        WEBGPU_KEY_FORMAT,
                        &JsValue::from_str(WEBGPU_VERTEX_FORMAT_FLOAT32X2),
                    ),
                ]),
                gpu_object(&[
                    (WEBGPU_KEY_SHADER_LOCATION, &JsValue::from_f64(2.0)),
                    (WEBGPU_KEY_OFFSET, &JsValue::from_f64(16.0)),
                    (
                        WEBGPU_KEY_FORMAT,
                        &JsValue::from_str(WEBGPU_VERTEX_FORMAT_UNORM8X4),
                    ),
                ]),
            ])
        } else {
            gpu_array(&[
                gpu_object(&[
                    (WEBGPU_KEY_SHADER_LOCATION, &JsValue::from_f64(0.0)),
                    (WEBGPU_KEY_OFFSET, &JsValue::from_f64(0.0)),
                    (
                        WEBGPU_KEY_FORMAT,
                        &JsValue::from_str(WEBGPU_VERTEX_FORMAT_FLOAT32X2),
                    ),
                ]),
                gpu_object(&[
                    (WEBGPU_KEY_SHADER_LOCATION, &JsValue::from_f64(1.0)),
                    (WEBGPU_KEY_OFFSET, &JsValue::from_f64(8.0)),
                    (
                        WEBGPU_KEY_FORMAT,
                        &JsValue::from_str(WEBGPU_VERTEX_FORMAT_UNORM8X4),
                    ),
                ]),
            ])
        };
        let stride: u32 = if textured {
            WEBGPU_TEXTURED_VERTEX_STRIDE
        } else {
            WEBGPU_SOLID_VERTEX_STRIDE
        };
        let vertex_buffer: JsValue = gpu_object(&[
            ("arrayStride", &JsValue::from_f64(f64::from(stride))),
            ("stepMode", &JsValue::from_str(WEBGPU_STEP_MODE_VERTEX)),
            ("attributes", &attributes),
        ]);
        let target: JsValue = gpu_object(&[
            (WEBGPU_KEY_FORMAT, &JsValue::from_str(format)),
            (
                "blend",
                &gpu_object(&[
                    (
                        "color",
                        &gpu_object(&[
                            (
                                WEBGPU_KEY_SRC_FACTOR,
                                &JsValue::from_str(WEBGPU_BLEND_FACTOR_ONE),
                            ),
                            (
                                WEBGPU_KEY_DST_FACTOR,
                                &JsValue::from_str(WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA),
                            ),
                            (
                                WEBGPU_KEY_OPERATION,
                                &JsValue::from_str(WEBGPU_BLEND_OPERATION_ADD),
                            ),
                        ]),
                    ),
                    (
                        "alpha",
                        &gpu_object(&[
                            (
                                WEBGPU_KEY_SRC_FACTOR,
                                &JsValue::from_str(WEBGPU_BLEND_FACTOR_ONE),
                            ),
                            (
                                WEBGPU_KEY_DST_FACTOR,
                                &JsValue::from_str(WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA),
                            ),
                            (
                                WEBGPU_KEY_OPERATION,
                                &JsValue::from_str(WEBGPU_BLEND_OPERATION_ADD),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]);
        let descriptor: JsValue = gpu_object(&[
            (WEBGPU_KEY_LAYOUT, layout),
            (
                WEBGPU_KEY_VERTEX,
                &gpu_object(&[
                    (WEBGPU_KEY_MODULE, shader),
                    (
                        WEBGPU_KEY_ENTRY_POINT,
                        &JsValue::from_str(WEBGPU_VERTEX_ENTRY_POINT),
                    ),
                    ("buffers", &gpu_array(&[vertex_buffer])),
                ]),
            ),
            (
                "fragment",
                &gpu_object(&[
                    (WEBGPU_KEY_MODULE, shader),
                    (
                        WEBGPU_KEY_ENTRY_POINT,
                        &JsValue::from_str(WEBGPU_FRAGMENT_ENTRY_POINT),
                    ),
                    ("targets", &gpu_array(&[target])),
                ]),
            ),
            (
                "primitive",
                &gpu_object(&[
                    ("topology", &JsValue::from_str("triangle-list")),
                    ("cullMode", &JsValue::from_str("none")),
                ]),
            ),
            (
                "multisample",
                &gpu_object(&[("count", &JsValue::from_f64(1.0))]),
            ),
        ]);
        gpu_method(device, WEBGPU_METHOD_CREATE_RENDER_PIPELINE)?
            .call1(device, &descriptor)
            .ok()
    }

    /// Creates the render pass descriptor and the colour attachment inside
    /// it.
    ///
    /// The descriptor is built once and reused: only the view of the
    /// current swap-chain texture changes from frame to frame.
    ///
    /// # Returns
    ///
    /// - `(JsValue, JsValue)` - A `(JsValue, JsValue)` value.
    fn create_pass_descriptor() -> (JsValue, JsValue) {
        let attachment: JsValue = gpu_object(&[
            ("view", &JsValue::UNDEFINED),
            (
                "clearValue",
                &gpu_object(&[
                    ("r", &JsValue::from_f64(0.0)),
                    ("g", &JsValue::from_f64(0.0)),
                    ("b", &JsValue::from_f64(0.0)),
                    ("a", &JsValue::from_f64(0.0)),
                ]),
            ),
            ("loadOp", &JsValue::from_str("clear")),
            ("storeOp", &JsValue::from_str("store")),
        ]);
        let descriptor: JsValue =
            gpu_object(&[("colorAttachments", &gpu_array(from_ref(&attachment)))]);
        (descriptor, attachment)
    }

    /// Writes the clip-space transform into the uniform buffer.
    ///
    /// The chart works in CSS pixels with the origin in the top-left
    /// corner; clip space wants the origin in the middle and the vertical
    /// axis pointing up. The transform therefore scales the coordinates
    /// and flips the vertical axis. The device pixel ratio cancels out:
    /// the canvas backing store is `dpr` times the CSS size, so the two
    /// factors meet and only the CSS size matters.
    fn update_uniform(&mut self) {
        let width: f64 = if self.get_width() > 0.0 {
            self.get_width()
        } else {
            1.0
        };
        let height: f64 = if self.get_height() > 0.0 {
            self.get_height()
        } else {
            1.0
        };
        let data: [f32; 4] = [
            (2.0 / width) as f32,
            (-2.0 / height) as f32,
            -1.0_f32,
            1.0_f32,
        ];
        let view: Float32Array = js_sys::Float32Array::from(data.as_slice());
        let _: Result<JsValue, JsValue> = self.get_api().write_buffer.call3(
            self.get_queue(),
            self.get_uniform_buffer(),
            &JsValue::from_f64(0.0),
            &view,
        );
    }

    /// Creates the glyph atlas texture, or re-creates it when the atlas
    /// changed size.
    ///
    /// # Arguments
    ///
    /// - `&GlyphAtlas` - Shared reference to a `GlyphAtlas`.
    fn sync_atlas(&mut self, atlas: &GlyphAtlas) {
        let width: u32 = atlas.width();
        let height: u32 = atlas.height();
        if self.get_atlas().texture.is_some()
            && self.get_atlas().width == width
            && self.get_atlas().height == height
        {
            return;
        }
        if let Some(texture) = &self.get_atlas().texture {
            Self::destroy(texture);
        }
        let usage: u32 = WEBGPU_TEXTURE_USAGE_COPY_DST
            | WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING
            | WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT;
        let Some(texture) = Self::create_texture(
            self.get_device(),
            width,
            height,
            WEBGPU_TEXTURE_FORMAT,
            usage,
        ) else {
            return;
        };
        let Some(view) = Self::create_view(&texture) else {
            return;
        };
        let Some(bind_group) = Self::create_bind_group(
            self.get_device(),
            self.get_texture_layout(),
            &[
                gpu_object(&[
                    (WEBGPU_KEY_BINDING, &JsValue::from_f64(0.0)),
                    (WEBGPU_KEY_RESOURCE, self.get_sampler()),
                ]),
                gpu_object(&[
                    (WEBGPU_KEY_BINDING, &JsValue::from_f64(1.0)),
                    (WEBGPU_KEY_RESOURCE, &view),
                ]),
            ],
        ) else {
            return;
        };
        self.get_mut_atlas().texture = Some(texture);
        self.get_mut_atlas().bind_group = Some(bind_group);
        self.get_mut_atlas().width = width;
        self.get_mut_atlas().height = height;
        // The bind group changed, so the batches have to be built again.
        self.set_fingerprint(None);
    }

    /// Uploads the glyph atlas into its texture.
    ///
    /// # Arguments
    ///
    /// - `&GlyphAtlas` - Shared reference to a `GlyphAtlas`.
    fn upload_atlas(&mut self, atlas: &GlyphAtlas) {
        let Some(texture) = self.get_atlas().texture.clone() else {
            return;
        };
        let canvas: &HtmlCanvasElement = atlas.canvas();
        let source: JsValue = gpu_object(&[
            (WEBGPU_KEY_SOURCE, canvas.as_ref()),
            (WEBGPU_KEY_FLIP_Y, &JsValue::FALSE),
        ]);
        let destination: JsValue = gpu_object(&[
            (WEBGPU_KEY_TEXTURE, &texture),
            (WEBGPU_KEY_PREMULTIPLIED_ALPHA, &JsValue::TRUE),
        ]);
        let size: JsValue = gpu_object(&[
            (
                WEBGPU_KEY_WIDTH,
                &JsValue::from_f64(f64::from(atlas.width())),
            ),
            (
                WEBGPU_KEY_HEIGHT,
                &JsValue::from_f64(f64::from(atlas.height())),
            ),
            (WEBGPU_KEY_DEPTH_OR_ARRAY_LAYERS, &JsValue::from_f64(1.0)),
        ]);
        let _: Result<JsValue, JsValue> = self.get_api().copy_external_image.call3(
            self.get_queue(),
            &source,
            &destination,
            &size,
        );
    }

    /// Computes the fingerprint of a display list.
    ///
    /// The fingerprint covers the shape of every item and the geometry
    /// inside it, so a list that changed in any visible way produces a
    /// different fingerprint. It is a pure Rust computation: it never
    /// crosses into JavaScript, not even once per frame.
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    ///
    /// # Returns
    ///
    /// - `u64` - A `u64` value.
    fn list_fingerprint(&mut self, list: &DisplayList) -> u64 {
        let mut hasher: GpuHasher = GpuHasher::new();
        list.len().hash(&mut hasher);
        for item in list.items() {
            item.kind().hash(&mut hasher);
            item.z.hash(&mut hasher);
            item.element_id.hash(&mut hasher);
            item.series_index.hash(&mut hasher);
            item.data_index.hash(&mut hasher);
            match &item.cmd {
                DrawCmd::Fill {
                    path,
                    paint,
                    opacity,
                } => {
                    hash_path(&mut hasher, path);
                    hash_fill(&mut hasher, paint);
                    hasher.write_f64(*opacity);
                }
                DrawCmd::Stroke {
                    path,
                    paint,
                    style,
                    opacity,
                } => {
                    hash_path(&mut hasher, path);
                    hash_stroke(&mut hasher, paint);
                    hash_line_style(&mut hasher, style);
                    hasher.write_f64(*opacity);
                }
                DrawCmd::Strip {
                    points,
                    baseline,
                    paint,
                    opacity,
                } => {
                    hasher.write_u32(points.len() as u32);
                    for point in points {
                        hasher.write_f32(point.x);
                        hasher.write_f32(point.y);
                    }
                    hasher.write_f32(*baseline);
                    hash_fill(&mut hasher, paint);
                    hasher.write_f64(*opacity);
                }
                DrawCmd::Text { text, x, y, style } => {
                    text.hash(&mut hasher);
                    hasher.write_f64(*x);
                    hasher.write_f64(*y);
                    hash_text_style(&mut hasher, style);
                }
                DrawCmd::Image {
                    image,
                    x,
                    y,
                    width,
                    height,
                } => {
                    let id: u32 = self.image_id(image.as_ref());
                    id.hash(&mut hasher);
                    hasher.write_f64(*x);
                    hasher.write_f64(*y);
                    hasher.write_f64(*width);
                    hasher.write_f64(*height);
                }
                DrawCmd::Clip { path } => hash_path(&mut hasher, path),
                DrawCmd::PopClip => {}
            }
        }
        hasher.finish()
    }

    /// Rebuilds the geometry of the display list.
    ///
    /// The rebuild is the expensive part of a frame: it tessellates every
    /// path and shapes every text run. It only happens when the display
    /// list actually changed, which is what keeps a steady-state frame
    /// cheap.
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn rebuild(&mut self, list: &DisplayList, shaper: &mut TextShaper) {
        self.get_mut_solid_vertices().clear();
        self.get_mut_solid_indices().clear();
        self.get_mut_textured_vertices().clear();
        self.get_mut_textured_indices().clear();
        self.get_mut_batches().clear();
        self.get_mut_clips().clear();

        for item in list.items() {
            match &item.cmd {
                DrawCmd::Fill {
                    path,
                    paint,
                    opacity,
                } => self.push_fill(path, paint, *opacity),
                DrawCmd::Stroke {
                    path,
                    paint,
                    style,
                    opacity,
                } => self.push_stroke(path, paint, style, *opacity),
                DrawCmd::Strip {
                    points,
                    baseline,
                    paint,
                    opacity,
                } => self.push_strip(points, *baseline, paint, *opacity),
                DrawCmd::Text { text, x, y, style } => {
                    self.push_text(text, *x, *y, style, shaper);
                }
                DrawCmd::Image {
                    image,
                    x,
                    y,
                    width,
                    height,
                } => self.push_image(image, *x, *y, *width, *height),
                DrawCmd::Clip { path } => self.push_clip(path),
                DrawCmd::PopClip => self.pop_clip(),
            }
        }

        self.upload_buffers();
    }

    /// Adds a filled path to the solid batch.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `&Fill` - Shared reference to a `Fill`.
    /// - `f64` - A 64-bit float (`f64`).
    fn push_fill(&mut self, path: &Path, paint: &Fill, opacity: f64) {
        if matches!(paint, Fill::None) {
            return;
        }
        let mesh: SolidMesh = self.get_mut_tessellator().fill(path, 0xffff_ffff);
        if mesh.indices.is_empty() {
            return;
        }
        let base: u32 =
            (self.get_solid_vertices().len() / WEBGPU_SOLID_VERTEX_STRIDE as usize) as u32;
        let first_index: u32 = self.get_solid_indices().len() as u32;
        self.open_batch(WebGpuBatchKind::Solid, first_index, None);
        for vertex in &mesh.vertices {
            let color: Color = self.resolve_fill_color(paint, vertex.x, vertex.y);
            let alpha: u8 = scaled_alpha(color, opacity);
            push_solid_vertex(
                self.get_mut_solid_vertices(),
                vertex.x,
                vertex.y,
                color,
                alpha,
            );
        }
        for index in &mesh.indices {
            self.get_mut_solid_indices().push(base + index);
        }
        self.close_batch(mesh.indices.len() as u32);
    }

    /// Adds a stroked path to the solid batch.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `&Stroke` - Shared reference to a `Stroke`.
    /// - `&LineStyle` - Shared reference to a `LineStyle`.
    /// - `f64` - A 64-bit float (`f64`).
    fn push_stroke(&mut self, path: &Path, paint: &Stroke, style: &LineStyle, opacity: f64) {
        if matches!(paint, Stroke::None) || style.width <= 0.0 {
            return;
        }
        let stroke_style: StrokeStyle = StrokeStyle {
            width: style.width as f32,
            cap: style.cap,
            join: style.join,
            dash: style.dash.clone(),
        };
        let mesh: SolidMesh = self
            .get_mut_tessellator()
            .stroke(path, &stroke_style, 0xffff_ffff);
        if mesh.indices.is_empty() {
            return;
        }
        let base: u32 =
            (self.get_solid_vertices().len() / WEBGPU_SOLID_VERTEX_STRIDE as usize) as u32;
        let first_index: u32 = self.get_solid_indices().len() as u32;
        self.open_batch(WebGpuBatchKind::Solid, first_index, None);
        for vertex in &mesh.vertices {
            let color: Color = self.resolve_stroke_color(paint, vertex.x, vertex.y);
            let alpha: u8 = scaled_alpha(color, opacity * style.opacity);
            push_solid_vertex(
                self.get_mut_solid_vertices(),
                vertex.x,
                vertex.y,
                color,
                alpha,
            );
        }
        for index in &mesh.indices {
            self.get_mut_solid_indices().push(base + index);
        }
        self.close_batch(mesh.indices.len() as u32);
    }

    /// Adds an area strip to the solid batch.
    ///
    /// The strip is already a run of triangles, so it is not triangulated
    /// again: it goes straight into the vertex buffer.
    ///
    /// # Arguments
    ///
    /// - `&[Point]` - Shared reference to a `[Point]`.
    /// - `f32` - A 32-bit float (`f32`).
    /// - `&Fill` - Shared reference to a `Fill`.
    /// - `f64` - A 64-bit float (`f64`).
    fn push_strip(&mut self, points: &[Point], baseline: f32, paint: &Fill, opacity: f64) {
        if matches!(paint, Fill::None) || points.len() < 2 {
            return;
        }
        let mesh: SolidMesh = self
            .get_mut_tessellator()
            .fill_strip(points, baseline, 0xffff_ffff);
        if mesh.indices.is_empty() {
            return;
        }
        let base: u32 =
            (self.get_solid_vertices().len() / WEBGPU_SOLID_VERTEX_STRIDE as usize) as u32;
        let first_index: u32 = self.get_solid_indices().len() as u32;
        self.open_batch(WebGpuBatchKind::Solid, first_index, None);
        for vertex in &mesh.vertices {
            let color: Color = self.resolve_fill_color(paint, vertex.x, vertex.y);
            let alpha: u8 = scaled_alpha(color, opacity);
            push_solid_vertex(
                self.get_mut_solid_vertices(),
                vertex.x,
                vertex.y,
                color,
                alpha,
            );
        }
        for index in &mesh.indices {
            self.get_mut_solid_indices().push(base + index);
        }
        self.close_batch(mesh.indices.len() as u32);
    }

    /// Returns the colour of a fill at the given position.
    ///
    /// # Arguments
    ///
    /// - `&Fill` - Shared reference to a `Fill`.
    /// - `f32` - A 32-bit float (`f32`).
    ///
    /// # Returns
    ///
    /// - `Color` - A `Color` value.
    fn resolve_fill_color(&self, paint: &Fill, x: f32, y: f32) -> Color {
        match paint {
            Fill::Solid(color) => *color,
            Fill::Linear(gradient) => linear_gradient_color(gradient, f64::from(x), f64::from(y)),
            Fill::Radial(gradient) => radial_gradient_color(gradient, f64::from(x), f64::from(y)),
            Fill::None => Color::default(),
        }
    }

    /// Returns the colour of a stroke at the given position.
    ///
    /// # Arguments
    ///
    /// - `&Stroke` - Shared reference to a `Stroke`.
    /// - `f32` - A 32-bit float (`f32`).
    ///
    /// # Returns
    ///
    /// - `Color` - A `Color` value.
    fn resolve_stroke_color(&self, paint: &Stroke, x: f32, y: f32) -> Color {
        match paint {
            Stroke::Solid(color) => *color,
            Stroke::Linear(gradient) => linear_gradient_color(gradient, f64::from(x), f64::from(y)),
            Stroke::Radial(gradient) => radial_gradient_color(gradient, f64::from(x), f64::from(y)),
            Stroke::None => Color::default(),
        }
    }

    /// Adds a text run to the batches.
    ///
    /// The label background goes into the solid batch and the glyphs into
    /// the textured one, in that order, so the background ends up behind
    /// the text exactly as it does in Canvas2D.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `&TextStyle` - Shared reference to a `TextStyle`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn push_text(
        &mut self,
        text: &str,
        x: f64,
        y: f64,
        style: &TextStyle,
        shaper: &mut TextShaper,
    ) {
        let run: TextRun = shaper.shape(text, style, x, y);
        if let Some(background) = style.background {
            for line in &run.lines {
                let left: f64 = line.left - style.padding.3;
                let top: f64 = line.baseline - run.ascent - style.padding.0;
                let width: f64 = line.width + style.padding.1 + style.padding.3;
                let height: f64 = run.ascent + run.descent + style.padding.0 + style.padding.2;
                self.push_solid_quad(
                    left as f32,
                    top as f32,
                    width as f32,
                    height as f32,
                    background,
                    1.0,
                );
            }
        }
        if run.quads.is_empty() {
            return;
        }
        let bind_group: Option<JsValue> = self.get_atlas().bind_group.clone();
        let base: u32 =
            (self.get_textured_vertices().len() / WEBGPU_TEXTURED_VERTEX_STRIDE as usize) as u32;
        let first_index: u32 = self.get_textured_indices().len() as u32;
        self.open_batch(WebGpuBatchKind::Textured, first_index, bind_group);
        let color: Color = style.color;
        let alpha: u8 = style.color.alpha;
        for quad in &run.quads {
            push_textured_vertex(
                self.get_mut_textured_vertices(),
                quad.x,
                quad.y,
                quad.u,
                quad.v,
                color,
                alpha,
            );
            push_textured_vertex(
                self.get_mut_textured_vertices(),
                quad.x + quad.width,
                quad.y,
                quad.u + quad.u_size,
                quad.v,
                color,
                alpha,
            );
            push_textured_vertex(
                self.get_mut_textured_vertices(),
                quad.x + quad.width,
                quad.y + quad.height,
                quad.u + quad.u_size,
                quad.v + quad.v_size,
                color,
                alpha,
            );
            push_textured_vertex(
                self.get_mut_textured_vertices(),
                quad.x,
                quad.y + quad.height,
                quad.u,
                quad.v + quad.v_size,
                color,
                alpha,
            );
        }
        for quad in 0..run.quads.len() as u32 {
            let vertex: u32 = base + quad * 4;
            self.get_mut_textured_indices().extend_from_slice(&[
                vertex,
                vertex + 1,
                vertex + 2,
                vertex,
                vertex + 2,
                vertex + 3,
            ]);
        }
        self.close_batch((run.quads.len() * 6) as u32);
    }

    /// Adds an image to the textured batch.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
    /// - `f64` - A 64-bit float (`f64`).
    fn push_image(&mut self, image: &HtmlImageElement, x: f64, y: f64, width: f64, height: f64) {
        if !image.complete() {
            return;
        }
        let natural_width: u32 = image.natural_width();
        let natural_height: u32 = image.natural_height();
        if natural_width == 0 || natural_height == 0 {
            return;
        }
        let source: String = image.src();
        let id: u32 = self.image_id(image.as_ref());
        let Some(bind_group) =
            self.prepare_image(id, image, &source, natural_width, natural_height)
        else {
            return;
        };
        let base: u32 =
            (self.get_textured_vertices().len() / WEBGPU_TEXTURED_VERTEX_STRIDE as usize) as u32;
        let first_index: u32 = self.get_textured_indices().len() as u32;
        self.open_batch(WebGpuBatchKind::Textured, first_index, Some(bind_group));
        let color: Color = Color::rgb(255, 255, 255);
        push_textured_vertex(
            self.get_mut_textured_vertices(),
            x as f32,
            y as f32,
            0.0,
            0.0,
            color,
            255,
        );
        push_textured_vertex(
            self.get_mut_textured_vertices(),
            (x + width) as f32,
            y as f32,
            1.0,
            0.0,
            color,
            255,
        );
        push_textured_vertex(
            self.get_mut_textured_vertices(),
            (x + width) as f32,
            (y + height) as f32,
            1.0,
            1.0,
            color,
            255,
        );
        push_textured_vertex(
            self.get_mut_textured_vertices(),
            x as f32,
            (y + height) as f32,
            0.0,
            1.0,
            color,
            255,
        );
        self.get_mut_textured_indices().extend_from_slice(&[
            base,
            base + 1,
            base + 2,
            base,
            base + 2,
            base + 3,
        ]);
        self.close_batch(6);
    }

    /// Adds a flat-coloured rectangle to the solid batch.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    /// - `Color` - A `Color` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    fn push_solid_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        opacity: f64,
    ) {
        let base: u32 =
            (self.get_solid_vertices().len() / WEBGPU_SOLID_VERTEX_STRIDE as usize) as u32;
        let first_index: u32 = self.get_solid_indices().len() as u32;
        self.open_batch(WebGpuBatchKind::Solid, first_index, None);
        let alpha: u8 = scaled_alpha(color, opacity);
        push_solid_vertex(self.get_mut_solid_vertices(), x, y, color, alpha);
        push_solid_vertex(self.get_mut_solid_vertices(), x + width, y, color, alpha);
        push_solid_vertex(
            self.get_mut_solid_vertices(),
            x + width,
            y + height,
            color,
            alpha,
        );
        push_solid_vertex(self.get_mut_solid_vertices(), x, y + height, color, alpha);
        self.get_mut_solid_indices().extend_from_slice(&[
            base,
            base + 1,
            base + 2,
            base,
            base + 2,
            base + 3,
        ]);
        self.close_batch(6);
    }

    /// Pushes a clip rectangle.
    ///
    /// The clip is approximated by the bounding box of the path, which is
    /// exact for the rectangular clips a chart uses to keep a series
    /// inside the grid.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    fn push_clip(&mut self, path: &Path) {
        let bounds: BBox = path.bounds();
        let scissor: WebGpuScissor = WebGpuScissor {
            x: (bounds.min_x * self.get_dpr()).floor().max(0.0) as u32,
            y: (bounds.min_y * self.get_dpr()).floor().max(0.0) as u32,
            width: ((bounds.max_x - bounds.min_x) * self.get_dpr())
                .ceil()
                .max(0.0) as u32,
            height: ((bounds.max_y - bounds.min_y) * self.get_dpr())
                .ceil()
                .max(0.0) as u32,
        };
        self.get_mut_clips().push(scissor);
    }

    /// Pops a clip rectangle.
    fn pop_clip(&mut self) {
        self.get_mut_clips().pop();
    }

    /// Returns the scissor rectangle in effect, if any.
    ///
    /// Nested clips intersect, exactly as they do in Canvas2D.
    ///
    /// # Returns
    ///
    /// - `Option<WebGpuScissor>` - `Some(...)` on success, `None` otherwise.
    fn current_scissor(&self) -> Option<WebGpuScissor> {
        let mut result: Option<WebGpuScissor> = None;
        for clip in self.get_clips() {
            result = Some(match result {
                None => *clip,
                Some(current) => {
                    let left: u32 = current.x.max(clip.x);
                    let top: u32 = current.y.max(clip.y);
                    let right: u32 = (current.x + current.width).min(clip.x + clip.width);
                    let bottom: u32 = (current.y + current.height).min(clip.y + clip.height);
                    WebGpuScissor {
                        x: left,
                        y: top,
                        width: right.saturating_sub(left),
                        height: bottom.saturating_sub(top),
                    }
                }
            });
        }
        result
    }

    /// Opens a batch, or extends the previous one when it is compatible.
    ///
    /// Consecutive shapes of the same kind that share a bind group end up
    /// in one batch, and therefore in one draw call. The paint order is
    /// preserved either way, because the batches are drawn in the order
    /// they were opened.
    ///
    /// # Arguments
    ///
    /// - `WebGpuBatchKind` - A `WebGpuBatchKind` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `Option<JsValue>` - A `Option<JsValue>` parameter.
    fn open_batch(&mut self, kind: WebGpuBatchKind, first_index: u32, bind_group: Option<JsValue>) {
        let compatible: bool = match self.get_batches().last() {
            Some(last) => last.kind == kind && last.bind_group == bind_group,
            None => false,
        };
        if compatible {
            return;
        }
        let scissor: Option<WebGpuScissor> = self.current_scissor();
        self.get_mut_batches().push(WebGpuBatch {
            kind,
            first_index,
            index_count: 0,
            bind_group,
            scissor,
        });
    }

    /// Closes the batch that is currently open.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    fn close_batch(&mut self, index_count: u32) {
        if let Some(last) = self.get_mut_batches().last_mut() {
            last.index_count += index_count;
        }
    }

    /// Returns the identifier of an image element, assigning one on first
    /// sight.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    fn image_id(&mut self, element: &JsValue) -> u32 {
        for image in self.get_images() {
            if image.element == *element {
                return image.id;
            }
        }
        let id: u32 = self.get_next_image_id();
        self.set_next_image_id(self.get_next_image_id() + 1);
        self.get_mut_images().push(WebGpuImage {
            element: element.clone(),
            id,
            texture: None,
            bind_group: None,
            width: 0,
            height: 0,
            source: String::new(),
        });
        id
    }

    /// Makes sure the image has a texture, and returns its bind group.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn prepare_image(
        &mut self,
        id: u32,
        image: &HtmlImageElement,
        source: &str,
        width: u32,
        height: u32,
    ) -> Option<JsValue> {
        let position: usize = self
            .get_images()
            .iter()
            .position(|entry: &WebGpuImage| entry.id == id)?;
        let stale: bool = {
            let entry: &_ = &self.get_images()[position];
            entry.width != width || entry.height != height || entry.source != source
        };
        if stale {
            let usage: u32 = WEBGPU_TEXTURE_USAGE_COPY_DST
                | WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING
                | WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT;
            let texture: JsValue = Self::create_texture(
                self.get_device(),
                width,
                height,
                WEBGPU_TEXTURE_FORMAT,
                usage,
            )?;
            let view: JsValue = Self::create_view(&texture)?;
            let bind_group: JsValue = Self::create_bind_group(
                self.get_device(),
                self.get_texture_layout(),
                &[
                    gpu_object(&[
                        (WEBGPU_KEY_BINDING, &JsValue::from_f64(0.0)),
                        (WEBGPU_KEY_RESOURCE, self.get_sampler()),
                    ]),
                    gpu_object(&[
                        (WEBGPU_KEY_BINDING, &JsValue::from_f64(1.0)),
                        (WEBGPU_KEY_RESOURCE, &view),
                    ]),
                ],
            )?;
            let images: &mut Vec<WebGpuImage> = self.get_mut_images();
            let entry: &mut WebGpuImage = &mut images[position];
            entry.texture = Some(texture);
            entry.bind_group = Some(bind_group);
            entry.width = width;
            entry.height = height;
            entry.source = source.to_string();
        }
        let texture: JsValue = self.get_images()[position].texture.clone()?;
        let copy_source: JsValue = gpu_object(&[
            (WEBGPU_KEY_SOURCE, image.as_ref()),
            (WEBGPU_KEY_FLIP_Y, &JsValue::FALSE),
        ]);
        let copy_destination: JsValue = gpu_object(&[
            (WEBGPU_KEY_TEXTURE, &texture),
            (WEBGPU_KEY_PREMULTIPLIED_ALPHA, &JsValue::TRUE),
        ]);
        let size: JsValue = gpu_object(&[
            (WEBGPU_KEY_WIDTH, &JsValue::from_f64(f64::from(width))),
            (WEBGPU_KEY_HEIGHT, &JsValue::from_f64(f64::from(height))),
            (WEBGPU_KEY_DEPTH_OR_ARRAY_LAYERS, &JsValue::from_f64(1.0)),
        ]);
        let _: Result<JsValue, JsValue> = self.get_api().copy_external_image.call3(
            self.get_queue(),
            &copy_source,
            &copy_destination,
            &size,
        );
        self.get_images()[position].bind_group.clone()
    }

    /// Uploads the vertex and index data to the GPU.
    ///
    /// The data is uploaded in one call per buffer, never one call per
    /// vertex, and only when the display list changed.
    fn upload_buffers(&mut self) {
        let device: JsValue = self.get_device().clone();
        let queue: JsValue = self.get_queue().clone();
        let write_buffer: Function = self.get_api().write_buffer.clone();
        let Self {
            solid_buffer,
            solid_index_buffer,
            solid_vertices,
            solid_indices,
            textured_buffer,
            textured_index_buffer,
            textured_vertices,
            textured_indices,
            ..
        } = &mut *self;
        WebGpuBuffer::upload_bytes(
            &device,
            &queue,
            &write_buffer,
            solid_buffer,
            solid_vertices,
            WEBGPU_BUFFER_USAGE_VERTEX | WEBGPU_BUFFER_USAGE_COPY_DST,
        );
        WebGpuBuffer::upload_indices(
            &device,
            &queue,
            &write_buffer,
            solid_index_buffer,
            solid_indices,
            WEBGPU_BUFFER_USAGE_INDEX | WEBGPU_BUFFER_USAGE_COPY_DST,
        );
        WebGpuBuffer::upload_bytes(
            &device,
            &queue,
            &write_buffer,
            textured_buffer,
            textured_vertices,
            WEBGPU_BUFFER_USAGE_VERTEX | WEBGPU_BUFFER_USAGE_COPY_DST,
        );
        WebGpuBuffer::upload_indices(
            &device,
            &queue,
            &write_buffer,
            textured_index_buffer,
            textured_indices,
            WEBGPU_BUFFER_USAGE_INDEX | WEBGPU_BUFFER_USAGE_COPY_DST,
        );
    }

    /// Encodes one frame and submits it to the queue.
    fn encode(&mut self) {
        let Some(texture) = self
            .get_api()
            .get_current_texture
            .call0(self.get_context())
            .ok()
        else {
            return;
        };
        let Some(view) = Self::create_view(&texture) else {
            return;
        };
        gpu_set(self.get_pass_attachment(), WEBGPU_PROPERTY_VIEW, &view);

        let Some(encoder) = self
            .get_api()
            .create_command_encoder
            .call0(self.get_device())
            .ok()
        else {
            return;
        };
        if self.try_get_encoder_api().is_none() {
            self.set_encoder_api(WebGpuEncoderApi::new(&encoder));
        }
        let Some(pass) =
            self.try_get_encoder_api()
                .as_ref()
                .and_then(|encoder_api: &WebGpuEncoderApi| {
                    encoder_api
                        .begin_render_pass
                        .call1(&encoder, self.get_pass_descriptor())
                        .ok()
                })
        else {
            return;
        };
        if self.try_get_pass_api().is_none() {
            self.set_pass_api(WebGpuPassApi::new(&pass));
        }
        let Some(pass_api) = self.try_get_pass_api().as_ref() else {
            return;
        };
        let _: Result<JsValue, JsValue> = pass_api.set_bind_group.call2(
            &pass,
            &JsValue::from_f64(0.0),
            self.get_uniform_bind_group(),
        );
        self.draw_batches(&pass, pass_api);
        let _: Result<JsValue, JsValue> = pass_api.end_pass.call0(&pass);
        let Some(command) =
            self.try_get_encoder_api()
                .as_ref()
                .and_then(|encoder_api: &WebGpuEncoderApi| {
                    encoder_api.finish_encoder.call0(&encoder).ok()
                })
        else {
            return;
        };
        let commands: JsValue = gpu_array(&[command]);
        let _: Result<JsValue, JsValue> = self.get_api().submit.call1(self.get_queue(), &commands);
    }

    /// Draws the batches.
    ///
    /// The pipeline, the vertex buffer, the index buffer, and the bind
    /// group are only rebound when they actually change, so a frame with
    /// a handful of batches costs a handful of JavaScript calls.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&WebGpuPassApi` - Shared reference to a `WebGpuPassApi`.
    fn draw_batches(&self, pass: &JsValue, pass_api: &WebGpuPassApi) {
        let mut current_kind: Option<WebGpuBatchKind> = None;
        let mut current_bind_group: Option<JsValue> = None;
        let mut current_scissor: Option<WebGpuScissor> = None;
        for batch in self.get_batches() {
            if batch.index_count == 0 {
                continue;
            }
            if current_kind != Some(batch.kind) {
                let (pipeline, vertex_buffer, index_buffer) = match batch.kind {
                    WebGpuBatchKind::Solid => (
                        self.get_solid_pipeline(),
                        self.get_solid_buffer(),
                        self.get_solid_index_buffer(),
                    ),
                    WebGpuBatchKind::Textured => (
                        self.get_textured_pipeline(),
                        self.get_textured_buffer(),
                        self.get_textured_index_buffer(),
                    ),
                };
                let _: Result<JsValue, JsValue> = pass_api.set_pipeline.call1(pass, pipeline);
                if let Some(buffer) = &vertex_buffer.buffer {
                    let _: Result<JsValue, JsValue> =
                        pass_api
                            .set_vertex_buffer
                            .call2(pass, &JsValue::from_f64(0.0), buffer);
                }
                if let Some(buffer) = &index_buffer.buffer {
                    let _: Result<JsValue, JsValue> = pass_api.set_index_buffer.call3(
                        pass,
                        buffer,
                        &JsValue::from_str("uint32"),
                        &JsValue::from_f64(0.0),
                    );
                }
                current_kind = Some(batch.kind);
                current_bind_group = None;
            }
            if batch.kind == WebGpuBatchKind::Textured {
                let group: Option<JsValue> = batch
                    .bind_group
                    .clone()
                    .or_else(|| self.get_atlas().bind_group.clone());
                if let Some(group) = group
                    && current_bind_group.as_ref() != Some(&group)
                {
                    let _: Result<JsValue, JsValue> =
                        pass_api
                            .set_bind_group
                            .call2(pass, &JsValue::from_f64(1.0), &group);
                    current_bind_group = Some(group);
                }
            }
            if batch.scissor != current_scissor {
                match &batch.scissor {
                    Some(scissor) => {
                        let _: Result<JsValue, JsValue> = pass_api.set_scissor_rect.call4(
                            pass,
                            &JsValue::from_f64(f64::from(scissor.x)),
                            &JsValue::from_f64(f64::from(scissor.y)),
                            &JsValue::from_f64(f64::from(scissor.width)),
                            &JsValue::from_f64(f64::from(scissor.height)),
                        );
                    }
                    None => {
                        let _: Result<JsValue, JsValue> = pass_api.set_scissor_rect.call4(
                            pass,
                            &JsValue::from_f64(0.0),
                            &JsValue::from_f64(0.0),
                            &JsValue::from_f64(f64::from(self.get_canvas().width())),
                            &JsValue::from_f64(f64::from(self.get_canvas().height())),
                        );
                    }
                }
                current_scissor = batch.scissor;
            }
            let _: Result<JsValue, JsValue> = pass_api.draw_indexed.call5(
                pass,
                &JsValue::from_f64(f64::from(batch.index_count)),
                &JsValue::from_f64(1.0),
                &JsValue::from_f64(f64::from(batch.first_index)),
                &JsValue::from_f64(0.0),
                &JsValue::from_f64(0.0),
            );
        }
    }
}

/// Implements [`ChartBackend`] for [`WebGpuChartBackend`].
impl ChartBackend for WebGpuChartBackend {
    /// Implements the `kind` operation on [`WebGpuChartBackend`].
    ///
    /// # Returns
    ///
    /// - `BackendKind` - A `BackendKind` value.
    fn kind(&self) -> BackendKind {
        BackendKind::WebGpu
    }

    /// Implements the `resize` operation on [`WebGpuChartBackend`].
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn resize(&mut self, width: f64, height: f64, dpr: f64) {
        // The clip rectangles are kept in physical pixels, so a change of
        // the device pixel ratio moves them and the geometry has to be
        // built again.
        if (dpr - self.get_dpr()).abs() > f64::EPSILON {
            self.set_fingerprint(None);
        }
        self.set_dpr(dpr);
        self.set_width(width);
        self.set_height(height);
        let physical_width: u32 = (width * dpr).round().max(1.0) as u32;
        let physical_height: u32 = (height * dpr).round().max(1.0) as u32;
        if self.get_canvas().width() != physical_width {
            self.get_canvas().set_width(physical_width);
        }
        if self.get_canvas().height() != physical_height {
            self.get_canvas().set_height(physical_height);
        }
        self.update_uniform();
    }

    /// Implements the `render` operation on [`WebGpuChartBackend`].
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn render(&mut self, list: &DisplayList, shaper: &mut TextShaper) {
        self.sync_atlas(shaper.atlas());

        // The atlas starts over when it runs out of room, which moves the
        // glyphs to other places in the texture. The text geometry holds
        // the texture coordinates of the glyphs, so it has to be built
        // again when the atlas was reset.
        let glyphs: usize = shaper.atlas().len();
        if glyphs < self.get_atlas_glyphs() {
            self.set_fingerprint(None);
        }
        self.set_atlas_glyphs(glyphs);

        let fingerprint: u64 = self.list_fingerprint(list);
        if *self.try_get_fingerprint() != Some(fingerprint) {
            self.rebuild(list, shaper);
            self.set_fingerprint(Some(fingerprint));
        }

        if shaper.atlas().is_dirty() {
            self.upload_atlas(shaper.atlas());
            shaper.atlas_mut().mark_clean();
        }

        self.encode();
    }

    /// Implements the `dispose` operation on [`WebGpuChartBackend`].
    fn dispose(&mut self) {
        let Self {
            solid_buffer,
            solid_index_buffer,
            textured_buffer,
            textured_index_buffer,
            ..
        } = &mut *self;
        for buffer in [
            solid_buffer,
            solid_index_buffer,
            textured_buffer,
            textured_index_buffer,
        ] {
            if let Some(handle) = &buffer.buffer {
                Self::destroy(handle);
            }
            buffer.buffer = None;
            buffer.capacity = 0;
        }
        if let Some(texture) = &self.get_atlas().texture {
            Self::destroy(texture);
        }
        self.get_mut_atlas().texture = None;
        self.get_mut_atlas().bind_group = None;
        self.get_mut_atlas().width = 0;
        self.get_mut_atlas().height = 0;
        for image in self.get_mut_images().iter_mut() {
            if let Some(texture) = &image.texture {
                Self::destroy(texture);
            }
            image.texture = None;
            image.bind_group = None;
        }
        self.get_mut_batches().clear();
        self.set_fingerprint(None);
    }
}

/// Implements [`Drop`] for [`WebGpuChartBackend`].
impl Drop for WebGpuChartBackend {
    /// Releases the GPU resources when the backend is dropped.
    ///
    /// A chart that is dropped without being disposed would hold on to
    /// its buffers until the browser collects them, and a chart over a
    /// million points holds on to a lot of memory.
    fn drop(&mut self) {
        <Self as ChartBackend>::dispose(self);
    }
}

/// Inherent implementation of [`WebGpuApi`].
impl WebGpuApi {
    /// Fetches the JavaScript methods of the device, the queue, and the
    /// canvas context.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    fn new(device: &JsValue, queue: &JsValue, context: &JsValue) -> Option<Self> {
        Some(Self {
            create_command_encoder: gpu_method(device, WEBGPU_METHOD_CREATE_COMMAND_ENCODER)?,
            submit: gpu_method(queue, WEBGPU_METHOD_SUBMIT)?,
            write_buffer: gpu_method(queue, WEBGPU_METHOD_WRITE_BUFFER)?,
            copy_external_image: gpu_method(queue, WEBGPU_METHOD_COPY_EXTERNAL_IMAGE)?,
            get_current_texture: gpu_method(context, WEBGPU_METHOD_GET_CURRENT_TEXTURE)?,
        })
    }
}

/// Inherent implementation of [`WebGpuEncoderApi`].
impl WebGpuEncoderApi {
    /// Fetches the JavaScript methods of a command encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    fn new(encoder: &JsValue) -> Option<Self> {
        Some(Self {
            begin_render_pass: gpu_method(encoder, WEBGPU_METHOD_BEGIN_RENDER_PASS)?,
            finish_encoder: gpu_method(encoder, WEBGPU_METHOD_FINISH)?,
        })
    }
}

/// Inherent implementation of [`WebGpuPassApi`].
impl WebGpuPassApi {
    /// Fetches the JavaScript methods of a render pass.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    fn new(pass: &JsValue) -> Option<Self> {
        Some(Self {
            set_pipeline: gpu_method(pass, WEBGPU_METHOD_SET_PIPELINE)?,
            set_vertex_buffer: gpu_method(pass, WEBGPU_METHOD_SET_VERTEX_BUFFER)?,
            set_index_buffer: gpu_method(pass, WEBGPU_METHOD_SET_INDEX_BUFFER)?,
            set_bind_group: gpu_method(pass, WEBGPU_METHOD_SET_BIND_GROUP)?,
            set_scissor_rect: gpu_method(pass, WEBGPU_METHOD_SET_SCISSOR_RECT)?,
            draw_indexed: gpu_method(pass, WEBGPU_METHOD_DRAW_INDEXED)?,
            end_pass: gpu_method(pass, WEBGPU_METHOD_END)?,
        })
    }
}

/// Inherent implementation of [`WebGpuBuffer`].
impl WebGpuBuffer {
    /// Makes sure the buffer can hold the given number of bytes.
    ///
    /// The capacity is rounded up to a power of two, so a chart whose data
    /// grows a little at a time does not re-create the buffer every time.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
    fn ensure(&mut self, device: &JsValue, size: u32, usage: u32) -> Option<JsValue> {
        if size == 0 {
            return None;
        }
        if self.get_capacity() < size {
            let capacity: u32 = size
                .checked_next_power_of_two()
                .unwrap_or(u32::MAX)
                .max(1024);
            let buffer: JsValue = WebGpuChartBackend::create_buffer(device, capacity, usage)?;
            self.set_buffer(Some(buffer));
            self.set_capacity(capacity);
        }
        self.try_get_buffer().clone()
    }

    /// Uploads raw bytes into the buffer.
    ///
    /// The bytes are handed to the GPU in one call, never one call per
    /// vertex, and the typed array is only alive for the duration of the
    /// call — `writeBuffer` copies the data before it returns.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&Function` - Shared reference to a `Function`.
    /// - `&mut Self` - Mutable reference to a `Self` (mutated in place).
    /// - `&[u8]` - Shared reference to a `[u8]`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    fn upload_bytes(
        device: &JsValue,
        queue: &JsValue,
        write_buffer: &Function,
        buffer: &mut Self,
        bytes: &[u8],
        usage: u32,
    ) {
        if bytes.is_empty() {
            return;
        }
        let Some(handle) = buffer.ensure(device, bytes.len() as u32, usage) else {
            return;
        };
        let data: Uint8Array = js_sys::Uint8Array::from(bytes);
        let _: Result<JsValue, JsValue> =
            write_buffer.call3(queue, &handle, &JsValue::from_f64(0.0), &data);
    }

    /// Uploads indices into the buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&Function` - Shared reference to a `Function`.
    /// - `&mut Self` - Mutable reference to a `Self` (mutated in place).
    /// - `&[u32]` - Shared reference to a `[u32]`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    fn upload_indices(
        device: &JsValue,
        queue: &JsValue,
        write_buffer: &Function,
        buffer: &mut Self,
        indices: &[u32],
        usage: u32,
    ) {
        if indices.is_empty() {
            return;
        }
        let Some(handle) = buffer.ensure(device, (indices.len() * 4) as u32, usage) else {
            return;
        };
        let data: Uint32Array = js_sys::Uint32Array::from(indices);
        let _: Result<JsValue, JsValue> =
            write_buffer.call3(queue, &handle, &JsValue::from_f64(0.0), &data);
    }
}

/// Inherent implementation of [`GpuHasher`].
impl GpuHasher {
    /// The FNV-1a offset basis.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    /// The FNV-1a prime.
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    /// Returns a hasher primed with the offset basis.
    pub(crate) fn new() -> Self {
        Self {
            state: Self::OFFSET,
        }
    }

    /// Mixes a floating-point value into the hash.
    ///
    /// Floating-point values do not implement `Hash`, so their bits are
    /// mixed in instead.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub(crate) fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    /// Mixes a double into the hash.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub(crate) fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }
}

/// Implements `Hasher` for [`GpuHasher`].
impl Hasher for GpuHasher {
    /// Implements the `finish` operation on [`GpuHasher`].
    ///
    /// # Returns
    ///
    /// - `u64` - A `u64` value.
    fn finish(&self) -> u64 {
        self.get_state()
    }

    /// Implements the `write` operation on [`GpuHasher`].
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Shared reference to a `[u8]`.
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte);
        }
    }

    /// Implements the `write_u8` operation on [`GpuHasher`].
    ///
    /// # Arguments
    ///
    /// - `u8` - A `u8` parameter.
    fn write_u8(&mut self, value: u8) {
        self.set_state(self.get_state() ^ u64::from(value));
        self.set_state(self.get_state().wrapping_mul(Self::PRIME));
    }

    /// Implements the `write_u32` operation on [`GpuHasher`].
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    fn write_u32(&mut self, value: u32) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    /// Implements the `write_u64` operation on [`GpuHasher`].
    ///
    /// # Arguments
    ///
    /// - `u64` - A `u64` parameter.
    fn write_u64(&mut self, value: u64) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    /// Implements the `write_i32` operation on [`GpuHasher`].
    ///
    /// # Arguments
    ///
    /// - `i32` - A 32-bit signed integer (`i32`).
    fn write_i32(&mut self, value: i32) {
        self.write_u32(value as u32);
    }
}
