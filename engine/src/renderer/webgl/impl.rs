use super::*;

thread_local! {
    static GL_TEX_IMAGE_2D_CACHE: RefCell<Option<Function>> = const { RefCell::new(None) };
}

/// Implements diagnostic helpers on [`WebGl2InitError`].
impl WebGl2InitError {
    /// Returns a short, machine-readable identifier for this error variant.
    ///
    /// Suitable for use as a stable error code in logs or telemetry.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The error code (e.g. `"WEBGL_CONTEXT_UNAVAILABLE"`).
    pub fn code(&self) -> &'static str {
        match self {
            Self::CanvasNotFound(_) => GL_ERROR_CANVAS_NOT_FOUND,
            Self::CanvasQuery(_) => GL_ERROR_CANVAS_QUERY,
            Self::ContextUnavailable => GL_ERROR_CONTEXT_UNAVAILABLE,
            Self::ContextLookup(_) => GL_ERROR_CONTEXT_LOOKUP,
            Self::ContextCast => GL_ERROR_CONTEXT_CAST,
        }
    }
}

/// Implements the formatted message for [`WebGl2InitError`].
impl Display for WebGl2InitError {
    /// Formats the [`WebGl2InitError`] via the supplied formatter.
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
            Self::CanvasNotFound(selector) => {
                write!(
                    formatter,
                    "[{}] canvas element {selector:?} not found in DOM",
                    self.code()
                )
            }
            Self::CanvasQuery(selector) => {
                write!(
                    formatter,
                    "[{}] querySelector({selector:?}) threw",
                    self.code()
                )
            }
            Self::ContextUnavailable => write!(
                formatter,
                "[{}] canvas.get_context('webgl2') returned null - the browser does not support WebGL 2 or the canvas already uses another context type",
                self.code()
            ),
            Self::ContextLookup(selector) => write!(
                formatter,
                "[{}] canvas.get_context('webgl2') threw while resolving {selector:?}",
                self.code()
            ),
            Self::ContextCast => write!(
                formatter,
                "[{}] get_context('webgl2') result could not be cast to WebGl2RenderingContext",
                self.code()
            ),
        }
    }
}

/// Implements the formatted message for [`WebGlProgramError`].
impl Display for WebGlProgramError {
    /// Formats the [`WebGlProgramError`] via the supplied formatter.
    ///
    /// The browser's own info log is appended verbatim, because a GLSL
    /// diagnostic is only actionable in the driver's own wording.
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
            Self::ShaderCompile(log) => write!(formatter, "WebGL shader compilation failed: {log}"),
            Self::ProgramLink(log) => write!(formatter, "WebGL program link failed: {log}"),
        }
    }
}

/// Registers both WebGL error types as standard [`Error`] values.
impl Error for WebGl2InitError {}

impl Error for WebGlProgramError {}

/// Implements buffer creation, upload, and lifetime for a vertex, index,
/// or uniform buffer.
///
/// # Performance
///
/// The wrapper tracks its own byte capacity so
/// [`GlBuffer::upload`](super::GlBuffer::upload) can choose between two
/// paths without asking the driver. A payload that fits the existing
/// allocation goes through `bufferSubData` and leaves the driver's
/// storage untouched; one that does not goes through `bufferData` with
/// the new size, which orphans the old allocation so the driver can hand
/// the same memory back without a free-and-realloc pair. That second
/// path is the buffer-orphaning trick, and it is what stops a per-frame
/// vertex stream that grows by one vertex every frame from allocating a
/// new `WebGlBuffer` and leaving the old one for the garbage collector.
impl GlBuffer {
    /// Allocates a buffer sized for `size` bytes.
    ///
    /// The allocation carries no contents, so the first
    /// [`GlBuffer::upload`](super::GlBuffer::upload) re-orphans it at the
    /// exact size it needs. Binding to `target` first is required:
    /// `bufferData` sizes whichever buffer is bound to that target, so
    /// allocating into an unbound target silently resizes whatever was
    /// bound last.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    /// - `u32` - The buffer target, as returned by
    ///   [`gl_buffer_target`].
    /// - `u32` - The size in bytes to reserve.
    /// - `u32` - The `bufferData` usage hint, as returned by
    ///   [`gl_buffer_usage_hint`].
    ///
    /// # Returns
    ///
    /// - `Option<GlBuffer>` - The allocated buffer, or `None` when the
    ///   driver refused to create one.
    pub fn create(
        context: &WebGl2RenderingContext,
        target: u32,
        size: u32,
        usage: u32,
    ) -> Option<GlBuffer> {
        let buffer: WebGlBuffer = context.create_buffer()?;
        context.bind_buffer(target, Some(&buffer));
        context.buffer_data_with_i32(target, size as i32, usage);
        Some(GlBuffer {
            buffer,
            capacity: size,
            usage,
        })
    }

    /// Allocates a buffer for a role, deriving both the target and the
    /// usage hint from the role itself.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    /// - `BufferUsage` - The role the buffer will play.
    /// - `u32` - The size in bytes to reserve.
    ///
    /// # Returns
    ///
    /// - `Option<GlBuffer>` - The allocated buffer, or `None` when the
    ///   driver refused to create one.
    pub fn create_for(
        context: &WebGl2RenderingContext,
        usage: BufferUsage,
        size: u32,
    ) -> Option<GlBuffer> {
        GlBuffer::create(
            context,
            gl_buffer_target(usage),
            size,
            gl_buffer_usage_hint(usage),
        )
    }

    /// Reports whether the buffer's current allocation can hold `size`
    /// bytes without re-allocating.
    ///
    /// Exposed separately from
    /// [`GlBuffer::upload`](super::GlBuffer::upload) so a caller batching
    /// several regions into one buffer can size the whole batch once,
    /// rather than growing the buffer on every region.
    ///
    /// # Arguments
    ///
    /// - `u32` - The byte count to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `size` is within the current capacity.
    pub fn fits(&self, size: u32) -> bool {
        size <= self.get_capacity()
    }

    /// Binds the buffer to `target`.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    /// - `u32` - The buffer target to bind to.
    pub fn bind(&self, context: &WebGl2RenderingContext, target: u32) {
        context.bind_buffer(target, Some(self.get_buffer()));
    }

    /// Writes `data` over the whole buffer, re-allocating only when the
    /// payload outgrows the current capacity.
    ///
    /// The reallocation path binds first, because `bufferData` operates
    /// on whichever buffer is bound to the target rather than on a buffer
    /// argument. The sub-data path binds too, so a caller that bound
    /// something else since the last upload still writes to the right
    /// object.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `u32` - The buffer target the buffer is bound under.
    /// - `&[u8]` - The bytes to write.
    pub fn upload(&mut self, context: &WebGl2RenderingContext, target: u32, data: &[u8]) {
        let size: u32 = data.len() as u32;
        if !self.fits(size) {
            self.grow(context, target, size);
        }
        context.bind_buffer(target, Some(self.get_buffer()));
        context.buffer_sub_data_with_i32_and_u8_array(target, 0, data);
    }

    /// Writes `data` at `offset` bytes into the buffer, leaving the rest
    /// untouched.
    ///
    /// The write is rejected outright when it would run past the end of
    /// the allocation. Checking here rather than letting the driver reject
    /// it is what turns a caller's off-by-one into a `false` return
    /// instead of a write outside the buffer.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `u32` - The buffer target the buffer is bound under.
    /// - `u32` - The byte offset to write at.
    /// - `&[u8]` - The bytes to write.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the write was issued, `false` when the
    ///   range fell outside the allocation.
    pub fn update(
        &mut self,
        context: &WebGl2RenderingContext,
        target: u32,
        offset: u32,
        data: &[u8],
    ) -> bool {
        let end: u32 = offset.saturating_add(data.len() as u32);
        if end > self.get_capacity() {
            return false;
        }
        context.bind_buffer(target, Some(self.get_buffer()));
        context.buffer_sub_data_with_i32_and_u8_array(target, offset as i32, data);
        true
    }

    /// Releases the buffer's driver-side storage and marks the wrapper
    /// empty.
    ///
    /// The capacity is zeroed so a later
    /// [`GlBuffer::upload`](super::GlBuffer::upload) against the same
    /// wrapper takes the reallocation path instead of writing into
    /// storage the driver has already reclaimed. Unbinding first matters
    /// because `deleteBuffer` on a bound buffer silently unbinds it,
    /// leaving the target's binding ambiguous for the next caller.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to release against.
    /// - `u32` - The buffer target the buffer is bound under.
    pub fn delete(&mut self, context: &WebGl2RenderingContext, target: u32) {
        context.bind_buffer(target, None);
        context.delete_buffer(Some(self.get_buffer()));
        self.set_capacity(0);
    }

    /// Reallocates the buffer to hold at least `size` bytes, orphaning
    /// the previous allocation.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    /// - `u32` - The buffer target the buffer is bound under.
    /// - `u32` - The new minimum size in bytes.
    fn grow(&mut self, context: &WebGl2RenderingContext, target: u32, size: u32) {
        context.bind_buffer(target, Some(self.get_buffer()));
        context.buffer_data_with_i32(target, size as i32, self.get_usage());
        self.set_capacity(size);
    }
}

/// Implements texture allocation, DOM-source upload, sampling state, and
/// release.
///
/// # Performance
///
/// Wrap and filter are set once by
/// [`GlTexture::set_parameters`](super::GlTexture::set_parameters) and
/// never re-issued per frame, and a freshly created texture is pinned to
/// a single-level `LINEAR` minification filter because GL's own default
/// is mip-aware: a texture with no mip chain sampled under
/// `LINEAR_MIPMAP_LINEAR` is incomplete and reads as solid black.
impl GlTexture {
    /// Allocates a texture and uploads `pixels` into its base mip level.
    ///
    /// `pixels` is a tightly packed `width * height * 4` RGBA byte slice.
    /// Pass an empty slice to allocate storage without initializing it,
    /// which is what a render target wants; the contents are then
    /// undefined until something is drawn into or uploaded onto them.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    /// - `u32` - The width in texels.
    /// - `u32` - The height in texels.
    /// - `GpuTextureFormat` - The texel format.
    /// - `&[u8]` - The tightly packed RGBA pixels, or empty.
    ///
    /// # Returns
    ///
    /// - `Option<GlTexture>` - The allocated texture, or `None` when the
    ///   driver refused to create one.
    pub fn create(
        context: &WebGl2RenderingContext,
        width: u32,
        height: u32,
        format: GpuTextureFormat,
        pixels: &[u8],
    ) -> Option<GlTexture> {
        let texture: WebGlTexture = context.create_texture()?;
        let target: u32 = gl_texture_target_2d();
        context.bind_texture(target, Some(&texture));
        let (internal, pixel_format, pixel_type): (i32, u32, u32) = gl_texture_layout(format);
        let payload: Option<&[u8]> = if pixels.is_empty() {
            None
        } else {
            Some(pixels)
        };
        let _: Result<(), JsValue> = context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                0,
                internal,
                width as i32,
                height as i32,
                0,
                pixel_format,
                pixel_type,
                payload,
            );
        GlTexture::apply_default_parameters(context);
        Some(GlTexture {
            texture,
            width,
            height,
            levels: 1,
            mipmapped: false,
        })
    }

    /// Pins a fresh texture to a single-level minification filter, so it
    /// is complete before any mip chain exists.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to configure against.
    fn apply_default_parameters(context: &WebGl2RenderingContext) {
        let target: u32 = gl_texture_target_2d();
        let linear: i32 = gl_filter_mode(FilterMode::Linear) as i32;
        let clamp: i32 = gl_address_mode(AddressMode::ClampToEdge) as i32;
        context.tex_parameteri(target, WebGl2RenderingContext::TEXTURE_MIN_FILTER, linear);
        context.tex_parameteri(target, WebGl2RenderingContext::TEXTURE_MAG_FILTER, linear);
        context.tex_parameteri(target, WebGl2RenderingContext::TEXTURE_WRAP_S, clamp);
        context.tex_parameteri(target, WebGl2RenderingContext::TEXTURE_WRAP_T, clamp);
    }

    /// Uploads a decoded DOM image into a fresh texture, sized to the
    /// image's own dimensions.
    ///
    /// The image is flipped on upload, because a DOM image's origin is
    /// at the top left while GL's texture origin is at the bottom left.
    /// Flipping here rather than negating a v coordinate in the shader
    /// costs one unpack-time transpose and leaves the sampler state
    /// identical to what a raw pixel upload produces.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&HtmlImageElement` - The decoded image to upload.
    /// - `GpuTextureFormat` - The texel format to allocate as.
    ///
    /// # Returns
    ///
    /// - `Option<GlTexture>` - The uploaded texture, or `None` when the
    ///   driver refused to create one.
    pub fn create_from_image(
        context: &WebGl2RenderingContext,
        image: &HtmlImageElement,
        format: GpuTextureFormat,
    ) -> Option<GlTexture> {
        let width: u32 = image.natural_width().max(1);
        let height: u32 = image.natural_height().max(1);
        let texture: WebGlTexture = context.create_texture()?;
        let target: u32 = gl_texture_target_2d();
        context.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, gl_flip_y(true));
        context.bind_texture(target, Some(&texture));
        let (internal, pixel_format, pixel_type): (i32, u32, u32) = gl_texture_layout(format);
        let _: Result<(), JsValue> = context.tex_image_2d_with_u32_and_u32_and_html_image_element(
            target,
            0,
            internal,
            pixel_format,
            pixel_type,
            image,
        );
        context.pixel_storei(
            WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL,
            gl_flip_y(false),
        );
        GlTexture::apply_default_parameters(context);
        Some(GlTexture {
            texture,
            width,
            height,
            levels: 1,
            mipmapped: false,
        })
    }

    /// Uploads another canvas element into a fresh texture, sized to
    /// that canvas's dimensions.
    ///
    /// This is the cheapest path from runtime-generated art to a sampled
    /// surface: a 2D context draws into the source canvas, and one
    /// `texImage2D` copies the result into the texture with no CPU round
    /// trip. The canvas is not flipped, because a 2D canvas is already
    /// addressed with a bottom-left origin in `drawImage`'s coordinate
    /// system.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&HtmlCanvasElement` - The source canvas to upload.
    /// - `GpuTextureFormat` - The texel format to allocate as.
    ///
    /// # Returns
    ///
    /// - `Option<GlTexture>` - The uploaded texture, or `None` when the
    ///   driver refused to create one.
    pub fn create_from_canvas(
        context: &WebGl2RenderingContext,
        canvas: &HtmlCanvasElement,
        format: GpuTextureFormat,
    ) -> Option<GlTexture> {
        let width: u32 = canvas.width().max(1);
        let height: u32 = canvas.height().max(1);
        let texture: WebGlTexture = context.create_texture()?;
        let target: u32 = gl_texture_target_2d();
        context.bind_texture(target, Some(&texture));
        let (internal, pixel_format, pixel_type): (i32, u32, u32) = gl_texture_layout(format);
        let _: Result<(), JsValue> = context.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
            target,
            0,
            internal,
            pixel_format,
            pixel_type,
            canvas,
        );
        GlTexture::apply_default_parameters(context);
        Some(GlTexture {
            texture,
            width,
            height,
            levels: 1,
            mipmapped: false,
        })
    }

    /// Uploads an `ImageBitmap` into a fresh texture, sized to the
    /// bitmap's dimensions.
    ///
    /// The bitmap is reached through a cached `texImage2D` method
    /// reference rather than a typed `web-sys` overload, so the module
    /// does not have to enable the `ImageBitmap` feature for a single
    /// call. The method is looked up once per wasm instance and reused on
    /// every later upload, so the steady-state cost matches a typed
    /// binding's. The bitmap is flipped, exactly as for a DOM image.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&Object` - The `ImageBitmap` to upload.
    /// - `u32` - The bitmap's width in pixels.
    /// - `u32` - The bitmap's height in pixels.
    /// - `GpuTextureFormat` - The texel format to allocate as.
    ///
    /// # Returns
    ///
    /// - `Option<GlTexture>` - The uploaded texture, or `None` when the
    ///   driver refused to create one or the upload threw.
    pub fn create_from_bitmap(
        context: &WebGl2RenderingContext,
        bitmap: &Object,
        width: u32,
        height: u32,
        format: GpuTextureFormat,
    ) -> Option<GlTexture> {
        let texture: WebGlTexture = context.create_texture()?;
        let target: u32 = gl_texture_target_2d();
        context.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, gl_flip_y(true));
        context.bind_texture(target, Some(&texture));
        let (internal, pixel_format, pixel_type): (i32, u32, u32) = gl_texture_layout(format);
        let outcome: Result<JsValue, JsValue> = GlTexture::call_tex_image_2d(
            context.as_ref(),
            internal,
            pixel_format,
            pixel_type,
            bitmap.as_ref(),
        );
        context.pixel_storei(
            WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL,
            gl_flip_y(false),
        );
        if outcome.is_err() {
            return None;
        }
        GlTexture::apply_default_parameters(context);
        Some(GlTexture {
            texture,
            width: width.max(1),
            height: height.max(1),
            levels: 1,
            mipmapped: false,
        })
    }

    /// Invokes the 5-argument `texImage2D(target, level, internalformat,
    /// format, type, source)` overload through a cached method
    /// reference.
    ///
    /// This is the only route to the `ImageBitmap` overload, which
    /// web-sys gates behind a feature this module does not enable for a
    /// single call. The `Function` is looked up once per wasm instance
    /// in a thread-local slot and reused on every later upload, so the
    /// steady-state cost is a direct call with no `Reflect::get`. The
    /// `Function` object lives on `WebGL2RenderingContext.prototype` and
    /// is therefore a per-class singleton, which is what makes keying
    /// the cache on the method name alone sound.
    ///
    /// The cache borrow is never held across a call out to JS. `context`
    /// is host-supplied, and a getter installed on it (or on
    /// `WebGL2RenderingContext.prototype`) runs arbitrary page code that
    /// can re-enter this module; a re-entry under a live `borrow_mut`
    /// would panic the wasm instance, which has no unwinder to catch it.
    /// The hit path therefore clones the cached `Function` out and drops
    /// the guard. The miss path uses `try_borrow_mut` and, if the cell is
    /// already borrowed, skips the write and resolves the `Function` fresh
    /// for this call only — a fresh `Reflect::get` costs one property
    /// lookup, whereas a wrong `Function` would silently render garbage.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The context, as the JS receiver.
    /// - `i32` - The sized internal format.
    /// - `u32` - The pixel format.
    /// - `u32` - The pixel type.
    /// - `&JsValue` - The image source.
    ///
    /// # Returns
    ///
    /// - `Result<JsValue, JsValue>` - The call's own result, so a thrown
    ///   exception surfaces instead of being swallowed.
    fn call_tex_image_2d(
        context: &JsValue,
        internal: i32,
        pixel_format: u32,
        pixel_type: u32,
        source: &JsValue,
    ) -> Result<JsValue, JsValue> {
        let cached: Option<Function> = GL_TEX_IMAGE_2D_CACHE
            .with(|slot: &RefCell<Option<Function>>| slot.borrow().as_ref().cloned());
        let function: Function = match cached {
            Some(function) => function,
            None => Self::resolve_tex_image_2d(context)?,
        };
        function.call6(
            context,
            &JsValue::from_f64(gl_texture_target_2d() as f64),
            &JsValue::from_f64(GL_MIP_LEVEL_ZERO),
            &JsValue::from_f64(internal as f64),
            &JsValue::from_f64(pixel_format as f64),
            &JsValue::from_f64(pixel_type as f64),
            source,
        )
    }

    /// Resolves `WebGL2RenderingContext.prototype.texImage2D` and caches
    /// it when the cache slot is free.
    ///
    /// Split out of [`GlTexture::call_tex_image_2d`](Self::call_tex_image_2d)
    /// so the `Reflect::get` property lookup — arbitrary page code if the
    /// context or its prototype carries a getter — runs with no cache
    /// borrow live. The `try_borrow_mut` write is best-effort: a
    /// concurrent holder means this call already has the `Function` it
    /// needs, so dropping the write costs one lookup next frame and
    /// panicking would cost the whole instance.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The context, as the JS receiver.
    ///
    /// # Returns
    ///
    /// - `Result<Function, JsValue>` - The `texImage2D` `Function`, or
    ///   the `Reflect::get` error when the property is absent.
    fn resolve_tex_image_2d(context: &JsValue) -> Result<Function, JsValue> {
        let name: &JsValue = &JsValue::from_str(GL_METHOD_TEX_IMAGE_2D);
        let found: JsValue = Reflect::get(context, name)?;
        let function: Function = found.unchecked_into();
        Self::store_tex_image_2d(&function);
        Ok(function)
    }

    /// Stores a resolved `texImage2D` `Function` in the thread-local cache
    /// when the slot is free.
    ///
    /// The `try_borrow_mut` write is best-effort and never panics. A
    /// holder at this point is a re-entrant call that has already resolved
    /// the `Function` it needs, so skipping the write costs one property
    /// lookup on some later frame; `borrow_mut()` would panic the wasm
    /// instance, which has no unwinder to catch it. The cache is an
    /// optimisation, so a dropped write is always the cheaper failure.
    ///
    /// # Arguments
    ///
    /// - `&Function` - The resolved `texImage2D` `Function` to cache.
    fn store_tex_image_2d(function: &Function) {
        GL_TEX_IMAGE_2D_CACHE.with(|slot: &RefCell<Option<Function>>| {
            match slot.try_borrow_mut() {
                Ok(mut borrow) => {
                    *borrow = Some(function.clone());
                }
                Err(_already_borrowed) => {}
            }
        });
    }

    /// Sets the wrap and filter modes the texture is sampled with.
    ///
    /// Four `texParameteri` calls, issued once at setup. The
    /// minification filter folds in the mip decision, so selecting a
    /// [`MipmapFilter`] the texture does not have requires a prior
    /// [`GlTexture::generate_mipmap`](super::GlTexture::generate_mipmap);
    /// without the chain the texture is incomplete and samples as solid
    /// black.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to configure against.
    /// - `FilterMode` - The interpolation mode within one mip level.
    /// - `MipmapFilter` - How two adjacent mip levels are combined.
    /// - `AddressMode` - What sampling does outside `[0, 1]`.
    pub fn set_parameters(
        &self,
        context: &WebGl2RenderingContext,
        filter: FilterMode,
        mipmap: MipmapFilter,
        address: AddressMode,
    ) {
        let target: u32 = gl_texture_target_2d();
        let wrap: i32 = gl_address_mode(address) as i32;
        context.tex_parameteri(
            target,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            gl_min_filter(filter, mipmap) as i32,
        );
        context.tex_parameteri(
            target,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            gl_filter_mode(filter) as i32,
        );
        context.tex_parameteri(target, WebGl2RenderingContext::TEXTURE_WRAP_S, wrap);
        context.tex_parameteri(target, WebGl2RenderingContext::TEXTURE_WRAP_T, wrap);
    }

    /// Builds the full mip chain and switches the texture to a mip-aware
    /// minification filter.
    ///
    /// Required before any filter that samples across levels: a texture
    /// with a mip-aware min filter and no chain is incomplete, and an
    /// incomplete texture samples as solid black. Costs a
    /// full-texture-sized pass, so it belongs at load time rather than in
    /// a per-frame path.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to generate against.
    /// - `FilterMode` - The interpolation mode within one mip level.
    /// - `MipmapFilter` - How two adjacent mip levels are combined.
    /// - `AddressMode` - What sampling does outside `[0, 1]`.
    pub fn generate_mipmap(
        &mut self,
        context: &WebGl2RenderingContext,
        filter: FilterMode,
        mipmap: MipmapFilter,
        address: AddressMode,
    ) {
        let mut levels: u32 = 1;
        let mut halved: u32 = self.get_width().max(self.get_height()).max(1);
        while halved > 1 {
            halved /= 2;
            levels += 1;
        }
        self.set_levels(levels);
        self.set_mipmapped(true);
        context.generate_mipmap(gl_texture_target_2d());
        self.set_parameters(context, filter, mipmap, address);
    }

    /// Writes a sub-rectangle of the texture without re-allocating it.
    ///
    /// The write is rejected when the rectangle falls outside the
    /// texture, so a caller cannot drive the driver past its own
    /// allocation. The rectangle is assumed to match the given format,
    /// which is the same assumption every sub-image call makes.
    ///
    /// The rectangle is carried as a [`GlScissor`] rather than as four
    /// loose arguments because its bounds are checked against the
    /// texture's own dimensions: passing the box as one value is what
    /// makes it a single value to compare, rather than four
    /// independently-supplied edges that could disagree.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&GlScissor` - The sub-rectangle, in texels, whose top-left is
    ///   the origin.
    /// - `GpuTextureFormat` - The texel format `data` is laid out in.
    /// - `&[u8]` - The tightly packed pixels for the sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the write was issued, `false` when the
    ///   rectangle fell outside the texture.
    pub fn update(
        &self,
        context: &WebGl2RenderingContext,
        region: &GlScissor,
        format: GpuTextureFormat,
        data: &[u8],
    ) -> bool {
        let x: u32 = (*region.get_x()).max(0) as u32;
        let y: u32 = (*region.get_y()).max(0) as u32;
        let width: u32 = (*region.get_width()).max(0) as u32;
        let height: u32 = (*region.get_height()).max(0) as u32;
        let inside: bool = x.saturating_add(width) <= self.get_width()
            && y.saturating_add(height) <= self.get_height();
        if !inside {
            return false;
        }
        let (_, pixel_format, pixel_type): (i32, u32, u32) = gl_texture_layout(format);
        let _: Result<(), JsValue> = context
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                gl_texture_target_2d(),
                0,
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                pixel_format,
                pixel_type,
                Some(data),
            );
        true
    }

    /// Releases the texture's driver-side storage and marks the wrapper
    /// empty.
    ///
    /// The dimensions are zeroed so a stale size can never be mistaken
    /// for a live allocation by a later sub-image write.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to release against.
    pub fn delete(&mut self, context: &WebGl2RenderingContext) {
        context.delete_texture(Some(self.get_texture()));
        self.set_width(0);
        self.set_height(0);
        self.set_mipmapped(false);
    }
}

/// Implements vertex array binding and attribute description.
///
/// # Performance
///
/// Everything a draw needs to know about its vertex data, which buffer
/// feeds which attribute at what stride and offset and how far each
/// instance advances, is recorded once into the array object. A draw
/// then costs one `bindVertexArray` instead of roughly three calls per
/// attribute, which is what makes it affordable to change mesh between
/// draws in the same frame.
impl GlVertexArray {
    /// Allocates an unbound vertex array object.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    ///
    /// # Returns
    ///
    /// - `Option<GlVertexArray>` - The allocated array object, or `None`
    ///   when the driver refused to create one.
    pub fn create(context: &WebGl2RenderingContext) -> Option<GlVertexArray> {
        let vao: WebGlVertexArrayObject = context.create_vertex_array()?;
        Some(GlVertexArray { vao })
    }

    /// Makes the array object current, so subsequent attribute and
    /// buffer bindings are recorded into it rather than into the
    /// context's default state.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_vertex_array(Some(self.get_vao()));
    }

    /// Binds `buffer` and describes every attribute in `layout` against
    /// it.
    ///
    /// A layout whose step mode is `Instance` sets a per-attribute
    /// divisor of one, which is what makes
    /// [`WebGl2Backend::draw_elements_instanced`](super::WebGl2Backend::draw_elements_instanced)
    /// advance the buffer once per instance rather than once per vertex.
    /// The divisor is set on every attribute rather than assumed fresh,
    /// because reusing an attribute index with a different step mode
    /// would otherwise inherit the previous divisor.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to record into.
    /// - `&GlBuffer` - The buffer the attributes read from.
    /// - `&VertexBufferLayout` - The stride, step mode, and attribute
    ///   list to record.
    pub fn set_layout(
        &self,
        context: &WebGl2RenderingContext,
        buffer: &GlBuffer,
        layout: &VertexBufferLayout,
    ) {
        self.bind(context);
        buffer.bind(context, WebGl2RenderingContext::ARRAY_BUFFER);
        let stride: i32 = layout.array_stride as i32;
        let divisor: u32 = match layout.step_mode {
            VertexStepMode::Vertex => 0,
            VertexStepMode::Instance => 1,
        };
        for attribute in layout.attributes.iter() {
            let index: u32 = attribute.shader_location;
            let (components, attribute_type, normalized): (u32, u32, bool) =
                gl_attribute_layout(attribute.format);
            context.enable_vertex_attrib_array(index);
            context.vertex_attrib_pointer_with_i32(
                index,
                components as i32,
                attribute_type,
                normalized,
                stride,
                attribute.offset as i32,
            );
            context.vertex_attrib_divisor(index, divisor);
        }
    }

    /// Binds a second buffer and describes the attributes that read from
    /// it, leaving the ones already recorded alone.
    ///
    /// The case a single buffer cannot express is a mesh whose positions
    /// advance per vertex while its per-instance transforms live in a
    /// separate buffer. One VAO records one array-buffer binding, so the
    /// second buffer is bound and described inside the same VAO.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to record into.
    /// - `&GlBuffer` - The buffer the attributes read from.
    /// - `&VertexBufferLayout` - The attributes to record, whose step
    ///   mode should be `Instance`.
    pub fn set_instance_layout(
        &self,
        context: &WebGl2RenderingContext,
        buffer: &GlBuffer,
        layout: &VertexBufferLayout,
    ) {
        self.set_layout(context, buffer, layout);
    }

    /// Records an explicit per-attribute divisor, overriding whatever
    /// the layout's step mode implied.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to record into.
    /// - `u32` - The attribute index, as recorded on the layout.
    /// - `u32` - The divisor: zero per vertex, one per instance.
    pub fn set_divisor(&self, context: &WebGl2RenderingContext, index: u32, divisor: u32) {
        self.bind(context);
        context.vertex_attrib_divisor(index, divisor);
    }

    /// Unbinds the array object, restoring the context's default vertex
    /// state.
    ///
    /// Worth calling before a draw that reads no vertex data at all, a
    /// full-screen triangle generated from `gl_VertexID` say, because
    /// leaving an array object bound would keep its attribute buffers
    /// alive in the driver's state.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to unbind against.
    pub fn unbind(context: &WebGl2RenderingContext) {
        context.bind_vertex_array(None);
    }

    /// Releases the array object's recorded state.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to release against.
    pub fn delete(&self, context: &WebGl2RenderingContext) {
        context.delete_vertex_array(Some(self.get_vao()));
    }
}

/// Implements framebuffer assembly, completeness checking, and
/// rebinding.
///
/// # Performance
///
/// The stored dimensions are what make a rebind after a window resize
/// cheap: a caller can compare the requested size against the stored one
/// and skip the whole re-assembly when they still agree, so binding a
/// framebuffer once per frame costs one `bindFramebuffer` and two integer
/// comparisons.
impl GlFramebuffer {
    /// Allocates a framebuffer with a color texture and, when `depth` is
    /// `Some`, a depth renderbuffer of the same size.
    ///
    /// The color attachment is always a [`GlTexture`], never a
    /// renderbuffer, so the rendered result can be sampled by a later
    /// pass or read back without a resolve blit. The depth attachment is
    /// a renderbuffer, which is cheaper than a depth texture and is
    /// correct for a depth buffer that is written and tested but never
    /// sampled.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    /// - `u32` - The width in pixels.
    /// - `u32` - The height in pixels.
    /// - `GpuTextureFormat` - The color attachment's format.
    /// - `Option<GpuTextureFormat>` - The depth renderbuffer's format,
    ///   or `None` for a color-only target.
    ///
    /// # Returns
    ///
    /// - `Option<GlFramebuffer>` - The assembled framebuffer, or `None`
    ///   when the driver refused to allocate one of its parts.
    pub fn create(
        context: &WebGl2RenderingContext,
        width: u32,
        height: u32,
        color_format: GpuTextureFormat,
        depth_format: Option<GpuTextureFormat>,
    ) -> Option<GlFramebuffer> {
        let color: GlTexture = GlTexture::create(context, width, height, color_format, &[])?;
        let framebuffer: WebGlFramebuffer = context.create_framebuffer()?;
        let mut depth: Option<WebGlRenderbuffer> = None;
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));
        context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            GL_DEFAULT_COLOR_ATTACHMENT,
            gl_texture_target_2d(),
            Some(color.get_texture()),
            0,
        );
        if let Some(format) = depth_format {
            let renderbuffer: WebGlRenderbuffer = context.create_renderbuffer()?;
            context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, Some(&renderbuffer));
            context.renderbuffer_storage(
                WebGl2RenderingContext::RENDERBUFFER,
                gl_renderbuffer_format(format),
                width as i32,
                height as i32,
            );
            let attachment: u32 = if format == GpuTextureFormat::Depth24PlusStencil8 {
                WebGl2RenderingContext::DEPTH_STENCIL_ATTACHMENT
            } else {
                WebGl2RenderingContext::DEPTH_ATTACHMENT
            };
            context.framebuffer_renderbuffer(
                WebGl2RenderingContext::FRAMEBUFFER,
                attachment,
                WebGl2RenderingContext::RENDERBUFFER,
                Some(&renderbuffer),
            );
            depth = Some(renderbuffer);
        }
        context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, None);
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        Some(GlFramebuffer {
            framebuffer,
            depth,
            color,
            width,
            height,
        })
    }

    /// Binds the framebuffer as both the draw and read target, and
    /// checks that it is complete.
    ///
    /// Binding both targets is deliberate: WebGL 2 separates
    /// `DRAW_FRAMEBUFFER` from `READ_FRAMEBUFFER`, and a framebuffer
    /// bound to only one of them makes a later `readPixels` silently
    /// return the default framebuffer's contents instead. The status
    /// check is what turns a driver-side `FRAMEBUFFER_INCOMPLETE_*`,
    /// which otherwise shows up as a frame that renders nothing at all,
    /// into a value the caller can react to.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    ///
    /// # Returns
    ///
    /// - `GlFramebufferStatus` - Whether the framebuffer is renderable.
    pub fn bind(&self, context: &WebGl2RenderingContext) -> GlFramebufferStatus {
        let handle: &WebGlFramebuffer = self.get_framebuffer();
        context.bind_framebuffer(WebGl2RenderingContext::DRAW_FRAMEBUFFER, Some(handle));
        context.bind_framebuffer(WebGl2RenderingContext::READ_FRAMEBUFFER, Some(handle));
        gl_framebuffer_status(context.check_framebuffer_status(WebGl2RenderingContext::FRAMEBUFFER))
    }

    /// Reports whether the stored attachments still match `width` by
    /// `height`.
    ///
    /// The cheap half of a resize check: a caller that compares this
    /// before [`GlFramebuffer::create`](super::GlFramebuffer::create) can
    /// skip re-allocating every attachment just to rebind an unchanged
    /// render target once per frame.
    ///
    /// # Arguments
    ///
    /// - `u32` - The width to test against.
    /// - `u32` - The height to test against.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the attachments already match.
    pub fn matches(&self, width: u32, height: u32) -> bool {
        self.get_width() == width && self.get_height() == height
    }

    /// Releases the framebuffer and its depth renderbuffer.
    ///
    /// The color texture is deliberately left alone: it is a
    /// [`GlTexture`] the caller also owns, and a render target whose
    /// contents are about to be sampled must outlive its framebuffer.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to release against.
    pub fn delete(&mut self, context: &WebGl2RenderingContext) {
        if let Some(renderbuffer) = self.try_get_depth() {
            context.delete_renderbuffer(Some(&renderbuffer));
        }
        context.delete_framebuffer(Some(self.get_framebuffer()));
        self.set_width(0);
        self.set_height(0);
    }
}

/// Implements shader compilation, program linking, and cached uniform
/// upload.
///
/// # Performance
///
/// Every uniform name this program has ever been asked about is resolved
/// exactly once and cached, including the negative result for a name the
/// GLSL compiler optimized out. A per-frame `set_uniform_1f` is therefore
/// a hash lookup plus the upload, with no trip into the GL frontend's
/// uniform table. Matrix uploads additionally go through a fixed-size
/// stack array rather than a heap one, so uploading a [`Matrix4x4`]
/// costs no allocation on any frame, including the first.
impl GlProgram {
    /// Compiles and links a vertex and fragment shader into a program.
    ///
    /// Both shader objects are deleted once linking succeeds, since the
    /// program keeps the compiled code and the shader objects are pure
    /// intermediate products. On failure the browser's info log is
    /// returned verbatim, because a GLSL diagnostic is only actionable
    /// in the driver's own wording.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to compile against.
    /// - `&str` - The vertex shader source.
    /// - `&str` - The fragment shader source.
    ///
    /// # Returns
    ///
    /// - `Result<GlProgram, WebGlProgramError>` - The linked program with
    ///   an empty uniform cache, or the compile and link info log.
    pub fn create(
        context: &WebGl2RenderingContext,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<GlProgram, WebGlProgramError> {
        let vertex_shader: WebGlShader =
            GlProgram::compile(context, GlShaderKind::Vertex, vertex_source)?;
        let fragment_shader: WebGlShader =
            GlProgram::compile(context, GlShaderKind::Fragment, fragment_source)?;
        let program: WebGlProgram = match context.create_program() {
            Some(value) => value,
            None => {
                return Err(WebGlProgramError::ProgramLink(
                    GL_PROGRAM_CREATE_FAILED.to_string(),
                ));
            }
        };
        context.attach_shader(&program, &vertex_shader);
        context.attach_shader(&program, &fragment_shader);
        context.link_program(&program);
        let linked: bool = context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or_default();
        context.delete_shader(Some(&vertex_shader));
        context.delete_shader(Some(&fragment_shader));
        if !linked {
            let log: String = context.get_program_info_log(&program).unwrap_or_default();
            context.delete_program(Some(&program));
            return Err(WebGlProgramError::ProgramLink(log));
        }
        Ok(GlProgram {
            program,
            uniforms: HashMap::new(),
            matrix_scratch: [0.0; GL_MAT4_FLOATS],
            blocks: HashMap::new(),
        })
    }

    /// Compiles one shader stage and returns it, or the compile info log.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to compile against.
    /// - `GlShaderKind` - The stage to compile for.
    /// - `&str` - The GLSL source.
    ///
    /// # Returns
    ///
    /// - `Result<WebGlShader, WebGlProgramError>` - The compiled shader,
    ///   or the compile info log.
    fn compile(
        context: &WebGl2RenderingContext,
        kind: GlShaderKind,
        source: &str,
    ) -> Result<WebGlShader, WebGlProgramError> {
        let shader: WebGlShader = match context.create_shader(gl_shader_type(kind)) {
            Some(value) => value,
            None => {
                return Err(WebGlProgramError::ShaderCompile(
                    GL_SHADER_CREATE_FAILED.to_string(),
                ));
            }
        };
        context.shader_source(&shader, source);
        context.compile_shader(&shader);
        let compiled: bool = context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or_default();
        if !compiled {
            let log: String = context.get_shader_info_log(&shader).unwrap_or_default();
            context.delete_shader(Some(&shader));
            return Err(WebGlProgramError::ShaderCompile(log));
        }
        Ok(shader)
    }

    /// Makes the program current, so subsequent uniform writes and
    /// draws target it.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(self.get_program()));
    }

    /// Resolves a uniform's location once and caches it, including the
    /// case where the GLSL compiler removed the uniform entirely.
    ///
    /// Caching the `None` matters as much as caching the location: a
    /// uniform the compiler optimized out is the common case for a
    /// per-feature flag, and re-querying it every frame would walk the
    /// program's uniform table for a result that can never change.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to resolve against.
    /// - `&str` - The uniform name, with an explicit `[0]` index for
    ///   array uniforms per the `getUniformLocation` spec.
    ///
    /// # Returns
    ///
    /// - `Option<WebGlUniformLocation>` - The location, or `None` when
    ///   the uniform does not exist in the program.
    pub fn uniform(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
    ) -> Option<WebGlUniformLocation> {
        match self.get_uniforms().get(name) {
            Some(cached) => cached.clone(),
            None => {
                let resolved: Option<WebGlUniformLocation> =
                    context.get_uniform_location(self.get_program(), name);
                let mut table: HashMap<String, Option<WebGlUniformLocation>> =
                    self.get_uniforms().clone();
                table.insert(name.to_string(), resolved.clone());
                self.set_uniforms(table);
                resolved
            }
        }
    }

    /// Uploads a `float` uniform through its cached location.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&str` - The uniform name.
    /// - `f32` - The value to upload.
    pub fn set_uniform_1f(&mut self, context: &WebGl2RenderingContext, name: &str, value: f32) {
        let location: Option<WebGlUniformLocation> = self.uniform(context, name);
        context.uniform1f(location.as_ref(), value);
    }

    /// Uploads a `vec2` uniform through its cached location.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&str` - The uniform name.
    /// - `f32` - The x component.
    /// - `f32` - The y component.
    pub fn set_uniform_2f(&mut self, context: &WebGl2RenderingContext, name: &str, x: f32, y: f32) {
        let location: Option<WebGlUniformLocation> = self.uniform(context, name);
        context.uniform2f(location.as_ref(), x, y);
    }

    /// Uploads a `vec3` uniform through its cached location.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&str` - The uniform name.
    /// - `f32` - The x component.
    /// - `f32` - The y component.
    /// - `f32` - The z component.
    pub fn set_uniform_3f(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let location: Option<WebGlUniformLocation> = self.uniform(context, name);
        context.uniform3f(location.as_ref(), x, y, z);
    }

    /// Uploads a `vec4` uniform through its cached location.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&str` - The uniform name.
    /// - `f32` - The x component.
    /// - `f32` - The y component.
    /// - `f32` - The z component.
    /// - `f32` - The w component.
    pub fn set_uniform_4f(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        let location: Option<WebGlUniformLocation> = self.uniform(context, name);
        context.uniform4f(location.as_ref(), x, y, z, w);
    }

    /// Uploads a `mat4` uniform from the engine's [`Matrix4x4`], with no
    /// allocation.
    ///
    /// This is the single most important primitive the WebGL path was
    /// missing: the engine's matrices are `f64` and GL's uniforms are
    /// `f32`, so every upload needs a conversion, and doing that into a
    /// freshly allocated `Vec` would put a heap allocation in the middle
    /// of the draw path. The conversion goes through a fixed-size stack
    /// array, so it is a sixteen-element copy into memory that already
    /// exists.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&str` - The uniform name.
    /// - `&Matrix4x4` - The matrix to upload.
    pub fn set_uniform_mat4(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        matrix: &Matrix4x4,
    ) {
        let location: Option<WebGlUniformLocation> = self.uniform(context, name);
        let mut scratch: [f32; GL_MAT4_FLOATS] = [0.0; GL_MAT4_FLOATS];
        gl_matrix4_into_f32(matrix, &mut scratch);
        context.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &scratch);
    }

    /// Uploads a flat `vec4` array uniform from a caller-owned slice.
    ///
    /// The slice is handed to the driver directly, so there is no
    /// staging copy and no allocation: the only cost is the borrow for
    /// the duration of the call. `data.len()` should be a multiple of
    /// four, since a `vec4` array element is four floats; a length that
    /// is not is the caller's bug, and GL rejects it.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `&str` - The uniform name, with an explicit `[0]` index.
    /// - `&[f32]` - The packed float data.
    pub fn set_uniform_vec4_array(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        data: &[f32],
    ) {
        let location: Option<WebGlUniformLocation> = self.uniform(context, name);
        context.uniform4fv_with_f32_array(location.as_ref(), data);
    }

    /// Points a named uniform block at a binding index, and caches the
    /// block's index so the lookup happens once.
    ///
    /// The driver's answer for a block name the program does not declare
    /// is `UNIFORM_BLOCK_INDEX`, which is not a valid index. That
    /// negative result is deliberately not cached: a miss here usually
    /// means the shader changed under the program, and caching it would
    /// make the mismatch permanent rather than recoverable on the next
    /// call.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    /// - `&str` - The block name as written in the shader.
    /// - `u32` - The binding point the block should read from.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the block exists and was bound.
    pub fn bind_uniform_block(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        binding: u32,
    ) -> bool {
        let cached: Option<u32> = self.get_blocks().get(name).copied();
        let index: Option<u32> = match cached {
            Some(value) => Some(value),
            None => {
                let resolved: u32 = context.get_uniform_block_index(self.get_program(), name);
                if resolved == WebGl2RenderingContext::UNIFORM_BLOCK_INDEX {
                    None
                } else {
                    let mut table: HashMap<String, u32> = self.get_blocks().clone();
                    table.insert(name.to_string(), resolved);
                    self.set_blocks(table);
                    Some(resolved)
                }
            }
        };
        match index {
            Some(value) => {
                context.uniform_block_binding(self.get_program(), value, binding);
                true
            }
            None => false,
        }
    }

    /// Releases the program and drops its uniform cache.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to release against.
    pub fn delete(&mut self, context: &WebGl2RenderingContext) {
        context.delete_program(Some(self.get_program()));
        self.set_uniforms(HashMap::new());
        self.set_blocks(HashMap::new());
    }
}

/// Implements allocation, record upload, and binding-point management
/// for a uniform buffer.
///
/// # Performance
///
/// The capacity check mirrors [`GlBuffer`]'s, so a block whose record
/// count grows re-orphans rather than reallocating. Updating one record
/// at a fixed offset is what makes a per-object transform upload
/// possible without rewriting the whole block.
impl GlUniformBlock {
    /// Allocates a uniform buffer of at least `size` bytes and binds it
    /// to `binding`.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to allocate against.
    /// - `u32` - The byte capacity to reserve.
    /// - `u32` - The binding point the shader reads from.
    ///
    /// # Returns
    ///
    /// - `Option<GlUniformBlock>` - The allocated block, or `None` when
    ///   the driver refused to create one.
    pub fn create(
        context: &WebGl2RenderingContext,
        size: u32,
        binding: u32,
    ) -> Option<GlUniformBlock> {
        let capacity: u32 = size.max(1);
        let buffer: GlBuffer = GlBuffer::create(
            context,
            WebGl2RenderingContext::UNIFORM_BUFFER,
            capacity,
            WebGl2RenderingContext::DYNAMIC_DRAW,
        )?;
        let block: GlUniformBlock = GlUniformBlock {
            buffer: buffer.get_buffer().clone(),
            binding,
            capacity,
        };
        block.bind(context);
        Some(block)
    }

    /// Binds the block's buffer to its recorded binding point.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer_base(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            self.get_binding(),
            Some(self.get_buffer()),
        );
    }

    /// Writes `data` at `offset` bytes, re-allocating only if the write
    /// would outgrow the block.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `u32` - The byte offset to write at.
    /// - `&[u8]` - The bytes to write.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the write was issued.
    pub fn update(&mut self, context: &WebGl2RenderingContext, offset: u32, data: &[u8]) -> bool {
        let end: u32 = offset.saturating_add(data.len() as u32);
        if end > self.get_capacity() {
            context.bind_buffer(
                WebGl2RenderingContext::UNIFORM_BUFFER,
                Some(self.get_buffer()),
            );
            context.buffer_data_with_i32(
                WebGl2RenderingContext::UNIFORM_BUFFER,
                end as i32,
                WebGl2RenderingContext::DYNAMIC_DRAW,
            );
            self.set_capacity(end);
        }
        context.bind_buffer(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            Some(self.get_buffer()),
        );
        context.buffer_sub_data_with_i32_and_u8_array(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            offset as i32,
            data,
        );
        true
    }

    /// Writes one [`Matrix4x4`] into the record at `offset`, in bytes.
    ///
    /// The conversion goes through a fixed-size stack array rather than a
    /// heap one, so uploading a per-object transform costs no
    /// allocation. The bound is checked rather than trusted, because an
    /// out-of-range offset would otherwise hand the driver a sub-range
    /// write that silently truncates the transform.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to upload through.
    /// - `u32` - The byte offset of the record to write.
    /// - `&Matrix4x4` - The transform to upload.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the record fit inside the block.
    pub fn set_mat4(
        &mut self,
        context: &WebGl2RenderingContext,
        offset: u32,
        matrix: &Matrix4x4,
    ) -> bool {
        if offset.saturating_add(GL_MAT4_BYTES) > self.get_capacity() {
            return false;
        }
        let mut scratch: [f32; GL_MAT4_FLOATS] = [0.0; GL_MAT4_FLOATS];
        gl_matrix4_into_f32(matrix, &mut scratch);
        let mut bytes: [u8; GL_MAT4_BYTES as usize] = [0; GL_MAT4_BYTES as usize];
        for (index, value) in scratch.iter().enumerate() {
            let raw: [u8; GL_F32_SIZE as usize] = value.to_ne_bytes();
            let base: usize = index * GL_F32_SIZE as usize;
            for (slot, byte) in bytes[base..base + GL_F32_SIZE as usize]
                .iter_mut()
                .zip(raw.iter())
            {
                *slot = *byte;
            }
        }
        self.update(context, offset, &bytes)
    }

    /// Releases the block's buffer.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to release against.
    pub fn delete(&mut self, context: &WebGl2RenderingContext) {
        context.bind_buffer_base(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            self.get_binding(),
            None,
        );
        context.delete_buffer(Some(self.get_buffer()));
        self.set_capacity(0);
    }
}

/// Implements default construction for the fixed-function state shadow.
impl GlRenderState {
    /// Builds the state a freshly created WebGL 2 context is actually
    /// in.
    ///
    /// Every value here is one the driver has already set up, not a zero
    /// placeholder, and that is the point: seeding the diffing shadow
    /// with the real defaults means the first
    /// [`WebGl2Backend::apply_state`](super::WebGl2Backend::apply_state)
    /// emits only the calls that genuinely differ from what the driver
    /// already did, rather than a dozen redundant re-assertions. Getting
    /// a single default wrong would instead leave one field permanently
    /// un-applied, because the diff would consider it unchanged forever.
    ///
    /// Depth test off, depth write on, `LEQUAL` once enabled; blending
    /// off; culling off with counter-clockwise front faces; all four
    /// color channels writable; scissor off; viewport covering the given
    /// size.
    ///
    /// # Arguments
    ///
    /// - `i32` - The viewport width in pixels.
    /// - `i32` - The viewport height in pixels.
    ///
    /// # Returns
    ///
    /// - `GlRenderState` - The state a new context starts in.
    pub fn context_defaults(width: i32, height: i32) -> GlRenderState {
        GlRenderState {
            depth: GlDepthState {
                enabled: false,
                compare: CompareFunction::LessEqual,
                write_enabled: true,
            },
            blend: GlBlendState::default(),
            cull: GlCullState::default(),
            color_mask: GlColorMask {
                bits: GL_COLOR_WRITE_ALL,
            },
            scissor: None,
            viewport: GlViewport {
                x: 0,
                y: 0,
                width,
                height,
            },
        }
    }
}

/// Implements construction and lifecycle for the WebGL 2 backend.
impl WebGl2Backend {
    /// Builds a backend from a render configuration, resolving the
    /// canvas, scaling the backing store by the device pixel ratio, and
    /// acquiring the `webgl2` context.
    ///
    /// The shadow state is seeded with the values a freshly created GL
    /// context actually starts in rather than with all-zero
    /// placeholders, so the first [`WebGl2Backend::apply_state`] emits
    /// only the calls that genuinely differ from what the driver already
    /// set up.
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGl2Backend, WebGl2InitError>` - The backend, or a
    ///   typed error describing the specific failure.
    pub fn init(config: &RenderConfig) -> Result<WebGl2Backend, WebGl2InitError> {
        let selector: String = config.get_canvas_selector().clone();
        let Some(window_value) = window() else {
            return Err(WebGl2InitError::CanvasNotFound(
                config.get_canvas_selector().clone(),
            ));
        };
        let Some(document_value) = window_value.document() else {
            return Err(WebGl2InitError::CanvasNotFound(
                config.get_canvas_selector().clone(),
            ));
        };
        let element: Element = document_value
            .query_selector(selector.as_str())
            .map_err(|_| WebGl2InitError::CanvasQuery(selector.to_string()))?
            .ok_or_else(|| WebGl2InitError::CanvasNotFound(config.get_canvas_selector().clone()))?;
        let canvas: HtmlCanvasElement = element.unchecked_into();
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (config.get_width() * dpr).round() as u32;
        let physical_height: u32 = (config.get_height() * dpr).round() as u32;
        canvas.set_width(physical_width);
        canvas.set_height(physical_height);
        let context_object: Object = canvas
            .get_context(GL_CONTEXT_ID_WEBGL2)
            .map_err(|_| WebGl2InitError::ContextLookup(selector.clone()))?
            .ok_or(WebGl2InitError::ContextUnavailable)?;
        let context: WebGl2RenderingContext = context_object
            .dyn_into()
            .map_err(|_| WebGl2InitError::ContextCast)?;
        context.viewport(0, 0, physical_width as i32, physical_height as i32);
        Ok(WebGl2Backend {
            canvas,
            context,
            shadow: GlRenderState::context_defaults(physical_width as i32, physical_height as i32),
            active_unit: GL_TEXTURE_UNIT_NONE,
            bound_program: JsValue::UNDEFINED,
            clear_color_field: Color::new(0.0, 0.0, 0.0, 1.0),
            readback: Vec::new(),
        })
    }

    /// Reports whether the browser can create a WebGL 2 context at all.
    ///
    /// Creates a throwaway off-DOM canvas and requests a `webgl2`
    /// context. No shaders are compiled and nothing on the page is
    /// touched, so this is cheap enough to call as a capability probe
    /// before deciding which backend to construct.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a `webgl2` context could be acquired.
    pub fn is_available() -> bool {
        let Some(window_value) = window() else {
            return false;
        };
        let Some(document_value) = window_value.document() else {
            return false;
        };
        let element: Element = match document_value.create_element(GL_DOM_TAG_CANVAS) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let canvas: HtmlCanvasElement = element.unchecked_into();
        canvas
            .get_context(GL_CONTEXT_ID_WEBGL2)
            .ok()
            .flatten()
            .is_some()
    }

    /// Resizes the canvas backing store and re-points the shadow's
    /// viewport at the new size.
    ///
    /// The viewport is written into the shadow rather than issued
    /// immediately, because a resize changes the canvas dimensions and
    /// the viewport that was correct for the old ones must be
    /// re-asserted against the new ones even if the caller's own state
    /// value did not change. Seeding the shadow makes the next
    /// [`WebGl2Backend::apply_state`] emit it exactly once.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new physical pixel width, already multiplied by the
    ///   device pixel ratio.
    /// - `u32` - The new physical pixel height.
    pub fn resize(&mut self, width: u32, height: u32) {
        let mut state: GlRenderState = *self.get_shadow();
        state.viewport = GlViewport {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        };
        self.set_shadow(state);
    }

    /// Resizes the canvas backing store and re-asserts the GL viewport
    /// immediately, rather than deferring it to the next
    /// [`WebGl2Backend::apply_state`].
    ///
    /// [`WebGl2Backend::resize`] is the cheaper call and is what a render
    /// loop should use, because the viewport it seeds is emitted once by
    /// the following state apply. This variant exists for the caller that
    /// resizes the DOM canvas itself and needs the GL viewport correct
    /// before the next paint, with no intervening apply.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new physical pixel width, already multiplied by the
    ///   device pixel ratio.
    /// - `u32` - The new physical pixel height.
    pub fn resize_now(&mut self, width: u32, height: u32) {
        self.resize(width, height);
        self.get_context()
            .viewport(0, 0, width as i32, height as i32);
    }

    /// Reports whether the context has been lost, which happens when the
    /// browser reclaims the GPU: a tab backgrounded for long enough, a
    /// driver reset, a device change.
    ///
    /// Worth checking once per frame. Every subsequent GL call against a
    /// lost context is a no-op that reports no error, so a renderer that
    /// does not check silently draws nothing with a clean console.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the context is lost and must be rebuilt.
    pub fn is_context_lost(&self) -> bool {
        self.get_context().is_context_lost()
    }
}

/// Implements the state-diffing apply and the frame-level entry points.
///
/// Every method here exists to make the per-frame cost proportional to
/// what actually changed rather than to what was asked for.
impl WebGl2Backend {
    /// Applies a whole [`GlRenderState`], issuing a GL call only for
    /// each field that differs from the currently-bound state.
    ///
    /// The diff is against a shadow copy of the last applied state, not
    /// against the driver, so no `getParameter` round trip is needed: the
    /// driver is known to agree with the shadow because the shadow is
    /// only ever updated after the calls that establish it. This is the
    /// single largest saving in the whole backend, since a batch of a
    /// thousand draws sharing one state pays for it once instead of for a
    /// thousand `enable` / `depthFunc` / `blendFunc` / `cullFace` /
    /// `colorMask` sequences, each of which the driver re-validates.
    ///
    /// # Arguments
    ///
    /// - `&GlRenderState` - The state to make current.
    pub fn apply_state(&mut self, state: &GlRenderState) {
        let context: &WebGl2RenderingContext = self.get_context();
        let current: GlRenderState = *self.get_shadow();
        let depth_toggled: bool = current.depth.enabled != state.depth.enabled;
        if depth_toggled {
            WebGl2Backend::set_capability(
                context,
                WebGl2RenderingContext::DEPTH_TEST,
                state.depth.enabled,
            );
        }
        if depth_toggled || current.depth.compare != state.depth.compare {
            context.depth_func(gl_compare_function(state.depth.compare));
        }
        if current.depth.write_enabled != state.depth.write_enabled {
            let write: bool = state.depth.write_enabled;
            context.depth_mask(write);
        }
        let blend_toggled: bool = current.blend.enabled != state.blend.enabled;
        if blend_toggled {
            WebGl2Backend::set_capability(
                context,
                WebGl2RenderingContext::BLEND,
                state.blend.enabled,
            );
        }
        if blend_toggled
            || current.blend.color != state.blend.color
            || current.blend.alpha != state.blend.alpha
        {
            WebGl2Backend::apply_blend(context, state);
        }
        if current.cull.mode != state.cull.mode {
            let culling: bool = state.cull.mode != CullMode::None;
            WebGl2Backend::set_capability(context, WebGl2RenderingContext::CULL_FACE, culling);
            if culling {
                context.cull_face(gl_cull_face(state.cull.mode));
            }
        }
        if current.cull.front_face != state.cull.front_face {
            context.front_face(gl_front_face(state.cull.front_face));
        }
        if current.color_mask.bits != state.color_mask.bits {
            let bits: u32 = state.color_mask.bits;
            context.color_mask(
                bits & GL_COLOR_CHANNEL_RED != 0,
                bits & GL_COLOR_CHANNEL_GREEN != 0,
                bits & GL_COLOR_CHANNEL_BLUE != 0,
                bits & GL_COLOR_CHANNEL_ALPHA != 0,
            );
        }
        if current.scissor != state.scissor {
            match state.scissor {
                Some(rectangle) => {
                    WebGl2Backend::set_capability(
                        context,
                        WebGl2RenderingContext::SCISSOR_TEST,
                        true,
                    );
                    context.scissor(
                        *rectangle.get_x(),
                        *rectangle.get_y(),
                        *rectangle.get_width(),
                        *rectangle.get_height(),
                    );
                }
                None => {
                    WebGl2Backend::set_capability(
                        context,
                        WebGl2RenderingContext::SCISSOR_TEST,
                        false,
                    );
                }
            }
        }
        if current.viewport != state.viewport {
            context.viewport(
                *state.viewport.get_x(),
                *state.viewport.get_y(),
                *state.viewport.get_width(),
                *state.viewport.get_height(),
            );
        }
        self.set_shadow(*state);
    }

    /// Issues the pair of calls that establish a blend state.
    ///
    /// Split out of [`WebGl2Backend::apply_state`] so the diff block
    /// above reads as a table of "what changed" rather than as a wall of
    /// GL calls.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to write through.
    /// - `&GlRenderState` - The state whose blend is to be applied.
    fn apply_blend(context: &WebGl2RenderingContext, state: &GlRenderState) {
        context.blend_equation_separate(
            gl_blend_equation(state.blend.color.get_operation()),
            gl_blend_equation(state.blend.alpha.get_operation()),
        );
        context.blend_func_separate(
            gl_blend_factor(state.blend.color.get_source()),
            gl_blend_factor(state.blend.color.get_destination()),
            gl_blend_factor(state.blend.alpha.get_source()),
            gl_blend_factor(state.blend.alpha.get_destination()),
        );
    }

    /// Enables or disables one GL capability, used by the state diff.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to write through.
    /// - `u32` - The capability enum to toggle.
    /// - `bool` - `true` to enable, `false` to disable.
    fn set_capability(context: &WebGl2RenderingContext, capability: u32, enabled: bool) {
        if enabled {
            context.enable(capability);
        } else {
            context.disable(capability);
        }
    }

    /// Resets the shadow to the context's own initial state, which
    /// forces the next apply to re-assert everything.
    ///
    /// Needed after anything that can change GL state behind the
    /// backend's back: a context restore, a debug extension, or a
    /// driver's own reset. Without it the shadow would claim a state the
    /// driver no longer holds and every diffed call would be skipped.
    ///
    /// # Arguments
    ///
    /// - `u32` - The viewport width to seed the shadow with.
    /// - `u32` - The viewport height to seed the shadow with.
    pub fn invalidate_state(&mut self, width: u32, height: u32) {
        let fresh: GlRenderState = GlRenderState::context_defaults(width as i32, height as i32);
        self.set_shadow(fresh);
        self.set_active_unit(GL_TEXTURE_UNIT_NONE);
        self.set_bound_program(JsValue::UNDEFINED);
    }

    /// Sets the color [`WebGl2Backend::begin_frame`] clears to.
    ///
    /// # Arguments
    ///
    /// - `Color` - The clear color, with channels in `0.0..=1.0`.
    pub fn set_clear_color(&mut self, color: Color) {
        self.set_clear_color_field(color);
    }

    /// Clears the bound framebuffer and folds a full-size viewport into
    /// the shadow.
    ///
    /// The clear is issued through `clearColor` plus `clear` rather than
    /// `clearBufferfv` because the former is a two-call pair the driver
    /// folds into a single tile-clear, while the latter is the WebGL 2
    /// entry point that most drivers route through a slower generic
    /// path. The viewport is written into the shadow as well, so a
    /// caller that set a smaller viewport for a pass does not have it
    /// silently left in place by the next frame.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to clear through.
    /// - `u32` - The width of the bound framebuffer, in pixels.
    /// - `u32` - The height of the bound framebuffer, in pixels.
    pub fn begin_frame(&mut self, context: &WebGl2RenderingContext, width: u32, height: u32) {
        let color: Color = self.get_clear_color_field();
        context.clear_color(
            color.get_red() as f32,
            color.get_green() as f32,
            color.get_blue() as f32,
            color.get_alpha() as f32,
        );
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        let mut state: GlRenderState = *self.get_shadow();
        state.viewport = GlViewport {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        };
        self.set_shadow(state);
    }

    /// Clears only the depth buffer, leaving color untouched.
    ///
    /// Split out because a second pass that needs a fresh depth range
    /// without disturbing the color underneath is common in deferred
    /// and post-process work, and a full clear here would throw away
    /// results the pass is about to read.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to clear through.
    pub fn clear_depth(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    /// Renders a complete frame: clears to `color`, binds `program`, and
    /// issues one triangle-list draw.
    ///
    /// The one-call frame shape a demo loop needs, built from the same
    /// primitives the multi-pass API exposes. Geometry is generated
    /// inside the vertex shader from `gl_VertexID`, so there are no
    /// vertex buffers to bind; `vertex_count` is the number of vertices
    /// the shader is expected to emit.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to draw through.
    /// - `&GlProgram` - The program to draw with.
    /// - `Color` - The color to clear to before drawing.
    /// - `u32` - The number of vertices to draw.
    pub fn render_frame(
        &mut self,
        context: &WebGl2RenderingContext,
        program: &GlProgram,
        color: Color,
        vertex_count: u32,
    ) {
        self.set_clear_color(color);
        self.begin_frame(context, self.canvas_width(), self.canvas_height());
        self.use_program(context, program);
        let args: DrawArgs = DrawArgs::whole_stream(vertex_count, 1);
        self.draw_arrays(context, PrimitiveTopology::TriangleList, &args);
    }

    /// Returns the canvas backing-store width in physical pixels.
    ///
    /// # Returns
    ///
    /// - `u32` - The canvas `width` attribute.
    fn canvas_width(&self) -> u32 {
        self.get_canvas().width()
    }

    /// Returns the canvas backing-store height in physical pixels.
    ///
    /// # Returns
    ///
    /// - `u32` - The canvas `height` attribute.
    fn canvas_height(&self) -> u32 {
        self.get_canvas().height()
    }
}

/// Implements every draw entry point, plus binding helpers and readback.
impl WebGl2Backend {
    /// Binds a program, issuing `useProgram` only when it is not already
    /// current.
    ///
    /// The identity test is `Object.is` rather than a pointer compare,
    /// because a `WebGlProgram` is a JS handle whose Rust address is a
    /// stack slot a later handle may reuse; comparing addresses would let
    /// two different programs compare equal and skip a needed rebind.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    /// - `&GlProgram` - The program to make current.
    pub fn use_program(&mut self, context: &WebGl2RenderingContext, program: &GlProgram) {
        let candidate: &JsValue = program.get_program().as_ref();
        if !Object::is(self.get_bound_program(), candidate) {
            context.use_program(Some(program.get_program()));
            let owned: JsValue = candidate.clone();
            self.set_bound_program(owned);
        }
    }

    /// Binds a texture to a unit, issuing `activeTexture` only when the
    /// unit is not already current.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to bind against.
    /// - `u32` - The texture unit index, as passed to
    ///   [`gl_texture_unit`].
    /// - `&GlTexture` - The texture to bind to that unit.
    pub fn bind_texture_unit(
        &mut self,
        context: &WebGl2RenderingContext,
        unit: u32,
        texture: &GlTexture,
    ) {
        if self.get_active_unit() != unit {
            context.active_texture(gl_texture_unit(unit));
            self.set_active_unit(unit);
        }
        context.bind_texture(gl_texture_target_2d(), Some(texture.get_texture()));
    }

    /// Draws `args.vertex_count` vertices from the bound vertex streams.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to draw through.
    /// - `PrimitiveTopology` - How the vertices are assembled.
    /// - `&DrawArgs` - The counts and offsets to draw.
    pub fn draw_arrays(
        &self,
        context: &WebGl2RenderingContext,
        topology: PrimitiveTopology,
        args: &DrawArgs,
    ) {
        let mode: u32 = gl_primitive_mode(topology);
        let first: i32 = args.get_first_vertex() as i32;
        let count: i32 = args.get_vertex_count() as i32;
        if args.get_instance_count() <= 1 {
            context.draw_arrays(mode, first, count);
        } else {
            context.draw_arrays_instanced(mode, first, count, args.get_instance_count() as i32);
        }
    }

    /// Draws `args.index_count` indices from the bound index buffer.
    ///
    /// WebGL 2 has no `baseVertex`, the way WebGPU's
    /// [`DrawIndexedArgs`] does: the element offset reaches the vertex
    /// fetch through the attribute pointers recorded in the vertex array
    /// object instead. `base_vertex` is therefore only meaningful as an
    /// assertion here, and a non-zero value is reported as `false` rather
    /// than silently ignored, because silently ignoring it would draw
    /// the wrong mesh with no diagnostic at all.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to draw through.
    /// - `PrimitiveTopology` - How the vertices are assembled.
    /// - `&DrawIndexedArgs` - The counts and offsets to draw.
    /// - `IndexFormat` - The element width of the bound index buffer.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the draw was issued, `false` when
    ///   `base_vertex` is non-zero and therefore cannot be honored.
    pub fn draw_elements(
        &self,
        context: &WebGl2RenderingContext,
        topology: PrimitiveTopology,
        args: &DrawIndexedArgs,
        index_format: IndexFormat,
    ) -> bool {
        if args.get_base_vertex() != 0 {
            return false;
        }
        let (index_type, stride): (u32, u32) = gl_index_type(index_format);
        let mode: u32 = gl_primitive_mode(topology);
        let count: i32 = args.get_index_count() as i32;
        let offset: i32 = (args.get_first_index() * stride) as i32;
        if args.get_instance_count() <= 1 {
            context.draw_elements_with_i32(mode, count, index_type, offset);
        } else {
            context.draw_elements_instanced_with_i32(
                mode,
                count,
                index_type,
                offset,
                args.get_instance_count() as i32,
            );
        }
        true
    }

    /// Draws a contiguous index range without disturbing the rest of the
    /// index buffer.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to draw through.
    /// - `PrimitiveTopology` - How the vertices are assembled.
    /// - `u32` - The first index to read.
    /// - `u32` - The last index to read, inclusive.
    /// - `IndexFormat` - The element width of the bound index buffer.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the range was non-empty and was drawn.
    pub fn draw_element_range(
        &self,
        context: &WebGl2RenderingContext,
        topology: PrimitiveTopology,
        start: u32,
        end: u32,
        index_format: IndexFormat,
    ) -> bool {
        if end < start {
            return false;
        }
        let (index_type, _stride): (u32, u32) = gl_index_type(index_format);
        context.draw_range_elements_with_i32(
            gl_primitive_mode(topology),
            start,
            end,
            (end - start + 1) as i32,
            index_type,
            0,
        );
        true
    }

    /// Draws an instanced vertex range, for geometry generated entirely
    /// on the GPU.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to draw through.
    /// - `PrimitiveTopology` - How the vertices are assembled.
    /// - `&DrawArgs` - The counts and offsets to draw.
    pub fn draw_arrays_instanced(
        &self,
        context: &WebGl2RenderingContext,
        topology: PrimitiveTopology,
        args: &DrawArgs,
    ) {
        let instances: i32 = args.get_instance_count().max(1) as i32;
        context.draw_arrays_instanced(
            gl_primitive_mode(topology),
            args.get_first_vertex() as i32,
            args.get_vertex_count() as i32,
            instances,
        );
    }

    /// Draws an instanced index range, the form a mesh rendered many
    /// times per frame takes.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to draw through.
    /// - `PrimitiveTopology` - How the vertices are assembled.
    /// - `&DrawIndexedArgs` - The counts and offsets to draw.
    /// - `IndexFormat` - The element width of the bound index buffer.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the draw was issued, `false` when
    ///   `base_vertex` is non-zero and therefore cannot be honored.
    pub fn draw_elements_instanced(
        &self,
        context: &WebGl2RenderingContext,
        topology: PrimitiveTopology,
        args: &DrawIndexedArgs,
        index_format: IndexFormat,
    ) -> bool {
        self.draw_elements(context, topology, args, index_format)
    }

    /// Reads a rectangle of the bound framebuffer back into Rust memory.
    ///
    /// The destination buffer is owned by the backend and reused across
    /// calls, so a steady-state readback, a picking query or a screenshot
    /// say, allocates nothing after the first time it runs at that size.
    /// The result is returned as an owned `Vec`: a caller that needs the
    /// bytes past the next readback keeps its own copy, which is one
    /// explicit clone rather than a borrow held across a driver call.
    ///
    /// Reading from the default framebuffer is legal but slow on most
    /// drivers, because the contents may already have been discarded by
    /// the compositor; reading from a bound
    /// [`GlFramebuffer`](super::GlFramebuffer) is the supported path.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - The context to read through.
    /// - `i32` - The left edge in pixels.
    /// - `i32` - The bottom edge in pixels.
    /// - `u32` - The width in pixels.
    /// - `u32` - The height in pixels.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The tightly packed RGBA bytes, row-major from the
    ///   bottom edge, or empty when the rectangle has no area.
    pub fn read_pixels(
        &mut self,
        context: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Vec<u8> {
        if width == 0 || height == 0 {
            return Vec::new();
        }
        let needed: usize = (width as usize) * (height as usize) * GL_RGBA_TEXEL_SIZE;
        if self.get_readback().len() < needed {
            self.set_readback(vec![0; needed]);
        }
        let mut storage: Vec<u8> = mem::take(self.get_mut_readback());
        let _: Result<(), JsValue> = context.read_pixels_with_opt_u8_array(
            x,
            y,
            width as i32,
            height as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&mut storage[..needed]),
        );
        let bytes: Vec<u8> = storage[..needed].to_vec();
        self.set_readback(storage);
        bytes
    }
}
