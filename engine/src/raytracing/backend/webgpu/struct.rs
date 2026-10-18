use super::*;

/// The WebGPU backend: a fullscreen triangle over two storage buffers.
///
/// The device, pipeline, bind group, and both packed buffers are created
/// once. A buffer grows only when a block no longer fits, and a
/// steady-state frame is two `writeBuffer` calls plus one render pass
/// over three vertices — no allocation and no await.
///
/// Every WebGPU handle is a `JsValue` reached through `Reflect`. The
/// typed `Gpu*` bindings in `web-sys` sit behind a `cfg` the library
/// cannot force on its own consumers, and a `Reflect` call reports a
/// throw as a `Result` the caller must handle, so every failure is
/// propagated instead of being folded into a silent `undefined`.
///
/// Validation is awaited during construction rather than assumed: the
/// shader module's compilation info and every resource allocation are
/// wrapped in an error scope that is popped and inspected. Wrapping the
/// synchronous calls alone would report success for an invalid pipeline,
/// because WebGPU reports validation asynchronously.
///
/// The device is requested here and owned here for the backend's whole
/// lifetime. It is destroyed on release: a device this backend created
/// is not a shared resource, and leaving it alive would pin the
/// adapter's resources for the life of the page.
#[derive(Data, Debug)]
pub struct WebGpuRayTraceBackend {
    /// The WebGPU canvas rendering context, already configured.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: JsValue,
    /// The device every resource is created from, owned by this backend.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) device: JsValue,
    /// The device's queue, used for the packed-buffer writes and the
    /// command submission.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) queue: JsValue,
    /// The bind-group layout the persistent bind group was created
    /// against.
    ///
    /// A bind group records the layout it was created with, so a grown
    /// buffer needs a new bind group built from the *same* layout the
    /// pipeline layout names. Recreating the layout here and passing
    /// the new one to `createBindGroup` would produce a group the
    /// pipeline rejects at draw time.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) layout: JsValue,
    /// The storage buffer holding the packed scene block.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scene_buffer: JsValue,
    /// The storage buffer holding the packed frame block.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frame_buffer: JsValue,
    /// The bind group pairing both buffers with the persistent layout.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bind_group: JsValue,
    /// The compiled WGSL pipeline, resolved asynchronously at attach.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline: JsValue,
    /// The byte capacity of the current scene buffer, so a smaller
    /// scene reuses it instead of re-allocating.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scene_capacity: u32,
    /// The byte capacity of the current frame buffer.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frame_capacity: u32,
    /// The device's largest addressable storage binding in bytes, or
    /// zero when the device reports none.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) storage_binding_limit: u32,
    /// The device's largest allocatable buffer in bytes, or zero when
    /// the device reports none.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) buffer_size_limit: u32,
    /// The device's largest texture edge in pixels, or zero when the
    /// device reports none.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) texture_dimension_limit: u32,
    /// The failure the browser reported since the last frame drained
    /// it, shared by the lost and uncaptured-error handlers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) failure: Rc<RefCell<Option<String>>>,
    /// The registered `uncapturederror` listener, removed on release.
    ///
    /// The listener owns a Rust callback and the device holds a strong
    /// reference to it, so the handle is kept here rather than dropped:
    /// releasing the backend has to hand the exact same function
    /// reference back to `removeEventListener`, and a callback that were
    /// dropped first would be invoked by the next validation error on a
    /// device this backend no longer owns.
    ///
    /// The `device.lost` callback is *not* stored. That promise settles
    /// exactly once — and destroying the device settles it — so it is
    /// built as a one-shot callback owned by JavaScript
    /// (`Closure::once_into_js`), which frees its captured state the
    /// moment the browser calls it. Holding it in this struct and
    /// clearing it before `destroy` is what produced `closure invoked
    /// recursively or after being dropped`.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) error_listener: Option<JsValue>,
    /// Whether release already ran, so a second release destroys
    /// nothing twice.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) released: bool,
}
