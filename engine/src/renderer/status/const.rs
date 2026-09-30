pub(crate) const WEBGPU_VERTEX_FORMAT_FLOAT32: &str = "float32";

/// The CSS composite operation string for the `Luminosity` blend mode.
pub(crate) const BLEND_MODE_LUMINOSITY: &str = "luminosity";

pub(crate) const WEBGPU_COMPARE_NOT_EQUAL: &str = "not-equal";

pub(crate) const WEBGPU_VERTEX_FORMAT_FLOAT32X4: &str = "float32x4";

/// The bitmask value for `GPUBufferUsage.MAP_WRITE` (`0x02`), the counterpart
/// of [`BufferUsage::MapWrite`].
///
/// Required on any buffer the CPU writes through `mapAsync`.
pub(crate) const RENDER_USAGE_MAP_WRITE: f64 = 2.0;

/// The CSS composite operation string for the `Saturation` blend mode.
pub(crate) const BLEND_MODE_SATURATION: &str = "saturation";

pub(crate) const WEBGPU_BLEND_OPERATION_SUBTRACT: &str = "subtract";

/// Validation-layer error filter; catches shader compile/link errors,
/// bind-group mismatches, OOB draws, etc.
pub(crate) const WEBGPU_ERROR_FILTER_VALIDATION: &str = "validation";

pub(crate) const WEBGPU_COMPARE_LESS_EQUAL: &str = "less-equal";

/// The CSS composite operation string for the `Hue` blend mode.
pub(crate) const BLEND_MODE_HUE: &str = "hue";

/// The CSS composite operation string for the `Exclusion` blend mode.
pub(crate) const BLEND_MODE_EXCLUSION: &str = "exclusion";

/// `clamp-to-edge` address mode for U / V / W sampler axes.
pub(crate) const WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE: &str = "clamp-to-edge";

/// The bitmask value for `GPUBufferUsage.UNIFORM` (`0x40`), the counterpart
/// of [`BufferUsage::Uniform`].
///
/// Required on any buffer bound as `var<uniform>` in WGSL.
pub(crate) const RENDER_USAGE_UNIFORM: f64 = 64.0;

/// The CSS composite operation string for the `Lighten` blend mode.
pub(crate) const BLEND_MODE_LIGHTEN: &str = "lighten";

pub(crate) const WEBGPU_INDEX_FORMAT_UINT16: &str = "uint16";

/// The WebGPU `clear` load operation string.
pub(crate) const WEBGPU_LOAD_OP_CLEAR: &str = "clear";

pub(crate) const WEBGPU_BLEND_FACTOR_DST: &str = "dst";

pub(crate) const WEBGPU_VERTEX_FORMAT_SINT32: &str = "sint32";

pub(crate) const WEBGPU_BLEND_FACTOR_ONE: &str = "one";

/// The CSS composite operation string for the `HardLight` blend mode.
pub(crate) const BLEND_MODE_HARD_LIGHT: &str = "hard-light";

/// The bitmask value for `GPUTextureUsage.COPY_SRC` (`0x01`), the counterpart
/// of [`TextureUsage::CopySource`].
///
/// Deliberately distinct from [`RENDER_USAGE_COPY_SRC`], which is the
/// *buffer* bit (`0x04`): the two namespaces share their variant names
/// but not their values, which is why they get their own constants
/// rather than sharing one.
pub(crate) const RENDER_TEXTURE_USAGE_COPY_SRC: f64 = 1.0;

/// The CSS composite operation string for the `SoftLight` blend mode.
pub(crate) const BLEND_MODE_SOFT_LIGHT: &str = "soft-light";

/// The bitmask value for `GPUBufferUsage.COPY_DST` (`0x08`), the counterpart
/// of [`BufferUsage::CopyDestination`].
///
/// Required on any buffer that is the destination of a copy or of
/// `queue.writeBuffer`.
pub(crate) const RENDER_USAGE_COPY_DST: f64 = 8.0;

pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC: &str = "one-minus-src";

/// `textureSampleCompare` comparison function: keep fragments closer to
/// the camera than the reference depth.
pub(crate) const WEBGPU_COMPARE_LESS: &str = "less";

pub(crate) const WEBGPU_BLEND_FACTOR_SRC: &str = "src";

pub(crate) const WEBGPU_INDEX_FORMAT_UINT32: &str = "uint32";

pub(crate) const WEBGPU_CULL_MODE_FRONT: &str = "front";

/// The bitmask value for `GPUBufferUsage.VERTEX` (`0x08`), the counterpart
/// of [`BufferUsage::Vertex`].
///
/// Required on any buffer handed to `setVertexBuffer`.
pub(crate) const RENDER_USAGE_VERTEX: f64 = 32.0;

/// The bitmask value for `GPUBufferUsage.MAP_READ` (`0x01`), the counterpart
/// of [`BufferUsage::MapRead`].
///
/// Required on any buffer the CPU reads through `mapAsync`.
pub(crate) const RENDER_USAGE_MAP_READ: f64 = 1.0;

pub(crate) const WEBGPU_VERTEX_FORMAT_UINT32: &str = "uint32";

pub(crate) const WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: &str = "triangle-strip";

pub(crate) const WEBGPU_COMPARE_EQUAL: &str = "equal";

pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: &str = "one-minus-src-alpha";

pub(crate) const WEBGPU_BLEND_OPERATION_MAX: &str = "max";

/// 24-bit depth, 8-bit stencil, no multisample. The most common choice
/// for a render pass's depth-stencil attachment.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS_STENCIL8: &str = "depth24plus-stencil8";

/// The CSS composite operation string for the `ColorBurn` blend mode.
pub(crate) const BLEND_MODE_COLOR_BURN: &str = "color-burn";

/// The bitmask value for `GPUTextureUsage.RENDER_ATTACHMENT` (`0x10`), the counterpart
/// of [`TextureUsage::RenderAttachment`].
///
/// Required on any texture used as an attachment of a render pass.
pub(crate) const RENDER_USAGE_RENDER_ATTACHMENT: f64 = 16.0;

pub(crate) const WEBGPU_BLEND_FACTOR_DST_ALPHA: &str = "dst-alpha";

pub(crate) const WEBGPU_FORMAT_BGRA8UNORM: &str = "bgra8unorm";

/// The bitmask value for `GPUBufferUsage.STORAGE` (`0x80`), the counterpart
/// of [`BufferUsage::Storage`].
///
/// Required on any buffer bound as `var<storage>` in WGSL.
pub(crate) const RENDER_USAGE_STORAGE: f64 = 128.0;

/// The CSS composite operation string for the `Difference` blend mode.
pub(crate) const BLEND_MODE_DIFFERENCE: &str = "difference";

pub(crate) const WEBGPU_BLEND_FACTOR_CONSTANT_ALPHA: &str = "constant-alpha";

pub(crate) const WEBGPU_PRIMITIVE_TOPOLOGY_LINE_LIST: &str = "line-list";

/// 24-bit depth (no stencil) without MSAA. The default for shadow maps
/// and other single-sample depth render targets.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS: &str = "depth24plus";

/// The CSS composite operation string for the `Darken` blend mode.
pub(crate) const BLEND_MODE_DARKEN: &str = "darken";

pub(crate) const WEBGPU_BLEND_OPERATION_MIN: &str = "min";

/// The bitmask value for `GPUBufferUsage.INDIRECT` (`0x100`), the counterpart
/// of [`BufferUsage::Indirect`].
///
/// Required on any buffer supplying the arguments of an indirect draw or
/// dispatch.
pub(crate) const RENDER_USAGE_INDIRECT: f64 = 256.0;

pub(crate) const WEBGPU_CULL_MODE_BACK: &str = "back";

pub(crate) const WEBGPU_ADDRESS_MODE_REPEAT: &str = "repeat";

/// The WebGPU primitive topology string for triangle lists.
pub(crate) const WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: &str = "triangle-list";

/// The CSS composite operation string for the `Lighter` blend mode.
pub(crate) const BLEND_MODE_LIGHTER: &str = "lighter";

/// The bitmask value for `GPUShaderStage.FRAGMENT` (`0x02`), the counterpart
/// of [`ShaderStage::Fragment`].
pub(crate) const SHADER_STAGE_FRAGMENT: f64 = 2.0;

pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: &str = "one-minus-constant-alpha";

/// `loadOp` / `depthLoadOp` value that preserves the previous contents
/// of the attachment.
pub(crate) const WEBGPU_LOAD_OP_LOAD: &str = "load";

pub(crate) const WEBGPU_FRONT_FACE_COUNTER_CLOCKWISE: &str = "ccw";

/// Minification / magnification filter mode that picks the nearest texel.
pub(crate) const WEBGPU_FILTER_MODE_NEAREST: &str = "nearest";

pub(crate) const WEBGPU_VERTEX_FORMAT_FLOAT32X3: &str = "float32x3";

pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: &str = "one-minus-dst-alpha";

pub(crate) const WEBGPU_COMPARE_NEVER: &str = "never";

pub(crate) const WEBGPU_COMPARE_GREATER_EQUAL: &str = "greater-equal";

pub(crate) const WEBGPU_PRIMITIVE_TOPOLOGY_POINT_LIST: &str = "point-list";

pub(crate) const WEBGPU_BLEND_FACTOR_ZERO: &str = "zero";

pub(crate) const WEBGPU_BLEND_OPERATION_REVERSE_SUBTRACT: &str = "reverse-subtract";

/// The bitmask value for `GPUTextureUsage.TEXTURE_BINDING` (`0x04`), the counterpart
/// of [`TextureUsage::TextureBinding`].
///
/// Required on any texture sampled through a `texture_2d` binding.
pub(crate) const RENDER_USAGE_TEXTURE_BINDING: f64 = 4.0;

/// 32-bit float depth, no stencil. Required for view-space z-buffers
/// used in deferred renderers.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH32_FLOAT: &str = "depth32float";

pub(crate) const WEBGPU_ADDRESS_MODE_MIRROR_REPEAT: &str = "mirror-repeat";

/// The bitmask value for `GPUShaderStage.VERTEX` (`0x01`), the counterpart
/// of [`ShaderStage::Vertex`].
pub(crate) const SHADER_STAGE_VERTEX: f64 = 1.0;

pub(crate) const WEBGPU_BLEND_FACTOR_SRC_ALPHA_SATURATED: &str = "src-alpha-saturated";

/// The CSS composite operation string for the `ColorDodge` blend mode.
pub(crate) const BLEND_MODE_COLOR_DODGE: &str = "color-dodge";

pub(crate) const WEBGPU_PRIMITIVE_TOPOLOGY_LINE_STRIP: &str = "line-strip";

pub(crate) const WEBGPU_VERTEX_FORMAT_UNORM8X4: &str = "unorm8x4";

/// The bitmask value for `GPUTextureUsage.COPY_DST` (`0x02`), the counterpart
/// of [`TextureUsage::CopyDestination`].
///
/// Deliberately distinct from [`RENDER_USAGE_COPY_DST`], which is the
/// *buffer* bit (`0x08`).
pub(crate) const RENDER_TEXTURE_USAGE_COPY_DST: f64 = 2.0;

pub(crate) const WEBGPU_BLEND_FACTOR_CONSTANT: &str = "constant";

/// The bitmask value for `GPUShaderStage.COMPUTE` (`0x04`), the counterpart
/// of [`ShaderStage::Compute`].
pub(crate) const SHADER_STAGE_COMPUTE: f64 = 4.0;

pub(crate) const WEBGPU_FORMAT_RGBA8UNORM: &str = "rgba8unorm";

/// The bitmask value for `GPUBufferUsage.INDEX` (`0x04`), the counterpart
/// of [`BufferUsage::Index`].
///
/// Required on any buffer handed to `setIndexBuffer` and consumed by
/// `drawIndexed`.
pub(crate) const RENDER_USAGE_INDEX: f64 = 16.0;

pub(crate) const WEBGPU_ERROR_FILTER_OUT_OF_MEMORY: &str = "out-of-memory";

/// The bitmask value for `GPUBufferUsage.COPY_SRC` (`0x01`), the counterpart
/// of [`BufferUsage::CopySource`].
///
/// Required on any buffer that is the source of a copy.
pub(crate) const RENDER_USAGE_COPY_SRC: f64 = 4.0;

/// The CSS composite operation string for the `Multiply` blend mode.
pub(crate) const BLEND_MODE_MULTIPLY: &str = "multiply";

pub(crate) const WEBGPU_FILTER_MODE_LINEAR: &str = "linear";

/// The bitmask value for `GPUBufferUsage.QUERY_RESOLVE` (`0x200`), the counterpart
/// of [`BufferUsage::QueryResolve`].
///
/// Required on the destination buffer of `resolveQuerySet`.
pub(crate) const RENDER_USAGE_QUERY_RESOLVE: f64 = 512.0;

pub(crate) const WEBGPU_COMPARE_GREATER: &str = "greater";

pub(crate) const WEBGPU_FORMAT_RGBA32FLOAT: &str = "rgba32float";

pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_DST: &str = "one-minus-dst";

/// The CSS composite operation string for the `Normal` blend mode.
pub(crate) const BLEND_MODE_NORMAL: &str = "source-over";

pub(crate) const WEBGPU_BLEND_FACTOR_SRC_ALPHA: &str = "src-alpha";

pub(crate) const WEBGPU_FORMAT_R32FLOAT: &str = "r32float";

/// `storeOp` value that discards the result (saves bandwidth when we
/// will not read it back, e.g. the depth buffer at the end of a pass).
pub(crate) const WEBGPU_STORE_OP_DISCARD: &str = "discard";

/// The bitmask value for `GPUTextureUsage.STORAGE_BINDING` (`0x10`), the counterpart
/// of [`TextureUsage::StorageBinding`].
///
/// Required on any texture bound as a `texture_storage_2d` in WGSL.
pub(crate) const RENDER_USAGE_STORAGE_BINDING: f64 = 8.0;

pub(crate) const WEBGPU_FORMAT_RGBA16FLOAT: &str = "rgba16float";

pub(crate) const WEBGPU_VERTEX_FORMAT_FLOAT32X2: &str = "float32x2";

pub(crate) const WEBGPU_CULL_MODE_NONE: &str = "none";

pub(crate) const WEBGPU_COMPARE_ALWAYS: &str = "always";

pub(crate) const WEBGPU_BLEND_FACTOR_ONE_MINUS_CONSTANT: &str = "one-minus-constant";

pub(crate) const WEBGPU_ERROR_FILTER_INTERNAL: &str = "internal";

pub(crate) const WEBGPU_FRONT_FACE_CLOCKWISE: &str = "cw";

/// The WebGPU `store` store operation string.
pub(crate) const WEBGPU_STORE_OP_STORE: &str = "store";

pub(crate) const WEBGPU_BLEND_OPERATION_ADD: &str = "add";

/// The CSS composite operation string for the `Color` blend mode.
pub(crate) const BLEND_MODE_COLOR: &str = "color";

/// The CSS composite operation string for the `Screen` blend mode.
pub(crate) const BLEND_MODE_SCREEN: &str = "screen";

/// The CSS composite operation string for the `Overlay` blend mode.
pub(crate) const BLEND_MODE_OVERLAY: &str = "overlay";

/// 2D view dimension (the default for `GpuTexture.createView`).
pub(crate) const WEBGPU_TEXTURE_VIEW_DIMENSION_2D: &str = "2d";

/// The `GPUVertexStepMode.VERTEX` string, the counterpart of
/// [`VertexStepMode::Vertex`].
pub(crate) const WEBGPU_VERTEX_STEP_MODE_VERTEX: &str = "vertex";

/// The `GPUVertexStepMode.INSTANCE` string, the counterpart of
/// [`VertexStepMode::Instance`].
pub(crate) const WEBGPU_VERTEX_STEP_MODE_INSTANCE: &str = "instance";
