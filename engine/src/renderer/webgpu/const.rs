/// The `sampleType` property key inside `GpuTextureBindingLayout`.
pub(crate) const WEBGPU_PROPERTY_SAMPLE_TYPE: &str = "sampleType";

/// `label` property key (debug marker string, set on every WebGPU object).
pub(crate) const WEBGPU_PROPERTY_LABEL: &str = "label";

/// The JavaScript method name `getPreferredCanvasFormat` on `Gpu`.
pub(crate) const WEBGPU_METHOD_GET_PREFERRED_FORMAT: &str = "getPreferredCanvasFormat";

/// `rowsPerImage` property key inside `GpuImageDataLayout`.
pub(crate) const WEBGPU_PROPERTY_ROWS_PER_IMAGE: &str = "rowsPerImage";

/// `depthWriteEnabled` property key inside `GpuDepthStencilState`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_WRITE_ENABLED: &str = "depthWriteEnabled";

/// `addressModeV` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ADDRESS_MODE_V: &str = "addressModeV";

/// `magFilter` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MAG_FILTER: &str = "magFilter";

/// 16-bit unorm depth, no stencil. Cheaper than depth24plus, but no
/// stencil, slightly more z-fighting.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH16_UNORM: &str = "depth16unorm";

/// The JavaScript method name `finish` on `GpuCommandEncoder`.
pub(crate) const WEBGPU_METHOD_FINISH: &str = "finish";

/// The JavaScript method name `resolveQuerySet` on `GpuCommandEncoder`.
pub(crate) const WEBGPU_METHOD_RESOLVE_QUERY_SET: &str = "resolveQuerySet";

/// The JavaScript method name `configure` on `GpuCanvasContext`.
pub(crate) const WEBGPU_METHOD_CONFIGURE: &str = "configure";

/// `compare` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_COMPARE: &str = "compare";

/// `offset` property key inside `GpuVertexAttribute`.
pub(crate) const WEBGPU_PROPERTY_OFFSET: &str = "offset";

/// The JavaScript method name `createShaderModule` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_SHADER_MODULE: &str = "createShaderModule";

/// The JavaScript property name `view` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_VIEW: &str = "view";

/// `origin` property key inside `GpuImageCopyTexture`.
pub(crate) const WEBGPU_PROPERTY_ORIGIN: &str = "origin";

/// `shaderLocation` property key inside `GpuVertexAttribute`.
pub(crate) const WEBGPU_PROPERTY_SHADER_LOCATION: &str = "shaderLocation";

/// The JavaScript method name `setBindGroup` on `GpuComputePassEncoder`.
/// The JavaScript method name `setPipeline` on `GpuComputePassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_PIPELINE_COMPUTE: &str = "setPipeline";

/// `GpuDevice.createComputePipeline(descriptor)` method name.
pub(crate) const WEBGPU_METHOD_CREATE_COMPUTE_PIPELINE: &str = "createComputePipeline";

/// The `read-only` value for `GpuBufferBindingLayout.type` (read-only storage).
pub(crate) const WEBGPU_BUFFER_BINDING_TYPE_READ_ONLY_STORAGE: &str = "read-only-storage";

/// Default `maxDepth` value passed to `setViewport` when the caller did not
/// supply one. WebGPU's spec default is `1.0`.
pub(crate) const WEBGPU_DEFAULT_VIEWPORT_MAX_DEPTH: f64 = 1.0;

/// `offsetBytes` property key inside `GpuImageDataLayout` (used by `copyTextureToBuffer`).
pub(crate) const WEBGPU_PROPERTY_OFFSET_BYTES: &str = "offsetBytes";

/// Map a buffer for CPU write. Requires `MAP_WRITE` usage.
pub(crate) const WEBGPU_MAP_MODE_WRITE: f64 = 2.0;

/// `baseMipLevel` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_BASE_MIP_LEVEL: &str = "baseMipLevel";

/// `GpuTexture.generateMipmap()` method name (Chrome extension, not spec).
pub(crate) const WEBGPU_METHOD_GENERATE_MIPMAP: &str = "generateMipmap";

/// The JavaScript property name `b` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_B: &str = "b";

pub(crate) const WEBGPU_PROPERTY_DST_FACTOR: &str = "dstFactor";

pub(crate) const WEBGPU_PROPERTY_CULL_MODE: &str = "cullMode";

/// `textureView` property key inside `GpuTextureBinding`.
pub(crate) const WEBGPU_PROPERTY_TEXTURE_VIEW: &str = "textureView";

/// The JavaScript method name `createBindGroup` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BIND_GROUP: &str = "createBindGroup";

pub(crate) const WEBGPU_PROPERTY_COLOR: &str = "color";

/// `sampler` property key inside a sampler binding entry.
pub(crate) const WEBGPU_PROPERTY_SAMPLER: &str = "sampler";

/// `GpuBuffer.unmap()` method name.
pub(crate) const WEBGPU_METHOD_UNMAP: &str = "unmap";

/// `width` property key (used by `GpuExtent3D`, `GpuOrigin3D`, viewport / scissor).
pub(crate) const WEBGPU_PROPERTY_WIDTH: &str = "width";

/// The JavaScript property name `queue` on `GpuDevice`.
pub(crate) const WEBGPU_PROPERTY_QUEUE: &str = "queue";

/// `GpuDevice.createSampler(descriptor)` method name.
pub(crate) const WEBGPU_METHOD_CREATE_SAMPLER: &str = "createSampler";

/// The JavaScript property name `entryPoint` on `GpuVertexState` / `GpuFragmentState`.
pub(crate) const WEBGPU_PROPERTY_ENTRY_POINT: &str = "entryPoint";

/// `GpuCommandEncoder.beginComputePass(descriptor)` method name.
pub(crate) const WEBGPU_METHOD_BEGIN_COMPUTE_PASS: &str = "beginComputePass";

/// `minFilter` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MIN_FILTER: &str = "minFilter";

/// The bitmask value for `GpuTextureUsage.RENDER_ATTACHMENT`.
///
/// WebGPU spec defines this as `0x10`. The multisample texture must carry this
/// usage so it can be bound as a color attachment in `beginRenderPass`.
pub(crate) const WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT: f64 = 16.0;

/// The JavaScript property name `a` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_A: &str = "a";

/// `GpuBuffer.mapAsync(mode, offset, size)` method name.
pub(crate) const WEBGPU_METHOD_MAP_ASYNC: &str = "mapAsync";

/// The `GpuMultisampleState` property key in a render pipeline descriptor.
pub(crate) const WEBGPU_PROPERTY_MULTISAMPLE: &str = "multisample";

/// The JavaScript property name `primitive` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_PRIMITIVE: &str = "primitive";

/// `bytesPerRow` property key inside `GpuImageDataLayout`.
pub(crate) const WEBGPU_PROPERTY_BYTES_PER_ROW: &str = "bytesPerRow";

/// The JavaScript property name `clearValue` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_CLEAR_VALUE: &str = "clearValue";

pub(crate) const WEBGPU_PROPERTY_ALPHA: &str = "alpha";

/// `mipmapFilter` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MIPMAP_FILTER: &str = "mipmapFilter";

/// The JavaScript property name `buffers` on `GpuVertexState`.
pub(crate) const WEBGPU_PROPERTY_BUFFERS: &str = "buffers";

/// The JavaScript method name `drawIndexed(indexCount)` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_DRAW_INDEXED: &str = "drawIndexed";

/// The `multisampled` property key inside `GpuTextureBindingLayout`.
pub(crate) const WEBGPU_PROPERTY_MULTISAMPLED: &str = "multisampled";

/// `arrayLayerCount` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ARRAY_LAYER_COUNT: &str = "arrayLayerCount";

/// `depthLoadOp` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_LOAD_OP: &str = "depthLoadOp";

/// `height` property key (used by `GpuExtent3D`, `GpuOrigin3D`, viewport / scissor).
pub(crate) const WEBGPU_PROPERTY_HEIGHT: &str = "height";

/// The JavaScript method name `createBuffer` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BUFFER: &str = "createBuffer";

/// `GpuBuffer.getMappedRange(offset, size)` method name.
pub(crate) const WEBGPU_METHOD_GET_MAPPED_RANGE: &str = "getMappedRange";

/// `depthClearValue` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_CLEAR_VALUE: &str = "depthClearValue";

/// `depthCompare` property key inside `GpuDepthStencilState`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_COMPARE: &str = "depthCompare";

/// `texture` property key inside a texture binding entry / `writeTexture` dest.
pub(crate) const WEBGPU_PROPERTY_TEXTURE: &str = "texture";

/// The JavaScript property name `loadOp` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_LOAD_OP: &str = "loadOp";

/// The JavaScript method name `createBindGroupLayout` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BIND_GROUP_LAYOUT: &str = "createBindGroupLayout";

/// The JavaScript method name `requestDevice` on `GpuAdapter`.
pub(crate) const WEBGPU_METHOD_REQUEST_DEVICE: &str = "requestDevice";

/// The JavaScript method name `setBindGroup` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_BIND_GROUP: &str = "setBindGroup";

/// The JavaScript property name for `powerPreference` on `GpuRequestAdapterOptions`.
pub(crate) const WEBGPU_PROPERTY_POWER_PREFERENCE: &str = "powerPreference";

/// The `resource` property key inside `GpuBindGroupEntry`.
pub(crate) const WEBGPU_PROPERTY_RESOURCE: &str = "resource";

/// `GpuComputePassEncoder.dispatch(x, y, z)` method name.
pub(crate) const WEBGPU_METHOD_DISPATCH: &str = "dispatch";

/// The JavaScript method name `unconfigure` on `GpuCanvasContext`.
///
/// Releases the GPU resources associated with the canvas context so that
/// the DOM canvas can be detached/GCed and the WebGPU device can be
/// safely destroyed. Called from `WebGpuRenderer::dispose` to tear down
/// a renderer cleanly when its host component is unmounted.
pub(crate) const WEBGPU_METHOD_UNCONFIGURE: &str = "unconfigure";

/// The spec-compliant value for `GPURenderPipelineDescriptor.layout`
/// when no explicit `GPUPipelineLayout` is supplied.
///
/// Per the WebGPU spec, `layout` is `GPUPipelineLayout | "auto"`. The
/// string `"auto"` instructs the implementation to derive an implicit
/// pipeline layout from the shader bindings. Setting `layout` to JS
/// `null` works on some browsers but is NOT spec-compliant and produces
/// a pipeline that the GPU driver may reject silently (the symptom we
/// saw: `WebGPU: No pipeline set.` at draw time, with no create error).
pub(crate) const WEBGPU_AUTO_LAYOUT: &str = "auto";

/// The `binding` property key inside `GpuBindGroupEntry`.
pub(crate) const WEBGPU_PROPERTY_BINDING: &str = "binding";

/// `GpuRenderPassEncoder.setStencilReference(value)` method name.
pub(crate) const WEBGPU_METHOD_SET_STENCIL_REFERENCE: &str = "setStencilReference";

pub(crate) const WEBGPU_PROPERTY_BLEND: &str = "blend";

/// The `sampleCount` property key inside `GpuTextureDescriptor`.
///
/// Sets the multisample count for the texture. Must be `1` for non-multisampled
/// textures or `4` (or `8`, depending on adapter support) for MSAA textures.
pub(crate) const WEBGPU_PROPERTY_SAMPLE_COUNT: &str = "sampleCount";

/// The JavaScript method name `setIndexBuffer(buffer, format)` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_INDEX_BUFFER: &str = "setIndexBuffer";

/// `depthStoreOp` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_STORE_OP: &str = "depthStoreOp";

/// Upper bound, in milliseconds, for how long `WebGpuRenderer::init` is
/// allowed to wait on the adapter and device promises before falling into
/// the `WebGPU Not Supported` branch.
///
/// The browser's `navigator.gpu.requestAdapter()` / `requestDevice()`
/// promises are not always guaranteed to settle — headless contexts,
/// sandboxed iframes, and some GPU device-lost paths leave them pending
/// indefinitely. To keep the UI from stalling on `Initializing...` in those
/// cases, each promise is raced against a timer that rejects after this
/// many milliseconds (see `timeout_promise` / `with_timeout` in
/// `impl.rs`); on timeout the caller surfaces `RENDERER_TIMEOUT_ERROR_MESSAGE`
/// and treats the outcome the same as "no adapter available".
pub(crate) const INIT_PROMISE_TIMEOUT_MILLIS: i32 = 3000;

/// Error message used when the WebGPU init promise race loses to the
/// `INIT_PROMISE_TIMEOUT_MILLIS` timer. Surfaced to `JsFuture::await` as
/// `Err`, which causes the caller to fall into its `WebGPU Not Supported`
/// branch instead of leaving the UI stuck on `Initializing...`.
pub(crate) const RENDERER_TIMEOUT_ERROR_MESSAGE: &str = "WebGPU initialization timed out";

/// The `depthOrArrayLayers` property key inside `GpuExtent3dDict`.
///
/// Always `1` for 2D textures; required by the spec even when unused.
pub(crate) const WEBGPU_PROPERTY_EXTENT_DEPTH: &str = "depthOrArrayLayers";

/// The `hasDynamicOffset` property key inside `GpuBufferBindingLayout`.
/// The `storage` value for `GpuBufferBindingLayout.type` (read-write storage).
pub(crate) const WEBGPU_BUFFER_BINDING_TYPE_STORAGE: &str = "storage";

/// `GpuDevice.popErrorScope()` method name.
pub(crate) const WEBGPU_METHOD_POP_ERROR_SCOPE: &str = "popErrorScope";

/// The JavaScript property name `layout` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_LAYOUT: &str = "layout";

/// `pass.timestampWrites` operation on render / compute pass encoders.
pub(crate) const WEBGPU_METHOD_TIMESTAMP: &str = "timestamp";

/// The `visibility` property key inside `GpuBindGroupLayoutEntry`.
pub(crate) const WEBGPU_PROPERTY_VISIBILITY: &str = "visibility";

/// Texture usage: storage texture binding.
pub(crate) const WEBGPU_TEXTURE_USAGE_STORAGE_BINDING: f64 = 8.0;

/// The `usage` property key inside `GpuTextureDescriptor`.
///
/// Bitmask of `GpuTextureUsage` flags. We use `RENDER_ATTACHMENT` (bit 0x10)
/// for the multisample color buffer that render passes draw into and from
/// which the swap chain resolves.
pub(crate) const WEBGPU_PROPERTY_USAGE: &str = "usage";

/// The `width` property key inside `GpuExtent3dDict`.
pub(crate) const WEBGPU_PROPERTY_EXTENT_WIDTH: &str = "width";

/// `lost` property key inside the `GpuDevice.lost` Promise.
pub(crate) const WEBGPU_PROPERTY_LOST: &str = "lost";

/// `aspect` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ASPECT: &str = "aspect";

/// `addressModeW` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ADDRESS_MODE_W: &str = "addressModeW";

/// Texture usage: read in a shader (sampled texture / uniform texel buffer).
pub(crate) const WEBGPU_TEXTURE_USAGE_COPY_SRC: f64 = 1.0;

/// The `height` property key inside `GpuExtent3dDict`.
pub(crate) const WEBGPU_PROPERTY_EXTENT_HEIGHT: &str = "height";

/// `destination` property key inside `GpuImageCopyTexture` (used by `writeTexture`).
pub(crate) const WEBGPU_PROPERTY_DESTINATION: &str = "destination";

/// The `Navigator` property name that exposes the WebGPU `GPU` interface.
///
/// Per the WebGPU spec and MDN (`Navigator.gpu`), browsers expose the
/// entry point to the API under the property name `"gpu"` - not
/// `"webgpu"`. Using [`WEBGPU_CONTEXT_TYPE`] (which is `"webgpu"`)
/// as the key for `Reflect::get(navigator, ...)` always returns
/// `undefined`, even when WebGPU is fully supported, because
/// `navigator` does not have a `"webgpu"` property. This constant
/// exists to make the correct key explicit at every probe site.
pub(crate) const WEBGPU_NAVIGATOR_GPU_KEY: &str = "gpu";

/// The JavaScript method name `createCommandEncoder` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_COMMAND_ENCODER: &str = "createCommandEncoder";

/// The JavaScript property name `fragment` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_FRAGMENT: &str = "fragment";

/// The JavaScript method name `requestAdapter` on `Gpu`.
pub(crate) const WEBGPU_METHOD_REQUEST_ADAPTER: &str = "requestAdapter";

/// The JavaScript property name `device` on `GpuCanvasConfiguration`.
pub(crate) const WEBGPU_PROPERTY_DEVICE: &str = "device";

/// The JavaScript property name `storeOp` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_STORE_OP: &str = "storeOp";

/// `GpuCommandEncoder.copyTextureToBuffer(...)` method name.
pub(crate) const WEBGPU_METHOD_COPY_TEXTURE_TO_BUFFER: &str = "copyTextureToBuffer";

/// The JavaScript property name `format` on `GpuCanvasConfiguration`.
pub(crate) const WEBGPU_PROPERTY_FORMAT: &str = "format";

/// `baseArrayLayer` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_BASE_ARRAY_LAYER: &str = "baseArrayLayer";

/// The vertex shader entry point name used in WGSL shaders.
pub(crate) const WEBGPU_VERTEX_ENTRY_POINT: &str = "vs_main";

/// The JavaScript method name `setPipeline` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_PIPELINE: &str = "setPipeline";

/// The JavaScript method name `writeBuffer` on `GpuQueue`.
///
/// Uploads host data into a `GpuBuffer` without a staging encoder, which is
/// the canonical way to refresh small per-frame uniform buffers.
pub(crate) const WEBGPU_METHOD_WRITE_BUFFER: &str = "writeBuffer";

/// The JavaScript method name `createQuerySet` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_QUERY_SET: &str = "createQuerySet";

/// The `uniform` value for `GpuBufferBindingLayout.type`.
pub(crate) const WEBGPU_BUFFER_BINDING_TYPE_UNIFORM: &str = "uniform";

/// The `depth` value for `GpuTextureBindingLayout.sampleType`.
/// The `float` value for `GpuTextureBindingLayout.sampleType`.
/// The `unfilterable-float` value for `GpuTextureBindingLayout.sampleType`.
/// The `2d-array` view-dimension value inside texture binding layouts.
/// The `cube` view-dimension value inside texture binding layouts.
/// `uint16` value for `GpuRenderPipelineDescriptor.primitive.indexFormat`.
/// `uint32` value for `GpuRenderPipelineDescriptor.primitive.indexFormat`.
/// The `entries` property key inside `GpuBindGroupDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ENTRIES: &str = "entries";

pub(crate) const WEBGPU_PROPERTY_SRC_FACTOR: &str = "srcFactor";

/// `source` property key inside `GpuImageCopyTexture` (used by `copyTextureToBuffer`).
pub(crate) const WEBGPU_PROPERTY_SOURCE: &str = "source";

/// The JavaScript property name `targets` on `GpuFragmentState`.
pub(crate) const WEBGPU_PROPERTY_TARGETS: &str = "targets";

/// The `storageTexture` value for `GpuBindingLayoutEntry`/`GpuBindGroupEntry`.
pub(crate) const WEBGPU_PROPERTY_STORAGE_TEXTURE: &str = "storageTexture";

pub(crate) const WEBGPU_TEXTURE_DIMENSION_2D: &str = "2d";

/// The `count` property key inside `GpuMultisampleState`.
///
/// When `count` is `1`, MSAA is disabled (one sample per pixel). Values like
/// `4` enable 4x multisample anti-aliasing.
pub(crate) const WEBGPU_PROPERTY_COUNT: &str = "count";

/// The JavaScript property name `module` on `GpuVertexState` / `GpuFragmentState`.
pub(crate) const WEBGPU_PROPERTY_MODULE: &str = "module";

/// The JavaScript method name `draw` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_DRAW: &str = "draw";

/// The `resolveTarget` property key inside `GpuRenderPassColorAttachment`.
///
/// Holds the destination `GpuTextureView` for MSAA resolve. Omit when MSAA is
/// disabled; present when the attachment's texture has `sampleCount > 1`.
pub(crate) const WEBGPU_PROPERTY_RESOLVE_TARGET: &str = "resolveTarget";

pub(crate) const WEBGPU_PROPERTY_WRITE_MASK: &str = "writeMask";

/// The `format` property key inside `GpuTextureDescriptor`.
pub(crate) const WEBGPU_PROPERTY_TEXTURE_FORMAT: &str = "format";

pub(crate) const WEBGPU_PROPERTY_MASK: &str = "mask";

/// `mipLevel` property key inside a texture binding entry / `writeTexture` dest.
pub(crate) const WEBGPU_PROPERTY_MIP_LEVEL: &str = "mipLevel";

/// Default `minDepth` value passed to `setViewport` when the caller did not
/// supply one. WebGPU's spec default is `0.0`.
pub(crate) const WEBGPU_DEFAULT_VIEWPORT_MIN_DEPTH: f64 = 0.0;

/// `attributes` property key inside `GpuVertexBufferLayout`.
pub(crate) const WEBGPU_PROPERTY_ATTRIBUTES: &str = "attributes";

/// The WebGPU context type string used to obtain a `GpuCanvasContext` from a canvas element.
///
/// This is the argument to `HTMLCanvasElement.getContext(...)` - **not** a
/// property name on `Navigator`. The browser exposes WebGPU via
/// `Navigator.gpu` (the string `"gpu"`), which is a separate concept.
/// Mixing the two up is a long-standing bug; see
/// [`WEBGPU_NAVIGATOR_GPU_KEY`] for the navigator-side key.
pub(crate) const WEBGPU_CONTEXT_TYPE: &str = "webgpu";

/// `GpuRenderPassEncoder.setScissorRect(x, y, w, h)` method name.
pub(crate) const WEBGPU_METHOD_SET_SCISSOR_RECT: &str = "setScissorRect";

/// `GpuRenderPassEncoder.setViewport(x, y, w, h, minDepth, maxDepth)` method name.
pub(crate) const WEBGPU_METHOD_SET_VIEWPORT: &str = "setViewport";

/// The JavaScript property name `vertex` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_VERTEX: &str = "vertex";

/// The `viewDimension` property key inside texture binding layouts.
pub(crate) const WEBGPU_PROPERTY_VIEW_DIMENSION: &str = "viewDimension";

/// The JavaScript method name `destroy` on `GpuDevice`.
///
/// Used by `WebGpuRenderer::dispose` to release GPU memory. After
/// `destroy` is called any further use of the device raises a JS
/// error, so this must only run as the final teardown step.
pub(crate) const WEBGPU_METHOD_DESTROY: &str = "destroy";

/// `GpuRenderPassEncoder.setBlendConstant(color)` method name.
pub(crate) const WEBGPU_METHOD_SET_BLEND_CONSTANT: &str = "setBlendConstant";

/// The JavaScript method name `createRenderPipeline` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_RENDER_PIPELINE: &str = "createRenderPipeline";

/// The `buffer` property key inside `GpuBufferBinding`.
pub(crate) const WEBGPU_PROPERTY_BUFFER: &str = "buffer";

/// `depthStencilAttachment` property key inside `GpuRenderPassDescriptor` (alt spelling).
pub(crate) const WEBGPU_PROPERTY_DEPTH_STENCIL_ATTACHMENT: &str = "depthStencilAttachment";

/// Map a buffer for CPU readback. Requires `MAP_READ` usage.
pub(crate) const WEBGPU_MAP_MODE_READ: f64 = 1.0;

/// The JavaScript property name `colorAttachments` on `GpuRenderPassDescriptor`.
pub(crate) const WEBGPU_PROPERTY_COLOR_ATTACHMENTS: &str = "colorAttachments";

/// The JavaScript method name `createView` on `GpuTexture`.
pub(crate) const WEBGPU_METHOD_CREATE_VIEW: &str = "createView";

/// `mipLevelCount` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MIP_LEVEL_COUNT: &str = "mipLevelCount";

pub(crate) const WEBGPU_PROPERTY_UNCLIPPED_DEPTH: &str = "unclippedDepth";

/// Texture usage: sampled texture binding.
pub(crate) const WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING: f64 = 4.0;

pub(crate) const WEBGPU_PROPERTY_OPERATION: &str = "operation";

/// The JavaScript method name `end` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_END: &str = "end";

/// Texture usage: written in a shader (storage texture).
pub(crate) const WEBGPU_TEXTURE_USAGE_COPY_DST: f64 = 2.0;

/// `dimension` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_DIMENSION: &str = "dimension";

/// The JavaScript property name `code` on `GpuShaderModuleDescriptor`.
pub(crate) const WEBGPU_PROPERTY_CODE: &str = "code";

/// `addressModeU` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ADDRESS_MODE_U: &str = "addressModeU";

/// `GpuQueue.writeTexture(destination, data, dataLayout, size)` method name.
pub(crate) const WEBGPU_METHOD_WRITE_TEXTURE: &str = "writeTexture";

pub(crate) const WEBGPU_PROPERTY_FRONT_FACE: &str = "frontFace";

pub(crate) const WEBGPU_PROPERTY_STRIP_INDEX_FORMAT: &str = "stripIndexFormat";

/// The JavaScript method name `getCurrentTexture` on `GpuCanvasContext`.
pub(crate) const WEBGPU_METHOD_GET_CURRENT_TEXTURE: &str = "getCurrentTexture";

/// `compute` property key inside `GpuRenderPassDescriptor` (compute-pass dispatch).
pub(crate) const WEBGPU_PROPERTY_COMPUTE: &str = "compute";

/// `depthOrArrayLayers` property key inside `GpuExtent3D`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_OR_1: &str = "depthOrArrayLayers";

/// `depthReadOnly` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_READ_ONLY: &str = "depthReadOnly";

/// `copySize` property key inside `GpuImageCopyTexture` (used by `copyTextureToBuffer`).
pub(crate) const WEBGPU_PROPERTY_COPY_SIZE: &str = "copySize";

/// The JavaScript method name `createTexture` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_TEXTURE: &str = "createTexture";

/// The JavaScript method name `getBindGroupLayout` on `GpuRenderPipeline`.
///
/// With `layout: "auto"` pipelines the bind group layout is derived from the
/// shader; index `0` corresponds to `@group(0)` in WGSL.
pub(crate) const WEBGPU_METHOD_GET_BIND_GROUP_LAYOUT: &str = "getBindGroupLayout";

/// The `comparison` property key inside `GpuSamplerBindingLayout`.
/// The `readOnly` property key inside storage-texture binding layouts.
pub(crate) const WEBGPU_PROPERTY_READ_ONLY: &str = "readOnly";

/// The JavaScript method name `setVertexBuffer(slot, buffer)` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_VERTEX_BUFFER: &str = "setVertexBuffer";

/// The JavaScript property name `r` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_R: &str = "r";

/// The `texture` property key inside `GpuTextureBindingLayout`.
/// The `sampler` property key inside `GpuSamplerBindingLayout`.
pub(crate) const WEBGPU_PROPERTY_SAMPLER_BINDING_TYPE: &str = "sampler";

/// The JavaScript property name `g` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_G: &str = "g";

/// The `size` property key inside `GpuTextureDescriptor`.
///
/// Holds a `GpuExtent3dDict` describing width/height/depth of the texture.
pub(crate) const WEBGPU_PROPERTY_SIZE: &str = "size";

/// The `querySet` property key inside `GpuComputePassDescriptor.timestampWrites`.
/// The `beginningOfPassWriteIndex` property key for timestamp writes.
/// The `endOfPassWriteIndex` property key for timestamp writes.
/// The `timestamp-writes` descriptor field on render / compute passes.
/// The bitmask value for `GPUBufferUsage.MAP_READ` (`0x01`).
/// The bitmask value for `GPUBufferUsage.MAP_WRITE` (`0x02`).
/// The bitmask value for `GPUBufferUsage.STORAGE` (`0x80`).
///
/// Required on any buffer that is bound as `var<storage>` in WGSL.
/// The bitmask value for `GPUBufferUsage.INDIRECT` (`0x100`).
///
/// Required on buffers used as the indirect argument of `drawIndexedIndirect` / `drawIndirect`.
/// The bitmask value for `GPUBufferUsage.QUERY_RESOLVE` (`0x200`).
///
/// Required on the destination buffer of `commandEncoder.resolveQuerySet`.
/// `timestamp` query type inside `GpuQuerySetDescriptor`.
pub(crate) const WEBGPU_QUERY_TYPE_TIMESTAMP: &str = "timestamp";

/// `GpuDevice.pushErrorScope(filter)` method name.
pub(crate) const WEBGPU_METHOD_PUSH_ERROR_SCOPE: &str = "pushErrorScope";

/// The JavaScript property name `topology` on `GpuPrimitiveState`.
pub(crate) const WEBGPU_PROPERTY_TOPOLOGY: &str = "topology";

/// The JavaScript method name `submit` on `GpuQueue`.
pub(crate) const WEBGPU_METHOD_SUBMIT: &str = "submit";

/// `depthStencil` property key inside `GpuRenderPassDescriptor`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_STENCIL: &str = "depthStencil";

/// `arrayStride` property key inside `GpuVertexBufferLayout`.
pub(crate) const WEBGPU_PROPERTY_ARRAY_STRIDE: &str = "arrayStride";

/// `stepMode` property key inside `GpuVertexBufferLayout`.
pub(crate) const WEBGPU_PROPERTY_STEP_MODE: &str = "stepMode";

/// The `indexFormat` property key inside `GpuRenderPipelineDescriptor.primitive`.
/// The `type` property key inside `GpuBufferBindingLayout`.
pub(crate) const WEBGPU_PROPERTY_TYPE: &str = "type";

/// The fragment shader entry point name used in WGSL shaders.
pub(crate) const WEBGPU_FRAGMENT_ENTRY_POINT: &str = "fs_main";

/// The JavaScript method name `beginRenderPass` on `GpuCommandEncoder`.
pub(crate) const WEBGPU_METHOD_BEGIN_RENDER_PASS: &str = "beginRenderPass";
