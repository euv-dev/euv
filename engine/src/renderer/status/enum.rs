use super::*;

/// Defines how new pixels are composited with existing pixels on the canvas.
///
/// Maps directly to the CSS `globalCompositeOperation` property.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BlendMode {
    /// The source is drawn over the destination (default alpha blending).
    #[default]
    Normal,
    /// The source color is multiplied with the destination, producing a darker result.
    Multiply,
    /// The source and destination are inverted, multiplied, then inverted again.
    Screen,
    /// The source and destination colors are added together, clamped to maximum brightness.
    Lighter,
    /// Combines `Multiply` and `Screen` based on the destination color.
    Overlay,
    /// Keeps the darker of the source and destination per channel.
    Darken,
    /// Keeps the lighter of the source and destination per channel.
    Lighten,
    /// Dodges the destination color brightening it based on the source.
    ColorDodge,
    /// Burns the destination color darkening it based on the source.
    ColorBurn,
    /// A harsher version of `Overlay` using the source color as the filter.
    HardLight,
    /// A softer version of `Overlay` using the source color as the filter.
    SoftLight,
    /// Subtracts the darker color from the lighter color per channel.
    Difference,
    /// Similar to `Difference` but with lower contrast.
    Exclusion,
    /// Uses the hue of the source with the saturation and luminosity of the destination.
    Hue,
    /// Uses the saturation of the source with the hue and luminosity of the destination.
    Saturation,
    /// Uses the hue and saturation of the source with the luminosity of the destination.
    Color,
    /// Uses the luminosity of the source with the hue and saturation of the destination.
    Luminosity,
}

/// Rendering quality preset controlling anti-aliasing smoothing strategy.
///
/// Maps to the canvas `imageSmoothingQuality` value plus an explicit
/// `imageSmoothingEnabled` toggle. Combined with a CSS `image-rendering:
/// pixelated` rule on the consumer side, `Low` produces crisp pixel-art
/// rendering while `High` produces smooth vector-style rendering.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RenderQuality {
    /// Fastest rendering, pixelated scaling.
    ///
    /// Disables `imageSmoothingEnabled` on the canvas context and sets
    /// `imageSmoothingQuality = "low"`. Pair with CSS `image-rendering:
    /// pixelated` for sharp nearest-neighbour scaling.
    Low,
    /// Balanced rendering with default smoothing quality.
    ///
    /// Sets `imageSmoothingQuality = "medium"`.
    Medium,
    /// Highest fidelity rendering with smooth edges and high-quality scaling.
    ///
    /// Sets `imageSmoothingQuality = "high"`. Best for vector-style content
    /// on HiDPI displays. This is the default — when no explicit quality is
    /// requested, the engine errs on the side of visual fidelity rather than
    /// performance, since users typically notice aliasing artifacts before
    /// they notice a few extra milliseconds of GPU time.
    #[default]
    High,
}

/// Whether a vertex buffer is consumed per-vertex or per-instance.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum VertexStepMode {
    /// Advance the buffer one vertex at a time.
    #[default]
    Vertex,
    /// Advance the buffer one entry at a time, for all vertices of an
    /// instance.
    Instance,
}

/// What a render pass attachment does with the contents it already holds
/// before the pass draws anything.
///
/// Replaces the `"clear"` / `"load"` string literals that
/// `GpuRenderPassColorAttachment.loadOp` and
/// `GpuRenderPassDepthStencilAttachment.depthLoadOp` accept. A typo in a
/// string literal is a silent runtime failure (the browser rejects the pass
/// and the frame renders nothing); a typo in a variant is a compile error.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LoadOp {
    /// Overwrite the attachment with its clear value before drawing.
    ///
    /// The clear value is [`ColorAttachment::clear`] for color
    /// attachments and [`DepthStencilAttachment::depth_clear`] for
    /// depth-stencil attachments. WebGPU requires the clear value to be
    /// present whenever this variant is used, which is why the clear
    /// fields on those two structs are `Option` and paired with a
    /// non-optional load op rather than with `Option<&'static str>`
    /// defaults resolved by heuristic.
    #[default]
    Clear,
    /// Keep the attachment's existing contents and draw over them.
    ///
    /// The clear value must be absent; any value present is ignored.
    Load,
}

/// What a render pass attachment does with the contents it produced once the
/// pass ends.
///
/// Replaces the `"store"` / `"discard"` string literals accepted by
/// `storeOp` and `depthStoreOp`. `Discard` saves bandwidth for attachments
/// the pass writes but nothing ever reads back — the depth buffer of a
/// depth-test-only pass is the canonical case.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum StoreOp {
    /// Keep the attachment's contents after the pass so later passes can
    /// read or resolve them.
    #[default]
    Store,
    /// Drop the attachment's contents after the pass.
    Discard,
}

/// One legal use of a GPU buffer, as declared in `GpuBufferDescriptor.usage`.
///
/// The WebGPU buffer usage field is a bitmask, not an enum: a buffer may
/// be used for several purposes at once. This enum names the individual
/// bits so call sites never hand-write a magic number, and the renderer
/// ORs the selected variants into the bitmask at the call site. Combine
/// bits with `|`, e.g. `BufferUsage::Vertex | BufferUsage::CopyDestination`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BufferUsage {
    /// The buffer may be mapped for reading from the CPU.
    ///
    /// Implies the buffer is host-visible, so it cannot be combined with
    /// most other uses on conformant implementations.
    MapRead,
    /// The buffer may be mapped for writing from the CPU.
    MapWrite,
    /// The buffer may be the source of a `copyBufferToBuffer` / `copyTextureToBuffer` call.
    CopySource,
    /// The buffer may be the destination of a `copyBufferToBuffer` call or
    /// of `queue.writeBuffer`.
    CopyDestination,
    /// The buffer holds an index list for `setIndexBuffer` and
    /// `drawIndexed`.
    Index,
    /// The buffer holds per-vertex or per-instance attributes for `setVertexBuffer`.
    Vertex,
    /// The buffer is bound as a `var<uniform>` in WGSL.
    Uniform,
    /// The buffer is bound as a `var<storage>` in WGSL.
    Storage,
    /// The buffer holds the arguments of an indirect draw or dispatch.
    Indirect,
    /// The buffer is the destination of `commandEncoder.resolveQuerySet`.
    QueryResolve,
}

/// One legal use of a GPU texture, as declared in `GpuTextureDescriptor.usage`.
///
/// Like [`BufferUsage`], the underlying WebGPU field is a bitmask and the
/// renderer ORs the selected variants together at the call site. A render
/// target needs `RenderAttachment`; a sampled texture needs
/// `TextureBinding`; a write-once compute output needs `StorageBinding`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextureUsage {
    /// The texture may be the source of a texture-to-texture or
    /// texture-to-buffer copy.
    CopySource,
    /// The texture may be the destination of `queue.writeTexture` or of a
    /// texture-to-texture copy.
    CopyDestination,
    /// The texture is sampled through a `texture_2d<f32>` binding.
    TextureBinding,
    /// The texture is written through a `texture_storage_2d<...>` binding.
    StorageBinding,
    /// The texture is bound as a color, depth, or stencil attachment of a
    /// render pass.
    RenderAttachment,
}

/// How texels are interpolated when a sample is fetched.
///
/// Used for both the magnification filter (`magFilter`) and the
/// minification filter (`minFilter`) of a sampler. The two filters are
/// separate WebGPU fields because a magnified surface and a minified
/// surface of the same texture want different behaviour — pixel art wants
/// `Nearest` when magnified and `Linear` when minified — so a single
/// [`SamplerDescriptor::filter`] covers both by letting the caller pick
/// the same mode or override it per stage.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FilterMode {
    /// Sample the nearest texel. Keeps hard edges and is the correct
    /// choice for pixel art and for integer data.
    Nearest,
    /// Blend the neighbouring texels. Smooths magnification and
    /// minification alike.
    #[default]
    Linear,
}

/// How texels are blended between adjacent mip levels of a mipmapped
/// texture.
///
/// This is the `mipmapFilter` field of a sampler, which is a separate
/// decision from [`FilterMode`]: `FilterMode` decides how one mip level is
/// sampled, this decides how the results of two levels are combined.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MipmapFilter {
    /// Pick one mip level. Avoids the extra texture fetches that
    /// trilinear filtering costs, at the price of visible mip transitions.
    Nearest,
    /// Blend the two nearest mip levels. Smoother across the mip
    /// transition, one extra texture fetch per sample.
    #[default]
    Linear,
}

/// What a sampler does with coordinates that fall outside `[0, 1]`.
///
/// One value per texture axis (`addressModeU`, `addressModeV`,
/// `addressModeW` on the wire). The W axis is only meaningful for 3D
/// textures; for 2D textures it is accepted and ignored.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AddressMode {
    /// Clamp to the edge texel, stretching it outwards. The right default
    /// for anything that is not meant to tile.
    #[default]
    ClampToEdge,
    /// Mirror the texture at each edge, so `0..1` then `1..0` then `0..1`.
    MirrorRepeat,
    /// Tile the texture, so `0..1` then `0..1` again.
    Repeat,
}

/// The comparison performed between two values, used both for depth tests
/// and for comparison samplers.
///
/// Replaces the bare `&'static str` that
/// `GpuSamplerDescriptor.compare` could not express: a `bool` there forced
/// the renderer to hardcode a single comparison, while
/// [`SamplerDescriptor::compare`] carries the comparison itself as
/// `Option<CompareFunction>` — `None` for a plain filtering sampler.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CompareFunction {
    /// Never passes: the test always fails.
    Never,
    /// Passes when the incoming value is strictly less than the reference.
    #[default]
    Less,
    /// Passes when the two values are equal.
    Equal,
    /// Passes when the incoming value is less than or equal to the reference.
    LessEqual,
    /// Passes when the incoming value is strictly greater than the reference.
    Greater,
    /// Passes when the two values differ.
    NotEqual,
    /// Passes when the incoming value is greater than or equal to the reference.
    GreaterEqual,
    /// Always passes: the test never fails.
    ///
    /// Useful for an always-visible overlay drawn after the opaque pass,
    /// where the depth buffer must be bound but must not reject anything.
    Always,
}

/// One term of a blend equation: the factor the source or destination value
/// is multiplied by.
///
/// A blend state pairs one of these as the source factor and one as the
/// destination factor, then combines the two results with a
/// [`BlendOperation`]. The conventional alpha-blend pairing is
/// `SourceAlpha` as the source and `OneMinusSourceAlpha` as the
/// destination, with `BlendOperation::Add`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BlendFactor {
    /// `0` — the term contributes nothing.
    Zero,
    /// `1` — the term passes through unchanged.
    One,
    /// The source color, for a source factor; the destination color, for a
    /// destination factor.
    SourceColor,
    /// `1` minus the source color, for a source factor.
    OneMinusSourceColor,
    /// The destination color, for a source factor; the source color, for a
    /// destination factor.
    DestinationColor,
    /// `1` minus the destination color, for a source factor.
    OneMinusDestinationColor,
    /// The source alpha.
    #[default]
    SourceAlpha,
    /// `1` minus the source alpha. The conventional destination factor for
    /// alpha blending.
    OneMinusSourceAlpha,
    /// The destination alpha, for a destination factor.
    DestinationAlpha,
    /// `1` minus the destination alpha, for a source factor.
    OneMinusDestinationAlpha,
    /// The blend constant set via `setBlendConstant`.
    ConstantColor,
    /// `1` minus the blend constant.
    OneMinusConstantColor,
    /// The alpha of the blend constant.
    ConstantAlpha,
    /// `1` minus the alpha of the blend constant.
    OneMinusConstantAlpha,
    /// The smaller of the source alpha and `1 - destination alpha`, which is
    /// what keeps repeated alpha-blended passes from accumulating
    /// saturation.
    SourceAlphaSaturated,
}

/// How the two blended terms are combined into the final value.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BlendOperation {
    /// `source + destination`. The default, and what alpha blending needs.
    #[default]
    Add,
    /// `source - destination`.
    Subtract,
    /// `destination - source`.
    ReverseSubtract,
    /// `min(source, destination)`.
    Min,
    /// `max(source, destination)`.
    Max,
}

/// How the vertex stage assembles the stream of vertices into primitives.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum PrimitiveTopology {
    /// One independent point per vertex.
    PointList,
    /// One independent line segment per pair of vertices.
    LineList,
    /// One connected polyline through every vertex.
    LineStrip,
    /// One independent triangle per three vertices. The default, and the
    /// only topology an indexed mesh needs.
    #[default]
    TriangleList,
    /// One connected triangle strip through every vertex.
    TriangleStrip,
}

/// The element width of the index buffer handed to `setIndexBuffer`.
///
/// The format must match the element type of the index data and the
/// [`PrimitiveState::strip_index_format`] when a strip topology is used.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum IndexFormat {
    /// 16-bit unsigned indices. Halves the index buffer, but a mesh with
    /// more than 65536 vertices would silently wrap.
    Uint16,
    /// 32-bit unsigned indices.
    #[default]
    Uint32,
}

/// The shader stages a bind group layout entry is visible to.
///
/// The wire field is a bitmask, so entries are usually visible to more
/// than one stage; the renderer ORs the selected variants together at the
/// call site. This replaces the raw `visibility: u32` on
/// [`BindGroupLayoutEntry`], where `0x1` / `0x2` / `0x4` were written by
/// hand and a wrong bit silently produced a binding the shader cannot see.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ShaderStage {
    /// The vertex stage, mask value `0x1`.
    Vertex,
    /// The fragment stage, mask value `0x2`.
    Fragment,
    /// The compute stage, mask value `0x4`.
    Compute,
}

/// Which winding order counts as the front face of a triangle.
///
/// The two sides are labelled only relative to this choice, so flipping
/// it swaps what [`CullMode::Front`] and [`CullMode::Back`] discard.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FrontFace {
    /// Counter-clockwise triangles, in framebuffer coordinates, are front
    /// facing.
    #[default]
    CounterClockwise,
    /// Clockwise triangles, in framebuffer coordinates, are front facing.
    Clockwise,
}

/// Which triangle faces are discarded before rasterization.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CullMode {
    /// Keep both faces. The default, and the right choice for 2D content
    /// and for double-sided materials.
    #[default]
    None,
    /// Discard front-facing triangles.
    Front,
    /// Discard back-facing triangles. The usual choice for closed solid
    /// geometry, which roughly halves rasterization work.
    Back,
}

/// The in-memory layout of one vertex attribute.
///
/// Maps to the `GPUVertexAttribute.format` field. A position of three
/// `f32` is `Float32x3`; a packed 8-bit-per-channel color is `Unorm8x4`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum VertexAttributeFormat {
    /// One 32-bit float.
    Float32,
    /// Two 32-bit floats, laid out as an `x` / `y` pair.
    #[default]
    Float32x2,
    /// Three 32-bit floats, laid out as an `x` / `y` / `z` triple.
    Float32x3,
    /// Four 32-bit floats, laid out as `x` / `y` / `z` / `w`.
    Float32x4,
    /// Four 8-bit unsigned-normalized channels in one 32-bit word.
    Unorm8x4,
    /// One 32-bit unsigned integer.
    Uint32,
    /// One 32-bit signed integer.
    Sint32,
}

/// The texture formats the renderer supports, as a closed set.
///
/// Deliberately a subset of the WebGPU format list: every variant here is
/// one the renderer's attachment, binding, and sampling paths are known
/// to handle. Naming them as an enum turns a wrong format string into a
/// compile error rather than a validation-layer rejection at creation time.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GpuTextureFormat {
    /// Four 8-bit channels, normalized to `0.0..=1.0`. The default, and
    /// the format `gpu.getPreferredCanvasFormat()` usually resolves to
    /// when the platform is not BGRA.
    #[default]
    Rgba8Unorm,
    /// Four 8-bit channels in blue-green-red-alpha order. The swap-chain
    /// format on most desktop GPUs.
    Bgra8Unorm,
    /// Four 16-bit floating-point channels. A render target for HDR passes,
    /// which then tone-map on resolve.
    Rgba16Float,
    /// 24-bit depth, no stencil. The usual choice for a shadow map or any
    /// other single-sample depth attachment.
    Depth24Plus,
    /// 24-bit depth plus an 8-bit stencil, in one 32-bit word.
    Depth24PlusStencil8,
    /// 32-bit floating-point depth. Needed for view-space z-buffers.
    Depth32Float,
    /// One 32-bit floating-point channel. A single-channel render target
    /// and the usual format for a read-only storage binding.
    R32Float,
    /// Four 32-bit floating-point channels. The format for a compute
    /// shader's read-write storage output.
    Rgba32Float,
}

/// The class of error a WebGPU error scope collects.
///
/// Pushed onto `GpuDevice` via
/// [`WebGpuRenderer::push_error_scope`]; every error of this class raised
/// while the scope is open is captured instead of being reported to the
/// console immediately. A typo in the filter string is a silent
/// behaviour change - `pushErrorScope` accepts any string and simply
/// never matches - so the filter is named as a variant.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GpuErrorFilter {
    /// Captures spec violations: shader compile errors, bind-group
    /// mismatches, out-of-bounds draws, invalid descriptors. The filter
    /// to reach for when wrapping a single `create_*` call.
    #[default]
    Validation,
    /// Captures allocation failures. Distinct from `Validation` because a
    /// driver may reject a large allocation that is perfectly legal.
    OutOfMemory,
    /// Captures failures originating inside the browser or driver rather
    /// than in the submitted commands. Rare, and usually fatal for the
    /// device.
    Internal,
}
