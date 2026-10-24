use super::*;

/// A GPU buffer holding vertex, index, or uniform data.
///
/// Unlike the WebGPU backend, a WebGL buffer carries its own allocation
/// size, so this wrapper remembers how many bytes were last written and
/// can answer [`GlBuffer::fits`](super::GlBuffer::fits) without asking the
/// driver. That is what lets
/// [`GlBuffer::upload`](super::GlBuffer::upload) re-issue `bufferData` on
/// a resize instead of creating a new `WebGlBuffer` object and leaving
/// the old one to the garbage collector.
#[derive(Clone, Data, Debug)]
pub struct GlBuffer {
    /// The GL buffer object.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) buffer: WebGlBuffer,
    /// The byte capacity the driver last allocated for this buffer.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub capacity: u32,
    /// The GL usage hint passed at allocation: `STATIC_DRAW` for data
    /// uploaded once, `DYNAMIC_DRAW` for data rewritten every frame.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub usage: u32,
}

/// A GPU texture holding sampled image data or render-target contents.
///
/// Carries the dimensions and mip count the driver was told about so a
/// resize re-allocates instead of stretching stale texels, and records
/// whether a mip chain exists so a caller cannot accidentally select a
/// minification filter that samples across levels the texture does not
/// have, which samples as solid black and is the single most common "my
/// texture is invisible" WebGL bug.
#[derive(Clone, Data, Debug)]
pub struct GlTexture {
    /// The GL texture object.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub texture: WebGlTexture,
    /// The width in texels the texture was allocated with.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub width: u32,
    /// The height in texels the texture was allocated with.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub height: u32,
    /// The number of mip levels allocated.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub levels: u32,
    /// Whether `generateMipmap` was called, so a minification filter that
    /// samples across levels is known to be legal.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub mipmapped: bool,
}

/// A vertex array object: the recorded bundle of buffer bindings and
/// vertex attribute formats.
///
/// Binding a VAO replaces up to a dozen `bindBuffer` /
/// `enableVertexAttribArray` / `vertexAttribPointer` calls with one,
/// which is what makes per-mesh draw state cheap enough to change on
/// every draw call.
#[derive(Clone, Data, Debug)]
pub struct GlVertexArray {
    /// The GL vertex array object.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) vao: WebGlVertexArrayObject,
}

/// A framebuffer: a color attachment texture plus an optional depth (and
/// stencil) renderbuffer.
///
/// The color attachment is always a [`GlTexture`] rather than a
/// renderbuffer, so the result can be sampled by a later pass or read
/// back without a resolve blit.
#[derive(Clone, Data, Debug)]
pub struct GlFramebuffer {
    /// The GL framebuffer object.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) framebuffer: WebGlFramebuffer,
    /// The depth renderbuffer, optionally packed with an 8-bit stencil,
    /// or `None` for a color-only target such as a post-process pass
    /// that reads depth from a texture instead of writing its own.
    #[get(type(clone))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub depth: Option<WebGlRenderbuffer>,
    /// The color attachment, retained so a later pass can sample the
    /// rendered result without a separate resolve.
    #[get(type(clone))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub color: GlTexture,
    /// The width every attachment was allocated with, re-checked on
    /// rebind so a window resize re-allocates rather than leaving a
    /// frame whose attachments disagree on size.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub width: u32,
    /// The height every attachment was allocated with.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub height: u32,
}

/// A linked GLSL program together with its cached uniform locations.
///
/// Uniform locations are stable for the lifetime of a linked program, but
/// resolving one costs a round trip into the GL frontend that walks the
/// program's uniform table. Caching them in a name-keyed map means a
/// per-frame `set_uniform_1f` is a hash lookup plus the upload with no
/// re-resolution, and a name the GLSL compiler optimized out caches the
/// `None` so the miss is paid exactly once rather than every frame.
#[derive(Clone, Data, Debug)]
pub struct GlProgram {
    /// The GL program object.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) program: WebGlProgram,
    /// Uniform locations resolved so far, keyed by uniform name. A
    /// `None` entry records a uniform the compiler removed, so the
    /// negative result is not recomputed every frame either.
    ///
    /// The cache is read through a borrow and written through the
    /// mutable accessor, so looking a name up costs a hash lookup
    /// rather than a copy of the whole table. Only the single location
    /// handle a caller actually receives is cloned.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) uniforms: HashMap<String, Option<WebGlUniformLocation>>,
    /// The uniform block indices resolved so far, keyed by block name.
    ///
    /// Like the uniform cache above, this is borrowed on lookup and
    /// filled in place on a miss, so a cached block costs no copy of
    /// the table.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) blocks: HashMap<String, u32>,
}

/// A uniform buffer bound to a numbered binding point.
///
/// WebGL 2 has no bind groups: the equivalent of a `var<uniform>` block
/// is a `WebGlBuffer` bound to `UNIFORM_BUFFER` at a binding index the
/// shader agrees on. This holds both halves of that agreement, the buffer
/// and the binding point, so swapping programs cannot leave the two out
/// of sync.
#[derive(Clone, Data, Debug)]
pub struct GlUniformBlock {
    /// The GL buffer holding the block's records.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) buffer: WebGlBuffer,
    /// The binding point index the shader's `layout(binding = N)`
    /// qualifier agrees with.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub binding: u32,
    /// The byte capacity allocated for the block.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub capacity: u32,
}

/// The blend and color-write state of one draw.
///
/// Mirrors the alpha/color pair of [`BlendState`] exactly, so the WebGL
/// path can consume the same blend the WebGPU path builds, but carries
/// the extra `enabled` flag WebGL needs: WebGPU expresses "no blend" as
/// `blend: None` on the target, whereas WebGL expresses it as leaving
/// `BLEND` disabled, which is a different call with a different cost.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct GlBlendState {
    /// Whether `BLEND` is enabled for this draw.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub enabled: bool,
    /// The blend applied to the RGB channels.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub color: BlendComponent,
    /// The blend applied to the alpha channel, which is frequently not
    /// the same blend as the color channels.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub alpha: BlendComponent,
}

/// The depth-test state of one draw.
///
/// WebGL splits "test" from "write" into two independent calls
/// (`DEPTH_TEST` plus `depthFunc`, and `depthMask`), whereas WebGPU's
/// [`DepthStencilState`] derives both from `depth_write_enabled` plus the
/// presence of an attachment. Keeping the two axes separate here is what
/// lets a transparent overlay bind a depth buffer it must read but must
/// not write.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct GlDepthState {
    /// Whether `DEPTH_TEST` is enabled.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub enabled: bool,
    /// The comparison between the fragment's depth and the stored depth.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub compare: CompareFunction,
    /// Whether a fragment that passes the test updates the depth buffer.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) write_enabled: bool,
}

/// The culling and winding state of one draw.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct GlCullState {
    /// Which faces are discarded before rasterization.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub mode: CullMode,
    /// Which winding order counts as front facing. Flipping it swaps what
    /// [`CullMode::Front`] and [`CullMode::Back`] discard.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub front_face: FrontFace,
}

/// Which channels the fragment stage may write, as a raw four-bit mask.
///
/// Red is `0x1`, green `0x2`, blue `0x4`, alpha `0x8`, and all four is
/// `0xf`. WebGL's `colorMask` is the only place a per-channel write mask
/// can be expressed, and keeping it a bitmask rather than four booleans
/// matches the [`ColorTargetState`] write-mask convention the WebGPU path
/// already uses.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct GlColorMask {
    /// The four channel bits.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub bits: u32,
}

/// A scissor rectangle.
///
/// A zero-sized box is legal and clips everything away, which is
/// meaningfully different from scissoring being off; the off case is
/// therefore expressed by the `Option` on [`GlRenderState::scissor`]
/// rather than by a sentinel rectangle.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct GlScissor {
    /// The left edge in framebuffer pixels.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub x: i32,
    /// The bottom edge in framebuffer pixels.
    #[get(type(copy))]
    pub y: i32,
    /// The width in framebuffer pixels.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub width: i32,
    /// The height in framebuffer pixels.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub height: i32,
}

/// The viewport rectangle, in framebuffer pixels.
///
/// Stored as signed values because GL accepts a negative origin for
/// flipped-coordinate passes even though the default is the origin.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct GlViewport {
    /// The left edge in framebuffer pixels.
    #[get(type(copy))]
    pub x: i32,
    /// The bottom edge in framebuffer pixels.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub y: i32,
    /// The width in framebuffer pixels.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub width: i32,
    /// The height in framebuffer pixels.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub height: i32,
}

/// The whole fixed-function state of one draw, as a single `Copy` value.
///
/// The point of bundling these six fields into one struct is that it can
/// be diffed field by field against the state currently bound, so
/// [`WebGl2Backend::apply_state`](super::WebGl2Backend::apply_state)
/// issues a GL call only for what actually changed. A renderer that
/// calls `enable`, `depthFunc`, `depthMask`, `blendFunc`,
/// `blendEquation`, `cullFace`, `frontFace`, `colorMask`, `scissor`, and
/// `viewport` unconditionally pays for a dozen redundant calls per draw,
/// each one a validation check inside the driver.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct GlRenderState {
    /// The depth-test state.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub depth: GlDepthState,
    /// The blend state.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub blend: GlBlendState,
    /// The culling and winding state.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub cull: GlCullState,
    /// Which channels the fragment stage may write.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub color_mask: GlColorMask,
    /// The scissor rectangle, or `None` when scissoring is off.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub scissor: Option<GlScissor>,
    /// The viewport rectangle.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub viewport: GlViewport,
}

/// The complete WebGL 2 backend: a context plus the three shadows that
/// make a frame cheap.
///
/// # Performance
///
/// Three things are shadowed rather than re-queried. The fixed-function
/// state is shadowed by a [`GlRenderState`] copy, so applying a state
/// costs only the calls for fields that actually changed. The current
/// texture unit is shadowed by a `u32`, so binding a texture to the unit
/// that is already active costs no `activeTexture` at all. The current
/// program is shadowed by a `JsValue` compared with `Object.is`, so
/// rebinding the same program costs no `useProgram`. None of the three
/// can be answered by asking the driver without a synchronous
/// round trip, which is exactly why they are tracked here instead.
#[derive(Clone, Data, Debug)]
pub struct WebGl2Backend {
    /// The canvas element this backend draws into.
    ///
    /// Stored at construction instead of re-derived from
    /// [`WebGl2RenderingContext::canvas`], so
    /// [`WebGl2Backend::get_canvas`] is a plain field read and cannot
    /// fail (the old `context.canvas().expect(..)` could panic if the
    /// context was ever detached from its element).
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub canvas: HtmlCanvasElement,
    /// The WebGL 2 rendering context every call goes through.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub context: WebGl2RenderingContext,
    /// The last state applied, diffed against to skip redundant calls.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) shadow: GlRenderState,
    /// The texture unit that is currently active, or
    /// [`GL_TEXTURE_UNIT_NONE`] when the active unit is not yet known.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) active_unit: u32,
    /// The program currently bound, compared with `Object.is`.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) bound_program: JsValue,
    /// The color [`WebGl2Backend::begin_frame`] clears to.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) clear_color_field: Color,
    /// The reused destination for
    /// [`WebGl2Backend::read_pixels`](super::WebGl2Backend::read_pixels),
    /// grown on demand so a steady-state readback allocates nothing.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) readback: Vec<u8>,
}

/// Constructor inputs for [`WebGl2Backend`].
///
/// `WebGl2Backend::init` is the only constructor, and it needs a live
/// document: it queries the canvas by selector and acquires a `webgl2`
/// context. Every field on the backend is `pub(crate)`, so without this
/// struct there is no way to build one from outside the crate and the
/// state-diffing, draw-call and resize surface is untestable. Naming the
/// inputs lets a caller that already holds a context construct a backend
/// without going through DOM lookup.
#[derive(Clone, Data, New)]
pub struct WebGl2BackendInit {
    /// The canvas the backend draws into.
    pub canvas: HtmlCanvasElement,
    /// The `webgl2` context every call goes through.
    pub context: WebGl2RenderingContext,
    /// The physical pixel width of the drawing buffer.
    #[get(type(copy))]
    pub width: u32,
    /// The physical pixel height of the drawing buffer.
    #[get(type(copy))]
    pub height: u32,
    /// The colour `begin_frame` clears to.
    pub clear_color: Color,
}

/// Constructor inputs for [`GlRenderState`].
///
/// `GlRenderState::context_defaults` is the only way to build one today, and
/// it hard-codes the state a *freshly created* GL context happens to be in.
/// Every other state — a depth-tested draw, a scissored viewport, a masked
/// render target — has to be assembled field by field to reach
/// `WebGl2Backend::apply_state`, which means it cannot be built from outside
/// the crate. Naming the inputs closes that gap.
#[derive(Clone, Copy, Data, New)]
pub struct GlRenderStateInit {
    /// The depth-test state.
    pub depth: GlDepthState,
    /// The blend state.
    pub blend: GlBlendState,
    /// The culling and winding state.
    pub cull: GlCullState,
    /// Which channels the fragment stage may write.
    pub color_mask: GlColorMask,
    /// The scissor rectangle, or `None` when scissoring is off.
    #[get(type(copy))]
    pub scissor: Option<GlScissor>,
    /// The viewport rectangle.
    pub viewport: GlViewport,
}
