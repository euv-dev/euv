/// The `navigator` property key carrying the `GPU` object.
pub(crate) const BACKEND_WEBGPU_NAVIGATOR_KEY: &str = "gpu";

/// The method name reading the preferred canvas format.
pub(crate) const BACKEND_WEBGPU_PREFERRED_FORMAT: &str = "getPreferredCanvasFormat";

/// The lowercase `bgra8unorm` swap-chain format.
pub(crate) const BACKEND_WEBGPU_FORMAT_BGRA8_UNORM: &str = "bgra8unorm";

/// The property carrying a `GPUError`'s message.
pub(crate) const BACKEND_WEBGPU_ERROR_MESSAGE: &str = "message";

/// The `GPUCompilationMessageType` value that makes a message fatal.
pub(crate) const BACKEND_WEBGPU_MESSAGE_ERROR: &str = "error";

/// The debug label attached to the ray-trace shader module.
pub(crate) const BACKEND_WEBGPU_SHADER_LABEL: &str = "euv-raytrace";

/// The `GPUErrorFilter` value wrapping each resource allocation.
pub(crate) const BACKEND_WEBGPU_SCOPE_VALIDATION: &str = "validation";

/// The `GPUBufferUsage.STORAGE` flag both packed buffers carry.
pub(crate) const BACKEND_WEBGPU_USAGE_STORAGE: u32 = 128;

/// The `GPUBufferUsage.COPY_DST` flag, so `writeBuffer` may refill a
/// packed buffer in place.
pub(crate) const BACKEND_WEBGPU_USAGE_COPY_DST: u32 = 8;

/// The `GPUShaderStage.FRAGMENT` flag both binding entries declare,
/// because only the fragment stage reads them.
pub(crate) const BACKEND_WEBGPU_VISIBILITY_FRAGMENT: u32 = 2;

/// The WebGPU bind-group binding index of the packed scene buffer.
pub(crate) const BACKEND_WEBGPU_BINDING_SCENE: u32 = 0;

/// The WebGPU bind-group binding index of the packed frame buffer.
pub(crate) const BACKEND_WEBGPU_BINDING_FRAME: u32 = 1;

/// The `@group` index the shader's two bindings live in.
pub(crate) const BACKEND_WEBGPU_BIND_GROUP_INDEX: u32 = 0;

/// The number of vertices the WebGPU fullscreen triangle issues.
pub(crate) const BACKEND_WEBGPU_VERTEX_COUNT: u32 = 3;

/// The instance count a fullscreen draw issues.
pub(crate) const BACKEND_WEBGPU_INSTANCE_COUNT: u32 = 1;

/// The first vertex the fullscreen triangle starts at.
pub(crate) const BACKEND_WEBGPU_FIRST_VERTEX: u32 = 0;

/// The first instance the fullscreen triangle starts at.
pub(crate) const BACKEND_WEBGPU_FIRST_INSTANCE: u32 = 0;

/// The size in bytes of one `f32` slot in a packed storage buffer.
pub(crate) const BACKEND_WEBGPU_F32_SIZE: usize = 4;

/// The byte size each packed storage buffer starts at before the
/// first real block decides its final capacity.
///
/// The initial allocation exists so a bind group can reference both
/// buffers at all — a zero-sized buffer is not a valid binding — while
/// staying far below every adapter's storage limit. Pre-reserving the
/// whole 512 MiB packed budget would exceed the storage-binding limit
/// on the common adapters and fail the very first allocation.
pub(crate) const BACKEND_WEBGPU_INITIAL_BUFFER_BYTES: u32 = 4096;

/// The `GPUTextureUsage.RENDER_ATTACHMENT` flag the swap chain needs.
pub(crate) const BACKEND_WEBGPU_TEXTURE_RENDER_ATTACHMENT: u32 = 16;

/// The `GPUCanvasAlphaMode.opaque` value the context is configured with.
pub(crate) const BACKEND_WEBGPU_ALPHA_OPAQUE: &str = "opaque";

/// The `GPUBufferBindingType.read-only-storage` value.
pub(crate) const BACKEND_WEBGPU_BUFFER_READ_ONLY_STORAGE: &str = "read-only-storage";

/// The descriptor property naming the device.
pub(crate) const BACKEND_WEBGPU_PROPERTY_DEVICE: &str = "device";

/// The descriptor property naming the swap-chain format.
pub(crate) const BACKEND_WEBGPU_PROPERTY_FORMAT: &str = "format";

/// The descriptor property naming the alpha mode.
pub(crate) const BACKEND_WEBGPU_PROPERTY_ALPHA_MODE: &str = "alphaMode";

/// The descriptor property naming the texture usage.
pub(crate) const BACKEND_WEBGPU_PROPERTY_USAGE: &str = "usage";

/// The descriptor property naming a bind-group binding index.
pub(crate) const BACKEND_WEBGPU_PROPERTY_BINDING: &str = "binding";

/// The descriptor property naming a binding's shader stage visibility.
pub(crate) const BACKEND_WEBGPU_PROPERTY_VISIBILITY: &str = "visibility";

/// The descriptor property naming a binding layout's buffer layout.
pub(crate) const BACKEND_WEBGPU_PROPERTY_BUFFER: &str = "buffer";

/// The descriptor property naming a binding layout's type.
pub(crate) const BACKEND_WEBGPU_PROPERTY_TYPE: &str = "type";

/// The descriptor property naming a bind group's entries.
pub(crate) const BACKEND_WEBGPU_PROPERTY_ENTRIES: &str = "entries";

/// The descriptor property naming a bind group's layout.
pub(crate) const BACKEND_WEBGPU_PROPERTY_LAYOUT: &str = "layout";

/// The descriptor property naming a bind-group entry's resource.
pub(crate) const BACKEND_WEBGPU_PROPERTY_RESOURCE: &str = "resource";

/// The descriptor property naming a pipeline layout's bind-group layouts.
pub(crate) const BACKEND_WEBGPU_PROPERTY_BIND_GROUP_LAYOUTS: &str = "bindGroupLayouts";

/// The descriptor property naming a pipeline's vertex stage.
pub(crate) const BACKEND_WEBGPU_PROPERTY_VERTEX: &str = "vertex";

/// The descriptor property naming a pipeline's fragment stage.
pub(crate) const BACKEND_WEBGPU_PROPERTY_FRAGMENT: &str = "fragment";

/// The descriptor property naming a stage's shader module.
pub(crate) const BACKEND_WEBGPU_PROPERTY_MODULE: &str = "module";

/// The descriptor property naming a stage's colour targets.
pub(crate) const BACKEND_WEBGPU_PROPERTY_TARGETS: &str = "targets";

/// The descriptor property naming a colour target's format.
pub(crate) const BACKEND_WEBGPU_PROPERTY_FORMAT_TARGET: &str = "format";

/// The descriptor property naming a pipeline's primitive state.
pub(crate) const BACKEND_WEBGPU_PROPERTY_PRIMITIVE: &str = "primitive";

/// The descriptor property naming a primitive topology.
pub(crate) const BACKEND_WEBGPU_PROPERTY_TOPOLOGY: &str = "topology";

/// The primitive topology value a triangle list names.
pub(crate) const BACKEND_WEBGPU_TOPOLOGY_TRIANGLE_LIST: &str = "triangle-list";

/// The descriptor property naming a stage's shader entry point.
pub(crate) const BACKEND_WEBGPU_PROPERTY_ENTRY_POINT: &str = "entryPoint";

/// The descriptor property naming a render pass's colour attachments.
pub(crate) const BACKEND_WEBGPU_PROPERTY_COLOR_ATTACHMENTS: &str = "colorAttachments";

/// The descriptor property naming a colour attachment's view.
pub(crate) const BACKEND_WEBGPU_PROPERTY_VIEW: &str = "view";

/// The descriptor property naming a colour attachment's clear colour.
pub(crate) const BACKEND_WEBGPU_PROPERTY_CLEAR_VALUE: &str = "clearValue";

/// The descriptor property naming an attachment's load operation.
pub(crate) const BACKEND_WEBGPU_PROPERTY_LOAD_OP: &str = "loadOp";

/// The descriptor property naming an attachment's store operation.
pub(crate) const BACKEND_WEBGPU_PROPERTY_STORE_OP: &str = "storeOp";

/// The attachment load operation value.
pub(crate) const BACKEND_WEBGPU_LOAD_CLEAR: &str = "clear";

/// The attachment store operation value.
pub(crate) const BACKEND_WEBGPU_STORE_STORE: &str = "store";

/// The descriptor property naming a shader module's code.
pub(crate) const BACKEND_WEBGPU_PROPERTY_CODE: &str = "code";

/// The descriptor property naming a shader module's label.
pub(crate) const BACKEND_WEBGPU_PROPERTY_LABEL: &str = "label";

/// The descriptor property naming a buffer's size in bytes.
pub(crate) const BACKEND_WEBGPU_PROPERTY_SIZE: &str = "size";

/// The descriptor property naming a buffer's usage flags.
pub(crate) const BACKEND_WEBGPU_PROPERTY_USAGE_FLAGS: &str = "usage";

/// The property a compilation info carries its messages under.
pub(crate) const BACKEND_WEBGPU_PROPERTY_MESSAGES: &str = "messages";

/// The property a compilation message carries its kind under.
///
/// Distinct from [`BACKEND_WEBGPU_PROPERTY_TYPE`], which names a
/// binding layout's dictionary type; both happen to be the string
/// `type`.
pub(crate) const BACKEND_WEBGPU_PROPERTY_MESSAGE_TYPE: &str = "type";

/// The property a compilation message carries its text under.
pub(crate) const BACKEND_WEBGPU_PROPERTY_MESSAGE_TEXT: &str = "message";

/// The property a compilation message carries its line under.
pub(crate) const BACKEND_WEBGPU_PROPERTY_LINE_NUM: &str = "lineNum";

/// The property carrying a device's submit queue.
///
/// `GPUDevice.queue` is an attribute, not a method: looking it up and
/// calling the result throws `queue is not callable` before a single
/// buffer exists.
pub(crate) const BACKEND_WEBGPU_PROPERTY_QUEUE: &str = "queue";

/// The property carrying a device's limits.
///
/// Like [`BACKEND_WEBGPU_PROPERTY_QUEUE`], `GPUDevice.limits` is an
/// attribute holding a `GPUSupportedLimits` dictionary.
pub(crate) const BACKEND_WEBGPU_PROPERTY_LIMITS: &str = "limits";

/// The property carrying a device's loss promise.
///
/// `GPUDevice.lost` is always present, so its existence proves
/// nothing; only the promise's resolution says the device died.
pub(crate) const BACKEND_WEBGPU_PROPERTY_LOST: &str = "lost";

/// The `GPUDeviceLostInfo.reason` value of a deliberate destroy.
pub(crate) const BACKEND_WEBGPU_LOST_REASON: &str = "reason";

/// The method chaining a `device.lost` promise onto the loss handler.
pub(crate) const BACKEND_WEBGPU_METHOD_THEN: &str = "then";

/// The event name a device reports uncaptured errors under.
pub(crate) const BACKEND_WEBGPU_EVENT_UNCAPTURED: &str = "uncapturederror";

/// The `GPUUncapturedErrorEvent.error` property.
pub(crate) const BACKEND_WEBGPU_PROPERTY_ERROR: &str = "error";

/// The method registering an event listener on a device.
pub(crate) const BACKEND_WEBGPU_METHOD_ADD_LISTENER: &str = "addEventListener";

/// The method unregistering an event listener from a device.
///
/// The `uncapturederror` listener owns a Rust closure, so it has to be
/// unregistered before that closure is dropped: a device that keeps
/// the listener would invoke an invalidated callback the next time any
/// other validation error fires.
pub(crate) const BACKEND_WEBGPU_METHOD_REMOVE_LISTENER: &str = "removeEventListener";

/// The limit naming the largest addressable storage binding, in bytes.
pub(crate) const BACKEND_WEBGPU_LIMIT_STORAGE_BINDING: &str = "maxStorageBufferBindingSize";

/// The limit naming the largest allocatable buffer, in bytes.
pub(crate) const BACKEND_WEBGPU_LIMIT_BUFFER_SIZE: &str = "maxBufferSize";

/// The limit naming the largest texture edge, in pixels.
///
/// This one counts pixels, not bytes: it bounds `width` and `height`
/// separately, and is the only limit a surface size has to respect.
pub(crate) const BACKEND_WEBGPU_LIMIT_TEXTURE_2D: &str = "maxTextureDimension2D";

/// The `vs_main` entry point the vertex stage names.
///
/// The WGSL module the pipeline compiles declares two entry points,
/// `vs_main` and `fs_main`, and no `main`: naming one constant for both
/// stages asks the vertex stage for a fragment entry point and the
/// browser rejects the whole pipeline.
pub(crate) const BACKEND_WEBGPU_ENTRY_VERTEX: &str = "vs_main";

/// The `fs_main` entry point the fragment stage names.
pub(crate) const BACKEND_WEBGPU_ENTRY_FRAGMENT: &str = "fs_main";

/// The method binding a pipeline to a render pass.
pub(crate) const BACKEND_WEBGPU_METHOD_SET_PIPELINE: &str = "setPipeline";

/// The method binding a bind group to a render pass.
pub(crate) const BACKEND_WEBGPU_METHOD_SET_BIND_GROUP: &str = "setBindGroup";

/// The method issuing a draw inside a render pass.
pub(crate) const BACKEND_WEBGPU_METHOD_DRAW: &str = "draw";

/// The method closing a render pass.
pub(crate) const BACKEND_WEBGPU_METHOD_END: &str = "end";

/// The method configuring a canvas context.
pub(crate) const BACKEND_WEBGPU_METHOD_CONFIGURE: &str = "configure";

/// The method releasing a canvas context's swap chain.
pub(crate) const BACKEND_WEBGPU_METHOD_UNCONFIGURE: &str = "unconfigure";

/// The method reading a context's swap-chain texture.
pub(crate) const BACKEND_WEBGPU_METHOD_CURRENT_TEXTURE: &str = "getCurrentTexture";

/// The method deriving a bindable view from a texture.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_VIEW: &str = "createView";

/// The method requesting an adapter.
pub(crate) const BACKEND_WEBGPU_METHOD_REQUEST_ADAPTER: &str = "requestAdapter";

/// The method requesting a device.
pub(crate) const BACKEND_WEBGPU_METHOD_REQUEST_DEVICE: &str = "requestDevice";

/// The method opening a validation error scope.
pub(crate) const BACKEND_WEBGPU_METHOD_PUSH_SCOPE: &str = "pushErrorScope";

/// The method closing a validation error scope.
pub(crate) const BACKEND_WEBGPU_METHOD_POP_SCOPE: &str = "popErrorScope";

/// The method creating a buffer.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_BUFFER: &str = "createBuffer";

/// The method creating a bind-group layout.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_LAYOUT: &str = "createBindGroupLayout";

/// The method creating a bind group.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_BIND_GROUP: &str = "createBindGroup";

/// The method creating a pipeline layout.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_PIPELINE_LAYOUT: &str = "createPipelineLayout";

/// The method creating a render pipeline asynchronously.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_PIPELINE: &str = "createRenderPipelineAsync";

/// The method creating a shader module.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_MODULE: &str = "createShaderModule";

/// The method creating a command encoder.
pub(crate) const BACKEND_WEBGPU_METHOD_CREATE_ENCODER: &str = "createCommandEncoder";

/// The method reading a shader module's compilation info.
pub(crate) const BACKEND_WEBGPU_METHOD_COMPILATION_INFO: &str = "getCompilationInfo";

/// The method opening a render pass on an encoder.
pub(crate) const BACKEND_WEBGPU_METHOD_BEGIN_PASS: &str = "beginRenderPass";

/// The method finishing a command encoder.
pub(crate) const BACKEND_WEBGPU_METHOD_FINISH: &str = "finish";

/// The method submitting a command buffer.
pub(crate) const BACKEND_WEBGPU_METHOD_SUBMIT: &str = "submit";

/// The method writing a storage buffer.
pub(crate) const BACKEND_WEBGPU_METHOD_WRITE_BUFFER: &str = "writeBuffer";

/// The method destroying a buffer.
pub(crate) const BACKEND_WEBGPU_METHOD_DESTROY: &str = "destroy";

/// The error reported when the shader module compiled with errors.
pub(crate) const BACKEND_WEBGPU_ERROR_SHADER: &str = "the WGSL shader module is invalid";

/// The error reported when the render pipeline failed validation.
pub(crate) const BACKEND_WEBGPU_ERROR_PIPELINE: &str = "the WebGPU render pipeline is invalid";

/// The error reported when the swap chain produced no usable texture.
pub(crate) const BACKEND_WEBGPU_ERROR_TEXTURE: &str = "the WebGPU swap chain produced no texture";

/// The error reported when a texture yielded no bindable view.
pub(crate) const BACKEND_WEBGPU_ERROR_VIEW: &str = "the WebGPU texture produced no view";

/// The diagnostic used when the failure record is momentarily borrowed.
pub(crate) const WEBGPU_FAILURE_BUSY: &str =
    "the WebGPU failure record is being read by another frame";

/// The diagnostic used when a popped scope reported no message at all.
pub(crate) const WEBGPU_SCOPE_SILENT: &str =
    "the WebGPU device rejected the operation without a validation message";
