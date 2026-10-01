use super::*;

/// Describes a 2D viewport rectangle plus optional depth range, in the same
/// pixel space as the destination render target.
///
/// Used by [`WebGpuRenderer::set_viewport`] (and any future caller that needs
/// to push a `GpuViewport`-shaped JS object through `Reflect::set`). The
/// depth-range fields are omitted from the `::new` constructor via
/// `#[new(skip)]`; they default to zero-initialised `f32` and are typically
/// overwritten by [`WebGpuRenderer::set_viewport`] to the WebGPU spec
/// defaults of `0.0` / `1.0`.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct ViewportDescriptor {
    /// X coordinate of the viewport's top-left in pixels.
    pub(crate) x: f32,
    /// Y coordinate of the viewport's top-left in pixels.
    pub(crate) y: f32,
    /// Viewport width in pixels.
    pub(crate) width: f32,
    /// Viewport height in pixels.
    pub(crate) height: f32,
    /// Minimum depth, clamped to `[0, 1]`. Set to `0.0` to disable.
    #[new(skip)]
    pub(crate) min_depth: f32,
    /// Maximum depth, clamped to `[0, 1]`. Set to `1.0` to disable.
    #[new(skip)]
    pub(crate) max_depth: f32,
}

/// A single vertex attribute within a vertex buffer layout.
///
/// Mirrors the fields of `GPUVertexAttribute` exactly. The shader location
/// is the `@location(N)` qualifier in the WGSL source. The offset is in
/// bytes from the start of the vertex, and `format` is one of the
/// WGSL vertex format strings (e.g. `"float32x4"`, `"unorm8x4"`).
#[derive(Clone, Copy, Debug, Eq, Getter, Hash, New, PartialEq)]
pub struct VertexAttribute {
    /// The shader location the attribute maps to.
    #[get(type(copy))]
    pub(crate) shader_location: u32,
    /// The byte offset from the start of the vertex.
    #[get(type(copy))]
    pub(crate) offset: u64,
    /// The in-memory layout of the attribute, as one of the
    /// [`VertexAttributeFormat`] variants. Naming it turns a wrong
    /// format string into a compile error rather than a pipeline
    /// validation failure at creation time.
    #[get(type(copy))]
    pub(crate) format: VertexAttributeFormat,
}

/// The layout of a single vertex buffer, expressed as an array stride plus
/// a list of attributes.
///
/// Mirrors `GPUVertexBufferLayout` from the WebGPU spec. The renderer
/// passes the assembled descriptor straight to `createRenderPipeline` via
/// `Reflect`.
#[derive(Clone, Debug, Getter, New)]
pub struct VertexBufferLayout {
    /// The byte stride of one vertex in the buffer.
    #[get(type(copy))]
    pub(crate) array_stride: u64,
    /// Whether the buffer should be advanced per-instance (`true`) or
    /// per-vertex (`false`).
    #[get(type(copy))]
    pub(crate) step_mode: VertexStepMode,
    /// The attributes that describe how to interpret the bytes of one
    /// vertex.
    pub(crate) attributes: Vec<VertexAttribute>,
}

/// A 2D texture descriptor for `create_texture_2d`.
///
/// Defaults produce a 1x1 RGBA8 texture with `TEXTURE_BINDING | COPY_DST
/// | COPY_SRC` usage, which is the right baseline for a sampled color
/// texture that is uploaded to via `queue.writeTexture`. Override fields
/// after constructing to set `mip_level_count`, `sample_count`, or
/// different `usage` flags.
#[derive(Clone, Debug, Getter, New)]
pub struct Texture2DDescriptor {
    /// The texture width in pixels. Must be > 0.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The texture height in pixels. Must be > 0.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// The texel format, as one of the [`GpuTextureFormat`] variants.
    #[get(type(copy))]
    pub(crate) format: GpuTextureFormat,
    /// The number of mip levels. `0` is treated as `1`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) mip_level_count: u32,
    /// The number of samples per texel (`1` for non-MSAA, `4` for MSAA).
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) sample_count: u32,
}

/// Descriptor for `GpuTexture.createView(descriptor)`.
///
/// Sub-selects a single cube face / mip / array slice / depth-aspect of a
/// texture. When you need the full texture as a 2D view (the common case),
/// just call `create_view` without a descriptor; the new method accepts an
/// `Option<&TextureViewDescriptor>` for callers that need the full
/// flexibility of the WebGPU spec.
#[derive(Clone, Debug, Getter, New)]
pub struct TextureViewDescriptor {
    /// View format override, or `None` to use the texture's own format.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) format: Option<&'static str>,
    /// View dimension (`"2d"`, `"2d-array"`, `"cube"`, `"cube-array"`, ...).
    /// `None` means the dimension is inferred from the texture.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) dimension: Option<&'static str>,
    /// Most significant mip level (inclusive). `None` → `0`.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) base_mip_level: u32,
    /// Number of mip levels in the view. `0` → all the way to the top.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) mip_level_count: u32,
    /// First array layer (inclusive). `None` → `0`. Only meaningful for
    /// `2d-array` / `cube` / `cube-array` views.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) base_array_layer: u32,
    /// Number of array layers. `0` → all remaining layers.
    #[get(type(copy))]
    #[new(value = "0")]
    pub(crate) array_layer_count: u32,
    /// Which aspect of the texture to expose. One of:
    /// `"all"`, `"depth-only"`, `"stencil-only"`. `None` → `"all"`.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) aspect: Option<&'static str>,
}

/// Descriptor for `queue.writeTexture(destination, data, dataLayout, size)`.
///
/// WebGPU's `writeTexture` lets you upload CPU-side pixel data directly to a
/// texture without staging through a buffer. Use it for: ImGui font atlases,
/// procedural noise textures, sprite sheets, `ImageBitmap` pixels, etc.
#[derive(Clone, Debug, Getter, New)]
pub struct TextureWriteDescriptor {
    /// The pixel data to upload. Bytes are laid out according to
    /// `bytes_per_row` / `rows_per_image`.
    #[get(type(clone))]
    pub(crate) data: Vec<u8>,
    /// Bytes per row of the source data. Must be a multiple of 256.
    #[get(type(copy))]
    pub(crate) bytes_per_row: u32,
    /// Number of rows per image. `0` for 2D textures without mip chains.
    #[get(type(copy))]
    pub(crate) rows_per_image: u32,
    /// Destination mip level to write into.
    #[get(type(copy))]
    pub(crate) mip_level: u32,
    /// Destination texture to write into.
    #[get(type(clone))]
    pub(crate) texture: JsValue,
    /// Origin within the destination texture. `None` → `(0, 0, 0)`.
    #[get(type(clone))]
    #[new(value = "None")]
    pub(crate) origin: Option<JsValue>,
    /// Whether to flip the source data vertically before writing.
    /// `true` is essential when uploading from `<img>` / `<canvas>` whose
    /// rows are top-to-bottom but WebGPU textures are bottom-to-top.
    #[get(type(copy))]
    #[new(value = "false")]
    pub(crate) flip_y: bool,
}

/// A single entry inside a `GpuBindGroupLayoutDescriptor`.
///
/// Together these describe one slot of the bind group layout used by
/// a render / compute pipeline. The `visibility` field names the
/// shader stages that can read the binding as [`ShaderStage`] values
/// combined with `|`; a hand-written `0x1` / `0x2` / `0x4` there
/// silently produced a binding the shader could not see.
#[derive(Clone, Debug)]
pub struct BindGroupLayoutEntry {
    /// The binding slot (matches `@binding(N)` in the shader).
    pub binding: u32,
    /// The shader stages that can read this binding. Combine several
    /// with `|`, e.g. `ShaderStage::Vertex | ShaderStage::Fragment`.
    pub visibility: ShaderStage,
    /// The resource kind bound at this slot.
    pub ty: BindGroupEntryType,
}

/// A strongly-typed sampler descriptor for `create_sampler`.
///
/// Replaces the historical `GpuSamplerDescriptor`, which spelled the
/// filter and address modes as six independent `&'static str` fields plus
/// a `compare: bool`. The new layout keeps the same information but as
/// three orthogonal decisions — how to interpolate, how to blend mip
/// levels, how to wrap — so the combinations the old fields allowed but
/// the renderer could not express (a `Nearest` magnification filter with a
/// `Linear` minification filter, a comparison other than the hardcoded
/// one) are now expressible, and an invalid value is a compile error
/// rather than a silently rejected `createSampler` call.
#[derive(Clone, Copy, Data, Debug, New)]
pub struct SamplerDescriptor {
    /// How texels are interpolated within one mip level. Applied to both
    /// the magnification and the minification filter.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) filter: FilterMode,
    /// How results from adjacent mip levels are blended together.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) mipmap_filter: MipmapFilter,
    /// What sampling does with U coordinates outside `[0, 1]`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) address_mode_u: AddressMode,
    /// What sampling does with V coordinates outside `[0, 1]`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) address_mode_v: AddressMode,
    /// What sampling does with W coordinates outside `[0, 1]`. Only
    /// meaningful for 3D textures.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) address_mode_w: AddressMode,
    /// The depth comparison to apply, or `None` for a plain filtering
    /// sampler that performs no comparison. Strictly more expressive than
    /// the old `compare: bool`, which could only mean "compare with the
    /// single hardcoded comparison".
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) compare: Option<CompareFunction>,
}

/// One color attachment of a render pass.
///
/// Replaces the historical `RenderPassColorAttachment`. The two `Option<&'static str>`
/// load and store ops disappear: a [`LoadOp`] and a [`StoreOp`] say
/// directly what the old optional strings inferred, and a clear color is
/// the engine's own [`Color`] rather than a bare `(f64, f64, f64, f64)`
/// tuple whose channel order had to be re-checked at every call site.
#[derive(Clone, Debug, Getter)]
pub struct ColorAttachment {
    /// The texture view to draw into, or `None` to let the renderer use
    /// the swap-chain view (or the MSAA intermediate view when
    /// antialiasing is on).
    pub view: Option<JsValue>,
    /// The destination view that a multisampled attachment resolves into,
    /// or `None` when multisampling is disabled. The renderer substitutes
    /// the swap-chain view when this is `None` but the attachment is
    /// multisampled.
    pub resolve_target: Option<JsValue>,
    /// The color to overwrite the attachment with when `load_op` is
    /// [`LoadOp::Clear`]. `None` means no clear, so `load_op` must then be
    /// [`LoadOp::Load`].
    pub clear: Option<Color>,
    /// What to do with the attachment's existing contents before drawing.
    pub load_op: LoadOp,
    /// What to do with the attachment's contents once the pass ends.
    pub store_op: StoreOp,
}

/// The depth-stencil attachment of a render pass.
///
/// Replaces the historical `RenderPassDepthStencilAttachment`. As with
/// [`ColorAttachment`], the optional load and store op strings are
/// replaced by typed ops, and the clear depth is `None`-able so that
/// "clear to 1.0" and "keep what is there" are distinguishable at the
/// type level rather than by which fields happen to be set.
#[derive(Clone, Debug, Getter)]
pub struct DepthStencilAttachment {
    /// The depth-stencil texture view to use, or `None` to let the
    /// renderer use the default view into its own depth texture,
    /// allocating that texture lazily if needed.
    pub view: Option<JsValue>,
    /// The depth to overwrite the attachment with when `depth_load_op` is
    /// [`LoadOp::Clear`], in `0.0..=1.0`. `None` means no clear, so
    /// `depth_load_op` must then be [`LoadOp::Load`].
    pub depth_clear: Option<f64>,
    /// What to do with the existing depth before drawing.
    pub depth_load_op: LoadOp,
    /// What to do with the depth once the pass ends.
    pub depth_store_op: StoreOp,
    /// Whether the pass reads depth without writing it. `true` lets the
    /// driver skip the depth write entirely.
    pub depth_read_only: bool,
}

/// One channel group of a [`BlendState`]: an operation plus the factors
/// applied to the incoming and the existing value.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct BlendComponent {
    /// How the two scaled terms are combined.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) operation: BlendOperation,
    /// The factor the incoming value is multiplied by.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) source: BlendFactor,
    /// The factor the existing value is multiplied by.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) destination: BlendFactor,
}

/// The full blend state of a single color target.
///
/// Alpha blending is `BlendComponent { operation: Add, source:
/// SourceAlpha, destination: OneMinusSourceAlpha }` on both the color and
/// the alpha channel; replacing the source and keeping the destination is
/// how a fade-to-black and other multiply blends are expressed.
#[derive(Clone, Copy, Data, Debug, Default, New)]
pub struct BlendState {
    /// The blend applied to the RGB channels.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) color: BlendComponent,
    /// The blend applied to the alpha channel, which is frequently not
    /// the same as the color blend.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) alpha: BlendComponent,
}

/// One entry of a fragment stage's `targets` array: the format of that
/// color attachment and how fragments write into it.
#[derive(Clone, Copy, Data, Debug, Default, New)]
pub struct ColorTargetState {
    /// The format of the color attachment this target writes to. It must
    /// match the attachment's own format.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) format: GpuTextureFormat,
    /// The blend to apply, or `None` to overwrite the target with the
    /// fragment's color.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) blend: Option<BlendState>,
    /// Which channels the fragment stage may write, as a bitmask:
    /// `0x1` red, `0x2` green, `0x4` blue, `0x8` alpha. All four channels
    /// is `0xf`; `0xf` is the right value for almost every opaque
    /// fragment.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) write_mask: u32,
}

/// The pipeline's depth-stencil state: whether depth is written, and the
/// test fragments must pass.
#[derive(Clone, Copy, Data, Debug, Default, New)]
pub struct DepthStencilState {
    /// The format of the pipeline's depth-stencil attachment. It must
    /// match the attachment's own format.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) format: GpuTextureFormat,
    /// Whether a fragment that passes the test updates the depth buffer.
    /// `false` is correct for a pipeline that only reads depth, such as a
    /// transparent overlay drawn after the opaque pass.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) depth_write_enabled: bool,
    /// The comparison performed between the fragment's depth and the
    /// stored depth.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) depth_compare: CompareFunction,
}

/// The pipeline's multisample state: how many samples each pixel holds and
/// which of them are written.
#[derive(Clone, Copy, Data, Debug, Default, New)]
pub struct MultisampleState {
    /// The samples per pixel: `1` disables multisampling, `4` enables 4x
    /// MSAA. The value must match the sample count of every color
    /// attachment the pipeline draws into.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) count: u32,
    /// Which samples are written, as a bitmask. Leave at `0xf` to write
    /// all four samples of a 4x pipeline.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) mask: u32,
}

/// The pipeline's primitive-assembly state: how vertices become
/// triangles, and which of those triangles survive rasterization.
#[derive(Clone, Copy, Data, Debug, Default, New)]
pub struct PrimitiveState {
    /// How vertices are assembled into primitives.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) topology: PrimitiveTopology,
    /// Which winding order counts as front facing.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) front_face: FrontFace,
    /// Which faces are discarded before rasterization.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) cull_mode: CullMode,
    /// The index format for a strip topology, or `None` for a
    /// non-stripped topology where the field does not apply.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) strip_index_format: Option<IndexFormat>,
    /// Whether fragments outside the depth range of `0.0..=1.0` survive.
    /// `false`, the default, discards them.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) unclipped_depth: bool,
}

/// The pipeline's vertex stage: which shader runs, and how the vertex
/// buffers feed it.
#[derive(Clone, Debug, Getter, New)]
pub struct VertexState {
    /// The `GpuShaderModule` compiled from WGSL source.
    #[get(type(clone))]
    pub(crate) module: JsValue,
    /// The `@vertex` entry point's name inside that module.
    #[get(type(clone))]
    pub(crate) entry_point: String,
    /// The layouts of the vertex buffers this stage reads. The i-th entry
    /// matches the i-th `setVertexBuffer(i, ...)` call.
    pub(crate) buffers: Vec<VertexBufferLayout>,
}

/// The pipeline's fragment stage: which shader runs, and how it writes to
/// the color attachments.
#[derive(Clone, Debug, Getter, New)]
pub struct FragmentState {
    /// The `GpuShaderModule` compiled from WGSL source.
    #[get(type(clone))]
    pub(crate) module: JsValue,
    /// The `@fragment` entry point's name inside that module.
    #[get(type(clone))]
    pub(crate) entry_point: String,
    /// One entry per color attachment, in attachment order.
    pub(crate) targets: Vec<ColorTargetState>,
}

/// The single full-control render pipeline constructor.
///
/// This is the one type a caller needs for any render pipeline: the
/// renderer assembles it into a `GpuRenderPipelineDescriptor` and hands
/// that to `device.createRenderPipeline`. Every narrower renderer method
/// is sugar over this struct — `create_render_pipeline` is a preset with
/// auto layout, triangle-list topology, no culling, and a single
/// `Rgba8Unorm` target, and `create_render_pipeline_full` is the same
/// preset with vertex buffers and a depth-stencil state added. Reach for
/// this type when either of those presets is too narrow, and for nothing
/// else.
#[derive(Clone, Debug, Getter, New)]
pub struct RenderPipelineDescriptor {
    /// The vertex stage, and the only stage a pipeline cannot do without.
    pub(crate) vertex: VertexState,
    /// How vertices are assembled and which faces are discarded.
    pub(crate) primitive: PrimitiveState,
    /// The depth-stencil state, or `None` for a pipeline that neither
    /// reads nor writes depth.
    pub(crate) depth_stencil: Option<DepthStencilState>,
    /// The multisample state, which must match the attachments.
    pub(crate) multisample: MultisampleState,
    /// The fragment stage, or `None` for a pipeline that writes no color
    /// — a depth-only prepass, for instance.
    pub(crate) fragment: Option<FragmentState>,
}

/// A compute pipeline: one shader module and one entry point.
///
/// The compute counterpart of [`RenderPipelineDescriptor`], and much
/// smaller because a compute pipeline has no vertex buffers, no
/// attachments, and no rasterization state.
#[derive(Clone, Debug, Getter, New)]
pub struct ComputePipelineDescriptor {
    /// The `GpuShaderModule` compiled from WGSL source.
    #[get(type(clone))]
    pub(crate) module: JsValue,
    /// The `@compute` entry point's name inside that module.
    #[get(type(clone))]
    pub(crate) entry_point: String,
}

/// The general texture constructor, covering 2D, 2D-array, and 3D
/// textures.
///
/// [`Texture2DDescriptor`] is the narrow preset: a single layer, a
/// format string, and a usage string, with no label. The two fields
/// [`TextureDescriptor`] adds on top of it are what keep it general —
/// `dimension` selects between a 2D texture, a 2D array, and a 3D
/// texture, and `label` names the resource in the browser's GPU debug
/// tooling, which is the only way to tell two same-sized textures apart
/// in a capture.
#[derive(Clone, Debug, Getter, New)]
pub struct TextureDescriptor {
    /// The width in texels. Must be greater than zero.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The height in texels. Must be greater than zero.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// The number of array layers for a `2d-array` texture, or the depth
    /// in texels for a `3d` texture. `1` for a plain 2D texture.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) depth_or_layers: u32,
    /// The texture's dimensionality. Defaults to `"2d"`; a
    /// `"2d-array"` or `"3d"` texture needs this set explicitly.
    #[get(type(clone))]
    pub(crate) dimension: &'static str,
    /// The texel format.
    #[get(type(copy))]
    pub(crate) format: GpuTextureFormat,
    /// The legal uses of this texture, as a `usage` bitmask. Combine
    /// several [`TextureUsage`] values with `|`.
    #[get(type(copy))]
    pub(crate) usage: u32,
    /// The number of mip levels. Zero is treated as one.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) mip_level_count: u32,
    /// The samples per texel: `1` for an ordinary texture, `4` for a
    /// multisampled render target.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) sample_count: u32,
    /// A debug label shown for this texture in the browser's GPU
    /// tooling, or `None` to leave it unnamed.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) label: Option<String>,
}

/// A buffer constructor, naming both the size and the legal uses of a
/// GPU buffer.
///
/// Replaces the raw `usage: u32` bitmask that `create_buffer` takes
/// today, where a caller had to know the bit values and pass `64 | 8` by
/// hand for a uniform-or-vertex buffer. Combine several uses with `|`.
#[derive(Clone, Debug, Getter, New)]
pub struct BufferDescriptor {
    /// The size in bytes. Must be greater than zero.
    #[get(type(copy))]
    pub(crate) size: u64,
    /// The legal uses of this buffer, as a `usage` bitmask. Combine
    /// several [`BufferUsage`] values with `|`.
    #[get(type(copy))]
    pub(crate) usage: u32,
    /// A debug label shown for this buffer in the browser's GPU tooling,
    /// or `None` to leave it unnamed.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) label: Option<String>,
}

/// One entry in the dynamic-offset list passed to
/// `set_bind_group_with_dynamic_offsets`.
///
/// A uniform buffer that a shader declares with `hasDynamicOffset: true`
/// can hold many independent records back to back, and the bind group
/// picks one per draw by adding an offset. Grouping the pair into a struct
/// keeps the two values together at the call site, where writing them as
/// two separate positional arguments is easy to transpose.
#[derive(Clone, Copy, Data, Debug, Default, New)]
pub struct UniformSlice {
    /// The byte offset of this record from the start of the bound range.
    /// The spec requires this to be a multiple of the binding's minimum
    /// uniform buffer offset alignment, typically 256.
    #[get(type(copy))]
    pub(crate) offset: u64,
    /// The size in bytes of this record. A size of zero means "from this
    /// offset to the end of the bound range".
    #[get(type(copy))]
    pub(crate) size: u64,
}

/// The five arguments of an indexed draw, as one value.
///
/// `drawIndexed` and `drawIndexedOffset` take these as five positional
/// arguments today, with no named parameter in between to keep them in
/// order and no way to leave a trailing one at its default. Grouping them
/// makes the call self-documenting and lets a caller vary one of the five
/// without restating the other four.
#[derive(Clone, Copy, Data, Debug, New)]
pub struct DrawIndexedArgs {
    /// The number of indices to read from the bound index buffer.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) index_count: u32,
    /// How many instances of the indexed geometry to draw. `1` draws a
    /// single instance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) instance_count: u32,
    /// The index of the first index to read, for drawing a sub-range of
    /// the index buffer.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) first_index: u32,
    /// The value added to every index before it is used to fetch a vertex.
    /// Lets several meshes share one vertex buffer.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) base_vertex: i32,
    /// The first instance index, for reading per-instance attributes from
    /// an offset.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) first_instance: u32,
}

/// The four arguments of a non-indexed draw, as one value.
///
/// The non-indexed counterpart of [`DrawIndexedArgs`], and the same
/// argument in the same order as `draw(vertexCount, instanceCount,
/// firstVertex, firstInstance)`.
#[derive(Clone, Copy, Data, Debug, New)]
pub struct DrawArgs {
    /// The number of vertices to draw, taken from the bound vertex buffers
    /// as a non-indexed stream.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) vertex_count: u32,
    /// How many instances of the geometry to draw. `1` draws a single
    /// instance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) instance_count: u32,
    /// The vertex to start from, for drawing a sub-range of the vertex
    /// stream.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) first_vertex: u32,
    /// The first instance index, for reading per-instance attributes from
    /// an offset.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) first_instance: u32,
}

/// The three workgroup counts of a compute dispatch, as one value.
///
/// `dispatch(x, y, z)` today takes these as three positional numbers
/// whose order is easy to transpose, since `x` is the fastest-varying
/// axis and the natural reading order of a 3D problem is `z, y, x`.
#[derive(Clone, Copy, Data, Debug, New)]
pub struct DispatchArgs {
    /// The workgroups dispatched along the x axis.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) x: u32,
    /// The workgroups dispatched along the y axis.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) y: u32,
    /// The workgroups dispatched along the z axis.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) z: u32,
}
