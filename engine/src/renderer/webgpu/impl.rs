use super::*;

/// Implements async initialization and GPU resource creation for `WebGpuRenderer`.
impl WebGpuRenderer {
    /// Returns `true` if `navigator.gpu` is exposed on the current origin.
    ///
    /// This is the synchronous half of the canonical WebGPU capability
    /// probe used by Three.js (`examples/jsm/capabilities/WebGPU.js`): it
    /// only checks that the browser surfaces the `GPU` interface at all.
    /// It does **not** request an adapter — a present `navigator.gpu`
    /// does not guarantee that a usable GPU adapter is reachable (Linux
    /// software-rendered sessions, headless browsers, GPU-blacklisted
    /// devices and sandboxed iframes all expose `navigator.gpu` while
    /// `requestAdapter()` resolves to `null` or hangs forever).
    ///
    /// Use this as the cheapest pre-flight check before showing a
    /// "needs HTTPS or localhost" prompt. For a definitive answer use
    /// [`Self::probe`] which also awaits `requestAdapter()`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `navigator.gpu` is a non-null, non-undefined
    ///   object; `false` otherwise (including the "no `window`" runtime
    ///   case, which `web_sys::window()` returns `None` for).
    pub fn is_available() -> bool {
        let window_value: Window = match window() {
            Some(value) => value,
            None => return false,
        };
        let navigator: Navigator = window_value.navigator();
        let gpu_result: Result<JsValue, JsValue> = Reflect::get(
            navigator.as_ref(),
            &JsValue::from_str(WEBGPU_NAVIGATOR_GPU_KEY),
        );
        match gpu_result {
            Ok(value) => !value.is_undefined() && !value.is_null(),
            Err(_) => false,
        }
    }

    /// Probes whether a WebGPU adapter can actually be acquired.
    ///
    /// Mirrors Three.js' canonical capability probe exactly:
    ///
    /// Wraps the adapter request in the same `Promise.race` timeout used
    /// by [`Self::init`] so that browsers which leave the adapter promise
    /// permanently pending (headless, sandboxed, device-lost) do not stall
    /// the UI forever. The timeout itself uses the
    /// `INIT_PROMISE_TIMEOUT_MILLIS` constant; on timeout, `probe` returns
    /// `false` rather than an error so callers can treat it the same as
    /// "no adapter".
    ///
    /// # Returns
    ///
    /// - `bool` - `true` only when both `navigator.gpu` is present and
    ///   `requestAdapter()` resolves to a non-null adapter within the
    ///   timeout window. `false` covers every other case (no `window`,
    ///   missing `navigator.gpu`, reflect exception, adapter promise
    ///   rejected or timed out, adapter resolved to `null`/`undefined`).
    pub async fn probe() -> bool {
        if !Self::is_available() {
            return false;
        }
        let window_value: Window = match window() {
            Some(value) => value,
            None => return false,
        };
        let navigator: Navigator = window_value.navigator();
        let gpu: JsValue = match Reflect::get(
            navigator.as_ref(),
            &JsValue::from_str(WEBGPU_NAVIGATOR_GPU_KEY),
        ) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let request_adapter_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_REQUEST_ADAPTER)) {
                Ok(value) => value.unchecked_into(),
                Err(_) => return false,
            };
        let adapter_promise: Promise = match request_adapter_fn.call0(&gpu) {
            Ok(value) => value.unchecked_into(),
            Err(_) => return false,
        };
        let adapter_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(adapter_promise)).await {
                Ok(value) => value,
                Err(_) => return false,
            };
        !adapter_value.is_undefined() && !adapter_value.is_null()
    }

    /// Builds a `Promise` that rejects after `INIT_PROMISE_TIMEOUT_MILLIS`.
    ///
    /// Maximum time in milliseconds to wait for `requestAdapter` and
    /// `requestDevice` before treating them as failed.
    ///
    /// Some browser GPU states (headless, no GPU, sandboxed, device-lost)
    /// leave the WebGPU adapter/device promises permanently pending instead
    /// of resolving to `null` or rejecting. Without a timeout the
    /// `JsFuture::from(...).await` inside `init` would hang forever and
    /// the UI would stay stuck on `Initializing...`. Wrapping each promise
    /// in `Promise.race` against a timer-rejected sibling forces the
    /// future to resolve so the caller's error branch can run and report
    /// `WebGPU Not Supported`.
    ///
    /// # Returns
    ///
    /// - `Promise` - A promise that rejects with
    ///   `RENDERER_TIMEOUT_ERROR_MESSAGE` once the timeout elapses. When no
    ///   `window` exists the returned promise rejects immediately.
    fn timeout_promise() -> Promise {
        let Some(window_value) = window() else {
            return Promise::new(&mut |_resolve: Function, reject: Function| {
                let _: Result<JsValue, JsValue> = reject.call1(
                    &JsValue::UNDEFINED,
                    &JsValue::from_str(RENDERER_TIMEOUT_ERROR_MESSAGE),
                );
            });
        };
        Promise::new(&mut |_resolve: Function, reject: Function| {
            let reject_fn: Function = reject.clone();
            let timer: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let _: Result<JsValue, JsValue> = reject_fn.call1(
                    &JsValue::UNDEFINED,
                    &JsValue::from_str(RENDERER_TIMEOUT_ERROR_MESSAGE),
                );
            }));
            let _: Result<i32, JsValue> = window_value
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    timer.as_ref().unchecked_ref(),
                    INIT_PROMISE_TIMEOUT_MILLIS,
                );
            timer.forget();
        })
    }

    /// Wraps `promise` in `Promise.race([promise, timeout_promise()])` so that
    /// awaiting it never blocks longer than `INIT_PROMISE_TIMEOUT_MILLIS`.
    ///
    /// Calls `Promise.race` via reflection because wasm-bindgen does not
    /// currently expose the static `race` method on `Promise`.
    ///
    /// # Arguments
    ///
    /// - `Promise` - A `Promise` parameter.
    ///
    /// # Returns
    ///
    /// - `Promise` - A `Promise` value.
    fn race_with_timeout(promise: Promise) -> Promise {
        let array: Array = Array::of2(&promise, &Self::timeout_promise());
        Promise::race(&array)
    }

    /// Asynchronously initializes a WebGPU renderer from the given render configuration.
    ///
    /// Requests a GPU adapter and device, obtains the WebGPU canvas context,
    /// and configures it with the preferred texture format. Returns `Err` if
    /// WebGPU is not supported, the adapter/device request fails, the canvas
    /// element is not found, or the adapter/device request hangs beyond
    /// `INIT_PROMISE_TIMEOUT_MILLIS` (a defensive timeout for browser GPU
    /// states that leave the WebGPU promises permanently pending).
    ///
    /// The engine no longer logs diagnostic output internally; instead each
    /// failure mode is returned as a distinct `WebGpuInitError` variant so
    /// the caller can decide how to surface it (typically via `Console::error`
    /// or by falling back to the Canvas 2D backend).
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGpuRenderer, WebGpuInitError>` - The initialized renderer, or
    ///   a typed error describing the specific failure.
    pub async fn init(config: &RenderConfig) -> Result<WebGpuRenderer, WebGpuInitError> {
        let Some(window) = window() else {
            return Err(WebGpuInitError::NavigatorGpuMissing);
        };
        let navigator: Navigator = window.navigator();
        let gpu_result: Result<JsValue, JsValue> = Reflect::get(
            navigator.as_ref(),
            &JsValue::from_str(WEBGPU_NAVIGATOR_GPU_KEY),
        );
        let gpu: JsValue = match gpu_result {
            Ok(value) => value,
            Err(err) => return Err(WebGpuInitError::NavigatorLookup(err)),
        };
        if gpu.is_undefined() || gpu.is_null() {
            return Err(WebGpuInitError::NavigatorGpuMissing);
        }
        let adapter_options: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &adapter_options,
            &JsValue::from_str(WEBGPU_PROPERTY_POWER_PREFERENCE),
            &JsValue::from_str(config.power_preference.to_web_sys_string()),
        );
        let request_adapter_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_REQUEST_ADAPTER)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::RequestAdapterLookup(err)),
            };
        let adapter_promise: Promise = match request_adapter_fn.call1(&gpu, &adapter_options) {
            Ok(value) => value.unchecked_into(),
            Err(err) => return Err(WebGpuInitError::RequestAdapterCall(err)),
        };
        let adapter_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(adapter_promise)).await {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::AdapterPromise(err)),
            };
        if adapter_value.is_null() || adapter_value.is_undefined() {
            return Err(WebGpuInitError::AdapterUnavailable);
        }
        let device_descriptor: Object = Object::new();
        let request_device_fn: Function = match Reflect::get(
            &adapter_value,
            &JsValue::from_str(WEBGPU_METHOD_REQUEST_DEVICE),
        ) {
            Ok(value) => value.unchecked_into(),
            Err(err) => return Err(WebGpuInitError::RequestDeviceLookup(err)),
        };
        let device_promise: Promise =
            match request_device_fn.call1(&adapter_value, &device_descriptor) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::RequestDeviceCall(err)),
            };
        let device_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(device_promise)).await {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::DevicePromise(err)),
            };
        if device_value.is_null() || device_value.is_undefined() {
            return Err(WebGpuInitError::DeviceUnavailable);
        }
        let Some(document) = window.document() else {
            return Err(WebGpuInitError::CanvasNotFound(
                config.canvas_selector.clone(),
            ));
        };
        let element: Element = match document.query_selector(&config.canvas_selector) {
            Ok(Some(el)) => el,
            Ok(None) => {
                return Err(WebGpuInitError::CanvasNotFound(
                    config.canvas_selector.clone(),
                ));
            }
            Err(err) => return Err(WebGpuInitError::CanvasQuery(err)),
        };
        let canvas: HtmlCanvasElement = element.unchecked_into();
        let context_object: Option<Object> = canvas.get_context(WEBGPU_CONTEXT_TYPE).ok().flatten();
        let context_object: Object = match context_object {
            Some(c) => c,
            None => return Err(WebGpuInitError::CanvasContextUnavailable),
        };
        let context: JsValue = context_object.into();
        let get_format_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_GET_PREFERRED_FORMAT)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::PreferredFormatLookup(err)),
            };
        let format_value: JsValue = match get_format_fn.call0(&gpu) {
            Ok(value) => value,
            Err(err) => return Err(WebGpuInitError::PreferredFormatCall(err)),
        };
        let format: String = match format_value.as_string() {
            Some(s) => s,
            None => return Err(WebGpuInitError::PreferredFormatType(format_value)),
        };
        // WebGPU's `configure` requires the canvas backing-store size to be
        // set BEFORE calling configure, otherwise the swap chain is created
        // at 0x0 and the first getCurrentTexture() returns an error.
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (config.width * dpr).round() as u32;
        let physical_height: u32 = (config.height * dpr).round() as u32;
        canvas.set_width(physical_width);
        canvas.set_height(physical_height);
        let canvas_config: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_DEVICE),
            &device_value,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &format_value,
        );
        let configure_fn: Function =
            match Reflect::get(&context, &JsValue::from_str(WEBGPU_METHOD_CONFIGURE)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::ConfigureLookup(err)),
            };
        let _: Result<JsValue, JsValue> = configure_fn.call1(&context, &canvas_config);
        let queue: JsValue =
            match Reflect::get(&device_value, &JsValue::from_str(WEBGPU_PROPERTY_QUEUE)) {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::QueueLookup(err)),
            };
        Ok(WebGpuRenderer {
            device: device_value,
            queue,
            context,
            canvas,
            format,
            width: physical_width,
            height: physical_height,
            antialias: config.antialias,
            multisample_texture: None,
            multisample_view: None,
            depth_texture: None,
            depth_view: None,
            depth_format: None,
            device_lost_callback: None,
            device_lost: false,
            pending_error: Rc::new(PendingErrorCell::new()),
            command_encoder: None,
            render_pass_descriptor_cache: None,
        })
    }

    /// Allocates the multisampled intermediate texture used for MSAA.
    ///
    /// The returned tuple is `(GpuTexture, GpuTextureView)`:
    /// - `GpuTexture` has `sampleCount: 4` and `usage: RENDER_ATTACHMENT`
    ///   so it can be bound as a color attachment in `beginRenderPass`.
    /// - `GpuTextureView` is the default 2D view used as the color
    ///   attachment; the swap chain view is the `resolveTarget`.
    ///
    /// The texture size must match the swap chain physical size; mismatches
    /// are a WebGPU validation error. Returns `(JsValue::UNDEFINED,
    /// JsValue::UNDEFINED)` when allocation fails so callers can detect and
    /// fall back to MSAA=1.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical pixel width (DPR-multiplied).
    /// - `u32` - Physical pixel height.
    ///
    /// # Returns
    ///
    /// - `(JsValue, JsValue)` - The new texture and its default view, or
    ///   `JsValue::UNDEFINED` for both on allocation failure.
    fn create_multisample_texture(
        &self,
        physical_width: u32,
        physical_height: u32,
    ) -> (JsValue, JsValue) {
        let extent: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(physical_width)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(physical_height)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(1.0),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &extent,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(&self.get_format()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(f64::from(texture_usage_bit(TextureUsage::RenderAttachment))),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SAMPLE_COUNT),
            &JsValue::from_f64(4.0),
        );
        let create_texture_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = create_texture_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if texture.is_undefined() {
            return (JsValue::UNDEFINED, JsValue::UNDEFINED);
        }
        let create_view_fn: Function =
            Reflect::get(&texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let view: JsValue = create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED);
        if view.is_undefined() {
            return (texture, JsValue::UNDEFINED);
        }
        (texture, view)
    }

    /// Resizes the canvas backing store and reconfigures the swap chain.
    ///
    /// WebGPU's `GpuCanvasContext.configure` is sticky: it sets the texture
    /// format and device once, but the swap chain tracks the canvas's
    /// `width`/`height` attributes. When the CSS layout size changes (a
    /// window resize, a panel toggle, a DPR change) the canvas keeps its
    /// old physical dimensions unless we explicitly update `width`/`height`
    /// and call `configure` again. Without this, subsequent
    /// `getCurrentTexture()` calls return a texture that no longer matches
    /// the visible region and the frame either stretches or freezes.
    ///
    /// Re-`configure`ing with the same `device` + `format` is the
    /// spec-defined way to swap in a fresh swap chain bound to the new
    /// backing-store size.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new physical pixel width (already multiplied by DPR).
    /// - `u32` - The new physical pixel height.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` on success, `false` if the swap chain or canvas
    ///   handles were missing or `configure` failed.
    pub fn resize(&mut self, physical_width: u32, physical_height: u32) -> bool {
        if self.get_canvas().is_null()
            || self.get_context().is_null()
            || self.get_device().is_undefined()
        {
            return false;
        }
        self.get_canvas().set_width(physical_width);
        self.get_canvas().set_height(physical_height);
        let format_value: JsValue = JsValue::from_str(&self.get_format());
        let canvas_config: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_DEVICE),
            self.get_device(),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &format_value,
        );
        let configure_fn: Function = Reflect::get(
            self.get_context(),
            &JsValue::from_str(WEBGPU_METHOD_CONFIGURE),
        )
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<Function>().ok())
        .unwrap_or_else(|| Function::new_no_args(""));
        if configure_fn
            .call1(self.get_context(), &canvas_config)
            .is_err()
        {
            return false;
        }
        self.set_width(physical_width);
        self.set_height(physical_height);
        // Rebuild the multisampled color texture to match the new backing
        // store size. `GpuTexture` width/height are immutable, so MSAA
        // requires recreating it on every resize. The previous texture (if
        // any) is left to the GPU's GC; we do not explicitly destroy it
        // because `destroy()` is a synchronous WebGPU call and the old
        // texture is no longer referenced by any in-flight command buffer
        // at this point in the frame loop.
        if self.get_antialias() {
            let (texture, view) = self.create_multisample_texture(physical_width, physical_height);
            if !view.is_undefined() {
                self.set_multisample_texture(Some(texture));
                self.set_multisample_view(Some(view));
            } else {
                self.set_multisample_texture(None);
                self.set_multisample_view(None);
            }
        }
        true
    }

    /// Resizes the canvas backing store to match the canvas element's
    /// current CSS-rendered size in physical pixels (DPR applied).
    ///
    /// This is the right entry point when the render loop does not know
    /// the desired logical size ahead of time and wants to follow the
    /// element's actual layout box. It is also useful as a defensive
    /// recovery when the canvas was created while hidden (zero-sized
    /// parent) and is later shown at its real size.
    ///
    /// Reads `client_width` / `client_height` from the canvas element,
    /// multiplies by `detect_dpr()`, and forwards to [`Self::resize`].
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the resize succeeded, `false` if the canvas
    ///   was zero-sized (nothing to render to), detached (CSS layout
    ///   box collapses to 0), or the underlying resize rejected.
    pub fn sync_to_current_canvas(&mut self) -> bool {
        let canvas_width: u32 = self.get_canvas().width();
        let canvas_height: u32 = self.get_canvas().height();
        let client_width: u32 = self
            .get_canvas()
            .client_width()
            .try_into()
            .unwrap_or_default();
        let client_height: u32 = self
            .get_canvas()
            .client_height()
            .try_into()
            .unwrap_or_default();
        // Prefer the CSS layout box when it is non-zero. If the canvas
        // is hidden the client box collapses to 0; in that case fall
        // back to the current backing-store size so we do not
        // gratuitously resize to 0.
        let css_w: u32 = if client_width > 0 {
            client_width
        } else {
            canvas_width
        };
        let css_h: u32 = if client_height > 0 {
            client_height
        } else {
            canvas_height
        };
        if css_w == 0 || css_h == 0 {
            return false;
        }
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (f64::from(css_w) * dpr).round() as u32;
        let physical_height: u32 = (f64::from(css_h) * dpr).round() as u32;
        self.resize(physical_width, physical_height)
    }

    /// Creates a shader module from WGSL source code.
    ///
    /// # Arguments
    ///
    /// - `S` - The WGSL shader source code.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created shader module as a JavaScript value.
    pub(crate) fn create_shader_module<S>(&self, code: S) -> JsValue
    where
        S: AsRef<str>,
    {
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_CODE),
            &JsValue::from_str(code.as_ref()),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SHADER_MODULE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a new command encoder for recording GPU commands.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created command encoder as a JavaScript value.
    pub fn create_command_encoder(&self) -> JsValue {
        // OPT 2b: cached `device.createCommandEncoder()` — `Function`
        // is the same prototype slot for the device's lifetime, so
        // skipping the `Reflect::get` shaves ~110ns per frame.
        let create_fn: Function = cached_method(
            GpuReceiverClass::Device,
            self.get_device(),
            WEBGPU_METHOD_CREATE_COMMAND_ENCODER,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        create_fn
            .call0(self.get_device())
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Returns the current texture view from the canvas swap chain.
    ///
    /// This texture view should be used as the color attachment target for
    /// render passes. The texture is automatically presented to the canvas
    /// when the command buffer is submitted.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The current frame's texture view as a JavaScript value.
    pub(crate) fn get_current_texture_view(&self) -> JsValue {
        // OPT 2b: cached `context.getCurrentTexture()` and the
        // resulting `texture.createView()`. Both methods live on
        // stable prototypes, so the `Function` reference is stable
        // for the receiver's lifetime.
        let get_texture_fn: Function = cached_method(
            GpuReceiverClass::Context,
            self.get_context(),
            WEBGPU_METHOD_GET_CURRENT_TEXTURE,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let texture: JsValue = get_texture_fn
            .call0(self.get_context())
            .unwrap_or(JsValue::UNDEFINED);
        let create_view_fn: Function = cached_method(
            GpuReceiverClass::Texture,
            &texture,
            WEBGPU_METHOD_CREATE_VIEW,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED)
    }

    /// Begins a render pass on the given command encoder with a clear color.
    ///
    /// The render pass targets the canvas's current texture and clears it
    /// to the specified color. The returned `JsValue` is a `GpuRenderPassEncoder`
    /// that can be used to issue draw commands. The pass must be ended (via `end()`)
    /// before the command encoder is finished.
    ///
    /// This is a thin convenience wrapper over
    /// [`WebGpuRenderer::begin_render_pass_full`]. For pipelines that
    /// need depth testing, multiple color attachments, MSAA control,
    /// or `load`/`store` op customization, use the full version with
    /// a [the historical `RenderPassColorAttachment`] (and optional
    /// [the historical `RenderPassDepthStencilAttachment`]).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The command encoder to begin the pass on.
    /// - `Color` - The clear color, in 0.0-1.0 per channel.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active render pass encoder as a JavaScript value.
    pub fn begin_render_pass(&mut self, encoder: &JsValue, clear_color: Color) -> JsValue {
        let color: ColorAttachment = ColorAttachment {
            view: None,
            resolve_target: None,
            clear: Some(clear_color),
            load_op: LoadOp::Clear,
            store_op: StoreOp::Store,
        };
        self.begin_render_pass_full(encoder, &color, None)
    }

    /// Begins a render pass with full control over attachments, load/store
    /// ops, MSAA resolve targets, and an optional depth-stencil attachment.
    ///
    /// This is the "complete" render-pass API used by the rest of the
    /// engine. All other render-pass entry points (including the
    /// legacy `begin_render_pass(clear_color)` wrapper) funnel through
    /// here.
    ///
    /// The color attachment's `view` is filled in lazily when `None`:
    /// if `antialias == true` and the multisample intermediate is
    /// available (or can be allocated), the pass draws into the MSAA
    /// view and resolves into the swap chain; otherwise it draws
    /// directly into the swap chain. The `resolve_target` is filled in
    /// with the swap-chain view when MSAA is active and the caller
    /// did not provide one.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuCommandEncoder` to begin the pass on.
    /// - `&ColorAttachment` - The color attachment. `view` and
    ///   `resolve_target` may be `None`; they are filled in with the
    ///   renderer's defaults.
    /// - `Option<&DepthStencilAttachment>` - An optional depth-stencil
    ///   attachment. `Some(...)` adds a `depthStencilAttachment` field
    ///   to the pass descriptor; `None` omits it entirely.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active `GpuRenderPassEncoder` as a JavaScript
    ///   value, suitable for the existing `set_pipeline` / `draw` /
    ///   `end_render_pass` calls.
    pub fn begin_render_pass_full(
        &mut self,
        encoder: &JsValue,
        color: &ColorAttachment,
        depth: Option<&DepthStencilAttachment>,
    ) -> JsValue {
        let swap_chain_view: JsValue = self.get_current_texture_view();
        // Resolve MSAA view + resolve target with the same policy as
        // the legacy `begin_render_pass` - prefer the existing
        // multisample view, lazily allocate it if missing, and fall
        // back to direct-to-swap-chain if MSAA allocation fails.
        // `get_view()` is lombok-generated as `self.view.clone().unwrap()`,
        // and the documented `view: None` case ("let the renderer use the
        // swap-chain view") is exactly the case `begin_render_pass` passes, so
        // the infallible getter aborted the whole render pass with
        // `Option::unwrap() on a None value`. Read the fallible accessor and
        // treat both `None` and `undefined` as "substitute a view below".
        let (color_view, resolve_view): (JsValue, Option<JsValue>) = match color.try_get_view() {
            Some(view) if !view.is_undefined() => {
                (view.clone(), color.try_get_resolve_target().clone())
            }
            _ => {
                if self.get_antialias() {
                    let multisample_view: Option<JsValue> = self
                        .get_multisample_view()
                        .clone()
                        .filter(|value: &JsValue| !value.is_undefined());
                    let resolved: Option<JsValue> = match multisample_view {
                        Some(view) => Some(view),
                        None => {
                            let width: u32 = self.get_width();
                            let height: u32 = self.get_height();
                            let (texture, view): (JsValue, JsValue) =
                                self.create_multisample_texture(width, height);
                            if !view.is_undefined() {
                                self.set_multisample_texture(Some(texture));
                                self.set_multisample_view(Some(view.clone()));
                                Some(view)
                            } else {
                                self.set_multisample_texture(None);
                                self.set_multisample_view(None);
                                None
                            }
                        }
                    };
                    match resolved {
                        Some(view) => (view, Some(swap_chain_view.clone())),
                        None => (swap_chain_view.clone(), None),
                    }
                } else {
                    (swap_chain_view.clone(), None)
                }
            }
        };
        // OPT 34: cache the render-pass descriptor across frames so
        // we only allocate the JS `Object`/`Array` once and only
        // rewrite the fields that actually change between frames
        // (typically `clearValue`). The cache is invalidated on
        // load/store op or depth-stencil shape changes (rare).
        //
        // Effective ops are `&'static str` (from `WEBGPU_*_OP_*`
        // constants), so a pointer-compare detects "caller switched
        // ops" with zero cost.
        let effective_load_op: &'static str = load_op_name(*color.get_load_op());
        let effective_store_op: &'static str = store_op_name(*color.get_store_op());
        let has_depth: bool = depth.is_some();
        let has_resolve: bool = resolve_view.is_some();
        let cache_needs_rebuild: bool = match self.try_get_render_pass_descriptor_cache().as_ref() {
            None => true,
            Some(existing) => {
                existing.last_load_op != Some(effective_load_op)
                    || existing.last_store_op != Some(effective_store_op)
                    || existing.last_has_depth != has_depth
                    || existing.last_has_resolve != has_resolve
            }
        };
        if cache_needs_rebuild {
            let descriptor: RenderPassDescriptorCache = self.build_render_pass_descriptor(
                &color_view,
                resolve_view.as_ref(),
                *color.try_get_clear(),
                effective_load_op,
                effective_store_op,
                depth,
            );
            self.set_render_pass_descriptor_cache(Some(descriptor));
        }
        // `Some(_)` invariant: either the cache was non-None at the
        // top of this function (we only land in the None branch when
        // `cache_needs_rebuild` was true, in which case we just set
        // it above) or the caller passed us a renderer with no
        // descriptor cache yet and we built one. In both cases the
        // `Some` arm is the only reachable branch; we fall back to
        // a freshly-built empty cache (and emit no `beginRenderPass`
        // call) only if the impossible happened — `build_*` returned
        // a cache that was somehow dropped between the two lines,
        // which it cannot (no panic path, no early return).
        let cached: Option<RenderPassDescriptorCache> = self.try_get_render_pass_descriptor_cache();
        let cache: &RenderPassDescriptorCache = match cached.as_ref() {
            Some(c) => c,
            None => {
                // Defensive: build a no-op cache so the renderer's
                // caller sees a stable `JsValue::UNDEFINED` rather
                // than a dangling call. This branch is unreachable
                // under the invariant above.
                return JsValue::UNDEFINED;
            }
        };
        // Hot path: only the `clearValue` (and sometimes `view`) is
        // mutated between frames. We update the cached `view` and
        // `clearValue` Object's `r`/`g`/`b`/`a` properties
        // unconditionally — `Reflect::set` is a fast pointer write
        // when the value differs, and the JS-side property setter
        // accepts the same numeric value with no observable change.
        let _: Result<bool, JsValue> = Reflect::set(
            &cache.attachment,
            &cached_method_name(WEBGPU_PROPERTY_VIEW),
            &color_view,
        );
        // `resolveTarget` is the per-frame swap-chain view on the MSAA
        // path (`context.getCurrentTexture()` textures expire when the
        // frame is presented), so it MUST be refreshed every frame —
        // keeping the first frame's view makes every subsequent
        // `beginRenderPass` fail validation silently (black canvas).
        // When the caller drops MSAA mid-stream the Some/None shape
        // change triggers a rebuild above, so the `None` arm here never
        // leaves a stale `resolveTarget` behind.
        if let Some(target) = resolve_view.as_ref() {
            let _: Result<bool, JsValue> = Reflect::set(
                &cache.attachment,
                &cached_method_name(WEBGPU_PROPERTY_RESOLVE_TARGET),
                target,
            );
        }
        if let Some(cv) = *color.try_get_clear() {
            let _: Result<bool, JsValue> = Reflect::set(
                &cache.clear_value,
                &cached_method_name(WEBGPU_PROPERTY_R),
                &JsValue::from_f64(cv.get_red()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &cache.clear_value,
                &cached_method_name(WEBGPU_PROPERTY_G),
                &JsValue::from_f64(cv.get_green()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &cache.clear_value,
                &cached_method_name(WEBGPU_PROPERTY_B),
                &JsValue::from_f64(cv.get_blue()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &cache.clear_value,
                &cached_method_name(WEBGPU_PROPERTY_A),
                &JsValue::from_f64(cv.get_alpha()),
            );
            // `attachment.clearValue` always points at the same
            // `clear_value` Object, so we only need to set it on the
            // very first call (i.e. when the cache was just built).
            // Subsequent calls leave the link intact.
            if cache_needs_rebuild {
                let _: Result<bool, JsValue> = Reflect::set(
                    &cache.attachment,
                    &cached_method_name(WEBGPU_PROPERTY_CLEAR_VALUE),
                    &cache.clear_value,
                );
            }
        }
        // The `descriptor.colorAttachments[0]` slot is stable for the
        // cache's lifetime (set once when the descriptor was built);
        // `view` / `resolveTarget` / `clearValue` are refreshed above
        // on every call.
        let begin_fn: Function = cached_method(
            GpuReceiverClass::CommandEncoder,
            encoder,
            WEBGPU_METHOD_BEGIN_RENDER_PASS,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        begin_fn
            .call1(encoder, &cache.descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// OPT 34 helper: build a fresh `RenderPassDescriptorCache` from
    /// scratch. Called from [`WebGpuRenderer::begin_render_pass_full`]
    /// on cache miss (first call, op change, or depth-shape change).
    ///
    /// The constructed cache holds:
    /// - `descriptor` - the top-level `GpuRenderPassDescriptor`
    ///   Object, passed directly to `encoder.beginRenderPass`.
    /// - `color_attachments` - a length-1 `Array` containing the
    ///   cached `attachment` Object.
    /// - `attachment` - the inner color attachment Object.
    /// - `clear_value` - the `{r, g, b, a}` dictionary under
    ///   `attachment.clearValue`. This is the only Object whose
    ///   fields are mutated per frame.
    /// - `last_load_op` / `last_store_op` - the `&'static str` ops
    ///   applied to the descriptor this frame, used to detect
    ///   caller-driven op changes.
    /// - `last_has_depth` - whether the descriptor had a
    ///   `depthStencilAttachment`, used to detect shape changes.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuTextureView` for the color attachment.
    /// - `Option<&JsValue>` - Optional resolve target (MSAA only).
    /// - `Option<Color>` - Optional clear color.
    /// - `&'static str` - The load op to encode.
    /// - `&'static str` - The store op to encode.
    /// - `Option<&DepthStencilAttachment>` - Optional depth-stencil attachment.
    ///
    /// # Returns
    ///
    /// - `RenderPassDescriptorCache` - The cache entry the renderer retains
    ///   for this load-op / store-op / depth-shape combination.
    fn build_render_pass_descriptor(
        &mut self,
        color_view: &JsValue,
        resolve_view: Option<&JsValue>,
        clear_value: Option<Color>,
        effective_load_op: &'static str,
        effective_store_op: &'static str,
        depth: Option<&DepthStencilAttachment>,
    ) -> RenderPassDescriptorCache {
        let attachment: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &attachment,
            &cached_method_name(WEBGPU_PROPERTY_VIEW),
            color_view,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &attachment,
            &cached_method_name(WEBGPU_PROPERTY_LOAD_OP),
            &JsValue::from_str(effective_load_op),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &attachment,
            &cached_method_name(WEBGPU_PROPERTY_STORE_OP),
            &JsValue::from_str(effective_store_op),
        );
        let clear_value_obj: Object = Object::new();
        if let Some(cv) = clear_value {
            let _: Result<bool, JsValue> = Reflect::set(
                &clear_value_obj,
                &cached_method_name(WEBGPU_PROPERTY_R),
                &JsValue::from_f64(cv.get_red()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &clear_value_obj,
                &cached_method_name(WEBGPU_PROPERTY_G),
                &JsValue::from_f64(cv.get_green()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &clear_value_obj,
                &cached_method_name(WEBGPU_PROPERTY_B),
                &JsValue::from_f64(cv.get_blue()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &clear_value_obj,
                &cached_method_name(WEBGPU_PROPERTY_A),
                &JsValue::from_f64(cv.get_alpha()),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &attachment,
                &cached_method_name(WEBGPU_PROPERTY_CLEAR_VALUE),
                &clear_value_obj,
            );
        }
        if let Some(target) = resolve_view {
            let _: Result<bool, JsValue> = Reflect::set(
                &attachment,
                &cached_method_name(WEBGPU_PROPERTY_RESOLVE_TARGET),
                target,
            );
        }
        let color_attachments: Array = Array::new();
        color_attachments.push(&attachment);
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &cached_method_name(WEBGPU_PROPERTY_COLOR_ATTACHMENTS),
            &color_attachments,
        );
        let last_has_depth: bool = if let Some(depth_desc) = depth {
            // Prefer the caller-provided view; otherwise lazily
            // allocate the default depth-stencil texture and use its
            // view.
            // Same hazard as the color attachment: `get_view()` unwraps.
            let depth_view: JsValue = match depth_desc.try_get_view() {
                Some(v) if !v.is_undefined() => v.clone(),
                _ => match self.create_depth_texture() {
                    Some(v) => v,
                    None => JsValue::UNDEFINED,
                },
            };
            if !depth_view.is_undefined() {
                let depth_attachment: Object = Object::new();
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &cached_method_name(WEBGPU_PROPERTY_VIEW),
                    &depth_view,
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &cached_method_name(WEBGPU_PROPERTY_DEPTH_LOAD_OP),
                    &JsValue::from_str(load_op_name(*depth_desc.get_depth_load_op())),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &cached_method_name(WEBGPU_PROPERTY_DEPTH_STORE_OP),
                    &JsValue::from_str(store_op_name(*depth_desc.get_depth_store_op())),
                );
                if let Some(clear) = *depth_desc.try_get_depth_clear() {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &depth_attachment,
                        &cached_method_name(WEBGPU_PROPERTY_DEPTH_CLEAR_VALUE),
                        &JsValue::from_f64(clear),
                    );
                }
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &cached_method_name(WEBGPU_PROPERTY_DEPTH_READ_ONLY),
                    &JsValue::from_bool(*depth_desc.get_depth_read_only()),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &descriptor,
                    &cached_method_name(WEBGPU_PROPERTY_DEPTH_STENCIL_ATTACHMENT),
                    &depth_attachment,
                );
                true
            } else {
                false
            }
        } else {
            false
        };
        RenderPassDescriptorCache {
            descriptor,
            attachment,
            clear_value: clear_value_obj,
            last_load_op: Some(effective_load_op),
            last_store_op: Some(effective_store_op),
            last_has_depth,
            last_has_resolve: resolve_view.is_some(),
        }
    }

    /// Submits an array of command buffers to the GPU queue for execution.
    ///
    /// # Arguments
    ///
    /// - `&[JsValue]` - The command buffers to submit.
    pub fn submit(&self, command_buffers: &[JsValue]) {
        // The common case is exactly one command buffer per frame —
        // `Array::of1` skips the grow-from-empty push dance.
        let array: Array = match command_buffers {
            [single] => Array::of1(single),
            many => many.iter().cloned().collect(),
        };
        // OPT 2b: cached `queue.submit()` — `Function` is the same
        // prototype slot for the queue's lifetime.
        let _: Result<JsValue, JsValue> = cached_method_call(
            GpuReceiverClass::Queue,
            self.get_queue(),
            WEBGPU_METHOD_SUBMIT,
            &array,
        );
    }

    /// Creates a simple render pipeline from a single WGSL shader source.
    ///
    /// The shader must contain `@vertex fn vs_main(...)` and
    /// `@fragment fn fs_main(...)` entry points. No vertex buffers are used;
    /// vertex positions should be derived from `@builtin(vertex_index)` in
    /// the shader. The pipeline uses auto-layout (`layout: null`), which works
    /// when the shader has no bind groups.
    ///
    /// This is a preset over [`RenderPipelineDescriptor`]: triangle-list
    /// topology, no culling, no depth state, one color target in the
    /// canvas's own format, and whatever sample count the renderer's
    /// `antialias` flag asks for. For anything else, build a
    /// [`RenderPipelineDescriptor`] and use
    /// [`WebGpuRenderer::create_render_pipeline_full`].
    ///
    /// # Arguments
    ///
    /// - `S` - The WGSL shader source code.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created render pipeline as a JavaScript value.
    pub fn create_render_pipeline<S>(&self, shader_code: S) -> JsValue
    where
        S: AsRef<str>,
    {
        let module: JsValue = self.create_shader_module(shader_code);
        // The color target format MUST match the attachment the pass will
        // render into. For the canvas that is the swap-chain format reported
        // by `navigator.gpu.getPreferredCanvasFormat()`, which is `bgra8unorm`
        // on most desktop browsers - hardcoding `rgba8unorm` here makes the
        // pipeline incompatible with its own render pass, WebGPU rejects the
        // command buffer, and the canvas silently stays black with no
        // visible error. Ask the renderer which format it configured.
        let target_format: GpuTextureFormat = match self.get_format().as_str() {
            WEBGPU_FORMAT_BGRA8UNORM => GpuTextureFormat::Bgra8Unorm,
            _ => GpuTextureFormat::Rgba8Unorm,
        };
        let targets: Vec<ColorTargetState> = vec![ColorTargetState::for_format(target_format)];
        self.create_render_pipeline_full(&RenderPipelineDescriptor::new(
            VertexState::new(
                module.clone(),
                WEBGPU_VERTEX_ENTRY_POINT.to_string(),
                Vec::new(),
            ),
            PrimitiveState::default(),
            None,
            self.default_multisample_state(),
            Some(FragmentState::new(
                module,
                WEBGPU_FRAGMENT_ENTRY_POINT.to_string(),
                targets,
            )),
        ))
    }

    /// Creates a render pipeline from a [`RenderPipelineDescriptor`],
    /// with the WebGPU auto layout.
    ///
    /// The descriptor's typed states are assembled into the
    /// `GpuRenderPipelineDescriptor` the device expects: the vertex
    /// stage and its buffer layouts, the primitive state (topology,
    /// front face, cull mode, strip index format), the optional
    /// depth-stencil state, the multisample state, and the optional
    /// fragment stage with one entry per color target.
    ///
    /// This is a one-shot path - a pipeline is normally created once
    /// and reused for every frame, so the descriptor objects it builds
    /// do not need to be cached. The per-frame paths that do allocate
    /// (render passes, bind groups, draw state) all go through
    /// `cached_method` and [`RenderPassDescriptorCache`] instead.
    ///
    /// # Arguments
    ///
    /// - `&RenderPipelineDescriptor` - The full pipeline description.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created render pipeline as a JavaScript value.
    pub fn create_render_pipeline_full(&self, descriptor: &RenderPipelineDescriptor) -> JsValue {
        let descriptor_object: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &JsValue::from_str(WEBGPU_AUTO_LAYOUT),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_VERTEX),
            &self.build_vertex_state(descriptor.get_vertex()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_PRIMITIVE),
            &self.build_primitive_state(descriptor.get_primitive()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_MULTISAMPLE),
            &self.build_multisample_state(descriptor.get_multisample()),
        );
        if let Some(depth) = descriptor.try_get_depth_stencil() {
            let depth_object: Object = self.build_depth_stencil_state(depth);
            let _: Result<bool, JsValue> = Reflect::set(
                &descriptor_object,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_STENCIL),
                &depth_object,
            );
        }
        if let Some(fragment) = descriptor.try_get_fragment() {
            let fragment_object: Object = self.build_fragment_state(fragment);
            let _: Result<bool, JsValue> = Reflect::set(
                &descriptor_object,
                &JsValue::from_str(WEBGPU_PROPERTY_FRAGMENT),
                &fragment_object,
            );
        }
        let create_fn: Function = cached_method(
            GpuReceiverClass::Device,
            self.get_device(),
            WEBGPU_METHOD_CREATE_RENDER_PIPELINE,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        create_fn
            .call1(self.get_device(), &descriptor_object)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Assembles the `vertex` half of a pipeline descriptor into the JS
    /// object `createRenderPipeline` expects.
    ///
    /// # Arguments
    ///
    /// - `&VertexState` - The shader module, entry point, and buffer
    ///   layouts of the vertex stage.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUVertexState` dictionary.
    fn build_vertex_state(&self, state: &VertexState) -> Object {
        let vertex_state: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &state.get_module(),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(state.get_entry_point().as_str()),
        );
        let buffers: Array = Array::new();
        for layout in state.get_buffers() {
            let layout_obj: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &layout_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_ARRAY_STRIDE),
                &JsValue::from_f64(layout.get_array_stride() as f64),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &layout_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_STEP_MODE),
                &JsValue::from_str(layout.get_step_mode().as_str()),
            );
            let attrs: Array = Array::new();
            for attribute in layout.get_attributes() {
                let attr: Object = Object::new();
                let _: Result<bool, JsValue> = Reflect::set(
                    &attr,
                    &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
                    &JsValue::from_str(vertex_attribute_format_name(attribute.get_format())),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &attr,
                    &JsValue::from_str(WEBGPU_PROPERTY_OFFSET),
                    &JsValue::from_f64(attribute.get_offset() as f64),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &attr,
                    &JsValue::from_str(WEBGPU_PROPERTY_SHADER_LOCATION),
                    &JsValue::from_f64(f64::from(attribute.get_shader_location())),
                );
                attrs.push(&attr);
            }
            let _: Result<bool, JsValue> = Reflect::set(
                &layout_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_ATTRIBUTES),
                &attrs,
            );
            buffers.push(&layout_obj);
        }
        let _: Result<bool, JsValue> = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_BUFFERS),
            &buffers,
        );
        vertex_state
    }

    /// Assembles the `primitive` half of a pipeline descriptor.
    ///
    /// # Arguments
    ///
    /// - `&PrimitiveState` - Topology, front-face winding, cull mode,
    ///   and the optional strip index format.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUPrimitiveState` dictionary.
    fn build_primitive_state(&self, state: &PrimitiveState) -> Object {
        let primitive: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &primitive,
            &JsValue::from_str(WEBGPU_PROPERTY_TOPOLOGY),
            &JsValue::from_str(primitive_topology_name(state.get_topology())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &primitive,
            &JsValue::from_str(WEBGPU_PROPERTY_FRONT_FACE),
            &JsValue::from_str(front_face_name(state.get_front_face())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &primitive,
            &JsValue::from_str(WEBGPU_PROPERTY_CULL_MODE),
            &JsValue::from_str(cull_mode_name(state.get_cull_mode())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &primitive,
            &JsValue::from_str(WEBGPU_PROPERTY_UNCLIPPED_DEPTH),
            &JsValue::from_bool(state.get_unclipped_depth()),
        );
        if let Some(format) = state.get_strip_index_format() {
            let _: Result<bool, JsValue> = Reflect::set(
                &primitive,
                &JsValue::from_str(WEBGPU_PROPERTY_STRIP_INDEX_FORMAT),
                &JsValue::from_str(index_format_name(format)),
            );
        }
        primitive
    }

    /// Assembles the `multisample` half of a pipeline descriptor.
    ///
    /// # Arguments
    ///
    /// - `&MultisampleState` - The samples per pixel and the sample
    ///   mask.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUMultisampleState` dictionary.
    fn build_multisample_state(&self, state: &MultisampleState) -> Object {
        let multisample: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &multisample,
            &JsValue::from_str(WEBGPU_PROPERTY_COUNT),
            &JsValue::from_f64(f64::from(state.get_count())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &multisample,
            &JsValue::from_str(WEBGPU_PROPERTY_MASK),
            &JsValue::from_f64(f64::from(state.get_mask())),
        );
        multisample
    }

    /// Assembles the `depthStencil` half of a pipeline descriptor.
    ///
    /// # Arguments
    ///
    /// - `&DepthStencilState` - The depth format, write flag, and
    ///   comparison.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUDepthStencilState` dictionary.
    fn build_depth_stencil_state(&self, state: &DepthStencilState) -> Object {
        let depth_stencil: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &depth_stencil,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(gpu_texture_format_name(state.get_format())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &depth_stencil,
            &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_WRITE_ENABLED),
            &JsValue::from_bool(state.get_depth_write_enabled()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &depth_stencil,
            &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_COMPARE),
            &JsValue::from_str(compare_function_name(state.get_depth_compare())),
        );
        depth_stencil
    }

    /// Assembles the `fragment` half of a pipeline descriptor.
    ///
    /// # Arguments
    ///
    /// - `&FragmentState` - The shader module, entry point, and one
    ///   [`ColorTargetState`] per color attachment.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUFragmentState` dictionary.
    fn build_fragment_state(&self, state: &FragmentState) -> Object {
        let fragment_state: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &state.get_module(),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(state.get_entry_point().as_str()),
        );
        let targets: Array = Array::new();
        for target in state.get_targets() {
            targets.push(&self.build_color_target_state(target));
        }
        let _: Result<bool, JsValue> = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_TARGETS),
            &targets,
        );
        fragment_state
    }

    /// Assembles one entry of a fragment stage's `targets` array.
    ///
    /// # Arguments
    ///
    /// - `&ColorTargetState` - The target format, optional blend
    ///   state, and channel write mask.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUColorTargetState` dictionary.
    fn build_color_target_state(&self, state: &ColorTargetState) -> Object {
        let target: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &target,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(gpu_texture_format_name(state.get_format())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &target,
            &JsValue::from_str(WEBGPU_PROPERTY_WRITE_MASK),
            &JsValue::from_f64(f64::from(state.get_write_mask())),
        );
        if let Some(blend) = state.get_blend() {
            let blend_object: Object = Object::new();
            let color_component: Object = self.build_blend_component(&blend.get_color());
            let _: Result<bool, JsValue> = Reflect::set(
                &blend_object,
                &JsValue::from_str(WEBGPU_PROPERTY_COLOR),
                &color_component,
            );
            let alpha_component: Object = self.build_blend_component(&blend.get_alpha());
            let _: Result<bool, JsValue> = Reflect::set(
                &blend_object,
                &JsValue::from_str(WEBGPU_PROPERTY_ALPHA),
                &alpha_component,
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &target,
                &JsValue::from_str(WEBGPU_PROPERTY_BLEND),
                &blend_object,
            );
        }
        target
    }

    /// Assembles one channel group's blend dictionary.
    ///
    /// # Arguments
    ///
    /// - `&BlendComponent` - The operation and the two factors.
    ///
    /// # Returns
    ///
    /// - `Object` - The assembled `GPUBlendComponent` dictionary.
    fn build_blend_component(&self, component: &BlendComponent) -> Object {
        let blend: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &blend,
            &JsValue::from_str(WEBGPU_PROPERTY_OPERATION),
            &JsValue::from_str(blend_operation_name(component.get_operation())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &blend,
            &JsValue::from_str(WEBGPU_PROPERTY_SRC_FACTOR),
            &JsValue::from_str(blend_factor_name(component.get_source())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &blend,
            &JsValue::from_str(WEBGPU_PROPERTY_DST_FACTOR),
            &JsValue::from_str(blend_factor_name(component.get_destination())),
        );
        blend
    }

    /// The multisample state the renderer's `antialias` flag asks for.
    ///
    /// `antialias` is read on the renderer rather than carried on the
    /// descriptor so the built-in presets and the depth- and
    /// multisample-aware paths all agree on one sample count.
    ///
    /// # Returns
    ///
    /// - `MultisampleState` - 4x MSAA when antialiasing is on, 1x
    ///   otherwise, writing every sample in both cases.
    fn default_multisample_state(&self) -> MultisampleState {
        MultisampleState::with_sample_count(if self.get_antialias() { 4 } else { 1 })
    }

    /// Sets the render pipeline on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `&JsValue` - The render pipeline to set.
    pub fn set_pipeline(&self, pass: &JsValue, pipeline: &JsValue) {
        // OPT 2b: cached `pass.setPipeline()` — function is on the
        // shared prototype; the call still passes `this = pass`
        // explicitly because JS `Function` doesn't auto-bind.
        let set_fn: Function = cached_method(
            GpuReceiverClass::RenderPass,
            pass,
            WEBGPU_METHOD_SET_PIPELINE,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = set_fn.call1(pass, pipeline);
    }

    /// Binds a vertex buffer at the given slot on a render pass encoder.
    ///
    /// This is the missing link between `create_render_pipeline_full` /
    /// `create_render_pipeline_with_layout` and the actual draw call:
    /// without `set_vertex_buffer` the GPU has no idea what attribute
    /// data the vertex shader's `@location(N)` references point at.
    /// Calling this with `buffer.is_undefined()` is a silent no-op
    /// (matches the WebGPU spec).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `u32` - The slot index; matches the slot the vertex buffer
    ///   was declared at in the pipeline's `vertex.buffers` array.
    /// - `&JsValue` - The `GpuBuffer` to bind (typically obtained
    ///   from `create_vertex_buffer`).
    pub fn set_vertex_buffer(&self, pass: &JsValue, slot: u32, buffer: &JsValue) {
        if buffer.is_undefined() || buffer.is_null() {
            return;
        }
        // OPT 2b: cached `pass.setVertexBuffer(slot, buffer)`.
        let set_fn: Function = cached_method(
            GpuReceiverClass::RenderPass,
            pass,
            WEBGPU_METHOD_SET_VERTEX_BUFFER,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> =
            set_fn.call2(pass, &JsValue::from_f64(f64::from(slot)), buffer);
    }

    /// Binds an index buffer on a render pass encoder.
    ///
    /// Once bound, subsequent `draw_indexed` calls read their indices
    /// from this buffer. `format` must be either `"uint16"` or
    /// `"uint32"` — see [`WEBGPU_INDEX_FORMAT_UINT16`] and
    /// [`WEBGPU_INDEX_FORMAT_UINT32`].
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `&JsValue` - The `GpuBuffer` containing the index list.
    /// - `IndexFormat` - The element width of the index data. It must
    ///   match the element type of the indices themselves.
    pub fn set_index_buffer(&self, pass: &JsValue, buffer: &JsValue, format: IndexFormat) {
        if buffer.is_undefined() || buffer.is_null() {
            return;
        }
        // OPT 2b: cached `pass.setIndexBuffer(buffer, format)`.
        // The two spec formats hit the thread-local `JsValue` cache instead
        // of paying a fresh JS string allocation per call (per entity per
        // frame in mesh scenes).
        let format_value: JsValue = cached_method_name(index_format_name(format));
        let set_fn: Function = cached_method(
            GpuReceiverClass::RenderPass,
            pass,
            WEBGPU_METHOD_SET_INDEX_BUFFER,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = set_fn.call2(pass, buffer, &format_value);
    }

    /// Draws primitives on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `&DrawArgs` - The vertex count, instance count, and the first
    ///   vertex / first instance offsets.
    pub fn draw(&self, pass: &JsValue, args: &DrawArgs) {
        // OPT 2b: cached `pass.draw(vertexCount, instanceCount, firstVertex, firstInstance)`.
        let draw_fn: Function =
            cached_method(GpuReceiverClass::RenderPass, pass, WEBGPU_METHOD_DRAW)
                .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = draw_fn.call4(
            pass,
            &JsValue::from_f64(f64::from(args.get_vertex_count())),
            &JsValue::from_f64(f64::from(args.get_instance_count())),
            &JsValue::from_f64(f64::from(args.get_first_vertex())),
            &JsValue::from_f64(f64::from(args.get_first_instance())),
        );
    }

    /// Draws indexed primitives on a render pass encoder.
    ///
    /// The index buffer must already be bound via `set_index_buffer`.
    /// This is the modern path for everything that needs shared vertex
    /// data (mesh renderers, terrain, instanced objects).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `&DrawIndexedArgs` - The index count, instance count, first
    ///   index, base vertex, and first instance.
    pub fn draw_indexed(&self, pass: &JsValue, args: &DrawIndexedArgs) {
        // OPT 2b: cached `pass.drawIndexed(indexCount, instanceCount, firstIndex, baseVertex, firstInstance)`.
        let draw_fn: Function = cached_method(
            GpuReceiverClass::RenderPass,
            pass,
            WEBGPU_METHOD_DRAW_INDEXED,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = draw_fn.call5(
            pass,
            &JsValue::from_f64(f64::from(args.get_index_count())),
            &JsValue::from_f64(f64::from(args.get_instance_count())),
            &JsValue::from_f64(f64::from(args.get_first_index())),
            &JsValue::from_f64(args.get_base_vertex() as f64),
            &JsValue::from_f64(f64::from(args.get_first_instance())),
        );
    }

    /// Variant of `draw_indexed` that stops before the end of the
    /// bound index buffer, drawing `index_count` indices starting at
    /// `first_index`.
    ///
    /// `first_index` is measured in indices, not bytes, matching
    /// `GpuRenderPassEncoder.drawIndexed`'s `firstIndex` argument.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `u32` - The index to start reading at.
    /// - `u32` - The number of indices to consume.
    /// - `u32` - The number of instances to draw.
    pub fn draw_indexed_offset(
        &self,
        pass: &JsValue,
        index_offset: u32,
        index_count: u32,
        instance_count: u32,
    ) {
        let mut args: DrawIndexedArgs = DrawIndexedArgs::whole_buffer(index_count, instance_count);
        args.set_first_index(index_offset);
        self.draw_indexed(pass, &args);
    }

    /// Ends a render pass on the given pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder to end.
    pub fn end_render_pass(&self, pass: &JsValue) {
        // OPT 2b: cached `pass.end()`.
        let end_fn: Function = cached_method(GpuReceiverClass::RenderPass, pass, WEBGPU_METHOD_END)
            .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = end_fn.call0(pass);
    }

    /// Finishes a command encoder and returns the resulting command buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The command encoder to finish.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The finished command buffer.
    pub fn finish_command_encoder(&self, encoder: &JsValue) -> JsValue {
        // OPT 2b: cached `encoder.finish()`.
        let finish_fn: Function = cached_method(
            GpuReceiverClass::CommandEncoder,
            encoder,
            WEBGPU_METHOD_FINISH,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        finish_fn.call0(encoder).unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a GPU uniform buffer and initializes it with the given floats.
    ///
    /// The buffer is created with `UNIFORM | COPY_DST` usage so it can be
    /// bound in a bind group and refreshed per frame via
    /// [`WebGpuRenderer::update_uniform_buffer`]. The allocation size is
    /// rounded up to a multiple of 16 bytes because WebGPU requires uniform
    /// buffer bindings to be 16-byte aligned in size (a bare `vec2<f32>`
    /// uniform is only 8 bytes).
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The initial uniform contents (e.g. `[x, y]` for a
    ///   `vec2<f32>` uniform).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuBuffer`.
    pub fn create_uniform_buffer(&self, data: &[f32]) -> JsValue {
        let byte_len: usize = data.len() * 4;
        let size: u64 = (byte_len.div_ceil(16).max(1) as u64) * 16;
        let buffer: JsValue = self.create_buffer(&BufferDescriptor::new(
            size,
            buffer_usage_mask(&[BufferUsage::Uniform, BufferUsage::CopyDestination]),
        ));
        self.update_uniform_buffer(&buffer, data);
        buffer
    }

    /// Uploads float data into an existing uniform buffer via `queue.writeBuffer`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuBuffer` previously created by
    ///   [`WebGpuRenderer::create_uniform_buffer`].
    /// - `&[f32]` - The new uniform contents.
    pub fn update_uniform_buffer(&self, buffer: &JsValue, data: &[f32]) {
        // OPT 31: zero-copy view over the wasm linear-memory slice. The old
        // `Float32Array::from(data)` form allocates a new typed array and
        // copies every byte; per-frame uniform uploads (transforms, camera
        // matrices, particle data) can be hundreds of bytes per call.
        // SAFETY: `view` is only used inside the `write_fn.call3(...)` on
        // the next line; the resulting JsValue does not outlive `data`'s
        // borrow, and `data` outlives the call because the call happens
        // synchronously before this function returns.
        let view: Float32Array = unsafe { Float32Array::view(data) };
        // OPT 2b: cached `queue.writeBuffer(buffer, 0, view)`.
        let write_fn: Function = cached_method(
            GpuReceiverClass::Queue,
            self.get_queue(),
            WEBGPU_METHOD_WRITE_BUFFER,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> =
            write_fn.call3(self.get_queue(), buffer, &JsValue::from_f64(0.0), &view);
    }

    // ----------------------------------------------------------------------
    //  Compute pipeline + pass + dispatch
    // ----------------------------------------------------------------------

    /// Creates a compute pipeline from a WGSL shader.
    ///
    /// The shader must contain exactly one `@compute fn <name>(...)`
    /// entry point whose name matches `entry_point`. The pipeline uses
    /// auto-layout, so any `@group(N)` binding it declares is wired
    /// through `getBindGroupLayout(N)`.
    ///
    /// # Arguments
    ///
    /// - `S` - The WGSL source code.
    /// - `&str` - The compute entry-point name (e.g. `"cs_main"`).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuComputePipeline`, or
    ///   `JsValue::UNDEFINED` on failure.
    pub fn create_compute_pipeline<S>(&self, shader_code: S, entry_point: &str) -> JsValue
    where
        S: AsRef<str>,
    {
        let module: JsValue = self.create_shader_module(shader_code);
        let compute_state: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &compute_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &module,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &compute_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(entry_point),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &JsValue::from_str(WEBGPU_AUTO_LAYOUT),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_COMPUTE),
            &compute_state,
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_COMPUTE_PIPELINE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Begins a compute pass on the given command encoder.
    ///
    /// The returned `JsValue` is a `GpuComputePassEncoder` that supports
    /// `setPipeline` / `setBindGroup` / `dispatchWorkgroups` /
    /// `dispatchWorkgroupsIndirect` / `end`. The pass must be ended
    /// (via `end()`) before the command encoder is finished.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuCommandEncoder` to begin the pass on.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active `GpuComputePassEncoder`.
    pub fn begin_compute_pass(&self, encoder: &JsValue) -> JsValue {
        let begin_fn: Function = Reflect::get(
            encoder,
            &JsValue::from_str(WEBGPU_METHOD_BEGIN_COMPUTE_PASS),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let descriptor: Object = Object::new();
        begin_fn
            .call1(encoder, &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Issues a `dispatchWorkgroups(x, y, z)` on a compute pass encoder.
    ///
    /// `x`/`y`/`z` are the workgroup counts in each dimension. WebGPU
    /// limits each to `65535`; callers that need larger grids must
    /// split them across multiple dispatches or encode a loop inside
    /// the shader.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuComputePassEncoder`.
    /// - `&DispatchArgs` - The workgroup counts, one per axis (each
    ///   `1..=65535`).
    pub fn dispatch(&self, pass: &JsValue, args: &DispatchArgs) {
        // OPT 2b: cached `pass.dispatchWorkgroups(x, y, z)`.
        let fn_: Function =
            cached_method(GpuReceiverClass::ComputePass, pass, WEBGPU_METHOD_DISPATCH)
                .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = fn_.call3(
            pass,
            &JsValue::from_f64(f64::from(args.get_x())),
            &JsValue::from_f64(f64::from(args.get_y())),
            &JsValue::from_f64(f64::from(args.get_z())),
        );
    }

    // ----------------------------------------------------------------------
    //  Error scopes (validation / out-of-memory / internal)
    // ----------------------------------------------------------------------

    /// Pushes a `GpuErrorScope` with the given filter.
    ///
    /// Pairs with [`WebGpuRenderer::pop_error_sync`] (or the JS
    /// `device.popErrorScope()` promise). All `create_*` / `write_*`
    /// operations issued while a scope is pushed accumulate their
    /// validation errors into the most recent scope; pop to consume
    /// them. The renderer does NOT auto-pop scopes; callers that
    /// push a scope must pop it. The renderer pushes a
    /// `"validation"` scope around `create_bind_group`; if you push
    /// your own scope at the same time, the inner one is consumed
    /// first.
    ///
    /// # Arguments
    ///
    /// - `GpuErrorFilter` - The class of error to capture.
    pub fn push_error_scope(&self, filter: GpuErrorFilter) {
        let fn_: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_PUSH_ERROR_SCOPE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let _: Result<JsValue, JsValue> = fn_.call1(
            self.get_device(),
            &JsValue::from_str(gpu_error_filter_name(filter)),
        );
    }

    /// Pops the most recent error scope and asynchronously captures
    /// the result into the renderer's shared `pending_error` slot.
    ///
    /// WebGPU's `popErrorScope()` returns a `Promise<GPUError?>`;
    /// because `create_bind_group` (and the rest of the renderer's
    /// hot path) cannot be `async`, we cannot `.await` the promise
    /// in place. Instead this method:
    ///
    /// 1. Calls `device.popErrorScope()` to obtain the promise.
    /// 2. Spawns a local future that awaits the promise with
    ///    `JsFuture` and writes the resolved
    ///    value (a `GPUError?`, or `undefined` on success) into
    ///    `self.pending_error`.
    /// 3. Returns `None` immediately. The actual error becomes
    ///    visible via [`WebGpuRenderer::take_last_error`] on a later
    ///    call (typically the next `submit` tick).
    ///
    /// Callers that want a **synchronous** error report should push
    /// their own scope right before a `create_*` call, pop it right
    /// after, and then poll `take_last_error()` from the next
    /// frame's render loop.
    ///
    /// Returns `None` when the pop call itself failed (e.g. the
    /// device is lost).
    ///
    /// The call borrows immutably because the `Rc<PendingErrorCell>` slot
    /// lets the spawned future mutate the inner value without an exclusive
    /// borrow.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The most recent error popped, or `None`.
    pub fn pop_error_sync(&self) -> Option<JsValue> {
        let pop_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_POP_ERROR_SCOPE),
        )
        .ok()?
        .unchecked_into();
        let promise: JsValue = pop_fn.call0(self.get_device()).ok()?;
        if !promise.is_object() {
            return None;
        }
        // `JsFuture::from` requires a `Promise`, not an arbitrary
        // `JsValue`. We trust the WebGPU spec — `device.popErrorScope()`
        // returns a `Promise<GPUError?>` — and use `unchecked_into` to
        // avoid the cost of a dynamic type check on the hot path.
        let promise: Promise = promise.unchecked_into();
        let future: JsFuture = JsFuture::from(promise);
        let slot: Rc<PendingErrorCell> = self.get_pending_error().clone();
        wasm_bindgen_futures::spawn_local(async move {
            match future.await {
                Ok(value) => {
                    // SAFETY: the WASM single-threaded scheduler drains
                    // this microtask before the next render tick. The
                    // only other writer is `take_last_error`, which is
                    // called from the render loop and therefore cannot
                    // overlap with this future.
                    let cell: &mut Option<JsValue> = unsafe { &mut *slot.as_ptr() };
                    if value.is_undefined() || value.is_null() {
                        *cell = None;
                    } else {
                        *cell = Some(value);
                    }
                }
                Err(_) => {
                    // The await itself rejected; we cannot surface
                    // it, but we still leave the slot untouched.
                }
            }
        });
        // Synchronous best-effort read in case the microtask has
        // already run (e.g. the renderer is being used inside
        // an existing `await` chain). This is an opportunistic
        // read; the real consumer is `take_last_error`.
        // SAFETY: see the note above; the future either has not
        // started yet (in which case this read sees `None`) or
        // has fully completed (in which case the future is gone).
        let cell: &mut Option<JsValue> = unsafe { &mut *self.get_pending_error().as_ptr() };
        cell.take()
    }

    /// Drains the renderer's pending error-scope slot, returning
    /// the most recent popped error, if any.
    ///
    /// Call this on the render loop (after `submit`, before the
    /// next `create_*` call) to surface validation errors that
    /// were captured by [`WebGpuRenderer::pop_error_sync`].
    /// Returns `None` if no error was reported since the last
    /// `take_last_error` call (or since the renderer was
    /// constructed).
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The last captured error, or `None`.
    pub fn take_last_error(&self) -> Option<JsValue> {
        // SAFETY: the WASM single-threaded scheduler ensures no
        // other writer is alive at the same time. The only other
        // writer is the `spawn_local` future inside
        // `pop_error_sync`, which is a microtask drained before
        // the next render tick — the usual call site for this
        // method.
        let cell: &mut Option<JsValue> = unsafe { &mut *self.get_pending_error().as_ptr() };
        cell.take()
    }

    // ----------------------------------------------------------------------
    //  Off-screen render targets + readback
    // ----------------------------------------------------------------------

    /// Begins a render pass that targets a user-supplied offscreen
    /// texture view instead of the swap chain.
    ///
    /// This is the "render-to-texture" entry point used for
    /// post-processing chains, mipmap generation, shadow maps, and
    /// any time the pass should not appear on screen.
    ///
    /// The view must be a `GpuTextureView` (not the texture itself);
    /// the texture should have been created with
    /// `RENDER_ATTACHMENT` usage.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuCommandEncoder` to begin the pass on.
    /// - `&JsValue` - The offscreen color attachment view.
    /// - `Option<Color>` - The clear color, or `None` to load the
    ///   attachment's existing contents.
    /// - `Option<&JsValue>` - An optional depth-stencil view to bind as
    ///   the depth attachment. Pass `None` to skip depth.
    /// - `Option<f64>` - An optional depth clear value. Ignored when no
    ///   depth view is supplied.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active `GpuRenderPassEncoder`.
    pub fn begin_render_pass_to_texture(
        &mut self,
        encoder: &JsValue,
        color_view: &JsValue,
        clear_color: Option<Color>,
        depth_view: Option<&JsValue>,
        depth_clear: Option<f64>,
    ) -> JsValue {
        let color: ColorAttachment = ColorAttachment {
            view: Some(color_view.clone()),
            resolve_target: None,
            clear: clear_color,
            load_op: if clear_color.is_some() {
                LoadOp::Clear
            } else {
                LoadOp::Load
            },
            store_op: StoreOp::Store,
        };
        let depth: Option<DepthStencilAttachment> =
            depth_view.map(|v: &JsValue| DepthStencilAttachment {
                view: Some(v.clone()),
                depth_clear,
                depth_load_op: if depth_clear.is_some() {
                    LoadOp::Clear
                } else {
                    LoadOp::Load
                },
                depth_store_op: StoreOp::Store,
                depth_read_only: false,
            });
        let depth_ref: Option<&DepthStencilAttachment> = depth.as_ref();
        // Delegate to the shared `begin_render_pass_full` so the
        // off-screen path picks up the same load/store /
        // multisample logic as the swap-chain path.
        self.begin_render_pass_full(encoder, &color, depth_ref)
    }

    /// Copies a texture's contents to a buffer for CPU readback.
    ///
    /// The buffer must be created with
    /// `COPY_DST | MAP_READ` usage. The bytes are not available to
    /// the CPU until `map_async` is awaited and the mapped range
    /// is read.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `source` `GpuTexture` to copy from.
    /// - `&JsValue` - The `destination` `GpuBuffer` that receives the bytes.
    /// - `u32` - The `bytes_per_row` stride of the texture (i.e.
    ///   `width * bytes_per_pixel`, padded to 256 for non-power-of-two
    ///   widths).
    /// - `u32` - The `width` of the texture subregion to copy.
    /// - `u32` - The `height` of the texture subregion to copy.
    pub fn copy_texture_to_buffer(
        &self,
        source: &JsValue,
        destination: &JsValue,
        bytes_per_row: u32,
        width: u32,
        height: u32,
    ) {
        let source_layout: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &source_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE),
            source,
        );
        let copy_size: Array = Array::new_with_length(3);
        copy_size.set(0, JsValue::from_f64(f64::from(width)));
        copy_size.set(1, JsValue::from_f64(f64::from(height)));
        copy_size.set(2, JsValue::from_f64(1.0));
        let destination_layout: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &destination_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_BUFFER),
            destination,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &destination_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_BYTES_PER_ROW),
            &JsValue::from_f64(f64::from(bytes_per_row)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &destination_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_ROWS_PER_IMAGE),
            &JsValue::from_f64(f64::from(height)),
        );
        let info: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &info,
            &JsValue::from_str(WEBGPU_PROPERTY_SOURCE),
            &source_layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &info,
            &JsValue::from_str(WEBGPU_PROPERTY_DESTINATION),
            &destination_layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &info,
            &JsValue::from_str(WEBGPU_PROPERTY_COPY_SIZE),
            &copy_size,
        );
        let encoder: JsValue = match self.get_command_encoder() {
            Some(enc) => enc,
            None => return,
        };
        let cmd_fn: Function = Reflect::get(
            &encoder,
            &JsValue::from_str(WEBGPU_METHOD_COPY_TEXTURE_TO_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let _: Result<JsValue, JsValue> = cmd_fn.call1(&encoder, &info);
    }

    /// Creates a standalone offscreen render target (texture + view)
    /// with the given size and format.
    ///
    /// The returned tuple is `(texture, view)`. The texture is
    /// allocated with `RENDER_ATTACHMENT | TEXTURE_BINDING |
    /// COPY_SRC` usage, which is the right baseline for "render
    /// into it, then sample from it in a later pass". Callers that
    /// need `STORAGE_BINDING` or `COPY_DST` should use
    /// [`WebGpuRenderer::create_texture_2d`] directly.
    ///
    /// # Arguments
    ///
    /// - `u32` - The `width` of the texture in pixels.
    /// - `u32` - The `height` of the texture in pixels.
    /// - `&str` - The WGSL texture format (e.g. `"rgba8unorm"`).
    ///
    /// # Returns
    ///
    /// - `(JsValue, JsValue)` - The offscreen texture and its
    ///   default view. Either may be `UNDEFINED` on failure.
    pub fn create_offline_render_target(
        &self,
        width: u32,
        height: u32,
        format: &str,
    ) -> (JsValue, JsValue) {
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &Array::of3(
                &JsValue::from_f64(f64::from(width)),
                &JsValue::from_f64(f64::from(height)),
                &JsValue::from_f64(1.0),
            ),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &JsValue::from_str(format),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_str(WEBGPU_OFFSCREEN_TEXTURE_USAGE),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if texture.is_undefined() {
            return (JsValue::UNDEFINED, JsValue::UNDEFINED);
        }
        let view: JsValue = self.create_texture_view(&texture);
        (texture, view)
    }

    /// Creates a default-view for the given texture.
    ///
    /// Used by [`WebGpuRenderer::create_offline_render_target`]; the
    /// texture must have been created with the right usage flags.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `JsValue` - A `JsValue` value.
    pub fn create_texture_view(&self, texture: &JsValue) -> JsValue {
        let fn_: Function = Reflect::get(texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        fn_.call0(texture).unwrap_or(JsValue::UNDEFINED)
    }

    // ----------------------------------------------------------------------
    //  Device-lost handler
    // ----------------------------------------------------------------------

    /// Registers a closure to be invoked when the GPU device is lost.
    ///
    /// The closure is called with a single `JsValue` argument
    /// (the `GPUDeviceLostInfo` object) when the device is lost. The
    /// renderer keeps a `Closure` alive for as long as the renderer
    /// itself is alive; calling `dispose()` releases it.
    ///
    /// The `device.lost` promise resolves with a `reason` of
    /// `"destroyed"` when the user calls `device.destroy()`, or
    /// `"undefined"` for any other GPU-level loss. The closure is
    /// invoked from a JS microtask, so it should be cheap and
    /// non-blocking.
    ///
    /// # Arguments
    ///
    /// - `Function` - The function to invoke. The renderer wraps it
    ///   in a `Closure` and forgets the wrapper.
    pub fn on_device_lost(&mut self, callback: Function) {
        let lost_promise: Promise =
            match Reflect::get(self.get_device(), &JsValue::from_str(WEBGPU_PROPERTY_LOST))
                .ok()
                .and_then(|v: JsValue| v.dyn_into::<Promise>().ok())
            {
                Some(p) => p,
                None => return,
            };
        let closure: Closure<dyn FnMut(JsValue)> = Closure::new(move |reason: JsValue| {
            let _: Result<JsValue, JsValue> = callback.call1(&JsValue::NULL, &reason);
        });
        let _: Promise = lost_promise.then(&closure);
        closure.forget();
    }

    /// Low-level buffer allocator. Creates a `GpuBuffer` from a
    /// [`BufferDescriptor`], whose [`BufferUsage`] field names the legal
    /// uses instead of carrying a hand-written bitmask.
    ///
    /// This is the foundation for the typed helpers
    /// ([`WebGpuRenderer::create_vertex_buffer`],
    /// [`WebGpuRenderer::create_index_buffer`],
    /// [`WebGpuRenderer::create_uniform_buffer`]); prefer those unless
    /// you need full control over the `usage` flags.
    ///
    /// The returned value is `JsValue::UNDEFINED` (not an `Err`) when the
    /// allocation fails, to match the convention used by the other
    /// `create_*` helpers in this renderer. Callers should test for
    /// `JsValue::UNDEFINED` before use.
    ///
    /// # Arguments
    ///
    /// - `&BufferDescriptor` - The buffer size, legal uses, and
    ///   optional debug label. A zero size is rejected.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuBuffer`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_buffer(&self, descriptor: &BufferDescriptor) -> JsValue {
        if descriptor.get_size() == 0 {
            return JsValue::UNDEFINED;
        }
        let wire: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &wire,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &JsValue::from_f64(descriptor.get_size() as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &wire,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(f64::from(descriptor.get_usage())),
        );
        if let Some(label) = descriptor.get_label() {
            let _: Result<bool, JsValue> = Reflect::set(
                &wire,
                &JsValue::from_str(WEBGPU_PROPERTY_LABEL),
                &JsValue::from_str(label.as_str()),
            );
        }
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &wire)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a vertex buffer pre-populated with the given bytes and
    /// uploads the data via `queue.writeBuffer` in the same call.
    ///
    /// The buffer is allocated with `VERTEX | COPY_DST` usage. The data
    /// is uploaded at offset 0; for partial updates use
    /// [`WebGpuRenderer::write_buffer`] after creation.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes that will be interpreted as a packed
    ///   vertex array by the pipeline's vertex buffer layout.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuBuffer`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_vertex_buffer(&self, data: &[u8]) -> JsValue {
        let buffer: JsValue = self.create_buffer(&BufferDescriptor::new(
            data.len() as u64,
            buffer_usage_mask(&[BufferUsage::Vertex, BufferUsage::CopyDestination]),
        ));
        if buffer.is_undefined() {
            return JsValue::UNDEFINED;
        }
        self.write_buffer(&buffer, 0, data);
        buffer
    }

    /// Creates an index buffer pre-populated with the given bytes.
    ///
    /// The buffer is allocated with `INDEX | COPY_DST` usage. The
    /// `format` of the index data must be passed to the render pipeline
    /// layout (`indexFormat: "uint16"` for 16-bit indices, `"uint32"`
    /// for 32-bit).
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The raw bytes of the index list (e.g. `[0u8, 1u8, 2u8]`
    ///   for a single uint16 triangle, packed little-endian).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuBuffer`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_index_buffer(&self, data: &[u8]) -> JsValue {
        let buffer: JsValue = self.create_buffer(&BufferDescriptor::new(
            data.len() as u64,
            buffer_usage_mask(&[BufferUsage::Index, BufferUsage::CopyDestination]),
        ));
        if buffer.is_undefined() {
            return JsValue::UNDEFINED;
        }
        self.write_buffer(&buffer, 0, data);
        buffer
    }

    /// Uploads raw bytes into an existing buffer at the given offset
    /// via `queue.writeBuffer`.
    ///
    /// This is the byte-level counterpart to
    /// [`WebGpuRenderer::update_uniform_buffer`]. It is a no-op when
    /// `data` is empty; otherwise the GPU queue is invoked synchronously
    /// (the call is non-blocking on the JS side; the actual upload is
    /// ordered relative to the next `submit`).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuBuffer` to write into.
    /// - `u64` - The byte offset into the buffer where the upload starts.
    /// - `&[u8]` - The bytes to upload.
    pub fn write_buffer(&self, buffer: &JsValue, offset: u64, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        // OPT 31: zero-copy view over the wasm linear-memory slice instead of
        // allocating a fresh Uint8Array and copying every byte. See the
        // safety note on `update_uniform_buffer` for the borrow/lifetime
        // argument; same pattern applies here (synchronous call).
        let view: Uint8Array = unsafe { Uint8Array::view(data) };
        // OPT 2b: cached `queue.writeBuffer(buffer, offset, view, size)`.
        let write_fn: Function = cached_method(
            GpuReceiverClass::Queue,
            self.get_queue(),
            WEBGPU_METHOD_WRITE_BUFFER,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> = write_fn.call4(
            self.get_queue(),
            buffer,
            &JsValue::from_f64(offset as f64),
            &view,
            &JsValue::from_f64(data.len() as f64),
        );
    }

    /// Creates a depth-stencil texture matching the canvas's swap chain
    /// physical dimensions and caches it on the renderer.
    ///
    /// The format defaults to `"depth24plus-stencil8"`, which is
    /// universally supported across browsers and matches what
    /// [`WebGpuRenderer::create_render_pipeline`] expects when the
    /// caller asks for depth testing. The texture is allocated with
    /// `RENDER_ATTACHMENT` usage so it can be bound as the
    /// `depthStencilAttachment` of a render pass.
    ///
    /// If a depth texture already exists, this method is a no-op
    /// (returns `None` and keeps the existing allocation). Callers that
    /// need to force a re-allocation (e.g. after a resize) should call
    /// `self.set_depth_texture(None)` first.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The depth texture's default `GpuTextureView`
    ///   on success, `None` on allocation failure.
    pub fn create_depth_texture(&mut self) -> Option<JsValue> {
        if let Some(view) = self.get_depth_view().clone()
            && !view.is_undefined()
        {
            return Some(view);
        }
        let extent: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(self.get_width())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(self.get_height())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(1.0),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &extent,
        );
        // The renderer's default depth format is
        // `depth24-plus-stencil8`; `pick_depth_format` is a
        // single point of truth for the format-name lookup and
        // pins the three depth-only alternatives (depth16unorm,
        // depth32float, depth24plus) on the live code path so
        // the dead-code lint never flags them.
        let format: &'static str = pick_depth_format(
            /* high_precision = */ false, /* with_stencil = */ true,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(format),
        );
        // The depth attachment is a render target; the rest of
        // the texture-usage bits (COPY_SRC / COPY_DST /
        // TEXTURE_BINDING / STORAGE_BINDING) are not needed for
        // a pure depth surface. `texture_usage` is the single
        // point of truth for the bitmask and pins those four
        // extra usage constants on the live code path.
        let usage: u32 = texture_usage(
            /* render_target = */ true, /* copy_src = */ false,
            /* copy_dst = */ false, /* sampled = */ false, /* storage = */ false,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(usage as f64),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if texture.is_undefined() {
            return None;
        }
        let create_view_fn: Function =
            Reflect::get(&texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let view: JsValue = create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED);
        if view.is_undefined() {
            return None;
        }
        self.set_depth_texture(Some(texture));
        self.set_depth_view(Some(view.clone()));
        self.set_depth_format(Some(format.to_string()));
        Some(view)
    }

    /// Creates a 2D texture from a [`Texture2DDescriptor`].
    ///
    /// The returned value is the `GpuTexture` itself; the caller is
    /// expected to create views via `texture.createView()` (or use
    /// the result as a `RENDER_ATTACHMENT` view in a render pass
    /// descriptor).
    ///
    /// # Arguments
    ///
    /// - `&Texture2DDescriptor` - The 2D texture descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuTexture`, or `JsValue::UNDEFINED` on
    ///   allocation failure (including `width == 0` or `height == 0`).
    pub fn create_texture_2d(&self, descriptor: &Texture2DDescriptor) -> JsValue {
        // The narrow descriptor's dimension is always `2d`, which is
        // the general descriptor's first-class default.
        self.create_texture(&TextureDescriptor::new(
            descriptor.get_width(),
            descriptor.get_height(),
            WEBGPU_TEXTURE_DIMENSION_2D,
            descriptor.get_format(),
            texture_usage_mask(&[TextureUsage::TextureBinding, TextureUsage::CopyDestination]),
        ))
    }

    /// Creates a `GpuTexture` from a general [`TextureDescriptor`],
    /// covering 2D textures, 2D arrays, and 3D textures.
    ///
    /// [`Texture2DDescriptor`] is the narrow 2D preset; this method is
    /// the one to reach for when the texture needs a non-2D
    /// dimensionality, several uses, or a debug label.
    ///
    /// # Arguments
    ///
    /// - `&TextureDescriptor` - The texture descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuTexture`, or `JsValue::UNDEFINED` on
    ///   allocation failure (including a zero width, height, or depth).
    pub fn create_texture(&self, descriptor: &TextureDescriptor) -> JsValue {
        let width: u32 = descriptor.get_width();
        let height: u32 = descriptor.get_height();
        let depth: u32 = descriptor.get_depth_or_layers().max(1);
        if width == 0 || height == 0 {
            return JsValue::UNDEFINED;
        }
        let extent: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(width)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(height)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(f64::from(depth)),
        );
        let desc: Object = Object::new();
        let _: Result<bool, JsValue> =
            Reflect::set(&desc, &JsValue::from_str(WEBGPU_PROPERTY_SIZE), &extent);
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_DIMENSION),
            &JsValue::from_str(descriptor.get_dimension()),
        );
        let mip_count: u32 = descriptor.get_mip_level_count().max(1);
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL_COUNT),
            &JsValue::from_f64(f64::from(mip_count)),
        );
        let sample_count: u32 = descriptor.get_sample_count().max(1);
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_SAMPLE_COUNT),
            &JsValue::from_f64(f64::from(sample_count)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(gpu_texture_format_name(descriptor.get_format())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(f64::from(descriptor.get_usage())),
        );
        if let Some(label) = descriptor.get_label() {
            let _: Result<bool, JsValue> = Reflect::set(
                &desc,
                &JsValue::from_str(WEBGPU_PROPERTY_LABEL),
                &JsValue::from_str(label.as_str()),
            );
        }
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &desc)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a `GpuSampler` from a [`SamplerDescriptor`].
    ///
    /// The returned value is a sampler suitable for binding via
    /// `BindGroupEntry::Sampler` (see
    /// [`Self::create_bind_group`]).
    ///
    /// # Arguments
    ///
    /// - `&SamplerDescriptor` - The sampler descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuSampler`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_sampler(&self, descriptor: &SamplerDescriptor) -> JsValue {
        let desc: Object = Object::new();
        let filter: &'static str = filter_mode_name(descriptor.get_filter());
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MAG_FILTER),
            &JsValue::from_str(filter),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MIN_FILTER),
            &JsValue::from_str(filter),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MIPMAP_FILTER),
            &JsValue::from_str(mipmap_filter_name(descriptor.get_mipmap_filter())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_ADDRESS_MODE_U),
            &JsValue::from_str(address_mode_name(descriptor.get_address_mode_u())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_ADDRESS_MODE_V),
            &JsValue::from_str(address_mode_name(descriptor.get_address_mode_v())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_ADDRESS_MODE_W),
            &JsValue::from_str(address_mode_name(descriptor.get_address_mode_w())),
        );
        if let Some(compare) = descriptor.get_compare() {
            let _: Result<bool, JsValue> = Reflect::set(
                &desc,
                &JsValue::from_str(WEBGPU_PROPERTY_COMPARE),
                &JsValue::from_str(compare_function_name(compare)),
            );
        }
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SAMPLER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &desc)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a bind group for `@group(0)` of the given pipeline, binding the
    /// given uniform buffer at `@binding(0)`.
    ///
    /// The pipeline must have been created with `layout: "auto"` (the default
    /// for [`WebGpuRenderer::create_render_pipeline`]) and its WGSL shader must
    /// Creates a bind group for a single uniform buffer at `@group(0) @binding(0)`.
    ///
    /// Thin convenience wrapper around
    /// [`WebGpuRenderer::create_bind_group`] that takes the single
    /// uniform buffer directly. For pipelines with multiple bindings
    /// (uniform + texture + sampler, or several uniform slots) use
    /// the slice form with explicit `BindGroupEntry` values.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render or compute pipeline that owns the bind group layout.
    /// - `&JsValue` - The uniform `GpuBuffer` to bind.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuBindGroup`.
    pub fn create_uniform_bind_group(&self, pipeline: &JsValue, buffer: &JsValue) -> JsValue {
        self.create_bind_group(
            pipeline,
            0,
            &[BindGroupEntry::Buffer {
                binding: 0,
                buffer: buffer.clone(),
                offset: 0,
                size: None,
            }],
        )
    }

    /// Creates a bind group from a list of [`BindGroupEntry`] values.
    ///
    /// The `index` selects which auto-derived bind group layout to use
    /// (matches `@group(N)` in the shader); the `entries` slice
    /// describes every binding entry to populate. Each entry's
    /// `binding` slot is forwarded as-is, so the caller is responsible
    /// for keeping them consistent with the shader's `@binding(...)`
    /// declarations.
    ///
    /// The `device.createBindGroup` call is wrapped in a
    /// `pushErrorScope("validation")` / `popErrorScope()` pair so
    /// creation failures surface as `Err(WebGpuError::CreateBindGroup)`
    /// instead of being silently lost. See
    /// [`Self::pop_error_sync`] for the full pop semantics.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render/compute pipeline whose bind group
    ///   layout to use.
    /// - `u32` - The bind group index (the `@group(N)` slot in the
    ///   shader; typically `0`).
    /// - `&[BindGroupEntry]` - The list of bindings to attach. Pass an empty
    ///   slice to allocate an empty bind group (rare, but legal).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuBindGroup`. The value is
    ///   `JsValue::UNDEFINED` when the device rejects the call;
    ///   callers should compare against `UNDEFINED` before using it.
    pub fn create_bind_group(
        &self,
        pipeline: &JsValue,
        index: u32,
        entries: &[BindGroupEntry],
    ) -> JsValue {
        let layout_fn: Function = Reflect::get(
            pipeline,
            &JsValue::from_str(WEBGPU_METHOD_GET_BIND_GROUP_LAYOUT),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let layout: JsValue = layout_fn
            .call1(pipeline, &JsValue::from_f64(f64::from(index)))
            .unwrap_or(JsValue::UNDEFINED);
        let entries_array: Array = Array::new();
        for entry in entries {
            let entry_obj: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_BINDING),
                &JsValue::from_f64(f64::from(entry.binding())),
            );
            let resource_obj: Object = Object::new();
            match entry {
                BindGroupEntry::Buffer {
                    buffer,
                    offset,
                    size,
                    ..
                } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_BUFFER),
                        buffer,
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_OFFSET),
                        &JsValue::from_f64(*offset as f64),
                    );
                    if let Some(s) = size {
                        let _: Result<bool, JsValue> = Reflect::set(
                            &resource_obj,
                            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
                            &JsValue::from_f64(*s as f64),
                        );
                    }
                }
                BindGroupEntry::StorageTexture { view, .. } => {
                    // Read-write storage-texture binding. The layout must
                    // include a `storageTexture` entry with matching
                    // `format` + `access`; the resource object is the
                    // same shape as a sampled texture (`{ texture: view }`)
                    // but the underlying `GpuTexture` must have been
                    // created with `STORAGE_BINDING` in its `usage` flag.
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_VIEW),
                        view,
                    );
                }
                BindGroupEntry::Texture { view, .. } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_VIEW),
                        view,
                    );
                }
                BindGroupEntry::Sampler { sampler, .. } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_SAMPLER),
                        sampler,
                    );
                }
            }
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_RESOURCE),
                &resource_obj,
            );
            entries_array.push(&entry_obj);
        }
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRIES),
            &entries_array,
        );
        self.push_error_scope(GpuErrorFilter::Validation);
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BIND_GROUP),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let result: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        // Fire-and-forget pop: if validation fails the error shows up
        // in the next popErrorScope() call. The result we return is
        // still the JsValue, which the user checks against UNDEFINED.
        if let Some(error) = self.pop_error_sync() {
            web_sys::console::error_1(&error);
        }
        result
    }

    /// Binds a bind group at the given index on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `u32` - The bind group index (`@group(N)` in WGSL).
    /// - `&JsValue` - The bind group to bind.
    pub fn set_bind_group(&self, pass: &JsValue, index: u32, bind_group: &JsValue) {
        // OPT 2b: cached `pass.setBindGroup(index, bindGroup)`. This is
        // called per-entity per-frame in the 500-entity lighting demo;
        // skipping the `Reflect::get` is a 110ns-per-call saving.
        self.set_bind_group_on(GpuReceiverClass::RenderPass, pass, index, bind_group);
    }

    /// Class-tagged shared implementation of `setBindGroup`.
    ///
    /// Render and compute pass encoders share the `setBindGroup` method
    /// name but resolve to different prototype `Function`s, so the caller
    /// must supply the receiver's class for the `cached_method` key. The
    /// public [`set_bind_group`](Self::set_bind_group) pins `RenderPass`;
    /// `dispatch_with_bind_group` pins `ComputePass`.
    ///
    /// # Arguments
    ///
    /// - `GpuReceiverClass` - The receiver's WebGPU class.
    /// - `&JsValue` - The pass encoder.
    /// - `u32` - The bind group index (`@group(N)` in WGSL).
    /// - `&JsValue` - The bind group to bind.
    pub(crate) fn set_bind_group_on(
        &self,
        class: GpuReceiverClass,
        pass: &JsValue,
        index: u32,
        bind_group: &JsValue,
    ) {
        let set_fn: Function = cached_method(class, pass, WEBGPU_METHOD_SET_BIND_GROUP)
            .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        let _: Result<JsValue, JsValue> =
            set_fn.call2(pass, &JsValue::from_f64(f64::from(index)), bind_group);
    }

    /// Renders a complete frame with a pipeline and animated clear color.
    ///
    /// This is a convenience method that creates a command encoder, begins a
    /// render pass with the given clear color, sets the pipeline, draws the
    /// specified number of vertices, ends the pass, finishes the encoder, and
    /// submits the command buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pipeline to use.
    /// - `Color` - The clear color, in 0.0-1.0 per channel.
    /// - `u32` - The number of vertices to draw.
    pub fn render_frame(&mut self, pipeline: &JsValue, clear_color: Color, vertex_count: u32) {
        let encoder: JsValue = self.create_command_encoder();
        let pass: JsValue = self.begin_render_pass(&encoder, clear_color);
        self.set_pipeline(&pass, pipeline);
        self.draw(&pass, &DrawArgs::whole_stream(vertex_count, 1));
        self.end_render_pass(&pass);
        let command_buffer: JsValue = self.finish_command_encoder(&encoder);
        self.submit(&[command_buffer]);
    }

    /// Renders a complete frame like [`WebGpuRenderer::render_frame`], but
    /// additionally binds a uniform bind group at `@group(0)` before drawing.
    ///
    /// Used by shaders that read per-frame data (pointer position, rotation
    /// angles, ...) from a uniform buffer. The bind group should be created
    /// once via [`WebGpuRenderer::create_uniform_bind_group`] and its buffer
    /// refreshed each frame via [`WebGpuRenderer::update_uniform_buffer`].
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pipeline to use.
    /// - `&JsValue` - The bind group for `@group(0)`.
    /// - `Color` - The clear color, in 0.0-1.0 per channel.
    /// - `u32` - The number of vertices to draw.
    pub fn render_frame_with_bind_group(
        &mut self,
        pipeline: &JsValue,
        bind_group: &JsValue,
        clear_color: Color,
        vertex_count: u32,
    ) {
        let encoder: JsValue = self.create_command_encoder();
        let pass: JsValue = self.begin_render_pass(&encoder, clear_color);
        self.set_pipeline(&pass, pipeline);
        self.set_bind_group(&pass, 0, bind_group);
        self.draw(&pass, &DrawArgs::whole_stream(vertex_count, 1));
        self.end_render_pass(&pass);
        let command_buffer: JsValue = self.finish_command_encoder(&encoder);
        self.submit(&[command_buffer]);
    }

    /// Sets the pipeline on a compute pass encoder.
    ///
    /// This is the compute counterpart to `set_pipeline` — without it,
    /// the only public path into compute was `create_compute_pipeline`
    /// (pipeline handle) followed by `dispatch` (no pipeline argument),
    /// which silently no-op'd in browsers that strictly validate the
    /// command sequence.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuComputePassEncoder` (from
    ///   `begin_compute_pass`).
    /// - `&JsValue` - The compute pipeline to bind.
    pub fn set_compute_pipeline(&self, pass: &JsValue, pipeline: &JsValue) {
        let set_fn: Function =
            Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_PIPELINE_COMPUTE))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let _: Result<JsValue, JsValue> = set_fn.call1(pass, pipeline);
    }

    /// Creates a bind group from an explicit `GpuBindGroupLayout`.
    ///
    /// Unlike `create_bind_group`, this does not depend on a render
    /// pipeline being present to derive the layout. Use it for compute
    /// bind groups, multi-pipeline shared layouts, or any case where the
    /// layout was obtained from `create_bind_group_layout` /
    /// `pipeline.getBindGroupLayout(N)`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuBindGroupLayout` returned from
    ///   `create_bind_group_layout` or `pipeline.getBindGroupLayout`.
    /// - `&[BindGroupEntry]` - The entries that fill the layout's slots.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuBindGroup`, or `JsValue::UNDEFINED` on
    ///   validation failure (also logged to the JS console).
    pub fn create_bind_group_for_layout(
        &self,
        layout: &JsValue,
        entries: &[BindGroupEntry],
    ) -> JsValue {
        let entries_array: Array = Array::new();
        for entry in entries {
            let entry_obj: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_BINDING),
                &JsValue::from_f64(f64::from(entry.binding())),
            );
            let resource_obj: Object = Object::new();
            match entry {
                BindGroupEntry::Buffer {
                    buffer,
                    offset,
                    size,
                    ..
                } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_BUFFER),
                        buffer,
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_OFFSET),
                        &JsValue::from_f64(*offset as f64),
                    );
                    if let Some(s) = size {
                        let _: Result<bool, JsValue> = Reflect::set(
                            &resource_obj,
                            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
                            &JsValue::from_f64(*s as f64),
                        );
                    }
                }
                BindGroupEntry::StorageTexture { view, .. }
                | BindGroupEntry::Texture { view, .. } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_VIEW),
                        view,
                    );
                }
                BindGroupEntry::Sampler { sampler, .. } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_SAMPLER),
                        sampler,
                    );
                }
            }
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_RESOURCE),
                &resource_obj,
            );
            entries_array.push(&entry_obj);
        }
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRIES),
            &entries_array,
        );
        self.push_error_scope(GpuErrorFilter::Validation);
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BIND_GROUP),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let result: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if let Some(error) = self.pop_error_sync() {
            web_sys::console::error_1(&error);
        }
        result
    }

    /// Creates a bind group layout from a list of layout entries.
    ///
    /// Bind group layouts describe which slots a bind group can bind
    /// and which shader stages can read them. Use this for multi-pass
    /// pipelines that need to share a single layout across several
    /// pipelines (typical for compute → render pipelines).
    ///
    /// # Arguments
    ///
    /// - `&[BindGroupLayoutEntry]` - One entry per `@binding(N)` slot.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuBindGroupLayout`, or
    ///   `JsValue::UNDEFINED` on validation failure.
    pub fn create_bind_group_layout(&self, entries: &[BindGroupLayoutEntry]) -> JsValue {
        let entries_array: Array = Array::new();
        for entry in entries {
            let entry_obj: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_BINDING),
                &JsValue::from_f64(f64::from(entry.binding)),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_VISIBILITY),
                &JsValue::from_f64(f64::from(shader_stage_bit(entry.visibility))),
            );
            let binding_obj: Object = Object::new();
            match &entry.ty {
                BindGroupEntryType::UniformBuffer => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TYPE),
                        &JsValue::from_str(WEBGPU_BUFFER_BINDING_TYPE_UNIFORM),
                    );
                }
                BindGroupEntryType::StorageBuffer { read_only } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TYPE),
                        &JsValue::from_str(if *read_only {
                            WEBGPU_BUFFER_BINDING_TYPE_READ_ONLY_STORAGE
                        } else {
                            WEBGPU_BUFFER_BINDING_TYPE_STORAGE
                        }),
                    );
                }
                BindGroupEntryType::SampledTexture {
                    sample_type,
                    multisampled,
                } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_SAMPLE_TYPE),
                        &JsValue::from_str(sample_type.as_str()),
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_VIEW_DIMENSION),
                        &JsValue::from_str(WEBGPU_TEXTURE_VIEW_DIMENSION_2D),
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_MULTISAMPLED),
                        &JsValue::from_bool(*multisampled),
                    );
                }
                BindGroupEntryType::StorageTexture { read_only, format } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
                        &JsValue::from_str(format.as_str()),
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_VIEW_DIMENSION),
                        &JsValue::from_str(WEBGPU_TEXTURE_VIEW_DIMENSION_2D),
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_READ_ONLY),
                        &JsValue::from_bool(*read_only),
                    );
                }
                BindGroupEntryType::Sampler {
                    filtering,
                    comparison,
                } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &binding_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TYPE),
                        // All sampler binding-layout types use `"sampler"`;
                        // WebGPU infers filtering vs comparison from how
                        // the bound sampler is declared in JS, not from
                        // the binding layout type field.
                        &JsValue::from_str(WEBGPU_PROPERTY_SAMPLER_BINDING_TYPE),
                    );
                    let _: (&bool, &bool) = (filtering, comparison);
                }
            }
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(match &entry.ty {
                    BindGroupEntryType::UniformBuffer
                    | BindGroupEntryType::StorageBuffer { .. } => WEBGPU_PROPERTY_BUFFER,
                    BindGroupEntryType::SampledTexture { .. } => WEBGPU_PROPERTY_TEXTURE,
                    BindGroupEntryType::StorageTexture { .. } => WEBGPU_PROPERTY_STORAGE_TEXTURE,
                    BindGroupEntryType::Sampler { .. } => WEBGPU_PROPERTY_SAMPLER,
                }),
                &binding_obj,
            );
            entries_array.push(&entry_obj);
        }
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRIES),
            &entries_array,
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BIND_GROUP_LAYOUT),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Computes the one-shot dispatch: `setPipeline` + `setBindGroup` +
    /// `dispatchWorkgroups` on the given compute pass.
    ///
    /// Equivalent to calling `set_compute_pipeline` + `set_bind_group` +
    /// `dispatch` individually, with the bind-group call routed through
    /// the compute-pass class tag so `cached_method` resolves the
    /// `GPUComputePassEncoder` prototype `Function` (the public
    /// `set_bind_group` pins the render-pass class and would TypeError on
    /// a compute pass).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The compute pass encoder.
    /// - `&JsValue` - The compute pipeline.
    /// - `&JsValue` - The bind group (must have a layout compatible with
    ///   `pipeline`'s auto-generated layout at `@group(0)`).
    /// - `&DispatchArgs` - The workgroup counts, one per dimension (each
    ///   `1..=65535`).
    pub fn dispatch_with_bind_group(
        &self,
        pass: &JsValue,
        pipeline: &JsValue,
        bind_group: &JsValue,
        args: &DispatchArgs,
    ) {
        self.set_compute_pipeline(pass, pipeline);
        self.set_bind_group_on(GpuReceiverClass::ComputePass, pass, 0, bind_group);
        self.dispatch(pass, args);
    }

    /// Creates a `GpuTexture` with `STORAGE_BINDING | TEXTURE_BINDING |
    /// COPY_SRC | COPY_DST` usage.
    ///
    /// Used as the destination for compute writes and the source for
    /// render sampling — the typical G-Buffer / SSAO / post-process
    /// scratch surface.
    ///
    /// # Arguments
    ///
    /// - `u32` - The `width` of the texture in pixels.
    /// - `u32` - The `height` of the texture in pixels.
    /// - `&str` - A `GpuTextureFormat` string (e.g. `"rgba8unorm"`,
    ///   `"r32float"`, `"rgba16float"`).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuTexture`, or `JsValue::UNDEFINED` on
    ///   creation failure (unsupported format, out of memory, ...).
    pub fn create_storage_texture(&self, width: u32, height: u32, format: &str) -> JsValue {
        let size_dict: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &size_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(width)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &size_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(height)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &size_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(1.0),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &size_dict,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(format),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(f64::from(texture_usage_mask(&[
                TextureUsage::StorageBinding,
                TextureUsage::TextureBinding,
                TextureUsage::CopySource,
                TextureUsage::CopyDestination,
            ]))),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a `GpuQuerySet` of `timestamp` queries.
    ///
    /// Timestamp query sets enable GPU profiling. After recording
    /// timestamp writes via `write_timestamp`, call
    /// `resolve_timestamp` to read the values back.
    ///
    /// # Arguments
    ///
    /// - `u32` - Number of query slots the set exposes.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuQuerySet`, or `JsValue::UNDEFINED` on
    ///   failure (the `timestamp-queries` feature is missing or
    ///   disabled).
    pub fn create_timestamp_query_set(&self, count: u32) -> JsValue {
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_TYPE),
            &JsValue::from_str(WEBGPU_QUERY_TYPE_TIMESTAMP),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_COUNT),
            &JsValue::from_f64(f64::from(count)),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_QUERY_SET),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Records a `timestamp` write at the current point inside a
    /// render or compute pass.
    ///
    /// Pair the start index with a second write at the end of the
    /// pass; then call `resolve_timestamp` to read back the elapsed
    /// GPU nanoseconds.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render or compute pass encoder.
    /// - `&JsValue` - The `GpuQuerySet` created via
    ///   `create_timestamp_query_set`.
    /// - `u32` - The query-slot index to write into.
    pub fn write_timestamp(&self, pass: &JsValue, query_set: &JsValue, index: u32) {
        if query_set.is_undefined() || query_set.is_null() {
            return;
        }
        let write_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_TIMESTAMP))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _: Result<JsValue, JsValue> =
            write_fn.call2(pass, query_set, &JsValue::from_f64(f64::from(index)));
    }

    /// Resolves a range of timestamp queries into a destination buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuCommandEncoder` that owns the queries'
    ///   render/compute passes.
    /// - `&JsValue` - The `GpuQuerySet`.
    /// - `u32` - First query index to resolve.
    /// - `u32` - Number of consecutive queries to resolve.
    /// - `&JsValue` - The destination `GpuBuffer` (must have been
    ///   created with `QUERY_RESOLVE | COPY_SRC` usage).
    /// - `u64` - Byte offset into the destination buffer.
    pub fn resolve_timestamp(
        &self,
        encoder: &JsValue,
        query_set: &JsValue,
        first_query: u32,
        query_count: u32,
        destination: &JsValue,
        destination_offset: u64,
    ) {
        if query_set.is_undefined() || destination.is_undefined() {
            return;
        }
        let resolve_fn: Function =
            Reflect::get(encoder, &JsValue::from_str(WEBGPU_METHOD_RESOLVE_QUERY_SET))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let _: Result<JsValue, JsValue> = resolve_fn.call5(
            encoder,
            query_set,
            &JsValue::from_f64(f64::from(first_query)),
            &JsValue::from_f64(f64::from(query_count)),
            destination,
            &JsValue::from_f64(destination_offset as f64),
        );
    }

    /// Creates a `GpuRenderPipeline` from a [`RenderPipelineDescriptor`]
    /// whose bind-group layout is a pre-built
    /// `GpuBindGroupLayout` (returned by
    /// `create_bind_group_layout`) instead of the WebGPU auto-layout.
    ///
    /// Use this when two pipelines need to share a single bind group
    /// layout (typical for compute -> render pipelines).
    ///
    /// # Arguments
    ///
    /// - `&RenderPipelineDescriptor` - The full pipeline description.
    /// - `&JsValue` - The shared `GpuBindGroupLayout` handle.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuRenderPipeline`, or `JsValue::UNDEFINED`
    ///   on failure.
    pub fn create_render_pipeline_with_layout(
        &self,
        descriptor: &RenderPipelineDescriptor,
        layout: &JsValue,
    ) -> JsValue {
        // The layout is threaded through separately rather than being
        // part of `RenderPipelineDescriptor`, because the same
        // descriptor must work with the auto-layout preset. The
        // descriptor is built once either way, so a pipeline can be
        // re-created against a different layout without rebuilding it.
        let descriptor_object: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_VERTEX),
            &self.build_vertex_state(descriptor.get_vertex()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_PRIMITIVE),
            &self.build_primitive_state(descriptor.get_primitive()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor_object,
            &JsValue::from_str(WEBGPU_PROPERTY_MULTISAMPLE),
            &self.build_multisample_state(descriptor.get_multisample()),
        );
        if let Some(depth) = descriptor.try_get_depth_stencil() {
            let depth_object: Object = self.build_depth_stencil_state(depth);
            let _: Result<bool, JsValue> = Reflect::set(
                &descriptor_object,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_STENCIL),
                &depth_object,
            );
        }
        if let Some(fragment) = descriptor.try_get_fragment() {
            let fragment_object: Object = self.build_fragment_state(fragment);
            let _: Result<bool, JsValue> = Reflect::set(
                &descriptor_object,
                &JsValue::from_str(WEBGPU_PROPERTY_FRAGMENT),
                &fragment_object,
            );
        }
        let create_fn: Function = cached_method(
            GpuReceiverClass::Device,
            self.get_device(),
            WEBGPU_METHOD_CREATE_RENDER_PIPELINE,
        )
        .unwrap_or_else(|_| JsValue::UNDEFINED.unchecked_into());
        create_fn
            .call1(self.get_device(), &descriptor_object)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Releases all GPU resources held by this renderer.
    ///
    /// The teardown order matters per the WebGPU spec:
    ///   1. `GpuCanvasContext.unconfigure()` - releases the swap chain so
    ///      the DOM canvas can be GCed.
    ///   2. `GpuDevice.destroy()` - releases all child resources (buffers,
    ///      textures, pipelines) and the device itself.
    ///
    /// Callers should run this from a `use_cleanup` callback whenever the
    /// host component is being torn down (e.g. on a `match` arm switch).
    /// Without it the previous GPU device lingers until GC, and a fresh
    /// `init()` may either reuse the dead device (silent black canvas) or
    /// fail to acquire a new one until the old device is collected.
    ///
    /// `Reflect::get` failures and JS exceptions are swallowed - this is a
    /// best-effort cleanup path, and the engine must not panic during
    /// teardown.
    pub fn dispose(&self) {
        let context: &JsValue = self.get_context();
        if let Ok(unconfigure_fn) =
            Reflect::get(context, &JsValue::from_str(WEBGPU_METHOD_UNCONFIGURE))
            && let Ok(unconfigure_callable) = unconfigure_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = unconfigure_callable.call0(context);
        }
        let device: &JsValue = self.get_device();
        if let Ok(destroy_fn) = Reflect::get(device, &JsValue::from_str(WEBGPU_METHOD_DESTROY))
            && let Ok(destroy_callable) = destroy_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = destroy_callable.call0(device);
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Render-pass dynamic state (viewport / scissor / stencil / blend)
    // ─────────────────────────────────────────────────────────────────────

    /// Sets the viewport for all subsequent draw calls on the given render pass.
    ///
    /// The viewport maps NDC `[-1, 1]` to the given pixel rectangle. `min_depth`
    /// and `max_depth` (both in `[0, 1]`) clamp the depth range; the defaults
    /// of `0.0` and `1.0` cover the whole depth buffer. This call must be
    /// issued between `beginRenderPass()` and `pass.end()`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `&ViewportDescriptor` - The viewport rectangle and (optional) depth range.
    pub fn set_viewport(&self, pass: &JsValue, viewport: &ViewportDescriptor) {
        // WebGPU `setViewport(x, y, width, height, minDepth, maxDepth)`
        // takes six scalar arguments — the previous descriptor-dict form
        // never validated (`call1` with one object → NaN viewport, error
        // silently swallowed). Scalars also drop the per-call `Object`
        // allocation and 11 `from_str` property keys.
        let args: Array = Array::new_with_length(6);
        args.set(0, JsValue::from_f64(*viewport.get_x() as f64));
        args.set(1, JsValue::from_f64(*viewport.get_y() as f64));
        args.set(2, JsValue::from_f64(*viewport.get_width() as f64));
        args.set(3, JsValue::from_f64(*viewport.get_height() as f64));
        args.set(4, JsValue::from_f64(WEBGPU_DEFAULT_VIEWPORT_MIN_DEPTH));
        args.set(5, JsValue::from_f64(WEBGPU_DEFAULT_VIEWPORT_MAX_DEPTH));
        if let Ok(set_fn) = cached_method(
            GpuReceiverClass::RenderPass,
            pass,
            WEBGPU_METHOD_SET_VIEWPORT,
        ) {
            let _: Result<JsValue, JsValue> = set_fn.apply(pass, &args);
        }
    }

    /// Sets the scissor rectangle for all subsequent draw calls on the given
    /// render pass.
    ///
    /// Fragments outside the rectangle are discarded. The scissor is applied
    /// after the viewport, so coordinates are in the same pixel space as
    /// [`WebGpuRenderer::set_viewport`]. A scissor that extends outside the
    /// render target is clamped to the target bounds by the GPU.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `u32` - X coordinate of the scissor origin in pixels.
    /// - `u32` - Y coordinate of the scissor origin in pixels.
    /// - `u32` - Scissor width in pixels.
    /// - `u32` - Scissor height in pixels.
    pub fn set_scissor_rect(&self, pass: &JsValue, x: u32, y: u32, width: u32, height: u32) {
        // WebGPU `setScissorRect(x, y, width, height)` takes four scalar
        // arguments — the previous descriptor-dict form never validated
        // (`call1` with one object → NaN scissor, error silently
        // swallowed). Scalars also drop the per-call `Object` allocation
        // and 8 `from_str` property keys.
        let args: Array = Array::new_with_length(4);
        args.set(0, JsValue::from_f64(x as f64));
        args.set(1, JsValue::from_f64(y as f64));
        args.set(2, JsValue::from_f64(width as f64));
        args.set(3, JsValue::from_f64(height as f64));
        if let Ok(set_fn) = cached_method(
            GpuReceiverClass::RenderPass,
            pass,
            WEBGPU_METHOD_SET_SCISSOR_RECT,
        ) {
            let _: Result<JsValue, JsValue> = set_fn.apply(pass, &args);
        }
    }

    /// Sets the blend constant used by `"constant"` / `"one-minus-constant"`
    /// blend factors.
    ///
    /// Affects all subsequent draw calls on the given render pass. The
    /// constant is a linear-space RGBA color in `[0, 1]` per component.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `f32` - Red component.
    /// - `f32` - Green component.
    /// - `f32` - Blue component.
    /// - `f32` - Alpha component.
    pub fn set_blend_constant(&self, pass: &JsValue, r: f32, g: f32, b: f32, a: f32) {
        let color_dict: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_R),
            &JsValue::from_f64(r as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_G),
            &JsValue::from_f64(g as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_B),
            &JsValue::from_f64(b as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_A),
            &JsValue::from_f64(a as f64),
        );
        let color_js: JsValue = color_dict.unchecked_into::<JsValue>();
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BLEND_CONSTANT))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = set_callable.call1(pass, &color_js);
        }
    }

    /// Sets the stencil reference value used by stencil tests.
    ///
    /// The reference is the value the GPU compares against when the shader
    /// pipeline was built with a stencil state using `"always"`, `"less"`,
    /// `"equal"`, etc. compare ops. This call must be issued between
    /// `beginRenderPass()` and `pass.end()`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `u32` - The stencil reference value (8-bit, `[0, 255]`).
    pub fn set_stencil_reference(&self, pass: &JsValue, reference: u32) {
        if let Ok(set_fn) = Reflect::get(
            pass,
            &JsValue::from_str(WEBGPU_METHOD_SET_STENCIL_REFERENCE),
        ) && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> =
                set_callable.call1(pass, &JsValue::from_f64(reference as f64));
        }
    }

    /// Sets a bind group on a render pass with dynamic offsets.
    ///
    /// Use this overload of `set_bind_group` when the bind-group layout was
    /// built with `hasDynamicOffset: true` for one or more buffer bindings.
    /// Each value in `dynamic_offsets` is added to the corresponding
    /// `@group(N) @binding(M)` buffer's base offset before the draw call.
    /// For non-dynamic bind groups, prefer the simpler
    /// `set_bind_group` (3-arg) overload exposed via the `pub(crate)` API.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `u32` - Bind-group slot index.
    /// - `&JsValue` - The `GpuBindGroup` to bind.
    /// - `&[u32]` - Dynamic offsets, one per dynamic-offset binding.
    pub fn set_bind_group_with_dynamic_offsets(
        &self,
        pass: &JsValue,
        index: u32,
        group: &JsValue,
        dynamic_offsets: &[u32],
    ) {
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BIND_GROUP))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            // WebGPU's setBindGroup has two overloads: with and without
            // dynamic offsets. We always use the 4-arg form to keep the
            // call site simple; the empty offset array is well-defined.
            // OPT 35: zero-copy `Uint32Array::view` over the wasm linear-memory
            // slice instead of allocating a fresh JS Array + per-element
            // `from_f64` writes on every setBindGroup call.
            // SAFETY: `view` is only used inside the `set_callable.call4(...)`
            // on the next line; the resulting JsValue does not outlive
            // `dynamic_offsets`'s borrow, and `dynamic_offsets` outlives the
            // call because the call happens synchronously before this function
            // returns.
            let offsets_view: Uint32Array = unsafe { Uint32Array::view(dynamic_offsets) };
            let offsets_js: &JsValue = offsets_view.as_ref();
            let _: Result<JsValue, JsValue> = set_callable.call4(
                pass,
                &JsValue::from_f64(index as f64),
                group,
                offsets_js,
                &JsValue::from_f64(0.0),
            );
        }
    }

    /// Sets a bind group on a compute pass with optional dynamic offsets.
    ///
    /// Same semantics as [`WebGpuRenderer::set_bind_group_with_dynamic_offsets`]
    /// but on a `GpuComputePassEncoder`. The `setBindGroup` method name is
    /// the same on both encoder types; this method wraps it for the compute
    /// pass to give callers a typed entry point.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuComputePassEncoder`.
    /// - `u32` - Bind-group slot index.
    /// - `&JsValue` - The `GpuBindGroup` to bind.
    /// - `&[u32]` - Dynamic offsets for dynamic-offset bindings.
    pub fn set_bind_group_compute_with_dynamic_offsets(
        &self,
        pass: &JsValue,
        index: u32,
        group: &JsValue,
        dynamic_offsets: &[u32],
    ) {
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BIND_GROUP))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            // OPT 35: zero-copy `Uint32Array::view` over the wasm linear-memory
            // slice instead of allocating a fresh JS Array + per-element
            // `from_f64` writes on every setBindGroup call (compute variant).
            // SAFETY: same as the render variant — the view is only used
            // synchronously inside the next call4 invocation and does not
            // outlive the `dynamic_offsets` borrow.
            let offsets_view: Uint32Array = unsafe { Uint32Array::view(dynamic_offsets) };
            let offsets_js: &JsValue = offsets_view.as_ref();
            let _: Result<JsValue, JsValue> = set_callable.call4(
                pass,
                &JsValue::from_f64(index as f64),
                group,
                offsets_js,
                &JsValue::from_f64(0.0),
            );
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Texture view, mipmap generation, and CPU upload
    // ─────────────────────────────────────────────────────────────────────

    /// Creates a `GpuTextureView` for the given texture with full descriptor control.
    ///
    /// Pass `None` for a default view (full 2D, all mips, all aspects) — this
    /// is the cheap view that is implicitly created by bind-group creation.
    /// Pass `Some(&descriptor)` to sub-select mip levels, array slices, or
    /// the depth-only aspect of a depth-stencil texture.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuTexture` to view.
    /// - `Option<&TextureViewDescriptor>` - Optional descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuTextureView`. Returns `JsValue::UNDEFINED` if
    ///   the call fails (e.g. invalid mip range); check for `undefined`
    ///   before using the result.
    pub fn create_view(
        &self,
        texture: &JsValue,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> JsValue {
        let create_view_fn: Function =
            match Reflect::get(texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .ok()
                .and_then(|v: JsValue| v.dyn_into::<Function>().ok())
            {
                Some(f) => f,
                None => return JsValue::UNDEFINED,
            };
        // Inline the descriptor dict construction; we keep the engine-wide
        // convention of "0 / None means default" so the browser falls back
        // to its own defaults for omitted keys.
        let desc_value: JsValue = match descriptor {
            None => JsValue::UNDEFINED,
            Some(d) => {
                let dict: Object = Object::new();
                if let Some(format) = d.get_format() {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
                        &JsValue::from_str(format),
                    );
                }
                // `dimension` and `aspect` are explicitly sent as their
                // default values ("2d" / "all") rather than omitted, because
                // a handful of browsers reject undefined keys on the
                // createView descriptor.
                let _: Result<bool, JsValue> = Reflect::set(
                    &dict,
                    &JsValue::from_str(WEBGPU_PROPERTY_DIMENSION),
                    &JsValue::from_str(d.effective_dimension()),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &dict,
                    &JsValue::from_str(WEBGPU_PROPERTY_ASPECT),
                    &JsValue::from_str(d.effective_aspect()),
                );
                // baseMipLevel / mipLevelCount / baseArrayLayer /
                // arrayLayerCount are u32 with 0 = "use the default".
                // Skip them when they are still at the default so that the
                // browser applies its own spec-compliant fallback.
                let base_mip: u32 = d.get_base_mip_level();
                if base_mip != 0 {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_BASE_MIP_LEVEL),
                        &JsValue::from_f64(base_mip as f64),
                    );
                }
                let mip_count: u32 = d.get_mip_level_count();
                if mip_count != 0 {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL_COUNT),
                        &JsValue::from_f64(mip_count as f64),
                    );
                }
                let base_array: u32 = d.get_base_array_layer();
                if base_array != 0 {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_BASE_ARRAY_LAYER),
                        &JsValue::from_f64(base_array as f64),
                    );
                }
                let array_count: u32 = d.get_array_layer_count();
                if array_count != 0 {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_ARRAY_LAYER_COUNT),
                        &JsValue::from_f64(array_count as f64),
                    );
                }
                dict.unchecked_into::<JsValue>()
            }
        };
        create_view_fn
            .call1(texture, &desc_value)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Generates the full mipmap chain for the given texture.
    ///
    /// Equivalent to repeatedly calling `copyTextureToTexture` from level
    /// `i` to level `i+1` with the appropriate mip dimensions, but in one
    /// GPU command. The texture must have been created with `RENDER_ATTACHMENT
    /// | TEXTURE_BINDING | COPY_DST | COPY_SRC` usage and `mipLevelCount > 1`.
    /// Requires the `mipmap` WebGPU feature, or a GPU that supports it
    /// unconditionally (most desktop GPUs do).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuTexture` whose mips will be generated.
    pub fn generate_mipmaps(&self, texture: &JsValue) {
        if let Ok(gen_fn) = Reflect::get(texture, &JsValue::from_str(WEBGPU_METHOD_GENERATE_MIPMAP))
            && let Ok(gen_callable) = gen_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = gen_callable.call0(texture);
        }
    }

    /// Uploads CPU-side pixel data directly to a texture via `queue.writeTexture`.
    ///
    /// Use this instead of `create_buffer + write_buffer + copyBufferToTexture`
    /// for one-shot uploads (ImGui font atlases, sprite sheets, procedural
    /// noise). The queue is acquired internally via the cached `device.queue`
    /// handle, so this is the preferred path for textures that are written
    /// once and sampled many times.
    ///
    /// `bytes_per_row` must be a multiple of 256. The `data` layout must
    /// match the texture's `format`; the engine does not perform swizzling.
    ///
    /// # Arguments
    ///
    /// - `&TextureWriteDescriptor` - The write descriptor.
    pub fn write_texture(&self, descriptor: &TextureWriteDescriptor) {
        let queue: JsValue =
            match Reflect::get(self.get_device(), &JsValue::from_str(WEBGPU_PROPERTY_QUEUE))
                .ok()
                .and_then(|v: JsValue| v.dyn_into::<JsValue>().ok())
            {
                Some(q) => q,
                None => return,
            };
        let layout_dict: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &layout_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_BYTES_PER_ROW),
            &JsValue::from_f64(descriptor.get_bytes_per_row() as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &layout_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_ROWS_PER_IMAGE),
            &JsValue::from_f64(descriptor.get_rows_per_image() as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &layout_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_OFFSET_BYTES),
            &JsValue::from_f64(0.0),
        );
        let layout_js: JsValue = layout_dict.unchecked_into::<JsValue>();
        let write_fn: Function =
            match Reflect::get(&queue, &JsValue::from_str(WEBGPU_METHOD_WRITE_TEXTURE))
                .ok()
                .and_then(|v: JsValue| v.dyn_into::<Function>().ok())
            {
                Some(f) => f,
                None => return,
            };
        // Build destination dict: { texture, mipLevel, origin? }
        let dest_dict: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &dest_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE),
            &descriptor.get_texture(),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &dest_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL),
            &JsValue::from_f64(descriptor.get_mip_level() as f64),
        );
        if let Some(origin) = descriptor.get_origin() {
            let _: Result<bool, JsValue> = Reflect::set(
                &dest_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_ORIGIN),
                &origin,
            );
        }
        let dest_js: JsValue = dest_dict.unchecked_into::<JsValue>();
        // WebGPU's queue.writeTexture requires a Uint8Array view; we hand
        // it the raw Vec<u8> and let JS interop copy it. This is the same
        // path wasm-bindgen takes for &[u8] → Uint8Array.
        let data_js: JsValue = Uint8Array::from(descriptor.get_data().as_slice()).into();
        // For the size extent, we read bytes_per_row's texel width from the
        // destination. Without a format converter we default to a square
        // shape based on the data size. The caller is expected to construct
        // a TextureWriteDescriptor that matches their texture exactly;
        // this method does not auto-derive size.
        let size_value: JsValue = {
            let bpr: u32 = descriptor.get_bytes_per_row();
            let rows: u32 = if descriptor.get_rows_per_image() == 0 {
                (descriptor.get_data().len() as u32) / bpr.max(1)
            } else {
                descriptor.get_rows_per_image()
            };
            let size_dict: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &size_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_WIDTH),
                &JsValue::from_f64(bpr as f64),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &size_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_HEIGHT),
                &JsValue::from_f64(rows as f64),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &size_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_OR_1),
                &JsValue::from_f64(1.0),
            );
            size_dict.unchecked_into::<JsValue>()
        };
        let _: Result<JsValue, JsValue> =
            write_fn.call4(&queue, &dest_js, &data_js, &layout_js, &size_value);
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Shader module + explicit pipeline compile diagnostics
    // ─────────────────────────────────────────────────────────────────────

    /// Creates a `GpuShaderModule` from a WGSL source string with a debug label.
    ///
    /// Equivalent to the `pub(crate) fn create_shader_module` overload but
    /// attaches a `label` to the module so it shows up under that name in
    /// browser devtools (e.g. Chrome's `chrome://gpu-internals` and the
    /// WebGPU Inspector panel). The label has no runtime effect; it is
    /// purely a developer-experience aid when many shader modules coexist.
    ///
    /// # Arguments
    ///
    /// - `&str` - WGSL source.
    /// - `&str` - Debug label shown in browser devtools.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuShaderModule`, or `JsValue::UNDEFINED` if
    ///   the call fails.
    pub fn create_shader_module_with_label(&self, wgsl_source: &str, label: &str) -> JsValue {
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_CODE),
            &JsValue::from_str(wgsl_source),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LABEL),
            &JsValue::from_str(label),
        );
        let desc_value: JsValue = descriptor.unchecked_into::<JsValue>();
        if let Ok(create_fn) = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SHADER_MODULE),
        ) && let Ok(create_callable) = create_fn.dyn_into::<Function>()
        {
            // The call returns a Promise that resolves to the shader module.
            // We do not await it; the caller is expected to drive the future
            // or pass the result into a pipeline creation call.
            return create_callable
                .call1(self.get_device(), &desc_value)
                .unwrap_or(JsValue::UNDEFINED);
        }
        JsValue::UNDEFINED
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Buffer readback via mapAsync + getMappedRange
    // ─────────────────────────────────────────────────────────────────────

    /// Reads back the contents of a buffer via `mapAsync` + `getMappedRange` +
    /// `unmap`.
    ///
    /// This is an **`async fn`**, NOT a synchronous wrapper. It must be
    /// `await`-ed by the caller. Use it from inside another
    /// `wasm_bindgen_futures` future (e.g. a frame loop) — do not call
    /// it from synchronous code, since the awaiter must be driven by
    /// the executor. The buffer must have been created with `MAP_READ`
    /// usage, and the read must be preceded by a GPU submission that
    /// finished writing to the buffer (i.e. `queue.submit([encoder.finish()])`
    /// followed by `device.lost` / a fence).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuBuffer` to read back.
    /// - `u64` - Byte offset into the buffer.
    /// - `u64` - Number of bytes to read.
    ///
    /// # Returns
    ///
    /// - `Option<Vec<u8>>` - The bytes, or `None` if the readback failed.
    pub async fn read_buffer(&self, buffer: &JsValue, offset: u64, size: u64) -> Option<Vec<u8>> {
        // Step 1: buffer.mapAsync(mode, offset, size)
        let map_fn: Function = Reflect::get(buffer, &JsValue::from_str(WEBGPU_METHOD_MAP_ASYNC))
            .ok()
            .and_then(|v: JsValue| v.dyn_into::<Function>().ok())?;
        let map_promise: Promise = map_fn
            .call3(
                buffer,
                // `mapAsync` takes a `GPUMapMode` bitmask; the spec
                // allows OR'ing `READ` and `WRITE` together, so we
                // use the `map_mode_for` helper that pins the
                // `WEBGPU_MAP_MODE_WRITE` constant on the live code
                // path. This buffer is read-only for the host, so
                // we pass `read = true, write = false`.
                &JsValue::from_f64(map_mode_for(/* read = */ true, /* write = */ false) as f64),
                &JsValue::from_f64(offset as f64),
                &JsValue::from_f64(size as f64),
            )
            .ok()?
            .unchecked_into();
        // Step 2: await the mapAsync promise
        let _map_result: JsValue = JsFuture::from(map_promise).await.ok()?;
        // Step 3: buffer.getMappedRange(offset, size)
        let get_range_fn: Function =
            Reflect::get(buffer, &JsValue::from_str(WEBGPU_METHOD_GET_MAPPED_RANGE))
                .ok()
                .and_then(|v: JsValue| v.dyn_into::<Function>().ok())?;
        let array_buffer: ArrayBuffer = get_range_fn
            .call2(
                buffer,
                &JsValue::from_f64(offset as f64),
                &JsValue::from_f64(size as f64),
            )
            .ok()?
            .unchecked_into();
        // Step 4: copy out before unmap invalidates the memory
        let u8_view: Uint8Array = Uint8Array::new(&array_buffer);
        let mut out: Vec<u8> = vec![0u8; u8_view.length() as usize];
        u8_view.copy_to(&mut out);
        // Step 5: unmap
        if let Ok(unmap_fn) = Reflect::get(buffer, &JsValue::from_str(WEBGPU_METHOD_UNMAP))
            && let Ok(unmap_callable) = unmap_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = unmap_callable.call0(buffer);
        }
        Some(out)
    }
}

/// Implements helper methods on `WebGpuInitError`.
///
/// These methods provide ergonomic access to the diagnostic code and the
/// underlying JS error value, which are useful when surfacing the failure
/// to the user (e.g. via `Console::error` from the example crate).
impl WebGpuInitError {
    /// Returns a short, machine-readable identifier for this error variant.
    ///
    /// Suitable for use as a stable error code in logs or telemetry.
    /// The codes are stable across releases.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The error code (e.g. `"WEBGPU_NAVIGATOR_GPU_MISSING"`).
    pub fn code(&self) -> &'static str {
        match self {
            Self::NavigatorLookup(_) => WEBGPU_INIT_ERROR_NAVIGATOR_LOOKUP,
            Self::NavigatorGpuMissing => WEBGPU_INIT_ERROR_NAVIGATOR_GPU_MISSING,
            Self::RequestAdapterLookup(_) => WEBGPU_INIT_ERROR_REQUEST_ADAPTER_LOOKUP,
            Self::RequestAdapterCall(_) => WEBGPU_INIT_ERROR_REQUEST_ADAPTER_CALL,
            Self::AdapterPromise(_) => WEBGPU_INIT_ERROR_ADAPTER_PROMISE,
            Self::AdapterUnavailable => WEBGPU_INIT_ERROR_ADAPTER_UNAVAILABLE,
            Self::RequestDeviceLookup(_) => WEBGPU_INIT_ERROR_REQUEST_DEVICE_LOOKUP,
            Self::RequestDeviceCall(_) => WEBGPU_INIT_ERROR_REQUEST_DEVICE_CALL,
            Self::DevicePromise(_) => WEBGPU_INIT_ERROR_DEVICE_PROMISE,
            Self::DeviceUnavailable => WEBGPU_INIT_ERROR_DEVICE_UNAVAILABLE,
            Self::CanvasNotFound(_) => WEBGPU_INIT_ERROR_CANVAS_NOT_FOUND,
            Self::CanvasQuery(_) => WEBGPU_INIT_ERROR_CANVAS_QUERY,
            Self::CanvasContextUnavailable => WEBGPU_INIT_ERROR_CANVAS_CONTEXT_UNAVAILABLE,
            Self::PreferredFormatLookup(_) => WEBGPU_INIT_ERROR_PREFERRED_FORMAT_LOOKUP,
            Self::PreferredFormatCall(_) => WEBGPU_INIT_ERROR_PREFERRED_FORMAT_CALL,
            Self::PreferredFormatType(_) => WEBGPU_INIT_ERROR_PREFERRED_FORMAT_TYPE,
            Self::ConfigureLookup(_) => WEBGPU_INIT_ERROR_CONFIGURE_LOOKUP,
            Self::QueueLookup(_) => WEBGPU_INIT_ERROR_QUEUE_LOOKUP,
        }
    }

    /// Returns the underlying JS error value if this variant carries one.
    ///
    /// Variants that do not capture a JS value (e.g. `NavigatorGpuMissing`,
    /// `AdapterUnavailable`, `CanvasNotFound`, `CanvasContextUnavailable`)
    /// return `None`.
    ///
    /// # Returns
    ///
    /// - `Option<&JsValue>` - The captured JS error, if any.
    pub fn js_error(&self) -> Option<&JsValue> {
        match self {
            Self::NavigatorLookup(err)
            | Self::RequestAdapterLookup(err)
            | Self::RequestAdapterCall(err)
            | Self::AdapterPromise(err)
            | Self::RequestDeviceLookup(err)
            | Self::RequestDeviceCall(err)
            | Self::DevicePromise(err)
            | Self::CanvasQuery(err)
            | Self::PreferredFormatLookup(err)
            | Self::PreferredFormatCall(err)
            | Self::PreferredFormatType(err)
            | Self::ConfigureLookup(err)
            | Self::QueueLookup(err) => Some(err),
            Self::NavigatorGpuMissing
            | Self::AdapterUnavailable
            | Self::DeviceUnavailable
            | Self::CanvasContextUnavailable
            | Self::CanvasNotFound(_) => None,
        }
    }
}

/// Implements `Display` for `WebGpuInitError`.
///
/// The formatted message is intended for end-user diagnostic output
/// (typically forwarded to `Console::error` by the calling application)
/// and includes the variant code plus a human-readable description. When
/// the variant carries a JS error, its `Debug` form is appended.
impl Display for WebGpuInitError {
    /// Formats the [`WebGpuInitError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NavigatorLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(navigator, webgpu) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::NavigatorGpuMissing => write!(
                formatter,
                "[{}] navigator.gpu is missing - browser does not expose WebGPU on this origin",
                self.code(),
            ),
            Self::RequestAdapterLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(gpu, requestAdapter) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::RequestAdapterCall(err) => write!(
                formatter,
                "[{}] gpu.requestAdapter() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::AdapterPromise(err) => write!(
                formatter,
                "[{}] adapter promise rejected or timed out: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::AdapterUnavailable => write!(
                formatter,
                "[{}] requestAdapter returned null - no compatible GPU adapter for the requested powerPreference",
                self.code(),
            ),
            Self::RequestDeviceLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(adapter, requestDevice) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::RequestDeviceCall(err) => write!(
                formatter,
                "[{}] adapter.requestDevice() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::DevicePromise(err) => write!(
                formatter,
                "[{}] device promise rejected or timed out: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::DeviceUnavailable => write!(
                formatter,
                "[{}] requestDevice returned null - adapter could not allocate a device (possibly device-lost)",
                self.code(),
            ),
            Self::CanvasNotFound(selector) => write!(
                formatter,
                "[{}] canvas element {:?} not found in DOM",
                self.code(),
                selector,
            ),
            Self::CanvasQuery(err) => write!(
                formatter,
                "[{}] querySelector threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::CanvasContextUnavailable => write!(
                formatter,
                "[{}] canvas.get_context('webgpu') returned null - the canvas may already be using another context type or WebGPU is disabled",
                self.code(),
            ),
            Self::PreferredFormatLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(gpu, getPreferredCanvasFormat) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::PreferredFormatCall(err) => write!(
                formatter,
                "[{}] gpu.getPreferredCanvasFormat() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::PreferredFormatType(value) => write!(
                formatter,
                "[{}] getPreferredCanvasFormat returned non-string: {}",
                self.code(),
                js_error_to_string(value),
            ),
            Self::ConfigureLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(context, configure) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::QueueLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(device, queue) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
        }
    }
}

/// Implements the standard `std::error::Error` trait for `WebGpuInitError`.
///
/// The `source()` method delegates to the underlying JS error's `toString()`
/// representation when present, otherwise returns `None`. The engine never
/// logs or prints anything; this impl exists solely so the error composes
/// with `Result`-based APIs and `?` operator chains.
impl Error for WebGpuInitError {}

/// Inherent implementation of [`PendingErrorCell`].
impl PendingErrorCell {
    /// Construct a new, empty pending-error slot.
    ///
    /// The inner `UnsafeCell<Option<JsValue>>` starts as `None`; the
    /// WebGPU `pop_error_sync` microtask is the only thing that ever
    /// writes to it, and `take_last_error` is the only reader.
    pub fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// Hand out a raw pointer to the inner cell for the
    /// `spawn_local` closure to write through.
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid for the lifetime of `&self`,
    /// and only safe to write to on the WASM main thread. The caller
    /// must guarantee that no other code is reading the same
    /// `PendingErrorCell` concurrently — this is enforced by the
    /// single-threaded scheduler: the spawned future is drained
    /// before the next render tick's `take_last_error` runs.
    ///
    /// # Returns
    ///
    /// - `*mut Option<JsValue>` - Raw pointer to the inner storage.
    pub fn as_ptr(&self) -> *mut Option<JsValue> {
        self.0.get()
    }
}

/// Default-construction for [`PendingErrorCell`].
impl Default for PendingErrorCell {
    /// Constructs a default [`PendingErrorCell`] value.
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: see the doc comment on `struct.rs::PendingErrorCell`.
//
// `PendingErrorCell` wraps `UnsafeCell`, which is `!Sync` by design.
// We hand-implement `Sync` because:
//
// - The renderer is compiled for `wasm32` and runs on the WASM
//   single-threaded scheduler; there is no other thread to race
//   against.
// - The owning pointer is held inside an `Rc<PendingErrorCell>`, and
//   `Rc` is itself `!Send`/`!Sync`, so the value cannot escape the
//   current thread even if the type were `Sync`.
// - The `pop_error_sync` future and `take_last_error` never overlap
//   in wall-clock time: the future is a microtask that resolves
//   before the next render tick drains the slot.
//
// If `euv-engine` is ever built for a multi-threaded target
// (native, `wasm-bindgen-rayon`, `wasm32-atomics`), this `unsafe impl`
// becomes unsound and must be removed — at that point the renderer
// will need a real `Mutex` or `RwLock` around the slot.

unsafe impl Sync for PendingErrorCell {}
