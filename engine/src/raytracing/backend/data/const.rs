/// The `getContext` context id for a Canvas 2D rendering context.
pub(crate) const CANVAS_CONTEXT_ID_2D: &str = "2d";

/// The `getContext` context id for a WebGL 2 rendering context.
pub(crate) const WEBGL_CONTEXT_ID_2D: &str = "webgl2";

/// The `getContext` context id for a WebGPU canvas rendering context.
pub(crate) const WEBGPU_CONTEXT_ID: &str = "webgpu";

/// The `GL_TEXTURE_2D` target the packed data textures bind to.
pub(crate) const WEBGL_TEXTURE_TARGET_2D: u32 = 0x0de1;

/// The first texture unit the packed scene texture binds to.
pub(crate) const WEBGL_TEXTURE_UNIT_SCENE: u32 = 0x84c0;

/// The second texture unit the packed frame texture binds to.
pub(crate) const WEBGL_TEXTURE_UNIT_FRAME: u32 = 0x84c1;

/// The base mip level every packed texture allocation and sub-image
/// write targets.
pub(crate) const WEBGL_MIP_LEVEL_ZERO: i32 = 0;

/// The `RGBA32F` sized internal format the packed data textures use.
pub(crate) const WEBGL_FORMAT_RGBA32F: i32 = 0x8814;

/// The `RGBA` pixel format matching [`WEBGL_FORMAT_RGBA32F`].
pub(crate) const WEBGL_PIXEL_FORMAT_RGBA: u32 = 0x1908;

/// The `FLOAT` pixel type a `RGBA32F` texture uploads through.
pub(crate) const WEBGL_TYPE_FLOAT: u32 = 0x1406;

/// The `TEXTURE_MIN_FILTER` parameter, set to `NEAREST` so no float
/// filtering extension is ever consulted.
pub(crate) const WEBGL_TEXTURE_MIN_FILTER: u32 = 0x2801;

/// The `TEXTURE_MAG_FILTER` parameter, set to `NEAREST`.
pub(crate) const WEBGL_TEXTURE_MAG_FILTER: u32 = 0x2800;

/// The `TEXTURE_WRAP_S` parameter.
pub(crate) const WEBGL_TEXTURE_WRAP_S: u32 = 0x2802;

/// The `TEXTURE_WRAP_T` parameter.
pub(crate) const WEBGL_TEXTURE_WRAP_T: u32 = 0x2803;

/// The `CLAMP_TO_EDGE` wrap mode, the only wrap mode that keeps an
/// out-of-range `texelFetch` from reading the opposite edge.
pub(crate) const WEBGL_CLAMP_TO_EDGE: i32 = 0x812f;

/// The `NEAREST` minification and magnification filter.
pub(crate) const WEBGL_NEAREST: i32 = 0x2600;

/// The base of the `TRIANGLES` draw every backend issues: the
/// fullscreen triangle is three vertices.
pub(crate) const WEBGL_FIRST_VERTEX: i32 = 0;

/// The vertex count of the fullscreen triangle.
pub(crate) const WEBGL_VERTEX_COUNT: i32 = 3;

/// The error reported when a renderer is used after
/// [`RayTraceRenderer::dispose`](super::RayTraceRenderer::dispose).
pub(crate) const ERROR_DISPOSED: &str = "the ray-trace renderer has been disposed";

/// The error reported when a zero-sized surface is requested.
pub(crate) const ERROR_ZERO_DIMENSION: &str = "a ray-trace surface must be at least 1x1";

/// The error reported when the requested dimensions overflow the
/// addressable byte count of a `usize` allocation.
pub(crate) const ERROR_DIMENSION_OVERFLOW: &str =
    "the requested ray-trace dimensions exceed the addressable buffer size";

/// The error reported when one requested dimension exceeds the
/// per-axis pixel ceiling the backend enforces.
pub(crate) const ERROR_GPU_LIMIT: &str =
    "one requested ray-trace dimension exceeds this backend's per-axis pixel limit";

/// The error reported when the scene cannot be packed for the GPU.
pub(crate) const ERROR_SCENE_PACK: &str = "the scene cannot be packed for the GPU";

/// The error reported when the frame block cannot be packed.
pub(crate) const ERROR_FRAME_PACK: &str = "the frame block cannot be packed";

/// The error reported when the browser exposes no WebGL 2 context.
pub(crate) const ERROR_WEBGL_UNAVAILABLE: &str =
    "canvas.get_context('webgl2') did not yield a WebGL 2 context";

/// The error reported when a WebGL call throws.
pub(crate) const ERROR_WEBGL_CALL: &str = "a WebGL 2 call threw";

/// The error reported when the WebGL context was lost.
pub(crate) const ERROR_WEBGL_CONTEXT_LOST: &str = "the WebGL 2 context is lost";

/// The error reported when the packed texture could not be created.
pub(crate) const ERROR_WEBGL_TEXTURE: &str = "the WebGL 2 packed texture could not be created";

/// The error reported when the ray-trace program could not be created.
pub(crate) const ERROR_WEBGL_PROGRAM: &str = "the ray-trace WebGL 2 program could not be created";

/// The error reported when the browser exposes no WebGPU.
pub(crate) const ERROR_WEBGPU_UNAVAILABLE: &str = "navigator.gpu is unavailable";

/// The error reported when the WebGPU adapter request failed.
pub(crate) const ERROR_WEBGPU_ADAPTER: &str = "no WebGPU adapter is available";

/// The error reported when the WebGPU device request failed.
pub(crate) const ERROR_WEBGPU_DEVICE: &str = "no WebGPU device is available";

/// The error reported when the WebGPU canvas context is missing.
pub(crate) const ERROR_WEBGPU_CONTEXT: &str =
    "canvas.get_context('webgpu') did not yield a WebGPU context";

/// The error reported when a WebGPU resource creation threw.
pub(crate) const ERROR_WEBGPU_CALL: &str = "a WebGPU call threw";

/// The error reported when the WebGPU device was lost.
pub(crate) const ERROR_WEBGPU_DEVICE_LOST: &str = "the WebGPU device was lost";

/// The error reported when the browser exposes no Canvas 2D context.
pub(crate) const ERROR_CANVAS_UNAVAILABLE: &str =
    "canvas.get_context('2d') did not yield a 2D context";

/// The error reported when the `ImageData` covering the surface could
/// not be allocated.
pub(crate) const ERROR_CANVAS_IMAGE_DATA: &str =
    "the ImageData covering the ray-trace surface could not be allocated";

/// The error reported when the canvas blit threw.
pub(crate) const ERROR_CANVAS_PUT: &str = "the canvas ImageData blit threw";

/// The error reported when an adapter resolves to no device.
pub(crate) const ERROR_WEBGPU_NO_ADAPTER: &str = "the WebGPU adapter request resolved to null";

/// The number of RGBA byte channels one surface pixel occupies, which
/// is what turns a pixel count into the output buffer's length.
pub(crate) const SURFACE_RGBA_CHANNELS: usize = 4;
