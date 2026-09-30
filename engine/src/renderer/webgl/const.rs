/// The size in bytes of one `UNSIGNED_SHORT` element: the stride of a
/// `Uint16` index buffer and of a 16-bit index byte offset.
pub(crate) const GL_U16_SIZE: u32 = 2;

/// The size in bytes of one `FLOAT` element, the width of every
/// `float32` vertex attribute component.
pub(crate) const GL_F32_SIZE: u32 = 4;

/// The size in bytes of one `UNSIGNED_INT` element: the stride of a
/// `Uint32` index buffer and of a 32-bit index byte offset.
pub(crate) const GL_U32_SIZE: u32 = 4;

/// The size in bytes of one RGBA texel, the stride of a tightly packed
/// `width * height * 4` pixel slice.
pub(crate) const GL_RGBA_TEXEL_SIZE: usize = 4;

/// The number of floats one `mat4` uniform column occupies, which is
/// also the length of the engine's own [`Matrix4x4`].
pub(crate) const GL_MAT4_FLOATS: usize = 16;

/// The size in bytes of one `mat4` uploaded through a uniform buffer,
/// the record stride of a block holding one transform per object.
pub(crate) const GL_MAT4_BYTES: u32 = 64;

/// The color-attachment index every single-target render-to-texture
/// framebuffer writes to.
pub(crate) const GL_DEFAULT_COLOR_ATTACHMENT: u32 = 0;

/// The `TEXTURE_2D` target every texture call in this module binds to.
pub(crate) const GL_TEXTURE_TARGET_2D: u32 = 0x0de1;

/// The mip level every allocation and sub-image write targets: the base
/// of the chain, and the only level a caller may write directly.
pub(crate) const GL_MIP_LEVEL_ZERO: f64 = 0.0;

/// The value the texture-unit shadow starts at, chosen so that unit zero
/// is a genuine change and the first bind of a frame is never skipped as
/// a no-op.
pub(crate) const GL_TEXTURE_UNIT_NONE: u32 = 0xffff_ffff;

/// The color-write mask that lets every channel through: red `0x1`,
/// green `0x2`, blue `0x4`, alpha `0x8`.
pub(crate) const GL_COLOR_WRITE_ALL: u32 = 0xf;

/// The red channel bit of a [`GlColorMask`].
pub(crate) const GL_COLOR_CHANNEL_RED: u32 = 0x1;

/// The green channel bit of a [`GlColorMask`].
pub(crate) const GL_COLOR_CHANNEL_GREEN: u32 = 0x2;

/// The blue channel bit of a [`GlColorMask`].
pub(crate) const GL_COLOR_CHANNEL_BLUE: u32 = 0x4;

/// The alpha channel bit of a [`GlColorMask`].
pub(crate) const GL_COLOR_CHANNEL_ALPHA: u32 = 0x8;

/// The address mode a sampler uses when its [`AddressMode`] is
/// `ClampToEdge`.
pub(crate) const GL_ADDRESS_CLAMP_TO_EDGE: u32 = 0x812f;

/// The address mode a sampler uses when its [`AddressMode`] is
/// `MirrorRepeat`.
pub(crate) const GL_ADDRESS_MIRRORED_REPEAT: u32 = 0x8370;

/// The address mode a sampler uses when its [`AddressMode`] is
/// `Repeat`.
pub(crate) const GL_ADDRESS_REPEAT: u32 = 0x2901;

/// The blend equation that adds the two scaled terms, the counterpart of
/// [`BlendOperation::Add`].
pub(crate) const GL_BLEND_EQUATION_ADD: u32 = 0x8006;

/// The blend factor that discards the term, the counterpart of
/// [`BlendFactor::Zero`].
pub(crate) const GL_BLEND_FACTOR_ZERO: u32 = 0;

/// The blend factor that passes the term through, the counterpart of
/// [`BlendFactor::One`].
pub(crate) const GL_BLEND_FACTOR_ONE: u32 = 1;

/// The blend factor `GL_ONE_MINUS_SRC_ALPHA`, the conventional
/// destination factor for alpha blending and the counterpart of
/// [`BlendFactor::OneMinusSourceAlpha`].
pub(crate) const GL_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;

/// The blend factor `GL_SRC_ALPHA`, the counterpart of
/// [`BlendFactor::SourceAlpha`].
pub(crate) const GL_BLEND_FACTOR_SRC_ALPHA: u32 = 0x0302;

/// The blend factor `GL_SRC_ALPHA_SATURATE`, the counterpart of
/// [`BlendFactor::SourceAlphaSaturated`].
pub(crate) const GL_BLEND_FACTOR_SRC_ALPHA_SATURATE: u32 = 0x0308;

/// The `FLOAT` vertex attribute component type, and the pixel type a
/// floating-point texture format uploads through.
pub(crate) const GL_TYPE_FLOAT: u32 = 0x1406;

/// The `INT` vertex attribute component type, the counterpart of
/// [`VertexAttributeFormat::Sint32`].
pub(crate) const GL_TYPE_INT: u32 = 0x1404;

/// The `UNSIGNED_BYTE` vertex attribute component type and the pixel type
/// every normalized eight-bit format uploads through.
pub(crate) const GL_TYPE_UNSIGNED_BYTE: u32 = 0x1401;

/// The `UNSIGNED_INT` vertex attribute component type, the counterpart of
/// [`VertexAttributeFormat::Uint32`].
pub(crate) const GL_TYPE_UNSIGNED_INT: u32 = 0x1405;

/// The `RED` sized-internal-format a single-channel `R32Float` texture
/// allocates.
pub(crate) const GL_FORMAT_R32F: u32 = 0x822e;

/// The `RED` pixel format matching [`GL_FORMAT_R32F`].
pub(crate) const GL_FORMAT_RED: u32 = 0x1903;

/// The info log substituted for a `createShader` that returned `None`,
/// which happens only when the context is lost or out of memory.
pub(crate) const GL_SHADER_CREATE_FAILED: &str = "createShader returned null";

/// The info log substituted for a `createProgram` that returned `None`,
/// which happens only when the context is lost or out of memory.
pub(crate) const GL_PROGRAM_CREATE_FAILED: &str = "createProgram returned null";

/// The name of the `texImage2D` method, looked up once and cached
/// because the `ImageBitmap` overload has no typed `web-sys` binding
/// without a feature this module does not enable for a single call.
pub(crate) const GL_METHOD_TEX_IMAGE_2D: &str = "texImage2D";

/// The `getContext` context id for a WebGL 2 context, which is a
/// different context object from the WebGL 1 one rather than a version
/// flag on the same object.
pub(crate) const GL_CONTEXT_ID_WEBGL2: &str = "webgl2";

/// The DOM tag name used when creating a throwaway canvas for the
/// capability probe.
pub(crate) const GL_DOM_TAG_CANVAS: &str = "canvas";

/// The stable error code reported when the configured selector matched no
/// element in the document.
pub(crate) const GL_ERROR_CANVAS_NOT_FOUND: &str = "WEBGL_CANVAS_NOT_FOUND";

/// The stable error code reported when the selector could not be
/// evaluated at all.
pub(crate) const GL_ERROR_CANVAS_QUERY: &str = "WEBGL_CANVAS_QUERY";

/// The stable error code reported when the browser exposes no WebGL 2
/// context for the canvas.
pub(crate) const GL_ERROR_CONTEXT_UNAVAILABLE: &str = "WEBGL_CONTEXT_UNAVAILABLE";

/// The stable error code reported when `getContext` threw instead of
/// returning `None`.
pub(crate) const GL_ERROR_CONTEXT_LOOKUP: &str = "WEBGL_CONTEXT_LOOKUP";

/// The stable error code reported when a `webgl2` context failed to cast
/// to `WebGl2RenderingContext`, which should not be reachable.
pub(crate) const GL_ERROR_CONTEXT_CAST: &str = "WEBGL_CONTEXT_CAST";

/// The assertion message for the invariant that a live WebGL 2 context
/// always carries the canvas it was obtained from.
pub(crate) const GL_CONTEXT_HAS_CANVAS: &str = "a WebGL 2 context always has a canvas";
