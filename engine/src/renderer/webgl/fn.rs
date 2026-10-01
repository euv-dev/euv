use super::*;

/// Maps a [`GlShaderKind`] to the raw `u32` shader-type enum
/// `createShader` expects.
///
/// # Arguments
///
/// - `GlShaderKind` - The shader stage to compile for.
///
/// # Returns
///
/// - `u32` - `VERTEX_SHADER` or `FRAGMENT_SHADER`.
pub(crate) fn gl_shader_type(kind: GlShaderKind) -> u32 {
    match kind {
        GlShaderKind::Vertex => WebGl2RenderingContext::VERTEX_SHADER,
        GlShaderKind::Fragment => WebGl2RenderingContext::FRAGMENT_SHADER,
    }
}

/// Maps a [`PrimitiveTopology`] to the raw `u32` primitive mode every
/// `draw*` call takes.
///
/// # Arguments
///
/// - `PrimitiveTopology` - The topology to assemble vertices with.
///
/// # Returns
///
/// - `u32` - The `POINTS` / `LINES` / `LINE_STRIP` / `TRIANGLES` /
///   `TRIANGLE_STRIP` constant.
pub(crate) fn gl_primitive_mode(topology: PrimitiveTopology) -> u32 {
    match topology {
        PrimitiveTopology::PointList => WebGl2RenderingContext::POINTS,
        PrimitiveTopology::LineList => WebGl2RenderingContext::LINES,
        PrimitiveTopology::LineStrip => WebGl2RenderingContext::LINE_STRIP,
        PrimitiveTopology::TriangleList => WebGl2RenderingContext::TRIANGLES,
        PrimitiveTopology::TriangleStrip => WebGl2RenderingContext::TRIANGLE_STRIP,
    }
}

/// Maps an [`IndexFormat`] to the raw `u32` index element type and to the
/// element stride in bytes.
///
/// WebGL has one `drawElements` for both widths and distinguishes them by
/// the `type` argument, so the format has to resolve into a pair rather
/// than a single value. The stride is what turns a first-index into a
/// byte offset for a sub-range draw.
///
/// # Arguments
///
/// - `IndexFormat` - The element width of the bound index buffer.
///
/// # Returns
///
/// - `(u32, u32)` - The `UNSIGNED_SHORT` or `UNSIGNED_INT` type, and the
///   element size in bytes.
pub(crate) fn gl_index_type(format: IndexFormat) -> (u32, u32) {
    match format {
        IndexFormat::Uint16 => (WebGl2RenderingContext::UNSIGNED_SHORT, GL_U16_SIZE),
        IndexFormat::Uint32 => (WebGl2RenderingContext::UNSIGNED_INT, GL_U32_SIZE),
    }
}

/// Maps a [`BufferUsage`] to the raw `u32` buffer-target enum it binds
/// to.
///
/// WebGL splits the WebGPU usage bitmask along a different axis: a
/// buffer's role is decided by which binding point it is bound to rather
/// than by a usage flag declared at creation, so only the roles that have
/// a dedicated target are named here and everything else binds as
/// `ARRAY_BUFFER`.
///
/// # Arguments
///
/// - `BufferUsage` - The role the buffer plays.
///
/// # Returns
///
/// - `u32` - `ARRAY_BUFFER`, `ELEMENT_ARRAY_BUFFER`, or
///   `UNIFORM_BUFFER`.
pub(crate) fn gl_buffer_target(usage: BufferUsage) -> u32 {
    match usage {
        BufferUsage::Index => WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        BufferUsage::Uniform => WebGl2RenderingContext::UNIFORM_BUFFER,
        _ => WebGl2RenderingContext::ARRAY_BUFFER,
    }
}

/// Maps a [`BufferUsage`] to the `bufferData` usage hint that matches its
/// write pattern.
///
/// A buffer rewritten every frame is marked `DYNAMIC_DRAW` so the driver
/// picks a staging path that makes re-uploads cheap, and a buffer written
/// once is `STATIC_DRAW` so the driver picks an immutable-storage path.
/// Getting this wrong is the difference between a per-frame buffer
/// upload that is a pointer bump and one that stalls the pipeline.
///
/// # Arguments
///
/// - `BufferUsage` - The role the buffer plays.
///
/// # Returns
///
/// - `u32` - `STATIC_DRAW` or `DYNAMIC_DRAW`.
pub(crate) fn gl_buffer_usage_hint(usage: BufferUsage) -> u32 {
    match usage {
        BufferUsage::CopyDestination | BufferUsage::MapWrite | BufferUsage::Storage => {
            WebGl2RenderingContext::DYNAMIC_DRAW
        }
        _ => WebGl2RenderingContext::STATIC_DRAW,
    }
}

/// Maps a [`FilterMode`] to the raw `u32` `TEXTURE_MIN_FILTER` or
/// `TEXTURE_MAG_FILTER` value for a texture with no mip chain.
///
/// # Arguments
///
/// - `FilterMode` - The interpolation mode within one mip level.
///
/// # Returns
///
/// - `u32` - `NEAREST` or `LINEAR`.
pub(crate) fn gl_filter_mode(filter: FilterMode) -> u32 {
    match filter {
        FilterMode::Nearest => WebGl2RenderingContext::NEAREST,
        FilterMode::Linear => WebGl2RenderingContext::LINEAR,
    }
}

/// Maps a [`FilterMode`] plus a [`MipmapFilter`] to the combined
/// `TEXTURE_MIN_FILTER` value.
///
/// The minification filter is the only one that has a mip dimension, so
/// it is the only one that needs the mip blend decision folded in: eight
/// possible combinations, four of which differ from the plain
/// single-level filter the same [`FilterMode`] names for magnification.
///
/// # Arguments
///
/// - `FilterMode` - The interpolation mode within one mip level.
/// - `MipmapFilter` - How two adjacent mip levels are combined.
///
/// # Returns
///
/// - `u32` - One of the four `*_MIPMAP_*` values.
pub(crate) fn gl_min_filter(filter: FilterMode, mipmap: MipmapFilter) -> u32 {
    match (filter, mipmap) {
        (FilterMode::Nearest, MipmapFilter::Nearest) => {
            WebGl2RenderingContext::NEAREST_MIPMAP_NEAREST
        }
        (FilterMode::Nearest, MipmapFilter::Linear) => {
            WebGl2RenderingContext::NEAREST_MIPMAP_LINEAR
        }
        (FilterMode::Linear, MipmapFilter::Nearest) => {
            WebGl2RenderingContext::LINEAR_MIPMAP_NEAREST
        }
        (FilterMode::Linear, MipmapFilter::Linear) => WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR,
    }
}

/// Maps an [`AddressMode`] to the raw `u32` `TEXTURE_WRAP_S` or
/// `TEXTURE_WRAP_T` value.
///
/// # Arguments
///
/// - `AddressMode` - What sampling does outside `[0, 1]`.
///
/// # Returns
///
/// - `u32` - `REPEAT`, `CLAMP_TO_EDGE`, or `MIRRORED_REPEAT`.
pub(crate) fn gl_address_mode(mode: AddressMode) -> u32 {
    match mode {
        AddressMode::ClampToEdge => GL_ADDRESS_CLAMP_TO_EDGE,
        AddressMode::MirrorRepeat => GL_ADDRESS_MIRRORED_REPEAT,
        AddressMode::Repeat => GL_ADDRESS_REPEAT,
    }
}

/// Maps a [`CompareFunction`] to the raw `u32` `depthFunc` value.
///
/// # Arguments
///
/// - `CompareFunction` - The comparison to perform.
///
/// # Returns
///
/// - `u32` - The `NEVER` / `LESS` / `EQUAL` / `LEQUAL` / `GREATER` /
///   `NOTEQUAL` / `GEQUAL` / `ALWAYS` constant.
pub(crate) fn gl_compare_function(compare: CompareFunction) -> u32 {
    match compare {
        CompareFunction::Never => WebGl2RenderingContext::NEVER,
        CompareFunction::Less => WebGl2RenderingContext::LESS,
        CompareFunction::Equal => WebGl2RenderingContext::EQUAL,
        CompareFunction::LessEqual => WebGl2RenderingContext::LEQUAL,
        CompareFunction::Greater => WebGl2RenderingContext::GREATER,
        CompareFunction::NotEqual => WebGl2RenderingContext::NOTEQUAL,
        CompareFunction::GreaterEqual => WebGl2RenderingContext::GEQUAL,
        CompareFunction::Always => WebGl2RenderingContext::ALWAYS,
    }
}

/// Maps a [`BlendFactor`] to the raw `u32` blend-factor constant.
///
/// The source and destination factors share one numeric space, and every
/// variant in [`BlendFactor`] has a real constant, so there is no
/// fallback: a named factor with no wire value would be a silent no-op
/// blend, which is far harder to diagnose than a visibly wrong result.
///
/// # Arguments
///
/// - `BlendFactor` - The factor one blended term is scaled by.
///
/// # Returns
///
/// - `u32` - The `glBlendFunc` factor constant.
pub(crate) fn gl_blend_factor(factor: BlendFactor) -> u32 {
    match factor {
        BlendFactor::Zero => GL_BLEND_FACTOR_ZERO,
        BlendFactor::One => GL_BLEND_FACTOR_ONE,
        BlendFactor::SourceColor => WebGl2RenderingContext::SRC_COLOR,
        BlendFactor::OneMinusSourceColor => WebGl2RenderingContext::ONE_MINUS_SRC_COLOR,
        BlendFactor::DestinationColor => WebGl2RenderingContext::DST_COLOR,
        BlendFactor::OneMinusDestinationColor => WebGl2RenderingContext::ONE_MINUS_DST_COLOR,
        BlendFactor::SourceAlpha => GL_BLEND_FACTOR_SRC_ALPHA,
        BlendFactor::OneMinusSourceAlpha => GL_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        BlendFactor::DestinationAlpha => WebGl2RenderingContext::DST_ALPHA,
        BlendFactor::OneMinusDestinationAlpha => WebGl2RenderingContext::ONE_MINUS_DST_ALPHA,
        BlendFactor::ConstantColor => WebGl2RenderingContext::CONSTANT_COLOR,
        BlendFactor::OneMinusConstantColor => WebGl2RenderingContext::ONE_MINUS_CONSTANT_COLOR,
        BlendFactor::ConstantAlpha => WebGl2RenderingContext::CONSTANT_ALPHA,
        BlendFactor::OneMinusConstantAlpha => WebGl2RenderingContext::ONE_MINUS_CONSTANT_ALPHA,
        BlendFactor::SourceAlphaSaturated => GL_BLEND_FACTOR_SRC_ALPHA_SATURATE,
    }
}

/// Maps a [`BlendOperation`] to the raw `u32` blend-equation constant.
///
/// # Arguments
///
/// - `BlendOperation` - How the two scaled terms are combined.
///
/// # Returns
///
/// - `u32` - `FUNC_ADD`, `FUNC_SUBTRACT`, `FUNC_REVERSE_SUBTRACT`,
///   `MIN`, or `MAX`.
pub(crate) fn gl_blend_equation(operation: BlendOperation) -> u32 {
    match operation {
        BlendOperation::Add => GL_BLEND_EQUATION_ADD,
        BlendOperation::Subtract => WebGl2RenderingContext::FUNC_SUBTRACT,
        BlendOperation::ReverseSubtract => WebGl2RenderingContext::FUNC_REVERSE_SUBTRACT,
        BlendOperation::Min => WebGl2RenderingContext::MIN,
        BlendOperation::Max => WebGl2RenderingContext::MAX,
    }
}

/// Maps a [`CullMode`] to the raw `u32` `cullFace` face selector.
///
/// # Arguments
///
/// - `CullMode` - Which faces to discard.
///
/// # Returns
///
/// - `u32` - `FRONT`, `BACK`, or `FRONT_AND_BACK`.
pub(crate) fn gl_cull_face(mode: CullMode) -> u32 {
    match mode {
        CullMode::None => WebGl2RenderingContext::FRONT_AND_BACK,
        CullMode::Front => WebGl2RenderingContext::FRONT,
        CullMode::Back => WebGl2RenderingContext::BACK,
    }
}

/// Maps a [`FrontFace`] to the raw `u32` `frontFace` winding selector.
///
/// # Arguments
///
/// - `FrontFace` - Which winding order is front facing.
///
/// # Returns
///
/// - `u32` - `CW` or `CCW`.
pub(crate) fn gl_front_face(face: FrontFace) -> u32 {
    match face {
        FrontFace::CounterClockwise => WebGl2RenderingContext::CCW,
        FrontFace::Clockwise => WebGl2RenderingContext::CW,
    }
}

/// Maps a [`VertexAttributeFormat`] to the component count, component
/// type, and normalization flag `vertexAttribPointer` takes.
///
/// WebGL describes a vertex format with three separate arguments rather
/// than one format enum, so this is the one place the three-tuple is
/// assembled and a format that is legal for WebGPU but not expressible
/// in WebGL fails in exactly one function.
///
/// # Arguments
///
/// - `VertexAttributeFormat` - The in-memory layout of one attribute.
///
/// # Returns
///
/// - `(u32, u32, bool)` - The component count, the `FLOAT` /
///   `UNSIGNED_BYTE` / `UNSIGNED_INT` / `INT` type, and whether the
///   integer components are normalized into `0.0..=1.0`.
pub(crate) fn gl_attribute_layout(format: VertexAttributeFormat) -> (u32, u32, bool) {
    match format {
        VertexAttributeFormat::Float32 => (1, GL_TYPE_FLOAT, false),
        VertexAttributeFormat::Float32x2 => (2, GL_TYPE_FLOAT, false),
        VertexAttributeFormat::Float32x3 => (3, GL_TYPE_FLOAT, false),
        VertexAttributeFormat::Float32x4 => (4, GL_TYPE_FLOAT, false),
        VertexAttributeFormat::Unorm8x4 => (4, GL_TYPE_UNSIGNED_BYTE, true),
        VertexAttributeFormat::Uint32 => (1, GL_TYPE_UNSIGNED_INT, false),
        VertexAttributeFormat::Sint32 => (1, GL_TYPE_INT, false),
    }
}

/// Maps a [`GpuTextureFormat`] to the `(internalformat, format, type)`
/// triple `texImage2D` takes.
///
/// WebGL 2's sized-internal-format argument is not interchangeable with
/// the pixel format and pixel type that follow it, and a mismatch among
/// the three is an `INVALID_OPERATION` at texture creation rather than a
/// compile error, so the triple is assembled in one place. The depth
/// formats map to a `DEPTH_COMPONENT` format with a floating-point or
/// integer type, which is what makes a depth texture both attachable to
/// a framebuffer and sampleable with a comparison sampler.
///
/// # Arguments
///
/// - `GpuTextureFormat` - The texel format to allocate.
///
/// # Returns
///
/// - `(i32, u32, u32)` - The sized internal format, the pixel format, and
///   the pixel type.
pub(crate) fn gl_texture_layout(format: GpuTextureFormat) -> (i32, u32, u32) {
    match format {
        GpuTextureFormat::Rgba8Unorm | GpuTextureFormat::Bgra8Unorm => (
            WebGl2RenderingContext::RGBA8 as i32,
            WebGl2RenderingContext::RGBA,
            GL_TYPE_UNSIGNED_BYTE,
        ),
        GpuTextureFormat::Rgba16Float => (
            WebGl2RenderingContext::RGBA16F as i32,
            WebGl2RenderingContext::RGBA,
            GL_TYPE_FLOAT,
        ),
        GpuTextureFormat::Rgba32Float => (
            WebGl2RenderingContext::RGBA32F as i32,
            WebGl2RenderingContext::RGBA,
            GL_TYPE_FLOAT,
        ),
        GpuTextureFormat::R32Float => (GL_FORMAT_R32F as i32, GL_FORMAT_RED, GL_TYPE_FLOAT),
        GpuTextureFormat::Depth24Plus => (
            WebGl2RenderingContext::DEPTH_COMPONENT24 as i32,
            WebGl2RenderingContext::DEPTH_COMPONENT,
            WebGl2RenderingContext::UNSIGNED_INT,
        ),
        GpuTextureFormat::Depth24PlusStencil8 => (
            WebGl2RenderingContext::DEPTH24_STENCIL8 as i32,
            WebGl2RenderingContext::DEPTH_STENCIL,
            WebGl2RenderingContext::UNSIGNED_INT_24_8,
        ),
        GpuTextureFormat::Depth32Float => (
            WebGl2RenderingContext::DEPTH_COMPONENT32F as i32,
            WebGl2RenderingContext::DEPTH_COMPONENT,
            GL_TYPE_FLOAT,
        ),
    }
}

/// Maps a [`GpuTextureFormat`] to the `renderbufferStorage` internal
/// format that matches it.
///
/// A framebuffer's depth attachment is a renderbuffer rather than a
/// texture in the common case, and it needs the same format enum but
/// through a different entry point, so this is a second mapping of the
/// same enum. Depth textures created through
/// [`GlTexture::create`](super::GlTexture::create) use
/// [`gl_texture_layout`] instead.
///
/// # Arguments
///
/// - `GpuTextureFormat` - The texel format to allocate.
///
/// # Returns
///
/// - `u32` - The sized internal format for `renderbufferStorage`.
pub(crate) fn gl_renderbuffer_format(format: GpuTextureFormat) -> u32 {
    match format {
        GpuTextureFormat::Rgba8Unorm | GpuTextureFormat::Bgra8Unorm => {
            WebGl2RenderingContext::RGBA8
        }
        GpuTextureFormat::Rgba16Float => WebGl2RenderingContext::RGBA16F,
        GpuTextureFormat::Rgba32Float => WebGl2RenderingContext::RGBA32F,
        GpuTextureFormat::R32Float => GL_FORMAT_R32F,
        GpuTextureFormat::Depth24Plus => WebGl2RenderingContext::DEPTH_COMPONENT24,
        GpuTextureFormat::Depth24PlusStencil8 => WebGl2RenderingContext::DEPTH24_STENCIL8,
        GpuTextureFormat::Depth32Float => WebGl2RenderingContext::DEPTH_COMPONENT32F,
    }
}

/// Classifies a raw `checkFramebufferStatus` code into a named verdict.
///
/// # Arguments
///
/// - `u32` - The raw status code.
///
/// # Returns
///
/// - `GlFramebufferStatus` - The named verdict for that code.
pub(crate) fn gl_framebuffer_status(status: u32) -> GlFramebufferStatus {
    match status {
        WebGl2RenderingContext::FRAMEBUFFER_COMPLETE => GlFramebufferStatus::Complete,
        WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
            GlFramebufferStatus::Incomplete
        }
        WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_DIMENSIONS
        | WebGl2RenderingContext::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
            GlFramebufferStatus::Unsupported
        }
        _ => GlFramebufferStatus::Other,
    }
}

/// Maps a texture unit index to the raw `u32` `TEXTURE0`-relative enum
/// `activeTexture` expects.
///
/// # Arguments
///
/// - `u32` - The texture unit index, zero-based.
///
/// # Returns
///
/// - `u32` - The `TEXTURE0`-relative constant for that unit.
pub(crate) fn gl_texture_unit(unit: u32) -> u32 {
    WebGl2RenderingContext::TEXTURE0 + unit
}

/// Returns the `TEXTURE_2D` target every texture call in this module
/// binds to.
///
/// # Returns
///
/// - `u32` - The `TEXTURE_2D` constant.
pub(crate) fn gl_texture_target_2d() -> u32 {
    GL_TEXTURE_TARGET_2D
}

/// Flattens a [`Matrix4x4`] into the `[f32; 16]` a `uniformMatrix4fv`
/// call takes, into a caller-owned array.
///
/// The engine's matrix is `f64` and GL's uniform is `f32`, so every
/// matrix upload needs a conversion. Doing it into a caller-supplied
/// array is what keeps the conversion allocation-free: the caller holds
/// the array on its own stack frame, so there is nothing to allocate on
/// any frame including the first. The order is a straight element-wise
/// copy because both sides are column-major and the conversion is what
/// converts, not a transpose.
///
/// # Arguments
///
/// - `&Matrix4x4` - The matrix to upload.
/// - `&mut [f32]` - The destination array, which must be exactly
///   [`GL_MAT4_FLOATS`] elements long.
///
/// # Returns
///
/// - `()` - The array is filled in place.
pub(crate) fn gl_matrix4_into_f32(matrix: &Matrix4x4, out: &mut [f32]) {
    let source: [f64; GL_MAT4_FLOATS] = matrix.get_elements();
    for (index, value) in source.iter().enumerate() {
        out[index] = *value as f32;
    }
}

/// Chooses the `UNPACK_FLIP_Y_WEBGL` setting for an image upload.
///
/// A DOM image has its origin at the top left while GL's texture
/// coordinate origin is at the bottom left, so sampling an un-flipped
/// texture renders it upside down. Flipping on upload rather than
/// negating a v coordinate in the shader costs one unpack-time transpose
/// instead of a per-fragment sign change, and leaves the sampler state
/// identical to what a raw pixel upload would produce.
///
/// # Arguments
///
/// - `bool` - Whether the caller wants the image vertically flipped.
///
/// # Returns
///
/// - `i32` - The value to pass to `pixel_storei`.
pub(crate) fn gl_flip_y(flip: bool) -> i32 {
    if flip { 1 } else { 0 }
}
