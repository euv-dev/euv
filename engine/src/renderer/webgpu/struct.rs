use super::*;

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
    /// than `Rc<RefCell<...>>` - the WASM single-threaded scheduler
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
pub struct RenderPassDescriptorCache {
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
