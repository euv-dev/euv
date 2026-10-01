use super::*;

/// The `loadOp` / `depthLoadOp` wire value.
///
/// The single place the renderer turns a [`LoadOp`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `LoadOp` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into the attachment's `loadOp` or
///   `depthLoadOp` field.
pub(crate) fn load_op_name(value: LoadOp) -> &'static str {
    match value {
        LoadOp::Clear => WEBGPU_LOAD_OP_CLEAR,

        LoadOp::Load => WEBGPU_LOAD_OP_LOAD,
    }
}

/// The `storeOp` / `depthStoreOp` wire value.
///
/// The single place the renderer turns a [`StoreOp`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `StoreOp` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into the attachment's `storeOp` or
///   `depthStoreOp` field.
pub(crate) fn store_op_name(value: StoreOp) -> &'static str {
    match value {
        StoreOp::Store => WEBGPU_STORE_OP_STORE,

        StoreOp::Discard => WEBGPU_STORE_OP_DISCARD,
    }
}

/// The `GPUBufferUsage` bit for one buffer use.
///
/// The single place the renderer turns a [`BufferUsage`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `BufferUsage` - The variant to translate.
///
/// # Returns
///
/// - `u32` - the bit this use occupies in the `usage` bitmask of a
///   `GpuBufferDescriptor`; combine several with `|`.
pub(crate) fn buffer_usage_bit(value: BufferUsage) -> u32 {
    match value {
        BufferUsage::MapRead => RENDER_USAGE_MAP_READ as u32,

        BufferUsage::MapWrite => RENDER_USAGE_MAP_WRITE as u32,

        BufferUsage::CopySource => RENDER_USAGE_COPY_SRC as u32,

        BufferUsage::CopyDestination => RENDER_USAGE_COPY_DST as u32,

        BufferUsage::Index => RENDER_USAGE_INDEX as u32,

        BufferUsage::Vertex => RENDER_USAGE_VERTEX as u32,

        BufferUsage::Uniform => RENDER_USAGE_UNIFORM as u32,

        BufferUsage::Storage => RENDER_USAGE_STORAGE as u32,

        BufferUsage::Indirect => RENDER_USAGE_INDIRECT as u32,

        BufferUsage::QueryResolve => RENDER_USAGE_QUERY_RESOLVE as u32,
    }
}

/// The `GPUTextureUsage` bit for one texture use.
///
/// The single place the renderer turns a [`TextureUsage`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `TextureUsage` - The variant to translate.
///
/// # Returns
///
/// - `u32` - the bit this use occupies in the `usage` bitmask of a
///   `GpuTextureDescriptor`; combine several with `|`.
pub(crate) fn texture_usage_bit(value: TextureUsage) -> u32 {
    match value {
        TextureUsage::CopySource => RENDER_TEXTURE_USAGE_COPY_SRC as u32,

        TextureUsage::CopyDestination => RENDER_TEXTURE_USAGE_COPY_DST as u32,

        TextureUsage::TextureBinding => RENDER_USAGE_TEXTURE_BINDING as u32,

        TextureUsage::StorageBinding => RENDER_USAGE_STORAGE_BINDING as u32,

        TextureUsage::RenderAttachment => RENDER_USAGE_RENDER_ATTACHMENT as u32,
    }
}

/// The `GPUShaderStage` bit for one pipeline stage.
///
/// The single place the renderer turns a [`ShaderStage`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `ShaderStage` - The variant to translate.
///
/// # Returns
///
/// - `u32` - the bit this stage occupies in the `visibility` bitmask of a bind
///   group layout entry; combine several with `|`.
pub(crate) fn shader_stage_bit(value: ShaderStage) -> u32 {
    match value {
        ShaderStage::Vertex => SHADER_STAGE_VERTEX as u32,

        ShaderStage::Fragment => SHADER_STAGE_FRAGMENT as u32,

        ShaderStage::Compute => SHADER_STAGE_COMPUTE as u32,
    }
}

/// The `magFilter` / `minFilter` wire value.
///
/// The single place the renderer turns a [`FilterMode`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `FilterMode` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a sampler's magnification or
///   minification filter field.
pub(crate) fn filter_mode_name(value: FilterMode) -> &'static str {
    match value {
        FilterMode::Nearest => WEBGPU_FILTER_MODE_NEAREST,

        FilterMode::Linear => WEBGPU_FILTER_MODE_LINEAR,
    }
}

/// The `mipmapFilter` wire value.
///
/// The single place the renderer turns a [`MipmapFilter`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `MipmapFilter` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a sampler's `mipmapFilter` field.
pub(crate) fn mipmap_filter_name(value: MipmapFilter) -> &'static str {
    match value {
        MipmapFilter::Nearest => WEBGPU_FILTER_MODE_NEAREST,

        MipmapFilter::Linear => WEBGPU_FILTER_MODE_LINEAR,
    }
}

/// The `addressModeU` / `V` / `W` wire value.
///
/// The single place the renderer turns a [`AddressMode`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `AddressMode` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into the requested axis of a sampler's
///   address-mode field.
pub(crate) fn address_mode_name(value: AddressMode) -> &'static str {
    match value {
        AddressMode::ClampToEdge => WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,

        AddressMode::MirrorRepeat => WEBGPU_ADDRESS_MODE_MIRROR_REPEAT,

        AddressMode::Repeat => WEBGPU_ADDRESS_MODE_REPEAT,
    }
}

/// The WebGPU comparison-operator wire value.
///
/// The single place the renderer turns a [`CompareFunction`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `CompareFunction` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a depth-stencil state's
///   `depthCompare` field or a comparison sampler's `compare` field.
pub(crate) fn compare_function_name(value: CompareFunction) -> &'static str {
    match value {
        CompareFunction::Never => WEBGPU_COMPARE_NEVER,

        CompareFunction::Less => WEBGPU_COMPARE_LESS,

        CompareFunction::Equal => WEBGPU_COMPARE_EQUAL,

        CompareFunction::LessEqual => WEBGPU_COMPARE_LESS_EQUAL,

        CompareFunction::Greater => WEBGPU_COMPARE_GREATER,

        CompareFunction::NotEqual => WEBGPU_COMPARE_NOT_EQUAL,

        CompareFunction::GreaterEqual => WEBGPU_COMPARE_GREATER_EQUAL,

        CompareFunction::Always => WEBGPU_COMPARE_ALWAYS,
    }
}

/// The WebGPU blend-factor wire value.
///
/// The single place the renderer turns a [`BlendFactor`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `BlendFactor` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a blend component's `srcFactor`
///   or `dstFactor` field.
pub(crate) fn blend_factor_name(value: BlendFactor) -> &'static str {
    match value {
        BlendFactor::Zero => WEBGPU_BLEND_FACTOR_ZERO,

        BlendFactor::One => WEBGPU_BLEND_FACTOR_ONE,

        BlendFactor::SourceColor => WEBGPU_BLEND_FACTOR_SRC,

        BlendFactor::OneMinusSourceColor => WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC,

        BlendFactor::DestinationColor => WEBGPU_BLEND_FACTOR_DST,

        BlendFactor::OneMinusDestinationColor => WEBGPU_BLEND_FACTOR_ONE_MINUS_DST,

        BlendFactor::SourceAlpha => WEBGPU_BLEND_FACTOR_SRC_ALPHA,

        BlendFactor::OneMinusSourceAlpha => WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,

        BlendFactor::DestinationAlpha => WEBGPU_BLEND_FACTOR_DST_ALPHA,

        BlendFactor::OneMinusDestinationAlpha => WEBGPU_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,

        BlendFactor::ConstantColor => WEBGPU_BLEND_FACTOR_CONSTANT,

        BlendFactor::OneMinusConstantColor => WEBGPU_BLEND_FACTOR_ONE_MINUS_CONSTANT,

        BlendFactor::ConstantAlpha => WEBGPU_BLEND_FACTOR_CONSTANT_ALPHA,

        BlendFactor::OneMinusConstantAlpha => WEBGPU_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,

        BlendFactor::SourceAlphaSaturated => WEBGPU_BLEND_FACTOR_SRC_ALPHA_SATURATED,
    }
}

/// The WebGPU blend-operation wire value.
///
/// The single place the renderer turns a [`BlendOperation`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `BlendOperation` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a blend component's `operation`
///   field.
pub(crate) fn blend_operation_name(value: BlendOperation) -> &'static str {
    match value {
        BlendOperation::Add => WEBGPU_BLEND_OPERATION_ADD,

        BlendOperation::Subtract => WEBGPU_BLEND_OPERATION_SUBTRACT,

        BlendOperation::ReverseSubtract => WEBGPU_BLEND_OPERATION_REVERSE_SUBTRACT,

        BlendOperation::Min => WEBGPU_BLEND_OPERATION_MIN,

        BlendOperation::Max => WEBGPU_BLEND_OPERATION_MAX,
    }
}

/// The WebGPU primitive-topology wire value.
///
/// The single place the renderer turns a [`PrimitiveTopology`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `PrimitiveTopology` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a pipeline's `primitive.topology`
///   field.
pub(crate) fn primitive_topology_name(value: PrimitiveTopology) -> &'static str {
    match value {
        PrimitiveTopology::PointList => WEBGPU_PRIMITIVE_TOPOLOGY_POINT_LIST,

        PrimitiveTopology::LineList => WEBGPU_PRIMITIVE_TOPOLOGY_LINE_LIST,

        PrimitiveTopology::LineStrip => WEBGPU_PRIMITIVE_TOPOLOGY_LINE_STRIP,

        PrimitiveTopology::TriangleList => WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,

        PrimitiveTopology::TriangleStrip => WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
    }
}

/// The WebGPU index-format wire value.
///
/// The single place the renderer turns a [`IndexFormat`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `IndexFormat` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a strip primitive's
///   `stripIndexFormat` field or into a `setIndexBuffer` call.
pub(crate) fn index_format_name(value: IndexFormat) -> &'static str {
    match value {
        IndexFormat::Uint16 => WEBGPU_INDEX_FORMAT_UINT16,

        IndexFormat::Uint32 => WEBGPU_INDEX_FORMAT_UINT32,
    }
}

/// The WebGPU front-face wire value.
///
/// The single place the renderer turns a [`FrontFace`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `FrontFace` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a pipeline's `primitive.frontFace`
///   field.
pub(crate) fn front_face_name(value: FrontFace) -> &'static str {
    match value {
        FrontFace::CounterClockwise => WEBGPU_FRONT_FACE_COUNTER_CLOCKWISE,

        FrontFace::Clockwise => WEBGPU_FRONT_FACE_CLOCKWISE,
    }
}

/// The WebGPU cull-mode wire value.
///
/// The single place the renderer turns a [`CullMode`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `CullMode` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a pipeline's `primitive.cullMode`
///   field.
pub(crate) fn cull_mode_name(value: CullMode) -> &'static str {
    match value {
        CullMode::None => WEBGPU_CULL_MODE_NONE,

        CullMode::Front => WEBGPU_CULL_MODE_FRONT,

        CullMode::Back => WEBGPU_CULL_MODE_BACK,
    }
}

/// The WebGPU vertex-attribute-format wire value.
///
/// The single place the renderer turns a [`VertexAttributeFormat`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `VertexAttributeFormat` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into one vertex attribute's `format`
///   field.
pub(crate) fn vertex_attribute_format_name(value: VertexAttributeFormat) -> &'static str {
    match value {
        VertexAttributeFormat::Float32 => WEBGPU_VERTEX_FORMAT_FLOAT32,

        VertexAttributeFormat::Float32x2 => WEBGPU_VERTEX_FORMAT_FLOAT32X2,

        VertexAttributeFormat::Float32x3 => WEBGPU_VERTEX_FORMAT_FLOAT32X3,

        VertexAttributeFormat::Float32x4 => WEBGPU_VERTEX_FORMAT_FLOAT32X4,

        VertexAttributeFormat::Unorm8x4 => WEBGPU_VERTEX_FORMAT_UNORM8X4,

        VertexAttributeFormat::Uint32 => WEBGPU_VERTEX_FORMAT_UINT32,

        VertexAttributeFormat::Sint32 => WEBGPU_VERTEX_FORMAT_SINT32,
    }
}

/// The WebGPU texture-format wire value.
///
/// The single place the renderer turns a [`GpuTextureFormat`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `GpuTextureFormat` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer writes into a texture descriptor's `format`
///   field or a color target state's format.
pub(crate) fn gpu_texture_format_name(value: GpuTextureFormat) -> &'static str {
    match value {
        GpuTextureFormat::Rgba8Unorm => WEBGPU_FORMAT_RGBA8UNORM,

        GpuTextureFormat::Bgra8Unorm => WEBGPU_FORMAT_BGRA8UNORM,

        GpuTextureFormat::Rgba16Float => WEBGPU_FORMAT_RGBA16FLOAT,

        GpuTextureFormat::Depth24Plus => WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS,

        GpuTextureFormat::Depth24PlusStencil8 => WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS_STENCIL8,

        GpuTextureFormat::Depth32Float => WEBGPU_DEPTH_FORMAT_DEPTH32_FLOAT,

        GpuTextureFormat::R32Float => WEBGPU_FORMAT_R32FLOAT,

        GpuTextureFormat::Rgba32Float => WEBGPU_FORMAT_RGBA32FLOAT,
    }
}

/// The WebGPU error-filter wire value.
///
/// The single place the renderer turns a [`GpuErrorFilter`] into the value the
/// WebGPU API expects, so a variant can never reach the wire as a
/// mistyped literal.
///
/// # Arguments
///
/// - `GpuErrorFilter` - The variant to translate.
///
/// # Returns
///
/// - `&'static str` - the value the renderer passes to `GPUDevice.pushErrorScope`.
pub(crate) fn gpu_error_filter_name(value: GpuErrorFilter) -> &'static str {
    match value {
        GpuErrorFilter::Validation => WEBGPU_ERROR_FILTER_VALIDATION,

        GpuErrorFilter::OutOfMemory => WEBGPU_ERROR_FILTER_OUT_OF_MEMORY,

        GpuErrorFilter::Internal => WEBGPU_ERROR_FILTER_INTERNAL,
    }
}

/// Combines several texture uses into one `GPUTextureUsage` bitmask.
///
/// `|` is left-associative and each step yields a `u32`, so it cannot
/// fold an arbitrary number of variants. This helper is the variadic
/// form for the call sites that need more than two uses - the
/// four-way storage-texture mask the compute path allocates, for
/// instance.
///
/// # Arguments
///
/// - `&[TextureUsage]` - The uses to combine, in any order and with
///   no repeats.
///
/// # Returns
///
/// - `u32` - The bitmask carrying every listed use.
pub(crate) fn texture_usage_mask(usages: &[TextureUsage]) -> u32 {
    let mut mask: u32 = 0;
    for usage in usages {
        mask |= texture_usage_bit(*usage);
    }
    mask
}

/// Combines several buffer uses into one `GPUBufferUsage` bitmask.
///
/// The buffer counterpart of the variadic
/// [`texture_usage_mask`], for the call sites that need three or more
/// uses at once.
///
/// # Arguments
///
/// - `&[BufferUsage]` - The uses to combine, in any order and with no
///   repeats.
///
/// # Returns
///
/// - `u32` - The bitmask carrying every listed use.
pub(crate) fn buffer_usage_mask(usages: &[BufferUsage]) -> u32 {
    let mut mask: u32 = 0;
    for usage in usages {
        mask |= buffer_usage_bit(*usage);
    }
    mask
}
