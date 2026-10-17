use super::*;

/// A reusable ray-trace surface driven by one fixed backend.
///
/// The renderer owns every resource its backend needs: the packed CPU
/// buffers, the GPU textures, buffers, program, pipeline, and bind
/// groups, and the output pixel buffer. They are created once — during
/// [`RayTraceRenderer::init`](super::RayTraceRenderer::init), a resize,
/// or a scene update — and reused by every later
/// [`render`](super::RayTraceRenderer::render). A steady-state frame
/// allocates nothing and repacks nothing.
///
/// The backend is explicit. There is no capability probe and no
/// fallback: a WebGPU request on a browser without WebGPU returns an
/// error instead of quietly drawing a Canvas 2D frame.
///
/// Every method returns [`JsValue`]. An `Ok` always means the frame was
/// submitted; a failure never degrades to `JsValue::UNDEFINED`, which
/// would make a dropped frame indistinguishable from a good one.
///
/// Every field is `pub(crate)` but is only ever touched through the
/// accessors the `Data` derive generates, so no consumer can hold a
/// second reference to an owned resource or write a size the renderer's
/// own accounting does not know about.
#[derive(Debug, Data)]
pub struct RayTraceRenderer {
    /// The canvas element every backend draws into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas: HtmlCanvasElement,
    /// The backend this renderer was constructed for, immutable for the
    /// renderer's whole lifetime.
    #[get(pub(crate), type(copy))]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) backend: RayTraceBackend,
    /// The active backend adapter, or `None` once released.
    ///
    /// There is deliberately no plain getter: the adapter owns every GL
    /// and GPU object the renderer holds and is not `Clone`, so handing
    /// out a copy is impossible and a `get_*` accessor on an `Option`
    /// field would unwrap. Reads borrow through the mutable accessor,
    /// which yields the only reference to the live adapter.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) adapter: Option<RayTraceBackendAdapter>,
    /// The CPU mirror of the scene, so a failed GPU upload keeps the
    /// previous, still renderable, scene. The packed block itself is not
    /// mirrored: the GPU copy is the live one and a second copy would be
    /// allocated on every update and read by nobody.
    #[get(pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) scene: RayTraceScene,
    /// The packed frame block, refilled every frame through its reused
    /// capacity.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) packed_frame: Vec<f32>,
    /// The reused RGBA output buffer of the Canvas path. It stays empty
    /// on the GPU paths, which present through the driver instead of
    /// through a CPU copy of the frame.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pixels: Vec<u8>,
    /// The surface width in pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) width: u32,
    /// The surface height in pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) height: u32,
    /// Whether [`dispose`](super::RayTraceRenderer::dispose) already ran.
    ///
    /// This is set before any resource is released, which is what makes
    /// `dispose` idempotent and stops the drop cleanup from destroying
    /// the same GL or GPU object twice.
    #[get(pub(crate), type(copy))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(crate) disposed: bool,
}
