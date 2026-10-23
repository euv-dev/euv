/// The WGSL source of the solid-colour pipeline.
///
/// The pipeline draws flat-coloured triangles: fills, strokes, area strips,
/// and label backgrounds. The vertex stage maps the chart's CSS-pixel
/// coordinates into clip space through the uniform transform, and the
/// fragment stage premultiplies the colour so the blend state can use the
/// standard "source over" equation.
pub(crate) const WEBGPU_SOLID_SHADER: &str = r#"
struct Uniforms {
    scale: vec2<f32>,
    offset: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec4<f32>) -> VsOut {
    var out: VsOut;
    out.position = vec4<f32>(position * uniforms.scale + uniforms.offset, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.rgb * in.color.a, in.color.a);
}
"#;

/// The WGSL source of the textured pipeline.
///
/// The pipeline draws triangles that sample a texture: glyph quads read the
/// glyph atlas, and image quads read the image itself. Both use the same
/// layout, so the two share one pipeline and differ only in the bind group
/// bound to group one.
pub(crate) const WEBGPU_TEXTURED_SHADER: &str = r#"
struct Uniforms {
    scale: vec2<f32>,
    offset: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var atlas_sampler: sampler;
@group(1) @binding(1) var atlas_texture: texture_2d<f32>;

struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> VsOut {
    var out: VsOut;
    out.position = vec4<f32>(position * uniforms.scale + uniforms.offset, 0.0, 1.0);
    out.uv = uv;
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let texel = textureSample(atlas_texture, atlas_sampler, in.uv);
    let alpha = texel.a * in.color.a;
    return vec4<f32>(in.color.rgb * in.color.a * texel.rgb, alpha);
}
"#;

/// The vertex name of the solid pipeline's entry point.
pub(crate) const WEBGPU_VERTEX_ENTRY_POINT: &str = "vs_main";

/// The vertex name of the textured pipeline's entry point.
pub(crate) const WEBGPU_FRAGMENT_ENTRY_POINT: &str = "fs_main";

/// The byte stride of one solid vertex: two `f32` positions and four
/// normalised colour bytes.
pub(crate) const WEBGPU_SOLID_VERTEX_STRIDE: u32 = 12;

/// The byte stride of one textured vertex: two `f32` positions, two `f32`
/// texture coordinates, and four normalised colour bytes.
pub(crate) const WEBGPU_TEXTURED_VERTEX_STRIDE: u32 = 20;

/// The byte size of the uniform buffer holding the clip-space transform.
pub(crate) const WEBGPU_UNIFORM_SIZE: u32 = 16;

/// The `GPUBufferUsage.VERTEX` flag.
pub(crate) const WEBGPU_BUFFER_USAGE_VERTEX: u32 = 0x0020;

/// The `GPUBufferUsage.INDEX` flag.
pub(crate) const WEBGPU_BUFFER_USAGE_INDEX: u32 = 0x0010;

/// The `GPUBufferUsage.COPY_DST` flag.
pub(crate) const WEBGPU_BUFFER_USAGE_COPY_DST: u32 = 0x0008;

/// The `GPUBufferUsage.UNIFORM` flag.
pub(crate) const WEBGPU_BUFFER_USAGE_UNIFORM: u32 = 0x0040;

/// The `GPUTextureUsage.COPY_DST` flag.
pub(crate) const WEBGPU_TEXTURE_USAGE_COPY_DST: u32 = 0x0002;

/// The `GPUTextureUsage.TEXTURE_BINDING` flag.
pub(crate) const WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING: u32 = 0x0004;

/// The `GPUTextureUsage.RENDER_ATTACHMENT` flag.
pub(crate) const WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT: u32 = 0x0010;

/// The `GPUShaderStage.VERTEX` flag.
pub(crate) const WEBGPU_SHADER_STAGE_VERTEX: u32 = 0x0001;

/// The `GPUShaderStage.FRAGMENT` flag.
pub(crate) const WEBGPU_SHADER_STAGE_FRAGMENT: u32 = 0x0002;

/// The context id a canvas is asked for when the backend is created.
pub(crate) const WEBGPU_CONTEXT_ID: &str = "webgpu";

/// The alpha mode the canvas context is configured with.
///
/// The chart draws with premultiplied alpha so that the blend state can use
/// the "one, one-minus-src-alpha" equation, which is what Canvas2D does
/// internally as well.
pub(crate) const WEBGPU_ALPHA_MODE: &str = "premultiplied";

/// The texture format the glyph atlas and the images are uploaded as.
pub(crate) const WEBGPU_TEXTURE_FORMAT: &str = "rgba8unorm";

/// The power preference the adapter is requested with.
pub(crate) const WEBGPU_POWER_PREFERENCE: &str = "high-performance";

/// The JavaScript method name `requestAdapter` on `GPU`.
pub(crate) const WEBGPU_METHOD_REQUEST_ADAPTER: &str = "requestAdapter";

/// The JavaScript method name `requestDevice` on `GPUAdapter`.
pub(crate) const WEBGPU_METHOD_REQUEST_DEVICE: &str = "requestDevice";

/// The JavaScript method name `getPreferredCanvasFormat` on `GPU`.
pub(crate) const WEBGPU_METHOD_GET_PREFERRED_CANVAS_FORMAT: &str = "getPreferredCanvasFormat";

/// The JavaScript method name `configure` on `GPUCanvasContext`.
pub(crate) const WEBGPU_METHOD_CONFIGURE: &str = "configure";

/// The JavaScript method name `createCommandEncoder` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_COMMAND_ENCODER: &str = "createCommandEncoder";

/// The JavaScript method name `createBuffer` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BUFFER: &str = "createBuffer";

/// The JavaScript method name `createTexture` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_TEXTURE: &str = "createTexture";

/// The JavaScript method name `createSampler` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_SAMPLER: &str = "createSampler";

/// The JavaScript method name `createShaderModule` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_SHADER_MODULE: &str = "createShaderModule";

/// The JavaScript method name `createRenderPipeline` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_RENDER_PIPELINE: &str = "createRenderPipeline";

/// The JavaScript method name `createBindGroupLayout` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BIND_GROUP_LAYOUT: &str = "createBindGroupLayout";

/// The JavaScript method name `createPipelineLayout` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_PIPELINE_LAYOUT: &str = "createPipelineLayout";

/// The JavaScript method name `createBindGroup` on `GPUDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BIND_GROUP: &str = "createBindGroup";

/// The JavaScript method name `beginRenderPass` on `GPUCommandEncoder`.
pub(crate) const WEBGPU_METHOD_BEGIN_RENDER_PASS: &str = "beginRenderPass";

/// The JavaScript method name `finish` on `GPUCommandEncoder`.
pub(crate) const WEBGPU_METHOD_FINISH: &str = "finish";

/// The JavaScript method name `submit` on `GPUQueue`.
pub(crate) const WEBGPU_METHOD_SUBMIT: &str = "submit";

/// The JavaScript method name `writeBuffer` on `GPUQueue`.
pub(crate) const WEBGPU_METHOD_WRITE_BUFFER: &str = "writeBuffer";

/// The JavaScript method name `copyExternalImageToTexture` on `GPUQueue`.
pub(crate) const WEBGPU_METHOD_COPY_EXTERNAL_IMAGE: &str = "copyExternalImageToTexture";

/// The JavaScript method name `getCurrentTexture` on `GPUCanvasContext`.
pub(crate) const WEBGPU_METHOD_GET_CURRENT_TEXTURE: &str = "getCurrentTexture";

/// The JavaScript method name `createView` on `GPUTexture`.
pub(crate) const WEBGPU_METHOD_CREATE_VIEW: &str = "createView";

/// The JavaScript method name `destroy` on `GPUBuffer` and `GPUTexture`.
pub(crate) const WEBGPU_METHOD_DESTROY: &str = "destroy";

/// The JavaScript method name `setPipeline` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_PIPELINE: &str = "setPipeline";

/// The JavaScript method name `setVertexBuffer` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_VERTEX_BUFFER: &str = "setVertexBuffer";

/// The JavaScript method name `setIndexBuffer` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_INDEX_BUFFER: &str = "setIndexBuffer";

/// The JavaScript method name `setBindGroup` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_BIND_GROUP: &str = "setBindGroup";

/// The JavaScript method name `setScissorRect` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_SCISSOR_RECT: &str = "setScissorRect";

/// The JavaScript method name `drawIndexed` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_DRAW_INDEXED: &str = "drawIndexed";

/// The JavaScript method name `end` on `GPURenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_END: &str = "end";

/// The JavaScript property name `view` inside a colour attachment.
pub(crate) const WEBGPU_PROPERTY_VIEW: &str = "view";

/// The JavaScript property name `gpu` on the navigator.
pub(crate) const WEBGPU_PROPERTY_GPU: &str = "gpu";

/// The descriptor key of a bind group layout entry's binding number.
pub(crate) const WEBGPU_KEY_BINDING: &str = "binding";

/// The descriptor key of a bind group layout entry's visibility.
pub(crate) const WEBGPU_KEY_VISIBILITY: &str = "visibility";

/// The descriptor key of a bind group entry's resource.
pub(crate) const WEBGPU_KEY_RESOURCE: &str = "resource";

/// The descriptor key of a bind group layout's entries.
pub(crate) const WEBGPU_KEY_ENTRIES: &str = "entries";

/// The descriptor key of a bind group's layout.
pub(crate) const WEBGPU_KEY_LAYOUT: &str = "layout";

/// The descriptor key of a buffer or texture descriptor's size.
pub(crate) const WEBGPU_KEY_SIZE: &str = "size";

/// The descriptor key of a buffer or texture descriptor's usage.
pub(crate) const WEBGPU_KEY_USAGE: &str = "usage";

/// The descriptor key of a texture or target descriptor's format.
pub(crate) const WEBGPU_KEY_FORMAT: &str = "format";

/// The descriptor key of a texture descriptor's width.
pub(crate) const WEBGPU_KEY_WIDTH: &str = "width";

/// The descriptor key of a texture descriptor's height.
pub(crate) const WEBGPU_KEY_HEIGHT: &str = "height";

/// The descriptor key of a texture descriptor's depth or array layers.
pub(crate) const WEBGPU_KEY_DEPTH_OR_ARRAY_LAYERS: &str = "depthOrArrayLayers";

/// The descriptor key of a vertex attribute's shader location.
pub(crate) const WEBGPU_KEY_SHADER_LOCATION: &str = "shaderLocation";

/// The descriptor key of a vertex attribute's offset.
pub(crate) const WEBGPU_KEY_OFFSET: &str = "offset";

/// The descriptor key of the vertex state of a render pipeline.
pub(crate) const WEBGPU_KEY_VERTEX: &str = "vertex";

/// The descriptor key of a shader module.
pub(crate) const WEBGPU_KEY_MODULE: &str = "module";

/// The descriptor key of a shader entry point.
pub(crate) const WEBGPU_KEY_ENTRY_POINT: &str = "entryPoint";

/// The descriptor key of a blend factor's source factor.
pub(crate) const WEBGPU_KEY_SRC_FACTOR: &str = "srcFactor";

/// The descriptor key of a blend factor's destination factor.
pub(crate) const WEBGPU_KEY_DST_FACTOR: &str = "dstFactor";

/// The descriptor key of a blend state's operation.
pub(crate) const WEBGPU_KEY_OPERATION: &str = "operation";

/// The descriptor key of a bind group layout entry's buffer type.
pub(crate) const WEBGPU_KEY_TYPE: &str = "type";

/// The descriptor key of a texture binding.
pub(crate) const WEBGPU_KEY_TEXTURE: &str = "texture";

/// The descriptor key of a texture binding's buffer.
pub(crate) const WEBGPU_KEY_BUFFER: &str = "buffer";

/// The descriptor key of a texture copy's source.
pub(crate) const WEBGPU_KEY_SOURCE: &str = "source";

/// The descriptor key of a texture copy's vertical flip.
pub(crate) const WEBGPU_KEY_FLIP_Y: &str = "flipY";

/// The descriptor key of a texture copy's premultiplied alpha flag.
pub(crate) const WEBGPU_KEY_PREMULTIPLIED_ALPHA: &str = "premultipliedAlpha";

/// The vertex buffer step mode that advances per vertex.
pub(crate) const WEBGPU_STEP_MODE_VERTEX: &str = "vertex";

/// The vertex format of a two-component float attribute.
pub(crate) const WEBGPU_VERTEX_FORMAT_FLOAT32X2: &str = "float32x2";

/// The vertex format of a four-component normalised colour attribute.
pub(crate) const WEBGPU_VERTEX_FORMAT_UNORM8X4: &str = "unorm8x4";

/// The sampler filter that interpolates between the sampled texels.
pub(crate) const WEBGPU_FILTER_LINEAR: &str = "linear";

/// The sampler address mode that clamps to the edge of the texture.
pub(crate) const WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE: &str = "clamp-to-edge";

/// The blend factor that keeps the source colour.
pub(crate) const WEBGPU_BLEND_FACTOR_ONE: &str = "one";

/// The blend factor that fades the source colour by the alpha of the
/// destination.
pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: &str = "one-minus-src-alpha";

/// The blend operation that adds the source and the destination.
pub(crate) const WEBGPU_BLEND_OPERATION_ADD: &str = "add";
