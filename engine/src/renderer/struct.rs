use super::*;

/// A 2D camera that defines the viewport into the game world.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera2D {
    /// The world-space position of the camera center.
    #[get(type(copy))]
    pub(crate) position: Vector2D,
    /// The zoom factor (1.0 = no zoom, 2.0 = 2x magnification).
    #[get(type(copy))]
    pub(crate) zoom: f64,
    /// The rotation angle in radians.
    #[get(type(copy))]
    pub(crate) rotation: f64,
    /// The viewport width in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A 3D camera that defines the viewport into a 3D world using perspective
/// or orthographic projection.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera3D {
    /// The world-space position of the camera (eye).
    #[get(type(copy))]
    pub(crate) position: Vector3D,
    /// The point the camera is looking at (target).
    #[get(type(copy))]
    pub(crate) target: Vector3D,
    /// The up direction for the camera.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) up: Vector3D,
    /// The vertical field of view in radians.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) fov: f64,
    /// The near clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) near: f64,
    /// The far clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) far: f64,
    /// The viewport width in pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A wrapper around `CanvasRenderingContext2d` providing convenience
/// drawing methods and camera management for the game engine.
#[derive(Clone, Data, New)]
pub struct CanvasRenderer {
    /// The underlying canvas 2D rendering context.
    pub(crate) context: CanvasRenderingContext2d,
    /// The active camera controlling the viewport.
    #[get(type(copy))]
    pub(crate) camera: Camera2D,
    /// The active rendering quality preset.
    ///
    /// Controls `imageSmoothingEnabled`, `imageSmoothingQuality`, and
    /// `textRendering` on the underlying context. Defaults to `Medium`.
    #[get(type(copy))]
    pub(crate) quality: RenderQuality,
}

/// A linear gradient defined by two endpoints and a list of color stops.
///
/// Used to create smooth color transitions along a straight line
/// for fill or stroke operations on the canvas.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct LinearGradient {
    /// The starting point of the gradient in world space.
    #[get(type(copy))]
    pub(crate) start: Vector2D,
    /// The ending point of the gradient in world space.
    #[get(type(copy))]
    pub(crate) end: Vector2D,
    /// The ordered list of color stops, each containing a position (0.0 to 1.0) and a CSS color string.
    pub(crate) stops: Vec<(f64, String)>,
}

/// A radial gradient defined by inner and outer circles and a list of color stops.
///
/// Used to create smooth color transitions radiating outward from a center point
/// for fill or stroke operations on the canvas.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct RadialGradient {
    /// The center of the inner circle of the gradient.
    #[get(type(copy))]
    pub(crate) inner_center: Vector2D,
    /// The radius of the inner circle.
    #[get(type(copy))]
    pub(crate) inner_radius: f64,
    /// The center of the outer circle of the gradient.
    #[get(type(copy))]
    pub(crate) outer_center: Vector2D,
    /// The radius of the outer circle.
    #[get(type(copy))]
    pub(crate) outer_radius: f64,
    /// The ordered list of color stops, each containing a position (0.0 to 1.0) and a CSS color string.
    pub(crate) stops: Vec<(f64, String)>,
}

/// Shadow rendering configuration for drop shadow effects on canvas primitives.
///
/// When applied, all subsequent fill, stroke, and draw operations will cast
/// a shadow with the specified color, blur radius, and offset.
#[derive(Clone, Data, Debug, New, PartialEq, PartialOrd)]
pub struct ShadowConfig {
    /// The CSS color string of the shadow (e.g., `"rgba(0,0,0,0.5)"`).
    #[get(type(clone))]
    pub(crate) color: String,
    /// The blur radius of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) blur: f64,
    /// The horizontal offset of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) offset_x: f64,
    /// The vertical offset of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) offset_y: f64,
}

/// Represents the rendering priority layer for draw call ordering.
///
/// Higher z-index values are drawn on top of lower values,
/// enabling correct visual layering of game objects.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct RenderLayer {
    /// The z-index determining draw order. Higher values draw later (on top).
    #[get(type(copy))]
    pub(crate) z_index: i32,
    /// Whether objects in this layer should be rendered.
    #[get(type(copy))]
    pub(crate) visible: bool,
}

/// An ordered buffer of deferred draw commands recorded during a frame.
///
/// Scenes and components push `DrawCommand`s into the list during `on_render`
/// instead of drawing immediately. The engine then replays the whole list once
/// per frame via `CanvasRenderer::replay`, which batches consecutive same-style
/// shapes into a single path and skips redundant canvas state changes. The
/// backing `Vec` is reused across frames via `clear()` to avoid reallocation.
#[derive(Clone, Data, Debug, Default, New)]
pub struct DrawList {
    /// The recorded draw commands for the current frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Vec<DrawCommand>,
}

/// A supersampling anti-aliasing (SSAA) canvas wrapper that renders at a higher
/// resolution on an offscreen canvas and downscales to the display canvas for
/// smoother polygon edges in software-rendered 3D scenes.
///
/// The offscreen context is scaled by `scale_factor` so that all drawing
/// code can use logical pixel coordinates without modification. After
/// rendering, call `present()` to draw the high-resolution buffer onto the
/// visible canvas with high-quality image smoothing.
#[derive(Clone, Data, New)]
pub struct SsaaCanvas {
    /// The display canvas element visible to the user.
    pub(crate) display_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the display canvas used for final presentation.
    pub(crate) display_context: CanvasRenderingContext2d,
    /// The offscreen canvas used for high-resolution rendering.
    pub(crate) offscreen_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the offscreen canvas, pre-scaled by `scale_factor`.
    pub(crate) offscreen_context: CanvasRenderingContext2d,
    /// The supersampling scale factor (e.g., 2.0 means 4x SSAA).
    #[get(type(copy))]
    pub(crate) scale_factor: f64,
    /// The rendering quality preset for the downscaling present step.
    ///
    /// Controls the smoothing strategy when the offscreen buffer is
    /// downscaled onto the display canvas. Defaults to `Medium`.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) quality: RenderQuality,
    /// The logical display width in CSS pixels.
    #[get(type(copy))]
    pub(crate) width: f64,
    /// The logical display height in CSS pixels.
    #[get(type(copy))]
    pub(crate) height: f64,
}

/// A WebGPU rendering backend wrapping the GPU device, queue, and canvas context
/// for GPU-accelerated rendering on the web.
///
/// Created asynchronously via `WebGpuRenderer::init` because adapter and
/// device acquisition returns JavaScript Promises that must be awaited.
/// Once initialized, the renderer provides methods to create GPU resources
/// (buffers, shader modules, command encoders) and execute render passes.
///
/// WebGPU types are stored as `JsValue` to avoid feature-gated import issues
/// with `web_sys`. Method calls are performed via `Reflect` and `JsCast`.
#[derive(Clone, Data)]
pub struct WebGpuRenderer {
    /// The WebGPU device (`GpuDevice`) used to create GPU resources.
    pub(crate) device: JsValue,
    /// The device's command queue (`GpuQueue`) for submitting command buffers.
    pub(crate) queue: JsValue,
    /// The WebGPU canvas rendering context (`GpuCanvasContext`).
    pub(crate) context: JsValue,
    /// The HTML canvas element backing the WebGPU context.
    pub(crate) canvas: HtmlCanvasElement,
    /// The texture format string used by the canvas's swap chain (e.g., `"bgra8unorm"`).
    #[get(type(clone))]
    pub(crate) format: String,
    /// The physical pixel width of the canvas backing store.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The physical pixel height of the canvas backing store.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Whether MSAA anti-aliasing is enabled for render pipelines.
    ///
    /// When `true`, the renderer allocates a multisampled intermediate texture
    /// (`sampleCount: 4`) and resolves into the swap chain each frame; when
    /// `false`, render passes attach directly to the swap chain view at
    /// `sampleCount: 1`.
    #[get(type(copy))]
    pub(crate) antialias: bool,
    /// The multisampled color texture used when `antialias` is `true`.
    ///
    /// `None` when MSAA is disabled. Rebuilt on every resize because the
    /// `width`/`height` are immutable for a given `GpuTexture`.
    #[get(type(clone))]
    pub(crate) multisample_texture: Option<JsValue>,
    /// The default `GpuTextureView` into `multisample_texture`.
    ///
    /// Cached at texture-create time so `begin_render_pass` does not have to
    /// recreate the view each frame. `None` when MSAA is disabled.
    #[get(type(clone))]
    pub(crate) multisample_view: Option<JsValue>,
    /// The depth-stencil texture used for depth-tested passes.
    ///
    /// Created lazily on the first call to [`WebGpuRenderer::begin_render_pass`]
    /// that includes a `depthStencil` attachment. Rebuilt on every resize
    /// because the dimensions are immutable for a given `GpuTexture`. The
    /// matching default view is cached in `depth_view`.
    ///
    /// `None` until the first depth-tested render pass is opened.
    #[get(type(clone))]
    pub(crate) depth_texture: Option<JsValue>,
    /// The default `GpuTextureView` into `depth_texture`.
    ///
    /// `None` when no depth texture has been allocated.
    #[get(type(clone))]
    pub(crate) depth_view: Option<JsValue>,
    /// The depth-stencil format used for `depth_texture`.
    ///
    /// Stored so subsequent render-pass openers can pass the same format
    /// to the pipeline layout without having to remember it externally.
    /// `None` until the first depth texture is allocated.
    #[get(type(clone))]
    pub(crate) depth_format: Option<String>,
    /// User-supplied closure fired when the underlying `GpuDevice` enters
    /// the `lost` state (browser-initiated context loss, OS driver crash,
    /// `device.destroy()`, ...).
    ///
    /// `None` until the caller calls [`WebGpuRenderer::on_device_lost`].
    /// The renderer also stores a separate `device_lost_handle` that
    /// forwards the `GPUDeviceLostInfo` JS value into this callback.
    #[get(type(clone))]
    pub(crate) device_lost_callback: Option<js_sys::Function>,
    /// Whether the device is currently in the `lost` state.
    ///
    /// Once flipped to `true`, every GPU operation returns
    /// `Err(WebGpuError::RendererDisposed)` until the caller destroys the
    /// renderer and creates a new one (WebGPU has no "recover from lost
    /// device" API).
    #[get(type(copy))]
    pub(crate) device_lost: bool,
    /// Shared slot for the most recent popped error-scope value.
    ///
    /// `device.popErrorScope()` returns a `Promise<GPUError?>`; we
    /// cannot `.await` it from a sync call site. Instead, every
    /// `push_error_scope` + `pop_error_scope` pair registers a
    /// microtask via `wasm_bindgen_futures::spawn_local` that stores
    /// the resolved value here. Callers that want the error
    /// synchronously call [`WebGpuRenderer::take_last_error`] to
    /// drain the slot.
    ///
    /// Holding a `Rc<PendingErrorCell>` lets the spawn_local future
    /// own its own handle independently of `&self`, so the
    /// renderer's borrow checker stays happy. The slot is empty
    /// (`None`) by default and after each successful take.
    ///
    /// The cell is intentionally `PendingErrorCell` (a `Sync`
    /// `UnsafeCell` newtype, see [`crate::renderer::static`]) rather
    /// than `Rc<RefCell<...>>`: the WASM single-threaded scheduler
    /// makes the runtime borrow check `RefCell` provides unreachable
    /// in practice, so we trade it for a raw `UnsafeCell` deref
    /// confined to two call sites. This mirrors how euv-core
    /// implements its global registries
    /// (`core/src/renderer/registry/struct.rs:62`).
    pub(crate) pending_error: Rc<PendingErrorCell>,
    /// The currently-open `GpuCommandEncoder`, if any.
    ///
    /// WebGPU expects the application to encode all work for a
    /// frame (clear, render passes, compute passes, copy ops) into
    /// a single command encoder, then call `encoder.finish()` to
    /// produce a `GpuCommandBuffer` and submit it to the queue.
    /// The encoder is `None` after `submit()` finishes and must
    /// be re-acquired via `device.createCommandEncoder()` before
    /// the next frame.
    #[get(type(clone))]
    pub(crate) command_encoder: Option<JsValue>,
    /// OPT 34: cached render-pass descriptor, allocated lazily on the
    /// first call to [`WebGpuRenderer::begin_render_pass_full`].
    ///
    /// The pre-WebGPU-audit path allocated a fresh `Object` +
    /// `Array` + 8-15 `Reflect::set` calls every frame. We keep the
    /// descriptor Object alive for the renderer's lifetime, refreshing
    /// the per-frame fields (`view`, `resolveTarget`, `clearValue`) in
    /// place on every call. The cache invalidates itself automatically
    /// when `load_op` / `store_op`, the depth-stencil shape, or the
    /// resolve-target shape changes.
    ///
    /// `None` until the first `begin_render_pass_full` call; `Some(_)`
    /// afterwards and persists for the lifetime of the renderer.
    #[get(type(clone))]
    pub(crate) render_pass_descriptor_cache: Option<RenderPassDescriptorCache>,
}

/// OPT 34: persistent render-pass descriptor and its inner
/// attachments, reused across `begin_render_pass_full` calls.
///
/// # Why
///
/// `begin_render_pass_full` historically allocated a fresh descriptor
/// `Object`, a `colorAttachments` `Array`, and one inner
/// `color_attachment` `Object` (plus an optional `clearValue` `Object`)
/// on every frame, then ran 8-15 `Reflect::set` calls to populate
/// them. WebGPU re-validates the descriptor each call, but the JS-side
/// `Object` / `Array` allocations and the per-property `Reflect::set`
/// crossings are pure overhead — only the `clearValue` and (rarely)
/// `view` / `loadOp` / `storeOp` fields change between frames.
///
/// # What we cache
///
/// - The top-level descriptor `Object` (the one passed to
///   `beginRenderPass`).
/// - The `colorAttachments` `Array` (always exactly one element —
///   we keep the same `Array` reference and mutate its slot 0 in
///   place).
/// - The inner color attachment `Object` (slot 0 of
///   `colorAttachments`).
/// - The `clearValue` `Object` (the `{r, g, b, a}` dictionary that
///   is the actual per-frame mutating field).
/// - Last-applied `loadOp` / `storeOp` string slices, to detect when
///   the caller switched ops and the cached descriptor must be
///   rebuilt (rare; WebGPU does not hot-swap ops every frame).
/// - Last-applied depth-stencil shape (present / absent), to detect
///   when the depth-stencil shape changes.
///
/// # Invalidation
///
/// The cache is invalidated (rebuilt from scratch) when any of:
/// - `load_op` changes between calls,
/// - `store_op` changes between calls,
/// - the depth-stencil shape changes (None → Some / Some → None),
/// - the resolve-target shape changes (MSAA on/off).
///
/// `view` / `resolveTarget` / `clearValue` are refreshed on every call
/// (the swap-chain view expires after each presented frame, so caching
/// it across frames silently invalidates every subsequent render pass).
///
/// These are all `&'static str` (they come from `WEBGPU_*_OP_*`
/// constants), so invalidation is a pointer-compare.
///
/// `Clone` is derived so the parent `WebGpuRenderer`'s `Data` derive
/// (which adds a `Clone` bound on every field) keeps compiling;
/// `js_sys::Object` and `js_sys::Array` both derive `Clone`, so the
/// derived `Clone` impl just clones the inner JS-side references
/// (cheap, no JS allocation).
#[derive(Clone, Debug)]
pub(crate) struct RenderPassDescriptorCache {
    /// The cached top-level `GpuRenderPassDescriptor` Object.
    /// Pass directly to `encoder.beginRenderPass(descriptor)`.
    pub(crate) descriptor: Object,
    /// The cached inner color attachment Object.
    /// `descriptor.colorAttachments[0]` in JS terms.
    pub(crate) attachment: Object,
    /// The cached `clearValue` Object (the `{r, g, b, a}` dictionary
    /// under `attachment.clearValue`). The hot-path field — only
    /// this is mutated on most frames.
    pub(crate) clear_value: Object,
    /// Last applied `loadOp` (as a `&'static str`). Used to detect
    /// op changes that invalidate the descriptor.
    pub(crate) last_load_op: Option<&'static str>,
    /// Last applied `storeOp` (as a `&'static str`). Used to detect
    /// op changes that invalidate the descriptor.
    pub(crate) last_store_op: Option<&'static str>,
    /// Whether the last applied descriptor had a depth-stencil
    /// attachment (`true`) or not (`false`). Used to detect shape
    /// changes that invalidate the descriptor.
    pub(crate) last_has_depth: bool,
    /// Whether the last applied descriptor had a `resolveTarget`
    /// (`true`, MSAA path) or not (`false`). Used to detect shape
    /// changes that invalidate the descriptor, so a stale
    /// `resolveTarget` never survives an MSAA -> non-MSAA switch.
    pub(crate) last_has_resolve: bool,
}

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

/// A WebGL 2 rendering backend wrapping the `WebGl2RenderingContext`.
///
/// Unlike `WebGpuRenderer`, which stores all GPU handles as opaque `JsValue`s,
/// WebGL exposes concrete `web_sys` types, so the context and canvas are kept
/// as strongly typed values. Shader programs created via
/// [`WebGlRenderer::create_program`] are managed by the caller.
///
/// Construct via [`WebGlRenderer::init`], which resolves the canvas from the
/// [`RenderConfig`], applies device-pixel-ratio scaling to the backing store,
/// and acquires the `webgl2` context.
#[derive(Clone, Data)]
pub struct WebGlRenderer {
    /// The WebGL 2 rendering context used for all GL calls.
    pub(crate) context: WebGl2RenderingContext,
    /// The HTML canvas element backing the WebGL context.
    pub(crate) canvas: HtmlCanvasElement,
    /// The physical pixel width of the canvas backing store.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// The physical pixel height of the canvas backing store.
    #[get(type(copy))]
    pub(crate) height: u32,
}

// =====================================================================
// WebGPU: descriptor & data structs (consumed by the WebGpuRenderer API)
// =====================================================================

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
    /// The WGSL vertex format (e.g. `"float32x4"`).
    #[get(type(clone))]
    pub(crate) format: &'static str,
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
    /// The WGSL texture format (e.g. `"rgba8unorm"`, `"bgra8unorm"`,
    /// `"rgba16float"`, `"depth24plus-stencil8"`).
    #[get(type(clone))]
    pub(crate) format: &'static str,
    /// The number of mip levels. `0` is treated as `1`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) mip_level_count: u32,
    /// The number of samples per texel (`1` for non-MSAA, `4` for MSAA).
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) sample_count: u32,
    /// The WGSL usage flags (e.g. `"RENDER_ATTACHMENT | TEXTURE_BINDING |
    /// COPY_DST | COPY_SRC"`).
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) usage: &'static str,
}

/// A sampler descriptor for `create_sampler`.
///
/// Defaults produce a non-filtering clamp-to-edge sampler. Override
/// fields after constructing to enable linear filtering, repeat
/// addressing, or depth comparison.
#[derive(Clone, Debug, Getter, New)]
pub struct GpuSamplerDescriptor {
    /// Minification filter.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) mag_filter: &'static str,
    /// Magnification filter.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) min_filter: &'static str,
    /// Mipmap filter.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) mipmap_filter: &'static str,
    /// U address mode.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) address_mode_u: &'static str,
    /// V address mode.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) address_mode_v: &'static str,
    /// W address mode.
    #[get(type(clone))]
    #[new(skip)]
    pub(crate) address_mode_w: &'static str,
    /// Whether the sampler is a comparison sampler.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) compare: bool,
}

/// The descriptor for a single (color or depth-stencil) render pass
/// attachment, used as input to `begin_render_pass` / `begin_render_pass_to_texture`.
#[derive(Clone, Debug)]
pub struct RenderPassColorAttachment {
    /// The texture view to draw into.
    ///
    /// When `None`, the renderer uses the swap-chain view (or the MSAA
    /// intermediate view if `antialias == true`).
    pub(crate) view: Option<JsValue>,
    /// An optional resolve target for MSAA.
    ///
    /// `None` when MSAA is disabled. The renderer fills in the default
    /// resolve target (the swap-chain view) when MSAA is enabled and the
    /// caller leaves this as `None`.
    pub(crate) resolve_target: Option<JsValue>,
    /// The clear color as `(r, g, b, a)` in `0.0..=1.0`. `None` means
    /// `"load"` (keep the previous contents).
    pub(crate) clear_value: Option<(f64, f64, f64, f64)>,
    /// The load operation. `None` → `"clear"` when `clear_value` is
    /// `Some`, otherwise `"load"`.
    pub(crate) load_op: Option<&'static str>,
    /// The store operation. `None` → `"store"`.
    pub(crate) store_op: Option<&'static str>,
}

/// The depth-stencil portion of a `RenderPassDescriptor`, used as input to
/// `begin_render_pass` / `begin_render_pass_to_texture`.
#[derive(Clone, Debug)]
pub struct RenderPassDepthStencilAttachment {
    /// The depth-stencil texture view to use.
    ///
    /// When `None`, the renderer uses the default view into its
    /// `depth_texture` field, allocating the depth texture lazily if
    /// needed.
    pub(crate) view: Option<JsValue>,
    /// The depth clear value in `0.0..=1.0`. `None` means
    /// `"load"` (keep previous depth).
    pub(crate) depth_clear_value: Option<f32>,
    /// The depth load op. `None` → `"clear"` when
    /// `depth_clear_value` is `Some`, otherwise `"load"`.
    pub(crate) depth_load_op: Option<&'static str>,
    /// The depth store op. `None` → `"store"`.
    pub(crate) depth_store_op: Option<&'static str>,
    /// Whether depth reads should be enabled. `None` → `false`.
    pub(crate) depth_read_only: Option<bool>,
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

/// Interior-mutable slot for the renderer's pending error-scope value.
///
/// This is the `euv-engine` analog of euv-core's `HandlerRegistryCell`
/// (`core/src/renderer/registry/struct.rs:62`): a single-element
/// `Sync` wrapper that holds an `Option<JsValue>` behind an
/// `UnsafeCell`.
///
/// # Why this type exists
///
/// `WebGpuRenderer::pending_error` needs interior mutability
/// because:
///
/// 1. `pop_error_sync` takes `&self` (the WebGPU hot path cannot
///    be `async`), but the spawned `wasm_bindgen_futures::spawn_local`
///    future must mutate the slot to store the resolved
///    `Promise<GPUError?>` value.
/// 2. `take_last_error` also takes `&self` and drains the slot
///    on the next render tick.
///
/// The first implementation used `Rc<RefCell<Option<JsValue>>>`,
/// which works but pays for:
///
/// - a `RefCell::borrow_mut` runtime borrow check on every
///   write (the panic path is unreachable in practice — only
///   the spawn_local future and `take_last_error` ever touch
///   the slot, and they never overlap because the future is
///   a microtask drained before the next render tick).
/// - a heap allocation for the `RefCell`'s borrow state.
///
/// The newtype keeps the interior-mutability primitive (`Rc`),
/// because the spawn_local future needs its own owning handle,
/// but swaps the inner cell from `RefCell` to `UnsafeCell`:
///
/// - zero runtime borrow check (the WASM single-threaded
///   scheduler makes the borrow impossible to violate).
/// - zero allocation (the cell is just a `*mut Option<JsValue>`
///   sitting inside the `Rc`-managed box).
///
/// # Sync safety
///
/// `PendingErrorCell` is **not** `Sync` by default (`UnsafeCell`
/// explicitly opts out). We hand-implement `Sync` for it because
/// the renderer is only ever used in the WASM single-threaded
/// runtime; the `Rc` ensures the same instance is never shared
/// across threads (it is not `Send`/`Sync` either), and the
/// WASM main thread is the only place that ever touches the
/// slot. This matches euv-core's pattern
/// (`unsafe impl Sync for HandlerRegistryCell {}`).
///
/// If the engine is ever compiled for a multi-threaded target
/// (native, `wasm-bindgen-rayon`), this `unsafe impl Sync` is
/// unsound and must be removed.
pub struct PendingErrorCell(
    /// Interior-mutable storage for the optional `JsValue`.
    ///
    /// Marked `pub(crate)` (not just `pub`) because the field is
    /// only meant to be touched from inside the renderer module —
    /// specifically from the `impl PendingErrorCell` block in
    /// `impl.rs`. The struct itself stays `pub` so external code
    /// can name the type, but the raw `UnsafeCell` is an
    /// implementation detail.
    pub(crate) UnsafeCell<Option<JsValue>>,
);

/// A single entry inside a `GpuBindGroupLayoutDescriptor`.
///
/// Together these describe one slot of the bind group layout used by
/// a render / compute pipeline. The visibility bitmask controls
/// which shader stages can read the binding (`VERTEX = 0x1`,
/// `FRAGMENT = 0x2`, `COMPUTE = 0x4`); `VERTEX | FRAGMENT = 0x3` and
/// `VERTEX | FRAGMENT | COMPUTE = 0x7` are the most common values.
#[derive(Clone, Debug)]
pub struct BindGroupLayoutEntry {
    /// The binding slot (matches `@binding(N)` in the shader).
    pub binding: u32,
    /// Visibility bitmask (`VERTEX = 0x1`, `FRAGMENT = 0x2`, `COMPUTE = 0x4`).
    pub visibility: u32,
    /// The resource kind bound at this slot.
    pub ty: BindGroupEntryType,
}
